# Evidence: `WO-MOK-025`, the `llm` decision source and its connector port

This directory retains implementation evidence for `WO-MOK-025`, the first of Phase 5's three work orders.
`VER-MOK-018` is the verification contract it serves.

**This packet is no longer the base capture alone.** It now holds three things: the state of the engine's
output *before* any change of this work order; one governance act that came out of taking that capture; and
the candidate side — the measurements of the implemented change, one file per required-evidence item, plus
the instruments that produced them. What is still outside it is named in *What this packet does not
establish*: no candidate commit is bound here, no completion report is filed here, and no verification
record exists.

The base capture was taken and committed **before** the change, and the work order's *Constraints* require
that ordering and say why — "**The base-commit captures are taken first.** A capture taken after the change
is not a base-commit capture, and `REQ-MOK-068` becomes uncheckable without one. This is the one ordering
constraint that cannot be repaired later." A capture is the one kind of evidence that cannot be
reconstructed once the tree has moved.

Nothing here approves verification, creates a candidate commit, or authorizes release. No live LLM run was
made, none is authorized, and no credential exists anywhere in this directory or in the tree that produced
it: the owner's standing instruction is that "an explicit permission from the repository owner is needed to
launch a real run", and no such permission has been given. Every figure in this packet comes from the four
existing deterministic decision sources, from the scripted stub, or from the transcript this repository
commits.

## Commit binding

| Fact | Value |
| --- | --- |
| Base commit | `cc5418553cb433715b7d6b15dea3886bff30ffaa`, `gov(WO-MOK-025): transition from approved to in_progress`, 2026-08-23 |
| Implementation branch | `feature/phase-5-definition` |
| Candidate commit | not yet taken. The verification record will bind it and its `commit` field will be the authority; this table does not anticipate it |
| Base capture taken on | 2026-08-23, from a `git worktree` at the base commit, built and run there |
| Candidate captures taken on | 2026-08-24, in the implementation tree, at the commits *The candidate side* tabulates — thirteen of the sixteen captures name their own, and three do not |
| Toolchain | rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable, Python 3.14.6, se-harness 0.4.0 |

The base commit is fixed by `WO-MOK-025`'s *Lifecycle*, in the `in_progress` transition subsection, together
with this directory's path. Both were fixed there because neither can be corrected afterwards.

**The toolchain row covers both sides**, re-measured on 2026-08-24 and found identical to the base
capture's. That matters for exactly one reason: a difference between a base figure and a candidate figure is
then this work order's change and not a different compiler's output.

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

**The same eighty runs were taken again on the candidate**, which is what makes `REQ-MOK-068` a measurement
rather than a claim: `candidate/nosink-manifest.txt` and `candidate/sink-manifest.txt` are the candidate's
manifests in the same form, and `candidate/req-068-comparison.txt` is the comparison.

## The candidate side

Each candidate capture is a measurement of one thing, taken at one commit. The commit is stated per file
rather than once for the directory, because they were not all taken at the same commit and a reader who
assumes otherwise would be reading a figure against the wrong tree. The *Taken at* column is the commit the
file's own header names; where a file names none, the column says so and gives the commit that added it,
which is an upper bound on when it was taken and not a claim about the tree it read.

| Capture | What it measures | Taken at |
| --- | --- | --- |
| `candidate/nosink-manifest.txt`, `candidate/sink-manifest.txt` | the eighty runs on the candidate, in `manifest.sh`'s form | `8162b18` |
| `candidate/entropy-manifest.txt` | the twenty configurations' tick-boundary entropy states on the candidate | `8162b18` |
| `candidate/req-068-comparison.txt` | those three manifests against the base's, which is `REQ-MOK-068` | `8162b18` |
| `candidate/schema-digit.txt` | the record stream's `schema` value moving `2` → `3`, and nothing else moving with it | `5ae4f46` |
| `candidate/public-surface.txt` | what this work order added to the engine's public interface, and `SPEC-MOK-002` rule 5's own mechanical checks run | names no commit; committed at `1854f7b` |
| `candidate/replay-identity.txt` | a replay reproducing a recorded run byte for byte, and the same transcript through both hosts | `1854f7b` |
| `candidate/observer-screen.txt` | the observer's six panes reconstructed from the drawn screen, which is what `L31` is about | names no commit; committed at `3675592` |
| `candidate/static-checks.txt` | `L3` and `L11`'s static checks | `3c7a551` |
| `candidate/architecture-checks.txt` | the architecture, usage and security checks that have no other runner | `6309f9c` |
| `candidate/declared-dependencies.txt` | `S1`: each package's resolved graph against its declared set, via the repository's own gate | `6309f9c` |
| `candidate/request-layout.txt` | completion-report item 6: the request's blocks as built, measured over the committed transcript | names no commit; committed at `6309f9c`, amended at `bdaad99` |
| `candidate/transcript-reading.txt` | the independent reading over the transcript: `L4`, `L5`, `L6`, `L12`–`L14`, `L15a`, `L17` | `84f0452` |
| `candidate/gates.txt` | the six gates and the two harness readings at the candidate | the four Rust gates at `5ae4f46`, the two Python gates re-read at `84f0452`; committed at `5336656`, amended at `6cd8002` |
| `candidate/verification-cases.txt` | completion-report item 7: every required case, its result and its evidence | `bdaad99`, amended at `3fbcdda` |
| `candidate/per-tick-lending.txt` | required-evidence item 11: both halves of `L30`, each with the failure a rebuilt port would produce | `d002f04` |

**Three of the sixteen captures name no commit of their own, and that is a gap rather than a decision.**
`candidate/observer-screen.txt` is a drawn frame with no header at all — a header would have been part of the
screen. `candidate/public-surface.txt` and `candidate/request-layout.txt` say "base" and "candidate" and
name the tree they were taken in without pinning it. What holds them together is the same fact that holds
`gates.txt`'s four Rust readings together: `bce4229` is the last commit to change a file under either
package, every one of these was taken after it, and `request-layout.txt`'s figures come from the committed
transcript rather than from a build at all. A capture cannot be corrected after the fact, so the binding is
supplied here instead of edited into them.

**One of those commits is earlier than `bce4229`, and that is measured rather than waved through.**
`candidate/gates.txt` establishes that `bce4229` is the last commit to change a file under either package.
`8162b18` — the three manifests and the `REQ-MOK-068` comparison — precedes it, and what intervened is three
test files and nothing else: `git diff --stat 8162b18
bce4229` over both packages reports `mokiterions-core/tests/no_outcome_threshold.rs`,
`mokiterions-core/tests/replay.rs` and `mokiterions-tui/tests/replay.rs`, `3 files changed`. A test file
cannot change what the eighty runs print, so the manifests taken there are manifests of the candidate's
product code. `bce4229` itself is `test(tui)`, one file, `mokiterions-tui/tests/replay.rs`.

**`bfdbf71` appears in `candidate/request-layout.txt` and is not that file's capture commit.** It is the
commit the repository owner's six rule 11 rulings were taken over, cited where the file reports the finding
those rulings bear on. The file's own figures come from the committed transcript, which is a file in this
repository rather than a build, so a reader who distrusts them can re-derive them with no toolchain at all.

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
- **`candidate/`** — seventeen files: the sixteen captures *The candidate side* tabulates, plus
  **`candidate/verify-schema-digit.sh`**, which re-derives `schema-digit.txt`'s comparison from a checkout.
- **`analysis/`** — the five instruments, each retained beside the capture it produced so that the capture
  is re-derivable and the reader can see what was and was not measured:
  - **`analysis/static-checks.py`** — `L3` and `L11`'s static checks over both packages, at both tiers.
  - **`analysis/architecture-checks.py`** — the architecture, usage and security checks with no other
    runner. It imports `WO-MOK-019`'s instrument for the source-reading and report machinery rather than
    copying it, so the two packets cannot drift in how they read a file.
  - **`analysis/request-layout.py`** — the request's four blocks as the transcript carries them, with the
    flat-versus-nested measurement for block D.
  - **`analysis/observer-screen.py`** — the observer's panes read out of the drawn screen region by region.
  - **`analysis/lending-cursor.py`** — the committed transcript's per-tick record layout, measured from
    outside the crate, which is the outside-in half of `L30`.

## Retention: what is kept whole, what is kept as a digest, and why

**The captures are not retained whole.** The eighty runs produce 246 MB of standard output and record
streams — 63 MB with no sink, 183 MB with one. Committing that would put a quarter of a gigabyte of
generated text into the repository to establish a claim of the form "these two captures are identical, and
this third one differs exactly here", which a digest establishes more strongly than an eyeball comparison
of two 1.2 MB files.

This follows the form `WO-MOK-006`, `WO-MOK-007`, `WO-MOK-011` and `WO-MOK-019` established. What is kept:

- **A digest manifest of every cell of both modes, on both sides.** 43 lines each, four files, 36,692 bytes
  together, covering all one hundred and sixty runs.
- **Four whole text streams**, at seed 42 with tracing off, one per decision source. `WO-MOK-019` retained
  three cells at the same seed and density, so the packets compare directly; the fourth is `social`, which
  did not exist as a retained cell there.
- **One whole record stream**, `seed42-baseline-traceon.jsonl`. It is the counterpart of the one
  `WO-MOK-019` retained for the same cell, and the pair is what makes `wo-019-comparison.txt` section 3 a
  measurement rather than a claim.

**The whole streams are the base capture's alone, and the candidate has none.** The candidate's eighty runs
are retained as manifests only, because what a candidate stream would be read for is a *difference* from the
base, and `candidate/req-068-comparison.txt` measures every one of those differences over the digests. Where
this packet needed a whole candidate artifact it kept the artifact rather than the stream: the committed
transcript is in the repository as a test fixture, and `candidate/observer-screen.txt` retains a whole drawn
frame because a pane cannot be digested into a claim about a pane.

**The untraced baseline record stream is byte-identical to `WO-MOK-019`'s and is deliberately not copied
here.** It already exists in the repository at
`docs/engineering/simulation/evidence/WO-MOK-019/post/full/seed42-baseline-d0.75-traceoff.jsonl`, and a
second copy would only be free to drift from the first. The identity is measured in
`base/wo-019-comparison.txt` section 3.

**What the packet costs, measured.** 43 tracked files, 5,096,271 bytes, of which `base/full/`'s five whole
streams are 4,537,713 — 89 percent. The candidate side is 349,382 bytes across seventeen files and the
instruments are 119,243 across five.

A reviewer cannot inspect an arbitrary cell's output without re-running the capture. That is real, and it is
accepted because a digest already establishes what reading the cell would. Everything not retained is
reproducible with `capture.sh` at the commit each manifest names, and the manifests are what detect a
reproduction that failed. Digests are taken over the bytes exactly as written — nothing is decoded,
normalized or newline-translated.

## The entropy capture

`REQ-MOK-068` requires that adding the `llm` source perturbs none of the four existing ones. Equal output
is the visible half of that; equal *entropy consumption* is the half output cannot show, because a source
that drew a number and discarded it would leave the same text behind and a different world one tick later.

`SplitMix64` advances its state by a fixed odd constant per draw, so the state **is** a draw counter: equal
states after equal tick counts means equal draw counts. `WO-MOK-019`'s `entropy-per-tick.txt` established
this reading. `base/entropy-manifest.txt` captures the state at every tick boundary of all twenty
configurations, so a later capture that differs names the configuration, and re-running the instrument
names the boundary. `candidate/entropy-manifest.txt` is that later capture, and it does not differ: the two
files are byte-identical below their first line, which is each one's own provenance comment, so all twenty
configurations reached the same boundary count, the same per-configuration digest and the same final state.
`candidate/req-068-comparison.txt` is where that comparison is recorded rather than asserted.

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

On the candidate the instrument is in the tree rather than in a patch, as
`the_four_existing_sources_entropy_state_at_every_tick_boundary`, and it is `#[ignore]`d: it prints roughly
twenty thousand lines and is run by name. `candidate/verification-cases.txt` names it, and names the other
two ignored tests, because a case whose evidence is an ignored test looks exactly like a case whose evidence
runs.

## No secret is retained

No API key, token, endpoint credential or provider account identifier appears in any file here. Nothing in
this packet contacted a network. This is the property `ADR-MOK-001` and `SPEC-MOK-007` rules 10.5 and 13.4
fix.

At the base commit it held trivially: no connector existed and no code path read a credential. **On the
candidate it is measured, because the `llm` source now exists.** `candidate/architecture-checks.txt` reports
zero `env::var`, `env::var_os` and `env::vars` sites anywhere in either package at any tier; zero
process-spawning sites; and no socket type, `connect`, URL or HTTP identifier anywhere in either package —
"so there is no route by which anything leaves at all". The one `env::` site that is neither those nor
`env::args` is `env::temp_dir`, in the observer's own test module, choosing a directory for an export test.

That is the same fact `SPEC-MOK-007` rule 12.2 states as holding "whether or not a credential is present in
the environment": there is nothing to read it with. The check `WO-MOK-025` could not make is the one
`scripts/check_workflow_credentials.py` names in its own closing lines — "**NOT CHECKABLE HERE**: whether a
provider credential is present in the repository's Actions secrets" — which `VER-MOK-018` carries as owner
attestation **C6** and which no file in this packet can supply.

## A note on absolute paths in two captures

`candidate/static-checks.txt` and `candidate/architecture-checks.txt` print the repository root as a header
field, so both contain the machine-local path of the tree they were taken in. That is the report form
`WO-MOK-019`'s instrument established and this packet imports rather than re-implements, and it is left as
taken: the files are verbatim instrument output, and editing a line out of a capture presented as verbatim
is a worse defect than the line. No other file in this packet names an absolute path, and
`analysis/lending-cursor.py` reads its transcript relative to the repository root for that reason.

## A note on line endings

`.gitattributes` carries `docs/engineering/simulation/evidence/** -text`, which disables end-of-line
conversion in both directions for this tree. A retained file is stored exactly as it appears in the working
tree and comes back out of a checkout unchanged, so re-hashing it reproduces a digest taken before the
checkout. That is the whole point, and it makes the endings a file is written with the endings it keeps
forever.

**Every one of the 43 tracked files in this directory is `LF`, and none contains a `CR`** — re-measured over
the whole packet on 2026-08-24, counting carriage returns per file rather than matching them per line, which
is the form that does not report a file's line count as its `CR` count. That matches `WO-MOK-019`, the
packet this one is shaped after, and it is required rather than stylistic for the five `.sh` files, which do
not run with `CRLF`. `WO-MOK-024`'s packet is `CRLF` throughout for the same reason in reverse — its files
were written that way and cannot now be changed.

## What this packet does not establish

- **That verification passed.** No verification record exists, and the verification decision is the
  assurance owner's. What is here is the evidence a record would bind.
- **That the candidate commit is what a record will bind.** The candidate commit is not fixed until
  `WO-MOK-025` is `implemented`, which is the engineering owner's transition, and the record's own `commit`
  field is the authority when it exists. Thirteen of the sixteen candidate captures name the commit they were
  taken at instead, and *The candidate side* supplies what it can for the other three.
- **That `WO-MOK-025`'s completion report is filed.** It is not in this directory; the report is the work
  order's, and this packet is what its items 3 to 11 point at.
- **That the `llm` source works against a provider.** No live run is authorized and none was made. Every
  figure in the candidate captures comes from the four deterministic sources, from the scripted stub, or
  from the committed transcript. `candidate/per-tick-lending.txt`'s closing section states what that leaves
  unmeasured: nothing here establishes a live run's cost, because the unit price and the usage are declared
  and synthetic and only the arithmetic over them is measured.
- **That every required case is a plain green.** `candidate/verification-cases.txt` carries 67 rows, one per
  case and per required half-case, and nine of them are not: six are not green at all — three pass in part
  and are escalated for the part that does not run, three are owner acts and not an implementation agent's
  to make — one does not apply at this stage, one passes and is escalated anyway because its literal wording
  is wider than the check, and one passes against a figure the artifacts state differently. Each says why in
  its own row. That file exists because "a case that cannot be run is escalated, not omitted".
- **That the artifacts it cites are free of defects.** Three findings are reported and deliberately not
  repaired, because the artifacts are approved and an implementation agent may not amend one:
  `SPEC-MOK-007` rule 11.4.1's character list, which holds for three of its five characters and is restated
  at two further sites in the engine's source; `L30`'s two-exchange ceiling, which the test replaces with
  eighteen and says why; and the `L20`/`L32` halves this work order's enumerated matrix list omits while its
  prose brings them in. Each is put to the owner.
