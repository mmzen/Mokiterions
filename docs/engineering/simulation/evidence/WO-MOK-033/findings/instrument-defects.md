# The four defects verification found, and the proof each repair bites

`VER-MOK-019`'s *Independence* point 2: an instrument's own cross-checks are never their own
evidence. The corollary applied here is that a test which passes against the defect it claims to
catch has verified nothing. `harness/bite/check.py` copies the driver and its suite, patches one
defect back into the copy, runs the one test that names it, and records the result. It exits 0 only
if every test fails on its own pre-fix copy. Its output is `harness/bite/bite-output.txt`:

    3 of 3 tests fail on their own pre-fix copy

Each patch keeps the surrounding structure -- `pass` for a removed statement, `if True:` for a
removed `finally:` -- so what is measured is the defect rather than a hole where the code was. One
patch is applied per run, on a fresh copy, so no patch can mask another.

## 1. The explicit binary path was not normalised

`SPEC-MOK-008` rule 2.3 takes `--binary <path>` from the operator. On Windows a relative path written
with forward slashes -- `target/release/Mokiterions.exe` -- passes `os.path.isfile` and then fails to
launch, because the child is started from a command line the system parses rather than from a path it
opens. A 400-cell sweep therefore reported 400 identical failures and no cells.

Found by case B1, against the real engine. The `runner` seam the suite uses cannot find it: the seam
never reaches the process boundary, so the path is never opened by anything. Repaired by normalising
in `resolve_binary`, which lets an operator write the path either way.

Test: `TestBinaryResolution.test_an_explicit_path_is_normalised_to_the_platforms_separator`.

## 2. The suite's test for defect 1 did not bite

The test above was written to catch a regression, and against a copy with the normalisation removed
it passed. It took its relative path from `os.path.relpath(__file__)`, which carries a separator only
when the suite is started from somewhere other than `scripts/`; started from that directory the path
is a bare file name, the forward-slash substitution does nothing, and both forms of the code agree.
The suite is run from the repository root, where the test does bite -- so this was a test that worked
only from the directory it happened to be run from.

Found by `harness/bite/check.py`, which runs the suite from its own directory. This is the only defect
in this packet that was found by an instrument built to check the instruments, and it is the reason
that instrument exists. Repaired by constructing the relative path inside a temporary tree, so it
carries a separator whatever the working directory is; the case now uses `os.chdir` around the call
rather than `contextlib.chdir`, which would have put a Python 3.11 floor under a suite whose
specification states no minimum version.

## 3. One reusable stream path was handed to two concurrent cells

Rule 7.1 gives the driver one reusable path per worker; rule 2.8's `--jobs <n>` runs cells
concurrently and promises to change no row. The slot was derived from the cell's position in the
sweep, `index % jobs`. With eight cells over four workers, position 4 begins as soon as *any* of
positions 0 to 3 finishes, which is not necessarily position 0, so two concurrent cells could hold
one path.

Measured against the real engine: of eight cells at `--jobs 4`, **two were lost** -- one read a
half-written line, the other found the file already removed by its neighbour. Repaired by taking the
slot from a `queue.SimpleQueue` pool, which makes exclusion a property of the pool's size rather than
of the scheduler's order.

Test: `TestRetainedFileAndStream.test_no_two_concurrent_cells_share_a_reusable_path`. It states the
property as an exclusion over observed time spans rather than as an output comparison, because the
byte-identity test cannot see the defect: a runner that writes its stream instantly leaves no window
for two cells to overlap. The cells are given deliberately unequal durations for the same reason --
equal ones let the first wave finish together and no two ever overlap.

## 4. The stream was not removed when a cell was interrupted

Rule 7.3 removes each cell's stream when the cell is done. On `KeyboardInterrupt` the removal was
skipped, leaving `batch-stream-0.jsonl` on disk while the batch record's `leftover_streams` said `[]`.
Both halves are wrong, and the second is worse: the record disagreed with the disk, which is exactly
what a leftover-streams field exists to prevent.

Repaired with a `finally:`. Test:
`TestRetainedFileAndStream.test_the_stream_is_removed_when_a_cell_is_interrupted`, which asserts the
record and the directory listing agree. On the pre-fix copy it reports
`['batch-stream-0.jsonl'] != []`.

## What this says about the instrument

Three of the four were only findable against the real engine or against a real interrupt. Two of them
-- 1 and 3 -- lose cells silently or noisily but in either case produce a sweep an operator would have
read as a measurement. The suite's 78 cases were all passing before any of the four was found. That is
the argument for `VER-MOK-019`'s Independence point 3, that at least one case runs the real engine end
to end, and it is worth restating in the next contract of this shape.
