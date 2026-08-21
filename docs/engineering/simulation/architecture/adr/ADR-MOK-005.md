+++
id = "ADR-MOK-005"
type = "adr"
title = "A host-supplied record sink, a closed value alphabet instead of an escaper, and one parameter of interface growth"
status = "approved"
owners = ["technical owner"]
created = "2026-08-20"
updated = "2026-08-20"

[relations]
decides = ["ARCH-MOK-001"]
+++

# ADR: A host-supplied record sink, a closed value alphabet instead of an escaper, and one parameter of interface growth

## Status

Accepted by the repository owner on 2026-08-20, acting as technical owner, together with the `ARCH-MOK-001`,
`SPEC-MOK-001` and `SPEC-MOK-002` amendments this ADR requires. The same act approved `INT-MOK-009`, `CAP-MOK-009`,
`REQ-MOK-042` through `REQ-MOK-046`, `SPEC-MOK-006` and `VER-MOK-012`, and authorized `WO-MOK-018` to begin.

This ADR refines `ARCH-MOK-001` on three points — where a filesystem destination is resolved, how a serialization
format is produced without a serialization library, and how much the enumerated public interface grows — and
supersedes nothing. `ADR-MOK-001`'s engine authority and trust boundary, `ADR-MOK-002`'s library-and-thin-binary
target shape and enumerated interface, `ADR-MOK-003`'s two-package split and `ADR-MOK-004`'s provenance-closed
observer interface are all unaffected in substance.

The amendments this ADR requires are listed under *Required amendments* and are the technical owner's act. This ADR
does not claim them and `WO-MOK-018` makes them approval preconditions, in the same way `WO-MOK-003`, `WO-MOK-005`
and `WO-MOK-006` did for their own chains. The owner's approval of this ADR on 2026-08-20 covered those amendments as
stated here; each is written into its own document with its own amendment-record row, and `WO-MOK-018`'s *Lifecycle*
states that reading so that it is disclosed rather than assumed.

One precondition predates this chain and is stated here because it bears on the same architecture document:
`ARCH-MOK-001`'s amendment record row of 2026-08-18 — narrowing the prohibition on public items from "mutable or
owned authoritative state" to a mutable borrow of, or a reference into, that state — is recorded **OUTSTANDING** and
requires the technical owner. This ADR does not resolve it, does not depend on it, and does not restate it. It is
noted so that an owner amending `ARCH-MOK-001` for this chain sees it rather than discovers it.

## Context

Phase 4a asks the engine to write a machine-readable record stream. Three constraints of the existing architecture
collide with the obvious way to do that, and each collision has a real answer rather than a workaround. They are
taken in turn because the third depends on the first.

### The engine has no filesystem, by construction

`SPEC-MOK-001` states that "invalid input is treated as data and never interpreted as code or a filesystem path" and
that "state is held in memory and no persistence is required". `ARCH-MOK-001`'s data-flow diagram ends at standard
output, and its dependency direction says "no component depends on network or persistence infrastructure". The engine
package's library target has never opened a file.

The observer package has. `SPEC-MOK-003` rules 9.4 to 9.6 already specify an export: `mokiterions-tui/src/export.rs`
resolves a default filename from the seed and tick, creates the file, writes, and — on failure — drops the writer and
removes the partial file. So file writing is not new to the *product*. It is new to the *engine*, and specifically new
to the engine's library target, which is the part of the repository whose surface is enumerated item by item because
each item is a potential path to authoritative state.

The interesting detail is that the observer's precedent already separates the two concerns: `write_records` takes a
`Write` and knows nothing about files; `write_file` resolves the path, creates, flushes and cleans up. The lower half
of that split is exactly what the engine's library target can host without acquiring a filesystem at all.

### The engine has no serialization library and cannot acquire one

`ARCH-MOK-001` prohibits, in the engine package, "network calls, API credentials, asynchronous runtimes, databases, UI
frameworks, plugin systems, or dependency injection containers", and states that "the engine package's external
dependency set is empty and admits no exception, including a dependency shared with another package in the same
workspace." `ADR-MOK-003` measured that property per package rather than per workspace precisely so it could not be
satisfied by proximity. There is no `serde`, there is no `serde_json`, and there will not be.

So JSON must be written by hand. Written naïvely, that is a correctness obligation with no compiler behind it: every
string value must be checked for a quotation mark, a reverse solidus and a control character, and a missed case
produces a stream that a consumer's parser rejects — or worse, accepts as something else. A hand-written escaper is
about twenty lines and about twenty cases, and nothing in the build would notice if one were wrong.

What makes this tractable is a fact about the engine rather than about JSON. Every string the engine can put in a
record is drawn from a closed set: `world`, `A`, `B`, `M[0-9]{2}`, `F[0-9]{4}`, twelve fixed Mokiterion names, twelve
event types, three resource classes, three policies, eight directions, four action words, two termination reasons, two
skip reasons, two trace statuses, the density's two-decimal rendering, the package version, and eight forms of the
action trace's `detail` field. Every one of those is `[A-Za-z0-9_.+:;>-]+`. None is operator-supplied. The union
contains no quotation mark, no reverse solidus and no code point below U+0020.

That is not a lucky accident to be relied on quietly. It is a property that can be enumerated, stated as a rule, and
tested exhaustively — and the enumeration is small enough that a reviewer can read it.

### The engine's public interface is enumerated, and `execute` is on it

`SPEC-MOK-002` rule 5 enumerates the library target's public items, with a growth clause: the interface grows only
when an approved requirement needs it to. `execute` is on that list, so its signature is governed. `REQ-MOK-042`
cannot be satisfied without the library learning where to put records, so something must change; the question is
what, and by how much.

`SPEC-MOK-002` rule 6 is the harder constraint and is not in question: no public item may yield a mutable borrow of,
or a reference into, the world grid, the agent collection, the resource collection, the tick counter, the entropy
state or the event log, in any build configuration including tests. Nothing this phase needs comes close to it — a
record is a value written out — so rule 6 is not relaxed and this ADR asks for no exception to it.

### What is not in question

Three things a reader might expect to be decided here are settled elsewhere and are not reopened. The record format is
line-oriented JSON with four record kinds, fixed by `SPEC-MOK-006`. The per-tick metrics record's redundancy with the
event stream is `REQ-MOK-043`'s decision and is deliberate. That no record carries an outcome classification is
`REQ-MOK-044`'s. This ADR decides only the three architectural collisions above.

## Decision drivers

- The engine's library target must not acquire a filesystem. It is the enumerated, authority-bearing surface, and
  "opens a file the operator named" is a capability that has to be argued for rather than added.
- The engine package's dependency table must stay empty, per package, and demonstrably so.
- Hand-written serialization must be *provably* total, not carefully total. There is no library and no compiler check,
  so the correctness argument has to rest on something a test can exhaust.
- `REQ-MOK-045` must hold to the byte and to the entropy draw. A design whose non-perturbation rests on care rather
  than on structure is the wrong design.
- Interface growth must be the minimum an approved requirement needs, and rule 6 must not be relaxed by any part of
  it.
- The existing file-writing precedent should be followed rather than contradicted. One repository with two rules for
  writing a file would need a reason.
- The one-to-one correspondence between the text stream and the record stream should be structural. A property
  maintained by discipline across many call sites rots; one maintained by there being a single call site does not.
- Reversal must stay cheap. Removing the option should remove the capability.

## Considered options

### Collision 1 — where the destination is resolved

#### Option 1.1: The library resolves the path and opens the file

`execute` takes an `Option<&str>` and the library creates the file. Cost: one parameter, no host change. Consequence:
the engine's library target acquires a filesystem, `SPEC-MOK-001`'s "never interpreted as a filesystem path" must be
narrowed for the library rather than for the binary, and the library gains a failure mode — path resolution — that has
nothing to do with simulation. It also makes every library test that exercises the sink a test that touches the
filesystem, which is slower, order-dependent, and a temporary-directory problem the repository does not currently
have. Rejected.

#### Option 1.2: The library takes a `Write`; the binary resolves the path, creates, flushes and cleans up

`execute` gains an optional sink parameter. The binary target — which already locks and buffers stdout and stderr, and
already maps a flush failure to exit code `1` — additionally resolves the path, creates the file, hands over a
buffered writer, flushes, closes, and removes the partial file on failure. Consequence: the library's contract is
"write bytes to what you were given", every sink test is an in-memory buffer, and the split matches
`mokiterions-tui/src/export.rs`'s existing `write_records`/`write_file` division exactly. **Selected.**

#### Option 1.3: The observer exports the structured stream instead

Leave the engine alone and add a structured export to the observer, which already writes files. Consequence: a
measurement of a run would require a terminal, or a headless mode for a terminal program; `SPEC-MOK-003` rule 9.4
deliberately made the export the text format; and the facts `REQ-MOK-043` and `REQ-MOK-044` need are engine facts that
the observer would have to be granted access to, growing the observation surface to serve a measurement need. It also
puts measurement behind the least automatable host in the repository. Rejected.

#### Option 1.4: A third package that converts a retained text stream into records

A separate tool parsing standard output. Consequence: `ARCH-MOK-001` prohibits a third package without an approved
requirement and `REQ-MOK-026` authorizes exactly one further package; and the design institutionalizes the very thing
`INT-MOK-009` exists to remove — a parser of a human-facing stream standing between the engine's facts and the
conclusion. It also cannot produce the three facts no text line states. Rejected.

### Collision 2 — how JSON is produced without a library

#### Option 2.1: A hand-written escaping function

Write an escaper over the general case. Consequence: about twenty cases, no compiler check, no library, and a silent
failure mode. The verification obligation is unbounded — "all strings" — so it can only be sampled, and a sampled
proof of totality is not one. Rejected as the largest correctness risk in the phase, adopted for no benefit the
engine's actual value domain needs.

#### Option 2.2: Vendor a minimal JSON writer into the engine package

Copy a small serializer in as first-party source. Consequence: it is not a dependency in Cargo's sense, so
`ARCH-MOK-001`'s letter is satisfied — and its spirit is not: the engine would carry code it does not own the design
of, unreviewed against a specification, to solve a problem its own value domain does not have. Rejected.

#### Option 2.3: A closed value alphabet, enumerated in the specification and verified exhaustively

Enumerate every string-valued field and its domain as a specification rule, observe that the union admits no
character JSON requires escaping, and verify that exhaustively over the enumeration. Any future field either joins
the enumeration or arrives with an escaper and its own verification. Consequence: no escaping code exists, the
correctness argument is finite and readable, and the constraint is stated where a later phase will encounter it
before adding a free-text field. **Selected.**

#### Option 2.4: A non-JSON format that needs no escaping — TSV, or a length-prefixed encoding

Consequence: TSV cannot express the nested shapes the metrics and run records need without inventing a nesting
convention, which is a format the repository would then own and no consumer would already read. A binary encoding
gives up the property that a retained capture is readable by a person reviewing evidence, which is much of why this
stream is useful to assurance. Rejected, but noted as the fallback if a future field defeats rule 3.3.

### Collision 3 — how much the interface grows

#### Option 3.1: `execute` gains one optional sink parameter

Consequence: one signature change on one already-public item, no new public item, no new type. Every existing caller
is a caller in this repository — one binary target and the test suites — so the change is mechanical and total.
**Selected.**

#### Option 3.2: A second public entry point, leaving `execute` untouched

Add `execute_with_records` alongside `execute`. Consequence: two entry points that must not diverge, and the
enumerated interface grows by one item rather than by one parameter. The duplication is the cost and there is no
compensating benefit; the repository has one internal caller. Rejected.

#### Option 3.3: A configuration or builder object passed to `execute`

Consequence: growth by a public struct with public accessors, and a shape that invites future options to arrive
without amending anything. The enumerated interface exists precisely so that growth is visible; a builder makes it
invisible. Rejected.

#### Option 3.4: A public trait for record sinks

Consequence: a new public trait, and an extension seam nobody has asked for. `Write` is in the standard library, is
already how the engine takes its output, and a plain `Write` is what the observer's precedent uses. Rejected as
speculative generality.

## Decision

**The library target takes a sink; the binary target owns the destination.** `execute` gains one optional parameter,
an implementation of `Write`. The library target writes records to it and performs no filesystem operation of any
kind: it resolves no path, creates no file and removes no file. The binary target parses `--events-path`, resolves the
path, creates the file — truncating an existing file at that destination, as `SPEC-MOK-003` rule 9.4 already does —
supplies a buffered writer, flushes and closes it, and on failure removes the file it created. `SPEC-MOK-001`'s
prohibition on interpreting input as a filesystem path is narrowed for the binary target and holds unchanged for the
library target, which is the stronger of the two statements and the one worth keeping absolute.

**JSON is written without an escaper, on a closed value alphabet.** `SPEC-MOK-006` rule 3.2 enumerates every
string-valued field in the stream and its domain; rule 3.3 states that the union admits no quotation mark, no reverse
solidus and no code point below U+0020, so a writer emitting these values between quotation marks unaltered produces
valid JSON for every value the engine can produce; rule 3.4 makes that a property of the code, verified exhaustively
over the enumeration, and obliges any future string field either to join the enumeration or to arrive with an
escaping function and its own verification. **No field whose value could be operator-supplied, environment-derived or
free text may be added under rule 3.3.** The sink's own path is the nearest such value and is therefore absent from
every record.

**The enumerated interface grows by exactly one parameter.** No new public item, no new public type, no trait, no
builder, no second entry point. Every type an event record needs — `Event`, `EventDetail`, `EventType`, `Coordinate`,
`Territory`, `FoodClass`, `Action`, `Direction`, `TerminationReason`, `RegenerationSkipReason` — is already public and
already carries the values the record states. **`SPEC-MOK-002` rule 6 is not relaxed**, and nothing added here yields
a mutable borrow of, or a reference into, authoritative state in any build configuration including test builds.

**Records originate at one point.** Every authoritative event already passes through a single emission function, which
is why the text stream and a collecting host cannot disagree about order or content today. The record projection is
placed there, so `REQ-MOK-042`'s one-to-one correspondence is structural rather than maintained across call sites. A
second emission point would turn that correspondence into something to keep true; there is one, and adding one is a
defect rather than a refactor.

**Cumulative counters are simulation state, not sink state.** The counters `REQ-MOK-044` needs exist whether or not a
sink is configured, because a counter that existed only under an option would make the option a behavior change. They
draw no entropy, participate in no rule, decision, validation or applied action, and reach no text line.

## Required amendments

Each is the technical owner's act. `WO-MOK-018` makes all of them approval preconditions. None is claimed by this
ADR.

The `SPEC-MOK-001` list below has grown from the five seams named in `docs/PHASE_4_PROPOSAL.md` to nine, on a closer
reading of that specification's own text. The additional four are `Actors and external systems`, `Observability`,
`Compatibility and migration` and `Explicitly unspecified decisions`; none changes a behavior, and all four would
otherwise leave the specification stating something no longer true.

### `ARCH-MOK-001`

- *Components and responsibilities* item 1 lists the application entry point's duties as parsing arguments,
  constructing the run, applying the action-trace policy, streaming formatted events, mapping errors to exit codes and
  owning process termination. Amend to add: resolving the optional record sink's path, creating and truncating the
  destination, supplying the buffered writer, flushing and closing it, and removing a file it created on failure.
- *Components and responsibilities* item 2 lists the engine's ownership as the world, agents, food, tick, entropy
  state, validation, rule application, event creation, termination and summary. Amend to add: the run's cumulative
  measurement counters, and the production of structured records from authoritative events and state.
- *Components and responsibilities*, the paragraph on the read-only observation surface: amend to state that the
  library target additionally accepts a host-supplied record sink, that writing to it is a fifth responsibility of the
  engine package and not a fourth component, and that the library target performs no filesystem operation.
- *Dependency direction*: "No component depends on network or persistence infrastructure." Amend to state that no
  component depends on network infrastructure or on a persistence layer, database or index, and that the binary
  target's creation of one operator-named output file at the operator's instruction is an output destination and not
  persistence of state — nothing is read back, and no state survives the process in a form the engine consumes.
- *Data and control flow*: the diagram ends `ordered event -> text formatter -> standard output`. Amend to show the
  second, optional branch: `ordered event -> record projection -> host-supplied sink`, with the text branch unchanged
  and unconditional, and add that the projection mutates no simulation state and draws no entropy.
- *Prohibited patterns*: add that the engine package's library target performs no filesystem operation; that no
  record-writing path draws against the entropy stream; and that no field whose value is operator-supplied,
  environment-derived or free text is written to the record stream while `SPEC-MOK-006` rule 3.3 is the totality
  argument for its escaping.
- *Quality attributes*, **Determinism**: "the same inputs produce byte-identical events and final state." Amend to
  add the record stream, and to state that configuring a sink changes neither the text stream's bytes nor the entropy
  draw sequence.
- *Quality attributes*, **Debuggability**: extend from optional action tracing to state that optional structured
  recording likewise exposes a run's facts without altering engine behavior.
- *Conformance checks*: add four. That the engine package's library target performs no filesystem operation, checked
  against its source rather than asserted. That the text stream's bytes are identical with and without a sink, at
  every declared seed, each policy, and tracing off and on. That the per-tick entropy draw sequence is identical with
  and without a sink. That every string-valued field in the record stream is a member of `SPEC-MOK-006` rule 3.2's
  enumeration, checked exhaustively.
- *Related architecture and ADRs*: add this ADR, stating that it decides the record sink's location, the closed value
  alphabet and the interface growth, and supersedes nothing.
- `addresses`: add `REQ-MOK-042` and `REQ-MOK-045`. `conforms_to`: add `SPEC-MOK-006`.
- `decision_assessment.rationale`: append an amendment paragraph recording that the location of the record sink, the
  hand-written serialization strategy and the extent of interface growth are decided by `ADR-MOK-005`; that this fires
  the already-declared public-interface-or-protocol, technology-framework-vendor-or-external-service and
  material-alternatives triggers; that engine authority, the trust boundary, the dependency direction and the
  determinism properties are untouched; and that the assessment therefore stays `adr_required` and is covered by
  `ADR-MOK-001` through `ADR-MOK-005` together.
- *Amendment record*: one new row, dated on approval, describing the above and naming this ADR as the deciding one.

### `SPEC-MOK-001`

- *Scope*, second paragraph: "It does not define OpenAI integration, combat, social behavior, persistence, structured
  output, or a user interface." Amend to remove structured output from that list and to state that the structured
  record stream is defined by `SPEC-MOK-006`, which is a projection of the output this specification fixes. Persistence
  stays on the list.
- *Actors and external systems*: "There are no external systems or network calls." Amend to state that there are no
  external systems and no network calls, and that the filesystem is a destination for the optional record stream —
  written by the binary target at the operator's instruction, never read by the engine, and never a source of engine
  input.
- *Inputs*: the synopsis block and the option list gain `--events-path <path>`, absent by default, at most once, its
  value rejected as invalid configuration when empty or the single character `-`. State that a well-formed path the
  platform refuses is a runtime failure and not an invalid configuration.
- *Help output*: the content property gains the new option's entry, so that the usage text and the parser stay held
  equal by the existing test.
- *Outputs*: add that structured records are additionally written to the operator-named sink only when
  `--events-path` is given, that their content and framing are `SPEC-MOK-006`'s, and that the text stream is
  unaffected by the option's presence. The exit-code list is unchanged.
- *Error and recovery behavior*: "A standard-output write failure terminates the process with runtime exit code `1`;
  no successful summary is claimed." Amend to add that a failure to create the record sink, and a failure to write,
  flush or close it, likewise terminate the process with runtime exit code `1` with no successful summary claimed;
  that a sink that cannot be created stops the run before any tick and any text observation record; and that a file
  the process created is removed on failure so that no partial stream reads as a complete run.
- *Security and privacy properties*: "Invalid input is treated as data and never interpreted as code or a filesystem
  path." Amend to state that the sink path is the one operator-supplied value interpreted as a filesystem path, that
  it is interpreted only by the binary target and only as a path — never as code, a format string, an option or engine
  input — that the library target interprets no path at all, and that every other input remains data. Add that no
  record carries a path, a wall-clock time, a hostname, a user, an environment value or a credential.
- *Performance and capacity*: "State is held in memory and no persistence is required." Amend to state that state is
  held in memory, that no persistence of state is required or performed, and that the record stream is a write-only
  output the engine never reads back — its size grows linearly with the run and memory use does not.
- *Observability*: "Identical runs with identical trace configuration produce byte-identical standard output." Amend
  to add that identical runs with identical trace and sink configuration additionally produce a byte-identical record
  stream, and that configuring a sink leaves standard output byte-identical.
- *Compatibility and migration*: add that the record stream carries its own schema version, governed by
  `SPEC-MOK-006` rule 10, and that no existing behavior, default or exit code changes.
- *Explicitly unspecified decisions*: add an entry stating that the record stream's framing, field names, value
  alphabet, schema version and failure behavior are not unspecified — they are governed by `SPEC-MOK-006` — mirroring
  the existing entry that defers crate layout and the public interface to `SPEC-MOK-002`.
- *Amendment record*: one new row, dated on approval.

### `SPEC-MOK-002`

- Rule 5's enumeration: `execute`'s signature gains one optional sink parameter, under rule 5's growth clause, because
  `REQ-MOK-042` needs it. No item is added and no item's visibility changes.
- Rule 5's mechanical check: whatever form it takes for `execute`'s signature is updated to the new signature, so that
  the check continues to fail when the interface drifts.
- Rule 6: **no amendment.** Recorded here explicitly so that the absence is a decision rather than an omission.
  Nothing this chain adds yields a mutable borrow of, or a reference into, authoritative state in any build
  configuration including tests.
- *Scope* and *Compatibility and migration*: add that `SPEC-MOK-006` governs the record stream and that this
  specification's authority over the enumerated interface is unchanged by it.
- *Amendment record*: one new row, dated on approval.

## Consequences

### Positive

- The engine's library target keeps its strongest property — it touches nothing outside its arguments — while the
  product gains a file. The prohibition that mattered is preserved absolutely rather than narrowed a little.
- Every sink test is an in-memory buffer. No temporary directory, no filesystem ordering, no cleanup, and a failing
  writer is three lines rather than a permissions fixture.
- The escaping obligation is discharged rather than implemented. There is no escaper to get wrong, and the argument
  that replaces it is finite, enumerated in the specification, and exhaustively tested.
- The repository ends with one rule for writing a file, followed by both packages, with the path-resolving half in the
  host and the byte-writing half in a library — the split the observer already had.
- The one-to-one correspondence between streams is a consequence of there being one emission point, not of care at
  many.
- Interface growth is one parameter, so the diff to `SPEC-MOK-002` rule 5 is one line and a reviewer can see the whole
  of what was added.
- Removing the capability is removing an option and a parameter. Nothing else depends on it.

### Negative

- `execute`'s signature changes, so every call site changes. There are few and they are all in this repository, but a
  signature change to the enumerated interface is not a small documentary act.
- The closed alphabet is a constraint future work will meet. The first phase that wants a free-text field — an error
  message, a rejection ground stated in prose, an operator note — pays for an escaper and its verification then. That
  cost is deferred, not avoided, and rule 3.4 is what makes the bill arrive at the right time rather than silently.
- The action trace's `detail` field stays rendered text inside the record. It is within the alphabet and it is
  reconstructible, but a consumer wanting its parts parses a string. Typing it would change the engine's action result,
  which no approved requirement needs.
- The engine's `Cargo.toml` stays dependency-free at the cost of the repository owning a small amount of formatting
  code that a library would otherwise own. That code is trivial and total; it is still code.
- The binary target grows. It was a nineteen-line shim; it acquires path resolution, file creation, cleanup and one
  more failure path, so `ARCH-MOK-001`'s "stays thin" is under mild tension. The alternative puts a filesystem in the
  library, which is worse.
- Stream size is unbounded in run length, and a long traced run produces a large file. No rotation or sampling is
  imposed, because a sampled stream would silently misreport.

### Operational and security

- No network access, no credential read, no environment read and no wall-clock read is introduced. Neither package
  gains a dependency.
- One new filesystem effect exists: the binary target creates, truncates and may remove a file at a destination the
  operator named. That is the operator's instruction, it matches the observer's existing export, and it is stated in
  the usage text.
- The sink path never appears in any record, so a retained capture is safe to attach to a work order as evidence
  without redaction. Rule 3.4 makes that a rule rather than a habit.
- The trust boundary is unmoved. A decision source still receives immutable observations and returns typed proposals;
  the record stream reaches neither `Observation` nor `DecisionSource`, and both stay private.
- World authority is unchanged. A record is a value written out. Nothing in this chain reads a proposal, validates an
  action, or reaches authoritative state mutably.
- The cumulative counters cannot affect a run. They are written where their events are emitted, read by no rule, and
  saturating rather than wrapping.

### Migration

- One work order, `WO-MOK-018`, performs the counters, the projection, the three new record kinds, the option and the
  host-side file handling together, because the counters without the run record are unobservable and the run record
  without the counters is unimplementable.
- Non-perturbation is demonstrated by comparison, not review: text bytes and per-tick entropy draw counts compared
  with and without a sink, and every capture retained under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011` reproduced
  byte for byte.
- Records bound to commits are not edited. `VREC-MOK-001` through `VREC-MOK-011` and the evidence retained under
  earlier work orders stand as they are.
- Reversal is the removal of one option, one parameter, one projection and the counters. No simulation rule, default
  or exit code is touched, so nothing downstream depends on the outcome.

## Validation

- The engine package's library target contains no filesystem operation: no file creation, no path resolution, no
  removal, no directory access. Checked against its source, not asserted.
- The engine package's dependency table is empty, resolved per package rather than for the workspace.
- `execute`'s public signature matches `SPEC-MOK-002` rule 5 as amended exactly, and the enumerated interface gains no
  item.
- No public item yields a mutable borrow of, or a reference into, authoritative state or any cumulative counter, in
  any build configuration including test builds.
- Every string value the engine can write to the stream is a member of `SPEC-MOK-006` rule 3.2's enumeration, checked
  exhaustively over that enumeration; and every retained capture parses under a JSON parser this repository does not
  own.
- The text stream's bytes are identical with and without a sink, at every declared seed, under each of the three
  policies, with tracing off and on.
- The per-tick entropy draw sequence is identical with and without a sink, at every declared seed.
- Every run retained under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011` reproduces byte for byte.
- Two processes at one seed with identical options and sink configuration produce byte-identical sink streams.
- Every text line of a run reconstructs from its structured record, byte for byte, over the whole stream.
- A sink that cannot be created stops the run before any tick, reports on standard error, and exits `1`. A sink that
  fails mid-run reports, exits `1`, and leaves no file the process created.
- With no sink configured, the run's standard output, standard error and exit code are identical to the predecessor
  commit's at every declared seed and policy.
