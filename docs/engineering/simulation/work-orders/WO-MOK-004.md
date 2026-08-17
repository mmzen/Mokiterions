+++
id = "WO-MOK-004"
type = "work_order"
title = "State every option's effect and default in the help output"
status = "implemented"
owners = ["engineering owner"]
created = "2026-08-17"
updated = "2026-08-17"

[assurance]
commit_bound_verification = "required"
rationale = "This work changes observable program output that assurance and release decisions rely on. The usage text is emitted on two paths, one of them the diagnostic path taken by every invalid invocation, so fourteen of the sixteen recorded diagnostic cases change their standard-error text. Its central claim is again an equivalence claim — everything byte-identical except the usage block — and a claim of that shape is only worth anything when it has been demonstrated against a recorded baseline at a specific commit rather than asserted."
decided_by = "engineering owner"

[relations]
implements = ["REQ-MOK-018"]
specifications = ["SPEC-MOK-001", "SPEC-MOK-002"]
verification = ["VER-MOK-004"]
+++

# Work Order: State every option's effect and default in the help output

## Lifecycle

This work order was approved on 2026-08-17 by the repository owner acting as product, technical, engineering, and
assurance owner, and moved to `in_progress` on the same date. Approval authorizes only the scope below. Transition to
`implemented` requires the completed change and retained evidence. Commit-bound verification is classified
`required`, so assurance acceptance requires a separate verification record naming a candidate commit, prepared by
the managed capture command and transitioned to `verified` by the accountable assurance owner as a later act.

One correction was made to this work order before approval rather than deviated from afterwards. The original draft
listed `tests/process.rs` as untouched, while `VER-MOK-004` requires both emission paths to carry the same bytes and
the existing assertion on the diagnostic path checks only for the substring `Usage:`. The scope and the expected
change surface below now authorize one added test in that file.

Moved to `implemented` on 2026-08-17. The change is complete and the evidence `VER-MOK-004` requires is retained
under `docs/engineering/simulation/evidence/WO-MOK-004/`, indexed by its `README.md` and reported in
`completion-summary.md` in the eleven-item form the *Completion report format* section below requires. The emitted
help output is byte-identical to the rendering authorized below. All 43 matrix cells are byte-identical to the
pre-change baseline; the only changes outside the usage block are the `--help` case's standard output. The census
rose 52 → 60 with no pre-existing test altered, and the drift check was demonstrated failing in both directions and
in a third case beyond the contract's requirement. Gates: `fmt`, `clippy -D warnings`, `build`, and `test` all exit
`0`; `validate_engineering_artifacts.py` PASS at 40 / 0 / 0; `preflight --phase review` PASS; `doctor` PASS.

One authorized local decision differs in name from the list below: the second test was written as
`the_documented_options_are_exactly_the_options_the_parser_accepts` rather than
`every_documented_option_is_accepted_by_the_parser`, because it asserts both directions. Test names are inside the
authorized decision envelope, and the change is recorded in the evidence rather than left implicit.

Committed on 2026-08-17 to `feature/help-output-options-block`, branched from `master` at
`08dc8d47aa569c77fb7c9b091e328eb8f40c5285` — the exact commit the pre-change baseline was captured at, so that the
diff under review is the whole of the change the comparison measured. `c565cd09` carries the implementation and the
evidence; `95f0aa20` corrects two statements in that evidence which described the repository wrongly, one claiming the
post-change traced runs had not been retained when `after/full/` holds both, the other stating a commit count that
the next commit falsified. Opened as pull request #8. `VREC-MOK-004` names `95f0aa20` as the candidate commit and is
`ready`; the transition to `verified` is the accountable assurance owner's separate act and has not been performed.
Nothing was tagged, released, published, or merged, and no release record exists.

`SPEC-MOK-001` is selected because it is the contract that must be amended: it declares all five defaults in its
*Inputs* section and says nothing about what the help text contains, so the content this work adds has no specified
authority until that gap is closed. `SPEC-MOK-002` is selected because it is the contract that must be left
untouched: it owns the closed public interface that `cli::USAGE` belongs to, and its rules 7 and 8 decide where the
new tests go.

**Architecture is deliberately not selected, unlike `WO-MOK-003`.** `ARCH-MOK-001` addresses `REQ-MOK-004`,
`REQ-MOK-008`, `REQ-MOK-009`, `REQ-MOK-010`, and `REQ-MOK-016`, and does not address `REQ-MOK-018`. Nothing here
moves the system boundary, the dependency direction, the trust boundary, or the membership of the enumerated public
interface: `cli::USAGE` stays a public `&'static str` constant of the same form and only its value changes. One
counter-argument is disclosed rather than left for a reviewer to find. `SPEC-MOK-002` rule 2 remarks that
"`REQ-MOK-010`'s observable interface includes that text", which could be read as pulling the usage text under an
addressed requirement. The remark is about the operator-facing command name appearing in the first line of `USAGE`,
and `REQ-MOK-010` obliges the program to emit an event when a simulation occurrence happens; a help block is not an
event. A reviewer who reads it the other way should say so at approval, and the remedy is to add `ARCH-MOK-001` and
a nominal `ADR-MOK-001` to the relations rather than to change the work.

### Approval preconditions

Approval was blocked until all four held, in this order. All four were satisfied on 2026-08-17 by the repository
owner, who holds every accountable role in this repository, and approval followed. No implementation began before
they were complete and approved.

1. **Satisfied.** `REQ-MOK-018` is `approved` by the product owner.
2. **Satisfied.** `SPEC-MOK-001` is amended exactly as tabulated below, `REQ-MOK-018` is added to its `specifies`
   relation, and the amendment is recorded as a row in its amendment record — approved by the technical owner. The
   ordering mattered: an `approved` requirement with no active specification coverage is a validation error, so the
   requirement and the amendment were approved together. `preflight --phase start` reported exactly this as `W016`
   against the draft, which is independent evidence that the precondition was load-bearing rather than ceremonial.
3. **Satisfied.** `VER-MOK-004` is `approved` by the assurance owner.
4. **Satisfied by explicit deferral.** The product owner deferred the `--ticks` default on 2026-08-17, recorded in
   `REQ-MOK-018`'s *Open decisions*. This work order prints `100`, the value the program already applies; had the
   owner decided to change the value, the authorized rendering below would have changed with it and the work would
   have become a behavior change with its own verification cost.

An implementation agent may not approve any of the above. `DECISION_RIGHTS.md` assigns requirements to the product
owner, specification and architecture to the technical owner, and verification to the assurance owner.

### Tabulated `SPEC-MOK-001` amendment

| Location | Current | Amended to |
|---|---|---|
| Frontmatter `specifies` | Ends at `"REQ-MOK-015"` | Adds `"REQ-MOK-018"` |
| *Amendment record* | Five rows, last dated 2026-08-17 | Adds a row: "Specified the content of the help text: an options block stating each accepted option's effect, its default where it has one, and its value constraint where it has one, with the stated defaults required to equal the applied defaults. Narrowed the *Explicitly unspecified decisions* entry on help text to alignment, width, and wrapping. No simulation behavior changed." |
| *Inputs*, after the bullet list and before *Outputs* | No statement about help content beyond "`--help` prints usage and exits successfully without starting a simulation." | Adds a `### Help output` subsection, stated in full below |
| *Explicitly unspecified decisions*, last entry | "Cosmetic whitespace in help text." | "Column alignment, column widths, line wrapping, and cosmetic whitespace in the help text. Which options the options block describes, and which defaults and constraints it states, are not unspecified: they are fixed by the *Help output* section above." |

The `### Help output` subsection reads:

> `--help` writes the usage text to standard output and exits `0`. The same usage text follows the
> `configuration error:` line on standard error when configuration is invalid. There is one usage text and two paths
> that emit it, so the content below is a property of that text and not of either path.
>
> The usage text contains, in order: the synopsis block above; an options block; a statement that options may appear
> in any order and at most once; and the explanatory prose on the decision sources and on what `--density` binds
> together.
>
> The options block contains one entry for each option the program accepts — `--seed`, `--ticks`, `--policy`,
> `--density`, `--trace-actions`, and `--help`, in that order. Each entry states the option, its value placeholder
> where it takes a value, and a short description of its effect that is intelligible without this specification at
> hand. Each entry additionally states:
>
> | Option | Stated default | Stated constraint |
> |---|---|---|
> | `--seed` | `0` | none |
> | `--ticks` | `100` | must be greater than zero |
> | `--policy` | `reference` | only `baseline` and `reference` are valid, which the value placeholder states |
> | `--density` | `0.75` | at most two decimal places |
> | `--trace-actions` | no value; the entry states that tracing is off unless the option is given | none |
> | `--help` | none | none |
>
> Every default stated in the options block is the value the program applies when the option is omitted. The two are
> required to be equal, and that equality is verified rather than maintained by convention.
>
> Each of these facts is stated once. Where the explanatory prose previously repeated a default or a value
> constraint that the options block now states, the repetition is removed.
>
> The options block states no behavior this specification does not state elsewhere. It restates the *Inputs* list for
> the operator's benefit, and where the two differ this specification's *Inputs* list governs.

## Objective

Make the program's own interface discoverable from the program: state, for every option it accepts, what the option
does and what happens when it is omitted — and bind each printed default to the applied default with a test, so the
two cannot drift. Change nothing else that the program emits.

## In scope

- Rewrite `cli::USAGE` to the authorized rendering below, keeping it a `pub const USAGE: &str`.
- Remove from the retained `--density` paragraph the two facts the options block now states — the two-decimal limit
  and the default of `0.75` — and re-wrap the paragraph. Keep what only that paragraph says: the three roles density
  binds together, and that only the declared densities carry a viability floor.
- Keep the decision-source paragraph as it stands. It states why the reference source exists, which an options
  entry cannot carry.
- Add public-tier tests to `tests/cli.rs` that establish, for each option with a default, that the value stated in
  `cli::USAGE` and the value `cli::parse` applies on an empty argument list are the same value; that an entry exists
  for every option the parser accepts; and that the order-and-once statement is present. `tests/cli.rs` is the right
  file under `SPEC-MOK-002` rule 8, whose declared subject for it already begins with defaults.
- Add one public-tier test to `tests/process.rs` asserting that the standard-error text of an invalid configuration
  ends with `cli::USAGE` in full. `VER-MOK-004` requires both emission paths to carry the same bytes, and the
  existing assertion on that path checks only for the substring `Usage:`, which a truncated or wrong usage block
  would still satisfy. The four existing tests in that file are unchanged.
- Capture the pre-change baseline required by `VER-MOK-004` before making any change, and the post-change
  comparison after.
- Perform the two deliberate scratch divergences of `VER-MOK-004` acceptance scenario 3, record the failing test
  names and messages as evidence, and commit neither.
- Retain every evidence item listed in `VER-MOK-004` under this work order's ID.

## Out of scope

- Any change to simulation behavior. No simulation constant, event field, event order, summary field, action-trace
  line, exit code, or the `configuration error:` line may change.
- **Changing any applied default, including `--ticks`.** The value printed is the value the program already applies.
  The mismatch between the default of `100` and the 1,000-tick viability floor is made visible here and decided
  elsewhere.
- Generating the usage text at compile time, converting `cli::USAGE` into a function, or adding `const_format` or any
  other dependency. Stable Rust cannot format integers in a constant; the equality is established by a test instead.
  Converting the constant to a function would change item 4 of `SPEC-MOK-002` rule 5's closed enumeration and needs
  a rule 5 amendment that this work order does not carry.
- Any addition to, removal from, or change of form in the public interface. No item is made public to let a test read
  the help.
- A new file under `tests/`. The help content belongs to an existing rule 8 subject, and one file per test is not the
  arrangement.
- Short flags such as `-h`, a `--version` option, subcommands, colorized or paginated help, help topics, or any
  change to argument parsing. `cli::parse` is not edited.
- Rewording or reformatting the decision-source paragraph, and any tidying of `src/cli.rs` beyond the constant.
- Editing the retained evidence of `WO-MOK-001`, `WO-MOK-002`, or `WO-MOK-003`, or any of `VREC-MOK-001`,
  `VREC-MOK-002`, and `VREC-MOK-003`. They are bound to their own commits.
- Any harness-managed policy file, and any artifact lifecycle change outside this work order.

## Authorized decision envelope

The implementation agent may choose:

- column alignment, column width, and where descriptions wrap, provided no line the options block introduces
  exceeds 80 columns;
- the exact wording of each description, within the substance the amended specification fixes;
- whether the equality assertions are one test or several, their names, and whether a helper is shared;
- ordering of items within `tests/cli.rs`;
- the mechanism for the baseline comparison, provided it is byte-exact and its method is recorded;
- comments and non-authoritative developer notes.

The implementation agent may **not**:

- change any applied default, or any behavior of `cli::parse`;
- add, remove, or change the form of any public item, or add a dependency, dev-dependency, feature, `cfg`
  attribute, or build script;
- change any existing assertion, including `tests/process.rs`'s assertion that the help output equals `cli::USAGE`,
  which stays as the routing check it is;
- state in help any behavior the specification does not declare, or omit a default the specification declares;
- write the equality check against a literal the test declares itself, which would restate the drift problem one
  level up;
- regenerate the pre-change baseline after observing a discrepancy;
- relax the lint gate, narrow its invocation, or add an `allow` attribute;
- change requirement scope, specification content beyond the approved amendment, or any artifact's lifecycle status.

## Constraints

- Follow `REQ-MOK-018`, the amended `SPEC-MOK-001`, `SPEC-MOK-002`, and `VER-MOK-004`, and preserve everything
  verified under `VREC-MOK-001`, `VREC-MOK-002`, and `VREC-MOK-003` except the usage text itself.
- One Cargo package, two targets, empty dependency and dev-dependency tables. `Cargo.toml` and `Cargo.lock` are
  untouched.
- Capture the baseline first. A change made before the baseline exists cannot be verified and must be reverted.
- Both emission paths carry the same bytes, because both read the same constant. Do not introduce a second string
  for the diagnostic path.
- The usage text stays a compile-time constant that reads no environment variable, file, clock, or runtime state.
- Preserve `REQ-MOK-009`. Nothing here touches the entropy stream, and any diff that does is out of scope.
- Preserve unrelated user changes and do not modify generated output as source.

### Authorized rendering

The following is the authorized content. Alignment and wrapping are within the envelope above; the facts are not.

```text
Usage: Mokiterions [--seed <u64>] [--ticks <u64>] [--policy <baseline|reference>]
                   [--density <percent>] [--trace-actions]
       Mokiterions --help

Options:
  --seed <u64>                   Entropy stream seed. Default: 0.
  --ticks <u64>                  Ticks to run; must be greater than zero.
                                 Default: 100.
  --policy <baseline|reference>  Decision source. Default: reference.
  --density <percent>            Resource density per territory, at most two
                                 decimal places. Default: 0.75.
  --trace-actions                Emit one action trace per living-agent decision
                                 opportunity. Off unless given.
  --help                         Print this usage and exit without running.

Options may appear in any order and at most once.

The reference policy is a deterministic development instrument, not autonomous
behavior. It seeks and consumes perceived food so that world viability can be
measured. The baseline policy selects uniformly among valid actions.

--density is the percentage of a territory's cells that hold a resource. It sets
the initial endowment, the territory capacity, and the replenishment target
together. Only the densities declared in the requirements carry a population
viability floor.
```

That rendering is 25 lines, 21 of them non-blank, against 12 and 10 before, and it states five of five declared
defaults against one of five before. A different wrapping choice changes the line counts, and the evidence records
the final text rather than these numbers.

The synopsis block is reproduced unchanged, including its first line, which is 81 columns today. The 80-column
limit therefore applies to the lines this work introduces, not to text it leaves alone; narrowing the synopsis is
not authorized here.

## Expected change surface

- `src/cli.rs`: the `USAGE` constant only. `parse`, `option_value`, and every error string are unchanged.
- `tests/cli.rs`: the new tests. The five existing tests are unchanged.
- `tests/process.rs`: one added test pinning the diagnostic path to the whole constant. The four existing tests,
  including the assertion that the help output equals `cli::USAGE`, are unchanged.
- `docs/engineering/simulation/specifications/SPEC-MOK-001.md`: amended as precondition 2, before any code changes.
- Work-order-keyed evidence under `docs/engineering/simulation/evidence/WO-MOK-004/`.
- No change to `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `src/main.rs`, `src/simulation.rs`, `tests/density.rs`,
  `tests/termination.rs`, `tests/viability.rs`, or any harness-managed policy file.

## Required verification

- Complete every case, invariant, check, and manual assessment in `VER-MOK-004`.
- Capture the pre-change baseline at the starting commit: the 43-cell matrix and the 16 named exit-code and
  standard-error cases, with the commit recorded.
- Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo build`.
- Run `cargo test` once, with no extra flag or environment variable.
- Run `cargo test -- --list` before and after and reconcile the census against the recorded 52 tests and 0 ignored,
  identifying every addition by name.
- Compare post-change output against the baseline: all 43 matrix cells byte-identical; the `--help` case changed only
  in standard output; the fourteen invalid-configuration cases changed only in the appended usage block, with exit
  code, empty standard output, and the `configuration error:` line byte-identical; the single-tick case unchanged.
- Diff the pre-change and post-change usage text line by line and retain both in full.
- Diff the transcribed `SPEC-MOK-001` table against the rendered help, and record the documented-default count before
  and after.
- Regenerate the public-surface inventory and diff it against the one retained under `WO-MOK-003`.
- Re-run the 1,000-tick per-seed survivor measurement at the declared density and compare against the baseline
  counts, not merely against the floor, and one 10,000-tick run under each decision source.
- Demonstrate both directions of the drift check by deliberate scratch divergence, and record the failures.

## Evidence to record

Retain, under `docs/engineering/simulation/evidence/WO-MOK-004/`, every item listed in `VER-MOK-004`'s evidence
retention section.

Two items carry particular weight. The **pre-change baseline** is the only independent oracle for the claim that
nothing but the usage block moved; it is captured once, before any edit, and is never regenerated. The **record of
the two deliberate scratch divergences** is the only evidence that the printed-to-applied equality check can fail,
and therefore the only evidence that the requirement's second clause is met rather than merely written down.

## Stop and escalate conditions

Stop before implementation if preflight does not report this exact work order as eligible, if any approval
precondition above is unmet, or if any governing artifact remains incomplete or unapproved.

During implementation, stop and escalate if:

- any matrix cell, exit code, or `configuration error:` line differs from the recorded baseline and the cause is not
  an obvious local mistake correctable within this scope. A differing baseline is never accepted as the new truth;
- an applied default turns out to differ from the value `SPEC-MOK-001` declares. That is a conformance defect
  predating this work, and it is reported rather than papered over by printing the applied value;
- the printed-to-applied equality cannot be asserted without adding a public item, a dependency, or a second literal
  in the test;
- either deliberate scratch divergence fails to fail, because a check that cannot fail is not a check;
- a description cannot be written that is both accurate against the specification and intelligible without it, which
  is a signal that the specification's wording needs amendment rather than paraphrase;
- a line the options block introduces cannot be kept within 80 columns while stating what the specification
  requires;
- the new tests cannot be placed in the public tier under `SPEC-MOK-002` rule 7 using only rule 5's interface;
- the amended `SPEC-MOK-001` proves to conflict with `SPEC-MOK-002` or with another approved artifact;
- required verification fails and cannot be corrected within authorized scope;
- the change expands into parsing, defaults, short flags, a version option, or another artifact domain.

## Completion report format

Report:

1. the implemented requirement and the affected components;
2. the final usage text in full, with its line and non-blank line counts and the documented-default count;
3. any authorized local decisions, including alignment, wrapping, and description wording;
4. the tests added, with their names, the tier each is in, and the public items each uses;
5. the census reconciliation, naming every addition and confirming no pre-existing test changed;
6. verification commands and results, including the comparison against the baseline stated cell class by cell class;
7. the result of both deliberate scratch divergences, with the failing test names and messages;
8. the per-seed survivor counts compared against the baseline;
9. retained evidence paths;
10. residual limitations and explicitly deferred items, including the `--ticks` default;
11. final worktree and candidate-commit status.
