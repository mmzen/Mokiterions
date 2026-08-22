# The rule 8 order of loss — what was measured and what was decided

`WO-MOK-008` reports an obstacle rather than a design: the footer's four-tier ladder had no rule for what
to do when the narrowest tier still did not fit, and the renderer handed the pane a row wider than the
pane. This file records the arithmetic the technical owner was shown before fixing rule 8, the alternative
not taken for the one provision left OUTSTANDING, and the ladder the implementation uses. It decides
nothing; `SPEC-MOK-003`'s amendment row of 2026-08-22 is the decision.

## The arithmetic the floor imposes

`SPEC-MOK-001`'s start-up contract accepts an entropy seed and a configured tick limit anywhere in `u64`,
so each is up to 20 decimal digits. `SPEC-MOK-003` rule 5 fixes the floor at `34 × 22` and the footer at
one row spanning the full width. The seed and the tick limit alone, in the terse labelling, with the other
four fields already shed:

```
s18446744073709551615 t18446744073709551615
1 + 20            + 1 + 1 + 20            = 43 columns
```

43 of 34. **No order over the other four fields reaches the floor in that configuration**, which is why
rule 8 clause 6 states the seed as the only guarantee rather than the whole field set.

## The four candidates and their measured cost

| Candidate | Measured cost at the floor | Outcome |
|---|---|---|
| Raise the floor width so all six fields always fit | **50** columns for a `u64::MAX` seed at the default 100 ticks, **64** at 1e9 ticks, **84** at the arithmetic worst case | Rejected. `WO-MOK-008` names a floor change as an escalation trigger; it moves rule 5's floor, `REQ-MOK-024` and two `VER-MOK-005` viewport rows. |
| Give the footer a second row | The canvas interior at `34 × 22` falls to **15** rows, against rule 5's declared minimum spatial fidelity of `32 × 16` | Rejected. It would force the floor's height to 23 as a rule 5 decision, not a rule 8 one. |
| Present the unbounded numerics in a denser radix | **72** columns in hexadecimal, **63** in base 36, against a floor of **34** | Rejected. It defers shedding rather than removing it, and changes what clause 1 calls the value as supplied. Clause 8 now withholds the radix in terms. |
| Shed fields in a fixed order, never clipping one | 0 columns of new width; the loss becomes specified rather than emergent | **Chosen.** |

The owner was shown all four before deciding, and fixed the order of loss in the same act.

## The order, and the ground the owner gave for it

Clause 4, top to bottom: candidate commit, retained-event count with its marker, current tick, active
decision source, resource density, configured tick limit. The entropy seed is never shed.

Two of those positions were decided on ground the agent measured rather than proposed:

- **The retained-event count sheds before the five fields `REQ-MOK-027` names.** It is rule 8's own
  addition and no requirement claims it.
- **The resource density outranks the active decision source** — the 7-column field kept over the
  2-column one — because `REQ-MOK-027`'s rationale makes density the field without which an observation
  supports no comparison, per-seed survivor counts being non-monotonic in it. The cheaper field is not
  automatically the one to keep.

## The alternative not taken, for the OUTSTANDING first row

Clause 4's first row places the candidate commit ahead of every other field. **The owner decided the
order over the preamble's six fields and was not shown the commit's position**, because the question put
to them did not contain it. It is OUTSTANDING in `SPEC-MOK-003`'s row of the same date.

The agent placed it first on two grounds. Clause 2 already makes it the one contingent field: every other
field exists in every run, and this one exists only in a stamped build. And placing it first is what
discharges the obstacle `WO-MOK-008` reports — before this change, a stamp cost the row two rule 8 fields
at the floor.

**The alternative is the commit shed last**, on the ground that it identifies which binary drew the frame
and a frame from an unknown binary is a frame no one can act on. Its measured cost is exactly the
obstacle: at `34 × 22` with the default seed, a 40-character stamp displaces both the resource density and
the active decision source. **The reversal is one table row in clause 4 and one array element in
`SHED_ORDER`**; the case `the_candidate_commit_is_shed_before_every_field_rule_8_requires` is what would
change with it, and it asserts the position rather than assuming it.

## The ladder the implementation uses, and why shedding is the outer loop

Rule 8 clause 8 grants the labelling to the implementation and clause 4 sheds only where the width will
not hold a field. Those two together fix the search order: **for each number of fields shed, every
labelling is tried before one more field is lost.** A terser spelling of the whole row is always preferred
to a row missing a field, because labelling costs nothing a requirement claims and a field costs
information.

```
for shed in 0..=6:            # clause 4's order, a prefix at a time
    for spelling in [spaced, labelled, terse, tersest]:
        if the row fits the width: return it
return the seed alone         # clause 7's fall-back
```

Two consequences are worth naming because neither is obvious from the rule:

- **At the greatest shed level the row uses the *widest* labelling that fits.** At `34 × 22` with a
  20-digit seed and a 20-digit tick limit the row is `seed 18446744073709551615`, not
  `s18446744073709551615`: once the tick limit is gone, 25 columns fit inside 34 and the ladder prefers the
  labelled form. This is clause 8 working as written — the labelling is free, so the most legible one that
  fits is the one drawn.
- **Clause 7's fall-back is unreachable at any viewport the observer draws.** A `u64` seed is at most 20
  characters and `s` plus 20 is 21, against a floor of 34. It exists so that a width no viewport produces
  still yields the seed whole rather than a cut value, and the in-crate cases exercise it by calling the
  private row function at widths below the floor.
