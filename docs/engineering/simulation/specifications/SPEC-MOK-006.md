+++
id = "SPEC-MOK-006"
type = "specification"
title = "Structured record stream"
status = "approved"
owners = ["technical owner"]
created = "2026-08-20"
updated = "2026-08-23"

[relations]
specifies = [
  "REQ-MOK-042",
  "REQ-MOK-043",
  "REQ-MOK-044",
  "REQ-MOK-045",
  "REQ-MOK-046",
]
+++

# Specification: Structured record stream

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-20 | Original content for `REQ-MOK-042` through `REQ-MOK-046`. | Approved 2026-08-20 by the repository owner acting as technical owner, in the act that also approved `INT-MOK-009`, `CAP-MOK-009`, `REQ-MOK-042` through `REQ-MOK-046`, `ADR-MOK-005`, `VER-MOK-012` and the work order. Written under `WO-MOK-019`, which was `WO-MOK-012` until the 2026-08-21 renumbering `evidence/WO-MOK-012/identifier-collision.md` decision 3 required of whichever branch reached `master` second. |
| 2026-08-21 | **This section is added, and the stream's vocabulary is extended from twelve event kinds to fifteen.** The merge of `master` at `fa065cc` into this branch at `fa0bfd9`, as `1e09f85`, is the first tree in which `CAP-MOK-010`'s combat and this stream both exist, and it emits fourteen field names this specification does not name. Rule 3.4 is the provision that makes this an amendment rather than an implementation detail: a string field added to the stream "must either be added to that enumeration or arrive together with an escaping function and its own verification". **Rule 3.2**: `config.policy` and `result.source` gain `social`; the `event` row reads **fifteen** event types rather than twelve; `result.proposal.action` gains the seven targeted verbs `attack`, `threaten`, `fight`, `retreat`, `surrender`, `approach` and `avoid`, making eleven; one new row gives `result.target`, `result.recipient`, `result.proposal.target` and `result.suffered[].attacker` the domain `M[0-9]{2}`; a new row gives `result.target_died` the **closed two-value domain** `yes`, `no`; and `result.detail` gains eleven members — the eight words master's targeted validation and its co-located rejection can produce (`target_unknown`, `target_dead`, `target_is_actor`, `target_not_perceived`, `target_not_in_contact`, `target_not_in_record`, `target_co_located`, and `target_missing`, which sits on two arms guarded by `debug_assert!` and is unreachable while the invariant holds) and the three patterns `damage:<u8>`, `increase:<u8>` and `transferred:<u8>`. **Rule 3.3's union does not move, and that is measured rather than argued**: every value the enumeration gains is an identifier, a fixed word or a colon-joined integer, and no character outside `A`–`Z`, `a`–`z`, `0`–`9` and `_ . - + : ; >` appears in the 1,365,884 records of the two captures `VER-MOK-012` oracle 1 took at this merge. So the stream still needs no escaping function, for the reason rule 3.3 gives and for no other. **Rules 4.4 and 4.5 do not move either**: there are still two absences and still exactly two booleans, because `target_died` is the string `yes` or `no` on `status`'s precedent rather than a JSON boolean, and no field of the new vocabulary is ever `null` in either capture; rule 4.5 gains one sentence saying so. **Rules 5.3 and 10: `schema` becomes `2`.** Rule 10.2 requires the increment when a field is added and when a value's domain in rule 3.2 gains a member, and this amendment does the first fourteen times and the second thirteen. `VER-MOK-012` recorded that whether rule 10.2's triggers are complete "cannot be verified before a second version exists" and that "the first increment will be the first real test"; this is that increment. **The engine does not conform until it is changed**: at `1e09f85` and at `e8114ad` it writes `"schema":1` on the header of a stream carrying every field this row adds. The change is `RECORD_SCHEMA_VERSION` and one asserted literal in the header unit test, and it is **not made here**, because it is a product change and this row is a specification correction the owner has not ratified. **Rule 6.5 admits a fourth shape**: a targeted proposal is `{"action":"attack","target":"M03"}`, the target nested inside the proposal object where the text line states it as a field beside `proposal`. That is what the merge emits — measured at `result.proposal.target` and at no `result` sibling of it on any `action_trace` record — and it is the one place rule 6.4's "one key per text field" does not hold, so 6.5 and 6.6 carry the exception explicitly instead of leaving a reader to find it. **Rule 6.6's walk is extended** in exactly the two places the record and its text line differ: a proposal object carrying `target` renders as two text fields rather than one `:`-joined value, and `suffered` renders as `;`-joined `attacker:damage` pairs and renders no field at all when empty. **New rule 6.9, and rule 6.8 qualified by it**: `result.suffered` is present on every `action_trace` record and empty where nothing was suffered, although the text line omits it when empty — `,"suffered":[]` on 402,475 of the two captures' 402,610 traced actions. It is the only field in the stream present where its text field is absent, and the reason is rule 4.4's: the absence of an attack is an empty record rather than a missing fact, and a consumer that had to tell "no attacks" from "not written" would be reading the writer's convenience instead of the world. **Rule 7.8's absence is unchanged and its stated reason is corrected**: at this merge the engine does compute conflict, threat responses, retreats and surrenders, so "absent for that reason" is no longer true of them; the metrics record still carries no field for any of them, and the reason is now rule 10.4's — no approved requirement needs them there, and a field whose arrival is expected is not reserved. The *reserved field* counterexample is corrected the same way and still holds. **Rule 12 does not move, measured rather than asserted**: no new type is exported to carry a record, because `suffered` rides `Vec<(String, u8)>` on the already-public `EventDetail::ActionTrace` and the engine's own `SufferedAttack` stays private; the dependency table is still empty; and rule 12.3's borrow prohibition is untouched. The three new event kinds' **field orders are not restated here**: rule 6.4 defers them to `SPEC-MOK-001`, which fixes all three, so they reach this stream through that deference. The record kinds stay **four**, rule 9.1's order is untouched, and the `run` record gains nothing — the three new kinds have no cumulative counter, so rule 8.6's equalities are unaffected. *Explicitly unspecified decisions* corrects its **Conflict, combat and social metrics** bullet: Phase 3 has approved the phenomena and they are emitted as events, and what remains unspecified is their aggregation into the metrics and run records. Two examples are added, taken verbatim from a capture rather than composed and both quoted byte for byte — the three resolution records, and an `action_trace` that carries both a nested target and an absorbed strike together with the text line it reconstructs to. The existing action trace example gains `"suffered":[]` under rule 6.9, which its text line does not gain, and the three `"schema":1` literals in the specification's examples become `"schema":2`. | **Ratified 2026-08-23** by the repository owner acting as accountable technical owner. It stood **OUTSTANDING from 2026-08-21 to 2026-08-23**, and that is recorded rather than overwritten: the row was drafted first, measured second and ratified third, which is the order that makes it a record instead of a description. Drafted by the implementation agent under `WO-MOK-019`, on the repository owner's direction of 2026-08-21 to amend this specification rather than restate the record shapes elsewhere, and on that owner's instruction that the agent drafts and the owner ratifies each correction. **It carried three things the owner had not decided**, each put separately rather than buried in the field list, and each **decided separately** in the ratifying act. **(1) The `schema` increment is ratified**: `schema` becomes `2`, and the two-line product change it obliges — `RECORD_SCHEMA_VERSION` and the one asserted literal in the header unit test, which is the whole of it as this row predicted — is made **in the same commit as this ratification**, on `SPEC-MOK-005`'s 2026-08-20 precedent, so that no commit carries one without the other. The owner declined the offered alternative of folding this increment together with the `llm` source's coming additions into a single move to `2`: this row is a specification correction describing an earlier merge, and `2` must describe the contract the tree actually had rather than one no commit ever ran. `WO-MOK-025` therefore writes `3`, which is `WO-MOK-025` stop-and-escalate condition 5 discharged — the value is one more than what this ratification leaves standing, and it is now measurable instead of guessed. **(2) The record's nesting of a targeted verb's `target` inside `proposal` is accepted as emitted**, against the offered alternative of a sibling key matching the text field for field: this specification records what the engine does, rules 6.5 and 6.6 already carry the exception explicitly rather than leaving a reader to find it, and the alternative was a product change that would have moved every traced stream in the repository. **(3) The unconditional presence of `suffered` is accepted**, against the offered alternative of omitting it when empty to match the text line, on rule 4.4's reason as this row states it: the absence of an attack is an empty record rather than a missing fact. **Decisions 2 and 3 change no byte the engine writes; decision 1 changes one integer in every header record and nothing else.** That the change is confined to that one integer is measured rather than asserted, in `evidence/WO-MOK-025/ratification/`. **The divergence this row disclosed was re-measured before the ratification, at a third commit.** The row states it at `1e09f85` and at `e8114ad`; `WO-MOK-025`'s base capture finds `"schema":1` in all **forty** sink cells at `cc5418553cb433715b7d6b15dea3886bff30ffaa`, across all twenty of `REQ-MOK-068`'s configurations in both trace modes, and finds rule 6.9's field independently — one traced cell differs from the same cell at `de33d744` in 1,451 of 3,225 records by `"suffered":[]` alone, and stripping that field makes the two streams identical. `evidence/WO-MOK-025/base/schema-divergence.txt` and `base/wo-019-comparison.txt` hold both measurements. **One consequence is stated because it is not repairable by argument**: `evidence/WO-MOK-025/base/sink-manifest.txt`, committed at `2ba15cc`, holds record-stream digests taken while `schema` was `1`, so this ratification moves every one of them. The text-stream digests beside them are untouched, because `schema` appears in no text line. Every field name in this row was measured rather than read off master's source: `VER-MOK-012` oracle 3 was run with this field set against both captures and reports PASS with no field of either stream left unnamed and, on the social capture, no drafted name left unexercised — `evidence/WO-MOK-019/merge/oracle3/drafted-social.txt` and `oracle1/value-domains.txt`. What was **not** measured is disclosed in the same evidence: neither capture rejects a targeted proposal, so the eight `detail` words are enumerated from the engine's source and from the unit tests that assert every one, and from no stream. No record bound to a commit is re-opened: `VREC-MOK-012` is verified at `50364a3`, whose stream carries none of this. |
| 2026-08-23 | **Rule 3.2's policy and source domains gain `llm`, and `schema` becomes `3`.** `ADR-MOK-007`'s *Required amendments* states this amendment; `WO-MOK-025` scope item 1 is the act. **Rule 3.2**: `config.policy` gains `llm` as its fifth member and `result.source` gains `llm` as its fifth, and they are the same string — the value an operator passes to `--policy` is the value the stream emits for the decision source, as it is for the four existing sources, which `ADR-MOK-007` decision 1 fixes and `SPEC-MOK-007` rule 18.1 states. No other row of the enumeration moves. **Rules 5.3 and 10: `schema` becomes `3`**, on rule 10.2's trigger "a value's domain in rule 3.2 gains or loses a member", which this amendment does twice. The value is measured rather than guessed: it is one more than what the 2026-08-21 row's ratification of the same date leaves standing, which is `WO-MOK-025` stop-and-escalate condition 5 discharged. **The engine conforms in the same commit as this row**, unlike the 2026-08-21 row, which deliberately left the product behind: `RECORD_SCHEMA_VERSION` becomes `3` and the one asserted literal in the header unit test follows it, which is the whole of the product change here as it was there. **The three `"schema":2` literals in this specification's examples become `"schema":3`** — the two conforming header examples and the *Security and privacy* counterexample, whose violation is the path it carries and not its version. The 2026-08-21 row's own sentence recording that those literals moved from `1` to `2` is **not** edited: it records an earlier act, and a blanket replacement that caught it was reverted. **Rule 3.3's union does not move, and it is measured rather than argued**, on the 2026-08-21 row's precedent: `llm` is three characters, all of them lowercase letters already in the admitted set, so the union of admitted characters is unchanged and the stream still needs no escaping function — for the reason rule 3.3 gives and for no other. **No record kind is added and no field is added.** The record kinds stay **four**, rule 9.1's order is untouched, rule 8.6's equalities are unaffected and rule 7's metrics record gains nothing. The transcript `SPEC-MOK-007` rule 11 fixes is a **third stream** and is not this one: it is not written to this sink, carries no record of this vocabulary, and this specification does not govern it. **Rules 4.4 and 4.5 do not move**: no absence and no boolean is added. **Rule 12 does not move**: no type is exported to carry a record. The engine's public surface does grow in the same commit — by the decision port's interface and the request type, which `SPEC-MOK-002` rule 5's row of the same date enumerates — and neither carries a record, neither is reachable from one, and rule 12.3's borrow prohibition is untouched, `SPEC-MOK-007` rule 1.3 composing the request from values for the same reason this rule forbids borrows. **One consequence is stated because it is not repairable by argument**: every record-stream digest in `evidence/WO-MOK-025/base/sink-manifest.txt` moves again. The 2026-08-23 ratification above moved all forty from the `1` they were captured at; this row moves them from `2` to `3`. The text-stream digests beside them are untouched both times, because `schema` appears in no text line. | Approved 2026-08-23 by the repository owner acting as accountable technical owner, by way of `ADR-MOK-007`, whose *Required amendments* section states each provision of this row, and which the same owner approved on the same date together with `WO-MOK-025`. The implementation agent wrote the text under `WO-MOK-025`; it did not decide the substance, and the one number it could have guessed — the `schema` value — it measured against the ratification instead, which is what that condition existed to force. **`WO-MOK-025` scope item 14 does not list this specification**, and `ADR-MOK-007` requires it; the ADR governs, and this row is written under item 1 rather than deferred to item 14, because the code in the same commit writes `3` and a commit that wrote `3` against a specification saying `2` would be the defect the 2026-08-21 row spent four sentences describing. **What is not measured here is disclosed**: no capture of an `llm` stream exists, because `REQ-MOK-072` forbids running this source without the owner's explicit permission and nothing in this commit can reach a provider. So `llm` is admitted to both domains from the engine's source and from the unit test that asserts the value reaches both, and from no stream. `VER-MOK-018`'s `S2` measures it against a scripted-stub capture when scope item 12 lands. No record bound to a commit is re-opened: `VREC-MOK-012` is verified at `50364a3`, whose stream carries none of this. |
| 2026-08-29 | **Rule 8.9 states a live run's accounting, which rule 8.2's shape carried none of.** `SPEC-MOK-007` rule 15.2 requires a run record to carry the four token totals, the cache ratio, the accumulated cost, the declared ceiling, the fallback count, the model identifier and the reasoning level. Rule 8.2 fixes this stream's run-record shape and held **none** of them, so that rule could not be satisfied by any stream this specification admitted: the two rules were written for the same object by two work orders that did not meet. A `live` object is added, present when and only when the run obtained its decisions from a connector, and absent from a replay under rule 15.6 — a replay spends nothing, and a second account of one spending event would be one account too many. `cost` and `ceiling` are integers in US cents per `SPEC-MOK-007` rule 14.2 as amended the same day, and **`cache_ratio` is a pair of integers rather than a decimal**, because rule 12.4's prohibition on a floating-point value in this stream holds here as everywhere and `0.85` would be a formatted decimal whose bytes vary by platform. The six were found by a **conformance pass** the owner directed on 2026-08-29, after `WO-MOK-028` and `WO-MOK-029` had each been authored, approved, evidenced, transitioned and pushed to close one gap apiece and a third and fourth appeared immediately after. The pass read the specifications against what stage 5b's remaining items need rather than waiting for the next stop, and is retained at `../evidence/WO-MOK-026/conformance-pass.md`. **What this row does not do**: rules 8.1 to 8.8 keep their text, no other record kind moves, rule 8.7's prohibition on classification is untouched and the `live` object carries none, and no figure changes. | **Approved 2026-08-29 by the repository owner acting as accountable technical owner**, in four decisions taken in the turn each question was asked: the US cent as the minor unit; `--prices` as a compact option rather than a file; a retry bound of three; and the provider binding staying in the connector with the response reporting it back, over the two alternatives of telling the engine or leaving the request's fields advisory. A fifth decision routed the work into one chain rather than four. The implementation agent ran the pass, measured every figure and wrote the text; it decided none of the substance. |

## Scope

This specification fixes the exact behavior of the engine's structured record stream: the option that configures a
sink, the stream's framing and encoding, the four record kinds and every field of each, the closed value alphabet
that makes escaping total, the schema version, the cumulative state the engine must begin retaining, the division of
labor between the library target and the binary target, and the failure behavior.

It specifies `REQ-MOK-042` through `REQ-MOK-046` and nothing else.

It does not restate the text stream. `SPEC-MOK-001` remains the sole authority for the text record's vocabulary,
field order, rendering and ordering, and this specification refers to that authority rather than copying it: the
structured stream is defined as a *projection* of the text stream, so every text-format question has exactly one
answer and it is `SPEC-MOK-001`'s. Five provisions of `SPEC-MOK-001`, one of `SPEC-MOK-002` and one of `ARCH-MOK-001`
require amendment for this specification to be consistent with them; the amendment text is stated in full in
`ADR-MOK-005` and is not restated here.

It specifies nothing about the terminal observer. `SPEC-MOK-003` rule 9.4's export keeps the text format and this
specification does not reach it. It specifies nothing about batch execution across seeds, run persistence beyond one
stream per run, or outcome classification; all three are unauthorized.

Throughout, *the sink* means the destination of the structured record stream, *the text stream* means the
standard-output stream `SPEC-MOK-001` fixes, and *a text line* means one line of that stream, whether an event record
or the terminal summary line.

## Actors and external systems

- **The engine's library target** produces records and writes them to a `Write` sink its caller supplies. It resolves
  no path, opens no file, creates no directory and removes no file. It is the sole author of every record's content.
- **The engine's binary target** parses the option, resolves the path, creates and truncates the file, hands the
  library a buffered writer over it, flushes and closes it, and removes it on failure. It authors no record content.
- **A consumer** is any program that reads a retained stream. It is not part of this repository, and no consumer is
  specified. The stream is specified so that writing one requires no knowledge of the engine's rules.
- **The terminal observer** is unaffected and is not a sink.
- There is no external system. No network destination, no database, no service, and no environment variable
  participates.

## Inputs

One new command-line option, and no change to any existing one.

`--events-path <path>` — the destination of the structured record stream. Absent by default. May appear at most once,
in any order relative to other options, subject to the same rules every existing option follows: a missing value and
a value beginning with `--` are both a missing value.

The option's value is rejected as an invalid configuration, before the run, when it is the empty string or the single
character `-`. Both spellings conventionally denote a standard stream, and a sink that interleaves with the text
stream cannot satisfy `REQ-MOK-045`; rejecting the spelling is cheaper and clearer than defining behavior for it.

No other constraint is placed on the value's form. It is passed to the platform as a path and is never interpreted as
anything else — not as a format string, not as a shell word, not as an option, and not as engine input. Whether the
platform accepts it is a runtime matter, and rule 13.2 governs it.

The usage text gains a corresponding entry, and `SPEC-MOK-001`'s *Help output* section is amended accordingly. The
option's presence and default are held equal between the usage text and the parser by the same test that already
holds every other option's, so neither can move alone.

## Outputs

One stream, written to the sink, present only when the sink option is given. Nothing else changes: the text stream,
the diagnostic stream, the exit codes and every default are what they were.

The sink stream is a sequence of lines. Each line is one JSON object and each is terminated by a single line feed
(U+000A), including the last. The stream is UTF-8, contains no byte-order mark, contains no carriage return, and
contains no blank line. The encoding is fixed by this specification rather than taken from the platform, for the
reason `SPEC-MOK-001` gives for the usage text's line endings: a stream whose bytes depend on how the repository was
checked out or on which platform ran it is not a byte-identical stream.

## State model

The engine gains cumulative counters. They are private state of the simulation, are reachable through no public item
this specification does not name, and exist whether or not a sink is configured — a counter that existed only under
an option would make the option a behavior change.

- **Territory crossings.** One `u64` counting every crossing over the run.
- **Consumption.** Three `u64` counters, one per resource class, counting every consumption over the run.
- **Regeneration.** One `u64` counting every resource the run regenerated.
- **Skipped regeneration.** Two `u64` counters, one per skip reason, counting every skipped regeneration.
- **Death tick.** One `Option<u64>` per Mokiterion, absent until it dies and thereafter the tick at which it died.

Each counter is incremented at exactly the point its corresponding event is emitted, in the same statement sequence,
so that a counter and the event stream cannot disagree. No counter is derived from a draw against the entropy stream,
no counter participates in any rule, decision, proposal, validation or applied action, and no counter reaches the
text stream. A counter saturates rather than wrapping; `u64` cannot be exhausted by any run the tick limit admits, and
saturating arithmetic makes that a stated property rather than an assumption.

No other state is added. Every figure the metrics record states is computed from state the engine already holds, at
the instant the record is written.

## Behavioral rules

### 1. The sink

1.1 A sink is configured when, and only when, `--events-path` is given. Absent it, no record is produced, no
sink-related code path runs, and the run is indistinguishable from the same run in a build without this capability.

1.2 The library target accepts a sink as a `Write` implementation and writes records to it. It performs no filesystem
operation of any kind. This is `INT-MOK-009` principle 7 and it is what keeps `SPEC-MOK-001`'s prohibition on
interpreting input as a filesystem path true of the library.

1.3 The binary target resolves the path, creates the file, truncating an existing file at that destination without
prompting, and supplies a buffered writer over it. Truncation matches `SPEC-MOK-003` rule 9.4, which is this
repository's only other file-writing behavior; a differing rule for the engine would be surprising and would have to
be justified rather than merely stated.

1.4 The binary target flushes and closes the sink before the process reports success. A record buffered and never
flushed was not written, so a flush failure is a write failure under rule 13.

1.5 Exactly one sink per run. There is no second sink, no sink rotation, no size limit, no truncation of the stream
itself, and no directory convention.

### 2. Framing

2.1 One record per line. One JSON object per record. No record spans lines and no line holds two records.

2.2 Every record carries a `record` field, first, whose value is one of exactly four words: `header`, `event`,
`metrics`, `run`. A consumer discriminates on that field alone and needs no other information to know a record's
shape.

2.3 Field order within a record is fixed by this specification and is part of the byte-identical stream. A consumer
is not required to depend on it — JSON objects are unordered — but the writer is required to produce it, because
`REQ-MOK-045` makes the stream's bytes a property.

2.4 A record contains no field this specification does not name. There is no extension field, no free-form
annotation, no comment and no padding.

2.5 The four record kinds appear in one order per run, fixed by rule 9.

### 3. The value alphabet

3.1 Every string value in the stream is drawn from the closed set enumerated in rule 3.2. No string value in the
stream is supplied by the operator, read from the environment, or derived from a path.

3.2 The enumeration, exhaustive at this schema version:

| Field | Domain |
|---|---|
| `record` | `header`, `event`, `metrics`, `run` |
| `engine` | the package version, a compile-time constant of this repository, matching `[0-9A-Za-z.+-]+` |
| `config.policy` | `baseline`, `reference`, `individual`, `social`, `llm` |
| `config.density` | `[0-9]+\.[0-9]{2}` |
| `subject` | `world`, `A`, `B`, `M[0-9]{2}`, `F[0-9]{4}` |
| `event` | the fifteen event types `SPEC-MOK-001` fixes, in their existing snake_case spellings |
| `result.name` | one of the twelve fixed Mokiterion names, letters only |
| `result.territory`, `result.from`, `result.to`, territory keys, `agents[].territory` | `A`, `B` |
| `result.class`, consumption keys | `low`, `medium`, `high` |
| `result.source` | `baseline`, `reference`, `individual`, `social`, `llm` |
| `result.reason` (regeneration skipped), skip keys | `depleted`, `capacity` |
| `result.reason` (simulation ended), `run.reason` | `tick_limit`, `extinction` |
| `result.status` | `accepted`, `rejected` |
| `result.target`, `result.recipient`, `result.proposal.target`, `result.suffered[].attacker` | `M[0-9]{2}` |
| `result.target_died` | `yes`, `no` |
| `result.proposal.action` | `wait`, `sleep`, `eat`, `move`, `attack`, `threaten`, `fight`, `retreat`, `surrender`, `approach`, `avoid` |
| `result.proposal.direction` | the eight fixed direction words, in their existing snake_case spellings |
| `result.proposal.food`, `result.food` | `F[0-9]{4}` |
| `result.detail` | one of `agent_dead`, `waited`, `energy_full`, `out_of_bounds`, `food_unavailable`, `target_unknown`, `target_dead`, `target_is_actor`, `target_not_perceived`, `target_not_in_contact`, `target_not_in_record`, `target_co_located`, `target_missing`, `energy:<u8>-><u8>`, `position:<u8>:<u8>`, `damage:<u8>`, `increase:<u8>`, `transferred:<u8>`, `food:F[0-9]{4};class:(low|medium|high)` |
| `agents[].id` | `M[0-9]{2}` |
| `agents[].name` | one of the twelve fixed names, letters only |

3.3 The union of characters that set admits is `A`–`Z`, `a`–`z`, `0`–`9`, and `_ . - + : ; >`. It contains no
quotation mark, no reverse solidus, and no code point below U+0020. Therefore a writer that emits any of these values
between quotation marks, unaltered, produces valid JSON for every value the engine can produce. **This is why the
stream needs no escaping function, and it is the only reason.** The engine has no serialization library, cannot
acquire one under `ARCH-MOK-001`, and a hand-written escaper would be a correctness obligation with no compiler and
no library behind it. Closing the alphabet discharges the obligation instead of implementing it.

3.4 Rule 3.3 is a property of the code, not a hope about it. It is verified exhaustively over the enumeration in rule
3.2, and a string field added to the stream at any future schema version must either be added to that enumeration or
arrive together with an escaping function and its own verification. A field whose value could be operator-supplied,
environment-derived, or free text may not be added under rule 3.3.

3.5 `result.detail` is carried verbatim, as the engine holds it, rather than decomposed. `result.proposal` is
decomposed, because the engine holds it as a typed value. The rule is general: **a field is decomposed where the
engine holds it structured and carried verbatim where the engine holds it as rendered text.** The record therefore
adds no interpretation of its own, and the one field that is still text is text in the engine too. That `detail` is
compound text rather than a typed value is a limitation of the engine's action result, not of this stream; typing it
is out of scope here and is recorded as a residual in *Explicitly unspecified decisions*.

### 4. Numbers and absence

4.1 Every numeric value is a JSON integer: an optional `-`, then digits, with no decimal point, no exponent, no
leading zero except for `0` itself, and no `+`. No value in the stream is a JSON number that is not an integer.

4.2 No mean, average, ratio, percentage, rate, delta or trend appears anywhere in the stream. Where an average is
wanted, the record carries a sum and, in the same record, the count that divides it.

4.3 `config.density` is a string, not a number, and it is exactly the two-decimal rendering the engine's `Display`
already produces and the operator already types. It is a string because rule 4.1 forbids the only number that would
represent it faithfully; it is not converted to hundredths because the operator-facing form is the useful one and the
resolved integer the density implies is already stated, per territory, as `capacity` in every metrics record.

4.4 An absent value is JSON `null`. It is never `0`, never `-1`, never omitted and never an empty string. Two absences
exist at this schema version: an extremum over an empty living population (rule 7.5) and a survivor's death tick
(rule 8.4). Both are cases where a sentinel would collide with a legitimate value — zero health is a living
Mokiterion, and tick `0` is a legitimate death tick — so `null` is a correctness requirement rather than a style.

4.5 `true` and `false` are JSON booleans. There are two boolean fields at this schema version:
`config.trace_actions` and a territory's `depleted`.

`target_died` is not a third. Rule 22's verdict is the string `yes` or `no`, exactly as `status` is the string
`accepted` or `rejected`, because that is what the text field carries and rule 6.6 requires the record to reproduce
it — and because a verdict that later gains a third outcome must not have to change a field's type to say so.

### 5. The header record

5.1 Exactly one header record per run, first in the stream, written before the first tick and before any other
record.

5.2 Its shape:

```json
{"record":"header","schema":3,"engine":"0.1.0","config":{"seed":0,"ticks":200,"policy":"reference","density":"0.75","trace_actions":false}}
```

5.3 `schema` is the stream's schema version, an integer, `3` at this specification. Rule 10 governs it; the 2026-08-21
amendment record states what moved it from `1`, and the 2026-08-23 row what moved it from `2`.

5.4 `engine` is the engine package's version, taken from the package metadata at compile time.

5.5 `config` states the *resolved* configuration — every value the run actually used, including each default the
operator did not supply — so that a retained stream is interpretable without the command line that produced it, and
so that a default's later change cannot silently reinterpret an old capture. It carries the sink's path in no form:
the path is an operator-supplied string, rule 3.4 forbids it, and it is not a fact about the simulation.

5.6 The header carries no wall-clock time, no duration, no hostname, no user, no process identifier, no working
directory, no environment value and no credential. This is what makes two runs at one seed byte-identical, and it is
also what keeps a retained capture safe to attach to a work order as evidence.

### 6. The event record

6.1 Exactly one event record per emitted text event line, in the same order, whatever the tracing setting. This is
`REQ-MOK-042`'s correspondence, and rule 9.3 states it as a completeness property over the whole stream.

6.2 Its shape:

```json
{"record":"event","tick":0,"subject":"M01","event":"agent_initialized","result":{"name":"Zug","position":{"x":89,"y":34},"territory":"A","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":6}}
```

6.3 `tick`, `subject` and `event` are the same three values the text line's `tick=`, `subject=` and `event=` fields
carry, unaltered.

6.4 `result` is an object whose keys are the text result's keys, **in the text result's order**, one key per text
field, for the event type in question. The event types and their field orders are `SPEC-MOK-001`'s and are not
restated here; adding, removing, renaming or reordering a field is an amendment to `SPEC-MOK-001` and reaches this
stream through rule 6.4 automatically.

6.5 Four value shapes differ from the text rendering, and only these four:

- A coordinate, rendered `x:y` in text, is the object `{"x":<int>,"y":<int>}`.
- A before-and-after pair, rendered `before->after` in text, is the object `{"from":<int>,"to":<int>}`.
- A proposed action, rendered `wait`, `sleep`, `eat:<food>` or `move:<direction>` in text, is the object
  `{"action":"wait"}`, `{"action":"sleep"}`, `{"action":"eat","food":"F0001"}` or
  `{"action":"move","direction":"north"}`.
- A proposed **targeted** action, one of `CAP-MOK-010`'s seven verbs, is the object
  `{"action":"attack","target":"M03"}` — the target inside the proposal object, under the name that value has,
  exactly as `eat` carries `food`. **This is the one place a record nests what the text line states beside it.**
  `SPEC-MOK-001` renders the target as a field of its own immediately after `proposal`, by the technical owner's
  decision of 2026-08-20, so a text line carries two fields where the record carries one object; rule 6.4's
  key-for-key correspondence does not hold here and rule 6.6's walk maps the difference. The two cannot disagree,
  because both are read from one `Action` in one arm of the writer.

Every other value is the same scalar the text carries, as an integer where it is an integer and as a string where
rule 3.2 lists it as a string. The action trace's `accepted` boolean is a string, `accepted` or `rejected`, because
that is what the text field `status:` carries and rule 6.6 requires the record to reproduce it. `target_died` is a
string for the same reason, under rule 4.5.

6.6 A text event line is reconstructible from its event record by a walk that needs no per-event-type knowledge:
render `tick=`, `subject=`, `event=` and `result=`, then the result's keys in order joined by `,`, each as
`key:value`, with a coordinate rendered `x:y`, a pair rendered `from->to`, and a proposed action rendered by its
`action` word optionally followed by `:` and its one remaining value. Two further cases complete the walk, and both
belong to the two places the record's shape is not the text line's:

- A proposal object carrying a `target` renders as **two** text fields, `proposal:<verb>` and then `target:<id>`, not
  as one `:`-joined value. `eat` and `move` keep the `:` form they have always had.
- `suffered` renders as its entries' `attacker:damage` pairs joined by `;`, in the array's own order, and an **empty
  array renders no text field at all**. This is the one place the walk drops a record field rather than rewriting it,
  and rule 6.9 is why.

The walk runs in one direction, from record to text line, and each case above is a mapping rather than an
equivalence. Rule 6.5's four shapes plus these two cases are exactly what it needs, which is why there are six and
not more.

6.7 An event whose text result carries no field carries `"result":{}`. It is not omitted and it is given no invented
field. No such event type exists at this schema version; the rule fixes the answer in advance so that adding one is
not a design question.

6.8 An event record carries no value the text line does not carry, with the single exception rule 6.9 states. In
particular it does not enrich an event with state read elsewhere in the tick, because a record and its text line must
state the same facts about the same instant.

6.9 `result.suffered` is present on every `action_trace` record, as an array, and is `[]` where the subject absorbed
nothing. The text line omits the field when the record is empty and this stream does not, and the asymmetry is
required rather than tolerated: it is rule 4.4's reasoning applied to a collection. An absence is not a sentinel and
not an omission — a consumer that had to distinguish "no attacks were absorbed" from "the writer had nothing to say"
would be reading the writer's convenience instead of the world, and it could not tell an empty window from a stream
written before the window existed. `SPEC-MOK-001` states the text line's own reason for omitting it: appending the
field unconditionally there would change every `action_trace` line of every `baseline` run, which `CAP-MOK-010` holds
byte-identical. The two rules therefore differ deliberately and neither is a defect in the other.

Each entry is the object `{"attacker":"M[0-9]{2}","damage":<u8>}`, and the array carries the entries in the order the
attacks resolved. **It is not capped at one.** One entry per non-empty array is what every capture taken so far
produces, because no measured window has been struck into twice; a window that two attackers reach carries two, and
nothing in this specification limits the length.

### 7. The metrics record

7.1 Exactly one metrics record per completed tick, after every event record of that tick and before the first event
record of the next. A tick that terminates the run is a completed tick and carries its metrics record; the record
follows that tick's `simulation_ended` event.

7.2 Its shape:

```json
{"record":"metrics","tick":1,"living":12,"deaths":0,"population":{"A":6,"B":6},"health":{"sum":1200,"min":100},"satiety":{"sum":1188,"min":99},"energy":{"sum":1188,"min":99},"fear":{"sum":0,"max":0},"territories":{"A":{"standing":61,"low":20,"medium":20,"high":21,"capacity":61,"depleted":false},"B":{"standing":61,"low":21,"medium":20,"high":20,"capacity":61,"depleted":false}}}
```

7.3 Every figure describes the state at the end of the tick the record names. No figure in one record is read at a
different point in the tick from any other.

7.4 `living` is the living population and `deaths` is the cumulative death count; the two sum to the roster size at
every tick. `population` states the living population per territory and sums to `living`. `living` is the divisor for
every sum in the same record, so an average needs no other record.

7.5 Each of the four dynamic attributes carries a `sum` over the living population and one extremum: `min` for
`health`, `satiety` and `energy`, whose depletion threatens survival, and `max` for `fear`, whose accumulation is the
attribute's own direction of harm. The choice per attribute is fixed here so that a consumer comparing runs compares
the same figure. When `living` is `0`, every `sum` is `0` and every extremum is `null` under rule 4.4.

7.6 Each territory states `standing`, its total standing resource count; `low`, `medium` and `high`, its standing
count by class, which sum to `standing`; `capacity`, the count the run's density resolves to, which does not vary
within a run and is stated every tick so that a single record is interpretable alone; and `depleted`, whether the
territory is permanently depleted. `standing` may be `0` while `depleted` is `false` only if the two can diverge in
the engine's own rules; where the engine derives permanent depletion from a zero standing count, the two agree, and
the record states both regardless, because a consumer must not have to know which derivation the engine uses.

7.7 The record states no rate, no delta and no trend. A change between ticks is the consumer's subtraction.

7.8 The record carries no field for a phenomenon the engine does not compute, and does not carry such a field at zero.
`SPEC-MOK-003` rule 4.5 established this repository's precedent by refusing to render a gauge for an attribute nothing
yet consumed; a field fixed at zero reads to a consumer as a measurement.

Conflict frequency, threat responses, retreats and surrenders are absent from this record, and **they are no longer
absent for that reason**. The engine computes all four: `CAP-MOK-010`'s rules resolve them and the stream reports each
as an event. What is absent is their *aggregation* — no approved requirement asks the metrics record to count them, and
rule 10.4 forbids reserving a field for a figure nobody has approved. A consumer that wants a per-tick conflict count
derives it from the event records of that tick, exactly as it derives a change between ticks by subtraction under rule
7.7.

7.9 Computing a metrics record reads authoritative state and draws no entropy. It exposes no borrow of that state:
`SPEC-MOK-002` rule 6 is not relaxed by anything in this rule group.

### 8. The run record

8.1 Exactly one run record per run, last in the stream, after the final tick's metrics record.

8.2 Its shape:

```json
{"record":"run","reason":"tick_limit","ticks":200,"survivors":12,"deaths":0,"crossings":4,"consumed":{"low":31,"medium":18,"high":2},"regenerated":38,"regeneration_skipped":{"depleted":0,"capacity":1},"final":{"territories":{"A":{"population":6,"low":20,"medium":20,"high":21},"B":{"population":6,"low":21,"medium":20,"high":20}}},"agents":[{"id":"M01","name":"Zug","territory":"A","died_at":null}]}
```

8.3 `reason`, `ticks`, `survivors`, `deaths` and the `final` object carry exactly the twelve figures the text summary
line carries, so that the summary line is reconstructible from this record. `crossings`, `consumed`, `regenerated`,
`regeneration_skipped` and `agents` are the facts no text line states; they come from the cumulative counters the
*State model* adds.

8.4 `agents` holds one entry per Mokiterion the run created, living or dead, in ascending identifier order. Each
carries `id`, `name`, the `territory` it stood in at termination or at death, and `died_at`, the tick at which it
died, `null` for a survivor under rule 4.4. Ordering is fixed here rather than left to the roster's iteration order,
because an ordering that came from a collection's traversal is a determinism defect waiting to manifest.

8.5 `regeneration_skipped` distinguishes the two skip reasons. Collapsing them would lose the difference between a
world at capacity and a world that can never restock, which is the difference between a healthy run and a dead one.

8.6 Every cumulative figure equals the number of corresponding event records in the same stream, and `survivors` and
`deaths` equal the final metrics record's `living` and `deaths`. This consistency is a property of the stream, so a
disagreement is a defect the stream itself reveals.

8.7 The run record states no outcome classification, label, category, verdict, severity or interpretation, and
neither does any other record. `REQ-MOK-044` states the obligation; this rule states that it holds stream-wide, so a
classification cannot arrive by being placed in a different record kind. Classification is Phase 4b's, and a
threshold must be revisable without invalidating a retained capture.

8.8 No duration, wall-clock time or elapsed measurement appears. Survival time is measured in ticks, which is the
only clock the simulation has.

8.9 **A live run's record carries `SPEC-MOK-007` rule 15.2's accounting, as amended 2026-08-29 under
`WO-MOK-030`.** A `live` object is present when, and only when, the run obtained its decisions from a connector:
`tokens` with `prompt`, `cached_prompt`, `output` and `reasoning`; `cache_ratio`; `cost` and `ceiling`, both
integers in US cents per that rule 14.2; `fallbacks`; `model` and `reasoning_level` as the connector reported
them; and `fit`, false when `fallbacks` exceeds zero. A replay carries no `live` object at all — rule 15.6, and
the reason is that a replay spends nothing and a second account of one spending event would be one account too
many.

`cache_ratio` is a pair of integers, `cached` over `total`, and not a decimal — rule 12.4's prohibition on a
floating-point value in this stream holds here as everywhere, and a ratio rendered as `0.85` would be a formatted
decimal whose bytes vary by platform. A reader divides.

Until this amendment rule 8.2's shape carried none of the seven figures rule 15.2 requires of a run record, so
that rule could not be satisfied by any stream this specification admitted. The two rules were written for the
same object by two work orders that did not meet.

8.9 No run record is written for a run that terminated because a write failed. Rule 13.4 governs that case.

### 9. Ordering and completeness

9.1 The stream's record order is: the header; then, per tick in ascending order, that tick's event records in the
order `SPEC-MOK-001` fixes followed by that tick's metrics record; then the run record. Tick `0`'s initialization
events precede tick `1`; tick `0` has no metrics record, because no tick has completed.

9.2 No tick is skipped, none is repeated, and no record appears out of this order.

9.3 The correspondence between the two streams is total in both directions, over the whole stream: every text line
has exactly one structured record, and every structured record that has a text counterpart has exactly one. Event
lines correspond to event records; the summary line corresponds to the run record. The header and the metrics records
are the only records with no text counterpart, and they are the only two kinds this specification permits to have
none.

9.4 Two runs with the same seed, options and sink configuration produce byte-identical sink streams. This follows
from rules 2.3, 3.1, 4.1, 5.6, 8.4 and 9.1 together, and is verified rather than derived.

### 10. Schema version and compatibility

10.1 `schema` is an integer, present in the header record, and therefore in every retained stream from its first
line. It is declared a compatibility surface of this product.

10.2 `schema` is incremented when a record kind is added or removed, a field is added, removed or renamed, a field's
type changes, a value's domain in rule 3.2 gains or loses a member, or the ordering rule 9.1 changes. It is not
incremented by a change that cannot alter any byte any conforming writer produces.

10.3 A consumer that does not recognize a `schema` value must refuse the stream rather than interpret it. This
specification states the obligation on the consumer's behalf; it specifies no consumer.

10.4 Fields whose arrival is expected are not reserved, stubbed, or emitted as null or zero in advance. Phase 3's
conflict metrics and Phase 4b's needs arrive with a version increment when an approved requirement needs them. A
reserved field is a claim about a design nobody has approved.

10.5 The stream is versioned independently of the engine package version. `engine` identifies the producer; `schema`
identifies the contract. An engine release that does not change the contract does not change `schema`.

### 11. Non-perturbation

11.1 With a sink configured, the bytes written to the text stream are identical to the bytes the same run writes with
no sink configured. There is no tolerance, no whitespace exemption, and no exception for any option combination.

11.2 With a sink configured, the run draws the same values from the shared entropy stream, in the same order, at the
same points in the same ticks. No record-writing path draws, and no counter is derived from a draw.

11.3 The exit code, the diagnostic stream and every simulation outcome are the same with a sink and without one,
excepting only what rule 13 adds on failure.

11.4 The cumulative counters the *State model* adds alter no rule, no decision, no proposal, no validation, no
applied action and no text record. Every run recorded before they existed reproduces byte for byte.

11.5 No unordered collection is iterated where the order reaches a record. Rule 8.4's explicit ordering is the one
place a traversal order would otherwise have been visible.

11.6 A run may take longer with a sink configured. Time is not a property of this specification.

### 12. Interface growth

12.1 `execute` gains one parameter: an optional sink. This grows the public interface `SPEC-MOK-002` rule 5
enumerates, under that rule's growth clause, because `REQ-MOK-042` cannot be satisfied without it.

12.2 The interface grows by nothing else that this specification does not require. `Event`, `EventDetail`,
`EventType`, `Coordinate`, `Territory`, `FoodClass`, `Action`, `Direction`, `TerminationReason`,
`RegenerationSkipReason` and `Density`'s rendering are already public and already carry every value an event record
needs; no new type is exported to carry a record, and no record type is public unless a relocated test requires it
under `SPEC-MOK-002`'s test-placement rules.

That still holds of the fifteen-kind vocabulary, and it holds by measurement rather than by assertion. `Action`,
`EventDetail` and `EventType` gain variants under `CAP-MOK-010` and `SPEC-MOK-002`'s own enumerated growth, which is
that specification's business and not this one's, and **this specification exports nothing to carry the additions**:
rule 6.9's window rides `Vec<(String, u8)>` on the already-public `EventDetail::ActionTrace`, both halves of a pair
being already-public values, and the engine's own `SufferedAttack` stays private.

12.3 `SPEC-MOK-002` rule 6 is not relaxed. Nothing this specification adds yields a mutable borrow of, or a reference
into, the world grid, the agent collection, the resource collection, the tick counter, the entropy state, the event
log or any counter the *State model* adds, in any build configuration including test builds.

12.4 The engine package's dependency table stays empty. No serialization library, no JSON library, and no dependency
shared with `mokiterions-tui`. `ARCH-MOK-001` admits no exception and this specification asks for none.

### 13. Error and recovery behavior

13.1 A malformed sink argument — absent, beginning with `--`, empty, or the single character `-` — is an invalid
configuration. The engine reports it on the diagnostic stream, writes the usage text there, exits with the
configuration error code `2`, and runs nothing. This is the existing treatment of every malformed option and is not
changed by this specification.

13.2 A well-formed path the platform refuses — a missing directory, a permission denial, a destination that cannot be
created — is a runtime failure. The engine reports the failure and its reason on the diagnostic stream, exits with
the runtime failure code `1`, runs no tick, and writes no text observation record. A run that cannot be recorded is
not run, because a partly-recorded run is what `REQ-MOK-046` exists to prevent.

13.3 A failed write to an opened sink, including a failed flush or close, is a runtime failure. The engine reports the
failure and its reason on the diagnostic stream, exits with the runtime failure code `1`, and claims no successful
completion. The write is not retried, the failure is not suppressed, and no further record is written.

13.4 On a write failure the binary target removes the file it created, so that no partial stream survives to be read
as a complete run. Removal is limited to a destination this process created; where that cannot be established the
process does not remove it and says so. A failure of the removal is reported in addition to the original failure,
never instead of it, and does not change the exit code. The library target cannot perform this removal and does not
attempt to; the obligation is the host's, exactly as it already is in `mokiterions-tui`.

13.5 A diagnostic message for a sink failure is distinguishable from one for a text-stream failure, is deterministic
in form, and carries no value beyond the sink's identity and the platform's reason.

13.6 This specification introduces no new exit code and changes the meaning of none.

13.7 A text-stream write failure remains exactly what `SPEC-MOK-001` makes it. Where both streams fail, both are
reported if possible and the exit code is `1` either way, so no precedence question arises.

## Data and interface contracts

- The sink is a `Write`. The library target's contract with its host is that it writes bytes and returns the host's
  error; it makes no assumption about the destination's nature, seekability, or size.
- Every record originates at the single point through which every authoritative event already passes, so the
  one-to-one correspondence of rule 6.1 is structural rather than maintained. A second emission point would make
  rule 9.3 a thing to keep true; there is one, and adding one is a defect.
- No public item returns a record type by reference, and no record is retained after it is written. The stream is
  write-only and the engine reads nothing back from it.
- `schema` is the only compatibility surface this specification introduces. Field order is a byte-level property
  under rule 2.3 and is not a contract a consumer may rely on semantically.

## Security and privacy properties

- No record carries a credential, a token, a secret, a hostname, a user name, an environment value, a working
  directory, a process identifier, an absolute or relative path, or a wall-clock time. Rules 3.2, 5.5 and 5.6 make
  this a property of the enumerated field set rather than a review obligation.
- The operator-supplied path reaches the platform's file API and nothing else. It is never rendered into a record,
  never interpreted as a format string, never interpreted as an option, and never interpreted as engine input.
- The engine performs no network operation, opens no socket, and reads no environment variable. Its dependency table
  is empty, so no dependency can do so on its behalf.
- A retained stream is therefore safe to attach to a work order as evidence without redaction, which is the property
  that makes this stream useful to assurance at all.
- The binary target creates a file at an operator-chosen destination and may truncate an existing file there. That is
  the operator's instruction, matches `SPEC-MOK-003` rule 9.4, and is stated in the usage text.

## Performance and capacity

- The sink is written through a buffered writer, one record per line, without retaining records in memory. Memory use
  is independent of run length.
- Stream size grows linearly with the run: one metrics record per tick, one event record per text line. A long run
  with tracing enabled produces a large stream; no limit, rotation or sampling is imposed, because a sampled stream
  would silently misreport, and `REQ-MOK-043`'s per-tick obligation is what makes a run's shape recoverable.
- No performance target is set. Rule 11.6 states that time is not a property here.

## Observability

- The stream is itself the observability surface this specification adds. Nothing observes the stream.
- A sink failure is visible on the diagnostic stream under rule 13.5 and in the exit code under rules 13.2 and 13.3.
- The absence of a run record in a retained stream indicates a stream that was not completed; rule 13.4 means such a
  stream should not exist where the process created the file, and rule 8.9 means the record is never written for a
  failed run.

## Compatibility and migration

- Every existing behavior is preserved. No default changes, no existing option changes, no exit code changes, and the
  text stream's bytes are unchanged under rule 11.1.
- No migration is required. A build without this capability and a build with it, invoked without the sink option,
  produce identical output.
- Every capture retained before this change reproduces byte for byte under rule 11.4.
- `SPEC-MOK-003` and the terminal observer are untouched. The observer's export remains the text format.
- Forward compatibility is `schema`'s, under rule 10.

## Examples and counterexamples

### Example: a run's stream, abbreviated

```json
{"record":"header","schema":3,"engine":"0.1.0","config":{"seed":0,"ticks":2,"policy":"reference","density":"0.75","trace_actions":false}}
{"record":"event","tick":0,"subject":"world","event":"world_initialized","result":{"width":128,"height":128,"territories":2}}
{"record":"event","tick":0,"subject":"F0001","event":"food_initialized","result":{"class":"low","position":{"x":12,"y":7},"territory":"A"}}
{"record":"event","tick":0,"subject":"M01","event":"agent_initialized","result":{"name":"Zug","position":{"x":89,"y":34},"territory":"B","health":100,"satiety":100,"energy":100,"fear":0,"waste_tolerance":6}}
{"record":"event","tick":0,"subject":"world","event":"decision_source_selected","result":{"source":"reference"}}
{"record":"event","tick":1,"subject":"M01","event":"survival_changed","result":{"health":{"from":100,"to":100},"satiety":{"from":100,"to":99},"energy":{"from":100,"to":99},"fear":{"from":0,"to":0}}}
{"record":"metrics","tick":1,"living":12,"deaths":0,"population":{"A":6,"B":6},"health":{"sum":1200,"min":100},"satiety":{"sum":1188,"min":99},"energy":{"sum":1188,"min":99},"fear":{"sum":0,"max":0},"territories":{"A":{"standing":61,"low":20,"medium":20,"high":21,"capacity":61,"depleted":false},"B":{"standing":61,"low":21,"medium":20,"high":20,"capacity":61,"depleted":false}}}
{"record":"event","tick":2,"subject":"world","event":"simulation_ended","result":{"reason":"tick_limit"}}
{"record":"metrics","tick":2,"living":12,"deaths":0,"population":{"A":6,"B":6},"health":{"sum":1200,"min":100},"satiety":{"sum":1176,"min":98},"energy":{"sum":1176,"min":98},"fear":{"sum":0,"max":0},"territories":{"A":{"standing":61,"low":20,"medium":20,"high":21,"capacity":61,"depleted":false},"B":{"standing":61,"low":21,"medium":20,"high":20,"capacity":61,"depleted":false}}}
{"record":"run","reason":"tick_limit","ticks":2,"survivors":12,"deaths":0,"crossings":0,"consumed":{"low":0,"medium":0,"high":0},"regenerated":0,"regeneration_skipped":{"depleted":0,"capacity":0},"final":{"territories":{"A":{"population":6,"low":20,"medium":20,"high":21},"B":{"population":6,"low":21,"medium":20,"high":20}}},"agents":[{"id":"M01","name":"Zug","territory":"B","died_at":null}]}
```

### Example: an action trace record

```json
{"record":"event","tick":3,"subject":"M04","event":"action_trace","result":{"proposal":{"action":"move","direction":"north_east"},"status":"accepted","detail":"position:41:63","position":{"x":41,"y":63},"territory":"A","health":100,"satiety":97,"energy":97,"fear":0,"suffered":[]}}
```

Its text line reconstructs, by rule 6.6, as
`tick=3 subject=M04 event=action_trace result=proposal:move:north_east,status:accepted,detail:position:41:63,position:41:63,territory:A,health:100,satiety:97,energy:97,fear:0`.

The trailing `"suffered":[]` is rule 6.9's, and this is the record that shows what rule 6.8's exception costs: the
text line carries no `suffered` field, so the reconstruction above ends at `fear:0` and the record still ends at the
empty list. Nothing else about the example moves.

### Example: the three resolution records

```json
{"record":"event","tick":9,"subject":"M05","event":"attack_resolved","result":{"target":"M02","damage":29,"target_health":{"from":100,"to":71},"striker_energy":{"from":92,"to":87},"target_died":"no"}}
{"record":"event","tick":10,"subject":"M02","event":"surrender_resolved","result":{"recipient":"M05","transferred":9,"discarded":36,"subject_satiety":{"from":91,"to":46},"recipient_satiety":{"from":91,"to":100}}}
{"record":"event","tick":11,"subject":"M05","event":"threat_resolved","result":{"target":"M02","increase":0,"target_fear":{"from":100,"to":100}}}
```

Three lines from one stream, not composed: the first occurrence of each of the three kinds in a capture at the fourth
policy, quoted byte for byte and in the order that stream carries them, which is why they are not adjacent in it and
why the ticks run 9, 10, 11. Four things in them are rules of this specification rather than of the engine. Each
`from`/`to`
pair is rule 6.5's second shape, so the three transitions read the same way `survival_changed`'s do. `target_died` is
the string `no` under rule 3.2's closed two-value domain and not a JSON boolean, on `status`'s precedent, which is why
rule 4.5 still admits exactly two. `increase` is `0` on a `target_fear` that was already at its ceiling — a resolution
that changed nothing is still a resolution and is still recorded, under rule 6.3. And `discarded` is `36` against a
`transferred` of `9`: the record carries both halves of the transfer, because a consumer that saw only the delivered
figure could not tell a small surrender from a wasteful one, and rule 6.8 forbids the record from carrying the
difference as its own field when the text line does not.

Their field orders are not fixed here. Rule 6.4 defers every event's order to `SPEC-MOK-001`, which fixes all three,
and these lines exhibit that order rather than establishing it.

### Example: an action trace that carries a target and an absorbed strike

```json
{"record":"event","tick":10,"subject":"M02","event":"action_trace","result":{"proposal":{"action":"surrender","target":"M05"},"status":"accepted","detail":"transferred:9","position":{"x":57,"y":23},"territory":"A","health":71,"satiety":46,"energy":91,"fear":90,"suffered":[{"attacker":"M05","damage":29}]}}
```

Its text line, verbatim from the same stream, is

`tick=10 subject=M02 event=action_trace result=proposal:surrender,target:M05,status:accepted,detail:transferred:9,position:57:23,territory:A,health:71,satiety:46,energy:91,fear:90,suffered:M05:29`

and it is the record that exercises both places where rule 6.4's key-for-key projection does not hold. The proposal
object is rule 6.5's fourth shape and renders as **two** text fields, `proposal:surrender` and `target:M05`; the
`suffered` list is rule 6.9's and renders as `suffered:M05:29`, one `;`-joined pair per absorbed strike. Both
departures are rule 6.6's walk, and this line is the reconstruction that walk must produce.

It is the same subject and the same tick as the surrender record above, which is the point of showing the two
together: the resolution record states what the surrender did to both agents, and the trace states the state the
subject was in when it proposed it, including the strike it had absorbed inside the window. The `29` appears in both
and means the same thing in both, and neither record is derivable from the other.

`suffered` holds one entry here because one attacker struck. Rule 6.9 does not cap it, and a window that two
attackers struck into would carry two entries in resolution order.

### Example: an extinct population

```json
{"record":"metrics","tick":88,"living":0,"deaths":12,"population":{"A":0,"B":0},"health":{"sum":0,"min":null},"satiety":{"sum":0,"min":null},"energy":{"sum":0,"min":null},"fear":{"sum":0,"max":null},"territories":{"A":{"standing":0,"low":0,"medium":0,"high":0,"capacity":61,"depleted":true},"B":{"standing":3,"low":3,"medium":0,"high":0,"capacity":61,"depleted":false}}}
```

### Counterexample: an average

`{"record":"metrics","tick":1,"living":12,"health":{"mean":99.0}}` — violates rules 4.1 and 4.2. A float has no place
in a byte-identical stream and the rounding would be inherited by every consumer.

### Counterexample: a sentinel for absence

`{"id":"M01","died_at":0}` for a survivor — violates rule 4.4. Tick `0` is a legitimate death tick, so the record
would be ambiguous.

### Counterexample: a classification

`{"record":"run","reason":"extinction","outcome":"collapse"}` — violates rule 8.7. The label is an interpretation
against a threshold nobody has approved, and embedding it would make a threshold change a change to the engine's
observable output.

### Counterexample: a reserved field

`{"record":"metrics","tick":1,"conflicts":0}` — violates rules 7.8 and 10.4. It violated them when the engine computed
no conflict, and it still violates them now that it does: the figure is an aggregation no approved requirement asks
this record to carry, and a field fixed at zero reads as a measurement of none. The tick's own event records are where
a conflict count comes from.

### Counterexample: a path in the stream

`{"record":"header","schema":3,"events_path":"/tmp/run.jsonl"}` — violates rules 3.4, 5.5 and the *Security and
privacy properties*. The value is operator-supplied, so rule 3.3's totality argument does not cover it, and it is not
a fact about the simulation.

### Counterexample: a sink that interleaves

`--events-path -` — rejected under rules 1.3 and 13.1 before the run. Interleaved streams cannot satisfy rule 11.1.

## Explicitly unspecified decisions

- **The consumer.** No reader, parser, library, schema file, or example script is specified or required. A consumer
  written for verification purposes is `VER-MOK-012`'s to describe and is evidence, not product.
- **Batch execution, run persistence beyond one stream, and outcome classification.** Unauthorized. Phase 4b.
- **Conflict, combat and social metrics.** The phenomena are approved and the engine resolves them, and the stream
  carries them as the three resolution event kinds rule 3.2 enumerates. What is unspecified is their *aggregation*: no
  field of the metrics record and no field of the run record counts an attack, a threat, a retreat or a surrender, for
  rule 10.4's reason rather than for the engine's — no approved requirement asks either record to carry the figure. A
  tick's own event records are where such a count comes from, and adding one to either record would arrive under rule
  10.2 with a `schema` increment. Rule 7.8 and the *reserved field* counterexample state the same absence.
- **Typing the action trace's `detail`.** It stays rendered text under rule 3.5. Making it a typed value would change
  the engine's action result, which no approved requirement needs. Recorded as a residual.
- **Compression, framing alternatives, and a binary encoding.** Not specified and not needed; the format is
  line-oriented JSON and rule 2.1 fixes it.
- **A schema file.** Whether the schema is ever published as a machine-readable document is unspecified; this
  specification is the authority either way.
- **The engine version's own format.** `engine` is whatever the package metadata holds, constrained only by rule
  3.2's character class.
- **Whether `standing` and `depleted` can diverge.** Rule 7.6 requires both to be stated and does not fix the
  engine's derivation, which is `SPEC-MOK-001`'s.
