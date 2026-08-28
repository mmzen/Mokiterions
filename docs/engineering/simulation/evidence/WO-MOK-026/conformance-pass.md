# Conformance pass: what stage 5b needs that its specifications do not yet say

Taken 2026-08-29, on the repository owner's instruction, after the fourth stop in three work orders. Every
gap below was found by asking what an implementer must know in order to write items 4 to 14 of
`WO-MOK-026`, and reading the specifications for the answer. None was found by writing code.

The pass exists because gap-by-gap patching was costing more than it saved: `WO-MOK-028` and `WO-MOK-029`
were each authored, approved, evidenced, transitioned and pushed to close one gap apiece, and a third and
fourth appeared immediately afterwards. This document is the whole list, so that one amendment chain can
close it.

## A. Scope walls

`WO-MOK-026`'s execution scope was derived from its own *Expected change surface*, faithfully. That surface
was written before execution scopes existed as a field, and it names three documents. Stage 5b needs to
amend **three more**, and the gate refuses each:

| File | Why 5b needs it | Status |
|---|---|---|
| `SPEC-MOK-007` | the wire format, the error vocabulary, the option names | breached twice; closed by `WO-MOK-028` and `WO-MOK-029` |
| `SPEC-MOK-002` | rule 5's interface census names `Config`'s public fields | **refused**, and already contradicted — see C |
| `SPEC-MOK-006` | rule 8.2 fixes the run record's shape | **refused**, and rule 15.2 requires seven new fields in it |

`SPEC-MOK-001` is also refused, and is listed only so a reader knows it was checked: nothing in items 4 to
14 requires amending it.

**The pattern is not that the scope is wrong.** It admits every path the surface text describes. The
surface text is incomplete about which *specifications* stage 5b must move, and it could not have been
otherwise — the work order was approved on 2026-08-23 believing `SPEC-MOK-007` already said everything the
stage needed.

## B. Specification gaps

### B1. The minor unit is never stated

Rule 14.2 requires cost to be "an integer in a **stated** minor unit". No rule states one. Every cost
figure in the specification is written in dollars — `$1.04`, `$0.004`, the owner's `$2` ceiling — so the
currency is implied everywhere and normative nowhere.

Consequence: `--spend-ceiling 2` means two of something unnamed, and the run record's cost integer has no
readable unit. **Decided 2026-08-29: the US cent**, to be stated in rule 14.2.

### B2. The unit prices have no declared input

Rule 14.3: "The declared unit prices are inputs of the run, not compiled-in constants. The provider's
prices are the provider's to change." The *State model* names them as what accumulated cost grows by.

No rule says **how** they enter the run. No option carries them, no file format holds them, no field
receives them. `WO-MOK-026` item 5 says only that "they are declared to the engine per rule 14", and rule
14 does not say how. A compiled-in default is the one thing rule 14.3 forbids, so the implementation
cannot fall back on one.

### B3. The model identifier and reasoning level are declared in two places at once

This is a contradiction rather than a silence, and it is the most consequential finding here.

- **Rule 10.3** — the *request object* the engine sends "carries the prompt text, the model identifier, the
  reasoning level and the response schema".
- **Rule 15.2** — the *run record*, which the engine writes, carries "the model identifier and the
  reasoning level".
- **`WO-MOK-026` item 5** — "The provider binding, **declared in the connector rather than in the
  engine**: the model identifier, the reasoning level and the endpoint."

If the connector declares them, the engine cannot put them in the request and cannot report them. If the
engine declares them, item 5 is false and the engine holds a provider binding it says it does not hold.
Three readings are possible and the artifacts do not choose:

1. The engine is told them by an input, and item 5 means only that the *endpoint* is the connector's.
2. The connector reports them back and the engine echoes what it was told.
3. The request's fields are advisory and the connector may override them, which makes rule 15.2's report
   a record of what was asked for rather than what was used.

The third is the only one consistent with item 5 as written, and it is also the one that makes the run
record least trustworthy — it would state a model that may not be the model that answered.

### B4. The retry bound is not a number

Rule 19.5: "A transport failure within a live run is retried a **bounded** number of times." Rule 11.2
records each attempt. No rule gives the bound, and a search for one finds nothing.

Any implementation must choose a number, and that number decides how much of the owner's $2 a stuck
provider can consume before the run gives up on an exchange.

### B5. `SPEC-MOK-006` rule 8.2 does not carry rule 15.2's figures

Rule 15.2 requires the run record to carry the four token totals, the cache ratio, the accumulated cost,
the declared ceiling, the fallback count, the model identifier and the reasoning level. `SPEC-MOK-006`
rule 8.2 fixes the run record's shape as a literal example, and it holds **none** of them.

Items 10 and 11 cannot be implemented without amending it, and it is refused by the scope.

## C. A defect already committed

`c13c327` added `spend_ceiling: Option<u64>` to `simulation::Config`. `SPEC-MOK-002` rule 5's interface
census enumerates that struct's public fields as exactly `seed`, `tick_limit`, `policy`, `density`,
`trace_actions`. **That census is now false**, and `WO-MOK-026`'s own line 75 says `SPEC-MOK-002` is not
amended.

The field is not removable without stalling items 9 to 11: rule 14.6 stops the run *before* an exchange
and rule 15.2 puts the ceiling in the run record, neither of which a host can do on the library's behalf
without the library knowing the number. Every alternative route is also an interface change — a sixth
`execute` parameter moves rule 4, and putting the ceiling on the port leaves the library unable to write
the run record rule 15.2 requires.

**Decided 2026-08-29: amend `SPEC-MOK-002` rule 5 under a successor work order**, together with the
amendments B1 to B5 need.

## D. What is specified well enough to build

Stated so the amendment chain is not read as a verdict on the whole specification:

- **Item 4, the spawn**, by rules 10.1 and 10.2 — a child process, one JSON object per line each way, in
  order, one response per request.
- **The environment pass-through**, by rule 10.5, unambiguously: the child inherits the parent's
  environment and no component here reads a credential.
- **Item 6, the observer's refusals**, by rules 18.4.2 and 20.3, including what the diagnostic must say.
- **Item 7, the two gates**, by rule 13.1, which is precise about the two conditions living in two
  components so that neither alone can authorise spending.
- **Item 8, the usage accounting**, by rules 10.4 and 14.1, and by rule 11.3.1 which already reserves the
  transcript's `response` and `usage` fields and names this work order as where they first carry a value.
- **Item 12's retry shape**, by rules 19.5 and 11.2 — every attempt its own record, exhaustion becoming a
  counted fallback rather than ending the run. Only the bound is missing.

Four of the five files the scope admits are the right ones. What the pass changes is that the remaining
work can now be planned against a known list rather than discovered one stop at a time.
