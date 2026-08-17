+++
id = "REQ-MOK-018"
type = "requirement"
title = "State every option's effect and default in the help output"
status = "approved"
owners = ["product owner"]
created = "2026-08-17"
updated = "2026-08-17"
statement = "WHEN the program prints its usage text, THE SYSTEM SHALL state, for every option it accepts, a short description of that option's effect and, for every option that has one, the default value the program itself applies when the option is omitted."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-001"]
+++

# Requirement: State every option's effect and default in the help output

## Rationale

The program accepts five options and `--help`. `SPEC-MOK-001` declares a default for all five. The usage text
states one of them.

An operator who runs `--help` therefore learns each option's name and the shape of its value, and nothing about
what the option does or what happens when it is left out. `--seed` and `--ticks` appear only as type placeholders.
`--policy`'s effect is described in a prose paragraph that never says which source is used by default.
`--trace-actions` is named and not explained. The only ways to discover the program's own defaults are to read
`src/cli.rs` or to read the specification, and neither is part of what is delivered to an operator. The gap is
between an approved specification and the operator-visible interface, so closing it invents no behavior: the
content already exists and has already been approved.

The second clause is the one that carries risk, and it is stated as an equality on purpose. A default written into
help as prose is a second copy of a value the code computes elsewhere, and two copies drift silently — the program
keeps working while the text becomes a lie. This requirement is therefore not satisfied by writing correct text
once. It is satisfied when the printed default and the applied default are the same value and are held equal by a
check that fails when they diverge. The repository already carries one instance of the unchecked form: the usage
text says `--density` "defaults to 0.75" and nothing binds that sentence to `Density::DEFAULT`. Multiplying that
arrangement fourfold without the check would make the interface less trustworthy, not more.

## Preconditions and trigger

The program renders its usage text. There is one usage text and two paths that emit it: `--help` writes it to
standard output, and an invalid configuration writes it to standard error after the `configuration error:` line.
The obligation is a property of the text, so it holds on both paths and cannot be satisfied on one alone.

## Required response

The usage text contains a synopsis and an options block. The options block contains one entry for each option the
program accepts — `--seed`, `--ticks`, `--policy`, `--density`, `--trace-actions`, and `--help` — and each entry
states:

- the option, with its value placeholder where it takes a value;
- a short description of the option's effect, readable without the specification at hand;
- the default the program applies when the option is omitted, for every option that has one;
- the constraint that decides whether a value is accepted, for every option that constrains its values.

`--trace-actions` accepts no value. Its entry states that tracing is off unless the option is given, and states no
default *value*: printing one would invite `--trace-actions false`, which the program rejects. `--help` has no
value and no default.

Every default the options block states is the value the program applies when that option is omitted. The two are
required to be equal.

Help continues to be written to standard output, to exit `0`, and to start no simulation. The same text continues
to follow the `configuration error:` line on standard error when configuration is invalid.

## Failure and boundary behavior

- If an applied default changes and the usage text does not, or the usage text changes and the applied default does
  not, that disagreement is a failure of this requirement. It is not permitted for the two to differ, and it is not
  enough for them to agree by inspection at one moment.
- A description that cannot be understood without reading `SPEC-MOK-001` does not satisfy this requirement. The
  operator reading a terminal is the audience.
- The usage text stays a fixed constant. It reads no environment variable, no file, no wall clock, and no runtime
  state, so it is identical on every invocation and remains reproducible.
- Adding descriptions changes no simulation output. Event lines, the run summary, action traces, exit codes, and the
  `configuration error:` line itself are unchanged.
- Verbosity stays bounded: one entry per option, not a paragraph per option, and the text stays readable in an
  eighty-column terminal.
- A fact stated in the options block is stated once. Where the existing explanatory prose already states the same
  default, the duplicate is removed rather than left to disagree later.

## Constraints

- Preserve `REQ-MOK-009`, `REQ-MOK-010`, `REQ-MOK-011`, and `REQ-MOK-012`. No byte of any event line, summary line,
  or action trace changes, and determinism is untouched.
- Preserve `REQ-MOK-016`. The public interface does not grow. `cli::USAGE` remains a public `&'static str` constant
  of the same form, and no public item is added, removed, or changed in signature.
- Add no dependency. Stable Rust cannot format integers in a constant, and the requirement is not to be met by
  importing a compile-time formatting crate. The equality between printed and applied defaults is established by a
  test, not by generating the string.
- The operator-facing command name stays `Mokiterions`.
- Content is fixed by `SPEC-MOK-001`. Column alignment, column widths, and line wrapping are not.

## Acceptance examples

### Example: normal behavior

**Given** the program is built

**When** an operator who has never run it invokes `Mokiterions --help`

**Then** standard output contains one entry for each of the six options; `--seed` states its default `0`; `--ticks`
states its default `100` and that it must be greater than zero; `--policy` states its default `reference` and that
`baseline` and `reference` are the only valid values; `--density` states its default `0.75` and the two-decimal
limit; `--trace-actions` states that tracing is off unless the option is given and states no value; the process
exits `0`; and no simulation event is emitted.

### Example: failure behavior

**Given** a later change raises the applied default tick limit from `100` to `1000` and leaves the usage text
alone

**When** the test suite runs

**Then** a test fails and names the disagreement between the printed default and the applied default, rather than
the program continuing to advertise a default it no longer applies.

## Open decisions

No product decision is open for this requirement. Two matters are recorded so that neither is mistaken for part of
it.

1. **The value of the `--ticks` default is a separate product decision, and it was explicitly deferred on
   2026-08-17** by the repository owner acting as product owner, when this requirement was approved and `WO-MOK-004`
   was authorized to print the default the program already applies. Every viability obligation in
   this repository is stated at 1,000 ticks — `REQ-MOK-014`, `VER-MOK-002`, and the per-seed survivor floor of
   eight — while the applied default is `100`. A run with no arguments therefore satisfies no declared viability
   floor. This requirement makes that visible by printing the default, and deliberately does not change it: doing
   so would alter simulation behavior on the most common invocation and carries its own verification cost. The
   product owner may take that decision separately, before or after this requirement, and this requirement neither
   makes it nor forecloses it.
2. **`CAP-MOK-001`'s *Candidate requirements* list does not name this requirement.** That list is informative
   rather than authoritative; the graph edge is this requirement's `derives_from`. Adding a line to it is an
   editorial act available to the product owner and is not a precondition of approval here.
