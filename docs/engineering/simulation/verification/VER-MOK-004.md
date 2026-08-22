+++
id = "VER-MOK-004"
type = "verification"
title = "Help output content verification"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-17"
updated = "2026-08-22"

[relations]
verifies = ["REQ-MOK-018"]
+++

# Verification Contract: Help output content verification

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-22 | **Realigned to `REQ-MOK-018` as amended the same date, and three stale enumerations corrected.** The additions are in *Amendment of 2026-08-22* below: four matrix rows and two acceptance scenarios, covering the third clause of the amended statement, the retirement of the trailing prose, the two statements the text now makes about output and exit status, and the byte-identity of the four shared entries across the two targets. The corrections are stale counts this contract has carried since it was written for `WO-MOK-004`: "six options" and "all six" meant five options and `--help`, and `--policy`'s row named two accepted values when four have been accepted since 2026-08-20. **Nothing already in the matrix is withdrawn or weakened**, and the *Independence* section's hazard is unchanged and still the governing one: every check is written against a source outside `cli::USAGE`, and the new byte-identity check is written against the *other* target's constant rather than against a literal in a test file. The baseline figures for `WO-MOK-004` are historical and are not restated. | **Approved 2026-08-22 by the repository owner acting as assurance owner**, after the product owner's `REQ-MOK-018` amendment of the same date and not before it, since three of the four added rows verify clauses that amendment introduces. It was taken together with the engineering owner's transition of `WO-MOK-020` to `implemented`, in that order, because a work order cannot leave `draft` without assurance coverage of what it changed. `WO-MOK-020` states it in full in its *Required amendments* section. The alternatives put beside it were to approve these rows and leave that work order `draft`, and to decline the rows entirely — the second declined because the three stale enumerations are wrong today independently of this change, and leaving them would leave this contract asserting that `--policy` accepts two values. The implementation agent wrote the rows and the two scenarios; it decided no pass condition that the amended requirement does not already fix. |

## Independence

This contract faces the mirror image of `VER-MOK-003`'s hazard. That contract verified a change required to alter
nothing observable. This one verifies a change required to alter exactly one observable thing and nothing else, and
the specific danger is that the suite already contains the test a careless reviewer would reach for.

`tests/process.rs::help_exits_successfully` asserts that the help output equals `cli::USAGE`. That assertion is
true for every possible value of `USAGE`, including an empty string. It pins the routing and the exit code, which is
what it was written for, and it says nothing whatever about content. Any check written only against the constant is
a tautology. Every check below is therefore written against a source outside the constant, and every one of them can
fail.

Three independent oracles are used.

1. **The specification, transcribed without reading the code.** The six options, the five defaults, and the three
   value constraints are taken from `SPEC-MOK-001`'s *Inputs* and *Help output* sections into a table, and that
   table is diffed against the rendered help output. A default the specification declares and the help omits is a
   shortfall; a default the help states and the specification does not declare is a surplus; a stated value that
   differs from the declared one is a defect. All three are findings. This oracle is prepared from the
   specification, so it cannot be satisfied by making the text agree with the code.
2. **The program's applied configuration.** For each option with a default, the value the program actually applies
   when the option is omitted is obtained through `cli::parse` on an empty argument list, and compared against the
   value the help text states. This is the check that makes a hardcoded default safe: it fails if either side moves
   alone. It is verified by an executing test, not by review, because review cannot be re-run on a future change.
3. **A recorded pre-change baseline.** Output, exit codes, and standard-error text are captured from the program
   *before* the change, at the commit the work begins from, and compared afterwards. The baseline is evidence,
   captured once and never regenerated. Everything except the usage text must be byte-identical, and the usage text
   must change only in the way this contract specifies.

The recorded pre-change facts are fixed here so that they cannot be adjusted afterwards.

- **52 tests, 0 ignored**: 37 in `src/simulation.rs`, 0 in `src/lib.rs` and `src/main.rs`, and 15 in the public
  tier — 5 in `tests/cli.rs`, 4 in `tests/process.rs`, 2 in `tests/density.rs`, 3 in `tests/termination.rs`, and 1
  in `tests/viability.rs` — plus a doc-test pass reporting zero tests.
- The usage text is **12 lines, 10 of them non-blank**, and states **one of five** declared defaults, that of
  `--density`.
- The capture matrix is **43 cells** — 5 seeds × 2 decision sources × 2 declared densities × 2 trace settings at
  1,000 ticks, plus two 20-tick traced runs and the no-argument invocation — and **16 named cases** covering exit
  codes and standard error: the `--help` case, fourteen invalid-configuration cases, and one valid single-tick run.

The expected change surface against that baseline is stated in advance, and a difference outside it fails this
contract:

| Baseline artifact | Cells | Required result |
|---|---:|---|
| Matrix manifest and summary lines | 43 | Byte-identical: digest, line count, exit code, and full summary text |
| `--help` case | 1 | Exit code `0` and empty standard error unchanged; standard output is the new usage text |
| Invalid-configuration cases | 14 | Exit code `2` unchanged, standard output still empty, the `configuration error: …` line byte-identical, and the appended usage block replaced |
| Valid single-tick case | 1 | Byte-identical in every field |

This contract verifies content and the printed-to-applied equality. It does not verify column alignment, column
widths, or line wrapping, which `SPEC-MOK-001` leaves unspecified, and it asserts nothing about the wording of a
description beyond the substance the specification fixes and the readability assessment recorded below.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-018` | automated-test | An options entry exists for each of `--seed`, `--ticks`, `--policy`, `--density`, `--trace-actions`, and `--help` | All six present; none missing and no entry for an option the program does not accept |
| `REQ-MOK-018` | automated-test | `--seed` default | Help states `0`; `cli::parse` on an empty argument list yields `seed == 0`; the two agree |
| `REQ-MOK-018` | automated-test | `--ticks` default and constraint | Help states `100` and that the value must exceed zero; parse yields `tick_limit == 100`; the two agree |
| `REQ-MOK-018` | automated-test | `--policy` default and value set | Help states `reference` and names `baseline` and `reference`; parse yields `Policy::Reference`; the two agree |
| `REQ-MOK-018` | automated-test | `--density` default and constraint | Help states `0.75` and the two-decimal limit; parse yields `Density::DEFAULT`; the two agree |
| `REQ-MOK-018` | automated-test | `--trace-actions` | Help states that tracing is off unless the option is given, and states no default value; parse yields `trace_actions == false` |
| `REQ-MOK-018` | automated-test | Every declared default is stated | The five declared defaults each appear in the help text; the count of documented defaults is 5 of 5, against 1 of 5 before |
| `REQ-MOK-018` | automated-test | Order independence is stated | The help text states that options may appear in any order and at most once |
| `REQ-MOK-018` | static-analysis | Help-to-specification diff | The transcribed specification table and the rendered help agree on every option, default, and constraint; no surplus and no shortfall |
| `REQ-MOK-018` | automated-test | Both emission paths carry the same text | `--help` writes the new text to standard output; an invalid configuration writes the same text to standard error after the `configuration error:` line |
| `REQ-MOK-018` | automated-test | Help routing and exit code | `--help` exits `0`, writes nothing to standard error, and emits no simulation event |
| `REQ-MOK-018` | static-analysis | No duplicated default statement | Each declared default appears in exactly one place in the usage text |
| `REQ-MOK-018` | static-analysis | Constant form and interface | `cli::USAGE` is still `pub const USAGE: &str`; the public-surface inventory is unchanged item for item against `SPEC-MOK-002` rule 5 |
| `REQ-MOK-018` | static-analysis | No new dependency | Dependency and dev-dependency tables empty; no build script; `Cargo.toml` and `Cargo.lock` unchanged |
| `REQ-MOK-018` | static-analysis | Test placement | The new tests are in the public tier in `tests/cli.rs` under `SPEC-MOK-002` rule 8's existing subject; no new test file, and no test moved between tiers |
| `REQ-MOK-018` | automated-test | Simulation output unchanged | All 43 matrix cells byte-identical to the baseline |
| `REQ-MOK-018` | automated-test | Diagnostics unchanged apart from the usage block | For each of the fourteen invalid-configuration cases, exit code, empty standard output, and the `configuration error:` line are byte-identical |
| `REQ-MOK-018` | review | Readability | Each description is intelligible to a reader who has not read `SPEC-MOK-001`; a description that requires the specification to interpret is an adverse observation |
| `REQ-MOK-018` | static-analysis | Line width | No line the options block introduces exceeds 80 columns; the reproduced synopsis block, whose first line is 81 columns before and after, is unchanged |
| `REQ-MOK-018` | automated-test | Applied defaults unchanged | `cli::parse` on an empty argument list yields exactly the pre-change configuration: `seed 0`, `tick_limit 100`, `Policy::Reference`, `Density::DEFAULT`, `trace_actions false` |

### Amendment of 2026-08-22

Three rows above carry stale enumerations, corrected here rather than edited in place so that a reader of
`VREC-MOK-004` can see what they said. **"An options entry exists for each of `--seed`, `--ticks`, `--policy`,
`--density`, `--trace-actions`, and `--help`"** and its "All six present" pass condition were written when the
program accepted five options and `--help`; the set is now `--seed`, `--ticks`, `--policy`, `--density`,
`--trace-actions`, `--events-path` and `--help`, and the pass condition is that the options block and the synopsis
name exactly the options the parser's own match arms accept, in the same order, with no surplus and no shortfall.
**`--policy`'s row** named `baseline` and `reference` as the value set; it is `baseline`, `reference`, `individual`
and `social`, each of which the parser must accept and none of which may be absent from the entry. **"Every declared
default is stated"** counted five; it is six declared defaults, of which four are values stated once each and two are
stated absences.

These rows are added:

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-018` | automated-test | Each accepted value of a closed-set option is explained | The `--policy` entry contains all four value names *and* a description of each; a value named in the placeholder and not described in the entry is a failure |
| `REQ-MOK-018` | automated-test | The retired prose is gone rather than duplicated | No text after the options block states a default or a value constraint; the decision-source descriptions and the `--density` explanation appear in their entries and nowhere else |
| `REQ-MOK-018` | automated-test | The two added statements are present and true | The text states what it writes and to which stream, and states exit `0` for a finished run or printed help, `2` for invalid configuration and `1` for an output failure; the three codes are the three `SPEC-MOK-001` *Outputs* fixes and no fourth is named |
| `REQ-MOK-018` | automated-test | The observer describes the shared inputs in the engine's words | Each of the `--seed`, `--ticks`, `--policy` and `--density` entries read out of `mokiterions_tui::options::USAGE` appears byte for byte in `mokiterions::cli::USAGE`; an edit to one alone fails and names the option |

And these acceptance scenarios:

6. An operator who has not read any specification runs `--help`, chooses a `--policy` value on the strength of the
   entry alone, and can say what that choice will make the Mokiterions do — the failure this amendment exists to
   remove being a choice made from four bare names.
7. A description of one shared input is edited in one target and not the other in a scratch copy; the suite fails and
   names the option. The scratch change is not committed, and its purpose is to show that the duplication is held
   rather than merely intended.

## Acceptance scenarios

1. A reviewer holds the transcribed `SPEC-MOK-001` table beside the rendered help output and finds them in agreement
   on all six options, all five defaults, and all three value constraints, with nothing in the help that the
   specification does not declare.
2. An operator who has not read the specification runs `--help`, and for each option can say what it does and what
   happens if it is omitted.
3. The suite is run after an applied default is changed in a scratch copy and the usage text is left alone; a test
   fails and names the disagreement. The same is done in reverse, changing the printed default and leaving the code
   alone, and a test fails again. Neither scratch change is committed; both are recorded as the demonstration that
   the equality check is load-bearing rather than decorative.
4. The program is run before and after the change at each declared seed, under each decision source, at each
   declared density, with and without `--trace-actions`, and the captured output is byte-identical in every
   combination.
5. Each of the fourteen invalid-configuration invocations produces the same exit code and the same
   `configuration error:` line as before, with the appended usage block replaced and nothing else changed.
6. `cargo test` is run once with no extra flag or environment variable; the pre-existing 52 tests all pass, the new
   tests pass, and no test is ignored.
7. The public-surface inventory is regenerated and is identical to the one retained under `WO-MOK-003`, confirming
   that the interface did not grow to serve the new tests.

## Property and invariant tests

- **Printed equals applied.** For every option with a default, the value stated in the usage text and the value the
  program applies when the option is omitted are the same value. This is the central invariant; a single
  disagreement fails this contract.
- **Simulation surface unchanged.** No simulation constant, event field, event order, summary field, action-trace
  line, or exit code differs from the baseline. The only permitted textual difference in the whole program's output
  is the usage block.
- **Determinism preserved.** Two post-change runs at one seed remain byte-identical, and match the baseline, so
  `REQ-MOK-009` is intact.
- **One text, two paths.** The bytes written to standard output by `--help` and the bytes appended to standard error
  after a configuration error are the same bytes, so the two paths cannot diverge.
- **Coverage totality.** Every option the parser accepts has an entry, and every entry names an option the parser
  accepts. A future option added to the parser without an entry fails this property.
- **Statement uniqueness.** Each declared default is stated exactly once in the usage text, so there is no second
  copy inside the text to fall out of agreement with the first.
- **Constancy.** The usage text is a compile-time constant. It is identical on every invocation and depends on no
  environment variable, file, clock, or runtime state.
- **Interface conservation.** The public interface is item-for-item identical to `SPEC-MOK-002` rule 5 as verified
  under `VREC-MOK-003`. Nothing was made public to let a test read the help.
- **Census growth is accounted.** The test total rises from 52 by exactly the number of tests this work adds, every
  new test is named in the evidence, no pre-existing test is removed, renamed, or ignored, and the ignored count
  stays 0.

## Static and architecture checks

- `cargo fmt --all -- --check` reports no differences.
- `cargo clippy --all-targets --all-features -- -D warnings` reports no findings, with neither the invocation
  narrowed nor an `allow` attribute added.
- `cargo build` and `cargo test` succeed. `Cargo.toml` and `Cargo.lock` are unchanged.
- `SPEC-MOK-002` rules 5 and 6 hold unchanged: the public interface is the same closed enumeration, and no
  prohibited item became public or reachable.
- `SPEC-MOK-002` rules 7 and 8 hold: the new tests require only rule 5's interface, so they belong in the public
  tier, and they sit in `tests/cli.rs`, whose declared subject already includes defaults. No new public-tier file is
  created and no test changes tier.
- `SPEC-MOK-001`'s *Help output* amendment is present and approved **before** the usage text changes. Its absence
  fails this contract regardless of code state, because without it the new content has no specified authority.
- No architecture is engaged. `ARCH-MOK-001` addresses `REQ-MOK-004`, `REQ-MOK-008`, `REQ-MOK-009`, `REQ-MOK-010`,
  and `REQ-MOK-016`, and does not address `REQ-MOK-018`; the system boundary, dependency direction, trust boundary,
  and interface membership are all unchanged. This check is recorded so that the omission is deliberate and
  reviewable rather than silent.

## Security and privacy checks

- The usage text contains no credential, no token, no absolute filesystem path, no host name, and no user name. It
  is a constant, so this is established by reading it once.
- No network access, credential read, filesystem access, environment read, or wall-clock read is introduced. The
  dependency table stays empty.
- Invalid input remains data and is never interpreted as code or a path. The `configuration error:` line still
  reports the operator's rejected value through the existing formatting, unchanged from the baseline.
- The help text discloses only what `SPEC-MOK-001` already declares publicly. It names no private type, no internal
  constant, and no file path, so a longer help text does not widen what the program reveals about its interior.
- No test-support seam, feature flag, or conditional visibility is introduced; the new tests reach the program
  through the same interface an external caller has.

## Performance and resilience checks

- The 1,000-tick population floor at the declared density is met on every declared seed, with survivor counts
  identical to the baseline rather than merely above the floor: `8, 11, 8, 9, 11` on seeds `0, 1, 42, 123, 777`.
- A 10,000-tick run under each decision source completes without panic, with survivors plus deaths equal to twelve,
  at the same termination ticks as recorded before.
- Runtime work per tick and memory use are unchanged, which follows from byte-for-byte equivalence on the matrix and
  needs no separate measurement. The usage text is a constant, so a longer one costs binary size and no runtime work.
- Output volume for a simulation run is unchanged. Only the help and diagnostic paths write more bytes, and both are
  bounded and non-recurring.

## Manual assessments

- Read the rendered help as an operator who has not read `SPEC-MOK-001`, and judge for each option whether its
  effect and its behavior on omission are clear. An entry that is merely a restatement of the option's name is an
  adverse observation.
- Confirm that the two retained explanatory paragraphs still earn their place: they say why the reference source
  exists and what `--density` binds together, which an options list cannot carry. A paragraph that only repeats an
  options entry is an adverse observation and should be shortened rather than kept for continuity.
- Confirm the equality check is written against the program's applied configuration and not against a second literal
  in the test. A test that compares the text to a constant it declares itself restates the drift problem one level
  up and is an adverse observation.
- Confirm no description states a behavior the specification does not declare. Help text is not a place to decide
  behavior.
- Judge whether the printed `--ticks` default of `100`, now visible beside a viability floor stated at 1,000 ticks,
  reads as misleading to an operator. This is an observation for the product owner's deferred decision on that
  default, recorded here rather than resolved: the value is deliberately unchanged by this work.

## Evidence retention

Retain under `docs/engineering/simulation/evidence/WO-MOK-004/`:

- the recorded pre-change baseline, with the commit it was captured at: the 43-cell matrix manifest and summary
  lines, and the 16 named cases with exit codes, standard-output digests, and full standard-error text;
- the post-change capture for the same matrix and the same cases, and the comparison result, stating explicitly
  which cells were required to be identical and which were required to change;
- the full pre-change and post-change usage text, and a line-by-line diff of the two, so that the whole of the
  change to observable output is legible in one place;
- the specification-to-help mapping: the table transcribed from `SPEC-MOK-001` and its diff against the rendered
  help, with the documented-default count before and after;
- the output of the new tests, and their names;
- the record of the two deliberate scratch divergences of acceptance scenario 3 — one changing an applied default,
  one changing the printed default — each with the failing test's name and message, and confirmation that neither
  was committed;
- the regenerated public-surface inventory and its diff against the inventory retained under `WO-MOK-003`;
- the test census before and after, reconciled name by name, with the new tests identified as additions;
- formatter, linter, build, and test output;
- the 1,000-tick per-seed survivor counts and the 10,000-tick resilience result, compared against the baseline;
- confirmation that the `SPEC-MOK-001` amendment was approved before the usage text changed, with its approval date;
- a completion summary naming the final usage text, the tests added, and the documented-default count.

## Residual uncertainty

- **The equality is verified for the defaults that exist today.** An option added later with a default and without
  an entry would be caught by the coverage totality property only if that property is written over the parser's
  accepted options rather than over a hand-written list. The distinction is recorded here because the weaker form is
  easy to write and looks identical when it passes.
- **Readability is assessed by review, not measured.** No mechanical check can establish that a description helps an
  operator, so this contract states an assessment and names the adverse observation rather than pretending to a
  threshold. A reviewer who has already read the specification is a poor judge of it, which is why the assessment
  names the audience explicitly.
- **The change is verified against the declared matrix, not the input space.** Undeclared seeds, undeclared
  densities, and the release profile are not covered. A help-text change has no mechanism for seed-dependent
  divergence, so this is the shape of the claim rather than a shortfall in executing it.
- **Line width is checked, terminal rendering is not.** Whether the text reads well in a narrow window, under
  wrapping, or through a pager is not verified, and a complaint about it later is an amendment rather than a defect
  in this verification.
- **This contract does not verify that the descriptions are the best possible wording**, only that they are present,
  accurate against the specification, unique, in agreement with the applied defaults, and readable. A later
  improvement to wording is an amendment to `SPEC-MOK-001`'s *Help output* section.
- **The `--ticks` default remains unverified against the viability floor**, because this work does not change it.
  Printing it makes the mismatch with the 1,000-tick floor visible, which is the intended effect; whether `100` is
  the right default is a product decision outside this contract, and nothing here should be read as accepting it.
