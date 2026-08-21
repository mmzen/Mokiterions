# Completion report item 13: the nine updated tests, read by hand

| Field | Value |
|---|---|
| Item | `WO-MOK-016` completion report item 13, second clause — "with every updated test named and its assertion count before and after" |
| Measurement | `post/updated-tests.txt`, eight runs of `analysis/updated-tests.py`, five of them controls |
| What this adds | the reading behind the counts. Six of the nine updated tests hold the same assertion count on both sides, and a count cannot tell a widened sweep from a weakened check |
| Baseline | `39662d13abd08e3410648d1c59ad38384f8ad2d2` — 212 names, 212 passed, exit `0` (`baseline/test-run.txt`) |
| Candidate | `139061530f1dba72c9a20427eeaac6ce69492fb2` — the tree `post/updated-tests.txt` run 1 measures |
| Renumber | `d72465c4e7f2d874eef0fe5bd14ae25b2432ee40` — run 3, seven further bodies, read in §3 |
| Merge | `259859dffe1f5f856e154263c48d8d1e04808903` — 264 names, 264 passed, exit `0` (`post/test-run-merged.txt`) |
| Method | every body below was extracted from both extracted trees by the same parser the measurement uses and diffed line by line. No figure here is retyped from the record; both come from the same reader |
| Date | 2026-08-21 |

Stop condition 8 is reached if any existing test's assertions are **relaxed, widened, removed,
renamed away or `#[ignore]`d**. `post/updated-tests.txt` measures three of the five clauses and
says so: no retained test asserts fewer things than before, no test is `#[ignore]`d on either
side, and the one absent name is a rename resolved by body. The two it cannot measure are the
first: an assertion whose *content* was weakened while its count held. Six of the nine changed
bodies hold the same count, and this file is those six read one at a time, with the three whose
count rose read beside them so the nine are settled in one place rather than two.

The reading has a conclusion and one defect. The conclusion is that all six are the same edit —
a swept list of three policies became a swept list of four — so every assertion the baseline made
is still made, over strictly more sources. The defect is a comment in the seventh, which this file
states in §1.3 rather than fixing, for the reason given there.

---

## 1. The nine

| test | assertions | body lines | the change | reading |
|---|---:|---:|---|---|
| `src/simulation.rs :: a_name_is_the_same_value_at_both_ends_of_a_run` | 2 → 2 | 32 → 37 | `Policy::Social` appended to the swept list | widened, §1.1 |
| `src/simulation.rs :: naming_draws_nothing_and_reads_neither_the_seed_nor_the_configuration` | 4 → 4 | 31 → 36 | the same | widened, §1.1 |
| `src/simulation.rs :: the_trait_is_fixed_for_the_run_and_independent_of_every_configuration` | 2 → 2 | 28 → 33 | the same | widened, §1.1 |
| `tests/naming.rs :: every_run_reports_the_specified_twelve_names_in_identifier_order` | 2 → 2 | 33 → 38 | the same | widened, §1.1 |
| `tui/tests/authority.rs :: every_event_type_the_observer_can_present_has_an_entry` | 3 → 3 | 14 → 19 | the same, over a vocabulary that also grew | widened, §1.1 |
| `tui/tests/options.rs :: the_usage_text_advertises_every_policy_the_engine_accepts` | 3 → 3 | 13 → 19 | the same, plus a fourth arm the compiler required | widened, §1.2 |
| `tests/cli.rs :: the_entries_state_the_constraints_that_decide_validity` | 12 → 14 | 18 → 20 | two assertions appended | added to, §1.3 |
| `tui/tests/authority.rs :: the_decision_source_maps_by_the_source_the_record_names` | 5 → 6 | 15 → 16 | one assertion appended | added to, §1.4 |
| `tui/tests/authority.rs :: the_mapping_is_the_specified_one` | 1 → 2 | 19 → 23 | three rows added to the swept table, and a new tripwire | added to, §1.5 |

34 assertions before, 38 after. `src/` here is `mokiterions-core/src`, `tests/` is
`mokiterions-core/tests`, and `tui/` is `mokiterions-tui`.

### 1.1 Five of the six are one edit, repeated

The whole of the change to five of the six same-count tests is this, and it is the same three
lines in each:

    -        for policy in [Policy::Baseline, Policy::Reference, Policy::Individual] {
    +        for policy in [
    +            Policy::Baseline,
    +            Policy::Reference,
    +            Policy::Individual,
    +            Policy::Social,
    +        ] {

Every `+5` in the body-line column above is that reflow: `rustfmt` breaks a four-element array
literal onto one element per line where it kept a three-element one inline, so five of the six
tests gained exactly five non-comment lines and no statement.

What that does to each test's assertions is the point, and it is a widening in the strict sense —
the swept list is the baseline's list, in the baseline's order, with a fourth element appended:

| test | cases before | cases after | the sweep |
|---|---:|---:|---|
| `a_name_is_the_same_value_at_both_ends_of_a_run` | 3 | 4 | one full 1,000-tick run per policy |
| `naming_draws_nothing_...` | 45 | 60 | 3 densities × 5 declared seeds × policies |
| `the_trait_is_fixed_...` | 18 | 24 | policies × 3 densities × 2 tick limits |
| `every_run_reports_the_specified_twelve_names_in_identifier_order` | 90 | 120 | 5 seeds × policies × 3 densities × 2 trace settings |
| `every_event_type_the_observer_can_present_has_an_entry` | 36 | 60 | policies × `EventType::ALL` |

The last row grew on both axes, and the second axis is checked rather than assumed:
`EventType::ALL` went from 12 entries to 15, all 12 retained in their original relative order and
three inserted. So the 36 pairs this test checked at the baseline are a subset of the 60 it checks
now.

That subset relation is what makes "widened" a measurement instead of a claim. The baseline tree
*is* the three-element sweep, `baseline/test-run.txt` records all 212 of its cases passing at exit
`0`, and the diffs above show the three existing elements unaltered and in place. A relaxed
assertion would have had to change a line, and in these five tests no line changed except the
array literal.

### 1.2 The sixth is the one the compiler forced

`tui/tests/options.rs :: the_usage_text_advertises_every_policy_the_engine_accepts` took the same
sweep edit and one more, because its body maps each policy to the name the help must advertise:

             let name = match policy {
                 Policy::Baseline => "baseline",
                 Policy::Reference => "reference",
                 Policy::Individual => "individual",
    +            Policy::Social => "social",
             };

The `match` is wildcard-free and deliberately so. Adding a variant to `Policy` without adding this
arm is `error[E0004]` and the file does not compile, which is the strongest form the check takes
and is unchanged in kind: the test's own comment moved from "a fourth policy stops the compilation
here" to "a fifth policy stops the compilation here … The fourth one did exactly that under
`WO-MOK-016`". Its three assertions now run four times instead of three, over the same two
claims — the engine parses the advertised name, and the observer's parsed configuration carries
the policy that name selects.

### 1.3 `the_entries_state_the_constraints_that_decide_validity`: two appended, and a stale comment

Twelve assertions became fourteen by appending two, in the two places the entry's existing pattern
puts them — one that the help text names the value, one that the parser accepts it:

         assert!(policy.contains("individual"), "{policy}");
    +    assert!(policy.contains("social"), "{policy}");
         assert!(parse(["--policy", "baseline"]).is_ok());
         assert!(parse(["--policy", "reference"]).is_ok());
         assert!(parse(["--policy", "individual"]).is_ok());
    +    assert!(parse(["--policy", "social"]).is_ok());
         assert!(parse(["--policy", "random"]).is_err());

The twelve are untouched, `--ticks` and `--density` are untouched, and the rejection assertion
still closes the block.

The comment above them was not updated, and at the merge tip it still reads:

    // The value set is stated in the placeholder, so the whole entry is read here. Every value
    // the parser accepts is named and every value it names is accepted, so the help can neither
    // hide the third source nor advertise a fourth.

Both counts in the last clause are now wrong: there are four sources, and a fourth is exactly what
the help is now required to advertise. Nothing the test asserts is affected — the sentence before
it still describes what the assertions do, and it is the one that carries the reason. But it is a
defect, and it is the same class of edit the change made correctly one file away, in §1.2's
comment. It is stated here rather than fixed because every figure in this packet is bound to a
commit: correcting three words in `mokiterions-core/tests/cli.rs` moves the tree that
`post/test-run-merged.txt`, `post/test-census-merged.txt`, `post/test-census-reconciliation.md` §9
and all eight runs of `post/updated-tests.txt` were taken against, and `SPEC-MOK-004` rule 11
requires the figures to be re-derived rather than edited. Whether a comment is worth that
re-derivation, or belongs with `REQ-MOK-060` in the follow-on work order, is the owner's call and
not this file's.

### 1.4 `the_decision_source_maps_by_the_source_the_record_names`: one appended

Five assertions became six, appended to the run of three that name each source's authorizing
requirement:

         assert_eq!(for_event(&source("baseline")), Some("REQ-MOK-008"));
         assert_eq!(for_event(&source("reference")), Some("REQ-MOK-015"));
         assert_eq!(for_event(&source("individual")), Some("REQ-MOK-033"));
    +    assert_eq!(for_event(&source("social")), Some("REQ-MOK-057"));

         // A source the observer does not know is reported as missing, never guessed.
         assert_eq!(for_event(&source("something-else")), None);
         assert_eq!(for_type(EventType::DecisionSourceSelected, None), None);

The two `None` assertions are the ones a fourth source could most easily have loosened — an
unknown source is still reported as missing rather than resolved to the new arm — and both stand
unaltered. The identifier in the added line is `REQ-MOK-048` at the candidate and `REQ-MOK-057`
after the renumber; §3 is where that substitution is read.

### 1.5 `the_mapping_is_the_specified_one`: one assertion, then two

This test carried a single assertion, inside a loop over a table transcribed from `SPEC-MOK-003`
rule 11. The table gained three rows:

    +    // Rule 11's three added rows. `attack_resolved` carries one identifier for `attack` and
    +    // for `fight` alike, because they are one resolution, and `REQ-MOK-052` has no row of its
    +    // own because it adds no event type.
    +    (EventType::AttackResolved, "REQ-MOK-053"),
    +    (EventType::ThreatResolved, "REQ-MOK-055"),
    +    (EventType::SurrenderResolved, "REQ-MOK-056"),

so the one pre-existing assertion — `assert_eq!(for_type(event_type, None), Some(identifier))` —
now runs over 14 rows where it ran over 11. The second assertion did not exist at the baseline:

    +    // The table above is the whole of rule 11 minus its one source-dependent row, so its length
    +    // plus that row is the vocabulary. A fourth added type would fail here rather than pass
    +    // untested.
    +    assert_eq!(expected.len() + 1, EventType::ALL.len());

It is an exhaustiveness tripwire rather than a mapping check: a variant added to `EventType::ALL`
without a row here fails, instead of passing untested. It is also the assertion
`post/observer.md` §5's control A fires, and `post/merge-recheck.txt` §7 re-fires at the merge
tip — deleting the `ThreatResolved` row there reports `left: 14, right: 15` at
`tests/authority.rs:70`. So this is a check whose failure path has been read twice, at two
commits.

`post/observer.md` §6 records the residual on the other side, and this file does not narrow it:
the assertion catches a variant added to `ALL` without a table row, and not a variant added to
`EventType` and omitted from `ALL`.

---

## 2. The tenth name, which is a rename

`post/updated-tests.txt` run 1 reports one name absent on the candidate side:

    gone     mokiterions-tui/tests/verification.rs :: no_shipped_decision_source_has_a_proposal_rejected
      ->     no_source_confined_to_the_valid_action_list_has_a_proposal_rejected
             assertions: 1 before, 1 after; body identical with the name blanked

There is nothing to read by hand: the two bodies are the same bytes once each side's own function
name is blanked out, which is the reader's rename test and is why the pairing is a measurement
rather than a judgement. `post/test-census-reconciliation.md` §3 reconciles the same rename
independently, from the census logs, and records the shared body's digest. Controls C and D of
`post/updated-tests.txt` are the two runs that show the matcher would have refused this pairing
had the body moved at all.

---

## 3. The renumber's seven, read the same way

`d72465c` moved this chain out of four occupied identifier ranges. Ten Rust files differ across
it; seven test bodies do. The difference is doc comments sitting above `#[test]` rather than
inside the body, and non-test code — `mokiterions-tui/src/authority.rs`'s mapping arms among
them. `post/updated-tests.txt` run 3 measures the seven at 250 names on both sides, 25 assertions
on both sides, and not one changed non-comment body line.

| test | assertions | what moved inside the body |
|---|---:|---|
| `src/simulation.rs :: a_threat_composes_with_rule_12_in_turn_order_and_outlasts_its_tick` | 8 → 8 | one comment: `REQ-MOK-048` → `-057` |
| `tests/decisions.rs :: every_targeted_verb_applies_somewhere_in_the_declared_matrix` | 2 → 2 | one comment: `REQ-MOK-048` → `-057` |
| `tests/viability.rs :: survival_by_turn_position_stays_inside_the_stated_bound` | 2 → 2 | two inner doc comments: `VER-MOK-012` → `-016`, `REQ-MOK-049` → `-058` |
| `tui/tests/options.rs :: the_usage_text_advertises_every_policy_the_engine_accepts` | 3 → 3 | one comment: `WO-MOK-012` → `-016` |
| `tests/viability.rs :: the_social_source_keeps_the_world_habitable_and_combat_lethal` | 2 → 2 | an inner doc comment, and a `println!` label: `REQ-MOK-049` → `-058` |
| `tui/tests/authority.rs :: the_decision_source_maps_by_the_source_the_record_names` | 6 → 6 | an assertion's expected value: `REQ-MOK-048` → `-057` |
| `tui/tests/authority.rs :: the_mapping_is_the_specified_one` | 2 → 2 | three of the swept table's expected values: `044/046/047` → `053/055/056` |

Four of the seven are comment text and change nothing a test can fail on. The bottom three are
the ones worth stating, because two of them assert *which* identifier the mapping names, and an
identifier renumber that had moved only one side of that pairing would leave the assertion
unweakened and wrong.

It did not, and the check is not this file's reading: `mokiterions-tui/src/authority.rs` moved in
the same commit, and `post/test-run-merged.txt` records all 264 cases passing at exit `0` at the
merge tip, `the_mapping_is_the_specified_one` and
`the_decision_source_maps_by_the_source_the_record_names` among them. A one-sided renumber fails
there. The `println!` in the third is a label on retained output and carries no assertion.

---

## 4. Stop condition 8, clause by clause

| clause | how it is settled |
|---|---|
| relaxed | read, §§1.1–1.5. Nine changed bodies; five are one appended array element, one is that plus a compiler-required match arm, three append assertions. No existing assertion's text changed except `the_mapping_is_the_specified_one`'s table rows, which grew, and §3's three identifier substitutions |
| widened | the same reading, and the same nine. "Widened" here means the loosening sense of the word; §1.1's five widen their *sweep*, which is the opposite |
| removed | measured. `post/updated-tests.txt` run 1 §3: 0 retained tests assert fewer things than before, 202 retained bodies byte-identical. Control B is one deleted `assert!` in a target this change does not touch, and it fires |
| renamed away | measured. One absent name, paired by body, §2. Control C is that rename with its body changed and is refused; control D is the rename alone and is accepted |
| `#[ignore]`d | measured. 0 before, 0 after, in run 1 §4; and `post/merge-recheck.txt` §4 re-greps the merge tip at 0 |

---

## 5. What this file does not settle

`post/updated-tests.txt`'s four closing bullets stand as written, and this file discharges the
first of them. Two bounds are its own:

- It reads the nine updated bodies and the renumber's seven. It does not read the 39 added tests.
  That those additions assert what `SPEC-MOK-001` and `SPEC-MOK-003` require is `VER-MOK-016`'s
  question, argued row by row there; 206 assertions is a volume and not a claim about coverage.
- It reads source text. Where a claim needed a run — the baseline's 212 passing, the merge tip's
  264, control A's `left: 14, right: 15` — the figure is cited from the record that holds it
  rather than re-taken here, so the two files cannot disagree.
