# What the sweep found, recorded and not acted on

## 1. `asymmetric_collapse` fires at 20 seeds where 35 runs saw none

`ADR-MOK-008` predicted the class would be rare. Over 400 cells it accounts for 32 -- 7 under
`reference`, 9 under `individual`, 16 under `social`, and none under `baseline`, which cannot reach it
because it has no survivors anywhere. The class is real and the vocabulary needs it. Manual assessment
2 is whether the five classes are the right five.

## 2. The seed changes the class in half the groups

`WO-MOK-033`'s scenario 2 requires this be recorded either way. Over the 20 source-and-density groups
of 20 seeds each:

- The seed moves the run figures in **20 of 20** groups.
- The seed moves the *class* in **10 of 20** groups: `individual` at densities 0.25, 0.50 and 1.00;
  `reference` at 0.25, 0.50 and 0.75; `social` at 0.25, 0.50, 0.75 and 1.00.

This reverses the finding recorded on 2026-08-30 from five seeds, which was that the seed moved
figures but not classes. Five seeds was too few to see it. The consequence for anyone reading a
distribution: a single-seed run states nothing about its cell, and a 20-seed group states a
distribution rather than an outcome.

Density moves the class distribution for all three deciding sources -- 5 of 5 distinct class vectors
each -- and for `baseline` it cannot, because all 100 of its cells are `extinction` at every density.
`baseline`'s own figures do still move across all five densities: 5 of 5 distinct vectors of deaths,
crossings, consumed and ticks. Its distribution has nowhere to move, which is a floor rather than a
null result.

## 3. The threat mechanism is inert, measured over the full sweep

Threats resolve 7,701 times across the 400 cells and are effective 8 times -- 0.1039%. All 8 are under
`social`; `reference` and `individual` produce none. Per density: 2828/1, 1861/1, 1793/2, 577/2,
642/2.

This is the *before* figure for the repair chain the owner deferred on 2026-08-30, when the decision
was that Phase 4b discloses the finding and a later chain repairs it. Whether the disclosure as built
is sufficient to serve as that figure is manual assessment 8. The earlier reading over 35 runs was 1
effective of 1,448; this one is over 400 runs and agrees with it.

## 4. `retreat` is a targeted action with no resolved-event kind

Case T8 measures it: `SPEC-MOK-001`'s resolved-event vocabulary is `attack_resolved`,
`encounter_resolved`, `surrender_resolved`, `threat_resolved`, and declares no `retreat_resolved`. Over
eight real streams the engine emitted 14 event kinds, none of them naming retreat. No retained byte of
25 outputs contains a field or value named for retreat, which is what T8 asserts -- the instrument does
not synthesise one.

`docs/ROADMAP.md`'s Phase 6 wording treats retreat as an available action. It is available as an
intention and unobservable as an outcome, so no sweep can report on it. This belongs to the roadmap
reconciliation, which is outside every work order.
