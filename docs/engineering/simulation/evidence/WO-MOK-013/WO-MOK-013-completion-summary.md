# `WO-MOK-013` — completion summary

| Field | Value |
|---|---|
| Work order | `WO-MOK-013` — crates policy, declared-set comparison |
| Deciding ADR | `ADR-MOK-006`, accepted 2026-08-20 by the repository owner acting as accountable technical owner |
| New requirement | `REQ-MOK-047`, approved 2026-08-20, product owner |
| Verification contract | `VER-MOK-013`, approved 2026-08-20, assurance owner |
| Branch | `governance/adr-mok-006-third-party-crates` |
| Baseline | `ff3a155f3ce006fdc38abb62df3fca4a2c3c3aa3` (`origin/master`, *Merge pull request #30*) |
| Commits | **none.** Nothing is committed, pushed, tagged or opened as a pull request |
| Work-order status | `in_progress`, moved there from `approved` on the instruction to implement. **Not** transitioned to `implemented` |
| Toolchain | cargo 1.97.1 (c980f4866 2026-06-30) · rustc 1.97.1 (8bab26f4f 2026-07-14) · Python 3.14.6 |
| Date | 2026-08-20 |

The approving instruction was *"i approve the ADR, i approve the work order: go implement"*. Implementation is done.
Committing, pushing, tagging, opening a pull request and transitioning the work order are five separate acts, none of
them named in that instruction, and none performed. The change sits in the working tree.

## What the policy is now

The engine package's empty-dependency-table rule is withdrawn. Third-party crates are admissible in **both** workspace
packages on the owner's test, quoted normatively in `ADR-MOK-006` decision 1 and not paraphrased anywhere:

> Choose third-party components when they provide stable, well-maintained functionality that accelerates delivery
> without introducing excessive dependency debt. Prioritize proven solutions for standard, non-core features so
> development teams can focus engineering effort entirely on building proprietary business value

Everything that made the old rule enforceable is replaced rather than dropped: every admitted crate is **declared** in a
specification with its exact version and exact feature set, admission is a specification amendment approved by the
technical owner and never an implementation choice, and a mechanical check compares each package's declared set against
its resolved graph in both directions on all three release targets.

**The original statement was verified before being relaxed**, as the first instruction asked. It was true for the engine
package — an absolute ban, restated in `REQ-MOK-026`, `REQ-MOK-036`, `ARCH-MOK-001`, `ADR-MOK-003` decision 4,
`REPOSITORY_CONTEXT.md`, both manifests' comments, `lib.rs`'s module documentation and a `release.yml` step. It was
**false for the observer package**, where `ratatui 0.30.2` was admitted and further crates were gated by escalation
rather than prohibited. `ADR-MOK-006`'s *Context* records the asymmetry and one measured detail nobody had written
down: the dependency check ran only in `release.yml`, so an engine dependency added on 2026-08-19 would have passed
pull-request CI and been refused at release.

## Change inventory

17 modified files, 8 new paths (`git status --short`, with the evidence directory collapsed as one entry):

| Kind | Files |
|---|---|
| New governed artifacts | `ADR-MOK-006`, `REQ-MOK-047`, `VER-MOK-013`, `WO-MOK-013` |
| Amended governed artifacts | `REQ-MOK-026`, `REQ-MOK-036`, `ARCH-MOK-001`, `ARCH-MOK-002`, `ADR-MOK-001`, `ADR-MOK-003`, `SPEC-MOK-002`, `SPEC-MOK-003`, `SPEC-MOK-004`, `SPEC-MOK-005` |
| New code | `scripts/check_declared_dependencies.py` (1,225 lines), `scripts/test_check_declared_dependencies.py` (856 lines, **56 tests, all passing**) |
| New workflow | `.github/workflows/dependency-declarations.yml` (108 lines, `pull_request` only, `contents: read`) |
| Amended workflow | `.github/workflows/release.yml` — the empty-table step replaced by `cargo fetch --locked` and the declared-set check |
| Non-authoritative text | `docs/engineering/REPOSITORY_CONTEXT.md`, `Cargo.toml`, `mokiterions-core/Cargo.toml`, `mokiterions-tui/Cargo.toml`, `mokiterions-core/src/lib.rs`, `rust-toolchain.toml` |
| Evidence | 19 files under `evidence/WO-MOK-013/`, every one named `WO-MOK-013-*` |

**No Rust behaviour changes.** `mokiterions-core/src/lib.rs` is touched in its module documentation only; both manifests
are touched in comments only. The declared sets are unchanged in substance: the engine's is empty and the observer's is
the one `ratatui` entry that `ADR-MOK-003` already fixed. **This change admits no crate.** It changes what admitting one
would require.

## Gates, at the candidate tree

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | exit 0, no output |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | exit 0, no `allow` added |
| `cargo test --workspace --locked` | exit 0, every tier of both packages in one invocation |
| `python scripts/check_declared_dependencies.py --root .` | exit 0 — *"Every declared set matches its resolved graph. 8.4a–8.4d pass."* |
| `python -m unittest test_check_declared_dependencies -v` | 56 tests, OK |
| `python scripts/validate_engineering_artifacts.py` | PASS, 106 artifacts, 0 errors, 0 warnings |
| `python -m se_harness doctor` (pinned 0.4.0, the version CI installs) | exit 0, 81 PASS, 0 FAIL — **identical to the baseline**, see below |

Sources were touched before clippy and before the test run so neither reports a cached result. Every `cargo`
invocation passes `--locked`, and none reaches a registry.

## Baseline against candidate, because a single number is not evidence

`WO-MOK-013-harness.txt` holds this in full. Three things in it matter to a reader of the figures.

**Measurement method changed the finding set.** The baseline was first taken from a `git archive` extraction, which has
no `.git`, so the inspector's `I-REV-001` could not run and reported nothing — making the candidate's **12** info
findings look new. Re-measured with `git worktree add --detach origin/master`, the baseline has **the same 12**. The
first reading is retained as a caution rather than deleted, because it is exactly the kind of difference a completion
report would otherwise have claimed.

**An unchanged warning count hides a completely changed finding set.** Both trees report 7 warnings, 0 errors, 19
findings. `W-HEX-003`'s five observations are **disjoint** between them:

| | Observations |
|---|---|
| baseline | `ADR-MOK-001`→`ARCH-MOK-001`; `ARCH-MOK-001`→`SPEC-MOK-001`/`SPEC-MOK-002`; `ARCH-MOK-002`→`SPEC-MOK-003`/`SPEC-MOK-004` |
| candidate | `ADR-MOK-002`→`ARCH-MOK-001`; `ADR-MOK-004`→`ARCH-MOK-002`; `VER-MOK-005`→`REQ-MOK-026`; `VER-MOK-008`→`REQ-MOK-036`; `WO-MOK-008`→`SPEC-MOK-003` |

The mechanism is the amendment dates: ten existing artifacts moved to 2026-08-20, so five warnings cleared because
their *sources* caught up and five appeared because their *targets* moved ahead of unamended sources. *"7 warnings
before, 7 after"* without that table would be true and misleading at once.

**The five new warnings are a real signal and an owner decision.** `REQ-MOK-026`'s prohibition was reworded and
`VER-MOK-005` verifies the old wording; `REQ-MOK-036` and `VER-MOK-008` stand in the same relation. Whether those two
verification contracts are reassessed is not an implementation call and is not made here.

**Doctor passes, and which harness ran is part of the figure.** Under the pinned **0.4.0** venv — the version
`.github/workflows/engineering-harness.yml` installs, pinned exactly — both trees report **exit 0, 81 PASS, 0 FAIL**.
Every `managed:` check reports `unchanged`, and the managed `.github/workflows/engineering-harness.yml` is not edited.
An earlier reading of this gate, taken with the machine-wide **0.4.1** install, reported exit 1 with eight
`distribution:` FAILs and concluded that doctor *"is not a gate this change can be said to pass or fail"*. That was
wrong: the eight are template skew between 0.4.1's templates and a repository whose declared runtime is 0.4.0, the
baseline reports the same eight under 0.4.1 and none under 0.4.0, and the correction with both figures is retained in
`WO-MOK-013-harness.txt` rather than replaced — the wrong reading is what a reader reproduces by running the obvious
command.

## The check, and that it can be made to fail

`WO-MOK-013-injection.txt` records the program refusing in **ten distinct ways**, each from one edited declaration on a
throwaway copy: a moved version (4 refusals, in both directions), a removed feature (4, including a feature arriving by
unification rather than by declaration), a wrong per-target count, a deleted build-script row, a renamed disclosure row
(3, including a disclosure no target reaches), a declared entry absent from the manifest, `ratatui` declared for the
engine, an undeclared manifest entry, a stale lockfile, and `ratatui` fully added to the engine — which produces 23
refusal lines including the surviving `REQ-MOK-026` user-interface prohibition that no declaration can satisfy. A check
that cannot be made to fail has not been demonstrated to work.

**The stale-lockfile case is ordered deliberately.** A manifest dependency added without updating `Cargo.lock` makes
cargo refuse under `--locked` *before* any comparison runs, so the message says so and says the fix — update the
lockfile in the same change and commit both — rather than sending a reader to fetch. `cargo_failure()` distinguishes it
from an empty cache, and a test asserts the two are told apart.

**Offline-ness is proved, not asserted.** With `CARGO_HTTP_PROXY=http://127.0.0.1:1`, a control command
(`cargo search ratatui --limit 1`) **fails** with exit 101 and `[7] Could not connect to server`, and then
`cargo build -p Mokiterions --locked --offline` and `cargo test -p Mokiterions --locked --offline` both succeed.
`Cargo.lock`'s sha256 and `git status --porcelain` are byte-identical before and after. Without the failing control,
the passing build would establish nothing.

**One declaration, two readers — checked mechanically.** Both workflows were searched for all 13 declared and disclosed
crate names, all 13 versions, 5 feature names and 7 declared figures, using the program's own parser to produce the
terms. **No hit**, except the string `10` in *"rule 10's three targets"*. This search is what found the one restatement
that existed: `check_declared_dependencies.py`'s module docstring carried 57, 63 and 62, and was rewritten to point at
`SPEC-MOK-003` instead — a count in a comment is a second declaration, and the copy in the code is the one that goes
stale.

## Figures a reader can confuse, each with the question it answers

Every count in the specifications is now stated with its target, because none of them is target-independent.

| Question | Answer |
|---|---|
| Crates a build of the observer compiles, per target | **57** Windows · **63** Linux · **62** macOS |
| Distinct crates across rule 10's three targets | **66** |
| Crates any target could reach (`--target all`) — not a build of anything | **71** |
| Packages in the lockfile | **182** |
| Crates in the engine's graph, per target, at `-e normal,build,dev` | **1** — itself |

**Crate identity is (name, version), not name.** `hashbrown` resolves at both `0.16.1` and `0.17.1`, and `syn` at both
`2.0.119` and `3.0.3`, on all three targets. Counting by name gives 55/61/60; the declared figures are 57/63/62. Two
readers using two rules would disagree by exactly two on every target, so the specification states the rule.

**`std` is implied, not declared.** `ratatui` resolves with `crossterm`, `layout-cache`, `std`, `underline-color` while
three features are declared. `std` arrives from the declared three, is recorded as implied, and the checking program
distinguishes *implied* from *undeclared* — otherwise every run would refuse.

## Build scripts: the ADR's figure matches none of the four

`ADR-MOK-006` decision 13 states *"8 of the observer's 59 resolved crates already ship one"* and names `instability`,
`parking_lot_core`, `proc-macro2`, `quote`, `rustversion`, `syn`, `thiserror`, `winapi`. **Four counts are
reconstructible and the ADR's figure is none of them.** The denominator is not the problem: **59 reproduces**, and the
ADR states its own derivation at line 270 — `cargo tree -p mokiterions-tui -e normal --locked` on the host resolves 59,
the 57 external crates of `SPEC-MOK-003` plus both workspace members, since a package's own graph includes itself and
the observer depends on the engine. Only the numerator is unreconstructible:

| Count | Value |
|---|---|
| Windows | **7** — `instability`, `parking_lot_core`, `proc-macro2`, `quote`, `rustversion`, `thiserror`, `winapi` |
| Linux | **9** — the above minus `winapi`, plus `libc`, `rustix`, `signal-hook` |
| macOS | **9** — the same nine as Linux |
| Union of the three | **10** |
| Observer at `--target all` | **12** (adds the two `winapi-*-pc-windows-gnu` shims) |
| Packages in `Cargo.lock` carrying one | **29**, of which **17** are reachable from neither package at `--target all` |

The ADR's list is the Windows seven **plus `syn`** — and `syn` carries a build script only at `1.0.109`, which is one of
the 17 lockfile packages no build reaches (`cargo tree -i syn@1.0.109 --target all` prints *"nothing to print"*, as does
the same query for `thiserror@1.0.69`). That is consistent with the figure having been read off the lockfile rather than
off a graph, though the provenance cannot be reconstructed and this summary does not claim it. **The declared table
records the per-target counts**, because those are what a build executes, and three artifacts now state that the ADR's
figure could not be reproduced and give all four: `ADR-MOK-006`'s own note, `SPEC-MOK-003`'s amendment row and
`SPEC-MOK-003`'s prose. `WO-MOK-013-build-scripts.txt` is the measurement. **This is the first time this repository has
written down that it executes third-party code at build time.**

## The by-name scan, and what it cannot do

126 term entries — 104 across the six capability classes `ADR-MOK-006` decision 4 preserves, plus 22 user-interface
terms scanned against the engine's graph only. They were written from the prohibitions and not from the graph, and the
**raw hits before any disclosure** are retained so a later reader can see that: a term list derived from the graph would
pass by construction.

**Two raw hits, disclosed rather than filtered.** `mio 1.2.2` and `signal-hook-mio 0.2.5`, on Linux and macOS, both in
the network-access class, reached through `ratatui` → `ratatui-crossterm` → `crossterm`. `mio` resolves with `net`
enabled, which **compiles TCP and UDP socket types into two of the three release builds**. The observer opens no socket,
binds no port and resolves no name — it uses the poll to wait for terminal input and signals — and
`signal-hook-mio` carries no socket type of its own, matching only on the `mio` token.

**These crates were in the graph before `ADR-MOK-006` and nothing in this repository had looked.** The two available
shortcuts were both wrong: refuse every release, or trim the term list until it passed. What was built instead is a
disclosure mechanism — `SPEC-MOK-005` rule 8.4d and `SPEC-MOK-003`'s *Disclosed transitive capabilities* table — under
which a transitive prohibited-class name refuses until the specification records the crate, the chain, the activating
feature and what the package does not do with it, **and the technical owner judges it**. An implementation agent may
measure a transitive capability and may not accept one, so both rows were written `assessment OUTSTANDING`, in the text
the program itself parses. **The owner made the judgement on 2026-08-20 and both rows now carry it**: accepted, on the
three grounds `ADR-MOK-006` decision 4 prohibits *admitting* such a crate while these arrive transitively inside a graph
`ADR-MOK-003` accepted, and on the ground that no behaviour uses the socket types — **with the acceptance limited to a
compiled and uncalled capability and void the moment a behaviour calls it.** The program prints
`disclosed and accepted` and **still prints the row**: an acceptance that removed the line would be indistinguishable
from the disclosure never having existed, which is what
`test_an_accepted_disclosure_is_still_printed` asserts.

Matching is by token and not by substring: `name_tokens` splits on `-` and `_`. The near misses a substring rule would
have hit are retained (`ratatui*` containing `tui`, `windows-link` containing `ws`), along with a positive control.
`VER-MOK-013` states the blind spot beside the passing result: a crate can open a socket without saying so in its name.

## Manual assessments: six, none outstanding, and a numbering defect the owner resolved

`WO-MOK-013-manual-assessment.md` is the record. Summary:

| # | Assessment | Role | Status |
|---|---|---|---|
| 1 | No declared entry implements simulation semantics | technical owner | **Recorded 2026-08-20**, with a line for future entries: presenting a value the engine produced is admissible, computing the value is not |
| 2 | Decision 1's criteria applied to each entry | technical owner | **Recorded 2026-08-20** — satisfied, with dependency debt named as the yardstick |
| 3 | The reach of the by-name scan | assurance owner | **Recorded 2026-08-20** — the instrument's insufficiency stated, compensating controls named |
| 4 | The determinism baseline substitution | assurance owner | **Recorded**, and its precision correction now applied to both artifacts |
| 5 | — | — | **does not exist**, and the owner confirmed six is the intended set |
| 6 | The disclosed transitive capabilities | technical owner | **Recorded 2026-08-20** — accepted on the "admission, not arrival" basis, limited to a compiled and uncalled capability |
| 7 | The strength this change gives up | technical owner | Not yet due — triggers at the first admission beyond `ratatui` |

**`VER-MOK-013`'s *Manual assessments* list is numbered 1, 2, 3, 4, 6, 7. There is no assessment 5** — and `WO-MOK-013`
twice said *"seven"* and once *"Assessments 1 to 5 and 7"*, naming a fifth the contract does not contain. This was a
defect in artifacts the owner approved on 2026-08-20, found while writing the record. Renumbering was not available:
assessments 3, 4 and 6 are cited by number in **twenty** places across `SPEC-MOK-003`, `SPEC-MOK-005`, `VER-MOK-013`,
`WO-MOK-013` and `check_declared_dependencies.py`, each re-derived in the record. Inventing a fifth would have been
worse — it would create an owner obligation no approved text states.

**The owner resolved it on 2026-08-20: six is the intended set, and `WO-MOK-013`'s "seven" is the error.** The work order
is corrected to six in a dated *Lifecycle* note — it carries no *Amendment record* section, which the template confirms —
and the gap at 5 is preserved rather than closed, with `## 5. — does not exist` standing in the record so that a reader
counting six items against a list ending at 7 finds the reason rather than a missing file. Relatedly, `WO-MOK-013`'s
bullet *"The two manual assessments this change itself creates"* named only one; that bullet is rewritten in the same
note to state every assessment's disposition, so that **a reader can tell an acceptance from a silence**.

**Two assessments were outstanding beyond what the work order anticipated** — it anticipated only 6 — and both were put
to the owner with the candidate wording written out in full, and recorded in the owner's chosen text:

- **Assessment 2**: no approved text applied decision 1's criteria to `ratatui`. `ADR-MOK-006` decision 2 says the pin
  *"is unaffected"* and `ADR-MOK-003`'s note says the pin is *"untouched"* — both statements that the criteria were
  **not** applied. `VER-MOK-013` asks the owner to record whether `ADR-MOK-003` satisfies them retrospectively *"rather
  than assuming a pinned crate is grandfathered"*. **Recorded as satisfied**, with dependency debt named as the
  yardstick the criteria are measured against rather than as a bare assertion of compliance.
- **Assessment 3**: the term list postdates the approval that would otherwise have discharged it. It lives in code
  written during this work order, no approved artifact enumerates it, and the implementation agent cannot assess its own
  instrument — the same shape as `VER-MOK-011`'s still-outstanding fifth. **Recorded with the instrument's insufficiency
  stated as a limit rather than a caveat** — *"no passing scan is proof of absence"* — and four compensating controls
  named behind it: the technical owner's per-entry judgement at admission, decision 11's review obligation, the offline
  build and test with a failing control, and rule 8.4d's disclosure obligation. The trigger is stated too: the term list
  is re-read at each admission rather than assumed complete.

Assessments 1, 2 and 3 are the owner's own words, quoted verbatim as blockquotes in the record; the only change is
typographic. The provenance paragraph there states the mechanism — the agent measured, then put each open judgement as a
question with the full candidate text, and the owner chose. Assessment 6 was recorded as a decision between stated
bases, and its text in `SPEC-MOK-003` was written by the agent from that decision and is marked as such. Assessment 4
was recorded by the approval of 2026-08-20 on the reading the record states.

**No manual assessment of `VER-MOK-013` is outstanding.** Assessment 7's trigger has not occurred, which under the
contract's unqualified sentence would make it permanently unsatisfiable; that tension is flagged in the record for the
owner and is not resolved by an agent. Whether `VER-MOK-013` is satisfied is the assurance owner's decision, not this
report's.

## The determinism baseline was substituted, and why

`ADR-MOK-006` and `VER-MOK-013` name the four replay hashes retained under `VREC-MOK-002` in
`evidence/WO-MOK-002/determinism-and-resilience.md` as oracle 3's baseline. **They cannot serve.**
They were taken on 2026-08-17 at commit `68163ac452619e2f8d5a05ed3a73d42b920ba5f6`; since then `WO-MOK-010` added the
`fear` trait and `WO-MOK-011` added `name:` to the text record. Both change the bytes of every replay **by design** and
both were verified as doing so, so comparing this candidate against those hashes would report a mismatch on all 90 cells
and the mismatch would be those two approved changes, not this one.

`evidence/WO-MOK-011/post/post-manifest.txt`, bound by `VREC-MOK-011`, is substituted. `VREC-MOK-002` is **not edited**,
and neither is the evidence it binds: the four were correct at the commit that record names, and it stays bound to the
content it was written for.

| seed 123, 1,000 ticks, trace off | 2026-08-17 | candidate, twice each |
|---|---|---|
| reference `0.75%` | `97e0581c…` | `cebe44c4…` |
| reference `1.50%` | `58b7edc1…` | `9621f5f8…` |
| baseline `0.75%` | `82aa98b3…` | `fcd03d6f…` |
| baseline `1.50%` | `85f052bb…` | `44a448a1…` |

All four differ from 2026-08-17 and all four equal `WO-MOK-011`'s retained capture, as do all 90 manifest cells. The
substitution is `VER-MOK-013` manual assessment 4, the assurance owner's.

**One precision correction, now applied.** `VREC-MOK-002.md` contains **no replay hash** — its only SHA-256 is
`artifact_snapshot_sha256`, its own figures are survivor counts, and it names none of the four configurations. The four
hashes are in `evidence/WO-MOK-002/determinism-and-resilience.md:12–15`, which the record binds through `evidence_paths`.
*"Retained under `VREC-MOK-002`"* is right; *"`VREC-MOK-002`'s four replay hashes"* is loose. **The owner decided on
2026-08-20 to correct both artifacts**, and both are corrected: `ADR-MOK-006`'s first *Negative* consequence and
`VER-MOK-013`'s oracle 3 now name the evidence file, each recording the correction in a dated entry — a second *Status*
note in the ADR and a new amendment row in `VER-MOK-013` — that states no decision, check, oracle, pass condition or
figure changes with it. The three other references in the ADR that already read *"retained under"* are named as
unchanged, at lines 158, 514 and 660. `VREC-MOK-002` itself is **not edited**, and neither is the evidence it binds: the
four were correct at the commit that record names. The substitution is unaffected; what changes is that the owner
judging it is pointed at the file that holds them.

## Every reach beyond what the ADR enumerated

`WO-MOK-013-amendments.md` locates all sixteen required amendments and lists these separately, because an
implementation that quietly widened its own mandate is the failure mode that list exists to expose. Each is also
disclosed in the amendment row that carries it.

- `SPEC-MOK-003`'s *Disclosed transitive capabilities* section and security bullet — forced by the `mio` hits.
- `SPEC-MOK-005` rule 8.4d's disclosure mechanism as a provision.
- `SPEC-MOK-002` rule 13's mechanical reading convention, which the program depends on.
- Three `specifies` relations to `REQ-MOK-047`, from `SPEC-MOK-002`, `SPEC-MOK-003`, `SPEC-MOK-005`.
- Further clauses in `SPEC-MOK-004` and `ARCH-MOK-001` where a restatement of the empty-table premise would have
  outlived it.
- The root `Cargo.toml` and `rust-toolchain.toml` comments, which the ADR's source-comment entry does not name.
- `VER-MOK-013`'s own count corrected from fifteen to sixteen required amendments.
- Three provenance corrections for the build-script figure.
- The `VREC-MOK-002` precision correction in `ADR-MOK-006` and `VER-MOK-013`, which the owner decided on 2026-08-20 —
  and the same phrase in `WO-MOK-013`'s *Required verification* and completion-report format, which is a **reach beyond
  what that answer enumerated**: the owner was asked about the ADR and the contract, and the work order carries the
  loose phrase too. Disclosed in the work order's own dated note as well as here.
- The two `SPEC-MOK-003` *Assessment* cells and its security bullet rewritten to carry the acceptance of manual
  assessment 6, with `SPEC-MOK-005` rule 8.4d's disclosure wording following, and `check_declared_dependencies.py`
  changed to print `disclosed and accepted` while **still printing the row** — a program change and a new test
  (`test_an_accepted_disclosure_is_still_printed`) that no approved text asked for, and whose reason is that a removed
  line is indistinguishable from a disclosure that never existed.

## Where a declared figure is duplicated

Neither workflow and no part of `check_declared_dependencies.py` names a declared crate, version or figure, so an
approved amendment reaches both placements with no edit. **Two places do hold a copy:**

1. **`scripts/test_check_declared_dependencies.py`** — its reading tests assert today's values against the real
   specifications, so an admission that left them untouched would fail the suite. Its docstring says so, and calls that
   the intended reminder rather than an obstacle. It is the one file in this repository an amendment must also edit.
2. **`docs/engineering/REPOSITORY_CONTEXT.md`** — names `ratatui 0.30.2`, its three features and 57/63/62, in the same
   sentence as the specification that declares them. No authority, but a copy, and copies go stale.

## What was not done, and what is outstanding

**Not authorized and not performed:** commit, push, tag, pull request, or transitioning `WO-MOK-013` to `implemented`.

**Deliberately out of scope**, each considered and declined by the ADR: advisory scanning, licence policy, `cargo deny`,
`cargo audit`, vendoring, a crate-count ceiling, a numeric dependency-debt threshold, and adding `cargo fmt`, `cargo
clippy` or the test suite to pull-request CI. A check this work order did not build is not a check this repository has.
`REQ-MOK-047` states expressly that declared-set conformance is **not** supply-chain assurance.

**Outstanding, owed by an accountable role:**

1. **No manual assessment of `VER-MOK-013` is outstanding.** Assessments 1, 2, 3 and 6 were recorded by the owner on
   2026-08-20 and 4 stands as it did; 7's trigger has not occurred. Whether that satisfies `VER-MOK-013`, and how its
   unqualified sentence reads against a not-yet-due assessment, is the assurance owner's to decide.
2. The numbering defect is **resolved**: the owner decided six is the intended set and `WO-MOK-013`'s *"seven"* the
   error, and the work order is corrected with the gap at 5 preserved.
3. Whether `VER-MOK-005` and `VER-MOK-008` are reassessed, now that `REQ-MOK-026` and `REQ-MOK-036` have moved beneath
   them — the two new `W-HEX-003` warnings that name them. **The owner chose on 2026-08-20 to leave both for a separate
   change**, so neither is edited here and both stay owed.
4. A verification record binding this work order to its merge commit, once there is one.

**Standing from earlier work, untouched, neither cleared nor inherited:** the OUTSTANDING amendment rows on
`ARCH-MOK-001`, `SPEC-MOK-002` (two) and `SPEC-MOK-003` (one) dated 2026-08-18 and `SPEC-MOK-004` dated 2026-08-19, and
`VER-MOK-011`'s manual assessment 5. Every pre-2026-08-20 amendment row in the six amended artifacts is byte-identical
to `ff3a155`, checked row by row.

## Security and privacy

`ADR-MOK-006` decision 4 preserves every prohibition of `ADR-MOK-001`'s trust envelope without relaxation: no network
access, no API credential, **no secret in the repository**, no asynchronous runtime, no database, no plugin system, no
dependency-injection container, no model provider, and every user-interface dependency confined to the observer
package. What changes is that they are now **checked by name and by declaration** rather than inferred from an empty
table — and the scan's insufficiency is stated where it is used rather than where it flatters.

No credential, secret, token or environment value is read by any check, and none appears in retained evidence. Every
command is offline. Retained evidence contains dependency graphs, feature sets, hashes and simulation output — no
secret and no personal data. **The honest cost, in the ADR's own words:** the supply-chain surface of the engine package
changes from zero to whatever is declared, and the repository gives up a fact about a manifest in exchange for a check
that a reviewer, a script and a release gate have to keep running.
