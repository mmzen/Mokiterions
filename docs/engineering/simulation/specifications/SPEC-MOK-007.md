+++
id = "SPEC-MOK-007"
type = "specification"
title = "Model-backed decision source: the decision port, the cache-ordered request, and the transcript"
status = "draft"
owners = ["technical owner"]
created = "2026-08-23"
updated = "2026-08-23"

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
]
+++

# Specification: Model-backed decision source: the decision port, the cache-ordered request, and the transcript

## Scope

This specification fixes the exact behavior of the fifth decision source: the port the engine obtains a decision
through, the content and byte order of a decision request, the grammar a response must satisfy, what happens when a
response does not satisfy it, the transcript a live run retains, how a replay consumes that transcript, the usage and
cost accounting a live run performs, the conditions under which a provider call may happen at all, and the failure
behavior of each.

It specifies `REQ-MOK-063` through `REQ-MOK-076` and nothing else.

It does not restate the simulation's rules. `SPEC-MOK-001` remains the sole authority for what a Mokiterion may
propose, what an observation carries, how a proposal is validated and resolved, and what the text stream contains; this
specification refers to that authority rather than copying it, so that every rules question has exactly one answer. It
does not restate the structured record stream either, whose authority is `SPEC-MOK-006`. The transcript this
specification defines is a **third** stream, distinct from both.

**One provision here depends on a decision the repository owner has not taken.** `ADR-MOK-007` puts two transport
bindings for the provider: an HTTPS client inside the workspace, or a separate provider program the host drives over
pipes. This specification is written against the second, which `ADR-MOK-007` recommends. If the owner selects the
first, **rules 10 and 18.4 change and no other rule does** — the port of rule 1, the request of rules 2 through 7, the
response grammar of rule 8, the transcript of rule 11 and the replay of rule 12 are identical under either binding.
That localisation is deliberate and is the reason the port is specified before the transport.

Throughout, *the port* means the interface the engine obtains a proposal through; *the host* means the code outside the
engine that connects the port to something; *the provider* means the language model service; *an exchange* means one
request sent to the provider and the response or error that came back; *a live run* means a run that makes provider
calls; *a replay* means a run that obtains its decisions from a retained transcript; and *the transcript* means the
stream rule 11 defines.

Amounts in currency and token counts given as *estimated* are estimates made on 2026-08-23 against the published
`gpt-5.6-luna` prices and a measured count of 10,954 decision opportunities in a 1,000-tick `social` run at seed 0 and
density 0.75. They are stated so that a later reader can see what the design was sized against; none of them is a
conformance condition. The conformance conditions are rule 14's ratio and rule 14's ceiling.

## Actors and external systems

- **The engine's library target** composes each decision request from the observation it already holds, hands it to the
  port, and receives a proposal. It resolves no path, opens no file, creates no directory, removes no file, opens no
  socket, spawns no process and reads no environment variable. It is the sole author of every request's content and of
  every transcript record's content.
- **The engine's binary target — the host** parses the options, reads the credential from the process environment,
  establishes whatever the transport binding requires, resolves the transcript path, opens the transcript for writing or
  for reading, hands the library a connected port and an open stream, and flushes and closes what it opened. It authors
  no request content and no transcript record content.
- **The provider** is `gpt-5.6-luna`, reached in the manner `ADR-MOK-007` decides. It is not part of this repository, it
  is not deterministic, and nothing here assumes it is.
- **The repository owner** authorises a live run. The authorization is a retained artifact, rule 17, and no code
  consults it.
- **A consumer** is any program that reads a retained transcript. No consumer is specified. The transcript is specified
  so that writing one requires no knowledge of the provider.

## Inputs

The decision source is selected by a command-line option, rule 18. Beyond the inputs every run already takes — seed,
tick limit, density, tracing selection, record-stream sink — a run under this source takes:

- **A mode**: live or replay. Replay is the default, rule 13.1.
- **A transcript**: an open stream the host supplies. Written in live mode, read in replay mode. The engine never names
  it.
- **In live mode only**: a provider credential from the process environment, a spend ceiling, and a model
  identifier with its reasoning level. The credential never reaches the library target.

No input reaches the simulation's rules. A Mokiterion's behaviour is a function of the observation and the response,
and of nothing else in this list.

## Outputs

- The **standard output text stream** of `SPEC-MOK-001`, unchanged in form. Under this source it carries `luna` where
  it carries a source name, and nothing else about it moves.
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
either a proposal or the fact that none was obtained. The interface names no provider, no transport, no model, no
credential, no file and no mode.

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

### 10. The provider binding

*Under the binding `ADR-MOK-007` recommends. See Scope.*

10.1 The host drives a **separate provider program** and exchanges with it over that program's standard input and
standard output. The engine's workspace acquires no HTTPS client, no TLS stack and no asynchronous runtime, so
`REQ-MOK-050`'s dependency prohibition and `ARCH-MOK-001`'s conformance check are untouched and need no amendment.

10.2 The framing is one JSON object per line in each direction: one request object per line to the program's standard
input, one response object per line from its standard output, in the same order. Lines are newline-terminated and
contain no newline within an object.

10.3 A request object carries the prompt text rules 3 through 7 compose, the model identifier, the reasoning level and
the response schema rule 8.4 needs. It carries no credential.

10.4 A response object carries the action, the provider's reported usage counts, and either success or an error. The
usage counts are the provider's own figures, passed through unmodified; the program computes none of them.

10.5 The provider program reads the credential from its own process environment. It is the only component that holds
one, and it never writes one to its standard output, its standard error or any file.

10.6 The provider program's own dependency surface is constrained to its language's standard library. This is stated as
a rule because option B's honest cost, recorded in `ADR-MOK-007`, is that it moves a dependency surface outside the
declared-set discipline unless something holds it; rule 10.6 is that something.

10.7 The engine does not know rule 10 exists. Everything in it is on the host's side of rule 1.1's interface.

### 11. The transcript

11.1 A live run writes a transcript. The engine authors every record; the host owns the destination and hands the
engine an already-open stream, on `SPEC-MOK-006` rule 1.2's precedent — the engine resolves no path and opens no file.

11.2 The framing is one record per line, one line per exchange, in the order the run made them. A retry is its own
record, because it was its own billed exchange.

11.3 A record carries: the tick and the acting Mokiterion, so the exchange is bound to its opportunity; the request as
sent, in full; the response as received, in full, or the error; the provider's reported prompt, cached-prompt, output
and reasoning token counts; and the action the response was parsed into, or the fact that it was not parsed and why.

11.4 `SPEC-MOK-006`'s constraints are adopted for the transcript: no floating-point value, no timestamp, no path, no
value outside a closed alphabet, and bytes comparable between runs. A transcript is diffable evidence, comparable with
`cmp`, rather than a log.

11.5 A reported count that the provider did not report is recorded as **absent**, not as zero. A missing count and a
count of zero mean different things, and rule 14.5 depends on telling them apart.

11.6 A transcript contains no credential, no authorization header and no provider account identifier. It is retained
inside the repository, and `REPOSITORY_CONTEXT.md` requires credentials to remain outside it.

11.7 A transcript is never truncated or abbreviated to fit a size budget. Its size is bounded by the horizon chosen: an
**estimated** 4.7 MB for a 1,000-tick run, an **estimated** 100 to 260 KB for a 20-to-50-tick run. What is retained
where is `VER-MOK-018`'s.

11.8 A replay writes no transcript. It has one; it is reading it.

### 12. Replay

12.1 A replay obtains each decision from the transcript, in order, through the same port rule 1.1 defines and the same
code path a live run uses.

12.2 A replay makes no provider call, opens no socket, spawns no provider program and reads no credential. This holds
whether or not a credential is present in the environment.

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

13.1 Replay is the default. A live run happens only when **both** an explicit live-mode selection was made **and** a
provider credential is present in the process environment. `REQ-MOK-072` states this and rule 13.1 is its whole
mechanism.

13.2 When the live-mode selection is absent, the run replays if a transcript was supplied and otherwise refuses with the
usage-error status, rule 19.2. A present credential is never taken as consent.

13.3 When the credential is absent, empty or malformed, no provider call is made. The run reports which condition was
missing and names no value. The credential is looked for in the process environment and nowhere else: no file, no
keychain, no configuration directory is searched.

13.4 The credential is read by the host, never by the library target. It is never written to a tracked file, never
printed to either output stream, never placed in a request record and never placed in an error message.

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
stated minor unit. `SPEC-MOK-006`'s prohibition on floating-point values in a stream holds for the run record, so the
figure reported is an integer in a stated unit rather than a formatted decimal whose bytes vary by platform.

14.3 The declared unit prices are inputs of the run, not compiled-in constants. The provider's prices are the
provider's to change.

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

18.1 The decision-source option admits a fifth value. The four existing values, their order and their help text are
unchanged.

18.2 The usage text gains the fifth value with its own description, in the form the existing four use. Its description
states that this source calls a model, is not deterministic in itself, and replays deterministically from a transcript.

18.3 The existing sentence *"None of the four learns anything or calls a model; all four are deterministic"* becomes
wrong when a fifth exists and is corrected in the same change. Rule 18.3 is stated because a usage text that contradicts
the program is a defect a reader meets before any other.

18.4 The live-mode selection, the transcript path, the spend ceiling, the model identifier and the reasoning level are
options of the host. They are rejected when any source other than this one is selected, rather than accepted and
ignored.

18.5 The terminal observer's authority mapping gains an entry for the fifth source, and its hard-coded four-source
description is corrected. It maps the new source to `REQ-MOK-063`.

### 19. Error and recovery behavior

19.1 A well-formed run that completes exits 0, whether the population survived or went extinct. Extinction is a result.

19.2 A usage error — an unknown option value, a live-mode selection with no credential, a live run with no ceiling, a
host option given with the wrong source, a replay with no transcript — exits with the documented usage-error status,
before any tick runs and before any provider call.

19.3 A run stopped at its ceiling exits with a status distinct from both a clean completion and an error, so that a
caller can tell the three apart.

19.4 A replay that fails under rule 12.3 or 12.4 exits with a status distinct from a clean completion, names the
opportunity and the mismatch, and leaves the output produced so far intact and readable.

19.5 A transport failure within a live run is retried a bounded number of times, and each attempt is a transcript record
under rule 11.2. When the retries are exhausted, rule 9.5 applies: the run continues with a counted fallback rather
than ending. A run of an **estimated** 10,954 exchanges that died on its first timeout would be an instrument nobody
could use.

19.6 A failure to write the transcript ends the run with an error status. A live run whose exchanges were spent and not
recorded has produced cost and no evidence, which is the one failure worth aborting for.

19.7 No error message contains a credential, and no error message contains a path the engine resolved, because the
engine resolves none.

## Data and interface contracts

- **Rule 1.1's interface** is the only interface this specification adds to the engine's public surface. It carries the
  request type and the engine's existing action type, both by value. It carries no transport type, no error type of a
  transport's, and no type owned by a dependency.
- **The request type** is composed of the engine's existing observation-derived values and owned strings. It exposes no
  reference into engine state, honouring `SPEC-MOK-006` rule 12.3's borrow prohibition.
- **The engine's dependency table does not grow.** Under rule 10's binding the workspace acquires no crate, so
  `REQ-MOK-050` and `ARCH-MOK-001` are unamended. Under the alternative binding `ADR-MOK-007` puts, they are not, and
  the ADR states the amendment.
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

- **The transport binding.** `ADR-MOK-007`'s, and the owner's to take. Rules 10 and 18.4 are written against the
  recommended binding; Scope states what changes under the other.
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
- **The provider program's language.** Rule 10.6 constrains its dependency surface and not its language.
- **The transcript's exact serialisation.** Rule 11.3's fields and rule 11.4's constraints hold; the encoding is the
  implementation's, provided it is diffable and stable.
- **Concurrency.** Not specified, and rule 11.2's ordering is why. A later intent may propose it.
- **Bounded per-Mokiterion memory.** Not specified and not available. `REQ-MOK-066` records that it would be
  engine-owned, bounded, specified and emitted, under its own intent — not acquired by loosening rule 2.4.
- **A second provider or a second model.** Unauthorized. `INT-MOK-011`'s non-goals record it.
