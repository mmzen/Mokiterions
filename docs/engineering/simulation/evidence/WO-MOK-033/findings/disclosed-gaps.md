# Gaps between the approved artifacts and what was buildable

`WO-MOK-033`'s stop-and-escalate condition 6 forbids amending an approved artifact on an
implementation agent's judgement. Each item below is therefore built one way, disclosed here, and left
for the owner to decide. None of them blocked the work. Nine are in `SPEC-MOK-008`; the tenth is in
`VER-MOK-019` itself.

## 1. Rule 12.7 and rule 14.1 disagree on an unwritable output

12.7 says a driver whose retained output cannot be written "fails under rule 15 and retains nothing".
Rule 15 is the refusal rule: status `2`, *nothing executed and nothing was written*. But an unwritable
output is discovered after the cells have run, and rule 14.1's table gives that case status `1`.

Built as 14.1 says: status `1`. Case B25 measures the unwritable output and B23 measures all four
statuses of the table. 12.7's *retains nothing* is honoured either way; only the status is in question.

## 2. Rule 7.2 constrains the stream's location and rule 2 gives no option to set it

7.2 requires the stream path to be outside `docs/`, outside `scripts/`, and not the operator's output
path. Rule 2's option list has no way to say where it should be instead.

Built with a `--stream-dir <path>` option, defaulting to a fresh temporary directory. This is an
option the specification does not list, which is the disclosure.

## 3. Rule 7.1's "one reusable path" and rule 2.8's `--jobs` cannot both be literal

One path and concurrent cells are incompatible. Built as one reusable path *per worker*, taken from a
pool. See `instrument-defects.md` defect 3 -- reading 7.1 as literally one path is what the first
implementation did, and it lost cells.

## 4. Rule 12.3's `refused` stage is unreachable per cell

The vocabulary gives a missing cell the stage `refused`, "rejected before execution". Every refusal
the specification defines -- rule 4.2's `llm` source, rule 15.1's usage errors -- is refused for the
whole sweep before any cell executes, and exits `2` with nothing written. There is no path on which
one cell is refused and the others run.

Built with the stage present in the vocabulary and never emitted. Five of the six stages are observed
in this packet -- `run`, `read` and `derive` in case B20, `crosscheck` in B17, `not_attempted` in phase
E's interruption -- and `refused` is the one that is not, because there is no way to reach it.

## 5. `crosschecks_passed` is a constant on any retained row

Rule 10 gives seven equalities and rule 10.3 makes a disagreeing cell a missing cell with stage
`crosscheck`. A row therefore exists only if all seven passed, so `crosschecks_passed` is `7` on every
retained row that has ever existed and can never be anything else.

Built as specified. The field is not useless -- it states which contract version's checks were run --
but a reader who treats it as a measurement is reading a constant.

## 6. Rule 14 states exit statuses for the driver only

14.1's table is the batch driver's. Rule 16's classifier has no status table anywhere in the
specification.

Built with two statuses: `0` for a report produced, and `2` for any input it cannot use -- a missing
file, an unreadable one, one carrying no `sweep` record, an unknown option. That follows rule 14.3's
reasoning, that an operator should see one convention across both instruments, and the code says so at
the `return`. There is no `1`, because the classifier has no work of its own to fail at, and no `3`,
which has no classifier meaning. Case Q17 measures `2` over a file with no `sweep` record; Q16
measures that a report over an incomplete sweep is still a `0` and carries the incompleteness.

## 7. Q4's missing-fact rule is absent from rule 16

`VER-MOK-019` case Q4 requires the classifier to state a fact it cannot compute as missing rather than
as zero. Rule 16 has no such rule, and rule 17.1's list of what a group states has no place to put
one.

Built to satisfy Q4: a row whose stream lacked a field is reported with the field absent, not zeroed.
Rule 16 does not require this and does not forbid it.

## 8. Rule 17.1 omits `attempted`

Rule 9.6 records the measurement that justifies two threat counters, attempted and effective. Rule
17.1's list of per-group figures names "each of the three `effective` counters" and does not mention
`attempted` at all.

Built with both, and case Q10 measures that `attempted` stays distinct from `effective` in the group
figures. This is a superset of 17.1, which is why it is a disclosure and not a conflict --
but a reader comparing the output to 17.1 will find a block that rule does not describe.

## 9. Rule 16.8's famine predicate was still unreached at 400 runs

16.8 states the `famine` predicate and discloses that measurement does not support it, on 35 runs. The
400-cell sweep does not reach it either: `famine` is 0 over 400 runs, against 12,827 recorded capacity
skips. At density `0.10` all 80 cells end extinct with 16 units of food standing across both
territories -- agents starve beside food they never reach, which is a spatial failure and not a
famine.

Built as specified, with the class reported as zero rather than omitted, per rule 16.9. Whether to keep
an unreached predicate at all is manual assessment 3.

## 10. `VER-MOK-019` case T6 cites a rule that does not record what it asks for

T6 requires the instruments not be build targets, and cites `SPEC-MOK-002` rule 5 for a census of the
workspace's targets. That rule does not record a target census. The case was executed against the
three `Cargo.toml` files directly -- 25 targets, none of them a Python file -- which is the fact T6
wants; the citation is what is wrong.
