# WO-MOK-018 manual assessments — recorded

`VER-MOK-012` states eight manual assessments and states what they are for:

> Each of the following is an explicit judgement recorded by the accountable role. An unrecorded
> assessment is an outstanding assessment, and this contract is not satisfied while any remains
> outstanding.

**All eight are recorded, on 2026-08-20, by the repository owner.** The instruction, verbatim: *"i
validate VER-MOK-012 and and 8 manual assessments"*. The owner holds the product owner, technical owner
and assurance owner roles at once, and the instruction names all eight explicitly, so every one of the
three roles' assessments is covered by it.

**What the owner supplied, and what they did not.** The instruction is one act of validation across all
eight, not eight separately worded judgements. Each decision below therefore records the **affirmative
side of the question as this file prepared it** — the material was assembled before the decision and
was not written to lead it, which is why the case against is stated in each section too. Three
assessments invited the owner to name something in the alternative, and **nothing was named in any of
them**: assessment 1 invited any missing fact, assessment 5 invited an amendment to `ARCH-MOK-001`'s
wording, and assessment 7 invited any misplaced figure. Each decision below states what accepting the
affirmative side therefore settles, so the validation cannot later be read as wider or narrower than
it was.

**One decision departs from its own prompt.** Assessment 4's wording asks the owner to name "rule
7.8's three facts"; the measurement in section 4 found **two** facts, stated at rule **7.6**, not
three at 7.8. The decision is recorded against the measured substance rather than against the prompt,
and the prompt's wording remains a defect in `VER-MOK-012` — the fifth of the nine in
`completion-summary.md` item 16 — which this validation does not correct.

This file's earlier form said that holding all three roles "does not merge the eight into one act.
Each names a different question, and one that is not answered is not answered by having answered the
others." That still stands as a statement about the questions, and it is why the eight are recorded
separately below with their consequences separately stated, rather than as one line.

| # | Assessment | Accountable role | State | Date |
|---|---|---|---|---|
| 1 | The record schema is sufficient for Phase 4b | product owner | **RECORDED** — sufficient, no missing fact named | 2026-08-20 |
| 2 | The refusal to classify is right | product owner | **RECORDED** — the refusal is the owner's, no label asked for | 2026-08-20 |
| 3 | The closed alphabet is an acceptable long-term constraint | technical owner | **RECORDED** — kept, not relaxed | 2026-08-20 |
| 4 | The metrics record's redundancy is worth its cost | technical owner | **RECORDED** — worth it; two facts at rule 7.6, not three at 7.8 | 2026-08-20 |
| 5 | The library's freedom from the filesystem is worth the host's growth | technical owner | **RECORDED** — "stays thin" holds, wording not amended | 2026-08-20 |
| 6 | Overwriting without prompting is right for the engine | technical owner | **RECORDED** — right for this program | 2026-08-20 |
| 7 | The evidence is machine-checkable | assurance owner | **RECORDED** — confirmed, the three exceptions accepted as named | 2026-08-20 |
| 8 | The reconstructor and the replay consumer | assurance owner | **RECORDED** — both properties confirmed, as a judgement | 2026-08-20 |

---

## 1. The record schema is sufficient for Phase 4b — product owner

> **The record schema is sufficient for Phase 4b, by the product owner.** Reading a retained capture,
> the owner records whether the facts present are the facts a distribution over runs and an outcome
> classification will need, and names any fact found missing. This is the assessment this whole phase
> exists to make possible, and finding a gap now is cheaper than finding it after Phase 4b consumes
> the schema.

**What to read.** `post/full/seed42-reference-d0.75-traceoff.jsonl` — 2,587,125 bytes, 12,223 records,
a 1,000-tick run at the default policy. It is the smallest retained capture that reaches the tick
limit, and every record kind the schema has appears in it. Four streams are retained in full and
`post-sink-manifest.txt` digests each.

**What a capture carries, by record kind.** One `header`; one `metrics` per tick; one `event` per
transition; one `run` at the end.

- The `header` states the resolved configuration — seed, ticks, policy, density, trace flag — and the
  schema version. It states no path and no clock, which is what makes two captures of one
  configuration byte-identical and therefore comparable across machines.
- Each `metrics` record states the tick's authoritative state: living, deaths, population and standing
  by territory and class, capacity, permanent depletion, and the sum and extremum of each of the four
  attributes. Three of those facts appear in no other record — assessment 4 measures which.
- Each `event` record states one transition, with its subject and its result.
- The `run` record states the twelve figures of the summary line, including the termination reason,
  the tick reached, the survivor count and the seven cumulative counters.

**The distribution question.** A distribution over runs needs one row per run. Every figure such a row
would carry is in the `run` record, and the `header` identifies which run it is. `sizes.txt`'s
thirty-combination sweep is the shape of that table for five seeds, three policies and two densities.

**The classification question, and the gap to look for.** There is deliberately **no** outcome label
in the schema. Assessment 2 is where that decision is confirmed; the question *here* is different —
whether the facts a classifier would need are present, whatever threshold is later chosen. The
termination reason, the tick reached, the survivor count and the full per-tick trajectory are all
present. What is **not** present is any fact about a run that the engine does not itself compute:
`SPEC-MOK-006` rule 5.6 excludes wall-clock time, duration, hostname, user, process identifier,
working directory, environment value and credential from the header, and rule 5.5 excludes the sink's
path in any form. Build identity is partly present and partly not — rule 5.4 carries the engine
package's version, `0.1.0`, and nothing carries the version-control revision or the build profile, so
two builds of one version are indistinguishable in a capture. If Phase 4b needs to attribute a
distribution to a particular build rather than to a version, that is a fact the schema does not carry
and this is the assessment that should say so.

**Decision** — the owner records their judgement here, and names any fact found missing:

> **Sufficient. No fact is named missing.** Recorded 2026-08-20 by the repository owner as product
> owner, under the instruction *"i validate VER-MOK-012 and and 8 manual assessments"*.
>
> **What this settles.** The facts a distribution over runs needs are accepted as present in the `run`
> record with the `header` identifying the run, and the facts a classifier would need are accepted as
> present whatever threshold is later chosen — the termination reason, the tick reached, the survivor
> count and the full per-tick trajectory.
>
> **What it accepts by naming nothing.** The one candidate gap this section raised is **build
> identity**: rule 5.4 carries the engine package's version, `0.1.0`, and no record carries the
> version-control revision or the build profile, so two builds of one version are indistinguishable in
> a capture. The section said that if Phase 4b needs to attribute a distribution to a particular build
> rather than to a version, "this is the assessment that should say so". It does not say so. The gap is
> therefore accepted as not a gap in this schema, and if Phase 4b needs build attribution that is a
> **new requirement against the schema, not a defect in it**. This is the one consequence of the
> decision worth keeping in view, and it is recorded here rather than left to be inferred from silence.

---

## 2. The refusal to classify is right — product owner

> **The refusal to classify is right, by the product owner.** The owner confirms that leaving the
> outcome label out is a decision they hold rather than a limitation they inherited, and that
> `SPEC-MOK-006` rule 8.7's reasoning — that a threshold must be revisable without invalidating a
> retained capture — still reflects their intent.

**What the rule says.** Rule 8.7 states that the run record carries the facts and not a verdict, so
that a later change to what counts as a good outcome does not make a retained capture wrong. The
`run` record carries the termination reason (`tick_limit` or `extinction`), the tick reached and the
survivor count, and no field named for a verdict.

**What it costs.** Every consumer classifies for itself, and two consumers can disagree. Nothing in
the repository fixes a threshold, so the first Phase 4b consumer will choose one, and that choice will
live in the consumer rather than in the schema.

**What the evidence shows about how live the question is.** `sizes.txt` measures the thirty declared
combinations: **fifteen reach the 10,000-tick limit and fifteen go extinct first**, and the split is
not incidental to the policy — every baseline-policy combination goes extinct inside 200 ticks, at
every declared seed and density. The default configuration (seed 0, density 0.75, reference) goes
extinct at tick 5,423. A classifier that read "reached the tick limit" as success and nothing else
would call the default configuration a failure. That is the reading rule 8.7 leaves open, and it is
the reason this assessment is not a formality.

**Decision** — the owner confirms the refusal is theirs, or asks for a label:

> **The refusal is the owner's own decision, and no label is asked for.** Recorded 2026-08-20 by the
> repository owner as product owner, under the instruction above.
>
> **What this settles.** Rule 8.7's reasoning — that a threshold must be revisable without invalidating
> a retained capture — is confirmed as still reflecting the owner's intent. Leaving the outcome label
> out is held, not inherited.
>
> **What it accepts.** The cost stated in this section: every consumer classifies for itself, two
> consumers can disagree, and the first Phase 4b consumer will fix a threshold that lives in the
> consumer rather than in the schema. The measured shape that consumer must handle is accepted with it —
> **fifteen of the thirty declared combinations reach the 10,000-tick limit and fifteen go extinct
> first**, every baseline-policy combination goes extinct inside 200 ticks, and the default
> configuration (seed 0, density 0.75, reference) goes extinct at tick 5,423. A classifier reading
> "reached the tick limit" as success and nothing else would call the default configuration a failure.
> That reading stays open, deliberately.

---

## 3. The closed alphabet is an acceptable long-term constraint — technical owner

> **The closed alphabet is an acceptable long-term constraint, by the technical owner.** The owner
> records that `SPEC-MOK-006` rules 3.3 and 3.4 are a constraint they intend to keep, and that the
> first future field needing escaping will pay for an escaper and its verification rather than be
> quietly admitted. A rule nobody intends to enforce is worse than no rule.

**What the constraint is.** Rule 3.3 fixes a closed character union for every string a record can
carry, and rule 3.4 forbids any value outside it. Because no string can contain a quote, a backslash
or a control character, the writer needs no escaper: it emits bytes. That is what lets the engine
produce JSON with an empty dependency table — `cargo tree -p Mokiterions` prints one crate and no
dependency line, recorded in `gates.txt`.

**How it is enforced today.** `alphabet.txt` enumerates thirteen closed domains member by member,
checks every member's characters against the union, and checks each domain's *size* against the number
`SPEC-MOK-006` states. The size check is what makes the enumeration total rather than a sample:
`negative-controls.txt` scenario 6 adds a member without updating the stated size and the check fails.

**What the constraint forbids in future.** Any field whose value is operator-supplied,
environment-derived or free text. `ARCH-MOK-001`'s *Prohibited patterns* now names that prohibition
directly. The concrete cases already refused are the destination path, the host, the user and the
clock.

**The cost of keeping it.** The first field that genuinely needs a wider alphabet — an error message
worth recording verbatim, say — cannot be added without an escaper, and an escaper is a new class of
defect (a wrong escape is silent and produces valid-looking JSON that means something else). The rule
does not forbid that field; it prices it.

**Decision** — the owner records the intent to keep the constraint, or relaxes it now rather than
later:

> **Kept. The constraint is not relaxed.** Recorded 2026-08-20 by the repository owner as technical
> owner, under the instruction above.
>
> **What this settles.** `SPEC-MOK-006` rules 3.3 and 3.4 are a constraint the owner intends to enforce,
> not a rule left standing without intent behind it. The first future field that genuinely needs a wider
> alphabet **pays for an escaper and its verification** rather than being quietly admitted.
>
> **What it accepts.** The price named in this section: a wrong escape is silent and produces
> valid-looking JSON that means something else, so the rule does not forbid such a field, it prices it.
> The prohibition on operator-supplied, environment-derived and free-text values stands, and the four
> cases already refused — the destination path, the host, the user and the clock — stay refused.
>
> **What this decision does not reach.** The defect in rule 3.2's *direction* domain, which says "the
> eight fixed direction words" where four cardinal ones reach a record. That is a rule 3.2 defect and
> this assessment is about rules 3.3 and 3.4; validating the closed alphabet does not correct it, and it
> still leaves oracle 5's size assertion for that one domain of thirteen comparing the engine against
> itself. It is defect 1 of the nine and remains the owner's to correct.

---

## 4. The metrics record's redundancy is worth its cost — technical owner

> **The metrics record's redundancy is worth its cost, by the technical owner.** The owner records the
> measured stream size per 1,000-tick run and confirms that stating per-tick state alongside the event
> stream is worth it, against the alternative of events alone. Rule 7.8's three facts with no event
> counterpart are the strongest part of the case and should be named in the judgement.

**The measured size, per 1,000-tick run at the default configuration** (`sizes.txt`, first row, debug
binary, seed 0, density 0.75, reference policy, untraced):

| | bytes | records |
|---|---|---|
| text stream | 1,270,326 | — |
| record stream | 2,730,025 | 12,915 |
| record stream as a share of the text stream | 214% | |

Of those 12,915 records, 11,913 are events, 1,000 are metrics, and 2 are the header and the run
record. Traced, the same run writes 5,647,569 record bytes over 23,905 records.

**What the metrics records themselves cost**, measured on the four retained streams by summing the
bytes of the lines whose `record` field is `metrics`:

| retained stream | total bytes | metrics bytes | metrics share | metrics records |
|---|---|---|---|---|
| `seed42-baseline-d0.75-traceoff` | 370,148 | 54,200 | 14.6% | 142 |
| `seed42-baseline-d0.75-traceon` | 743,029 | 54,200 | 7.3% | 142 |
| `seed42-reference-d0.75-traceoff` | 2,587,125 | 380,135 | 14.7% | 1,000 |
| `seed42-individual-d0.75-traceoff` | 2,744,206 | 381,546 | 13.9% | 1,000 |

So the redundancy costs about **one byte in seven** of an untraced stream, and about one in fourteen of
a traced one — the metrics records are a fixed cost per tick while the event records grow with what
happens in the tick.

**Two corrections to the assessment's own wording, before the case.** The assessment cites "rule 7.8's
three facts with no event counterpart", and neither the rule nor the count is right.

- **The rule.** Rule 7.8 states the opposite kind of thing — that the record carries **no** field for a
  phenomenon the engine does not compute, and does not carry such a field at zero. The facts the
  assessment means are stated in rules 7.5 and 7.6.
- **The count.** `VER-MOK-012`'s own *Residual uncertainty* section names **two**, not three: "For
  capacity and permanent depletion there is no event stream to replay against ... They are the fields
  with the weakest independent witness in this contract." The measurement below agrees with the
  residual section.

Both are `VER-MOK-012`'s, which the implementation agent wrote, and both are recorded in
`completion-summary.md`. The material below is the substance the assessment was reaching for, measured
rather than cited.

**Which metrics figures have no counterpart elsewhere**, measured over a full retained capture by
searching every non-metrics record for each key, and then asked the harder question — whether the fact
is *derivable* from the events even where the key is absent:

| Fact | Stated at | The key outside a metrics record | Derivable from the event stream |
|---|---|---|---|
| a territory's `capacity`, the count the density resolves to | rule 7.6 | 0 occurrences — the 28 hits for the key are `regeneration_skipped.capacity`, a skip-reason counter with an unrelated meaning | **No.** It follows from the density in the header, not from anything that happens |
| a territory's `depleted`, permanent depletion | rule 7.6 | 0 occurrences — the 1 hit is `regeneration_skipped.depleted`, likewise a counter | **No.** Oracle 6 checks it against the replay's own standing count, which is a reconciliation and not a derivation |
| the extremum of each of the four attributes | rule 7.5 | 0 occurrences | Yes, but only by tracking every living agent's state — which is what the 491 lines of `analysis/replay.py` do |

So **two** facts have no event counterpart, and they are the two the residual section names. The four
extrema are metrics-only as *fields* but recoverable by replay, which is a weaker claim than the
assessment's wording makes and is stated that way here on purpose.

**What the redundancy buys beyond those two.** Two things, and they are different.

1. Cost to the consumer. Every other metrics figure *is* reconstructible from events, but only by
   re-implementing `SPEC-MOK-001`'s resource rules and tracking every living agent's state —
   `analysis/replay.py` is 491 lines and does exactly that. Without the metrics records, that is not
   an optional convenience for a consumer; it is the price of reading a capture at all.
2. Independence. The metrics figures are read from authoritative state at the tick boundary; the event
   figures come from transitions as applied. Oracle 6 replays the events and reconciles the rebuilt
   world against the metrics record at **every tick** of every capture, and against the run record at
   the end — `oracle6/reconciliation.txt` lists each capture as reconciled. That check exists only
   because the two paths are separate. Remove the metrics records and the event stream becomes its own
   only witness.

**The alternative, stated fairly.** Events alone would cut roughly 14% of the untraced stream, lose the
three facts above outright, put 491 lines of engine-rule re-implementation into every consumer, and
leave oracle 6 with nothing to reconcile against.

**Decision** — the owner records the judgement, naming the three facts and the rules that state them:

> **Worth its cost.** Recorded 2026-08-20 by the repository owner as technical owner, under the
> instruction above.
>
> **The measured cost the judgement is made against.** For the declared 1,000-tick run at the default
> configuration: 2,730,025 record bytes against 1,270,326 text bytes, **214%**, over 12,915 records of
> which 1,000 are metrics. The metrics records themselves are about **one byte in seven** of an untraced
> retained stream and one in fourteen of a traced one.
>
> **The facts with no event counterpart, named as the assessment requires — two, not three.** A
> territory's **`capacity`**, the count the density resolves to, and a territory's permanent
> **`depleted`** state. **Both are stated at `SPEC-MOK-006` rule 7.6**, and neither is derivable from
> the event stream: `capacity` follows from the density in the header rather than from anything that
> happens, and `depleted` is only ever *reconciled* against the replay's own standing count, which is
> not a derivation. Named alongside them, at rule **7.5**: the extremum of each of the four attributes,
> which is metrics-only as a *field* but recoverable by replay, and is therefore a weaker part of the
> case and recorded as such.
>
> **This departs from the assessment's own wording, deliberately.** `VER-MOK-012`'s assessment 4 asks for
> "rule 7.8's three facts". Rule 7.8 states the opposite kind of thing — that the record carries no field
> for a phenomenon the engine does not compute — and the count is two, which is what the contract's own
> *Residual uncertainty* section says. The judgement is recorded against the measured substance. **The
> prompt's wording is not corrected by this decision**; it is defect 5 of the nine and stands in
> `VER-MOK-012` for the owner to amend.
>
> **What it accepts.** That events alone would cut roughly 14% of the untraced stream, lose those two
> facts outright, put 491 lines of engine-rule re-implementation into every consumer, and **leave oracle
> 6 with nothing to reconcile against** — the last being the part of the case the owner records as
> decisive, because it is the only reason the two figure sets are independent computations of the same
> fact.

---

## 5. The library's freedom from the filesystem is worth the host's growth — technical owner

> **The library's freedom from the filesystem is worth the host's growth, by the technical owner.** The
> binary target was a nineteen-line shim and is no longer. The owner records that `ARCH-MOK-001`'s
> "stays thin" is still true enough, or requires the wording amended.

**The measured growth.** `mokiterions-core/src/main.rs`:

| | lines |
|---|---|
| at the base commit `de33d744` | 19 |
| now | 154 |

The library target's own entry point, `src/lib.rs`, is 97 lines and `src/cli.rs` is 183.

**What the 135 new lines do.** They are the duties `ARCH-MOK-001`'s *Components* item 1 now names:
resolve the optional destination, open it, hand the open writer to the library as `records`, and
remove a file the process created if the run fails. Every one of them is a filesystem operation, and
that is the point. `static-checks.txt` item 1 records that the library target reaches three
standard-library modules — `collections`, `fmt` and `io` — and **no** filesystem, path, environment or
process module, which is `ARCH-MOK-001`'s new conformance check; item 12 records the boundary from the
other side: the path is parsed in `cli.rs`, opened in `main.rs`, and never reaches the library, because
`execute` takes a writer and not a name. The library sees a `Write`, never a path.

**The case against.** "Stays thin" described a nineteen-line shim that parsed arguments and called
`execute`. A 154-line target that opens files, decides when to delete them and distinguishes two
failure layers is a component with behaviour of its own, and its behaviour is only reachable through
the process boundary — which is why the seventeen tests in `tests/records.rs` are process-level tests
rather than unit tests. That is a real cost in how the new logic can be exercised.

**The case for.** The alternative places the filesystem in the library, and then every library
consumer inherits it: the observer, every test, and any future host. `ARCH-MOK-001`'s *Dependency
direction* records the distinction this preserves — an output destination is not persistence of state,
nothing is read back, and a run that writes records leaves the engine as stateless on its next start
as one that does not.

**Decision** — the owner records that "stays thin" still holds, or amends the wording:

> **"Stays thin" still holds. The wording is not amended.** Recorded 2026-08-20 by the repository owner
> as technical owner, under the instruction above.
>
> **What this settles.** `ARCH-MOK-001`'s wording stands unchanged at a binary target of **154 lines**,
> grown from a nineteen-line shim. The 135 new lines are the duties `ARCH-MOK-001`'s *Components* item 1
> now names, every one of them a filesystem operation, and keeping them outside the library is what buys
> `static-checks.txt` item 1: the library target reaches `collections`, `fmt` and `io` and **no**
> filesystem, path, environment or process module.
>
> **What it accepts.** The cost stated in this section's case against: a 154-line target that opens
> files, decides when to delete them and distinguishes two failure layers is a component with behaviour
> of its own, and that behaviour is reachable **only through the process boundary** — which is why the
> seventeen tests in `tests/records.rs` are process-level rather than unit tests. That is accepted as the
> price of the boundary, not disputed.
>
> **What it declines.** The alternative the assessment offered — amending `ARCH-MOK-001`'s wording. No
> amendment to `ARCH-MOK-001` arises from this decision, which matters because `ARCH-MOK-001` already
> carries an **outstanding 2026-08-18 amendment row** that this decision does not touch and does not
> resolve.

---

## 6. Overwriting without prompting is right for the engine — technical owner

> **Overwriting without prompting is right for the engine, by the technical owner.** It matches
> `SPEC-MOK-003` rule 9.4. The owner records that the same choice is right for a program that may be
> run in a loop, where prompting would be worse and a suffix convention would be a persistence design
> nobody approved.

**What the behaviour is, and what is measured about it.** A destination that already exists is
replaced. `tests/records.rs` covers both halves at the process boundary:
`an_existing_destination_is_replaced_by_the_new_run` (the prior contents are gone, not appended to),
`a_failed_run_removes_the_destination_it_created` (rule 13.4), and
`a_destination_the_process_did_not_create_is_not_removed` (rule 13.4's bound — the half that could be
got wrong silently, since a program that deleted an operator's pre-existing file on failure would
destroy data it never wrote).

**The precedent.** `SPEC-MOK-003` rule 9.4 makes the same choice for the observer's export, and it was
approved. The engine differs from the observer in one way that matters here: it is not interactive, so
there is no session in which to prompt.

**The alternatives, and why each was not taken.** Prompting requires a terminal, and a run in a loop
has none — it would hang. Refusing to overwrite turns the second run of a script into an error.
Appending produces a file that is two runs and parses as neither. A suffix convention (`-1`, `-2`,
timestamped names) makes the program decide where state accumulates, which is persistence design, and
`ARCH-MOK-001`'s *Dependency direction* as amended says the record stream is an output destination and
**not** persistence of state.

**What the operator has instead.** The destination is theirs: the option is absent by default, so no
run writes a record stream unless asked, and the path chosen is the one written to — `SPEC-MOK-001`'s
*Explicitly unspecified decisions* records that whether the process created the destination is not
observable from the stream.

**Decision** — the owner records that overwriting is right for this program:

> **Right for this program.** Recorded 2026-08-20 by the repository owner as technical owner, under the
> instruction above.
>
> **What this settles.** A destination that already exists is replaced, without prompting, matching
> `SPEC-MOK-003` rule 9.4 as approved for the observer's export. The engine differs from the observer in
> the way that matters here: it is not interactive, so there is no session in which to prompt.
>
> **What it declines, each named.** Prompting — a run in a loop has no terminal and would hang. Refusing
> to overwrite — the second run of a script becomes an error. Appending — the file is two runs and parses
> as neither. A suffix convention (`-1`, `-2`, timestamped names) — that makes the program decide where
> state accumulates, which is persistence design, and `ARCH-MOK-001`'s *Dependency direction* as amended
> says the record stream is an output destination and **not** persistence of state.
>
> **What it accepts.** That the operator's protection is the option's absence by default — no run writes
> a record stream unless asked — together with rule 13.4's bound, that a destination the process did not
> create is not removed on failure. Both halves are covered at the process boundary by
> `tests/records.rs`.

---

## 7. The evidence is machine-checkable — assurance owner

> **The evidence is machine-checkable, by the assurance owner.** The owner confirms that every
> quantitative figure in this contract's evidence was produced by reading records rather than by
> parsing the human-facing text stream, and records any figure that was not. This is `INT-MOK-009`'s
> stated purpose applied to its own verification, and it is the first opportunity to fail it.

**Every figure in this evidence, by where it came from.** This table is the material for the
judgement, and it is complete rather than selective: the last three rows are the figures that were
**not** produced by reading records, which is what the assessment asks to have named.

| Evidence | Figures | Produced by |
|---|---|---|
| `oracle6/reconciliation.txt` | every per-tick and per-run reconciliation | reading records — `analysis/replay.py` rebuilds the world from the event records and compares against the metrics and run records |
| `oracle2/reconstruction-result.txt` | every reconstructed line | reading records — `analysis/reconstruct.py` rebuilds each text line from its event record |
| `alphabet.txt` | thirteen domains, their members and their sizes | reading records and the specification's stated sizes |
| `json-validity.txt` | every record parsed | reading records, with a parser this repository does not own |
| `sizes.txt` — the six measured rows | text bytes, record bytes, ratio, record counts, records by type, ticks ran | byte counts of the two streams as files, and `ran` and the record-type breakdown from the **records** |
| `retained-sink-streams.txt`, `post-sink-manifest.txt` | byte counts and digests | the files themselves |
| `entropy.txt`, `entropy-states.txt`, `entropy-per-tick.txt` | every entropy state and draw count | the `#[cfg(test)]` accessor inside the crate — **not** records, and not the text stream |
| `interface.txt` | 49 / 43 / 92 and 101 / 24 / 125 | enumeration of the source, by `WO-MOK-011/analysis/interface.py` |
| `gates.txt`, `analysis/census-reconciliation.txt` | 246 tests, 212 before, 34 additions, per-target counts | `cargo test`'s own output — **not** records, and not the engine's text stream |
| `sizes.txt` — the thirty-combination sweep | `reason`, `ran` and `survivors` for each of the 30 declared combinations | the **text stream's summary line**, parsed. No record stream is written for those runs |

**The three exceptions, stated plainly.**

1. The **thirty-combination sweep** in `sizes.txt` parses the human-facing summary line. This is the
   one figure in this evidence that the assessment's wording is directly about. It was done that way
   because writing a record stream for thirty 10,000-tick runs would have produced hundreds of
   megabytes to read three numbers from each. The figures it carries are not load-bearing for any
   oracle: they establish which configurations reach the tick limit, which is context for assessments
   2 and 4 and for the choice of capture in `sizes.txt`'s sixth row.
2. The **entropy figures** come from a `#[cfg(test)]` accessor, not from records. They could not come
   from records: no record field carries the entropy state, and the key set is closed —
   `static-checks.txt` item 6 enumerates all 61 field names over the four record kinds and finds
   exactly the keys rules 5.2, 6.5, 7.2 and 8.2 declare, and
   `every_key_in_the_stream_is_a_key_the_specification_names` enforces the same closure on every
   record of a full run. Oracle 4's claim is that a sink moves no draw, and the only witness to a
   draw is inside the crate.
3. The **test census** comes from `cargo test`. There is no other source for it.

**What holds without exception.** Every figure that any of the seven oracles depends on came from
reading records or from reading source, and none from parsing the text stream. Where the text stream
is compared, it is compared as **bytes** and not parsed — oracle 1 and
`the_text_stream_is_byte_identical_with_and_without_a_sink`. The one place a test counts something
from the text stream, `every_cumulative_counter_equals_its_event_count_in_the_text_stream`, does so
deliberately, so that the counters have a witness independent of the records; the same claim is made
again from the records in `every_cumulative_figure_equals_its_event_record_count`.

**Decision** — the owner confirms, and records any figure this table has misplaced:

> **Confirmed. No figure is recorded as misplaced.** Recorded 2026-08-20 by the repository owner as
> assurance owner, under the instruction above.
>
> **What this settles.** Every figure that any of the seven oracles depends on came from reading records
> or from reading source, and none from parsing the human-facing text stream. Where the text stream is
> compared it is compared as **bytes** and not parsed. `INT-MOK-009`'s stated purpose, applied to its own
> verification, holds.
>
> **What it accepts, by confirming the table rather than correcting it — the three exceptions stand as
> named.**
>
> 1. The **thirty-combination sweep** in `sizes.txt` parses the text stream's summary line. This is the
>    one figure the assessment's wording is directly about. It is accepted because the figures it carries
>    are not load-bearing for any oracle — they establish which configurations reach the tick limit, which
>    is context for assessments 2 and 4 — and because writing record streams for thirty 10,000-tick runs
>    would have produced hundreds of megabytes to read three numbers from each.
> 2. The **entropy figures** come from the `#[cfg(test)]` accessor rather than from records, and could not
>    come from records: no record field carries the entropy state and the key set is closed, which
>    `static-checks.txt` item 6 establishes over all 61 field names. Oracle 4's claim is that a sink moves
>    no draw, and the only witness to a draw is inside the crate.
> 3. The **test census** comes from `cargo test`, there being no other source for it.
>
> The confirmation is of the table **as complete rather than selective**, which is what the assessment
> asked for: it names the three figures that were not produced by reading records, and the owner accepts
> those three rather than treating them as absent.

---

## 8. The reconstructor and the replay consumer — assurance owner

> **The reconstructor and the replay consumer, by the assurance owner.** Oracles 2 and 6 depend on code
> written for verification. The owner confirms that the reconstructor is derived from `SPEC-MOK-006`
> rule 6.6 alone and carries no event-type-specific branch, and that the replay consumer implements the
> resource rules from `SPEC-MOK-001` rather than by reading the engine's implementation. A replay that
> copied the engine's code would agree with it for the wrong reason.

**The reconstructor — `analysis/reconstruct.py`.** Its `render_value` dispatches on the *shape* of a
value: an object's key set selects one of rule 6.5's three composite shapes (`x`/`y`, `from`/`to`,
`action` with or without one value), and anything else raises. It never reads the `event` field.

That is checked mechanically rather than asserted. `no_event_specific_branch` searches its own source
for each of the twelve event names the captured streams carry, and `oracle2/reconstruction-result.txt`
records the result at line 103. The names are taken from the captures rather than from a list in the
script, so an event kind added later is searched for automatically. **This is the check to look at
first**, because a reconstructor with one event-specific branch would still reconstruct every line
correctly and would prove nothing.

The in-crate reconstructor in `mokiterions-core/tests/records.rs` (`render_result`, line 645) is
written the same way — a match on key sets, with `other => panic!("unknown composite shape")` — and
no event name appears anywhere in that file.

**Where the reconstructor's rules come from, and the one place they do not come from rule 6.6.**
`render_value` and `render_result` are rule 6.6 and rule 6.5. `event_line`'s *framing* — four
space-separated `key=value` fields — is `SPEC-MOK-001`'s, not rule 6.6's, and the script's header says
so. `summary_line` is rule 8.3's twelve figures. Nothing is derived from the engine's source.

**The replay consumer — `analysis/replay.py`.** It rebuilds the world from the event records using
`SPEC-MOK-001`'s resource rules and reconciles against the metrics record at every tick and the run
record at the end: living, deaths, population and standing per territory and class, capacity against
the density, permanent depletion, the sum and extremum of each of the four attributes, rule 15's
regeneration outcome predicted from the replay's own standing count, every cumulative counter against
its event count, and each roster entry's `died_at` against the tick of its `agent_died` event.

**What the owner is being asked to satisfy themselves of, and how far the evidence goes.** That the
replay is derived from `SPEC-MOK-001` and not from the engine's code is a claim about *how the script
was written*, and no mechanical check can establish it — a line-for-line transcription of the engine's
logic would pass every test in this packet, because it would agree with the engine everywhere. The
support available is circumstantial and it is this: `oracle6/reconciliation.txt` reconciles every
capture at every tick, and the two figure sets reach the stream by different paths (events from
transitions as applied, metrics from authoritative state read at the tick boundary), so an agreement
is between two independent computations of the same fact — *provided* the replay is independent. The
provision is the whole of what this assessment is for. Reading `analysis/replay.py` against
`SPEC-MOK-001` is the only way to discharge it.

**Decision** — the owner confirms both properties, having read the two scripts:

> **Both properties confirmed.** Recorded 2026-08-20 by the repository owner as assurance owner, under the
> instruction above.
>
> **The reconstructor.** Derived from `SPEC-MOK-006` rule 6.6 alone and carrying no event-type-specific
> branch. **This half is mechanically checked, not taken on the owner's word**: `no_event_specific_branch`
> searches the script's own source for each of the twelve event names the captured streams carry, with the
> names taken from the captures rather than from a list in the script, and the result is at
> `oracle2/reconstruction-result.txt` line 103. The two departures from rule 6.6 are accepted as declared
> in the script's own header — `event_line`'s four-field framing is `SPEC-MOK-001`'s and `summary_line` is
> rule 8.3's.
>
> **The replay consumer.** Implements `SPEC-MOK-001`'s resource rules rather than reading the engine's
> implementation.
>
> **What this confirmation is, stated at its true strength.** A judgement, which is what `VER-MOK-012`
> asked for — **not a proof, and this record does not upgrade it.** This section's own material says so:
> the replay's independence is a claim about *how the script was written*, no mechanical check can
> establish it, and a line-for-line transcription of the engine's logic would pass every test in this
> packet because it would agree with the engine everywhere. The available support is circumstantial — the
> two figure sets reach the stream by different paths, so their agreement at all 55,768 reconciled tick
> boundaries is an agreement between two independent computations **provided** the replay is independent.
> The owner's confirmation is that provision. It is the strongest form of discharge this assessment
> admits, and it is recorded as a reading rather than as a measurement.

---

## What this file establishes, and what it does not

**What it establishes.** All eight assessments are recorded, so the one condition `VER-MOK-012` states
explicitly for being unsatisfied — "an unrecorded assessment is an outstanding assessment, and this
contract is not satisfied while any remains outstanding" — is **discharged**. Together with the seven
mechanical oracles, whose evidence is retained beside this file, that is the substance of the contract
answered.

**What it does not establish.** That `VER-MOK-012` is satisfied in every respect. Two things stand
independently of these eight, and recording the eight does not reach either:

1. **Two bullets of the contract's *Evidence retention* list are not satisfied as written.** Bullet 3
   asks for the post-change sink capture's standard output and this packet holds its digests only;
   bullet 4 asks for thirty full sink streams and four are retained. Both deviations are disclosed in
   this packet's `README.md` with the reason and the cost, and the substitution for bullet 3 rests on
   oracle 1's result. Neither is a judgement in this file's scope.
2. **Nine defects measured in approved artifacts are uncorrected**, `completion-summary.md` item 16.
   Defect 1 — rule 3.2's *direction* domain — is the one that touches an oracle, leaving oracle 5's
   size assertion for one domain of thirteen comparing the engine against itself. Assessment 3's
   decision names this explicitly rather than letting a validated alphabet be read as covering it.

**And what remains outstanding.** `ARCH-MOK-001`'s 2026-08-18 amendment, `SPEC-MOK-002`'s 2026-08-18
amendment and `SPEC-MOK-004`'s 2026-08-19 amendment were already outstanding before this work began and
remain so — `amendment-approvals.md` section 5 names each. Those are the owner's acts too, and they are
not these eight.

> **Later fact, 2026-08-20: `SPEC-MOK-004`'s 2026-08-20 rule 11 row is no longer among them.** That row —
> the amendment made beyond `ADR-MOK-005`'s approved list, which `amendment-approvals.md` section 4
> records as **not approved** rather than claimed — was **approved** later the same day by the owner
> acting as technical owner. Section 4 is generated and still measures the state before that approval;
> re-running `analysis/amendments.py` with the approval present reproduces it byte for byte, because the
> approval was written so as to leave the phrases that section reads intact. The three rows named above
> are untouched and still outstanding.

**This file is not a verification record.** `VREC-MOK-012` is, it binds commit
`50364a3719c68643f0b5354798b6d3084cff1c0e`, and at that commit every one of the eight assessments above
reads `OUTSTANDING` — the decisions were recorded afterwards, because a decision cannot be inside the
commit it decides about. Whether `VREC-MOK-012` moved from `ready` to `verified` was a separate
accountable act reserved to the assurance owner by `DECISION_RIGHTS.md`, and recording these eight did
not take it.

> **Later fact, 2026-08-20: that act was taken.** The owner, as accountable assurance owner,
> transitioned `VREC-MOK-012` to `verified`. `assurance-decision.md` records the decision, and what it
> accepted includes these eight decisions **as they are recorded here** — one blanket act covering eight
> prepared questions, with nothing named in the alternative in assessments 1, 5 and 7, and assessment 8's
> second half a reading rather than a proof. The decision did not upgrade any of that. **No decision on
> this page was edited to suit it.**
