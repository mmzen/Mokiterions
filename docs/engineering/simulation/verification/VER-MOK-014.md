+++
id = "VER-MOK-014"
type = "verification"
title = "Declared dependency set verification: what each package resolves, at which features, and what no graph read can answer"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-20"
updated = "2026-08-20"

[relations]
verifies = ["REQ-MOK-050"]
+++

# Verification Contract: Declared dependency set verification

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-20 | Original approved content for `REQ-MOK-050`, required by `ADR-MOK-006`. | Approved 2026-08-20 by the repository owner acting as accountable assurance owner, by way of `ADR-MOK-006`, whose *Required amendments* section states this contract's minimum content in full. Written under `WO-MOK-014`; the implementation agent wrote the text and took the measurements, and decided neither. |
| 2026-08-20 | **Four manual assessments recorded; no check, oracle or pass condition changed.** This contract was approved with assessments 1, 2, 3 and 6 all unrecorded. The repository owner recorded them later the same day — 1, 2 and 6 as accountable technical owner, 3 as accountable assurance owner — and the statements of *current state* in this contract are corrected to match: the matrix rows for 1, 2 and 6, the four entries in *Manual assessments*, the *Security and privacy* bullet, the residual-uncertainty bullet on disclosed capabilities, and acceptance scenario 11, which now requires a reader to be able to tell an acceptance from a silence. Assessment 6 is **accepted**, limited to a compiled and uncalled capability. **Assessments 4 and 7 are unchanged and 7 is not yet due**, and no outstanding item in any other artifact is cleared. The assessments themselves are in `evidence/WO-MOK-014/WO-MOK-014-manual-assessment.md`; this row records only that the contract no longer says they are absent. | Approved 2026-08-20 by the repository owner acting as accountable assurance owner. The assessments recorded were made by the repository owner in the roles named above; the implementation agent wrote this text and made no judgement. |
| 2026-08-20 | **One reference corrected; no rule, check or figure changed.** Five statements in this contract said *`VREC-MOK-002`'s four replay hashes* or *the four configurations `VREC-MOK-002` names*. That record carries no replay hash and names no configuration — it carries its own `artifact_snapshot_sha256` and binds the evidence, and the four hashes are in `evidence/WO-MOK-002/determinism-and-resilience.md` at lines 12 to 15. Each statement now reads *retained under `VREC-MOK-002` in `evidence/WO-MOK-002/determinism-and-resilience.md`*. The baseline substitution of oracle 3, the four `WO-MOK-011` hashes and every pass condition are untouched; what is corrected is where a reader is sent to find the superseded figures. `ADR-MOK-006` carries the same correction as a dated note in its *Status*. | Approved 2026-08-20 by the repository owner acting as accountable assurance owner. |

## Independence

The change this contract verifies adds no Rust code. It replaces a structural property — *the engine's dependency table
is empty* — with a comparison, and the risk it carries is not that the comparison is wrong today but that it is
**weaker than the property it replaced while reading as though it were not**. A contract written by the effort that
made the change is the wrong witness to that. Three habits of this repository are therefore reversed here on purpose:
the checks are anchored to artifacts that existed before the change, the by-name scan is stated as insufficient rather
than as a check, and two of the obligations are recorded as judgements with no automated companion at all.

Five independent oracles are used.

1. **The resolved graph, read from Cargo and not from a manifest.** Every set claim is measured with
   `cargo tree -p <package> -e normal --locked --offline --target <triple> --prefix none --no-dedupe` and with
   `cargo metadata --locked`, never by reading `[dependencies]` and never by reading `Cargo.lock` as though it were a
   package's graph. A manifest read cannot see a transitive addition and a lockfile read cannot see that a crate in it
   resolves into no build of either package. The distinction is not academic in this repository: `syn 1.0.109` and
   `thiserror 1.0.69` are in `Cargo.lock`, are unreachable from either package at any edge kind including
   `--target all`, and one of them carries a build script. A check that read the lockfile would attribute a build-time
   code-execution surface to a package that never executes it.

2. **The declaration, read from the specification text.** The expected set comes from `SPEC-MOK-002` rule 13 and
   `SPEC-MOK-003`'s *Declared dependency set*, parsed from those documents. It is not restated in a script, a workflow
   or a fixture. A second copy of the declaration would be a third source of truth and would pass while disagreeing
   with the approved artifact, which is the specific failure `SPEC-MOK-005` rule 15.5 forbids.

3. **A retained pre-existing capture as the determinism baseline.** `ADR-MOK-006` names the four replay hashes retained
   under `VREC-MOK-002` in `evidence/WO-MOK-002/determinism-and-resilience.md` for this — the record itself carries no
   replay hash, only its own `artifact_snapshot_sha256`, and the four sit in the evidence file its `evidence_paths`
   binds, at lines 12 to 15. **They cannot serve, and this contract says so rather than reporting a false failure.**
   They were
   taken on 2026-08-17 at commit `68163ac452619e2f8d5a05ed3a73d42b920ba5f6`, before `WO-MOK-007` added `fear` and
   `WO-MOK-011` added `name:` to the event stream under approved amendments, so every one of the four differs from what
   the engine prints today for reasons that have nothing to do with a dependency. The baseline used instead is the
   newest retained capture of the same configurations,
   `evidence/WO-MOK-011/post/post-manifest.txt`, which `VREC-MOK-011` binds, and the four cells are named in the matrix
   below. `VREC-MOK-002` is **not edited**, and neither is the evidence it binds: the four were correct at the commit
   that record names, and this paragraph records why a later tree does not reproduce them.

4. **An independently written prohibition list.** The names the scan of `SPEC-MOK-005` rule 8.4d looks for are written
   from `ADR-MOK-006` decision 4's capability classes, not from the crates that happen to be present. A list derived
   from the current graph would pass by construction on the day it was written and would never fail afterwards.

5. **The governance state of what this change amends.** The sixteen entries of `ADR-MOK-006`'s *Required amendments*
   section must be present, and each of the twelve that lands in a governed artifact must carry its approval, before
   this contract can be satisfied; the two checks that replaced the empty-graph inference must be present in
   `ARCH-MOK-001`. Their absence fails this contract regardless of the state of the code, for the reason `VER-MOK-006`
   gives: an amendment nobody approved is not a specification. The count is sixteen and not fifteen, which this
   contract said when it was drafted: the entries were counted rather than carried over, and the four that carry no
   approval — `REPOSITORY_CONTEXT.md`, `release.yml`, the new pull-request workflow and the source comments — are
   checked for presence and consistency, since the ADR states that none of them carries authority.

The declared verification seed set is `0`, `1`, `42`, `123` and `777`, fixed by `VER-MOK-002` and reused unchanged. The
three targets are `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu` and `aarch64-apple-darwin`, which are the three
`SPEC-MOK-005` rule 10 builds.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-050` | static-analysis | **Declared-set equality, engine package, both directions** | Every direct dependency and dev-dependency entry of `mokiterions-core/Cargo.toml` is an entry of `SPEC-MOK-002` rule 13, and every entry of rule 13 is in that manifest. Measured 2026-08-20: both sides empty, `cargo tree -p Mokiterions -e normal --locked --offline` prints one crate on all three targets |
| `REQ-MOK-050` | static-analysis | **Declared-set equality, observer package, both directions** | Every direct entry of `mokiterions-tui/Cargo.toml` other than the `Mokiterions` path dependency is an entry of `SPEC-MOK-003`'s declared set, and every entry of that set is in the manifest. Measured 2026-08-20: one entry each side, `ratatui 0.30.2` |
| `REQ-MOK-050` | static-analysis | Set equality fails in the undeclared direction | An entry added to either manifest and to no declared set fails the check. Demonstrated by injection, not by inspection (scenario 2) |
| `REQ-MOK-050` | static-analysis | Set equality fails in the missing direction | A declared entry removed from a manifest fails the check. Demonstrated by injection (scenario 3) |
| `REQ-MOK-050` | static-analysis | Version match | Each entry resolves at the declared version exactly. A range where a version is declared, or a resolved version other than the declared one, fails |
| `REQ-MOK-050` | static-analysis | **Feature match, including negatives and the implied feature** | The manifest declares `default-features = false` and exactly `crossterm`, `layout-cache`, `underline-color`; `ratatui` resolves with those three and `std`, which the declaration records as implied, and with `default` and `serde` **absent**. Equality is against declared-plus-implied, so a feature arriving by unification fails and an implied feature is not a loophole. `SPEC-MOK-002` rule 13 fixes how the cell is read |
| `REQ-MOK-050` | static-analysis | The set is the resolved graph, not the lockfile | The check reads `cargo tree`/`cargo metadata` resolution and not `Cargo.lock` as a package's set. Evidence records both figures, the **182**-against-**59** gap, and the two unreachable lockfile crates by name |
| `REQ-MOK-050` | static-analysis | **Per-target resolution** | The comparison runs for all three release targets. Recorded external-crate counts for the observer: 57, 63, 62, and 66 in union. A check run on one target only does not satisfy this row |
| `REQ-MOK-050` | static-analysis | **Trust envelope, by name** | No crate in either package's resolved graph is a network, asynchronous-runtime, database or model-provider crate. Recorded as a named scan over the union graph of all three targets, with the search terms retained |
| `REQ-MOK-050` | review | The scan's insufficiency is stated where it is used | The retained scan output states that it is by name and not exhaustive, and points at manual assessment 3. A scan presented as conclusive fails this row even when it passes |
| `REQ-MOK-050` | static-analysis | **A prohibited-class name in a declared entry refuses** | The check refuses with no exception available, per `ADR-MOK-006` decision 4. Demonstrated by injection, by declaring such a crate and finding no disclosure able to admit it |
| `REQ-MOK-050` | static-analysis | **A transitive prohibited-class name refuses until disclosed** | An undisclosed transitive hit refuses; a hit disclosed in the declaring specification, with crate, version, targets, chain, activating feature and non-use, passes. Demonstrated in both states |
| `REQ-MOK-050` | review | **The two disclosed hits are the measured ones and no more** | `mio 1.2.2` and `signal-hook-mio 0.2.5`, on Linux and macOS only, with the chain and the `net` feature recorded. A disclosure table with an entry the graph does not reach fails this row as surely as a missing one |
| `REQ-MOK-050` | manual | **The disclosed transitive capabilities are accepted or refused** | Manual assessment 6, by the technical owner. Unrecorded is **OUTSTANDING**. It was outstanding as this contract was approved and was **recorded later the same day**: accepted, on the grounds `SPEC-MOK-003`'s disclosure table states |
| `REQ-MOK-050` | static-analysis | **User-interface confinement** | No user-interface crate is in the engine package's resolved graph, on any target, and the engine does not depend on `mokiterions-tui` directly or transitively |
| `REQ-MOK-050` | static-analysis | **Offline resolution, build and test of the engine** | `cargo build -p Mokiterions --locked --offline` and `cargo test -p Mokiterions --locked --offline` succeed with no package-registry access, from the committed lockfile, with no lockfile modification |
| `REQ-MOK-050` | static-analysis | No check modifies a tracked file | `git status --porcelain` is empty after every check in this contract, and `Cargo.lock` is byte-identical before and after. A check that updated the lockfile would have released a different commit than it verified |
| `REQ-MOK-050` | static-analysis | **Build-script status matches the declaration** | The set of crates in each target's resolved graph carrying a `build.rs` equals what `SPEC-MOK-003`'s declared set records for that target: 7 on Windows, 9 on Linux, 9 on macOS, 10 in union. A crate that gains or loses one is a mismatch |
| `REQ-MOK-050` | automated-test | **Determinism, byte-identical replay** (oracle 3) | At seed `123`, 1,000 ticks, the four configurations retained under `VREC-MOK-002` in `evidence/WO-MOK-002/determinism-and-resilience.md` each reproduce byte-identically across two runs, and each equals the raw hash retained in `evidence/WO-MOK-011/post/post-manifest.txt`: `reference 0.75%` `cebe44c4…`, `reference 1.50%` `9621f5f8…`, `baseline 0.75%` `fcd03d6f…`, `baseline 1.50%` `44a448a1…` |
| `REQ-MOK-050` | automated-test | Determinism across the declared seeds | Byte-identical output across two runs at each of the five declared seeds under each of the three policies, and equality with the corresponding cell of the same retained manifest |
| `REQ-MOK-050` | review | **Why the hashes retained under `VREC-MOK-002` are not the baseline** (oracle 3) | The retained evidence states the two approved amendments that moved the stream, names the record that binds the replacement baseline, and shows the four current hashes differing from the four 2026-08-17 hashes. An unexplained substitution of a baseline fails this row |
| `REQ-MOK-050` | manual | **No declared entry implements simulation semantics** | Manual assessment 1, by the technical owner. Unrecorded is **OUTSTANDING** and leaves this contract unsatisfied. **Recorded 2026-08-20**, with the line for future entries stated: a declared crate may compute the presentation of a value the engine produced and may not compute the value |
| `REQ-MOK-050` | manual | **The criteria of `ADR-MOK-006` decision 1 were applied to each entry** | Manual assessment 2, by the technical owner. Unrecorded is **OUTSTANDING**. **Recorded 2026-08-20**, applied to `ratatui 0.30.2` retrospectively and not grandfathered, with the measured debt named as the yardstick for the next admission |
| `REQ-MOK-050` | static-analysis | **Required amendments present and approved** (oracle 5) | All sixteen entries of `ADR-MOK-006`'s *Required amendments* section are present, the twelve that land in a governed artifact carry their approval, and `ARCH-MOK-001` carries both replacement conformance checks. Absence fails this contract regardless of code state |
| `REQ-MOK-050` | static-analysis | Both placements exist and read one declaration | The release gate and the pull-request workflow both run rule 8.4's checks, and neither restates a declared set. Two implementations that could disagree about what is declared fail this row |
| `REQ-MOK-050` | static-analysis | The managed workflow is untouched | `.github/workflows/engineering-harness.yml` is byte-identical to its locked digest and `python -m se_harness doctor` reports no integrity finding |

## Acceptance scenarios

1. A reviewer reads `SPEC-MOK-002` rule 13 and `SPEC-MOK-003`'s declared set on paper, runs the three-target
   comparison, and finds the manifests and the tables agreeing in both directions with nothing left over.
2. A reviewer adds a real crate to `mokiterions-core/Cargo.toml` without amending rule 13, runs the check, and finds it
   refuse and name the undeclared entry. They then add the same crate to the observer instead and find it refuse again.
   A check that cannot be made to fail has not been demonstrated to work.
3. A reviewer deletes the `ratatui` entry from the observer's manifest, runs the check, and finds it refuse for the
   opposite reason — a declared entry that is not present — rather than passing because the graph shrank.
4. A reviewer enables `serde` on `ratatui`, runs the check, and finds the feature audit refuse. They then enable a
   feature through unification from a second crate rather than in the manifest, and find it refuse for the same reason.
5. A reviewer runs the engine's build and test with the network disabled and no registry cache reachable, and both
   succeed from the committed lockfile.
6. A reviewer runs the comparison on each of the three release targets and finds the recorded per-target figures — 57,
   63 and 62 external crates for the observer, one crate for the engine — reproduced.
7. A reviewer counts the crates carrying a build script on each target and finds 7, 9 and 9, matching the declared
   table's target column crate for crate.
8. A reviewer runs the four seed-`123` configurations twice each, hashes the full output, and finds the four hashes
   equal to the raw hashes retained under `WO-MOK-011` — then runs them against the four retained under
   `VREC-MOK-002` in `evidence/WO-MOK-002/determinism-and-resilience.md` and finds all four differ, in the fields the
   two approved amendments added and nowhere else.
9. A reviewer opens a pull request that adds an undeclared dependency and finds the repository-owned workflow refuse
   it, with the managed harness workflow reporting no tampering.
10. A reviewer runs the scan on Linux, reads the raw hits before the disclosure is applied, and finds exactly `mio` and
    `signal-hook-mio`. They then delete one row from `SPEC-MOK-003`'s disclosure table and find the check refuse, and
    delete a term from the scan's list and find the raw hits shrink — which is how they establish that the pass is a
    disclosure and not an omission.
11. A reviewer reads the technical owner's recorded assessments and finds each naming the crate it judged. With no crate
    declared beyond `ratatui`, they find the assessments stating that, rather than stating nothing, and they find
    assessment 6 **recorded** rather than absent — accepted or outstanding, and if accepted then with its grounds and
    with the limit of the acceptance stated, so that a reader can tell an acceptance from a silence.
12. A reviewer greps the release workflow and the pull-request workflow for a crate name, a version or a feature name
    and finds none: both read the declaration from the specifications.

## Property and invariant tests

- **Set equality is symmetric.** The comparison fails on an undeclared entry and on a missing declared entry. A check
  that only detects additions satisfies half of `REQ-MOK-050` and this property is what distinguishes them.
- **The declaration is target-independent and its resolution is not.** The same declared set holds on all three
  targets; the resolved graph does not, and no figure in this contract is stated without its target.
- **Feature negatives are asserted, not assumed.** `serde` being off is a positive assertion about an absence, checked
  against the resolved feature set rather than against the manifest text.
- **`--locked` everywhere.** No check in this contract may resolve a version the committed lockfile does not fix, and
  no check may write it. Both halves are asserted.
- **Determinism is a property of the run, not of the dependency count.** Byte-identical replay holds at every declared
  seed under every policy, exactly as `REQ-MOK-009` requires, and is re-derived rather than inherited.
- **The engine's graph contains no user-interface crate on any target.** Asserted per target, because a crate can be
  conditional on one.
- **One declaration, two readers.** The release gate and the pull-request workflow produce the same verdict on the same
  commit, by construction rather than by comparison.

## Static and architecture checks

- `cargo fmt --all -- --check` clean.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` clean, with no `allow` added.
- `cargo test --workspace --locked` clean, every tier of both packages in one invocation, with no terminal present.
- `cargo tree -p Mokiterions -e normal --locked --offline` resolves to the engine package alone, on each target, which
  is what an empty declared set predicts rather than what a rule requires.
- `cargo tree -p mokiterions-tui -e normal --locked --offline` resolves the observer, the engine and the `ratatui`
  graph, at the counts the declared set records for each target.
- The engine package does not depend on `mokiterions-tui`, directly or transitively, on any target.
- `python scripts/validate_engineering_artifacts.py` clean, and `python -m se_harness doctor` reporting no integrity
  finding against `.engineering-harness.lock`.
- `ARCH-MOK-001` carries the two conformance checks that replace the empty-graph inference, and `ARCH-MOK-002` carries
  the per-package comparison. Read as text, because their absence is the failure mode this oracle exists for.
- No declared set appears in a script, a workflow or a fixture. Checked by searching both workflows for every crate
  name, version and feature name in either declared set.

## Security and privacy checks

- **Every prohibition of `ADR-MOK-001`'s trust envelope is checked rather than inferred.** No network access, no API
  credential, no secret in the repository, no asynchronous runtime, no database, no plugin system, no
  dependency-injection container and no model provider in either package. `ADR-MOK-006` decision 4 preserves all of
  them and relaxes none, and this contract is where the inference an empty table used to supply is replaced.
- **The by-name scan is not a security control on its own.** It is a filter with a known blind spot: a crate can open a
  socket without saying so in its name, and a transitive crate can do it without appearing in any declaration. This is
  stated in the retained evidence beside the passing result, and manual assessment 3 is what stands behind it.
- **The scan's two hits are disclosed rather than filtered.** `mio 1.2.2` with `net` enabled, and `signal-hook-mio
  0.2.5`, are in the observer's Linux and macOS graphs through `crossterm`'s event polling. They are recorded in
  `SPEC-MOK-003`'s *Disclosed transitive capabilities* table with the chain that reaches them and the behavior that does
  not use them, and manual assessment 6 is **recorded**: the technical owner accepted both on 2026-08-20, on the ground
  that decision 4 prohibits *admitting* such a crate while these arrive transitively inside a graph `ADR-MOK-003`
  accepted, and that no behavior uses the socket types. **The acceptance covers a compiled and uncalled capability and
  is void if a behavior calls it.** Verifying this contract includes verifying that the term list was not written around
  them: the retained evidence shows the raw hits before the disclosure is applied.
- **The build-time code-execution surface is enumerated.** Seven crates on Windows, nine on Linux, nine on macOS execute
  a `build.rs` during a build of this repository. That was true before `ADR-MOK-006` and unwritten; the declared set
  records it, and a change to the set is a check failure rather than an unremarked change.
- No credential, secret, token or environment value is read by any check in this contract, and none appears in retained
  evidence. Every command is offline; the only reason a check would reach a network is a registry access, and 8.4c
  asserts that it does not.
- Retained evidence contains dependency graphs, feature sets, hashes and simulation output. No secret and no personal
  data.
- **Advisories and licences are expressly out of scope**, as `REQ-MOK-050` states. No `cargo audit`, no advisory
  database and no licence scan is part of this contract, and a passing result here says nothing about either. This is
  recorded so a reader does not mistake declared-set conformance for supply-chain assurance.

## Performance and resilience checks

- The comparison is bounded by the resolved graph: 66 crates in union today, three targets, two packages. It adds no
  per-tick work and touches no simulation path.
- Every check is offline and reads the committed lockfile, so neither the release gate nor the pull-request workflow
  depends on registry availability. A registry outage cannot turn a passing commit into a failing one.
- The engine's 1,000-tick run at the default density stays within the bound `SPEC-MOK-001` states, on each declared
  seed. Re-derived rather than assumed, because the point of this contract is that a dependency could move it.
- A check that cannot resolve a target fails loudly. A silently skipped target would report success over a graph it
  never read, which scenario 6 is written to catch.

## Manual assessments

Each of the following is an explicit judgement recorded by the accountable role. An unrecorded assessment is an
outstanding assessment, and this contract is not satisfied while any remains outstanding — as `VER-MOK-011`'s fifth
still is.

1. **No declared entry implements simulation semantics, by the technical owner.** `ADR-MOK-006` decision 11 reserves
   the proprietary core: no crate in either declared set implements the rules `SPEC-MOK-001` fixes, the world model or
   agent decision-making, owns or advances entropy, or performs action validation. No graph read answers this. The
   owner records the judgement per entry. **With one entry declared, the assessment is about `ratatui`: a terminal
   rendering library that computes no simulation value.** The assessment is recorded even so, because an assessment
   that is only made when it is difficult is a habit nobody has. **Recorded 2026-08-20**, in
   `evidence/WO-MOK-014/WO-MOK-014-manual-assessment.md`, and it states the line for future entries as well as the
   judgement on this one.
2. **The criteria of decision 1 were applied to each entry, by the technical owner.** Stable, well-maintained
   functionality that accelerates delivery without excessive dependency debt, and a proven solution for a standard,
   non-core feature. There is no numeric threshold, by decision 10, so this is a judgement and the owner records it per
   crate. For `ratatui` the record is `ADR-MOK-003`, which decided it before these criteria existed; the owner records
   whether that decision satisfies them retrospectively rather than assuming a pinned crate is grandfathered.
   **Recorded 2026-08-20**: satisfied, with the dependency debt measured rather than asserted — 57, 63 and 62 crates,
   7, 9 and 9 build scripts, and the disclosed `mio` capability — and that measured surface named as the yardstick the
   next admission is judged against.
3. **The reach of the by-name scan, by the assurance owner.** The owner records that the scan's name list covers the
   capability classes decision 4 prohibits as far as a name can, states what it cannot see, and confirms that no
   passing scan is being treated as proof of absence. This assessment exists because the scan replaced an inference
   that was airtight and is not. **Recorded 2026-08-20**: the 126 terms reach the six prohibited classes as far as a
   name can, the insufficiency is stated as a limit rather than a caveat, and the four compensating controls that stand
   behind the blind spot are named.
4. **The determinism baseline substitution, by the assurance owner.** The owner records that
   `evidence/WO-MOK-011/post/post-manifest.txt` is the correct baseline, that the four hashes retained under
   `VREC-MOK-002` in `evidence/WO-MOK-002/determinism-and-resilience.md` describe a stream two approved amendments have
   since changed, and that the substitution is a correction of a stale reference and not a relaxation of the comparison.
6. **The disclosed transitive capabilities, by the technical owner.** `SPEC-MOK-003`'s *Disclosed transitive
   capabilities* table records `mio 1.2.2` — reached on Linux and macOS through `ratatui` → `ratatui-crossterm` →
   `crossterm`, with the `net` feature enabling TCP and UDP socket types — and `signal-hook-mio 0.2.5` beside it. The
   observer opens no socket, and the capability is nevertheless compiled into two of the three release builds. The owner
   records whether that is accepted, and on what basis: that `ADR-MOK-003` accepted this graph in 2026-08-17, that
   decision 4's prohibition is on *admitting* a crate rather than on a transitive arrival, that no behavior uses it, or
   that it is not accepted and `crossterm`'s event source must change. **This assessment was OUTSTANDING as this
   contract was approved, and was recorded later the same day: accepted**, on the first three of those grounds together
   — `ADR-MOK-003` accepted the graph, decision 4's prohibition is on admitting, and no behavior uses the socket types —
   **with the acceptance limited to a compiled and uncalled capability and void the moment a behavior calls it.** The
   grounds are in `SPEC-MOK-003`'s disclosure table, which is where a reader looking at the crate will land. It is the
   one assessment here that the implementation produced rather than anticipated: the crates
   were in the graph before `ADR-MOK-006` and nothing in this repository had looked. An implementation agent may
   measure a transitive capability and may not accept one, which is why this row exists instead of a term list with
   `mio` left out of it.
7. **The strength this change gives up, by the technical owner.** `ADR-MOK-006`'s first *Negative* consequence states
   that the repository loses a structural property in exchange for a check. The owner records, at the first admission
   of any crate beyond `ratatui`, that the check has in fact been running — in both placements, on all three targets —
   rather than discovering at admission time that one placement had been failing silently.

## Evidence retention

Retained under `docs/engineering/simulation/evidence/WO-MOK-014/`, in files named `WO-MOK-014-*`:

- the per-target resolved graphs for both packages, as recorded command output with the command line and the toolchain
  version, for all three release targets and for `--target all`;
- the declared-set comparison for both packages in both directions, with the parsed declaration beside the resolved
  graph so a reader can see which document each side came from;
- the `cargo metadata --locked` package count beside the resolved-graph counts, and the two lockfile crates that
  resolve into no build, with the `cargo tree -i` output showing nothing to invert;
- the resolved feature set for every entry, including the negative assertions;
- the build-script crate set per target, with the `custom-build` target detection recorded, and its comparison against
  the declared table;
- the by-name scan: the search terms, their provenance in `ADR-MOK-006` decision 4, the **raw hits before any
  disclosure is applied**, the disclosed set they are matched against, the residue, and the statement of what the scan
  cannot see. The raw hits are retained precisely so a later reader can see that the term list was not written around
  the graph;
- the engine's offline build and test output, with the registry unreachable, and `git status --porcelain` empty
  afterwards with `Cargo.lock` byte-identical;
- the four seed-`123` replay hashes measured twice each, their equality with `WO-MOK-011`'s retained raw hashes, and
  their inequality with the four retained under `VREC-MOK-002` in
  `evidence/WO-MOK-002/determinism-and-resilience.md`, with the two amendments that explain the difference named;
- the declared-seed determinism sweep across the three policies;
- `cargo fmt`, `cargo clippy`, `cargo test`, the artifact validator, the dashboard and the harness doctor output;
- both workflow files as they stand, and the search showing neither restates a declared set;
- oracle 5's sixteen entries: the twelve amendment approvals, each located in the artifact that carries it, and the
  four unapproved entries recorded as present and consistent;
- the manual assessments above, each with its accountable role and date, and each outstanding one recorded as
  **OUTSTANDING** rather than omitted.

**What an admission must retain.** This contract is also the artifact that decides what evidence adding a crate
requires, since `ADR-MOK-006` admits none itself. An admission retains, in the work order that performs it:

1. the amendment row that added the entry, approved by the technical owner, in the declaring specification;
2. manual assessments 1 and 2 for that crate specifically, named, not carried over from a previous admission;
3. the crate's resolved version and feature set on each of the three targets, and its build-script status on each;
4. the resolved graph before and after, so the transitive addition is visible as a set difference rather than as a
   count;
5. a determinism re-derivation at the declared seeds under every policy, compared against the newest retained capture,
   with any difference explained by an approved amendment or treated as a defect in the admission;
6. the by-name scan over the new graph, and a statement of which prohibited capability classes the new crates could
   plausibly touch;
7. where the crate is a dev-dependency, the same evidence unreduced: decision 6 binds a test-only crate, and a flaky
   test in a repository whose figures are replay hashes is a determinism failure with a different name.

Evidence is retained in the repository, is reproducible from the recorded commands and commit, and contains no secret.

## Residual uncertainty

- **The replaced property was stronger, and no arrangement of checks here restores it.** *The table is empty* could not
  be satisfied by a crate that lied about itself. *The resolved set equals the declared set* can. `ADR-MOK-006` states
  this as its first negative consequence and this contract does not claim to have closed it.
- **Set equality is checked over direct declared entries, not over the transitive graph.** 66 crates across three
  targets cannot be enumerated in prose and kept true, so the transitive graph is held by `--locked`, by the
  build-script table and by the by-name scan. A crate that changed behaviour without changing its name, its version,
  its features or its build script would pass every check in this contract. This is the largest residual and it is
  named rather than mitigated.
- **The by-name scan is not exhaustive over the capability classes it searches.** Manual assessment 3 is what stands
  behind it, and a review is not a proof.
- **A disclosed capability is a recorded one, not a removed one.** `mio`'s `net` feature compiles TCP and UDP socket
  types into the Linux and macOS observer builds today. The disclosure makes it visible and the check makes it stable;
  neither makes it absent, and manual assessment 6 accepted the capability rather than removing it. A reader who takes a
  passing 8.4d as *no socket code in the binary* has read it wrong, and one who takes the acceptance that way has read it
  wrong twice: what is accepted is a compiled and uncalled capability, and the acceptance is void if a behavior calls it.
- **The scan found this by name, and it was luck that the name carried it.** `mio` is legible as an I/O crate. Nothing
  in this contract would have surfaced the same capability inside a crate named after what it renders, which is the
  blind spot manual assessment 3 exists for and the reason that assessment is not a formality.
- **Decision 11 is a judgement that will get harder.** *Does this crate implement simulation semantics* is easy for a
  terminal backend and debatable for a spatial index or a fixed-point arithmetic crate. The prohibition holds exactly
  as well as manual assessment 1, and assessments drift.
- **Determinism is verified over the declared seeds and policies, not over the input space.** A crate that introduced
  nondeterminism only at a density outside the sweep, or only past tick 1,000, or only on a platform not among the
  three, would pass. `ARCH-MOK-001`'s extended prohibition on admitted crates is a rule this contract cannot fully
  measure.
- **Per-target coverage is three targets, not every target Cargo can resolve.** `--target all` reaches 71 external
  crates for the observer and is recorded, but a crate conditional on a fourth platform is outside every figure here.
- **This contract verifies conformance to a declaration; it does not verify that the declaration is wise.** Whether a
  crate should be in the repository at all is decisions 1, 10 and 11 — three judgements by the technical owner — and no
  evidence here substitutes for one.
- **The two placements are verified to agree by construction, not by differential testing.** They read one declaration,
  which is why they cannot disagree about what is declared; nothing here proves they cannot disagree about what is
  resolved, for instance on a runner whose target differs from the one under test.
- **This contract does not close any outstanding item elsewhere.** `VER-MOK-011`'s fifth manual assessment, the four
  2026-08-18 amendment rows awaiting the technical owner, and the verification record owed against the merge commit are
  all untouched by it.
