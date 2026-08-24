+++
id = "INT-MOK-001"
type = "intent"
title = "Establish the minimum simulation foundation"
status = "approved"
owners = ["product owner"]
created = "2026-08-11"
updated = "2026-08-24"

[relations]
+++

# Intent: Establish the minimum simulation foundation

## Amendment record

This section is created by the row below. This intent was approved on 2026-08-11 and had never been amended, so no amendment record existed to append to; no other intent in this initiative carries one either. The table takes the same three columns the specifications and architectures use, so that a reader who knows one form knows this one.

| Date | Change | Approval |
|---|---|---|
| 2026-08-24 | The determinism determinand, under `CAP-MOK-011`. Two sentences move, and they are the same sentence stated twice. The **success measure** *"Repeated runs with identical configuration and seed that produce identical results"* gains the retained transcript for a decision source outside the engine, and the matching **desired outcome** *"Identical configuration and entropy seed produce reproducible runs"* gains the same clause. Both keep their target and their observation window; a paragraph beneath the measures table records what moved, and why the four in-process sources' determinand does not. **`REQ-MOK-009` is not amended**, because the entropy stream is untouched and this source draws from it not at all. `ADR-MOK-007` decision 2 is the reason and `SPEC-MOK-007` rule 12 states the property. **Considered and not amended.** The row *"Network services or API credentials required to run the foundation — 0 — every local run"* stands, and was measured rather than assumed: at this candidate no local run requires either, a replay of the fifth source included, because a replay makes no provider call, opens no socket, spawns no connector and reads no credential whether or not one is present in the environment, which is `SPEC-MOK-007` rule 12.2; the live path that would need one does not exist in this tree and is `WO-MOK-026`'s. That is recorded in a paragraph beneath the table rather than left for a reader to work out, and whether the target of 0 still reads as true once a live run exists is the product owner's question then. The desired outcome *"A later decision source can use OpenAI GPT nano without changing authoritative simulation rules"* stands as written: it states a capability rather than a binding, the port admits any provider, and the model this initiative actually names — `gpt-5.6-luna`, declared by the connector rather than by either host — is a swappable choice by `ADR-MOK-007` decision 3's design rather than a departure from this outcome. The **Non-goals** list stands, `"OpenAI API integration, prompts, model memory, or model-provider operations"` included: a later intent taking up what an earlier one placed outside its scope is the governance path this repository uses, and `INT-MOK-011` is that intent — its own outcomes and measures carry the new source, including the replayability figure this row does not duplicate. The *Risks and assumptions* entry *"an engine-first foundation should be verified before adding an external LLM decision source"* stands and is satisfied rather than superseded. The empty `[relations]` block is unchanged, because no relation is required of an intent here and `ADR-MOK-007` names none. | **Two acts.** The measure was approved 2026-08-23 by the repository owner, who holds the product owner's role, in the act *"i approve the artifact pack"*, and by way of `ADR-MOK-007`, whose *Required amendments* section states it in full and whose own rationale records that it *"is put to the product owner rather than written by the technical owner because a success measure is the product owner's"*. The **desired outcome** is a separate act: the same owner decided it on 2026-08-24, on being shown that this intent states the determinism property twice in the same words while `ADR-MOK-007` names only the measure, and that under `--policy llm` two runs at one seed and one `--transcript-path` diverge if the file's bytes changed — the exact defect the ADR amends the measure to fix. **Stop condition 6 of `WO-MOK-025` was invoked and the artifact was not amended on the implementation agent's judgement.** Two courses were declined: amending the measure alone and recording the outcome as true because this intent's *Non-goals* scope it to the four in-process sources, which is defensible and would have left the document stating the property with the transcript in one place and without it in another; and amending the measure alone with an **OUTSTANDING** row for the outcome, on the form `SPEC-MOK-004`'s 2026-08-21 row used, which would have sent an intent that contradicts itself on determinism to `VER-MOK-018`. The implementation agent wrote both texts under `WO-MOK-025` and decided neither. Sibling rows for the pack approval stand in `SPEC-MOK-004`, `SPEC-MOK-007`, `SPEC-MOK-003`, `ARCH-MOK-002` and `ARCH-MOK-001`; `SPEC-MOK-003` and `ARCH-MOK-002` carry their own second act from the same date, and no part of either reaches this artifact. |

## Problem

Mokiterions has no executable simulation foundation. The world, survival rules, resources, decisions, and observations cannot yet be exercised or verified independently of an external model provider.

## Desired outcomes

- A Rust program runs a bounded Mokiterions simulation from initialization through termination.
- Twelve Mokiterions inhabit the specified world and interact with basic survival mechanics.
- The simulation engine remains the sole authority over world state.
- Identical configuration and entropy seed produce reproducible runs, and for a decision source outside the engine the retained transcript is a determinand alongside the seed.
- Operators can reconstruct material activity from simple text output.
- A later decision source can use OpenAI GPT nano without changing authoritative simulation rules.

## Actors and stakeholders

- The product owner defines the intended simulation outcomes and accepts product behavior.
- Developers implement and maintain the Rust simulation.
- Operators run local experiments and inspect their results.
- Assurance reviewers assess requirement coverage and retained evidence.

## Success measures

| Measure | Baseline | Target | Observation window |
|---|---:|---:|---|
| Valid simulations that start and terminate without panic | 0% | 100% | Every automated and manual run |
| Repeated runs with identical configuration, seed and — for a decision source outside the engine — retained transcript, that produce identical results | Not available | 100% | Automated verification |
| Material state mutations performed through validated engine operations | Not available | 100% | Automated verification and review |
| Approved requirements covered by automated tests | 0% | 100% | Before the first foundation release |
| Network services or API credentials required to run the foundation | Not applicable | 0 | Every local run |

**Amended 2026-08-24 for `REQ-MOK-067`.** The determinism measure read "identical configuration and seed", which is the whole determinand for every source that decides inside the engine and is not the whole determinand for one that does not. A decision the engine did not compute cannot be recovered from the seed, so a run of the model-backed source is reproduced by replaying the decisions it made, and the retained transcript is what makes that possible. The transcript's *path* is configuration and was already covered; its *bytes* are not, and two runs at one seed and one path diverge if those bytes change. **Repeated runs at an identical seed and transcript give 100 percent identical results**, which is the target this row already carried, now stated over the inputs it is actually true of. `REQ-MOK-009` is **not** amended and the four existing sources' measure does not change: the entropy stream is untouched and this source draws from it not at all, so where there is no transcript the pair is the seed and the row means exactly what it meant before. `SPEC-MOK-007` rule 12 states the property, `ADR-MOK-007` decision 2 is the reason, and `INT-MOK-011` carries the new source's own measure — that every recorded run is replayable byte-identically, offline — which this row does not duplicate.

The **credential and network-service row is unchanged and was measured rather than assumed**: at this candidate no local run requires either, including a run of the fifth source, because a replay makes no provider call, opens no socket, spawns no connector and reads no credential whether or not one is present in the environment, which is `SPEC-MOK-007` rule 12.2. The live path that would need a credential does not exist in this tree and is `WO-MOK-026`'s, and the credential lives in a connector outside both packages and outside this repository. Whether this row's target of 0 still reads as true once a live run exists is the product owner's question then, not a claim this amendment makes.

## Non-goals

- OpenAI API integration, prompts, model memory, or model-provider operations.
- Combat, threats, surrender, fear-driven behavior, or social relationships.
- Sophisticated migration, cooperation, or conflict strategies.
- A graphical or web user interface.
- Persistence, reproduction, genetics, economies, crafting, or scripted narratives.

## Principles and immutable constraints

- Keep the implementation small, direct, and easy to inspect.
- Decision sources propose actions; only the simulation engine validates and applies them.
- Invalid actions never partially mutate authoritative state.
- Controlled entropy remains reproducible from an explicit seed.
- Text output is sufficient for this stage.
- Do not introduce abstractions solely for hypothetical future needs.

## Risks and assumptions

- Fact: the product concept calls for a 128 by 128 world, two territories, twelve Mokiterions, three food classes, survival state, conditional food regeneration, and observable behavior.
- Assumption: an engine-first foundation should be verified before adding an external LLM decision source.
- Risk: a temporary baseline decision source could be mistaken for final autonomous behavior; output and documentation must identify it as a development baseline.
- Open decision: numeric survival values and timing defaults must be fixed in specifications before the related requirements are approved.
- Approval decision: the repository owner validated this governing chain on 2026-08-11.
