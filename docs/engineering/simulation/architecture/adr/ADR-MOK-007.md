+++
id = "ADR-MOK-007"
type = "adr"
title = "A decision port at the existing trust boundary, a retained transcript as the second determinand of a run, and the model provider outside the workspace"
status = "draft"
owners = ["technical owner"]
created = "2026-08-23"
updated = "2026-08-23"

[relations]
decides = ["ARCH-MOK-001", "ARCH-MOK-002"]
+++

# ADR: A decision port at the existing trust boundary, a retained transcript as the second determinand of a run, and the model provider outside the workspace

## Status

Proposed. It decides `ARCH-MOK-001` and `ARCH-MOK-002`, and it is a precondition of `WO-MOK-025`, `WO-MOK-026` and
`WO-MOK-027`.

Three decisions are taken together because they are one shape and separating them would leave each incoherent. Decision
1 puts the port where the trust boundary already is; decision 2 makes the provider's answers a run input rather than a
source of non-determinism, which is only possible because of where decision 1 put the port; decision 3 keeps the
transport outside the workspace, which is only harmless because decision 1 gave the engine an interface that names no
transport. Accepting one without the others produces a design this ADR does not describe.

**One part of decision 3 is put to the owner as a choice rather than as a recommendation carried out.** Both bindings are
stated in full under *Considered options*, and *Required amendments* marks which rows exist only under the binding this
ADR does not recommend.

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

**Option 3a: an HTTPS client inside the workspace.** A third package, or the observer package, gains an HTTP client, a
TLS stack and probably an asynchronous runtime.

| | 3a — in the workspace | 3b — a separate program over pipes |
|---|---|---|
| Crates added | An **estimated** 40 to 60 transitive crates | None |
| `REQ-MOK-050` | Amended: the network-access and credential-handling prohibitions must admit an exception | Untouched |
| `ADR-MOK-006` | Its declared set and admission criteria must be exercised for the largest admission yet | Untouched |
| `ARCH-MOK-001`'s conformance check | Amended: the scan for a model-provider crate in the engine graph must gain a scope | Untouched |
| What the host needs | The client's API | `std::process` and `std::io` |
| Credential holder | A workspace package | The provider program only |
| Dependency discipline | Inside the declared set | **Outside it** unless something holds it |
| Offline build | Unaffected for the engine; the new package needs the network at build time | Unaffected |

**Option 3b: a separate provider program the host drives over pipes.** Recommended, as decision 3. It costs no crate, no
amendment to `REQ-MOK-050` and no amendment to `ADR-MOK-006`, and it keeps the credential in a component that touches
nothing else.

Its honest cost is the row in bold: the provider program has its own dependencies, and those are not in the declared set
`ADR-MOK-006` disciplines. Decision 3 therefore carries a constraint that closes it — the program's dependency surface
is its language's standard library — and that constraint is what makes the comparison a fair one rather than a
relocation of the problem.

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
grows by one interface and one request type and by nothing else. The four existing sources do not move onto the port.

**Decision 2 — the transcript is the second determinand of a run.** A live run records every exchange; a replay obtains
every decision from that record and makes no provider call. Determinism is claimed for the pair *(seed, transcript)*
rather than for the seed alone. There is no mode branch in the engine: the difference between recording and replaying is
which stream the host connected, so byte-identity is structural rather than two implementations to keep in agreement.
`REQ-MOK-009` does not move, because the entropy stream is untouched — this source draws from it not at all.

**Decision 3 — the provider lives outside the workspace, driven as a separate program over pipes, and its dependency
surface is its language's standard library.** The host writes one request object per line to the program's standard input
and reads one response object per line from its standard output. The provider program reads the credential from its own
environment and is the only component that holds one. No crate is added to either package; `REQ-MOK-050`,
`ADR-MOK-006` and `ARCH-MOK-001`'s dependency conformance check are untouched.

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

## Required amendments

Each is stated in full so that accepting this ADR accepts the change. **Two of them are not the technical owner's**:
`INT-MOK-001` is the product owner's and `REPOSITORY_CONTEXT.md` is the repository owner's. `WO-MOK-025` makes every row
below an approval precondition, as `WO-MOK-014` did for `ADR-MOK-006`.

**No amendment is made in this packet.** Every artifact named below is `approved`, and amending an approved artifact is
an owner act; these rows state what must move and leave the moving to the owner, on `ADR-MOK-006`'s and `WO-MOK-024`'s
precedent.

Rows marked **only under option 3a** do not exist if the owner accepts decision 3 as recommended. They are stated so
that the two bindings can be compared on their full cost rather than on their code.

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
- Amendment record row in the form the specification already uses, whose *Approval* cell names the technical owner and
  this ADR.

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

- **Rule 5, the authorized public interface**, gains the decision port's interface and the decision request type, and
  nothing else.
- **Rule 6 does not move.** The prohibited public interface stands as written: the request crosses as values, so no
  public item yields a mutable borrow of, or a reference into, authoritative state. Decision 1 is a use of rule 6, not an
  exception to it. This row exists to record that the rule was checked and holds, because a reader would otherwise expect
  it to have been relaxed.
- **Rule 13, the declared dependency set, does not move** under decision 3 as recommended. Under option 3a it does; see
  that row.
- Amendment record row.

### `SPEC-MOK-004` — technical owner

- The census of decision sources and policy values it carries gains one member, in each table and paragraph that
  enumerates them. The figures are **measured against the tree at the candidate commit and never inferred from an
  unchanged total**, which is the discipline the specification's own reconciliation already requires.
- **Rule 1's repository layout gains the provider program's directory**, under decision 3 as recommended. This is the
  amendment most easily missed, because the program is not a Rust package and the rule's prohibition is on packages.
  Rule 1 states that the root "holds the workspace manifest, the lock file, the repository-level configuration and
  documentation, and no package", and closes with "No third package directory, no nested workspace, and no directory
  holding the sources of more than one package." A directory holding a program in another language is admitted by none
  of that and prohibited by none of it, so the amendment **adds the entry and states that it is not a package**, leaving
  the closing prohibition intact rather than weakening it. Under option **3a** this bullet does not apply and a
  different one does: the provider becomes a third member of the workspace, which rule 1's closing sentence and rule
  2's `members` list both prohibit today, and relaxing that is a materially larger amendment than adding a
  non-package directory.
- **Rule 11's test-count figures** move for every stage that adds a test, by that rule's own delegating clause. This is
  an obligation on each work order rather than a single amendment, and it is discharged where the tests land.
- Amendment record row.

### `ARCH-MOK-001` — technical owner

- The **component inventory** gains two entries: the decision port at the engine boundary, and the host's provider
  binding outside it. The port is stated as sitting at the same boundary `ADR-MOK-001` fixed, not beside it.
- The **prohibited-pattern list** gains one sentence of scope, not an exception: the prohibition on network access is a
  prohibition on network access *in the engine package*, which is what it already says, and the provider program of
  decision 3 is not in either package. Under decision 3 as recommended, nothing is relaxed.
- The **dependency conformance check** — the scan for a model-provider crate in the engine's graph — is unchanged and
  continues to pass. Under decision 3 there is no such crate to find.
- The architecture's **determinism property** gains the pair *(seed, transcript)* for this source, referring to
  `SPEC-MOK-007` rule 12 for the property and to decision 2 for the reason.
- **`relations.addresses`** gains `REQ-MOK-063`, `REQ-MOK-067` and `REQ-MOK-068` — the three implemented requirements
  that are architecturally significant: the port at the boundary, the determinism property, and the non-perturbation
  property. The other eleven are behavioural or procedural and acquire no `addresses` edge, on the principle
  `WO-MOK-025`'s template states, that routine requirements do not require fabricated architecture coverage.
- **`relations.conforms_to`** gains `SPEC-MOK-007`.
- **`decision_assessment`** records this ADR: its `rationale` gains an amendment sentence naming the
  security-privacy-or-trust-boundary, public-interface-or-protocol,
  technology-framework-vendor-or-external-service and material-alternatives triggers as the ones this decision fires,
  and stating that engine authority, dependency direction, the trust boundary and the entropy stream are all untouched.
  The recorded trigger list does not gain a member, because all four are already present.
- Amendment record row.

### `ARCH-MOK-002` — technical owner

- The observer's authority mapping gains an entry for the fifth source, mapping it to `REQ-MOK-063` in the form the
  existing four entries take, and its hard-coded four-source description is corrected. The observer acquires no
  knowledge of the provider, the transcript or the credential; it names a source, as it already does.
- **`relations.addresses` gains `REQ-MOK-063`** and nothing else, because the mapping is the only surface of this work
  the observer holds. `relations.conforms_to` does not gain `SPEC-MOK-007`: the observer's conformance is to
  `SPEC-MOK-003` and `SPEC-MOK-004`, and the mapping change is a `SPEC-MOK-004` matter.
- **`decision_assessment`** records this ADR in its `rationale`, stating that no boundary, no dependency direction and
  no trust property moves. Its trigger list does not gain a member.
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

### `REQ-MOK-050` — product owner. **Only under option 3a**

- The prohibitions on a crate providing network access, credential handling and an asynchronous runtime must gain a
  named exception for the provider package, with that package named and its declared crate set enumerated at declared
  versions and feature sets.
- Under decision 3 as recommended, this requirement is untouched. The row is stated so that the owner sees that option
  3a costs a product-owner amendment to a static-analysis requirement, not only a technical decision.

### `ADR-MOK-006` and `SPEC-MOK-002` rule 13 — technical owner. **Only under option 3a**

- `ADR-MOK-006`'s admission criteria must be exercised for an **estimated** 40 to 60 transitive crates — the largest
  admission this repository would have made — and its mechanical envelope must admit the new package.
- The declared dependency set those crates enter is `SPEC-MOK-002` rule 13's, per `ARCH-MOK-001`'s statement that *"the
  engine package's external dependency set is exactly the set declared for it in `SPEC-MOK-002`"*. Each crate is
  enumerated at its declared version and feature set.
- `ARCH-MOK-001`'s conformance check that the resolved graph *"contains no network, asynchronous-runtime, database,
  model-provider or user-interface crate"* must gain a scope, because under 3a it would fail as written.
- Under decision 3 as recommended, all three are untouched.

### `mokiterions-core/src/cli.rs` — source, no authority

- `USAGE` gains the fifth policy value with its own description, in the form the existing four take.
- The existing sentence *"None of the four learns anything or calls a model; all four are deterministic"* becomes false
  when a fifth exists and is corrected in the same change. It is named here because a usage text that contradicts the
  program is the first defect a reader meets.

### `mokiterions-tui/src/authority.rs` — source, no authority

- `for_type` gains the fifth source's mapping and `table` gains its row. The hard-coded four-source string is corrected.

## Consequences

### Positive

- **The trust boundary is unchanged.** The largest new capability this repository has taken on arrives through the
  narrowest interface it has: one function, values in, a value or nothing out.
- **A model-backed measurement is reproducible offline and free, forever.** A reader with the repository and a transcript
  reproduces a published figure byte for byte, with no credential, no network and no budget. That is a stronger
  reproducibility property than most work with language models has, and it comes from decision 2 alone.
- **Verification needs no money.** Every check in `VER-MOK-018` except the ones that inherently require a live run runs
  in continuous integration, free, against a committed transcript.
- **No dependency is added** under decision 3, so `ADR-MOK-006`'s declared set, `REQ-MOK-050`'s prohibitions and
  `ARCH-MOK-001`'s conformance check all continue to hold unamended. The offline build story is unchanged.
- **The cost of the design is a number.** Decision 4 turns a tenfold price difference from a thing an author might
  remember into a check that fails.
- **A live run cannot happen by accident**, and the accident-proof default is also the mode verification uses, so there
  is no pressure to route around it.

### Negative

- **The transcript is large and is provenance.** An **estimated** 4.7 MB per 1,000-tick run, an **estimated** 23 MB for
  five seeds. Once a verification record binds one, its bytes and its path can never be corrected; a rename forces a
  fresh capture. `VER-MOK-018` must decide what is retained where before the first live run, not after.
- **Decision 3 moves a dependency surface outside the declared set.** The provider program's dependencies are not
  disciplined by `ADR-MOK-006`. Decision 3's standard-library constraint is what closes this, and that constraint is a
  real limit: it rules out using a vendor SDK in the provider program.
- **A second program is a second thing to build, test and version.** Option 3a would have one artifact where decision 3
  has two, and the pipe protocol is a contract that can drift.
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

### Migration

- Nothing migrates. The four existing sources are byte-identical, entropy draws included; no retained capture is retired
  and no published figure is invalidated. `SPEC-MOK-007` rule 16 states the property and `VER-MOK-018` measures it on
  both sides of the change.
- The `schema` increment in the structured record stream is the one compatibility event, and it is a consumer-visible
  one. There is no consumer in this repository.
- The staging is three work orders: `WO-MOK-025` builds the port, the request, the transcript and the replay with no
  provider at all and no cost; `WO-MOK-026` builds the provider program and the live path; `WO-MOK-027` performs the
  authorised measurement. Everything verifiable without money is verified before any money is spent.

## Validation

Conformance is checked by `VER-MOK-018`, and the checks that establish each decision are named here so that a reader can
see which decision is held by what.

| Decision | How conformance is checked |
|---|---|
| 1 — the port at the boundary | Static analysis of the engine's public surface: one interface and one request type added, no transport type, no reference or mutable borrow exposed. `SPEC-MOK-002` rule 6's existing check is re-run unchanged. |
| 1 — no mode branch | Static analysis: no live-versus-replay branch and no mode value in the library target. |
| 2 — replay byte-identity | A recorded run and a replay of it are compared with `cmp` on standard output, on the record stream and on the exit code, with no credential in the environment and no network reachable. |
| 2 — mismatch detection | A replay at a different seed fails and names the mismatch, rather than producing a run. |
| 3 — no dependency added | The engine's and the observer's dependency graphs are compared against the declared set, unchanged. `ARCH-MOK-001`'s existing scan for a model-provider crate is re-run and continues to find none. |
| 3 — the provider program's surface | Static analysis of that program's own dependency declaration against its language's standard library. |
| 4 — the cache ratio | Summed from the provider's reported usage over a live run of at least 200 exchanges, and held at 0.85 or above. Requires a live run; owner-gated. |
| 4 — the layout | Static analysis over a retained transcript: the shared block is byte-identical across every request of the run, and the shared-plus-actor prefix is byte-identical across every request for one Mokiterion. Runs against a committed transcript, free. |
| 5 — the two conditions | A run with the flag and no credential makes no call; a run with a credential and no flag makes no call. Verified with no credential ever present. |
| 5 — automation holds none | Static analysis over the repository's workflow definitions: no credential reference, no live-mode selection. Plus one owner attestation, that the secret is not configured, which no check can see. |
| 5 — the ceiling | A run declared with a ceiling below its expected cost stops before exceeding it, with a distinct exit status. Verified against a stubbed port with declared prices, free. |
| 6 — the fallback | An unanswered and an unparseable exchange each yield `wait`, increment the count, and mark the run. A proposal the rules reject does neither. |
| 7 — no floor | Static analysis: no survivor threshold, death ceiling or outcome assertion exists for this source anywhere in the verification suite. The check is that an assertion is **absent**, which is stated as a check because an absence nobody looks for is indistinguishable from an oversight. |
| 7 — no strategy in the prompt | Manual assessment of the shared rules block against rule 4.4's prohibitions, by the assurance owner, recorded as an assessment rather than a test. |

Two limits of this validation are stated rather than left to be found. **A green continuous-integration run does not mean
a model was consulted** — decision 5 guarantees it was not — so the ratio check and the measurement itself are
owner-gated manual acts, and `VER-MOK-018` marks them as such. And **there is no outcome oracle**: no check in the table
above asserts anything about what the population does, because decision 7 removed the only ground on which such an
assertion could stand. `VER-MOK-018` records that absence as a decision with this ADR's rationale, so that a later
reader does not read it as a gap.
