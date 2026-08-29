+++
id = "SPEC-MOK-002"
type = "specification"
title = "Crate targets, public interface, and test placement"
status = "approved"
owners = ["technical owner"]
created = "2026-08-17"
updated = "2026-08-29"

[relations]
specifies = [
  "REQ-MOK-016",
  "REQ-MOK-017",
  "REQ-MOK-032",
  "REQ-MOK-033",
  "REQ-MOK-050",
  "REQ-MOK-052",
  "REQ-MOK-053",
  "REQ-MOK-054",
  "REQ-MOK-055",
  "REQ-MOK-056",
  "REQ-MOK-057",
]
+++

# Specification: Crate targets, public interface, and test placement

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-17 | Original approved content for `REQ-MOK-016` and `REQ-MOK-017`. | Approved; implemented under `WO-MOK-003` and verified under `VREC-MOK-003`. |
| 2026-08-18 | Four provisions amended so that the terminal observer of `SPEC-MOK-003` can be conformed to. **Rule 1**: "no second package, no workspace" narrowed to a workspace of exactly two packages, on the approved requirement `REQ-MOK-026` that the clause reserved the exception for. **Rule 3**: the clause freezing `src/cli.rs` and `src/simulation.rs` scoped to the `WO-MOK-003` restructuring it was written for, so that an approved requirement may add code to them. **Rule 5**: the closed enumeration grown by the read-only observation surface, under rule 5's own growth clause. **Rule 6**: the prohibition narrowed from five named value types to the capability it was written to deny. Nothing about mutation, dependency direction, determinism or observable behavior is relaxed, and the engine package's dependency table stays empty. | **Ratified 2026-08-20 by the repository owner acting as technical owner**, all four provisions as written and without modification, in the assessment review recorded under `WO-MOK-012`. It was **OUTSTANDING** from 2026-08-18 until that act. All four were approval preconditions of `WO-MOK-005`, alongside the 2026-08-18 amendment to `ARCH-MOK-001`, and none could have been part of the 2026-08-17 approval of the observer chain: this specification was not on that branch when the approval was given, and it reached `master` afterwards. The owner was shown each provision separately, together with the clause it narrows and the approved requirement it rests on, and ratified each on its own terms rather than as a block. The implementation agent wrote this text and decided none of the substance. `VREC-MOK-003`, which binds this specification's 2026-08-17 content to `WO-MOK-003`'s commit, is not edited: what it verified was correct at its commit. |
| 2026-08-18 | Every root-relative path re-based on `mokiterions-core/`, the engine package's own directory, and rule 1's "unchanged in source location" clause corrected. Stated once in *Scope* under **Paths** and at rule 1, so that no rule's substance is restated and none is re-opened. The two target paths in rule 1's table, the file list in *Inputs*, the rule 3 and rule 4 file names, rule 5's `grep` check, rule 8's file table and rule 9's locations all move by prefix alone. No file is renamed, no rule changes what it requires, no target, target name, target kind or package name changes, and the engine package's dependency table stays empty. `REQ-MOK-030` is the approved requirement; `SPEC-MOK-004` rules 1 to 3 fix the layout and `ADR-MOK-004` decides it. | Approved 2026-08-18 by the repository owner as technical owner, by way of `ADR-MOK-004`, whose *Required amendments* section states this amendment in full. The implementation agent wrote the text under `WO-MOK-006`; it did not decide it. `VREC-MOK-003`, which binds this specification's 2026-08-17 content to `WO-MOK-003`'s commit, is not edited: the paths it names were correct at its commit and this row records why they differ afterwards. The 2026-08-18 row above is untouched. It was **OUTSTANDING** when this row was written, and the technical owner ratified it on 2026-08-20 under `WO-MOK-012`. |
| 2026-08-19 | Rule 5's enumeration amended in two entries, under `REQ-MOK-032` and `REQ-MOK-033`. `simulation::Policy` gains a third variant, `Individual`; `Default` is unchanged and still resolves to `Reference`. `simulation::AgentSnapshot` carries four `u8` attributes rather than three, the fourth being `fear`. Its justification holds unchanged, because `REQ-MOK-032` requires `fear` in the event stream as well. Rule 6 is **not** amended and was re-checked instead: the added field carries a value, so no public item yields a mutable borrow of or a reference into authoritative state, and the trait-aware source and the `Observation` it consumes stay private, keeping the `ADR-MOK-001` trust boundary where it is. `waste_tolerance` deliberately does **not** join the snapshot: no approved requirement needs the observer to render it, and rule 5 holds the interface to what approved requirements need. It reaches the observer through the event log, which `REQ-MOK-022` already retains. Public interface growth is therefore exactly two items. | Approved 2026-08-19 by the repository owner acting as technical owner, together with `WO-MOK-010`. The implementation agent wrote the text and did not decide the substance. **The two rows above this one, dated 2026-08-18, are untouched.** They belonged to `WO-MOK-005` and were awaiting the same owner's separate act, which came on 2026-08-20 under `WO-MOK-012`. This sentence as originally written said both rows remained **OUTSTANDING**, which overstated it: only the first of the two ever was, the second having been approved the same day by way of `ADR-MOK-004`. The miscount is recorded rather than silently dropped, and is reported in `WO-MOK-012`. `VREC-MOK-003`, which binds this specification, is not edited. |
| 2026-08-20 | **Rule 1's empty-table rule withdrawn** and replaced by the declared-set form, decided by `ADR-MOK-006`. *"The dependency and dev-dependency tables stay empty, with no exception, including a dependency shared with another package in the same workspace"* becomes: both tables contain exactly what rule 13 declares, at the declared versions and feature sets; dev-dependencies are declared the same way and are **not** exempt; a crate shared with the observer is admissible only as a declared entry of both, with `SPEC-MOK-004` rule 1 governing where its version is keyed. *"No third target and no build script"* is **unchanged and stated as unchanged** — it binds this repository's own package, while `ADR-MOK-006` decision 13 concerns a *dependency's* build script, which this rule never spoke to. Rule 1's `cargo tree` sentence becomes the declared-set comparison with offline resolution beside it, the superseded wording recorded in place. **New rule 13, *Declared dependency set***, holds the engine package's table — crate, version, features, build script, admitting amendment — and **the table is empty as this amendment lands**, which is now a fact about the declaration rather than a rule; rule 13 also states the five checks an added row must pass and that adding one is an amendment approved by the technical owner, not an implementation act. Rule 13 is a **new** number: rules 1 to 12 keep their numbers, so every citation of them elsewhere is unaffected, and no statement of a total rule count exists to update. Rule 13 also fixes how a *Features* cell is read mechanically, since `SPEC-MOK-005` rule 8.4b is a program and a program left to guess at the reading would be a second declaration; the convention binds `SPEC-MOK-003`'s cell, which is the only one with content. **`REQ-MOK-050` joins `specifies`**, which `ADR-MOK-006` did not enumerate and which is disclosed here for that reason: rule 13 is the engine package's half of the declaration that requirement is about, and `ARCH-MOK-001` names the requirement in `addresses` while conforming to this specification, so without the relation the requirement would have a rule written for it and no declared coverage by it. | Approved 2026-08-20 by the repository owner acting as accountable technical owner, by way of `ADR-MOK-006`, whose *Required amendments* section states this amendment in full. Written under `WO-MOK-014`; the implementation agent wrote the text and did not decide it, and chose no crate, version or feature set — there is none to choose while the table is empty. **Neither 2026-08-18 row above was touched.** This sentence first said both "stay OUTSTANDING", which repeated the miscount `WO-MOK-012` had already found and corrected in the 2026-08-19 row: only the first of the two ever was outstanding, and the second, the path re-basing, was approved the same day by way of `ADR-MOK-004`. The first was `WO-MOK-005`'s precondition, this approval neither cleared nor inherited it, and the repository owner acting as technical owner ratified it as written on 2026-08-20 under `WO-MOK-012`, which reached this branch by merge after this row was written. The miscount is recorded rather than deleted, following that work order's precedent. `VREC-MOK-003`, which binds this specification's 2026-08-17 content, is not edited. |
| 2026-08-20 | Rule 5's enumeration amended and rule 6 re-checked, under `CAP-MOK-010`, and the frontmatter's `specifies` gains `REQ-MOK-052` through `REQ-MOK-057`. `simulation::Policy` gains a fourth variant, `Social`; `simulation::Action` gains seven target-carrying variants; `simulation::EventType` gains `AttackResolved`, `ThreatResolved` and `SurrenderResolved` with their three `EventDetail` payloads, and `EventType::ALL`'s length moves from `12` to `15`, which is public-surface growth because that array is a `pub const`. A growth table states each figure and a paragraph states what does **not** grow: no verb reaches rule 3's valid-proposal list, so the observation's existing fields keep their types. **The observation's two new fields are not interface growth**, because `Observation` is declared without `pub` at `mokiterions-core/src/simulation.rs:500` and rule 6 lists it among the ten names that stay private; the distinction is load-bearing, and it is the one place `WO-MOK-016` stated the opposite, which is corrected there. Rule 6 is re-checked and recorded as **not amended**, cross-agent mutation being introduced for the first time: a target is an identifier and not a reference, the mutation is entirely inside the engine, the three event types carry copies of printed values, all ten prohibited names stay private, and no `pub(crate)` is widened. | Approved 2026-08-20 by the repository owner acting as technical owner, in the **single act this amendment's own ordering requires**: together with `REQ-MOK-051` through `REQ-MOK-060`, `VER-MOK-016` and `WO-MOK-016`. The act is single because this amendment's `specifies` relation is what makes those ten requirements approvable at all — without it `validate` raises `E007` on every one of them and `preflight --phase start` raises `W016`, both measured on 2026-08-20 and recorded in that work order. Implementation begins after this act and not before. It is stated in full in `WO-MOK-016`'s *Required amendments* section. The implementation agent wrote the text and did not decide the substance: the eleven values it fixes were the owners' decisions of 2026-08-19 and 2026-08-20, and the three the validation did not supply were taken on 2026-08-20, all recorded in that work order's *Decision record*. Eight consequences the text derived rather than decided are named in that work order's *Required amendments* section; the owner took the four of them that were genuinely open before approving, and those four are recorded in its decision table with the alternatives declined. |
| 2026-08-20 | **Rule 5's growth table gains a fourth row, which the amendment above omitted**: `simulation::EventDetail`'s pre-existing `ActionTrace` variant gains one field, `suffered: Vec<(String, u8)>`, appended after `fear`. Growth under `CAP-MOK-010` is therefore `1 + 7 + 3 + 3 + 1` and the table says "four items change shape" where it said three. The omission was a defect in the enumeration and not in the implementation: the row above enumerates added *variants*, and a field appended to a public variant that already existed is the one form of growth such an enumeration does not catch. `SPEC-MOK-001` rule 7 obliges the trace line to report the suffered-attack record and its rule 6 fixes the line's shape, but neither can admit a field to this interface, which is closed here. The field is a `Vec` of pairs of a `String` and a `u8` rather than of the engine's `SufferedAttack`, so **no type is added and rule 6's ten private names are untouched** — both halves of a pair are already public values. Rule 6 needs no further re-check: a pair of copies grants no path into engine-owned state, which is what the 2026-08-20 re-check above establishes for the three event payloads on identical grounds. | Approved 2026-08-20 by the repository owner acting as technical owner, in a **separate act** from the amendment above, the omission having been found after that act was taken. The alternatives were put with it and declined: bundling the row with `REQ-MOK-060`'s deferred numeric amendment, which would leave this record knowingly incomplete in the interval, and treating the field as covered by `SPEC-MOK-001`'s trace provision, which would let a behavior authority admit an item to an interface this specification closes. The implementation agent found the omission while comparing the engine's public surface against this table for `VER-MOK-016`, and wrote the row; it did not decide the substance. **The implementation is unchanged by this amendment** — the field was already present and already in pair form, at `mokiterions-core/src/simulation.rs:1425` where its own comment gives that reason — so this row records the interface authority catching up with an approved obligation, not a code change. It is stated in full in `WO-MOK-016`'s *Required amendments* section as provision 3. |
| 2026-08-20 | Five provisions amended so that `SPEC-MOK-006`'s record stream can be conformed to, under `REQ-MOK-042` through `REQ-MOK-046`. **Rule 4**: `execute` gains exactly one parameter, `records: Option<&mut dyn Write>`, and nothing else; the exit codes are unchanged and none is added, a record-sink write, flush or close failure being an output failure and therefore `1`. **Rule 5**: the `execute` row reworded from "two writers" to "the caller's writers", the enumeration otherwise untouched — a parameter is not an item, so the interface grows by no item, and the rows for `cli::Command`, `simulation::Config` and `simulation::Simulation::run` are **not** amended. **Rule 5's mechanical checks**: restated as two greps for `execute`'s signature, and the mutating-method check recorded as still returning exactly `run` and `advance_tick`, with the crate-private carrier `run_recording` named so that its non-match is disclosed rather than relied on silently. **Rule 6**: **not** amended, and the omission recorded at the rule — `SplitMix64` stays private, the ten prohibited names stay ten, and the entropy value the projection reads is an owned `u64` behind `#[cfg(test)]`. **Scope and *Compatibility and migration***: `SPEC-MOK-006` named as the authority on the stream and this specification as the authority on the seam, with the four `execute` call sites listed and `mokiterions-tui` recorded as passing `None`. Nothing about mutation, dependency direction, determinism or observable text behavior is relaxed, no target or package changes, and this amendment adds no row to the engine package's declared dependency set, which is still empty. This sentence originally read "the engine package's dependency table stays empty", invoking a rule of rule 1 that `ADR-MOK-006` withdrew on the same date for the declared-set form; the superseded wording is recorded rather than deleted, and the `ADR-MOK-006` row above is the authority. | Approved 2026-08-20 by the repository owner acting as technical owner, by way of `ADR-MOK-005`, whose *Required amendments* section states this amendment in full and which the same owner accepted on the same date. The implementation agent wrote the text under `WO-MOK-019`; it did not decide the substance. **The first 2026-08-18 row above was not touched.** It was **OUTSTANDING** when this row was written, and the repository owner acting as technical owner ratified it as written on 2026-08-20 under `WO-MOK-012`, in the assessment review that reached this branch by merge after this row was written. It was `WO-MOK-005`'s precondition; nothing here depended on it, and this approval neither cleared nor inherited it. `VREC-MOK-003` and `VREC-MOK-010`, which bind earlier content of this specification to their commits, are not edited; `VER-MOK-012` covers this amendment. This row reached the table by merge, after the three rows above it, which is why it is last on a date it shares with them. |
| 2026-08-23 | **Rules 4 and 5 amended so that `SPEC-MOK-007`'s decision port can be conformed to, under `REQ-MOK-063`, and rules 6 and 13 re-checked and recorded as unmoved.** **Rule 4**: `execute` gains exactly one parameter, `port: Option<&mut dyn Proposer>`, bringing it to five, and nothing else; the exit codes are unchanged and none is added, `SPEC-MOK-007` rule 20.8's refusal — this source selected with no port supplied — being an invalid configuration and therefore `2`. Where the port and the record sink part company is recorded at the rule: a missing sink is silently no records, a missing port under the one source that needs it is refused, because rule 9.7 forbids borrowing another source's selection and there is no run left to fall back to. **Rule 5, amended in three ways**: it gains `simulation::Proposer` and `simulation::DecisionRequest` as items, both values or a function of a value; `Simulation::advance_tick`'s row in the observation-surface list gains the parameter, which is where the second of `SPEC-MOK-007` rule 20.5's two doors is amended, `advance_tick` being enumerated by that list and not by rule 4; and the mechanical checks are restated. Growth is enumerated item by item as `1 + 1 + 1 + 1` — one `Policy` variant `Llm`, one parameter on `advance_tick`, and the two added items — with `execute`'s parameter counted at rule 4 and not twice, and no `pub const` changed. **Rule 5's mechanical checks**: `execute`'s form becomes three greps rather than two, the third being `port: Option<&mut dyn Proposer>`, and the failure conditions are stated against five parameters rather than four. This restatement is not editorial: the standing 2026-08-20 text reads "A fifth parameter, a second sink, or a sink that is not optional fails the second", and the port on `execute` **is** that fifth parameter, so leaving it standing would give this specification a drift check that condemns the build the specification requires. The mutating-method check still returns exactly `run` and `advance_tick`, and it **gains a second obligation it did not have**: `advance_tick`'s signature must be one line in the source, because a signature the formatter wraps separates the declaration keyword from the receiver and the check would then report one door where there are two — passing while doing so, which is a weakened check and not a failing one. That obligation is why `Proposer` is spelled as it is, and the rule records the width measurement rather than the preference. The pattern must also not appear in prose in that file; it matched a documentation comment during implementation and the comment was reworded rather than the check loosened. **Rule 5's first list**: `simulation::Policy`'s row gains the variant `Llm`; `Default` is unchanged and still resolves to `Reference`, and `simulation::Simulation::run`'s row is **not** amended — it delegates with the port absent, for the second amendment running. **Rule 6**: **not** amended, and the check recorded at the rule because a reader would expect the opposite of a public trait a caller implements. The request crosses by value and the proposal returns by value, so the port is a use of rule 6 rather than an exception to it; the ten prohibited names stay ten and stay private, `Observation` never crossing and `DecisionSource` gaining a fifth implementation that stays internal. **Rule 13**: **not** amended, and re-measured rather than asserted — the table is still empty and `cargo tree -p Mokiterions -e normal --locked --offline` still resolves to one crate on 2026-08-23, which is `ADR-MOK-007` decision 3's whole point. **What this row does not do**: the four remaining amendments `ADR-MOK-007` requires of this specification are **not** made here. Two of them — *Security and privacy properties*' first bullet and *Actors and external systems* — describe the binary target spawning a connector, passing its environment to the child and interpreting two more operator-supplied paths, and **no commit has yet made that true of this tree**; writing them now would put a false statement in an approved specification. They land with the code they describe, under `WO-MOK-025` scope item 14. Nothing about mutation, dependency direction, determinism or observable text behavior is relaxed, no target or package changes, and no `Config` field is added. | Approved 2026-08-23 by the repository owner acting as technical owner, by way of `ADR-MOK-007`, whose *Required amendments* section states rule 4's amendment and rule 5's three amendments in full and which the same owner approved on the same date, together with `WO-MOK-025`, whose scope item 1 requires that rule 5's restated checks land in the same commit as the code. The implementation agent wrote the text under `WO-MOK-025`; it did not decide the substance. **Two things in this row are the implementation agent's and are marked as such.** The identifier `Proposer` is one: `SPEC-MOK-007` rule 1.1 fixes the interface's shape and no approved artifact names it, `WO-MOK-025`'s decision envelope leaves local naming to the agent, and the choice was forced by the line-width consequence this row records rather than preferred on its merits — `DecisionPort`, the artifacts' own words, does not fit. The second is the **second obligation now attached to the mutating-method check**, which no artifact anticipated: it was found by the check failing, on `mokiterions-tui/tests/verification.rs`'s `the_engine_still_exposes_exactly_two_mutating_entry_points`, and the alternative considered and **rejected** was restating the check as two greps on `execute`'s own 2026-08-20 precedent, which would have contradicted a literal sentence of the approved work order. Both are disclosed here rather than left in a diff. `VER-MOK-018`'s `S4a` runs the restated checks. No record bound to a commit is re-opened: `VREC-MOK-003`, `VREC-MOK-010` and `VREC-MOK-012` bind earlier content of this specification to their commits and are not edited. |
| 2026-08-24 | **The last two amendments `ADR-MOK-007` requires of this specification, which the 2026-08-23 row declined to write because no commit had yet made them true. Both take a target scope this specification has never needed, and both are written as far as this tree makes them true and no further.** ***Security and privacy properties*, first bullet.** *"No network access, credential read, filesystem access, environment read, or wall-clock read is introduced"* now says of **which target**. Of the **library target** all five continue to hold, and that half is the load-bearing one: `src/lib.rs`, `src/cli.rs` and `src/simulation.rs` contain no `std::fs`, no `File::`, no `OpenOptions`, no `remove_file`, no `env::`, no process spawn, no socket and no clock, **measured at the candidate rather than asserted**, which is what lets the port arrive already constructed and the transcript already open. Of the **binary target** filesystem access holds no longer and has not since 2026-08-20; it now interprets **two** operator-supplied paths, `--events-path` written and `--transcript-path` read, and the other four properties still hold of it — the command line is read and no environment variable, no socket is opened, no process is spawned and no clock is read. ***Actors and external systems*.** The closing bullet takes the same scope: of the library target it stands word for word, and of the binary target a filesystem location **does** participate — since 2026-08-20 as the record stream's destination, which nothing reads back, and now as **a source the run's decisions come from**, which is the stronger case because the transcript's bytes decide what is proposed. No external service, no network endpoint and no credential participates in either target. **What is not written, and why.** `ADR-MOK-007` states that three of the five properties stop holding of the engine **package** and that this section gains the connector as a process the binary target starts; the third and fourth are the spawn and the environment passed to that child, and **neither exists in this tree.** `WO-MOK-025`'s *Out of scope* excludes the connector, its protocol implementation, the canned connector and any process spawn, so the only decision source outside the engine that exists here is a replay, which starts nothing. Both are **`WO-MOK-026`'s to write**, in the target-scoped form these two amendments establish, and each section says so in as many words rather than leaving a reader to infer it. **The 2026-08-23 row's forecast is corrected here and that row is not edited.** It said these two *"land with the code they describe, under `WO-MOK-025` scope item 14"*. Item 14 is where they land, and the transcript's half of them is written; the connector's half cannot be, because the same work order excludes the code it describes. A row records the act it recorded, so the correction is a later row — the `ARCH-MOK-001` precedent of the same date. **Nothing else moves.** No rule is amended: rules 4 and 5 were amended on 2026-08-23 and are untouched here, rules 6 and 13 stand as that row recorded them, the ten prohibited names stay ten, the dependency table is still empty, no target or package changes, no `Config` field is added, no public signature moves and no mechanical check is restated. A process spawn appears in the **public-tier tests**, which invoke the compiled binary in order to observe a process boundary from outside it; that predates this amendment, it is what a process-boundary test is, and it is not a target spawning a child. | Approved 2026-08-23 by the repository owner acting as technical owner, in the act *"i approve the artifact pack"*, and by way of `ADR-MOK-007`, whose *Required amendments* section states both provisions in full — the same act that approved the 2026-08-23 row above, which is why these two carry that date's authority and this date's text. The implementation agent wrote the text under `WO-MOK-025` and decided none of the substance; the target scope is `ADR-MOK-007`'s own instruction, *"the sentence gains the target scope it has not needed until now"*. **Stop condition 6 of `WO-MOK-025` was not invoked**: both provisions are ones this ADR names, and the two partial landings are the form `SPEC-MOK-002`'s own approved *"What this row does not do"* paragraph fixed and which `SPEC-MOK-001`, `SPEC-MOK-003` and `ARCH-MOK-002` used on the same authorization. **The measured figures in this row were taken at the candidate commit and none is inferred from an unchanged total**: the library-target absence of all five properties, the count of two interpreted paths, and the location of every filesystem call in `src/main.rs` alone. Sibling rows for the same act stand in `SPEC-MOK-004`, `SPEC-MOK-007`, `SPEC-MOK-003`, `ARCH-MOK-002`, `ARCH-MOK-001`, `INT-MOK-001` and `SPEC-MOK-001`. No record bound to a commit is re-opened: `VREC-MOK-003`, `VREC-MOK-010` and `VREC-MOK-012` bind earlier content of this specification and are not edited. |
| 2026-08-24 | **Rule 5's additions list gains `ReplayPort` and two `DecisionRequest` accessors, and rule 6 states that a reference into a value the caller owns is outside it.** Two findings, both raised by `WO-MOK-025`'s own static checks against this rule at its candidate, and both about this specification rather than about the build. **Rule 5's closing prohibition** — "nothing outside the three lists becomes public" — was contradicted by four of the twelve added public declarations. `simulation::ReplayPort` and `ReplayPort::new` were authorized in substance and unenumerated in form: `ARCH-MOK-002` names the type — "the transcript's parsing is the engine's `ReplayPort`, in the engine package" — and `SPEC-MOK-007` rule 12.1.1 puts the opening of the transcript in the host, so **two crates construct it**, `mokiterions-core/src/main.rs:85` and `mokiterions-tui/src/main.rs:118`, and it cannot be narrowed to `pub(crate)` without contradicting one of those two artifacts. It gains a growth row and an admissibility row. `DecisionRequest::tick` and `DecisionRequest::actor_id` were outside the 2026-08-23 row's wording, which described "per-part accessors returning `&str` and one accessor returning them in the composition order" — five accessors, the four blocks and `blocks` — where these are a sixth and a seventh and `tick` returns `u64`; they bind a proposal to the opportunity it answers, and their only callers outside the crate are public-tier tests. The growth arithmetic becomes `1 + 1 + 1 + 1 + 1`. **The two counting conventions are now both stated with their decomposition**, because the arithmetic disagreeing with a `pub` census was itself a source of doubt: five items, twelve declarations, and which expands into which. **Rule 6 gains a stated non-example.** Its first bullet forbids a reference into seven named things, and a `&str` borrowed from a `DecisionRequest` handed to the caller by value is none of them — the referent's owner is the caller. The rule now says so and says what follows for its mechanical form: the carve-out must admit both a `'static` referent and a caller-owned one, because a form admitting only the first reports every accessor of every value type this rule admits and so **fails a conforming build**. That is the same hazard this rule's 2026-08-23 row identified for `execute`'s grep, in the same position, found the same way. **What this row does not do**: no capability is relaxed, no prohibited name becomes public, no mutating method is added — the grep still returns exactly `run` and `advance_tick` — rule 13 is untouched, and no check is edited to pass. The build is unchanged by this row: no source file is amended for it. | **Approved 2026-08-24 by the repository owner acting as accountable technical owner.** Both findings were raised by the implementation agent as `WO-MOK-025`'s escalations **E13** and **E14**, put to the owner together with nine siblings with each measurement displayed, and approved in the turn the question was asked. Neither was repaired when found: `candidate/static-checks.txt` records both as FINDING and states why it does not repair them — "a check edited to pass is not the same check" — and `WO-MOK-025` stop-and-escalate condition 6 forbids amending an approved artifact on an implementation agent's judgement. **The agent's recommendation on E14 was wrong in its mechanism and the record says so**: it proposed naming `EventType::as_str` in the carve-out, where `as_str` is already carved out by the `'static` clause and the six references actually remaining are `DecisionRequest`'s accessors. The owner approved the correction the measurement supports, not the one first proposed. The implementation agent measured every figure in this row and wrote the text; it decided none of the substance. No record bound to a commit is re-opened. |
| 2026-08-29 | **Rule 5's census gains `simulation::Config`'s `spend_ceiling` field, which a commit had already added.** The census enumerated that struct's public fields as exactly `seed`, `tick_limit`, `policy`, `density` and `trace_actions`; commit `c13c327` under `WO-MOK-026` added a sixth, `spend_ceiling: Option<u64>`, and so falsified the census **before** an amendment authorized it. That ordering is wrong and is recorded rather than tidied: it was found by the 2026-08-29 conformance pass and not by a gate, because `validate` reads this census as prose and cannot compare it to a struct. The field is **not removable** without stalling `WO-MOK-026` items 9 to 11 — `SPEC-MOK-007` rule 14.6 stops the run *before* an exchange and rule 15.2 puts the ceiling in the run record, neither of which a host can do on the library's behalf without the library knowing the number, and every alternative route is also an interface change: a sixth `execute` parameter moves rule 4, and putting the ceiling on the port leaves the library unable to write the run record rule 15.2 requires. This row is therefore **the census catching up with a change already made**, which is the same shape as this table's 2026-08-20 row for `EventDetail::ActionTrace` and is disclosed for the same reason that row disclosed itself. **What this row does not do**: rule 4 is untouched and `execute` keeps five parameters; rule 6's ten prohibited names stay ten and stay private, a `u64` behind an `Option` granting no path into engine-owned state; rule 13's declared dependency set is untouched and still empty; and no mechanical check is restated, the field being a public field of an already-public struct rather than a new item or a moved signature. | **Approved 2026-08-29 by the repository owner acting as accountable technical owner**, in four decisions taken in the turn each question was asked: the US cent as the minor unit; `--prices` as a compact option rather than a file; a retry bound of three; and the provider binding staying in the connector with the response reporting it back, over the two alternatives of telling the engine or leaving the request's fields advisory. A fifth decision routed the work into one chain rather than four. The implementation agent ran the pass, measured every figure and wrote the text; it decided none of the substance. |
| 2026-08-29 | **Rule 5's census gains `simulation::Config`'s `prices` field and its additions list gains `simulation::UnitPrices`, before the commit that adds either.** `SPEC-MOK-007` rule 14.3a, approved the same day under `WO-MOK-030`, puts the run's four unit prices on the command line as `--prices <prompt:cached:output:reasoning>` and obliges the shared parser to **retain** them, unlike the paths it validates and discards. A retained value has to be somewhere on this interface, and rule 4 is not it: a sixth `execute` parameter would move a signature this specification freezes and checks with three greps. So `Config` gains `prices: Option<UnitPrices>` and the additions list gains the value type it names, with its four public `u64` fields and its one associated function `parse`. **Growth is one item and seven public declarations**, both figures stated with their decomposition under the 2026-08-24 convention, and a growth table at the rule enumerates it. **The type is a named type rather than four bare `u64` fields or a `[u64; 4]`**, which is the one substantive choice here and is the implementation agent's under `WO-MOK-026`'s decision envelope, that envelope reserving the unit-price *representation* to the agent while the *unit* — the US cent — was the owner's decision of the same day: three of the four prices are plausible values for each other's position, so a positional or unnamed form puts a silent eighty-fold cost error one transposition away from a run this repository pays for. **The four fields are public where `Density`'s single field is private**, and that difference is disclosed rather than glossed: only a public-tier test that reads the four separately can establish that a transposed `--prices` is refused rather than accepted, comparing one `parse` against another proving nothing about order, and public-tier tests are the type's only callers outside the crate because rules 14.1, 14.2 and 14.6's arithmetic is inside it. There is no `Default`, rule 14.3 forbidding a compiled-in price, and a live run with none declared is refused by `cli::parse` rather than run at a guess. **What this row does not do**: rule 4 is untouched and `execute` keeps five parameters, its three greps standing word for word; the mutating-method grep still returns exactly `run` and `advance_tick`, `parse` having no receiver; rule 6 is **not** amended and is re-checked instead, the ten prohibited names staying ten and four `u64` copies of the operator's own command line granting no path into engine-owned state, on the grounds this rule already admits `Coordinate`'s two public `u8` fields; and rule 13's declared dependency set is untouched and **still empty**, measured rather than assumed, `parse` being hand-written against `str::split` and `str::parse`. **The ordering is deliberately the opposite of the row above.** | **Authorized 2026-08-29 by the repository owner acting as accountable technical owner**, in two acts. The substance is `SPEC-MOK-007` rule 14.3a, which that owner approved the same day under `WO-MOK-030` together with the US cent as rule 14.2's minor unit; this specification decides nothing about the option, its four integers, their order or their retention. The **authority to edit this file at all** is the second act: `WO-MOK-026`'s execution scope did not admit it, `harnessctl check … --changed-path docs/engineering/simulation/specifications/SPEC-MOK-002.md --changes-complete` measured `QGP-G4I-PATHS: WEX201` and directed the escalation under `DR-REMEDIATION-SCOPE`, and the owner was shown that measurement with two routes — a scope amendment to `WO-MOK-026` or a separate governance work order — and chose the scope amendment, the alternative being declined and recorded in that work order's own amendment record. The implementation agent wrote this text and decided only what the decision envelope reserves to it, disclosed above and in the source's own documentation. No record bound to a commit is re-opened: `VREC-MOK-003`, `VREC-MOK-010` and `VREC-MOK-012` bind earlier content of this specification and are not edited. |
| 2026-08-29 | **Rule 5's census records the port's grown return: `simulation::Proposal` and `simulation::ReportedUsage` are added, and `simulation::Proposer`'s proposing method returns the first where it returned `Option<Action>`.** `SPEC-MOK-007` rules 1.1a and 1.4a, approved the same day under `WO-MOK-026`, grow the port's return to carry "the evidence of the exchange the proposal came from — the response as received, and the provider's four reported token counts", because rule 11.3 obliges the exchange record to carry all three, rule 11.1 puts the authoring of every record in the engine, and the port is the engine's only contact with what answered. A trait on this interface cannot change shape without this rule saying so, and rule 5 closes with "nothing outside the three lists becomes public". **Growth is two items and ten public declarations**, both figures stated with their decomposition under the 2026-08-24 convention, and a growth table at the rule enumerates it: `Proposal`, its three public fields and `Proposal::nothing`, then `ReportedUsage` and its four public `Option<u64>` fields — with `Proposer`'s row counting `0`, a return type changing being a change of shape on an item this rule already encloses, which is the `ActionTrace` form of the 2026-08-20 row and the form `Config`'s row of one day earlier takes. **Two substantive choices are recorded rather than left in a diff, and both are the implementation agent's under `WO-MOK-026`'s decision envelope.** The counts are a **named type** rather than four bare fields on `Proposal`, on `UnitPrices`' measured ground: four unlabelled integers of similar magnitude in a fixed order, from which rule 14's cost and `REQ-MOK-070`'s ratio are both computed, put a transposition one keystroke from a wrong figure in a run this repository pays for. And the four are `Option<u64>` and not `u64`, which is rule 11.5 in a signature — "a reported count that the provider did not report is recorded as **absent**, not as zero", and rule 14.5 depends on telling the two apart. **What this row does not do**: rule 4 is untouched and `execute` keeps five parameters with its three greps word for word, the port parameter's type naming the trait and not its return; the mutating-method grep still returns exactly `run` and `advance_tick`, `Proposal::nothing` having no receiver; `Simulation::advance_tick`'s signature stays one line, no parameter of it moving; rule 6 is **not** amended and is re-checked instead, the ten prohibited names staying ten and what the return gained being an owned `String` the port composed and four `Option<u64>` copies of figures a provider reported; and rule 13's declared dependency set is untouched and **still empty**, measured rather than assumed, nothing here being parsed by a crate. The 2026-08-23 and 2026-08-24 rows are **not edited**: each records the growth it recorded, and the two places where rule 5 states the old return carry a dated parenthetical pointing at this row rather than a rewritten claim. | **Authorized 2026-08-29 by the repository owner acting as accountable engineering owner**, in two acts. The substance is `SPEC-MOK-007` rules 1.1a and 1.4a, which that owner approved the same day under `WO-MOK-026` over two measured alternatives — recording the growth's admissibility here while leaving those rules word for word, and a second port method the engine called after each proposal, whose cost was a temporal contract between two calls that no type enforces so that a port returning the previous exchange's evidence would write a wrong record in silence. This specification decides nothing about the port's shape. The **authority to edit `SPEC-MOK-007` at all** is the second act, recorded in `WO-MOK-026`'s amendment record together with a second admission the trait change forces, `mokiterions-tui/src/state.rs`, whose `LentPort` implements this trait. This file was already in that work order's execution scope, admitted earlier the same day for `Config`'s `prices` field. The implementation agent wrote this text and decided only what the decision envelope reserves to it, disclosed above and in the source's own documentation. No record bound to a commit is re-opened: `VREC-MOK-003`, `VREC-MOK-010` and `VREC-MOK-012` bind earlier content of this specification and are not edited. |
| 2026-08-29 | **Rule 5's census gains `simulation::ConnectorPort`, the port `SPEC-MOK-007` rule 10's connector binding requires, and the frontmatter's `updated` field is corrected.** Rule 10.1 puts the live provider behind "an executable the operator names by path as a host option" that "the host spawns as a child process", rule 20.1 makes the engine's binary target that recording host, and `SPEC-MOK-006` rule 1.2 keeps every process and every path resolution out of the library target. The two halves of one live exchange therefore fall on opposite sides of a crate boundary — the host connects the pipes, the engine composes the request and applies rule 10.4's grammar check to the response — and something public has to carry the second half to the first. A trait implementation cannot be `pub(crate)` when its one constructor is reached from `src/main.rs`, which is a separate crate from `src/lib.rs`, and rule 5 closes with "nothing outside the three lists becomes public". **Growth is one item and two public declarations**, both figures stated with their decomposition under the 2026-08-24 convention, and a growth table and an admissibility table at the rule enumerate it: `ConnectorPort` and `ConnectorPort::new`, which is `ReplayPort`'s decomposition of 2026-08-24 item for item — a type and its one associated function. **No field is public**, unlike `UnitPrices`', `Proposal`'s and `ReportedUsage`'s, because nothing outside the crate assembles a port from its parts or reads one back: its four fields are the child's two streams, the host's transcript sink and rule 8.4's response schema, and the associated function is the only way any of them is set. No `pub const` changes. **One substantive choice is recorded rather than left in a diff, and it is the implementation agent's under `WO-MOK-026`'s decision envelope, made on a measurement rather than a preference.** The alternative was to build the port in `mokiterions-core/src/main.rs`, where the streams already are, and admit nothing here at all; it was rejected because rule 20.4.1 lends one port for the whole run while `PortDecisionSource` is constructed per opportunity, so the accumulation of rule 14's cost lives in the port — and a port in the binary target would reach rules 14 and 15's arithmetic only by this specification publishing `accounting::RunAccount` together with the price and usage types it computes with, which is a larger surface than one type and one associated function and puts the cost arithmetic's own mutable state on the interface. **A finding is recorded and not repaired.** The 2026-08-24 row cites `ReplayPort`'s two callers as `mokiterions-core/src/main.rs:85` and `mokiterions-tui/src/main.rs:118`; at `d96cced`, the commit this work order branched from and so before any change of this branch's, the two constructions stand at lines 99 and 130. **Neither number holds and both had already decayed before this work order began.** That row is not edited — it records what it recorded — and the new admissibility row therefore names the function `run_live` rather than a line a later commit falsifies. **What this amendment does not do**: rule 4 is untouched and `execute` keeps five parameters with its three greps word for word, the port arriving through the `Option<&mut dyn Proposer>` parameter that already exists; the mutating-method grep still returns exactly `run` and `advance_tick`, measured at the candidate, `ConnectorPort::new` having no receiver and a trait method not being a `pub fn` declaration; `Simulation::advance_tick`'s signature stays one line; rule 6 is **not** amended and is re-checked instead, the ten prohibited names staying ten and staying private, the transcript sink being a referent the caller owns under that rule's own 2026-08-24 carve-out and the two streams being owned outright; rule 8's table is **not** amended, `tests/connector.rs` entering the public tier under that rule's standing clause admitting "a further file when a further public subject appears", the spawn, the environment inheritance and the reaping being that subject; and rule 13's declared dependency set is untouched and **still empty**, measured rather than assumed, because rule 10.1's "neither Rust package acquires a crate" is what makes the response line's reader a hand-written private module in `src/simulation.rs`. The frontmatter's `updated` field read `2026-08-24` while this record already carried a 2026-08-29 row written before this branch existed; it now reads `2026-08-29`. That is a corrected fact and not an amendment: no provision moves with it. | **Authorized 2026-08-29 by the repository owner acting as accountable engineering owner**, and the two acts are separate. The substance is `SPEC-MOK-007` rules 10.1 through 10.8 and 20.1 through 20.4.1, which are original content of that specification, approved 2026-08-23 by that owner by way of `ADR-MOK-007` and not amended here or by this work order. This specification decides nothing about the connector binding, the framing, the grammar or which host spawns; it records what the binding makes public and why that is admissible. The **authority to edit this file at all** is the second act: `SPEC-MOK-002.md` was admitted to `WO-MOK-026`'s `[execution_scope]` and `[relations].specifications` on 2026-08-29 by the same owner, over the declined alternative of a fourth governance work order scoped to this file alone, and that decision is recorded in full in that work order's own amendment record — including its measured cost, that the formal snapshot moves and that no evidence was yet bound to the snapshot it moves from. That admission is standing and this amendment needs no further scope act. The implementation agent measured every figure in this row and wrote the text; it decided only what the envelope reserves to it, disclosed above and in the source's own documentation. No record bound to a commit is re-opened: `VREC-MOK-003`, `VREC-MOK-010` and `VREC-MOK-012` bind earlier content of this specification and are not edited. |
| 2026-08-29 | **Rule 5's census records `simulation::ConnectorPort::new`'s grown signature: the run's declared unit prices and its ceiling are added as parameters, and the growth is nil.** `SPEC-MOK-007` rule 14.2 computes a live run's cost from the reported counts and the prices declared for the run, rule 14.6 stops the run once that cost reaches the declared ceiling, and rule 20.4.1 builds one port per run and lends it per tick — so the accumulation lives in the port, and the two figures it accumulates against are inputs the port cannot obtain for itself. Rule 14.3 forbids the alternative outright: prices are inputs of the run and never constants, so there is nothing for this item to read and nowhere to read it from. **Growth is nought items and nought public declarations**, both figures stated with their decomposition under the 2026-08-24 convention, and a growth table at the rule enumerates it: a signature changing on an item this rule already encloses is a change of shape, which is the `ActionTrace` form of the 2026-08-20 row and the form `simulation::Proposer`'s row of earlier this same day takes. Both parameter types are already enclosed — `simulation::UnitPrices` was admitted to the additions list earlier this same day for rule 14.3a's `--prices`, and `Option<u64>` is a primitive over a primitive — and **no field becomes public**, the item gaining a fifth field whose type, `accounting::RunAccount`, is private to a private module. **One substantive choice is recorded rather than left in a diff, and it is the implementation agent's under `WO-MOK-026`'s decision envelope, made on a measurement.** The ceiling crosses this interface in whole US cents and nothing finer, which is rule 14.2's stated minor unit, `--spend-ceiling`'s own parsed unit and the unit `simulation::Config`'s `spend_ceiling` already carries; the accumulation *behind* the parameter is finer, because one exchange at rule 14.3a's own example prices was measured at about 0.03 of a cent, so a cost accumulated in whole cents would add nought every exchange, rule 14.6's ceiling would never be reached and rule 15.2's cost would report `0` for a run that spent money. That finer unit is private to `src/simulation.rs` and is recorded there; **no unit conversion is asked of any caller**, which is the property the parameter's type is chosen for. **What this row does not do**: rule 4 is untouched and `execute` keeps five parameters with its three greps word for word, the prices reaching the library through `simulation::Config` and the port still arriving through the `Option<&mut dyn Proposer>` parameter that already exists; the mutating-method grep still returns exactly `run` and `advance_tick`, measured at the candidate, `ConnectorPort::new` still having no receiver; `Simulation::advance_tick`'s signature stays one line; rule 6 is **not** amended and is re-checked instead, the ten prohibited names staying ten and staying private, the fifth field holding four `u64` copies of the operator's own command line and six accumulators derived from figures a provider reported; rule 8's table is **not** amended and no file joins the public tier; and rule 13's declared dependency set is untouched and **still empty**, measured rather than assumed. The 2026-08-29 row above that admitted `ConnectorPort` is **not edited** — it records the growth it recorded — and the two places where rule 5 states that item's earlier shape carry a dated parenthetical pointing at this row rather than a rewritten claim, which is the form the 2026-08-24 return-type row already fixed. | **Authorized 2026-08-29 by the repository owner acting as accountable engineering owner**, and no new act of authority is needed for either half. The substance is `SPEC-MOK-007` rules 14.1 through 14.8 and 20.4.1, which are original content of that specification, approved 2026-08-23 by that owner by way of `ADR-MOK-007`, together with rule 14.2's minor unit and rule 14.3a's `--prices`, which that owner approved 2026-08-29 under `WO-MOK-030`. This specification decides nothing about the arithmetic, the unit or the ceiling; it records what the arithmetic makes visible on this interface, which is two parameters and no declaration. The **authority to edit this file** is the standing admission of `SPEC-MOK-002.md` to `WO-MOK-026`'s `[execution_scope]` and `[relations].specifications`, taken 2026-08-29 by the same owner and recorded in full in that work order's amendment record; the row above it rests on the same admission and no further scope act is required. The implementation agent measured every figure in this row and wrote the text; it decided only what the envelope reserves to it — the unit the accumulation is held in, and that the parameter's own unit is the specification's — disclosed above and in the source's own documentation. No record bound to a commit is re-opened: `VREC-MOK-003`, `VREC-MOK-010` and `VREC-MOK-012` bind earlier content of this specification and are not edited. |
| 2026-08-29 | **Rule 4 gains a fourth exit code, the first it has ever added, and rule 5's census gains the constant that names it and the port method that stops the run.** `SPEC-MOK-007` rule 14.6 stops a live run once its accumulated cost reaches the declared ceiling and rule 19.3 requires that stop to report "a status distinct from a clean completion and from an error", fixing no number. **Rule 4**: the signature does not move and keeps its five parameters with its three greps word for word; one code is added, `3`, and one diagnostic line on standard error, without the usage text. Why none of the three standing codes can carry it is recorded at the rule as three different reasons: `0` is the clean completion rule 19.3 names; `2` is an invalid configuration and the configuration was valid, the operator having declared the ceiling the run obeyed; and `1` is an output failure where nothing failed to write, which would be worse than imprecise, because `SPEC-MOK-006` rule 13.4 has the binary target remove a record sink it created when the run fails while rule 14.7 requires a ceiling-stopped run's streams to survive "complete and readable to the tick reached" — a `1` would instruct the host to delete the evidence the other rule preserves. **Rule 5**: the census gains `CEILING_STOP_EXIT`, a `u8` constant in `src/lib.rs`, and records `simulation::Proposer`'s third method, `halted`, taking `&self` and returning `bool` with a default body of `false`. **Growth is one item and one public declaration**, both figures stated with their decomposition under the 2026-08-24 convention: the constant is the one, and the trait method is nought on that convention's own wording — "its two trait methods not being `pub fn` declarations" — a method added to a trait this rule already encloses being a change of shape, the `ActionTrace` form of the 2026-08-20 row and the form this same trait's return took earlier the same day. **It is the first `pub const` any amendment to this rule has added**, the four amendments above each closing by recording that none changed. The three places where rule 5 states the trait's method count carry a dated parenthetical pointing at this amendment rather than a rewritten claim, which is the form the 2026-08-24 return-type row fixed. **The status is a named constant and not a literal** because it crosses a crate boundary — `src/main.rs` is a separate crate from `src/lib.rs` and rule 13.4's removal exception is the binary target's to apply — and the contrast is recorded because it is why this is one declaration and not two: `simulation::MISSING_DECISION_PORT` is shared between two *modules* of this crate, stays `pub(crate)`, and is not widened. The public-tier test asserts the literal `3` and not the constant, a test reading the constant agreeing with any value it took. **The *Observability* section is amended too**, for one sentence: it enumerated the exit codes as "exactly as verified" and would otherwise be false. The line it now records is a diagnostic this specification records rather than adds, rules 14.7 and 15.5 fixing what it may say; the event stream, the action-trace lines and the summary line are unchanged in form, and a ceiling-stopped run writes no summary line at all. **What this row does not do**: the mutating-method grep still returns exactly `run` and `advance_tick`, measured at the candidate, `halted` taking `&self` and being a trait method in any case; `Simulation::advance_tick`'s signature stays one line; the *Authorized additions* table is **not** edited, the amendment block's growth table being the enumeration, which is the form `simulation::Proposer` and `simulation::ConnectorPort` already take; rule 6 is **not** amended and is re-checked instead, the constant being a process status in the process-boundary module and not a simulation constant, `halted` returning a copy of a fact by value, and the ten prohibited names staying ten and staying private; rule 8's table is **not** amended and no file joins the public tier, `tests/connector.rs` having joined it earlier the same day; rule 13's declared dependency set is untouched and **still empty**, measured rather than assumed; and `SPEC-MOK-003` is **not** amended, the observer offering none of the three live options. | **Authorized 2026-08-29 by the repository owner acting as accountable engineering owner**, in one act, and the rest is standing authority. The substance of the stop is `SPEC-MOK-007` rules 14.6, 14.7, 14.8 and 19.3, original content of that specification approved 2026-08-23 by that owner by way of `ADR-MOK-007`. The substance of the *question* is that specification's rule 1.1b, which the same owner approved the same day under `WO-MOK-026`, over the measured alternative of a field on `Proposal` — nought items and one or two declarations, cheaper on this census and dearer everywhere else, having no ordering contract at all and putting "no exchange was issued" one field away from rule 9.5's "the exchange yielded nothing", which a reader who confused them would answer by writing a fallback record for an exchange that never happened. The **authority to edit this file** is the standing admission of `SPEC-MOK-002.md` to `WO-MOK-026`'s `[execution_scope]` and `[relations].specifications`, taken 2026-08-29 by the same owner and recorded in full in that work order's amendment record; the four rows above rest on the same admission and no further scope act is required. **Two things are the implementation agent's under that work order's decision envelope and are marked as such.** The number `3` is one: rule 19.3 fixes a status and no value, and three is the first value the standing three leave free. The second is that the status is stated as a public constant rather than a literal, which is what grows this rule's census by the one declaration above; it was forced by the crate boundary rather than preferred, and the alternative — a `3` written out in both crates — is a figure that can drift with nothing to catch it. Neither is a decision this envelope reserves to the owner: that list names the model identifier, the reasoning level, whether both gates are required, whether the ceiling check precedes the exchange, whether reported usage or an estimate is authoritative, the observer's options, a compiled-in connector default, and the live run's horizon, seed set, ceiling and existence. The implementation agent measured every figure in this row and wrote the text. No record bound to a commit is re-opened: `VREC-MOK-003`, `VREC-MOK-010` and `VREC-MOK-012` bind earlier content of this specification and are not edited. |

## Scope

This is the structural contract for the **simulation engine package**: which targets it builds, exactly which items
its library target makes public, and where every automated test of that package lives.

**Paths, as amended 2026-08-18 for `REQ-MOK-030`.** Every path this specification writes relative to the repository
root — `Cargo.toml`, `src/lib.rs`, `src/main.rs`, `src/cli.rs`, `src/simulation.rs`, `tests/` and each file under it,
and the `grep` check in rule 5 — is read relative to `mokiterions-core/`, the engine package's own directory. The
paths in the manifest are unchanged, because a package manifest's paths were always relative to the manifest. The
paths that move are the ones this document writes as though the package were the whole repository, and they move by
prefix alone: no file is renamed, no rule changes what it requires, and no target, target name, target kind or
package name changes.
Rule 1's claim that this package is at the root "unchanged in source location" is corrected by this amendment; the
root now holds a workspace manifest and no package's sources. Where a rule's substance depends on the correction it
says so at the rule.

**Boundary, as amended 2026-08-18.** Every rule below is a rule about the engine package. It was written when the
repository contained only that package, so the scoping narrows nothing: each rule previously bound a repository that
was exactly this package. What the amendment removes is the implication that no other package may exist, an
implication rule 1 already qualified by reserving an exception for an approved requirement. `SPEC-MOK-003` is the
structural and behavioral contract for the terminal observer package, and this specification states nothing about it.

It states no simulation behavior. `SPEC-MOK-001` remains the single behavior contract, and rule 11 below binds this
specification to preserving it byte-for-byte. Where the two touch — `SPEC-MOK-001` currently delegates "test
organization and helper functions" to the implementation agent — that delegation is narrowed by rules 7 to 10 and
by nothing else; see *Compatibility and migration*.

**The record stream, as amended 2026-08-20 for `REQ-MOK-042` through `REQ-MOK-046`.** `SPEC-MOK-006` is the contract
for the machine-readable record stream: its framing, its record kinds, its fields, their order, their value alphabet,
its failure behavior and its non-perturbation obligation. This specification states none of that. What it states about
the stream is structural and is confined to two places: rule 4, which fixes the one parameter by which a sink reaches
the engine, and rule 5, whose enumeration the parameter does not grow because a parameter is not an item. Rules 1 to 3
are unchanged — no target, target name, target kind or package name changes, and the engine package's dependency table
stays empty, which is what makes `SPEC-MOK-006` rule 3's closed value alphabet load-bearing rather than a convenience.
Where the two specifications touch, `SPEC-MOK-006` is the authority on what a record contains and this one on what the
interface looks like; if they ever disagree about the interface, this one governs, and the disagreement is a defect to
be recorded here rather than resolved silently.

## Actors and external systems

- The Rust toolchain and Cargo, which decide what a target is and what an integration test may link.
- Clippy under `-D warnings`, whose `non_snake_case` lint constrains the library target's name.
- Implementation agents and developers, who place tests and maintain the public interface.
- No external service, network endpoint, credential, or filesystem location participates. Amended 2026-08-24 for
  `REQ-MOK-063`, by way of `ADR-MOK-007`: **this bullet takes the same target scope the first bullet of *Security and
  privacy properties* now takes, and for the same reason.** Of the **library target** it stands word for word: no
  external service, no network endpoint, no credential and no filesystem location participates, and the port reaches
  it as a borrowed trait object with the transcript already open behind it. Of the **binary target** a filesystem
  location does participate, in two ways that are worth distinguishing: since 2026-08-20 as the record stream's
  destination, which nothing reads back, and — added at this date — as **a source the run's decisions come from**,
  which is the stronger case, because the transcript's bytes decide what the Mokiterions propose. No external service,
  no network endpoint and no credential participates in either target at this candidate.

**Added 2026-08-24 for `REQ-MOK-063`: the connector is not here yet, and this section does not pretend otherwise.**
`ADR-MOK-007` requires this section to gain the connector as a process the binary target starts and the library never
sees. **No commit in `WO-MOK-025` makes that true**: its *Out of scope* excludes the connector, its protocol
implementation, the canned connector and any process spawn, so the only decision source outside the engine that exists
here is a replay of a transcript, which starts nothing. The connector's bullet is `WO-MOK-026`'s to write under the
same authorization, and the form above is the form it takes — an actor of the binary target and of no other. The
precedent for landing an authorized amendment in the part that is true of the tree is this specification's own
2026-08-23 row, whose *"What this row does not do"* paragraph declined to write these two sections for exactly this
reason; what that paragraph got wrong was only which work order would make them true.

## Inputs

`Cargo.toml` target declarations; the source files `src/lib.rs`, `src/main.rs`, `src/cli.rs`, and
`src/simulation.rs`; the test files under `tests/`; the commands `cargo build`, `cargo test`, `cargo fmt`, and
`cargo clippy --all-targets --all-features -- -D warnings`.

Amended 2026-08-18: since rule 1 now admits a workspace, the engine-only form of each command is the one that answers
a question about this package — `cargo build -p Mokiterions`, `cargo test -p Mokiterions`, `cargo tree -p Mokiterions`.
The workspace-wide forms are also run, and the lint gate is not relaxed for either package.

## Outputs

One library artifact and one executable artifact; one test binary per internal-tier target and one per public-tier
file; the observable program output, which this specification requires to be unchanged.

## State model

This specification governs no runtime state. Its subject is the compile-time arrangement of the crate, which has
two states: conformant, when every rule below holds, and non-conformant, when any does not. There is no partial or
transitional state — a build either satisfies the rules or fails a conformance check.

## Behavioral rules

### 1. Targets

The package name stays `Mokiterions`. It declares exactly two targets:

| Target | Kind | Name | Path |
|---|---|---|---|
| Library | `[lib]` | `mokiterions` | `src/lib.rs` |
| Binary | `[[bin]]` | `Mokiterions` | `src/main.rs` |

No third target and no build script.

The dependency and dev-dependency tables contain exactly the entries rule 13, *Declared dependency set*, declares for
this package, at the versions and feature sets declared there, and nothing else. Dev-dependencies are
declared in the same way and are **not** exempt: a test-only crate that draws entropy can make a test flake in a
repository whose figures are replay hashes. A dependency shared with another package in the same workspace is
admissible only as a declared entry of both packages' sets; `SPEC-MOK-004` rule 1 governs where its version may be
keyed.

**Amended 2026-08-20.** The two sentences above replace *"The dependency and dev-dependency tables stay empty, with no
exception, including a dependency shared with another package in the same workspace."* `ADR-MOK-006` withdrew the
empty-table rule and admits third-party crates in both packages against a declared set. **The declared set for this
package is empty as this amendment lands**, so nothing about the manifest changes on the day of the amendment — what
changes is that an empty table is a fact about the current declaration rather than a rule. *"No third target and no
build script"* is **unchanged**: a build script is a code-execution surface in **this repository's own package**, and
this decision does not open it. `ADR-MOK-006` decision 13 concerns a *dependency's* build script, which this clause
never spoke to, and requires each declared entry to disclose whether it carries one.

**Amended 2026-08-18.** This rule read "No third target, no second package, no workspace, no build script." The
repository is a Cargo workspace of exactly two packages: this one, unchanged in package name and in both target
names, and the terminal observer `mokiterions-tui` as its only other member. `REQ-MOK-026` is the approved
requirement that this rule and `ARCH-MOK-001`'s prohibited-pattern list both reserved the exception for;
`ADR-MOK-003` decides the split and `SPEC-MOK-003` governs the observer package. No third package, no service, no
network boundary and no separate release artifact is admitted. Every other clause of this rule is unchanged, and the
dependency table is the check that the split cost this package nothing: `cargo tree -p Mokiterions -e normal --locked
--offline` resolves to exactly the declared set plus this package itself — which is one crate while that set is empty —
and it resolves from the committed lockfile with no registry access. *(The sentence read "the empty dependency table …
resolves to one crate" until the 2026-08-20 amendment, which replaced the count with the comparison and added the
offline resolution; the figure it asserted is unchanged today, and it is now derived from the declaration rather than
fixed by a rule.)*

That amendment also described this package as "at the root, unchanged in package name, in both target names and in
source location". The location half is superseded by the second amendment of the same date: this package's manifest,
sources and tests are under `mokiterions-core/`, and the two target paths in the table above are relative to that
manifest, as a manifest's paths always were. `REQ-MOK-030` is the approved requirement, `SPEC-MOK-004` rules 1 to 3
fix the layout, and `ADR-MOK-004` decides it. Both target names, the package name and the target kinds are unchanged
by that move, which is what keeps the rest of this rule intact.

### 2. Target names

The library target is named `mokiterions` rather than inheriting the package name. The declared lint gate is
`cargo clippy --all-targets --all-features -- -D warnings`, which implies `non_snake_case`; a library crate named
`Mokiterions` fails it with `crate 'Mokiterions' should have a snake case name`, and the gate is not to be relaxed.

The binary target keeps the name `Mokiterions`. It is the operator-facing command, it appears in the first line of
`USAGE`, and `REQ-MOK-010`'s observable interface includes that text. The two names differ deliberately, and this
rule is the reason.

### 3. Module ownership

`src/lib.rs` declares `pub mod cli;` and `pub mod simulation;` and defines the process-boundary function of rule 4.
It contains no simulation logic and no test.

`src/main.rs` contains only: the use of the library target, `fn main`, standard-output and standard-error locking
and buffering, one call to the process-boundary function, flush handling that maps a failed flush to exit code `1`,
and the conversion of the returned code to a process exit code. It declares no module and contains no test.

`src/cli.rs` and `src/simulation.rs` keep their contents through the restructuring this specification governs, apart
from the visibility changes rule 5 authorizes and the test relocations rules 7 to 9 require.

**Amended 2026-08-18.** This paragraph read "keep their current contents apart from…", which read as a standing rule
forbids ever adding code to either file and so would freeze the engine. It is scoped to the restructuring that
`WO-MOK-003` performed, which is the change rule 11 requires to be equivalence-preserving. An approved requirement may
add code to `src/simulation.rs`; `REQ-MOK-019` through `REQ-MOK-027` are the first to do so, adding the observation
surface rule 5 admits. Rule 11 still binds: whatever is added, the program's observable output, final state and exit
codes for identical inputs do not change. Rules 5 and 6 still bind what any addition may make public.

### 4. Process-boundary function

`pub fn execute<I, S, W, E>(args: I, stdout: &mut W, stderr: &mut E) -> u8` moves from `src/main.rs` to
`src/lib.rs`. Its signature, its behavior, its diagnostic text, and its exit codes are unchanged: `0` on success or
help, `1` on output failure, `2` on invalid configuration, with the usage text written to standard error on
invalid configuration. It becomes public because rule 7 places the exit-code tests in the public tier.

**Amended 2026-08-20 for `REQ-MOK-042`, by way of `ADR-MOK-005`.** The signature is

```rust
pub fn execute<I, S, W, E>(
    args: I,
    stdout: &mut W,
    stderr: &mut E,
    records: Option<&mut dyn Write>,
) -> u8
```

one parameter more than before and nothing else. The exit codes are unchanged and none is added: a failure to write,
flush or close the record sink is an output failure and is therefore `1`, which `SPEC-MOK-006` rule 13.6 states as a
rule and this one restates only so that the enumerated interface and the exit-code contract agree.

The parameter is `Option<&mut dyn Write>` rather than a fifth generic bounded by `Write`, so that a caller with no sink
passes `None` and needs no type annotation for a writer it does not have. That choice is the implementation agent's
under `WO-MOK-019`'s decision envelope; what this rule fixes is that there is exactly one new parameter, that it is
optional, and that it is a sink the caller owns. `execute` does not resolve it, open it, create it or remove it —
`SPEC-MOK-006` rule 1.2 places every filesystem operation in the binary target, and rule 3 of this specification keeps
`src/lib.rs` the process boundary and nothing more.

Records are written when, and only when, this parameter is `Some`. The option `--events-path` is what makes the binary
target supply one, so `SPEC-MOK-006` rule 1.1's "when, and only when, `--events-path` is given" is a property of the
product; within the library the parameter is the whole of the condition. Stated because the two are easy to conflate: a
caller that passes the option and no sink gets no records, and that is the caller's own arrangement rather than a
defect.

`cli::parse` learns `--events-path` and validates it — at most once, a value required, the empty string and the single
character `-` rejected — and retains nothing. The value is the destination, the destination is the binary target's
under `ADR-MOK-005`, and the library has no use for a path it may not interpret. `cli::Command` and
`simulation::Config` are therefore unchanged, which is what keeps this amendment to one parameter and no item.

**Amended 2026-08-23 for `REQ-MOK-063`, by way of `ADR-MOK-007`.** The signature is

```rust
pub fn execute<I, S, W, E>(
    args: I,
    stdout: &mut W,
    stderr: &mut E,
    records: Option<&mut dyn Write>,
    port: Option<&mut dyn Proposer>,
) -> u8
```

one parameter more than the 2026-08-20 form and nothing else. `SPEC-MOK-007` rule 20.5 names this as one of the two
doors the port reaches, and rule 20.4 fixes its shape: borrowed, optional, and built and owned by the caller. That is
the shape the record sink established three months of artifacts ago rather than a new convention, which is why
`ADR-MOK-007` treats it as one parameter added and not as an interface replaced.

The exit codes are unchanged and none is added. Rule 20.8's refusal — the `llm` source selected with no port supplied —
is an **invalid configuration** and is therefore `2`, the code this rule already fixes for one. It is not an output
failure and not a new code. The refusal is reported before a `Simulation` is constructed, so it reads like every other
configuration rejection; unlike them it is not followed by the usage text, for the reason `Simulation::new`'s own
failures are not.

Proposals are obtained through this parameter when, and only when, it is `Some` **and** the selected policy is the one
that uses it. The four existing sources ignore a port exactly as an absent sink is ignored, per `SPEC-MOK-007` rule
20.9, and that is not an error. Where the sink and the port part company is the other direction: a missing sink is
silently no records, whereas a missing port under the one source that needs it is refused rather than substituted.
`SPEC-MOK-007` rule 9.7 forbids borrowing another source's selection, so there is no run left to fall back to — a run
that proceeded would be a run of a different source reported under this one's name.

`execute` does not build the port, does not close it, and cannot know what is behind it. `SPEC-MOK-007` rules 10.5 and
13.4 place the provider credential in a component this target does not contain; `cli::parse` gains no option naming a
port and `simulation::Config` gains no field. Every item this change adds to rule 5's lists is enumerated by rule 5's
own 2026-08-23 amendment and by nothing here, which is what keeps this rule's amendment to one parameter.

**Amended 2026-08-29 for `SPEC-MOK-007` rule 19.3's ceiling stop, under `WO-MOK-026`.** The signature does not move: five
parameters in the order the two amendments above fixed, and the three greps rule 5's checks compare against stand word
for word. **One exit code is added, and it is the first this rule has added.** A run that reached its declared spend
ceiling and stopped before the next exchange exits `3`, which the library states as the public constant
`CEILING_STOP_EXIT` rather than as a literal.

Rule 19.3 is what requires a fourth: a ceiling stop must report "a status distinct from a clean completion and from an
error", and that rule fixes no number. None of the three already here can carry it, and the reasons are three different
reasons rather than one.

- `0` is the clean completion rule 19.3 names, so a ceiling stop reported as `0` is the one thing that rule forbids
  outright.
- `1` is an output failure, and nothing failed to write. Reporting one here would be worse than imprecise:
  `SPEC-MOK-006` rule 13.4 has the binary target remove a record sink it created when the run fails, while rule 14.7 of
  `SPEC-MOK-007` requires a ceiling-stopped run's streams to survive "complete and readable to the tick reached" — so a
  `1` would instruct the host to delete the evidence the other rule preserves.
- `2` is an invalid configuration, and the configuration was valid. The operator declared a ceiling and the run obeyed
  it; a configuration code would report the operator's own instruction back to them as their mistake.

`3` is the value, and it is the implementation agent's under `WO-MOK-026`'s decision envelope rather than any artifact's:
rule 19.3 fixes a status and no number, and three is the first number the three above leave free. It is a named constant
because it crosses a crate boundary — `src/main.rs` is a separate crate from `src/lib.rs` and has to act on this value to
satisfy rule 13.4's exception, so a `3` written out in both crates is a `3` that can drift. The other three stay literals
inside `execute`, where no host acts differently on any of them. The constant is enumerated as an item by rule 5's
amendment of the same date and is **not** counted a second time here, which is the division of labour the 2026-08-20 row
fixed in the other direction for `execute`'s parameter.

**One diagnostic line is added, and the usage text is not.** The stop is reported to standard error, naming the tick the
run reached and no figure. Standard error and not standard output, because rule 14.7 requires the text stream complete and
readable *to the tick reached* and a line after the last tick's events is a line no replay of that stream produces; and no
figure, because rule 15.5 puts the ceiling and the accumulated cost in the run record, where a reader has the seed, the
horizon and the token totals beside them. The usage text does not follow it, on `Simulation::new`'s precedent rather than
the argument parser's and for that precedent's reason: the operator's command line was well formed.

### 5. Authorized public interface

The library target's public interface is exactly the union of the three lists below.

**Already public, and unchanged.**

| Item | Form |
|---|---|
| `cli::USAGE` | `&'static str` constant |
| `cli::Command` | enum with variants `Help` and `Run(Config)` |
| `cli::parse` | function returning `Result<Command, String>` |
| `simulation::Config` | struct with public fields `seed`, `tick_limit`, `policy`, `density`, `trace_actions`, `spend_ceiling`, `prices` |
| `simulation::Density` | value type with associated constant `DEFAULT` and function `parse` |
| `simulation::Policy` | enum with variants `Baseline`, `Reference`, `Individual`, `Social` and `Llm`, with `parse` and `Default` |
| `simulation::RunSummary` | opaque value type; its fields stay private |
| `simulation::Simulation::new` | `Config` in, `Result<Simulation, String>` out |
| `simulation::Simulation::run` | `&mut self` and a writer in, `io::Result<RunSummary>` out |

**Authorized additions.**

| Item | Form | Why it is admissible |
|---|---|---|
| `execute` | rule 4 | Maps arguments and the caller's writers to an exit code; owns no state |
| `simulation::TerminationReason` | enum with variants `TickLimit` and `Extinction`, with `Display` | A reported outcome; required for a public `RunSummary` accessor |
| `RunSummary` accessors | each returns a copy: the termination reason, the tick count, survivors, deaths, population per territory, and resource counts per territory by calorie class | Every value is already printed in the summary line |
| `Density::resources_per_territory` | `self` in, `usize` out | A pure function of a value; the resolved count is already reported |
| `simulation::CELLS_PER_TERRITORY` | `usize` constant | A fixed world dimension, already implied by `SPEC-MOK-001` |
| `simulation::UnitPrices` | **added** *(2026-08-29)*: value type with four public `u64` fields — `prompt`, `cached`, `output`, `reasoning` — and associated function `parse`; `Density`'s derive set item for item, and no `Default` | Four copies of the operator's own input, admissible on `Density`'s grounds: a value with a parser and no interior state, granting no path into engine-owned state. It is public at all because `Config`'s field is and `Config` is already public. **The four fields are public where `Density`'s one is private**, which is the one way this item is not `Density`, and the reason is the hazard the named type exists for: three of the four prices are plausible values for each other's position, so only a public-tier test that reads the fields separately can establish that a transposed `--prices` is not silently accepted — comparing one `parse` against another proves nothing about order. Public-tier tests are its only callers outside the crate, the arithmetic of `SPEC-MOK-007` rules 14.1, 14.2 and 14.6 being inside it; that is the `DecisionRequest::tick` ground of this table's 2026-08-24 row. It carries no `Default` because rule 14.3 forbids a compiled-in price |

**The additions list is a ceiling, not a checklist.** An item on it that no relocated test requires must not be
added. Nothing outside the three lists becomes public. `Simulation` itself is reachable, but it exposes no public
field and no method beyond `new`, `run` and the observation surface of the third list.

**Observation surface, added 2026-08-18 under `REQ-MOK-019` through `REQ-MOK-027`.**

`SPEC-MOK-003`'s *Data and interface contracts* section is the authority for the content, ordering and ownership of
every item below; this list is the enumeration that closes the interface, and the two must agree.

| Item | Form | Why it is admissible |
|---|---|---|
| `Simulation::snapshot` | `&self` in, `WorldSnapshot` out | Returns an owned tree of copies; borrows nothing and mutates nothing |
| `Simulation::advance_tick` | `&mut self` and an optional borrowed port in, `Result<TickOutcome, String>` out | The single mutating operation; refuses a finished run and consumes entropy exactly as `run` does, because both route through one internal step. The parameter is rule 5's 2026-08-23 amendment; a host with no port passes `None` |
| `Simulation::is_finished` | `&self` in, `bool` out | A copy of a termination fact the summary line already reports |
| `Simulation::termination_reason` | `&self` in, `Option<TerminationReason>` out | Same fact, already public as a `RunSummary` accessor |
| `Simulation::configuration` | `&self` in, `Config` out | A copy of the operator's own input; `Config` is already public |
| `Simulation::initialization_events` | `&self` in, `Vec<Event>` out | An owned copy of events the text stream already emits before tick 1 |
| `simulation::WorldSnapshot` | struct of owned fields, with `[TerritorySnapshot; 2]` and `Vec` of the three snapshot types | The observed state of one completed tick |
| `simulation::TerritorySnapshot` | struct of `Territory` and `usize`/`bool` counts | Every field is already printed in the summary line or derivable from it |
| `simulation::AgentSnapshot` | struct of `String`, `Coordinate`, `Territory`, four `u8` attributes and `Option<Action>` | Every field is already printed in the event stream |
| `simulation::ResourceSnapshot` | struct of `String`, `Coordinate`, `Territory`, `FoodClass` | Every field is already printed in the event stream |
| `simulation::DecisionSnapshot` | struct of `String`, `Action`, `DecisionOutcome`, `Option<Action>` | Exactly what a `--trace-actions` line already prints |
| `simulation::TickOutcome` | struct of `Vec<Event>`, `bool` and `Option<TerminationReason>` | What one tick emitted, owned |
| `simulation::Event` | struct with public `tick`, `subject`, `detail`, and `event_type` | One already-emitted line as a value; not the event log, and no path to it |
| `simulation::EventDetail` | enum with `event_type` | The per-event payload the text stream already formats |
| `simulation::EventType` | enum with `ALL: [Self; 15]` and `as_str` | The closed set of emitted kinds, so a host can filter by kind without parsing text |
| `simulation::DecisionOutcome` | enum carrying a rejection ground | Already printed on a trace line |
| `simulation::RegenerationSkipReason` | enum | Already printed in a regeneration event |
| `simulation::Coordinate` | struct with public `x` and `y`, both `u8` | A position already printed in the event stream; two bytes by value |
| `simulation::Territory` | enum | Already printed in the event stream and the summary line |
| `simulation::Direction` | enum | Reachable only inside `Action`, whose values a trace line already prints |
| `simulation::FoodClass` | enum | The calorie class already printed in the summary line |
| `simulation::Action` | enum | The proposed and applied action a trace line already prints |

Every item is a value, a pure function of a value, or an accessor returning a copy, exactly as `ADR-MOK-002` requires
of an admission. Five of the type names — `Coordinate`, `Territory`, `Direction`, `FoodClass` and `Action` — are named
by rule 6 as it was written; rule 6's 2026-08-18 amendment is what admits them, and it admits them as values only.

**Amended 2026-08-20 under `CAP-MOK-010`, and the growth is enumerated item by item so that it can be checked rather
than described.** Four items on the lists above change shape and no item is added or removed:

| Item | Growth | Count |
|---|---|---|
| `simulation::Policy` | one variant, `Social`. `Default` is unchanged and still resolves to `Reference` | 1 |
| `simulation::Action` | seven variants — `Attack`, `Threaten`, `Fight`, `Retreat`, `Surrender`, `Approach`, `Avoid` — each carrying one target field holding an agent identifier | 7 |
| `simulation::EventType` | three variants — `AttackResolved`, `ThreatResolved`, `SurrenderResolved` — so `ALL` goes from twelve entries to fifteen, together with the three `EventDetail` payloads that carry their fixed field lists | 3 + 3 |
| `simulation::EventDetail`, the **existing** `ActionTrace` variant | one field, `suffered: Vec<(String, u8)>`, appended after `fear` | 1 |

**The fourth row was added 2026-08-20, after the first three had been approved, and its absence was a defect in this
enumeration rather than in the implementation.** The row is here because this list is closed and checkable: three of the
four items are new variants, and the fourth is a **field appended to a public variant that already existed**, which is
the one form of growth an enumeration of added variants does not catch. `SPEC-MOK-001` rule 7 obliges the trace line to
report the suffered-attack record, and rule 6 of that specification fixes the line's shape; a public payload cannot
carry a field the interface authority has not enumerated, whatever obliges the line to print it.

Two properties of the field are part of the row rather than incidental to it. It is a `Vec` of **pairs of a `String` and
a `u8`** and not of the engine's own `SufferedAttack`, so no type is added to the interface and rule 6's ten private
names are untouched — both halves of a pair are already public values, an identifier being what `AgentSnapshot` carries
and a damage being what `AttackResolved` carries. And it is a growth of one field on one variant, so the shape of every
other `EventDetail` variant is unchanged; a host matching on `ActionTrace` with named fields and a `..` rest pattern is
unaffected, while one matching it exhaustively by field must add the name.

**What this correction does not do is relax the accounting it belongs to.** Public interface growth under
`CAP-MOK-010` is `1 + 7 + 3 + 3 + 1`, and `EventType::ALL`'s length moving from `12` to `15` remains the only change to
a `pub const`. `VER-MOK-016`'s interface-growth check compares the engine's public surface item for item against this
table, so the check is re-run against four rows rather than three.

**What does not grow is part of the enumeration.** `AgentSnapshot` still carries a `String`, a `Coordinate`, a
`Territory`, four `u8` attributes and an `Option<Action>`: no fifth attribute exists, the suffered-attack record is not
an attribute, and no approved requirement needs the observer to render it. `DecisionSnapshot` is unchanged in shape and
carries the seven new verbs through the `Action` it already holds. `WorldSnapshot`, `TerritorySnapshot`,
`ResourceSnapshot`, `TickOutcome`, `Event`, `DecisionOutcome`, `RegenerationSkipReason`, `Coordinate`, `Territory`,
`Direction` and `FoodClass` are untouched, as are all nine items of the first list beyond `Policy` and all five
authorized additions. No accessor is added, no method is added, and the two `&mut self` methods stay exactly two, so
rule 5's `grep` check is unchanged.

**The observation's two new fields are not interface growth, and the distinction is load-bearing.** The observer's own
`fear` and its suffered-attack record are fields on `Observation`, which rule 6 keeps private and names among the ten
types that stay private. They are counted in this amendment's accounting because `REQ-MOK-054` obliges that they be, not
because anything about them becomes public: a host cannot construct an `Observation`, cannot receive one, and cannot
read either field. The same is true of the fourth decision source itself, which is an implementation of the private
`DecisionSource` trait.

**Mutating methods on the interface: exactly two, and both are simulation steps.** `advance_tick`, added by this list,
and `Simulation::run`, already in the first list, are the only `pub fn` items in the library target taking `&mut self`.
`grep -n 'pub fn .*&mut self' src/simulation.rs` returning exactly those two is the check. `run` predates this list —
it is the `REQ-MOK-010` whole-run entry point — and it is on the interface because rule 3 places `simulation` in the
library target, not because this list admits it. Both route through one internal step, so the two hosts execute the
identical tick sequence, and the observer calls only `advance_tick`. Nothing else on the interface takes `&mut self`,
and no `&self` method mutates through interior mutability, because no engine type contains a `Cell`, a `RefCell`, an
`Rc`, an `Arc`, a lock or an atomic.

**Amended 2026-08-20 for `REQ-MOK-042`, by way of `ADR-MOK-005`. The checks, restated so that they still detect drift
after the record projection exists.**

`execute`'s signature is enumerated by rule 4 and by nothing else, so rule 4's literal is the reference the check
compares against. Because the signature now spans several lines, the mechanical form is two greps rather than one:
`grep -n 'pub fn execute' src/lib.rs` returning exactly one line, and `grep -n 'records: Option<&mut dyn Write>'
src/lib.rs` returning exactly one line. A fifth parameter, a second sink, or a sink that is not optional fails the
second; a second public process-boundary function fails the first.

`grep -n 'pub fn .*&mut self' src/simulation.rs` still returns exactly `run` and `advance_tick`. The record projection
needs the sink carried down the same call chain the text stream travels, and the carrier that takes it is
`pub(crate) fn run_recording`, which the pattern does not match because `pub(crate) fn` is not `pub fn`. That is a fact
about the check, not a way around it: `run_recording` is crate-private, is not on the interface, and is not reachable
from any item that is, so the interface still has exactly two mutating methods and both are still simulation steps.
`Simulation::run`'s enumerated form — `&mut self` and a writer in, `io::Result<RunSummary>` out — is unchanged, and it
delegates to `run_recording` with no sink.

**Amended 2026-08-23 for `REQ-MOK-063`, by way of `ADR-MOK-007`, and the growth is again enumerated item by item so that
it can be checked rather than described.** Two items on the lists above change shape, two items are added, and nothing is
removed:

| Item | Growth | Count |
|---|---|---|
| `simulation::Policy` | one variant, `Llm`, last. `Default` is unchanged and still resolves to `Reference`, so no existing caller's behaviour moves | 1 |
| `simulation::Simulation::advance_tick` | one parameter, `Option<&mut dyn Proposer>`, appended. A caller that passes `None` is the caller that exists today | 1 |
| `simulation::Proposer` | **added**: trait with one method taking a request by value and returning `Option<Action>`. *(The return became `Proposal` on 2026-08-29 under `SPEC-MOK-007` rule 1.1a; this row records the 2026-08-23 growth and is not edited for it, the later growth being enumerated in its own table below.)* | 1 |
| `simulation::DecisionRequest` | **added**: opaque value type of four owned or `'static` string parts, with per-part accessors returning `&str`, one accessor returning them in the composition order, and two accessors naming the opportunity the request is for — `tick` returning `u64` and `actor_id` returning `&str` | 1 |
| `simulation::ReplayPort` | **added** *(2026-08-24)*: generic struct over `BufRead` implementing `Proposer` from a retained transcript, with one associated function, `new`, taking the reader by value. It is the port a replay host hands the engine | 1 |

`execute`'s fifth parameter is rule 4's amendment of the same date and is not counted a second time here. Interface
growth under `REQ-MOK-063` is therefore `1 + 1 + 1 + 1 + 1`, and no `pub const` changes.

**The item count and the public-item count are different measurements, and both are stated so that neither is mistaken
for the other.** The five rows above are five *items*, this rule's convention being that a type and its accessors are one.
Counted as public declarations instead — which is what a `pub` census returns — the same growth is **twelve**, and the
decomposition is given so a later check need not guess it: `DecisionRequest` and its **seven** accessors (`actor`,
`actor_id`, `blocks`, `observation`, `permitted_set`, `shared_rules`, `tick`) is eight; `Proposer` is one, its two trait
methods not being `pub fn` declarations *(three from 2026-08-29, when the amendment below adds `halted`; the count of
declarations is unmoved for this same reason)*; `ReplayPort` and `ReplayPort::new` are two; and `Simulation::advance_tick` is
one. `Policy` gains a variant and not a declaration, so it does not appear in the twelve. A check comparing a census
against this table must expand the rows first, and `WO-MOK-025`'s `candidate/static-checks.txt` check 6 is that check.

| Item | Form | Why it is admissible |
|---|---|---|
| `simulation::Proposer` | `&mut self` and a `DecisionRequest` in, `Proposal` out *(`Option<Action>` until 2026-08-29)*; and, as amended 2026-08-29 below, a `&self` question answered `false` unless a port overrides it | `SPEC-MOK-007` rule 1.1: the engine's one means of obtaining a proposal from outside itself. It must be public because rule 20.4 puts the implementation in a host, and it names no provider, no transport, no model, no credential, no file and no mode |
| `simulation::DecisionRequest` | struct of four string parts — the shared rules, the actor block, the observation block, the permitted set — carried by value | `SPEC-MOK-007` rule 1.3: what crosses is a copy. It holds no reference into engine state, no mutable borrow and no handle, so an implementation cannot reach what it was told about |
| `simulation::ReplayPort` | generic over `BufRead`, constructed from the caller's own reader, implementing `Proposer` | `SPEC-MOK-007` rule 12.1.1 puts the opening of the transcript in the host, and `ARCH-MOK-002` puts its parsing in the engine package by name. **Two crates therefore construct it** — `mokiterions-core/src/main.rs:85` and `mokiterions-tui/src/main.rs:118` — so it cannot be narrowed to `pub(crate)` without moving the parsing out of the engine or the opening into it, and each would contradict one of those two artifacts. Its type parameter is the caller's reader, so it holds nothing of the engine's |

**Both additions are values or a function of a value, as `ADR-MOK-002` requires of an admission.** `DecisionRequest`
carries strings the engine composed from one `Observation`; `Proposer` carries no state of the engine's at all. Rule 6 is
untouched by either: `Observation` and `DecisionSource` stay private and stay named there, per `SPEC-MOK-007` rule 20.6,
so a host implementing the port sees a rendered request and never the observation behind it, and cannot implement the
engine's internal source abstraction.

**`Proposer`'s spelling is this specification's to record and was nobody's to fix.** `SPEC-MOK-007` rule 1.1 fixes the
interface's shape and no approved artifact names the identifier. It is recorded here because the mechanical check below
depends on it: the name is short enough that `advance_tick`'s amended signature fits the formatter's line limit and stays
on one line. `DecisionPort`, the artifacts' own words for the concept, reaches 109 columns in that signature and 104 with
the shortest sensible parameter name. The word *port* is kept in the parameter name, in the source's documentation and in
the refusal's message constant, where it costs no width.

**Mutating methods on the interface: still exactly two, and both are still simulation steps.** `advance_tick` gains a
parameter and does not stop being one door; `run` is not amended at all — it delegates with the port absent, so its
enumerated form in the first list stands unchanged for the second time. `run_recording` takes the port down the call
chain as it already takes the sink, and is still crate-private, still not on the interface, and still not reachable from
any item that is. No accessor is added, no method is added, and no `&self` method mutates through interior mutability,
because no engine type contains a `Cell`, a `RefCell`, an `Rc`, an `Arc`, a lock or an atomic.

**The checks, restated so that they still detect drift after the port exists.** The 2026-08-20 restatement above reads "a
fifth parameter … fails the second", and the port on `execute` **is** that fifth parameter, so leaving that sentence
standing would make a conforming build fail its own specification. The mechanical form for `execute` is now three greps,
each returning exactly one line: `grep -n 'pub fn execute' src/lib.rs`, `grep -n 'records: Option<&mut dyn Write>'
src/lib.rs`, and `grep -n 'port: Option<&mut dyn Proposer>' src/lib.rs`. A sixth parameter, a second sink, a second port,
a sink that is not optional or a port that is not optional fails one of the last two; a second public process-boundary
function fails the first. Rule 4's literal remains the reference all three compare against.

`grep -n 'pub fn .*&mut self' src/simulation.rs` **still returns exactly `run` and `advance_tick`, and this rule now
depends on how those two lines are formatted.** A signature the formatter wraps puts the declaration keyword and the
receiver on different lines, and this check matches neither line: it would then report one door where there are two and
pass while doing so, which is a weakened check rather than a failing one. So the check has a second obligation attached
to it — `advance_tick`'s signature is one line in the source, and it is the reason `Proposer` is named as it is above. A
future parameter on either method that cannot be added within the line limit must change this check's form in the same
commit, exactly as this amendment changes `execute`'s.

The pattern must also not appear in prose in that file. It matched a documentation comment during this stage's
implementation, which is a third way the check reports the wrong number, and the comment was reworded rather than the
check loosened.

**Amended 2026-08-29 for `SPEC-MOK-007` rule 14.3a's declared unit prices, and the growth is enumerated as the two
amendments above enumerate theirs.** One item is added, one already-public struct gains one field, and nothing is
removed:

| Item | Growth | Count |
|---|---|---|
| `simulation::UnitPrices` | **added**: value type of four `u64` fields with one associated function, `parse`. `Density`'s shape, `Density`'s derive set, and no `Default` | 1 |
| `simulation::Config` | one field, `prices: Option<UnitPrices>`, appended after `spend_ceiling`. A caller that leaves it `None` is every caller that exists today, and it is `None` for every replay — rule 14.8 gives a replay no cost, no ratio and no ceiling | 0 |

**Counted as items the growth is one, and counted as public declarations it is seven**, both figures being stated for
the reason the 2026-08-24 row states them: a check comparing a `pub` census against this rule must expand the rows and
must not guess the expansion. The seven are `UnitPrices`, its four public fields, `UnitPrices::parse`, and `Config`'s
new field. The item count is one because this rule's convention makes a type and its accessors one item, and because a
field appended to a struct this rule already encloses adds no item — which is why the second row's count is `0` and its
growth is nonetheless enumerated, that being exactly the form of growth the 2026-08-20 `ActionTrace` row exists to
catch. No `pub const` changes.

**Why the item is admitted rather than avoided.** Rule 14.3a obliges the shared parser to *retain* the four values,
unlike the paths it validates and discards, because the run computes with them; a retained value has to be somewhere,
and rule 4 is not it — a sixth `execute` parameter would move a signature this rule freezes and three greps check. The
alternative of four separate `u64` fields on `Config` was rejected on the ground the type's own documentation records:
three of the four prices are plausible values for each other's position, so four bare integers put a silent eighty-fold
cost error one transposition away from a run this repository pays for. **The additions list's ceiling clause is
satisfied on its own terms**: that clause forbids an item no relocated test requires, and this item is not admitted for
a test at all — rule 14.3a requires it of the parser, and the public-tier tests are the consequence rather than the
ground.

**What this amendment does not do.** Rule 4 is untouched and `execute` keeps five parameters, so its three greps stand
word for word. The mutating-method grep still returns exactly `run` and `advance_tick`: `UnitPrices::parse` is an
associated function with no receiver, so `pub fn .*&mut self` does not match it, and no method is added to any item that
was already on the interface. Rule 6 is **not** amended and is re-checked instead — the ten prohibited names stay ten
and stay private, and four `u64` copies of the operator's own command line grant no path into engine-owned state, no
mutable borrow and no handle, on the identical grounds this rule admits `Coordinate`'s two public `u8` fields. Rule 13
is untouched and the declared dependency set is **still empty**, measured rather than assumed: `parse` is hand-written
against `str::split` and `str::parse`, so no crate is added to read a colon-separated list. No target, package or
formatting obligation moves.

**The ordering here is deliberately the opposite of its predecessor's.** The amendment record's other 2026-08-29 row,
for `spend_ceiling`, records a census falsified by a commit that preceded its authorization. This one lands in the same commit as the type
and the field it enumerates, so at no commit does the census describe a struct that does not exist or omit a field that
does.

**Amended 2026-08-29 for `SPEC-MOK-007` rules 1.1a and 1.4a's grown port return, and the growth is enumerated as the
three amendments above enumerate theirs.** Two items are added, one already-public item changes shape, and nothing is
removed:

| Item | Growth | Count |
|---|---|---|
| `simulation::Proposal` | **added**: value type of three public fields — `action: Option<Action>`, `response: Option<String>`, `usage: ReportedUsage` — with one associated function, `nothing`, returning the proposal a port makes when it obtained nothing and has nothing to say about why | 1 |
| `simulation::ReportedUsage` | **added**: value type of four public `Option<u64>` fields — `prompt`, `cached_prompt`, `output`, `reasoning` — deriving `Default`, whose value is four **absent** counts | 1 |
| `simulation::Proposer` | its one proposing method's return becomes `Proposal` where it was `Option<Action>`. No method is added and none is removed, so the trait's two methods stay two *(a third, `halted`, is added by the amendment of later the same day below)* | 0 |

**Counted as items the growth is two, and counted as public declarations it is ten.** The ten are `Proposal`, its three
fields and `Proposal::nothing`, then `ReportedUsage` and its four fields. `Proposer`'s row counts `0` because a return
type changing is a change of shape on an item this rule already encloses and not an added item, which is the
`ActionTrace` form the 2026-08-20 row established and the same reason `Config`'s row above counts `0`. **A trait
method is not a `pub fn` declaration**, which the 2026-08-24 row already fixed for these same two methods, so nothing
in the ten is `propose` itself. No `pub const` changes.

**Why two items and not one.** The counts could have been four bare `Option<u64>` fields on `Proposal`, which would
have added no second item. They are a named type for the reason `UnitPrices` is one and measured the same way: the four
are unlabelled integers of similar magnitude in a fixed order, three of them plausible values for each other's
position, and rule 14's cost arithmetic and `REQ-MOK-070`'s cache ratio are both computed from them — so a transposed
pair is a wrong cost figure and a wrong ratio in a run this repository pays for, with nothing to catch it. The type
also carries rule 11.5's distinction in its own signature: `Option<u64>` and not `u64`, because "a reported count that
the provider did not report is recorded as **absent**, not as zero", and rule 14.5 depends on telling them apart.

**Why the fields are public.** `Proposal` and `ReportedUsage` are constructed by every implementation of a public
trait, which rule 20.4 puts in a host, and read by the engine that authors the record. Both directions cross the crate
boundary, so a private-field form would need a constructor taking seven arguments and seven accessors to read them
back — fourteen declarations where ten suffice, and a positional constructor is the transposition hazard the previous
paragraph rejects. They are admissible on the grounds this rule already recorded for `UnitPrices` and before it for
`Coordinate`: values with no interior state, granting no path into engine-owned state, no mutable borrow and no handle.
`Proposal::nothing` exists rather than a derived `Default` because "the default proposal" names nothing a reader can
check, where "nothing was obtained" is exactly rule 9.5's case; `ReportedUsage` derives `Default` instead, four absent
counts being a value rule 11.5 gives a meaning to.

**What this amendment does not do.** Rule 4 is untouched and `execute` keeps five parameters, its three greps standing
word for word — the port parameter's type is `Option<&mut dyn Proposer>` and the trait's name does not move. The
mutating-method grep still returns exactly `run` and `advance_tick`: `Proposal::nothing` is an associated function with
no receiver, and no method is added to any item already on the interface. Rule 6 is **not** amended and is re-checked
instead, above, at the paragraph that records the port as a use of that rule rather than an exception to it: the ten
prohibited names stay ten and stay private, and what the return gained is an owned `String` the port composed and four
`Option<u64>` copies of figures a provider reported. Rule 13 is untouched and the declared dependency set is **still
empty**, measured rather than assumed: nothing here is parsed by a crate. No target, package or formatting obligation
moves, and `Simulation::advance_tick`'s signature stays one line, this amendment changing no parameter of it.

**The ordering follows its predecessor's rather than the row before that.** This lands in the same commit as the two
types and the changed return, so at no commit does the census describe a type that does not exist or a return the build
does not have.

**Amended 2026-08-29 for `SPEC-MOK-007` rules 10.1 and 20.4's connector port, and the growth is enumerated as the four
amendments above enumerate theirs.** One item is added, nothing already public changes shape, and nothing is removed:

| Item | Growth | Count |
|---|---|---|
| `simulation::ConnectorPort` | **added**: struct generic over `BufRead` and `Write`, implementing `Proposer` over a connector's two already-connected streams and a transcript sink borrowed for the port's life, with one associated function, `new`, taking the two streams by value and the sink by mutable borrow (and, as amended later the same day below, the run's declared prices and ceiling). It is the port a recording host hands the engine | 1 |

**Counted as items the growth is one, and counted as public declarations it is two** — `ConnectorPort` and
`ConnectorPort::new` — which is `ReplayPort`'s decomposition of 2026-08-24 exactly, that row's two being the type and its
one associated function. **No field is public.** Unlike `Proposal`'s three and `ReportedUsage`'s four, admitted above
because a host constructs them from parts and the engine reads them back, nothing outside this crate assembles a
`ConnectorPort` from its parts or reads one back: the four fields are the streams, the sink and rule 8.4's schema (a
fifth, equally private, is added by the amendment below), and the associated function is the only way any of them is
set. No `pub const` changes; the protocol version and the verb
enumeration rule 8.4 builds the schema from are private constants in `src/simulation.rs`.

**Why the item is admitted rather than avoided, and the alternative was measured rather than dismissed.** Rule 10.1 makes
the connector "an executable the operator names by path as a host option" that "the host spawns as a child process", and
rule 20.1 makes the engine's binary target that recording host, while `SPEC-MOK-006` rule 1.2 keeps every process and
every path resolution out of the library target. The two halves of one live exchange therefore fall on opposite sides of
a crate boundary: the host starts the program and connects the two pipes, and the engine composes the request, frames
it, reads the response line and applies rule 10.4's grammar check. Something public has to carry the second half to the
first, and this is it.

The alternative was to build the port in `mokiterions-core/src/main.rs`, where the streams already are, and admit
nothing here at all. It was rejected on a measurement of what it would have cost instead. Rule 20.4.1 lends one port for
the whole run and rule 14 accumulates the run's cost inside it — `PortDecisionSource` is constructed per opportunity and
so cannot hold an accumulation — so a port in the binary target reaches rules 14 and 15's arithmetic only by this
specification publishing `accounting::RunAccount`, the per-token prices it computes with and the per-exchange usage it
consumes. That is a larger surface than one type and one associated function, and it puts the cost arithmetic's own
mutable state on the interface, which is the thing rule 6 exists to keep off it.

| Item | Form | Why it is admissible |
|---|---|---|
| `simulation::ConnectorPort` | generic over `BufRead` and `Write`, constructed from the caller's own two streams and a `&mut dyn Write` transcript sink — and, as amended later the same day below, the run's declared prices and ceiling — implementing `Proposer` | `SPEC-MOK-007` rule 10.1 puts the spawn in a host and rule 20.1 makes that host the engine's binary target, while `SPEC-MOK-006` rule 1.2 keeps every process out of the library target, so the streams are connected in one crate and read in another. **`src/main.rs` is a separate crate from `src/lib.rs`**, so `pub(crate)` does not reach the one construction site — `run_live` in `mokiterions-core/src/main.rs`, this item's only caller outside the crate. That is the fact that admits `ReplayPort`, reached by one caller here rather than two: the observer is a read-only host and `SPEC-MOK-003` gives it no live path, so it constructs no connector port and gets none of the three live options. It holds two streams the caller owns, a mutable borrow of a sink the caller owns and one owned `String`, so it holds nothing of the engine's; and it can neither start, signal nor reap a process, having been given no path and no handle |

**The construction site is cited by name and not by line, and the reason is a measurement.** The 2026-08-24 row cites
`ReplayPort`'s two callers as `mokiterions-core/src/main.rs:85` and `mokiterions-tui/src/main.rs:118`. **Neither line
number holds.** At `d96cced`, the commit this work order branched from and therefore before any change of this branch's,
the two constructions stand at lines 99 and 130; a line citation in an approved artifact decays with every commit that
touches the file above it, and this one had already decayed twice over. That row is **not edited** — it records what it
recorded, and `VREC-MOK-003`, `VREC-MOK-010` and `VREC-MOK-012` bind earlier content of this file — but the drift is
recorded here as a finding rather than reproduced, which is why this row names a function a reader can grep for instead
of a number the next commit falsifies.

**What this amendment does not do.** Rule 4 is untouched and `execute` keeps five parameters, its three greps standing
word for word: the port reaches the run through the `Option<&mut dyn Proposer>` parameter that already exists, which is
the whole point of rule 1.1 naming an interface rather than a type. The mutating-method grep still returns exactly `run`
and `advance_tick` — `ConnectorPort::new` is an associated function with no receiver, and `Proposer`'s two methods take
`&mut self` on an implementation and are not `pub fn` declarations, which the 2026-08-24 row already fixed for them. No
method is added to any item already on the interface.

Rule 6 is **not** amended and is re-checked instead. The ten prohibited names stay ten and stay private. What this item
holds a reference into is a sink the caller opened, which the 2026-08-24 paragraph on that rule already carves out — a
referent the caller owns is not a reference into engine state — and the two streams are owned outright, having been moved
in. `DecisionSource` and `Observation` stay private, per `SPEC-MOK-007` rule 20.6, so an implementation of this port sees
a composed request and never the observation behind it.

Rule 13 is untouched and the declared dependency set is **still empty**, measured rather than assumed, and this is the
amendment where that costs something. Rule 10.1 states of this binding that "**neither Rust package acquires a crate**"
and names this rule 13 among the provisions it therefore leaves untouched, so the responses of a program this repository
did not write are read by a hand-written reader in `src/simulation.rs`: a private
module, total over `&str`, bounded in nesting, with no floating-point type anywhere. No crate is added, no
dev-dependency is added, and rule 8's public tier gains `tests/connector.rs` under that rule's own clause admitting "a
further file when a further public subject appears" — the spawn, the inheritance and the reaping are that subject, and
they need a real child process to observe.

**The ordering follows the two rows above it.** This lands in the same commit as the type it enumerates, so at no commit
does the census describe a type that does not exist.

**Amended 2026-08-29 for `SPEC-MOK-007` rule 14's accounting, which the connector port accumulates, and the growth is
nil.** Nothing is added, nothing is removed, and one item already on this interface changes shape:

| Item | Growth | Count |
|---|---|---|
| `simulation::ConnectorPort::new` | **shape**: two parameters are added after the transcript sink — the run's four declared unit prices as `simulation::UnitPrices`, and its ceiling as `Option<u64>` in whole US cents | 0 |

**Counted as items the growth is nought, and counted as public declarations it is nought.** A signature changing on an
item this rule already encloses is a change of shape and not a growth of the interface, which is the `ActionTrace` form of
the 2026-08-20 row and the form `simulation::Proposer`'s row of earlier the same day takes. Both parameter types are
already enclosed: `simulation::UnitPrices` was admitted to the additions list earlier this same day for rule 14.3a's
`--prices`, and `Option<u64>` is a primitive over a primitive. **No field becomes public.** The item gains a fifth
private field, `accounting::RunAccount`, whose type is private to a private module of `src/simulation.rs`; the four fields
the row above enumerates are unchanged. No `pub const` changes, no method is added, and no trait gains or loses one.

**Why two parameters rather than none, and why on `new` rather than on `propose`.** Rule 14.2 computes a live run's cost
"from the reported counts and the unit prices declared for the run", rule 14.6 stops the run once that cost reaches the
declared ceiling, and rule 20.4.1 builds one port per run and lends it per tick — so the accumulation lives in the port,
and the two figures it accumulates against are inputs the port cannot obtain for itself. Rule 14.3 is what forbids the
alternative: prices are "inputs of the run" and never constants, so there is nothing for this item to read and nowhere to
read it from. They arrive on `new` and not on the proposing method because prices arriving per exchange could differ
between exchanges of one run, which is a cost figure no reader of the record could reconstruct — and because rule 1.1's
proposing method is the engine's, and the engine may not see an accounting figure at all under rule 14's *State model*.

**The ceiling crosses this interface in whole US cents and nothing finer.** Rule 14.2 as amended 2026-08-29 under
`WO-MOK-030` states the minor unit and it is the cent, `--spend-ceiling` parses to it, and `simulation::Config` already
carries `spend_ceiling: Option<u64>` in it. That the accumulation behind this parameter is finer than the parameter is a
private matter of `src/simulation.rs` and is recorded there: one exchange at rule 14.3a's own example prices costs about
0.03 of a cent, so a cost accumulated in whole cents would add nought every exchange and rule 14.6's ceiling would never
be reached. **No unit conversion is asked of any caller**, which is the property this parameter's type is chosen for.

**What this amendment does not do.** Rule 4 is untouched and `execute` keeps five parameters, its three greps standing
word for word: the prices reach the library through `simulation::Config`, which rule 14.3a's own amendment of earlier this
day admitted them to, and the port still reaches the run through the `Option<&mut dyn Proposer>` parameter that already
exists. The mutating-method grep still returns exactly `run` and `advance_tick`, measured at the candidate —
`ConnectorPort::new` still has no receiver, and the two parameters added to it are values. `Simulation::advance_tick`'s
signature stays one line, no parameter of it moving. Rule 6 is **not** amended and is re-checked instead: the ten
prohibited names stay ten and stay private, and the fifth field holds four `u64` copies of the operator's own command
line and six accumulators derived from figures a provider reported, so it is neither a reference into engine state nor a
name rule 6 lists. Rule 8's table is **not** amended and no file joins the public tier, the two parameters being
observable only through the run record a later scope item writes. Rule 13's declared dependency set is untouched and
**still empty**, measured rather than assumed, the arithmetic behind these two parameters being integer arithmetic in
`src/simulation.rs` with no crate anywhere near it.

**The ordering follows the three rows above it.** This lands in the same commit as the signature it enumerates.

**Amended 2026-08-29 for `SPEC-MOK-007` rule 19.3's status and rule 1.1b's question, and the growth is one declaration.**
One item is added, one item already on this interface changes shape, and nothing is removed:

| Item | Growth | Count |
|---|---|---|
| `CEILING_STOP_EXIT` | **added**: `u8` constant in `src/lib.rs`, the process status rule 19.3 requires a ceiling-stopped run to report. Rule 4's amendment of the same date fixes its meaning and its value | 1 |
| `simulation::Proposer` | **shape**: a third method, `halted`, taking `&self` and returning `bool`, with a default body of `false`. Nothing is removed and no signature already here moves, so the trait's two methods become three | 0 |

**Counted as items the growth is one, and counted as public declarations it is one**, both figures stated with their
decomposition under the 2026-08-24 convention. The one is the constant. The trait's new method is not a `pub fn`
declaration, which is that convention's own wording — "`Proposer` is one, its two trait methods not being `pub fn`
declarations" — and a method added to a trait this rule already encloses is a change of shape rather than a growth of the
interface, which is the `ActionTrace` form of the 2026-08-20 row and the form this same trait's return took earlier the
same day. **It is the first `pub const` any amendment to this rule has added.** The four amendments above each close by
recording that no `pub const` changed; this one does change one, and says so where they said the opposite.

**Why the status is a constant when the other three are literals.** Rule 4's amendment gives the reason and this rule
gives the consequence: the value crosses a crate boundary, `src/main.rs` being a separate crate from `src/lib.rs` and
`SPEC-MOK-006` rule 13.4's removal exception being the binary target's to apply, so `pub(crate)` cannot express it. The
contrast is inside this crate and is worth stating, because it is the reason this is one declaration and not two:
`simulation::MISSING_DECISION_PORT`, which rule 4's 2026-08-23 amendment reports through `execute`, is shared between two
*modules* of this crate and is `pub(crate)`, no host reading it. It stays `pub(crate)` and is not widened, so rule 6's
last clause is not engaged. The public-tier test in `tests/connector.rs` asserts the literal `3` and not this constant,
which is deliberate: a test reading the constant would agree with any value the constant took, and what rule 4 fixes is
the value.

**Why the question is a method on the port and not a figure the engine reads.** `SPEC-MOK-007` rule 1.1b, approved the
same day under `WO-MOK-026`, puts it there: rule 14.6 requires the check *before* the spending, rule 20.4.1 puts the
accumulation in the port because the port is what spends, and rule 14's *State model* lets the engine read no accounting
figure at all — so the engine asks a question it cannot answer for itself, at the decision opportunity it already has,
before the request is composed. The alternative the owner declined was a field on `Proposal`, measured at nought items and
one or two declarations: cheaper on this rule's census and dearer everywhere else, because it has no ordering contract at
all, and because "no exchange was issued" and rule 9.5's "the exchange yielded nothing" would become two absences one
field apart, which a reader who confused them would answer by writing a fallback record for an exchange that never
happened. `&self` and not `&mut self` is part of the shape and not an implementation detail: asking must move no figure,
or the number the answer depends on becomes a function of how often the engine asked.

**The default body is rule 14.8, and it is why this costs no implementor anything.** A replay spends nothing, computes no
ratio and has no ceiling, so `simulation::ReplayPort` takes the default and gains no line, and so does any port with
nothing behind it. One implementation in this workspace does override it and it is not a spending one: `mokiterions-tui`'s
`LentPort` wraps a port the observer was handed and forwards the question, because a wrapper that answered from the default
would answer *for* the port it wraps. That is the only obligation this method creates, it falls on wrappers alone, and
`SPEC-MOK-003` is not amended — the observer offers none of the three live options and constructs no connector port.

**What this amendment does not do.** Rule 4 **is** amended, in its own block above, and this is the first of these five
amendments where it moves: the fourth exit code is that rule's and is not counted again here. `execute` keeps five
parameters and its three greps stand word for word, the status crossing as the function's return value and no parameter
being added. The mutating-method grep still returns exactly `run` and `advance_tick`, measured at the candidate — `halted`
takes `&self`, and it is a trait method rather than a `pub fn` in any case, so it matches neither half of the pattern.
`Simulation::advance_tick`'s signature stays one line, no parameter of it moving. The *Authorized additions* table above
is **not** edited: the enumeration is this block's growth table, which is the form `simulation::Proposer` and
`simulation::ConnectorPort` already take. Rule 6 is **not** amended and is re-checked instead — the constant is a `u8`
process status in the process-boundary module and not a simulation constant, naming no world dimension, no rate and no
threshold, so rule 6's clause reserving `CELLS_PER_TERRITORY` is untouched; `halted` returns a copy of a fact by value and
grants no path into engine-owned state; and the ten prohibited names stay ten and stay private. Rule 8's table is **not**
amended and no file joins the public tier, `tests/connector.rs` having joined it earlier the same day. Rule 13's declared
dependency set is untouched and **still empty**, measured rather than assumed.

**The ordering follows the four rows above it.** This lands in the same commit as the constant and the method it
enumerates, so at no commit does the census describe an item that does not exist.

### 6. Prohibited public interface

None of the following may be public, and none may be reached from a public item by reference, borrow, public field,
trait method, callback, or closure argument:

- a mutable borrow of, or a reference into, the world grid, the agent collection, the resource collection, the tick
  counter, the entropy state or the event log, and any handle to the engine that permits mutation;
- `Mokiterion`, `Food`, `RelativeDirection`, `ActionResult`, `Observation`, `PerceivedFood`, `PerceivedMokiterion`,
  `SplitMix64`, `DecisionEntropy`, `DecisionSource` and its implementations;
- observation construction, action application, survival application, regeneration, food counting, decision
  dispatch, the summary constructor, and every simulation constant other than `CELLS_PER_TERRITORY`;
- any `pub(crate)` item widened to `pub` for a reason other than an approved requirement.

No feature flag, `cfg` attribute, self dev-dependency, or conditional-visibility mechanism may make any of the above
reachable from outside the crate, including in test builds. There is no test-support seam.

**A reference into a value the caller owns is not a reference into engine state, and this is stated because a check
missed it** *(2026-08-24)*. The first bullet forbids a reference into seven named things. A `&str` borrowed from a
`DecisionRequest` the caller was handed by value is none of them: the referent's owner is the caller, the borrow cannot
outlive it, and the `Simulation` is not reachable through it. The mechanical form of this rule must therefore carve out
**both** kinds of admissible reference — a `'static` referent, which cannot be engine-owned because `'static` outlives
the `Simulation`, **and** a referent the caller owns. A form that carves out only the first reports every accessor of
every value type this rule admits, which is a check that fails a conforming build. `WO-MOK-025`'s
`candidate/static-checks.txt` check 4 is that form and reports six such accessors, all of them `DecisionRequest`'s; the
capability it exists to deny is absent, which the same packet's check 3 establishes independently.

**Amended 2026-08-18, in two places.** The prohibition was written as a list of type names and as a ban on reaching
them "by … return value". Both are narrowed to the capability the rule exists to deny, and nothing else changes.

The first bullet read "the world grid, the agent collection, … the event log, or any handle to the engine that permits
mutation", and "return value" appeared in the list of paths above it. Together they forbade an owned, reference-free
copy of state the program already prints, which is what rule 5's observation surface returns and what
`REQ-MOK-019` through `REQ-MOK-027` require. What is denied is the capability: no public item hands out a borrow of
engine-owned state, a reference into it, an iterator over its collections, an interior-mutable value, or a handle
that permits mutation. A copy grants none of those. The two mutating methods rule 5 accounts for, `run` and
`advance_tick`, are unaffected: both take `&mut self` on a `Simulation` the caller owns, which is not a handle into
another owner's state.

The second bullet named fifteen types. `Coordinate`, `Direction`, `Territory`, `FoodClass` and `Action` are removed
from it, because rule 5's snapshots carry all five by value and cannot be expressed without them: a position, a
territory, a calorie class and an action are the facts being observed. Each is a small `Copy`-or-clone value with no
engine reference, and each is already printed in the event stream or the summary line, so publishing it grants no
capability that did not already exist. The other **ten** names stay in the bullet, stay prohibited and stay private:
`Mokiterion`, `Food`, `RelativeDirection`, `ActionResult`, `Observation`, `PerceivedFood`, `PerceivedMokiterion`,
`SplitMix64`, `DecisionEntropy` and `DecisionSource`. `Observation` and the `DecisionSource` trait are the two that
carry the `ADR-MOK-001` trust boundary, and they are deliberately among the ten: the observation surface is for a
host that watches, not for one that decides.

This rule remains the security-relevant rule of this specification, and `REQ-MOK-004` and `ADR-MOK-001` are preserved
exactly. Narrowing it makes it checkable by a property of the public surface rather than by a list that must be
maintained as types are renamed.

**Re-checked 2026-08-20 under `CAP-MOK-010` and not amended, because that initiative introduces the one thing this rule
had never had to consider: an action by one Mokiterion that mutates another.** The re-check is recorded rather than
assumed, because "no public item hands out a path to engine-owned state" is a claim about a surface whose meaning
changes when the engine gains cross-agent mutation.

- **A target is an identifier, not a reference.** The seven added `Action` variants each carry an agent identifier by
  value. `Action` is public as a value under this rule's 2026-08-18 narrowing, and a public value carrying the string
  `M03` grants no more reach than the `AgentSnapshot` that already carries it. There is no `&mut Mokiterion`, no index
  into the agent collection, and no callback through which one could be obtained.
- **Cross-agent mutation happens entirely inside the engine.** Rules 22 to 24 of `SPEC-MOK-001` resolve against
  authoritative state the engine owns, reached from the tick loop and not from anything a caller holds. A source
  proposes an identifier; the engine looks it up. That the source cannot reach its target is precisely why the
  `Observation` and `DecisionSource` prohibitions below stay where they are.
- **The three added `EventType` variants and their details are copies of what the text stream prints**, on the same
  ground the twelve existing ones are admissible, and they carry a second Mokiterion's transitions as numbers rather
  than as any path to it.
- **The ten prohibited type names are unchanged and all ten stay private**, including `Observation` with its two new
  fields and `DecisionSource` with its fourth implementation. The `ADR-MOK-001` trust boundary is where it was: the
  observation surface is for a host that watches, not for one that decides — and now not for one that fights either.
- **No `pub(crate)` item is widened** and no new mutating method exists, so the two-method `grep` check in rule 5 is
  still the whole of the mutation surface.

**Not amended on 2026-08-20, and recorded here so that the omission is deliberate rather than overlooked.**
`ADR-MOK-005` requires no change to this rule, and `WO-MOK-019` makes none. The record projection reads
`SplitMix64`'s state through a `#[cfg(test)]` accessor returning an owned `u64` — the value, never the type, never a
borrow — and `SplitMix64` stays on the second bullet, private in every build configuration. The ten prohibited names
stay ten. The sink the projection writes to is a `Write` the caller owns and passes in; it is not engine-owned state,
so handing the projection a borrow of it grants no reach into the engine. Nothing on this list becomes public, and
nothing public becomes a path to anything on it.

**Not amended on 2026-08-23 either, and recorded here for the same reason.** `ADR-MOK-007` requires no change to this
rule and states why: the decision port is a **use** of it, not an exception to it. A reader would expect the opposite,
because the port is a public trait a caller implements and this rule's opening sentence names "trait method, callback,
or closure argument" among the ways a prohibited item must not be reachable — so the check is worth stating rather than
assuming. The port's one method takes a `DecisionRequest` **by value** and returns a `Proposal` by value — an
`Option<Action>` by value until `SPEC-MOK-007` rule 1.1a grew it on 2026-08-29, and the check reaches the same
conclusion for the same reason. Both are values; neither is a borrow of engine state, an index into a collection or a
handle, and the two fields the return gained are an owned `String` and four `Option<u64>`. An implementation therefore
receives a rendered copy of what one Mokiterion perceived and can reach nothing behind it, which is `SPEC-MOK-007` rule
1.3 and this rule's first bullet meeting at the same conclusion.

The ten prohibited names stay ten and all ten stay private. `Observation` is the one to check by name: the request is
composed *from* an observation and the observation itself does not cross, so a host implementing the port sees the four
rendered blocks and never the type. `DecisionSource` stays private too, and the `llm` source is its fifth
implementation — `SPEC-MOK-007` rule 20.6 fixes that asymmetry deliberately, so that a host can supply a proposal
without being able to implement the engine's own dispatch. No `pub(crate)` item is widened, and the one that carries
the port down the call chain stays `pub(crate)`.

### 7. Tiers and the placement rule

Every test belongs to exactly one tier, and the tier is determined by the access the test requires:

- if the test can be written using only rule 5's interface, with its assertions unchanged, it belongs to the
  **public tier**;
- otherwise it belongs to the **internal tier**.

A test is not left inline for convenience when rule 5 suffices, and a test is not promoted to the public tier by
widening rule 5. Required access is a property of the test as written; the subject it covers does not decide the
tier.

### 8. Public tier

Located in `tests/`, one file per subject, each compiled as its own integration-test target and reaching the code
as `use mokiterions::…`. The initial arrangement is:

| File | Subject |
|---|---|
| `tests/cli.rs` | argument parsing: defaults, order independence, policy selection, duplicate and missing values, accepted and rejected density forms |
| `tests/process.rs` | the process boundary: help output, invalid configuration, a density resolving to no resources, and output failure, each with its exit code |
| `tests/density.rs` | resolved resources per territory, and the relationship between density, initial endowment, and capacity |
| `tests/termination.rs` | termination by tick limit and by extinction, and the emitted summary |
| `tests/viability.rs` | the population floor at the declared density on the declared seeds |

A further file may be added when a further public subject appears. One file per test is not the arrangement.

### 9. Internal tier

Located in a `#[cfg(test)] mod tests` inside the source file that owns the subject: `src/simulation.rs` for engine
state, observation construction, action validation and application, survival, regeneration, entropy, and both
decision sources; `src/cli.rs` for any parsing detail that rule 5 does not expose. `src/lib.rs` and `src/main.rs`
contain no tests, because rules 3 and 4 leave them with nothing private to assert.

### 10. One invocation

`cargo test` compiles and runs both tiers. Neither tier requires a feature, an environment variable, an
`#[ignore]` attribute, a separate command, or a particular working directory. The number of executed tests is the
same before and after relocation.

Amended 2026-08-18: `cargo test -p Mokiterions` runs both of this package's tiers and nothing else, with no terminal
present and with the observer package excluded from the build. That form is what demonstrates this rule. `cargo test`
at the workspace root additionally runs the observer package's tests, which `SPEC-MOK-003` governs.

### 11. Behavior preservation

The restructuring is equivalence-preserving. For identical arguments and identical seed, the program emits
byte-identical output, reaches an identical final state, and returns an identical exit code, with and without
`--trace-actions`, under both decision sources, at every declared density.

No simulation constant, event field, event order, summary field, exit code, diagnostic message, or byte of `USAGE`
changes. Every case, invariant, and check in `VER-MOK-001` and `VER-MOK-002` remains covered.

### 12. Test content preservation

A relocated test keeps its assertions verbatim. Only the path by which it reaches the code changes — a `use` of the
library target in place of `use super::*`, and public accessors in place of private field reads. A relocated test
whose assertions cannot survive the move is a rule 7 misclassification and stays in the internal tier.

### 13. Declared dependency set

**Added 2026-08-20 under `ADR-MOK-006`.** This is the engine package's declared set, referenced by rule 1. Every
external crate in this package's resolved dependency graph — including one reached transitively and one reached only by
a dev-dependency — is an entry in this table, and every entry is in the graph. A resolved set that differs from this
table in either direction is a violation of `REQ-MOK-050`, and `SPEC-MOK-005` rule 8.4 is the check.

| Crate | Version | Features | Build script | Admitted by |
|---|---|---|---|---|

**The table is empty.** That is the whole of it: as this rule is written the engine package declares no external
dependency, and `cargo tree -p Mokiterions -e normal --locked --offline` resolves to one crate, this package itself,
measured on 2026-08-20 in this checkout under `cargo 1.97.1 (c980f4866 2026-06-30)`. An empty
table is now a **fact about the current declaration** rather than a rule, which is exactly what `ADR-MOK-006` changed —
before it, the emptiness was the provision; after it, the emptiness is the state and the provision is the comparison.

Adding a row is a decision, not an implementation act:

1. **The technical owner applies the criteria of `ADR-MOK-006` decision 1** — stable, well-maintained functionality
   that accelerates delivery without excessive dependency debt, and proven solutions for standard, non-core features —
   to the candidate crate. There is no numeric threshold for *excessive*, by `ADR-MOK-006` decision 10, and there is
   no crate-count ceiling.
2. **The row is added by amendment to this specification**, approved by the technical owner, whose *Approval* cell
   records that the criteria were applied to that crate. An implementation agent may propose a row and may not decide
   one, and may choose neither the crate, nor the version, nor the feature set.
3. **The envelope of `ADR-MOK-006` decision 4 is checked before the criteria**, not after: no crate providing network
   access, credential handling, an asynchronous runtime, a database, a plugin system or dependency injection is
   admissible here however stable it is, and no user-interface crate is admissible in this package at all.
4. **`ADR-MOK-006` decision 11 is checked**: no entry may implement simulation semantics — the rules `SPEC-MOK-001`
   fixes, the world model, agent decision-making — own or advance entropy, or perform action validation. This is a
   review, retained as a manual assessment under `VER-MOK-014`.
5. **Determinism is checked**, per `ADR-MOK-006` decision 6: no entry may draw entropy, read wall-clock time, read the
   environment, or introduce iteration-order nondeterminism into any value the `REQ-MOK-010` stream, the authoritative
   event sequence or the final state observes. Where such a capability sits behind a feature, the feature is off and
   its absence is part of the *Features* column.

Column meanings, so a row is unambiguous:

- **Version** is the exact resolved version, not a range. A version change is an amendment.
- **Features** is the exact enabled feature set, written as it appears in the manifest, including
  `default-features = false` where that applies. A feature set change is an amendment, and a feature enabled by
  unification without one is a mismatch.
- **Build script** is `yes` or `no`, per `ADR-MOK-006` decision 13: whether the crate itself carries a `build.rs`, so
  the build-time code-execution surface is enumerated rather than discovered. A crate that acquires one is a mismatch,
  not an unremarked change. This column describes a *dependency's* build script and does not relax rule 1's prohibition
  on a build script in this package, which is unchanged.
- **Admitted by** names the amendment row that added the entry, so every crate is traceable to an approval that says
  the criteria were applied to it.

**Reading a *Features* cell mechanically.** `SPEC-MOK-005` rule 8.4b is a program, so the cell has a fixed reading and
not only a prose meaning. Within the cell, `default-features = false` is the default-features switch and every other
backticked token is a feature name. A sentence containing the word *off* names features that must be **absent** from the
resolved set and names nothing else. A sentence containing the word *implied* names features that the resolved set may
contain because a declared feature activates them, which the manifest therefore does not list. Every remaining token is
a feature the manifest declares and the resolved set must contain. The check is then exact: the resolved feature set
equals the declared features together with the implied ones, and intersects the prohibited ones nowhere. This
convention binds `SPEC-MOK-003`'s declared set too, which is the only cell that has content today. It is written here
rather than left to the checking program because a program that guessed at the reading would be a second declaration.

**Not amended on 2026-08-23, and re-measured rather than asserted.** `ADR-MOK-007` decision 3 is the reason this rule
survives a model-backed decision source at all: the provider client, its transport and its credential handling live in
a connector program this repository does not build, so nothing in this package needs a crate to reach a model. The table
is still empty, and `cargo tree -p Mokiterions -e normal --locked --offline` still resolves to one crate, this package
itself, measured on 2026-08-23 in this checkout under `cargo 1.97.1 (c980f4866 2026-06-30)`. The same command without
`-e normal`, which includes dev-dependencies, resolves to the same one crate. An earlier draft of `ADR-MOK-007`
estimated this table growing to forty to sixty entries; the option the owner accepted is the one that leaves it empty,
and that is worth recording here because the estimate is what the emptiness was traded against.

## Error and recovery behavior

- A public-tier file that fails to compile because it names a private item is a rule 7 signal: reclassify the test.
  It is never grounds for a rule 5 addition.
- A clippy failure caused by a target name is corrected under rule 2, never by narrowing the lint invocation or
  adding an `allow` attribute.
- Any observable difference from the pre-change baseline is a defect in the restructuring, not a new baseline.
- If rule 5's lists prove insufficient for a test that rule 7 assigns to the public tier, the correct outcome is that
  the test stays in the internal tier and the insufficiency is recorded, not that rule 5 grows. Rule 5 grows on an
  approved requirement and on nothing else; a test is never that requirement.

## Data and interface contracts

The public interface carries only values: configuration in, and copies of already-reported outcome facts out. No
public item returns a reference into engine-owned state, a mutable borrow, an iterator over engine-owned
collections, a trait object with mutating methods, or a closure holding engine state.

`RunSummary` stays opaque. Its accessors return owned copies, so the summary cannot be used as a window into live
state.

Amended 2026-08-18: the same holds of every snapshot type rule 5's third list admits. Each is a tree of owned values
with no reference into engine state, no shared handle and no interior mutability, so a snapshot is a photograph of one
completed tick and not a window onto the next. `SPEC-MOK-003` states the property and the ordering guarantee that
makes two frames of one tick identical.

## Security and privacy properties

- No network access, credential read, filesystem access, environment read, or wall-clock read is introduced. Amended
  2026-08-24 for `REQ-MOK-063`, by way of `ADR-MOK-007`: **the sentence takes the target scope it has not needed until
  now.** Of the **library target** all five continue to hold, and that is the load-bearing half — `src/lib.rs`,
  `src/cli.rs` and `src/simulation.rs` contain no `std::fs`, no `File::`, no `OpenOptions`, no `remove_file`, no
  `env::`, no process spawn, no socket and no clock, measured at this candidate rather than assumed, so the port
  arrives already constructed and the transcript already open. Of the **binary target** filesystem access holds no
  longer and has not since 2026-08-20: `src/main.rs` creates or truncates the record sink, removes a file it created
  on failure, and — added at this date — opens the operator's transcript for reading. It interprets **two**
  operator-supplied paths and no more. **The other four still hold of the binary target too, at this candidate**: it
  reads the command line and no environment variable, opens no socket, spawns no process and reads no clock.
- Added 2026-08-24 for `REQ-MOK-063`: **what this bullet does not yet say, and who owes it.** `ADR-MOK-007` states that
  three of the five stop holding of the engine **package**, the third and fourth being a spawned connector process and
  the environment passed through to that child. **Neither exists in this tree.** `WO-MOK-025`'s *Out of scope*
  excludes any process spawn and the connector itself, so the network access and the environment read that would
  follow from them are `WO-MOK-026`'s to write here, in the target-scoped form the bullet above now uses; the
  2026-08-23 row of the *Amendment record* forecast that they would land under `WO-MOK-025` scope item 14, and that
  forecast is corrected in the row below rather than by editing it. **No credential is read by either target at any
  stage**, `SPEC-MOK-007` rules 10.5 and 13.4 placing it in the connector alone, and **no wall-clock read is added by
  anything in this initiative.** A process spawn does appear in the **public-tier tests**, which invoke the compiled
  binary in order to observe a process boundary from outside it; that is what a process-boundary test is, it predates
  this amendment, and it is not a target spawning a child.
- Making items reachable from outside the crate grants no capability that did not already exist inside it: every
  authorized addition returns a copy of a value the program already prints.
- Rule 6 is the security-relevant rule of this specification. It preserves `REQ-MOK-004` and `ADR-MOK-001`'s
  prohibition on exposing mutable world state, and it holds in test builds as well as release builds.
- The trust boundary is unmoved. A decision source still receives immutable observations and returns typed
  proposals, and nothing in the public interface reaches that boundary.

## Performance and capacity

Runtime behavior is unchanged; rule 11 requires it. Compile-time cost grows: the binary target links the library
target, and each public-tier file becomes an additional test target, so `cargo test` builds more artifacts and
`target/` grows. Per-tick work, memory use, and output volume are unaffected.

## Observability

Unchanged. The event stream, the action-trace lines, the summary line, and the exit codes are exactly as verified.
This specification adds no log, metric, or diagnostic.

**Amended 2026-08-29 under `WO-MOK-026`: the exit codes are no longer exactly as verified.** Rule 4's amendment of that
date adds a fourth, `3`, for `SPEC-MOK-007` rule 19.3's ceiling stop, and one line on standard error reports it. This
section is amended because the sentence above enumerates the codes and would otherwise be false, which is the only reason
it is amended: the line is a diagnostic this specification records rather than one it adds, rules 14.7 and 15.5 of
`SPEC-MOK-007` fixing what it may and may not say. The event stream, the action-trace lines and the summary line are
unchanged in form, and a ceiling-stopped run writes no summary line at all — rule 14.7 ends its text stream at the last
tick it completed, and rule 15.5 forbids quoting a figure at a horizon the run did not reach. No log and no metric is
added.

## Compatibility and migration

- The public interface becomes a maintained contract. It grows only when an approved requirement needs it to grow,
  and this specification is amended in the same act.
- `ARCH-MOK-001` states one binary crate as a quality attribute and as a conformance check, and prohibits separate
  crates without an approved requirement. This specification cannot be conformed to until the technical owner
  amends it. `ADR-MOK-002` states the required amendments; `WO-MOK-003` makes them an approval precondition.
- `SPEC-MOK-001`'s *Explicitly unspecified decisions* delegates "test organization and helper functions" to the
  implementation agent. Rules 7 to 10 narrow that delegation to file-internal organization and helper structure.
  `SPEC-MOK-001` needs a one-row in-place amendment pointing at this specification; that amendment is the technical
  owner's act and is likewise an approval precondition of `WO-MOK-003`.
- `REPOSITORY_CONTEXT.md` records the test-placement convention already. Its *Architecture* section still names
  `src/main.rs` as the entry point and is updated when this specification is conformed to.
- `WO-MOK-002`'s retained requirement-to-test mapping states that tests live in `src/simulation.rs`, `src/cli.rs`,
  and `src/main.rs`. It is bound to its commit by `VREC-MOK-002` and is not edited. A superseding mapping is
  produced as evidence under the implementing work order.
- Added 2026-08-18: `SPEC-MOK-003` is the second specification the engine package's library target answers to. This
  one owns the target shape, the closed public enumeration and test placement; that one owns the observation surface's
  content, ordering and ownership, and the whole of the observer package. Rule 5's third list is the join between them,
  and the two must agree — if a future amendment to either changes the surface, the other is amended in the same act.
  `WO-MOK-005` implements the four amendments recorded above and `VER-MOK-005` covers them; `WO-MOK-003` and
  `VREC-MOK-003` remain the record of this specification as originally approved, and are not re-opened.
- Added 2026-08-20: `SPEC-MOK-006` is the third specification the engine package answers to, and the first that binds
  the binary target as well as the library target. It owns the record stream; this one owns the seam. The seam is one
  parameter on `execute`, stated at rule 4, and it is the whole of the coupling — `SPEC-MOK-006` rule 12.2 says as much
  from its side, and the two agree by construction because neither can grow the interface without the other being
  amended in the same act. Every call site of `execute` in the workspace gains an argument, mechanically and without a
  change to what it asserts: `mokiterions-core/src/main.rs`, `mokiterions-core/tests/process.rs`,
  `mokiterions-tui/src/verification.rs` and `mokiterions-tui/tests/verification.rs`. `mokiterions-tui` is otherwise
  untouched and does not offer the option: `SPEC-MOK-003` is not amended, the observer supplies no sink, and rule 4's
  parameter is `None` there, so the observer's behavior is unchanged. `VREC-MOK-003` and `VREC-MOK-010`, which bind
  earlier content of this specification to their commits, are not edited; the signature they name was correct at those
  commits and this row records why it differs afterwards.

## Examples and counterexamples

**Example.** The exit-code test for `--density 0.01` needs argument handling, `execute`, and two byte buffers. Rule
7 places it in the public tier; rule 8 places it in `tests/process.rs`; rule 12 keeps all three of its assertions —
the code is `2`, standard output is empty, and standard error contains both `zero resources` and `Usage:`.

**Example.** The density-resolution test asserts that `0.15%`, `0.75%`, and `1.50%` resolve to `12`, `61`, and `122`
resources per territory. It needs `Density::parse` and `Density::resources_per_territory`; the second is an
authorized rule 5 addition, so the test moves to `tests/density.rs`.

**Counterexample.** The survival test that sets an agent's satiety to zero and calls the private survival routine
requires the agent collection. Making `Simulation`'s agent collection public so the test can move to `tests/`
violates rule 6, misapplies rule 7, and contradicts `REQ-MOK-004`. The test stays in `src/simulation.rs`.

**Counterexample.** Adding `#[cfg(feature = "test-support")] pub mod internals` so that all fifty-two tests can
live in `tests/` violates rule 6. Gating does not make exposure conditional in any sense that matters: the feature
is enabled during the build that exposes the state.

**Counterexample.** Replacing the survival test's direct assertion with "a death event appears somewhere in a
1,000-tick run" in order to place it in the public tier violates rule 12. The relocated test is weaker, so the
relocation is a defect rather than a move.

## Explicitly unspecified decisions

- Ordering of items within each source or test file, and whether public-tier helpers are duplicated per file or
  shared through a `tests/common/` module.
- The exact names and signatures of the `RunSummary` accessors, provided each returns an owned copy.
- Where the failing-writer helper used by the output-failure test lives.
- Doc comments, internal comments, and non-authoritative developer notes.
- Whether the internal tier's existing helper functions are kept, renamed, or consolidated, provided no assertion
  changes.
- Whether `src/simulation.rs` is later split into several private modules. Rules 5 and 6 constrain visibility, not
  file count, and no rule here requires or forbids such a split.
