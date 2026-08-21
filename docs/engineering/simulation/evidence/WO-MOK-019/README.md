# Evidence: `WO-MOK-019`, the optional structured record stream

This packet is the evidence `VER-MOK-012` requires for `WO-MOK-019`. It is written as the work proceeds
rather than assembled afterwards, so the reading order below is also the order the artifacts were produced.

Fifty-five files, 10,674,199 bytes. The candidate tree is the working tree at
`bb4a21491eff321cbfd14ba3ea794e34535e3033` plus the implementation this work order adds; the pre-change
baseline is `de33d7440c323a98ac88db3fabaf87bea48ebf4e`, recorded in `baseline/COMMIT.txt`.

> **Later fact, 2026-08-20.** This directory now holds **fifty-six** files. The fifty-fifth figure above
> is the evidence set, and it is still fifty-five: `assurance-decision.md` is the addition and it is
> deliberately **not** part of it, because it postdates the commit this packet binds and a record's
> evidence set is the capture's rather than the decision's. `VREC-MOK-012`'s `evidence_paths` names the
> same fifty-five it named when it was captured.

## What discharges what

Each row names the artifact and the oracle, scenario or gate it answers. Every `.txt` file carries its own
command line, its provenance and its own `RESULT:` line in its header, and every generating script is
retained in the packet — either as a file under `analysis/` or as full text appended to the artifact it
produced, which is the form the earlier packets in this repository established.

### Oracle 1 — the text stream is unmoved, with a sink and without

| Path | What it is |
|---|---|
| `capture.sh` | The capture script for the whole matrix. Takes `<target-dir> [sink]`; in `sink` mode it additionally passes `--events-path`. |
| `analysis/digest.py` | Per-cell SHA-256, byte count and line count of standard output, standard error, exit code and — in sink mode — the record stream. |
| `analysis/retain.py` | Copies the stated subset of whole text streams into the packet. |
| `analysis/compare.py` | Compares two manifests cell by cell and states where they differ. |
| `baseline/COMMIT.txt` | The commit the pre-change capture was taken at. |
| `baseline/capture-state.txt` | The working tree's state at that commit, so that "before any code change" is checkable rather than asserted. |
| `baseline/pre-manifest.txt` | The digest manifest of all 90 pre-change cells. |
| `baseline/full/` | Three whole pre-change standard-output streams. |
| `post-nosink-manifest.txt` | The same 90 cells at the candidate tree, no sink configured. |
| `post-sink-manifest.txt` | The same 90 cells at the candidate tree with a sink, including each record stream's digest. |
| `oracle1/pre-vs-post-nosink.txt` | Comparison A: pre-change against post-change with no sink. |
| `oracle1/post-nosink-vs-post-sink.txt` | Comparison B: post-change with no sink against post-change with a sink. |
| `oracle1/pre-vs-post-sink.txt` | Comparison C: pre-change against post-change with a sink. |

### Oracle 2 — the text stream is reconstructible from the record stream

| Path | What it is |
|---|---|
| `analysis/reconstruct.py` | The reconstructor. Rebuilds standard output from the records alone, with no event-specific branch. |
| `oracle2/reconstruction-result.txt` | The byte comparison of reconstruction against standard output for every captured cell, and the mechanical check that the reconstructor has no per-event-kind branch. |

### Oracle 3 — every record is JSON, to a parser outside this repository

| Path | What it is |
|---|---|
| `analysis/validate.py` | Parses every record of every retained capture with Python's `json` module. |
| `json-validity.txt` | The result per capture, with the exact command. |

### Oracle 4 — a sink moves no entropy draw

| Path | What it is |
|---|---|
| `analysis/entropy.py` | Runs the three entropy assertions and retains what they printed. |
| `entropy.txt` | The oracle's own result file: the state after initialization, at tick 1,000, and against the pre-change build. |
| `entropy-states.txt` | Additivity, twelve combinations per row, and the state at the thousandth tick. |
| `entropy-per-tick.txt` | The state at **every** tick boundary, sink against no sink, five seeds × three policies × tracing off and on. |

### Oracle 5 — the value alphabet is closed, so no escaping function is needed

| Path | What it is |
|---|---|
| `alphabet.txt` | Every member of every one of the thirteen closed domains, its emitted bytes in hexadecimal, and each domain's size asserted against the specification's. |

### Oracle 6 — the metrics and run records reconcile against a replay of the events

| Path | What it is |
|---|---|
| `analysis/replay.py` | The replay consumer, written independently of the engine. |
| `oracle6/reconciliation.txt` | The reconciliation per tick per seed. |

### Oracle 7 — the amendments this change depends on are approved

| Path | What it is |
|---|---|
| `analysis/amendments.py` | Reads the artifacts and the git history and measures seven things about the chain's governance state. |
| `amendment-approvals.md` | The chain's status at the base and now, all twenty-eight required provisions found twice each, the provision counts, the two places the artifacts disagree, the amendment made beyond the approved list, and the earlier layer's outstanding rows named rather than counted. |

### The oracles shown to fail, and the boundaries

| Path | What it is |
|---|---|
| `negative-controls.txt` | The deliberate perturbations of acceptance scenarios 4, 5 and 6, each applied, captured failing, and reverted. |
| `analysis/capture-failures.py` | Drives the six process-boundary captures. |
| `failure-captures.txt` | The five failure captures — sink not creatable, write failure mid-run, flush failure, run-record write failure, reserved-spelling rejection — each with its standard error, exit code and the destination's state afterwards, plus the overwrite capture. |
| `analysis/prior-captures.py` | Re-runs every configuration retained under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011`. |
| `additivity.txt` | Scenario 12: that re-run, byte-compared against each retained capture. |
| `analysis/retain-sink.py` | Runs each retained cell twice to two deliberately different destinations and requires byte-identical records. |
| `retained-sink-streams.txt` | The retained subset, the path-independence result, the closed character set of the retained bytes, and the retention deviation. |
| `post/full/` | Four whole post-change record streams. |

### Sizes, static checks, the interface and the gates

| Path | What it is |
|---|---|
| `measure-sizes.sh` | Measures stream sizes for the 1,000-tick and 10,000-tick traced runs. |
| `sizes.txt` | Those sizes, over thirty combinations. |
| `analysis/static-checks.py` | The twelve static and architecture checks. |
| `static-checks.txt` | Their results. |
| `interface.txt` | The public interface of both packages before and after, item for item, by `WO-MOK-011`'s enumerator reused unmodified. |
| `gates.txt` | `cargo fmt`, `cargo clippy`, `cargo test` and `cargo tree -p Mokiterions` at the candidate tree. |
| `baseline/test-run.txt`, `baseline/test-census.txt` | The workspace test run at the base commit, and its 212 test names. |
| `post/test-run.txt`, `post/test-census.txt` | The same at the candidate tree, and its 246 test names. |
| `analysis/census-by-target.py`, `analysis/census-by-target.txt` | The census split by the binary that ran each test. |
| `analysis/census-reconciliation.txt` | The two censuses reconciled name by name. |

### What is not measured, and is recorded as unmeasured

| Path | What it is |
|---|---|
| `manual-assessment.md` | All eight manual assessments, prepared with the material each needs, and — **as of 2026-08-20, after the candidate commit this packet binds** — all eight **RECORDED** by the owner. Each decision states what accepting it settles and what it accepts by naming nothing in the alternative. At `50364a3` itself every one reads OUTSTANDING. |
| `assurance-decision.md` | **Not evidence and not among the fifty-five.** The record of the assurance owner's decision of 2026-08-20 to move `VREC-MOK-012` from `ready` to `verified`: the form the instruction took, what the decision accepted, what it does not retire, and the harness measured either side of it. It postdates the commit this packet binds. |
| `completion-summary.md` | `WO-MOK-019`'s *Completion report format*, all sixteen items in its order: what changed and what deliberately did not, one full record stream quoted, each oracle's result, the negative controls, the three amendment rows quoted, the eight outstanding assessments, and **nine defects measured in the approved artifacts**, none of them corrected here. |

## `VER-MOK-012`'s *Evidence retention* list, bullet by bullet

The list is the checklist this packet is answerable to, so it is mapped here in its own order rather than
paraphrased. A bullet whose row says anything other than a filename is a bullet this packet does not
discharge as written, and the row says so.

| # | The declaration | Where it is |
|---|---|---|
| 1 | the pre-change baseline capture, before any code change, with the commit recorded | `baseline/pre-manifest.txt`, `baseline/COMMIT.txt`, `baseline/capture-state.txt`; three cells whole in `baseline/full/` |
| 2 | the post-change capture with no sink, and the byte comparison for every combination | `post-nosink-manifest.txt`, `oracle1/pre-vs-post-nosink.txt` |
| 3 | the post-change capture with a sink, **its standard output**, and the byte comparison against the sinkless capture | `post-sink-manifest.txt`, `oracle1/post-nosink-vs-post-sink.txt`, `oracle1/pre-vs-post-sink.txt` — the standard output as digests only; see the deviation below |
| 4 | one full sink stream per declared seed at the default density under each policy, tracing off and on | `post/full/` — **four** of the thirty declared; `retained-sink-streams.txt` states the deviation and what stands in for the rest |
| 5 | the full text of the reconstructor, and the byte comparison of reconstruction for every combination | `analysis/reconstruct.py`, `oracle2/reconstruction-result.txt` |
| 6 | the full text of the replay consumer, and its reconciliation per tick per seed | `analysis/replay.py`, `oracle6/reconciliation.txt` |
| 7 | the JSON-parser check, including the exact command, for every retained capture | `json-validity.txt`, command on line 5; `analysis/validate.py` |
| 8 | the entropy-state comparison per tick, per seed, per policy, with and without a sink; and after initialization and at tick 1,000 against the pre-change build | `entropy-per-tick.txt`, `entropy-states.txt`, `entropy.txt` |
| 9 | the value-alphabet enumeration, each domain's members and size, and the emitted bytes | `alphabet.txt` |
| 10 | the deliberate-perturbation results of scenarios 4, 5 and 6 | `negative-controls.txt` |
| 11 | the re-run of every configuration retained under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011`, byte-compared | `additivity.txt`, `analysis/prior-captures.py` |
| 12 | the five failure captures, each with standard error, exit code and the destination's state afterwards | `failure-captures.txt`, captures 1–5 |
| 13 | the overwrite capture, showing a prior run's file replaced | `failure-captures.txt`, capture 6, line 332 |
| 14 | stream sizes for a 1,000-tick run and a 10,000-tick traced run | `sizes.txt`, `measure-sizes.sh` |
| 15 | the six static-check results | `static-checks.txt` items 1–6 are the six declared, in the list's own order; items 7–12 are additional. The rule 5 item-for-item comparison is also `interface.txt` |
| 16 | the workspace test census before and after, reconciled name by name | `baseline/test-census.txt`, `post/test-census.txt`, `analysis/census-reconciliation.txt`, `analysis/census-by-target.txt` |
| 17 | `cargo fmt`, `cargo clippy`, `cargo test` and `cargo tree -p Mokiterions` output | `gates.txt` |
| 18 | the eight manual assessments, each with its accountable role and date | `manual-assessment.md` — each carries its role, the date its material was measured, and **its decision, recorded 2026-08-20 by the owner**. At the candidate commit `50364a3` the decision line of all eight was blank; the decisions postdate it, because a decision cannot be inside the commit it decides about |
| 19 | the amendment-approval check of oracle 7, and the recorded state of `ARCH-MOK-001`'s outstanding 2026-08-18 row | `amendment-approvals.md`, sections 1–6; the `ARCH-MOK-001` row is named in section 5 |

## The capture matrix

Ninety cells: seeds `0 1 42 123 777`, policies `baseline reference individual`, densities `0.15 0.75 1.50`,
tracing off and on, `--ticks 1000` throughout.

`VER-MOK-012` declares sixty cells — the two densities `0.75` and `1.50` — and this capture is a **superset**,
adding `0.15`. That is the matrix `WO-MOK-011`'s capture used, and reusing it is what lets the re-run required
by *Evidence retention* compare against `WO-MOK-002`'s and `WO-MOK-011`'s retained captures cell for cell. A
superset satisfies the declared oracles and adds thirty more chances to fail them.

## Retention: what is kept whole, what is kept as a digest, and why

**This packet does not retain the captures whole, and `VER-MOK-012`'s *Evidence retention* list, read
literally, asks it to.** The deviation is stated here rather than left for a reader to infer from what is
absent.

The pre-change capture alone is 110 MB of standard output across its 90 cells. The list requires three
captures of that matrix — pre-change, post-change with no sink, post-change with a sink — plus, separately,
"one full sink stream per declared seed at the default density under each policy, with tracing off and on",
which is thirty more streams. Retaining all of that whole would put roughly a third of a gigabyte of
generated text into the repository, most of it three byte-identical copies of the same thing, since the whole
point of oracles 1 and 4 is that the three captures agree.

What is retained instead, following the form `WO-MOK-006`'s, `WO-MOK-007`'s and `WO-MOK-011`'s evidence
packets established for this oracle:

- **A digest manifest of every cell of every capture.** SHA-256, byte count and line count for each cell's
  standard output, standard error and exit code, and for its record stream where it has one. The comparison
  a reviewer must be able to make — "these two captures are byte-identical, and this third one differs
  exactly here" — is made on the manifests, and a digest comparison is stronger than an eyeball comparison of
  two 1.2 MB files, not weaker.
- **Three whole pre-change text streams**, at seed 42 and the default density, one per decision source,
  tracing off: `seed42-baseline-d0.75-traceoff`, `seed42-reference-d0.75-traceoff`,
  `seed42-individual-d0.75-traceoff`. These are the same three cells `WO-MOK-011` retained whole, so the two
  packets compare directly, and a reviewer who wants to read a complete 1,000-tick stream rather than trust a
  hash has three of them, with their record streams beside them in `post/full/`.
- **Four whole post-change record streams**, in `post/full/`: those same three cells' records, plus the
  traced baseline cell, which at 743 KB is the smallest traced cell in the matrix. `retained-sink-streams.txt`
  gives the reason for each and measures the gap: thirty declared, four retained, 86 cells covered by digest
  alone.

**No post-change *text* stream is retained whole.** Bullet 3 of the retention list asks for the sink
capture's standard output and this packet holds its digests only, in `post-nosink-manifest.txt` and
`post-sink-manifest.txt`. The reason it is not a gap in the evidence is that oracle 1 establishes the
post-change text stream is byte-identical to the pre-change one in all 90 cells, with a sink and without, so
the three whole streams in `baseline/full/` are the post-change streams too — that identity is the claim
oracle 1 exists to test, and `oracle1/pre-vs-post-sink.txt` is where it is tested. A reader who does not
accept oracle 1's result should not accept the substitution either, which is why it is named here rather than
relied on silently.

Everything not retained is reproducible: `capture.sh` at the commit in `baseline/COMMIT.txt` reproduces the
pre-change capture, and at the candidate commit the two post-change captures. The manifests are what detect a
reproduction that failed.

**What this costs.** A reviewer cannot inspect an arbitrary cell's text without re-running the capture. That
is the cost, it is real, and it is accepted here because the alternative buys nothing a digest does not
already establish. Digests are taken over the bytes exactly as written — nothing is decoded, normalized or
newline-translated, because `SPEC-MOK-006` rule 11.1 admits no whitespace exemption and a normalizing
comparison would verify a weaker property than the one required.

## No secret is retained

No capture command carries a credential, and no retained artifact contains one. **No retained record stream
carries the path it was written to**, which `VER-MOK-012` names as the property that makes this evidence class
safe to retain at all: `SPEC-MOK-006` rule 5.5 keeps the sink's path out of the header record in every form,
and rule 3.2's closed value alphabet means no operator-supplied text can reach any field.
`retained-sink-streams.txt` does not take that on trust. Each retained cell was run twice to two deliberately
different destinations and the record bytes required to be identical, the digests then compared against the
manifest taken at a **third** destination, and the complete set of distinct characters in all 6,444,508
retained bytes enumerated: 64 characters, and neither `/` nor `\` among them. A path cannot be spelled
without one of those two, so the enumeration is a statement about every destination rather than about the
ones used here.

Capture directories are under the system temporary directory and their paths appear in the artifact headers,
which is provenance rather than a secret. The binary's absolute path appears for the same reason.

## What this packet does not establish

That the change is verified. Seven oracles are measured here. The eight manual assessments were **not**
measured — they are judgements, prepared in `manual-assessment.md`, and the owner **recorded all eight on
2026-08-20**, after the candidate commit this packet binds. That discharges the one condition
`VER-MOK-012` states explicitly for being unsatisfied, and it discharges nothing else.

What still stood, unchanged by that validation: two deviations from the retention list, disclosed above;
the amendment beyond `ADR-MOK-005`'s approved list, disclosed in `amendment-approvals.md` section 4 and
recorded there as **not approved**; and nine defects measured in the approved artifacts, in
`completion-summary.md` item 16 — one of which, `SPEC-MOK-006` rule 3.2's direction domain, weakens oracle
5's size assertion for one domain of thirteen, and another of which is the miscited rule in the wording of
manual assessment 4 itself. `amendment-approvals.md` is generated rather than hand-edited, so its closing
paragraph still reads that the eight assessments are OUTSTANDING; that is the state its own generation
measured, and it is left rather than edited.

**Three of those then moved, later the same day, by three further acts.** The `SPEC-MOK-004` rule 11 row
was **approved** by the owner as technical owner. **`VREC-MOK-012` moved from `ready` to `verified`** by
the same owner as assurance owner; `assurance-decision.md` records that decision and what it accepted, and
what it accepted includes the two retention deviations as they stand and the nine defects uncorrected. The
nine were **deferred to a correction work order in Phase 4b** — a disposition, not a repair: that work
order does not exist yet, so six approved artifacts keep a known wrong statement until it lands.

**A fourth act followed, by a fourth role.** `WO-MOK-019` moved from `in_progress` to `implemented`, the
owner acting as engineering owner, in the commit after the transition. It changes nothing this packet
measured — the packet binds `50364a3`, and the status of a work order is not an input to any oracle here —
but it does change what the inspector reports about the packet: an `implemented` work order whose evidence
is not discoverable by filename raises `W-HEX-001`, and this directory's fifty-six files are named for
what they measure rather than for the work order, so the observation now names `WO-MOK-019` alongside
`WO-MOK-010` and `WO-MOK-011`. That is a naming convention meeting a discovery heuristic, disclosed here
rather than answered by renaming fifty-six files.

**What still stands after all of it**: the two retention deviations, so `VER-MOK-012`'s *Evidence
retention* list is answered in substance and not satisfied on bullets 3 and 4 as written; the nine
defects, uncorrected; and the three carried-forward `OUTSTANDING` amendment rows — `ARCH-MOK-001`
2026-08-18, `SPEC-MOK-002` 2026-08-18, `SPEC-MOK-004` 2026-08-19 — none of which is this chain's to pay.

**This packet still does not establish that the change is verified.** `VREC-MOK-012` does, for
`50364a3` and no further commit. `50364a3` is a branch commit and is not an ancestor of `master`: on a
merged tree the gates, the census, the interface enumeration and oracles 1 through 6 need re-running
rather than carrying over, and a record bound to the merge is a new record rather than an edit of that
one. Verification is not merge and not release.
