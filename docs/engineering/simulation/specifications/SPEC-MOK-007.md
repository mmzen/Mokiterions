+++
id = "SPEC-MOK-007"
type = "specification"
title = "Model-backed decision source: the decision port, the cache-ordered request, and the transcript"
status = "approved"
owners = ["technical owner"]
created = "2026-08-23"
updated = "2026-08-29"

[relations]
specifies = [
  "REQ-MOK-063",
  "REQ-MOK-064",
  "REQ-MOK-065",
  "REQ-MOK-066",
  "REQ-MOK-067",
  "REQ-MOK-068",
  "REQ-MOK-069",
  "REQ-MOK-070",
  "REQ-MOK-071",
  "REQ-MOK-072",
  "REQ-MOK-073",
  "REQ-MOK-074",
  "REQ-MOK-075",
  "REQ-MOK-076",
  "REQ-MOK-077",
]
+++

# Specification: Model-backed decision source: the decision port, the cache-ordered request, and the transcript

## Scope

This specification fixes the exact behavior of the fifth decision source: the port the engine obtains a decision
through, the content and byte order of a decision request, the grammar a response must satisfy, what happens when a
response does not satisfy it, the transcript a live run retains, how a replay consumes that transcript, the usage and
cost accounting a live run performs, the conditions under which a provider call may happen at all, and the failure
behavior of each.

It specifies `REQ-MOK-063` through `REQ-MOK-077` and nothing else.

It does not restate the simulation's rules. `SPEC-MOK-001` remains the sole authority for what a Mokiterion may
propose, what an observation carries, how a proposal is validated and resolved, and what the text stream contains; this
specification refers to that authority rather than copying it, so that every rules question has exactly one answer. It
does not restate the structured record stream either, whose authority is `SPEC-MOK-006`. The transcript this
specification defines is a **third** stream, distinct from both.

**The transport binding is settled, and it costs no crate.** `ADR-MOK-007` decision 3 reaches the provider through a
**connector**: an executable the operator names by path, which a host spawns and exchanges lines with. The connector is
not a package, not a workspace member, and this specification does not require it to live in this repository. Rule 10
is therefore the whole of what the binding costs, and it costs nothing to any approved artifact: `REQ-MOK-050`,
`ARCH-MOK-001`'s conformance check, `SPEC-MOK-002` rule 13, `SPEC-MOK-003`'s declared set and `SPEC-MOK-004` rules 1
and 2 are all untouched. The port of rule 1, the request of rules 2 through 7, the response grammar of rule 8, the
transcript of rule 11 and the replay of rule 12 do not depend on the binding at all — which is why the port is specified
before the transport, and why a later change of binding would reopen rule 10 and nothing else.

**This source has two hosts and they are not equally capable.** Rule 20 states which host may do what, and the reason is
measured rather than preferred: an interactive host owes a frame every 33 milliseconds and a live exchange costs
hundreds of them.

Throughout, *the port* means the interface the engine obtains a proposal through; *a host* means the code outside the
engine that connects the port to something; *the connector* means the executable a host spawns to reach the provider;
*the provider* means the language model service; *an exchange* means one request sent to the provider and the response
or error that came back; *a live run* means a run that makes provider calls; *a replay* means a run that obtains its
decisions from a retained transcript; and *the transcript* means the stream rule 11 defines.

Amounts in currency and token counts given as *estimated* are estimates made on 2026-08-23 against the published
`gpt-5.6-luna` prices and a measured count of 10,954 decision opportunities in a 1,000-tick `social` run at seed 0 and
density 0.75. They are stated so that a later reader can see what the design was sized against; none of them is a
conformance condition. The conformance conditions are rule 14's ratio and rule 14's ceiling.

## Amendment record

**This section did not exist until 2026-08-24.** This specification was approved on 2026-08-23 and had no amendment
behind it until the row below, which is the only reason it shipped without the table every other specification in this
repository carries. The table is placed where those keep theirs — immediately after *Scope*, on `SPEC-MOK-003`'s
arrangement — so that the next amendment of this artifact finds it rather than has to decide where it goes.

| Date | Change | Approval |
|---|---|---|
| 2026-08-23 | Original content for `ADR-MOK-007`, covering `REQ-MOK-063` through `REQ-MOK-077`. | Approved 2026-08-23 by the repository owner acting as accountable technical owner, as part of the artifact pack `ADR-MOK-007`'s *Status* records, on the instruction *"i approve the artifact pack"*. |
| 2026-08-24 | **Rule 11 amended in five places so that the transcript this repository now commits is described truthfully, discharging six rulings the repository owner took on 2026-08-24.** Every one of the five was found by building the transcript and measuring it, and every one is a place where this specification as approved is contradicted by the bytes in `mokiterions-core/tests/transcript-seed0-ticks20-hunting.jsonl`. **Rule 11.2** gains the second record kind: the framing is two kinds and not one, a **prefix** record stating blocks A and B for one Mokiterion once and an **exchange** record naming that prefix instead of repeating it, with rule 3.4 cited as what makes the split sound and rule 11.7's figures as why it was chosen. Nothing is abbreviated by the split, so rule 11.7's first sentence is undisturbed and is stated to be. **Rule 11.3** is reworded to the two kinds and gains the fallback flag, which rule 15.4's count is reconciled against, and **three sub-rules arrive**: 11.3.1, that the response and the usage figures are present and empty until a provider is called, read through rule 11.5 as an absent response and four **absent** counts rather than four zeros, with `WO-MOK-026` named as where either first carries a value; 11.3.2, that the prefix reference carries an FNV-1a 64 digest of the prefix it names, that rule 12.3's mismatch check extends to it, and that it is **not cryptographic and not required to be**, with both alternatives recorded — a crate, which `SPEC-MOK-006` rule 12.4 forbids, and a hand-written cryptographic digest, which verifies a property nothing here needs; and 11.3.3, that the action's second field is **`parameter`, not `target`**, because rule 8.1 admits a direction and a field named `target` would be false of every `move`. **Rule 11.4** loses "no value outside a closed alphabet" from the constraints it adopts, and **11.4.1** states why it cannot hold and what replaces it: blocks A to D are multi-line English prose against a record-stream alphabet of `A-Z a-z 0-9 _ . - + : ; >`, `SPEC-MOK-006` rule 3.4 names this exact branch, and the transcript takes that rule's second option — the named function **`escape_transcript_text`** with its own verification. **The obligation is stated as a round trip rather than as an alphabet**, which is what carries block C verbatim, and the unescaping is stated to be ungenerous so a hand-edited transcript fails rather than replaying as something the escaper could not have written. **Rule 11.7's estimated band is withdrawn for measured figures**, and 11.7.1 records the withdrawn wording rather than deleting it: the committed transcript is **305,568 bytes** at 20 ticks — twelve prefix records totalling 67,447, block A 5,385 bytes each time, 221 exchange records totalling 238,121, a mean exchange record of 1,078 — against a superseded band of "100 to 260 KB for a 20-to-50-tick run", which 298 KiB at 20 ticks already exceeds, and an extrapolated 12 MB at 1,000 ticks against a superseded estimate of 4.7 MB. The estimate was low because blocks C and D are larger than it assumed and not because anything is written twice, which is measured rather than argued: inlining blocks A and B would take the mean record to 6,502 bytes. **No size ceiling is added**, and rule 11.7's first sentence is given as the reason. **What this row does not do**: no rule outside rule 11 is amended, `schema` is not touched — the transcript is not `SPEC-MOK-006`'s stream and carries no `schema` — and no relation, actor, input, output or security property of this specification moves. Rules 8.1 and 8.2 are **not** amended: 8.2 already reads "its parameter", so 11.3.3 records the field name agreeing with the grammar rather than changing it. | **Approved 2026-08-24 by the repository owner acting as accountable technical owner**, in two acts, both recorded because neither alone is sufficient. The **substance** is the owner's six rulings of 2026-08-24, taken over commit `bfdbf71` when the implementation of rule 11 reached each contradiction in turn, and taken as rulings rather than as an amendment because an implementation agent may not amend an approved artifact: that the response and usage fields be present and empty until `WO-MOK-026`; that rule 11.4's closed alphabet be replaced by a named escaping function with a round-trip obligation so block C is carried verbatim; that the prefix digest be FNV-1a 64 for drift detection and not adversary resistance; that the action's second field be `parameter` and not `target`; that rule 11.7 carry the measured figures and drop its estimated band, with no size ceiling added; and that `--transcript-path` and both hosts' wiring go with `WO-MOK-025` scope item 9. **The sixth ruling amends nothing here** and is recorded as discharged rather than omitted: it is a scheduling decision about a work order's items, it was carried out at commit `8f31792`, and it leaves no text in this specification. The **routing** is the second act: `ADR-MOK-007`'s *Required amendments* section does not name this artifact and `WO-MOK-025` scope item 14 does not list it, so writing these rows was put to the owner under that work order's stop-and-escalate condition 6 with three options measured — write them here now, defer them to `WO-MOK-026`, or approve a draft in a separate act first — and the owner chose to write them here now, in the turn the question was asked. The alternative of deferring was declined on the ground the question stated: this branch would otherwise merge carrying an approved specification whose stated size band is measurably false. The implementation agent measured every figure in this row at the candidate commit and wrote the text; it decided none of the substance. **The estimated figures elsewhere in this specification are untouched** and remain estimates: rule 3.5's cacheable prefix share, the *Scope* preamble's 10,954 decision opportunities and its `gpt-5.6-luna` prices, and rule 9.8's $1.04. Only rule 11.7's became measurable, because only rule 11.7's describe a file that now exists; the others describe a provider call, and no provider has been called. |
| 2026-08-24 | **Rule 11.4.1's list of characters putting block A outside the record stream's alphabet is corrected, and the correction is a measurement.** The list as approved read "spaces, commas, parentheses, full stops and an em dash", and **two of those five are wrong in opposite directions**: block A contains **no parenthesis at all**, and a **full stop is inside** the alphabet — `A-Z a-z 0-9 _ . - + : ; >` includes `.`, and block A carries 65 of them. The list was also **incomplete**: it omitted two characters that do occur and are outside, **9 less-than signs** and **5 apostrophes**, the first of which is easy to miss because `>` is in the alphabet and `<` is not. Corrected to what was measured over the committed transcript's block A: **1,282 spaces, 44 commas, 9 less-than signs, 5 apostrophes and 2 em dashes**, with 90 newlines already covered by the clause that all four blocks are multi-line. The rule now states that the list is measured and not inferred, and names the two characters inference gets wrong, so that a later reader re-deriving it does not reproduce the same error. **The rule's conclusion does not move**: the alphabet still does not hold and still cannot, `SPEC-MOK-006` rule 3.4's second branch is still the one taken, `escape_transcript_text` is still the function, and the obligation is still a round trip rather than an alphabet. **Three sites carried the same list and all three are corrected**, which is why this is one row and not three: this rule, and two restatements of it in `mokiterions-core/src/simulation.rs` — the module comment above the escaping function and the documentation comment on `every_block_survives_the_escaping_unchanged`. Both are comments, so no behaviour moves and no test changes; the gates were re-run because a package file changed. **What this row does not do**: rule 11.4 is not amended, no other rule is touched, and no figure in rule 11.7 moves. | **Approved 2026-08-24 by the repository owner acting as accountable technical owner.** The defect was found by the implementation agent while writing `WO-MOK-025`'s completion report, measuring the list rather than restating it, and was put to the owner as that work order's escalation **E11** together with its ten siblings, with the measurement displayed and the consequence stated: correcting the two source restatements moves a package file, so the candidate commit moves and the gate readings must be re-taken. The owner approved the correction at all three sites in the turn the question was asked. The implementation agent measured every figure in this row and wrote the text; it decided none of the substance, and it did not decide that an approved rule could be corrected. |
| 2026-08-28 | **Three things this specification left open are stated, all three found by building against it rather than by reading it.** **Rules 10.3a, 10.4a and 10.4b** fix the wire format's field names. Rules 10.3 and 10.4 fixed what a request and a response *carry* and never what their keys are called, and a connector is written by someone outside this repository against a document — so an unnamed contract is not a contract: two independent connectors would disagree and neither would be violating anything. The names are recorded as `docs/CONNECTOR_PROTOCOL.md` already documents them rather than chosen afresh, because that document was published under `WO-MOK-026` before this amendment and a reader may already have built against it. **Rule 19.5a** fixes the error vocabulary. Rule 19.5 named a transport failure and its bounded retry and nothing else, and the difference between kinds matters in both directions: retrying a malformed answer re-sends an identical prompt hoping for a different reply, while *not* retrying a rate limit throws away a run over something that would have cleared. Four kinds — `transport` and `provider` retried, `malformed` and `refused` becoming immediate counted fallbacks under `REQ-MOK-074` — with every attempt a transcript record under rule 11.2 whichever kind it carried, and a kind outside the four treated as `malformed`, because rule 10.7 makes the connector untrusted in whole and that includes its error vocabulary. **Rules 18.4.2 and 18.4.4** add the fifth binary-target option, `--transcript-output`. `--transcript-path` was shipped by `WO-MOK-025` meaning *read*, a live run has nothing to read and must write, and rule 19.6 makes failing to write the one failure worth aborting a live run for. Rule 18.4.2 listed four options and the fifth had nowhere to be; the two are mutually exclusive and giving both is a usage error under rule 19.2. **What this row does not do**: rules 1 to 9, 11 to 17 and 20 are untouched, no figure moves, nothing executable changes, and `SPEC-MOK-004` rule 1's separate drift — its tree says five and eight test files where the tree holds ten and nine — is recorded as a finding under `WO-MOK-026` and deliberately not corrected here. | **Approved 2026-08-28 by the repository owner acting as accountable technical owner**, in three decisions taken in the turn each question was asked. That the field names become **normative in this specification** rather than staying repository-owned in a document that authorizes nothing, because a third party's connector depends on them exactly. That **all four error kinds** get defined handling rather than the vocabulary being reduced to the one rule 19.5 already named. That `--transcript-output` be a **separate option** rather than `--transcript-path` changing direction by mode, so `WO-MOK-025`'s help text stays true unchanged — a sentence that said "read" and came to mean "read or write depending on what else you passed" is the shape of defect rule 18.3 exists to correct. A fourth decision routed the work: the amendments were taken in their own work order rather than inside `WO-MOK-026`, whose *Expected change surface* never named this specification — it was written assuming this text already said everything stage 5b needed — and whose execution scope refused the file with `QGP-G4I-PATHS: WEX201`. That refusal is the boundary working, and keeping the amendments separate leaves stage 5b's diff implementation and this one specification repair. The implementation agent found all three gaps while building `docs/CONNECTOR_PROTOCOL.md` and the canned connector, and wrote the text; it decided none of the substance. |
| 2026-08-28 | **The three options this specification described eleven times and never named are named**, under `WO-MOK-029`. **Rule 18.4.0** fixes `--connector-path <path>` for the executable the host spawns, `--live` for rule 13.1's explicit live-mode selection, and `--spend-ceiling <amount>` for rule 14.6's declared ceiling. **Rules 18.4.2 and 18.4.3** are updated to use the names in place of the descriptive phrases, so the two cannot drift apart. Before this row the specification named `--transcript-path` and `--transcript-output` and referred to the other three only as "the connector path", "a live-mode selection" and "the spend ceiling" — rule 18.4.2 listed five options a host acts on and spelled two of them. An unnamed option is the same defect as an unnamed field, which the row above closed: the operator types these, a reader of this specification could not tell what to type, and two implementations would offer different command lines while both conforming. Each name follows the surface that already exists rather than a scheme invented for it. **What this row does not do**: rules 1 to 17, 19 and 20 keep their text, rule 18's existing sub-rule numbers do not move, no figure changes, nothing executable changes, and the unit and grammar of `--spend-ceiling`'s amount are left to rule 14.2's integer arithmetic and to `WO-MOK-026`'s implementation. | **Approved 2026-08-28 by the repository owner acting as accountable technical owner**, who chose the three names in the turn the question was asked, together with the routing decision that they be amended under a successor work order rather than added to `WO-MOK-028`. That work order was already `implemented` and `QGS-EDGE` refuses `implemented` to `in_progress`, so absorbing the work would have meant either reopening a closed work order or leaving the names unwritten. The gap was found by `WO-MOK-028`'s own stop-and-escalate condition firing — *"a fourth thing turns out to be unspecified, rather than being folded in silently"* — as the parser work it authorized was starting. The implementation agent found the gap, measured its eleven sites and wrote the text; it decided none of the substance. |
| 2026-08-29 | **Four gaps and one contradiction closed, so that stage 5b can be built without stopping.** **Rule 14.2** states the minor unit: the **US cent**. The rule required cost to be an integer in a "stated" minor unit and stated none, while every cost figure here is written in dollars — so the currency was implied throughout and normative nowhere, leaving `--spend-ceiling`'s amount and the run record's cost integer denominated in nothing. **Rule 14.3a** gives the prices an input: `--prices`, four integers in cents per million tokens, colon-separated, validated by the shared parser and **retained** because the run computes with them (`VER-MOK-018` case `S6a` scopes the discard rule to paths). Until this row rule 14.3 called the prices inputs of the run and no input carried them, which left a compiled-in constant — the one thing that rule forbids — as the only thing an implementation could do. **Rule 18.4.2** lists **six** binary-target options; it said four until `WO-MOK-028` and five until `WO-MOK-029`. **Rule 19.5** fixes the retry bound at **three**, so an exchange is attempted at most four times; the rule said "bounded" and gave no number while rule 11.2 makes each retry "its own billed exchange", so an unstated bound was an unstated spend. **Rules 10.3a, 10.4a and a new 10.4c** move the provider binding to the connector: the request drops `model` and `reasoning` and the response gains them. That one is a **contradiction rather than a silence** — rule 10.3 had the engine's request carry both, rule 15.2 has the engine's run record carry them, and `WO-MOK-026` item 5 declares them "in the connector rather than in the engine", which cannot all hold. Resolving it this way makes rule 15.2's report a record of the model that **answered** rather than of the model that was asked for, because a run record naming the wrong model is worse than one naming none. The six were found by a **conformance pass** the owner directed on 2026-08-29, after `WO-MOK-028` and `WO-MOK-029` had each been authored, approved, evidenced, transitioned and pushed to close one gap apiece and a third and fourth appeared immediately after. The pass read the specifications against what stage 5b's remaining items need rather than waiting for the next stop, and is retained at `../evidence/WO-MOK-026/conformance-pass.md`. **What this row does not do**: no other rule moves, no figure changes, every estimate stays an estimate, and `docs/CONNECTOR_PROTOCOL.md` — which rule 10.4c makes wrong about the request and the response — is **not** corrected here, being `WO-MOK-026`'s file and outside this work order's scope. That disagreement is disclosed rather than left to be found. | **Approved 2026-08-29 by the repository owner acting as accountable technical owner**, in four decisions taken in the turn each question was asked: the US cent as the minor unit; `--prices` as a compact option rather than a file; a retry bound of three; and the provider binding staying in the connector with the response reporting it back, over the two alternatives of telling the engine or leaving the request's fields advisory. A fifth decision routed the work into one chain rather than four. The implementation agent ran the pass, measured every figure and wrote the text; it decided none of the substance. |
| 2026-08-29 | **The port's return grows to carry what rule 11.3 obliges the engine to record, and one item of rule 19.2's list is recorded as unreachable.** **Rules 1.1 and 1.4 amended, and 1.1a and 1.4a added.** Rule 11.3 obliges an exchange record to carry the response as received, the four reported counts, and "the action the response was parsed into, or the fact that it was not parsed **and why**", while rule 11.1 puts the authoring of every record in the engine and rule 1.1 gave the engine one interface returning "either a proposal or the fact that none was obtained" — so the engine was obliged to write three things it had no route to. `mokiterions-core/src/simulation.rs` has carried this since `WO-MOK-025` as "a pre-existing tension between rule 1.1's port shape and rule 11.3's field list", naming this work order as where the return type must grow, and rule 11.3.1 names it from the other side. The return now carries the evidence beside the proposal, and **1.4a states what does not change**: rule 1.4 is undisturbed, the action applied is still the action type or its absence, the engine reads nothing else to decide, and rule 9's validation is untouched. Two alternatives are recorded as declined — leaving the rules word for word and recording the growth's admissibility in a different artifact, and a second port method the engine called after each proposal, whose cost was a temporal contract between two calls that no type enforces. **Rule 19.2a added.** The list item *a live-mode selection with no credential* cannot be reached where 19.2 places it: rule 13.1 puts that condition's check in the connector, rule 13.3 has the refusal arrive after the spawn on the first exchange, and rule 13.4 forbids either host to read the credential — so no host can observe it before a tick, and rule 19.5a makes a `refused` response an unconditional immediate counted fallback that a host cannot except this case out of without interpreting a message rule 10.7 makes untrusted. The sub-rule records the contradiction, states that a run follows rule 19.5a as the more specific and later-dated rule, and states that the four remaining items and rule 13.5's ceiling refusal are unaffected. **The defect is recorded and not repaired**, striking or restating an approved rule's item being a change of substance. **What this row does not do**: rules 2 to 18 and 20 are untouched; no figure moves and every estimate stays an estimate; rule 11.3 and rule 11.3.1 are **not** amended, this row supplying the route those rules already assumed rather than changing what they ask for; rule 13 is not touched and neither gate is relaxed; and rule 19.2's own wording is left standing, so a reader meets the contradiction at 19.2a rather than finding the item quietly gone. | **Approved 2026-08-29 by the repository owner acting as accountable engineering owner**, in two decisions taken in the turn each question was asked, each with the alternatives' costs measured and displayed: that the port's return grow and **these rules be amended to say so**, over the two alternatives above; and that a run follow rule 19.5a with 19.2's item disclosed as a defect, over giving a first-exchange refusal a distinct terminal status — which would have contradicted 19.5a for every other refusal — and over amending 19.2 now. A third decision was the **authority to edit this file at all**: this specification was outside `WO-MOK-026`'s execution scope, `harnessctl check … --changed-path docs/engineering/simulation/specifications/SPEC-MOK-007.md --changes-complete` measured `QGP-G4I-PATHS: WEX201` and directed the escalation under `DR-REMEDIATION-SCOPE`, and the owner was shown that measurement with two routes — a scope amendment to `WO-MOK-026` or a fifth governance work order — and chose the scope amendment, the alternative being declined and recorded in that work order's own amendment record together with a second admission, `mokiterions-tui/src/state.rs`, which the trait change forces. The implementation agent found both gaps by reading this specification against the code while starting item 8, measured each and wrote the text; it decided none of the substance, and it did not decide that an approved rule's list item could be struck. No record bound to a commit is re-opened. |
| 2026-08-29 | **Rule 1.1b added: the port answers whether the run has stopped spending, and the engine asks before each exchange.** Rule 14.6 requires the ceiling check to precede the spending and rule 20.4.1 puts the accumulated cost in the port, so the engine had no route to the figure the comparison needs — rule 1.1 as approved gave it one interface returning a proposal and the evidence of the exchange, and nowhere to ask about the run. The sub-rule fixes that the **same** interface answers one further question, that the engine asks it at each decision opportunity **before the request is composed**, and that asking moves no figure, writes no byte, reads no line and says nothing to whatever is behind the port — so a run's three streams are the same bytes however often it was asked, including at the opportunity where the answer is yes and no exchange follows. It also fixes the default: a port with no declared ceiling answers that it has not stopped, for ever, which is how rule 14.8's *a replay has no ceiling* holds without a replay implementing anything. **It is not the second port method rule 1.1a declined**, and the sub-rule states the difference in terms rather than by assertion: 1.1a declined a method called *after* each proposal, whose correctness was a temporal contract no type enforced and which would have written a wrong record in silence; this one is asked before the exchange it decides, concerns the run rather than one exchange, and if it answered from a figure one exchange out of date would stop the run early and never late. One alternative was measured and declined: **a field on the proposal** marking it halted, which needs no ordering contract at all but makes "no exchange was issued" and rule 9.5's "the exchange yielded nothing" two absences one field apart — and confusing those two writes a transcript record and counts a fallback for an exchange that was never issued. **What this row does not do**: rule 1.1's own words are untouched and its **one** is stated to be undisturbed, an interface being one door however many questions are asked at it; rules 14.6, 14.7, 14.8, 15.5 and 19.3 are **not** amended, this row supplying the route those rules already assumed rather than changing what they ask for; **rule 19.3's status number is left where rule 19.3 leaves it** — that rule requires a status distinct from a clean completion and from an error and fixes no number, the implementation chose `3` because the three it must differ from are taken, and `SPEC-MOK-002` rule 5's own amendment record carries the public constant that holds it; no figure moves and every estimate stays an estimate. **Two disagreements found while writing this row are disclosed and not corrected**: this specification's `updated` field still reads `2026-08-24` after four amendment rows dated later, and *Explicitly unspecified decisions* still calls the retry count "the implementation's" after the 2026-08-29 row fixed the bound at three. Both are `updated`-and-prose bookkeeping on an approved artifact, which an implementation agent may not decide. | **Approved 2026-08-29 by the repository owner acting as accountable engineering owner**, in one decision taken in the turn the question was asked, with both options' growth measured and their previews displayed: that a ceiling stop reach the engine as **a `Proposer` method asked first**, over a field on the proposal. The growth is **0 items and 0 public declarations** under the 2026-08-24 convention, a trait method not being a `pub fn` declaration, which is why `SPEC-MOK-002` rule 5's amendment for this commit counts the exit-status constant and nothing else. The authority to edit this file at all is the scope amendment the row above records — `WO-MOK-026`'s execution scope admitted this specification on 2026-08-29 by the owner's decision — so no further routing question arises and none was put. The implementation agent wrote the rule and this row **in the same commit as the method**, which is what the owner's decision on rules 1.1 and 1.4 established as this work order's practice; it decided none of the substance. No record bound to a commit is re-opened. |
| 2026-08-29 | **Rule 1.1c added: the proposal carries the exchanges the opportunity spent before it, which is how rule 19.5's retried attempts reach the engine.** Rule 19.5 retries a failed exchange up to three times, rule 11.2 gives each attempt "its own record, because it was its own billed exchange", and rule 11.1 puts the authoring of every record in the engine — so the attempts have to cross the boundary, and rule 1.1's one interface is the only route. The sub-rule fixes that the return is the proposal the opportunity **ended on**, carrying the earlier exchanges oldest first and empty for the single-attempt case that is every exchange of every port but a retrying one; that **rule 11.3's fallback flag is marked on the outcome's record alone**, an attempt that was retried not having been the opportunity's decision, which is what keeps rule 15.4's count reconciled against the marked records and `VER-MOK-018` case **P5** checkable; and that no earlier attempt carries an action, because an attempt that obtained one ended the retrying. It also fixes that **the retrying stays in the port**, and records that as forced rather than preferred: the port bills each attempt under rule 14.1 and holds the fallback count under rule 20.4.1, so an engine-driven loop would need a further act at the interface to say "exhausted" or the count would be wrong for a retry that later succeeded. Two alternatives were measured and declined: **a named `Attempt` type**, one item and four public declarations to state what the proposal already states and needing a rule fixing which of its fields the engine reads; and **a proposing method returning a list**, which declares nothing new but leaves *which element is the outcome* an invariant no type enforces, so a port returning them in the other order would have the engine author the outcome as an attempt and mark no record at all. **What this row does not do**: rules 1.1, 1.3, 1.4 and 1.4a are untouched and are each stated to hold of an earlier attempt unchanged, an attempt being the same values rule 1.1a already fixes; **rule 19.5 is not amended** — the bound, the vocabulary of 19.5a and rule 14.6's check before each attempt are its own words, and the check inside the retry loop is that sentence discharged rather than a new rule; rules 11.1, 11.2 and 11.3 are **not** amended, this sub-rule supplying the route rule 11.2 already assumed; **the backoff shape stays the implementation's** under this work order's envelope and no rule is added for it — the shape chosen is none, because rule 11.4 admits no timestamp in a transcript, so nothing this repository retains could show a delay happened and case **R5** forbids making a wall-clock figure a pass condition. **One consequence is disclosed and deliberately not repaired**: a transcript holding a retried exchange is not a replay input, rules 11.2 and 12.3 not reconciling, and the sub-rule records the failure as the loud refusal it is — two tests characterise it — together with the two possible repairs and the fact that both are changes of substance in an approved rule. The consequence for this work order is stated with it: a live run that retried cannot supply case `L30`'s replay identity. **The two disagreements disclosed in the row above are still not corrected**, this row adding a fifth amendment dated later than the `updated` field it does not move. | **Approved 2026-08-29 by the repository owner acting as accountable engineering owner**, in one decision taken in the turn the question was asked, with all three options' growth measured and their previews displayed: that the attempts reach the engine as **earlier attempts on the proposal**, a single new public field of type `Vec<Proposal>`, over a named `Attempt` type and over a proposing method returning a list. The owner's stated reason is recorded because it is the reason the sub-rule is written this way and not merely the choice: "it is the same shape you chose twice — the evidence travels with the proposal that carries it, no contract between two calls". The growth is **0 items and 1 public declaration** under the 2026-08-24 convention, a public field being one declaration, and `SPEC-MOK-002` rule 5's amendment for this commit counts exactly that. The authority to edit this file at all is the scope amendment two rows above — `WO-MOK-026`'s execution scope admitted this specification on 2026-08-29 by the owner's decision. The implementation agent wrote the rule and this row **in the same commit as the field**, this work order's established practice, and decided none of the substance; the disclosure of the replay gap is a statement of a measured fact and takes no decision, the repair being reserved. No record bound to a commit is re-opened. |
| 2026-08-29 | **Rule 1.1d added: the port reports the run's accounting, once, after the run has ended, and the absence of that report is rule 15.6.** Rule 15.1 makes the run record the one place every accounting figure this specification produces is stated, rule 20.4.1 keeps every accumulator in the port, and rule 11.1 puts the authoring of every record in the engine — three rules leaving one gap, which is that the figures were on one side of rule 1.1's boundary and the record on the other with no route between them. The sub-rule fixes that the **same** interface answers one further question, asked **once after the run loop has returned** so that rule 14's *State model* stays literal — no accounting figure may influence a decision, and there is no decision left to influence — and that what crosses is **a value the engine formats**, every figure an integer or an absence and **none a floating-point value**, which is case **P6** and why rule 14.4's ratio crosses as basis points. **The absence of an answer is rule 15.6 and needs no branch**: a port that spent nothing reports no account, so a replay reports no run record by answering rather than by a caller knowing what kind of port it holds — which rule 1.1 gives no way to ask and rule 1.2 forbids branching on. That is the blocker `mokiterions-core/src/simulation.rs` has carried since `WO-MOK-025`, that the engine "cannot tell a replaying port from a live one"; it is removed by the shape of a return and not by a party's judgement. **The model identifier and the reasoning level are the first reported and are never replaced**, which is what makes two figures rule 15.2 requires well defined for a run rather than a function of which exchange happened to be last; a run whose provider reported neither states neither, as rule 11.5's absence. **The destination of the rendered line is the host's and this rule names none**, on rule 11.1's own division of labour, and the recording host places it on **standard error** — the one destination that amends no approved rule, rule 12.6 claiming byte-identity for standard output, the structured record stream and the exit code and saying in terms that it is "not claimed for standard error". **Two things are disclosed and deliberately not repaired.** A connector that changed models mid-run is not detected and the record names the first: rule 10.7 declares the connector's output untrusted in whole, no rule obliges a provider to be consistent about what it is, and detecting it means a rule fixing what such a run reports. And **`SPEC-MOK-006` rule 8.9 contradicts rule 12.6** — its `live` object is present only for a connector-sourced run, which is a difference in the record stream's bytes between a recorded run and its replay, and rule 12.6 claims those bytes identical. This rule follows rule 12.6, whose identity is `REQ-MOK-067`'s promise and case **L7**'s pass condition; rule 8.9's text is left word for word, that specification being outside this work order's scope, and the defect is recorded with this stage's evidence. The two run records are stated to be different records and the spelling is what keeps them apart: rule 8's `record` on the structured stream, rule 15's `run_record` on standard error. **What this row does not do**: rule 1.1's own words are untouched and its **one** is undisturbed, an interface being one door however many questions are asked at it; rules 15.1 through 15.6, 14.4, 14.5, 11.5 and 12.6 are **not** amended, this row supplying the route those rules already assumed; rules 1.1a, 1.1b and 1.1c stand word for word and this question is distinguished from each in the sub-rule's own text; no figure moves and every estimate stays an estimate. **The two disagreements disclosed two rows above are still not corrected**, this row adding a sixth amendment dated later than an `updated` field reading `2026-08-24`, and the tension is now sharper rather than merely older: `SPEC-MOK-002`'s own 2026-08-29 row treated the same staleness in that file as "a corrected fact and not an amendment" and moved it, while rows 1.1b's and 1.1c's called it here "bookkeeping on an approved artifact, which an implementation agent may not decide". Both readings cannot be right, this row does not choose between them, and the choice is put to the owner rather than taken. | **Authorized 2026-08-29 by the repository owner acting as accountable engineering owner**, in three decisions taken in the turn each question was asked, each with the alternatives measured and displayed. **That the accounting reach the engine as a value type the engine formats**, framed at 1 item and 10 declarations, over a port that rendered rule 15's line itself — 0 items and 1 declaration, cheaper on `SPEC-MOK-002` rule 5's census and putting this specification's field set, spelling and escaping behind rule 1.1's boundary — and over a family of getters, rule 1.1a's declined shape a third time. **That the rendered line go to standard error**, over a sixth command-line option naming a path and over rule 8.9's structured record stream, with the measured cost stated in the framing: three existing `assert!(stderr(&output).is_empty())` in `mokiterions-core/tests/connector.rs` change, and a successful live run stops being silent on the diagnostic stream. **That rule 8.9's conflict be recorded as a defect** and rule 12.6 followed, over amending rule 8.9 — which is another specification's rule and outside this work order's scope — and over dropping the record stream's `live` object, which is the same amendment by another route. A fourth decision, taken the same turn, is that the binding is **the first reported and never replaced**, over the last reported and over refusing a run whose model changed. The **authority to edit this file** is the scope amendment recorded four rows above; `WO-MOK-026`'s execution scope admitted this specification on 2026-08-29 by the same owner's decision, and no further routing question arises. **`SPEC-MOK-002` rule 5's amendment for this commit measures the growth at 1 item and 12 public declarations, not the framing's 10**, and states the difference at the rule: the two fields beyond the framing's reading are `cache_ratio_basis_points` and `ceiling_cents`, both named in rule 15.2, and neither reverses a decision that was between a value, a rendered line and a family of getters. The wrong arithmetic in the framing is recorded rather than reconciled away, because a wrong cost figure in a framing is a wrong figure in the decision record even where the decision stands. The implementation agent wrote the sub-rule and this row **in the same commit as the method and the type**, this work order's established practice, and decided only what the envelope reserves to it — the eleven field names, the basis-point representation, and that no field is a floating-point value. No record bound to a commit is re-opened. |
| 2026-08-29 | **Rule 11.7 is made plural and rule 11.7.2 added, because this repository now commits two transcripts and the rule said "the".** Rule 11.7's figures were true of the file it named and stay true; what was false was the definite article, in the one place a reader goes for what a transcript costs to keep. The rule now states that there are **two** and that **rule 11.3.1 is why neither can stand for the other** — the first was written with nothing behind the port, the second with a provider behind it — and **11.7.2 carries the second one's measured figures**: `docs/engineering/simulation/evidence/WO-MOK-026/live-run-transcript.jsonl`, a 50-tick run at seed 0 and density 0.75 against `gpt-5.6-luna` at reasoning level `none`, **700,192 bytes**, twelve prefix records totalling **67,447** with block A **5,385** each, 503 exchange records totalling **632,745**, mean **1,258**, and no carriage return, the file being committed under `.gitattributes`' `-text` rule so those are the bytes as captured. **Three of those figures do work beyond stating a size.** The prefix total and block A's size are **identical to rule 11.7's to the byte**, so the two transcripts cross-check each other's prefix — blocks A and B do not vary within a run and do not depend on what is behind the port. The mean's growth of 180 bytes is *less* than the 251 the populated response and usage fields add (mean span 326 against 75), and the remaining −70 is recorded as **not attributed** to rule 11.3.1: the two runs share a seed but not their decisions, so their blocks C and D describe different worlds and only the prefix is comparable. And **the 12 MB extrapolation is confirmed at 12.7 MB** — 12,722,347 bytes at 10.06 exchanges per tick — making it the first size figure in this rule to survive a larger run, rule 11.7.1's superseded band having been exceeded already at 20 ticks. The frontmatter's `updated` moves to **2026-08-29**, which this branch's five earlier rows did not do and should have. **What this row does not do**: no rule outside 11.7 is amended, rule 11.7's own figures do not move, and **no size ceiling is added** for rule 11.7.1's reason. In particular **the third bullet of *Explicitly unspecified decisions* is deliberately left as it stands** although block D's flat-versus-nested trade-off has now been measured under this work order — that bullet states what this *specification* declines to fix, which a measurement by an implementation does not change, and turning a measured implementation choice into a normative one would narrow the rule rather than correct it. | **Approved 2026-08-29 by the repository owner acting as accountable technical owner.** The drift was found by the implementation agent after committing the live run's captures, and was put to the owner under `WO-MOK-026`'s stop-and-escalate condition 6 with three options and their costs stated — amend here, `SPEC-MOK-007.md` being already in this work order's execution scope; record it as a defect, as this work order did with the third target and with `SPEC-MOK-006`'s duplicate rule 8.9; or defer it to a governance work order behind a stacked pull request. The owner chose to amend it here, in the turn the question was asked, on the ground the question stated: the alternative merges an approved specification whose definite article is false about its own branch's committed evidence. The implementation agent measured every figure in this row at the candidate commit and wrote the text; it decided none of the substance, and it did not decide that an approved rule could be amended. |
| 2026-08-29 | **Rule 14.2a is added: the reported output count contains the reported reasoning count, so the billable output is the difference.** Rule 14.2 said cost is computed "from the reported counts", which reads as four disjoint quantities and was implemented as four; the provider's output count is a total containing its reasoning count, so pricing both in full **bills every reasoning token twice**. The rule now states the subtraction, clamps the reasoning count to the output count before it, and says that rule 14.1's four run totals stay exactly as reported so a reader recomputing from them must subtract likewise. **The fault was measured, not reasoned about**, and it was found by a live run rather than by a check: across all 567 exchanges of this work order's rejected first attempt, reported output minus reported reasoning was between 18 and 26 with a mean of 24.1 — the size of one action object every time — and the true cost was **28.21 cents** against the **37.37** the engine computed. **This rule already had the convention right for the other inclusive pair**: rule 14.4's ratio is cached prompt over total prompt, and cost has always been `prompt − cached` at the uncached price plus `cached` at the cached price. The two pairs have the same shape and one was handled; that asymmetry is the whole defect. **Why no check caught it**: no cost assertion anywhere in the engine crate declared a non-zero reasoning count, and the internal fixture sets the reasoning price *equal* to the output price — which is how a provider bills them and also what makes a misattribution between the two invisible. Both gaps are closed by one new internal-tier case, verified to fail against the previous arithmetic at 501,400 microcents against the correct 301,400. **What this row does not do**: no retained figure moves. The fault is reachable only when a run reports reasoning tokens, and this stage's accepted run reports none, so its 16.67-cent cost, its run record and its replay are unchanged — measured by the full suite passing before and after. Rules 14.1, 14.3, 14.3a and 14.4 through 14.8 are untouched, no price is added, and the **fifth** quantity this provider returns — `cache_write_tokens`, which rule 14's four prices cannot express — is **not** addressed here and remains an open finding in this stage's evidence, because whether it is charged instead of or in addition to the input rate is unread and would decide the shape of any price for it. | **Authorized 2026-08-29 by the repository owner acting as accountable technical owner.** The defect was measured by the implementation agent, put to the owner with both dispositions costed — amend now, or record as a defect for a later work order — and the owner chose to amend it here. The framing stated the measured fact that made it cheap: because the accepted run reports no reasoning tokens, the corrected arithmetic yields the identical figures and no evidence is invalidated. `WO-MOK-026` item 9 already places the cost arithmetic against real usage in scope and `SPEC-MOK-007.md` is already in this work order's `[execution_scope]`, so no scope act was needed for it. The implementation agent measured every figure in this row and wrote the text; it decided neither that the rule should be amended nor when. No record bound to a commit is re-opened. |
| 2026-08-29 | **Rule 11.7's count goes from two to three and rule 11.7.3 is added, because the owner's retention disposition of the same day commits a third transcript.** Rule 11.7 had been made plural earlier on this branch and said **two**; attempt 1's captures were then retained under `docs/engineering/simulation/evidence/WO-MOK-026/attempt-1/`, and the count was false again for exactly the reason it had been false before — this rule is where a reader goes for what a transcript costs to keep, and it now has to name three. **Rule 11.7.3 carries the third one's measured figures**: **759,168 bytes**, twelve prefix records totalling **67,447** with block A **5,385** each, 567 exchange records totalling **691,721**, mean **1,220**, **no carriage return**, and a 1,000-tick extrapolation of **13,901,867** bytes at 11.34 exchanges per tick. **Three of those figures do work beyond stating a size.** The prefix records are shown to be **byte-identical across all three transcripts**, by one digest over each file's twelve concatenated prefix records — which rule 11.7.2 could only infer from equal totals, and which now covers the horizon as well, 20 ticks against 50, so blocks A and B are established to depend on the seed and the density and on nothing else. The populated span's mean is **325** bytes against rule 11.7.2's **326**, which is what lets the mean exchange record being 38 bytes *smaller* be recorded as **not attributed** rather than as an unexplained shrinkage. And the 12 MB extrapolation is confirmed a second time at a different exchange rate. **Why the file is retained at all is stated in the rule rather than left to the evidence**: rule 14.2a's measurement is taken from these 567 exchanges and from no other file in this repository, so discarding them would leave that rule un-re-derivable. **What this row does not do**: no rule outside 11.7 is amended; rule 11.7's, 11.7.1's and 11.7.2's own figures do not move; no size ceiling is added, for rule 11.7.1's reason; and the retention **layout** is not described here — `VER-MOK-018`'s *Evidence retention* is where that lives, and `WO-MOK-026`'s evidence packet is where its departure from "one directory per run" is disclosed. | **Approved 2026-08-29 by the repository owner acting as accountable technical owner.** The substance is the owner's retention disposition of 2026-08-29 — retain attempt 1's captures, taken when the implementation agent put the option and its cost after attempt 1's figures turned out to carry rule 14.2a — together with the same act that authorised this branch's earlier rule 11.7 amendment under `WO-MOK-026`'s stop-and-escalate condition 6. **This row is the consequence of that disposition rather than a second decision**: retaining a third transcript and leaving this rule saying "two" is the state the owner declined earlier the same day, on the same rule, for the same reason. The implementation agent measured every figure in this row at the candidate commit and wrote the text; it decided none of the substance, and it did not decide that an approved rule could be amended. **It is reversible in either direction and was flagged to the owner as a consequential act rather than as a separately authorised one**: withdrawing the retention deletes this sub-rule together with the file, and directing that the drift be recorded as a defect instead replaces it with a defect record. |

## Actors and external systems

- **The engine's library target** composes each decision request from the observation it already holds, hands it to the
  port, and receives a proposal. It resolves no path, opens no file, creates no directory, removes no file, opens no
  socket, spawns no process and reads no environment variable. It is the sole author of every request's content and of
  every transcript record's content.
- **The engine's binary target — the recording host** parses the options, resolves the connector path and the transcript
  path, spawns the connector, opens the transcript for writing or for reading, hands the library a connected port, and
  flushes and closes what it opened. It authors no request content and no transcript record content, and **it does not
  read the credential**: rule 10.5 places that in the connector alone.
- **The terminal observer's binary target — the replay host** resolves a transcript path, opens it for reading, hands
  the library a connected port, and closes it. It spawns nothing, reads no credential, takes no ceiling and has no
  live mode. Rule 20 states why, and `ARCH-MOK-002` is amended to record it.
- **The connector** is an executable the operator names by path. It receives request lines, reaches the provider, and
  returns response lines. It reads the credential from its own process environment and is the only component that holds
  one. It is not a package, not a workspace member, and this specification does not require it to be in this repository —
  which is a limit as well as a freedom, and rule 10.6 states the limit.
- **The provider** is `gpt-5.6-luna`, reached only by the connector. It is not part of this repository, it is not
  deterministic, and nothing here assumes it is. No component other than the connector names it in a network sense.
- **The repository owner** authorises a live run. The authorization is a retained artifact, rule 17, and no code
  consults it.
- **A consumer** is any program that reads a retained transcript. No consumer is specified. The transcript is specified
  so that writing one requires no knowledge of the provider.

## Inputs

The decision source is selected by a command-line option, rule 18. Beyond the inputs every run already takes — seed,
tick limit, density, tracing selection, record-stream sink — a run under this source takes:

- **A mode**: live or replay. Replay is the default, rule 13.1, and it is the only mode the replay host offers.
- **A transcript**: an open stream a host supplies. Written in live mode, read in replay mode. The engine never names
  it.
- **In live mode only**: a connector path, a spend ceiling, and a model identifier with its reasoning level. The
  **credential is not an input to the run.** It lives in the connector's own environment, rule 10.5; no host reads it,
  no option carries it, and it reaches the library target by no route at all.

No input reaches the simulation's rules. A Mokiterion's behaviour is a function of the observation and the response,
and of nothing else in this list.

## Outputs

- The **standard output text stream** of `SPEC-MOK-001`, unchanged in form. Under this source it carries `llm` where
  it carries a source name, and nothing else about it moves. **The emitted name and the option value are the same string**,
  as they are for the four existing sources, so `SPEC-MOK-006` rule 3.2's two domains gain one value between them rather
  than one each.
- The **structured record stream** of `SPEC-MOK-006`, unchanged in form, with `config.policy` and `result.source`
  admitting the new value.
- The **transcript**, rule 11. Written in live mode only.
- The **run record**, rule 15: the accounting a live run reports.

## State model

The source itself holds no state between decision opportunities. That is the whole content of `REQ-MOK-066`, and it is
a property of this specification rather than a discipline an implementation is asked to maintain: rule 2's request is a
value composed from the observation, and there is nowhere for a previous exchange to be kept.

Three things do accumulate over a live run, and all three are accounting rather than behaviour. None of them is read by
any rule that composes a request or interprets a response, so none of them can influence a decision:

| Accumulator | Grows by | Read by |
|---|---|---|
| Prompt, cached-prompt, output and reasoning token totals | Each exchange's reported usage | Rules 14 and 15 |
| Accumulated cost | Each exchange's usage times the declared unit prices | Rule 14's ceiling, rule 15 |
| Fallback count | Each occurrence of rule 9.5 | Rule 15 |

In replay mode a position in the transcript advances. It is a cursor over an input, in the same class as the tick
counter, and rule 12.3 fixes what happens when it and the engine disagree.

## Behavioral rules

### 1. The port

1.1 The engine obtains a proposal through **one** interface, which takes a decision request by value and returns
either a proposal or the fact that none was obtained, **together with the evidence of the exchange the proposal came
from** — the response as received, and the provider's four reported token counts. The interface names no provider, no
transport, no model, no credential, no file and no mode.

1.1a **The evidence returns with the proposal, as amended 2026-08-29 under `WO-MOK-026`, and the reason is that no
other route to it exists.** Rule 11.3 obliges an exchange record to carry "the response as received, in full, or the
error", the four reported counts, and "the action the response was parsed into, or the fact that it was not parsed
**and why**"; rule 11.1 puts the authoring of every record in the engine. The port is the engine's only contact with
what answered, so a port that returned the action alone would leave the engine obliged to write three things it could
not know — which is what `mokiterions-core/src/simulation.rs` had recorded since `WO-MOK-025` as "a pre-existing
tension between rule 1.1's port shape and rule 11.3's field list", and what rule 11.3.1 names this work order as the
place to resolve.

Two alternatives were measured and declined. Leaving this rule word for word and recording elsewhere that it already
admits the growth would leave a rule whose plain words and whose build disagree, reconciled in a different artifact.
A second port method the engine called after each proposal would leave the two calls' agreement a temporal contract
no type enforces, so a port returning the previous exchange's evidence would write a wrong record in silence.

1.1b **The same interface answers whether the run has stopped spending, and the engine asks before each exchange, as
amended 2026-08-29 under `WO-MOK-026`.** Rule 14.6 requires the check to precede the spending and rule 20.4.1 puts the
accumulated cost in the port, so the engine cannot make that check out of anything it holds: the figure the comparison
needs lives on the other side of the boundary. The interface therefore answers one further question — whether the
declared ceiling has been reached — and the engine asks it at **each decision opportunity, before the request is
composed**, so that a run declaring a ceiling issues no exchange after reaching it rather than one.

It is the same interface and not a second one, so rule 1.1's **one** is undisturbed: an interface is one door however
many questions are asked at it, and a second interface for the figure would let a host supply the spending and the
answers about it separately.

**It is not the second port method rule 1.1a declined, and the difference is temporal.** What 1.1a declined was a
method called *after* each proposal to fetch that proposal's evidence, whose correctness rested on two calls happening
in one order with nothing enforcing it, so a port answering about the previous exchange would write a wrong record in
silence. This question is asked **before** the exchange it decides and is about the run rather than about one exchange,
and a port answering from a figure one exchange out of date stops the run early and never late.

The answer is read from the port's own accumulated cost and from nothing else. **Asking must move no figure, write no
byte, read no line and say nothing to whatever is behind the port**, so a run's three streams are the same bytes
whether the question was asked once or at every opportunity — including at the opportunity where the answer is yes and
no exchange follows. A port with no declared ceiling answers that it has not stopped, for ever, which is how rule
14.8's *a replay has no ceiling* holds without a replay implementing anything: the question has an answer by default
and a replay takes it.

What the engine does when the answer is yes is fixed by rules 14.7 and 19.3 and by nothing here: it ends the run in an
orderly way and reports a status distinct from a clean completion and from an error.

1.1c **The proposal carries the exchanges the opportunity spent before it, as amended 2026-08-29 under `WO-MOK-026`.**
Rule 19.5 retries a failed exchange a bounded number of times, rule 11.2 gives each attempt "its own record, because it
was its own billed exchange", and rule 11.1 puts the authoring of every record in the engine. The attempts therefore
have to reach the engine, and rule 1.1's one interface is the only route: a port returning the last attempt alone would
leave the engine obliged to author records for exchanges it was never told happened.

**The retrying itself stays in the port, and that is forced rather than preferred.** The port bills each attempt under
rule 14.1 and holds rule 15.4's fallback count under rule 20.4.1, so a loop driven from the engine's side would need a
further act at the interface to tell the port that the attempts were exhausted — and without one the count would be
wrong for a retry that later succeeded.

What returns is therefore the proposal the opportunity ended on, carrying the earlier exchanges in the order they were
made, oldest first, and empty for the single-attempt case that is every exchange of every port but a retrying one. The
engine authors one record for each earlier attempt and then the outcome's, and **rule 11.3's fallback flag is marked on
the outcome's alone**: an attempt that was retried was not the opportunity's decision, so rule 15.4's count reconciles
against the records marked as fallbacks and `VER-MOK-018` case **P5** stays checkable. A run that retried three times
and then succeeded writes four records, marks none of them, and moves the fallback count by nothing.

Nothing about how an attempt was obtained crosses with it. Each is the same value rule 1.1a fixes — the response as
received and the four reported counts — so rule 1.3's values-only boundary and rule 1.4a's *the evidence is not an
answer* hold of them unchanged, and the engine still reads nothing but the outcome to reach a decision. **No earlier
attempt carries an action**, because an attempt that obtained one ended the retrying and would be the outcome.

Two alternatives were measured and declined. A named type for an attempt adds an item and four public declarations to
state what the proposal already states, and would need a rule fixing which of its fields the engine reads. A proposing
method returning a list of proposals declares nothing new at all, but leaves *which element is the outcome* an invariant
no type enforces — so a port returning them in the other order would have the engine author the outcome as an attempt
and mark no record as a fallback.

**One consequence is disclosed here and deliberately not repaired: a transcript holding a retried exchange is not a
replay input.** Rule 11.2's record per attempt and rule 12.3's one record per decision opportunity do not reconcile.
The first attempt's record matches the first opportunity and is consumed by it — marked `false` under the paragraph
above and carrying rule 9.5's `wait`, so no field rule 11.3 fixes distinguishes it from a recorded decision — and the
second opportunity then meets a record naming the first actor and fails rule 12.3's check. The refusal is loud and
specific, which is why this is a disclosure and not a stop: no replay invents a run. `mokiterions-core/tests/connector.rs`
and `mokiterions-core/src/simulation.rs` both characterise it, so it is pinned rather than left to be discovered by a
live run, and the consequence for this work order is that **a live run that retried cannot supply case `L30`'s replay
identity**. Repairing it means either a new field on the exchange record or rule 12.3 reading a group per opportunity,
both changes of substance in an approved rule, and neither is an implementation agent's to take.

1.1d **The same interface reports the run's accounting, once, after the run has ended, as amended 2026-08-29 under
`WO-MOK-026`.** Rule 15.1 makes the run record the one place every accounting figure this specification produces is
stated, rule 20.4.1 keeps every accumulator in the port, and rule 11.1 puts the authoring of every record in the engine.
Those three leave exactly one gap and this sub-rule closes it: the figures are held on one side of the boundary and the
record is written on the other, so the interface answers one further question — what this port's account is — and the
engine renders rule 15.2's line from the answer.

It is the same interface and not a second one, on rule 1.1b's ground word for word: an interface is one door however
many questions are asked at it. It is asked **once, after the run loop has returned**, which is what keeps rule 14's
*State model* literal — no accounting figure may influence a decision, and there is no decision left to influence.

**What crosses is a value the engine formats, and the repository owner decided that over two alternatives.** A port that
rendered rule 15's line itself would put this specification's own field set, spelling and escaping behind rule 1.1's
boundary, leaving the engine that rule 11.1 makes the author of every other record unable to state what its own run
record says. Reading the figures out one method at a time is rule 1.1a's declined shape a third time: a set of calls
whose mutual agreement is a temporal contract no type enforces. One value, taken once. Every figure in it is an integer
or an absence and **none is a floating-point value**, which is `VER-MOK-018` case **P6** and why rule 14.4's ratio
crosses as basis points; rule 15.3's zero and rule 11.5's absence are told apart by the type rather than by a
convention, so the two figures a run can genuinely lack are optional and the totals are not.

**The answer's absence is rule 15.6, and that is the whole of it.** A port that spent nothing reports no account, so a
replay reports no run record by *answering* rather than by a caller knowing what kind of port it holds — which rule 1.1
gives no way to ask and rule 1.2 forbids branching on. The question the engine ends up asking is "did anything spend",
not "is this a replay", and a deterministic source reaches the same absence from the other side, having no port at all.
This is what `mokiterions-core/src/simulation.rs` had recorded since `WO-MOK-025` as the blocker on rule 15: that the
engine "cannot tell a replaying port from a live one", so a record it wrote would have been an account of spending that
never happened. The blocker is removed by the shape of a return and not by a party's judgement.

**The model identifier and the reasoning level are the first reported and are never replaced.** Rule 15.2 obliges the
record to state both and rule 10.4c has the engine learn them from the response — "it learns which model answered from
the response and reports that" — so a run of many exchanges is told them once per answering exchange.
The port retains the pair from the first response that carried both with a non-empty identifier and ignores every later
one, which is what makes the two figures well defined for a run rather than a function of which exchange happened to be
last. A run whose provider never reported either states neither, as rule 11.5's absence. **Disclosed: a connector that
changed models mid-run is not detected**, and the record would name the first. That is a property this repository cannot
check — rule 10.7 declares the connector's output untrusted in whole and no rule obliges a provider to be consistent
about what it is — and detecting it means a rule fixing what a run whose model changed reports, which is not an
implementation agent's to write. The repository owner took this over the two alternatives on 2026-08-29: retaining the
last reported pair, which makes the figures depend on exchange order, and refusing the run on a change, which turns a
provider's own inconsistency into a failure of a run that has already been paid for.

**The destination of the rendered line is the host's and this rule names none**, exactly as rule 11.1 leaves the
transcript's destination to the host. Rule 15 fixes what the record says and rule 12.6 fixes what a replay must
reproduce byte for byte — standard output, the structured record stream and the exit code — and says in terms that it is
"not claimed for standard error". The recording host therefore places the line on **standard error**, which the
repository owner decided on 2026-08-29 over a sixth command-line option naming a path and over rule 8.9's structured
record stream. It is the one destination that amends no approved rule: a line a live run writes and a replay does not
breaks nothing on a stream rule 12.6 exempts, and rule 15.6 holds structurally there because a replay's port reports no
account to render.

**A conflict in `SPEC-MOK-006` rule 8.9 is recorded as a defect under this work order and is not repaired here.** That
rule puts a `live` object in the record stream's own run record, present for a connector-sourced run and absent from a
replay — which is a difference in the record stream's bytes between a recorded run and its replay, and rule 12.6 claims
those bytes identical. Both are approved rules and only one can hold. This rule follows rule 12.6, whose byte-identity
is `REQ-MOK-067`'s promise and `VER-MOK-018` case **L7**'s pass condition — a recorded run and a replay of it compared
with `cmp` on standard output and on the structured record stream — and rule 8.9's text is left word for word: an
implementation agent amending an approved rule on its own judgement is what `WO-MOK-026` stop-and-escalate condition 6
forbids. The defect record is in that work order's evidence directory with the measurement that found it. **The two run
records are different records and the spelling keeps them apart**: rule 8's is `record` on the structured stream and
rule 15's is `run_record` on standard error, so a reader of either stream can tell which obligation a line answers.

1.2 The engine holds no other means of obtaining a proposal under this source. There is no branch on live-versus-replay
anywhere in the library target, and no mode value reaches it. The difference between recording and replaying is
entirely a difference in what the host connected, which is what makes `REQ-MOK-067`'s byte-identity structural rather
than a second implementation to be kept in agreement.

1.3 The request crosses the boundary as **values only**. It contains no reference into engine state, no mutable borrow
and no handle. This is `ADR-MOK-001`'s and `SPEC-MOK-002` rule 6's existing trust boundary, adopted unchanged: what
crosses is a copy, so a source cannot reach what it was told about.

1.4 The proposal returned crosses back as a value of the engine's existing action type, or as the absence of one. A
port implementation cannot construct any other kind of answer, so nothing arriving through it can bypass rule 9's
validation by being expressed in some other form.

1.4a **The evidence rule 1.1a adds is not an answer and the engine decides nothing from it, as amended 2026-08-29
under `WO-MOK-026`.** Rule 1.4 is undisturbed: the action the engine applies is still the action type or its absence,
and the engine still reads nothing else to reach a decision. The response text is consumed by the transcript and by
nothing else; the four counts are consumed by rule 14's accounting and by nothing else. A port that reported a
response contradicting its own action changes what the record says about the exchange and changes no decision, which
is the honest outcome — rule 10.7 makes the connector untrusted in whole, and a transcript that recorded a response
the engine had silently overruled would hide exactly the disagreement a reader needs to see.

1.5 The four existing decision sources do not use this port and are not moved onto it. Rule 16 is the reason: any
refactoring of their call path risks the byte-identity `REQ-MOK-068` holds, and this initiative buys nothing by taking
that risk.

### 2. The decision request

2.1 A decision request is composed for exactly one decision opportunity: one tick, one living Mokiterion, one
observation.

2.2 It carries, and carries nothing else:

| Part | Content | Source |
|---|---|---|
| The shared rules | Rule 4's text | A constant of the run |
| The actor block | Rule 5's text | The observation's `agent_id` and `waste_tolerance` |
| The observation block | Rule 6's text | The observation |
| The permitted set | Rule 7's text | Rule 7.1's enumeration |

2.3 It carries no attribute of any other Mokiterion, no aggregate over the population, and no value derived from any
other request or response. `REQ-MOK-065` states this obligation and rule 6 discharges it by construction: the
observation block renders the observation's fields and no field of the observation carries another Mokiterion's
condition. `PerceivedMokiterion` carries an identifier, a direction and a distance and, in the engine's own words,
*"no attribute of the perceived Mokiterion — not its `health`, its `energy` or its `fear`."*

2.4 It carries no earlier request, no earlier response, no running summary, no provider-side conversation identifier
and no turn counter used as memory. `REQ-MOK-066` states this obligation and rule 2.2 discharges it: there is no part
in which such content could be placed.

2.5 The request is a **run input** in the sense rule 12 needs: composed from the observation alone, it is identical
across two runs of the same seed, tick limit, density and tracing selection. That is what lets rule 12.3 detect a
transcript from a different configuration.

### 3. The prompt layout and its cache order

3.1 The request's four parts appear in exactly this order, and the order is not an implementation choice:

```
+-----------------------------------------------+
| A  shared rules        ~1,200 tokens   cached |
+-----------------------------------------------+
| B  actor block            ~30 tokens   cached |
+-----------------------------------------------+
| C  observation block     ~200 tokens variable |
+-----------------------------------------------+
| D  permitted set                     variable |
+-----------------------------------------------+
```

3.2 The reason is that the provider's prompt cache matches the **longest identical leading span** of a request against
a recent one. Block A is byte-identical across every request of a run, so it is a shared prefix for all of them. A and
B together are byte-identical across every request for one Mokiterion, so they are a shared prefix for that
Mokiterion's whole run. C and D are the only parts that vary, and they sit last where varying costs nothing.

3.3 Block A is byte-identical across every request of a run, including across Mokiterions and across ticks. Any
variation inside it — a name, a tick, a count, a whitespace difference — destroys the shared prefix for every request
of the run. This is a conformance condition and not a preference, and rule 14.4 is where it is measured.

3.4 Blocks A and B contain no value that changes within a run. `waste_tolerance` is in block B because it is a trait
constant; `health`, `satiety`, `energy` and `fear` are in block C because they are not.

3.5 The cacheable prefix is **estimated** at 1,230 tokens of an **estimated** 1,430, which is 86 percent, against
`REQ-MOK-070`'s floor of 85. Placing block C first would report a ratio near zero at ten times the price for the same
information.

3.6 Rules 3.1 through 3.4 are a specification of the request's bytes. They are not satisfied by an implementation that
composes the parts in this order and then serialises them through a structure whose field order is not guaranteed.

### 4. Block A — the shared rules

4.1 Block A states the world's rules in prose: what a Mokiterion is, what its attributes mean and their ranges, what
the four core verbs and the seven targeted verbs do, what a tick is, what perception is and its radius, how a proposal
may be rejected, and that exactly one action is to be chosen.

4.2 Its content is derived from `SPEC-MOK-001` and is a restatement for a reader, not a second authority. Where the two
disagree, `SPEC-MOK-001` governs and block A is wrong and is corrected.

4.3 It states the ranges as the engine holds them: `health`, `satiety`, `energy` and `fear` are integers from 0 to 100;
`waste_tolerance` is an integer from 0 to 40; perception reaches 16 units.

4.4 It contains **no strategy, no goal, no preference and no advice.** It does not say that survival is desirable, that
health should be kept high, that combat is risky, that cooperation pays, or that any action is better than any other in
any circumstance. `INT-MOK-011` sets no viability floor for this source, and a block A that told the model to survive
would be measuring the instruction rather than the model.

4.5 It contains no Mokiterion's identity, no tick, no seed and no count of anything that varies. Rule 3.3 requires
this; rule 4.5 states it as a content rule so that it is checked when block A is edited rather than only when a ratio
regresses.

4.6 It states the response grammar rule 8 fixes, so that a response can be well-formed from block A alone.

4.7 Block A's text is a constant of the source, held in one place, and its bytes are covered by the transcript: a
retained transcript's first request contains it in full, so a later reader can see which rules text produced a
measurement without consulting the source tree.

### 5. Block B — the actor block

5.1 Block B names the acting Mokiterion by its identifier and states its `waste_tolerance`.

5.2 It states nothing else. In particular it states no other Mokiterion, no history and no attribute that varies.

5.3 It is byte-identical across every request for that Mokiterion in a run.

### 6. Block C — the observation block

6.1 Block C renders the observation's varying fields, in a fixed order: the tick; the position and its territory;
`health`, `satiety`, `energy` and `fear`; the attacks suffered since the previous opportunity, each as an attacker
identifier and a damage figure, in the order they resolved; the identifiers of co-located food; each perceived food
resource as an identifier, a class, a relative direction and a distance; and each perceived living Mokiterion as an
identifier, a relative direction and a distance.

6.2 A perceived Mokiterion renders exactly the three values the observation carries. It renders no attribute of the
perceived Mokiterion, and no such attribute is available to render.

6.3 An absent relative direction — the co-located case — renders as a stated word, not as an omission and not as a
sentinel value. `SPEC-MOK-006` rule 4.4's principle is adopted: an absence is stated as an absence.

6.4 The attacks suffered are the engine's own one-tick memory and are rendered as part of the observation. They are not
retained context, and rule 2.4 is not weakened by them. An attacker's identifier renders; nothing about an attacker's
condition renders, because nothing about it is carried.

6.5 An empty list renders as a stated emptiness rather than as a missing line, for rule 6.3's reason.

6.6 Block C contains no aggregate: no count of living Mokiterions, no mean of anything, no ranking, no nearest-neighbour
summary beyond the per-entry distances the observation carries. `REQ-MOK-059` already forbids the engine to read a
population-level aggregate; rule 6.6 forbids composing one from what it may read.

### 7. Block D — the permitted set

7.1 Block D enumerates every action the specification permits this Mokiterion to propose at this opportunity, with each
targeted action named against each target it may name.

7.2 The enumeration is **not** the observation's list of currently valid core proposals. That list carries the core
proposals and never a targeted action, and `SPEC-MOK-001` rule 3 states both the fact and the reason: *"Rule 4's
baseline consumes one entropy selection over the length of this list, so a longer list moves that selection, and every
run ever recorded under `baseline` would diverge."* The same rule warns that *"a reader who takes this list as the whole
contract will be wrong about the social source"*, and such a reader would be wrong about this source too.

7.3 The enumeration is composed from the authority `SPEC-MOK-001` rule 6 gives, which is *"the complete statement of
what may be proposed"*: the core verbs as the observation carries them, `eat` against each co-located food identifier,
`move` against each valid cardinal direction, and each of `SPEC-MOK-001` rule 21's seven targeted verbs — `attack`,
`threaten`, `fight`, `retreat`, `surrender`, `approach`, `avoid` — against each perceived Mokiterion identifier whose
precondition that verb satisfies at this opportunity.

7.4 A verb whose preconditions no target satisfies is not enumerated. Block D never offers an action the engine would
reject on a ground block D could have known about.

7.5 An action the engine may still reject on a ground block D could not know about — a move into a cell occupied by
something the observation does not carry, say — **is** enumerated. That rejection is an ordinary rejected proposal,
resolved as it is for every other source, and rule 9.6 keeps it out of the fallback count. It is part of what a
measurement measures.

7.6 Block D and block A together are sufficient: a well-formed response can be produced from the request alone, with no
knowledge the request does not contain.

7.7 Block D's order is fixed and derived from the observation's order, so that two runs of the same configuration
compose identical requests, as rule 2.5 requires.

### 8. The response and its grammar

8.1 A response names exactly one action: a verb, and where the verb is targeted or parameterised, exactly one
identifier or direction.

8.2 The grammar is closed. A response is well-formed only if its verb is one of the eleven and its parameter is one
block D enumerated for that verb.

8.3 The response carries no prose, no explanation, no confidence and no alternative. A field for a reason is not
provided, because a reason nothing consumes is output tokens spent to no effect and a second thing a later reader might
mistake for evidence about the decision.

8.4 The response is requested through the provider's structured-output facility, so that well-formedness is the
provider's obligation as well as this system's check. That facility is documented for `gpt-5.6-luna`.

8.5 The reasoning level requested is `none`, on the repository owner's decision of 2026-08-23. Rule 15.2's reasoning
token count is where a run shows that it got what it asked for.

8.6 The response is not trusted because it is well-formed. Rule 9 validates it, and rule 9.3 sends it through the same
validation every other source's proposal passes.

### 9. Parsing, rejection and the fallback

9.1 A response is parsed into the engine's action type, or it is not parsed.

9.2 A response fails to parse when it is malformed, when its verb is not one of the eleven, when its parameter is not
one block D enumerated for that verb, or when a targeted verb names no target.

9.3 A parsed proposal is validated and resolved by **the same rules every other decision source's proposal passes**,
with no exemption, no relaxation and no separate path. `REQ-MOK-063` states this. A proposal from this source is not
privileged by having come from a model.

9.4 An exchange yields no response when the transport fails after the run's retries, when the provider returns an
error, or when the provider returns nothing.

9.5 When a response fails to parse or an exchange yields no response, the source proposes **`wait`** — the least
consequential action, available at every opportunity — and the occurrence is counted and recorded. `wait` is the
fallback at every opportunity, so a run's contamination is one identifiable thing.

9.6 A proposal the engine's rules then reject is **not** a fallback and is not counted. It is an ordinary rejected
proposal. Rule 9.6 and rule 7.5 are the same distinction stated from the two sides: rule 9.5 counts a source that did
not answer, not a source whose answer the world refused.

9.7 The fallback is never a proposal composed by another decision source. Substituting `baseline`'s selection would
make the run a mixture of two sources under one label, and `REQ-MOK-074` states why that is worse than a counted
substitution.

9.8 A run whose fallback count exceeds zero is marked in its run record as unfit to source a published figure, rule
15.4. The run itself is not aborted: its transcript replays and its ticks are real, and an abort would make one
transport hiccup cost an **estimated** $1.04 and hours of wall time.

### 10. The connector binding

10.1 A live run reaches the provider through a **connector**: an executable the operator names by path as a host option.
The host spawns it as a child process and exchanges with it over that child's standard input and standard output.
**Neither Rust package acquires a crate.** Spawning a child and reading its lines is standard-library work in both, so
`REQ-MOK-050`, `ARCH-MOK-001`'s conformance check, `SPEC-MOK-002` rule 13, `SPEC-MOK-003`'s declared dependency set and
`SPEC-MOK-004` rules 1 and 2 are untouched, and this specification requires no amendment to any of them. That is the
binding's principal merit and the reason it was chosen over the two `ADR-MOK-007` records as rejected.

10.2 The framing is one JSON object per line in each direction: one request object per line to the child's standard
input, one response object per line from its standard output, in the same order. Lines are newline-terminated and
contain no newline within an object.

10.3 A request object carries the prompt text rules 3 through 7 compose, the model identifier, the reasoning level and
the response schema rule 8.4 needs. It carries no credential.

10.3a **The request object's field names, as amended 2026-08-28 under `WO-MOK-028` and again 2026-08-29 under
`WO-MOK-030`.** `protocol`, an integer protocol version, `1` today; `tick`, the simulation tick; `actor`, the
Mokiterion's identifier; `prompt`, the whole composed prompt as one string; and `schema`, the response schema rule
8.4 needs. **`model` and `reasoning` are not request fields**: rule 10.4c places the provider binding in the
connector, so the engine has nothing to put in them. Every field is
present on every request. The names are fixed here because rules 10.3 and 10.4 fix what each object *carries* and not
what its keys are called, and a connector author writing against an unnamed contract has nothing to write against:
two independent connectors would not interoperate, and neither would be wrong.

10.4 A response object carries the action, the reported usage counts, and either success or an error. The usage counts
are the provider's own figures as the connector reports them; no component recomputes or adjusts them.

10.4a **The response object's field names, as amended 2026-08-28 under `WO-MOK-028` and again 2026-08-29 under
`WO-MOK-030`.** `protocol`; then exactly one of `action` or `error`. `action` carries `verb` and, for a verb that
takes one, `parameter`. `usage` carries `prompt`, `cached_prompt`, `output` and `reasoning`, and accompanies
`action`. `error` carries `kind` and `message`, and rule 19.5a fixes the vocabulary of `kind`. **`model` and
`reasoning` accompany `action` too**, naming what actually answered.

10.4c **The provider binding is the connector's, and rule 15.2 reports what answered rather than what was asked
for.** The model identifier, the reasoning level and the endpoint are declared in the connector, which is what
`WO-MOK-026` item 5 fixes and what keeps this engine free of a provider binding of any kind. The engine therefore
**cannot** send a model identifier and does not try; it learns which model answered from the response and reports
that.

Until 2026-08-29 rule 10.3 had the request carry both values while item 5 declared them in the connector, and
rule 15.2 required the engine to report them. The three could not all hold. Resolving it the other way — the
engine told, the connector obeying — would have put a model identifier and an endpoint's worth of knowledge back
into a component `ADR-MOK-007` keeps free of them, and resolving it by leaving the request's fields advisory would
let a published figure name a model that did not answer. **A run record that names the wrong model is worse than
one that names none**, so the report follows the answer.

A connector that reports neither is answering incompletely, and rule 10.7 applies: the response fails the grammar
check and becomes a counted fallback under rule 9.5, exactly as a missing action would.

10.4b **The names of 10.3a and 10.4a are those `docs/CONNECTOR_PROTOCOL.md` documents**, recorded as they already are
rather than improved. That document was published under `WO-MOK-026` before this amendment and a connector author may
have read it, so a name changed here would break a contract this repository has already published. The document is repository-owned
and authorizes nothing; these two rules are what make the names normative.

10.5 **The connector reads the credential from its own process environment, and no other component reads it at all.**
Neither host reads it, no command-line option carries it, and it appears in no request object. A host may pass its own
environment through to the child, which is how the credential reaches the connector without any component in this
repository naming it. The connector never writes it to its standard output, its standard error or any file.

10.6 **This specification does not constrain the connector's dependency surface, and cannot.** The connector is an
operator-supplied executable that need not be in this repository, so no check here can reach it. This is stated as a
rule because it is a deliberate limit and not an oversight: the earlier draft of this specification constrained a
repository-owned provider program to its language's standard library, and that constraint is **withdrawn** along with
the program it applied to. What this repository does own and does constrain is the **canned connector** of rule 20.5,
which exists for offline verification. `VER-MOK-018`'s `S2` checks that one and states plainly that it can check no
other.

10.7 **The connector's output is untrusted in whole.** A response passes rule 8's grammar check and then `SPEC-MOK-001`
rule 6's validation, unchanged, exactly as a local source's proposal does. `ADR-MOK-001`'s *"Model output is untrusted
input and must pass the same validation as the local baseline"* is read here as reaching the connector's entire output
and not only the model's action text — the usage counts and the success flag included, because a connector is a program
the operator supplied and not a component this repository verified.

10.8 **The ceiling of rule 14 protects against an honest connector, not a dishonest one.** Cost is computed from the
usage counts rule 10.4 passes through, so a connector that under-reports usage spends past the ceiling and the run
cannot tell. This is recorded as a limit rather than defended against: the operator writes the connector, and a
containment that assumes otherwise would be theatre. Rule 13's two gates and the absence of any credential in automation
are the containment that does not depend on the connector behaving.

10.9 **The engine's library target does not know rule 10 exists.** Everything in it is on a host's side of rule 1.1's
interface. The library resolves no path, spawns no process, opens no file and reads no environment variable, exactly as
`SPEC-MOK-001` and the library's own documented guarantee already require. **The connector path never enters the
configuration value the library holds**: a host parses it, spawns the child, and hands the library a connected port and
nothing else. Rule 20.4 is why that matters beyond tidiness.

### 11. The transcript

11.1 A live run writes a transcript. The engine authors every record; the host owns the destination and hands the
engine an already-open stream, on `SPEC-MOK-006` rule 1.2's precedent — the engine resolves no path and opens no file.

11.2 The framing is one record per line, in the order the run made them, and there are **two record kinds**. An
**exchange** record is one line per exchange, and a retry is its own record, because it was its own billed exchange. A
**prefix** record states blocks A and B for one Mokiterion, once, before that Mokiterion's first exchange. Rule 3.4 is
what makes the split sound — neither of those two blocks varies within a run for a given Mokiterion — and rule 11.7's
figures are why it was chosen. An exchange record therefore carries blocks C and D and **names** its prefix rather than
repeating it, and this costs a reader nothing: the request as sent is the named prefix's blocks followed by the exchange
record's own, concatenated in rule 3.1's order. Nothing is abbreviated, so rule 11.7's first sentence still holds.

11.3 An **exchange** record carries: the tick and the acting Mokiterion, so the exchange is bound to its opportunity;
the request as sent, in full, as the prefix it names together with this exchange's own blocks; the response as received,
in full, or the error; the provider's reported prompt, cached-prompt, output and reasoning token counts; the action the
response was parsed into, or the fact that it was not parsed and why; and whether the decision was rule 9.5's fallback,
which rule 15.4's count is reconciled against. A **prefix** record carries the Mokiterion it belongs to, that
Mokiterion's two blocks, and the digest rule 11.3.2 fixes.

11.3.1 **The response and the usage figures are present and empty until a provider is called.** No exchange this
repository has recorded came from a provider, so both are written as the format's empty value in every record that
exists; they are present rather than omitted so that the format does not change when the first live run writes into
them. Rule 11.5 is how they are read while empty: an empty response is not a response, and an empty usage is four
**absent** counts rather than four zeros. `WO-MOK-026` is where either first carries a value.

11.3.2 **The prefix reference carries a digest of the prefix it names, and the digest is FNV-1a 64.** Rule 12.3's
mismatch check extends to it, so an edit to block A invalidates every transcript taken before the edit, loudly, at the
first exchange, rather than replaying against a prefix the recorded run was never composed with. **It is not
cryptographic and is not required to be.** Its job is drift detection between a prefix and the run that replays against
it, over bytes that sit in the same file as the digest: an adversary who can edit the prefix can edit the digest beside
it, and no digest of any strength changes that. The alternatives were a crate, which `SPEC-MOK-006` rule 12.4 forbids,
and a hand-written cryptographic digest, which would be a second thing to verify for no property this needs.

11.3.3 **The action's second field is `parameter`, not `target`.** Rule 8.1 admits an identifier **or a direction** and
rule 8.2 already calls it a parameter; a field named `target` would be false of every `move`, which names a direction
and targets nothing.

11.4 `SPEC-MOK-006`'s constraints are adopted for the transcript: no floating-point value, no timestamp, no path, and
bytes comparable between runs. A transcript is diffable evidence, comparable with `cmp`, rather than a log.

11.4.1 **The closed alphabet is not among them, and cannot be.** The record stream's alphabet is `A-Z a-z 0-9 _ . - + :
; >`, and blocks A to D are English prose: block A alone carries 1,282 spaces, 44 commas, 9 less-than signs, 5
apostrophes and 2 em dashes, none of which is in that alphabet, and all four blocks are multi-line. **This list is
measured over the committed transcript's block A and not inferred from what prose usually contains**, because two
characters that inference suggests are in fact inside the alphabet: a full stop is, and block A carries 65 of them, and
so is `>` — while `<` is not. `SPEC-MOK-006` rule 3.4 names this exact branch — a value outside the enumeration
"must either be added to that enumeration or arrive together with an escaping function and its own verification". The
transcript takes the second of those, and this rule names the function: **`escape_transcript_text`**, which escapes the
framing characters and nothing else, and whose inverse the replay applies before a block is used. **The obligation is a
round trip rather than an alphabet**: every block survives escaping and unescaping byte-identically, which is what
carries block C **verbatim** and lets a reader compare what was sent rather than a rendering of it. The unescaping is
not generous — a sequence the function never writes is refused rather than interpreted, so a hand-edited transcript
fails instead of replaying as something the escaper could not have produced.

11.5 A reported count that the provider did not report is recorded as **absent**, not as zero. A missing count and a
count of zero mean different things, and rule 14.5 depends on telling them apart.

11.6 A transcript contains no credential, no authorization header and no provider account identifier. It is retained
inside the repository, and `REPOSITORY_CONTEXT.md` requires credentials to remain outside it.

11.7 A transcript is never truncated or abbreviated to fit a size budget. Its size is bounded by the horizon chosen, and
**the figures are measured rather than estimated**, against the transcripts this repository commits. There are **three**,
and rule 11.3.1 is why the first cannot stand for the other two: the first was written with nothing behind the port and
the other two with a provider behind it. The first is
`mokiterions-core/tests/transcript-seed0-ticks20-hunting.jsonl`, a 20-tick run at seed 0 and density 0.75 in which all
twelve Mokiterions act. It is **305,568 bytes**: twelve prefix records totalling **67,447**, of which block A is
**5,385** bytes each time, and 221 exchange records totalling **238,121**, a mean exchange record of **1,078** bytes.
Extrapolating that mean at the same exchanges per tick, a 1,000-tick run is an **estimated** 12 MB. The second is
rule 11.7.2's, and it is what confirms that extrapolation; the third is rule 11.7.3's, which confirms it again at a
different exchange rate. What is retained where is `VER-MOK-018`'s.

11.7.1 **The estimates this rule carried are recorded rather than deleted, because the measurement exceeded both.** It
read "an **estimated** 4.7 MB for a 1,000-tick run, an **estimated** 100 to 260 KB for a 20-to-50-tick run". The
committed transcript's 305,568 bytes is 298 KiB at **20** ticks, already above the top of a band that was meant to reach
50, and the 1,000-tick figure is about two and a half times the 4.7 MB. The estimate was low because blocks C and D are
larger than it assumed, and **not** because anything is written twice: rule 11.2's split already removes every
repetition there is to remove. A record that carried blocks A and B inline as well would average **6,502** bytes rather
than 1,078, those two blocks being 5,424 or 5,425 bytes at every exchange of a given Mokiterion. **No size ceiling is
added**, and this rule's first sentence is the reason — a ceiling on a transcript is an
instruction to abbreviate one, which is the single thing this rule forbids.

11.7.2 **The second committed transcript is the live run's, and it is the first estimate in this specification a
measurement has confirmed.** `docs/engineering/simulation/evidence/WO-MOK-026/live-run-transcript.jsonl` is a 50-tick
run at seed 0 and density 0.75 against `gpt-5.6-luna` at reasoning level `none`, and it is the first transcript here
whose response and four usage counts carry values — rule 11.3.1 names `WO-MOK-026` as where either first does, and this
is that file. It is **700,192 bytes**: twelve prefix records totalling **67,447**, of which block A is **5,385** bytes
each time, and 503 exchange records totalling **632,745**, a mean exchange record of **1,258** bytes. The figure is
reproducible because the file is committed under `.gitattributes`' `-text` rule for the evidence tree, so those are the
bytes as captured, and it contains **no carriage return**. **The prefix figures are identical to rule 11.7's, to the
byte**, which is stated rather than passed over: blocks A and B do not vary within a run and do not depend on what is
behind the port, so two runs at the same seed and density render them the same, and each transcript's prefix therefore
checks the other's. **The mean exchange record grew by 180 bytes, and the populated fields account for more than that**:
the span from `response` to `fallback` has a mean of **326** bytes here against **75** in rule 11.7's transcript, a
difference of **251**. The remaining **−70** is *not* attributed to rule 11.3.1 and no claim is made about it — the two
runs share a seed but not their decisions, so their blocks C and D describe different worlds and are not comparable.
Only the prefix is, which is why only the prefix is compared. **The 12 MB is confirmed at 12.7 MB**: 10.06 exchanges per
tick at a mean of 1,258 bytes extrapolates to **12,722,347** bytes at 1,000 ticks. Rule 11.7.1's superseded band was
already exceeded at 20 ticks, so this is the first size figure in this rule that survived contact with a larger run, and
**no size ceiling is added here either**, for rule 11.7.1's reason.

11.7.3 **The third committed transcript is the rejected first attempt at rule 11.7.2's run, and it is the only file here
whose reasoning count is not zero.** `docs/engineering/simulation/evidence/WO-MOK-026/attempt-1/live-run-transcript.jsonl`
is a 50-tick run at seed 0 and density 0.75 against `gpt-5.6-luna`, recorded on 2026-08-29 before rule 11.7.2's file and
rejected because the provider returned 76,350 reasoning tokens at reasoning level `none`. **It is retained because rule
14.2a was measured from it**: across all 567 of its exchange records `output` minus `reasoning` lies between 18 and 26 —
the size of one action object — and no other transcript here carries an exchange with a reasoning count at all. A rule
measured from a file that is not kept is a rule no later reader can re-derive. It is **759,168 bytes**: twelve prefix
records totalling **67,447**, of which block A is **5,385** bytes each time, and 567 exchange records totalling
**691,721**, a mean exchange record of **1,220** bytes. It contains **no carriage return** and is committed under the same
`-text` rule. **The three transcripts' prefixes are not merely the same size but the same bytes**, which a third file is
what made checkable: one digest over each file's twelve concatenated prefix records agrees across all three. Rule 11.7.2
recorded the totals as identical to the byte and inferred that blocks A and B do not depend on what is behind the port;
the digest says more, and says it of the horizon too — 20 ticks against 50 — so those two blocks depend on the seed and
the density and on nothing else. **The mean exchange record is 38 bytes *smaller* than rule 11.7.2's while carrying the
same populated fields**, and that is recorded as **not attributed**, for rule 11.7.2's reason: the span from `response` to
`fallback` has a mean of **325** bytes here against **326** there, so the populated fields are the same size and the
difference is in blocks C and D, two runs at one seed taking different decisions and describing different worlds. What
the pair does establish is that the populated span is **stable at about 325 bytes across both live runs** against **75**
in rule 11.7's transcript, which is the only regularity two runs' exchange records share. **The extrapolation is
confirmed a second time and at a different exchange rate**: 11.34 exchanges per tick here against 10.06, and a 1,000-tick
run extrapolates to **13,901,867** bytes against rule 11.7.2's 12,722,347 — a 9 % difference, which is the 13 % higher
exchange rate partly offset by the smaller mean record, and both figures are above rule 11.7's estimated 12 MB. **No size
ceiling is added here either**, for rule 11.7.1's reason.

11.8 A replay writes no transcript. It has one; it is reading it.

### 12. Replay

12.1 A replay obtains each decision from the transcript, in order, through the same port rule 1.1 defines and the same
code path a live run uses.

12.1.1 **The host opens the transcript and lends the engine an already-open reader**, which is rule 11.1 mirrored and
holds for the same reason: the library resolves no path and performs no filesystem operation. The transcript path no more
enters the configuration value the library holds than the connector path does, per rules 10.9 and 18.4. This is the one
place where the two hosts do the same work, so the reader-backed port is the engine library's own item and each host
supplies only the open stream — otherwise the replay reader would be written twice, once per host, and the two copies
would drift.

12.2 A replay makes no provider call, opens no socket, spawns no connector and reads no credential. This holds whether
or not a credential is present in the environment, and it holds in both hosts.

12.3 Before using a record, the replay checks that the record's tick and acting Mokiterion match the opportunity the
engine has reached. On a mismatch the replay **fails**, names the mismatch, and produces no further ticks. A transcript
from a different seed, density or horizon is detected here rather than producing a plausible wrong run.

12.4 When the transcript ends before the run does, the replay fails and names the opportunity it could not satisfy. It
does not shorten the run, does not apply rule 9.5's fallback, and does not substitute a rule-based proposal.

12.5 When the transcript is longer than the run needs, the surplus is unread and the run is unaffected. A run that
ended early through extinction leaves a tail; that is not an error.

12.6 A replay of a matched configuration produces standard output bytes, structured record stream bytes and an exit code
identical to the recorded run's. Byte-identity is claimed for the matched configuration, which includes the tracing
selection, and is not claimed for standard error.

12.7 A record whose exchange failed replays as rule 9.5's fallback, with the count incremented as it was in the
recorded run. A replay reproduces the run that happened, contamination included.

### 13. Live mode, the credential and automation

13.1 Replay is the default. A live run happens only when **both** an explicit live-mode selection was made, **and** a
provider credential is present in the connector's environment. `REQ-MOK-072` states this, and rule 13.1 is its whole
mechanism. **The two conditions are checked in two different components** — the selection by the host, the credential by
the connector — and neither component can satisfy the other's condition. That separation is a consequence of rule 10.5
and it strengthens the gate rather than complicating it: no single component can authorise spending.

13.2 When the live-mode selection is absent, the run replays if a transcript was supplied and otherwise refuses with the
usage-error status, rule 19.2. **No connector is spawned at all in this case**, so a present credential is not merely
"never taken as consent" — under this binding no component in the run is even in a position to observe that it is present.

13.3 When the credential is absent, empty or malformed, **the connector makes no provider call** and returns an error on
the first exchange. The run reports which condition was missing, in the terms the connector reported it, and names no
value. The credential is looked for in the connector's process environment and nowhere else: no file, no keychain and no
configuration directory is searched. The refusal therefore arrives after the connector was spawned and before any network
was reached, which costs nothing and calls nothing.

13.4 **Neither host reads the credential, and the library target reaches it by no route**, per rule 10.5. It is never
written to a tracked file, never printed to either output stream, never placed in a request record and never placed in an
error message. A host that passes its own environment through to the child transmits the value without observing it, and
that is the whole of how the credential travels.

13.5 A live run also requires a declared spend ceiling, rule 14.6. A live run with no ceiling is refused before the
first exchange rather than run unbounded.

13.6 No automated workflow in this repository makes a provider call, and no workflow file references a model-provider
credential — not as a secret, not as an environment variable, not as an input, and not through a step that fetches one.
A repository check reads the workflow definitions and fails the build on such a reference. `REQ-MOK-073` states this,
and the containment it rests on is that the credential is not present in the repository's automation secrets at all.

13.7 Automation exercises this source in replay mode against a transcript committed to the repository. Rule 13.6
forbids spending, not testing.

### 14. Usage, cost and the ceiling

14.1 After each exchange the run adds the provider's reported prompt, cached-prompt, output and reasoning token counts
to four run totals, and adds that exchange's cost to an accumulated cost.

14.2 Cost is computed from the reported counts and the unit prices declared for the run, as integer arithmetic in a
stated minor unit. **The minor unit is the US cent, as amended 2026-08-29 under `WO-MOK-030`.** Every cost figure
this specification states is written in dollars, so the currency was implied throughout and normative nowhere,
which left both `--spend-ceiling`'s amount and the run record's cost integer denominated in nothing. `SPEC-MOK-006`'s prohibition on floating-point values in a stream holds for the run record, so the
figure reported is an integer in a stated unit rather than a formatted decimal whose bytes vary by platform.

14.2a **The reported output count is inclusive of the reported reasoning count, so the billable output is the
difference, as amended 2026-08-29 under `WO-MOK-026`.** An exchange's cost prices `output − reasoning` at the output
price and `reasoning` at the reasoning price, with the reasoning count clamped to the output count before the
subtraction. Rule 14.1's four run totals are unaffected and stay exactly as the provider reported them, so a reader
recomputing a cost from those totals must make the same subtraction; the totals record what was reported and this rule
decides what is billed.

Until this amendment rule 14.2 said only that cost is computed "from the reported counts", which read naturally as
four disjoint quantities and was implemented as four. **The provider's output count is not disjoint from its reasoning
count**: it is a total that contains it, so pricing both in full bills every reasoning token twice. This was measured
rather than reasoned about, on the first live run at a non-zero reasoning level: across all 567 exchanges of
`WO-MOK-026`'s rejected first attempt, reported output minus reported reasoning was between 18 and 26 with a mean of
24.1 — the size of one action object, on every exchange — and the true cost was **28.21 cents** against the **37.37**
the engine computed. **This rule already had the convention right for the other pair and only for the other pair.**
Rule 14.4's ratio is cached prompt over total prompt, and cost has always been `prompt − cached` at the uncached price
plus `cached` at the cached price, because the prompt count likewise contains the cached count. The two pairs have the
same shape; one was handled and one was not. The clamp is there for the same reason the cached count is clamped to the
prompt count under rule 10.7 — a provider's figures are untrusted input, and a reasoning count exceeding the output
count would otherwise underflow the billable output into an enormous quantity.

**The fault was latent and is not retroactive.** It is reachable only when a run reports reasoning tokens, and the
accepted run of `WO-MOK-026` reports none, so no retained cost figure, run record or replay in this repository moves
with this amendment. A work order raising the reasoning level above `none` would have met it on its first paid
exchange.

14.3 The declared unit prices are inputs of the run, not compiled-in constants. The provider's prices are the
provider's to change.

14.3a **They arrive through `--prices`, as amended 2026-08-29 under `WO-MOK-030`.** Four integers in **cents per
million tokens**, colon-separated in the order prompt, cached, output, reasoning: `--prices 125:13:1000:0`. The
shared parser validates it and **retains** the four values, like `--spend-ceiling` and unlike the paths, because
the run computes with them; `VER-MOK-018` case `S6a` scopes the discard rule to paths and this is not one.

Until this amendment rule 14.3 called the prices inputs of the run and no input carried them, which left a
compiled-in constant — the one thing the rule forbids — as the only thing an implementation could do. An
option rather than a file, because a file needs a format, a grammar, error cases and a test tier that no rule
gives it, and because the price list is then visible in the command that produced a run, which is what a later
reader recomputing a published cost figure needs. Integers rather than a decimal, so rule 14.2's arithmetic is
integer arithmetic from the input onward and no rounding enters at the edge.

14.4 The cache ratio is cached prompt tokens divided by total prompt tokens, over the whole run, from the reported
figures and never from a local token estimate. A local estimate would let an implementation pass while paying full
price.

14.5 The ratio is held at 0.85 or above for a run of at least 200 exchanges. Below that count it is reported and not
held, because one uncached first prefix is a large share of a small denominator. When the provider reported no
cached-token figure the ratio cannot be computed, and that is a failure to evaluate rather than a pass.

14.6 Before issuing an exchange, the run stops if the accumulated cost has reached the declared ceiling. The check is
made **before** spending, so the ceiling bounds the run rather than being overshot by one call.

14.7 A run stopped at its ceiling ends in an orderly way: the transcript and the record stream are complete and
readable to the tick reached, and rule 19.3's status distinguishes the stop from a clean completion and from an error.

14.8 Rules 14.1 through 14.7 apply to live runs. A replay spends nothing, computes no ratio and has no ceiling.

### 15. The run record

15.1 A live run reports a run record. It is where every accounting figure this specification produces is stated, so that
a reader has one place to look and a later reader can recompute each figure from the transcript.

15.2 It carries: the four token totals; the cache ratio; the accumulated cost and the declared ceiling; the fallback
count; the tick reached; the seed, tick limit, density and tracing selection; the model identifier and the reasoning
level; and how the run ended.

15.3 A zero is reported as zero. A fallback count of zero and a reasoning-token total of zero are stated positively, so
that a clean run says it is clean rather than being inferred from a silence.

15.4 When the fallback count exceeds zero the record marks the run as unfit to source a published figure. The mark is a
property of the record, not of a summary written afterwards.

15.5 When the run stopped at its ceiling the record says so and states the tick reached, so that a figure is never
quoted at a horizon the run did not reach.

15.6 A replay reports no run record. The recorded run's record is the accounting of the spending that happened, and a
replay spends nothing; writing a second one would create two accounts of one event.

### 16. Non-perturbation

16.1 A run under `baseline`, `reference`, `individual` or `social` produces the same standard output bytes, the same
structured record stream bytes, the same per-tick entropy draw counts and the same exit code as before this source
existed. `REQ-MOK-068` states this and `INT-MOK-010` carries the promise for `baseline` specifically.

16.2 The entropy stream is not touched. This source draws nothing from it: a decision arrives from the port, and no
selection over any list is made on the engine's side. Rule 16.2 is what makes rule 16.1 achievable rather than
laborious.

16.3 The observation's list of currently valid core proposals does not change length, gain a member or change order.
Rule 7.2 is the reason this is possible: block D is composed beside that list rather than by extending it.

16.4 An observed run and an unobserved run remain byte-identical, as `ADR-MOK-006`'s validation list already requires.

16.5 Rules 16.1 through 16.4 are verified by comparing retained captures on both sides of the change, at a stated base
commit and a stated candidate commit, over every source and the declared seed set. No configuration is excluded on the
ground that the change cannot affect it.

### 17. The authorization record

17.1 A live run's retained evidence includes an authorization record naming the authorizing owner, the date of the
authorization, the horizon authorised, the seed set authorised, and the spend ceiling authorised in a stated currency
and unit. `REQ-MOK-076` states this.

17.2 No code consults it. It is an accountability artifact read by a person, and its verification method is static
analysis over retained evidence, because nothing observable at run time can establish that permission was given.

17.3 One record may cover several runs when it names the seed set and horizon they all fall within. A measurement over
five seeds is one authorised act.

17.4 It contains no credential and no provider account identifier. It names a role and an amount.

17.5 A live run's evidence without one is incomplete, and its figures are not published. A retrospective authorization
is not written, because it would record a decision nobody made at the time.

### 18. The command-line surface

18.1 **Both hosts'** decision-source option admits a fifth value, `llm`. The four existing values, their order and their
help text are unchanged in both.

18.2 Each host's usage text gains the fifth value with its own description, in the form the existing four use. The
description states that this source reaches a model **through a connector program the operator supplies**, is not
deterministic in itself, and replays deterministically from a transcript. It does not say that the program calls a
model, because neither program does: the connector does, and naming it is what keeps the text honest.

18.3 The existing sentence *"None of the four learns anything or calls a model; all four are deterministic"* becomes
wrong when a fifth exists and is corrected in the same change, in both hosts' texts. Rule 18.3 is stated because a usage
text that contradicts the program is a defect a reader meets before any other.

18.4.0 **The option names, as amended 2026-08-28 under `WO-MOK-029`.** `--connector-path <path>` names the
executable the host spawns; `--live` is rule 13.1's explicit live-mode selection, a bare flag on the
`--trace-actions` precedent because it selects rather than carries; `--spend-ceiling <amount>` declares rule 14.6's
ceiling. With `--transcript-path` and `--transcript-output` these are the five of rule 18.4.2.

The three are named because this specification referred to them eleven times without ever saying what an operator
types. An unnamed option is the same defect as an unnamed field: two implementations would offer different command
lines while both conforming, and a reader of this specification could not tell which was right. Each name follows
the surface that already exists rather than a scheme invented for them — every path-carrying option in this
repository is `<noun>-path` or `<noun>-output`, and `--trace-actions` is the precedent for a bare flag.

18.4 **The new options are parsed by the engine's shared parser, which validates each value and then discards it.** This
is not a new mechanism: `--events-path` already works this way, and the parser's own comment records why — the parser
holds a `bool` rather than the path, so that `SPEC-MOK-006` rule 1.2 keeps every path out of the library target, and the
binary target re-reads the raw argument it will open. Both new options follow it exactly. The parser recognizes each,
enforces at-most-once, rejects an empty value and the single character `-` for the reason `SPEC-MOK-001`'s `--events-path`
bullet gives, and **retains neither value**. The configuration value the library holds gains no field, which is rule
10.9's *"the connector path never enters the configuration value the library holds"* satisfied by an existing precedent
rather than by a new rule. Each host then re-reads the raw argument it is the one to act on.

18.4.1 A consequence of 18.4 is worth stating, because it is the difference between this specification and a defect this
repository already has. The observer recognizes its own inputs and hands **every other argument** to the engine's parser,
which `SPEC-MOK-003`'s *Start-up inputs* fixes and which is how the two hosts' parsing, validation, defaults and
rejection behaviour stay identical by construction. So the observer **accepts the connector path whether or not it wants
it**, exactly as it accepts `--events-path` today and then acts on neither. `SPEC-MOK-003`'s 2026-08-22 amendment records
that outcome as a **defect**, tracked as GitHub issue 40, on the ground that "an operator who passes the option and
receives no file and no diagnostic is worse served by silence". This specification does not reproduce it. The observer
**diagnoses** a connector path rather than ignoring it, and rule 18.4.2 fixes the diagnosis.

18.4.2 **The option sets differ by host, and the difference is rule 20 made operator-visible.** The engine's binary
target acts on `--connector-path`, `--live`, `--transcript-path`, `--transcript-output`, `--spend-ceiling` and
`--prices` — **six options**. `--transcript-output` was added 2026-08-28 under `WO-MOK-028`; the three that had
never been named were named the same day under `WO-MOK-029`; `--prices` was added 2026-08-29 under `WO-MOK-030`
by rule 14.3a. The terminal
observer acts on `--transcript-path` and on nothing else. Given `--connector-path`, `--live` or `--spend-ceiling`, the
observer **refuses at start-up with the usage-error status and states that this host replays only**. It is not an unknown
option — the shared parser accepts it, so calling it unknown would be false — and it is not silently ignored, which is
issue 40. It is a host that cannot do what it was asked, saying so. Each host's usage text states which options are its
own, per rule 18.2.

18.4.3 Every one of these options is **rejected when a source other than `llm` is selected**, rather than accepted and
ignored, in both hosts. This differs from rule 20.9's treatment of a supplied port for a good reason: a port is a value a
program passes to itself, while an option is an operator's stated intent, and an operator who names a transcript for a
`social` run has misunderstood something that a silent success would leave misunderstood.

18.4.4 **`--transcript-output` is where a live run writes its transcript, added 2026-08-28 under `WO-MOK-028`.**
`--transcript-path` reads, and keeps exactly the meaning `WO-MOK-025` shipped it with: *"read this run's decisions
from a transcript of an earlier run"*. A live run has none to read and must write one, and rule 19.6 makes failing to
write it the single failure worth aborting a live run for — a run whose exchanges were spent and not recorded has
produced cost and no evidence.

The owner decided on 2026-08-28 that this is a **separate option** rather than one option whose direction changes
with the mode. Its merit is that `WO-MOK-025`'s help text stays true unchanged: a sentence that said "read" and came
to mean "read or write depending on what else you passed" is the shape of defect rule 18.3 exists to correct, and
adding an option avoids creating one. Its cost is a fifth option and two names for one file format, which is accepted.

The two are mutually exclusive. A run reads a transcript or writes one and never both, so giving both is a usage
error under rule 19.2, refused before any tick and before any provider call. `--transcript-output` follows rule 18.4
exactly, like every other path-carrying option: the shared parser validates it and discards the value, and the binary
target re-reads the raw argument it is the one to open. It is the binary target's alone — the observer replays and
never records, so rule 18.4.2's refusal covers it as it covers `--connector-path` and `--spend-ceiling`.

18.5 The terminal observer accepts `llm` **only with a transcript** and refuses it without one, per rule 20.3. Its
authority mapping gains an entry for the fifth source, and its hard-coded four-source description is corrected. It maps
the new source to `REQ-MOK-063`.

### 19. Error and recovery behavior

19.1 A well-formed run that completes exits 0, whether the population survived or went extinct. Extinction is a result.

19.2 A usage error — an unknown option value, a live-mode selection with no credential, a live run with no ceiling, a
host option given with the wrong source, a replay with no transcript — exits with the documented usage-error status,
before any tick runs and before any provider call.

19.2a **One item of that list cannot be reached and is recorded as a defect rather than repaired, as amended
2026-08-29 under `WO-MOK-026`.** The item is *a live-mode selection with no credential*, and three rules of this
specification make it undetectable where 19.2 places it. Rule 13.1 puts the two conditions in two components — "the
selection by the host, the credential by the connector — and neither component can satisfy the other's condition".
Rule 13.3 has the refusal "arrive after the connector was spawned", returned by the connector on the first exchange,
which is inside tick 1. Rule 13.4 forbids either host to read the credential at all: "the library target reaches it by
no route". So no host can observe the condition, and none can observe it "before any tick runs".

Nor can a host recover the status by treating the connector's refusal as one. Rule 19.5a makes a `refused` response an
unconditional "immediate counted fallback", and a host cannot except the credential case out of it without
interpreting an `error.message` that rule 13.3 has the connector word in its own terms and rule 10.7 makes untrusted.

**What a run does, therefore, is rule 19.5a's and not this rule's**: the refusal is a counted fallback under
`REQ-MOK-074`, the connector's message reaches the transcript's `response` field verbatim under rules 11.3 and 1.1a,
and the run continues and exits 0 with `fit` false and every decision a fallback. That is what rule 13.3's "the run
reports which condition was missing, in the terms the connector reported it, and names no value" asks for, and it is
what `VER-MOK-018` case `L20` checks — that case requires "no provider call occurs and the run reports which
condition was missing without printing any value" and requires no early exit. **The remaining four items of 19.2's
list are unaffected**, each being observable by a host from its own command line, and *a live run with no ceiling* is
refused before the first exchange exactly as 19.2 and rule 13.5 both say.

**The defect is recorded and not repaired, and the reason is the same one that has this rule keep its wording.** The
repair is to strike the item or to restate it against rule 13.3, and either is a change of substance in an approved
rule that an implementation agent may not decide; rule 19.5a is the more specific rule and the later-dated one, so a
build following it is following this specification rather than departing from it. A successor may strike the item, and
until one does, this sub-rule is what a reader meets at the contradiction.

19.3 A run stopped at its ceiling exits with a status distinct from both a clean completion and an error, so that a
caller can tell the three apart.

19.4 A replay that fails under rule 12.3 or 12.4 exits with a status distinct from a clean completion, names the
opportunity and the mismatch, and leaves the output produced so far intact and readable.

19.5 A transport failure within a live run is retried a bounded number of times, and each attempt is a transcript record
under rule 11.2. **The bound is three, as amended 2026-08-29 under `WO-MOK-030`**, so an exchange is attempted at
most four times. The rule gave no number until then, while rule 11.2 makes each retry "its own billed exchange" —
so an unstated bound was an unstated spend. Three is chosen against this specification's own figures: an exchange
costs an **estimated** $0.0001, so every exchange retrying to exhaustion on a two-hundred-exchange run is an
**estimated** four cents, and rule 14.6's check runs before each attempt, so the bound cannot breach a ceiling
whatever it is set to. When the retries are exhausted, rule 9.5 applies: the run continues with a counted fallback rather
than ending. A run of an **estimated** 10,954 exchanges that died on its first timeout would be an instrument nobody
could use.

19.5a **The error vocabulary, as amended 2026-08-28 under `WO-MOK-028`.** A response's `error.kind` is one of
four, and what each does is fixed here because 19.5 named only a transport failure and a connector has more ways to
fail than one:

- `transport` — the connector could not reach the provider. Retried under 19.5.
- `provider` — the provider answered, and refused or failed. A rate limit and a server error are this shape.
  Retried under 19.5, because both are ordinarily transient and a run that gave up on the first rate limit would be
  the instrument 19.5 exists to prevent.
- `malformed` — the provider answered with something the connector could not turn into an action. **Not retried**:
  the same request would compose the same prompt and there is no reason to expect a different answer. It becomes an
  immediate counted fallback under `REQ-MOK-074`.
- `refused` — the connector declined to make the call at all, for a reason of its own. **Not retried**, for the same
  reason, and likewise a counted fallback.

Every attempt is a transcript record under rule 11.2 whichever kind it carried, so a run that fell back is
distinguishable from one that never tried, and a retry that succeeded is distinguishable from a first attempt that
did. An `error.kind` outside these four is treated as `malformed`: the connector is untrusted under rule 10.7, and
that includes its error vocabulary.

19.6 A failure to write the transcript ends the run with an error status. A live run whose exchanges were spent and not
recorded has produced cost and no evidence, which is the one failure worth aborting for.

19.7 No error message contains a credential, and no error message contains a path the engine resolved, because the
engine resolves none.

### 20. The two hosts, and where the port is wired

20.1 There are two hosts and their capabilities differ. **The engine's binary target is the recording host**: it may run
live and it may replay. **The terminal observer is the replay host**: it may replay only.

20.2 **The reason is latency against an interactive frame budget, and it is measured rather than preferred.** The observer
owes a frame every 33 milliseconds and an input poll every 16, which `SPEC-MOK-003` rules 6.1 and 6.2 fix. One exchange
takes an **estimated** 0.4 to 0.8 seconds, and a tick holds an **estimated** eleven decision opportunities — 10,954
measured over a 1,000-tick run. A single live tick would therefore block the observer's loop for an **estimated** 4 to 9
seconds, rendering no frame, polling no input and accepting no request to quit. Both rules would be violated for the
whole of every run, not marginally but by two orders of magnitude. The available remedies are concurrency or an
asynchronous runtime: rule 16 forbids the first and `REQ-MOK-050` the second. The restriction is structural, and it is
written here so that an implementer meets it in the specification rather than at the first live run.

20.3 The observer, given this source and **no** transcript, refuses at start-up with the usage-error status and names the
missing transcript. It does not begin a run whose decisions it cannot obtain, and it never falls back to another source:
a substituted source would present a run under the wrong label, which is what `ADR-MOK-007` decision 6 refuses for the
fallback case and refuses here for the same reason.

20.4 **The host builds the port, owns it for the whole run, and lends it to the library. The library builds none, holds
none and closes none.** The port arrives already connected, from a caller that owns the far side, in exactly the form
`SPEC-MOK-002` rule 4 fixed for the record sink: one optional parameter carrying a borrowed trait object, which is `None`
for the four existing sources and needs no type annotation at a call site that has no port. The reason is not symmetry
for its own sake. What sits behind the port is a resource the library is **forbidden** to hold — a spawned child process
in live mode, an open file in replay — and `SPEC-MOK-006` rule 1.2 places every filesystem operation in the binary
target, which is what keeps `SPEC-MOK-001`'s *"the library target interprets no path at all and performs no filesystem
operation"* true. A library that opened the transcript itself would falsify that sentence, and `SPEC-MOK-001` records it
under *Security and privacy properties* rather than as a convenience.

20.4.1 The consequence worth stating separately is that **the port is built once and lent per tick, never rebuilt per
tick.** The engine's four existing sources are stateless values constructed at the point of use, and following that
precedent here is wrong in a way that still compiles and still runs: a port backed by a connector or a transcript holds
the transcript cursor that rule 12.1's ordering depends on, the accumulated cost that rule 14's ceiling depends on, and
the fallback count that `REQ-MOK-074` depends on. A port rebuilt each tick resets all three — the cursor restarts from
the first record, the accumulated cost stays at zero, and the ceiling therefore never triggers. Caller ownership makes
this error unavailable rather than merely prohibited, which is why rule 20.4 is written as ownership rather than as a
warning.

20.5 **Two entry points take the port, as one new optional parameter each: `execute` and `Simulation::advance_tick`.**
The two hosts enter the library by different doors: the recording host drives a whole run to completion through the
process boundary, and the replay host advances a single tick and returns. Wiring the port into only one door excludes the
host that uses the other from this source entirely, while every rule above still reads as satisfied — so this is stated as
an obligation rather than left to follow from rule 20.1. A **canned connector**, a small executable in this repository
that answers from a fixed script and reaches no network, exists so that the live path's wiring is exercised offline at no
cost and on every push. Rule 10.6 records that it is the only connector this repository is able to constrain.

20.5.1 **The two are named rather than described as "the engine's run entry points", because that description has a third
referent and it is not one of them.** `SPEC-MOK-002` rule 5's first list enumerates `Simulation::run`, a public whole-run
method, and it is **not** amended: it delegates with the port absent, as it delegates today with the record sink absent,
and its enumerated form is unchanged. Neither host reaches this source through it. A library consumer that wants a whole
run under this source drives `advance_tick`, which is what the observer does. Stated as its own provision because the
description and the enumeration diverge here, and a reader who resolves "both run entry points" to `run` and
`advance_tick` builds the wrong two doors while every other rule still reads as satisfied.

20.5.2 The consequence for `SPEC-MOK-002` is that the amendment is **not** one rule applied twice. Rule 4 governs
`execute` alone and gains one parameter there, bringing that signature to five. `advance_tick` is enumerated by rule 5,
so its parameter is a rule 5 amendment. And rule 5's mechanical drift checks must be restated in the same change as the
code, because their standing text makes "a fifth parameter" on `execute` a failure and the port is that fifth parameter;
the crate-private carrier `run_recording` also takes the port and is disclosed rather than relied on silently, so
`grep -n 'pub fn .*&mut self' src/simulation.rs` still returns exactly `run` and `advance_tick`. `ADR-MOK-007` records the
amendment in these terms and `VER-MOK-018`'s `S4a` measures the result.

20.6 **The port is a new public interface, and the engine's existing decision-source abstraction stays private.** That
abstraction takes the observation type, which is private and which carries `ADR-MOK-001`'s trust boundary; publishing it
in order to reach this source would export the boundary itself. Rule 1.1's interface takes rule 2's request type by value
instead, and a private adapter inside the engine implements the existing abstraction in terms of it. `ADR-MOK-007`
decision 1's *"one interface and one request type and by nothing else"* fixes the extent, and `SPEC-MOK-002` rules 5 and
6 govern it unchanged.

20.7 **The entropy stream is untouched by this source.** The adapter of rule 20.6 receives whatever entropy handle the
existing abstraction passes it and draws from it not at all, so `REQ-MOK-009` does not move. This is a verified check and
not an assumption: one draw here would shift every subsequent draw in the run, and the four existing sources would then
behave differently at the same seed — which `REQ-MOK-068`'s byte-identity comparison exists to catch.

20.8 **This source selected with no port supplied is an invalid configuration and the run refuses.** The library makes
this check, and it is the one check of rule 13 that the library rather than a host makes. It never substitutes a source,
never proceeds with no decisions and never treats the absence as the fallback of rule 9: those would each produce a run
under the wrong label, which rule 20.3 and `ADR-MOK-007` decision 6 refuse. The rule earns its place because it converts
the failure rule 20.5 exists to prevent into a loud one — a host that admits `llm` on its command line and then omits the
port from its call is refused on the first tick instead of quietly running something else.

20.9 The mirror case is unremarkable and is stated so that it is not read into rule 20.8: a port supplied while one of
the four existing sources is selected is ignored, exactly as an absent sink is, and is not an error. The four sources
consult no port, so a host that supplies one has done something useless rather than something wrong, and rule 16's
non-perturbation obligation is what holds their runs byte-identical either way.

## Data and interface contracts

- **Rule 1.1's interface** is the only interface this specification adds to the engine's public surface. It carries the
  request type and the engine's existing action type, both by value. It carries no transport type, no error type of a
  transport's, and no type owned by a dependency.
- **The request type** is composed of the engine's existing observation-derived values and owned strings. It exposes no
  reference into engine state, honouring `SPEC-MOK-006` rule 12.3's borrow prohibition.
- **Neither package's dependency table grows, and no approved dependency artifact is amended.** Rule 10.1's binding adds
  no crate to either package, so `REQ-MOK-050`, `ARCH-MOK-001`'s conformance check, `SPEC-MOK-002` rule 13 and
  `SPEC-MOK-003`'s declared dependency set all stand unamended, and `SPEC-MOK-004` rules 1 and 2 acquire neither a package
  directory nor a workspace member. This is a stronger claim than any earlier draft of this specification could make, and
  it is the reason `ADR-MOK-007` decision 3 changed.
- **The port is public and the engine's existing decision-source abstraction is not**, per rule 20.6. The public surface
  gains one interface and one request type; the observation type and the existing abstraction stay private, so
  `ADR-MOK-001`'s boundary is reached by neither.
- **The port reaches the library as one new optional parameter on each of rule 20.5's two doors**, carrying a borrowed
  trait object owned by the caller — the shape `SPEC-MOK-002` rule 4 fixed for the record sink, for the reason rule 20.4
  gives. The two public signatures that change are therefore **`execute`**, which reaches five parameters, and
  **`Simulation::advance_tick`**. Rule 4's own precedent covers the form: the sink amendment changed one signature the
  same way and `SPEC-MOK-002` treated it as one parameter added rather than as an interface replaced. `Config` gains no
  field, so a caller that passes `None` is the caller that exists today. `SPEC-MOK-002` rule 5's `pub fn run` is not among
  the two and is not amended — it delegates with the port absent — and the crate-private carrier that does take the port
  down the call chain is disclosed by `ADR-MOK-007` rather than left to be found in a diff.
- **The library performs no filesystem operation and spawns no process, for this source as for every other.** Both
  streams this source needs — the transcript it writes in live mode and the transcript it reads in replay — arrive as
  already-open streams from the host that owns them, rules 11.1 and 12.1.1. `SPEC-MOK-001`'s *"the library target
  interprets no path at all and performs no filesystem operation"* is preserved verbatim and is not scoped, qualified or
  excepted by this specification.
- **The transcript** is a data contract with no consumer in this repository. Rule 11.3's fields and rule 11.4's
  constraints are the whole of it.

## Security and privacy properties

- The credential exists in exactly one place at run time: the process environment of the component rule 10.5 names. It
  is never in the working tree, never in the transcript, never in the record stream, never on either output stream and
  never in an error message.
- The repository's automation holds no credential, rule 13.6. This is the containment that does not depend on code
  being correct.
- A live run cannot be started by accident: two independent conditions must hold, rule 13.1, and the default is the
  free offline path.
- A live run's spending is bounded before it happens, rule 14.6, by a number the owner named, rule 17.1.
- No request carries any data about any Mokiterion other than the one deciding, rule 2.3. The isolation property is a
  privacy property of the population as well as an experimental one.
- Nothing leaves this repository except the request text: the world's rules, one Mokiterion's own state, and a list of
  actions. No source code, no path, no identity and no repository content is sent.
- **Three new capabilities enter this repository, all three in binary targets and none in the library**: a process spawn,
  an environment pass-through and a second and third operator-supplied value interpreted as a filesystem path. Each is
  named here rather than left to be inferred from the rules, because `SPEC-MOK-002` records under its own *Security and
  privacy properties* that "No network access, credential read, filesystem access, environment read, or wall-clock read
  is introduced", and that sentence stops being true of the engine **package** on the day this source lands. It stays true
  of the library target, which is the target rule 20.4 and rule 12.1.1 protect. The amendment is `ADR-MOK-007`'s and is
  stated there in full; what this specification is responsible for is that the difference is not silent.
- **`SPEC-MOK-001`'s "`--events-path`'s value is the one operator-supplied value that is interpreted as a filesystem
  path" becomes false and is amended, not scoped.** There will be three such values, and each is interpreted only by a
  binary target, only as a path, and never as code, a format string, an option or engine input — which is the property
  that sentence exists to hold, and it holds of all three. No credential is ever one of them, per rule 13.4.
- **No network access is added to either package.** The provider is reached by the connector, which is not in this
  repository, so the network capability lives in a process this repository neither builds nor declares. This is the
  security consequence of `ADR-MOK-007` decision 3, and rule 10.8 records its cost honestly: a containment that assumes
  the operator's own connector behaves is not containment.

## Performance and capacity

- **Estimated** at 10,954 decision opportunities for a 1,000-tick run at density 0.75, and 1,200 for a 100-tick
  `reference` run.
- **Estimated** cost per 1,000-tick run: $1.04 under rule 3's layout, $1.36 with caching but no layout discipline,
  $3.72 with no caching, $4.64 at reasoning `low`. Cache writes add an **estimated** $0.004.
- **Estimated** latency 0.4 to 0.8 seconds per exchange, giving 1.2 to 2.4 hours for a 1,000-tick run. A live run is an
  operation with a wall-clock cost measured in hours, which is a design constraint on the horizon and not something to
  be engineered away here: no concurrency across Mokiterions is specified, because concurrent exchanges would make the
  order of transcript records depend on timing and rule 11.2's order is what rule 12.1 replays.
- A replay is bounded by reading the transcript and is free.
- **The exchange latency is what confines live runs to one host.** At an **estimated** eleven decision opportunities per
  tick, one live tick costs an **estimated** 4 to 9 seconds, against the 33-millisecond frame and 16-millisecond input
  budgets `SPEC-MOK-003` rules 6.1 and 6.2 fix for the observer — over by two orders of magnitude. Rule 20.2 is that
  arithmetic turned into a rule. A replayed tick has none of this cost, which is why the observer replays.
- Rule 11.7's transcript sizes bound the evidence a measurement retains.

## Observability

- The text stream and the structured record stream carry this source exactly as they carry the other four. Nothing about
  a decision's origin appears in them beyond the source name, because nothing about it is different: a proposal is a
  proposal.
- Rule 15's run record is the accounting surface. Every figure in it is recomputable from the transcript, which is what
  makes it a report rather than an assertion.
- Rule 11's transcript is the evidence surface. It is the only place a request's and a response's bytes exist, and it is
  why `REQ-MOK-065`'s and `REQ-MOK-066`'s checks are made over transcripts rather than over source code alone.
- Rule 15.3's positive zeros are what let a clean run be distinguished from an unreported one.

## Compatibility and migration

- `SPEC-MOK-006`'s `config.policy` and `result.source` domains gain one value, which rule 10.2 of that specification
  makes a `schema` increment. `ADR-MOK-007` states the amendment; it is not made here.
- `SPEC-MOK-001`'s source vocabulary gains one value. `ADR-MOK-007` states the amendment.
- `INT-MOK-001`'s determinism measure changes in one sentence: the determinand becomes the seed **and the transcript**.
  Rule 12.6 is the property that replaces it, and `ADR-MOK-007` states the amendment. `REQ-MOK-009` does not move,
  because the entropy stream is untouched, rule 16.2.
- No existing requirement's outcome obligation is amended. `REQ-MOK-014`, `REQ-MOK-034`, `REQ-MOK-058` and
  `REQ-MOK-060` each name the source or sources they bind, so a fifth source inherits none of them, and
  `INT-MOK-011` records the absence of a floor for this one positively rather than by silence.
- **`ARCH-MOK-002` is amended, and by more than a name.** The observer becomes this source's replay host, rules 20.1 and
  20.3: it gains a transcript option, opens that file for reading, and hands the library a connected port. It still spawns
  nothing, holds no credential, takes no ceiling and has no live mode. `ADR-MOK-007` states the amendment; it is not made
  here.
- **`SPEC-MOK-003` is amended in three provisions**, and its *Declared dependency set* is not one of them. Rule 11's
  authority mapping gains a row for the fifth source; *Start-up inputs* gains this specification's four options with each
  one's disposition in the observer, per rule 18.4.1; and the usage text's byte-identity obligation extends to the shared
  options' new descriptions. **Rules 6.1 and 6.2 are not amended** — they are the reason rule 20.1 exists, and this
  specification is satisfied by leaving them intact.
- **`SPEC-MOK-001` is amended in seven provisions**, which is the largest amendment this initiative requires, because two
  of its recorded claims stop being true: "There are no external systems and no network calls", which the connector
  falsifies, and "no filesystem location is a source of engine input", which the transcript falsifies. Its
  *Security and privacy properties* sentence naming `--events-path` as "the one operator-supplied value that is
  interpreted as a filesystem path" becomes three values. The property that sentence carries is preserved: each is
  interpreted only by a binary target and only as a path. `ADR-MOK-007` states all seven.
- **`SPEC-MOK-002` is amended in four provisions**: rule 4 gains one optional port parameter on `execute` and on nothing
  else, because rule 4 is the process-boundary rule and governs that signature alone; rule 5 gains the port and the
  request type as items, gains the parameter on `Simulation::advance_tick`'s enumerated row, and has its mechanical drift
  checks restated, because the standing text makes "a fifth parameter" on `execute` a failure and the port is one; and its
  *Actors* and *Security and privacy properties* sections gain a target scope, because the engine **package** does now
  spawn a process, pass an environment through and interpret two more paths while the **library target** still does none
  of those. Rule 13's empty dependency table does not move. `ADR-MOK-007` states all four, and the reason the count is
  four rather than five is that the restated checks live inside rule 5 — which is how the 2026-08-20 amendment for
  `REQ-MOK-042` counted its own restatement.
- **No dependency artifact is amended at all.** `REQ-MOK-050`, `ARCH-MOK-001`'s conformance check, `SPEC-MOK-002` rule 13,
  `SPEC-MOK-003`'s declared dependency set and `SPEC-MOK-004` rules 1 and 2 all stand as written, because rule 10.1's
  binding adds no crate, no package directory and no workspace member. This is the one migration cost this initiative does
  **not** have, and it is stated positively so that a later reader does not assume it was overlooked.
- **`SPEC-MOK-003`'s GitHub issue 40 is neither closed nor worsened.** The observer's silent acceptance of `--events-path`
  is an existing recorded defect; this specification does not repeat its shape for the new options, rule 18.4.1, and does
  not fix it either, because that paragraph calls closing it "a governed change of its own".
- The four existing sources are unchanged, rule 16. No retained capture is retired and no published figure is
  invalidated.

## Examples and counterexamples

### Example: a request, abbreviated

```
[block A]  A Mokiterion lives on a grid. It has health, satiety, energy and fear,
           each an integer from 0 to 100, and a waste tolerance from 0 to 40. It
           perceives 16 units. On each tick it proposes exactly one action. ...
           Answer with one action from the list at the end of this message. ...
[block B]  You are M03. Your waste tolerance is 27.
[block C]  Tick 41. Position (12, 5), territory north-west.
           Health 63. Satiety 40. Energy 58. Fear 12.
           Suffered since your last action: M07 for 9.
           Co-located food: none.
           Perceived food: F12, plant, east, 4.
           Perceived Mokiterions: M07, east, 3. M11, south, 11.
[block D]  wait | sleep | move north | move east | move south | move west |
           attack M07 | threaten M07 | fight M07 | retreat M07 | surrender M07 |
           approach M07 | avoid M07 | approach M11 | avoid M11
```

Block A is byte-identical in every request of the run. A and B together are byte-identical in every request for M03.
`eat` is not enumerated because no food is co-located. The five contact verbs are enumerated against M07 and not M11,
because rule 7.4 drops a verb whose preconditions no target satisfies and M11 is out of contact.

### Example: a response

```
{"action":"avoid","target":"M07"}
```

One verb, one parameter, both from block D. No prose, no reason, no alternative.

### Example: a transcript record, abbreviated

```
{"tick":41,"actor":"M03","request":"...","response":"...","usage":{"prompt":1431,"cached":1230,"output":11,"reasoning":0},"action":{"verb":"avoid","target":"M07"}}
```

The reasoning count is zero and is written, rule 15.3's principle. The cached count against the prompt count is what
rule 14.4 sums.

### Example: an exchange that yielded nothing

```
{"tick":41,"actor":"M03","request":"...","error":"...","usage":null,"action":{"verb":"wait"},"fallback":true}
```

Usage is absent rather than zero, rule 11.5. The action is `wait`, rule 9.5. The run's fallback count moves, and rule
15.4 marks the run.

### Counterexample: the observation's core-proposal list as block D

Block D built from that list would offer the core actions and no targeted one, so the model could never propose an
attack, a retreat or an approach. The measurement would report that a model does not fight, when in fact it was never
asked. Rule 7.2 forbids it, and `REQ-MOK-064` states it as an obligation because it is the mistake a reasonable
implementer would make from reading the observation.

### Counterexample: block D by extending the core-proposal list

Extending the list would offer the same information and would move `baseline`'s entropy selection, diverging every run
ever recorded under it. Rules 16.2 and 16.3 forbid it.

### Counterexample: block A with an objective

*"Your goal is to survive as long as possible."* Forbidden by rule 4.4. The run would measure the sentence.

### Counterexample: block A carrying the tick

The tick in block A varies per request, so no two requests share a leading span, the cache ratio collapses to near
zero, and the run costs an **estimated** 3.6 times more for identical information. Rules 3.3 and 4.5 forbid it.

### Counterexample: a conversation per Mokiterion

Twelve provider conversations, each accumulating a thousand ticks, would give each Mokiterion a memory that lives in a
vendor's context window, appears in no record, and cannot be reconstructed from a transcript. Rule 2.4 forbids it, and
`REQ-MOK-066` gives the reasons at length.

### Counterexample: falling back to `baseline`

A run that substitutes `baseline`'s selection for an unanswered decision reports what a mixture of two sources did
under one label. Rule 9.7 forbids it.

### Counterexample: a locally estimated cache ratio

A ratio computed from a token estimate in this repository can be 0.86 while the provider charged for every token. Rule
14.4 forbids it.

### Counterexample: a provider key in a workflow secret

`REQ-MOK-072`'s gate would still refuse a run with no live-mode flag, but the credential would be one workflow edit
from being spendable. Rule 13.6 forbids the reference, and the containment is that the secret does not exist.

## Explicitly unspecified decisions

- **Block A's exact wording.** Rules 4.1 through 4.6 fix its content, its prohibitions and its constancy. The prose is
  the implementation's, and its token count is what rule 14.5's ratio measures.
- **Block C's and block D's exact rendering.** Rules 6.1 and 7.7 fix the fields and their order; the separators,
  punctuation and line breaks are the implementation's, subject to being identical across two runs of one
  configuration.
- **The action grammar's shape.** Whether block D enumerates verb-target pairs as one flat list or as a verb list with
  per-verb target lists is unspecified, and the trade-off is left to measurement: a flatter list is longer and costs
  more variable tokens, a nested one is shorter and may be harder to answer well. Rule 7.1's completeness holds either
  way.
- **The retry count and its backoff.** Rule 19.5 requires a bounded retry; the bound is the implementation's.
- **The connector's language, its dependency surface, its internal design and its location.** Rule 10.6 states that this
  specification constrains none of them, and why it cannot. The **canned connector** of rule 20.5 is the single exception,
  because that one is this repository's own; its language is still unspecified.
- **How a host passes its environment to the connector.** Rule 10.5 fixes that the credential reaches the connector by its
  own environment and by no option; whether the host inherits, filters or extends that environment is the
  implementation's, subject to security check `C1` finding no credential in any produced byte.
- **The transcript's exact serialisation.** Rule 11.3's fields and rule 11.4's constraints hold; the encoding is the
  implementation's, provided it is diffable and stable.
- **Concurrency.** Not specified, and rule 11.2's ordering is why. A later intent may propose it.
- **Bounded per-Mokiterion memory.** Not specified and not available. `REQ-MOK-066` records that it would be
  engine-owned, bounded, specified and emitted, under its own intent — not acquired by loosening rule 2.4.
- **A second provider or a second model.** Unauthorized. `INT-MOK-011`'s non-goals record it.
