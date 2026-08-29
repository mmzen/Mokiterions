+++
id = "WO-MOK-027"
type = "work_order"
title = "Stage 5c: the authorized measurement — the model-backed source published beside the reference and social sources at the same seeds and horizon"
status = "approved"
owners = ["engineering owner"]
created = "2026-08-23"
updated = "2026-08-29"

[assurance]
commit_bound_verification = "required"
rationale = "This work order produces the only figures this initiative exists to produce, and they are figures no one can regenerate cheaply: each seed's run costs an estimated $1.04 and takes an estimated 1.2 to 2.4 hours, so a defect found after publication is not repaired by re-running. Two claims here are checkable only over retained evidence. That every published figure comes from a run whose fallback count is zero is a claim about each run's record rather than about code, and `REQ-MOK-074`'s fitness rule is the only thing standing between an incomplete run and a published number. That the reference and social figures beside it were re-run at the same seeds and the same horizon, rather than quoted from their own longer runs, is a claim about which commands were issued — the exact error `VER-MOK-018`'s assessment M3 exists to catch. The stage also carries the initiative's central negative claim, that no outcome threshold governs the result, and a verification record is the only place a reader can confirm that the absence was decided rather than overlooked. Verification requires owner-authorized live runs, so the record binds evidence that a later commit cannot reproduce."
decided_by = "engineering owner"

[execution_scope]
paths = [
  "docs/PHASE_5_MEASUREMENT.md",
  "docs/engineering/simulation/evidence/WO-MOK-027/",
  "docs/engineering/simulation/intent/INT-MOK-001.md",
  "docs/engineering/simulation/work-orders/WO-MOK-027.md",
  "scripts/",
]

[relations]
implements = ["REQ-MOK-075", "REQ-MOK-076"]
specifications = ["SPEC-MOK-001", "SPEC-MOK-007"]
verification = ["VER-MOK-018"]
architecture = ["ARCH-MOK-001", "ADR-MOK-007"]
+++

# Work Order: Stage 5c — the authorized measurement

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope below.
Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the completed
change and the retained evidence. Verification requires a separate commit-bound record.

**This work order was approved on 2026-08-23**, by the repository owner acting as accountable product, technical,
engineering and assurance owner, in the words *"i approve the 3 work orders"* — one act covering `WO-MOK-025`,
`WO-MOK-026` and `WO-MOK-027`. `ADR-MOK-007`'s *Decision record* holds it as act 12. **That approval does not make this
work order startable and it does not authorize a single run.** Two gates stand between the approval and any work here:
the verification of `WO-MOK-026` with its cache-ratio result passed, stated in the next paragraph, and the separate owner
instruction each live run needs. From this act onward `preflight` reports this work order as start-eligible; it cannot see
either gate, and the gates are the authority.

**`WO-MOK-026` must be verified before this work order may begin**, and its cache-ratio result must have passed. If the
ratio missed, the per-run cost estimate this work order's authorization rests on is wrong, and the owner is authorizing
spend against a figure that no longer holds.

**Approval of this work order does not authorize the runs.** It authorizes building the comparison and the authorization
record, and it states what an authorization would have to say. The runs themselves need a separate owner instruction
naming the horizon, the seed set and the spend ceiling — the second act `REQ-MOK-076` requires, and the act the owner
described in their own words as *an explicit permission from the repository owner is needed to launch a real run*.

**The horizon and the seed set are deliberately absent from this work order, and that absence is a recorded decision
rather than a gap.** Asked on 2026-08-23 what horizon to fix for this stage, the repository owner answered **deferred**,
and `ADR-MOK-007`'s *Decision record* holds it as act 8. The reason it is a decision and not an omission: the only
figures that could justify a horizon — the real per-exchange cost and the real round-trip latency — do not exist yet, and
`WO-MOK-026` is what produces them. Fixing a horizon now would fix it against an estimate, and the estimate is the thing
this initiative is least confident about. So this work order specifies **what an authorization must say** and leaves the
values to the act that says them.

**Two consequences follow, and both are stated here so that neither is discovered late.** The owner's declared ceiling of
**$2**, decided on 2026-08-23, governs `WO-MOK-026`'s single instrument run and **does not reach this stage**: five
1,000-tick seeds are an **estimated** $5.20, so this stage needs its own ceiling in its own authorization. And a deferred
horizon means this work order cannot be estimated, scheduled or approved as a fixed quantity of spend — its approval
authorizes the *machinery* of the measurement, never its cost.

**This work order publishes no threshold and asserts no outcome.** There is no survivor floor, no death ceiling and no
expected result, because `ADR-MOK-007` decision 7 records the owner's decision that defining one would defeat the
purpose. What is verified is that the measurement was taken and reported honestly. What the population does is the
finding, and a finding is not a pass condition.

## Objective

Report what happens when the decisions come from a model: for the model-backed source over a declared seed set at a
declared horizon, and for the `reference` and `social` sources re-run at the same seeds and the same horizon, the
survivors, the deaths, and the deaths attributable to combat — with every published figure traceable to a run whose
fallback count is zero, and with the authorization for each run retained beside it.

## In scope

1. **The comparison report**, per `REQ-MOK-075`: three sources, one seed set, one horizon, three figures each, plus the
   per-seed detail behind every aggregate. `reference` and `social` are chosen because they are the two sources with
   `INT-MOK-001` figures already published; `baseline` and `individual` may be included but are not required.
2. **The re-runs of `reference` and `social`** at exactly the declared seeds and horizon, executed for this comparison
   rather than quoted from any existing run or artifact. These cost nothing and are the whole substance of assessment
   `M3`.
3. **The model-backed runs**, one per declared seed, each with its own declared ceiling, each producing a transcript, a
   record stream and a run record.
4. **The fitness rule applied**, per `REQ-MOK-074` and case `L25`: a run with a non-zero fallback count sources no
   published figure. Its evidence is still retained, and its exclusion is stated with its fallback count.
5. **The authorization record**, per `REQ-MOK-076`: the authorizing owner, the date, the horizon, the seed set and the
   spend ceiling authorized, retained with the evidence of every run it covers, and a static check that no live run's
   evidence directory lacks one.
6. **The published-figure provenance**: for each figure in the report, the run, the transcript and the run record it came
   from, so that a reader can re-derive it without re-running anything.
7. **The `INT-MOK-001` amendment** `ADR-MOK-007` requires, adding the fifth source's figures beside the existing ones and
   amending the determinism sentence to the `(seed, transcript)` pair — written as the approved amendment text.
8. **A statement of what the comparison does not establish**, in the report itself and not only in the verification
   record: one horizon, one seed set, one model, one reasoning level, no repetition at a seed, and no claim that a
   difference between sources is attributable to the model rather than to the prompt.

## Out of scope

- **Any change to the engine's behaviour, the port, the request layout, the transcript format or the accounting.** If
  any of them must change, `WO-MOK-025` or `WO-MOK-026` was wrong; that is an escalation, not this stage's work.
- **Any interpretation of the result as a finding about intelligence, strategy or capability.** The report states what
  happened. Whether the model-backed population survived longer than `social` is a number, not a conclusion about the
  model.
- **Any repetition at a seed to average out provider variation.** `gpt-5.6-luna` documents neither a temperature nor a
  seed parameter, so run-to-run variation at one seed is real and unquantified. Quantifying it means five more runs per
  seed and is a separate authorization the owner has not been asked for. Its absence is stated as a limit.
- **Any run beyond the authorized seed set and horizon**, for any reason including a run that failed and looks cheap to
  repeat. A repeat is a new run and needs its cost counted against the authorized ceiling.
- **Any tuning of the shared rules block to improve an outcome.** That would make every figure a measurement of the
  tuning, and assessment `M1` exists to catch it.
- **A release.** `SPEC-MOK-005` governs release authorization and nothing here proposes one.

## Authorized decision envelope

The implementation agent may decide locally:

- The report's format and where it lives, subject to every figure carrying its provenance. No approved specification
  fixes the location of a document under `docs/`: `SPEC-MOK-004` rule 1 governs the package directories and calls the
  root's remainder "the repository-level configuration and documentation" without enumerating it, so this is genuinely
  the agent's and is stated as such rather than left to be looked for.
- The order in which the runs are executed, and whether they run sequentially or on separate machines, subject to each
  run's ceiling being independent and the total staying within the authorized ceiling.
- How "deaths attributable to combat" is extracted from the record stream, subject to it being the same extraction for
  all three sources — the comparison is worthless if the sources are counted differently.
- Whether `baseline` and `individual` are included beside the required three.

The agent may **not** decide: the seed set; the horizon; the ceiling; whether a run with a non-zero fallback count may
be published; whether a threshold is stated; or what the figures mean. The first three are the owner's in the
authorization record; the fourth is `REQ-MOK-074`'s; the fifth is `ADR-MOK-007` decision 7's; the last is nobody's until
the figures exist.

## Constraints

- **No run without an authorization record naming the horizon, the seed set and the ceiling.** The record is written
  before the run, not after it, because a record written afterwards describes what happened rather than what was
  permitted.
- **A run with a non-zero fallback count sources no published figure.** Not a caveat on the figure — the figure is not
  published. `REQ-MOK-074` is why, and `L25` is the check.
- **`reference` and `social` are re-run, not quoted.** The `INT-MOK-001` figures were measured at their own horizon; a
  comparison that mixes horizons is not a comparison. This is the single most likely error in this stage.
- **The same horizon and the same seeds for all three sources**, and the same density.
- **No figure is inferred.** Every number in the report is read from a run record or recomputed from a transcript, and
  the report says which.
- **The estimated cost is stated against the ceiling before the first run.** At an **estimated** $1.04 per 1,000-tick
  run, five seeds are an **estimated** $5.20; the owner's ceiling governs, and the estimate is what makes the ceiling
  meaningful rather than arbitrary. The estimate is re-derived from `WO-MOK-026`'s **measured** per-exchange cost before
  the authorization is sought, because that stage exists in part to replace this figure.
- **The ceiling for this stage is not the $2 declared for `WO-MOK-026`.** That figure is below the five-seed estimate and
  was declared for a 200-exchange instrument run. Proceeding under it would either stop every run early or exceed it, and
  neither is a measurement.
- **The horizon and the seed set come from the authorization, not from this work order.** Deferring them is act 8 of
  `ADR-MOK-007`'s *Decision record*; an implementation agent that supplies a default for either has taken the owner's
  decision.
- **Every measurement run is executed from the engine's binary target.** `REQ-MOK-077` makes a live run reachable from
  that host and no other, so a figure sourced from an observer run is not merely wrong provenance — it could not have
  been produced.
- **Every run's evidence is committed complete**: transcript, record stream, run record, authorization. A **measured**
  12.7 to 13.9 MB per 1,000-tick transcript, and therefore an **extrapolated** 64 to 70 MB for five seeds, is what the
  repository takes on *(amended 2026-08-29 under `WO-MOK-031`)*. The per-transcript figures are quoted from
  `SPEC-MOK-007` rules 11.7.2 and 11.7.3 — 12,722,347 bytes and 13,901,867 bytes, the two extrapolations that rule
  makes from this repository's two live transcripts at their two different exchange rates — and the five-seed total is
  those figures multiplied out rather than a second measurement.
- **The evidence path is named before the first capture**, and never renamed. Here a rename means paying for the runs
  again.
- **Governance artifacts are written CRLF and retained evidence LF.** This stage commits the initiative's largest
  volume of evidence, all of it under the tree `.gitattributes` exempts from end-of-line conversion, where the bytes
  written are the bytes hashed. A digest taken over CRLF is one no reviewer can reproduce, and the remedy is the same as
  a rename's: paying for the runs again.

## Expected change surface

- **The comparison report**, a new document, with its per-seed detail.
- **The evidence path**: one directory per run, each holding a transcript, a record stream, a run record and the
  authorization record covering it.
- **A static check** that no live-run evidence directory lacks an authorization record, for `L28`. It lives as a
  repository script beside `scripts/validate_engineering_artifacts.py` rather than as a package test, so that neither
  package's test count moves and `SPEC-MOK-004` rule 11 stays where it is.
- **`INT-MOK-001`**, amended per `ADR-MOK-007`.
- **No change to either Rust package's source, and none to `SPEC-MOK-004`.** This stage adds no test, no package
  directory and no public item, so rule 11's figures do not move and rule 1's layout does not either. If either turns out
  to need amending, something outside this stage's scope changed and it is an escalation.

## Required verification

`VER-MOK-018`'s remaining cases, all three of which depend on the authorized runs:

**L24** — the comparison is published, **owner-gated**: for the declared seed set at the declared horizon, the survivors,
deaths and combat-attributable deaths appear for the model-backed source and for `reference` and `social`.

**L25** — only fit runs are published, **owner-gated**: every run whose figures are published has a fallback count of
zero, and every excluded run's exclusion is stated with its count.

**L28** — the authorization is retained: every live run's evidence includes an authorization record naming the owner, the
date, the horizon, the seed set and the ceiling, checked statically, plus the owner's attestation that each is genuine.

**Manual assessment M3** — the published comparison is honest: the assurance owner confirms that `reference` and `social`
were re-run rather than quoted, that no figure carries a threshold, and that no seed's absence is unexplained.

**Case L26 re-run** — no outcome threshold exists. This is the case most likely to be broken by this stage, because a
report is where a threshold naturally wants to be written.

**Cases L9 and L10 re-run** — the four existing sources stay byte-identical, since this stage re-runs two of them and any
drift would silently change the comparison's baseline.

**Case L31 re-run** — the observer replays. Each published run's transcript is replayed in the observer as well as in the
engine's binary, because a transcript that cannot be watched is worse evidence than one that can, and this is the only
stage that produces transcripts a reader will actually want to watch.

**Security check C1 re-run** over every newly committed evidence file, because this stage commits the largest volume of
provider-derived bytes in the initiative.

## Evidence to record

Under the evidence path this work order names, fixed before the first capture:

1. **The owner authorization record**, naming the authorizing owner, the date, the horizon, the seed set and the spend
   ceiling — retained in every run directory it covers, not only once.
2. **Per model-backed run**: the transcript, the record stream, the run record, the exit status, the accumulated cost,
   the fallback count and the cache ratio.
3. **Per `reference` and `social` re-run**: standard output, record stream, exit status and digest, with the exact command
   issued recorded so that `M3` can be assessed from evidence rather than from testimony.
4. **The comparison table itself**, with per-seed rows and the aggregate, and each figure's provenance.
5. **The recomputation of each published figure from its transcript or record stream**, independent of the run record.
6. **The fitness determination for every run**: its fallback count, and published or excluded, with excluded runs' counts
   stated.
7. **The total actual cost** against the authorized ceiling and against the estimate.
8. **The `L28` static check output**, run over every live-run evidence directory in the repository, not only this
   stage's.
9. **The assurance owner's `M3` record**, and the `L24` assessment record, each naming the owner and the date.
10. **The statement of limits**, retained with the evidence as well as printed in the report.

## Stop and escalate conditions

1. **`WO-MOK-026`'s cache ratio failed**, or the measured per-run cost is materially above the estimate. The
   authorization's basis is wrong and the owner must re-authorize against real figures.
2. **An authorization arrives without a horizon, without a seed set, or with a ceiling below the re-derived estimate for
   the seed set it names.** Do not fill in the missing value and do not run partially to stay inside a low ceiling. The
   horizon and seed set were deferred to this act precisely so that this act supplies them; an authorization that does not
   supply them is incomplete rather than permissive.
3. **Any run's fallback count is non-zero.** Do not publish it, do not average around it, do not repeat the run without
   authorization. Report it and escalate: a systematic fallback means the instrument is not ready, and a one-off means
   the seed set is incomplete, and which it is changes what the owner should do.
4. **A run stops on its ceiling before the declared horizon.** Its figures are not at the declared horizon and so are not
   comparable. Escalate for a ceiling decision rather than publishing a short run beside full-horizon ones.
5. **The authorized ceiling would be exceeded to complete the seed set.** Stop at the ceiling. Report which seeds were
   measured and which were not.
6. **`reference` or `social` cannot be re-run at the declared horizon**, or their re-run figures differ from
   `INT-MOK-001`'s at their own horizon in a way that is not explained by the horizon. The second case means something
   moved that `L9` should have caught.
7. **Any request arrives to add a threshold, an expectation, a target or a success criterion to the report.** `L26` fails
   on it and `ADR-MOK-007` decision 7 is the recorded reason. If the owner wants one, that is an amendment to a decision,
   not an edit to a report.
8. **The evidence volume exceeds what the repository should carry**, or an evidence path must be renamed after a capture.
   A rename here costs the price of every run again.
9. **A credential or any provider-side identifier appears in any committed evidence file.** Stop before committing.
10. **Either Rust package needs a source change.** Escalate to the prior work order rather than changing code under a
    measurement work order.
11. **The comparison shows something that invites interpretation.** Report the numbers and escalate the interpretation.
    Concluding anything about the model from one horizon, one seed set, one reasoning level and no repetition is the
    error this whole stage's limits section exists to prevent.

## Completion report format

1. **The authorization** as given: owner, date, horizon, seed set, ceiling.
2. **The comparison table**, per seed and aggregate, three sources, three figures — presented before any commentary.
3. **The fitness determination** for every run, published or excluded, with fallback counts.
4. **Each figure's provenance** and its independent recomputation.
5. **The cost**: per run, total, against the ceiling, and against the estimate as a factor.
6. **The confirmation that `reference` and `social` were re-run**, with the commands issued and their evidence paths.
7. **Each verification case** in the required list, with its result and its evidence path.
8. **The statement of limits**, verbatim as it appears in the report.
9. **A statement that no threshold was applied and none exists**, with `L26`'s result.
10. **Every local decision** taken under the envelope, and **every escalation** raised, with its resolution.

## Amendment record

**2026-08-29, the transcript-size figure replaced, under `WO-MOK-031`, by the engineering owner.**

*Constraints* stated "an **estimated** 4.7 MB per transcript and 23 MB for five seeds is what the repository takes on,
and that figure is confirmed against the tree rather than assumed". Both halves were false by the time they were read.

The figure is low by a factor of about **2.8**. `WO-MOK-026`'s accepted 50-tick live run produced a transcript of
**700,192 bytes**, which `SPEC-MOK-007` rule 11.7.2 extrapolates to **12,722,347 bytes** at 1,000 ticks, and rule 11.7.3
confirms a second time at **13,901,867 bytes** at a different exchange rate. Five 1,000-tick seeds are therefore roughly
64 to 70 MB rather than 23.

The claim of confirmation was the worse half. `SPEC-MOK-007` rule 11.7.1 **withdrew** the 4.7 MB estimate on 2026-08-24,
naming it as superseded together with the band "100 to 260 KB for a 20-to-50-tick run" that 298 KiB at 20 ticks had
already exceeded. So this work order cited as "confirmed against the tree" a figure the tree records as retired, five
days before the sentence was read.

**The specification's current estimate was not falsified; only this work order's retired one was.** Rule 11.7.1 replaced
4.7 MB with **12 MB** at 1,000 ticks on the same day it withdrew it, and both measurements are above that figure —
`WO-MOK-026`'s completion report calls 12.7 MB against it **1.06×, confirmed**, and rule 11.7.3 states that its own
13.9 MB is above it as well. What was wrong here was quoting the superseded number, not the estimating.

**The cost figures in this work order are deliberately not amended.** They are wrong too, and by a **larger** factor than
the size claim rather than the same one. `WO-MOK-026`'s completion report measures the per-run cost at **4.1× the
estimate** — 20.55 cents billed against 5 cents prorated — and puts this work order's `$5.20` for five seeds at about
**$20.55**, ten times the $2 ceiling that run was authorized under.

But *Constraints* already requires the estimate to be "re-derived from `WO-MOK-026`'s **measured** per-exchange cost
before the authorization is sought", so that figure is bound to be replaced by the act that spends the money. Amending it
here would put a second number in front of that step without removing the obligation to derive it. Only the size claim
moves, because only the size claim asserted a confirmation it did not have.

Nothing else in this work order moves: not its horizon, not its seed set, not its `[execution_scope]`, not its status,
and no figure in its *Objective*, *In scope* or *Required verification*. The `rationale` in `[assurance]` still carries
the `$1.04` estimate, and is left word for word for the reason above.

The frontmatter's `updated` moves to **2026-08-29**. The row below left it at the approval date of 2026-08-23 and should
not have; because the field records only the latest edit, today's date covers both amendments and that omission needs no
separate repair.

**2026-08-28, `[execution_scope]` added, under `WO-HUP-002`, by the engineering owner.**

Approved on 2026-08-23 under the `se_harness` 0.4.0 work-order template, which carried no `[execution_scope]`
table, and therefore unstartable under the 0.8.0 root with `QGP-G3-SCOPE: WO-MOK-027 has no assessable
execution scope`. The table is derived item by item from this work order's own *Expected change surface*, which
is unchanged, and the mapping is retained in `../../harness/evidence/WO-HUP-002/`.

`docs/PHASE_5_MEASUREMENT.md` is an **owner decision of 2026-08-28, not a derivation**. The surface calls the
comparison report only "a new document" and names no path, which fired `WO-HUP-002`'s stop-and-escalate
condition; the path was decided rather than inferred.

`scripts/` is a derivation: the surface places the static check "beside
`scripts/validate_engineering_artifacts.py`" and so names the directory itself.

The scope admits **no path under `mokiterions-core/` or `mokiterions-tui/` and not `SPEC-MOK-004`**, because
this work order's surface states positively that it changes neither. A scope wider than the work order would
defeat the boundary it exists to be.

Nothing else in this artifact moves.
