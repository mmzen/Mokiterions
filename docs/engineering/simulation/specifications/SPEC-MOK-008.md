+++
id = "SPEC-MOK-008"
type = "specification"
title = "Batch sweep, bound fact rows, and a revisable outcome classification"
status = "draft"
owners = ["technical owner"]
created = "2026-08-30"
updated = "2026-08-30"

[relations]
specifies = [
  "REQ-MOK-078",
  "REQ-MOK-079",
  "REQ-MOK-080",
  "REQ-MOK-081",
  "REQ-MOK-082",
  "REQ-MOK-083",
]
+++

# Specification: Batch sweep, bound fact rows, and a revisable outcome classification

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-30 | Original content for `REQ-MOK-078` through `REQ-MOK-083`. | Drafted under `WO-MOK-033`. Every figure in it is measured at `c90edc9` and stated with its measurement; no threshold is asserted without saying whether measurement supports it. Awaiting the repository owner's approval as accountable technical owner. |

## Scope

This specification fixes the exact behavior of two instruments: a **batch driver** that sweeps the engine over a
declared configuration space and retains one bound fact row per run, and a **classifier** that reads those rows,
assigns an outcome class and states a distribution. It fixes their locations, their command lines, the declared default
sweep, the retained output's three record kinds and every field of each, the behavioural counters and their
effectiveness tests, the cross-checks against the engine's own cumulative figures, the digest binding, the outcome
vocabulary and its ordered predicates, the distribution's shape, the exit statuses, and the failure behavior of both.

It specifies `REQ-MOK-078` through `REQ-MOK-083` and nothing else.

**It specifies no change to the engine.** `SPEC-MOK-006` remains the sole authority for the structured record stream,
its four record kinds, its schema version and its value alphabet, and this specification is a *consumer* of that
contract rather than an extension of it. The schema version stays **3**. No record kind, field or value domain moves,
so no retained record-stream digest anywhere in this repository moves. `SPEC-MOK-001` remains the sole authority for the
text stream and the event vocabulary, and the counters rule 9 defines are named from that vocabulary rather than
restating it.

It specifies nothing about the terminal observer, nothing about the `llm` decision source, and nothing about any live
run. `REQ-MOK-072` governs the last two and neither instrument can reach a provider.

Throughout, **a cell** means one point of the sweep — one decision source, one food density, one seed, at one tick
horizon. **A row** means the retained record of one completed cell. **The stream** means the engine's structured record
stream for one cell, which is temporary. **A class** means an outcome class the classifier assigns.

## Actors and external systems

- **The batch driver** enumerates cells, executes the engine once per cell as a child process, reads and hashes each
  stream, derives each row, discards the stream, and writes the retained output. It classifies nothing.
- **The classifier** reads retained output, assigns exactly one class per row, and states a distribution. It executes
  no process, reads no stream, and never writes into retained output.
- **The engine's binary target** is executed unmodified through its existing options. It is not aware a batch is
  driving it, and `REQ-MOK-078` requires that this remain observable: a cell run by the batch and the same cell run by
  hand produce identical bytes.
- **The operator** names a sweep or accepts the default, and supplies the paths.

### 1. Location, naming and runtime

1.1 The instruments are `scripts/run_simulation_batch.py` and `scripts/classify_simulation_runs.py`.

1.2 Each has a test file beside it: `scripts/test_run_simulation_batch.py` and
`scripts/test_classify_simulation_runs.py`. This is the convention every existing instrument in `scripts/` follows and
it is stated here so a later reader does not treat it as incidental.

1.3 Both are Python and import **only the Python standard library**. No third-party import appears in either
instrument or either test. `ADR-MOK-008` records the reason: a standard-library-only instrument adds nothing for
`SPEC-MOK-005` rule 15's declared-set comparison to compare, and both packages' declared Cargo sets stay unchanged.

1.4 Neither instrument is a Cargo target, a workspace member or a package. `ARCH-MOK-001`'s prohibition on a third
package and `SPEC-MOK-004` rule 1's structural census are untouched, and `ADR-MOK-008` is the decision record.

1.5 Neither instrument imports the other. The classifier is usable on retained rows in a checkout with no engine binary
present, which `REQ-MOK-082` requires.

### 2. The batch driver's command line

2.1 Invocation is `python scripts/run_simulation_batch.py --out <path> [options]`.

2.2 `--out <path>` is required and names the file the retained output is written to. The driver creates it and
truncates it. It creates no directory it was not given.

2.3 `--binary <path>` names the engine binary. Absent, the driver resolves `target/release/Mokiterions` and then
`target/debug/Mokiterions`, with the platform's executable suffix, and reports which it chose. Determinism across build
profiles is measured — a `social` cell at seed 2 over 1,000 ticks hashes to `ed1da4b6…` from both — so the choice does
not change a row, and it is reported because a reader should not have to trust that.

2.4 `--sources <list>`, `--densities <list>` and `--seeds <spec>` declare the axes. Lists are comma-separated. A seed
specification admits comma-separated integers and inclusive ranges written `a-b`.

2.5 `--ticks <n>` sets the tick horizon for every cell of the sweep. One horizon per sweep; a sweep with a varying
horizon would make survivor counts incomparable across cells.

2.6 Absent all four of `--sources`, `--densities`, `--seeds` and `--ticks`, the **declared default sweep** of rule 3
runs. Supplying some but not all replaces only those axes and leaves the rest at the default, and the resulting sweep is
stated in full in the output under rule 6 so that a partially defaulted sweep is not ambiguous.

2.7 `--keep-stream <source>:<density>:<seed>` retains that one cell's stream instead of discarding it, at a path the
driver reports. It is a debugging affordance, it may be given more than once, and it never changes a row.

2.8 `--jobs <n>` may execute cells concurrently. It changes no row, because a row is a function of its cell, and the
retained output's order is rule 5's regardless of completion order. Absent, `n` is 1.

2.9 Unknown options, a malformed seed specification, an empty axis, and `--out` naming a path under
`docs/engineering/` are usage errors under rule 15.

### 3. The declared default sweep

3.1 The default sweep is:

| Axis | Values | Count |
|---|---|---|
| decision source | `baseline`, `reference`, `individual`, `social` | 4 |
| food density | `0.10`, `0.25`, `0.50`, `0.75`, `1.00` | 5 |
| seed | `0` through `19` inclusive | 20 |
| tick horizon | `1000` | 1 |

3.2 That is **400 cells**. Measured cost, extrapolated from 20 runs at 0.047 s and 20 stream scans at 0.041 s at
`c90edc9`: about 19 s of engine execution and 16 s of scanning, about 500 KB retained, and about 1.2 GB of stream bytes
read and discarded. Peak disk is one stream, about 3 MB, because rule 7 reuses one path.

3.3 **Every axis value is declared on measured grounds.** The four sources are the four that decide in process;
`SPEC-MOK-007` rule 18.1's fifth value `llm` is excluded by rule 4.2. The five densities span the outcome range
measured on 2026-08-30: `0.10` produced extinction in 3 of 3 runs, `0.25` produced 1 or 2 survivors of 12, `0.50`
produced 6 or 7, `0.75` produced 7 to 9, and `1.00` produced 7 to 9 and is where the response saturates. Twenty seeds
is the axis that produced variation in the figures without changing the class; five seeds per cell already showed 3 of
4 sources yielding one class in every seed, so the seed axis is sized for dispersion rather than for class discovery.
The horizon is 1,000 because `baseline` self-terminates at ticks 119 to 163 and the other three sources' figures are
stable well before 1,000.

3.4 The default sweep is a value in this specification, not in the instrument's help text alone. Changing it is an
amendment here.

### 4. Cells, ordering and refusal

4.1 The cells of a sweep are the cross product of its axes. Duplicate values on any axis are a usage error under rule
15; they are not collapsed, because a silently collapsed duplicate and an accepted one differ in every later
distribution and the operator should say which was meant.

4.2 A sweep naming `llm` among its sources is **refused before any cell executes**, and the refusal names
`REQ-MOK-072`. It is not attempted and allowed to fail.

4.3 Cells are ordered by decision source in the order given, then density in the order given, then seed ascending. The
retained output is written in this order whatever order cells completed in.

4.4 Each cell is executed as `<binary> --seed <seed> --ticks <ticks> --policy <source> --density <density>
--events-path <stream path>` and with no other option. In particular `--trace-actions` is off, no transcript option is
passed, and no live option is passed.

4.5 The driver passes no environment variable to the child beyond what the operator's own environment supplies, and
reads none itself.

### 5. Execution and the child's own output

5.1 Each cell is one child process. The driver captures the child's standard output and standard error and its exit
status.

5.2 A cell's standard output is **not retained**. It is the text stream `SPEC-MOK-001` fixes, it is reconstructible
from the event records under `REQ-MOK-042`, and retaining both would retain the same facts twice.

5.3 A non-zero exit status is a cell failure under rule 13, and the child's standard error is carried into the failure's
reason verbatim. It is the one place the retained output carries text the driver did not author.

5.4 The driver does not retry a cell. A cell fails once and is reported once. `REQ-MOK-083`'s rationale is the reason:
a silent retry would hide a nondeterministic failure, and this engine is deterministic, so a retry that succeeded would
itself be the finding.

### 6. The retained output: framing and the `sweep` record

6.1 The retained output is one JSON object per line, in UTF-8, with `\n` line endings on every platform. Line endings
are fixed here because the file is compared for equality and hashed.

6.2 Every object carries a `record` field. There are exactly **three** kinds — `sweep`, `cell` and `batch` — and they
appear in that order: exactly one `sweep` first, zero or more `cell` records in rule 4.3's order, exactly one `batch`
last. The names are deliberately not `SPEC-MOK-006`'s, so no reader confuses this file with an engine stream.

6.3 The `sweep` record states the sweep in full:

```json
{"record":"sweep","format":1,"sources":["baseline","reference","individual","social"],
 "densities":["0.10","0.25","0.50","0.75","1.00"],"seeds":[0,1,2],"ticks":1000,
 "cells":60,"binary_profile":"release","digest_algorithm":"sha256","defaulted_axes":["sources","densities","ticks"]}
```

6.4 `format` is this file format's version, `1` here. It is incremented when a field is added or a value's domain gains
a member, on `SPEC-MOK-006` rule 10.2's precedent.

6.5 `defaulted_axes` names the axes rule 2.6 filled from the default, so a partially defaulted sweep is legible without
knowing the default's history.

6.6 Every axis value is stated, including values that produce no rows, which is what `REQ-MOK-082` needs to state a
sweep alongside a distribution.

### 7. The temporary stream

7.1 The driver writes every cell's stream to **one reusable path** it controls and reports. The engine overwrites an
existing path cleanly, which is measured, so one path suffices and peak disk is one stream.

7.2 The path is not under `docs/`, not under `scripts/`, and not a path the operator named for output. Absent an
operator-supplied location the driver uses the platform's temporary directory.

7.3 The stream is read once, hashed in the same pass, and then removed. A stream that cannot be removed does not fail
the cell and does not fail the batch: the row is complete, and the driver reports the leftover path.

7.4 `--keep-stream` copies the named cell's stream to a reported path before the reusable path is overwritten. It
changes no row.

7.5 The stream is never written to standard output. The engine refuses `--events-path -` with "expected a file path,
and no path denotes a standard stream", so no design here can pipe it.

### 8. The `cell` record

8.1 One `cell` record per completed run. Its shape:

```json
{"record":"cell","source":"social","density":"0.75","seed":0,"ticks":1000,
 "engine":"0.1.0","stream_sha256":"ed1da4b6...",
 "reason":"tick_limit","run_ticks":1000,"roster":12,"survivors":9,"deaths":3,"crossings":35,
 "consumed":{"low":147,"medium":137,"high":124},
 "regenerated":391,"regeneration_skipped":{"depleted":0,"capacity":4},
 "territories":{"A":{"population":6,"low":8,"medium":21,"high":21},
                "B":{"population":3,"low":13,"medium":16,"high":26}},
 "attempted":{"attack_resolved":9,"threat_resolved":12,"surrender_resolved":6},
 "effective":{"attack_resolved":9,"threat_resolved":0,"surrender_resolved":6},
 "lethal_attacks":1,
 "crosschecks_passed":7}
```

8.2 `source`, `density`, `seed` and `ticks` are the cell's coordinates as passed to the engine. `density` is the string
the engine's option takes, not a computed decimal, which is how rule 17.2's prohibition on a floating-point value is
satisfied on this axis.

8.3 `engine` is the engine version the stream's `header` record states, not a version the driver inferred.

8.4 `reason`, `run_ticks`, `survivors`, `deaths`, `crossings`, `consumed`, `regenerated`, `regeneration_skipped` and
`territories` are the run record's own figures, copied without reinterpretation. `run_ticks` is the run record's
`ticks`; it is named differently from the cell's `ticks` because a run that terminated early has fewer, and conflating a
requested horizon with an achieved one would make every early termination unreadable.

8.5 `roster` is the length of the run record's `agents` array. The array itself is **not** retained: per-agent names,
identifiers and death ticks are a run's facts and not a distribution's, and retaining twelve objects per row would
multiply the retained bytes for figures no requirement here consumes. This is a deliberate loss and it is recorded as
one; a question about individual survival is answered by re-running the cell, which the digest makes exact.

8.6 The `cell` record carries **no class, label, verdict, severity, threshold or interpretation**. `REQ-MOK-081`
requires it and `SPEC-MOK-006` rule 8.7 is the reason: a threshold must be revisable without invalidating a retained
capture, and a row carrying a label would not be.

8.7 No field is a rate, ratio, percentage, mean or floating-point value. `crosschecks_passed` is a count, not a
proportion.

### 9. The behavioural counters and their effectiveness tests

9.1 For every cell the driver counts occurrences of each event kind in the stream. The kinds counted into `attempted`
are those carrying a behaviour the run record states no cumulative figure for: **`attack_resolved`**,
**`threat_resolved`** and **`surrender_resolved`**.

9.2 `effective` states, for each of those three, how many occurrences changed authoritative state, decided from the
record's own fields and from nothing else:

| Kind | Effective when | Field it reads |
|---|---|---|
| `attack_resolved` | `damage > 0` | `result.damage` |
| `threat_resolved` | `increase > 0` | `result.increase` |
| `surrender_resolved` | `transferred > 0` | `result.transferred` |

9.3 `lethal_attacks` counts `attack_resolved` records with `result.target_died` equal to `yes`. It is a third figure
rather than a narrowing of `effective`, because a lethal attack is also an effective one and the two counts are not
alternatives.

9.4 **Both counts are stated for all three kinds, and a count of zero is stated rather than omitted**, so a mechanism
that never fired is distinguishable from one that was not counted.

9.5 The driver derives no figure from the difference of two counters and states no such difference. A consumer may
subtract; `SPEC-MOK-006` rule 7.7's principle applies.

9.6 **The attempted-and-effective split exists because of a measurement, which is recorded here rather than left in
evidence.** Across 35 runs at `c90edc9` the streams carried **1,448** `threat_resolved` records of which exactly
**one** had `increase > 0`; the other 1,447 targeted a creature whose `fear` was already at its ceiling of 100, so the
event fired and changed nothing. On `attempted` alone the cell with 106 inert threats is the most aggressive in the
batch and on `effective` it is the least. A single counter here would invert the ordering, not merely coarsen it.

9.7 **A phenomenon with no event kind gets no field and is disclosed as a gap.** `docs/ROADMAP.md`'s Phase 6 names
retreat; the vocabulary `SPEC-MOK-001` fixes has fifteen kinds and **no `retreat_resolved`** among them. No retreat
field is synthesised from `threat_resolved` or from any other kind, and `VER-MOK-019` records the absence.

9.8 The counters hold whether or not per-action tracing was enabled. Rule 4.4 leaves it off, and tracing adds
`action_trace` records and changes no resolution record, so the counts would be unaffected.

### 10. The cross-checks

10.1 In the same pass the driver checks every equality `SPEC-MOK-006` rule 8.6 makes a property of the stream. The set
is exactly:

| Counted kind | Must equal |
|---|---|
| `territory_crossed` | the run record's `crossings` |
| `agent_died` | the run record's `deaths` |
| `food_consumed` | `consumed.low + consumed.medium + consumed.high` |
| `food_regenerated` | the run record's `regenerated` |
| `food_regeneration_skipped` | `regeneration_skipped.depleted + regeneration_skipped.capacity` |
| `agent_initialized` | the length of the run record's `agents` array |
| — | `survivors + deaths` equals that same length |

10.2 That is **seven** checks, and `crosschecks_passed` states how many passed. All seven held in all 35 streams
measured at `c90edc9`, with zero mismatches, which is why they are required rather than advisory.

10.3 **A failed check is a cell failure**, reported under rule 13 with the two disagreeing figures named. The row is not
retained with the disagreement recorded, and neither figure is preferred: rule 8.6 makes a disagreement "a defect the
stream itself reveals", and a batch that retained the row would be deciding which of the engine's two accounts to
believe.

10.4 The driver adds no check of its own invention here and re-implements no simulation rule. Every equality above is
one the engine's own contract already asserts.

### 11. The digest

11.1 `digest_algorithm` in the `sweep` record is `sha256`. It is stated in the file so a later change of algorithm does
not make old rows ambiguous.

11.2 `stream_sha256` in each `cell` record is the digest of the stream's exact bytes as the engine wrote them, computed
before any decoding, normalisation or line-ending change.

11.3 The digest is over the stream, never over the row. A row hashing itself would prove nothing about the run.

11.4 Re-executing a row's coordinates on the same commit reproduces the stated digest, in either build profile. Both
directions are measured: a re-run at the same seed reproduces the stream and the standard output byte for byte, and a
debug build and a release build of the same cell both hash to `ed1da4b6…`.

11.5 A digest that does not reproduce is a **defect to report**, meaning either the engine changed without its version
changing or determinism has broken. Neither instrument offers to update a stale digest in place.

11.6 No `cell` record carries a filesystem path, a hostname, a process identifier, a wall-clock time, an elapsed
duration or an environment value. `SPEC-MOK-006` rule 8.8's reasoning holds here: ticks are the only clock, and a
retained row that carried a duration would not be comparable between machines.

### 12. The `batch` record and completeness

12.1 Exactly one `batch` record, last. Its shape:

```json
{"record":"batch","complete":false,"requested":60,"retained":57,
 "missing":[{"source":"social","density":"0.10","seed":7,"stage":"run","reason":"engine exited 2: <the engine's own message>"},
            {"source":"social","density":"0.10","seed":8,"stage":"crosscheck","reason":"food_consumed 407 != consumed sum 408"},
            {"source":"social","density":"0.10","seed":9,"stage":"not_attempted","reason":"interrupted by operator"}],
 "leftover_streams":[]}
```

12.2 `complete` is `true` only when `retained` equals `requested`. A reader must not have to infer completeness from the
absence of a warning, so a complete batch states `"complete":true` explicitly.

12.3 Every missing cell is named by its full coordinates with a `stage` and a `reason`. `stage` is a closed vocabulary:
`refused` (rejected before execution), `run` (the engine failed), `read` (the stream could not be read), `crosscheck`
(rule 10.3), `derive` (the row could not be built), `not_attempted` (the batch stopped first).

12.4 No missing cell is filled with a zero, a default, an interpolation, a neighbouring cell or a row from another
sweep.

12.5 Rows for cells that succeeded are retained and valid. One cell's failure does not discard another cell's row.

12.6 A batch in which every cell failed writes the `sweep` and `batch` records, no `cell` records, and `complete` is
`false`.

12.7 Where the retained output itself cannot be written the driver fails under rule 15 and retains nothing, rather than
retaining rows whose completeness is unrecorded.

### 13. Cell failure

13.1 A cell fails when it is refused, when the engine exits non-zero, when the stream cannot be read, when a rule 10
check fails, or when the row cannot be derived. It produces no `cell` record and one `missing` entry.

13.2 The reason carries the engine's own message verbatim where the engine produced one. The driver does not translate,
summarise or improve it.

13.3 A cell failure does not stop the batch. The remaining cells execute.

### 14. Exit statuses

14.1 The batch driver's exit statuses:

| Status | Meaning |
|---|---|
| `0` | every requested cell produced a row; `complete` is `true` |
| `3` | the batch ran and at least one cell is missing; `complete` is `false` and the output is written |
| `2` | usage or configuration error under rule 2.9 or rule 4; nothing executed and nothing was written |
| `1` | the driver could not complete its own work — the output was unwritable, or the binary was not found |

14.2 `3` is distinct from `0` so an automated caller cannot consume an incomplete sweep by ignoring output, which is
`REQ-MOK-083`'s requirement.

14.3 `2` matches the engine's own status for a configuration error, so an operator sees one convention.

### 15. Usage and configuration errors

15.1 Refused before anything executes, with status `2`: an unknown option; a malformed seed specification; an empty
axis; a duplicate value on any axis; a density or source the engine does not accept; `llm` among the sources; a
non-positive tick horizon; and `--out` naming a path under `docs/engineering/`, so that no batch can write into
governance artifacts.

15.2 The driver removes no file it did not create, and creates no directory it was not given.

### 16. The classifier's command line and the outcome vocabulary

16.1 Invocation is `python scripts/classify_simulation_runs.py <retained output> [options]`.

16.2 `--group-by <axes>` names the axes to aggregate over, comma-separated from `source`, `density`, `seed`. Absent, the
default is `source,density`.

16.3 `--format json|markdown` selects the output. `json` is the machine-readable form and carries integers and
closed-vocabulary values only. `markdown` is a rendering, is regenerable, and is not the authority for any figure.

16.4 The classifier reads the retained output and nothing else. It executes no process and reads no stream, so it works
in a checkout with no engine binary present.

16.5 **The outcome vocabulary has five members and the predicates are ordered. The first match wins, and the order is
part of the definition.** Every predicate reads only fields rule 8.1 retains.

| Order | Class | Predicate | Measured support at `c90edc9` |
|---:|---|---|---|
| 1 | `extinction` | `survivors == 0` | 15 of 35 runs — every `baseline` cell, and every cell at density `0.10` |
| 2 | `famine` | `regeneration_skipped.depleted > 0` | **none. See rule 16.8.** |
| 3 | `asymmetric_collapse` | exactly one territory has `population == 0` and another has `population > 0` | none observed in 35 runs |
| 4 | `collapse` | `deaths * 2 >= roster` | 7 of 35 runs — one `social` cell at `0.75` and every cell at `0.25` |
| 5 | `coexistence` | none of the above | 13 of 35 runs |

16.6 Exactly one class per run. The ordering is what makes the class counts within a group sum to the group's row count,
which `REQ-MOK-082` requires.

16.7 The classifier states, per row, which ordered clause assigned the class, so a class is traceable to its clause
rather than to the classifier as a whole.

16.8 **The `famine` predicate is stated and measurement does not support it, and that is disclosed rather than
resolved.** `regeneration_skipped.depleted` was **0 in all 35 runs**, at every density including `0.10`. More than
that: at `0.10`, where the population goes extinct in every seed, the final metrics record shows **8 units of food
still standing in each territory and `depleted` false**. So extinction at low density is not resource exhaustion — the
population fails to *reach* food rather than to have it, and the predicate above would not have fired even in the runs
where starvation is the intuitive reading. The predicate is retained because permanent depletion is a real engine state
and a class that named it is the honest place for it; it is recorded as **unreached in the declared sweep**, and
`VER-MOK-019` carries the finding. Inventing a threshold over standing supply to make the class fire would be asserting
a semantics no measurement supports, which `REQ-MOK-081` forbids.

16.9 A class observed zero times is reported with a count of zero and is not omitted, because an unobserved class is a
finding about the simulation or about the sweep.

16.10 A row matching no predicate cannot occur while clause 5 is the residual; if the clause set is ever edited so that
it can, such a row is reported as unclassified with its facts named, and is neither defaulted nor dropped.

16.11 **Every threshold and every clause above lives in `scripts/classify_simulation_runs.py` and in this
specification, and in no retained artifact.** Editing one changes classes and changes no retained row, which is
checkable by digesting the retained output before and after the edit. This is `SPEC-MOK-006` rule 8.7's requirement
discharged.

### 17. The distribution

17.1 For each group the classifier states: the row count, the count of each of the five classes and of any
unclassified rows, and for `survivors`, `deaths` and each of the three `effective` counters, the minimum, median and
maximum over the group.

17.2 **No figure in the machine-readable output is a floating-point value.** Dispersion is stated as order statistics
rather than as a mean or a standard deviation, on `SPEC-MOK-006` rule 12.4's reasoning that a formatted decimal's bytes
vary by platform, and because a mean over twenty runs invites more confidence than twenty runs support. The median of
an even-sized group is the lower of the two central values, stated here so it is not an implementation accident.

17.3 `attempted` and `effective` stay distinct through aggregation. No group figure merges them.

17.4 The output restates the `sweep` record in full, and states the batch's `complete` flag and its missing-cell count.
An incomplete sweep's distribution **cannot be read without its incompleteness**, which is `REQ-MOK-083`'s
requirement.

17.5 Rows carrying more than one distinct `engine` value are reported as separate groups, with the version as an
implicit grouping axis. They are never merged silently: two engine versions are two experiments.

17.6 A group with one row is reported, with its order statistics all equal. Small groups are not suppressed, because
they are exactly the cells a sweep failed to populate.

17.7 The output states no conclusion, verdict, expectation, target distribution or passing shape. It states counts. A
required distribution would contradict `INT-MOK-001`'s claim that outcomes are not predetermined.

17.8 A distribution over retained output whose `sweep` record is absent is refused. Unknown completeness is treated as
incomplete.

### 18. Determinism and the value alphabet

18.1 Two batches over the same sweep on the same commit produce **byte-identical** retained output. Rule 4.3's ordering
and rule 6.1's line endings are what make that hold under `--jobs` greater than 1.

18.2 Two classifier runs over the same retained output with the same options produce byte-identical output.

18.3 Every value in retained output and in machine-readable classifier output is an integer, a closed-vocabulary
string, a lowercase hexadecimal digest, or the engine's own message in a rule 13.2 reason. No floating-point value
appears anywhere.

18.4 The engine's own message in a reason is the single place either instrument emits text it did not author. It is
carried verbatim, and it is the reason rule 18.3 names an exception at all.

### 19. Division of labour

19.1 The driver executes and derives; it classifies nothing and holds no threshold.

19.2 The classifier classifies and aggregates; it executes nothing, reads no stream, and never writes into retained
output. **A retained row is immutable once written.**

19.3 Neither instrument modifies the engine, its sources, its manifests, its lockfile, any file under
`docs/engineering/`, or any workflow.

19.4 Neither instrument makes a network call, reads a credential, or accepts a live selection. Neither can reach a
provider: no connector option is passed under rule 4.4 and `llm` is refused under rule 4.2.

### 20. Tests

20.1 Each instrument's test file covers, at minimum: the default sweep's cell enumeration and ordering; a partially
defaulted sweep's `defaulted_axes`; the refusal of `llm`; every rule 15.1 usage error; each of rule 12.3's six failure
stages; the three effectiveness tests of rule 9.2, including a zero-effect case; a rule 10 cross-check failure
producing a missing cell rather than a row; each of the five ordered predicates of rule 16.5 including the ordering
between two that could both match; the invariance of retained rows under a threshold edit; and each exit status of rule
14.1.

20.2 The tests execute no real engine run. They read fixture streams, which keeps them fast and keeps the suite
independent of a built binary.

20.3 The instruments' tests are Python and are not part of the Cargo test suite. `SPEC-MOK-002`'s census of Rust test
targets is untouched.

## Explicitly unspecified decisions

- **Whether the run record should gain cumulative conflict counters.** It would make rule 9's counts self-checking
  under `SPEC-MOK-006` rule 8.6 and remove the need to read a 3 MB stream, at the cost of a schema increment from 3 to
  4, an engine change, and moving every retained stream digest in the repository. Considered and deferred on
  2026-08-30; `ADR-MOK-008` option 4 records it.
- **Whether the threat mechanism should be repaired.** Rule 9.6's measurement says it is inert in 1,447 of 1,448
  firings. That is a defect in the simulation, its repair changes simulation rules, and it is a separate chain. This
  specification measures it and does not touch it.
- **What the `famine` predicate should be, if permanent depletion is unreachable.** Rule 16.8 states the finding. Any
  replacement predicate is a semantics decision for the product owner over a measurement that does not exist yet.
- **Whether a batch runs in continuous integration.** Nothing here forbids it and nothing requires it. The default
  sweep's measured 35 s makes it feasible; whether a distribution is a gate is a separate question, and rule 17.7's
  refusal to state a passing shape is why it is not obviously one.
- **Whether any sweep's retained output is committed.** The work order decides what evidence it retains. Rule 15.1
  forbids a batch to write under `docs/engineering/` in any case, so committing output is a deliberate copy rather than
  a side effect.
- **Any threshold's value beyond rule 16.5's**, and any additional class. Both are amendments here.
