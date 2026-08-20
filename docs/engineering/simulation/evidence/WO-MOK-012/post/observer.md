# The observer's authority table, and what `EventType::ALL`'s exhaustiveness rests on

| Field | Value |
|---|---|
| Retention item | "the observer authority table's new rows and the `EventType::ALL` exhaustiveness check" |
| Matrix row | `VER-MOK-012`, both packages, automated-test, "New event types are authorized" |
| Static-analysis clause | "`EventType::ALL`'s length equals the number of variants, asserted against the variant set rather than against a literal, so that a variant added without a table row fails here" |
| Authority | `SPEC-MOK-003` rule 11, as amended 2026-08-20 — three rows added, `REQ-MOK-043` takes none — and `SPEC-MOK-001`'s *Data and interface contracts*, where the vocabulary gains three types |
| Baseline | `39662d13abd08e3410648d1c59ad38384f8ad2d2` (`baseline/COMMIT.txt`) |
| Candidate | `7fda440237e663049c7855f18ff3a97ea1dc9cdd`. The four files read here — `mokiterions-tui/src/authority.rs`, `mokiterions-tui/tests/authority.rs`, and `simulation.rs`'s `EventType` and `EventType::ALL` — are byte-identical at `7c4aef39` and at this commit, so this reading holds at both candidates |
| Date | 2026-08-20 |

The material here is source text and test outcomes, not stream bytes. Where a figure comes from a run it
is `post/runs.md`'s figure, cited rather than recaptured, so the two files cannot disagree.

Five one-line mutations are used as controls, in the way `analysis/regions.py`'s three controls are used:
a check that passes is worth reading only if something is known to make it fail. Each is stated exactly
enough to re-apply, each was reverted, and the worktree was clean afterwards.

---

## 1. The item is two obligations, and they are different kinds of thing

"The authority table's new rows" is a comparison against a table: rule 11 names fifteen event types and
fifteen identifiers, and the code either agrees with it row for row or does not. That is finite and it is
settled in §2.

"The `EventType::ALL` exhaustiveness check" is a property, and a property has directions. There are three
worth separating, and they are not enforced by the same thing:

| direction | enforced by |
|---|---|
| every `EventType` variant has an authorizing identifier | the compiler — §3 |
| a variant added to `ALL` without a row in the mapping table fails a test | `tests/authority.rs`, and §5 measures it |
| a variant added to the enum but omitted from `ALL` fails something | **nothing** — §6 |

The third is not the direction the contract's clause names, and §6 says so rather than reporting it as a
contract failure. It is recorded because the clause is the only place in this work order where `ALL`'s
completeness is discussed at all, and a reader who takes "exhaustiveness check" at its widest reading
would otherwise be misled about what is checked.

---

## 2. Rule 11's fifteen rows against the code, item for item

`as_str` is the `event=` string `SPEC-MOK-001` fixes; `for_type` is `mokiterions-tui/src/authority.rs`.
The order of the rows below is rule 11's own order, and it is also `EventType::ALL`'s order — §7.

| # | rule 11 row | `EventType` variant | `for_type` arm | identifier |
|---|---|---|---|---|
| 1 | `world_initialized` | `WorldInitialized` | shared arm 1 | `REQ-MOK-001` |
| 2 | `food_initialized` | `FoodInitialized` | shared arm 1 | `REQ-MOK-001` |
| 3 | `agent_initialized` | `AgentInitialized` | own arm | `REQ-MOK-002` |
| 4 | `decision_source_selected` | `DecisionSourceSelected` | nested on the source | `REQ-MOK-008` / `REQ-MOK-015` / `REQ-MOK-033` / **`REQ-MOK-048`** |
| 5 | `survival_changed` | `SurvivalChanged` | shared arm 2 | `REQ-MOK-003` |
| 6 | `agent_died` | `AgentDied` | shared arm 2 | `REQ-MOK-003` |
| 7 | `food_consumed` | `FoodConsumed` | own arm | `REQ-MOK-006` |
| 8 | `food_regenerated` | `FoodRegenerated` | shared arm 3 | `REQ-MOK-007` |
| 9 | `food_regeneration_skipped` | `FoodRegenerationSkipped` | shared arm 3 | `REQ-MOK-007` |
| 10 | `territory_crossed` | `TerritoryCrossed` | own arm | `REQ-MOK-005` |
| 11 | **`attack_resolved`** | **`AttackResolved`** | **own arm, added** | **`REQ-MOK-044`** |
| 12 | **`threat_resolved`** | **`ThreatResolved`** | **own arm, added** | **`REQ-MOK-046`** |
| 13 | **`surrender_resolved`** | **`SurrenderResolved`** | **own arm, added** | **`REQ-MOK-047`** |
| 14 | `simulation_ended` | `SimulationEnded` | own arm | `REQ-MOK-011` |
| 15 | `action_trace` | `ActionTrace` | own arm | `REQ-MOK-012` |

Fifteen rows, fifteen variants, fifteen identifiers, and no row without a variant or variant without a
row. The growth, measured on both sides:

| | baseline | candidate |
|---|---:|---:|
| rows in rule 11's table | 15 | 15 |
| `EventType` variants | 12 | 15 |
| `EventType::ALL` | `[Self; 12]` | `[Self; 15]` |
| `Policy` variants | 3 | 4 |
| `for_type` match arms / types covered | 9 / 12 | 12 / 15 |
| identifiers on the source row | 3 | 4 |

**Rule 11's table has fifteen rows at the baseline commit too, and that is the ordering working rather
than a defect.** The specification amendment was approved and committed with the rest of the artifacts
before implementation began, so the baseline commit — the commit the work begins from — already carries
the amended table while the code still carries twelve types. What this work order closes is that gap, and
the three rows and the fourth source arm are the whole of the closing.

### `REQ-MOK-043` takes no row, and the absence is required

Rule 11's amendment says why: the table maps event types, and `REQ-MOK-043` "authorizes seven verbs while
adding no event type of its own". The code says the same thing in a comment above the three added arms,
and neither the mapping nor the test list has an entry for it. An entry would have no event type to key
on. Three of the seven verbs — `approach`, `avoid`, `retreat` — resolve as rule 8 moves and emit only what
a move emits; the other four resolve into the three rows above, because `attack_resolved` is shared by
`attack` and `fight`, which are one resolution.

So the count is 3 rather than 7, and rule 11 clause 2's exhaustiveness runs from the event side, where it
is decidable. §3 is why that side cannot be short.

### The presented table is `ALL` mapped through the two functions

`authority::table(policy)` iterates `EventType::ALL`, keys each entry by `as_str`, and takes the
identifier from `for_type` — except on row 4, where it substitutes a hand-written string naming all four
sources at once, because the overlay presents the mapping and not one run's resolution of it. Two
consequences follow, and both matter later: the presented table's length is `ALL`'s length, which
`tests/authority.rs` asserts; and row 4's four-way string is the one identifier in the table that no
call to `for_type` produces, which is why §6 measures it separately.

---

## 3. The mapping cannot omit a type, and the compiler is what makes that true

`for_type`'s `match` names all fifteen variants and has **no wildcard arm**. That is not a stylistic
observation, it is the enforcement of rule 11 clause 2: a sixteenth variant makes the observer's library
fail to compile.

Control C — add a variant to `EventType`, to `EventType::ALL` (raising `[Self; 15]` to `[Self; 16]`) and
to `as_str`, and leave `for_type` alone:

    error[E0004]: non-exhaustive patterns: `EventType::Control16` not covered
        --> mokiterions-tui\src\authority.rs:20:16
        --> mokiterions-core\src\simulation.rs:1280:1
         = note: the matched value is of type `EventType`
    error: could not compile `mokiterions-tui` (lib) due to 1 previous error

The observer's library does not build, so no test of either package runs and no gate is reached. This is
the strongest form the clause could take, and it costs nothing to hold: it holds as long as nobody adds a
`_ =>` arm to `for_type`, and adding one is a visible edit to a nine-line function whose doc comment says
what it is for.

`for_event` inherits it. It resolves an `Event` by calling `event.event_type()`, and
`EventDetail::event_type`'s match over the fifteen payload variants is likewise exhaustive with no
wildcard, so a new detail cannot reach the overlay without a type, and a new type cannot reach it without
an identifier. The one deliberately reachable `None` is a `decision_source_selected` record naming a
source the observer does not know, which rule 11 clause 2 requires be stated as missing rather than
guessed, and `tests/authority.rs` exercises it in both forms.

---

## 4. The checks that run

| test | census target | asserts |
|---|---|---|
| `every_event_type_the_observer_can_present_has_an_entry` | `tests/authority.rs` | `for_type` is `Some` and starts with `REQ-MOK-` for every entry of `ALL` under each of the **four** policies, and `table(policy).len() == ALL.len()` |
| `the_mapping_is_the_specified_one` | `tests/authority.rs` | fourteen named `(variant, identifier)` pairs, and `expected.len() + 1 == EventType::ALL.len()` |
| `the_decision_source_maps_by_the_source_the_record_names` | `tests/authority.rs` | the four sources by the name the record carries, and both `None` paths |
| `an_ordinary_record_resolves_from_its_own_payload` | `tests/authority.rs` | `for_event` on a whole `Event`, through `event_type()` |
| `the_declared_sets_are_the_contracts` | `tests/verification.rs` | the same sweep over `ALL`, from the other suite, under `Policy::Reference` |
| `the_type_filter_cycles_the_whole_vocabulary_then_returns_to_none` | `tests/state.rs` | the filter reaches every entry of `ALL` in `ALL`'s order and then clears |
| `a_filter_changes_what_is_presented_and_nothing_else` | `unittests :: verification` | `ALL.len() + 1` cycles of the filter change presentation and leave the export and the record identical |
| `the_authority_overlay_names_identifiers_for_every_event_type` | `unittests :: render::tests` | the rendered overlay carries the highlighted line, a mapped row and rule 11.1's note |

The middle assertion of row 2 — `expected.len() + 1 == EventType::ALL.len()` — **is added by this work
order**; the baseline's version of that test ends at its loop. The `+ 1` is row 4, the one row `expected`
omits because its identifier is source-dependent. That single line is what the contract's static-analysis
clause asks for, and §5 measures that it earns its place.

Four of the eight cases above gained coverage here rather than being written from nothing: the sweep runs
under four policies instead of three, the named-pair list carries the three added rows, the source case
names `social`, and the length assertion is new. Nothing was removed, relaxed or widened;
`post/test-census-reconciliation.md` is where that claim is settled name by name for the whole suite.

---

## 5. Controls A and B: the two assertions that carry the table

**Control A — a row missing from the mapping table.** Delete
`        (EventType::ThreatResolved, "REQ-MOK-046"),` from `expected` in `tests/authority.rs` and run
`cargo test --locked -p mokiterions-tui --test authority`:

    test the_mapping_is_the_specified_one ... FAILED
    thread 'the_mapping_is_the_specified_one' panicked at mokiterions-tui\tests\authority.rs:70:5:
    assertion `left == right` failed
      left: 14
     right: 15
    test result: FAILED. 3 passed; 1 failed

(The panic is at line 70 of the mutated file, which is line 71 unmutated, the deletion having shifted it.)
This is the contract clause's own sentence measured: a variant in `ALL` "without a table row fails here".
Note which test did **not** fail — `every_event_type_the_observer_can_present_has_an_entry` passed, because
the type still resolves to a well-formed identifier. The sweep cannot see a missing row.

**Control B — a plausible wrong identifier.** Change `for_type`'s arm to
`EventType::ThreatResolved => "REQ-MOK-047",`, which is `surrender_resolved`'s identifier and therefore
well-formed, existing and wrong:

    test the_mapping_is_the_specified_one ... FAILED
    thread 'the_mapping_is_the_specified_one' panicked at mokiterions-tui\tests\authority.rs:65:9:
    assertion `left == right` failed: threat_resolved
      left: Some("REQ-MOK-047")
     right: Some("REQ-MOK-046")
    test result: FAILED. 3 passed; 1 failed

Again the sweep passed. Both controls fail in the same test and for different reasons, which is the point:
the sweep establishes that the mapping is total and says nothing about what it maps to, and the named-pair
list is the only thing standing between the three added rows and three plausible identifiers.

---

## 6. Two things no check would catch, measured

**Control D — a variant absent from `EventType::ALL`.** Add a sixteenth variant to `EventType`, an arm to
`as_str`, and an arm to `for_type` — the two the compiler demands — and leave `ALL` at fifteen entries:

    $ cargo test --locked --workspace --no-fail-fast
    250 passed, 0 failed, exit 0

    $ cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
    exit 0

Nothing fails. A `pub` enum variant that is never constructed is part of the crate's API, so `dead_code`
does not fire, and every consumer of the vocabulary iterates `ALL` rather than the enum: `table()` maps
it, `cycle_type_filter` walks it, and both suites' sweeps loop over it. `expected.len() + 1 == ALL.len()`
compares fifteen with fifteen and passes.

What such a defect would cost is presentational and silent. The type would resolve to its identifier
whenever a record of it was highlighted, because the compiler forced `for_type` to name it — so rule 11
clause 2 would hold — while the overlay's table would omit its row and the `e` filter could never select
it. **This is not the direction the contract's clause names**, and control A is the direction it does
name; it is recorded here because it is the one reading of "exhaustiveness" that nothing in the workspace
supports, and because a reader of this packet should not have to discover that by experiment.

**Control E — the overlay's source row stops naming `social`.** Remove `/ REQ-MOK-048 social` from
`table()`'s hand-written string for row 4, so the overlay lists three sources where rule 11 lists four:

    $ cargo test --locked -p mokiterions-tui --no-fail-fast
    128 passed, 0 failed, exit 0    (89 across the eight integration targets,
                                     31 library units, 8 binary units)

    $ cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
    exit 0

That string is the one identifier in the presented table no call to `for_type` produces, and no test
asserts its content: `every_event_type_…` asserts row 4's *presence* by asserting the table's length, and
the render case asserts the overlay contains `REQ-MOK-015`, which survives the mutation. `for_type`'s
`Social` arm is asserted twice — by the four-policy sweep and by
`the_decision_source_maps_by_the_source_the_record_names` — so what is unchecked is the row's rendering
and not the mapping behind it.

Both residuals are stated and neither is corrected here. Closing either is one assertion, which is
implementation and not evidence, and it would move a test census that this packet has already reconciled
name by name at 250. What to do about them is the accountable owner's call; §10 records that this file
prepares the question and does not answer it.

---

## 7. `ALL`'s order is rule 11's order, and the three are inserted where the amendment lists them

`EventType::ALL`'s fifteen entries, read top to bottom, are §2's fifteen rows read top to bottom. The
agreement is exact, including the two places where the enum's order is not alphabetical or obvious
(`survival_changed` and `agent_died` before `food_consumed`), and it holds at both commits for the twelve
types that existed at the baseline.

The three added entries sit between `TerritoryCrossed` and `SimulationEnded`, which is where rule 11's
amended table puts them and where `SPEC-MOK-001`'s *Data and interface contracts* lists them among the
stable core types. The engine's doc comment on `ALL` states the reason — "so that no existing type moves
relative to any other" — and two consumers depend on it:

- `cycle_type_filter` (`mokiterions-tui/src/state.rs:550`) walks `ALL` in order, so the first ten steps of
  the `e` filter cycle are the same ten they were, and the three new types are reached at steps 11 to 13
  rather than displacing anything.
- `table()` renders in `ALL`'s order, so the overlay's first ten rows are unchanged and
  `simulation_ended` and `action_trace` are still the last two, three rows further down.

Two bounds on that paragraph. First, `ALL`'s order is verified against rule 11 here **by inspection and by
no test**: `expected` in `the_mapping_is_the_specified_one` is written in rule 11's order, but its
assertion is per-variant, and `the_type_filter_cycles_the_whole_vocabulary_then_returns_to_none` compares
the cycle against `ALL` itself, which is self-referential as to order. Second, no rule fixes `ALL`'s order
at all, so the agreement is a property this implementation has rather than an obligation it meets — which
is also why the one place `ALL` and `SPEC-MOK-001`'s prose list diverge is not a defect: that list names
`survival_changed` and `agent_died` twelfth and thirteenth, and it does so at both commits.

`post/interface.txt` §7 carries the other consequence of the insertion point, which is on discriminants
rather than on order: `SimulationEnded` moves from 10 to 13 and `ActionTrace` from 11 to 14, while every
pair of pre-existing variants keeps its `Ord` relation.

---

## 8. "Every event type the observer can present" is not vacuous

Rule 11 clause 2 is about the types the observer *can* present, so the three added rows are worth nothing
if the three types are never emitted. They are emitted, unconditionally, one per resolution:

| type | emitted at | count over the fifteen traced `social` cells |
|---|---|---:|
| `attack_resolved` | `simulation.rs:2653`, in the strike resolution | **156** = 143 `attack` + 13 `fight` |
| `threat_resolved` | `simulation.rs:2710`, in the threat resolution | **620** |
| `surrender_resolved` | `simulation.rs:2759`, in the surrender resolution | **95** |

Each of the three construction sites is followed immediately by `self.emit(output, event)?` with no
condition on it, and in particular none on `--trace-actions`: the resolutions emit their records whether
tracing is on or off, and only the `action_trace` line is conditional. The verb counts are
`post/runs.md` §3's applied column and the resolution counts are its §6's `156 strikes, 620 threats, 95
surrenders`, which is the same measurement read twice and agreeing. Because the fifteen trace pairs agree
on the strikes, threats and surrenders columns — `post/runs.md` §1 — the fifteen untraced cells carry the
same figures, so all thirty `social` cells hold 312, 1,240 and 190.

On the pre-change side none of the three exists. `baseline/census.txt` lists the event kinds present in
each of the 90 cells and no cell lists any of the three, and the same file records all seven targeted
verbs at zero occurrences in 110 MB. So the three rows added to the table are three rows for types that
appear in the candidate's streams and in no capture taken before it.

---

## 9. The overlay is three lines longer, and at the floor viewport that costs two rows

The mapping is presented by `render::authority_lines`, whose length is a function of the vocabulary size:
one line per row, plus a highlighted line, an identifier line for it, two lines for the inspector and
perceived-entity identifiers rule 11 names outside its table, rule 11.1's note, and three blanks.

|  | baseline | candidate |
|---|---:|---:|
| lines, with an event highlighted | 20 | **23** |
| lines, with none highlighted | 19 | 22 |
| the longest other overlay (`help_lines`) | 20 | 20 |

The overlay is drawn into `layout::resolve`'s `overlay` rect, whose height is the viewport height less
`HEADER_HEIGHT` 3 and `FOOTER_HEIGHT` 1, inside a bordered `Block`, as a `Paragraph` carrying **no
`.scroll()`**. So the visible interior is `height - 6` lines and anything past it is clipped. For the nine
viewports `tests/verification.rs` declares renderable:

    160x48 -> 42    160x44 -> 38    160x40 -> 34
    140x44 -> 38    140x43 -> 37    120x48 -> 42
    120x30 -> 24    100x30 -> 24     34x22 -> 16

Twenty-three lines fit at eight of the nine. At the floor viewport `34x22` the interior is sixteen lines,
which now holds the highlighted line, its identifier, a blank and **thirteen of the fifteen rows**; the
same sixteen lines held all twelve rows and a blank at the baseline. What rule 11 obliges — "the `t`
control presents it for the highlighted event type" — is the first two lines and is never clipped; the
whole-table listing is more than the rule asks for, and it is what loses two rows. The authority overlay
is also now the longest of the four, where at the baseline it tied with the key bindings at twenty.

These figures are arithmetic on the retained layout constants, not measurements of a rendered frame, and
they are stated as such. The frame bound itself is a different clause of `VER-MOK-012` — "the observer's
frame remains within the bound `SPEC-MOK-003` states with the new event types ingested" — and it is not
this retention item.

---

## 10. What this file does not establish

- Nothing here is a verification verdict. `VER-MOK-012` is the contract, `VREC-MOK-012` the record.
- The five controls are mutations of retained source, applied one at a time, each reverted immediately.
  `git status` was empty after the last of them. They are reproducible from the edits stated above and
  from nothing else; no instrumented build survives, and no figure in this file depends on one.
- The emission counts in §8 are `post/runs.md`'s figures cited for a second purpose. This file takes no
  capture and runs no simulation.
- §9's line counts and pane heights are derived from source constants. Whether a given terminal clips a
  given row is a rendered-frame question this file does not answer.
- The two residuals in §6 are measured, not judged. Neither is a failure of a clause as written, both are
  one assertion away from closed, and closing either would move the reconciled test census — so the
  decision is the accountable owner's and is not taken here.
- `REQ-MOK-004` and `REQ-MOK-013`, the two identifiers rule 11 names outside its table, are unchanged at
  both commits and are not part of this growth. They are asserted by the render and verification cases
  above and are mentioned here only so that their absence from §2's table is not read as an omission.
- The observer gains no authority from any of this. It presents three more kinds of record and decides
  nothing, which is `ADR-MOK-001`'s boundary unchanged; the enumeration that settles who may write state
  is `post/reads.md` §7, not this file.
