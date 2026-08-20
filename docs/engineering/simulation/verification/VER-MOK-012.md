+++
id = "VER-MOK-012"
type = "verification"
title = "Structured record stream verification: projection, non-perturbation, additivity, and escaping totality"
status = "draft"
owners = ["assurance owner"]
created = "2026-08-20"
updated = "2026-08-20"

[relations]
verifies = [
  "REQ-MOK-042",
  "REQ-MOK-043",
  "REQ-MOK-044",
  "REQ-MOK-045",
  "REQ-MOK-046",
]
+++

# Verification Contract: Structured record stream verification

## Independence

Four of these five requirements are unusually easy to verify well, and the reason is worth stating because it changes
what this contract has to do.

`REQ-MOK-042` says the structured stream carries the same facts as the text stream. That is not a claim about output
looking right — it is an **invertibility** claim, and an invertibility claim is checkable without any oracle at all:
reconstruct the text from the records and compare bytes with the text the engine actually wrote. A suite written by the
same effort that made the change cannot fool that check by asserting what the new code does, because the check does not
read the writer's opinion of anything. It inverts the writer and compares against a stream the writer did not produce.

`REQ-MOK-045` is stronger still. `VER-MOK-010` had to compare pre-change and post-change streams *under a stated
projection*, because `WO-MOK-010` added fields to the text record, and that projection was the one place its strongest
oracle could be subverted — which is why it needed its own review, its own no-op check and its own retained text.
**This change adds nothing to the text stream, so the comparison is byte for byte with no projection.** There is
nothing to review, nothing to subvert, and any differing byte anywhere is a defect. That is a materially better
position than the last two chains had, and it is the single most important property of this contract.

What is left genuinely needs oracles: that no entropy draw moved, that the JSON is valid rather than plausible, and
that the metrics record agrees with the world it describes.

Seven independent oracles are used.

1. **A recorded pre-change baseline, compared byte for byte with no projection.** The engine's complete standard
   output, standard error and exit code are captured *before* any code changes, at the commit the work begins from,
   across the declared matrix: the seeds `0`, `1`, `42`, `123` and `777`; all three policies; the default density
   `0.75%` and the swept densities `VER-MOK-002` declares; `--ticks 1000`; with and without `--trace-actions`.
   Afterwards the same matrix is captured again, both with no sink configured and with a sink configured, and all
   three streams are compared byte for byte. Zero differing bytes is the pass condition, in both comparisons. There is
   no projection and none is permitted: a contract that needed one here would be verifying a different requirement.

   The baseline is captured once. A discrepancy is never resolved by recapturing it.

2. **Inversion of the record stream.** Every text line of a run is reconstructed from its structured record by the
   generic walk `SPEC-MOK-006` rule 6.6 fixes, and the reconstructed stream is compared byte for byte against the
   standard output the same process wrote. This is `REQ-MOK-042`'s correspondence in its checkable form, and it is
   independent of the writer because it consumes the writer's output rather than its intentions. It is total over the
   stream rather than sampled: every line, every seed, every policy, trace off and on.

   The reconstructor is written from `SPEC-MOK-006` rule 6.6 and from no other source. It is retained as evidence, and
   it must contain no event-type-specific branch — rule 6.6 exists so that three value shapes suffice, and a
   reconstructor that needed a twelfth case would be evidence that the record shapes drifted from the rule.

3. **A JSON parser this repository does not own.** Every retained capture is parsed by a general JSON parser outside
   the repository — Python's standard-library `json` module, invoked in the evidence capture — and every line must
   parse as an object. This oracle exists because the engine cannot link a parser under `ARCH-MOK-001` and therefore
   cannot check its own output's validity with anything but code from the same author as the writer. A hand-written
   emitter checked only by a hand-written reader is one mistake, made twice, agreeing with itself.

   The parser additionally asserts that every numeric value is an integer — that the parsed value's type is integral
   and not floating point — which is `REQ-MOK-043`'s and `SPEC-MOK-006` rule 4.1's obligation checked by a tool with
   no knowledge of the engine's intent.

4. **The entropy stream's own position, at every tick boundary.** The engine's generator advances its state by one
   fixed odd increment per draw, so its state *is* a draw counter: the number of draws between two states is exactly
   the state difference divided by that increment in modular arithmetic, and equal states imply equal draw counts. An
   internal-tier test drives two runs at one seed — one with a sink, one without — and asserts the generator's state
   equal at every tick boundary, not only at the end.

   This is the oracle that catches the defect `REQ-MOK-045` was written for, and it catches it where output cannot.
   A draw added and then compensated, or added at a density outside the swept set, or added past tick 1000, would pass
   oracle 1 on every declared seed and fail here. The precedent is the observer's own non-perturbation suite, which
   "drives two runs and diffs them rather than asserting something about the observer's structure"; this contract
   applies the same discipline to the entropy state rather than to the output.

   The state is read through a `#[cfg(test)]` accessor returning an owned `u64`. It is not a borrow of, and not a
   reference into, the entropy state, so `SPEC-MOK-002` rule 6 is satisfied; and it is compiled out of every shipped
   artifact and out of the build a public-tier test links, so it is internal-tier by definition.

5. **Exhaustive enumeration of the value alphabet.** `SPEC-MOK-006` rule 3.2 enumerates every string-valued field in
   the stream and its domain. A test iterates every member of every domain — every event type, every name, every
   direction, every class, every policy, every termination reason, every skip reason, every trace status, every trace
   detail form, every subject shape, the density rendering at its boundary values — emits it, and asserts the emitted
   bytes contain no quotation mark, no reverse solidus and no code point below U+0020.

   This is what replaces an escaping function, so it must be exhaustive rather than representative.
   `SPEC-MOK-006` rule 3.3's totality argument is only as good as this enumeration's completeness, so the test also
   asserts the enumeration's *size* per domain against the specification's, so that a domain gaining a member without
   the specification gaining a row fails here.

6. **Metrics reconciled against a replay of the events.** A consumer written for verification replays the event
   records — initializations, consumptions, regenerations, deaths, crossings — to reconstruct the standing resource
   count per class per territory and the living and death counts at each tick, and compares them against the metrics
   records. This is a genuine oracle rather than a restatement, because the two figures reach the record by different
   code paths: the events come from transitions as they are applied, the metrics from authoritative state read at the
   tick boundary. A disagreement means one of them is wrong, and the contract does not presume which.

   The same replay reconciles the run record's cumulative counters against the event counts, which is
   `SPEC-MOK-006` rule 8.6 checked rather than assumed. This is the check that would catch a counter incremented in
   the wrong branch — the most likely defect in the whole change, because a counter has no other witness.

7. **The governance state of the artifacts this change amends.** The `ARCH-MOK-001`, `SPEC-MOK-001` and
   `SPEC-MOK-002` amendments `ADR-MOK-005` requires must be present and approved, and `ADR-MOK-005` itself must be
   accepted, before this contract can be satisfied. Their absence fails this contract regardless of the state of the
   code, for the reason `VER-MOK-006` and `VER-MOK-010` give in the same position: an amendment nobody approved is not
   a specification.

   `ARCH-MOK-001`'s amendment record additionally carries a row dated 2026-08-18 recorded **OUTSTANDING** and
   predating this chain. This contract records its state at verification rather than requiring its resolution: it is
   not this chain's amendment, and `WO-MOK-012` does not claim it.

The declared verification seed set is `0`, `1`, `42`, `123` and `777`, fixed by `VER-MOK-002` and reused unchanged so
that this change's measurements and the control's are taken on the same worlds.

## Requirement-to-evidence matrix

| Requirement | Method | Case/evidence | Pass condition |
|---|---|---|---|
| `REQ-MOK-042` | automated-test | Record count against text line count, whole declared matrix | Event records equal text event lines exactly; one run record; one header; one metrics record per completed tick |
| `REQ-MOK-042` | automated-test | Inversion of the whole stream (oracle 2) | Reconstructed text byte-identical to the process's standard output, at every declared seed, each policy, trace off and on |
| `REQ-MOK-042` | review | The reconstructor used by oracle 2 | Written from `SPEC-MOK-006` rule 6.6 alone; contains no event-type-specific branch; full text retained |
| `REQ-MOK-042` | automated-test | Order preservation | The *n*-th event record corresponds to the *n*-th text event line, for all *n*, at every declared seed |
| `REQ-MOK-042` | automated-test | Orthogonality with tracing | The correspondence holds one-to-one with `--trace-actions` off and on; enabling it adds exactly one record per added text line |
| `REQ-MOK-042` | automated-test | Reproducibility of the sink | Two processes at one seed with identical options produce byte-identical sink streams, at every declared seed |
| `REQ-MOK-042` | automated-test | Every capture parses (oracle 3) | Every line of every retained capture parses as a JSON object under a parser outside this repository |
| `REQ-MOK-042` | static-analysis | Single emission point | The record projection is reachable from exactly one place, the function every authoritative event already passes through; no second emission site exists |
| `REQ-MOK-043` | automated-test | One metrics record per completed tick | Exactly `min(ticks, termination tick)` records, ascending, none skipped, none repeated, none for tick `0` |
| `REQ-MOK-043` | automated-test | Metrics reconciled against event replay (oracle 6) | Standing count per class per territory, living count and death count agree at every tick of every declared seed |
| `REQ-MOK-043` | automated-test | Internal consistency of each record | Per-territory populations sum to `living`; `living` plus `deaths` equals twelve; per-class counts sum to `standing` |
| `REQ-MOK-043` | automated-test | Attribute sums and extrema against a snapshot | Each sum equals the sum over the tick's living population; each extremum is the correct minimum or maximum per `SPEC-MOK-006` rule 7.5 |
| `REQ-MOK-043` | automated-test | Capacity and permanent depletion | `capacity` equals the count the density resolves to and is constant within a run; `depleted` is reported on every record and matches the engine's state |
| `REQ-MOK-043` | automated-test | Empty living population | At a seed and density producing extinction, the tick after the last death reports every sum `0` and every extremum `null`, never `0` |
| `REQ-MOK-043` | automated-test | Integers only (oracle 3) | No numeric value in any capture parses as a floating-point value; no field named as a mean, average, ratio or rate exists |
| `REQ-MOK-043` | static-analysis | No field for an uncomputed phenomenon | No conflict, combat, threat, retreat or surrender field, at zero or otherwise, in any record kind |
| `REQ-MOK-044` | automated-test | Exactly one run record, last | One per run, after the final metrics record, at every declared seed and policy |
| `REQ-MOK-044` | automated-test | Summary line reconstructible from the run record | The text summary line reconstructs byte-identically from `reason`, `ticks`, `survivors`, `deaths` and the `final` object |
| `REQ-MOK-044` | automated-test | Cumulative counters against event counts (oracle 6) | `crossings`, each `consumed` class, `regenerated` and each `regeneration_skipped` reason equal the corresponding event-record counts, at every declared seed and policy |
| `REQ-MOK-044` | automated-test | Per-Mokiterion outcomes | Twelve entries, ascending identifier order; each `died_at` equals the tick of that Mokiterion's `agent_died` event; a survivor's is `null`, never `0` |
| `REQ-MOK-044` | automated-test | Final territory of a dead Mokiterion | Equals the territory it stood in at its death tick, and does not change thereafter |
| `REQ-MOK-044` | automated-test | Total population loss | At a seed and density producing extinction, `survivors` is `0`, `deaths` is `12`, every `died_at` is an integer, and no field states an outcome label |
| `REQ-MOK-044` | static-analysis | **No classification anywhere** | No record kind carries an outcome, label, category, verdict, severity or interpretation field; the whole field set is compared against `SPEC-MOK-006` |
| `REQ-MOK-044` | static-analysis | Counters draw no entropy | No counter increment is on a path that draws; no counter is derived from a draw; every counter is saturating |
| `REQ-MOK-044` | static-analysis | Counters reach no rule | No rule, decision source, validation path, applied action, termination check or text record reads a counter; each has exactly one writer per event and no reader inside the engine |
| `REQ-MOK-045` | automated-test | **Text stream unchanged**, byte comparison against the recorded baseline (oracle 1) | Zero differing bytes across the whole declared matrix, with no sink configured; exit codes and standard error identical |
| `REQ-MOK-045` | automated-test | **Text stream unchanged with a sink**, byte comparison (oracle 1) | Zero differing bytes across the whole declared matrix between a sinkless run and a sink-configured run; exit codes and standard error identical |
| `REQ-MOK-045` | automated-test | Entropy state at every tick boundary (oracle 4) | Equal with and without a sink, at every tick of every declared seed, under each policy, trace off and on |
| `REQ-MOK-045` | automated-test | Entropy state against the pre-change build | The state after initialization and at tick `1000` matches the pre-change build's, at every declared seed and swept density |
| `REQ-MOK-045` | automated-test | **Prior captures reproduce** | Every run retained under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011` reproduces byte for byte at the configuration recorded with it |
| `REQ-MOK-045` | automated-test | Simulation outcomes identical | Survivor count, death count, every final position and attribute, and each territory's standing count and class distribution are equal with and without a sink |
| `REQ-MOK-045` | automated-test | No default changed | With no sink option, every default and every observable behavior is identical to the predecessor commit's |
| `REQ-MOK-045` | automated-test | A sink that would interleave | `--events-path` with the empty string, and with `-`, each exits `2` with the usage text on standard error and runs no tick |
| `REQ-MOK-045` | static-analysis | No unordered iteration reaches a record | Every collection whose traversal reaches a record field is explicitly ordered; rule 8.4's roster order is stated, not inherited |
| `REQ-MOK-046` | automated-test | Sink cannot be created | A destination in a non-existent directory exits `1`, reports on standard error, runs no tick, and writes no observation record to standard output |
| `REQ-MOK-046` | automated-test | Write fails mid-run | A failing writer at a record boundary exits `1`, reports on standard error, claims no successful summary, and writes no further record |
| `REQ-MOK-046` | automated-test | Flush or close fails | A writer that accepts every write and fails on flush exits `1` and claims no successful completion |
| `REQ-MOK-046` | automated-test | Partial file removed | After a mid-run failure with a real file destination, the file the process created does not exist |
| `REQ-MOK-046` | automated-test | A destination the process did not create is not removed | Where creation cannot be established the destination survives and the diagnostic says so |
| `REQ-MOK-046` | automated-test | Existing destination overwritten | A prior run's file at the same destination is replaced, holds only the new run's records, and no failure is reported |
| `REQ-MOK-046` | automated-test | Failure while writing the run record | Treated as any other write failure: exit `1`, reported, partial file removed |
| `REQ-MOK-046` | automated-test | Malformed argument | Missing value, a value beginning with `--`, and a duplicate `--events-path` each exit `2` with the usage text |
| `REQ-MOK-046` | automated-test | Exit-code contract unchanged | No new exit code; every existing case still maps as `SPEC-MOK-001` states; a text-stream failure still exits `1` |
| `REQ-MOK-046` | automated-test | Diagnostics distinguishable | A sink failure's message is distinguishable from a text-stream failure's, is deterministic in form, and carries no value beyond the sink's identity and the platform's reason |
| all five | automated-test | Value alphabet, exhaustive (oracle 5) | Every member of every domain in `SPEC-MOK-006` rule 3.2 emits bytes free of `"`, `\` and code points below U+0020; each domain's size matches the specification's |
| all five | static-analysis | Library performs no filesystem operation | No file creation, path resolution, removal or directory access anywhere in the engine package's library target |
| all five | static-analysis | Interface growth | The engine's public interface grows by exactly one parameter on `execute` and by no item; `SPEC-MOK-002` rule 5's enumeration is compared item for item |
| all five | static-analysis | Rule 6 re-checked | No public item yields a mutable borrow of, or a reference into, authoritative state or any counter, in any build configuration including test builds |
| all five | static-analysis | Nothing in the stream is sensitive | No record carries a path, wall-clock time, duration, hostname, user, environment value, process identifier or credential |
| all five | automated-test | Prior coverage preserved | Every case, invariant and check in `VER-MOK-001` through `VER-MOK-011` still maps to a passing test; the workspace census reconciles name by name against the predecessor commit, with additions accounted for and no removal |
| all five | static-analysis | **Required amendments present and approved** (oracle 7) | `ADR-MOK-005` accepted, and the `ARCH-MOK-001`, `SPEC-MOK-001` and `SPEC-MOK-002` amendments it requires approved, before this change is verified. Absence fails this contract regardless of code state |

## Acceptance scenarios

1. A reviewer captures the declared matrix at the commit the work begins from, captures it again afterwards with no
   sink and with a sink, and finds all three byte-identical on standard output, standard error and exit code. No
   projection is applied, and none is available to apply.
2. A reviewer reconstructs the text stream from a run's records using only `SPEC-MOK-006` rule 6.6's walk, and finds
   it byte-identical to the standard output the same process wrote — at every declared seed, under each policy, with
   tracing off and on.
3. A reviewer parses every retained capture with a JSON parser outside this repository and finds every line a valid
   object and every numeric value an integer.
4. A reviewer records the generator's state at every tick boundary of two runs at one seed, one with a sink and one
   without, and finds them equal throughout — then deliberately perturbs the record path to draw one value, confirms
   oracle 4 fails at the first tick, and reverts. A check that cannot be made to fail has not been demonstrated to
   work.
5. A reviewer replays a run's event records to reconstruct the standing resources, living count and death count at
   every tick, and finds the reconstruction equal to the metrics records — then perturbs one counter's increment into
   the wrong branch, confirms oracle 6 fails, and reverts.
6. A reviewer enumerates every domain in `SPEC-MOK-006` rule 3.2, emits every member, and finds no character requiring
   escaping — then adds a hypothetical thirteenth event type to the engine without adding it to the specification, and
   confirms the size assertion fails.
7. A reviewer runs a seed and density that produce extinction and reads the metrics record for the tick after the last
   death: every sum `0`, every extremum `null`. The reviewer then confirms that no record anywhere in that run names
   the outcome.
8. A reviewer points `--events-path` at a directory that does not exist, and finds exit `1`, a diagnostic on standard
   error, and standard output empty of observation records.
9. A reviewer supplies a writer that fails after a hundred records, and finds exit `1`, a diagnostic, no successful
   summary, and — with a real file destination — no file left behind.
10. A reviewer supplies `--events-path -` and finds exit `2` with the usage text, before any tick.
11. A reviewer runs a configuration whose file already exists from an earlier run, and finds the file replaced and
    holding only the new run's records.
12. A reviewer re-runs every configuration retained as evidence under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011` and
    finds each output byte-identical to its retained capture.
13. A reviewer searches the engine package's library target for a filesystem call and finds none.
14. A reviewer confirms `ADR-MOK-005` is accepted and every amendment it requires is approved, and records the state of
    `ARCH-MOK-001`'s outstanding 2026-08-18 amendment row without resolving it.

## Property and invariant tests

- **Total correspondence.** For every run in the declared matrix, the number of event records equals the number of
  text event lines, and the *n*-th of each correspond. Asserted over whole streams, not sampled lines.
- **Invertibility.** For every text line of every run in the declared matrix, reconstruction from its record yields
  the line's exact bytes. This is the property, and it is asserted line by line rather than by a digest, so a failure
  names the line.
- **Sink determinism.** Two processes at one seed, density, policy, tracing and sink configuration produce
  byte-identical sink streams.
- **Text non-perturbation.** For every configuration in the declared matrix, the standard-output bytes are equal with
  and without a sink. Zero tolerance, no whitespace exemption.
- **Entropy invariance.** For every tick of every run at every declared seed, the generator's state is equal with and
  without a sink. Asserted per tick, not only at termination.
- **Entropy additivity.** The generator's state after initialization is a function of seed and density alone,
  independent of the sink configuration, the policy and the tracing setting, and equal to the pre-change build's.
- **Metrics–event agreement.** For every tick of every run at every declared seed, the standing resource count per
  class per territory, the living count and the death count derived from the event replay equal the metrics record's.
- **Counter–event agreement.** For every run, each cumulative counter equals the number of its corresponding event
  records, and the per-class consumption counts sum to the total consumption record count.
- **Record-count invariant.** Exactly one header, exactly one run record, exactly one metrics record per completed
  tick, and no record outside `SPEC-MOK-006` rule 9.1's order.
- **Roster completeness.** The run record names all twelve Mokiterions in every run, whatever the outcome, in
  ascending identifier order.
- **Integer closure.** No value in any record of any run is a non-integer JSON number, checked by a parser outside the
  repository rather than by the writer's own types.
- **Alphabet closure.** No byte written between quotation marks in any record of any run is a quotation mark, a
  reverse solidus, or below U+0020. Asserted over full runs as well as over the enumeration.
- **Absence encoding.** Every absent value in every run is `null`. No `0`, no `-1`, no omission, no empty string.
- **Death finality in the record.** A Mokiterion's `died_at` and final territory, once set, never change; and no
  event record for it follows its death tick.
- **No classification.** No record in any run at any seed, density, policy or tracing setting carries a field naming
  an outcome. Asserted against the whole field set, not against a list of forbidden names.
- **Failure atomicity.** After any sink failure, the process exits `1`, claims no completion, and leaves no file it
  created.
- **Determinism of the whole engine.** `REQ-MOK-009`'s byte-identical reproducibility holds with a sink configured
  exactly as it does without one, at every declared seed.

## Static and architecture checks

- `cargo fmt --all -- --check` clean.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` clean, with no `allow` attribute
  added and no lint suppressed to accommodate this change.
- `cargo test` at the workspace root runs every tier of both packages in one invocation, with no feature, environment
  variable, ignore attribute, extra command, terminal or working-directory dependence.
- The engine's dependency and dev-dependency tables are empty, with no exception, and `cargo tree -p Mokiterions`
  resolves to the engine package alone. `ARCH-MOK-001` admits no exception, including a dependency shared with the
  observer. **No JSON or serialization dependency is added to either package.**
- The observer's dependency set is one path dependency on `Mokiterions` and `ratatui` at the pinned version and
  feature set, with nothing added.
- No new package, no new target, no build script, and no change to any package, library or binary name.
- The engine package's library target contains no filesystem operation: no file creation or opening, no path
  resolution, no removal, no directory access, no temporary-file use. Checked against its source.
- The record projection is reachable from exactly one call site — the function every authoritative event already
  passes through — so `REQ-MOK-042`'s correspondence is structural. A second emission site is a finding.
- The engine's public interface grows by exactly one parameter on `execute` and by no item. `SPEC-MOK-002` rule 5's
  enumeration is compared item for item, and rule 5's mechanical signature check is updated and still fails on drift.
- No public item yields a mutable borrow of, or a reference into, the world grid, the agent collection, the resource
  collection, the tick counter, the entropy state, the event log or any cumulative counter, in any build configuration
  including test builds. `SPEC-MOK-002` rule 6 is re-checked because a public signature changed and new state was
  added.
- The `#[cfg(test)]` entropy-state accessor returns an owned `u64`, is `#[cfg(test)]` in the merged tree, and is named
  by internal-tier tests only. No public-tier test names it.
- No floating-point type or operation appears in any record-producing path, in any counter, or in any metric
  computation.
- No unordered collection is iterated where the traversal order reaches a record field, a text line, or a decision.
- Test placement follows `SPEC-MOK-002` rules 7 to 10: a new test lives in the public tier only if it is writable
  through the library target's public interface with its assertions unchanged; no item is widened to `pub` to relocate
  a test. Every internal-tier test added here names the private item or hook it requires.
- The counters are private, have exactly one writer per event, and no reader inside the engine other than the record
  producer.
- `ARCH-MOK-001` requires amendment and `ADR-MOK-005` is its deciding ADR. Oracle 7 checks that both are approved;
  the technical owner additionally confirms in review that the amended component boundaries, prohibited patterns,
  dependency prohibition and conformance checks are satisfied as amended.

## Security and privacy checks

- **One new filesystem effect, and it is the operator's instruction.** The binary target creates, truncates and may
  remove a file at a destination the operator named. It reads no file, opens no directory, follows no configuration
  file, and consults no environment variable to choose a destination.
- The sink path is interpreted only as a path, only by the binary target. It is never rendered into a record, never
  interpreted as a format string, never interpreted as an option, and never fed back as engine input. The library
  target interprets no path at all, checked statically.
- No record carries a credential, token, secret, hostname, user name, environment value, working directory, absolute
  or relative path, process identifier, wall-clock time or duration. Checked against the whole field set rather than by
  search for known-bad strings.
- No new network access, no credential path, and no new environment dependence. The simulation remains a closed local
  computation over command-line arguments.
- No model-provider credential or other secret enters the repository, in code, in a test fixture, or in retained
  evidence.
- Retained evidence contains simulation output only. It carries no personal data.
- A retained capture is safe to attach to a work order without redaction, which is a property of `SPEC-MOK-006` rule
  3.2's enumeration and is verified rather than assumed.
- **Removal is bounded.** The process removes only a destination it created. A test confirms that a destination whose
  creation cannot be established is not removed, because a program that deletes an operator's file on a write error is
  a worse outcome than a partial stream.
- `ADR-MOK-001`'s trust boundary is unchanged. The record stream reaches neither `Observation` nor `DecisionSource`,
  both of which stay private, and no record path reads a proposal or validates an action.

## Performance and resilience checks

- Per-tick work with no sink configured is unchanged, checked statically rather than by timing: no record path is
  reachable when no sink is configured.
- Memory use is independent of run length with a sink configured: no record is retained after it is written, and the
  counters are a fixed number of integers.
- A 1,000-tick run with a sink configured at the default density completes on each declared seed, under each policy,
  with tracing off and on, and the resulting stream's size is recorded.
- A 10,000-tick traced run with a sink configured completes without panic, without overflow in a debug build, and
  without unbounded growth in retained state. The stream size is recorded so that the cost of tracing plus recording
  is a stated figure rather than a surprise.
- Every counter is saturating and cannot wrap; asserted at the type level and over full runs rather than argued.
- No arithmetic in any metric computation can panic in a debug build: every sum over twelve `u8` attributes is
  performed in a width that cannot overflow, and every extremum over an empty set is handled as absence rather than as
  a panic or a sentinel.
- A sink whose writes are slow does not change the run's output. Time is not a property of this contract, and no
  timeout, retry or buffering policy is verified beyond the flush obligation.

## Manual assessments

Each of the following is an explicit judgement recorded by the accountable role. An unrecorded assessment is an
outstanding assessment, and this contract is not satisfied while any remains outstanding.

1. **The record schema is sufficient for Phase 4b, by the product owner.** Reading a retained capture, the owner
   records whether the facts present are the facts a distribution over runs and an outcome classification will need,
   and names any fact found missing. This is the assessment this whole phase exists to make possible, and finding a
   gap now is cheaper than finding it after Phase 4b consumes the schema.
2. **The refusal to classify is right, by the product owner.** The owner confirms that leaving the outcome label out
   is a decision they hold rather than a limitation they inherited, and that `SPEC-MOK-006` rule 8.7's reasoning —
   that a threshold must be revisable without invalidating a retained capture — still reflects their intent.
3. **The closed alphabet is an acceptable long-term constraint, by the technical owner.** The owner records that
   `SPEC-MOK-006` rules 3.3 and 3.4 are a constraint they intend to keep, and that the first future field needing
   escaping will pay for an escaper and its verification rather than be quietly admitted. A rule nobody intends to
   enforce is worse than no rule.
4. **The metrics record's redundancy is worth its cost, by the technical owner.** The owner records the measured
   stream size per 1,000-tick run and confirms that stating per-tick state alongside the event stream is worth it,
   against the alternative of events alone. Rule 7.8's three facts with no event counterpart are the strongest part
   of the case and should be named in the judgement.
5. **The library's freedom from the filesystem is worth the host's growth, by the technical owner.** The binary target
   was a nineteen-line shim and is no longer. The owner records that `ARCH-MOK-001`'s "stays thin" is still true
   enough, or requires the wording amended.
6. **Overwriting without prompting is right for the engine, by the technical owner.** It matches `SPEC-MOK-003` rule
   9.4. The owner records that the same choice is right for a program that may be run in a loop, where prompting would
   be worse and a suffix convention would be a persistence design nobody approved.
7. **The evidence is machine-checkable, by the assurance owner.** The owner confirms that every quantitative figure in
   this contract's evidence was produced by reading records rather than by parsing the human-facing text stream, and
   records any figure that was not. This is `INT-MOK-009`'s stated purpose applied to its own verification, and it is
   the first opportunity to fail it.
8. **The reconstructor and the replay consumer, by the assurance owner.** Oracles 2 and 6 depend on code written for
   verification. The owner confirms that the reconstructor is derived from `SPEC-MOK-006` rule 6.6 alone and carries no
   event-type-specific branch, and that the replay consumer implements the resource rules from `SPEC-MOK-001` rather
   than by reading the engine's implementation. A replay that copied the engine's code would agree with it for the
   wrong reason.

## Evidence retention

Retained under `docs/engineering/simulation/evidence/WO-MOK-012/`:

- the pre-change baseline capture of the declared matrix, captured before any code change, with the commit it was
  taken at recorded;
- the post-change capture of the same matrix with no sink configured, and the byte comparison result for every
  combination;
- the post-change capture of the same matrix with a sink configured, its standard output, and the byte comparison
  against the sinkless capture for every combination;
- one full sink stream per declared seed at the default density under each policy, with tracing off and on, retained
  as the primary artifact this phase produces;
- the full text of the reconstructor used by oracle 2, and the byte comparison result of reconstruction against
  standard output for every combination;
- the full text of the replay consumer used by oracle 6, and its reconciliation result per tick per seed;
- the JSON-parser check of oracle 3, including the exact command, and its result for every retained capture;
- the entropy-state comparison of oracle 4, per tick, per seed, per policy, with and without a sink; and the state
  after initialization and at tick 1,000 against the pre-change build;
- the value-alphabet enumeration of oracle 5, with each domain's members and size, and the emitted bytes;
- the deliberate-perturbation results of acceptance scenarios 4, 5 and 6, demonstrating that oracles 4, 5 and 6 can
  fail;
- the re-run of every configuration retained under `WO-MOK-002`, `WO-MOK-010` and `WO-MOK-011`, with the byte
  comparison against each retained capture;
- the five failure captures: sink not creatable, write failure mid-run, flush failure, run-record write failure, and
  the reserved-spelling rejection — each with its standard error, exit code, and the destination's state afterwards;
- the overwrite capture, showing a prior run's file replaced;
- stream sizes for a 1,000-tick run and a 10,000-tick traced run;
- the static-check results: the library target's freedom from filesystem calls, the single emission site, the rule 5
  item-for-item comparison, the rule 6 re-check, the absence of any classification field, and the absence of any
  floating-point operation in a record path;
- the workspace test census before and after, reconciled name by name;
- `cargo fmt`, `cargo clippy`, `cargo test` and `cargo tree -p Mokiterions` output;
- the eight manual assessments above, each with its accountable role and date;
- the amendment-approval check of oracle 7, and the recorded state of `ARCH-MOK-001`'s outstanding 2026-08-18 row.

Evidence is retained in the repository, is reproducible from the recorded commands and commit, and contains no
secret. **No retained sink stream carries the path it was written to**, which is what makes this evidence class safe
to retain at all.

## Residual uncertainty

- **The alphabet argument is only as complete as the enumeration.** Oracle 5 is exhaustive over
  `SPEC-MOK-006` rule 3.2 and the size assertions make a silent addition fail, but a field added to the stream *and* to
  the specification's enumeration *and* wrongly characterized there would pass. The mitigation is that the enumeration
  is short enough to read, and that oracle 3 parses every capture with a parser this repository does not own.
- **Oracle 3 checks the captures, not the format.** A value form that never occurs in the declared matrix is never
  parsed. Oracle 5 is what covers the domains rather than the runs, and the two together are stronger than either.
- **Non-perturbation is verified over the declared matrix and at every tick within it.** A draw added only at a
  density outside the sweep, or only past tick 1,000, would pass oracle 1. Oracle 4 reduces this substantially by
  asserting state equality per tick rather than inferring it from output, but it too runs on the declared matrix.
- **The replay consumer is a second implementation of part of the engine.** Oracle 6's strength is that the two
  disagree when either is wrong; its weakness is that a misreading of `SPEC-MOK-001` shared by both would agree
  incorrectly. Manual assessment 8 is the mitigation and it is a judgement, not a proof.
- **The metrics record's per-tick figures are reconciled, not independently derived.** For capacity and permanent
  depletion there is no event stream to replay against, so those two fields are checked against the engine's own state
  and against the density's resolution. They are the fields with the weakest independent witness in this contract.
- **`fear`'s maximum is recorded and means nothing yet.** Nothing reads `fear`, so its sum and extremum are
  well-formed figures about an attribute with no consumer. `VER-MOK-010` recorded the same residual and this contract
  inherits it unchanged.
- **No claim is made about a consumer.** No reader, parser or schema file is product. The reconstructor and the replay
  consumer exist for verification, are retained as evidence, and are not maintained artifacts.
- **The schema version is verified to be present, not to be right.** Whether `1` is the correct starting value and
  whether rule 10.2's increment triggers are complete cannot be verified before a second version exists. The first
  increment will be the first real test.
- **Nothing here verifies that the records are useful.** Manual assessment 1 is the only place that question is asked,
  and it is asked of a person. A stream that satisfies every automated check in this contract and still fails to
  answer Phase 4b's questions would have passed verification and missed `INT-MOK-009`.
