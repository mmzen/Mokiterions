# Completion summary — WO-MOK-006

`WO-MOK-006`'s *Completion report format* asks for thirteen things. Each is a numbered section below.
The disclosures follow, and then what this work does not establish.

## 1. Implemented requirements and affected components

**Requirements:** `REQ-MOK-028` (the observer's presentation contract is exercised from outside the
crate), `REQ-MOK-029` (every test in exactly one tier, placed by required access), `REQ-MOK-030` (one
directory per package).

**Components:**

| Component | What happened |
|---|---|
| `Cargo.toml` (root) | replaced by a workspace manifest that declares no package |
| `mokiterions-core/` | the engine package moved here entire — manifest, `src/`, `tests/` |
| `mokiterions-tui/src/lib.rs` | new: the library target that carries the presentation layer |
| `mokiterions-tui/src/main.rs` | its seven `mod` declarations became three `use` statements |
| `mokiterions-tui/tests/` | new: eight files, 77 tests relocated out of the crate |
| `mokiterions-tui/Cargo.toml` | gained `[lib]` and `[[bin]]`; its engine path re-based |
| `SPEC-MOK-002`, `SPEC-MOK-003`, `ARCH-MOK-002` | amended (section 10) |
| `docs/engineering/REPOSITORY_CONTEXT.md` | brought into line (section 10) |
| `SIMULATION_RULES.md` | seven command forms corrected (disclosure D) |
| `.idea/Mokiterions.iml` | source roots corrected (disclosure F) |

**Not affected, and confirmed so rather than assumed:** `ARCH-MOK-001` has a zero-line diff.
`.github/workflows/engineering-harness.yml` and all nine files under `scripts/` name no cargo
invocation, no package, and no source path — a search for `cargo`, `Cargo`, `src`, and `mokiterions`
across `scripts/` returns nothing, and the workflow runs only the harness governor and the artifact
validators. `WO-MOK-006` listed CI in its expected change surface; the correct outcome there was no
change, and this is the measurement that says so. No engine source file changed, no simulation rule
changed, no test was added, removed or renamed.

**Line counts.** Engine: 3,231 lines under `src/`, 667 under `tests/`, 3,898 total — the same 3,898
as before, since all nine files are byte-identical. Observer: 3,747 under `src/`, 2,020 under
`tests/`, 5,767 total, against 5,500 before. The 267-line increase is `src/lib.rs` (33), the eight
public-tier file headers, and the `use` headers each relocated file needs; no presentation logic and
no assertion is in it.

## 2. Authorized local decisions

**The public-tier helper arrangement: no shared helper module.** `WO-MOK-006`'s decision envelope left
this to implementation. The eight files each carry their own helpers: there is no `tests/common/`, no
`mod` declaration, no `#[path]` attribute and no `include!` anywhere under `mokiterions-tui/tests/`.
25 helper definitions across 22 distinct names; `press` is defined in three files and `start` in two.

The reason to choose duplication is that it is not new duplication. The predecessor had the same
three `press` definitions and the same two `start` definitions, one per module test block, and the
relocation carried each file's own helper with it unchanged. Introducing a shared module would have
been a rewrite of test-support code inside a work order whose claim is that nothing was rewritten, and
it would have made each of the eight test crates depend on a ninth thing that no specification
describes. 24 of the 25 helper bodies are byte-identical after dedent; the 25th is a formatter
reflow, recorded in `verbatim-comparison.txt` and in disclosure I.

**The comparison mechanism: git blob identity, SHA-256 of extracted content, and recorded baselines.**
Three mechanisms, each for what it is good for.

- **Blob identity** for whole files: `git hash-object --path=<newpath> <file>` against `git rev-parse
  191db01:<oldpath>`. This is what proves the engine's nine files unchanged, and it is chosen because
  it is immune to the line-ending difference between the working tree (CRLF) and the object store
  (LF) that would defeat a naive `diff` on this machine.
- **SHA-256 of extracted content** where a file changed in one part and must be shown unchanged in
  another: the observer's seven `pub mod` files with every `#[cfg(test)]` item removed by a brace
  scanner, and `main.rs`'s 396-line retained region.
- **Recorded baselines** for behavior: engine output and observer frames and exports captured at
  191db01 before any change, retained under `baseline/`, and compared cell by cell after.

`VER-MOK-006` requires the comparison be bound to a commit, and 191db01 is that commit — the head of
this branch before implementation began, and an ancestor of the candidate.

## 3. Verification commands and results

| Command | Result |
|---|---|
| `cargo fmt --all --check` | exit 0 |
| `cargo fmt --check --package Mokiterions` | exit 0 |
| `cargo fmt --check --package mokiterions-tui` | exit 0 |
| `cargo clippy --all-targets --all-features -- -D warnings` | exit 0 |
| the same, `--workspace`, and per package by name | exit 0 each |
| `cargo build`, `--workspace`, and per package by name | exit 0 each |
| `cargo test --workspace` | **169 passed, 0 failed, 0 ignored**, 19 result lines |
| `cargo test` (bare, at the root) | 169 passed, 19 result lines |
| `cargo test -p Mokiterions` | 60 passed, 8 result lines |
| `cargo test -p mokiterions-tui` | 109 passed, 11 result lines |
| `harnessctl validate` | PASS, 67 artifacts, 0 errors, 0 warnings |
| `harnessctl preflight --work-order WO-MOK-006` | PASS, work order `in_progress`, eligible |

The test run was made immediately after `cargo clean -p Mokiterions -p mokiterions-tui`, so every one
of the 17 test binaries behind those 19 result lines was linked from source rather than replayed — the
other two lines are the packages' `Doc-tests` targets, which run 0 tests. Every result line reports `0
ignored`, which is `REQ-MOK-029`'s single-invocation requirement measured rather than asserted: no
feature flag, no environment variable and no terminal is needed to run any of the 169.

Retained in `static-checks.txt` and `test-run.txt`.

## 4. The final directory layout, and all three manifests

    Mokiterions/
      Cargo.toml                  workspace manifest, no package
      Cargo.lock
      mokiterions-core/
        Cargo.toml                package Mokiterions
        src/    lib.rs  main.rs  cli.rs  simulation.rs
        tests/  cli.rs  density.rs  process.rs  termination.rs  viability.rs
      mokiterions-tui/
        Cargo.toml                package mokiterions-tui
        src/    lib.rs  main.rs  authority.rs  export.rs  layout.rs  options.rs
                render.rs  spatial.rs  state.rs  verification.rs
        tests/  authority.rs  export.rs  layout.rs  options.rs  render.rs
                spatial.rs  state.rs  verification.rs

**Root** — `[workspace]`, `members = ["mokiterions-core", "mokiterions-tui"]`, `resolver = "3"`. No
`[package]`, no `[dependencies]`, no `[workspace.dependencies]`. The resolver is declared explicitly
because a virtual manifest does not inherit a member's edition default, and leaving it out produces a
Cargo warning rather than the edition-2024 behavior both members expect.

**`mokiterions-core/Cargo.toml`** — `[package] name = "Mokiterions"`, `version = "0.1.0"`, `edition =
"2024"`; `[lib] name = "mokiterions"`, `path = "src/lib.rs"`; `[[bin]] name = "Mokiterions"`, `path =
"src/main.rs"`; `[dependencies]` **empty**. Against the predecessor's root manifest the diff is a
single hunk at lines 1–12: the header comment, and the removal of the `[workspace]` table. Everything
from `[package]` down is identical, including both target paths, which did not need to change because
a manifest's paths were always relative to the manifest.

**`mokiterions-tui/Cargo.toml`** — `[package] name = "mokiterions-tui"`, `version = "0.1.0"`, `edition
= "2024"`; `[lib] name = "mokiterions_tui"`, `path = "src/lib.rs"`; `[[bin]] name =
"mokiterions-tui"`, `path = "src/main.rs"`; dependencies `Mokiterions = { path =
"../mokiterions-core" }` and `ratatui = { version = "0.30.2", default-features = false, features =
["crossterm", "layout-cache", "underline-color"] }`. The ratatui entry is byte-identical to before;
the engine entry changed only its path, and is still keyed by the package name `Mokiterions`. Both
library targets are snake case because the declared lint gate implies `non_snake_case`.

Retained verbatim in `manifests.txt`.

## 5. Non-test content comparison, per file, all eight observer module files

| File | Non-test bytes | Verdict |
|---|---|---|
| `authority.rs` | 2,865 | IDENTICAL — `3032447329df8368…` both sides |
| `export.rs` | 1,934 | IDENTICAL — `c64ed2793d3a99ac…` |
| `layout.rs` | 5,912 | IDENTICAL — `5bdc25de8a12189b…` |
| `options.rs` | 6,523 | IDENTICAL — `92dad845d59afb2c…` |
| `render.rs` | 34,365 | IDENTICAL — `4c6ccbe17ab6f682…` |
| `spatial.rs` | 6,202 | IDENTICAL — `23362e7e54e879b4…` |
| `state.rs` | 20,531 | IDENTICAL — `06cb91c4b8d2f5bc…` |
| `verification.rs` | — | **excluded, by construction** — see below |

Seven files, seven identical digests, 78,332 bytes of presentation logic unchanged. A second and
independent count agrees: `pub` occurrences outside every `#[cfg(test)]` block are 5 / 3 / 21 / 12 /
2 / 25 / 54 = **122** on both sides.

**`verification.rs` is excluded because it has no non-test content.** It is a `#[cfg(test)]` module in
its entirety — `lib.rs` declares it under that attribute — so subtracting its test content leaves
nothing to hash. Its 24 tests are compared as tests instead: 16 relocated to `tests/verification.rs`,
byte-identical, and **8 retained in place, byte-identical**, each retained because it reaches one of
the four `#[cfg(test)]` hooks. That separate comparison is the retention list's requirement for this
file and is in `verbatim-comparison.txt`.

## 6. Per-file relocation comparison, with every required path reference

**Byte-identical, at blob level, all nine engine files:** `src/cli.rs` `fe6cf9e6a9`, `src/lib.rs`
`7302d06f4f`, `src/main.rs` `6a52e79ef9`, `src/simulation.rs` `aa1ce41fb5`, `tests/cli.rs`
`61d8da560f`, `tests/density.rs` `2b76af7fed`, `tests/process.rs` `4d32cb71da`,
`tests/termination.rs` `5800fd0ba6`, `tests/viability.rs` `76256c716d`. Nine renames, zero content
changes.

**Byte-identical outside test blocks, all seven observer modules:** section 5.

**Every required path reference — seven, each justified:**

1. Root `Cargo.toml` replaced and the old one moved to `mokiterions-core/`. Required: `REQ-MOK-030`
   requires the root to declare no package, so the `[workspace]` table had to separate from the
   `[package]` tables.
2. `[workspace]` removed from the engine's manifest and its header comment rewritten. Required — the
   same separation from the other side. The comment is the one edit of the seven not strictly forced;
   it is kept because a comment claiming the package sits at the root would now be false.
3. `path = ".."` → `path = "../mokiterions-core"` in the observer's manifest. Required: the engine is
   no longer the parent directory.
4. Seven `mod` declarations in `mokiterions-tui/src/main.rs` → three `use mokiterions_tui::…`
   statements. Required in the strong sense: declaring the modules in both targets would compile them
   twice and give the package two copies of every type.
5. `mokiterions-tui/src/lib.rs` new. Required: it is the library target `REQ-MOK-028` asks for.
6. `crate::state::EVENT_CAPACITY` → `mokiterions_tui::state::EVENT_CAPACITY`, twice, in one test.
   Required: a `tests/` file is its own crate, so `crate::` would name itself.
7. `.idea/Mokiterions.iml` source roots. Required in that the IDE had written four new entries and
   left two stale ones pointing at a root `src/` and `tests/` that no longer exist.

Nothing else in either package names a path that moved. Retained in `file-comparison.txt`.

## 7. Every `SPEC-MOK-004` rule 14 command form, executed

| Form | Result |
|---|---|
| `cargo run --bin Mokiterions` | exit 0, 1,416 lines, `summary reason=tick_limit ticks=100 survivors=12 …` |
| `cargo run --bin Mokiterions -- --help` | exit 0, 25 lines |
| `cargo run -p Mokiterions --bin Mokiterions -- --help` | exit 0, same output |
| `cargo run --bin Mokiterions -- --seed 42 --ticks 1000` | exit 0, 11,222 lines, `survivors=8 deaths=4` |
| `cargo run --bin Mokiterions -- --policy baseline` | exit 0, 1,362 lines |
| `cargo run --bin Mokiterions -- --density 1.5` | exit 0, 1,549 lines |
| `cargo run --bin Mokiterions -- --ticks 40 --trace-actions` | exit 0, 1,119 lines |
| `cargo run --bin Mokiterions -- --seed 42 --ticks 200 --density 0.02` | exit 0, `reason=extinction ticks=134` |
| `cargo run -p mokiterions-tui -- --help` | exit 0, 23 lines |
| `cargo run -p mokiterions-tui -- --ticks 0` | exit 2, refusal on standard error, before any terminal |
| `cargo run -p mokiterions-tui` (bare, with stdin closed) | exit 124 under a 25-second timeout — it resolved and held its loop |

**The first line of `USAGE`, which is the load-bearing comparison:**

    Usage: Mokiterions [--seed <u64>] [--ticks <u64>] [--policy <baseline|reference>]

Byte-identical to two independent records of the predecessor's — `191db01:src/cli.rs` line 11 and the
retained `WO-MOK-003/after/exit-codes.txt`. The word `mokiterions-core` appears nowhere in an
operator's output.

**Both `cargo tree` results:**

    $ cargo tree -p Mokiterions --edges all
    Mokiterions v0.1.0 (C:\Users\mathi\RustroverProjects\Mokiterions\mokiterions-core)

    $ cargo tree -p Mokiterions --all-features --edges all
    Mokiterions v0.1.0 (C:\Users\mathi\RustroverProjects\Mokiterions\mokiterions-core)

One line each: the package itself. `--edges all` covers normal, build, dev, proc-macro and feature
edges, so this is not the narrow reading — the engine depends on nothing of any kind, which is
`ARCH-MOK-001` as amended and `REQ-MOK-026`. The observer's graph is unchanged too: 57 crates on
normal edges, 59 with build and proc-macro edges, 37 excluding proc-macro crates, 61 counting the
workspace's own two packages — the same four figures `WO-MOK-005` recorded, and `Cargo.lock` is
byte-identical, blob `824003e1cbd4757f28d09849e2faca0a4546e99f`.

Retained in `command-forms.txt` and `dependencies.txt`.

## 8. Public-item count per module, against the recorded 97

| Module | `SPEC-MOK-004` rule 6 | Measured |
|---|---|---|
| `authority` | 5 | 5 |
| `export` | 3 | 3 |
| `layout` | 13 | 13 |
| `options` | 8 | 8 |
| `render` | 2 | 2 |
| `spatial` | 19 | 19 |
| `state` | 47 | 47 |
| **total** | **97** | **97** |

Plus 25 public fields and 22 public variants, which rule 6's counting rule excludes from the 97 and
which are enumerated anyway. The counting rule applied is stated in `public-item-census.txt`, and the
census lists all 97 by name and kind. `lib.rs` adds nothing: it declares seven modules and defines no
item, so the library target's interface is exactly the pre-existing 97.

## 9. Test census reconciliation and final tier populations

**Engine — 60, unchanged:**

| Tier | Location | Tests |
|---|---|---|
| internal | `src/simulation.rs`, inline | 37 |
| public | `tests/cli.rs` 12, `density.rs` 2, `process.rs` 5, `termination.rs` 3, `viability.rs` 1 | 23 |
| — | `src/main.rs` (binary) | 0 |

**Observer — 109, redistributed:**

| Tier | Location | Tests |
|---|---|---|
| internal | `render.rs` 12, `state.rs` 4, `verification.rs` 8, `main.rs` 8 | 32 |
| public | `authority.rs` 4, `export.rs` 7, `layout.rs` 7, `options.rs` 7, `render.rs` 8, `spatial.rs` 7, `state.rs` 21, `verification.rs` 16 | 77 |

**Reconciliation: 169 before, 169 after.** Name by name, in both directions: every name in the
predecessor's `cargo test -- --list` appears in the candidate's, and no name appears twice anywhere in
the workspace. Before the change the observer's 109 were all internal; after it, 77 are public and 32
internal. The split is the 77/32 `SPEC-MOK-004` rules 9 and 10 predicted.

**No corrected assignment was needed.** Rule 9 provides for implementation finding a test assigned
differently and requires the corrected count be recorded; the measured split matched the predicted
one, so there is nothing to correct and nothing was moved by weakening it. Each of the 32 is
justified by required access in `test-placement.md`, and the split is **16 / 16**: sixteen call a
private item and no hook, sixteen reach one of the four hooks and no private item. None is held inside
by both, so each of the 32 has exactly one kind of reason to be there. All four hooks retain their
attribute, all nine `#[cfg(test)]` attributes in the observer's `src/` are present, and no public-tier
file names a hook — if one did, it would not compile.

Retained in `test-census.txt`, `test-placement.md`, `hooks-and-visibility.txt`, `test-run.txt`.

## 10. Amendment status of the four artifacts

| Artifact | Text written | Approved |
|---|---|---|
| `SPEC-MOK-002` | here, by the implementation agent under `WO-MOK-006` | 2026-08-18, repository owner as technical owner, by way of `ADR-MOK-004` |
| `SPEC-MOK-003` | here, same | same |
| `ARCH-MOK-002` | here, same | same |
| `REPOSITORY_CONTEXT.md` | here, same | **no approval required** — repository-owned guidance, not a governed artifact |

The four amendments are approved **by way of `ADR-MOK-004`**, whose *Required amendments* section
states each of them in full and which the owner approved with the rest of the chain on 2026-08-18.
That is a reading, and it is disclosed as one — see disclosure A. Nine lifecycle transitions were
recorded by the implementation agent on the owner's explicit instruction (*"You approve, I record
it"*); `WO-MOK-006`'s *Approval record* says the agent recorded them and did not make the decision.

No commit-bound record was edited. `VREC-MOK-001` through `VREC-MOK-005` and the evidence retained
under `WO-MOK-001` through `WO-MOK-005` name paths that no longer exist and stay as they are;
`requirement-to-test-mapping.md` in this packet is the superseding mapping `ADR-MOK-004` prescribed for
that purpose. The three `OUTSTANDING` amendment rows carried by `SPEC-MOK-002` and `SPEC-MOK-003` from
`WO-MOK-005` are untouched and still say OUTSTANDING.

Full record, including the three places where the agent exercised judgement about form, in
`amendment-approvals.md`.

## 11. Retained evidence paths

All under `docs/engineering/simulation/evidence/WO-MOK-006/`. `README.md` maps each of
`VER-MOK-006`'s 19 retention bullets to the file that discharges it.

    README.md                        the retention map
    completion-summary.md            this file
    manual-assessment.md             the eight judgements VER-MOK-006 asks a reader to make
    conformance.md                   ARCH-MOK-001, ARCH-MOK-002, ADR-MOK-001, ADR-MOK-003 re-run
    amendment-approvals.md           what was approved, what was written, and who did which
    requirement-to-test-mapping.md   the superseding mapping, 152 cited tests resolved
    baseline/                        capture scripts, the frame-and-export oracle, engine/, observer/
    comparison/                      engine-matrix.txt, observer-matrix.txt
    test-census.txt  test-placement.md  test-run.txt  verbatim-comparison.txt
    non-test-content.txt  public-item-census.txt  hooks-and-visibility.txt
    file-comparison.txt  manifests.txt  dependencies.txt  command-forms.txt
    static-checks.txt  resilience.txt  compile-and-target.txt

## 12. Residual limitations and deferred items

`VER-MOK-006`'s own *Residual uncertainty* section states eight; they hold as written and are not
restated here. What this work adds to them:

- **No compile-fail harness exists.** That a public-tier test *cannot* reach a private item is
  established by the compiler refusing to link one, which is real but only for tests that were
  written. It is not a mechanical proof that a future violating test fails to compile.
- **Equivalence is across the declared matrices, not the input space.** 40 observer cells (35 frames
  and 5 exports), 43 engine cells and 24 observer start-up cases, at declared seeds, densities and
  viewports. Neither a directory move nor a library target has a plausible mechanism for
  input-dependent divergence, but that is an argument, not a measurement.
- **Provenance closure inherits the predecessor's judgement.** Rule 6 makes the interface "what was
  public before", so if some of the 97 are public by accident, that accident is now the contract.
  Narrowing it is a later requirement and an amendment to `SPEC-MOK-004`.
- **Deferred, explicitly:** no shared public-tier helper module; no `pub use` facade at the observer's
  crate root; no enumeration of the 97 items in an approved artifact (disclosure C); `default-run` and
  `[workspace] default-members` considered and rejected for the bare-`cargo run` regression, the
  latter because it would reintroduce the CI defect this work order exists to remove.
- **The initiative's central benefit is not measured here.** That a break in what the observer presents
  now fails a test with no private access to repair itself with would require deliberately breaking the
  contract to demonstrate. What is verified is the precondition: 77 tests reach the observer only
  through its public interface.

## 13. Worktree and candidate-commit status

Branch `feature/package-layout-and-test-tiers`. Predecessor commit **191db01** (`docs: draft the
artifact pack for the package-layout restructure`), which is the baseline every comparison in this
packet is bound to.

**This section was written before the implementation was committed and is updated here rather than
left stale.** Every measurement above was taken on the working tree that became commit **83e487f**
(`refactor: one directory per package, and a test tier outside the observer's crate (WO-MOK-006)`,
112 files changed), so no figure changes; what changes is that the tree now has a name. The branch is
pushed to `origin` and open as pull request **#13** against `master`, where `harnessctl preflight
--phase review`, `harnessctl doctor` and both artifact scripts pass in the `governor` and `candidate`
jobs. No tag, release, publish or deploy has been made and none was authorized.

`VREC-MOK-006` is prepared against the branch commit that carries this correction, at
`status = "ready"`. A verification record binds a commit and is created after the one it names, which
is why it could not exist when this summary was first written. Transitioning it to `verified` is the
accountable assurance owner's decision and has not been taken here.

A scratch worktree used for the baseline captures was removed after use; `git worktree list` shows the
primary worktree only.

---

## Disclosures

`WO-MOK-006` requires the completion report to disclose what the work found that the artifacts do not
say. Ten items, **A** and **C** first because they need a decision rather than a read.

### A. The four amendments rest on a reading of what `ADR-MOK-004`'s approval covered

The owner approved `ADR-MOK-004`, whose *Required amendments* section enumerates all four amendments —
which artifact, which sections, what each must say. This packet treats that approval as covering the
amended text, which is the construction `WO-MOK-003` and `WO-MOK-005` used for their chains.

**An alternative reading exists** in which approving an ADR that *requires* amendments is not the same
act as approving the amended text; under it, the four remain outstanding until approved individually.
The distinction is not academic: two of the four artifacts are bound by a `verified` verification
record. Nothing in this packet depends on the reading — if the owner takes the narrower one, four
Approval columns change and no measurement does. **This is for the assurance owner to weigh.**

### B. `VER-MOK-006`'s public-item wording is ambiguous

The contract records 97 items and separately names fields and variants. Read as *items, excluding
fields and variants*, the figure is exact: 97, and 97 + 25 public fields = 122, which matches the
independent count of `pub` occurrences outside test blocks. Read as *items plus fields plus variants*,
the total is 144 and matches nothing recorded. The fields-only reading is the one that reconciles
across two independent measurements, so it is the reading applied. **The approved contract is not
edited**; the ambiguity is reported.

### C. The observer's interface cannot be enumerated from any approved artifact

`SPEC-MOK-004` rule 6 gives per-module counts and a provenance definition, not a list. A reader who
wants to know *which* 97 items must read the code or `public-item-census.txt` in this packet. This is
the deliberate trade `ADR-MOK-004` took — a maintained 97-item list for a component that holds no
authority would cost more than it protects — and it is the one qualified adverse finding in
`manual-assessment.md`. It is an observation about rule 6 as approved, not a defect in the change.

### D. Seven bare `cargo run` invocations in `SIMULATION_RULES.md` had stopped resolving

**A real regression, found and fixed under this work order.** At a virtual-manifest root with two
binaries, bare `cargo run` fails with exit 101 rather than choosing one. `VER-MOK-006` requires that
every command form in the repository's documentation resolve after the change, so this was a
conformance failure. All seven now name `--bin Mokiterions` and were re-run to exit 0; two sentences
were added naming `cargo run -p mokiterions-tui -- --help` for the observer.

### E. `VER-MOK-001` step 1 would no longer resolve, and was deliberately not edited

It reads `cargo run -- --seed 42 --ticks 100`. It is an approved verification contract already
discharged against its own commit, and editing it would rewrite a record about a tree in which the
command was correct. Disclosed, not edited.

### F. `.idea/Mokiterions.iml` source roots were corrected

The IDE had already written four entries for the new directories and left two stale ones pointing at a
root `src/` and `tests/` that no longer exist. Corrected to the four real roots. This is editor
configuration and affects no build, no test and no CI job.

### G. `ARCH-MOK-002`'s `addresses` relation lists only `REQ-MOK-028`

Which is exactly what `ADR-MOK-004` states. `REQ-MOK-029` and `REQ-MOK-030` are traced through
`conforms_to = SPEC-MOK-004`, which `specifies` all three, and both are visibly addressed in the
architecture's own text — the one-directory-per-package required pattern cites `REQ-MOK-030` by name.
`harnessctl validate` reports 0 errors and 0 warnings, so the harness is satisfied. Whether the two
should also be listed is the technical owner's call; the agent wrote the amendment as approved rather
than as it might have been improved.

### H. `ADR-MOK-004`'s prose status disagreed with its frontmatter

After the recorded transition the frontmatter said `approved` while the prose *Status* section still
said "Proposed." The prose now reads "Accepted by the technical owner on 2026-08-18", matching the
frontmatter and the form `ADR-MOK-001` and `ADR-MOK-003` use. Leaving the two disagreeing would have
made the record ambiguous about the one thing it exists to state.

### I. One test helper was reformatted by the formatter, not by hand

`render.rs`'s `press` helper went from six lines to three. Dedenting it from a `mod tests` block to
file scope brought its call line from 101 columns to 97, under `rustfmt`'s default `max_width` of 100
(there is no `rustfmt.toml`), so `cargo fmt` closed the wrap. Same call, same arguments, same
`KeyEvent`. Recorded because the comparison's value comes from printing everything it finds; the other
24 relocated helper bodies are byte-identical after dedent.

### J. `SPEC-MOK-004`'s Performance section predicts eleven observer test binaries; there are ten

The section says "the observer contributes eleven test binaries where it contributed one". The measured
figure is **ten**: the library target's, the binary target's, and eight integration binaries, one per
file under `mokiterions-tui/tests/`. Eleven is the count of *result lines* for the package, which
includes `Doc-tests mokiterions_tui` — a target, not a test binary, and one that runs 0 tests. On the
same reading the workspace has 17 test binaries and 19 result lines.

The prediction is off by one on the reading its own words carry. Nothing follows from it: the section
states an expected compile-time cost, attaches no threshold and no obligation, and
`compile-and-target.txt` measures the cost that was actually incurred. Carried here for the owner to
dispose of.

---

## What this summary does not establish

Every figure above is a measurement of this tree against 191db01. It says nothing about whether the
observer's public interface is *well designed*, only that it is counted, read-only, unchanged, and
sufficient for the 77 tests that now depend on it. It says nothing about whether one directory per
package makes the repository more legible to a reader — that benefit is real to argue and not
measurable by any check here. And it says nothing about the correctness of the simulation, which is
`VER-MOK-001` through `VER-MOK-005`'s subject and which this work order was written specifically not
to touch.

**The honest cost, stated because `VER-MOK-006` attaches no obligation to it and it would be easy to
omit:** `cargo test --no-run` from clean goes from 2,974 ms to 3,735 ms (+761 ms, +26%), `target/`
after a full test build from 349 MB to 499 MB (+150 MB), and the workspace links 17 test binaries where
it linked 8. That is what a second library target and eight integration crates cost.
