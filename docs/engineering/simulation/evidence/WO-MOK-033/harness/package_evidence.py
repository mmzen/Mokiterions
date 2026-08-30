#!/usr/bin/env python3
"""Assemble `docs/engineering/simulation/evidence/WO-MOK-033/` from the verification working tree.

Neither instrument does this. `SPEC-MOK-008` rule 15.1 refuses a batch whose output is under
`docs/engineering/`, and rule 16.4 gives the classifier no output path at all, so the packet is
assembled by this third script, which is retained beside the phases it copies.

Every byte is copied verbatim, in binary, with no line-ending translation. The evidence tree carries
`-text` in `.gitattributes`, so what is written here is what git stores and what the manifest hashes.
Some captures are Windows standard-output pipes and carry CRLF; that is how they were produced and it
is recorded, not corrected.

Files over 400 KB are not committed. They are reduced to a digest, a byte count and a line count in
`not-committed.txt`, on the precedent of `WO-MOK-025`'s `manifest.sh` -- the same reason: a packet is
read, and a reader cannot read 141 MB of engine streams. The phase scripts that produced them are
retained, so any of them can be produced again.

`manifest.sha256` is written last and does not cover itself.
"""

from __future__ import annotations

import datetime
import hashlib
import json
import os
import shutil

WORK = os.path.dirname(os.path.abspath(__file__))
REPO = "C:/Users/mathi/Mokiterions-understand-20260829-1340"
OUT = f"{REPO}/docs/engineering/simulation/evidence/WO-MOK-033"
LIMIT = 400_000

PHASES = (
    ("phase-a", "A", "the real engine end to end"),
    ("phase-b", "B", "failure, refusal and disagreement"),
    ("phase-c", "C", "options, classes and revisability"),
    ("phase-d", "D", "static, architecture, security and privacy"),
    ("phase-e", "E", "performance and interruption"),
    ("phase-f", "F", "acceptance scenarios and the famine measurement"),
)

HARNESS = (
    "oracle.py", "fixtures.py",
    "phase_a_real_engine.py", "phase_b_failures.py", "phase_c_options_and_classes.py",
    "phase_d_static_and_security.py", "phase_e_performance.py", "phase_f_scenarios.py",
    "scan_memory_probe.py", "shim.py", "shim.bat", "shim-plan.json", "argvlog.py", "argvlog.bat",
    "package_evidence.py",
    "nosocket/sitecustomize.py", "nosocket/sitecustomize.py.breaches",
    "bite/check.py", "bite/bite-output.txt",
    "bite2/check.py",
)

# Produced before the phases existed, from an earlier state of the classifier, and excluded for that
# reason. `default-sweep.jsonl` is the exception worth keeping a fact from: it is byte-identical to
# the sweep the phases produce, so it counts as a fifth independent production of the same 400 rows.
SKIPPED_ROOT = (
    ("default-sweep.jsonl", "byte-identical to phase-f/default-sweep.jsonl; the fact is kept, the duplicate is not"),
    ("default-sweep.log", "the standard output of that duplicate run"),
    ("dist-source-density.json", "produced before rule 9.6's attempted-and-effective split existed, so not a reading at the candidate"),
    ("dist-source-density.md", "same, in the markdown form"),
)


def digest_of(path):
    hasher = hashlib.sha256()
    with open(path, "rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def lines_of(path):
    count = 0
    with open(path, "rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            count += chunk.count(b"\n")
    return count


def copy(source, target):
    os.makedirs(os.path.dirname(target), exist_ok=True)
    # `copyfile`, not `copy2`: the bytes are the evidence, the modification time is not.
    shutil.copyfile(source, target)


def crs(path):
    with open(path, "rb") as handle:
        body = handle.read()
    return body.count(b"\r\n")


def main():
    shutil.rmtree(OUT, ignore_errors=True)
    os.makedirs(OUT)

    oversize = []
    copied = 0

    for name in HARNESS:
        copy(f"{WORK}/{name}", f"{OUT}/harness/{name}")
        copied += 1

    for directory, _, _ in PHASES + (("fixture-streams", "", ""),):
        for root, _, files in os.walk(f"{WORK}/{directory}"):
            if "__pycache__" in root:
                continue
            for name in sorted(files):
                source = os.path.join(root, name)
                relative = os.path.relpath(source, WORK).replace(os.sep, "/")
                size = os.path.getsize(source)
                if size > LIMIT:
                    oversize.append((relative, size, digest_of(source), lines_of(source)))
                else:
                    copy(source, f"{OUT}/{relative}")
                    copied += 1

    # ----------------------------------------------------------------------------------------------
    # The oversize manifest
    # ----------------------------------------------------------------------------------------------
    body = [
        "Captures over 400,000 bytes: not committed, stated instead",
        "",
        "Each is an engine record stream or a scan input produced by a retained phase script. A packet",
        "is read by a person; 141 MB of streams is not read. Each line is the path the phase wrote, the",
        "byte count, the sha256 of the bytes as written, and the count of newline bytes. Reproduce any of",
        "them by running the phase script that names it -- the streams are deterministic in the engine's",
        "seed, which is what phase A case B7 measures.",
        "",
        f"{len(oversize)} files, {sum(row[1] for row in oversize)} bytes withheld.",
        "",
        "bytes        lines     sha256                                                            path",
    ]
    for relative, size, sha, lines in sorted(oversize):
        body.append(f"{size:<12} {lines:<9} {sha}  {relative}")
    body.append("")
    body.append("Excluded from this packet for a different reason, from the working tree's own root:")
    for name, why in SKIPPED_ROOT:
        body.append(f"  {name}: {why}")
    body.append("")
    write(f"{OUT}/not-committed.txt", "\n".join(body))
    copied += 1

    # ----------------------------------------------------------------------------------------------
    # The case index, read from the phases' own result files rather than restated
    # ----------------------------------------------------------------------------------------------
    results = {}
    for directory, letter, title in PHASES:
        with open(f"{WORK}/{directory}/result.json", "r", encoding="utf-8") as handle:
            results[letter] = json.load(handle)

    index = ["# Case index", "",
             "Every case executed under `VER-MOK-019`, read from each phase's own `result.json`. A case is",
             "one assertion over measured figures; the figures themselves are in the `detail` field of that",
             "file and in the phase's retained text.", ""]
    for directory, letter, title in PHASES:
        result = results[letter]
        index.append(f"## Phase {letter}: {title}")
        index.append("")
        index.append(f"`{directory}/result.json` -- {result['cases']} cases, "
                     f"{len(result['failed'])} failed.")
        index.append("")
        for entry in result["entries"]:
            index.append(f"{'-' if entry['ok'] else '! '} {entry['case']}")
        index.append("")
    total_cases = sum(results[letter]["cases"] for _, letter, _ in PHASES)
    total_failed = sum(len(results[letter]["failed"]) for _, letter, _ in PHASES)
    index.append(f"{total_cases} cases, {total_failed} failed.")
    index.append("")
    write(f"{OUT}/case-index.md", "\n".join(index))
    copied += 1

    # The handoff checkpoint's own evidence. It is written by hand rather than generated, because it
    # carries a digest the governor states and a disclosure the owner reads, and it is copied here so
    # `manifest.sha256` covers it. The formal snapshot ignores retained evidence, so adding this file
    # does not move the digest it names.
    copy(f"{WORK}/handoff-check.md", f"{OUT}/handoff-check.md")
    copied += 1

    for name, text in findings().items():
        write(f"{OUT}/findings/{name}", text)
        copied += 1

    harness_lines = sum(lines_of(f"{WORK}/{name}") for name in HARNESS if name.endswith((".py", ".bat", ".json")))
    sweep_bytes = os.path.getsize(f"{WORK}/phase-f/default-sweep.jsonl")
    sweep_digest = digest_of(f"{WORK}/phase-f/default-sweep.jsonl")
    write(f"{OUT}/README.md", readme(results, total_cases, total_failed, oversize,
                                    harness_lines, sweep_bytes, sweep_digest))
    copied += 1

    # Read from the copies, not from the sources: what is stated is what this tree holds.
    carrying = []
    for root, _, files in os.walk(OUT):
        for name in sorted(files):
            path = os.path.join(root, name)
            count = crs(path)
            if count:
                carrying.append((os.path.relpath(path, OUT).replace(os.sep, "/"), count))
    write(f"{OUT}/assembly.txt", assembly(oversize, sorted(carrying)))
    copied += 1

    # ----------------------------------------------------------------------------------------------
    # The manifest, last, over everything else
    # ----------------------------------------------------------------------------------------------
    covered = []
    for root, _, files in os.walk(OUT):
        for name in sorted(files):
            path = os.path.join(root, name)
            relative = os.path.relpath(path, OUT).replace(os.sep, "/")
            if relative == "manifest.sha256":
                continue
            covered.append((relative, os.path.getsize(path), digest_of(path), lines_of(path)))
    covered.sort()
    manifest = [
        "SHA-256 manifest of docs/engineering/simulation/evidence/WO-MOK-033/",
        "",
        "One line per file: sha256, byte count, count of newline bytes, path relative to this file's",
        "directory. Digests are over the bytes as stored. The evidence tree carries `-text`, so no",
        "line-ending translation happens on checkout and these digests reproduce on any platform.",
        "",
        "This file does not cover itself, and is the only file it does not cover. A manifest that",
        "stated its own digest could not be written, and one that stated a byte total including itself",
        "could not be correct.",
        "",
        f"{len(covered)} files, {sum(row[1] for row in covered)} bytes.",
        f"{len(oversize)} further captures are stated in ../not-committed.txt and are not in this tree.",
        "",
    ]
    for relative, size, sha, lines in covered:
        manifest.append(f"{sha}  {size:<10} {lines:<8} {relative}")
    manifest.append("")
    write(f"{OUT}/manifest.sha256", "\n".join(manifest))

    print(f"{len(covered)} files committed, {sum(row[1] for row in covered)} bytes")
    print(f"{len(oversize)} captures withheld, {sum(row[1] for row in oversize)} bytes")
    crlf = [(relative, crs(f"{OUT}/{relative}")) for relative, _, _, _ in covered]
    carrying = [row for row in crlf if row[1]]
    print(f"{len(carrying)} of {len(covered)} files carry CRLF, as captured: "
          + ", ".join(f"{name} ({count})" for name, count in sorted(carrying, key=lambda row: -row[1])[:6]))
    return 0


def write(path, text):
    """Written with `\\n` only. The manifest hashes these bytes and `-text` keeps them."""
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8", newline="\n") as handle:
        handle.write(text if text.endswith("\n") else text + "\n")


# ==================================================================================================
# Written material
# ==================================================================================================


def readme(results, total_cases, total_failed, oversize, harness_lines, sweep_bytes, sweep_digest):
    counts = " ".join(f"{letter} {results[letter]['cases']}" for _, letter, _ in PHASES)
    return f"""# Evidence: `WO-MOK-033`, the sweep driver and the outcome classifier

## Summary

Two measuring instruments were built -- `scripts/run_simulation_batch.py`, which runs a sweep of
engine cells and retains one row per cell, and `scripts/classify_simulation_runs.py`, which reads
those rows and states an outcome distribution. This packet is what `VER-MOK-019` asked for: the
evidence that they measure what they claim to.

{total_cases} cases were executed across six phases, {total_failed} failed. Verification found four
defects -- three in the driver and one in its own test suite -- and all four are repaired with tests
that fail on the pre-fix code; `harness/bite/bite-output.txt` is the record that they do. Nine gaps
between the approved specification and what was buildable are disclosed rather than amended, and four
findings about the swept space are recorded rather than acted on. The eight manual assessments
`VER-MOK-019` reserves to the product, technical and assurance owners are **not recorded**: they are
the owner's to make, and `findings/manual-assessments.md` states what each needs read.

## What is bound here

| Fact | Value |
|---|---|
| Work order | `WO-MOK-033` |
| Verification contract | `VER-MOK-019` |
| Specification | `SPEC-MOK-008` |
| Base commit | `c90edc9` |
| Default sweep | 400 cells: 4 sources x 5 densities x 20 seeds, 1,000 ticks |
| Retained sweep | {sweep_bytes:,} bytes, sha256 `{sweep_digest[:16]}...` |
| Cases | {counts} -- {total_cases} total, {total_failed} failed |
| Harness | {harness_lines:,} lines under `harness/`, standard library only |
| Withheld captures | {len(oversize)} files, {sum(row[1] for row in oversize):,} bytes, stated in `not-committed.txt` |

The same 400-row output was produced four separate times during this work -- once before the phases
existed, twice in phase E at `--jobs 1` and `--jobs 4`, and once in phase F -- and all four are
byte-identical. That is `SPEC-MOK-008` rule 18.1's claim, and the `--jobs 4` production is what makes
it a measurement rather than a restatement of determinism.

## Where to start reading

1. `case-index.md` -- every case, by phase, in the order executed.
2. `findings/` -- the four defects, the nine disclosed gaps, the four sweep findings, and the eight
   assessments the owner has still to make.
3. `phase-f/scenario-1-distribution.txt` -- the answer to the question the sweep was built to ask.
4. `phase-<letter>/result.json` -- each phase's own machine-readable result, with the figure behind
   every case.
5. `assembly.txt` -- how this directory was produced, and what was left out of it.

## The distribution, in one place

The 400-cell default sweep, grouped by decision source:

| Source | extinction | asymmetric_collapse | collapse | coexistence | famine |
|---|---|---|---|---|---|
| `baseline` | 100 | 0 | 0 | 0 | 0 |
| `reference` | 29 | 7 | 14 | 50 | 0 |
| `individual` | 26 | 9 | 13 | 52 | 0 |
| `social` | 26 | 16 | 12 | 46 | 0 |

`baseline` takes no decision at all and every one of its 100 cells ends extinct. The three deciding
sources coexist in roughly half their cells. Whether that answers Phase 6's question -- *do the
conditions permit emergence*, and *are outcomes engine-determined* -- is manual assessment 1, and is
the product owner's to state, not this packet's.

## Reproducing a figure

The phase scripts are in `harness/` and take no arguments:

    python harness/phase_a_real_engine.py

Each wipes and rewrites its own output directory, so a re-run cannot leave a figure from an earlier
one standing. They expect a release build of the engine at `target/release/Mokiterions.exe` and the
repository at the path recorded in `assembly.txt`. Phase D runs the product's own cargo gates and
takes minutes; the others take seconds to a minute, except the sweep phases, which take about 30
seconds each.

Two things can be checked without running anything. Every committed file, against the manifest --
whose lines carry a byte count and a line count as well as the digest, so `sha256sum` needs the
columns it reads picked out:

    awk 'length($1)==64 {{print $1 "  " $4}}' manifest.sha256 | sha256sum -c

And the class of any retained row, against `phase-f/distribution-source-density.json`, whose per-row
entries name the ordered clause that assigned it.

## Line endings

Files here are stored as they were captured. Most are `\\n`; a few are Windows standard-output
captures and carry `\\r\\n`. `.gitattributes` marks this tree `-text`, so git stores and checks out
exactly these bytes and the digests in `manifest.sha256` reproduce on any platform. The CRLF-carrying
files and their counts are listed in `assembly.txt`.

No total byte count for this directory appears in this file. `manifest.sha256` states one, over the
files it covers, which is every file except itself. A total stated inside a file that the total covers
cannot be written down and stay true.
"""


def assembly(oversize, carrying):
    when = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    endings = "\n".join(f"  {count:<7} {name}" for name, count in
                        sorted(carrying, key=lambda row: (-row[1], row[0])))
    return f"""How this directory was produced
assembled {when}

The verification ran in a working tree outside the repository:

  work tree   C:/Users/mathi/mok-scratch/ver019
  repository  {REPO}

`SPEC-MOK-008` rule 15.1 refuses a batch whose `--out` is under `docs/engineering/`, and case T7
measures that refusal, so neither instrument could have written this directory. It was copied by
`harness/package_evidence.py`, which is itself retained here. That script performs, in order:

  1. remove and recreate docs/engineering/simulation/evidence/WO-MOK-033/
  2. copy the {len(HARNESS)} harness files to harness/
  3. walk phase-a .. phase-f and fixture-streams, copying every file of 400,000 bytes or fewer to
     the same relative path, and reducing every larger one to a line in not-committed.txt
  4. write not-committed.txt, case-index.md, findings/, README.md, this file
  5. write manifest.sha256 over everything above, excluding itself

Copies are `shutil.copyfile` on binary bytes: no line-ending translation, and the modification time
is not carried, because the bytes are the evidence and the timestamp is not. Files written by step 4
and 5 are written with `\\n` endings.

What was left out, and why

  {len(oversize)} captures over 400,000 bytes: stated in not-committed.txt with digest, bytes and
  lines. They are engine record streams and scan inputs; the largest single one is a 24 MB
  concatenation built as an input to a memory measurement. Committing 141 MB of streams into a
  governance packet is not retention, it is storage.

  The working tree's own root held four files produced before the phases existed, listed at the foot
  of not-committed.txt. One of them, an early 400-cell sweep, is byte-identical to the sweep the
  phases produce; the fact is recorded in README.md and the duplicate is not committed. The other
  three came from a state of the classifier before rule 9.6's attempted-and-effective split, so they
  are not readings at this candidate at all.

  bite2/run_simulation_batch.py and bite2/test_run_simulation_batch.py were patched copies of the
  driver and its suite, used to measure whether a regression test fails on the defect it claims to
  catch. The instrument that makes those copies is retained (harness/bite/check.py) along with its
  output; the patched copies are not, because a copy of the driver with a defect put back into it,
  sitting in a governance packet, is a second and wrong reading of a file the repository already
  holds. harness/bite/check.py deletes its own copies when it finishes.

Line endings

  `.gitattributes` marks this tree `-text`, so git stores and checks out these bytes unchanged and
  `manifest.sha256` reproduces on any platform. Files written by this script use `\\n`. The captures
  below were taken from a Windows process's standard output and carry `\\r\\n`; the count is of CRLF
  pairs. They are recorded as captured, because a capture corrected after the fact is not one.

{endings}

  No credential, and no value that could be mistaken for one, is in any byte of this packet. The
  privacy cases in phase D use a decoy whose value is self-describing -- "decoy-not-a-credential-"
  and 16 random bytes -- and retain only its length and digest, so a reader scanning this packet for
  a leaked secret finds a decoy rather than a candidate.
"""


def findings():
    return {
        "instrument-defects.md": DEFECTS,
        "disclosed-gaps.md": GAPS,
        "sweep-findings.md": SWEEP,
        "manual-assessments.md": ASSESSMENTS,
    }


DEFECTS = """# The four defects verification found, and the proof each repair bites

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
"""


GAPS = """# Gaps between the approved artifacts and what was buildable

`WO-MOK-033`'s stop-and-escalate condition 6 forbids amending an approved artifact on an
implementation agent's judgement. Each item below is therefore built one way, disclosed here, and left
for the owner to decide. None of them blocked the work. Nine are in `SPEC-MOK-008`; the tenth is in
`VER-MOK-019` itself.

## 1. Rule 12.7 and rule 14.1 disagree on an unwritable output

12.7 says a driver whose retained output cannot be written "fails under rule 15 and retains nothing".
Rule 15 is the refusal rule: status `2`, *nothing executed and nothing was written*. But an unwritable
output is discovered after the cells have run, and rule 14.1's table gives that case status `1`.

Built as 14.1 says: status `1`. Case B25 measures the unwritable output and B23 measures all four
statuses of the table. 12.7's *retains nothing* is honoured either way; only the status is in question.

## 2. Rule 7.2 constrains the stream's location and rule 2 gives no option to set it

7.2 requires the stream path to be outside `docs/`, outside `scripts/`, and not the operator's output
path. Rule 2's option list has no way to say where it should be instead.

Built with a `--stream-dir <path>` option, defaulting to a fresh temporary directory. This is an
option the specification does not list, which is the disclosure.

## 3. Rule 7.1's "one reusable path" and rule 2.8's `--jobs` cannot both be literal

One path and concurrent cells are incompatible. Built as one reusable path *per worker*, taken from a
pool. See `instrument-defects.md` defect 3 -- reading 7.1 as literally one path is what the first
implementation did, and it lost cells.

## 4. Rule 12.3's `refused` stage is unreachable per cell

The vocabulary gives a missing cell the stage `refused`, "rejected before execution". Every refusal
the specification defines -- rule 4.2's `llm` source, rule 15.1's usage errors -- is refused for the
whole sweep before any cell executes, and exits `2` with nothing written. There is no path on which
one cell is refused and the others run.

Built with the stage present in the vocabulary and never emitted. Five of the six stages are observed
in this packet -- `run`, `read` and `derive` in case B20, `crosscheck` in B17, `not_attempted` in phase
E's interruption -- and `refused` is the one that is not, because there is no way to reach it.

## 5. `crosschecks_passed` is a constant on any retained row

Rule 10 gives seven equalities and rule 10.3 makes a disagreeing cell a missing cell with stage
`crosscheck`. A row therefore exists only if all seven passed, so `crosschecks_passed` is `7` on every
retained row that has ever existed and can never be anything else.

Built as specified. The field is not useless -- it states which contract version's checks were run --
but a reader who treats it as a measurement is reading a constant.

## 6. Rule 14 states exit statuses for the driver only

14.1's table is the batch driver's. Rule 16's classifier has no status table anywhere in the
specification.

Built with two statuses: `0` for a report produced, and `2` for any input it cannot use -- a missing
file, an unreadable one, one carrying no `sweep` record, an unknown option. That follows rule 14.3's
reasoning, that an operator should see one convention across both instruments, and the code says so at
the `return`. There is no `1`, because the classifier has no work of its own to fail at, and no `3`,
which has no classifier meaning. Case Q17 measures `2` over a file with no `sweep` record; Q16
measures that a report over an incomplete sweep is still a `0` and carries the incompleteness.

## 7. Q4's missing-fact rule is absent from rule 16

`VER-MOK-019` case Q4 requires the classifier to state a fact it cannot compute as missing rather than
as zero. Rule 16 has no such rule, and rule 17.1's list of what a group states has no place to put
one.

Built to satisfy Q4: a row whose stream lacked a field is reported with the field absent, not zeroed.
Rule 16 does not require this and does not forbid it.

## 8. Rule 17.1 omits `attempted`

Rule 9.6 records the measurement that justifies two threat counters, attempted and effective. Rule
17.1's list of per-group figures names "each of the three `effective` counters" and does not mention
`attempted` at all.

Built with both, and case Q10 measures that `attempted` stays distinct from `effective` in the group
figures. This is a superset of 17.1, which is why it is a disclosure and not a conflict --
but a reader comparing the output to 17.1 will find a block that rule does not describe.

## 9. Rule 16.8's famine predicate was still unreached at 400 runs

16.8 states the `famine` predicate and discloses that measurement does not support it, on 35 runs. The
400-cell sweep does not reach it either: `famine` is 0 over 400 runs, against 12,827 recorded capacity
skips. At density `0.10` all 80 cells end extinct with 16 units of food standing across both
territories -- agents starve beside food they never reach, which is a spatial failure and not a
famine.

Built as specified, with the class reported as zero rather than omitted, per rule 16.9. Whether to keep
an unreached predicate at all is manual assessment 3.

## 10. `VER-MOK-019` case T6 cites a rule that does not record what it asks for

T6 requires the instruments not be build targets, and cites `SPEC-MOK-002` rule 5 for a census of the
workspace's targets. That rule does not record a target census. The case was executed against the
three `Cargo.toml` files directly -- 25 targets, none of them a Python file -- which is the fact T6
wants; the citation is what is wrong.
"""


SWEEP = """# What the sweep found, recorded and not acted on

## 1. `asymmetric_collapse` fires at 20 seeds where 35 runs saw none

`ADR-MOK-008` predicted the class would be rare. Over 400 cells it accounts for 32 -- 7 under
`reference`, 9 under `individual`, 16 under `social`, and none under `baseline`, which cannot reach it
because it has no survivors anywhere. The class is real and the vocabulary needs it. Manual assessment
2 is whether the five classes are the right five.

## 2. The seed changes the class in half the groups

`WO-MOK-033`'s scenario 2 requires this be recorded either way. Over the 20 source-and-density groups
of 20 seeds each:

- The seed moves the run figures in **20 of 20** groups.
- The seed moves the *class* in **10 of 20** groups: `individual` at densities 0.25, 0.50 and 1.00;
  `reference` at 0.25, 0.50 and 0.75; `social` at 0.25, 0.50, 0.75 and 1.00.

This reverses the finding recorded on 2026-08-30 from five seeds, which was that the seed moved
figures but not classes. Five seeds was too few to see it. The consequence for anyone reading a
distribution: a single-seed run states nothing about its cell, and a 20-seed group states a
distribution rather than an outcome.

Density moves the class distribution for all three deciding sources -- 5 of 5 distinct class vectors
each -- and for `baseline` it cannot, because all 100 of its cells are `extinction` at every density.
`baseline`'s own figures do still move across all five densities: 5 of 5 distinct vectors of deaths,
crossings, consumed and ticks. Its distribution has nowhere to move, which is a floor rather than a
null result.

## 3. The threat mechanism is inert, measured over the full sweep

Threats resolve 7,701 times across the 400 cells and are effective 8 times -- 0.1039%. All 8 are under
`social`; `reference` and `individual` produce none. Per density: 2828/1, 1861/1, 1793/2, 577/2,
642/2.

This is the *before* figure for the repair chain the owner deferred on 2026-08-30, when the decision
was that Phase 4b discloses the finding and a later chain repairs it. Whether the disclosure as built
is sufficient to serve as that figure is manual assessment 8. The earlier reading over 35 runs was 1
effective of 1,448; this one is over 400 runs and agrees with it.

## 4. `retreat` is a targeted action with no resolved-event kind

Case T8 measures it: `SPEC-MOK-001`'s resolved-event vocabulary is `attack_resolved`,
`encounter_resolved`, `surrender_resolved`, `threat_resolved`, and declares no `retreat_resolved`. Over
eight real streams the engine emitted 14 event kinds, none of them naming retreat. No retained byte of
25 outputs contains a field or value named for retreat, which is what T8 asserts -- the instrument does
not synthesise one.

`docs/ROADMAP.md`'s Phase 6 wording treats retreat as an available action. It is available as an
intention and unobservable as an outcome, so no sweep can report on it. This belongs to the roadmap
reconciliation, which is outside every work order.
"""


ASSESSMENTS = """# The eight manual assessments: not recorded

`VER-MOK-019` reserves eight assessments to the product owner, the technical owner and the assurance
owner, and states plainly: *an unsigned assessment is not a recorded one*. None is recorded here. An
implementation agent cannot sign any of them, and the three owner roles are all held by one person, so
none can be substituted by another role either.

The mechanical cases -- all 84 of them -- are executed and passing. These eight are the judgements
those cases cannot make. `VREC-MOK-027` discloses this gap rather than closing it.

| # | Assessment | Role | Read |
|---|---|---|---|
| 1 | Does the distribution answer Phase 6's question? | product owner | `phase-f/scenario-1-distribution.txt`, `phase-f/distribution-source.md`, README's distribution table |
| 2 | Is the class vocabulary the right one? | product owner | `phase-f/distribution-source-density.json` rows, not `SPEC-MOK-008` rule 16.5's text |
| 3 | Is the famine disclosure honest and sufficient? | technical owner | `phase-f/famine.txt`, `findings/disclosed-gaps.md` item 9 |
| 4 | Is the attempted-and-effective split justified by its measurement? | technical owner | `phase-f/threat-inertness.txt`, `findings/sweep-findings.md` item 3 |
| 5 | Is `ADR-MOK-008`'s "no amendment required" claim sound? | assurance owner | `phase-d/t1-census.txt`, `t2-touched.txt`, `t6-target-census.txt`, against `ARCH-MOK-001`, `SPEC-MOK-004` rule 1 and `ADR-MOK-003` decision 1 |
| 6 | Is Python becoming load-bearing for published results accepted? | technical owner | `harness/` as built, `phase-d/t4-declared-dependencies.txt`, `t5-imports.txt` |
| 7 | Are the reported measurements reproducible? | assurance owner | recompute one digest from `manifest.sha256` and one derived count from `phase-f/default-sweep.jsonl`, independently of any retained command output |
| 8 | Does the threat disclosure meet the 2026-08-30 decision? | assurance owner | `findings/sweep-findings.md` item 3, `phase-f/threat-inertness.txt` |

Assessment 5 is worth a note. `ADR-MOK-008` claims no amendment is required to `ARCH-MOK-001`'s
prohibited-patterns list, `SPEC-MOK-004` rule 1 or `ADR-MOK-003` decision 1. Cases T1 to T6 measure
what can be measured: the workspace has two members and no third package, the structural census is 40
paths blob-for-blob identical to `c90edc9` with a clean worktree over all of them, no product file is
touched, both packages' dependency tables are byte-identical and their declared sets unchanged, every
import of all four Python files resolves to the standard library, and neither instrument is a build
target. Whether that is the same thing as *none of the three is touched* is the judgement, and it is
the one the mechanical cases cannot make.
"""


if __name__ == "__main__":
    raise SystemExit(main())
