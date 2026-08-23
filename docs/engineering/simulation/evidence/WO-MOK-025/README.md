# Evidence: `WO-MOK-025`, the `llm` decision source and its connector port

This directory retains implementation evidence for `WO-MOK-025`, the first of Phase 5's three work orders.
`VER-MOK-018` is the verification contract it serves.

**This packet is incomplete and is committed incomplete on purpose.** What is here is the base-commit
capture, and one governance act that came out of taking it: the state of the engine's output *before* any
change of this work order, plus the owner's ratification of the `SPEC-MOK-006` row that the capture proved
was still outstanding at a third commit. Neither is an implementation of this work order's scope. The
work order's *Constraints* require it first and say why — "**The base-commit captures are taken first.** A
capture taken after the change is not a base-commit capture, and `REQ-MOK-068` becomes uncheckable without
one. This is the one ordering constraint that cannot be repaired later." A capture is the one kind of
evidence that cannot be reconstructed once the tree has moved, so it is taken and committed before the
change rather than kept in a working tree while the implementation proceeds around it.

Nothing here approves verification, creates a candidate commit, or authorizes release. No live LLM run was
made, none is authorized, and no credential exists anywhere in this directory or in the tree that produced
it: the owner's standing instruction is that "an explicit permission from the repository owner is needed to
launch a real run", and no such permission has been given. Every figure below comes from the four existing
deterministic decision sources.

## Commit binding

| Fact | Value |
| --- | --- |
| Base commit | `cc5418553cb433715b7d6b15dea3886bff30ffaa`, `gov(WO-MOK-025): transition from approved to in_progress`, 2026-08-23 |
| Implementation branch | `feature/phase-5-definition` |
| Candidate commit | not yet taken. The verification record will bind it and its `commit` field will be the authority; this table does not anticipate it |
| Captured on | 2026-08-23, from a `git worktree` at the base commit, built and run there |
| Toolchain | rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable, Python 3.14.6, se-harness 0.4.0 |

The base commit is fixed by `WO-MOK-025`'s *Lifecycle*, in the `in_progress` transition subsection, together
with this directory's path. Both were fixed there because neither can be corrected afterwards.

## The capture matrix

Five declared seeds — `0`, `1`, `42`, `123`, `777` — by four decision sources — `baseline`, `reference`,
`individual`, `social` — by tracing off and on. Forty cells, taken twice: once with no record sink and once
with one, for eighty runs. Every run is `--ticks 1000 --density 0.75`.

`--ticks 1000` is a local decision of this work order under its *Authorized decision envelope*, not a
figure any artifact fixes. `capture.sh`'s header states it and the reason: a thousand ticks is the horizon
`WO-MOK-019` used for the same comparison, so the two packets compare directly, and it is long enough that
every source has exhausted the interesting part of its behavior.

All eighty runs exited `0` and wrote nothing to standard error. Standard output is byte-identical between
the sink and no-sink modes in all forty cells, which is the property `REQ-MOK-046` fixed and this capture
re-measures rather than assumes.

## Contents

- **`capture.sh`** — reproduces the eighty runs. Takes a target directory and a mode; writes `.txt`,
  `.err`, `.exit` and, in sink mode, `.jsonl` per cell.
- **`manifest.sh`** — reduces a capture directory to one line per cell: the cell name, sha256 and byte and
  line counts of standard output, sha256 of standard error, the exit code, and the same three figures for
  the record stream where there is one.
- **`entropy-manifest.sh`** — reduces the per-boundary entropy capture to one line per configuration.
- **`base/nosink-manifest.txt`**, **`base/sink-manifest.txt`** — those manifests for the two modes.
  **`base/sink-manifest.txt`'s record-stream digests were superseded the same day** by the owner's
  ratification of `SPEC-MOK-006`'s 2026-08-21 amendment row, which moved `schema` from `1` to `2` and with it
  all forty of them. It is left uncorrected because it is a base-commit capture and the base commit emitted
  `1`; `ratification/sink-manifest.txt` is the schema-2 baseline the later record-stream comparison uses.
  The text-stream digests beside them are untouched, and `base/nosink-manifest.txt` stands as taken.
- **`base/entropy-manifest.txt`** — the twenty configurations' tick-boundary entropy states, as a boundary
  count, a digest of that configuration's own lines, and the final state.
- **`base/entropy-instrument.patch`** — the instrument that produces the entropy capture, as a patch
  against the base commit, so the base figures are re-derivable. See *The entropy capture* below.
- **`base/full/`** — five whole streams, kept whole so a reviewer need not re-run the capture to read one.
- **`base/gates.txt`** — formatting, lint, tests and the two harness readings at this commit, in the forms
  `preflight` prescribes, with what they do not establish stated.
- **`base/reproduction.txt`** — the retained patch applied to a pristine worktree at the base commit, the
  L9 check run there, and the entropy manifest regenerated and found byte-identical to the committed one.
- **`base/wo-019-comparison.txt`** — a free cross-check against `WO-MOK-019`'s retained capture.
- **`base/schema-divergence.txt`** — the record stream's `schema` value at the base commit, measured. This
  was the basis of the escalation `WO-MOK-025` stop-and-escalate condition 5 requires. **The owner settled it
  on 2026-08-23**, and `ratification/` holds that act and its measurement; the file is left as written,
  before the answer, because it is the escalation's evidence and not a summary of the outcome.
- **`ratification/`** — the owner's ratification of `SPEC-MOK-006`'s 2026-08-21 amendment row, the two-line
  product change it obliged, and the measurement that the increment changes one integer in every header
  record and nothing else. See that directory's own `README.md`.

## Retention: what is kept whole, what is kept as a digest, and why

**The captures are not retained whole.** The eighty runs produce 246 MB of standard output and record
streams — 63 MB with no sink, 183 MB with one. Committing that would put a quarter of a gigabyte of
generated text into the repository to establish a claim of the form "these two captures are identical, and
this third one differs exactly here", which a digest establishes more strongly than an eyeball comparison
of two 1.2 MB files.

This follows the form `WO-MOK-006`, `WO-MOK-007`, `WO-MOK-011` and `WO-MOK-019` established. What is kept:

- **A digest manifest of every cell of both modes.** 43 lines each, 18 KB together, covering all eighty
  runs.
- **Four whole text streams**, at seed 42 with tracing off, one per decision source. `WO-MOK-019` retained
  three cells at the same seed and density, so the packets compare directly; the fourth is `social`, which
  did not exist as a retained cell there.
- **One whole record stream**, `seed42-baseline-traceon.jsonl`. It is the counterpart of the one
  `WO-MOK-019` retained for the same cell, and the pair is what makes `wo-019-comparison.txt` section 3 a
  measurement rather than a claim.

**The untraced baseline record stream is byte-identical to `WO-MOK-019`'s and is deliberately not copied
here.** It already exists in the repository at
`docs/engineering/simulation/evidence/WO-MOK-019/post/full/seed42-baseline-d0.75-traceoff.jsonl`, and a
second copy would only be free to drift from the first. The identity is measured in
`base/wo-019-comparison.txt` section 3.

**What this costs.** A reviewer cannot inspect an arbitrary cell's output without re-running the capture.
That is real, and it is accepted because a digest already establishes what reading the cell would.
Everything not retained is reproducible with `capture.sh` at the base commit, and the manifests are what
detect a reproduction that failed. Digests are taken over the bytes exactly as written — nothing is
decoded, normalized or newline-translated.

## The entropy capture

`REQ-MOK-068` requires that adding the `llm` source perturbs none of the four existing ones. Equal output
is the visible half of that; equal *entropy consumption* is the half output cannot show, because a source
that drew a number and discarded it would leave the same text behind and a different world one tick later.

`SplitMix64` advances its state by a fixed odd constant per draw, so the state **is** a draw counter: equal
states after equal tick counts means equal draw counts. `WO-MOK-019`'s `entropy-per-tick.txt` established
this reading. `base/entropy-manifest.txt` captures the state at every tick boundary of all twenty
configurations, so a later capture that differs names the configuration, and re-running the instrument
names the boundary.

The boundary counts differ by source and that is expected, not a defect: `baseline` stops when its
population dies out, at 121, 121, 144, 170 and 136 boundaries for the five seeds, while the other three
reach the full 1002.

**The instrument had to be added to the base tree to take this capture, and that is disclosed rather than
hidden.** `Simulation::entropy_state` is `#[cfg(test)]` and module-private, so the states cannot be read
from the CLI; the instrument must be an in-crate `#[test]`. Its 213 lines are a pure addition —
`git diff --stat` against the base commit reports `213 insertions(+), 0 deletions(-)`, with no existing
line touched, which is what makes it behaviourally neutral by inspection rather than by assertion. The
patch is retained so a reviewer can apply it at `cc54185` and reproduce the figures — and that round trip
was made rather than assumed: `base/reproduction.txt` records the patch applying to a pristine worktree at
the base commit and the manifest regenerating **byte-identical** to the committed one.

The instrument ships with its automated counterpart,
`the_four_existing_sources_draw_what_the_base_commit_drew`, which holds the twenty measured figures as a
`const` and asserts them on every `cargo test` run. That test passes at the base commit with the patch
applied and in the current tree. It is `VER-MOK-018` case L9's in-crate half. It folds the states rather
than hashing them — the engine package declares no dependencies, so no digest function is available
in-crate — and the fold rotates before combining so that it is sensitive to order.

## No secret is retained

No API key, token, endpoint credential or provider account identifier appears in any file here. Nothing in
this capture contacted a network. This is the property `ADR-MOK-001` and `SPEC-MOK-007` rules 10.5 and 13.4
fix, and at this commit it holds trivially: no connector exists yet and no code path reads a credential.

## A note on line endings

`.gitattributes` carries `docs/engineering/simulation/evidence/** -text`, which disables end-of-line
conversion in both directions for this tree. A retained file is stored exactly as it appears in the working
tree and comes back out of a checkout unchanged, so re-hashing it reproduces a digest taken before the
checkout. That is the whole point, and it makes the endings a file is written with the endings it keeps
forever.

**Every file in this directory is `LF`, and none contains a `CR`.** That matches `WO-MOK-019`, the packet
this one is shaped after, and it is required rather than stylistic for the three `.sh` files, which do not
run with `CRLF`. `WO-MOK-024`'s packet is `CRLF` throughout for the same reason in reverse — its files were
written that way and cannot now be changed.

## What this packet does not establish

- **That the change is correct.** No change is in it.
- **That `REQ-MOK-068` holds.** It fixes the figures the later comparison is made against. The comparison
  is the next capture's, and a difference in it is a finding, not a failure of this one.
- **That the `llm` source works.** No live run is authorized and none was made.
- **That `WO-MOK-025`'s own `schema` value is written.** The row it depended on is ratified and `2` now
  stands, so the value this work order writes is `3` — but it is written under the work order's own change,
  measured there, and nothing here writes it.
