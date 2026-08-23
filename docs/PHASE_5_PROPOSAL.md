# Phase 5 Proposal — LLM Decision Source

> **Authority note.** This document is repository-owned functional planning, in the same class as
> `docs/ROADMAP.md` and `docs/PHASE_4_PROPOSAL.md`. It is **not** approved product intent, a requirement, an
> architecture decision, work authorization, a verification contract, or release authority. Only formal artifacts
> under `docs/engineering/` carry that authority, per `ENGINEERING_HARNESS.md`. Nothing here authorizes
> implementation. Every identifier proposed below is a proposal for an identifier, not a claim on one.

- **Created:** 2026-08-23
- **Proposes:** the artifact packet for `ROADMAP.md` Phase 5, a three-stage split, and the decisions still open
- **Audience:** product owner, technical owner, assurance owner, engineering owner
- **Base:** written against `master` at `2a93914`. Every figure below is either measured at that commit and labelled
  *measured*, taken from vendor documentation and labelled *published*, or an estimate and labelled *estimate*.
  The three are not mixed.

## Revision record

| Date | What moved |
|---|---|
| 2026-08-23 | Created. |
| 2026-08-23 | **Five owner decisions recorded** (see below) and the document rewritten around them. Isolation and reasoning level move from open questions to fixed inputs. Cache optimisation becomes a measurable requirement with a stated threshold, and the cost figures are restated for a cache-ordered prompt. The recommendation that the LLM policy get "its own floor at its own horizon" is **withdrawn**: it has no outcome floor at all, and the verification section is rewritten to hold the instrument rather than the result. A run-authorisation and cost-containment section is added. Two findings reduce the amendment set rather than growing it: the four existing outcome floors name their sources explicitly and so bind no fifth source, and the dependency prohibitions are package-scoped, so with the recommended transport nothing in them is relaxed. |

## Decisions recorded

Taken by the repository owner on 2026-08-23, before the packet is drafted. They are inputs to everything below, not
proposals.

| # | Decision |
|---|---|
| 1 | **Isolation is one fresh context per decision**, as a first step. Nothing accumulates across calls. |
| 2 | **A shared cacheable preamble is permitted**, and **reducing cost through the cache is itself a requirement** — not an optimisation left to an implementer. |
| 3 | **Reasoning level is `none`**, at first. |
| 4 | **The LLM policy carries no outcome floor.** Observing what happens is the point, so the existing viability constraints are relaxed for it. |
| 5 | **No real provider call happens in CI or in any automated test.** Not on push, not on release, not in a unit test. A real run needs the repository owner's explicit permission. |

## Recommendation in one paragraph

Do not put an HTTP client inside `mokiterions-core`. Give the engine a **decision port** — a line-oriented
request/response protocol carried on caller-supplied streams, exactly the shape `execute` already uses for the
`--events-path` record sink — and let a separate provider program own the socket and the credential. The engine's
declared dependency set stays empty, `Observation` stays private, and **replay costs nothing extra**: a recorded
transcript is a file that replaces the provider. Treat the transcript as a run *input* alongside the seed, which
leaves `REQ-MOK-009`'s reproducible-entropy obligation untouched and narrows the determinism amendment to one sentence
of `INT-MOK-001`. Order the prompt so the cacheable part comes first and the variable part last, and hold the
resulting cache hit ratio to a stated threshold read from the provider's own usage figures — that is decision 2 made
checkable, and it brings a five-seed measurement to an estimated $5.18. Split delivery in three: the port and replay
first, offline and verifiable with no API key at all; then the provider program; then the measurement. Because the
policy has no outcome floor, verification holds the **instrument** — protocol conformance, replay fidelity,
non-perturbation of the other four policies, isolation, cache efficiency, zero fallbacks, spend inside a declared
ceiling — and records survivors and deaths as observations with no pass or fail attached.

---

## Part 1 — Challenging the framing

Seven findings. Four are cheap to fix once named. Two remove work the roadmap implies is necessary. One is the trap
most likely to produce a confidently wrong result.

### 1. There is no boundary to plug into. The roadmap's own sentence is not achievable as written

`ROADMAP.md` Phase 5 says: *"Replace the baseline with a model-backed decision source at the **existing** trust
boundary."* Measured in `mokiterions-core/src/simulation.rs`:

| Item | Line | Visibility |
|---|---:|---|
| `trait DecisionSource` | 947 | **private** |
| `struct Observation` | 652 | **private** |
| `fn advance_tick_with_source` | 2130 | **private** |
| `fn advance_tick` | 2101 | public, but its body is a `match` over four hardcoded policies |

Nothing outside the crate can supply a decision, and `SPEC-MOK-003` keeps it that way deliberately — it names
`Observation` and `DecisionSource` as two of the ten types that stay private "which carry the `ADR-MOK-001` trust
boundary." So Phase 5 cannot be an additive plug-in. Something opens, and the packet has to say which thing and why.

**The useful part of this finding is how little needs to open.** `SPEC-MOK-003` already narrowed rule 6 "from a list
of type names to the capability it exists to deny" — no mutable borrow of, and no reference into, engine state.
`Observation` is entirely owned values (`String`, `Vec`, `u8`). Publishing a *serialized projection* of it denies that
capability just as completely as hiding it does. What moves is the named-exclusion list, not the invariant. That is a
much smaller argument than "we opened the trust boundary," and it should be made in those terms.

### 2. `valid_actions` is the wrong menu to show a model, and using it would silently disable every social verb

`Observation.valid_actions` looks like the obvious "here are your legal moves" field to put in a prompt. It is not.
Its own doc comment (line 620) says so:

> **A targeted action never appears here** … rule 4's baseline consumes one entropy selection over this list's
> length, so a longer list would move that selection and diverge every run ever recorded under `baseline`. The
> consequence is that this list is no longer everything a source may legitimately propose — rule 6 is — and a reader
> who takes it as the whole contract will be wrong about the social source.

A prompt built from `valid_actions` would offer `wait`, `sleep`, `eat`, `move` and nothing else. The model would never
propose `attack`, `threaten`, `fight`, `retreat`, `surrender`, `approach` or `avoid`, and the run would look like a
verbose `reference` policy. This is the single most likely way to build Phase 5 and get a confidently wrong result.
The prompt must present the **rule 6** set, computed for the port, and a requirement should state that.

### 3. Cost is not the blocker, and with decision 2 applied it is not close to one

`ROADMAP.md` defers Phase 5 partly because it "defers the only expensive, nondeterministic, credential-bearing work."
Expensive is not supported by the numbers.

**Measured** at `2a93914`, seed 0, default density 0.75, via `--trace-actions`:

| Run | Decision opportunities |
|---|---:|
| 1,000 ticks, `--policy social` | **10,954** |
| 100 ticks, default `reference` | **1,200** |

One decision opportunity is one LLM call. **Published** for `gpt-5.6-luna` (retrieved 2026-08-23): $0.20 per MTok
input, **$0.02 per MTok cached input**, $1.20 per MTok output, cache writes billed at 1.25× uncached input, 1.05M
context, 128K max output, reasoning levels none / low / medium (default) / high / xhigh / max, structured outputs and
function calling both supported.

**Estimate** for the per-call token shape. The unoptimised rows assume a ~1,200-token rules preamble and a ~350-token
variable block; the optimised row is the cache-ordered layout of Part 2, which moves the action grammar into the
cached prefix and leaves ~200 variable tokens. Output is ~25 tokens for one action in every row.

| Configuration | Per call | Per 1,000-tick run | Five declared seeds |
|---|---:|---:|---:|
| reasoning `none`, **cache-ordered layout** *(recommended)* | $0.0000946 | **$1.04** | **$5.18** |
| reasoning `none`, preamble cached, layout not ordered | $0.000124 | $1.36 | $6.79 |
| reasoning `none`, no caching at all | $0.00034 | $3.72 | $18.60 |
| reasoning `low`, preamble cached | — | $4.64 | $23.20 |

Cache writes add an estimated **$0.004** per run — twelve prefixes written once each, billed at 1.25× uncached input.
That figure only holds while entries stay warm; point 5 explains why they do.

A full five-seed measurement costs less than a sandwich. **The binding constraint is wall-clock latency, and it is
structural rather than budgetary** — see the next point. That does not make decision 5 unnecessary: a runaway loop, a
retry storm or an accidental 10,000-tick horizon are what a spend ceiling exists for, and none of them are priced by
the table above.

### 4. Decisions cannot be parallelised within a tick, and that fixes the wall clock

Within one tick, agents act in sequence and each observes the state the previous one left. `Observation.fear`'s doc
comment is conclusive: the value carried is *"the one standing at the start of this opportunity — after the previous
tick's rule 12 write, plus any threat applied by an earlier-acting Mokiterion in this tick."* Food eaten by an
earlier agent is gone; positions have moved. Issuing twelve concurrent calls from the tick-start state would change
the simulation, not merely accelerate it.

So the 10,954 calls are a chain. At an **estimated** 0.4–0.8 s per round trip at reasoning `none`, that is roughly
**1.2–2.4 hours per 1,000-tick run**; at reasoning `low`, plausibly 4–9 hours. Two consequences:

- **Parallelise across seeds, never within a tick.** Five seeds as five processes gives a five-seed measurement in
  the wall-clock time of one run.
- **A provider call in CI was never affordable in time either.** All 343 tests currently pass offline in about 70
  seconds, and the viability suite alone takes 36.5 s. Decision 5 forbids it on cost; latency would have forbidden it
  anyway. Replay keeps CI offline by construction.

Latency is the one figure here that cannot be computed from documentation. Stage 5b below exists partly to measure it
before anyone commits to a horizon.

### 5. Isolation: the decision is taken, and the naive cache worry does not apply

Decision 1 fixes isolation at **one fresh context per decision**. For the record, the three readings and what they
cost, because the packet will be read by people who did not see the question:

| Reading | What it means | Cost per 1,000-tick run |
|---|---|---:|
| **(a) No cross-agent information** | An agent's prompt never contains another agent's private state | free — already enforced |
| **(b) Fresh context per decision** — **decided** | Each call is its own context; nothing accumulates | $1.04 *(estimate)* |
| **(c) Persistent per-agent conversation** — rejected | Each agent keeps one context that grows for the whole run | ~$36 cached, ~$352 uncached *(estimate)* |

**(a) is already enforced by the engine and needs no work.** `PerceivedMokiterion` carries only `id`, `direction`
and `distance` — its doc comment states it carries "no attribute of the perceived Mokiterion — not its `health`, its
`energy` or its `fear`." `REQ-MOK-059` already forbids reading any population-level aggregate, and the roadmap
records that every read in every rule and source was enumerated against it. **The prompt contract is therefore
already governed**: whatever is in `Observation` may go in the prompt, and adding a field is a specification
amendment. That is a genuine asset and the packet should lean on it.

**(c) was rejected, and cost is the weaker of the two reasons.** The stronger one: under (c) the state that decides
behaviour lives in a vendor's context window, so it appears in no event, no metric and no record-stream line. That
contradicts the project's own observability goal — *"the simulation should make important behavior reconstructable"* —
and it would make Phase 6's emergence evaluation an argument about material nobody retained. It also silently
reverses a deliberate design choice: this system has **one tick** of memory, the `suffered` record, and `ROADMAP.md`
records that a longer window was refused because "a longer window is a stored relation and the contact rule is
deliberately positional." If per-agent memory is wanted later, it should be **engine-owned, bounded, and emitted in
the record stream** as its own requirement — not acquired as a side effect of how a provider client was written.

Decision 2 permits the shared preamble. It carries no agent information by construction: it is byte-identical for
every agent and every tick, so a prompt that contains it learns nothing about anyone. **And the obvious objection to
caching under fresh-per-decision isolation is wrong.** The worry is that twelve interleaved agents evict each other's
cache entries. They do not: one tick is twelve calls of roughly 0.5 s, so **each agent's prefix is re-touched about
every 6 seconds** for the whole run — far inside any plausible retention window. Round-robin ordering keeps twelve
entries warm rather than thrashing them.

### 6. Nothing needs relaxing to give the LLM policy no floor — the floors are already source-scoped

Decision 4 says the existing viability constraints must not bind the new policy. Checked against the artifacts, **no
amendment is required to achieve that**, because every outcome obligation names its source in its own `SHALL`
statement:

| Requirement | What it binds | Statement names |
|---|---|---|
| `REQ-MOK-014` | 8 of 12 survivors at 1,000 ticks | *"using the **reference** decision source"* |
| `REQ-MOK-034` | 8 of 12 survivors at 1,000 ticks | the **trait-aware** source |
| `REQ-MOK-058` | 5 of 12 survivors plus one combat death | *"using the **social** decision source"* |
| `REQ-MOK-060` | no class above three fifths of a territory's resources | *"under the **reference, trait-aware or social** decision source"* |

A fifth source is outside all four. The same is true of the tests: `mokiterions-core/tests/viability.rs` holds three
separate functions, each naming its own `Policy` variant, so **a fifth policy simply has no viability test** and the
file needs no edit. `baseline` is the existing proof that a policy can carry no floor — it goes extinct between ticks
119 and 193 on every declared seed, recorded as measurement and never as failure.

**What decision 4 does require is a positive statement, and that is the real risk.** Four sources in a row received a
floor. If the packet is silent, a later reader will add one by analogy, or will read the absence as an oversight and
"fix" it. So the new capability must say, in its own words, that this source carries no outcome obligation, and say
why: the observation is the deliverable, and an obligation on the observation would bias what gets built toward
meeting it. `REQ-MOK-034`'s 2026-08-20 amendment is the drafting precedent for the tone — it narrowed a
frozen-outcome constraint and stated explicitly that *"the floor of eight of twelve is not touched by this row"*,
so that nothing was left to inference.

### 7. The dependency rules do not need reconsidering either — they are package-scoped, and one of them is already withdrawn

The brief said previous rules may be reconsidered, giving "crates cannot contain third-party dependencies" as the
example. That rule **was already withdrawn on 2026-08-20** by `ADR-MOK-006`, which replaced the engine's
empty-dependency-table rule with a per-package declared set. Nothing needs to be reopened there.

The prohibitions that survive are the ones that matter here, and their scope is the whole finding:

- `ARCH-MOK-001` *Prohibited patterns*: *"Network calls, API credentials, asynchronous runtimes, databases, UI
  frameworks, plugin systems, or dependency injection containers **in the engine package**."* Engine-scoped.
- `REQ-MOK-050`: the resolved set must equal the declared set and contain no crate providing network access or
  credential handling *"in **either** package"* — the engine and the observer, the two that exist.
- `ARCH-MOK-001` conformance check: confirm by name that the engine's graph contains no network, asynchronous-runtime,
  database, **model-provider** or user-interface crate.

Two things follow. First, a model-provider crate inside `mokiterions-core` fails a named conformance check — the
design must keep it out, which is what Part 2 does. Second, **`ARCH-MOK-001`'s rationale already decided the shape**:
it *"keeps future provider credentials outside the engine … and defers an external model provider to an adapter at the
same boundary."* The port is not a departure from the architecture; it is the thing the architecture said would
happen.

The live question is where the adapter lives, and it decides whether `REQ-MOK-050` moves at all. See Part 2's
transport section and Part 3 decision 1.

### 8. Two traps in how success and failure get measured

**The silent-fallback trap.** If a call fails or returns an unparseable action and the port falls back to
`reference`, the run measures `reference` and labels it LLM. Proposal: a fallback is **counted, reported in the `run`
record, and disqualifying** — a run with fallbacks greater than zero may not source a published figure. Malformed but
*parseable* proposals need no new mechanism; the engine already validates and rejects, and rejected proposals are
already a counter in the inspector's activity profile.

**The success-by-assumption trap, which decision 4 makes sharper rather than softer.** With no outcome floor, nothing
in Phase 5 would fail if the LLM behaved indistinguishably from `reference`. That is acceptable **only if the
comparison is made and published either way.** The packet should require a falsifiable comparison against `reference`
and `social` on the same seeds and the same measured axes, with "indistinguishable on all measured axes" as a real,
reportable result. Removing the floor removes an obligation on the *world*; it must not remove the obligation to
*report honestly* on what was seen.

### 9. Two smaller facts worth having before drafting

- **`Action` has no parser.** It has `Display` (line 609), and targeted verbs render as the bare verb because the
  target is a separate field on the trace line. A round-trip parser is new code, and the wire format must carry verb
  and target separately.
- **`decide` cannot fail.** Its signature is `-> Action` with no `Result`. A network call can fail, so the port needs
  a fallible signature. `advance_tick` already returns `Result<TickOutcome, String>`, so the plumbing above it exists.
- **Temperature and seed support for `gpt-5.6-luna` are not documented** on the model page. Do not design determinism
  around either. This is why the recommendation puts determinism in the transcript rather than in provider parameters
  — which is also what `ROADMAP.md` already concluded when it noted temperature 0 "is not a bitwise determinism
  guarantee from any provider."

---

## Part 2 — The proposal

### The decision port

The engine gains a fifth policy that does not decide anything itself. It writes one request line describing the
observation, reads one response line naming an action, and validates that action exactly as it validates the other
four policies' proposals.

```text
  mokiterions-core (library)                      provider program (outside the engine)
  ┌───────────────────────────┐                   ┌───────────────────────────────┐
  │ rules, state, validation  │  request line     │ one fresh context per         │
  │ no network, no async,     │ ────────────────► │ decision                      │
  │ no credentials,           │                   │ blocking HTTPS → gpt-5.6-luna │
  │ empty dependency set      │ ◄──────────────── │ writes the transcript         │
  └───────────────────────────┘  response line    └───────────────────────────────┘
             ▲                                                    │
             │                                                    ▼
      replay: the same two streams, fed from a recorded transcript file
```

Both streams are supplied *by the caller*, as `&mut dyn Write` and `&mut dyn BufRead`. This is not a new pattern in
this repository — it is the pattern `execute` already uses:

```rust
pub fn execute<I, S, W, E>(args: I, stdout: &mut W, stderr: &mut E,
                           records: Option<&mut dyn Write>) -> u8
```

`WO-MOK-019` established the precedent explicitly: the library "resolves no path, opens no file, creates no directory
and removes none," and the binary owns the destination's whole lifetime. Phase 5 mirrors it one boundary further out
— the library owns no socket and no credential, and the binary owns the transport it was handed.

**What this buys, all of it structural rather than promised:**

| Property | Why it holds |
|---|---|
| Engine dependency set stays **empty** | A trait and two stream parameters need no crate |
| No async runtime anywhere | Blocking request/response on a stream; one of `ADR-MOK-006` decision 4's prohibitions survives untouched |
| No model-provider crate in the engine graph | `ARCH-MOK-001`'s by-name conformance check passes unchanged |
| `Observation` stays private | A serialized projection crosses, on `SPEC-MOK-006`'s precedent |
| `ADR-MOK-001` intact | The engine still validates every proposal and owns all mutable state |
| **Replay is free** | A transcript file substituted for the provider; no second code path, no mode flag inside the rules |
| CI stays offline | Every test drives the port from a canned transcript |
| Isolation is structural | The engine hands over one observation at a time; nothing in the engine can accumulate context, so decision 1 cannot be violated by a provider-side mistake |

Reuse `SPEC-MOK-006`'s existing constraints for the transcript format — one record per line, no floats, no
timestamps, closed value alphabet, byte-comparable between runs. That makes a transcript diffable evidence rather
than a log, and it means two runs can be compared with `cmp`.

### Where the provider lives, and the one dependency question it decides

Finding 7 leaves exactly one structural choice. Both options keep the engine clean; they differ in whether the
workspace acquires a network and credential surface.

| | **A — third Rust package** | **B — separate provider program** *(recommended)* |
|---|---|---|
| Transport | in-process trait, package linked into the binary | child process; engine speaks the same two streams over its pipes |
| Rust crates added | a blocking HTTPS client and TLS — an estimated 40–60 resolved crates | **none**; `std::process` and `std::io` only |
| `REQ-MOK-050` | **amended** — *"in either package"* no longer describes the workspace, and network plus credential handling must be admitted for the new one | **untouched**; the workspace still contains no network or credential crate |
| `ADR-MOK-006` decision 4 | a crate admission, with decision 1's criteria applied by the technical owner and recorded | no admission; nothing enters a declared set |
| Credential | inside the workspace's build, outside the engine | never in any Rust code |
| Honest cost | a real supply-chain surface, reviewable but present | the provider program has its own dependencies, **outside** the declared-set discipline. Constrain it to its language's standard library — the API is one HTTPS POST with a JSON body — or the surface moves rather than disappearing |
| Also needed | new package spec; `SPEC-MOK-004` rules 1–3 and its test census | architecture governance for a non-Rust product component, and a decision on how its tests enter `SPEC-MOK-004`'s census |

B is recommended because it is the option this repository's own stated values pick. `ADR-MOK-006` argued for
*"auditability by construction … checkable by reading one manifest,"* and B leaves both manifests exactly as they are.
It also makes the strongest claim available — *the workspace cannot make a network call* — a property of the tree
rather than a promise about a code path. Neither option is free of governance work; B's is an architecture decision,
A's is an architecture decision **plus** a requirement amendment **plus** a crate admission.

### Cache optimisation, as a requirement rather than an intention

Decision 2 makes cost reduction an obligation, so it needs a checkable form. Three parts.

**1. Cache-ordered prompt layout.** Provider prompt caching is prefix-based: the longest byte-identical prefix hits
cache. So the prompt is laid out most-stable-first, and a single early variable byte is what a violation looks like.

```text
  ┌─────────────────────────────────────────────┬─────────┬────────────────────┐
  │ block                                       │ ~tokens │ varies with        │
  ├─────────────────────────────────────────────┼─────────┼────────────────────┤
  │ 1  world rules, action grammar, reply shape │  1,200  │ nothing            │
  │ 2  this agent's constants: id, name,        │     30  │ agent (fixed for   │
  │    waste_tolerance                          │         │ the whole run)     │
  ├─────────────────────────────────────────────┼─────────┼────────────────────┤
  │ 3  tick, position, attributes, suffered,    │   ~200  │ every call         │
  │    perceptions, legal targets               │         │                    │
  └─────────────────────────────────────────────┴─────────┴────────────────────┘
     blocks 1–2 are the cached prefix; block 3 is never cached
```

Block 1 is byte-identical across all twelve agents, which is why decision 2 costs nothing in isolation terms: the
same text is cached twelve times over and carries no agent's information in any of them.

**2. A stated threshold, read from the provider's own numbers.** Cached input tokens ÷ total input tokens, taken from
the API response's `usage` field, **at or above 0.85** over a run. The layout above gives an estimated 1,230 of
~1,430, or **86%** — margin of one point, which is deliberately thin: the threshold should fail if the layout
regresses, and a regression is exactly what a stray timestamp or a reordered field produces. This is a measurement of
the provider's behaviour, not of our intentions, and it is the kind of obligation `verification_method =
"static-analysis"` cannot carry — it needs a measured run, which puts it in stage 5b.

**3. Per-call usage recorded in the transcript.** Prompt tokens, cached prompt tokens, output tokens, reasoning tokens.
This is what makes part 2 auditable after the fact rather than a number someone reported, and it turns the transcript
into cost evidence as well as behaviour evidence. It costs bytes and nothing else.

**One sub-decision, and it is a real trade-off rather than free.** The largest variable block today is the legal-action
enumeration: 4 core verbs, up to 8 `eat` options, 4 `move` directions, and 7 targeted verbs for each perceived
neighbour — 150 tokens and up, all of it in the uncached block. Stating the action grammar once in block 1 and using
**structured outputs** (published as supported) to constrain the reply to a verb plus a target drawn from the
perception block removes most of it. The cost is a likely higher rejection rate, because the model is no longer handed
the pre-filtered legal set. That is a behavioural cost paid for a cost saving, so it should be **measured in 5b and
decided on the measurement** — rejection rate and cache ratio both, side by side — not asserted here. If the rejection
rate is unacceptable, the fallback is to enumerate targeted verbs only, keeping `eat` and `move` in the grammar.

### Determinism: amend one sentence, not the requirement

Reframe the transcript as a run **input**, like the seed. Then:

- **`REQ-MOK-009` (reproducible entropy) needs no amendment.** The engine's entropy stream is untouched; given
  identical inputs the run is still byte-identical. This is the main prize and it is worth arguing for carefully.
- **`INT-MOK-001`'s success measure does need one.** It currently promises 100% identical results for repeated runs
  at an identical *seed*. Under Phase 5 the determinand is *seed plus transcript*. That is one sentence.
- A live run is a **recording** run; a verified run is a **replay** run. The transcript is retained as commit-bound
  evidence, which makes it provable which model outputs produced a verified run.

### Non-perturbation, on `WO-MOK-020`'s precedent

The four existing policies must be byte-identical after this change: same events, same stream bytes, same per-tick
entropy draw counts, same exit codes, same summary figures. `WO-MOK-020` proved exactly this property for an observer
change and its evidence layout can be reused wholesale.

### Run authorisation and cost containment

Decision 5 forbids a real provider call in CI and in automated tests, and requires the owner's explicit permission
for a real run. Written as a rule it is a prohibition someone can forget. Written as a capability it is a thing the
build **cannot do**, and that is how it should be recorded.

**The mechanisms, in the order of how hard they are to bypass:**

1. **The provider key is never added to GitHub Actions secrets, and that is stated as a requirement.** CI then cannot
   bill regardless of what any workflow file says, what a future contributor adds, or what a test invokes. This is the
   only one of the four that survives an honest mistake, and it costs nothing to adopt.
2. **Live mode requires an explicit flag *and* a key in the environment.** Absent either, the port replays or refuses;
   there is no path where a default produces a call.
3. **A declared spend ceiling the provider program refuses to exceed**, tracked from the `usage` figures it is already
   recording, and reported in the run record. Finding 3 shows the expected spend is dollars; the ceiling exists for
   the loop that was not expected.
4. **An owner-permission record retained in evidence for every live run**, naming the horizon, the seeds and the
   ceiling authorised. This is the artifact that makes "explicit permission" auditable after the fact rather than a
   remembered conversation.

**One correction the owner should have before settling the test tiers: replay costs $0 and needs no network.** A
recorded transcript is a file. Excluding the LLM policy from automated tests *entirely* therefore buys no saving and
leaves the new mechanism — the parser, the wire format, the fallback counter, the rejection path — with zero automated
coverage, which is where the defects will be. But there is a real reason not to put *full* transcripts in the
repository: an **estimated** 4.7 MB per 1,000-tick transcript, ~23 MB for five seeds, and `VREC` evidence paths are
permanent provenance — a transcript committed once cannot be removed from the record. So the size, not the cost, is
what should shape the tiers:

| Tier | What runs | Network | Cost | When |
|---|---|---|---|---|
| **1** | short canned transcript, 20–50 ticks, **estimated** ~100–260 KB, committed | none | **$0** | every push and PR, like any other test |
| **2** | full recorded-run replay from a retained transcript | none | **$0** | on demand; transcript kept as evidence, not in the source tree |
| **3** | live billed run against `gpt-5.6-luna` | yes | metered | **explicit owner permission only**; never triggered by push, PR, merge, tag or release |

Tier 1 is the only one that touches decision 5's boundary, and it does not cross it: no call, no key, no network. It
is offered as a correction to state once, not an argument to press — if the owner prefers the LLM policy to have no
automated coverage at all, that is a coherent position and the packet should record it as chosen rather than as
overlooked.

### Verifying a policy with no outcome oracle

Decision 4 removes the result from verification. It does not remove verification: what is held is the **instrument**.
Eight checks, every one of them decidable offline except the two marked:

| | Check | Where |
|---|---|---|
| 1 | Protocol conformance — every request well-formed, every response parsed or counted as a fallback | tier 1 |
| 2 | Replay fidelity — same seed and transcript give byte-identical text stream, record stream and summary | tiers 1–2 |
| 3 | Non-perturbation — the four existing policies unchanged, including per-tick entropy draw counts | tier 1 |
| 4 | Isolation — no agent's prompt contains another agent's attribute, or anything derived from another agent's prompt or response | tier 1, over a retained transcript |
| 5 | Cache efficiency — cached ÷ total input tokens ≥ 0.85, from `usage` | **tier 3** |
| 6 | Legality and rejection rate — proposals validated by the engine as always; rate reported | tiers 1–3 |
| 7 | Fallbacks — counted, reported, and **disqualifying above zero** | all tiers |
| 8 | Spend — actual against the declared ceiling | **tier 3** |

**And what is deliberately not verified: survivors, deaths, food composition, class shares.** Those are recorded as
observations, with no pass or fail attached, and they become Phase 6's material. This absence must be written into the
verification contract **as a decision with its rationale**, not left as a gap in a table. A `VER` artifact that simply
omits the viability check reads, to a later assurance owner, exactly like one where somebody forgot.

### Three stages, three work orders

**Stage 5a — the port, entirely offline.** The fifth policy, the wire format, the `Action` parser, the transcript
reader, the fallback counter, the canned tier-1 transcript. No third-party crate, no network, no credential, no API
key. Verifiable and mergeable on its own: a run driven from a canned transcript, byte-identical on repeat, plus the
non-perturbation evidence for the other four policies. **This is the majority of the risk and none of the cost.**

**Stage 5b — the provider program.** Fresh context per decision, cache-ordered prompt, structured outputs, transcript
with per-call usage, spend ceiling, the flag-plus-key gate. Ships with a short live smoke run under an owner
permission record, whose purpose is to **measure** what this document estimates: latency, real token counts, the cache
ratio against its 0.85 threshold, and the rejection rate for the action-grammar sub-decision. Every estimate above is
replaced before any horizon is fixed. Whatever `REQ-MOK-050` and architecture work Part 3 decision 1 selects lands
here, not in 5a.

**Stage 5c — the measurement.** Declared seeds at the agreed horizon, transcripts retained as evidence, the
falsifiable comparison against `reference` and `social`, and the observations published with no floor attached.

### Proposed artifact packet

**Identifiers below are proposals only.** Per this repository's own practice, every one must be re-checked against
every remote ref immediately before the packet is created — the local maximum is not the free number, and other
sessions are working on this repository concurrently. Local maxima at `2a93914`: `INT-MOK-010`, `CAP-MOK-010`,
`REQ-MOK-062`, `SPEC-MOK-006`, `ADR-MOK-006`, `VER-MOK-017`, `WO-MOK-024`, `VREC-MOK-023`. Note that
`WO-MOK-015`, `WO-MOK-021` through `WO-MOK-023`, `VER-MOK-009` and `VER-MOK-015` have no file here and may be
spent elsewhere.

**New:** one intent, one capability, roughly ten to fourteen requirements, one specification for the port and wire
format, one ADR for the provider adapter, one verification contract, three work orders. The requirement count grew
against the first draft because decisions 2 and 5 are obligations and only requirements carry those: the cache
threshold, the per-call usage record, the CI-secret prohibition, the flag-plus-key gate, the spend ceiling and the
permission record are six requirements that the first draft treated as design notes.

**In-place amendments required:**

| Artifact | What moves |
|---|---|
| `ARCH-MOK-001` | A provider component admitted outside the engine boundary, on `ADR-MOK-003`'s precedent for the observer. `ROADMAP.md` already anticipates a similar argument for Phase 4b; if both are needed they should be made once, together |
| `SPEC-MOK-002` | Rules 5 and 6: the port added to the public interface, argued as a value-only projection against the capability rule 6 denies rather than as an exception to it. Rule 13's declared set stays **empty** |
| `SPEC-MOK-001` | The fifth policy and its rule; the rule-6 legal-action set the port presents; the `--policy` help text, which names four values today |
| `SPEC-MOK-004` | The test census rule 11 obliges, and — under either transport option — how the provider component's own tests enter it |
| `INT-MOK-001` | The success measure: seed **and transcript** |
| `REPOSITORY_CONTEXT.md` | Commands, the credential boundary, and the model identifier corrected from "OpenAI GPT nano" to `gpt-5.6-luna` |
| `REQ-MOK-050` | **Only under transport option A.** Under B it is untouched — see Part 3 decision 1 |
| `ADR-MOK-006` | **Only under transport option A**, as a crate admission with decision 1's criteria applied and recorded. Under B, nothing enters a declared set |

**Not amended, and each one worth stating as a result rather than leaving silent:**

- `REQ-MOK-009` — reproducible entropy, preserved by making the transcript an input.
- `ADR-MOK-001` — engine authority and the trust boundary, untouched.
- `REQ-MOK-059` — the read prohibition; the prompt is built from `Observation` and nothing else.
- `REQ-MOK-010` — the text record stream, unchanged for the four existing policies.
- `REQ-MOK-014`, `REQ-MOK-034`, `REQ-MOK-058`, `REQ-MOK-060` — the outcome floors and the composition ceiling. Each
  names its own sources, so none binds a fifth. Finding 6 explains why silence here is nevertheless a risk.
- `mokiterions-core/tests/viability.rs` — three per-policy tests, none of which a fifth policy enters.

**Note for the packet's author:** `REPOSITORY_CONTEXT.md` currently records the intended model as "`OpenAI GPT nano`"
with an instruction to "confirm the exact OpenAI API model identifier before integration." That confirmation is now
available and the file is stale.

---

## Part 3 — Decisions still open

Four. Decision 1 is structural and blocks the ADR; the rest can be taken later without holding up stage 5a.

1. **Where the provider lives** — a third Rust package with an HTTPS client *(option A: an estimated 40–60 crates
   admitted, `REQ-MOK-050` amended, a crate admission recorded)*, or a separate provider program spoken to over
   pipes *(option B, recommended: no crate anywhere in the workspace, `REQ-MOK-050` untouched, the credential never in
   Rust code, at the price of governing a non-Rust product component and its stdlib-only dependency discipline)*.
   This is the only remaining question that changes which artifacts move.

2. **Whether tier 1 is accepted.** A committed 20–50 tick canned transcript, replayed on every push at $0 with no
   network — or no automated coverage of the LLM policy at all, which is coherent and should then be recorded as
   chosen. Tiers 2 and 3 are not in question: tier 3 is owner-permissioned by decision 5.

3. **The spend ceiling to declare**, per run and in total. Finding 3's estimate is $1.04 per 1,000-tick run and $5.18
   for five seeds, so any ceiling in the tens of dollars is generous against expected use while still stopping a
   runaway loop. A number is needed because mechanism 3 refuses to exceed it.

4. **The horizon for stage 5c.** 1,000 ticks matches the other policies' measurements and costs an estimated 1.2–2.4 h
   per seed; 200 ticks costs an estimated 15–30 minutes. Recommendation: defer until 5b has measured real latency, then
   fix it. No floor attaches to it either way, per decision 4.

## What this proposal does not close

- **Latency is an estimate.** It cannot be measured without a provider call, which is why 5b measures it under an
  owner permission record before any horizon is fixed. Every latency figure here is provisional.
- **The token shape is an estimate.** The 1,200 / 30 / 200 / 25 split is a construction. 5b replaces it, and with it
  the cache ratio's one point of margin — if the real ratio lands below 0.85 the threshold is what needs revisiting,
  on measurement.
- **The action-grammar trade-off is unmeasured.** Removing the enumerated legal set saves tokens and may raise the
  rejection rate. Both numbers come from 5b; neither is asserted here.
- **Temperature and seed support are unconfirmed** for `gpt-5.6-luna`. The design deliberately does not depend on
  either, but a reader should not infer from this document that they are absent.
- **Whether the LLM produces behaviour distinguishable from `reference`** is the question Phase 5 exists to answer,
  and this document takes no position. Finding 8 exists so that a negative answer is publishable.
- **Phase 6's evaluation criteria.** Decision 4 makes Phase 5 produce observations rather than verdicts. What counts
  as emergence, and against what, is Phase 6's problem and is not solved by anything here.
- **Phase 4b is untouched.** If it also needs a component admitted outside the engine boundary, the two arguments
  should be made once, together, rather than twice.
