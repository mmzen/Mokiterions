+++
id = "WO-MOK-025"
type = "work_order"
title = "Stage 5a: the decision port, the request, the transcript and the replay — everything the model-backed source needs, with no provider and no cost"
status = "draft"
owners = ["engineering owner"]
created = "2026-08-23"
updated = "2026-08-23"

[assurance]
commit_bound_verification = "required"
rationale = "This work order changes executable engine behavior, grows the engine's public interface for the first time since `WO-MOK-019`, and adds a fifth arm to the decision-source selection that every published figure in this repository is downstream of. Three claims here cannot be asserted by inspection. That the four existing sources are byte-identical, entropy draws included, is a claim about twenty configurations at every declared seed rather than about a diff, and `INT-MOK-010` carries the promise for `baseline` specifically. That a replay reproduces a recorded run byte for byte is the property `INT-MOK-001`'s amended determinism measure will rest on, and it is a claim about whole streams. That no request carries another Mokiterion's state and no request carries an earlier exchange are properties of bytes that were sent, checkable only over a retained transcript. The work also commits the transcript that continuous integration will replay for the life of this initiative, which becomes provenance the moment a record binds it, and it obliges amendments to four approved specifications, two approved architectures and one approved intent — so the artifacts a later reader would cite as the oracle are themselves part of what changes."
decided_by = "engineering owner"

[relations]
implements = [
  "REQ-MOK-063",
  "REQ-MOK-064",
  "REQ-MOK-065",
  "REQ-MOK-066",
  "REQ-MOK-067",
  "REQ-MOK-068",
  "REQ-MOK-073",
  "REQ-MOK-074",
  "REQ-MOK-077",
]
specifications = ["SPEC-MOK-001", "SPEC-MOK-002", "SPEC-MOK-003", "SPEC-MOK-004", "SPEC-MOK-007"]
verification = ["VER-MOK-018"]
architecture = ["ARCH-MOK-001", "ARCH-MOK-002", "ADR-MOK-007"]
+++

# Work Order: Stage 5a — the port, the request, the transcript and the replay

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below, and **it is the same act as the approval of every row in `ADR-MOK-007`'s *Required amendments* that this stage
needs** — the amendments to `SPEC-MOK-001`, `SPEC-MOK-002`, `SPEC-MOK-003`, `SPEC-MOK-004`, `ARCH-MOK-001`,
`ARCH-MOK-002` and `INT-MOK-001`. Five of those are prerequisites of the change rather than consequences of it: without
them the change contradicts an approved artifact on the day it lands. Transition to `in_progress` records that implementation has begun.
Transition to `implemented` requires the completed change and the retained evidence. Verification requires a separate
commit-bound record.

**The definition-layer artifacts this work order implements are all `draft` and none has been approved.** Nothing in
this stage may be started before `INT-MOK-011`, `CAP-MOK-011`, `REQ-MOK-063` through `REQ-MOK-077`, `SPEC-MOK-007`,
`ADR-MOK-007` and `VER-MOK-018` have been approved by the roles that own them. In this repository one person holds all
three governance roles, which means nothing here is approved by implication: the approval of the packet is a distinct
act from the approval of this work order.

**No provider is contacted in this stage and nothing is spent.** That is not a limitation of the stage, it is its point.
Everything `VER-MOK-018` can check without money is checked before any money can be spent, and the live path does not
exist yet to be used by accident.

## Objective

Build the fifth decision source's whole structure with the provider replaced by a scripted stub and by a retained
transcript: one port at the engine boundary, wired into **both** of rule 20.5's two doors, the cache-ordered
request, the complete action enumeration, the transcript format, the replay, the fallback accounting, the command-line
and observer surfaces, and the workflow check that keeps a provider credential out of continuous integration — while
leaving the four existing decision sources byte-identical.

At the end of this stage the model-backed source runs end to end, offline and free, **in both hosts**, and twenty-eight
of `VER-MOK-018`'s thirty-five cases pass in continuous integration, with two more passing in the half that needs no
live path. That the free set is this large is the owner's decision of 2026-08-23 that a short committed transcript may be
replayed in automation — verification tier 1 — recorded in `ADR-MOK-007`'s *Decision record* beside the decision that
keeps the provider itself out of automation entirely.

## In scope

1. **The decision port.** One interface on the engine's public surface, taking a decision request by value and returning
   a proposal or the absence of one, per `SPEC-MOK-007` rules 1 and 2. No transport type, no mode value, no branch on
   live-versus-replay in the library target.

   **The port reaches both of rule 20.5's doors**, per `SPEC-MOK-007` rules 20.4 and 20.5: one new optional parameter
   each, of the same borrowed shape `SPEC-MOK-002` rule 4 already fixes for the record sink. The two public signatures
   that change here are **`execute`**, which reaches five parameters, and **`Simulation::advance_tick`** — the process
   boundary the recording host drives a whole run through, and the single tick the replay host advances. They change here,
   before any live path exists, which is the cheapest moment in this initiative for a signature change.

   Three things follow, and each is a place a plausible implementation goes wrong. **`pub fn run` is not amended**: it
   delegates with the port absent, its enumerated form in `SPEC-MOK-002` rule 5's first list stands, and adding the
   parameter to it would grow the interface by something no approved artifact authorizes. **`pub(crate) fn run_recording`
   does take the port**, as the crate-private carrier down the call chain, exactly as it carries the record sink today;
   `ADR-MOK-007` discloses this so it is an expected diff rather than an undeclared third signature change. And
   **`SPEC-MOK-002` rule 5's mechanical checks must be updated in the same commit as the code**, because the standing text
   reads "A fifth parameter, a second sink, or a sink that is not optional fails the second" and the port on `execute` is
   that fifth parameter — a build that adds the port and leaves the check as written fails its own specification. The
   amendment `ADR-MOK-007` authorizes restates the checks; writing the code without writing the restatement is the defect
   to avoid. `grep -n 'pub fn .*&mut self' src/simulation.rs` must still return exactly `run` and `advance_tick`.

   The host builds the port, owns it for the run, and lends it per tick; the library builds none, holds none and closes
   none, which is `SPEC-MOK-006` rule 1.2 satisfied rather than excepted. Rule 20.8's refusal — this source selected with no port
   supplied is an invalid configuration — is built here, and it is the one check of rule 13 that the library rather than a
   host makes.
2. **Request composition** in the cache order `SPEC-MOK-007` rules 3 through 7 fix: the shared rules block, the actor
   block, the observation block, the enumerated action set. Including the shared rules block's prose, held in exactly
   one place in the source.
3. **The complete enumeration**, composed beside the observation's core-proposal list and never by extending it, per
   rule 7 — every core proposal, `eat` per co-located resource, each valid cardinal move, and each of the seven targeted
   verbs against each identifier whose precondition it satisfies.
4. **The transcript**: its records, its framing and its constraints, per rule 11. Written by the engine to a stream the
   host opens.
5. **Replay**, per rule 12: reading decisions from a transcript through the same port and the same code path, with
   mismatch and exhaustion detection.
6. **The fallback**: `wait`, counted, recorded, and marking the run, per rule 9 — and the distinction from an ordinary
   rejected proposal, which is not counted.
7. **The run record's** structure and its fields, per rule 15, with the accounting figures present and zero where
   nothing was spent. The cost arithmetic is built and exercised against declared prices and synthetic usage; no real
   usage exists yet.
8. **The command-line surface**, per rule 18: the fifth policy value, its help text, the correction of the sentence
   *"None of the four learns anything or calls a model; all four are deterministic"*, and the host options for the
   transcript path and the replay selection. **The live-mode flag, the credential read and the ceiling option are out of
   scope**; see below.
9. **The observer as a replay host**, per `REQ-MOK-077` and `SPEC-MOK-007` rules 12.1.1 and 18.4: the fifth value
   accepted, the transcript option accepted and the file opened by the observer, the already-open reader lent to the
   engine's single-tick entry point, and every pane, key binding and export behaving as it does under `social`. Plus the
   **refusals**: this source with no transcript, and — for the options `WO-MOK-026` adds — a diagnostic rather than
   silence, per rule 18.4.2. The observer forwards options it does not recognise to the engine's shared parser, so an
   option that parser now accepts reaches the observer as *accepted and ignored* unless the observer diagnoses it. That
   outcome is the defect `SPEC-MOK-003`'s *Start-up inputs* discloses for `--events-path` and GitHub issue 40 tracks;
   repeating it in the same file would be a known defect knowingly added.
10. **The observer's authority mapping**: the fifth entry and the correction of the hard-coded four-source description.
11. **The workflow check** for `REQ-MOK-073`: no workflow references a model-provider credential and none selects live
    mode, plus the workflow step that replays the committed transcript.
12. **A committed transcript** produced by the scripted stub, covering a run in which every Mokiterion acts, targeted
    actions are enumerated, food is and is not co-located, and at least one Mokiterion dies — the coverage
    `VER-MOK-018`'s residual-uncertainty section says the transcript-reading cases rest on.
13. **Base-commit captures** of all four existing sources at every declared seed, taken **before** any change is made,
    with their digests. Without these `REQ-MOK-068` cannot be checked at all.
14. The amendments `ADR-MOK-007` requires of `SPEC-MOK-001`, `SPEC-MOK-002`, `SPEC-MOK-003`, `SPEC-MOK-004`,
    `ARCH-MOK-001`, `ARCH-MOK-002` and `INT-MOK-001`, each written as the approved amendment text and each with its
    amendment record row. `SPEC-MOK-004`'s part is the fifth decision source in every table and paragraph that enumerates
    them, plus **rule 11**'s test-count figures for the tests this stage adds — every figure measured against the tree at
    the candidate commit and none inferred from an unchanged total. Rule 1's layout does not move here: this stage adds no
    directory. `SPEC-MOK-003`'s part is the rule 11 authority row, the *Start-up inputs* amendment giving each new option
    its disposition in the observer, and the extension of the byte-identity obligation to the new shared option
    descriptions; its rules 6.1 and 6.2, its *Actors* section and its declared dependency set do not move, and
    `ADR-MOK-007` records each of those as a considered non-amendment rather than an omission.

## Out of scope

- **Any provider call, any credential read, any network access, any process spawn.** `WO-MOK-026`'s.
- **The live-mode flag and the spend-ceiling option as user-facing options.** The ceiling *arithmetic* is in scope; the
  option that declares one is not, because an option that selects a path that does not exist is a defect.
- **`REQ-MOK-069`, `REQ-MOK-070`, `REQ-MOK-071`, `REQ-MOK-072`.** All four are about a live run and are `WO-MOK-026`'s.
- **`REQ-MOK-075` and `REQ-MOK-076`.** The measurement and its authorization are `WO-MOK-027`'s.
- **Any crate added to either package.** `ADR-MOK-007` decision 3's whole value is that none is needed. The connector
  lives outside this workspace and is named by the operator, so there is no workspace member to add and no dependency to
  admit — not for the engine's library target, not for its binary target and not for the observer.
- **The connector, its protocol implementation and the canned connector.** `WO-MOK-026`'s. This stage's port is fed by an
  in-process scripted stub and by a transcript, so nothing in it spawns, writes to or reads from another process.
- **Any live path in the observer, ever.** Not deferred to a later stage; `REQ-MOK-077` prohibits it. What this stage
  builds in the observer is one file read and a refusal.
- **Any change to the four existing sources' behaviour**, their selection order, their entropy consumption, or the
  observation's core-proposal list.
- **The `schema` increment's dependence on `SPEC-MOK-006`'s outstanding 2026-08-21 amendment row.** This stage adds no
  value to any record-stream domain until that row's state is settled; see *Stop and escalate conditions*.
- **Any concurrency**, any per-Mokiterion memory, any second provider, any outcome assertion.

## Authorized decision envelope

The implementation agent may decide locally:

- The shared rules block's exact prose, subject to `SPEC-MOK-007` rules 4.1 through 4.6 and to `VER-MOK-018`'s
  assessment **M1**. Its token count is a cost input, so shorter is better where accuracy allows.
- The observation block's and the enumerated set's exact separators, punctuation and line breaks, subject to being
  identical across two runs of one configuration.
- Whether the enumerated set is rendered as a flat verb-target list or as a verb list with per-verb targets.
  `SPEC-MOK-007` leaves this to measurement and rule 7.1's completeness holds either way; record which was chosen and
  its token cost, because `WO-MOK-026` will measure the other.
- The transcript's exact serialisation, subject to rule 11.3's fields and rule 11.4's constraints.
- The scripted stub's design and where its script lives, including its ability to fail on command, which cases **L22**
  and **R3** need.
- Internal module placement, naming and test organisation within the tiers the specifications assign.

The agent may **not** decide: the request's block order; whether the shared block varies; what the enumeration contains;
whether the fallback is `wait`; whether a rejected proposal counts as a fallback; whether the four existing sources may
move; or the wording of any amendment, all of which are the owner's.

## Constraints

- **The trust boundary does not widen.** The request crosses as values. No public item yields a mutable borrow of, or a
  reference into, authoritative state — `SPEC-MOK-002` rule 6, unchanged and re-checked.
- **The library target performs no filesystem operation, opens no socket, spawns no process and reads no environment
  variable.** `ARCH-MOK-001`'s 2026-08-20 prohibition, extended to three new capability classes by `ADR-MOK-007`.
- **The engine consumes no entropy under this source.** Not "the same amount"; none.
- **The observation's core-proposal list does not change**, in members, order or length.
- **The public surface grows by exactly one interface and one request type**, and **two existing public signatures gain
  one optional parameter each — `execute` and `Simulation::advance_tick`**, per `SPEC-MOK-007` rule 20.5. Nothing else on
  the public surface moves; in particular `pub fn run` does not. Those two signature changes are the whole of this
  initiative's breakage of existing callers, and they happen here rather than in a later stage so that no caller is broken
  twice.
- **The library holds no resource this source needs.** Not the connector's process, not the transcript's file handle, not
  the reader's cursor. Each arrives per tick as a borrowed parameter from the host that owns it, which is why the defect
  case **L30** exists to catch — a port rebuilt each tick — is structurally unavailable rather than merely prohibited.
- **The base-commit captures are taken first.** A capture taken after the change is not a base-commit capture, and
  `REQ-MOK-068` becomes uncheckable without one. This is the one ordering constraint that cannot be repaired later.
- **The shared rules block exists in exactly one place**, per `VER-MOK-018` check **S7**.
- **No amendment to an approved artifact is written before the owner has approved that amendment's text.** They are
  approved in the same act as this work order, so in practice they are written after that act and not before.
- **Governance artifacts are written CRLF; retained evidence is written LF.** These are two different rules and
  `.gitattributes` is why. A governance artifact falls under `core.autocrlf = true`, which converts in both directions:
  the worktree holds CRLF, the blob holds LF, and either worktree form commits to the same blob — so CRLF is what keeps
  a new artifact consistent with every other file in the tree rather than what the commit depends on.
  `docs/engineering/simulation/evidence/**` carries `-text`, so nothing is converted either way and the bytes written
  are the bytes hashed. A transcript written CRLF commits as CRLF, and every digest recorded beside it is then a digest
  no reviewer can reproduce — the failure `.gitattributes` records from `WO-MOK-010`, naming the file and both digests.
  `VER-MOK-018` states the exemption as its third retention decision.
- **No figure in any artifact is inferred from an unchanged total.** Every count `SPEC-MOK-004`'s census gains is
  measured against the tree at the candidate commit.

## Expected change surface

Components rather than files, since the shape of some of them is what this stage decides.

- **The engine's library target**: the decision port interface and the request type on its public surface; the source
  selection's fifth arm; the request composition; the transcript writer and reader; the fallback accounting; the run
  record.
- **The engine's library target, at `execute` and at `Simulation::advance_tick`**: one optional port parameter each. This
  is the only part of the surface that existing callers see change. `pub fn run` keeps its signature;
  `pub(crate) fn run_recording` gains the parameter as the crate-private carrier, which is a change to the diff and not to
  the surface.
- **The engine's binary target**: the transcript path resolution, the stream opening and closing, the replay selection,
  the port construction and the per-tick lending, and the usage text. The shared parser validates the new path option and
  discards its value on the `--events-path` precedent; this target re-reads the raw argument, which is where a path is
  allowed to exist.
- **The observer's option parsing and start-up path**: the fifth value accepted, the transcript option accepted and the
  file opened, the reader lent to the single-tick entry point, and the refusals — including a diagnostic, never silence,
  for an option the shared parser accepts but this host cannot honour.
- **The observer's authority module**: the fifth mapping and the four-source description.
- **The engine's test tiers**: the cases `VER-MOK-018` names, in the tiers `SPEC-MOK-002` rule 7 and `SPEC-MOK-004` rule
  9 assign.
- **The repository's workflows**: the credential and live-mode static check, and the replay step.
- **Seven approved definition-layer artifacts**: the amendments named in scope.
- **The evidence path**: the base-commit captures, the committed transcript, and the stage's own capture set.

## Required verification

`VER-MOK-018`, restricted to the cases that need no provider — which is every case except **L15b**, **L24**, **L25**,
the live half of **L20**, the connector half of **L32**, and the two owner attestations **C6** and **L28**.

**Matrix cases**: **L1**, **L2**, **L3**, **L4**, **L5**, **L6**, **L7**, **L8**, **L9**, **L10**, **L11**, **L12**,
**L13**, **L14**, **L15a**, **L16**, **L17**, **L18**, **L19**, **L21a**, **L21b**, **L22**, **L23**, **L26**, **L27**,
**L29**, **L30**, **L31**, **L33**.

**L29** and **L30** are checked here with the scripted stub standing where the canned connector will later stand. The
stub is an in-process implementation of the port, so it exercises both entry points and the per-tick lending without a
process, and **L30**'s ceiling half runs against declared prices and synthetic usage. That the stub rather than a
connector supplies the proposals is stated in the completion report, because it is the difference between this stage's
green and `WO-MOK-026`'s.

**L20** is in scope only in the half that needs no live-mode flag: a run with a credential present in the environment and
no live-mode selection makes no provider call. The other half — a live-mode selection with no credential present — needs
the flag, and is `WO-MOK-026`'s.

**L32** is in scope only in the half whose options exist: this source selected in the observer with **no** transcript
exits `2` and names the missing transcript. The connector-path, live-mode and ceiling halves need options `WO-MOK-026`
adds, and they are that work order's — with the obligation carried forward there explicitly, because the failure mode is
an option silently accepted rather than an option missing, and a missing test looks the same as a passing one.

**Acceptance scenarios A1**, **A2**, **A3**, **A5**, **A6** and the replay half of **A7**. **A4** needs a declared
ceiling, which needs the option that declares one, so it is `WO-MOK-026`'s, as is **A7**'s refusal half.

**Properties P1** through **P7** — all seven, all checkable over a committed transcript and a stubbed port.

**Static checks S1**, **S2a**, **S3**, **S3a**, **S4**, **S4a**, **S5**, **S5a**, **S6**, **S6a**, **S6b** and **S7**.
**S4a** is this stage's check on itself: the port lands here, so `SPEC-MOK-002` rule 5's restated drift checks must land
here too, and a build that adds the fifth parameter without restating them fails the specification it implements. **S2**
does not apply: no connector exists yet, canned or otherwise. **S3a** applies in its negative half only — that no process spawn
appears anywhere in either package, which at this stage is the whole of it.

**Security checks C1**, **C2** and **C4** in full. **C3** and **C5** in their negative half only — that the library
target opens no file, keychain or configuration directory in search of a credential, and that nothing leaves the
repository — since neither a credential read nor an outbound request exists yet. Their positive halves and the
attestation **C6** are `WO-MOK-026`'s.

**Resilience checks R3**, **R4** and **R5**. **R1** and **R2** cover transport retry, which this stage does not build,
and are `WO-MOK-026`'s.

**Manual assessments M1** and **M2**, both of which the assurance owner can make from the source alone. **M3** belongs to
the measurement and is `WO-MOK-027`'s.

The completion report states, for each item above, that it passes and where its evidence is. A case that cannot be run is
escalated, not omitted.

## Evidence to record

Under the evidence path this work order names, whose name is fixed before the first capture because it becomes
provenance:

1. **Base-commit captures** of `baseline`, `reference`, `individual` and `social` at every declared seed and the default
   density: standard output, structured record stream, per-tick entropy draw counts, exit codes, and digests. Taken at
   the stated base commit, with that commit recorded.
2. **Candidate-commit captures** of the same twenty configurations, with digests, and the comparison result for each.
3. **The committed transcript** the workflow replays, with the run's configuration and its coverage stated — which
   Mokiterions acted, that targeted actions were enumerated, that a death occurred.
4. **The recorded-run and replay capture pair** for `REQ-MOK-067`, at every declared seed, with tracing on and off, and
   the `cmp` results.
5. **The transcript-reading check outputs** for **L4**, **L5**, **L12**, **L13** and **L14**, each naming the transcript
   it read and the number of requests it examined.
6. **The layout check output** for **L15a**: the shared block's byte length, the per-Mokiterion prefix lengths, and the
   confirmation of byte-identity across the run.
7. **The assessment records** for **M1** and **M2**, naming the assurance owner and the date, with the shared rules
   block quoted in full in the **M1** record.
8. **The public-surface diff** and the dependency-graph comparison for **S1** and **S4**, together with the output of each
   of `SPEC-MOK-002` rule 5's restated mechanical checks and the amended text of the rule itself, for **S4a**.
9. **The workflow check's output** for **L21a**, run against the repository's workflows at the candidate commit.
10. **The two-host capture set** for `REQ-MOK-077`: the same transcript replayed through the engine's binary and through
    the observer, with the observer's run reaching the transcript's horizon; and the observer's refusal outputs — this
    source with no transcript — captured as exit status and standard-error bytes. The observer's captures state which
    panes were exercised, since **L31**'s pass condition is about panes rather than about bytes.
11. **The per-tick lending evidence** for **L30**: a replay of at least three ticks showing successive transcript records
    consumed, and a stubbed run showing accumulated cost rising across ticks and tripping a ceiling set to two exchanges.
    Both halves of a port rebuilt each tick are recorded as the failure they would produce, so a later reader can tell the
    case was exercised rather than merely present.
12. **A statement of what was not verified and why**, naming **L15b**, **L24**, **L25**, **L28**, the attestation **C6**,
    the live half of **L20** and the connector, live-mode and ceiling halves of **L32**, and for each the reason — a
    provider call, an option this stage does not add, or an owner attestation.

## Stop and escalate conditions

Stop and escalate — do not decide locally — if any of these is reached.

1. **A base-commit capture cannot be reproduced**, or a comparison in **L9** fails. This is `REQ-MOK-068`'s obligation
   and `INT-MOK-010`'s promise; a byte difference in an existing source is not a thing to investigate and work around.
2. **The port cannot be built without the engine acquiring a prohibited capability** — a socket, a process, a path, an
   environment read. That would mean `ADR-MOK-007` decision 1 is wrong, which is the owner's to reconsider.
3. **The complete enumeration cannot be composed without changing the observation's core-proposal list.** Escalate;
   `SPEC-MOK-001` rule 3's paragraph states why extending it is not available.
4. **The shared rules block cannot be written without stating a goal or a preference**, or cannot be written accurately
   without exceeding a token count that puts `REQ-MOK-070`'s ratio out of reach. Either is a decision about what is
   being measured.
5. **`SPEC-MOK-006`'s 2026-08-21 amendment row is still OUTSTANDING** when a record-stream domain must gain a value.
   Escalate before writing any `schema` value. The increment is to one more than whatever the ratification leaves
   standing, measured against the tree, and guessing it would put a wrong version number in a stream that later becomes
   provenance.
6. **An amendment turns out to be needed that `ADR-MOK-007` does not name.** No approved artifact is amended on an
   implementation agent's judgement.
7. **A verification case in the required list cannot be written** as `VER-MOK-018` states it.
8. **`execute` or `Simulation::advance_tick` cannot take the port as a borrowed optional parameter**, or the observer cannot lend an
   already-open reader to the single-tick path without the library opening something. That would mean the design rests on
   the library holding a resource `SPEC-MOK-006` rule 1.2 forbids it, and the shape of the parameter is the owner's, not
   an implementation agent's, to change.
9. **The observer cannot diagnose an option the engine's shared parser accepts.** Silently ignoring it is not an
   available local decision: it is the defect GitHub issue 40 tracks, in the same file, and `SPEC-MOK-007` rule 18.4.2
   forbids repeating it.
10. **The transcript's committed size exceeds what the repository should carry**, or the evidence path must be renamed
   after a capture exists. A bound evidence path can never be corrected; a rename forces a whole fresh capture.
11. **Any temptation arises to add a survivor floor, an outcome comparison or any assertion about what the population
   does.** Case **L26** exists to fail on it, and `ADR-MOK-007` decision 7 is why.

## Completion report format

1. **What was built**, component by component, against the *In scope* list, with each item marked done or escalated.
2. **The public surface**, before and after, as a diff, with `execute`'s and `Simulation::advance_tick`'s changed
   signatures shown in full, `pub fn run`'s shown unchanged, `pub(crate) fn run_recording`'s disclosed as the
   crate-private carrier that also took the parameter, and the callers the two public changes broke named — the engine's
   binary target and the observer, and no others. `SPEC-MOK-002` rule 5's restated mechanical checks and their output go
   here too, because the diff and the check that guards it are one claim.
3. **The four sources' byte-identity**: the base commit, the candidate commit, the twenty configurations, and the
   comparison result for each, with the draw-count comparison stated separately from the output comparison.
4. **The replay identity**: seeds, tracing selections, and the `cmp` results.
5. **The two hosts**: that the same transcript replayed through the engine's binary and through the observer, which panes
   the observer's run exercised, and the observer's refusal output for this source with no transcript. State plainly which
   of `REQ-MOK-077`'s refusals could not be exercised because the option that triggers them does not exist yet.
6. **The request layout as built**: the shared block's token count, the actor block's, a representative observation
   block's and enumerated set's, the resulting cacheable share as an estimate, and which enumeration rendering was
   chosen with its measured cost. State plainly that no cache ratio was measured, because no provider was called.
7. **Each verification case** in the required list, with its result and the path to its evidence.
8. **The amendments made**, each with the artifact, the provision, and the approval act that authorised it.
9. **What was not verified and why**, naming each case left open and the reason for it.
10. **Every local decision taken** under the *Authorized decision envelope*, each with its rationale, so the owner can
   see what was decided on their behalf.
11. **Every escalation raised** and how it was resolved.
