# `WO-MOK-013` — the manual assessments of `VER-MOK-013`

| Field | Value |
|---|---|
| Work order | `WO-MOK-013` (crates policy, declared-set comparison) |
| Verification contract | `VER-MOK-013` |
| Baseline | `ff3a155f3ce006fdc38abb62df3fca4a2c3c3aa3` (`origin/master`) |
| Branch | `governance/adr-mok-006-third-party-crates` |
| Date of this record | 2026-08-20 |
| Written by | the implementation agent, which records measurements and states which role each judgement belongs to. **It makes none of them.** |

`VER-MOK-013:220` states: *"Each of the following is an explicit judgement recorded by the accountable role. An
unrecorded assessment is an outstanding assessment, and this contract is not satisfied while any remains outstanding — as
`VER-MOK-011`'s fifth still is."*

For each assessment below: whose it is, what approved text bears on it and where, what is measured and in which file,
and its status. Five of the six are now recorded and one is not yet due.

**How the five were recorded, stated plainly because it bears on whether they are the owner's, and stated per assessment
because they were not all recorded the same way.** The implementation agent measured, then put each open judgement to the
repository owner as a question with the candidate wording written out in full, and the owner chose. The agent drafted
wording and the owner made the judgement. An agent may present text for a role to adopt; it may not sign for the role,
and it did not.

- **Assessments 1, 2 and 3.** The blockquote under each is the owner's selected text, reproduced **verbatim**. The only
  change is typographic — where the selection was rendered in plain monospace and used capitals for emphasis, this file
  uses this repository's bold and backticks.
- **Assessment 6.** The owner's act was a choice between four bases `VER-MOK-013:254` enumerates, taken as *accepted on
  the admission-not-arrival ground*. **The wording in `SPEC-MOK-003` is the agent's, written from that decision**, and
  this file quotes the specification rather than presenting agent prose as an owner quotation. The grounds and the limit
  are the decision; the sentences carrying them are not owner text and are not claimed to be.
- **Assessment 4.** Recorded by the approval of 2026-08-20 on the reading the entry states, with no separate owner
  wording, and the reading is set out there so it can be rejected rather than assumed.

## The numbering, before the assessments

`VER-MOK-013`'s *Manual assessments* list is numbered **1, 2, 3, 4, 6, 7**. There is no assessment 5. `WO-MOK-013`
originally said *"seven"* twice and *"Assessments 1 to 5 and 7"* once, naming a fifth the contract does not contain.

**Six assessments exist.** The repository owner resolved the discrepancy on 2026-08-20 as engineering owner, in the
direction that six is the intended set and *"seven"* and *"1 to 5"* were the errors: `WO-MOK-013` is amended to say six,
with a dated note in its *Lifecycle* section recording it. `VER-MOK-013` is not renumbered and the gap stays.

**Renumbering was not available**, which is why the gap is preserved rather than closed. Assessment numbers are cited by
number in approved artifacts and in the checking program:

| Assessment | Cited by number in |
|---|---|
| 1 | `VER-MOK-013:104`, `:304`, `:339` |
| 2 | `VER-MOK-013:105`, `:304` |
| 3 | `SPEC-MOK-003:800`; `VER-MOK-013:186`, `:327`, `:336` |
| 4 | `WO-MOK-013:303` |
| 6 | `SPEC-MOK-003:59`, `:794`, `:818`; `SPEC-MOK-005:23`, `:24`; `scripts/check_declared_dependencies.py:1019`, `:1024`; `VER-MOK-013:96`, `:139`, `:190`, `:331` |

Renumbering would break twenty citations in approved artifacts and in code. Inventing a fifth would have been worse: it
would create an owner obligation that no approved text states, which is not an implementation act. The headings below
are therefore at the contract's numbers, gap included.

One reading `WO-MOK-013` left open is now answered by the same amendment. Its exclusions bullet said **"The two manual
assessments this change itself creates"** and named only assessment 6; on the contract's own text, assessments **3** and
**4** are also created by this change rather than inherited — the by-name scan did not exist before this work order, and
the baseline substitution arose while measuring. The bullet now speaks of every assessment the contract states rather
than of two, so nothing turns on which of the two was meant.

## 1. No declared entry implements simulation semantics — technical owner — **RECORDED**

> Recorded 2026-08-20 by the repository owner acting as accountable technical owner.
>
> The engine package's declared set is empty, so it has no entry to judge. The observer's has one: `ratatui 0.30.2`, a
> terminal rendering library. It implements none of the rules `SPEC-MOK-001` fixes, no world model and no agent
> decision-making; it owns and advances no entropy; it performs no action validation. It computes no simulation value.
>
> **The line for future entries:** a declared crate may compute the **presentation** of a value the engine produced. It
> may not compute the value. A crate that would need to know a rule of `SPEC-MOK-001` to do its job is refused under
> decision 11 regardless of how well it meets decision 1's criteria.

The approved text that bears on it, all approved by the repository owner acting as accountable **technical owner** in
the act that accepted `ADR-MOK-006`:

- `ADR-MOK-006` decision 11 (`ADR-MOK-006.md:287`) states the prohibition in full and states at `:293` that it *"is a
  review obligation, not a graph read, and item 7's checks cannot see it"*.
- `ADR-MOK-006`'s *Negative* consequences (`:601`): *"`Does this crate implement simulation semantics` has a clear
  answer for a terminal backend and a debatable one for, say, a spatial-index or fixed-point-arithmetic crate."*
- `ADR-MOK-006` decision 2 (`:243`): *"`ratatui`'s pin is unaffected: it is already a declared-set entry and stays
  one."*
- `ARCH-MOK-001.md:139`, the new conformance check: *"Review each crate in the engine package's declared set against
  what it supplies, and confirm that no declared entry implements simulation semantics, owns or advances entropy, or
  validates an action."*
- `VER-MOK-013.md:227`, approved as assurance owner: *"With one entry declared, the assessment is about `ratatui`: a
  terminal rendering library that computes no simulation value."*

What is measured, so the judgement is against figures rather than an assurance:

- the engine package's declared set is **empty**, and its resolved graph is **one crate — itself — on all three
  release targets** (`WO-MOK-013-graphs.txt`, `WO-MOK-013-counts.txt`). There is no engine entry to judge;
- the observer package's declared set is **one entry, `ratatui 0.30.2`**, `default-features = false`, features
  `crossterm`, `layout-cache`, `underline-color` (`WO-MOK-013-features.txt`). It is the only crate the judgement
  reaches: decision 11 binds the *declared set*, not the transitive graph;
- no simulation code is in the observer at all — `ARCH-MOK-002`'s dependency direction is one-way and the engine does
  not depend on `mokiterions-tui` on any target (`WO-MOK-013-graphs.txt`).

**What the recorded text adds beyond the present entry.** The judgement on `ratatui` was already legible in the
accepting text — a terminal backend has *"a clear answer"* — which is why an earlier draft of this file offered it as a
reading. The owner recorded the assessment separately and added the test for future entries, which the accepting text
does not contain: **presentation of a value the engine produced is admissible, computing the value is not.** That line
is what makes this assessment usable at the next admission rather than a restatement of the easy case, and
`VER-MOK-013:228` is what asked for it — *"the assessment is recorded even so, because an assessment that is only made
when it is difficult is a habit nobody has."*

## 2. The criteria of decision 1 were applied to each entry — technical owner — **RECORDED**

> Recorded 2026-08-20 by the repository owner acting as accountable technical owner. Applied retrospectively and not
> grandfathered.
>
> **Stable, well-maintained:** `ratatui 0.30.2` is pinned exactly, resolves reproducibly on all three release targets
> from the committed lockfile, and has been in this repository since 2026-08-17 without a forced version move.
>
> **Accelerates delivery, standard non-core feature:** a terminal rendering layer is the textbook case of decision 1's
> second sentence. Writing one by hand would consume engineering effort the simulation needs and would produce no
> proprietary value.
>
> **Dependency debt, measured rather than asserted:** 57 crates on Windows, 63 on Linux, 62 on macOS, 66 in union;
> 7/9/9 executing a build script; and the `mio`/`net` capability recorded in the disclosure table. That is the debt this
> entry carries and it is accepted as proportionate for an entire presentation layer confined to the observer.
>
> It is also the **yardstick**: the next admission is measured against this surface, and a crate proposing comparable
> debt for a narrower benefit is refused.

`VER-MOK-013:235` asked for exactly this: *"For `ratatui` the record is `ADR-MOK-003`, which decided it before these
criteria existed; the owner records whether that decision satisfies them retrospectively rather than assuming a pinned
crate is grandfathered."*

**No approved text discharged it, which is why it had to be recorded rather than read.** Checked before relying on it,
because a discharge here would have been convenient:

- `ADR-MOK-006` decision 2 (`:243`) says the pin *"is unaffected"* and *"stays"* a declared entry. That is a statement
  that the criteria were **not** applied to it, not a statement that they were.
- `ADR-MOK-003`'s note dated 2026-08-20 (`ADR-MOK-003.md:29`) says the version, `default-features = false`, the three
  features, the 57-crate surface and the `serde`-off clause are *"untouched"*, and that what changes is the word
  *"only"*. It does not weigh `ratatui` against *stable*, *well-maintained*, *dependency debt* or *standard, non-core*.
- `ADR-MOK-006`'s *Required amendments* entry for `ADR-MOK-003` asks the note for precisely that content and no more.

So this assessment was outstanding **beyond what `WO-MOK-013` anticipated**, and it is now recorded. It was the
assessment most exposed to the *grandfathering* the contract names: `ratatui` is the one crate in the repository, and if
the criteria were never applied to it, the first crate they were applied to would be judged against no precedent. The
recorded text closes that specifically by naming the measured surface as the yardstick.

What was measured, and what the owner judged against:

| Measurement | Value | File |
|---|---|---|
| Declared version, features | `ratatui 0.30.2`, `default-features = false`, `crossterm` + `layout-cache` + `underline-color` | `WO-MOK-013-features.txt` |
| External crates in the observer's graph | 57 Windows · 63 Linux · 62 macOS, 66 in union, counting crate identity as (name, version) | `WO-MOK-013-counts.txt` |
| Crates executing a `build.rs` | 7 Windows · 9 Linux · 9 macOS, 10 in union | `WO-MOK-013-build-scripts.txt` |
| Transitive prohibited-class names | `mio 1.2.2` (`net` on), `signal-hook-mio 0.2.5`, on Linux and macOS | `WO-MOK-013-scan.txt` |
| Advisory and licence status | **not measured, expressly out of scope** by `REQ-MOK-047` | — |

Nothing in that table decides the assessment; *excessive dependency debt* has no threshold, by decision 10, which is
why this is a judgement and why no count here stands in for it. What the recorded text does is make the counts the
comparison for the next one, which is the strongest thing available in the absence of a bar.

## 3. The reach of the by-name scan — assurance owner — **RECORDED**

> Recorded 2026-08-20 by the repository owner acting as accountable assurance owner.
>
> **Reach:** the 126 terms cover the six capability classes `ADR-MOK-006` decision 4 prohibits as far as a name can.
> They were written from the prohibitions and not from the resolved graph, and the raw hits before disclosure plus the
> positive control are retained so a later reader can verify that.
>
> **What it cannot see, stated as a limit and not a caveat:** a crate can open a socket without saying so in its name,
> and a transitive crate can do it without appearing in any declaration. Token matching also misses a capability carried
> under an unrelated name. **No passing scan is proof of absence**, and this repository does not treat it as one.
>
> **What stands behind the blind spot:** the technical owner's per-entry judgement at admission (assessments 1 and 2),
> decision 11's review obligation, the offline build and test with a failing control, and the disclosure obligation of
> rule 8.4d which refuses a transitive prohibited-class name until it is recorded and judged.
>
> **Trigger:** the term list is re-read at each admission rather than assumed complete. A crate admitted for a
> capability the list does not name is what would make it stale.

`VER-MOK-013:240` asked for it: the owner records that the term list *"covers the capability classes decision 4
prohibits as far as a name can, states what it cannot see, and confirms that no passing scan is being treated as proof
of absence."*

**Why no approval could have discharged this one.** The instrument being assessed was written after the approval. The
scan's term list lives in `scripts/check_declared_dependencies.py`, written during this work order; no approved
artifact enumerates it, and the implementation agent cannot assess its own instrument. That is the same shape as
`VER-MOK-011`'s outstanding fifth, and it is the second assessment outstanding beyond what `WO-MOK-013` anticipated.

What is measured, from `WO-MOK-013-scan.txt`, whose terms are printed by the program itself so the evidence cannot
disagree with what ran:

| Prohibited class, from decision 4 | Terms |
|---|---|
| network access | 38 |
| asynchronous runtime | 11 |
| database | 21 |
| model provider | 15 |
| credential handling | 12 |
| plugin system or dependency injection | 7 |
| user-interface terms, engine graph only | 22 |

104 prohibited-class entries across the six classes decision 4 names, plus the 22 user-interface terms — **126** in
all, the figure the recorded assessment cites; `tokio` is listed under two classes. The provenance is stated in that
file: the terms were written from decision 4's prohibitions and not from the present graph, *"a term list derived from
the graph would pass by construction."*

- **Raw hits before any disclosure**: `mio 1.2.2` and `signal-hook-mio 0.2.5`, on Linux and macOS, both in the network
  access class. Retained precisely so a later reader can see the list was not written around the graph.
- **Matching is by token, not substring.** `name_tokens` splits on `-` and `_`; `signal-hook-mio` yields
  `['hook', 'mio', 'signal', 'signal-hook-mio']`. The near-miss list — `ratatui`/`ratatui-core`/`ratatui-crossterm`/
  `ratatui-widgets` all containing `tui`, and `windows-link` containing `ws` — is what a substring rule would have hit.
- **A positive control is retained**: `tokio` matches two classes, the four `ratatui*` crates match the
  user-interface list, `signal-hook`, `windows-sys` and `windows-link` match nothing.
- **What the scan cannot see, stated beside the passing result** (`VER-MOK-013:185`): *"a crate can open a socket
  without saying so in its name, and a transitive crate can do it without appearing in any declaration."*

The four compensating controls the recorded text names are each real and each locatable: assessments 1 and 2 above;
decision 11 at `ADR-MOK-006:287` with its conformance check at `ARCH-MOK-001:139`; the offline build and test with the
failing control in `WO-MOK-013-injection.txt`; and rule 8.4d at `SPEC-MOK-005`, whose disclosure obligation is what
produced assessment 6 rather than a silent pass.

## 4. The determinism baseline substitution — assurance owner — **RECORDED**

**Status: recorded by the approval of 2026-08-20, on the reading below.** The precision correction this file previously
raised has since been applied to the two artifacts that carried the loose wording, under the owner's decision of
2026-08-20.

The approved text, approved by the repository owner acting as accountable **assurance owner** in the act that approved
`VER-MOK-013`:

- `VER-MOK-013:49` (oracle 3) states the substitution, and `:59` states *"`VREC-MOK-002` is **not edited**, and neither
  is the evidence it binds"*;
- `VER-MOK-013:103` requires the retained evidence to state *"the two approved amendments that moved the stream"*, name
  the binding record, and show the inequality;
- `WO-MOK-013`'s *Required verification* paragraph states the same in full and names this assessment as the assurance
  owner's, at `:303`.

What is measured (`WO-MOK-013-determinism.txt`, `WO-MOK-013-determinism-manifest.txt`), seed `123`, 1,000 ticks, trace
off:

| Configuration | 2026-08-17, retained under `VREC-MOK-002` | Candidate tree, twice each | Equal to `WO-MOK-011`'s retained post-capture |
|---|---|---|---|
| reference `0.75%` | `97e0581c…` | `cebe44c4…` | yes |
| reference `1.50%` | `58b7edc1…` | `9621f5f8…` | yes |
| baseline `0.75%` | `82aa98b3…` | `fcd03d6f…` | yes |
| baseline `1.50%` | `85f052bb…` | `44a448a1…` | yes |

All four differ from the 2026-08-17 figures and all four equal `evidence/WO-MOK-011/post/post-manifest.txt`. The
comparison was made over all **90** manifest cells, not these four. The two amendments that moved the stream are named:
`WO-MOK-010` added the `fear` trait and `WO-MOK-011` added `name:` to the text record, both verified as changing every
replay by design.

**The precision correction, and what happened to it.** `ADR-MOK-006` and `VER-MOK-013` spoke of *"`VREC-MOK-002`'s four
replay hashes"*, and in one place of *"the four configurations `VREC-MOK-002` names"*. The record
`verification-records/VREC-MOK-002.md` contains **no replay hash and names no configuration** — its only SHA-256 is
`artifact_snapshot_sha256 = "35a66d90…"`, and its own figures are survivor counts (*8, 11, 8, 9, 11*). The four hashes
are in `evidence/WO-MOK-002/determinism-and-resilience.md:12–15`, which `VREC-MOK-002` binds through its
`evidence_paths`, together with the four configurations at seed `123` for 1,000 ticks. This file raised it as a
correction for the owner to see before relying on the substitution. **The owner decided on 2026-08-20 to correct both
artifacts**: five statements in `VER-MOK-013` and one in `ADR-MOK-006` now read *"retained under `VREC-MOK-002` in
`evidence/WO-MOK-002/determinism-and-resilience.md`"*, each under a dated amendment row or note. `VREC-MOK-002` itself
is still not edited, and neither is the evidence it binds: both were correct at the commit that record names. The
substitution is unaffected either way — the hashes exist, at the commit and date claimed — and what changed is where a
reader is sent to find them.

## 5. — does not exist

See *The numbering* above. This heading is present so that a reader who is counting does not conclude that an
assessment was dropped from this file.

## 6. The disclosed transitive capabilities — technical owner — **ACCEPTED**

**Recorded 2026-08-20 by the repository owner acting as accountable technical owner: both disclosed capabilities are
accepted.** `SPEC-MOK-003`'s disclosure table at `:794` carries the acceptance and its grounds, because that is where a
reader looking at the crate will land; `SPEC-MOK-003:59` is the amendment row, `SPEC-MOK-005:24` corrects the one
statement of current state that said otherwise, and `VER-MOK-013` records it in its matrix at `:96`, in its
*Manual assessments* entry at `:250` and in its residual-uncertainty bullet at `:331`.

The recorded grounds, as `SPEC-MOK-003:794` states them:

- **`ADR-MOK-006` decision 4 prohibits *admitting* a crate in this class, and neither crate is admitted.** Both arrive
  transitively inside a graph `ADR-MOK-003` accepted on 2026-08-17. This is the *admission, not arrival* reading, and it
  is the reading the ADR's own wording carries rather than a relaxation of it.
- **No observer behavior uses the socket types `net` compiles in.** The observer opens no socket, binds no port and
  resolves no name.
- **The capability is disclosed here rather than filtered out of the scan, so the acceptance is auditable.**

**The limit is part of the acceptance and is stated in the same cell**: *"What is accepted is a compiled and uncalled
capability, not network access. If any behavior of either package ever uses it, this row is void and `REQ-MOK-026`'s
prohibition applies with nothing further required."* `signal-hook-mio 0.2.5` is accepted on the same grounds and by the
same role; it carries no socket type of its own and is disclosed because the scan matches the `mio` token in its name,
and a hit that is silently filtered is not a hit.

What was measured before the judgement (`WO-MOK-013-scan.txt`, `WO-MOK-013-graphs.txt`):

- **`mio 1.2.2`**, reached on Linux and macOS through `ratatui` → `ratatui-crossterm` → `crossterm`. Resolves with
  features `default`, `log`, `net`, `os-ext`, `os-poll`; **`net` compiles in TCP and UDP socket types.** The observer
  uses the poll to wait for terminal input and signals, which is `crossterm`'s use of it.
- **`signal-hook-mio 0.2.5`**, beside it, carrying no socket type of its own.
- Not reached on `x86_64-pc-windows-msvc`: the capability is compiled into **two of the three** release builds.

`VER-MOK-013:254` enumerated four bases the owner could record. Three were taken together and the fourth — *"that it is
not accepted and `crossterm`'s event source must change"* — was not.

**What this assessment did not do.** It removed nothing from any build. `mio`'s `net` feature still compiles TCP and UDP
socket types into the Linux and macOS observer, exactly as before, and `VER-MOK-013:331` says so in its residual
uncertainty. The acceptance is a judgement about a compiled and uncalled capability and is void the moment a behavior
calls it.

**And the disclosure stays printed.** `check_declared_dependencies.py` reported disclosures only when their assessment
text contained *outstanding*; under that logic, recording the acceptance would have made the line vanish from the
output — an acceptance indistinguishable from the disclosure never having existed, which is the state rule 8.4d's table
exists to prevent. The program now prints `disclosed and accepted` where it printed `disclosed and OUTSTANDING`
(`:1019` and `:1024`) and prints the row either way. That change was made because the acceptance was recorded, and it
is the reason this file's own evidence had to be re-captured rather than edited.

**This was the assessment the implementation produced rather than anticipated.** The crates were in the graph before
`ADR-MOK-006` and nothing in this repository had looked. **An implementation agent may measure a transitive capability
and may not accept one** — which is why the disclosure table exists instead of a term list with `mio` left out of it,
and why the alternative that would have made this file shorter was the one that had to be refused.

## 7. The strength this change gives up — technical owner — **not yet due**

**Not recorded, and its trigger has not occurred.** `VER-MOK-013:265` conditions it: the owner records, *"at the first
admission of any crate beyond `ratatui`"*, that the check has in fact been running — in both placements, on all three
targets — *"rather than discovering at admission time that one placement had been failing silently."*

No crate beyond `ratatui` is admitted by this change. The engine's declared set is empty and the observer's has one
entry, so the condition is unmet and there is nothing to record.

**The tension, for the owner.** `VER-MOK-013:220` says without qualification that an unrecorded assessment is
outstanding and that the contract is not satisfied while any remains. Read literally with assessment 7, the contract
cannot be satisfied until a crate is admitted, which is not a plausible reading of a contract written to govern a
repository that may never admit one. Recorded here as **not yet due** rather than as satisfied or outstanding, with the
reading flagged rather than chosen. What *is* measured today, so the first admission has something to check against:
both placements run rule 8.4's checks on all three targets from one declaration, and neither names a crate, a version
or a feature (`WO-MOK-013-workflows.txt`); the check refuses in ten distinct ways when a declaration is edited
(`WO-MOK-013-injection.txt`).

## Summary

| # | Assessment | Role | Status |
|---|---|---|---|
| 1 | No declared entry implements simulation semantics | technical owner | **Recorded 2026-08-20**, with the line for future entries stated |
| 2 | Decision 1's criteria applied to each entry | technical owner | **Recorded 2026-08-20**, retrospectively and not grandfathered, with the measured debt as the yardstick |
| 3 | The reach of the by-name scan | assurance owner | **Recorded 2026-08-20**, insufficiency stated as a limit, four compensating controls named |
| 4 | The determinism baseline substitution | assurance owner | **Recorded** by the approval of 2026-08-20; its precision correction has since been applied to both artifacts |
| 5 | — | — | Does not exist in `VER-MOK-013` |
| 6 | The disclosed transitive capabilities | technical owner | **Accepted 2026-08-20**, on three grounds, limited to a compiled and uncalled capability |
| 7 | The strength this change gives up | technical owner | Not yet due; triggers at the first admission beyond `ratatui` |

**No manual assessment of `VER-MOK-013` is outstanding.** Five are recorded and the sixth has not been triggered.
Whether *not yet due* satisfies a contract whose text says *"not satisfied while any remains outstanding"* is the
reading flagged under assessment 7, and it is the owner's to take; this file does not take it, and a verification record
against this contract is where it would be taken.

Two of the five were outstanding beyond what `WO-MOK-013` anticipated. The work order named only assessment 6 and said
*"is **OUTSTANDING** as of this commit"*; assessments 2 and 3 were also unrecorded, for the reasons stated above rather
than asserted — no approved text applied decision 1's criteria to `ratatui`, and the scan's term list postdates the
approval that would otherwise have discharged its assessment. Both are now recorded, and `WO-MOK-013`'s bullet is
amended to speak of every assessment the contract states.

Separately and unrelated to this change, `VER-MOK-011`'s manual assessment 5 remains outstanding and is not touched,
cleared or inherited here, and neither are the four amendment rows standing **OUTSTANDING** from earlier work in
`ARCH-MOK-001`, `SPEC-MOK-002` (two) and `SPEC-MOK-003` at 2026-08-18, or `SPEC-MOK-004`'s at 2026-08-19.
