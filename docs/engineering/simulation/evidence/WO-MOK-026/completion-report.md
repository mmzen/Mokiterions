# `WO-MOK-026` completion report

Stage 5b: the connector, the live path, the two gates, the usage accounting and the spend ceiling.

    work order   WO-MOK-026
    candidate    6e9ca13ba70ec46696113bb742f45d78d602d41e   (the tree every measurement was taken at)
    base         cc54185                                     (the commit this stage builds on)
    merge base   d96cced                                     (with main)
    branch       feature/wo-mok-026-live-path
    written      2026-08-29

The twelve sections below are this work order's *Completion report format*, in its order and with its
headings' substance. Two things are stated here rather than left to be found:

**`VER-MOK-018` case `L15b` FAILED.** The cache ratio is **0.00** against an obligation of **0.85**.
Section 6 states it before any explanation, as this work order's format requires, and section 8's row
carries it. The owner's disposition of 2026-08-29 is that `REQ-MOK-070` is recorded as outstanding and
nothing is amended.

**Every other required case passes or is escalated with its reason.** 39 of 45 pass outright, `L17` and
`L20` pass in part and are escalated, `L29` passes with a deviation recorded, `C6` is the owner's
attestation and `M2` is the owner's assessment and is not re-required.

---

## 1. What was built, against the *In scope* list

Sixteen items. **All sixteen are done.** Four carry a recorded escalation or deviation, named in the
item and taken up in section 12.

**1. The connector, and the protocol an operator needs to write one.** `docs/CONNECTOR_PROTOCOL.md`,
295 lines, created on this branch at `143eaab` and amended twice since — `28b2b04` moved the provider
binding out of the request, `3f838d1` stated the spawn contract. The program itself is outside this
repository and is not committed, per the owner's decision of 2026-08-29. `protocol-document-sufficiency.md`
is the check that the document alone suffices: the canned connector was written against it rather than
against the engine's internals.

**2. The canned connector.** `mokiterions-core/tests/support/canned_connector.rs`, a `[[bin]]` target of
the engine package declared at `mokiterions-core/Cargo.toml:64-66`. Not a third workspace member:
`Cargo.toml:22` still reads `members = ["mokiterions-core", "mokiterions-tui"]`. It is a `[[bin]]` rather
than a plain source file because a connector is a *child process*, which was `WO-HUP-002`'s recorded owner
decision of 2026-08-28 and which admitted `mokiterions-core/Cargo.toml` to this work order's execution
scope. It declares no dependency and adds none to the resolved graph. Section 2 is the whole boundary.

**3. The connector path option and the spawn.** `--connector-path` recognised, validated and enforced
at-most-once by the shared parser at `mokiterions-core/src/cli.rs`, which **discards the value**; the
engine's binary target re-reads the raw argument through `CONNECTOR_PATH_OPTION` at
`mokiterions-core/src/main.rs:60`, spawns the child at `main.rs:448-450`, connects its two pipes and reaps
it at `main.rs:532`. Section 3 is the spawn in full. `S6a` is the check that no configuration field
exists for either new path, and `interface-checks.md` carries it: `Config`'s seven public fields contain
no `String` and no `PathBuf` at all.

**4. The environment pass-through.** The child inherits the parent's environment because
`std::process::Command` does, and the builder at `main.rs:447-450` makes no `.env`, `.env_clear`,
`.env_remove` or `.envs` call — which the comment above it states as the deliberate act it is. Neither
Rust target reads the credential, parses it, logs it or places it in an argument. `spawn-and-passthrough.md`
measures all four claims and `architecture-checks.txt` check 4 measures **0 environment reads anywhere in
either package** beyond `args`, `var` and one `env::temp_dir` in the observer's own test module.

**5. The provider binding, declared in the connector rather than in the engine.** The model identifier
`gpt-5.6-luna`, the reasoning level `none` (the owner's decision of 2026-08-23) and the endpoint
`https://api.openai.com/v1/chat/completions` are all the connector's, and the run's own record stream
shows them arriving from it rather than from the engine. The **unit prices** are the declared exception,
reaching the engine through `--prices <prompt:cached:output:reasoning>` under rule 14.3a. The engine holds
prices; it holds no endpoint, and `architecture-checks.txt` check 9 measures no URL anywhere in either
package.

**6. The observer's refusals.** Five, not three, and section 4 says why. Each exits `2` with 0 bytes of
standard output and a diagnostic naming the host that owns the option, before the terminal is entered.
`mokiterions-tui/tests/options.rs:577 the_refused_list_and_the_engines_own_options_agree` is the drift
check that keeps the list from falling behind the parser it forwards to. **This is the item the work order
called the most easily left undone, and doing nothing here would have reproduced the `--events-path`
defect of issue 40 five times over.**

**7. The two gates.** All four combinations measured against the release binaries, in the engine's binary
target, which is the only host a live run is reachable from at all. Section 5 is the matrix.

**8. The usage accounting.** The provider's four counts per exchange, in the transcript, as reported.
Attempt 2's totals: prompt 776,963, cached_prompt 0, output 9,429, reasoning 0. `an_unreported_count_is_absent_and_not_zero`
at `simulation.rs:14489` holds the absent case, which this run does not exercise because the provider
reported all four on all 503 exchanges.

**9. The cost arithmetic against real usage.** Integer throughout, driven by reported usage.
**And it found two defects in rule 14's model, both amended in this work order under the owner's
dispositions of 2026-08-29**: the double-billing of reasoning tokens (rule 14.2a added), and rule 11.7's
singular phrasing. The cache-write multiplier this item names as open is **now settled from the provider's
billing record** and is recorded as an open finding rather than an amendment, because a fifth price moves
`--prices`' arity. Section 7 and section 12 both carry it.

**10. The spend ceiling.** `--spend-ceiling`, the check that runs before each exchange, the run's end when
it is reached, and the ceiling and accumulated cost in the run record. `CEILING_STOP_EXIT = 3` at
`mokiterions-core/src/lib.rs:80` is the fourth exit status, added so a caller can tell a ceiling stop from
a clean completion and from an error. **A limit is recorded rather than implied**: the ceiling bounds the
engine's *belief* about spend, which on this run was 16.67 cents against 20.55 billed — 81 %.

**11. The cache-ratio report.** Computed from reported usage, in basis points, in the run record.
`cache_ratio_basis_points: 0`. **This item is delivered and its obligation fails**, which are two different
things and section 6 keeps them apart.

**12. Retry.** Bounded at three retries after one initial attempt, each attempt its own transcript record,
exhaustion a counted fallback rather than the end of the run. Exercised against the canned connector,
which fails on command: `connector.rs:822`, `connector.rs:897`, and `retry-evidence.md`. The bound of
three is the owner's decision of 2026-08-29; the retry count, the backoff shape and which failures are
retried are what this work order's envelope delegates.

**13. One owner-authorised live run.** Two happened. Attempt 1 was rejected and attempt 2 is the
measurement; both are retained. Section 6 is the run and section 12 records the retention as an owner
disposition. Its transcript **supplements** `WO-MOK-025`'s synthetic one rather than replacing it, and
that is a measurement rather than a preference: the two are rule 11.3.1's two cases and neither can be
the other.

**14. The block D rendering, measured.** All 503 permitted blocks re-rendered and counted both ways
against a calibrated tokenizer. **The flat form stands, on the owner's disposition of 2026-08-29.** The
answer is not the one the trade-off anticipated: nesting saves 3,967 tokens, which is 19.6 % of block D
and **0.51 % of the run's prompt tokens** — 0.079 cents of a 16.67-cent run. Block D is 2.7 % of the
prompt and the standing instruction block is 84.6 %, so the cost half of the specification's trade-off
does not discriminate between the layouts at all.

**15. The two attestations.** `credential-attestation.md` is `C6`. `authorization-genuineness-attestation.md`
is `L28`'s second half. Both name the owner and the date, and both disclose that they were drafted by the
implementation agent and confirmed by the owner on 2026-08-29.

**16. The amendments this work order requires.** `REPOSITORY_CONTEXT.md`'s restricted-paths bullet gains
the live-run paragraph, including "one such run has now happened". `SPEC-MOK-003`'s *Start-up inputs*
gains the five live-run options with the observer's disposition of each. `SPEC-MOK-004` **rule 11**'s
figures are re-measured at the candidate, twice, each against the tree rather than inferred from an
unchanged total. **`SPEC-MOK-004` rule 1 does not move**, as this item requires: no directory is added.

---

## 2. The connector boundary

**The protocol document as shipped.** `docs/CONNECTOR_PROTOCOL.md`, 295 lines. It states the line protocol
in both directions, the framing, the error shape, the usage fields and the credential rule — that the
connector reads the credential from its own process environment and from nowhere else, and never writes it
to standard output, to standard error, to any file or into an `error.message`.

**The canned connector's language.** Rust, the same as both hosts. It is a `[[bin]]` target of the engine
package, at `mokiterions-core/tests/support/canned_connector.rs`, 3xx lines, with `fn main` at line 301.
The path is `tests/support/` and not `tests/` because Cargo auto-discovers `tests/*.rs` as integration-test
targets and a file directly in `tests/` would be built twice.

**Its dependency declaration.** None. It declares no dependency and it is built from the standard library
alone. `declared-dependencies.txt` is the repository's own gate at this candidate, exit 0, reading
`Mokiterions: 0 declared, 0 in the manifest` and 0 external crates on all three declared targets — so a
target of that package has nothing external to reach a network through. `canned-connector-dependencies.md`
is the direct measurement.

**`S2`'s result and location.** **PASS.** `docs/engineering/simulation/evidence/WO-MOK-026/canned-connector-dependencies.md`,
corroborated by `declared-dependencies.txt` and by `architecture-checks.txt` check 9 (no socket type, no
`connect`, no URL and no HTTP identifier anywhere in either package, at any tier).

**And nothing here establishes anything about an operator's connector.** Stated plainly, as this work
order's format requires and as `VER-MOK-018` itself does. That program is named by the operator, written
outside this repository, never committed, and invisible to every check here: this repository cannot see
what it declares, what it links, what it sends or where it sends it. `SPEC-MOK-007` rule 10.6 withdrew the
standard-library constraint an earlier draft placed on it for exactly this reason, and `ADR-MOK-007`'s
*Negative* consequences record the same limit. `VER-MOK-018` records it as a **limit rather than a gap**,
and section 11 repeats it there because that is where a reader looks for what was not verified.

What stands in its place is the document. `protocol-document-sufficiency.md` is the check that it is
sufficient: the canned connector was written from the document rather than from the engine's internals, so
an operator with the document and no access to this source can write one that works.

---

## 3. The spawn

**Where it lives.** `mokiterions-core/src/main.rs` — the engine's **binary** target — at four sites:
lines 448, 449 and 450 (the builder and its two pipes) and line 532 (`reap`'s `child.wait()`).
`architecture-checks.txt` check 4 measures **4 process sites, all four in that file, and 0 elsewhere in
either package**. The observer's source contains no process spawn at all, which is `REQ-MOK-077`'s
prohibition measured statically rather than assumed from its absence today.

**What the child inherits.** The environment the binary target inherited, untouched. The builder is:

    Spawn::new(&connector)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()

Two pipes and nothing else. **Standard error is left inherited on purpose** — rule 10.2 gives the
connector one channel for protocol and this leaves it one for its own diagnostics, which reach the
operator's terminal without passing through the engine at all. **There is no `.env`, `.env_clear`,
`.env_remove` or `.envs` call on this builder**, and the comment above it names rule 10.5 as the reason.
That absence is the pass-through: a credential in the parent's environment reaches the child because
nothing intervened, not because anything forwarded it.

**How it is reaped.** `fn reap` at `main.rs:531`, called on every exit path. It calls `child.wait()` and
reports a non-zero status or a signalled exit on standard error, and **returns nothing** — a run whose
every exchange succeeded is a successful run even if the child then exited badly on its way out. The
message carries neither of the two severity keywords the artifacts fix, because `configuration error:`
and `runtime error:` are the whole admitted set and inventing a third would put a word in the diagnostic
surface no artifact admits.

**`S3a`'s evidence that neither the library target nor the observer contains one.** `S3a`: **PASS**, at
`spawn-and-passthrough.md` and `architecture-checks.txt` check 4. `S3`: **PASS**, at check 3, a bare
PASS — no filesystem operation, no socket, no spawn and no environment read in `lib.rs`, `cli.rs` or
`simulation.rs`. `mokiterions-core/tests/process.rs:237 the_library_never_opens_the_transcript_it_was_told_about`
is the same property as a test, and `interface-checks.md` reads it off the signatures: `execute` takes the
record sink as an already-open `Write` and the port as an already-built `Proposer`.

**One paragraph of check 4's retained output is stale and is named rather than edited.** Its `limit:`
paragraph says the spawn and the pass-through do not exist yet — true of the previous stage and false at
this candidate, as the same check's own numbers show. It is retained unedited, because editing an
instrument's output is the one thing that would make it worthless, and
`architecture-checks.txt`'s preamble names it along with three other stale paragraphs.

---

## 4. The observer's refusals

**Five, not three.** The work order names three — connector path, live-mode selection, ceiling — and
`--transcript-output` and `--prices` are refused on identical terms, so all five were measured. **A
refusal list with two silent members would be a gap nobody had looked for**, which is why the two extra
are recorded rather than left out of the report.

| option | exit | standard output | standard error |
|---|---|---|---|
| `--connector-path <path>` | **`2`** | **0 bytes** | `configuration error: --connector-path belongs to the Mokiterions binary: …` |
| `--live` | **`2`** | **0 bytes** | `configuration error: --live belongs to the Mokiterions binary: …` |
| `--spend-ceiling 2` | **`2`** | **0 bytes** | `configuration error: --spend-ceiling belongs to the Mokiterions binary: …` |
| `--prices 125:13:1000:0` | **`2`** | **0 bytes** | `configuration error: --prices belongs to the Mokiterions binary: …` |
| `--transcript-output x.jsonl` | **`2`** | **0 bytes** | `configuration error: --transcript-output belongs to the Mokiterions binary: …` |

The message in full, for `--live`; the other four differ only in the option they name:

    configuration error: --live belongs to the Mokiterions binary: this program only replays --policy
    llm, so it starts no connector program, asks no model and spends nothing. Record a live run with
    that binary, then watch it back here with --transcript-path <path>

Then the usage text, 115 lines, on the same stream. Standard error is 6,518 to 6,531 bytes depending on
the option's own length; standard output is empty in all five.

**None was accepted-and-ignored, and that is measured three ways rather than asserted.**

1. **The exit status is `2` and standard output is 0 bytes.** An accepted-and-ignored option would have
   exited `0`, entered the terminal and replayed something, and the operator would have been told nothing.
2. **The refusal precedes the terminal.** `mokiterions-tui/src/main.rs:176` is the one call to `prepare`,
   and a refusal returns from `main` at that `match` without reaching line 188 — so there is no second
   entry point a refusal could have passed through. The captures agree: all five wrote to standard error
   and standard output, where a terminal session would have gone, is empty in all five.
   `a_refused_live_run_enters_no_terminal_and_spawns_nothing` at `mokiterions-tui/tests/options.rs:531`
   asserts the same ordering and asserts no child process.
3. **The list cannot drift behind the parser.**
   `the_refused_list_and_the_engines_own_options_agree` at `options.rs:577` holds the declaration against
   the list the test enumerates **and against the engine's own help text**: every refused option must be
   an option the engine actually accepts. An option in the list the engine does not accept is the worse
   failure, because the refusal would then be about nothing.

Evidence: `observer-refusals.md`, `cases/observer/` with digests in `cases-manifest.txt`, and
`mokiterions-tui/tests/options.rs` at 320, 446, 502, 531 and 577. `L32`: **PASS**. `A7`'s refusal half:
**PASS**.

---

## 5. The gate matrix

All four combinations of the two conditions `REQ-MOK-072` requires — an explicit live-mode selection, and
a credential present in the host process's environment. Measured against the release binaries, from
outside the process. `gate-matrix.md` is the capture; `cases/` holds the four case directories and
`cases-manifest.txt` their digests.

| row | live selected | credential present | outcome | stderr | evidence |
|---|---|---|---|---|---|
| 1 | no | no | **no spawn**, exit 0, the run completes offline | **0 bytes** | `gate-matrix.md`, `cases/row1-unselected-uncredentialled/` |
| 2 | no | **yes** | **no spawn**, exit 0, the run completes offline | **0 bytes** | `gate-matrix.md`, `cases/row2-unselected-credentialled/` |
| 3 | **yes** | no | spawns, then **refuses on the first exchange** | 334 bytes | `gate-matrix.md`, `cases/row3-selected-uncredentialled/` |
| 4 | **yes** | **yes** | spawns and answers | 359 bytes | `gate-matrix.md`, `cases/row4-selected-credentialled/` |

**No provider call occurred in three of the four**, which is the confirmation this work order's evidence
item 7 asks for. Rows 1 and 2 spawn nothing at all, so there is no process that could have made one; row 3
spawns and the connector refuses before any request, its refusal naming the variable an operator has to fix
and never the value.

**What makes rows 1 and 2 measurements rather than absences.** `--connector-path` was given
`cases/no-such-connector` — a well-formed path with no program at it. A run that reached the platform with
that path would exit 1 and say so. It exits 0 and writes 0 bytes to standard error, so the gate closed
before the spawn rather than the spawn having failed quietly.

The tests behind the same four rows: `mokiterions-core/tests/connector.rs:348 no_connector_is_spawned_without_the_live_selection`,
`:406 no_credential_refuses_on_the_first_exchange`, `:234 a_live_run_spawns_the_connector_and_records_every_exchange`,
and `:443 the_credential_value_reaches_no_produced_byte`.

**One clause of `L20` is untested and section 8's row names it**: three of the four `--live needs …`
refusals in the parser have no test, and the empty-credential entry of the connector's one refusal arm has
none either. Both are gaps in coverage, not in behaviour, and both are escalated rather than closed at a
candidate every capture in this packet is bound to.

---

## 6. The live run

**The cache ratio is 0.00 against the eighty-five percent obligation. `L15b` FAILS.** Stated first,
before any explanation, because this work order's format requires it and because a report that makes a
reader hunt for its one failure has hidden it.

    cached_prompt / prompt  =  0 / 776,963  =  0.000000       (0 basis points)

    REQ-MOK-070's obligation is 0.85.  The measurement is 0.00, on every one of 503 exchanges.

**The run.**

| | |
|---|---|
| Authorization | `live-run-authorization.md` — the repository owner, **2026-08-23**, as accountable product, technical, engineering and assurance owner |
| Seed | **0** |
| Horizon | **50 ticks**, and `tick_reached` is 50 with `ended: tick_limit` |
| Exchanges | **503** |
| Ceiling | **$2**, declared as `ceiling_cents: 200`, never approached |
| Cost, as the engine reported it | **16 cents** |
| Cost, recomputed from the transcript | **16.67 cents** |
| Cost, **billed** | **20.55 cents** |
| Fallbacks | **0** |
| Cache ratio | **0 basis points — FAIL** |
| Run on | 2026-08-29, by the owner, from a connector outside this repository |

The run's own record, retained at `live-run-record-stream.txt`:

    {"run_record":"llm","seed":0,"ticks":50,"density":"0.75","trace_actions":false,
     "model":"gpt-5.6-luna","reasoning":"none","exchanges":503,
     "tokens":{"prompt":776963,"cached_prompt":0,"output":9429,"reasoning":0},
     "cache_ratio_basis_points":0,"cost_cents":16,"ceiling_cents":200,"fallbacks":0,
     "unfit_to_publish":false,"tick_reached":50,"ended":"tick_limit"}

**The run fell inside its authorization on every dimension** — seed 0 of the authorized {0}, 50 ticks of
an authorized 50, 200 cents of an authorized 200 — and the authorization record states that it does not
extend to `WO-MOK-027`'s five-seed measurement.

**Now the explanation, and it is a finding about the requirement meeting the provider rather than a defect
in anything this repository owns.** Five candidate causes were measured and ruled out: not the missing
reasoning parameter (attempt 1 sent none, attempt 2 sent `none`, both 0); not the schema (one distinct
`response_format` across 24 real requests); not the prompt layout (a 5,402-character static prefix, about
1,316 tokens, above the provider's 1,024-token minimum, genuinely at the *start*); not the prefix being
sent too rarely (about 42 sends of each actor's prefix inside seven minutes); and not caching being
unavailable.

**Caching demonstrably works. This model caches an exact prompt, not a prefix.** Three requests built to
mirror the connector exactly:

| request | `prompt_tokens` | `cached_tokens` | `cache_write_tokens` |
|---|---|---|---|
| 1. prime the prefix | 1,627 | 0 | 1,624 |
| 2. **identical** prompt again | 1,627 | **1,624** | 0 |
| 3. **same 1,316-token prefix, different suffix** | 1,637 | **0** | 1,634 |

Request 3 is the run's case and it cached nothing. Request 2 is the control and it cached almost
everything — 1,624 of 1,627, the whole prompt less a three-token tail, and not a multiple of the 128-token
block a prefix-walking scheme would report.

**So no prompt layout can satisfy `REQ-MOK-070` under this binding.** The obligation rests on a long shared
prefix earning a discount; the run never repeats a prompt exactly, because each observation reports a
different world, so every exchange is a full-price miss **by construction**. The authorization record
predicted a marginal miss — about 0.866 falling below 0.85 through tokenizer granularity — and **the
prediction was wrong in kind, not in degree**.

**The disposition is the owner's and it was taken.** `REQ-MOK-070` is recorded as outstanding, its text
does not move, and `L15b` is recorded FAILED. Three alternatives were costed and declined: amending the
requirement into a provider-conditional obligation, retiring it as resting on a falsified premise, or
trying another model or endpoint at the price of a further authorised run and a decision rule 8.5 reserves
to the owner. **The requirement stands unsatisfied rather than satisfied-as-amended**, which is the
substantive content of the choice: an amendment would have made the contract passable, and recording it
outstanding keeps the contract honest at the price of a permanent red case.

`VER-MOK-018` needs no amendment for it. Its *Residual uncertainty* already says that `L15b` failing
against a layout that was correct when written is "a signal to re-measure and bring the layout or the floor
back to the owner, not a reason to soften the number in place". That process ran to completion: the
measurement was taken, the cause was diagnosed, and the owner declined to move either.

Full account: `live-run-measurements.md`, sections *The measured token split*, *The cache ratio, and why it
is zero*, *What was ruled out* and *What it is*. Item 4 of this work order's evidence list — the ratio
re-derivable from the transcript's own figures rather than taken from the run record — is that first
section.

---

## 7. The estimate against the measurement

**This is the first point in this initiative where an estimate meets a measurement.** Four quantities,
each as estimated in `SPEC-MOK-007` and `ADR-MOK-007` and as measured here, with the difference as a
factor.

| quantity | estimated | measured | factor |
|---|---|---|---|
| **Per-run cost**, this horizon | **5 cents** (`ADR-MOK-007`, prorated; the authorization record's projection at the published tariff agrees) | **20.55 cents** billed; 16.67 recomputed; 16 as reported | **4.1× the estimate** (3.3× on the engine's own figure) |
| **Cost per 1,000 ticks** | **$1.04** (`ADR-MOK-007`) | **$4.11**, scaling the billed figure at 0.4110 cents a tick | **4.0×** |
| **The token split** | block D and the observation dominate the variable part; the shared prefix is the bulk | prefix block 1 **1,249.0 tokens, 84.6 %**; prefix block 2 15.0, 1.0 %; observation block C 172.5, 11.7 %; **block D 40.2, 2.7 %**; framing +68.0; reported prompt **1,544.7** | confirmed in shape; **block D is smaller than the trade-off assumed** |
| **The cached share** | **0.85** (`REQ-MOK-070`); about **0.866** projected in the authorization record | **0.000000** | **the whole value** |
| **Transcript size per 1,000 ticks** | **12 MB** (`SPEC-MOK-007` rule 11.7) | **12.7 MB** | **1.06× — confirmed** |
| **Latency per exchange** | **0.4 to 0.8 s** (rule 11.7's neighbouring estimate), giving 1.2 to 2.4 hours for 1,000 ticks | about **0.8 s**, from an operator observation of 503 exchanges inside seven minutes | **at the top of the band**, and see the caveat below |

**The cost projection was arithmetically right and factually wrong, and the difference is exactly the
caching.** Recomputing attempt 2's own token counts with 85 % of the prompt cached gives **4.78 cents** —
the 5-cent projection, to the cent. The projection did not miscalculate; it assumed a discount the
provider did not give. Caching, had it engaged, would have made this run **3.5 times cheaper**.

**The factor propagates and it is recorded because a later record would otherwise inherit it.**
`WO-MOK-027`'s estimated $5.20 for five seeds becomes about **$20.55** on the same basis — ten times this
work order's $2 ceiling, and above any ceiling `ADR-MOK-007` act 8 anticipated. **Scaling from the
engine's own 16.67 would understate it again**, at $3.33 per 1,000 ticks and $16.67 for five seeds; those
are the figures a reader who trusts the run record arrives at, and they are wrong for two compounding
reasons below.

**The latency figure carries a caveat rather than a factor, and the caveat is by design.** Nothing in the
retained evidence carries a duration: `SPEC-MOK-007` rule 11.4 admits no timestamp in a transcript, so an
elapsed figure is not something this repository retains, and `VER-MOK-018` case `R5` forbids a timing
figure from being a pass condition anywhere. The seven-minute reading is the operator's observation of a
run they watched, not a measurement in this packet, and it is offered as one. It happens to sit at the top
of the estimated band, which is the most that can honestly be said.

**Two reasons the reported cost understates the money, both in the same direction.**

1. **The four-price model has no slot for a cache-write charge.** The provider returns
   `cache_write_tokens`, a fifth quantity rule 14's four prices cannot express, and the published rate is
   **$0.25 per million — higher than the $0.20 input rate**. Every exchange of both runs was a miss, so
   every prompt was written: 775,454 tokens on attempt 2 and 868,618 on attempt 1, 1,644,072 together.
   **The owner read the provider's billing record for 2026-08-29 and it says `$0.53` for the day**, which
   settles which of two readings applies: $0.25 *instead of* the input rate predicts 53.10 cents for the
   two runs and lands inside the window with room for the caching probe; $0.25 *in addition* predicts
   85.98 and is 33 cents outside a figure stated to the cent. **The first is confirmed**, so every cost
   figure in this packet is a figure and not a lower bound.
2. **The engine truncates rather than rounds.** 16.67 is reported as 16. Rule 14.2 fixes the minor unit
   and does not fix the direction, so this is an observation and not a defect — recorded because a
   truncating cost figure understates spend against a ceiling.

**The engine's cost figure is 81 % of the money spent, and that is a finding about the ceiling rather than
about the report.** `REQ-MOK-071`'s ceiling is a stop and it stops on the figure the engine computes; that
figure omits the cache-write charge entirely, so **the ceiling bounds the engine's belief about spend and
not spend**. Nothing was at risk here — 3.88 cents unbilled against a $2 ceiling — but the gap is 19 % of
the true cost in the direction that lets a run continue past its limit, and it compounds with the
truncation, which errs the same way. **A fifth price would close it and cannot be added without an owner
decision**, because `--prices` takes four values in a fixed order and a fifth term moves that option's
arity and rule 14.3a's format. It is recorded as an open finding, and it is now *decidable* where before
the bill it was not.

**One trap in the data, recorded so no later reader falls into it.** The engine's two reported figures sum
to 53 cents and the money spent was 53.10, so a reader comparing totals would conclude the pricing rule
was sound. It is not: attempt 1 was **over**-billed by 9.16 cents by double-counted reasoning tokens, both
runs were **under**-billed by 8.22 for cache writes, and truncation took a further 1.04 — three errors in
two directions that cancel to within a tenth of a cent at this one pair of runs. **The only reason the
defect was found is that the per-run figures were recomputed. The totals would have hidden it.**

**And the bill corroborates the rule 14.2a amendment independently, which is not the question it was
asked.** The 53.10 prediction depends on attempt 1's *corrected* cost of 28.21 cents. Recomputed with the
double-billing arithmetic this work order removed, the same reading predicts 62.26 cents — nine cents
above a bill the corrected figure matches to a tenth of a cent. The provider's own accounting agrees with
the amendment, and it is the only check on that amendment that does not come from this repository.

---

## 8. Each verification case, with its result and its evidence path

**`verification-cases.txt` in this directory is that section in full**: 45 rows, one per case in this work
order's *Required verification* list, each with its result and its evidence path, under the governing
sentence "A case that cannot be run is escalated, not omitted." It also carries the suite reading the rows
rest on, the three ignored tests named, and the test-count reconciliation.

The tally:

    39  PASS
     1  FAILED                             L15b
     2  PASS IN PART, ESCALATED            L17, L20
     1  PASS, WITH A DEVIATION RECORDED    L29
     1  ATTESTED                           C6, the owner's
     1  NOT MINE TO MAKE                   M2, the owner's, and not re-required
    ---
    45  rows, and no required case omitted

**The six that are not a bare PASS, each in a sentence.**

- **`L15b` FAILED.** Section 6.
- **`L17` PASS IN PART, ESCALATED.** The independent instrument returns FAIL on both live transcripts and
  its FAIL is retained verbatim. The cause was measured: **0 genuine JSON floats**; the 503
  fraction-shaped hits are **one distinct string, `5.6`, in one field, in one context** — the model
  identifier `gpt-5.6-luna`, once per exchange, stored as the opaque string rule 15.2 calls for. All three
  repairs belong to someone else: narrowing the instrument's predicate is a change to a check's strictness
  under `SPEC-MOK-004` rule 12; giving case `L17` a model-identifier exception amends an approved
  verification artifact, which stop condition 6 forbids on an implementation agent's judgement; recording
  it as holding in substance with the reading attached is what was done. **This is not stop condition 11**:
  that condition is the two transcripts disagreeing in *form*, and they agree in form on all seven other
  checks, on the record split, on the 5,385-byte shared block and on the actor blocks. One field's *value*
  differs.
- **`L20` PASS IN PART, ESCALATED.** The gate pairing is green in all four combinations (section 5). Two
  coverage gaps: three of the four `--live needs …` refusals in the parser have no test — only `--prices`
  does, at `tests/cli.rs:1066` — so deleting the ceiling check at `src/cli.rs:552` would leave the suite
  green; and the connector's one credential arm, which rule 13.3 deliberately gives absent, empty and
  malformed together, is exercised only on the absent entry. Closing either means moving a candidate that
  six captures and the paid run's replay assertions are bound to, so it is escalated in section 12 rather
  than done here.
- **`L29` PASS, WITH A DEVIATION RECORDED.** Both doors carry the port in one suite, at
  `tests/replay.rs:1171`. The canned connector was **not** substituted into that test, which this work
  order asks for and which `VER-MOK-018`'s own matrix row for `L29` also names — so the miss is against the
  contract as well as the work order and the row says both. What the canned connector does drive, through
  door one and through a real spawned child process, is `connector.rs`'s 13 tests, which is stronger than
  an in-process stub. Section 12 carries the deviation for the owner.
- **`C6` ATTESTED.** `credential-attestation.md`. Its five corroborating `gh api` readings are the owner's
  and are outstanding; the attestation states that it does not depend on them.
- **`M2` NOT MINE TO MAKE, and not re-required.** The shared rules block is measured unmoved — one
  declaration in the source, 5,385 bytes over the live run's 12 prefix records, the same figure
  `WO-MOK-025` measured — so `WO-MOK-025`'s `manual-assessment.md`, the owner's act of 2026-08-24 over
  commit `4cfb297`, still binds the same object.

**The suite the rows rest on**, on both platforms, at this candidate:

    cargo test --workspace --locked --no-fail-fast
      Windows 11, x86_64-pc-windows-msvc        -> 481 passed, 0 failed, 3 ignored, exit 0
      Ubuntu (WSL 2), x86_64-unknown-linux-gnu  -> 481 passed, 0 failed, 3 ignored, exit 0

Same total, same three ignored instruments, 25 test binaries and 2 doc-test targets each, no target
disagreeing. `both-platforms.txt` is the reading in full. The base commit's total was 425; 59 tests
arrive, 0 depart, 0 are renamed; the package total is 484, which is `SPEC-MOK-004` rule 11's figure and
reconciles as 502 strict `#[test]` lines less the 18 in retained evidence `.rs` oracles.

---

## 9. Every credential-handling decision

The whole path a secret takes through this stage, so the owner can see it end to end. **Ten decisions**,
each with what made it.

1. **The credential lives outside the repository and is never committed.** `REPOSITORY_CONTEXT.md`'s
   restricted-paths bullet, `ADR-MOK-001`, `REQ-MOK-073`. Four prohibitions, none relaxed here.
2. **The connector reads it, and nothing else in this system reads it at all.** `SPEC-MOK-007` rules 10.5
   and 13.4 place it there deliberately, so that `INT-MOK-011` principle 1's *neither target reads a
   credential* is literally true rather than nearly true. `architecture-checks.txt` check 8 measures **0
   environment reads in either package** beyond `args`, `var` and one `env::temp_dir` in a test module.
3. **It reaches the connector by inheritance and by nothing else.** The spawn builder makes no `.env`,
   `.env_clear`, `.env_remove` or `.envs` call (section 3). Nothing forwards it, names it or copies it.
4. **The engine never places it in an argument, a log or a diagnostic.** `--connector-path` and
   `--transcript-output` are paths, not secrets, and neither survives the parser (`S6a`). No engine
   diagnostic reads the environment.
5. **The connector may not write it anywhere.** Not to standard output, not to standard error, not to any
   file, and not into an `error.message`. `docs/CONNECTOR_PROTOCOL.md` states it and
   `mokiterions-core/tests/connector.rs:443 the_credential_value_reaches_no_produced_byte` asserts it over
   every produced byte, with a synthetic credential set in the child's environment. **At `WO-MOK-025` this
   test could not exist, because no code path read a credential.**
6. **The refusal cannot name the value even by accident.** The canned connector's one credential arm binds
   the value only on the succeeding side and discards it, and `Ok(_)` binds nothing on the failing side —
   so the refusal names the *variable*, which is what an operator has to fix, and the value is never in
   scope to be formatted. `canned_connector.rs:210` records the mechanism in the source.
7. **Empty, absent and unreadable get one treatment**, per rule 13.3, in one match arm whose message names
   all three. Only the absent entry is exercised, which section 8's `L20` row escalates.
8. **No workflow holds one and none selects live mode.** `workflow-credentials.txt` — the repository's own
   gate, exit 0, over 4 workflow files and 637 non-comment lines, with a 38-test self-test that constructs
   workflows which *do* reference a provider key and asserts the check fails and names the file and the
   line. Both run on every push at `.github/workflows/provider-credentials.yml:90` and `:100`. `L21a`,
   `C2`, `A6`: PASS.
9. **The owner held the credential and ran both live runs. The implementation agent never received it.**
   The connector was written outside the repository, the owner supplied the credential and invoked it, and
   the agent received only the produced files.
10. **The leak check ran before anything was committed, both times, and again in the suite.** The owner's
    check over attempt 1's three files reported clean; the check over attempt 2's reported clean; and the
    committed guard `the_live_evidence_carries_no_credential_and_names_the_provider_once` at
    `tests/replay.rs:1670` now scans all six retained files for ten credential-shaped strings on every
    `cargo test`. **No `error` record exists in either transcript**, which is the only path by which a
    provider message could have reached a retained file.

**What no check here can reach, and where it is recorded.** Whether a provider credential is configured in
the repository's *automation secrets* is not visible from inside the repository: a secret's **name** is
enumerable through the hosting platform's API at every scope, but its **value** is not, and no measurement
covers a moment other than the one it was taken at. That is `C6`, an owner attestation, and
`credential-attestation.md` states all three limits itself.
`workflow-credentials.txt`'s own `NOT CHECKABLE HERE` paragraph is where the boundary is drawn from the
repository's side.

---

## 10. The amendments made

**Twenty-one acts across seven artifacts.** Each is a row in its artifact's own amendment record with its
authorising act; the table below is the index, not the text.

| artifact | provision | what moved | authorising act |
|---|---|---|---|
| `SPEC-MOK-007` | rules **1.1** and **1.4** amended, **1.1a** and **1.4a** added | the port's return grows to carry what rule 11.3 obliges the engine to record; one item of rule 19.2's list recorded unreachable | repository owner, 2026-08-29 |
| `SPEC-MOK-007` | rule **1.1b** added | the port answers whether the run has stopped spending, and the engine asks before each exchange | repository owner, 2026-08-29 |
| `SPEC-MOK-007` | rule **1.1c** added | the proposal carries the exchanges the opportunity spent before it, which is how rule 19.5's retried attempts reach the engine | repository owner, 2026-08-29 |
| `SPEC-MOK-007` | rule **1.1d** added | the port reports the run's accounting once, after the run has ended | repository owner, 2026-08-29 |
| `SPEC-MOK-007` | rule **11.7** made plural, **11.7.2** added | this repository now commits two transcripts and the rule said "the"; 11.7.2 carries the live transcript's measured figures | repository owner, 2026-08-29, choosing to amend here over recording a defect or deferring |
| `SPEC-MOK-007` | rule **14.2a** added | the reported output count *contains* the reported reasoning count, so the billable output is the difference — the double-billing found on attempt 1 | repository owner, 2026-08-29, choosing to amend over recording a defect |
| `SPEC-MOK-007` | rule **11.7**'s count two → three, **11.7.3** added | the owner's retention disposition of the same day commits a third transcript | repository owner, 2026-08-29 |
| `SPEC-MOK-002` | rule 5's census + `simulation::UnitPrices` | the run's four unit prices reach the engine on the command line under rule 14.3a | repository owner, 2026-08-29 |
| `SPEC-MOK-002` | rule 5's census + `Proposal`, `ReportedUsage`, and `Proposer`'s grown return | the port's return grows from `Option<Action>` | repository owner, 2026-08-29 |
| `SPEC-MOK-002` | rule 5's census + `simulation::ConnectorPort`; frontmatter `updated` corrected | rule 10's connector binding needs a port | repository owner, 2026-08-29 |
| `SPEC-MOK-002` | rule 5's census, `ConnectorPort::new`'s grown signature | prices and ceiling become parameters; growth nil | repository owner, 2026-08-29 |
| `SPEC-MOK-002` | rule **4** gains a **fourth exit code** — the first it has ever added — and rule 5's census gains the constant and the stopping method | rule 14.6's ceiling stop needs a status distinct from a clean completion and from an error | repository owner, 2026-08-29 |
| `SPEC-MOK-002` | rule 5's census, a fourth public field on `Proposal` | the exchanges an opportunity spent before the one it ended on | repository owner, 2026-08-29 |
| `SPEC-MOK-002` | rule 5's census + `simulation::LiveAccounting`, and `Proposer`'s fourth method | the value a live port reports its account as | repository owner, 2026-08-29 |
| `SPEC-MOK-003` | *Start-up inputs* | the five live-run options the shared parser now accepts, each recognized and refused by the observer. One paragraph, six bullets; **no new rule, input, default, validation, rendered output or exit code** | repository owner, 2026-08-29, discharging the disposition the 2026-08-24 row left owed |
| `SPEC-MOK-004` | rules **9** and **11** re-measured at `7f9e20a`; rule 11's test-binary count corrected; rule 6 re-measured and unmoved | 53 tests arrive, none depart; **rule 1 does not move** | repository owner, 2026-08-29 |
| `SPEC-MOK-004` | rule **11** re-measured at `2574ff9` | six tests arriving, none departing or renamed; observer 202 unmoved, engine 282, workspace 484 of which 481 execute; the earlier row stays true of `7f9e20a` and is not edited | repository owner, 2026-08-29 |
| `REPOSITORY_CONTEXT.md` | restricted-paths bullet | a live run is an owner-authorised manual act and happens no other way; **"one such run has now happened"**, with its figures and its evidence path | this work order's *In scope* item 16 |
| `WO-MOK-026` | `[execution_scope]` added | the 0.8.0 root requires one to start work; the work order was authorized and unstartable without it | engineering owner, 2026-08-28, under `WO-HUP-002` |
| `WO-MOK-026` | `SPEC-MOK-002.md` admitted to `[execution_scope]` and `[relations].specifications` | rule 5's census cannot be re-measured from outside the scope that moves it | repository owner, 2026-08-29 |
| `WO-MOK-026` | `SPEC-MOK-007.md` and `mokiterions-tui/src/state.rs` admitted to `[execution_scope]` | two separate owner acts, each with its own row | repository owner, 2026-08-29 |

**Two things this table does not contain, deliberately.**

`VER-MOK-018` is not amended by this work order. `L15b`'s failure needs none, because that contract already
says what a `L15b` failure means. `L17`'s escalation asks for one and **it is not made here**, because stop
condition 6 forbids amending an approved artifact on an implementation agent's judgement.

**`SPEC-MOK-004` rule 1 does not move**, and its `updated` frontmatter is the one non-measurement in its
new rows.

---

## 11. What was not verified, and why

**`L24`, `L25` and `M3` are `WO-MOK-027`'s.** `L24` — the comparison is published — and `L25` — only fit
runs are published — are owner-gated and depend on a five-seed measurement this work order's authorization
explicitly does not cover: its **estimated** $5.20 exceeds this stage's $2 ceiling, and the measurement in
section 7 puts the real figure at about **$20.55**, so that stage needs its own authorization with a
revised ceiling. `M3` is a manual assessment and is not an implementation agent's to make.

**The dependency surface of any connector this repository does not own.** `S2` cannot reach it, and
`VER-MOK-018` records that as a **limit rather than a gap** — the distinction matters and it is
`SPEC-MOK-007` rule 10.6's: the program is named by the operator, written outside this repository, never
committed, and no check here can see what it declares or what it sends. Verifying a claim about a program
outside the repository is not possible, and pretending otherwise would be the worst kind of green.
`ADR-MOK-007`'s *Negative* consequences record the same limit. What stands in its place is
`docs/CONNECTOR_PROTOCOL.md` and the sufficiency check on it.

**`L15b` is verified and it failed.** It is listed here only to say that it is not in this section: a
failing measurement is a verified case with a negative result, not an unverified one.

**Three clauses of otherwise-passing cases are not covered by any test**, and each is named in
`verification-cases.txt` rather than left to be discovered:

- three of the four `--live needs …` parser refusals (`L20`);
- the empty and unreadable entries of the connector's credential arm (`L20`);
- the canned connector standing inside `L29`'s own test, which the contract and the work order both name.

**Two clauses are implemented and unexercised because this run did not reach them**, which is a property
of the run rather than of the coverage: `an_unreported_count_is_absent_and_not_zero` (the provider reported
all four counts on all 503 exchanges) and the retried-exchange half of `L16` (the transcript records no
retry at all, which `tests/replay.rs:1626` measures). Both are held by the canned connector against a real
child process instead, and `verification-cases.txt` says so in the rows rather than letting 503 clean
exchanges imply coverage of a path none of them took.

**Answerability.** Item 14 measured the token side of the flat-versus-nested trade-off exactly. The other
half — the specification's own "may be harder to answer well" — is not measurable from a transcript: it
needs the same exchanges sent both ways to a live provider, which is spend, which needs an authorization
that does not exist. It is recorded as the measurement's boundary rather than treated as a residual.

---

## 12. Every local decision under the envelope, and every escalation with its resolution

### Decisions taken under the envelope

The envelope delegates "the retry count, the backoff shape and which transport failures are retried,
subject to `R1` and `R2`". Everything else that could look like a decision was either the owner's or a
measurement.

1. **The retry bound is three**, after one initial attempt — four attempts in all. The owner fixed it on
   2026-08-29 and `SPEC-MOK-002`'s amendment record carries the decision;
   `simulation.rs:16685 a_transient_failure_is_attempted_four_times_and_counts_one_fallback` is the shape
   as implemented.
2. **A broken pipe is not retried.** `simulation.rs:16875 a_pipe_that_failed_is_not_retried`. The bound's
   other edge, and a decision under the envelope rather than a rule.
3. **The ceiling wins over the retry bound.** `simulation.rs:16837 the_ceiling_stops_the_retrying_before_the_bound`,
   so a run cannot spend past its ceiling by retrying.
4. **The canned connector is a `[[bin]]` target rather than a plain source file.** Recorded as
   `WO-HUP-002`'s owner decision of 2026-08-28, not taken by the agent.
5. **`tests/support/` rather than `tests/`**, because Cargo would otherwise build the file twice.
6. **The three nested block-D renderings in the measurement are the document's own constructions**, since
   `SPEC-MOK-007` names the nested option without fixing its punctuation. Disclosed in
   `live-run-measurements.md`: the 19.6 % figure is the best of three plausible forms, a different
   separator moves it between 10.7 % and 19.6 %, and none of that range changes the conclusion.

### The owner's dispositions, taken 2026-08-29

Seven questions this stage's measurements raised, each costed before it was put.
`live-run-measurements.md`'s *The owner's dispositions* holds them in full with the measurement each
turned on.

1. **`REQ-MOK-070`: recorded as outstanding, nothing amended.** Section 6.
2. **Block D: the flat form stands.** Item 14, section 1.
3. **Rule 11.7's singular phrasing: amended here**, over recording a defect or deferring to a governance
   work order behind a stacked pull request.
4. **`L15b` and this stage's verification record: verify, and record `L15b` as FAILED**, with the
   measurement and its provider-binding cause disclosed and a carried-forward item to revisit either the
   floor or the model binding. Withholding verification was costed and declined — nothing in this
   repository can make `L15b` pass — and deferring to `WO-MOK-027` was declined because that stage would
   meet the identical `0.000000` and spend another paid run to re-observe it.
5. **Rule 14's double-billing: amended here**, over recording a defect. What decided it is that this
   stage's accepted run reports **no** reasoning tokens, so the corrected arithmetic yields identical
   figures and the amendment invalidates no retained cost, no run record and no replay.
6. **The cache-write charge: settled from the provider's billing record.** The owner read it: `$0.53` for
   the day. Section 7.
7. **The two runs' evidence layout: the asymmetry stands and is disclosed.** Attempt 1 under
   `attempt-1/`, attempt 2 at the top level. **The literal reading of `VER-MOK-018`'s "one directory per
   run" is not satisfied for attempt 2, and this packet says so rather than claiming a compliance it does
   not have.** Moving attempt 2 was declined because the evidence path is provenance and a rename after
   capture means paying for another run; amending the retention wording was declined because it would edit
   an approved contract to fit the evidence.

### Escalations raised, with their resolutions

| # | escalation | resolution |
|---|---|---|
| 1 | **`L15b` fails** — stop-and-escalate condition 3, which anticipated this outcome exactly | **Resolved by the owner**: recorded outstanding, nothing amended, `L15b` FAILED. |
| 2 | **`L17`'s instrument returns FAIL on a transcript that carries no float** | **Open, and the choice is the owner's**: narrow the instrument's predicate, or give case `L17` a model-identifier exception. Recorded in substance with the reading attached; neither repair is an implementation agent's. |
| 3 | **`L20`: three untested parser refusals and an untested credential entry** | **Open**. Closing them moves a candidate six captures and the paid run's replay assertions are bound to. Whether under this work order or a follow-up is the owner's call. |
| 4 | **`L29`: the canned connector was not substituted into the test the contract names** | **Open, disclosed as a deviation**. The substance holds — a real child process drives door one in `connector.rs`'s 13 tests — but the literal instruction does not, and it is the contract's as well as the work order's. |
| 5 | **The third `[[bin]]` target contradicts `SPEC-MOK-002` rule 1** | **Recorded as a defect**, `defect-third-target-unamended.md`, with three unchosen repair routes. Amending rule 1 on an agent's judgement is stop condition 6. |
| 6 | **The record stream carries a live object where the artifacts describe a rendering** | **Recorded as a defect**, `defect-record-stream-live-object.md`. |
| 7 | **`SPEC-MOK-006` has two rules numbered 8.9** | **Recorded as a defect**, on the owner's disposition, rather than renumbered. |
| 8 | **`WO-MOK-025`'s `architecture-checks.py` reports 4 binary targets and prints the fourth as `? at ?`** | **Recorded, not repaired.** It counts the literal `[[bin]]` inside a *comment* at `mokiterions-core/Cargo.toml:49`. Visible rather than silent, and it fails toward over-reporting. The file is in `WO-MOK-025`'s verified packet, so editing it is not this work order's to do. `architecture-checks.txt`'s preamble carries it, and section 8's `S2a` corroborates the count of three from the source side: exactly three `fn main` in the whole tree. |
| 9 | **The same instrument quotes `WO-MOK-025`'s dependency capture rather than taking one**, by a path baked in at its line 60 | **Resolved by re-running the gate.** `declared-dependencies.txt` is that run, exit 0, and its body is byte-identical to `WO-MOK-025`'s line for line — so nothing in the resolved graph moved, measured rather than inferred from the manifests not having moved. |
| 10 | **Four prose paragraphs in that instrument's output are stale at this candidate** | **Retained unedited and named**, in `architecture-checks.txt`'s preamble. Editing an instrument's output is the one thing that would make it worthless. |
| 11 | **`advance_tick`'s signature is exactly 100 characters against a `max_width` of exactly 100** | **Recorded as a finding.** It confirms `SPEC-MOK-002`'s reason for the identifier `Proposer` — `DecisionPort` is 104 and wraps — and it sharpens the specification's own warning: the one-line obligation is prose with nothing in the tree measuring the width, so a `cargo fmt` that wrapped this line would weaken the two-door drift check while leaving a green build. Not a defect at this candidate. `interface-checks.md` carries all three parts. |
| 12 | **Rule 11.7's definite article, and rule 14's double-billing** | **Both amended here**, on the owner's dispositions 3 and 5 above. |
| 13 | **A fifth price for `cache_write_tokens`** | **Open finding, now decidable.** It moves `--prices`' arity and rule 14.3a's format, so it needs an owner decision; before the bill it was not decidable at all, and now the shape of the charge is known. |
| 14 | **The ceiling bounds the engine's belief about spend, not spend** — 16.67 against 20.55, 81 % | **Open finding**, recorded in section 7 and in `verification-cases.txt`'s `L18` and `L19` rows. |
| 15 | **`C6`'s five corroborating `gh api` readings** | **Outstanding.** They are the owner's to run, and `credential-attestation.md` states that the attestation does not depend on them. |
| 16 | **Carried-forward artifact drifts found while measuring** | **Reported, not repaired**, each for stop condition 6's reason: `ARCH-MOK-002` component 3 calls the command-line host "Unchanged"; `SPEC-MOK-002`'s 2026-08-24 row cites `main.rs:85` and `mokiterions-tui/src/main.rs:118` where the constructions now stand at 99 and 130; `SPEC-MOK-002` rule 5's census does not enumerate `DecisionRequest::tick`, `DecisionRequest::actor_id`, `ReplayPort` or `ReplayPort::new`; a retried transcript is not a replay input, which rule 11.2 and rule 12.3 read differently; `unfit_to_publish:false` on a run whose cache ratio is 0; and `VER-MOK-018`'s enumerated matrix and its prose divide `L20` and `L32` between the two hosts slightly differently. |
| 17 | **`WO-MOK-025`'s `L30` was recorded as a deviation, eighteen where the case then said two** | **Resolved by the contract's own amendment.** The 2026-08-24 row withdrew the two-exchange figure and requires a ceiling derived from the run's arity and stated with the run. At this candidate the arity is twelve, the ceiling is eighteen, the first tick is under and the second refuses at `spend ceiling reached at tick 2` — so the figure is compliance and nothing is carried forward. |

### Two more disclosures, because a report that omits them is not faithful

**The record stream is not byte-reproducible across platforms, and the authorization record claims it is.**
It carries **one CR** in two lines: the connector's diagnostic goes through Python's text-mode standard
error, which is CRLF on Windows, while the engine's run-record line is LF. Under `.gitattributes`' `-text`
the bytes are hashed as written, so the same run on Linux would produce a different digest for this file.
Both runs' record streams carry it. The transcripts and both standard-output captures are unaffected — 515
and 579 lines, 0 CRs. **The file is retained exactly as captured rather than normalized**, because
rewriting the bytes would make the evidence something edited rather than something produced, and the
connector's one-line fix is left unapplied so that the connector on disk remains exactly the one that
produced this attempt.

**The authorization record's cost section is superseded in part.** It records the tariff discrepancy as
settled in favour of `ADR-MOK-007`; section 7's measurement unsettles it. **No figure in that record is
edited**, its own rule being that a record written before a run is what the run is measured against.

---

## What a reader should take from this packet

Three things this stage established that no offline stage could, and one it could not establish at all.

**The path spends only when both gates open, and that is measured in all four combinations against the
release binaries** — not inferred from reading the parser. **The credential reaches the connector and
appears in no produced byte**, asserted by a test with a synthetic value over every stream and every
retained file, on every `cargo test`. **The ceiling precedes the exchange**, and the stop lands inside the
first tick when the ceiling is two exchanges, which is the strongest form of "before" available.

And the one it could not: **`REQ-MOK-070`'s eighty-five percent obligation cannot be satisfied under this
provider binding, by any prompt layout.** The requirement rests on a long shared prefix earning a discount
and this model caches an exact prompt. That is not a defect in the engine, the prompt or the connector, and
this report does not soften it into one.
