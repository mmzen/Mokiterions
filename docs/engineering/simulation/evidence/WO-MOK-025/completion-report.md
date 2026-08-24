# `WO-MOK-025` completion report — the model-backed decision source

`WO-MOK-025`'s *Completion report format* fixes eleven items and their order. This file follows that
order exactly, so a numbered heading below is that clause and not a topic chosen here.

| | |
|---|---|
| Work order | `WO-MOK-025`, Phase 5, status `in_progress` |
| Base commit | `cc5418553cb433715b7d6b15dea3886bff30ffaa` — the pre-change captures, taken before any change of this work order |
| Last commit that changed a package file | `77f2974` — the eleven owner rulings of 2026-08-24, whose only package edit is two comments in `simulation.rs` for escalation `E11`. It supersedes `bce4229`, which held this row until then. All four Rust gates were re-read there and the whole `REQ-MOK-068` matrix was recaptured there in both modes: **all eighty cells reproduce byte for byte.** `candidate/gates.txt` carries the measurement as its second disclosed amendment |
| Branch | `feature/phase-5-definition` |
| Date | 2026-08-24 |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `cargo 1.97.1 (c980f4866 2026-06-30)`, pinned in `rust-toolchain.toml` |
| Evidence packet | `docs/engineering/simulation/evidence/WO-MOK-025/`, whose `README.md` is the reader's entry point |

**This work order is not complete and does not claim to be.** All fourteen *In scope* items are built.
Of the **sixty-eight** required verification rows — sixty-seven when this report was first written, plus
**L34**, which the owner's ruling on `E19` added to `VER-MOK-018` on 2026-08-24 — sixty pass, three are the
assurance owner's assessments and are **recorded** rather than outstanding, three pass in part and are
escalated for the part that does not run, one passes and is escalated anyway, and one does not apply at this
stage. **Twenty escalations were raised. Seven were resolved as they arose; eleven were put to the owner in
one pass on 2026-08-24 and all eleven were ruled in the turn the question was asked; `E19` and `E20` were
raised after those rulings — one out of the defect CI found, one out of a measurement taken while assembling
the material for `C6` — and both were ruled later the same day** (item 11). What the rulings
authorized is written; what they left to an owner is in the section after them, and one of those is
`E15`, deferred to `WO-MOK-026` and recorded as untriggered rather than met. `VER-MOK-018`'s **C6** — the attestation
that no provider credential is configured in the repository's automation secrets — **is attested**: the owner
made that statement on 2026-08-24 and `credential-attestation.md` retains it beside the measurement that
corroborates it. It remains the single fact the cost containment rests on, and a retained zero is not a
substitute for it. The transition of this work order to `implemented`
is the engineering owner's act; the verification decision and its record are the assurance owner's; no
verification record for `WO-MOK-025` exists yet.

**Nothing in this file verifies anything.** It is the account of a candidate written by the agent that
wrote the code. Where it says a case passes, that means the named test or the named capture holds, not
that the case has been accepted.

---

## 1. What was built, against the *In scope* list

Fourteen items, each marked, each with the commit that built it. No item was dropped and none was
deferred.

| # | Item | State | Commit |
|---|---|---|---|
| 1 | The decision port, reaching both of rule 20.5's doors | **DONE** | `5b7c1b7`, wired to the hosts in `8f31792`, rule 5's checks restated in `ac827b1` |
| 2 | Request composition in the cache order of rules 3–7 | **DONE** | `5b7c1b7` |
| 3 | The complete enumeration, composed beside the core-proposal list | **DONE** | `5b7c1b7` |
| 4 | The transcript: records, framing, constraints | **DONE** | `bfdbf71` |
| 5 | Replay, with mismatch and exhaustion detection | **DONE** | `bfdbf71` |
| 6 | The `wait` fallback, counted, recorded, marking the run | **DONE** | `ccb5296` |
| 7 | The run record's structure and its accounting figures | **DONE** | `ccb5296` |
| 8 | The command-line surface, per rule 18 | **DONE** | library half in `5b7c1b7`, host halves in `8f31792` |
| 9 | The observer as a replay host, with its refusals | **DONE** | `8f31792`, `5af4d87`; case **L31** in `bce4229` |
| 10 | The observer's authority mapping and the four-source correction | **DONE** | `5af4d87` |
| 11 | The workflow check for `REQ-MOK-073`, and the replay step | **DONE** | static check `0e23ebe`, workflow step `aa95b7c` |
| 12 | A committed transcript covering the declared coverage | **DONE** | `aa95b7c` |
| 13 | Base-commit captures of all four existing sources | **DONE** | `2ba15cc`, taken before any change |
| 14 | The seven amendments `ADR-MOK-007` requires, each with its record row | **DONE**, one disclosure | `cc3479a`, `4878cda`, `0aa0527`, `f10a997`, `8059f51`, `3094baa`, `ac827b1`, `3c7a551`; the ratification it depended on is `b0d8a4b`; a repair is `5ae4f46` and is escalation **E7** |

Three of those commits carry more than one item because the items cannot be separated. `5b7c1b7`'s own
message states the reason: `Policy::Llm` cannot exist without behaviour behind it, rule 20.8's refusal
cannot exist without `Policy::Llm`, and `llm` in `config.policy`'s domain cannot exist without the
`schema` increment that `SPEC-MOK-006` rule 10.2 obliges. Splitting them would put a commit in history
that fails its own specification.

### The change, measured

`git diff --numstat --histogram cc54185 HEAD` over both packages, `scripts/`, `.github/` and the two
manifests:

| File | Added | Removed |
|---|---|---|
| `mokiterions-core/src/simulation.rs` | 4,394 | 29 |
| `mokiterions-core/src/cli.rs` | 128 | 24 |
| `mokiterions-core/src/main.rs` | 67 | 11 |
| `mokiterions-core/src/lib.rs` | 49 | 5 |
| `mokiterions-core/tests/replay.rs` | 1,287 | 0 |
| `mokiterions-core/tests/cli.rs` | 295 | 3 |
| `mokiterions-core/tests/no_outcome_threshold.rs` | 263 | 0 |
| `mokiterions-core/tests/process.rs` | 84 | 5 |
| `mokiterions-core/tests/transcript-seed0-ticks20-hunting.jsonl` | 233 | 0 |
| `mokiterions-tui/src/state.rs` | 81 | 4 |
| `mokiterions-tui/src/options.rs` | 68 | 9 |
| `mokiterions-tui/src/main.rs` | 56 | 15 |
| `mokiterions-tui/src/authority.rs` | 51 | 5 |
| `mokiterions-tui/src/render.rs` | 51 | 1 |
| `mokiterions-tui/tests/replay.rs` | 570 | 0 |
| `mokiterions-tui/tests/options.rs` | 203 | 6 |
| `mokiterions-tui/tests/authority.rs` | 57 | 0 |
| `scripts/check_transcript_reading.py` | 742 | 0 |
| `scripts/check_workflow_credentials.py` | 565 | 0 |
| `scripts/test_check_transcript_reading.py` | 546 | 0 |
| `scripts/test_check_workflow_credentials.py` | 506 | 0 |
| `.github/workflows/provider-credentials.yml` | 206 | 0 |
| **Total** | **10,502** | **117** |

  **The diff algorithm changes this table, and the figure above is the honest one.** Git's default
  Myers algorithm reports 17,089 insertions and 6,704 deletions for the same two commits, because it
  mis-pairs `simulation.rs`'s large test region and reports 6,616 lines of it as removed and rewritten.
  `--histogram` and `--patience` both report 4,394 added and 29 removed for that file, which is what
  the change is: the file grew from 8,878 lines to 13,243 by insertion. The Myers figure is recorded
  here so that a reader who runs the plain command and sees a different number knows which is which.

  **Every figure in this table was re-measured at `77f2974`**, the candidate the owner's eleven rulings
  of 2026-08-24 produced, and three of them moved: `simulation.rs` from 4,390 to 4,394 and
  `provider-credentials.yml` from 152 to 206, carrying the total from 10,444 to 10,502. The removed
  column did not move at all. Nothing else in the table changed, and the two rulings that reach code
  are `E11`'s comment corrections and `E17`'s third workflow job.

### The gates, at the candidate

`candidate/gates.txt` is the reading; `base/gates.txt` is its pair, taken before the change. **Every one
of the six was re-read at `77f2974`** after the owner's rulings of 2026-08-24, and again at `dbc9e6d`
after the cross-platform fix below, this time **on Linux as well as Windows**; every figure below is the
`dbc9e6d` reading and none of the six has moved across any of the three.

**Every reading in this report before `dbc9e6d` was taken on Windows and none of them said so, and two
were true of Windows only.** That is the subject of *The defect CI found* below, and the figures here are
now stated per platform because of it.

    cargo fmt --all -- --check                                                     exit 0, no output
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings   exit 0, no warnings
    cargo test --workspace --locked --no-fail-fast       exit 0; 422 passed, 0 failed, 3 ignored
      the same three figures on Windows 11 (x86_64-pc-windows-msvc) and on Ubuntu 24.04 under
      WSL 2 (x86_64-unknown-linux-gnu), each with the same 25 result lines
    cargo build -p Mokiterions --locked                                            exit 0
    python3 scripts/test_check_workflow_credentials.py                             38 tests, OK
    python3 scripts/check_workflow_credentials.py --root .                         exit 0

Two further commands were run and are **not** gates, and are named here rather than folded into the six,
because `WO-MOK-025`'s *Completion report format* fixes what a gate is and this report does not widen it.
Both come from escalation `E17`, which gave `scripts/check_transcript_reading.py` its first automated
caller: `python3 scripts/test_check_transcript_reading.py` reports 36 tests OK, and the program itself
reports PASS over the committed transcript for `L4`, `L5`, `L6`, `L12`, `L13`, `L14`, `L15a` and `L17`.
They are now the third job of `.github/workflows/provider-credentials.yml`, so from this candidate on
they run on every pull request and every push to `master` rather than by hand.

The suite declared 344 passing tests across 22 binaries at the base commit and declares 425 across 23
here, of which 422 execute. The three that do not are instruments, and they are named rather than
counted, because a case whose evidence is an ignored test is a false green that looks exactly like a
real one: `print_base_commit_entropy_literals`, `the_four_existing_sources_entropy_state_at_every_tick_boundary`
and `regenerate_the_committed_transcript`. All three are in the engine package; no observer test is
ignored, which is the measurement `ARCH-MOK-002`'s amended ignore-attribute check asks for.

The harness pair, read from the pinned 0.4.0 evaluator outside the checkout:

    se_harness validate                                       PASS, 179 artifacts, 0 errors, 0 warnings
    se_harness preflight --work-order WO-MOK-025 --phase review   PASS

`validate` reported **FAIL** from `0aa0527` until `5ae4f46`, and finding that is escalation **E7**.

### The defect CI found, and what it says about every gate above it

**`cargo test` was green on this machine and red in CI, and CI was right.** The `Dependency declarations`
lane failed at rule 8.4c's `cargo test -p Mokiterions --locked --offline` on
`mokiterions-core/tests/replay.rs:127`, `assertion failed: output.stdout.is_empty()`. The cause is one
platform difference: **`fs::File::open` on a directory succeeds on Linux** and refuses only when the file
is read. Both hosts opened the transcript and handed the reader straight to `ReplayPort`, so on Linux a
directory named as a transcript *began the run* — the engine binary printed the whole of tick 0 to
standard output and only then failed with `Is a directory (os error 21)`, and the observer entered the
terminal and was still drawing twenty seconds later with nothing on standard error. On Windows the open
itself fails, so nothing was printed and every local gate was green.

That contradicts `mokiterions-core/src/main.rs`'s own comment — *"opening it creates nothing, so a failure
here leaves the filesystem exactly as it was"*, offered as the reason the ordering means exiting `1`
before any tick runs — and it contradicts `SPEC-MOK-007` rule 13.2, which requires exactly that ordering.

`dbc9e6d` fixes it in **both** hosts, because rule 12.2's guarantee holds *"in both hosts"* and a fix in
one would have left them disagreeing on Linux. Each now calls `fill_buf()` on the `BufReader` before
lending it: `fill_buf` peeks without consuming, so the port reads exactly the bytes it would have read
anyway, and an empty file is still `Ok(&[])` — a transcript that ran out at the first opportunity, which
is the engine's own rule 12.4 refusal and a different case. Both properties were measured rather than
reasoned: a directory exits `1` with **0 bytes** of standard output on both platforms, and an empty file
exits `1` after **12,859 bytes** on both.

**Three things about this are worth more than the fix.**

- **The test that should have caught it was written in a way that could not.** Its directory case asserted
  `assert_ne!(code, Some(0))` rather than the exit code, and asserted an empty standard output at a point
  where the partial run had already been written. So on Linux the assertion that fired was the one about
  output, and the one that would have named the defect was never strict enough to fire at all. It now
  asserts the missing file's case exactly: exit `1`, empty standard output, the host's message naming the
  path, and no usage text.
- **No case in the required list covered this**, which was escalation **E19** below. `L32` covers the
  parser's exit `2` and `L8` covers a mismatch found while replaying; a transcript the platform refuses had
  no case on either host, so running the whole of `VER-MOK-018` would not have found this. **The owner ruled
  on 2026-08-24 that the case be added**, and `L34` is it: both hosts, exit `1`, empty standard output, the
  host's own message prefix, no tick started, and exercise on more than one platform as a pass condition. The
  gap is closed for the contract; the sentences above stand as the account of what it cost while it lasted.
- **The observer's half had no automated test of any kind**, and its failure was the worse of the two — a
  live observer over a transcript that cannot be read, with nothing to end it but a key press. That gap
  was already recorded in this packet before the defect was found; the defect is what it costs.

**Everything in the packet that depends on a build was re-measured at `dbc9e6d`**, because the source
change is behavioural this time rather than a comment: the four Rust gates on both platforms, the whole
eighty-cell `REQ-MOK-068` matrix in both modes, the twenty entropy configurations, all eight
`replay-identity.txt` cells and all twelve of its boundary and refusal cases, and the four Python
readings. **Every figure reproduces.** Two captures that print source line numbers —
`candidate/static-checks.txt` and `candidate/architecture-checks.txt` — were re-run and **replaced**
rather than annotated, 30 differing lines each of which one is the commit named and 29 are digit-only,
with no finding, verdict or sentence changed. `candidate/gates.txt`'s third amendment carries the session
in full and `README.md` carries the departure from this packet's no-editing-a-capture rule.

## 2. The public surface, before and after

`candidate/public-surface.txt` is the measurement. Base 49 items, 43 public fields, 92 `pub` lines;
candidate 60 items, 43 public fields, 103 `pub` lines. **Eleven arrivals and no departures.**

### `execute`, in `mokiterions-core/src/lib.rs`

```
  base                                   candidate
  pub fn execute<I, S, W, E>(            pub fn execute<I, S, W, E>(
      args: I,                               args: I,
      stdout: &mut W,                        stdout: &mut W,
      stderr: &mut E,                        stderr: &mut E,
      records: Option<&mut dyn Write>,       records: Option<&mut dyn Write>,
  ) -> u8                                    port: Option<&mut dyn Proposer>,
                                         ) -> u8
```

### `Simulation::advance_tick`, in `mokiterions-core/src/simulation.rs`

```
  base       pub fn advance_tick(&mut self) -> Result<TickOutcome, String> {
  candidate  pub fn advance_tick(&mut self, port: Option<&mut dyn Proposer>) -> Result<TickOutcome, String> {
```

### `pub fn run` is not amended

```
  pub fn run<W: Write>(&mut self, output: &mut W) -> io::Result<RunSummary> {
```

Byte-identical at both revisions, confirmed by diffing the base line against the candidate line. It
delegates with the port absent, so its enumerated form in `SPEC-MOK-002` rule 5's first list stands
unchanged for the second time. The line moved from 2004 to 3731 because the file grew above it, which
is a position and not a change.

### `pub(crate) fn run_recording`, disclosed rather than left to be found

```
  base                                    candidate
  pub(crate) fn run_recording<W: Write>(  pub(crate) fn run_recording<W: Write>(
      &mut self,                              &mut self,
      output: &mut W,                         output: &mut W,
      records: Option<&mut dyn Write>,        records: Option<&mut dyn Write>,
  ) -> io::Result<RunSummary> {               port: Option<&mut dyn Proposer>,
                                          ) -> io::Result<RunSummary> {
```

It carries the port down the call chain exactly as it already carries the record sink. It is not a
third public signature change: `pub(crate) fn` is not `pub fn`, so the two-door grep does not match it.
`ADR-MOK-007` discloses it so that it is an expected diff.

### The callers the two public changes broke

`execute`, in product code — two sites, both in the engine's own binary target:

    mokiterions-core/src/main.rs:99
    mokiterions-core/src/main.rs:142

`advance_tick`, in product code — one site, and it is the observer:

    mokiterions-tui/src/state.rs:462
      simulation.advance_tick(port.as_mut().map(|port| port as &mut dyn Proposer))?

That is the whole of the product-code breakage: the engine's binary target and the observer, and no
others. The engine's own run path does not call the public `advance_tick` — `run_recording` reaches the
private step and the public method delegates to `advance_tick_with_source` — so the library breaks no
caller of its own. Test callers were updated and are counted rather than listed: 12 sites in
`simulation.rs`'s own `#[cfg(test)]` region, 1 in the observer's `state.rs` test module, 1 `advance_tick`
site and 10 `execute` sites in the engine's public-tier tests, and no observer public-tier test calls
`advance_tick`.

### `SPEC-MOK-002` rule 5's restated mechanical checks, run at the candidate

The three greps for `execute`, each of which must return exactly one line:

    grep -n 'pub fn execute' mokiterions-core/src/lib.rs
      87:pub fn execute<I, S, W, E>(

    grep -n 'records: Option<&mut dyn Write>' mokiterions-core/src/lib.rs
      91:    records: Option<&mut dyn Write>,

    grep -n 'port: Option<&mut dyn Proposer>' mokiterions-core/src/lib.rs
      92:    port: Option<&mut dyn Proposer>,

One line each. The third grep **is** the amendment `ADR-MOK-007` authorized and this stage wrote. The
2026-08-20 restatement read *"a fifth parameter … fails the second"*, and the port on `execute` is that
fifth parameter — so a build that added the port and left that sentence standing would fail its own
specification. Rule 5 says so itself, and this is why the amendment and the code are in the same
commit.

The two-door check:

    grep -n 'pub fn .*&mut self' mokiterions-core/src/simulation.rs
      3731:    pub fn run<W: Write>(&mut self, output: &mut W) -> io::Result<RunSummary> {
      3899:    pub fn advance_tick(&mut self, port: Option<&mut dyn Proposer>) -> Result<TickOutcome, String> {

Exactly `run` and `advance_tick`. Rule 5's second obligation on this check is that `advance_tick`'s
signature is one line, because a wrapped signature puts the keyword and the receiver on different lines
and the pattern would then match neither — reporting one door where there are two, and passing while
doing so. It is one line, at exactly 100 columns including its indentation, which is rustfmt's default
`max_width`, and there is no `rustfmt.toml` in this repository. The pattern must also not appear in
prose in that file, and it does not; rule 5 records that a documentation comment matched during
implementation and that the comment was reworded rather than the check loosened.

Interior mutability, which rule 5's paragraph on mutating methods rests on:

    grep -nE '\b(Cell|RefCell|Rc|Arc|Mutex|RwLock|Atomic[A-Za-z0-9]+)\b' \
        mokiterions-core/src/*.rs mokiterions-tui/src/*.rs | wc -l
      0

No engine or observer type holds a `Cell`, a `RefCell`, an `Rc`, an `Arc`, a lock or an atomic, so no
`&self` method can mutate through one.

**A finding, reported and not repaired.** `candidate/static-checks.txt` check 6 measures the added
public items against rule 5's lists: **8 of 12 are enumerated, and four are not** — `ReplayPort`,
`ReplayPort::new`, `DecisionRequest::tick` and `DecisionRequest::actor_id`. Rule 5's own closing
prohibition is what they sit outside. This is escalation **E13**. Check 4 reports a second finding, and
it is a stale check rather than a defect in the code: property (b)'s `&'static str` carve-out no longer
covers what it was written for. That is escalation **E14**.

## 3. The four sources' byte-identity

`candidate/req-068-comparison.txt` is the measurement, against `base/` and `ratification/`.

    RESULT: no difference in any comparison. 40 text cells, 40 record cells and 20 entropy
    configurations, and the only difference anywhere is one digit in a record-stream header,
    which is this work order's own schema increment and is measured rather than assumed.

Base commit `cc5418553cb433715b7d6b15dea3886bff30ffaa`. Capture commit for the candidate side
`8162b188e21c8b12a21b86a4ac85a2d0e3eea71a`. **Twenty configurations**: five declared seeds — `0`, `1`,
`42`, `123`, `777` — by four existing sources — `baseline`, `reference`, `individual`, `social` —
each at `--ticks 1000 --density 0.75`, taken with tracing off and on for forty text cells, and the
whole matrix taken twice, once with a record sink and once without, for eighty runs.

**The output comparison.** 40 of 40 text cells identical with no sink, and 40 of 40 identical with one.
Each manifest line carries five figures per cell, so each is five comparisons: the sha256 of standard
output, its byte count, its line count, the sha256 of standard error, and the exit code. The exit code
is compared on its own, because a change that made a run fail earlier would leave a shorter output that
a digest comparison would report as a difference without saying what kind. All 40 exit `0`; all 40 write
nothing to standard error, digest `e3b0c442…7852b855`, the sha256 of no bytes; the 40 streams total
64,928,409 bytes. The sink comparison is against `ratification/` rather than `base/`, because the
owner's ratification of `SPEC-MOK-006`'s 2026-08-21 row moved `schema` from 1 to 2 and with it all
forty record digests of the base capture. This also re-measures `REQ-MOK-046`: the standard-output
digest equals the no-sink digest of the same cell in 40 of 40 cells.

**The draw-count comparison, stated separately.** 20 of 20 configurations identical in all three
figures — the tick-boundary count, the sha256 of that configuration's boundary lines, and the final
entropy state — over 15,722 boundaries in total. SplitMix64 advances by a fixed odd constant per draw,
so its state is a draw counter: equal states after equal tick counts is equal draw counts. This is the
half of **L9** that output cannot show, because a source that drew a number and discarded it would leave
the same text behind and a different world one tick later. The baseline boundary counts are 121, 121,
136, 144 and 170 across the five seeds against 1,002 for the other three sources, which is `baseline`'s
population dying out and not a difference: it stops at the same boundary at the candidate as at the base.

**The record comparison, and the one difference in the whole capture.** All 40 record digests differ
from the schema-2 baseline; all 40 byte counts and all 40 line counts are equal to it. That pattern is
*consistent with* a single fixed-width character changing, and consistent is not established, so it was
established. `candidate/verify-schema-digit.sh` rewrites each candidate stream's first-line `"schema":3`
to a named digit, digests the result, and compares it to the manifest's recorded digest for that cell:

    against ratification/sink-manifest.txt, digit 2  ->  40 cells, 0 failures, exit 0
    against base/sink-manifest.txt,         digit 1  ->  40 cells, 0 failures, exit 0

Both runs are over the recapture rather than the original capture, because the original streams were not
retained and only their manifest was, and the recapture is proven identical to them cell by cell by the
paragraph above. That order matters: the recapture is established first and the digit check rests on it.

**The candidate captures describe the candidate, not only the commit they name.** Both capture modes
were re-run at HEAD with the same `capture.sh` and both manifests regenerated with the same
`manifest.sh`; all eighty cells reproduce byte for byte below line 1, and line 1 alone is excluded
because it is the header naming the capture commit. An empty `git diff` over the packages would have
been an argument; this is a measurement.

## 4. The replay identity

`candidate/replay-identity.txt` is the measurement, taken against release builds of both binary
targets.

    RESULT: every replay of the committed transcript is byte-identical run to run and with and
    without a record sink; each of the seven mismatched configurations fails and names the
    mismatch, none leaves a partial record stream, and the one case that is not a mismatch — a
    run shorter than the transcript — completes with an empty standard error.

The transcript is `mokiterions-core/tests/transcript-seed0-ticks20-hunting.jsonl`: 305,568 bytes, 233
lines, sha256 `c249be329c1e1e70664d4e934beeaa68fde9a0a999f34b49991a59d602379f60`, 12 prefix records plus
221 exchange records. It is the configuration the repository's own workflow replays at
`.github/workflows/provider-credentials.yml:143`.

**Tracing selections, six cells, two digests.** Each cell is the engine's binary target against the
committed transcript; `sink` means `--events-path` was also given.

| cell | stdout sha256 | bytes |
|---|---|---|
| traceon repeat 1 | `e35cc3a1…c035772` | 78,315 |
| traceon repeat 2 | `e35cc3a1…c035772` | 78,315 |
| traceon with sink | `e35cc3a1…c035772` | 78,315 |
| traceoff repeat 1 | `a9c1da0d…9932b5` | 39,543 |
| traceoff repeat 2 | `a9c1da0d…9932b5` | 39,543 |
| traceoff with sink | `a9c1da0d…9932b5` | 39,543 |

Two digests where there are six cells, and each set of three that collapses is the set that must: a
repeat, and a run whose only difference is that a record stream was also written. All six exit `0` and
all six write zero bytes to standard error. Standard error is *measured* rather than compared, because
rule 12.6 declines to claim byte-identity for it, and zero bytes is the stronger reading.

The two record streams — `467e02fe…874557` at 139,638 bytes with tracing on, `15c22de2…7e06ad` at 77,820
with it off — differ from each other and no identity is claimed between them: rule 12.6 fixes
byte-identity to "the matched configuration, which includes the tracing selection", and a tracing
selection is exactly what differs.

**Seeds, and the `cmp` results.** What must be identical is each stream against the recording it
replays, and that is asserted in the suite over ten cells rather than these two, by
`mokiterions-core/tests/replay.rs::a_recording_and_its_replay_agree_at_every_declared_seed_with_tracing_on_and_off`:

    seed0-traceoff 39543 text byte(s) 77820 record byte(s) 221 opportunit(ies)
    seed0-traceon 78315 text byte(s) 139638 record byte(s) 221 opportunit(ies)
    seed1-traceoff 38839 text byte(s) 76601 record byte(s) 216 opportunit(ies)
    seed1-traceon 76414 text byte(s) 137077 record byte(s) 216 opportunit(ies)
    seed42-traceoff 38999 text byte(s) 76015 record byte(s) 193 opportunit(ies)
    seed42-traceon 73134 text byte(s) 130251 record byte(s) 193 opportunit(ies)
    seed123-traceoff 37253 text byte(s) 72747 record byte(s) 175 opportunit(ies)
    seed123-traceon 68440 text byte(s) 122180 record byte(s) 175 opportunit(ies)
    seed777-traceoff 39831 text byte(s) 78449 record byte(s) 225 opportunit(ies)
    seed777-traceon 79148 text byte(s) 141394 record byte(s) 225 opportunit(ies)

The first two lines are the two cells measured from the shipped binary and all four figures agree. That
agreement is worth stating on its own: the suite drives the library in-process and the capture drives
the installed binary through a shell, and a binary that wired the port differently from the way the test
harness does would pass the suite and produce different bytes. It produces the same bytes.

**The eight boundary cases**, each changing exactly one part of the workflow's configuration.

| case | change | exit | what fired |
|---|---|---|---|
| R1 | `--policy llm`, no `--transcript-path` | 2 | the parser, before any run |
| R2 | `--transcript-path` with `--policy social` | 2 | the parser |
| R3 | a transcript path that does not exist, absolute | 1 | the host's open |
| R3b | the same, relative | 1 | measured twice on purpose, for rule 19.7 |
| R4 | `--seed 1` against a seed-0 transcript | 1 | rule 11.3.2's prefix digest, at tick 1 |
| R5 | R4 again with `--events-path` | 1 | same message, **and no record file created at all** |
| R6 | `--density 1.5` against a 0.75 transcript | 1 | rule 12.3's tick-and-actor check, at tick 9 |
| R7 | `--ticks 25` against a 20-tick transcript | 1 | rule 12.4's exhaustion path |
| R8 | `--ticks 10` against a 20-tick transcript | **0** | rule 12.5: the surplus is unread |

R5 matters because the same directory had accepted a 139,638-byte record stream minutes earlier, so the
absence of a file is the engine's behaviour and not an unwritable path — not an empty file and not a
partial one. R6 fires at tick 9 rather than tick 1 because the transcript declares no density and blocks
A and B do not vary with one; rule 12.3 names "a different seed, density or horizon" as exactly what
that check is for. Nine ticks of a wrong run were printed before it fired, and rule 12.3's standard is
still met, because what it forbids is *a plausible wrong run*, not a partial one. R8 is in the list
because a build that treated a surplus as an error would satisfy every other case in it.

## 5. The two hosts

`candidate/replay-identity.txt` again, second half.

    RESULT: the observer replays the same transcript to tick 20, and its panes agree with the
    engine binary's own summary line in eight comparisons covering twelve figures, plus four
    positions and three actions.

The engine's last line for that transcript:

    summary reason=tick_limit ticks=20 survivors=9 deaths=3 territory_a=5 territory_b=4
    food_a_low=21 food_a_medium=20 food_a_high=20 food_b_low=20 food_b_medium=20 food_b_high=21

Against the observer's reconstructed frame, `candidate/observer-screen.txt`:

| engine | observer | agree |
|---|---|---|
| `reason=tick_limit` | header: `finished tick_limit` | yes |
| `ticks=20` | footer: `tick 20` | yes |
| `survivors=9` | roster: `living 9` | yes |
| `deaths=3` | roster: `deaths 3` | yes |
| `territory_a=5` | roster: 5 entries marked A | yes |
| `territory_b=4` | roster: 3 marked B plus the hidden one, which the engine has in B | yes |
| `food_a_low=21 medium=20 high=20` | A row: `low 21  medium 20  high 20` | yes |
| `food_b_low=20 medium=20 high=21` | B row: `low 20  medium 20  high 21` | yes |

Plus four positions and three actions read off the roster against the engine's tick-20 traces — M07 at
`19:94`, M08 at `43:105`, M10 at `41:48`, M11 at `8:68`, each with `move:north` matching
`proposal:move:north` — and the roster's satiety and energy bars agreeing with the engine's
`survival_changed` lines.

  **The observer's own run is disclosed rather than presented as clean.** The observer has no
  non-interactive exit: nothing ends the program but a key press, and on Windows crossterm reads the
  console input buffer rather than the standard input handle, so keys cannot be piped in. That was
  measured — keys written to the process's standard input were not read. The capture is therefore taken
  by letting the run reach the horizon and then killing it, so the process exit status is 124, the
  killer's, and not 0. What establishes that the run finished is the frame itself, which says
  `finished tick_limit` and `tick 20`. It is reproducible in spite of that: three separate runs produced
  byte-identical standard output, 27,948 bytes each, sha256 `4a5e19c6…1d2cb71d`, because the observer
  stops redrawing once the run is finished.

**Which panes the run exercised.** Two answers, because the capture and the suite reach different panes.
In the capture, at the viewport the process was given: the header, the two territory summary rows, the
roster pane, the view pane and the footer were all drawn and all carry content, over 30 rows. The log
and the inspector were not drawn, and the header says why in its own words — `overlays: log L at height
38  inspector i at width 140`. At that size the observer announces both as overlay-only, which is
`SPEC-MOK-003`'s existing behaviour and not something this source changed. The suite reaches all six, at
a viewport where `layout::resolve` places every optional pane on screen:
`mokiterions-tui/tests/replay.rs::the_observer_replays_this_source_to_the_horizon_with_every_pane`, which
is case **L31** and is a differential against `social` at the same seed for the same five ticks rather
than an assertion under this source alone:

    the observer reached tick 5 under both sources over 60 answered opportunit(ies); panes
    PaneReport { tick: 5, roster_accounts_for_every_living: true, map_drew: true,
    log_presented: true, inspector_names_the_selection: true, filter_narrows: true,
    filter_label: "event=world_initialized", exported: true } ; footer "seed 42  ticks 5
    density 0.75%  source llm  tick 5  events 258"

  **The one coverage gap this session found.** L31 does not open a transcript. Its port is a scripted
  stub, because `Proposer` is the engine's one interface for a decision from outside itself and this host
  cannot tell a stub from a reader-backed port. So the observer's path through the real `ReplayPort` over
  a real file is exercised by this packet's captures **and by no automated test.**

**The refusal output for this source with no transcript**, and the rest of the observer's refusals:

    O1  --policy llm, no --transcript-path                        exit 2
          first line of standard error byte-identical to R1's
    O2  --transcript-path with --policy social                    exit 2
          first line byte-identical to R2's
    O4  a transcript path that does not exist, relative           exit 1
          whole standard error byte-identical to R3b's, 104 bytes, diff empty
    O6  --transcript-path naming a directory                      exit 1
          runtime error: transcript mokiterions-core/tests: Access is denied. (os error 5)

O1 and O2 differ from R1 and R2 after line one only, and only in the usage block, because two programs
with different names print different usage. Line one is identical because the refusal is made once, in
the parser both hosts share — `mokiterions-core/src/cli.rs` — which is what makes rule 20.3's refusal
hold for the observer by construction rather than by a second implementation of it. Also measured,
because the help text claims it: the observer accepts `--events-path` and ignores it, and a run given one
with a writable destination created no file there.

### Which of `REQ-MOK-077`'s refusals could not be exercised

Stated plainly, as item 5 requires. **Three obligations have no exercise at this commit, and the reason
is the same for all three: the option that triggers them does not exist in either host.**

- **Required response 3** — "In the terminal observer, asked for a live run — by a connector path, a
  live-mode selection or a spend ceiling — refuse before the terminal is entered, exit with the
  invalid-configuration status, and state on standard error that this host replays only."
- **Failure and boundary behavior, second bullet** — "The observer is given a connector path. It
  refuses with a diagnostic. It does not accept the option and act on nothing."
- **Acceptance example, failure behavior** — the observer given the transcript and a connector path
  exits 2 before entering the terminal and "writes to standard error that this host replays only and
  that a live run is the engine binary's".

What the two hosts accept today, from their own usage lines:

    Mokiterions        --seed --ticks --policy --density --trace-actions --events-path
                       --transcript-path --help
    mokiterions-tui    --seed --ticks --policy --density --transcript-path --speed
                       --start-paused --export --help

No connector path, no live-mode selection and no spend ceiling in either. Measured rather than read off
the usage text: `connector-path`, `--live`, `spend-ceiling` and `max-spend` each appear **zero** times in
either package's `src/`. The two occurrences of `live-mode` are both inside one comment block, at
`mokiterions-core/src/cli.rs:301` and `:304`, which states the position in the source itself. This is
`WO-MOK-025`'s own scope and not a shortfall — its *Out of scope* names the live-mode flag and the
spend-ceiling option as user-facing options, and the connector with its protocol implementation.

  **What can be said, measured, and it is not the obligation being met.** The observer given any of the
  three option names today exits 2 and refuses before the terminal is entered:

      mokiterions-tui ... --connector-path /bin/true   ->  exit 2
        configuration error: unknown option: --connector-path
      mokiterions-tui ... --live /bin/true             ->  exit 2
        configuration error: unknown option: --live
      mokiterions-tui ... --spend-ceiling /bin/true    ->  exit 2
        configuration error: unknown option: --spend-ceiling

  The status and the timing `REQ-MOK-077` requires are already what happens; **the message is not.**
  "unknown option" is not "this host replays only, and a live run is the engine binary's". A reviewer
  should read these three as the status being right by accident of the parser rather than as the
  obligation being met. This is escalation **E15**.

The rest of `REQ-MOK-077` is exercised. Required response 2 is L31 and the capture above. Required
response 4 is R1 and O1. *Failure and boundary behavior*'s first bullet — the observer given the source
and no transcript, refusing at start-up and naming the missing transcript without substituting another
source — is O1. Its third bullet, a host that offers the source and does not wire the port into the
entry point it uses, is
`mokiterions-tui/tests/replay.rs::the_replay_source_with_no_port_is_refused_on_the_first_tick`, which
asserts that the refusal names `policy llm` and `decision port`, that the run advanced no tick, and
that it reached no other source. Required response 1's replay half is item 4 above; its live half is
`WO-MOK-026`'s.

## 6. The request layout as built

`candidate/request-layout.txt` is the measurement, and every figure in it comes from the transcript this
repository commits, because `SPEC-MOK-007` rule 11.3 carries the request as sent: blocks A and B in a
prefix record, blocks C and D in each exchange record. A reader who distrusts the file can re-derive it
from a committed file with no build and no run.

    RESULT: averaged over the transcript's 221 requests, the cacheable prefix is 87.16 percent of a
    request in bytes, ranging from 83.24 to 91.35.

| block | n | distinct | bytes | mean | total |
|---|---|---|---|---|---|
| A — shared rules | 12 | **1** | 5,385..5,385 | 5,385.0 | 64,620 |
| B — actor | 12 | 12 | 39..40 | 39.6 | 475 |
| C — observation | 221 | 221 | 433..933 | 689.9 | 152,474 |
| D — permitted set | 221 | 33 | 78..237 | 112.7 | 24,911 |

Per exchange, the whole request is 5,939 to 6,516 bytes, mean 6,227.2.

**The token counts, as estimates.** Characters rather than bytes, because the two differ by four in the
whole request — block A's two em dashes cost three bytes each where a character counts one, and nothing
else in any block is outside ASCII. Block A is 5,381 characters, block B a mean of 39.6, block C a mean
of 689.9, block D a mean of 112.7, and the mean request 6,223.2. The arithmetic is shown so a reader can
substitute a different divisor rather than take one on trust:

| characters/token | A | B | C | D | total | prefix share |
|---|---|---|---|---|---|---|
| 3.5 | 1,537 | 11 | 197 | 32 | 1,778 | 87.1% |
| 4.0 | 1,345 | 10 | 172 | 28 | 1,556 | 87.1% |
| 4.5 | 1,196 | 9 | 153 | 25 | 1,383 | 87.1% |

Against rule 3.1's diagram — about 1,200 tokens for A, about 30 for B, about 200 for C — A lands on it
at 4.5 characters per token and C lands on it at 3.5. B does not land at all: it estimates 9 to 11
tokens rather than 30, because it is three short lines and 40 characters, and it is too small for that
to move anything. Rule 3.5's 1,230 tokens for A and B together is 2 percent above what they estimate at
4.5, which is 1,205.

  **The estimate's uncertainty is larger than its margin over the floor, and that is the point.** A
  uniform ratio is the wrong model: block A is English prose and blocks C and D are identifiers,
  underscored words and integers, which tokenize more densely. Under a split ratio the share falls,
  because the denser blocks are the variable ones — 85.5 percent at A/B 4.0 and C/D 3.5, 83.5 percent at
  A/B 4.0 and C/D 3.0, 81.8 percent at A/B 4.5 and C/D 3.0. Two of those three are below
  `REQ-MOK-070`'s floor of 85 and one is above it. **So this estimate cannot decide whether that floor
  is met, and no estimate made in this repository can.** That is not a defect in the estimate; it is
  rule 14.4's reason, restated as a measurement.

**Which enumeration rendering was chosen, and its measured cost.** The **flat verb-target list** was
built: one action per line, two spaces of indentation, under a header line. The largest of the 221 sets
is 237 bytes, at tick 13 for M04, 17 lines. The instrument re-renders each of the 221 measured sets as
the other shape — a verb, a colon, and that verb's targets space-joined — and sizes both: the same set
nested is 186 bytes, and over all 221 sets **flat 24,911 bytes against nested 19,442, a saving of
5,469** — 22.0 percent of block D and **0.40 percent of a mean request**. So the flatter list does cost
more variable bytes, exactly as the specification predicted, and the amount is four tenths of one
percent of what is sent. Two things follow and only two: the cost side of the trade-off is settled, and
it is too small to buy anything with; the other side is not settled at all, because whether a nested
list is "harder to answer well" is a question about a model's answers and no model answered anything
here. The re-rendering is a cost model and was never sent anywhere.

**No cache ratio was measured, because no provider was called.** Stated plainly, as item 6 requires.
Nothing behind any figure in that file called a provider, opened a socket, spawned a connector or read a
credential. The 87.16 percent is a byte share of a composed request; it is not a cache ratio and must
not be quoted as one. Rule 14.4 fixes the ratio as cached prompt tokens over total prompt tokens "from
the reported figures and never from a local token estimate", rule 14.8 says a replay computes no ratio
and has no ceiling, and `REQ-MOK-070`'s trigger is a completed live run. The requirement is **untriggered
rather than unmet.** What does exist is the arithmetic that will consume a provider's figures, exercised
against figures the tests declare.

**A finding in this item, reported and not repaired.** `SPEC-MOK-007` rule 11.4.1 lists five characters
that put block A outside the record stream's closed alphabet, and **two of the five do not hold**: block
A contains no parenthesis at all, and its full stops are inside that alphabet rather than outside it.
The rule's conclusion is unaffected and is measured. The list has **three sites**, not one — the rule
itself and two restatements of it in the engine's source, all three named in the capture. Rule 11.4.1 is
approved text, so this is escalation **E11**.

## 7. Each verification case, with its result and its evidence

`candidate/verification-cases.txt` is that account, and its governing sentence is the required list's
own: *"A case that cannot be run is escalated, not omitted."*

    RESULT: 67 rows, one per case and per required half-case. 59 pass, one of those against a
    figure this work order and VER-MOK-018 both state differently. One passes and is escalated
    anyway. Three pass in part and are escalated for the part that does not run. Three are owner
    acts and are not mine to make. One does not apply at this stage. No required case is omitted,
    and none is reported green on evidence that does not reach it.

**Amended 2026-08-24, at the owner's later rulings of that day.** The quoted `RESULT` above is the account
at the reading it names and is left as written. Three things in it have since moved. `L34` was added to
`VER-MOK-018` and passes on evidence that already existed, so the account is **68 rows and 60 pass**; **M1**
and **M2** are **recorded**, in `manual-assessment.md`, and with **M1** so is **L27**; and **C6** is
**attested**, in `credential-attestation.md`. `candidate/verification-cases.txt`'s third amendment block
carries the same three facts where a reader of the rows will meet them, and adds this row:

| row | case | state | why |
|---|---|---|---|
| **L34** | a transcript the platform refuses | PASS | added to the required list on 2026-08-24 by the owner's ruling on `E19`. Satisfied by evidence measured on Windows and on Linux at `dbc9e6d`, none of it taken after the ruling: the directory case in `mokiterions-core/tests/replay.rs`, whose assertion is now the missing file's case exactly, and `candidate/replay-identity.txt`'s case `R3c` and row `O6` |

The eight rows that are not a plain green:

| row | case | state | why |
|---|---|---|---|
| **L5** | the enumeration is not the core list | PASS, AND ESCALATED | passes under the reading the program enforces — 104 of 221 requests enumerate a targeted action and none of those has a set equal to the core-proposal list. Under the case's **literal** wording the figure is 117 of 221: requests with nothing to target, whose block D is the core list by coincidence rather than by derivation. The program prints the literal figure and decides nothing. Stop condition 7; **E8** |
| **L16** | every exchange retained | PASS IN PART | not exercised: "a retried exchange appears as two records". A retry needs transport retry, which this stage does not build |
| **L17** | the transcript's constraints | PASS IN PART | the fourth clause, "no value outside the closed alphabet", was **withdrawn** by rule 11.4.1 as amended 2026-08-24 and replaced by a round trip through `escape_transcript_text`. `VER-MOK-018` still states the withdrawn clause. Stop condition 6; **E9**. What holds instead is checked, in-crate |
| **L30** | the port is lent, not rebuilt | PASS, figure disclosed | both halves hold. **The ceiling is eighteen exchanges, not the case's two**, deliberately; **E10** below and `candidate/per-tick-lending.txt` in full |
| **L27 / M1** | the prompt carries no strategy | **RECORDED 2026-08-24** | one assessment seen from two sections, and it was the assurance owner's to make: `DECISION_RIGHTS.md` is explicit that an implementation agent may not self-approve an assessment unless separately named as the accountable owner, and it is not. The owner made it on 2026-08-24 over `4cfb297` — *"Met — it carries no strategy"* — and `manual-assessment.md` is the record item 7 requires, with the block quoted in full |
| **M2** | the block agrees with `SPEC-MOK-001` | **RECORDED 2026-08-24** | also the assurance owner's. Nothing in this packet detects drift between the block and those rules automatically; the assessment is the mechanism. Made 2026-08-24 — *"Met — it agrees"* — and recorded in `manual-assessment.md` with the claim-by-claim cross-check against `SPEC-MOK-001` behind it |
| **S2** | the connector's dependency surface | N/A | no connector exists, canned or otherwise, so there is no surface to check. The required list states this exclusion |
| **C1** | no credential in any produced byte | PASS IN PART | cannot be run: "a test that sets a synthetic credential value and asserts it appears in no produced byte". No code path reads a credential, so there is no value to set and no byte for it to appear in. Reported as a FINDING by the instrument itself, so the row appears in its own summary table and cannot be lost |

**Two reader traps are disclosed in that file rather than left to be walked into.** The three `#[ignore]`d
tests are named with their line numbers, because a case whose evidence is an ignored test is a false
green that looks like a real one; only one of them is cited below any row, for L9's entropy half, and
that row says it does not run in the default suite. And `candidate/replay-identity.txt` numbers its own
rows R1 to R7, which are that file's local labels for seven mismatched configurations and are **not**
`VER-MOK-018`'s resilience checks R1 to R5; every row that cites the file says which of the two it
means.

**L30 in full**, because it is the case for the defect that compiles, runs and reports success.
`candidate/per-tick-lending.txt` carries both halves plus, for each, the figures a rebuilt port would
produce — because a test that only asserts the right answer cannot tell a reader it would have caught
the wrong one.

- *The replay half.* `mokiterions-core/tests/replay.rs:1134 both_of_rule_twenty_fives_doors_carry_the_port`
  opens the committed transcript, wraps the public `ReplayPort` in a counting port and drives
  `Simulation::advance_tick` for 20 ticks, asserting the same (tick, actor) pairs in the same order as
  the recording door one made and the same exchange-record count: 221 opportunities and 221 exchange
  records, with the head's 12 prefix records door one's alone. A rebuilt port does not reach a plausible
  wrong run here — it fails at tick 2, by an already-verified refusal whose message is "record is for
  tick 2".
- *The ceiling half.* `simulation.rs:13077 a_lent_ports_cost_rises_across_ticks_and_reaches_a_ceiling`:
  three ticks, 36 exchanges, costs 12, 24 and 36 synthetic units, ceiling reached `[false, true, true]`.
  A rebuilt port would report the first figure three times and never reach it. The observer has the same
  property through its own single-tick door at `mokiterions-tui/tests/replay.rs:159`.
- *A finding that fell out of the measurement rather than being looked for.* On this transcript the
  **actor** half of rule 12.3's check would not catch a restarted cursor, because every tick's first
  opportunity belongs to M01. The **tick** half is what catches it. Cumulative records consumed are
  12/24/36 lent against 12/12/12 rebuilt for ticks 1 to 3.

## 8. The amendments made

Fifteen amendment record rows across ten artifacts, counted afresh at `77f2974`. Each names its
provision in the row itself; what follows is the artifact, the provision in short, and the approval act.
**The count this paragraph carried until 2026-08-24 was nine rows across eight artifacts and it was
stale in both figures**: rows were added to the table below as the stage went on without the sentence
above them being re-counted, and the eleven owner rulings of 2026-08-24 then added three more. The
figures are stated here as measured rather than corrected silently, because a count that drifts once
will be read as a count that was never checked.

| Artifact | Row | Provision | Approval act |
|---|---|---|---|
| `SPEC-MOK-006` | 2026-08-23, line 28 | Rule 3.2's policy and source domains gain `llm`; `schema` becomes `3` | Approved 2026-08-23 by the repository owner as accountable technical owner, by way of `ADR-MOK-007`. **The 2026-08-21 row it depended on was ratified in the same act** — this is escalation **E1** |
| `SPEC-MOK-002` | 2026-08-23, line 40 | Rules 4 and 5 amended so the decision port can be conformed to; rules 6 and 13 re-checked and recorded as unmoved | Approved 2026-08-23 by the repository owner as technical owner, by way of `ADR-MOK-007`, which states rule 4's amendment and rule 5's three amendments in full |
| `SPEC-MOK-002` | 2026-08-24, line 41 | The last two amendments `ADR-MOK-007` requires, which the 2026-08-23 row declined to write because no commit had yet made them true | The same act; written when the code made them true, in `ac827b1` |
| `SPEC-MOK-002` | 2026-08-24, line 42 | Rule 5's additions list gains `ReplayPort` and two `DecisionRequest` accessors; rule 6 gains a carve-out for a reference into a `pub(crate)` type | Approved interactively on 2026-08-24 as escalations **E13** and **E14** — *"All four as recommended"*. **The row records that the recommendation the owner approved for E14 was wrong in mechanism**: it named `EventType::as_str`, which rule 5's `'static` clause already carves out, where the six references actually remaining are `DecisionRequest`'s accessors. The conclusion did not move; the mechanism did, and the row says so |
| `SPEC-MOK-007` | 2026-08-24, line 82 | Rule 11 amended in five places so the committed transcript is described truthfully, discharging six owner rulings of 2026-08-24 | **Two acts, both recorded because neither alone suffices**: the substance is the owner's six rulings, and the instruction to write them was given interactively — *"Write into `SPEC-MOK-007` now"*. Every one of the five was found by building the transcript and measuring it |
| `SPEC-MOK-007` | 2026-08-24, line 83 | Rule 11.4.1's list of the characters that put block A outside the record stream's alphabet is corrected to the census measured over the block | Approved interactively on 2026-08-24 as escalation **E11** — *"All four as recommended"*. **This is the only ruling of the eleven that reaches source**: both restatements of the list in `mokiterions-core/src/simulation.rs` are corrected in the same commit, which is why `77f2974` supersedes `bce4229` as the candidate |
| `SPEC-MOK-004` | 2026-08-24, line 33 | Rule 11 admits `#[ignore]` for **instruments**; rules 9, 10 and 11's figures corrected | **Three approvals covering different things, and the third is OUTSTANDING.** The figure corrections are covered by `ADR-MOK-007`; the instrument class was approved interactively — *"Amend rule 11 to admit instruments"* |
| `SPEC-MOK-004` | 2026-08-24, line 34 | Rules 9 and 11's figures re-measured at `bce4229`, after four commits that closed cases the row above did not cover | Covered by `ADR-MOK-007`. No rule's substance changes and no obligation on any test or target changes |
| `SPEC-MOK-003` | 2026-08-24, line 87 | The observer becomes a host of the fifth source: rule 11's authority row, *Start-up inputs*, and the byte-identity obligation extended to the new shared option descriptions | **Written under `ADR-MOK-007`'s authorization for three provisions, and under a separate owner act the same day for four locations** — *"Amend all three now"*. No new rule and no new exit code |
| `ARCH-MOK-002` | 2026-08-24, line 45 | The observer becomes a replay host, in replay only. No boundary, dependency direction, framework selection, package split or trust property moves; the trigger list gains no member | **Two authorizations, kept separate because they are two acts**: `ADR-MOK-007`, and the owner's interactive *"Amend all five now"* |
| `ARCH-MOK-002` | 2026-08-24, line 46 | `decision_assessment.rationale` re-expressed within the validator's 2,000-character limit | **Written by the implementation agent and reported rather than presented as authorized.** Stop condition 6 in form; escalation **E7** |
| `ARCH-MOK-001` | 2026-08-24, line 56 | The model-backed source: seven provisions, one deliberately empty; *Components* gains the decision port and the transcript | Approved 2026-08-23 by the repository owner, who holds the technical owner's role, by way of `ADR-MOK-007` |
| `SPEC-MOK-001` | 2026-08-24, line 80 | A decision source outside the engine: seven provisions landing in eleven places, and one appended rule; *Actors and external systems* names **five** sources | Approved 2026-08-23 by the repository owner as technical owner, by way of `ADR-MOK-007` |
| `VER-MOK-018` | 2026-08-24, line 37 | Three cases amended: **L5** restricted to the requests that enumerate a targeted action, **L17**'s closed-alphabet clause withdrawn, **L30**'s ceiling figure withdrawn and replaced by a derivation from the tick's arity | Approved interactively on 2026-08-24 as escalations **E8**, **E9** and **E10** — *"All three as recommended"*. **This row also created this contract's amendment record**, which had none before: `VER-MOK-018` was the only verification contract of the five carrying amendments without a table, and `VER-MOK-016`'s form was followed |
| `INT-MOK-001` | 2026-08-24, line 21 | The determinism determinand: the success measure and the matching desired outcome gain the retained transcript | **Two acts.** The measure was approved 2026-08-23 by the repository owner, who holds the product owner's role; `ADR-MOK-007`'s own rationale records that it is put to the product owner "because a success measure is the product owner's". The instruction to write both was *"Amend both now"* |

Three further commits belong to this item without being amendments. `792a877` corrects two source
comments that this stage's amendments falsified — a correction of prose that had gone false, not a change
of behaviour. `8162b18` reconciles the roadmap with the chain that was approved for Phase 5, and it
carries no work-order trailer and says why: the roadmap sits outside every work order. `db8cf46` moves two
`SPEC-MOK-001` amendment rows that had been written outside their own table into it — the 2026-08-20
record-stream row, which sat alone after the table, and the 2026-08-21 waste-condition row, which sat
between the title and *Scope*. **Nothing but their position moved**, verified as a set comparison of the
table's rows against `cc54185`, and it carries no work-order trailer either, for the reason that
specification's own placement note gives: it is escalation **E16**, authorized by the owner on 2026-08-24
as a separate commit rather than borrowed onto `WO-MOK-025`'s authorization.

**Two of the eleven rulings amend an artifact that carries no amendment record at all, and they are not
rows.** `WO-MOK-025`'s own *Required verification* section named the connector half of **L32** where three
halves are in scope (**E12**), and in-scope item 11 gained the transcript reader's first automated caller
(**E17**). `WORK_ORDER.template.md` defines no amendment record, so both are written into *Lifecycle* as a
dated correction note, on the precedent `WO-MOK-014` set at line 93 of that work order. The note states
which of the three is an addition rather than a correction, because `E17` is one.

## 9. What was not verified, and why

Ten entries, and they are the same list the required verification section excludes. **Every one of them
is a provider call, an option this stage does not add, or an owner attestation. None is a check that
could have been run here and was not.**

- **L15b** — the cached-prompt-token ratio over an authorised live run of at least 200 exchanges. Needs
  a provider call and an owner authorisation. Owner-gated.
- **L24** — the published comparison of survivors, deaths and combat deaths. Needs live runs at the
  declared seeds and horizon. Owner-gated, and its honesty assessment M3 is `WO-MOK-027`'s.
- **L25** — that only fit runs are published. Needs the runs L24 needs. Owner-gated.
- **L28** — the retained authorization record for a live run. There is no live run to authorise, and
  whether an authorisation is genuine is an owner attestation and not a check.
- **C6** — the attestation that the credential is not configured in the repository's automation
  secrets. **It was the repository owner's statement to make, and it was made on 2026-08-24** —
  *"Attest — none is configured"* — and is retained in `credential-attestation.md`. It stays in this list
  because it is still not a check that was run: no check inside this repository can make it, which is what
  `scripts/check_workflow_credentials.py` says in its own output — "NOT CHECKABLE HERE: whether a provider
  credential is present in the repository's Actions secrets." **`VER-MOK-018`'s broader claim that "no check
  can see this" was withdrawn the same day**, as escalation `E20`: secret *names* are enumerable through the
  hosting platform's API at every scope a workflow can read a secret from, measured **0** at repository
  Actions secrets, Dependabot secrets, Actions variables and each of the two environments, with no
  organization scope because the account is personal. A *value* is what no check can see, and no measurement
  covers a moment other than its own, so the measurement corroborates the attestation and does not make it.
  `VER-MOK-018` still calls it the single fact the whole cost containment rests on, and it is.
- **L20**, live half — a live-mode selection with no credential present. Needs the live-mode flag.
- **L32**, three halves — the connector-path, live-mode and ceiling cases. Need options this stage does
  not add.
- **R1 and R2** — transport retry, and with them L16's retried-exchange clause. This stage builds no
  transport, so there is nothing to retry.
- **A4** — the money runs out. Needs a declared ceiling, which needs the option that declares one.
- **A7**, refusal half — needs the connector-path option.

Three structural gaps belong here rather than in the list above, because they are about the required list
and about coverage rather than about a case being owner-gated. Two are recorded and one is closed:

- **The required list's enumerated matrix omitted L20 and L32** while its prose brought each in by half.
  That was escalation **E12**, and **the owner settled it on 2026-08-24**: the `**Matrix cases**`
  enumeration now names `L20` *(in part)* and `L32` *(in part)*, and *Required verification*'s opening
  sentence names the connector-path, live-mode and ceiling halves of `L32` rather than the connector half
  alone. The defect was in the work order's wording rather than in coverage — the halves this stage can
  run were run, and the halves it cannot are the two entries above. **This no longer blocks a verification
  record**, which is why it is recorded here as settled rather than removed.
- **The observer's path through the real `ReplayPort` over a real file has no automated test.** It is
  exercised by this packet's captures alone. L31 uses a scripted stub, for the reason its own header
  gives, and that reason is sound; the consequence is still a gap and is recorded as one.
- **The required list had no case for a transcript the platform refuses**, which was escalation **E19**, and
  this one is **closed rather than recorded**: the owner ruled on 2026-08-24 that `VER-MOK-018` gain the
  case, and `L34` is it. The gap was real while it lasted — running the required list in full would not have
  found the defect `dbc9e6d` fixes — and the case rests on evidence measured on both platforms at that
  commit, so no coverage is asserted ahead of its evidence.

## 10. Every local decision taken

Under the *Authorized decision envelope*. Each with the rationale, so the owner can see what was decided
on their behalf.

1. **The trait is named `Proposer`, not `DecisionPort`.** Measured, not preferred. Rule 5's two-door
   check carries a second obligation — that `advance_tick`'s signature is one line — and the candidate
   signature is one line at **exactly 100 columns**, rustfmt's default `max_width`, with no
   `rustfmt.toml` in the repository. `DecisionPort` reaches 109 columns in that signature and 104 with
   the shortest sensible parameter name, so the check would report one door where there are two and pass
   while doing so. **The margin is zero**, which means a future parameter on this method cannot be added
   within the limit and rule 5's own sentence covers that case: the check's form must change in the same
   commit.
2. **The accumulators belong to the port, not to `Simulation`.** Rule 20.4.1 names the accumulated cost
   and the fallback count among what a port holds; case L30 only discriminates a rebuilt port if the cost
   is port-held; and rule 15.6 — a replay reports no run record — cannot be honoured by the engine,
   because rule 1.1 hands it the same `Option<Action>` whether the port is live or replaying. Property P5
   gains from the same split: the transcript flag is written by the engine and the count is moved by the
   port, so a count and a flag that disagree can actually be caught.
3. **A crate-private `accounting` module, with every operation taking `self` by value** rather than
   through a `&mut self` receiver, so rule 5's drift check still returns exactly `run` and
   `advance_tick`. Nothing public moves.
4. **The transcript's serialisation: a `prefix` record per Mokiterion at the head plus one `exchange`
   per opportunity**, rather than the whole prompt in every record. Measured on the committed transcript,
   so a reader can re-derive it from a file in the repository: a prefix record is 5,619 or 5,620 bytes,
   twelve of them cost **67,435 bytes once**, and the 221 exchange records average 1,076.5 bytes for a
   305,568-byte file. Carried in full, each exchange would average 6,696 bytes of which 5,620 would be
   repetition, and the file would be **1,480,153 bytes — 1.48 MB, 4.8 times its size.** Each exchange
   carries a digest of the head record it belongs to, which is what rule 11.3.2 then checks.
5. **Block D as one flat verb-target list.** `SPEC-MOK-007` leaves the shape to measurement, and the
   measurement is item 6: flat 24,911 bytes against nested 19,442 over the same 221 sets, 0.40 percent of
   a mean request. `WO-MOK-026` will measure the other side, which is about a model's answers and not
   about bytes.
6. **The transcript sits directly in `mokiterions-core/tests/`,** not under the evidence path. Item 14
   states that rule 1's layout does not move at this stage — no directory is added — and stop condition
   10 is the other half of the reason: a bound evidence path can never be corrected, and this is a
   fixture that may need correcting. `.gitattributes` gains `mokiterions-core/tests/*.jsonl -text` so a
   CRLF checkout cannot fail the byte comparison the fixture exists for.
7. **`--ticks 1000 --density 0.75` for the eighty-run capture matrix.** No artifact fixes that horizon.
   A thousand ticks is what `WO-MOK-019` used for the same comparison, so the two packets compare
   directly, and it is long enough that every source has exhausted the interesting part of its behaviour.
8. **The two new checks are Python under `scripts/`, and the enumerator is re-implemented from the
   specification in another language.** `check_transcript_reading.py` recomputes what block D should have
   enumerated from `SPEC-MOK-001` rules 6 and 21 and `SPEC-MOK-007` rule 7, sharing nothing with the
   engine — which is what L4 means by "an independently written enumerator". A helper shared with the
   code under test would agree with it by construction, and a reviewer could not tell that from
   agreement.
9. **`mokiterions-core/tests/no_outcome_threshold.rs` as a new engine public-tier target,** reading the
   suite's own source as text. That is unusual for a test and it is the only thing that can carry L26: an
   absence cannot be observed by running anything, because a threshold assertion nobody has written
   produces no behaviour to observe. It objects to the *pairing* — an outcome assertion inside a test
   that names the fifth source — and not to the vocabulary, because the other four sources have floors
   and must keep them.
10. **The instruments import rather than copy.** `Source`, `Report` and the comment-and-literal blanking
    are `WO-MOK-019`'s, imported and called; `MODE_VOCABULARY` is imported from
    `analysis/static-checks.py`, because check 6 re-runs that file's check 2 over the observer and
    re-running it means the same word list. A copy can drift, and the day it drifts a reader comparing
    two capture files is comparing two different instruments.
11. **`analysis/observer-screen.py` reconstructs the drawing stream into the frame an operator would
    have seen,** because the observer has no non-interactive exit and keys cannot be piped in on Windows.
    The instrument states its own limits, and the capture's non-zero exit status is disclosed rather
    than presented as clean.
12. **The static checks run over four tiers separately** — engine library, engine binary, observer
    library, observer binary — because "the library performs no filesystem operation" is a claim about
    one tier and a check over the package would answer a different question. Zero filesystem-opening
    sites in the engine library; both of this source's streams arrive as already-open handles.
13. **Instruments print paths through `as_posix`.** A `Path` prints with the running platform's
    separator, and these lines are captured into retained evidence, so the same command on Windows and
    on Linux would otherwise produce two captures differing in a line that says nothing about the run.
14. **The credential check's secret allowlist is closed — `GITHUB_TOKEN` and nothing else — and its
    credential-name scan matches upper case only.** A closed list rather than a provider-name pattern,
    because a credential named `SPARKLE_JUICE` is just as spendable as one named for its provider, and a
    secret that genuinely belongs later needs a line in a diff rather than passing silently. Upper case
    only is what keeps `persist-credentials: false` out of the findings while `LUNA_API_KEY` is caught: a
    check that cried wolf on the first would be switched off before it ever caught the second.
15. **L31 is a differential against `social` at the same seed for the same ticks,** not an assertion
    under this source alone, so each pane is measured as equal to what it is under a source nobody
    doubts.
16. **The observer's `decision_source_selected` overlay row is derived from `for_type`** instead of
    restating four identifiers, and carries one line per source rather than five on one line — the
    appended form measured 135 columns against 117 before. Derived, the row and `for_event` cannot
    disagree; what the derivation does not buy is stated in a comment rather than left to be found.
17. **The scripted stub's design and where its script lives:** in-process implementations of `Proposer`,
    defined in the test files that use them, with the ability to fail on command that cases L22 and R3
    need. No process, no socket.
18. **L30's ceiling is set to eighteen exchanges, not the case's two.** A tick holds twelve
    opportunities, so a two-exchange ceiling is reached *inside* the first tick — and a port rebuilt
    every tick would reach it too, which destroys the case's own discriminator. Eighteen is one and a
    half ticks: reached in the second tick when the port is lent, unreachable when it is rebuilt. The
    case's substance is checked and its illustrative figure is not. **This is also escalation E10**,
    because `VER-MOK-018` line 150 and `WO-MOK-025` item 11 both say two.
19. **`--locked` is passed to the clippy and test gates.** The release workflow passes it, so a lint run
    without it can resolve around `Cargo.lock` and pass locally against dependencies the release would
    refuse. It is a check on the declaration, not on today's outcome.

The shared rules block's exact prose is a local decision too, and it is measured rather than described:
5,381 characters, **one distinct value** across all twelve prefix records, held in exactly one place in
the source, which is check S7. Its content is the object of assessments M1 and M2 and those are the
assurance owner's.

## 11. Every escalation raised, and how it was resolved

**Twenty were raised. Seven were resolved as they arose. Eleven were put to the owner in one pass on
2026-08-24 — each with its measurement displayed — and all eleven were ruled in the turn the question was
asked. `E19` and `E20` were raised after those rulings, out of the defect CI found and out of a measurement
taken while assembling the material for `C6`, and both were ruled later the same day. All twenty are
settled.** A ruling is not the same as the work it authorizes: each entry below states the
ruling and what was written under it, and the acts that remain nobody's but an owner's are in the section
after this one.

### Raised and resolved

- **E1 — stop condition 5.** `SPEC-MOK-006`'s 2026-08-21 amendment row was still OUTSTANDING when a
  record-stream domain had to gain a value. Escalated before any `schema` value was written, because the
  increment is to one more than whatever the ratification leaves standing and guessing it would put a
  wrong version number into a stream that later becomes provenance. **Resolved:** the owner ratified on
  2026-08-23, `b0d8a4b`; `schema` went 2 to 3; `ratification/` holds the act and is the schema-2 baseline
  the record comparison in item 3 runs against.
- **E2 — stop condition 6, `INT-MOK-001`.** A success measure is the product owner's, and two sentences
  needed the retained transcript. **Resolved** by the owner's *"Amend both now"*; written in `8059f51`.
- **E3 — stop condition 6, `SPEC-MOK-003`.** Three provisions plus four locations. **Resolved** by
  *"Amend all three now"*; written in `4878cda`.
- **E4 — stop condition 6, `ARCH-MOK-002`.** Five clauses. **Resolved** by *"Amend all five now"*;
  written in `0aa0527`.
- **E5 — stop condition 6, `SPEC-MOK-007` rule 11.** Five places, every one found by building the
  transcript and measuring it. **Resolved** by six owner rulings of 2026-08-24 plus the instruction
  *"Write into `SPEC-MOK-007` now"*; written in `cc3479a`.
- **E6 — stop condition 6, `SPEC-MOK-004` rule 11.** The rule listed `#[ignore]` among what no tier
  requires, and this stage has three instruments that need it. **Resolved** by *"Amend rule 11 to admit
  instruments"*; written in `cc3479a`, with the figures re-measured in `3c7a551`. The third of that
  row's three approvals is still OUTSTANDING and the row says so.
- **E7 — stop condition 6 in form, and a defect of my own.** `se_harness validate` reported **FAIL**
  from `0aa0527` for twelve commits: `ARCH-MOK-002`'s `decision_assessment.rationale` exceeded the
  validator's 2,000-character cap, standing at 3,223. It went undetected because this work order's gate
  readings were taken at the base commit, where `validate` was PASS, and were not re-taken until the
  candidate — **which is the argument for re-taking them rather than carrying a base reading forward.**
  The cause is arithmetic: the field had 36 characters of headroom and `0aa0527` wrote a
  1,259-character clause into it, so shortening only that clause could not have repaired it. Repaired in
  `5ae4f46` by compressing the two earlier clauses with it, to 1,966 characters; `outcome` and the six
  `triggers` are byte-identical, the 2026-08-20 clause is carried verbatim, and every fact the
  compressed clauses carried stands in full in the amendment record rows they now point at. A second
  2026-08-24 row records the repair rather than editing the row above it, and reports it to the owner
  rather than presenting it as authorized.

  **The standing hazard that row records, carried here because it asked the completion report to carry
  it:** 34 characters of headroom is not a clause either, so the next amendment to `ARCH-MOK-002`
  reaches the same limit. `ARCH-MOK-001` is at 1,975 of 2,000, with 25. **Both architecture rationales
  are effectively full**, which is a property of the field rather than of any one ADR.

### Raised and ruled on 2026-08-24

- **E8 — stop condition 7, case L5.** The case's literal wording is wider than the check the program
  enforces: 117 of 221 requests against 104. Which reading `VER-MOK-018` intends is the owner's.
  **Ruled:** the enforced reading is the intended one, and the case is amended to say so. The 13 requests
  the wider reading adds are exactly those with nothing in perception to target, whose set equals the core
  list by the absence of a target rather than by a failure to derive — so the literal wording fails a
  conforming run. Written into `VER-MOK-018`'s **L5** with a 2026-08-24 amendment row; the check is not
  edited and still prints both figures.
- **E9 — stop condition 6, case L17.** `VER-MOK-018` still states the closed-alphabet clause that rule
  11.4.1 withdrew on 2026-08-24. The verification document needs the same amendment the specification
  had.
  **Ruled:** amend it. **L17**'s fourth clause is withdrawn and the round trip through the escaping
  function stands in its place; the three surviving clauses are untouched. Written into `VER-MOK-018` in
  the same 2026-08-24 row as E8 and E10.
- **E10 — case L30's ceiling.** `VER-MOK-018` line 150 and `WO-MOK-025` required evidence item 11 both
  say "a ceiling set to the cost of two exchanges"; the test uses eighteen, for the reason in decision
  18. Two approved artifacts and the evidence disagree on a figure.
  **Ruled:** the figure is withdrawn from both artifacts, not from the case. Each now requires a ceiling
  reached in a later tick and not in the first, derived from the run's arity and stated with the run.
  Written into `VER-MOK-018`'s **L30** and into this work order's evidence item 11, with a *Lifecycle*
  note recording it as one finding against two artifacts rather than as two findings. The retained
  evidence is not re-taken: it was already the amended requirement's evidence.
- **E11 — rule 11.4.1's character list.** Two of the five listed characters do not hold of block A, and
  the list is also **incomplete**: measured over block A's 5,381 characters, the outside-the-alphabet
  census is space 1,282, newline 90, comma 44, `<` 9, apostrophe 5, em dash 2 — while parentheses number
  **zero** and the full stop, of which there are 65, is *in* the alphabet, as is `>`. The rule's
  conclusion is unaffected. **Three sites** need the same correction: the rule and two restatements in
  the engine's source.
  **Ruled:** correct all three. The list was wrong in two places *and* incomplete in two — it omitted the
  9 less-than signs and the 5 apostrophes — so the corrected list is measured over block A rather than
  described, and each site says that it is. Written into `SPEC-MOK-007` rule 11.4.1 with a 2026-08-24
  amendment row, and into `mokiterions-core/src/simulation.rs` at both comment sites. **This is why the
  candidate commit moves**, and why gates 1 to 4 are re-run below.
- **E12 — the required list's enumerated matrix omits L20 and L32** while its prose brings each in by
  half.
  **Ruled:** name both in the enumeration, each marked *(in part)*, and leave the paragraphs as the
  statement of which part. The opening sentence of *Required verification* is corrected in the same pass:
  it said "the connector half of **L32**" where three halves are out of scope. No obligation moves —
  `WO-MOK-026` keeps every half it had. Written into `WO-MOK-025` with a *Lifecycle* note.
- **E13 — four added public items are outside every list `SPEC-MOK-002` rule 5 carries:** `ReplayPort`,
  `ReplayPort::new`, `DecisionRequest::tick`, `DecisionRequest::actor_id`. Eight of the twelve are
  enumerated.
  **Ruled:** enumerate all four. `ReplayPort` gains a growth row and an admissibility row stating that it
  cannot be narrowed to `pub(crate)`, because `mokiterions-core/src/main.rs:85` and
  `mokiterions-tui/src/main.rs:118` both construct it under `SPEC-MOK-007` rule 12.1.1 and `ARCH-MOK-002`.
  The two accessors join the `DecisionRequest` row. The growth arithmetic becomes `1 + 1 + 1 + 1 + 1`, and
  the five-items-versus-twelve-declarations reconciliation is now stated with its decomposition rather
  than left to be re-derived. Written into `SPEC-MOK-002` with a 2026-08-24 amendment row.
- **E14 — rule 5 property (b)'s `&'static str` carve-out is stale.** Check 4 reports a FINDING at this
  candidate on an unchanged check, and the finding is about the check rather than about the code.
  **Ruled:** amend property (b) to carve out a referent the caller owns, alongside the existing `'static`
  carve-out. **My own recommendation on this one was wrong in its mechanism and is corrected on the
  record:** I proposed naming `EventType::as_str`, which the `'static` clause already covers; the six
  references the check actually reports are `DecisionRequest`'s accessors, borrowed from a value the
  caller was handed. The conclusion — amend property (b) — is unchanged. Written into `SPEC-MOK-002`
  rule 6 in the same row as E13. No check is edited to pass and no capability is relaxed.
- **E15 — `REQ-MOK-077` required response 3's message.** The observer's exit status and its timing are
  already right; the message is "unknown option" where the requirement asks for "this host replays only,
  and a live run is the engine binary's". Right by accident of the parser is not the obligation met.
  **Ruled: deferred to `WO-MOK-026`, and recorded here as untriggered rather than met.** That work order's
  in-scope item 6 already carries the obligation in the requirement's own words, and *Required
  verification* above already assigns **L32**'s connector-path, live-mode and ceiling halves there. **The
  obligation is untriggered at this candidate** because the shared parser accepts none of the three
  names, so nothing is accepted and silently ignored today — which is the `--events-path` defect, GitHub
  issue 40, that the deferred item exists to avoid reproducing. The alternative — giving the observer the
  message now for three option names the repository does not define — was put to the owner and declined,
  because it would mean naming options this work order's *Out of scope* excludes. **A reader must not read
  the exit `2` in the capture above as this obligation being met.**
- **E16 — two orphaned amendment record rows in `SPEC-MOK-001`,** at lines 47 and 81, which sit outside
  any table and render as literal pipe text. **Both pre-exist at the base commit** and neither is this
  work order's; found while writing the 2026-08-24 row, reported, and deliberately not repaired, because
  an approved artifact is not edited on an implementation agent's judgement.
  **Ruled:** repair them, in a separate `docs(spec)` commit **outside this work order**, carrying no
  work-order trailer and stating why — on the precedent that a change no work order authorizes says so
  rather than borrowing an authorization. Nothing in this packet depends on it and no figure here moves.
- **E17 — `check_transcript_reading.py` is not wired into anything.** Its sibling is:
  `.github/workflows/provider-credentials.yml` runs `check_workflow_credentials.py` at line 93 and its
  own 38-test suite at line 83, so the credential gate is enforced on every run rather than by hand. The
  transcript check has no such caller — it is run by hand, and a check nobody runs is a check that rots.
  Whether it should be wired in, and whether it belongs in `scripts/` at all rather than beside the
  instruments in the evidence packet, is not settled by any artifact. Both scripts are repository-owned
  and absent from `.engineering-harness.lock`.
  **Ruled:** wire it in. `.github/workflows/provider-credentials.yml` gains a third job running the
  program's own 36-test suite and then the program against the committed transcript — the same transcript
  the second job replays, and the same suite-then-check order the first job uses, for the same reason.
  Both steps are green locally. It stays in `scripts/`: its sibling is there, and a check that must be
  able to refuse a change to `.github/` cannot live inside the evidence packet of one work order. **This
  adds to in-scope item 11 rather than correcting it**, and the *Lifecycle* note says so.
- **E19 — `VER-MOK-018` has no case for a transcript the platform refuses.** Raised 2026-08-24 at
  `dbc9e6d`, out of the defect CI found. The required list covers the parser's refusal (`L32`, exit `2`)
  and a mismatch detected while replaying (`L8`), and rule 13.2's host behaviour on a read that fails has
  no case of its own on either host — so the required list, run in full, would not have found a defect
  that made a directory replay as a transcript on Linux. What would close it is a case in `VER-MOK-018`
  naming the exit code, the empty standard output and the message prefix for a transcript the platform
  refuses, exercised on more than one platform, on both hosts. **That is an owner act**: `VER-MOK-018` is
  approved, a case is not added to a required list on an implementation agent's judgement, and the
  alternative — recording it as a known gap for `WO-MOK-026` — is a decision and not a default. The
  behaviour itself is now tested and measured on both platforms; what is missing is the contract's row
  for it. `candidate/verification-cases.txt` raises the same escalation where a reader of the required
  list will meet it.
  **Ruled 2026-08-24, over commit `4cfb297`: add the case.** The deferral to `WO-MOK-026` was put beside it
  and **declined**. `VER-MOK-018` gains `L34` under `REQ-MOK-077` — a host refusal in the shape of `L33`
  rather than a content comparison in the shape of `L8` — with the exit code, the empty standard output and
  the host's own message prefix as its conditions, the message text left to the platform, and exercise on
  more than one platform as a pass condition rather than a precaution. `WO-MOK-025`'s *Matrix cases*
  enumeration names it in the same commit, `bf027c8`, so the list and the matrix agree. The case passes on
  evidence that already existed at `dbc9e6d`, and nothing was measured for it after the ruling.
- **E20 — `VER-MOK-018`'s `C6` claims more ignorance than the repository is in.** Raised 2026-08-24 while
  assembling the material for the attestation itself. `C6` said "No check can see this" of the credential's
  presence in the repository's automation secrets. Secret **names** are enumerable through the hosting
  platform's API at every scope a workflow can read a secret from, so the clause is stronger than the truth.
  The restatement of the same idea under *What a green build does and does not establish* is **not** wrong:
  it carries the qualifier "inside the repository" and is correct as written, so only `C6`'s own bullet is at
  issue. Not repaired when found — stop-and-escalate condition 6 forbids amending an approved artifact on an
  implementation agent's judgement, and this is the wording of the one check the whole cost containment rests
  on.
  **Ruled 2026-08-24: amend `C6`'s wording.** The clause is withdrawn and replaced by the measurement —
  repository Actions secrets, Dependabot secrets, Actions variables and each of the two environments, **0**
  at every one, and no organization scope because the account is personal — together with the two things that
  measurement does not do: a **value** is what no check can see, and no measurement covers a moment other
  than the one it was taken at. **So it corroborates the attestation and does not make it.** `bf027c8` writes
  the amendment; `credential-attestation.md` retains the measurement and states all three of its limits,
  including that a reading of a live remote surface is not re-derivable by a later reader as it was taken.
- **E18 — `REPOSITORY_CONTEXT.md`'s amendment,** which `ADR-MOK-007` requires and which is the
  repository owner's to write.
  **Ruled: draft it as a diff for the owner to approve or replace.** Drafting is not approving, and the
  draft is held outside the file until the owner acts on it. It stays in the list of acts that are not the
  implementation agent's, below, unchanged.
  The draft was written on 2026-08-24 and is **held outside this repository**, not under `docs/`, so that
  nothing in the tree reads as though the owner had acted and so that the worktree stays clean for the
  capture a verification record needs. It replaces one bullet — *Restricted or sensitive paths* — carrying
  the existing sentence character for character and adding the operational fact `ADR-MOK-007` decision 5
  states. It is not evidence and is not listed as evidence; it is a proposal awaiting an owner act, and this
  packet records only that it exists.

### The acts that are not the implementation agent's

Listed here so that no reader takes this report for a completion.

- **The transition of `WO-MOK-025` to `implemented`** — the engineering owner's, and **decided on
  2026-08-24**. It needs its own commit, and it must land before any verification record can bind a commit,
  so the transition commit follows the one this report is amended in. At this commit the work order is still
  `in_progress`.
- **The verification decision and its record** — the assurance owner's, and **taken on 2026-08-24** on the
  record drafted for it. `VREC-MOK-024` does not exist in the tree at this commit: `capture-verification`
  binds a commit that must already contain this packet and the `implemented` transition, so the record is
  captured later and its `verified` transition carries the owner's acceptance verbatim.
- **The M1 and M2 assessment records** — the assurance owner's, and **made on 2026-08-24**. Required
  evidence item 7 asks for a record naming the owner and the date, with the shared rules block quoted in
  full; `manual-assessment.md` is that record. What was prepared for them is in item 7 above and was not a
  substitute for them.
- **C6's attestation** — the repository owner's, **made on 2026-08-24** and retained in
  `credential-attestation.md`. It is the fact the cost containment rests on.
- **`E19`'s and `E20`'s rulings** — both taken on 2026-08-24 over commit `4cfb297`, and both written in
  `bf027c8`: `VER-MOK-018` gains `L34`, and `C6`'s wording is corrected.
- **`REPOSITORY_CONTEXT.md`'s amendment** — the repository owner's, per `ADR-MOK-007`. **Still
  outstanding.** Drafted as a one-bullet diff and held outside the repository; drafting is not approving.
