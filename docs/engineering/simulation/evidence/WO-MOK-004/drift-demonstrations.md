# WO-MOK-004 evidence: the drift check demonstrated failing

`VER-MOK-004` acceptance scenario 3 requires the suite to be run once with an applied default changed
and the usage text left alone, and once in reverse, with a test failing and naming the disagreement both
times. `WO-MOK-004`'s stop conditions state it as the sharper claim: "either deliberate scratch
divergence fails to fail, because a check that cannot fail is not a check."

Three divergences were performed. Two are the scenario. The third discharges `VER-MOK-004`'s residual
uncertainty about coverage totality.

| Fact | Value |
| --- | --- |
| `src/cli.rs` digest before the divergences | `5498a62e5eed36ac75d86048c59d0dfda1296bd17fac6cd420e07a7a95bb8419` |
| Digest after each revert | verified equal with `sha256sum -c`, three times |
| Committed | **None.** No divergence was staged or committed. |
| Residue check | `grep -c 'unwrap_or(1000)\|Default: 1\.\|--verbose' src/cli.rs` → `0` |

Raw transcripts: `drift-scratch-a.txt`, `drift-scratch-b.txt`, `drift-scratch-c.txt`.

## A — the applied default moves, the text does not

The change: `tick_limit: ticks.unwrap_or(100)` → `ticks.unwrap_or(1000)`. This is the exact scenario
`REQ-MOK-018`'s failure example names, and the exact value the deferred product decision on `--ticks`
would involve.

Result: `4 failed; 8 passed`. The new drift test failed and named the disagreement:

```
---- each_documented_default_parses_to_the_applied_default stdout ----
assertion `left == right` failed: the help states 100 as the default for --ticks, which is not the
value the program applies when --ticks is omitted
  left: Config { seed: 0, tick_limit: 100, policy: Reference, density: … , trace_actions: false }
```

Three pre-existing tests also failed — `defaults_are_stable`,
`both_policies_are_selectable_and_reference_is_the_default`, and
`density_is_accepted_in_the_specified_forms_and_rejected_otherwise` — because they pin the applied
defaults directly. That is expected and it is why this direction is the easy one: the repository already
defended it.

## B — the text moves, the applied default does not

The change: `Entropy stream seed. Default: 0.` → `Default: 1.` in `cli::USAGE`. Nothing in `parse`
touched.

Result: `1 failed; 11 passed`. The only failure is the new test, and it names the disagreement:

```
---- each_documented_default_parses_to_the_applied_default stdout ----
assertion `left == right` failed: the help states 1 as the default for --seed, which is not the value
the program applies when --seed is omitted
```

**This is the direction that matters.** Every one of the eleven other tests passed, including all five
that existed before this work order, and including `defaults_are_stable`, which asserts the applied
defaults, and `help_exits_successfully`, which asserts the help output equals `cli::USAGE`. Before this
work, the program could have advertised a default it did not apply and the entire suite would have been
green. That is the failure mode `REQ-MOK-018` was raised to close, and it is now closed by exactly one
test — which is why the test was demonstrated failing rather than asserted to work.

## C — a parser option with no options-block entry

Not part of scenario 3; performed because `VER-MOK-004`'s residual uncertainty states that the weak form
of the coverage property "looks identical when it passes".

The change: a `"--verbose" => { index += 1; }` arm added to `parse`, with no entry added to the help.

Result: `1 failed; 11 passed`. The coverage test failed:

```
---- the_documented_options_are_exactly_the_options_the_parser_accepts stdout ----
assertion `left == right` failed: every option the parser accepts is described, and nothing else is
  left: ["--density", "--help", "--policy", "--seed", "--ticks", "--trace-actions"]
```

The documented set is six; the accepted set is seven; the check fails. The property is written over the
parser's accepted options, so the residual uncertainty `VER-MOK-004` recorded is discharged rather than
carried forward.

## What was not demonstrated

- **A constraint stated but not enforced.** The three constraint checks pair each stated rule with a
  value the parser rejects, so a stated constraint the program does not enforce would fail. This was
  reasoned from the test body rather than demonstrated by a fourth scratch divergence; the two scenario
  divergences and one extra were judged sufficient, and the omission is recorded here rather than
  implied.
- **Readability.** No mechanical check establishes that a description helps an operator.
  `VER-MOK-004` states this as an assessment, and it is recorded in `specification-to-help.md`.
