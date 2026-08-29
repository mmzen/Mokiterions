# Defect: `SPEC-MOK-006` rule 8.9's `live` object cannot hold together with `SPEC-MOK-007` rule 12.6

Recorded 2026-08-29 under `WO-MOK-026` item 11, on the repository owner's decision of the same day:
**follow rule 12.6, leave rule 8.9's text untouched, and record the conflict as a defect with this
stage's evidence.** The two alternatives were put with it and declined — amending rule 8.9, which is
another specification's rule and outside this work order's execution scope, and dropping the record
stream's `live` object, which is the same amendment reached by another route.

Nothing here is repaired. `WO-MOK-026` stop-and-escalate condition 6 forbids an implementation agent
amending an approved artifact on its own judgement, and every item below is a change of substance in
an approved rule.

## The conflict

`SPEC-MOK-006` rule 8.9, as amended 2026-08-29 under `WO-MOK-030`:

> A `live` object is present when, and only when, the run obtained its decisions from a connector:
> `tokens` with `prompt`, `cached_prompt`, `output` and `reasoning`; `cache_ratio`; `cost` and
> `ceiling`, both integers in US cents per that rule 14.2; `fallbacks`; `model` and `reasoning_level`
> as the connector reported them; and `fit`, false when `fallbacks` exceeds zero. A replay carries no
> `live` object at all — rule 15.6, and the reason is that a replay spends nothing and a second
> account of one spending event would be one account too many.

`SPEC-MOK-007` rule 12.6, original content approved 2026-08-23 by way of `ADR-MOK-007`:

> A replay of a matched configuration produces standard output bytes, **structured record stream
> bytes** and an exit code identical to the recorded run's. Byte-identity is claimed for the matched
> configuration, which includes the tracing selection, and is not claimed for standard error.

A recorded live run's record stream carries the `live` object. A replay of that run's transcript
carries none. **The two streams therefore differ in bytes, and rule 12.6 says they do not.** Both are
approved rules about the same object and only one can hold.

`VER-MOK-018` case **L7** is the pass condition that makes the disagreement operative rather than
academic: *a recorded run and a replay of it compare equal with `cmp` on standard output and on the
structured record stream, and have the same exit code, with no credential in the environment and no
network reachable; checked at every declared seed, with tracing on and off.* `REQ-MOK-067` is the
requirement behind it. A stream conforming to rule 8.9 fails that case for every live recording, by
construction and not by a defect in the code.

**This is not the same shape as rule 8.9's own citation of rule 15.6.** That rule reasons that a
replay must not carry the object because a replay spends nothing — which is true of the *accounting*
and says nothing about the *bytes*. Rule 15.6 is satisfied by a replay reporting no run record on any
stream; rule 12.6 additionally constrains which stream that can be. Rule 8.9 read the first and not
the second.

## What was built, and why it does not conform to rule 8.9

`SPEC-MOK-007` rule 15's run record is rendered by the engine and written by the recording host to
**standard error**, which is the repository owner's decision of 2026-08-29 taken over two
alternatives: a sixth command-line option naming a path, and rule 8.9's structured record stream.
Standard error is the one destination that amends no approved rule — rule 12.6 exempts it in terms,
rule 15 leaves the destination to the host and names none, and rule 15.6 holds structurally there
because a replaying port reports no accounting to render.

**The record stream therefore carries no `live` object at this candidate, and rule 8.9's amended text
is unimplemented.** Measured at the candidate rather than asserted:

- `Simulation::write_run_record` in `mokiterions-core/src/simulation.rs` emits
  `{"record":"run","reason":…,"ticks":…,"survivors":…,"deaths":…,"crossings":…}` followed by
  `consumed`, `regenerated` and `regeneration_skipped`, and no `live` key. `grep -n '"live"'` over
  `mokiterions-core/src/` returns nothing.
- The two run records are **different records** and the spelling is what keeps them apart: rule 8's
  is `{"record":"run",…}` on the structured stream and rule 15's is `{"run_record":"llm",…}` on
  standard error. A reader of either stream can tell which obligation a line answers, and
  `mokiterions-core/tests/connector.rs` searches for `"run_record":` for exactly that reason.
- `mokiterions-core/tests/connector.rs`'s
  `a_live_run_stops_at_its_ceiling_and_leaves_the_record_stream_behind` asserts
  `!stream.contains("\"run_record\":")`, so the separation is pinned by a test rather than left to
  hold by accident.

The consequence for coverage is stated plainly: **`SPEC-MOK-006` rule 8.9's first sentence has no
implementation and no verification case at this candidate.** Rule 15.2's figures are all reported —
every one of them, on standard error — so no accounting figure is missing from the run's output; what
is missing is the object rule 8.9 places in a different stream.

## Three further defects ride with the same amendment

None is a consequence of the choice above. Each would hold of any implementation of rule 8.9 as
written, and each is why the conflict cannot be resolved by simply building the object.

### 1. Rule 10.2 obliges `schema` to increment and rule 8.9 does not say so

Rule 10.2: *`schema` is incremented when a record kind is added or removed, **a field is added**,
removed or renamed, a field's type changes, a value's domain in rule 3.2 gains or loses a member, or
the ordering rule 9.1 changes.*

The `live` object is at least ten added fields. No rule states the new `schema` value, rule 10.2's
own exemption does not reach it — the object alters bytes a conforming writer produces, which is the
one thing that exemption excludes — and rule 10.3 obliges a consumer that does not recognise a
`schema` value to refuse the stream. So a stream built to rule 8.9 either carries a `schema` value no
rule fixes, or carries the old one in breach of rule 10.2.

### 2. Rule 3.4 forbids the two fields rule 8.9 requires

Rule 3.3 closes the stream's character alphabet to `A`–`Z`, `a`–`z`, `0`–`9` and `_ . - + : ; >`, and
states that this is why *"the stream needs no escaping function, and it is the only reason"*. Rule
12.4 keeps the engine package's dependency table empty, so there is no serialization library to
supply one.

Rule 3.4: *a string field added to the stream at any future schema version must either be added to
that enumeration or arrive together with an escaping function and its own verification. **A field
whose value could be operator-supplied, environment-derived, or free text may not be added under rule
3.3.***

Rule 8.9 adds `model` and `reasoning_level` *"as the connector reported them"*. `SPEC-MOK-007` rule
10.7 declares the connector's output untrusted in whole, and rule 10.4c puts the provider binding in
the connector, so both values are free text arriving from outside this repository. Neither can be
added to rule 3.2's enumeration — it is exhaustive over values *the engine can produce* — so rule 3.4
admits them only with an escaping function that rule 3.3 says the stream does not have and rule 12.4
gives it no library to build.

**Standard error is not subject to any of this**, which is a property of the chosen destination rather
than an argument for it: rule 3.3's alphabet is a rule about this stream, and the run record on
standard error is not in it.

### 3. `cache_ratio`'s name meets rule 4.2, and rule 8.9 miscites the rule it relies on

Rule 4.2: *No mean, average, **ratio**, percentage, rate, delta or trend appears anywhere in the
stream. Where an average is wanted, the record carries a sum and, in the same record, the count that
divides it.*

Rule 8.9 satisfies rule 4.2's remedy in substance — `cache_ratio` is *"a pair of integers, `cached`
over `total`, and not a decimal"* — while the field's own name is one of the seven words rule 4.2
prohibits. This is the mildest of the three and is recorded for completeness rather than as a
blocker.

Rule 8.9 attributes the integer-pair form to *"rule 12.4's prohibition on a floating-point value in
this stream"*. **Rule 12.4 is the empty dependency table.** The prohibition is rules 4.1 and 4.2, as
that specification's own counterexample at *Examples and counterexamples* states — `"health":{"mean":99.0}`
*"violates rules 4.1 and 4.2"*. The reasoning is sound and the citation is wrong.

## A fourth observation, about the numbering

`SPEC-MOK-006` carries **two rules numbered 8.9**: the amendment above, and the standing *"No run
record is written for a run that terminated because a write failed. Rule 13.4 governs that case."*
Measured with `grep -n '^8\.9'`, which returns two lines. A citation of "rule 8.9" is therefore
ambiguous in that specification, and this document says "rule 8.9's first sentence" or quotes the text
wherever the distinction matters. It was found while writing this record and is not repaired here for
the same reason as everything else above.

## What repairing the conflict would take

Recorded so that the owner's options are on the record and none is chosen here.

1. **Amend rule 8.9** to place the accounting outside the record stream, or to state that the `live`
   object is exempt from rule 12.6's byte-identity. The first makes rule 8.9 agree with what is
   built; the second narrows `REQ-MOK-067`'s promise and case **L7**'s comparison, which is a
   reduction in what a green build establishes.
2. **Amend rule 12.6** to exclude a `live` object from the compared bytes. This needs a rule fixing
   how a comparison skips a region of a stream, which nothing in either specification has today, and
   `cmp` — case **L7**'s own tool — cannot express it.
3. **Have a replay carry the object with the recorded run's figures.** This restores byte-identity and
   breaks rule 15.6 instead: the replay would state an account of spending it did not do, which is
   the "second account of one spending event" rule 8.9 itself calls one account too many.

Each is a change of substance in an approved rule. Whichever is taken, defects 1 and 2 above must be
resolved before any object rule 8.9 describes can be written to this stream at all.
