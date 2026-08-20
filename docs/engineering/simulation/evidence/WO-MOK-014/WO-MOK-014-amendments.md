# WO-MOK-014 — `ADR-MOK-006`'s sixteen required amendments, located

**Work order** `WO-MOK-014` · **Branch** `governance/adr-mok-006-third-party-crates` · **Baseline**
`ff3a155f3ce006fdc38abb62df3fca4a2c3c3aa3` · **Date** 2026-08-20

`VER-MOK-014`'s oracle 5 reads: *all sixteen entries of `ADR-MOK-006`'s Required amendments section are present, the
twelve that land in a governed artifact carry their approval, and `ARCH-MOK-001` carries both replacement conformance
checks. Absence fails this contract regardless of code state.* This file is that oracle, entry by entry, with the file
and line where each landed.

`ADR-MOK-006` is itself accepted: `status = "approved"`, `created = updated = 2026-08-20`, and line 18 reads
**"Accepted 2026-08-20 by the repository owner acting as accountable technical owner."** Every approval below is *by
way of* that acceptance — the ADR states each amendment in full, so approving the ADR approved the text, which is why
the ADR was drafted first and approved before any of this was written.

## The twelve in governed artifacts

| # | Artifact | Where the amendment is | Approval |
|---|---|---|---|
| 1 | `REQ-MOK-026` | `requirements/REQ-MOK-026.md:23` | Approved 2026-08-20 by the repository owner acting as accountable **product owner**, by way of `ADR-MOK-006` |
| 2 | `REQ-MOK-050` (new) | `requirements/REQ-MOK-050.md` | `status = "approved"`, `created = updated = 2026-08-20`, `owners = ["product owner"]`. See *The three without an approval row* below |
| 3 | `REQ-MOK-036` | `requirements/REQ-MOK-036.md:23` | Approved 2026-08-20 by the repository owner acting as accountable **product owner**, by way of `ADR-MOK-006` |
| 4 | `ARCH-MOK-001` | `architecture/ARCH-MOK-001.md:48` | Approved 2026-08-20 by the repository owner acting as accountable **technical owner**, by way of `ADR-MOK-006`, *"whose Required amendments section states this amendment in full"* |
| 5 | `ARCH-MOK-002` | `architecture/ARCH-MOK-002.md:42` | same form, **technical owner** |
| 6 | `ADR-MOK-001` | `architecture/adr/ADR-MOK-001.md:22`, a dated note in *Status* | See below — the ADR required a note, not a row |
| 7 | `ADR-MOK-003` | `architecture/adr/ADR-MOK-003.md:29`, a dated note in *Status* | See below |
| 8 | `SPEC-MOK-002` | `specifications/SPEC-MOK-002.md:24` | same form, **technical owner** |
| 9 | `SPEC-MOK-003` | `specifications/SPEC-MOK-003.md:58` | same form, **technical owner** |
| 10 | `SPEC-MOK-004` | `specifications/SPEC-MOK-004.md:25` | same form, **technical owner** |
| 11 | `SPEC-MOK-005` | `specifications/SPEC-MOK-005.md:23` | same form, **technical owner** |
| 12 | `VER-MOK-014` (new) | `verification/VER-MOK-014.md:20` | Approved 2026-08-20 by the repository owner acting as accountable **assurance owner**, by way of `ADR-MOK-006`, *"whose Required amendments section states this contract's minimum content in full"* |

Nine of the twelve carry the approval clause in an amendment-record row, and it names the role the ADR assigned to
that entry: product owner for the two requirements, technical owner for the two architecture documents and the four
specifications, assurance owner for the verification contract. The role in each cell is the role `ADR-MOK-006`'s
subsection heading names, not a role chosen at writing time.

### The three without an approval row, and why each is right

- **`ADR-MOK-001` and `ADR-MOK-003`.** The ADR required *"a dated note"* in each one's *Status* section, not an
  amendment row — an ADR in this repository records a change in *Status* and is superseded or not. Both notes are
  present, both begin **"Note dated 2026-08-20"**, both say what changed and what did not, and both artifacts now
  carry `updated = "2026-08-20"`. `ADR-MOK-001`'s note says the support for its no-network assertion is withdrawn and
  the assertion is not; `ADR-MOK-003`'s says two of its statements are *reversed* rather than narrowed, and lists what
  still stands. Neither ADR is superseded, which is what its own text says and what the ADR required.
- **`REQ-MOK-050` has no *Amendment record* section at all.** That is this repository's convention rather than an
  omission. In the baseline, **2 of 42** requirements carried one — `REQ-MOK-014` and `REQ-MOK-037` — both because they
  had been amended; in the candidate there are 4, the two this change adds below. The other 38 approved requirements
  carry none. A requirement's original approval lives in its front matter — `status = "approved"` — and in the work
  order and ADR that produced it. `VER-MOK-014` does carry an *original approved content* row because the
  verification-contract template opens with one, which `VER-MOK-011` also shows.

### The two new *Amendment record* sections, which are not backdating

`REQ-MOK-026` and `REQ-MOK-036` had no amendment table in the baseline; the ADR required a row *"in the `REQ-MOK-014`
form"*, so the section was created. Creating it means writing the original-content row too, dated the artifact's own
approval date:

- `REQ-MOK-026.md:22` — `2026-08-17` · *"Original approved content: the engine component builds and tests with **no
  external dependency** …"* · **"Approved 2026-08-17 by the repository owner acting as accountable product owner."**
- `REQ-MOK-036.md:22` — `2026-08-19` · *"Original approved content, including required response 8 — the engine
  package's dependency tree resolves to exactly one crate …"* · **"Approved 2026-08-19 by the repository owner acting
  as accountable product owner."**

The precedent is `REQ-MOK-014`, whose table opens with a `2026-08-17` original-content row against
`created = "2026-08-17"`, and `REQ-MOK-037`, whose opens with `2026-08-19` against `created = "2026-08-19"`. Each row
records an approval that happened on the date it names; neither claims a 2026-08-20 approval for it.

### `ARCH-MOK-001`'s two replacement conformance checks

Oracle 5 names these separately because the empty-graph inference is what they replace. Both are present, and so are
the two other conformance-check edits the ADR required:

| `ARCH-MOK-001.md` | Check |
|---|---|
| line 132 | *"Confirm the **engine package's** resolved dependency graph equals the set `SPEC-MOK-002` declares for it, at the declared versions and the declared feature sets, with no undeclared entry and no declared entry missing."* |
| line 133 | *"Confirm by name that the engine package's resolved graph contains no network, asynchronous-runtime, database, model-provider or user-interface crate."* — carrying the sentence that this was previously an inference from emptiness |
| line 138 | The one-library-one-binary check, with *"and an empty dependency table"* struck and *"resolves, builds and tests from the committed lockfile with no registry access"* added (decision 5) |
| line 139 | The new decision-11 review: *"no declared entry implements simulation semantics, owns or advances entropy, or validates an action"* |

## The four with no authority

`ADR-MOK-006` states these four as required amendments and states in each heading that they carry no authority. They
are recorded here as present and consistent with the governed text they cite.

| # | Where | What changed |
|---|---|---|
| 13 | `docs/engineering/REPOSITORY_CONTEXT.md` | The three statements the ADR named, all in the declared-set form: *Additional required verification* (the `cargo tree` one-crate line becomes a per-package comparison with `SPEC-MOK-005` rule 8.4 as the check), *External services or dependencies* (the empty-table requirement becomes a declared set that is empty *today*, with admission routed to the technical owner under `ADR-MOK-006` decisions 1, 4 and 11), and *Local conventions* (the terminal interface is not a declared entry of the engine's set and no user-interface crate may become one) |
| 14 | `.github/workflows/release.yml` | The step *"The engine's dependency table must stay empty"* is replaced by `cargo fetch --locked` and `python3 scripts/check_declared_dependencies.py --root .`. The trigger is untouched. Quoted as a diff in `WO-MOK-014-workflows.txt` |
| 15 | `.github/workflows/dependency-declarations.yml` | New, `pull_request` only, `contents: read`, one job carrying rule 8.4's four checks and nothing else. The managed `engineering-harness.yml` is not touched — `python -m se_harness doctor` reports it `unchanged` in both trees, recorded in `WO-MOK-014-harness.txt` |
| 16 | Source comments | `mokiterions-core/Cargo.toml` above the empty `[dependencies]` table, `mokiterions-tui/Cargo.toml` fixing `ratatui`, and `mokiterions-core/src/lib.rs`'s module documentation asserting the engine *"has no external dependencies"*. Each now states the declared-set form and cites the specification that owns it |

## The second round of dated entries, later on 2026-08-20

The sixteen above are what `ADR-MOK-006` required. Six further dated entries were added later the same day, after the
owner recorded four manual assessments and decided one correction. **None of them changes a rule, a check, an oracle, a
pass condition or a figure**, and each says so in its own text. They are listed separately for the same reason the
reaches below are: an entry that recorded a judgement while quietly moving a rule would be indistinguishable from one
that did not, unless the distinction is written down and checkable.

| Where | Entry | What it records |
|---|---|---|
| `specifications/SPEC-MOK-003.md:59` | New amendment row | The two disclosed transitive capabilities are **accepted**, closing `VER-MOK-014` manual assessment 6. Both *Assessment* cells and the *Security and privacy properties* network bullet now carry the judgement, its three grounds and its limit. Approved 2026-08-20 by the repository owner acting as accountable **technical owner** — the role the assessment names |
| `specifications/SPEC-MOK-005.md:24` | New amendment row | Rule 8.4d's statement of current fact follows: the disclosure whose assessment is recorded is still printed. No rule changed |
| `verification/VER-MOK-014.md:21` | New amendment row | **Four manual assessments recorded** — 1, 2, 3 and 6 — no check, oracle or pass condition changed. Approved 2026-08-20 by the repository owner acting as accountable **assurance owner** |
| `verification/VER-MOK-014.md:22` | New amendment row | **One reference corrected** — the `VREC-MOK-002` phrasing in oracle 3 and four other statements — no rule, check or figure changed. Same approval form |
| `architecture/adr/ADR-MOK-006.md:77` | Second dated note in *Status* | The same reference corrected in the first *Negative* consequence, naming the three other references at lines 158, 514 and 660 as unchanged, and stating that **no decision changes** |
| `work-orders/WO-MOK-014.md:87` | Dated note in *Lifecycle* | Three corrections to the work order: *seven* → **six** manual assessments, the exclusions bullet's disposition of each, and the `VREC-MOK-002` phrase. **No scope change** |

**Why the work order's correction is a *Lifecycle* note and not an amendment row.** `docs/engineering/templates/WORK_ORDER.template.md`
has no *Amendment record* section, and no work order in this repository carries one — `WO-MOK-004` and `WO-MOK-007`
mention amendment records only as instructions to amend *other* artifacts. The precedent used instead is the dated
*Status* note that `ADR-MOK-001` and `ADR-MOK-003` carry at entries 6 and 7 above. The note is dated and states its
approving role, so it records what was true when written rather than rewriting the approved text in place.

**Why `VER-MOK-014` takes two rows and not one.** The assessments and the reference correction are separate acts with
separate reasons, and an amendment row records what was true when it was written. Folding them into one row would make a
later reader unable to tell which of the two the assurance owner approved, and a status change recorded by editing an
existing row would leave no trace that the row had said something else.

## Reaches beyond what the ADR enumerated

Each of these is an edit this work order made that `ADR-MOK-006`'s *Required amendments* section does not name. They
are listed here rather than folded into the sixteen, because an implementation that quietly widened its own mandate is
the failure mode this list exists to expose. Every one is also disclosed in the amendment row that carries it.

- **`SPEC-MOK-003`** gains a *Disclosed transitive capabilities* section and a security bullet. The by-name scan hit
  `mio` and `signal-hook-mio` in a graph `ADR-MOK-003` accepted in 2026-08-17, and without a disclosure mechanism the
  check had two possible outcomes and both were wrong: refuse every release, or carry a term list trimmed until it
  passed.
- **`SPEC-MOK-005` rule 8.4d** gains that disclosure mechanism as a provision — a prohibited-class name in a *declared
  entry* refuses outright; one reached *transitively* refuses until the declaring specification records the crate, the
  chain, the activating feature and what the package does not do with it, and the technical owner judges it.
- **`SPEC-MOK-002` rule 13** gains the mechanical reading convention the checking program depends on (one table, the
  column shape, how a *Features* cell states implied and prohibited features).
- **Three `specifies` relations** to `REQ-MOK-050`, from `SPEC-MOK-002`, `SPEC-MOK-003` and `SPEC-MOK-005`, so the
  requirement's only checks are traceable to the obligation they discharge.
- **Further clauses in `SPEC-MOK-004` and `ARCH-MOK-001`** beyond the enumerated ones, where a restatement of the
  empty-table premise would otherwise have outlived it.
- **The root `Cargo.toml` and `rust-toolchain.toml` comments**, which the ADR's source-comment entry does not name and
  which restate the same withdrawn rule.
- **`VER-MOK-014`'s own count corrected** from fifteen to sixteen required amendments in its oracle-5 row.
- **Three provenance corrections made while measuring**, all recording that `ADR-MOK-006`'s *"eight resolved crates
  that carry a build script"* matches none of the four reconstructible counts — 7 on Windows, 9 on Linux, 9 on macOS,
  10 in union, 12 at `--target all`, 29 in `Cargo.lock`. The note in `ADR-MOK-006`, the `SPEC-MOK-003` amendment row
  and the `SPEC-MOK-003` prose all now say the provenance cannot be reconstructed and give the four figures.
  `WO-MOK-014-build-scripts.txt` is the measurement.
- **The `VREC-MOK-002` precision correction**, which the owner decided on 2026-08-20 after being shown that the record
  holds no replay hash — only `artifact_snapshot_sha256` — and names none of the four configurations. The owner's answer
  enumerated `ADR-MOK-006` and `VER-MOK-014`, and both are corrected. **`WO-MOK-014` carries the same loose phrase in
  its *Required verification* and in its completion-report format, and correcting it there is a reach beyond what that
  answer enumerated**; it is disclosed in the work order's own dated note as well as here. `VREC-MOK-002` is not edited
  and neither is `evidence/WO-MOK-002/determinism-and-resilience.md`, which is where the four hashes are, at lines 12
  to 15, bound through the record's `evidence_paths`.
- **The acceptance of manual assessment 6 reached into code.** Recording the owner's judgement in `SPEC-MOK-003`'s two
  *Assessment* cells was the enumerated part. Beyond it: `check_declared_dependencies.py` now prints
  `disclosed and accepted` where it printed `disclosed and OUTSTANDING` and **still prints the row**, because an
  acceptance that removed the line would be indistinguishable from the disclosure never having existed; a new test,
  `test_an_accepted_disclosure_is_still_printed`, is what holds that; the existing reading test asserts that the cell
  still carries a judgement with its grounds and its limit rather than the bare word *accepted*; and
  `WO-MOK-014-capture.sh` prints the assessment text in full so the retained scan carries the judgement rather than a
  one-word state. No approved text asked for any of the four.

## Where a declared figure is duplicated, and what an amendment must update

Both workflows and `check_declared_dependencies.py` name no crate, version, feature or count — `WO-MOK-014-workflows.txt`
is the search. Two places do hold a copy, and an approved amendment to a declared set has to edit them:

1. **`scripts/test_check_declared_dependencies.py`** — its reading tests assert today's values against the real
   specifications, so an admission that left them untouched would fail the suite. The file's docstring says so.
2. **`docs/engineering/REPOSITORY_CONTEXT.md`** — its *External services or dependencies* paragraph now names
   `ratatui 0.30.2`, its three features and the 57/63/62 per-target figures, in the same sentence as the specification
   that declares them. It is operator-facing context with no authority, and the specification is where a reader is
   sent, but the numbers are a copy and copies go stale.

An earlier draft of `check_declared_dependencies.py` also carried 57, 63 and 62 in its module docstring. That was
rewritten to point at `SPEC-MOK-003` instead: a count in a comment is a second declaration, and the copy in the code
is the one that goes stale. The search in `WO-MOK-014-workflows.txt` is what found it.

## What was deliberately not touched

Every amendment row dated before 2026-08-20 in `ARCH-MOK-001`, `ARCH-MOK-002`, `SPEC-MOK-002`, `SPEC-MOK-003`,
`SPEC-MOK-004` and `SPEC-MOK-005` is **byte-identical** to `origin/master` at `ff3a155`, checked row by row:

| Artifact | Pre-2026-08-20 rows, baseline → candidate | Byte-identical |
|---|---|---|
| `ARCH-MOK-001` | 5 → 5 | yes |
| `ARCH-MOK-002` | 2 → 2 | yes |
| `SPEC-MOK-002` | 4 → 4 | yes |
| `SPEC-MOK-003` | 11 → 11 | yes |
| `SPEC-MOK-004` | 5 → 5 | yes |
| `SPEC-MOK-005` | 3 → 3 | yes |

Among them are the rows standing **OUTSTANDING** from earlier work — `ARCH-MOK-001` and `SPEC-MOK-002` (two) and
`SPEC-MOK-003` (one) at 2026-08-18, `SPEC-MOK-004` at 2026-08-19 — which `WO-MOK-014` is expressly forbidden to clear
or inherit. They are neither cleared nor inherited, and the check above is how that is established rather than
asserted. `VER-MOK-011`'s manual assessment 5 is likewise still outstanding and untouched.
