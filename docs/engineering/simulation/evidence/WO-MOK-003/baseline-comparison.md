# WO-MOK-003 evidence: pre-change baseline and equivalence comparison

`SPEC-MOK-002` rule 11 requires that for identical arguments and identical seed the program emits
byte-identical output, reaches an identical final state, and returns an identical exit code — with and
without `--trace-actions`, under both decision sources, at every declared density. `VER-MOK-003`
makes the pre-change baseline the independent oracle for that claim, captured once before any edit and
never regenerated.

This is the record of the capture and of the comparison.

## Why the baseline is the only oracle available

The suite cannot verify equivalence on its own. The 52 tests were relocated by the same change they
would be judging, and 15 of them now reach the code by a different path. A green suite after the
change shows the assertions still hold; it cannot show that no observable byte moved, because no test
asserts on the full event stream of a 1,000-tick run.

The baseline can, because it was taken from a build of the unmodified tree.

## Capture

**Commit.** The baseline was captured from the working tree at
`77010d02319051a20f8e45282f9c813ce4199956`, the tip of `master` and the parent of this work order's
first commit, before any file was edited. The `after` capture was taken from the finished tree on
branch `feature/library-target-and-test-placement`.

**Script.** `baseline/capture.sh`, retained verbatim and run unmodified for both captures. It takes
one argument, the output directory, so the identical script produced both sides. It reads nothing from
the environment and uses no timestamps, which is what makes its output comparable at all.

**Comparison mechanism.** `WO-MOK-003` leaves the mechanism to the implementer — "the comparison
mechanism for the baseline, provided it is byte-exact and its method is recorded" is inside its
envelope. The mechanism chosen is a **SHA-256 digest per matrix cell**, plus retained full text for
two short runs, plus retained full text for every diagnostic case.

Hashing is preferred to retaining every byte because the full matrix is roughly 90,000 lines of event
output; a digest is byte-exact, it is cheap to compare, and it cannot be partially checked and
reported as fully checked. Full text is retained where a human needs to read it: the two short runs
make the event format inspectable, and the diagnostic cases are where a wording change would matter
most.

**Matrix.** 5 seeds × 2 policies × 2 densities × 2 trace settings, 1,000 ticks each = **40 cells**.
Seeds `0, 1, 42, 123, 777`; policies `baseline` and `reference`; densities `0.75` and `1.50`; trace
off and on. Plus three further cells: the two 20-tick runs at seed 42 with tracing on, one per policy,
retained in full under `baseline/full/`; and one invocation with no arguments at all, which exercises
the default configuration path. **43 cells total**, one line each in `manifest.txt`.

Each manifest line is `<sha256>  <cell>  lines=<n>  exit=<code>`, so a difference in content, in
volume, or in exit status each shows up independently rather than collapsing into one digest.

**Summaries.** `summaries.txt` records the final summary line of each of the 40 matrix cells in full
text. The summary line carries the termination reason, tick count, survivors, deaths, per-territory
population, and per-class resource counts — that is, the final state. This is what makes "reaches an
identical final state" checkable rather than inferred from the digest.

**Diagnostics.** `exit-codes.txt` records **16 cases** — `--help` and 15 invalid or boundary
invocations — each with its argv, its exit code, the digest and byte length of standard output, and
the **full text of standard error**. Diagnostic wording is part of `REQ-MOK-010`'s observable
interface, so it is compared as text and not as a hash.

**Viability.** `viability.txt` records the five reference-policy summary lines at the declared density
that `VER-MOK-002` verifies, extracted so the survivor floor can be checked without re-reading the
matrix.

## Result

```
IDENTICAL  manifest.txt
IDENTICAL  summaries.txt
IDENTICAL  exit-codes.txt
IDENTICAL  viability.txt
```

All four files are byte-for-byte equal between `baseline/` and `after/`. Consequently:

- all 43 digests match, so no cell's output differs by a byte;
- all 43 line counts match, so no cell emits more or fewer events;
- all 43 exit codes match;
- all 40 summary lines match in full text, so every run reaches an identical final state, including
  the per-territory and per-class counts that no public accessor exposes;
- all 16 diagnostic cases match in exit code, in standard-output digest and length, and in the full
  text of standard error, so no message, no byte of `USAGE`, and no exit code changed.

The two retained full runs were also compared directly, file to file, and are identical. The `after`
copy is not retained in this directory because it is the same bytes; `after/manifest.txt` carries
their digests, which is the durable record:

```
3424133a9b208c990d28b798a1df97a19e25258e507e70aa1481c5cfdd1fab09  short_seed42_baseline_trace_on   lines=623
489897a4ebd4c246f3fc740eadc4ca90d36f50eba4ccc3337fb070747f358fa9  short_seed42_reference_trace_on  lines=624
9c4dde63a72d01dc728e1652d73d6535da69f2100dfcbe7955aaf7f0e5ed093b  no_arguments                     lines=1415
```

## What the baseline covers, and what it does not

Covered by construction: both decision sources; both declared densities; tracing on and off; the
extinction path and the tick-limit path (seed 0 under the baseline policy goes extinct at tick 119,
seeds under the reference policy reach the limit — both appear in `summaries.txt`); the default
configuration; and every diagnostic exit path.

Not covered, and stated so rather than implied: densities other than `0.75` and `1.50`; seeds outside
the declared five; tick limits other than 1, 20, 1,000, and 10,000; and the release profile. Rule 11
scopes the obligation to the declared densities and `VER-MOK-002`'s declared seeds, so this is the
scope of the claim, not a gap in executing it. The 10,000-tick runs are recorded separately in
`resilience-and-viability.md`.

## Integrity of the baseline

The baseline was captured before the first edit and was not touched afterward. It was not
regenerated at any point, including after the two occasions when the build had to be forced to
recompile for the static-check transcripts — those forced rebuilds of the *modified* tree, never of
the baseline. Where a pristine build was needed later, for the compile-time measurement, it was
obtained by extracting commit `77010d0` into a scratch directory with `git archive` rather than by
reverting the working tree, precisely so that no path existed by which the baseline could be
overwritten.
