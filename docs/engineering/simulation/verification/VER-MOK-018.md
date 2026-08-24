+++
id = "VER-MOK-018"
type = "verification"
title = "The instrument is verified, the outcome is not: replay identity, isolation over transcripts, cache order, and a deliberate absence of an outcome oracle"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-23"
updated = "2026-08-24"

[relations]
verifies = [
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

# Verification Contract: The instrument, not the outcome

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-23 | Original approved content, covering `REQ-MOK-063` through `REQ-MOK-077`. | Approved 2026-08-23 by the repository owner acting as accountable assurance owner, in the words *"i approve the artifact pack"*, together with `INT-MOK-011`, `CAP-MOK-011`, the fifteen requirements, `SPEC-MOK-007` and `ADR-MOK-007`. `ADR-MOK-007`'s *Decision record* holds it as act 11. |
| 2026-08-24 | **Three cases amended, each because the implementation of `WO-MOK-025` ran it and found the wording wider than, narrower than, or inconsistent with what it can enforce. No case is added, no case is removed, no pass condition is relaxed into a weaker check, and no requirement's coverage changes.** **L5** — *the enumeration is not the core list* — is restricted to the requests that enumerate a targeted action. The unrestricted clause reads on every request, including those at an opportunity with nothing in perception to target, whose enumerated set equals the observation's core-proposal list by the absence of a target rather than by a failure to derive; the check measures **104 of 221 requests** enumerating a targeted action, and **117 of 221** with a set equal to the core list, the difference being exactly the requests with nothing to target. The unrestricted reading therefore fails a conforming run, which is the failure this specification exists to prevent. The case now says which reading it means and the check prints both figures. **L17** — *the transcript's constraints* — loses its fourth clause, "no value outside the closed alphabet", and gains a round trip through the transcript's escaping function in its place. `SPEC-MOK-007` rule 11.4.1 withdrew that alphabet the same day on a measurement over the committed transcript's shared rules block: it carries 1,282 spaces, 90 newlines, 44 commas, 9 less-than signs, 5 apostrophes and 2 em dashes, none of which the alphabet holds. A transcript that satisfied the withdrawn clause could not carry a request. The three surviving clauses — no floating-point value, no timestamp, no path — are unchanged, and the round trip is stronger than the alphabet was for the property the clause was written for, since it holds of every text field rather than of a character set. **L30** — *the port is lent, not rebuilt* — withdraws its illustrative ceiling of "the cost of two exchanges" and requires instead a ceiling reached in a later tick and not in the first, derived from the run's arity and stated with the run. An exchange is issued per acting Mokiterion, not per tick; at a twelve-opportunity tick a two-exchange ceiling trips inside tick one, and **a port rebuilt every tick trips it there too** — so the stated figure defeated the discriminator this case exists to be. At `WO-MOK-025`'s configuration the ceiling is **eighteen**, one and a half ticks. The case's substance is unchanged and is what was checked; only the number moves. | **Approved 2026-08-24 by the repository owner acting as accountable assurance owner.** All three were raised by the implementation agent of `WO-MOK-025` as escalations **E8**, **E9** and **E10**, put to the owner together with eight siblings with each measurement displayed, and approved in the turn the question was asked. None was repaired when found: `WO-MOK-025` stop-and-escalate condition 6 forbids amending an approved artifact on an implementation agent's judgement, and condition 7 requires a case whose enforced reading differs from its stated one to be escalated rather than reconciled in the evidence. The completion report records **L5** as PASS AND ESCALATED, **L17** as PASS IN PART and **L30** as PASS with the figure disclosed, and those dispositions stand as written: this row states what the contract now requires, not that the candidate's result changed. **No verification record is re-opened.** `VREC-MOK-024`, which will bind `WO-MOK-025`'s candidate to this contract, is prepared after this row and cites the amended text. The implementation agent measured every figure here and wrote the text; it decided none of the substance. |

## The absence of an outcome oracle, stated as a decision

**This contract asserts nothing about what the population does.** No case below states a survivor floor, a death
ceiling, a combat rate, or that a model-backed run compares any particular way against `reference` or `social`. That is
not an omission and it is not a gap in coverage. It is a decision, and it is recorded here first, before the matrix, so
that a reader meets it as a decision rather than discovering it as a silence.

The decision is the repository owner's, taken on 2026-08-23: *"defining a floor is probably not possible as the whole
point of this is to empirically see what is going to happen, the constraints need to be relaxed for the LLM policy."*
`INT-MOK-011` records it as a non-goal and `ADR-MOK-007` decision 7 records its architectural consequence.

The reason it must be stated positively rather than left implicit is that **four decision sources in a row received a
floor**. `REQ-MOK-014` names the reference source, `REQ-MOK-034` the trait-aware source, `REQ-MOK-058` the social
source, and `REQ-MOK-060` names three of them. A fifth contract arriving with no such case would read as an oversight to
anyone who had read the other four, and the correction would arrive as a "missing" assertion that nobody decided to add.
`REQ-MOK-034`'s 2026-08-20 drafting set the precedent for stating a deliberate absence positively, and case **L26**
below turns the absence into a check of its own: it fails if a threshold ever appears.

There is also a precedent for a source held to no floor. `baseline` goes extinct between ticks 119 and 193 on every
declared seed, and that is recorded as measurement, never as failure.

What replaces an outcome oracle is `REQ-MOK-075`: the outcome is **reported**, beside `reference` and `social` at the
same seeds and horizon, with no threshold on any of the nine figures. A reporting obligation is verifiable — case
**L24** checks that the nine figures exist and were produced at one horizon — and it is the strongest thing that can be
checked about an outcome nobody has decided in advance.

## Independence

Five things keep this contract independent of how the source is written.

1. **The oracle for replay identity is a retained capture, not a second implementation.** Case **L7** compares a
   recorded run's bytes against a replay's with `cmp`. No test recomputes what the output should be, so no test can
   agree with the implementation about a shared mistake.

2. **The isolation and layout properties are checked over retained transcripts, not over source code.** `REQ-MOK-065`
   and `REQ-MOK-066` are properties of *what was sent*. A check that reads the composing function cannot see a value
   that arrived through a shared buffer, and cannot see a prefix broken by a formatting call three layers down. Cases
   **L4**, **L5** and **L14** read bytes.

3. **Every cost and cache figure comes from the provider's reported usage**, never from a token count computed in this
   repository. A local estimate would let an implementation satisfy `REQ-MOK-070` while paying full price, and it would
   make the check agree with the layout it was meant to police.

4. **Non-perturbation is checked against captures taken at the base commit**, before the change exists, on both sides.
   Case **L8** compares; it does not re-derive. `WO-MOK-025` obliges the capture, and a missing capture is a gap in
   evidence rather than a passing check.

5. **The tests live in the tier the specifications assign**, exercising the engine through its stated public interface
   from outside the crate wherever that tier applies. No oracle depends on a private item, and no item is widened to be
   reached from a test. Widening is a prohibited pattern under `ARCH-MOK-001` and is a failure of this contract, not a
   way of satisfying it.

This contract is authored before the implementation exists and names no file, no type and no function of it.

## What a green build does and does not establish

Stated here because it is the single most misreadable thing about this contract. `REQ-MOK-073` and `ADR-MOK-007`
decision 5 keep the provider credential out of continuous integration entirely, so **no case that requires a provider
call runs in automation.**

A green build establishes: the port, the request composition, the enumeration, the isolation properties, the cache
layout, the replay identity, the byte-identity of the four existing sources, the two hosts and their split, the gating,
the ceiling arithmetic, the fallback accounting, and the absence of an outcome threshold. That is thirty of the
thirty-five cases below, and all of them run free and offline against a committed transcript, a stubbed port and the
canned connector.

That the free set is thirty of thirty-five is itself an owner decision, taken on 2026-08-23: *"CI can not replay the LLM
policy after push, release etc … same for unit and automated tests"*, and, asked whether a short committed transcript
may nonetheless be replayed in automation, **yes**. Replay makes no call and needs no credential, so it is the whole of
verification tier 1 and the reason this contract is mostly free rather than mostly gated. `ADR-MOK-007`'s *Decision
record* holds both halves.

A green build does **not** establish: `REQ-MOK-070`'s cache ratio, `REQ-MOK-075`'s measurement, or that any model was
ever consulted. Those are cases **L15b**, **L24** and **L25**, marked **owner-gated** in the matrix, and each requires
an authorised live run under `REQ-MOK-076`. Two more are judgements rather than runs: **L27** is the assurance owner's
manual assessment that the shared rules block carries no strategy — free and offline, but not a build — and **L28**'s
static half runs everywhere while *whether the retained authorization is genuine* is an owner attestation. A sixth fact
sits outside the matrix entirely: check **C6**, that the credential is not configured in the repository's automation
secrets, which no check inside the repository can see.

Five cases therefore cannot be satisfied by a build. That is a consequence of the owner's cost decision and is recorded
as a limit rather than engineered around.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-063` | automated test | **L1** one path, no privilege | a proposal arriving from a stubbed port is validated, resolved, traced and rejected by the same code path and produces records of the same form as the identical action arriving from `social`; a differential run over every seed shows no record whose form differs |
| `REQ-MOK-063` | automated test | **L2** the source is named | the record stream's decision-source record names the model-backed source exactly once, before tick processing, on every seed |
| `REQ-MOK-063` | static check | **L3** the port is the only path | exactly one interface obtains a proposal under this source; the library target contains no branch on live-versus-replay and no mode value; no transport type, no reference and no mutable borrow of authoritative state appears on the added public surface, re-running `SPEC-MOK-002` rule 6's existing check unchanged |
| `REQ-MOK-064` | automated test | **L4** the enumeration is complete | over a retained transcript, for every request, the enumerated set equals the set an independently written enumerator computes from that request's observation under `SPEC-MOK-001` rule 6 and rule 21 — every core proposal, `eat` per co-located resource, each valid cardinal move, and each of the seven targeted verbs against each identifier whose precondition it satisfies |
| `REQ-MOK-064` | automated test | **L5** the enumeration is not the core list | at least one request in the transcript enumerates a targeted action, and **among the requests that enumerate one**, no request's enumerated set equals the observation's core-proposal list; a run in which no targeted action is ever enumerated fails this case as unexercised. **The clause is restricted to those requests deliberately** *(amended 2026-08-24)*: a request at an opportunity with nothing in perception to target has nothing to add to the core list, so its set equals that list by the absence of a target rather than by a failure to derive, and the wider reading would fail a conforming run on an opportunity the world never offered. A check reports both figures and decides neither |
| `REQ-MOK-064` | automated test | **L6** no unsatisfiable offer | no request enumerates a targeted verb against an identifier whose precondition that verb does not satisfy at that opportunity |
| `REQ-MOK-067` | automated test | **L7** replay identity | a recorded run and a replay of it compare equal with `cmp` on standard output and on the structured record stream, and have the same exit code, with no credential in the environment and no network reachable; checked at every declared seed, with tracing on and off |
| `REQ-MOK-067` | automated test | **L8** mismatch is detected | a replay of the same transcript at a different seed, at a different density, and against a transcript truncated mid-run each fail, name the opportunity and the mismatch, and produce no further ticks; a transcript longer than the run needs produces an unaffected run |
| `REQ-MOK-068` | automated test | **L9** four sources byte-identical | for `baseline`, `reference`, `individual` and `social` at every declared seed and the default density, standard output bytes, structured record stream bytes and exit codes equal the base-commit captures, and per-tick entropy draw counts equal them for the whole run |
| `REQ-MOK-068` | automated test | **L10** observed equals unobserved | an observed run and an unobserved run of each of the four sources remain byte-identical, as `ADR-MOK-006`'s validation list already requires |
| `REQ-MOK-068` | static check | **L11** the core list did not grow | the observation's list of currently valid core proposals has the same members and order as at the base commit; the engine consumes no entropy under the model-backed source |
| `REQ-MOK-065` | automated test | **L12** one Mokiterion per request | over a retained transcript, each request's variable content names exactly one Mokiterion's attributes — its own — and each perceived entry carries only an identifier, a relative direction and a distance; a request naming any other Mokiterion's health, satiety, energy, fear, waste tolerance or territory fails |
| `REQ-MOK-065` | automated test | **L13** no aggregate and no derived value | no request contains a count, mean, maximum or ranking over the population, and no request contains any byte sequence taken from another request or response in the transcript beyond the shared prefix; checked including a run in which every Mokiterion has acted |
| `REQ-MOK-066` | automated test | **L14** self-contained requests | for one Mokiterion, the request at the last tick and the request at the first differ only in the observation block and the enumerated set, and neither contains any part of the other; no request contains a provider-side conversation or session identifier |
| `REQ-MOK-070` | static check | **L15a** the layout holds | over a retained transcript: the shared rules block is byte-identical across every request of the run, and the shared-plus-actor prefix is byte-identical across every request for one Mokiterion; the observation block and the enumerated set appear after both. Runs free against a committed transcript |
| `REQ-MOK-070` | automated test | **L15b** the ratio holds — **owner-gated** | over an authorised live run of at least 200 exchanges, cached prompt tokens summed from the provider's reported usage are at least 85 percent of total prompt tokens; a run reporting no cached-token figure fails as unevaluable rather than passing |
| `REQ-MOK-069` | automated test | **L16** every exchange retained | the transcript holds one record per exchange, each binding tick and Mokiterion, each carrying the request, the response or the error, four token counts, and the parsed action or the parse failure; a retried exchange appears as two records; a count the provider did not report appears as absent, not as zero |
| `REQ-MOK-069` | static check | **L17** the transcript's constraints | no floating-point value, no timestamp and no path appears in a transcript, every text field survives a round trip through the transcript's escaping function unchanged, and two transcripts of the same recorded run compare equal with `cmp`. **The closed-alphabet clause is withdrawn** *(amended 2026-08-24)*: `SPEC-MOK-007` rule 11.4.1 withdrew it the same day, on the measurement that a request's own prose carries spaces, commas, apostrophes, less-than signs and em dashes, none of which the alphabet holds. The round trip is what replaced it and is what this case now checks |
| `REQ-MOK-071` | automated test | **L18** the ceiling bounds | against a stubbed port with declared unit prices and synthetic usage, a run declared with a ceiling reached mid-run issues no exchange after the ceiling, ends with the transcript and record stream complete to that tick, reports the ceiling, the accumulated cost and the tick reached, and exits with a status distinct from a clean completion and from an error |
| `REQ-MOK-071` | automated test | **L19** the check precedes the spend | no exchange is issued whose cost would cross the ceiling; a ceiling equal to the cost of two exchanges yields exactly two |
| `REQ-MOK-072` | automated test | **L20** both conditions required | **in the engine's binary target**, which is the only host a live run is reachable from at all: with a credential present and no live-mode selection, no provider call occurs and the run replays or refuses; with a live-mode selection and no credential, no provider call occurs and the run reports which condition was missing without printing any value; an empty or malformed credential is treated as absent; a live run with no declared ceiling is refused before the first exchange. Verified with no real credential ever present. In the observer the two conditions are unreachable rather than satisfied, which case **L32** checks instead |
| `REQ-MOK-073` | static check | **L21a** automation holds no credential | no workflow file in the repository references a model-provider credential as a secret, an environment variable, an input, or through a step that fetches one, and no workflow selects live mode; the check fails on the pull request that introduces such a reference |
| `REQ-MOK-073` | automated test | **L21b** automation still exercises the source | a workflow step runs the model-backed source in replay mode against the committed transcript, and that step's presence is checked rather than assumed |
| `REQ-MOK-074` | automated test | **L22** the fallback is counted | an exchange yielding no response and an exchange yielding an unenumerated action each propose `wait`, increment the count, record the cause in the transcript, and mark the run unfit; a response naming a target the observation did not carry does the same |
| `REQ-MOK-074` | automated test | **L23** a rejection is not a fallback | a well-formed enumerated proposal that the engine's rules reject increments the existing rejection counter and does **not** increment the fallback count and does **not** mark the run; a clean run reports the fallback count as `0` rather than omitting it |
| `REQ-MOK-075` | manual assessment | **L24** the comparison is published — **owner-gated** | for the declared seed set at the declared horizon, survivors, deaths and combat deaths are reported for the model-backed source and for `reference` and `social` **re-run at the same seeds and horizon**, with the seed set, horizon, density and source run records named, and with **no threshold applied to any figure** |
| `REQ-MOK-075` | automated test | **L25** only fit runs are published — **owner-gated** | every run whose figures are published reports a fallback count of `0` and did not stop at its ceiling; a seed whose run was unfit is reported as a gap rather than substituted |
| all fifteen | static check | **L26** no outcome threshold exists | no survivor floor, death ceiling, combat-rate bound or outcome comparison assertion for the model-backed source appears anywhere in the verification suite. **This case fails when such an assertion is added.** See the opening section for why an absence is checked |
| `REQ-MOK-063`–`REQ-MOK-076` | manual assessment | **L27** the prompt carries no strategy | the shared rules block states the world's rules, the attribute ranges, the verbs, the perception radius and the response grammar, and states no goal, preference, objective or advice; assessed against `SPEC-MOK-007` rule 4.4 by the assurance owner and recorded as an assessment |
| `REQ-MOK-076` | static check + attestation | **L28** the authorization is retained | every live run's retained evidence includes an authorization record naming the authorizing owner, the date, the horizon, the seed set and the ceiling; each run's actual seed, horizon and ceiling falls within it; the record contains no credential and no account identifier. **Whether the authorization is genuine is an owner attestation**, not a check |
| `REQ-MOK-077` | automated test | **L29** both of rule 20.5's doors carry the port | a port that supplies proposals drives a run through `execute`, and a replay drives the same source through `Simulation::advance_tick`, in one suite; a build in which either door lacks the port fails here rather than at the first host that tries it. `pub fn run` is not one of the doors and is not exercised with a port. Runs free: the port is the scripted stub in `WO-MOK-025` and the canned connector in `WO-MOK-026`, and the case is the same case in both |
| `REQ-MOK-077` | automated test | **L30** the port is lent, not rebuilt | a replay of at least three ticks through the single-tick entry point consumes successive transcript records rather than the first record three times, and a stubbed live run's accumulated cost rises across ticks and trips a ceiling **set high enough that it is reached in a later tick and not in the first**. **The figure was "the cost of two exchanges" and that figure is withdrawn** *(amended 2026-08-24)*: an exchange is issued per acting Mokiterion, not per tick, so at a twelve-opportunity tick a two-exchange ceiling trips inside tick one — where a port rebuilt every tick trips it too, destroying this case's own discriminator. The ceiling is therefore derived from the run's arity and stated with the run: at `WO-MOK-025`'s configuration it is **eighteen**, one and a half ticks, reached in the second tick when the port is lent and never reached when it is rebuilt. The substance is what this case checks; the figure was illustrative and is now the runner's to compute. **A port rebuilt each tick passes neither half**: the cursor restarts and the accumulated cost stays at zero, so the ceiling never triggers. This is the case for the defect that compiles, runs and reports success |
| `REQ-MOK-077` | automated test | **L31** the observer replays | the observer, given this source and a committed transcript with no credential in the environment and no network reachable, advances to the transcript's horizon under operator control; the roster, map, event log, inspector, filter and export behave as they do under `social`, and the provenance footer names the fifth source |
| `REQ-MOK-077` | automated test | **L32** the observer refuses a live run | the observer given a connector path, a live-mode selection or a spend ceiling exits `2` before entering the terminal, names on standard error that this host replays only, starts no run and spawns no child process; given this source and **no** transcript it exits `2` and names the missing transcript. It never falls back to another source, and it never accepts the option and acts on nothing — which is what distinguishes this case from the `--events-path` defect GitHub issue 40 tracks |
| `REQ-MOK-077` | automated test | **L33** no port is an invalid configuration | this source selected with no port supplied refuses as an invalid configuration, in both hosts, rather than substituting a source, producing no decisions or applying the fallback of `REQ-MOK-074` |

## Acceptance scenarios

**A1 — a run nobody paid for.** With no credential in the environment and no network reachable, the model-backed source
runs to its tick limit from a committed transcript, produces the recorded run's bytes, and exits 0. This is the scenario
verification lives in, and it is the one that makes every other case affordable.

**A2 — the same decision from two directions.** One `attack` proposal arrives from `social` and the same `attack`
proposal arrives from a stubbed port at the same opportunity. The engine's records differ only in the source name.

**A3 — a transcript from the wrong run.** A transcript recorded at seed 0 is replayed at seed 1. The run fails at the
first opportunity whose recorded tick and Mokiterion do not match, names the mismatch, and produces no ticks beyond it.

**A4 — the money runs out.** A run declared with a ceiling reached at roughly half its tick limit stops there, with a
complete transcript, a complete record stream, a run record naming the ceiling and the tick reached, and an exit status
that is neither a clean completion nor an error.

**A5 — the model declines.** One exchange returns an action the request never enumerated. The Mokiterion waits, the
transcript records the response and the cause, the run record reports a fallback count of one and marks the run unfit,
and the run continues to its tick limit.

**A6 — a workflow tries to spend.** A pull request adds a provider key to a workflow's environment. The static check
fails, names the file and the line, and the build is red until the reference is removed.

**A7 — the observer watches, and declines to spend.** The observer is started with the model-backed source and a
committed transcript, and it presents the run through every pane it already has, tick by tick, under the operator's
control. Started instead with a connector path, it exits `2` before the terminal is entered and says that this host
replays only. Neither invocation makes a provider call, and the second starts no child process. This is the pair of
behaviours `REQ-MOK-077` exists for, and the second is the one a build is most likely to get wrong quietly, because the
observer forwards options it does not recognise to the engine's shared parser, which now accepts them.

## Property and invariant tests

- **P1 — the shared prefix is a prefix.** For every pair of requests in a transcript, the shared rules block is a common
  prefix. For every pair of requests for the same Mokiterion, the shared-plus-actor block is. Checked as a property over
  the whole transcript rather than on sampled pairs.
- **P2 — the enumeration is a subset of the admissible and a superset of the knowably-legal.** Every enumerated action
  is admissible under `SPEC-MOK-001` rule 6 at that opportunity, and every action admissible on grounds the observation
  carries is enumerated. The gap between the two — actions the engine may still reject on grounds block D cannot know —
  is exactly what case **L23** allows and is checked to be non-empty at least once, so that the distinction is
  exercised rather than assumed.
- **P3 — one request, one Mokiterion.** No request's attribute-bearing content mentions more than one identifier's
  attributes. Checked over every request of every retained transcript.
- **P4 — replay is idempotent.** A replay of a replay's inputs produces identical bytes. Two replays of one transcript
  compare equal.
- **P5 — the fallback count and the transcript agree.** The run record's fallback count equals the number of transcript
  records marked as fallbacks. Recomputability of every run-record figure from the transcript is checked for all of
  them, not only this one.
- **P6 — accounting is integer.** No figure the run reports contains a decimal separator, and no floating-point type
  appears in the accounting code. `SPEC-MOK-006`'s prohibition is honoured in the run record and the transcript alike.
- **P7 — the entropy stream is untouched.** Under the model-backed source, per-tick entropy draw counts equal those of a
  run with no decisions taken at all from the stream, and no draw is attributable to this source at any tick.

## Static and architecture checks

- **S1** The engine's and the observer's resolved dependency graphs equal the declared sets, unchanged from the base
  commit — the engine's table empty, the observer's one entry with every other crate reached transitively through
  `ratatui`. `ARCH-MOK-001`'s by-name scan for a network, asynchronous-runtime, database, model-provider or
  user-interface crate is re-run and continues to find none. **This check is what makes `ADR-MOK-007` decision 3 true
  rather than intended.** It is run over the engine **package**, whose `[[bin]]` target is the recording host, so a crate
  admitted for the host alone would be found here.
- **S2** The connector's dependency surface is **not** checked, and the absence is the finding rather than a gap in this
  contract. `SPEC-MOK-007` rule 10.6 withdraws the standard-library constraint an earlier draft placed on the provider
  program, because the connector is named by the operator and this repository neither builds it nor ships it, so no check
  here can see it. What **is** checked is the **canned connector** of rule 20.5 — the one connector this repository owns —
  against its own dependency declaration and against reaching no network. The report states plainly that this establishes
  nothing about an operator's connector, and `ADR-MOK-007`'s *Negative* consequences record the same limit. Verifying a
  claim about a program outside the repository is not possible, and pretending otherwise would be the worst kind of green.
- **S2a** No third workspace member, no third package directory, no connector source outside the canned one, and no
  connector path compiled into either package as a default. A default path would make a live run reachable without the
  operator naming anything, which is `REQ-MOK-072`'s gate defeated by a constant.
- **S3** The library target performs no filesystem operation, opens no socket, spawns no process and reads no
  environment variable, extending `ARCH-MOK-001`'s 2026-08-20 prohibition to the three new capability classes. **Both of
  this source's streams are covered**: the transcript it writes in live mode and the transcript it reads in replay both
  arrive as already-open handles, per `SPEC-MOK-007` rules 11.1 and 12.1.1. This check is what keeps `SPEC-MOK-001`'s
  *"The library target interprets no path at all and performs no filesystem operation"* a measurement rather than a
  memory.
- **S3a** The process spawn and the environment pass-through appear in the engine's binary target and nowhere else in
  either package. The observer's source contains no process spawn at all, which is `REQ-MOK-077`'s prohibition checked
  statically rather than assumed from its absence today.
- **S4** The added public surface is exactly one interface and one request type. No transport type and no dependency-owned
  type appears on it.
- **S4a** `SPEC-MOK-002` rule 5's mechanical drift checks are run **in their restated form**, and the restatement is
  present in the specification. Three greps: `pub fn execute` in `src/lib.rs` returning exactly one line, the record
  sink's parameter returning exactly one line, and the port's parameter returning exactly one line — with the failure
  conditions stated against five parameters rather than four. `grep -n 'pub fn .*&mut self' src/simulation.rs` returns
  exactly `run` and `advance_tick`, and `Simulation::run`'s enumerated form is unchanged. **This check exists because the
  standing text of check 2 reads "A fifth parameter … fails the second", and the port on `execute` is that fifth
  parameter**: a build that adds the port without the restatement is a build its own interface authority condemns, and
  the failure is silent because nothing else notices. It is separate from **S4** because S4 measures what was added while
  this measures that the drift detector still detects drift. `ADR-MOK-007`'s `SPEC-MOK-002` amendment row and
  `SPEC-MOK-007` rule 20.5.2 are the authorities. Runs free, offline, on every push.
- **S5** The usage text's fifth policy value is present in both hosts' texts, and the sentence *"None of the four learns
  anything or calls a model; all four are deterministic"* no longer appears in either. A usage text that contradicts the
  program is the first defect a reader meets, so it is checked rather than reviewed.
- **S5a** The observer's descriptions of the shared options remain byte-identical to `mokiterions::cli::USAGE`, which
  `mokiterions-tui/tests/options.rs` already holds. The fifth policy value's description and the transcript option's
  description are therefore the engine's words in both texts; what the observer states in its own words is only that this
  host replays only, which is the observer's own fact and not a shared input's meaning.
- **S6** The observer's authority mapping contains the fifth source, mapped to `REQ-MOK-063`, and its hard-coded
  four-source description is gone. The mapping is exhaustive by construction — the observer resolves it in a `match` over
  the policy — which is why `SPEC-MOK-003`'s 2026-08-19 amendment record treats a missing row as a compiler-visible gap.
- **S6a** The configuration value the library holds gains no field for either new path. Both are validated by the shared
  parser and discarded there, on the `--events-path` precedent, so a path cannot reach the simulation's rules by
  travelling inside the configuration. `SPEC-MOK-007` rules 10.9 and 18.4 are what this measures.
- **S6b** Neither host contains a live-versus-replay branch inside the library target, and the observer contains no live
  path at all: no ceiling parsing that reaches a run, no connector spawn, no credential read. `REQ-MOK-077`'s prohibition
  is checked as an absence, and an absence nobody looks for is indistinguishable from an oversight.
- **S7** The shared rules block exists in exactly one place in the source, so that case **L27**'s assessment has one
  object and a drift between two copies is impossible.

## Security and privacy checks

- **C1** No credential appears in any transcript, any record stream, any run record, any authorization record, either
  output stream, or any error message. Checked by pattern over retained evidence and by a test that sets a synthetic
  credential value and asserts it appears in no produced byte.
- **C2** No workflow file references a model-provider credential and none selects live mode — case **L21a**.
- **C3** The credential is read from the process environment and from nowhere else: no file, keychain or configuration
  directory is opened in the search for one.
- **C4** No request carries any data about any Mokiterion other than the one deciding — cases **L12** and **L13**. This
  is checked as a privacy property of the population as well as an experimental one.
- **C5** What leaves the repository in a live run is the request text only. No source, no path, no repository content and
  no identity beyond a Mokiterion identifier appears in any request.
- **C6** The attestation that the credential is not configured in the repository's automation secrets. No check can see
  this; it is the repository owner's statement, retained with the evidence, and it is the single fact the whole cost
  containment rests on.

## Performance and resilience checks

- **R1** A transport failure is retried a bounded number of times and each attempt appears as its own transcript record.
  Verified against a stubbed port that fails a fixed number of times.
- **R2** Exhausted retries produce a counted fallback and the run continues. A run of an **estimated** 10,954 exchanges
  that ended on its first timeout would be an instrument nobody could use, so continuing is checked rather than
  tolerated.
- **R3** A failure to write the transcript ends the run with an error status. A live run whose exchanges were spent and
  not recorded produced cost and no evidence, and it is the one failure worth aborting for.
- **R4** Replay of a 1,000-tick transcript completes without loading the whole transcript into memory at once. The
  transcript is an **estimated** 4.7 MB, so this is a modest bound rather than a demanding one, and it is stated so that
  a streaming read is a requirement of the design rather than an accident of the file being small.
- **R5** No latency, throughput or wall-clock figure is a pass condition anywhere in this contract. A live run takes an
  **estimated** 1.2 to 2.4 hours and that is a property of the provider. Recording a timing threshold would make this
  contract fail for reasons outside the repository.

## Manual assessments

Three, and each is here because no check can make it.

**M1 — the shared rules block carries no strategy.** Case **L27**. The assurance owner reads the block against
`SPEC-MOK-007` rule 4.4's prohibitions — no goal, no preference, no objective, no advice — and against `SPEC-MOK-001`
for accuracy. This is the assessment on which `ADR-MOK-007` decision 7 depends: a block that told the model to survive
would make every figure a measurement of the instruction. An assessment that has not been made leaves this contract
unsatisfied.

**M2 — the shared rules block agrees with `SPEC-MOK-001`.** The block is a restatement, and `SPEC-MOK-007` rule 4.2
fixes which governs, but nothing detects drift automatically. The assessment is made whenever the block or
`SPEC-MOK-001`'s rules change, and check **S7** is what keeps it to one object.

**M3 — the published comparison is honest.** Case **L24**. The assurance owner confirms that `reference` and `social`
were re-run at the declared seeds and horizon rather than quoted from their own longer runs, that no figure carries a
threshold, and that no seed's absence is unexplained.

## Evidence retention

Retention is decided here, before the first live run, because a transcript becomes provenance the moment a verification
record binds it and **a bound evidence path can never be corrected afterwards** — renaming the directory forces a whole
fresh capture, and a verified record has no rebind.

| Evidence | Retained where | Size |
|---|---|---|
| The canned transcript automation replays | Committed under the evidence path the work order names | **estimated** 100 to 260 KB for 20 to 50 ticks |
| Base-commit captures of the four existing sources | Committed under the work order's evidence path | as the existing capture set |
| Each authorised live run's transcript | Committed under that measurement's evidence path, one directory per run | **estimated** 4.7 MB per 1,000-tick run |
| Each live run's record stream and run record | Beside its transcript | as the existing record streams |
| The authorization record | Beside the run or runs it authorises | one file |
| The credential attestation | Beside the measurement's evidence | one file |

Three retention decisions are stated rather than left to the work order:

1. **A transcript is never truncated to fit.** Its size is bounded by choosing the horizon, not by abbreviating the
   record. `SPEC-MOK-007` rule 11.7.
2. **The evidence directory name is fixed before the first capture**, because it is provenance. A measurement over five
   seeds at an **estimated** 23 MB is the largest evidence set this repository will have retained, and getting the path
   wrong costs the whole capture.
3. **`docs/engineering/simulation/evidence/**` is exempt from end-of-line conversion**, so every retained digest here
   reproduces on any platform. This matters more for this contract than for previous ones, because a transcript is
   compared with `cmp` rather than parsed.

## Residual uncertainty

- **The provider can change under us.** `gpt-5.6-luna`'s prices, its caching behaviour and its minimum cacheable prefix
  are the vendor's. Case **L15b** fails against a layout that was correct when written, and that is the intended
  behaviour: it is a signal to re-measure and bring the layout or the floor back to the owner, not a reason to soften
  the number in place. This contract cannot distinguish a regression in the repository from a change at the provider,
  and the transcript's retained usage figures are what a person uses to tell them apart.
- **Five cases cannot be satisfied by a build.** **L15b**, **L24** and **L25** need an authorised live run; **L27** is
  the assurance owner's assessment, and **M2** and **M3** sit beside it; **L28**'s static half is checkable and its
  genuineness is not. **L21a** *is* fully checkable, but **C6**'s attestation behind it is not, and that attestation is
  the single fact the whole cost containment rests on — it is outside the matrix because it is outside the repository.
  A green build is not a satisfied contract here, which is a weaker position than any previous contract in this
  repository has been in, and it follows directly from the cost decision rather than from a shortcut.
- **Cases L4, L5, L12, L13 and L14 are only as good as the transcript they read.** They check the requests a particular
  run produced. A code path that composes a different request under a configuration no retained transcript covers is not
  reached. The mitigation is that the committed transcript covers a run in which every Mokiterion acts, targeted actions
  are enumerated, food is and is not co-located, and at least one Mokiterion dies — but it is a mitigation, not a proof,
  and it is stated so that a later reader knows what the coverage rests on.
- **No case establishes that the model understood anything.** A well-formed enumerated response is the whole of what is
  checkable. Whether the decisions are reasonable, coherent over a run, or better than random is not verified, is not
  asserted anywhere, and would require an outcome oracle this contract deliberately does not have.
- **`SPEC-MOK-006`'s outstanding 2026-08-21 amendment row interacts with this work.** The `schema` increment
  `ADR-MOK-007` requires is to one more than whatever that row's ratification leaves standing. This contract measures
  the value in the tree at the candidate commit rather than assuming a number, and if the row is still outstanding when
  a live measurement happens, the retained record streams carry a version whose specification is unratified. That is a
  disclosed condition of the evidence, not a defect in it.
