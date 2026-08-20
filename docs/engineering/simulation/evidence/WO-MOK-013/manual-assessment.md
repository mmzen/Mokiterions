# Manual assessment record — WO-MOK-013

`VER-MOK-013` contracts **two** manual assessments and states that they "are the primary evidence for
their requirements rather than a confirmation of the automated cases". Both are recorded here.

**Assessment 1 was taken on 2026-08-20 and is SATISFIED. Assessment 2 is OUTSTANDING with no author,
and the route to closing it has been decided.** An assessment with no author is outstanding —
`VER-MOK-013`'s *Evidence retention* says so in the same sentence that requires an author, a date, a
role and a terminal, and `VREC-MOK-005` disclosing seven such is what this chain descends from. The
implementation agent could take neither: one required a person at a real terminal judging legibility,
and the other requires a person who has not read `SPEC-MOK-003` rule 7. Recording a status against
either would have put a signature where no person looked, which is the failure mode
`evidence/WO-MOK-012/manual-assessment.md` assessment 7 refused and stated its reason for.

The material and the procedure are prepared below so that each can be taken in one sitting. The
procedure is the one `evidence/WO-MOK-012/procedure-defects.md` corrected — its three defects are not
reintroduced here. Assessment 1 was taken on the procedure exactly as it stands below.

## 1. Reference-viewport gauge legibility

**Status: SATISFIED. Author: the repository owner. Date: 2026-08-20. Role: product owner. Terminal:
Windows Terminal on Windows 11.**

**The outcome as given:** *"SATISFIED — the bar carried it."*

The terminal field is recorded as the owner stated it. The host is Windows 11 Home 10.0.26200; that
build figure is the agent's, read from the session environment, and was not part of the answer.

**This assessment is `REQ-MOK-047`'s primary evidence and it now passes.** The 2026-08-20 adverse
observation on the roster gauges — the observation this whole chain exists to answer — is answered by
this row and by nothing else in this pack.

`VER-MOK-013`: at `160 × 48` on a real terminal, over a run of at least 200 ticks on a declared seed,
confirm that a Mokiterion's decline is visible as a change in the bar and not only in the number.

**This is assessment 2 of `VER-MOK-005` re-taken, and its earlier outcome is inherited in neither
direction.** On 2026-08-20 it returned "SATISFIED as to the subject `VER-MOK-005` enumerates. ADVERSE
OBSERVATION on the roster gauges", which is the observation this chain exists to answer. A pass here
does not follow from the arithmetic below and a fail here is not contradicted by it.

**Procedure.** In an interactive terminal at least 160 × 48:

```
cargo run -p mokiterions-tui -- --seed 42 --ticks 300 --policy reference --speed 8
```

`--ticks` is required: the default is 100 and the assessment needs 200. `--policy reference` is
required: seed 42 under baseline reaches extinction at tick 142. Watch one Mokiterion's health or
satiety gauge fall and judge whether the bar moved, without reading the number beside it.

**Which decline to watch.** "Watch one Mokiterion's gauge fall" left the assessor to hunt for a subject,
so one was traced out of `non-perturbation-unobserved.txt` before the assessment was put. **M01** under
this exact invocation gives two windows, and `--speed 8` advances eight ticks per second:

| Window | Ticks | What falls | Wall clock at `--speed 8` |
|---|---|---|---|
| Satiety, unfed | 200 → 260 | satiety 61 → 1, exactly one point per tick for all 60 steps, no refill in the window | about 7.5 s |
| Health, terminal | 261 → 279 | health 100 → 10 in eighteen five-point steps, then to 0 at tick 279: `tick=279 subject=M01 event=agent_died result=health:0` | about 2.4 s |

Both windows are re-derived from `non-perturbation-unobserved.txt` for this record rather than carried
from the note the assessment was put on.

**What was disclosed before the judgment, because it is adverse to a clean pass.** M01's health falls in
**five**-point steps, and `REQ-MOK-047` guarantees only that a change of **ten** moves the fill. At the
reference viewport's 13 cells a five-point step is 0.65 of a cell, so roughly every other one of those
eighteen steps draws the same bar as the step before it. The satiety window is the one that exercises
the guarantee cleanly; the health window is the harder case and was named as such. This was put to the
owner before they judged, so that the assessment was taken against the property the requirement states
rather than against a guarantee never made.

**Material.** `frames.txt` in this directory carries the reference viewport at tick 200 with all
twelve entries; `gauge-resolution.md` and `gauge-resolution.txt` carry what the gauge can and cannot
resolve. The before form is `evidence/WO-MOK-012/assessment-material/bar-quantization.txt`.

**What the material does not settle.** The gauge now has 14 renderable states where it had 3, and a
ten-point step moves the fill at every value. Neither figure is this assessment: 14 states over 101
values still means two values seven apart can draw the same bar, and whether that reads as movement
to a person watching is what the assessment asks. `VER-MOK-013`'s residual uncertainty records that a
person's terminal is not the buffer.

## 2. Overlay discoverability

**Status: OUTSTANDING. Author: none. Date: —. Role: product owner. Terminal: —.**

**The route to closing it was decided on 2026-08-20: find an admissible assessor.** Of the three
options below the owner took the first, as an **assurance-owner** act on the ground that it is the only
one that actually verifies `REQ-MOK-048` — the second amends the contract to something weaker and the
third closes the row without testing the property. The other two are recorded as declined rather than
left standing. The assessment itself remains outstanding: a route is not an outcome.

**What the decision obliges each side to do.** The agent prepares the frame and the exact question; the
owner finds the person. Both halves of the agent's share are delivered:
`discoverability-frame.txt` and `discoverability-frame-floor.txt` carry the two frames, and
`discoverability-assessment.md` is the packet an administrator can work from without reading
`SPEC-MOK-003` rule 7 to the assessor. Nothing further is blocked on this agent.

`VER-MOK-013`: show one frame to a person who has not read `SPEC-MOK-003` rule 7 and ask how they
would find out what the keys do. A pass is that they name the key from the screen.

**Procedure.** Draw one frame — the run may be held — and show it without saying anything about keys.
Ask the question as written. If the assessor finds the hint but cannot tell what it opens, that is an
adverse observation on the wording and belongs to `SPEC-MOK-003` rule 5, not to the implementation.

**Material.** `frames.txt` carries the header line at all nine renderable viewports. At `120 × 48`
the header reads `overlays: inspector i at width 140` beside the hint; at the floor `34 × 22` the
whole notice reduces to `HELD  ?  x8  r W100  L H38  i W140`, which is the last rung of the ladder
and the hardest case for this assessment.

**Two frames are carved out for the assessor**, because a frame cannot be shown from `frames.txt`
without its `===` banner naming the viewport and priming the reader:

| File | Size | Header as drawn | Role |
|---|---|---|---|
| `discoverability-frame.txt` | 48 × 160 | `HELD  ? keys  x8  overview  sel M01  filter none` | The contracted case, at the reference viewport |
| `discoverability-frame-floor.txt` | 22 × 34 | `HELD  ?  x8  r W100  L H38  i W140` | The hard case, for a second assessor only |

Both are unmodified extracts — `frames.txt` lines 14–61 and 389–410 — and in each the character `?`
appears on the header row and nowhere else, so neither frame answers its own question.

**The assessor cannot be the repository owner, and this is not the agent's to resolve.** The contract
asks for a person who has not read rule 7. The owner has read it and ratified six amendments to
`SPEC-MOK-003` on 2026-08-20, two of which — decisions 15 and 16 — fix the hint's content and the
abbreviation ladder's order of loss, so the owner knows the key from the specification rather than
from the screen. `VER-MOK-013`'s own residual uncertainty says the assessment "is available once per
assessor" and that a second run on the same person confirms nothing; here the first run is already
spent. **Finding an admissible assessor is outside what an implementation agent can arrange**, so the
options are the owner's:

- find a person who has not read rule 7 and take the assessment as contracted — **taken, 2026-08-20**;
- amend `VER-MOK-013` to state an assessment the available assessors can take, which is an
  assurance-owner act on an approved contract — **declined**;
- leave it outstanding by decision with the reason recorded, on the pattern of assessment 7 of
  `evidence/WO-MOK-012/manual-assessment.md` — **declined**.

The third is the only one that requires nothing further, and it is not the one that was chosen. **The
option that verifies the requirement was preferred over the option that closes the row**, which is why
this row is still outstanding rather than closed by decision as `VER-MOK-005`'s assessment 7 is.

**The automated case is not this assessment.** `VER-MOK-013` says so: that the character is in the
buffer of the first frame is "a necessary condition and not the property". That condition is measured
and passing —
`tests/render.rs::the_key_binding_hint_is_on_screen_in_the_first_frame_at_every_viewport` asserts it
at every viewport with no input delivered and at tick zero — and it does not close this row.

> **Later fact, added 2026-08-20 at the verification decision, beside the paragraph above and not in place
> of it.** On the same day, in a separate instruction — *"i approve VREC-MOK-013, making REQ-MOK-048
> verified."* — the assurance owner transitioned `VREC-MOK-013` to `verified` and **accepted this
> requirement's four automated cases in place of this assessment**, taking a fourth course that is not among
> the three options above. **Nothing in this section is retired by that.** This row is still
> **OUTSTANDING**, the author is still **none**, `VER-MOK-013` is **not amended** and its *"a necessary
> condition and not the property"* still governs, and `VER-MOK-013` is **not satisfied** on this count.
> What changed is what taking the assessment would now do: it would **confirm or contradict** a requirement
> already recorded as verified rather than verify one for the first time, and a contradiction would be a
> finding against that decision rather than against any measurement here. The material is unchanged and the
> route — find an admissible assessor — still stands, still unspent, still available once per assessor.
> `assurance-decision.md` records what was accepted and what it does not retire.

## Summary

| # | Assessment | Requirement | Status | Author | Blocking |
|---|---|---|---|---|---|
| 1 | Reference-viewport gauge legibility | `REQ-MOK-047` | **Satisfied**, 2026-08-20 | the repository owner, product owner | — |
| 2 | Overlay discoverability | `REQ-MOK-048` | **Outstanding** | none | a person who has not read `SPEC-MOK-003` rule 7; the route is decided and the material is prepared |

**Consequence for the chain.** `VER-MOK-013` classes both as primary evidence for their requirements, so
`VREC-MOK-013` can now record `REQ-MOK-047` as verified on assessment 1 and **must disclose
`REQ-MOK-048` as unverified** while assessment 2 stands outstanding. `REQ-MOK-049` was never gated on a
manual assessment — `VER-MOK-013` says so and gives its reason — so it turns only on the automated
cases. Any release record covering this chain inherits the one disclosure. None of this blocks the work
order's implementation, which is what this pack evidences; it blocks the verification of one of three
requirements, which is a separate commit-bound record and a separate accountable act.

> **Later fact, added 2026-08-20.** That separate accountable act was taken, and it did **not** go the way
> the paragraph above anticipated. `VREC-MOK-013` is `verified` and records `REQ-MOK-048` as **verified on
> its four automated cases, accepted by the assurance owner in place of this assessment** — so the record
> does not disclose the requirement as unverified, and the sentence above describes what the pack expected
> rather than what happened. **The table's row 2 is unchanged and correct**: the assessment is outstanding,
> its author is none, and the person it needs is still the blocker. The disclosure a release record inherits
> is therefore not "one requirement unverified" but "one requirement verified on the necessary condition
> rather than the property, with the contract's assessment still open" — which is the form
> `assurance-decision.md` states and the form `VREC-MOK-013`'s *Scope of the transition taken* carries.
