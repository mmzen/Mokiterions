# WO-MOK-004 evidence: specification-to-help mapping

`VER-MOK-004`'s first oracle is `SPEC-MOK-001`, transcribed into a table without reading the code, and
diffed against the rendered help. A default the specification declares and the help omits is a
shortfall; a default the help states and the specification does not declare is a surplus; a stated value
that differs from the declared one is a defect. All three are findings.

The table below is transcribed from `SPEC-MOK-001`'s *Help output* section. The *Rendered* columns are
read from `after/usage.txt`.

## Options, in order

`SPEC-MOK-001` fixes the order: `--seed`, `--ticks`, `--policy`, `--density`, `--trace-actions`,
`--help`. The rendered options block names them in that order, and the synopsis names them in the same
order. Six declared, six rendered, no surplus, no shortfall.

## Defaults and constraints

| Option | Declared default | Rendered default | Declared constraint | Rendered constraint |
| --- | --- | --- | --- | --- |
| `--seed` | `0` | `Default: 0.` | none | none |
| `--ticks` | `100` | `Default: 100.` | must be greater than zero | `must be greater than zero` |
| `--policy` | `reference` | `Default: reference.` | only `baseline` and `reference` are valid, which the value placeholder states | placeholder `<baseline|reference>` |
| `--density` | `0.75` | `Default: 0.75.` | at most two decimal places | `at most two decimal places` |
| `--trace-actions` | no value; the entry states that tracing is off unless the option is given | `Off unless given.`, and no `Default:` in the entry | none | none |
| `--help` | none | none | none | none |

Five declared defaults, five rendered. **No shortfall, no surplus, no defect.**

Documented-default count: **1 of 5 before, 5 of 5 after.** The single pre-change statement was
`It defaults to 0.75.` in the density prose paragraph; it has been removed from the prose and is now
stated in the `--density` entry, so the count rose by four without any fact being stated twice.

## Descriptions

`SPEC-MOK-001` requires "a short description of its effect that is intelligible without this
specification at hand" and fixes nothing else about wording. The rendered descriptions:

| Option | Rendered description |
| --- | --- |
| `--seed` | Entropy stream seed. |
| `--ticks` | Ticks to run; must be greater than zero. |
| `--policy` | Decision source. |
| `--density` | Resource density per territory, at most two decimal places. |
| `--trace-actions` | Emit one action trace per living-agent decision opportunity. Off unless given. |
| `--help` | Print this usage and exit without running. |

Read as an operator who has not opened `SPEC-MOK-001`: each says what the option does and what happens
when it is omitted. None is a restatement of the option's own name, which `VER-MOK-004` names as an
adverse observation. `--policy`'s two words are the shortest of the six and lean on the two paragraphs
below the block, which say what the sources are and why the reference one exists; that division is
deliberate and is the reason those paragraphs were kept.

## Ordered content

`SPEC-MOK-001` requires the usage text to contain, in order: the synopsis block; an options block; a
statement that options may appear in any order and at most once; and the explanatory prose on the
decision sources and on what `--density` binds together. The rendered text has exactly those four parts
in exactly that order, and nothing else.

## No behaviour invented

`SPEC-MOK-001` states that the options block "states no behavior this specification does not state
elsewhere". Every claim in the rendered block is checked against the running program by a test, not by
reading:

| Rendered claim | Test that holds the program to it |
| --- | --- |
| the four stated defaults | `each_documented_default_parses_to_the_applied_default` |
| `--ticks` must be greater than zero | `the_entries_state_the_constraints_that_decide_validity` (`--ticks 0` rejected) |
| only `baseline` and `reference` | `the_entries_state_the_constraints_that_decide_validity` (`--policy random` rejected) |
| at most two decimal places | `the_entries_state_the_constraints_that_decide_validity` (`--density 0.751` rejected) |
| tracing off unless given | `the_flags_state_their_effect_and_no_default_value` |
| `--help` prints and does not run | `the_flags_state_their_effect_and_no_default_value`, `help_exits_successfully` |
| any order, at most once | `the_help_text_states_order_and_repetition` |
| the six options are the accepted ones | `the_documented_options_are_exactly_the_options_the_parser_accepts` |

A description could still be badly worded and pass all of these. It could not be *false*.
