+++
id = "ADR-MOK-007"
type = "adr"
title = "A decision port at the existing trust boundary, a retained transcript as the second determinand of a run, and the model provider outside the workspace"
status = "approved"
owners = ["technical owner"]
created = "2026-08-23"
updated = "2026-08-23"

[relations]
decides = ["ARCH-MOK-001", "ARCH-MOK-002"]
+++

# ADR: A decision port at the existing trust boundary, a retained transcript as the second determinand of a run, and the model provider outside the workspace

## Status

**Accepted 2026-08-23 by the repository owner acting as accountable technical owner.** The instruction, verbatim and
complete, in the turn it was given: *"i approve the artifact pack"*. It decides `ARCH-MOK-001` and `ARCH-MOK-002`, and it
is a precondition of `WO-MOK-025`, `WO-MOK-026` and `WO-MOK-027`.

It was drafted by an implementation agent on the repository owner's instruction to realize a model-backed decision
source, and the agent did not take this decision. `DECISION_RIGHTS.md` reserves acceptance to the technical owner and
states that an implementation agent "may not self-approve that assessment unless it is separately named as the
accountable technical owner".

**What the acceptance covers is the *Required amendments* section, in full**, on the `ADR-MOK-006` precedent recorded in
that ADR's own *Status*: each amendment here is stated completely, so accepting this one file approves the whole change.
**Two** of those amendments are not the technical owner's — `INT-MOK-001` is the product owner's and
`REPOSITORY_CONTEXT.md` is the repository owner's. The owner holds all of those roles and approved them here, by way of
this section; each amended artifact will record which role approved it and that it was approved through this ADR rather
than separately.

**No amendment is written by this acceptance.** Every provision under *Required amendments* remains unwritten in its
target artifact and is `WO-MOK-025`'s, `WO-MOK-026`'s or `WO-MOK-027`'s to write under this authorization, on the
precedent `WO-MOK-020`'s transition set. The amendment record rows those provisions call for do not exist yet, and the
figures each row obliges to be re-measured are to be measured when the row is written rather than copied from here.

**What the acceptance does not cover: no work order is approved by it.** `WO-MOK-025`, `WO-MOK-026` and `WO-MOK-027`
remain `draft`, and no implementation is authorized. `WO-MOK-025`'s *Lifecycle* makes this explicit — *"the approval of
the packet is a distinct act from the approval of this work order"* — and the instruction names the pack and not the work
orders, where the same owner's instruction of 2026-08-22 said *"including the work order"* when a work order was meant.
Nothing here is approved by implication.

**The consequence for `WO-MOK-025`'s *Lifecycle* is that one of its two conditions is now met and the other is not.** Its
approval was written as *"the same act as the approval of every row in `ADR-MOK-007`'s *Required amendments* that this
stage needs"*. Those rows are approved as of this section, so that work order's approval now authorizes its scope alone.
Its own transition remains outstanding.

Three decisions are taken together because they are one shape and separating them would leave each incoherent. Decision
1 puts the port where the trust boundary already is; decision 2 makes the provider's answers a run input rather than a
source of non-determinism, which is only possible because of where decision 1 put the port; decision 3 keeps the
transport outside the workspace, which is only harmless because decision 1 gave the engine an interface that names no
transport. Accepting one without the others produces a design this ADR does not describe.

**Decision 3's binding was put to the owner as a choice and has been taken.** On 2026-08-23 the repository owner first
selected a third Rust crate inside the workspace — option 3a — and then, on the same day and after the design of the two
existing hosts was established, selected the **connector** binding of option 3d instead. Both acts are recorded under
*Decision record*. The earlier selection is superseded rather than deleted, because the reasoning that replaced it is the
substance of this decision: 3a's cost was found to be twice what this ADR first stated, and a binding that amends no
approved artifact was found to exist. All three rejected bindings remain stated in full under *Considered options*.

## Context

`ARCH-MOK-001` is a single-process architecture in which the engine is authoritative and every decision source is
in-process, pure and deterministic. Four sources exist. `ADR-MOK-001` fixed the boundary they sit behind: a source
receives a copy of an observation and returns a typed proposal, and no public item *"yields a mutable borrow of, or a
reference into, authoritative state"*. That boundary has held through four sources, two packages, a structured record
stream and a combat vocabulary.

**`ADR-MOK-001` anticipated this ADR and stated four of its conclusions in 2026-08-11.** Its *Negative* consequences
record that *"a later network-backed model adapter must translate provider responses into typed proposals and handle
latency and failure outside the engine's mutation path."* Its *Operational and security* consequences record that
*"future provider credentials must remain outside the engine and repository"* and that *"model output is untrusted input
and must pass the same validation as the local baseline."* Its *Migration* section instructs that if later requirements
justify remote decision execution, *"preserve the observation and proposal semantics while introducing transport outside
the authoritative engine."*

That matters for how this ADR should be read. Decisions 1, 3 and 6 are not new architectural positions; they are
`ADR-MOK-001`'s migration clause carried out, and `REQ-MOK-063` is its *"same validation as the local baseline"* written
as an obligation. `ADR-MOK-001` also states that *"replacing engine authority itself requires a superseding ADR"* — and
nothing here replaces engine authority, so this ADR supersedes nothing and `ADR-MOK-001` stands unamended.

`INT-MOK-011` now asks for a fifth source whose decisions come from a language model. Three properties of that source
are unlike anything the architecture has admitted so far, and each of them is what this ADR exists to resolve.

**It is not a function.** The four existing sources are pure functions of an observation. A model is a network service
with a price, a latency, an authentication requirement and no determinism guarantee. `INT-MOK-001`'s success measure is
that repeated runs at an identical seed give 100 percent identical results, and `ROADMAP.md` already recorded that
temperature 0 *"is not a bitwise determinism guarantee from any provider"* even where it is offered. Temperature and
seed support are not documented at all for `gpt-5.6-luna`.

**It costs money and takes hours.** A 1,000-tick run at density 0.75 has a measured 10,954 decision opportunities. At
`gpt-5.6-luna`'s published prices that is an **estimated** $1.04 per run under a cache-ordered prompt and an
**estimated** 1.2 to 2.4 hours of wall time. The repository owner decided on 2026-08-23 that continuous integration may
not spend this, that automated tests may not either, and that a real run needs the owner's explicit permission.

**It needs a network and a credential.** `REQ-MOK-050` forbids any crate providing network access, credential handling or
an asynchronous runtime in either package, and `ARCH-MOK-001`'s conformance check scans the engine's dependency graph by
name for a model-provider crate. `ADR-MOK-006` withdrew the engine's blanket empty-dependency rule and replaced it with
criteria-based admission against a declared set, so the question is not *whether* a dependency could be admitted but
what admitting an HTTPS stack would cost.

Two facts about the existing artifacts narrow this ADR considerably, and both were established by reading rather than
assumed.

**No existing outcome obligation binds a fifth source.** `REQ-MOK-014` names *"the reference decision source"*,
`REQ-MOK-034` the trait-aware source, `REQ-MOK-058` *"the social decision source"*, and `REQ-MOK-060` *"the reference,
trait-aware or social decision source"*. A fifth source inherits none of their survivor floors, and `tests/viability.rs`
holds three separate per-policy functions rather than one parameterised over the set. So `INT-MOK-011`'s decision to set
no viability floor requires no amendment anywhere — only a positive statement, which is decision 7.

**The dependency prohibitions are package-scoped.** `ARCH-MOK-001` says *"in the engine package"*, `REQ-MOK-050` says
*"in either package"*, and `ARCH-MOK-001`'s own rationale already *"defers an external model provider to an adapter at
the same boundary"*. Nothing prohibits a provider that is not a crate in either package. That is what makes decision 3's
recommended binding cost nothing in amendments.

## Decision drivers

1. **The trust boundary must not be widened.** A source that could reach engine state would retire the property four
   sources and six ADRs rest on.
2. **A measurement must be reproducible years later, offline and free.** A figure produced by an event that happened once
   and cannot be examined again is not evidence this repository can carry.
3. **The four existing sources must stay byte-identical**, entropy draws included. Every published figure rests on them.
4. **Cost must be bounded before it is spent**, and the bound must be a number the owner named.
5. **A live run must be impossible by accident**, and the safe default must also be the useful one, or it will be routed
   around.
6. **The dependency surface should not grow to buy this.** Not because growth is forbidden — `ADR-MOK-006` settled that
   — but because an HTTPS stack is the largest single admission this repository would have made, for a component that is
   not part of the simulation.
7. **The cost of the owner's decisions must be visible in one place.** Where a decision has a price, this ADR states the
   price.

## Considered options

### The port's shape

**Option 1a: a fifth arm of the existing source selection.** The model call happens where `baseline`'s entropy selection
happens, inside the engine. Rejected: the engine would then open a socket or spawn a process, contradicting the property
that it *"resolves no path, opens no file, creates no directory and removes none"*, and `REQ-MOK-050`'s prohibition would
bind the engine package directly.

**Option 1b: one port the engine calls, implemented by the host.** The engine gains one interface that takes a request
by value and returns a proposal or nothing. The host decides what is behind it. Selected as decision 1.

**Option 1c: an event-driven inversion — the engine yields a request and is resumed with an answer.** Equivalent in
power and strictly larger in change surface: it restructures the tick loop, which rule 16 of `SPEC-MOK-007` holds
byte-identical for four sources. Rejected on driver 3.

**Option 1d: move all five sources onto the port.** Rejected on driver 3. Refactoring four working sources' call path
buys nothing and risks the byte-identity every published figure rests on.

### Determinism

**Option 2a: ask the provider for determinism.** Set temperature 0, pass a seed, hope. Rejected: not documented for this
model, and not a bitwise guarantee anywhere. It would make `INT-MOK-001`'s success measure depend on a vendor's
unwritten behaviour.

**Option 2b: record the answers; replay from the record.** The transcript becomes a run input, in the same class as the
seed. Given the same seed *and* the same transcript, the run is identical. Selected as decision 2.

**Option 2c: declare this source non-deterministic and exempt it from the determinism measure.** Rejected on driver 2. It
would produce figures nobody could reproduce, and it would put a hole in the one property this repository has held from
`INT-MOK-001` onward.

**Option 2d: canonicalise responses so that only the chosen action is recorded.** A smaller transcript, and it replays.
Rejected: the request and the response bytes are the evidence that `REQ-MOK-065`'s isolation and `REQ-MOK-066`'s
statelessness held. Discarding them would make two requirements uncheckable to save an **estimated** 4.7 MB per run.

### Where the provider lives

Three bindings were put and the third is decided. The two rejected ones are recorded with the costs a later reading
established rather than the costs first drafted, because **this ADR's first draft understated option 3a materially** and a
reader comparing the options is entitled to the corrected figure.

**Option 3a: an HTTPS client inside the workspace.** A third package, or the observer package, gains an HTTP client, a TLS
stack and probably an asynchronous runtime. **Rejected, and it is the most expensive of the three by a wide margin.** The
first draft of this ADR costed it as contaminating the engine package. It contaminates **both**: the host is
`mokiterions-core`'s own `[[bin]]` target, Cargo declares dependencies per package rather than per target, so a provider
crate that binary reaches enters the engine package's resolved graph — and from there the observer's, which path-depends on
the engine package. `SPEC-MOK-003`'s declared set counts transitive crates exactly as the engine's rule 13 does, so its
sentence *"Every other crate in the observer's resolved graph is reached transitively through `ratatui`"* becomes false as
well.

**Option 3b: a repository-owned provider program the host drives over pipes.** **Rejected**, though it was this ADR's
original recommendation. Its two merits — no crate, and a credential confined to a component that touches nothing else —
survive intact into the decided option, which is why the change of decision is a simplification and not a reversal. What
3b additionally does is fix the provider *inside* this repository: a directory `SPEC-MOK-004` rule 1's layout must be
amended to admit, a second artifact to build, test and version, and a dependency surface held only by a rule constraining
it to its language's standard library — a constraint that buys real discipline and costs the ability to use a vendor SDK.

**Option 3d: a connector the operator names by path.** **Decided, as decision 3.** A host spawns an executable the
operator names on the command line and exchanges one JSON object per line with it over that child's standard streams. This
is 3b with the repository ownership removed: the connector is not a package, not a workspace member, and not required to be
in this repository at all.

| | 3a — a crate in the workspace | 3b — a repository-owned program | 3d — an operator-named connector |
|---|---|---|---|
| Crates added | An **estimated** 40 to 60 transitive, to **both** packages | None | None |
| `REQ-MOK-050` | Amended: it prohibits network, credential-handling and asynchronous-runtime crates *"in either package"*, and both would acquire all three | Untouched | Untouched |
| `ADR-MOK-006` decision 4 | Its **envelope reversed**, not its criteria exercised. That envelope bars exactly these three crate classes *"however stable it is"* and is checked **before** the criteria | Untouched | Untouched |
| `ARCH-MOK-001`'s conformance check | Amended: its by-name scan over the engine package's graph fails as written | Untouched | Untouched |
| `SPEC-MOK-002` rule 13 | The engine's declared set grows from empty to an **estimated** 40 to 60 entries | Untouched | Untouched |
| `SPEC-MOK-003`'s declared set | Amended: its transitive-reach sentence becomes false | Untouched | Untouched |
| `SPEC-MOK-004` rules 1 and 2 | A third package directory and a third workspace member — rule 1 prohibits both today | Rule 1 admits a non-package directory | Untouched |
| What a host needs | The client's API | `std::process` and `std::io` | `std::process` and `std::io` |
| Credential holder | A workspace package | The provider program only | The connector only |
| Dependency discipline | Inside the declared set | Outside it, held by a standard-library rule | **Outside it, and unconstrainable** |
| Offline build | The new package needs the network at build time | Unaffected | Unaffected |
| **Approved artifacts amended** | **Six** | One | **None** |
| Swappable without a code change | No | No | **Yes** |

Option 3d's honest cost is the two bold rows near the bottom. The connector is an operator-supplied executable, so nothing
in this repository constrains its dependency surface, its internal behaviour, or its honesty about the token usage the
spend ceiling is computed from. `SPEC-MOK-007` rules 10.6, 10.7 and 10.8 state each of those three limits explicitly
rather than leaving them to be discovered: the dependency surface is unconstrainable, the connector's whole output is
untrusted input, and the ceiling protects against an honest connector overspending rather than against a dishonest one
lying. What the repository keeps is the containment that does not depend on the connector at all — rule 13's two gates,
checked in two different components, and no credential in automation.

In exchange, 3d is the only binding of the three that amends **no approved artifact**, and the only one where the stub
used for offline verification is the same kind of thing as the real connector rather than a special build. That second
property is what makes free continuous verification of the live wiring possible; `SPEC-MOK-007` rule 20.5 requires the
canned connector for it.

**Option 3c: vendor an HTTPS client into the repository.** Rejected. `ADR-MOK-006` considered and rejected vendoring for
the general case, and nothing here argues differently.

**Option 3d: no live calls at all — hand-write transcripts.** Rejected. It would satisfy every property in this ADR and
measure nothing, which is `INT-MOK-011`'s entire purpose.

### Isolation

**Option 4a: one fresh request per decision.** Selected, on the repository owner's decision of 2026-08-23.

**Option 4b: one provider conversation per Mokiterion, accumulating.** Rejected. `REQ-MOK-066` gives the reasons in
full; the architectural one is that accumulated context is state that decides behaviour and that nothing retains, which
contradicts this repository's observability property and cannot be reconstructed from a transcript.

### Cost containment

**Option 5a: a documented instruction not to run this in automation.** Rejected on driver 5. It is a permission recorded
only in prose, and a script can bypass it by accident.

**Option 5b: capability-based containment.** The credential is never present in automation, a live run needs both a flag
and a key, and the ceiling is enforced in the run before each call. Selected as decisions 4 and 5.

## Decision

**Decision 1 — one decision port at the existing trust boundary.** The engine gains exactly one interface for obtaining
a proposal under this source. It takes a decision request by value and returns a proposal or the absence of one. It names
no provider, no transport, no model, no credential, no file and no mode. The request crosses as values only, yielding no
mutable borrow of and no reference into authoritative state, which is `ADR-MOK-001`'s boundary and `SPEC-MOK-002` rule
6's prohibition adopted unchanged rather than reinterpreted. Latency, failure and transport are handled on the host's
side of it, which is what `ADR-MOK-001` required of *"a later network-backed model adapter"*. The engine's public surface
grows by one interface and one request type, and **two existing public signatures each gain one optional parameter** —
`execute`, the process boundary the recording host drives a whole run through, and `Simulation::advance_tick`, the single
tick the replay host advances. Those are the two doors `SPEC-MOK-007` rule 20.5 names, and they are named here rather than
called "the two run entry points" because the engine has a third public way to drive a run and it is deliberately not one
of them: **`pub fn run` is not amended.** It delegates to a crate-private carrier with the port absent, exactly as it
delegates today with the record sink absent, so its enumerated form in `SPEC-MOK-002` rule 5's first list — "`&mut self`
and a writer in, `io::Result<RunSummary>` out" — is unchanged. Neither host reaches this source through it: the recording
host enters at the process boundary and the replay host advances tick by tick. A library consumer that wants a whole run
under this source drives `advance_tick`, which is what the observer does.

The carrier is `pub(crate) fn run_recording`, and it **does** gain the parameter. That is disclosed here for the same
reason `SPEC-MOK-002`'s 2026-08-20 amendment disclosed it rather than relying on its non-match silently: a reader
comparing this decision against the diff would otherwise find a third changed signature the ADR does not account for. It
is crate-private, is not on the interface, and is not reachable from any item that is. Nothing else on the surface moves,
and the four existing sources do not move onto the port.

**The source is named `llm`.** That is the value an operator passes to the policy option and the value the record stream
emits for the decision source; they are the same string, as they are for the four existing sources. The word names *how
the decision arrives* rather than which vendor answers, which is the point: the connector is swappable and the source
name must not go stale when it is swapped. `luna` appears nowhere as a source name, and `gpt-5.6-luna` appears only as
the model identifier the connector declares. `SPEC-MOK-007` rule 18.1 fixes the value and `SPEC-MOK-006` rule 3.2 admits
it as the record stream's fifth.

**Decision 2 — the transcript is the second determinand of a run.** A live run records every exchange; a replay obtains
every decision from that record and makes no provider call. Determinism is claimed for the pair *(seed, transcript)*
rather than for the seed alone. There is no mode branch in the engine: the difference between recording and replaying is
which stream the host connected, so byte-identity is structural rather than two implementations to keep in agreement.
`REQ-MOK-009` does not move, because the entropy stream is untouched — this source draws from it not at all.

**Decision 3 — the provider is reached through a connector the operator names by path, and no crate is added anywhere.** A
host spawns the executable the operator names on the command line and exchanges one JSON object per line with it over that
child's standard input and standard output. The connector reads the credential from its own process environment and is the
only component that holds one: neither host reads it and no option carries it. **No crate is added to either package, and
no approved artifact is amended** — `REQ-MOK-050`, `ADR-MOK-006`, `ARCH-MOK-001`'s dependency conformance check,
`SPEC-MOK-002` rule 13, `SPEC-MOK-003`'s declared dependency set and `SPEC-MOK-004` rules 1 and 2 all stand exactly as
written, because spawning a child and reading its lines is standard-library work in both packages.

The connector is deliberately **not** this repository's, and that single property is both the decision's substance and its
whole cost. It makes the binding swappable without a code change, and it makes the stub used for offline verification an
ordinary connector rather than a special build. It also puts the connector's dependency surface, its internal behaviour and
its honesty about reported token usage beyond anything this repository can check; `SPEC-MOK-007` rules 10.6, 10.7 and 10.8
state those three limits rather than implying them. The repository owns exactly one connector — the **canned** one of rule
20.5, which answers from a fixed script, reaches no network, and exists so that the live path's wiring is verified offline
at no cost on every push.

**Decision 4 — the prompt layout is cache-ordered, and the cached share is an obligation with a number.** Requests are
composed as shared rules, then the acting Mokiterion's constants, then its observation, then the enumerated action set —
stable first, variable last, because the provider's cache matches the longest identical leading span. The cached share of
prompt tokens is held at 0.85 or above over a run, computed from the provider's own reported usage and never from a local
estimate. This is an architectural decision and not an optimisation: the same information in a different order costs ten
times as much, and no test that checks the decision made would notice.

**Decision 5 — replay is the default; a live run requires two independent conditions; automation holds no credential.**
An explicit live-mode selection *and* a credential in the process environment are both required, and neither is acquired
by default. A live run also requires a declared spend ceiling, enforced before each call. No workflow in this repository
references a model-provider credential, and the credential is not placed in the repository's automation secrets — which
is the containment that does not depend on code being correct. Automation exercises this source in replay mode against a
transcript committed to the repository.

**Decision 6 — an unanswered decision falls back to `wait`, is counted, and disqualifies the run from publication.**
Never to another source's proposal: a run that substituted `baseline`'s selection would report what a mixture of two
sources did under one label. A proposal the engine's rules reject is *not* a fallback and is not counted; that is an
ordinary rejection and part of what a measurement measures. The disqualification threshold is zero rather than a
tolerance, because the property being measured is exactly the one a substitution interferes with, and a re-run costs an
**estimated** $1.04.

**Decision 7 — this source is held to no viability floor, and the absence is recorded as a decision.** `INT-MOK-011`
sets no survivor floor, no death ceiling and no better-or-worse claim. The architectural consequence is that the prompt's
shared rules block contains no strategy, no goal, no preference and no advice: a block that told the model to survive
would measure the instruction. `baseline` is the precedent for a floor-free source — extinct between ticks 119 and 193 on
every declared seed, recorded as measurement and never as failure. The reason this decision is written rather than left
implicit is that four sources in a row received a floor, so silence would read as an omission; `REQ-MOK-034`'s 2026-08-20
drafting set the precedent of stating a deliberate absence positively.

**Decision 8 — there are two hosts, they are not equally capable, and the port is held for the run rather than built per
tick.** The engine's binary target records and replays; the terminal observer replays only. The reason is arithmetic rather
than preference: the observer owes a frame every 33 milliseconds and an input poll every 16, which `SPEC-MOK-003` rules 6.1
and 6.2 fix, while one live tick costs an **estimated** 4 to 9 seconds — an **estimated** eleven decision opportunities per
tick at an **estimated** 0.4 to 0.8 seconds per exchange. The gap is two orders of magnitude, so this is not a tuning
problem. The only remedies are concurrency or an asynchronous runtime, forbidden by `SPEC-MOK-007` rule 16 and
`REQ-MOK-050` respectively.

Two further obligations travel with this decision, because the engine's existing shape invites getting both wrong in ways
that still compile and still run. **The port is supplied at construction and held for the whole run**: the four existing
sources are stateless values built at the point of use, and a port built on that precedent would reset the transcript
cursor, the accumulated cost and the fallback count every tick — so the cursor would restart, the cost would stay at zero
and the ceiling would never trigger. And **both of rule 20.5's two doors accept the port**: the two hosts enter the
library by different doors, one driving a whole run and one advancing a single tick, so wiring only one door silently
excludes the other host from this source while every other rule still reads as satisfied. `SPEC-MOK-007` rule 20 is the
whole of this decision, and `ARCH-MOK-002` is amended for the observer's half of it.

## Decision record

Every act below is the repository owner's, who holds the product, technical and engineering owner roles. Nothing here is
approved by implication, so each act is recorded separately with what it settled. All are dated **2026-08-23**.

1. **Isolation is fresh per decision**, as a first step. Decision 4's layout and `SPEC-MOK-007` rule 2.4 carry it.
2. **The shared cacheable preamble is adopted, and cache optimisation is an explicit requirement**, in the owner's words
   *"an important requirement"* rather than a nice-to-have. This is why decision 4 states the cached share as an obligation
   with a number instead of as an optimisation, and why `REQ-MOK-070` exists at all.
3. **Reasoning level `none`**, at first. `SPEC-MOK-007` rule 10.3 carries it as declared configuration. The **estimated**
   $4.64 per run at `low` against **estimated** $1.04 at `none` makes it a cost decision as much as a design one.
4. **No viability floor**, because in the owner's words *"the whole point of this is to empirically see what is going to
   happen, the constraints need to be relaxed for the LLM policy"*. Decision 7 carries it.
5. **No LLM policy in continuous integration or in automated tests**, and an explicit owner permission required to launch a
   real run. Decisions 5 and 6 and `SPEC-MOK-007` rules 13.6, 13.7 and 17 carry it.
6. **Verification tier 1 is accepted**: a canned transcript committed to the repository and replayed offline on every push,
   at no cost, with no network and no credential. `VER-MOK-018`'s `L21b` and `SPEC-MOK-007` rule 13.7 carry it. It is
   recorded as a decision because the alternative — no automated coverage of this source at all — was a live option and was
   declined rather than overlooked.
7. **The declared spend ceiling is $2.** It governs `WO-MOK-026`'s single authorised instrument run, which needs an
   **estimated** $0.02. It is **below** the **estimated** $5.20 that five 1,000-tick seeds cost, so it does **not** cover
   `WO-MOK-027`'s measurement; that stage's ceiling arrives with its horizon under act 8. Both figures are recorded here
   because a $2 ceiling read as per-run rather than as declared-once would misread the owner's act.
8. **Stage 5c's horizon and seed set are deferred**, which is a decision and not a gap. They are declared in the
   authorization record `REQ-MOK-076` requires, after `WO-MOK-026` has measured the real per-exchange cost and latency
   against the estimates. `WO-MOK-027` records the deferral positively for the reason `REQ-MOK-034` set the precedent for.
9. **The provider's location: a third Rust crate inside the workspace**, option 3a. **Superseded the same day by act 10.**
   Retained rather than deleted, because act 10's reasoning is the substance of decision 3.
10. **The provider's location: a connector the operator names by path**, option 3d, which decision 3 now states. Taken
    after the two existing hosts' shape was established — that the observer links the engine library and drives it
    in-process, that the host of a live run is the engine package's own binary target, and that option 3a therefore
    contaminates **both** packages rather than one. Act 10 is what changed decision 3, the option table under *Considered
    options*, and the six amendment rows this ADR no longer requires.
11. **The definition layer is approved**, in the owner's words *"i approve the artifact pack"*: `INT-MOK-011`,
    `CAP-MOK-011`, `REQ-MOK-063` through `REQ-MOK-077`, `SPEC-MOK-007`, `VER-MOK-018` and this ADR move from `draft` to
    `approved` — twenty artifacts in one act. Because it is this ADR's acceptance, it is also the approval of every row
    under *Required amendments*, which the *Status* section states in the terms `ADR-MOK-006` set. **It approves no work
    order and authorizes no implementation**: `WO-MOK-025`, `WO-MOK-026` and `WO-MOK-027` stay `draft`, and the act that
    moves the first of them is a separate one that has not been taken. Recorded as its own act, rather than folded into
    acts 1 to 10, because those settled what the design is and this one settles that the design may be built from.

## Required amendments

Each is stated in full so that accepting this ADR accepts the change. **Two of them are not the technical owner's**:
`INT-MOK-001` is the product owner's and `REPOSITORY_CONTEXT.md` is the repository owner's. `WO-MOK-025` makes every row
below an approval precondition, as `WO-MOK-014` did for `ADR-MOK-006`.

**No amendment is made in this packet.** Every artifact named below is `approved`, and amending an approved artifact is
an owner act; these rows state what must move and leave the moving to the owner, on `ADR-MOK-006`'s and `WO-MOK-024`'s
precedent.

**Decision 3 contributes nothing to this list.** Every row below follows from decisions 1, 2 and 4 to 8 — the port, the
determinism property, the accounting and the two hosts. The connector binding costs no amendment at all, which is the
substance of the option table's *Approved artifacts amended* row and the reason the first draft of this ADR carried six
rows that are now deleted. What those six were, and why they existed, is under *No dependency artifact is amended* below;
the rows are kept in summary rather than in full so that the deleted cost stays visible to a later reader.

### `SPEC-MOK-001` — technical owner

- **Rule 1**'s emitted decision-source value, and the source vocabulary the text stream carries, gain one member for the
  new source.
- **A new rule, appended** rather than placed beside rule 5, for the reason the *Behavioral rules* preamble gives and in
  the form rules 19 and 26 already take: *"the model-backed decision source"*, selected by the new policy value,
  occupying rule 5's position in tick order. Its body defers to `SPEC-MOK-007` for the request, the response and the
  transcript, and states in this specification only what a reader of the tick order needs: that the source consumes no
  entropy, that it returns one proposal of rule 6's admissible set, and that an unanswered decision returns `wait`.
- **Rule 3 does not move.** The list of currently valid core proposals does not gain a member, does not change length
  and does not change order. The rule's own paragraph already states why, and decision 1 is what makes leaving it alone
  possible: block D is composed beside that list rather than by extending it.
- **Rule 6 does not move.** The proposal from this source is validated by rule 6 exactly as every other source's is.
- The *Behavioral rules* preamble's rule-position table gains one row for the new rule.
- ***Actors and external systems* is amended, and this is the row a reader should check first.** The section says "There
  are no external systems and no network calls", amended once already, on 2026-08-20, to admit the filesystem as a
  destination and to close with "The engine never reads it, and no filesystem location is a source of engine input."
  **Both halves stop being true.** There is now an external process — the connector — spawned by the binary target and by
  nothing else, and behind it a provider reached over the network by the connector alone. And a filesystem location **is**
  now a source of engine input: the transcript, opened by a binary target and lent to the library as an already-open
  reader, per `SPEC-MOK-007` rule 12.1.1. The amendment states both, and states what does not move — the library target
  spawns nothing, opens nothing and reads no environment variable, which is `SPEC-MOK-006` rule 1.2's property extended to
  a second stream rather than excepted for one.
- ***Inputs* gains four options**, each in the form `--events-path` took on 2026-08-20: the connector path, the live-mode
  selection, the transcript path and the spend ceiling. Each is absent by default, may appear at most once, and — for the
  two paths — rejects the empty string and the single character `-` as invalid configuration while a well-formed path the
  platform refuses stays a runtime failure, because whether a path can be opened is not a property of the argument. That
  distinction is `--events-path`'s and is adopted rather than re-derived.
- ***Help output* gains the four entries**, and *Outputs* gains the transcript as a third stream, recording that the text
  stream and the record stream are unaffected by any of the four options' presence.
- ***Security and privacy properties* is amended in one sentence that is currently false-to-be.** It reads
  "**`--events-path`'s value is the one operator-supplied value that is interpreted as a filesystem path.**" There will be
  three. The amendment says three, names them, and **keeps the property that sentence exists to carry**: each is
  interpreted only by a binary target, only as a path, never as code, never as a format string, never as an option and
  never as engine input, so the engine still cannot be reached through any of the values. The following sentence — "The
  library target interprets no path at all and performs no filesystem operation" — is **unchanged and load-bearing**, and
  `SPEC-MOK-007` rules 20.4 and 12.1.1 exist to keep it that way.
- ***Error and recovery behavior* gains the refusals** `SPEC-MOK-007` rules 13.2, 20.3 and 20.8 fix, with no new exit
  code: an invalid configuration is `2` and an output failure is `1`, exactly as today. Stated because the natural
  instinct on adding a live mode is to add a code for it.
- Amendment record row in the form the specification already uses, whose *Approval* cell names the technical owner and
  this ADR. **This is the largest amendment in this list** — seven provisions against `SPEC-MOK-006`'s three — and the
  reason is that this specification is where the engine's external-systems claim and its filesystem claim both live.

### `SPEC-MOK-006` — technical owner

- **Rule 3.2**: the domains of `config.policy` and `result.source` gain the new source's value.
- **Rules 5.3 and 10**: `schema` increments, because rule 10.2 requires it when a value's domain in rule 3.2 gains a
  member. **The increment is to one more than whatever value the ratification of the specification's 2026-08-21
  amendment row leaves standing.** That row is recorded **OUTSTANDING** and takes `schema` to `2` against an engine that
  still writes `1`; this ADR does not resolve it, does not depend on its resolution and does not assume a number. The
  work order states the ordering: the outstanding row is ratified, or it is not, before this increment is written, and
  the increment is measured against the tree rather than inferred.
- **Rule 3.3's union does not move**, and this is to be measured rather than argued, on the 2026-08-21 row's own
  precedent: the new source's value is an identifier over `A`–`Z`, `a`–`z`, `0`–`9`, so no escaping function is needed
  for the reason rule 3.3 gives and for no other. The measurement is `VER-MOK-018`'s.
- **No record kind is added and no field is added.** The transcript is a third stream and is not this one. Rule 9.1's
  order, rule 8.6's equalities and rule 7's metrics record are untouched.
- Amendment record row.

### `SPEC-MOK-002` — technical owner

- **Rule 4 gains a second optional parameter on `execute`, bringing it to five**, carrying a borrowed port the caller
  owns. Rule 4 governs `execute` and nothing else — it is the process-boundary rule, and its own text says "`execute`'s
  signature is enumerated by rule 4 and by nothing else" — so this is the whole of rule 4's amendment, and the
  single-tick door is a rule 5 change rather than a second half of this one. The shape is rule 4's own reused, not a new
  one: the rule already fixes the record sink as `Option<&mut dyn Write>` rather than a generic, "so that a caller with
  no sink passes `None` and needs no type annotation for a writer it does not have", and it already records that the
  entry point "does not resolve it, open it, create it or remove it". Every word of that carries over to the port, with
  a spawned child process or an open transcript in place of a created file. The rule's closing clause — that `src/lib.rs`
  is "the process boundary and nothing more" — is what forbids the alternative of the library owning the connector, and
  it is cited rather than amended.
- **Rule 5, the authorized public interface, is amended in three ways and not one.** It gains the decision port's
  interface and the decision request type as items, and nothing else. `Simulation::advance_tick`'s row in the
  observation-surface list gains the parameter, which is where the single-tick door is amended: `advance_tick` is
  enumerated by that list and not by rule 4, and reading rule 4 as covering both doors is the error this row exists to
  keep out of the specification. And **rule 5's mechanical checks are restated**, on the exact precedent of its
  2026-08-20 restatement for `REQ-MOK-042`, for a reason that is not editorial: the standing text reads "**A fifth
  parameter**, a second sink, or a sink that is not optional fails the second", and a decision port on `execute` is a
  fifth parameter. An amendment that added the port and left that sentence standing would produce a specification whose
  own drift check condemns the build the specification requires. The restatement adds a third `grep` for the port
  parameter and states the failure conditions against five parameters rather than four; `VER-MOK-018`'s `S4a` runs it.
- **The two changed public signatures are `execute` and `Simulation::advance_tick`.** Rule 4's 2026-08-18 precedent
  covers the form: the sink amendment changed one signature the same way and this specification treated it as one
  parameter added rather than as an interface replaced. `Config` gains no field, so a caller that passes `None` twice is
  the caller that exists today.
- **`pub fn run` is not amended, and `pub(crate) fn run_recording` is.** `run` delegates with the port absent, so its
  enumerated form in rule 5's first list is unchanged and no existing caller of it sees anything move. The carrier that
  takes the port down the call chain is crate-private, and this row discloses that it changes rather than relying on its
  non-match silently — which is the disclosure the 2026-08-20 amendment made for the same carrier and the same reason.
  The consequence is that `grep -n 'pub fn .*&mut self' src/simulation.rs` still returns exactly `run` and
  `advance_tick`: the interface still has exactly two mutating methods and both are still simulation steps.
- **Rule 6 does not move.** The prohibited public interface stands as written: the request crosses as values, so no
  public item yields a mutable borrow of, or a reference into, authoritative state. Decision 1 is a use of rule 6, not an
  exception to it. This row exists to record that the rule was checked and holds, because a reader would otherwise expect
  it to have been relaxed.
- **Rule 13, the declared dependency set, does not move.** It records "The table is empty" and it still will. Decision 3
  is why, and `VER-MOK-018`'s `S1` measures it rather than asserting it.
- ***Security and privacy properties* is amended in its first bullet.** It reads "No network access, credential read,
  filesystem access, environment read, or wall-clock read is introduced." Of the engine **package**, three of those five
  stop holding: the binary target spawns a process, passes its environment through to the child, and interprets two more
  operator-supplied paths. Of the **library target** all five continue to hold, and the amendment says so in those terms —
  the sentence gains the target scope it has not needed until now. No wall-clock read is added by anything here, and no
  credential is read by either target: `SPEC-MOK-007` rules 10.5 and 13.4 place the credential in the connector alone.
- ***Actors and external systems* gains the connector**, as a process the binary target starts and the library never sees.
  The section's closing bullet, "No external service, network endpoint, credential, or filesystem location participates",
  is amended to the same target scope for the same reason.
- Amendment record row.

### `SPEC-MOK-003` — technical owner

This section did not exist in this ADR's first draft, and its absence was the largest hole in the pack: the observer is a
host of the new source, and its specification said nothing about it.

- **Rule 11's authority mapping**: the `decision_source_selected` row gains `REQ-MOK-063` for the fifth value, beside
  `REQ-MOK-008`, `REQ-MOK-015`, `REQ-MOK-033` and `REQ-MOK-057`. The 2026-08-19 amendment record states why this row is
  not optional — "that mapping is exhaustive by construction — the observer resolves it in a `match` over the policy — so
  a third source without a row is a gap the compiler reaches before an operator does."
- ***Start-up inputs* is amended in the paragraph added on 2026-08-22**, which enumerates what the forwarding accepts and
  the observer does not act on. Today that is two options; after this change the four new engine options pass through the
  same forwarding, so the paragraph gains them with each one's disposition: the transcript path is **acted on**, and the
  connector path, the live-mode selection and the ceiling are **refused with a diagnostic**. The paragraph's own reasoning
  is what forces the diagnosis rather than silence: "an operator who passes the option and receives no file and no
  diagnostic is worse served by silence."
- **The `--events-path` bullet and GitHub issue 40 are untouched.** This change neither closes nor worsens that defect. It
  is named here because the temptation is to fix it in passing, and closing it is "a governed change of its own" by that
  paragraph's own words.
- **The usage text's byte-identity obligation extends to the new shared options.** The observer's descriptions of
  `--seed`, `--ticks`, `--policy` and `--density` are the engine's byte for byte, held by
  `mokiterions-tui/tests/options.rs`. `--policy`'s description gains the fifth value, so the observer's text takes that
  same text; the transcript path's description is likewise the engine's. What the observer states **in its own words** is
  only what is its own: that this host replays and does not run live. That division is the paragraph's rule — "the
  observer may not restate a shared input's meaning in words of its own" — and a host-capability statement is not a shared
  input's meaning.
- ***Actors and external systems* needs no amendment, and this is worth stating.** Its sentence "No network, credential,
  model provider, database, or asynchronous runtime is involved in either component" **continues to hold**, because the
  replay host reads a file the repository commits. A transcript is provider-derived data at rest, not a provider involved
  at run time, and the distinction is the same one that lets committed evidence be replayed in continuous integration for
  free. If the observer ever ran live this sentence would fall, which is a second reason rule 20.1's split is written into
  a specification rather than left as a habit.
- **Rules 6.1 and 6.2 do not move.** The frame and input budgets are cited as the reason the observer cannot run live;
  they are not relaxed, scoped or excepted. Decision 8 is what keeps them intact.
- **The *Declared dependency set* does not move.** Its one entry stays `ratatui 0.30.2`, and its sentence that "Every
  other crate in the observer's resolved graph is reached transitively through `ratatui`" stays true — which option 3a
  would have falsified.
- Amendment record row.

### `SPEC-MOK-004` — technical owner

- The census of decision sources and policy values it carries gains one member, in each table and paragraph that
  enumerates them. The figures are **measured against the tree at the candidate commit and never inferred from an
  unchanged total**, which is the discipline the specification's own reconciliation already requires.
- **Rule 1's repository layout does not move, and neither does rule 2's workspace manifest.** This is decision 3's whole
  effect on this specification and it is stated positively because two earlier drafts of this ADR moved one or the other.
  Rule 1 admits the root's "workspace manifest, the lock file, the repository-level configuration and documentation, and
  no package" and closes with "No third package directory, no nested workspace, and no directory holding the sources of
  more than one package"; rule 2 fixes `members = ["mokiterions-core", "mokiterions-tui"]`. The connector is neither a
  package nor a directory in this repository — the operator names it by path from outside — so it is admitted by rule 1
  without an entry and prohibited by nothing in it. The one connector this repository does own, the canned connector of
  `SPEC-MOK-007` rule 20.5, is a test fixture inside an existing package and lands wherever `SPEC-MOK-002` rule 8 puts
  it.
- **Rule 11's test-count figures** move for every stage that adds a test, by that rule's own delegating clause. This is
  an obligation on each work order rather than a single amendment, and it is discharged where the tests land.
- Amendment record row.

### `ARCH-MOK-001` — technical owner

- The **component inventory** gains two entries: the decision port at the engine boundary, and the connector outside the
  repository entirely. The port is stated as sitting at the same boundary `ADR-MOK-001` fixed, not beside it. The
  connector is stated as **not a component of this system** but as the counterpart on the far side of a process boundary,
  in the way an inventory names a thing it depends on without owning — otherwise a reader would look for its source.
- The **prohibited-pattern list gains nothing, and no sentence of scope is added.** The prohibition on network access is
  already a prohibition on network access in the engine package, and the connector is in neither package. Nothing is
  relaxed, nothing is scoped and no exception is written. This row exists because the first draft of this ADR proposed
  adding a scoping sentence, and adding one would have implied the prohibition was in the way.
- The **dependency conformance check** — `ARCH-MOK-001`'s confirmation by name that the engine package's resolved graph
  holds no network, asynchronous-runtime, database, model-provider or user-interface crate — is **unchanged and continues
  to pass**, because there is no such crate to find. This is worth stating precisely, because the check is scoped to the
  engine *package* and the host of a live run is that package's own `[[bin]]` target: a crate admitted for the binary
  would enter the same graph the check scans. Decision 3 is what keeps the check passing, and `VER-MOK-018`'s `S1`
  measures it.
- The architecture's **determinism property** gains the pair *(seed, transcript)* for this source, referring to
  `SPEC-MOK-007` rule 12 for the property and to decision 2 for the reason.
- **`relations.addresses`** gains `REQ-MOK-063`, `REQ-MOK-067`, `REQ-MOK-068` and `REQ-MOK-077` — the four implemented
  requirements that are architecturally significant: the port at the boundary, the determinism property, the
  non-perturbation property, and the split of the two hosts. `REQ-MOK-077` earns an edge because which host may run live
  is a statement about the component inventory rather than about behaviour: it is the reason the engine's `[[bin]]` target
  is the only place the three new capabilities appear. The other eleven are behavioural or procedural and acquire no
  `addresses` edge, on the principle `WO-MOK-025`'s template states, that routine requirements do not require fabricated
  architecture coverage.
- **`relations.conforms_to`** gains `SPEC-MOK-007`.
- **`decision_assessment`** records this ADR: its `rationale` gains an amendment sentence naming the
  security-privacy-or-trust-boundary, public-interface-or-protocol,
  technology-framework-vendor-or-external-service and material-alternatives triggers as the ones this decision fires,
  and stating that engine authority, dependency direction, the trust boundary and the entropy stream are all untouched.
  The recorded trigger list does not gain a member, because all four are already present.
- Amendment record row.

### `ARCH-MOK-002` — technical owner

- The observer's authority mapping gains an entry for the fifth source, mapping it to `REQ-MOK-063` in the form the
  existing four entries take, and its hard-coded four-source description is corrected.
- **The observer becomes a host of this source in replay only**, per decision 8 and `SPEC-MOK-007` rule 20. This is the
  row the first draft of this ADR did not have, and its absence would have left the observer offering a source it cannot
  run. The observer gains one option, a transcript path, and it **refuses a selection of the new source without one**.
  It spawns no connector, reads no credential, takes no ceiling and has no live mode; the reason is `SPEC-MOK-003` rules
  6.1 and 6.2, whose 33-millisecond frame and 16-millisecond poll budgets an **estimated** 4-to-9-second live tick misses
  by two orders of magnitude. The observer therefore acquires knowledge of the transcript, and of nothing else on the far
  side of the port.
- **`relations.addresses` gains `REQ-MOK-063` and `REQ-MOK-077`.** The first is the source mapping and the replay option,
  which are two surfaces of one requirement. The second is this component's whole reason for changing: `REQ-MOK-077` is
  what makes the observer a replay host and what forbids it a live one, and an inventory entry that gained a capability
  with no requirement behind it would be a capability nobody asked for. `relations.conforms_to` **gains `SPEC-MOK-007`**,
  which the first draft said it would not: rule 20's obligations on the replay host are obligations on the observer, so
  its conformance now includes them. The observer's conformance to `SPEC-MOK-003` and `SPEC-MOK-004` is unaffected.
- **`decision_assessment`** records this ADR in its `rationale`, stating that no boundary, no dependency direction and
  no trust property moves, and that the observer's new capability is to read a file the repository already commits. Its
  trigger list does not gain a member.
- Amendment record row.

### `INT-MOK-001` — product owner

- The **determinism success measure** changes in one sentence: for the model-backed source the determinand is the seed
  **and the retained transcript**. Repeated runs at an identical seed and transcript give 100 percent identical results.
  The measure for the four existing sources does not change, and `REQ-MOK-009` is not amended, because the entropy
  stream is untouched.
- *Rationale*: decision 2. The sentence is the minimum change that keeps the measure true, and it is put to the product
  owner rather than written by the technical owner because a success measure is the product owner's.
- Amendment record row.

### `REPOSITORY_CONTEXT.md` — repository owner, no product or governance authority

- The existing sentence requiring model-provider credentials to remain outside the repository and not be committed is
  **unchanged and load-bearing**; it is cited by `REQ-MOK-072`, `REQ-MOK-073` and `SPEC-MOK-007` rule 11.6.
- What is added is the operational fact that follows from decision 5: the credential is not placed in the repository's
  automation secrets, and a live run is an owner-authorised manual act. This is a statement of how the repository is
  configured, not a governance rule, and `VER-MOK-018` records it as an owner attestation because no check can see it.

### No dependency artifact is amended — six rows deleted, recorded rather than dropped

Six amendment rows in this ADR's first draft were consequences of option 3a alone. Decision 3 removes all six, and they
are summarised here so that a later reader can see what the connector binding bought rather than having to reconstruct it.

- **`REQ-MOK-050` — product owner.** Its prohibitions on a crate providing network access, credential handling and an
  asynchronous runtime "in either package" would have needed a named exception, with the new package's crate set
  enumerated at declared versions and feature sets. **Untouched under decision 3.**
- **`ADR-MOK-006` — technical owner.** Its admission criteria would have been exercised for an **estimated** 40 to 60
  transitive crates, the largest admission this repository has considered, and its mechanical envelope would have had to
  be **reversed** rather than exercised: it excludes "no crate providing network access, credential handling, an
  asynchronous runtime, a database, a plugin system or dependency injection … however stable it is", which is checked
  before the criteria are reached. **Untouched under decision 3.**
- **`SPEC-MOK-002` rule 13 — technical owner.** Those crates would have entered the engine's declared set, which records
  "The table is empty", per `ARCH-MOK-001`'s statement that "the engine package's external dependency set is exactly the
  set declared for it in `SPEC-MOK-002`". **Untouched under decision 3.**
- **`ARCH-MOK-001`'s dependency conformance check — technical owner.** It would have had to gain a scope, because it
  confirms by name that the **engine package's** resolved graph holds no network, asynchronous-runtime, database,
  model-provider or user-interface crate — and the host of a live run is that package's own `[[bin]]` target, so the
  crates would have been in the graph the check scans. **Passes unamended under decision 3**, which is the row the first
  draft of this ADR got wrong: it recorded the check as already engine-scoped and therefore unaffected, without noting
  that the engine package is where the host lives.
- **`SPEC-MOK-003`'s declared dependency set — technical owner.** The observer path-depends on the engine package, and
  Cargo declares dependencies per package rather than per target, so the crates would have reached the observer's resolved
  graph too. That falsifies this specification's own sentence that "Every other crate in the observer's resolved graph is
  reached transitively through `ratatui`". **Untouched under decision 3.** This row did not exist in the first draft at
  all; it was found by tracing the observer's manifest.
- **`SPEC-MOK-004` rules 1 and 2 — technical owner.** A third workspace member is prohibited by rule 1's "No third package
  directory, no nested workspace" and absent from rule 2's `members` list. **Untouched under decision 3**, per the
  `SPEC-MOK-004` row above.

### `mokiterions-core/src/cli.rs` — source, no authority

- `USAGE` gains the fifth policy value with its own description, in the form the existing four take.
- **`USAGE`'s synopsis and option list gain the four new options** of `SPEC-MOK-007` rule 18.4: the connector path, the
  live-mode selection, the transcript path and the spend ceiling.
- **`parse` recognizes all four and retains none of the two path values**, on the `--events-path` precedent this file
  already implements and documents in place: it holds a `bool` rather than the path, because "`SPEC-MOK-006` rule 1.2
  keeps every path out of the library target, so this parser validates the option and forgets it; the binary target reads
  the argument it will open." `Config` gains no field. This is the mechanism that makes `SPEC-MOK-007` rules 10.9 and
  18.4 true without inventing anything.
- The existing sentence *"None of the four learns anything or calls a model; all four are deterministic"* becomes false
  when a fifth exists and is corrected in the same change. It is named here because a usage text that contradicts the
  program is the first defect a reader meets.

### `mokiterions-core/src/main.rs` — source, no authority

- It gains, for the connector path and the transcript path, the raw-argument re-read it already performs for
  `--events-path` at `events_path(arguments)`. The binary target spawns the connector, opens the transcript, builds the
  port and lends it to `execute`; it owns all three for the run's life and closes them after it.
- This is where the process spawn and the environment pass-through live, and nowhere else in either package.

### `mokiterions-tui/src/authority.rs` — source, no authority

- `for_type` gains the fifth source's mapping and `table` gains its row. The hard-coded four-source string is corrected.

### `mokiterions-tui/src/options.rs` and `state.rs` — source, no authority

- `options.rs` gains the observer's half of `SPEC-MOK-007` rule 18.4: it re-reads the raw transcript argument, and it
  **refuses** a connector path, a live-mode selection or a ceiling with the usage-error status and a message stating that
  this host replays only. Its own recognized-input set grows by nothing else, because the shared parser validates all four
  by the forwarding `SPEC-MOK-003`'s *Start-up inputs* already fixes.
- The refusal is not optional politeness. Without it the observer accepts a connector path and acts on nothing, which is
  the exact shape of the defect `SPEC-MOK-003`'s 2026-08-22 amendment recorded against `--events-path` and GitHub issue 40
  tracks. Repeating a known defect in the same file, in the same release, would be the worst-documented line in this pack.
- `state.rs` opens the transcript, builds the replay port once, holds it beside the `Simulation` for the run's life, and
  lends it to each `advance_tick`. It never rebuilds it per tick, per `SPEC-MOK-007` rule 20.4.1, and the `advance` method
  at `state.rs`'s tick step is the single place this is visible.
- `mokiterions-tui/tests/options.rs` holds the observer's four engine-option descriptions byte-identical to
  `mokiterions::cli::USAGE` today, so it moves with the usage text rather than after it.

## Consequences

### Positive

- **The trust boundary is unchanged.** The largest new capability this repository has taken on arrives through the
  narrowest interface it has: one function, values in, a value or nothing out.
- **A model-backed measurement is reproducible offline and free, forever.** A reader with the repository and a transcript
  reproduces a published figure byte for byte, with no credential, no network and no budget. That is a stronger
  reproducibility property than most work with language models has, and it comes from decision 2 alone.
- **Verification needs no money.** Every check in `VER-MOK-018` except the ones that inherently require a live run runs
  in continuous integration, free, against a committed transcript.
- **No dependency is added to either package** under decision 3, so `ADR-MOK-006`'s admission procedure, `REQ-MOK-050`'s
  prohibitions, `SPEC-MOK-002` rule 13's empty table, `ARCH-MOK-001`'s conformance scan and `SPEC-MOK-003`'s
  one-entry declared set all continue to hold unamended. The offline build story is unchanged, and six amendment rows this
  ADR's first draft carried are deleted.
- **The provider is swappable without a code change.** A different model, a different vendor or a local model is a
  different connector at a different path, which is an operator's argument rather than an amendment, a release or a crate.
  Neither package knows what is on the far side of the pipe.
- **The library target's guarantees survive verbatim.** It still resolves no path, opens no file, spawns no process and
  reads no environment variable, because both new streams arrive already open from the host that owns them. That is
  `SPEC-MOK-006` rule 1.2 extended to a second stream rather than excepted for one, and it is what keeps
  `SPEC-MOK-001`'s security property true rather than scoped.
- **The cost of the design is a number.** Decision 4 turns a tenfold price difference from a thing an author might
  remember into a check that fails.
- **A live run cannot happen by accident**, and the accident-proof default is also the mode verification uses, so there
  is no pressure to route around it.

### Negative

- **The transcript is large and is provenance.** An **estimated** 4.7 MB per 1,000-tick run, an **estimated** 23 MB for
  five seeds. Once a verification record binds one, its bytes and its path can never be corrected; a rename forces a
  fresh capture. `VER-MOK-018` must decide what is retained where before the first live run, not after.
- **Decision 3 moves a dependency surface outside the declared set, and outside this repository's reach entirely.** The
  connector's dependencies are not disciplined by `ADR-MOK-006`, and this ADR does **not** close the gap with a
  standard-library constraint, because a constraint on a program the repository neither builds nor ships is
  unenforceable — `SPEC-MOK-007` rule 10.6 withdraws it and says so. What the repository can constrain is the one
  connector it owns, the canned one of rule 20.5. This is the honest cost of decision 3 and it is larger than option 3a's
  on this axis alone.
- **The connector's output is untrusted in whole, including its usage counts.** The spend ceiling protects against an
  honest connector, not a dishonest one: a connector that under-reports usage spends past the ceiling and the run cannot
  tell. Rule 10.8 records this rather than defending against it, since the operator writes the connector, and the real
  containment is that automation holds no credential.
- **A second program is a second thing to build, test and version, and it is not in this repository.** The pipe protocol
  is a contract that can drift, and it can drift against a program under nobody's version control. Rule 10.2's
  one-JSON-object-per-line framing is deliberately the simplest contract that can carry the exchange, for that reason.
- **Two public signatures change — `execute` and `Simulation::advance_tick`** — so that both hosts can reach the source.
  `SPEC-MOK-002` rule 4's sink precedent makes this routine rather than novel, but it is still a change to an interface
  that has been stable, and a caller outside this repository — there is none today — would have to pass `None`. The cost
  is bounded by what does *not* change: `pub fn run` keeps its enumerated form, so the whole-run library entry point
  existing code is likeliest to call is source-compatible.
- **The prompt's shared rules block is a restatement of `SPEC-MOK-001` and can fall out of agreement with it.**
  `SPEC-MOK-007` rule 4.2 fixes which one governs, but nothing detects drift automatically, and `VER-MOK-018` must say
  how it is checked.
- **A run takes hours.** No concurrency is specified, because concurrent exchanges would make transcript order depend on
  timing and transcript order is what a replay consumes. The horizon of the first measurement is bounded by wall clock
  as much as by cost.
- **Decision 7 means the first measurement may produce a result nobody likes**, and there is no mechanism by which that
  becomes a failure. That is the intent; it is listed as a negative because it will feel like one.

### Operational and security

- The credential exists in exactly one process's environment, and that process is not the engine, not the observer and
  not any workflow. It is never in the working tree, never in the transcript, never in the record stream, never on either
  output stream and never in an error message.
- The repository's automation holds no credential. A compromised action, a fork pull request and a mistaken workflow
  edit can all fail to spend money, because there is nothing to spend.
- Spending is bounded before it happens, by a number the owner named in a retained authorization record.
- What leaves the repository during a live run is the request text: the world's rules, one Mokiterion's own state, and a
  list of actions. No source, no path, no identity and no repository content.
- No request carries any data about any Mokiterion other than the one deciding, so the isolation property is a privacy
  property of the population as well as an experimental one.
- **A process spawn and an environment pass-through are new capabilities, and both live in one binary target.** The engine
  binary starts the connector the operator named and may hand it the environment it inherited, which is how the credential
  reaches the connector without either host reading it. The library target does neither, and the observer does neither: it
  spawns nothing at all, per decision 8. An operator who names an arbitrary executable gets an arbitrary executable — this
  is the same trust the operator already places in the shell that started the run, and it is stated because "a path to a
  binary" is a capability worth naming rather than assuming.
- **The observer gains the ability to read one committed file and nothing else.** No credential, no socket, no child
  process, no ceiling and no live mode reach it.

### Migration

- Nothing migrates. The four existing sources are byte-identical, entropy draws included; no retained capture is retired
  and no published figure is invalidated. `SPEC-MOK-007` rule 16 states the property and `VER-MOK-018` measures it on
  both sides of the change.
- The `schema` increment in the structured record stream is the one compatibility event, and it is a consumer-visible
  one. There is no consumer in this repository.
- The staging is three work orders: `WO-MOK-025` builds the port, the request, the transcript, the replay and both hosts'
  wiring, with no connector at all and no cost; `WO-MOK-026` builds the live path, the accounting and the canned connector
  that exercises the live path offline, and takes the one small authorized instrument run; `WO-MOK-027` performs the
  authorized measurement. Everything verifiable without money is verified before any money is spent.
- **The two public signatures change in `WO-MOK-025`**, before any live path exists, so the interface settles once rather
  than twice. This is the ordering constraint that matters most in the staging, because a signature that moves in
  `WO-MOK-026` would move after `WO-MOK-025`'s verification record had already bound it.

## Validation

Conformance is checked by `VER-MOK-018`, and the checks that establish each decision are named here so that a reader can
see which decision is held by what.

| Decision | How conformance is checked |
|---|---|
| 1 — the port at the boundary | Static analysis of the engine's public surface: one interface and one request type added, no transport type, no reference or mutable borrow exposed. `SPEC-MOK-002` rule 6's existing check is re-run unchanged. |
| 1 — no mode branch | Static analysis: no live-versus-replay branch and no mode value in the library target. |
| 1 — two changed signatures, and only two | `SPEC-MOK-002` rule 5's mechanical drift checks, run in their **restated** form: `execute` matches rule 4's amended five-parameter literal, `grep -n 'pub fn .*&mut self' src/simulation.rs` returns exactly `run` and `advance_tick`, and `Simulation::run`'s enumerated form is unchanged. The restatement is itself part of what is checked, because the standing text of check 2 makes "a fifth parameter" a failure and the port is that parameter — so a build that adds the port and leaves the check as written is condemned by its own interface authority, silently. `VER-MOK-018`'s `S4a`. Free, offline, every push. |
| 2 — replay byte-identity | A recorded run and a replay of it are compared with `cmp` on standard output, on the record stream and on the exit code, with no credential in the environment and no network reachable. |
| 2 — mismatch detection | A replay at a different seed fails and names the mismatch, rather than producing a run. |
| 3 — no dependency added | The engine's and the observer's resolved dependency graphs are compared against their declared sets and found unchanged — the engine's empty, the observer's one entry reached transitively through `ratatui`. `ARCH-MOK-001`'s existing scan for a network, asynchronous-runtime, database, model-provider or user-interface crate is re-run over the engine package, whose `[[bin]]` target is the recording host, and continues to find none. Free, offline, every push. |
| 3 — the connector is not in this repository | Static analysis: no third workspace member, no third package directory, no connector source, and no default connector path compiled in. The only executable this repository owns that speaks the protocol is the canned connector, and it reaches no network. |
| 3 — the connector's surface is unconstrained | **Not checked, and the absence is the finding.** Rule 10.6 withdraws the standard-library constraint as unenforceable against a program the repository does not build. What is checked is the canned connector's dependency declaration, and the report states plainly that this establishes nothing about an operator's connector. |
| 4 — the cache ratio | Summed from the provider's reported usage over a live run of at least 200 exchanges, and held at 0.85 or above. Requires a live run; owner-gated. |
| 4 — the layout | Static analysis over a retained transcript: the shared block is byte-identical across every request of the run, and the shared-plus-actor prefix is byte-identical across every request for one Mokiterion. Runs against a committed transcript, free. |
| 5 — the two conditions | A run with the flag and no credential makes no call; a run with a credential and no flag makes no call. Verified with no credential ever present. |
| 5 — automation holds none | Static analysis over the repository's workflow definitions: no credential reference, no live-mode selection. Plus one owner attestation, that the secret is not configured, which no check can see. |
| 5 — the ceiling | A run declared with a ceiling below its expected cost stops before exceeding it, with a distinct exit status. Verified against a stubbed port with declared prices, free. |
| 6 — the fallback | An unanswered and an unparseable exchange each yield `wait`, increment the count, and mark the run. A proposal the rules reject does neither. |
| 7 — no floor | Static analysis: no survivor threshold, death ceiling or outcome assertion exists for this source anywhere in the verification suite. The check is that an assertion is **absent**, which is stated as a check because an absence nobody looks for is indistinguishable from an oversight. |
| 7 — no strategy in the prompt | Manual assessment of the shared rules block against rule 4.4's prohibitions, by the assurance owner, recorded as an assessment rather than a test. |
| 8 — both doors wired | The canned connector drives a live-path run through `execute`, and a replay drives the same source through `Simulation::advance_tick`, in the same suite. Wiring one and not the other fails here rather than at the first host that tries. Free, offline, every push. |
| 8 — the port is lent, not rebuilt | A replay of more than one tick through the single-tick entry point consumes successive transcript records, and a stubbed live run's accumulated cost rises across ticks and trips a low ceiling. A port rebuilt per tick passes neither: the cursor restarts and the cost stays at zero. This is the check that catches the compiling, running, silent version of the defect. Free. |
| 8 — the library owns neither stream | Static analysis of the library target: no path resolution, no file open, no process spawn, no environment read. Both streams enter as already-open handles. `SPEC-MOK-001`'s and `SPEC-MOK-006` rule 1.2's existing property is re-measured rather than assumed. Free. |
| 8 — the observer replays and refuses live | The observer given this source with a transcript replays to completion; given it without one, or given a connector path, a live-mode selection or a ceiling, it exits with the usage-error status and a message naming the reason. It never falls back to another source and it never accepts silently, which is what distinguishes this from GitHub issue 40. Free. |
| 8 — no port is an invalid configuration | This source selected with no port supplied refuses, per rule 20.8, rather than substituting a source or producing no decisions. Free. |
| 8 — the frame budget is unrelaxed | `SPEC-MOK-003` rules 6.1 and 6.2 are unamended, and the observer's existing frame and input timing checks are re-run unchanged. Decision 8 is held by leaving them alone, so the check is that they still pass. Free. |

Two limits of this validation are stated rather than left to be found. **A green continuous-integration run does not mean
a model was consulted** — decision 5 guarantees it was not — so the ratio check and the measurement itself are
owner-gated manual acts, and `VER-MOK-018` marks them as such. And **there is no outcome oracle**: no check in the table
above asserts anything about what the population does, because decision 7 removed the only ground on which such an
assertion could stand. `VER-MOK-018` records that absence as a decision with this ADR's rationale, so that a later
reader does not read it as a gap.
