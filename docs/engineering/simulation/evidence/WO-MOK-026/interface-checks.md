# VER-MOK-018 cases S4a, S6a, R3 and R5: the four checks that run free and offline

Four cases whose whole cost is a grep, re-measured at the candidate rather than carried over from
`WO-MOK-025`. `WO-MOK-026` requires `S4a` explicitly and says why: it is "re-run rather than
skipped because it costs nothing and because a stage that adds options to both hosts is exactly
where a fifth parameter could reappear on a signature this stage claims not to touch."

    candidate  6e9ca13ba70ec46696113bb742f45d78d602d41e
    base       cc54185
    measured   2026-08-29, natively on Windows 11, Git Bash
    toolchain  no rustfmt.toml and no .rustfmt.toml in the tree, so `max_width` is the default 100

Every command below is quoted with its complete output. Nothing is elided, so a line count in
this file is a line count in the tree.

## S4a --- rule 5's mechanical drift checks, in their restated form

`SPEC-MOK-002` rule 5 carries two restatements of these checks, one per stage that moved
`execute`'s signature. `S4a` exists because the 2026-08-20 restatement reads *"A fifth parameter,
a second sink, or a sink that is not optional fails the second"* and the port on `execute` **is**
that fifth parameter --- so a build that adds the port without restating the check is a build its
own specification condemns, silently, because nothing else notices. The 2026-08-23 restatement is
the repair, and it is present in the specification at
`docs/engineering/simulation/specifications/SPEC-MOK-002.md`: the standing sentence survives at
line 451 as the 2026-08-20 text it is, and lines 513-518 restate the form as three greps with the
failure condition moved to a **sixth** parameter. The check here is that both the restatement and
the tree it describes still hold.

### The three greps on `execute`, each of which must return exactly one line

```
$ grep -n 'pub fn execute' mokiterions-core/src/lib.rs
109:pub fn execute<I, S, W, E>(

$ grep -n 'records: Option<&mut dyn Write>' mokiterions-core/src/lib.rs
113:    records: Option<&mut dyn Write>,

$ grep -n 'port: Option<&mut dyn Proposer>' mokiterions-core/src/lib.rs
114:    port: Option<&mut dyn Proposer>,
```

One line each, three for three. The line numbers moved --- `WO-MOK-025`'s
`candidate/public-surface.txt` recorded 87, 91 and 92 --- and that is the point of re-running
rather than citing: the figures a check reports are not the check.

`execute` has **five** parameters and no sixth, which is rule 4's literal and the reference all
three greps compare against:

```
$ sed -n '109,115p' mokiterions-core/src/lib.rs
pub fn execute<I, S, W, E>(
    args: I,
    stdout: &mut W,
    stderr: &mut E,
    records: Option<&mut dyn Write>,
    port: Option<&mut dyn Proposer>,
) -> u8
```

The base commit had four --- `git show cc54185:mokiterions-core/src/lib.rs` matches
`pub fn execute` at 60 and the sink at 64 and matches the port **nowhere**, because the port is
this stage's addition. So the third grep is a check that did not exist at the base commit and it
returns exactly what the restatement says it must.

### The two-door check, which must return exactly `run` and `advance_tick`

```
$ grep -n 'pub fn .*&mut self' mokiterions-core/src/simulation.rs
5470:    pub fn run<W: Write>(&mut self, output: &mut W) -> io::Result<RunSummary> {
5700:    pub fn advance_tick(&mut self, port: Option<&mut dyn Proposer>) -> Result<TickOutcome, String> {
```

Exactly two lines, and both are real signatures --- which also settles the third failure mode rule
5 names, that the pattern "must also not appear in prose in that file". It matched a documentation
comment during `WO-MOK-025`'s implementation; at this candidate it matches none, so the two lines
above are two doors and not one door and one sentence.

`Simulation::run`'s enumerated form is **unchanged**: `&mut self` and a writer in,
`io::Result<RunSummary>` out. Compared to the base commit, where the same grep returns

```
2004:    pub fn run<W: Write>(&mut self, output: &mut W) -> io::Result<RunSummary> {
2101:    pub fn advance_tick(&mut self) -> Result<TickOutcome, String> {
```

`run` is character-for-character the same line at a different number, and `advance_tick` has
gained the one parameter rule 5's 2026-08-23 amendment enumerates. Two doors before, two doors
after.

### The second obligation, and it holds with no margin at all

The 2026-08-23 restatement attaches an obligation the check did not previously have:
`advance_tick`'s signature must be **one line in the source**, because a signature the formatter
wraps separates `pub fn` from `&mut self` and the pattern then matches neither line --- reporting
one door where there are two, and *passing* while doing so. Measured:

```
$ awk 'NR==5470||NR==5700{printf "%d  width=%d\n", NR, length($0)}' mokiterions-core/src/simulation.rs
5470  width=79
5700  width=100
```

`advance_tick`'s signature is **exactly 100 characters, which is exactly `max_width`.** It holds,
and it holds with **zero characters of margin.** This is a finding and it is recorded as one:

- It confirms the specification's own reasoning rather than merely agreeing with it. `SPEC-MOK-002`
  records that `Proposer` was named as it is because the width forced it and that `DecisionPort`,
  the artifacts' own words, "does not fit". `DecisionPort` is four characters longer, which is 104,
  which wraps. The rule records a measurement, and the measurement is that the identifier had
  nowhere to go.
- It means **any** future change to that signature weakens the check silently: one more character
  in a parameter name, one more generic bound, one more argument. The specification anticipates the
  parameter case --- "A future parameter on either method that cannot be added within the line
  limit must change this check's form in the same commit" --- but the margin being zero rather than
  small makes the trigger immediate, and nothing in the tree measures the width. The obligation is
  prose in a specification, and a `cargo fmt` that wraps this line would leave a green build.
- It is not a defect at this candidate. The line is one line, the check returns two doors, and
  `mokiterions-tui/tests/verification.rs`'s
  `the_engine_still_exposes_exactly_two_mutating_entry_points` --- the test that found the
  obligation in the first place, by failing --- passes.

### The interior-mutability check

```
$ grep -nE '\b(Cell|RefCell|Rc|Arc|Mutex|RwLock|Atomic[A-Za-z0-9]+)\b' \
      mokiterions-core/src/*.rs mokiterions-tui/src/*.rs | wc -l
0
```

Zero, over both packages' whole source and not only the engine's. So no `&self` method mutates
through interior mutability, which is the clause rule 5's paragraph ends on and the reason "two
mutating methods" means what it says.

### The library's public surface, for completeness

```
$ grep -n '^pub ' mokiterions-core/src/lib.rs
56:pub mod cli;
57:pub mod simulation;
80:pub const CEILING_STOP_EXIT: u8 = 3;
109:pub fn execute<I, S, W, E>(
```

Two modules, one `pub const` and one function. `CEILING_STOP_EXIT` is this stage's fourth exit
status, `SPEC-MOK-007` rule 14.6's stop, and it is a `pub const` that rule 5's census enumerates.

**S4a: PASS.** Three greps returning one line each, the two-door grep returning exactly two real
signatures, `run` unmoved in form, the interior-mutability grep returning zero, and the
restatement present in the specification --- with the zero-margin width finding recorded above.

## S6a --- no configuration field for either new path

The obligation: "The configuration value the library holds gains no field for either new path.
Both are validated by the shared parser and discarded there, on the `--events-path` precedent, so
a path cannot reach the simulation's rules by travelling inside the configuration."
`SPEC-MOK-007` rules 10.9 and 18.4 are what this measures.

The two new paths are `--connector-path` and `--transcript-output`. `--transcript-path` and
`--events-path` are earlier stages'.

### What the parser knows, and what it hands on

```
$ grep -n '"--[a-z-]*"' mokiterions-core/src/cli.rs   (distinct literals, in source order)
286  "--seed"                298  "--ticks"               312  "--policy"
324  "--density"             335  "--trace-actions"       342  "--events-path"
363  "--transcript-path"     382  "--connector-path"      399  "--transcript-output"
412  "--live"                419  "--spend-ceiling"       429  "--prices"
440  "--help"                586  "--"
```

Thirteen options and one non-option: the fourteenth hit, at `cli.rs:586`, is
`if value.starts_with("--")` inside `option_value`, the test that refuses an option's value when
the next argument is itself an option. It is listed because the grep returns it, not because it is
an option.

`cli::parse` returns `Result<Command, String>`, and `Command` is:

```
pub enum Command {
    Help,
    Run(Config),
}
```

So `Config` is the *only* thing a successful parse hands to the library. Its public fields, in
full:

```
    pub seed: u64,
    pub tick_limit: u64,
    pub policy: Policy,
    pub density: Density,
    pub trace_actions: bool,
    pub spend_ceiling: Option<u64>,
    pub prices: Option<UnitPrices>,
```

Seven fields. **None is a path, and none is a `String` or a `PathBuf` at all.** The two fields
this stage added are a `u64` behind an `Option` and a struct of integers; `SPEC-MOK-002`'s two
2026-08-29 amendment rows authorize both, and the first of those rows records that the field was
added by commit `c13c327` *before* the amendment that authorized it, which is a fact that row
discloses rather than tidies.

### Validated, then discarded, in the parser

Both new options are validated for real and then dropped. `--connector-path`:

```
            "--connector-path" => {
                if connector_path {
                    return Err("--connector-path may appear at most once".into());
                }
                let value = option_value(&args, index, "--connector-path")?;
                if value.is_empty() || value == "-" {
                    return Err(format!(
                        "invalid --connector-path value: {value}; expected a path to a connector program, and no path denotes a standard stream"
                    ));
                }
                connector_path = true;
                index += 2;
            }
```

`--transcript-output` has the same shape with its own message. In both arms `value` is bound,
checked, named in the diagnostic --- and then **nothing but the `bool` survives the arm.** The
`bool` exists to detect a repeat, not to carry a path. That is the `--events-path` precedent
exactly, and it is why an operator's path cannot reach a simulation rule: there is no field for it
to sit in and no variable that outlives the match arm.

The engine's binary target reads the two values from its own argument list, through three
constants of its own:

```
$ grep -n 'connector-path\|transcript-output\|events-path' \
      mokiterions-core/src/main.rs mokiterions-tui/src/main.rs
mokiterions-core/src/main.rs:45:const EVENTS_PATH_OPTION: &str = "--events-path";
mokiterions-core/src/main.rs:60:const CONNECTOR_PATH_OPTION: &str = "--connector-path";
mokiterions-core/src/main.rs:67:const TRANSCRIPT_OUTPUT_OPTION: &str = "--transcript-output";
```

Three hits, all three in the engine's **binary** target, and **the observer's `main.rs` names none
of the three** --- which is the same confinement `S3a` measures for the spawn, arrived at from the
options' side.

The test that holds the positive half is
`mokiterions-core/tests/cli.rs:934 the_prices_option_is_validated_and_its_four_values_are_retained`,
for the one option whose value the configuration *does* keep, and the asymmetry between it and the
two paths is documented on `Config::spend_ceiling` in the source rather than left to be inferred.

**S6a: PASS.**

## R3 --- a failure to write the transcript ends the run with an error status

The obligation, and its reason: "A live run whose exchanges were spent and not recorded produced
cost and no evidence, and it is the one failure worth aborting for."

The test, `SPEC-MOK-007` rule 19.6:

    mokiterions-core/src/simulation.rs:14308
        a_transcript_that_cannot_be_written_ends_the_run

It refuses the write at the thirteenth record --- the first exchange, after the twelve prefix
records --- and asserts three things: the run ends in an error whose text starts `transcript: `,
so a reader can tell it from the engine's own output failure; exactly 13 records reached the sink;
and the run produced **no summary**, because it stopped at the opportunity rather than after the
tick.

Two neighbours cover the destination rather than the write:

    mokiterions-core/tests/connector.rs:493
        an_existing_transcript_destination_is_refused_and_left_untouched
    mokiterions-core/tests/connector.rs:524
        a_connector_that_cannot_be_started_creates_no_transcript

**R3: PASS.** All three run on every `cargo test`, on both platforms, in the reading
`both-platforms.txt` carries.

## R5 --- no latency, throughput or wall-clock figure is a pass condition anywhere

This is a case about the contract, so it is measured over the contract. Every occurrence of the
timing vocabulary in `VER-MOK-018.md`:

```
$ grep -niE '\b(latency|throughput|wall-clock|wall clock|seconds|milliseconds|minutes|hours|timeout|elapsed|duration|faster|slower)\b' \
      docs/engineering/simulation/verification/VER-MOK-018.md
302:  that ended on its first timeout would be an instrument nobody could use, so continuing is checked rather than
309:- **R5** No latency, throughput or wall-clock figure is a pass condition anywhere in this contract. A live run takes an
310:  **estimated** 1.2 to 2.4 hours and that is a property of the provider. Recording a timing threshold would make this
```

Three hits in the whole document, and **not one of them is a pass condition**: 302 is `R2`'s
*rationale* for why continuing after exhausted retries is checked, 309 is `R5` stating its own
prohibition, and 310 is `R5`'s own estimate, carrying the word **estimated** as this contract's
convention requires. No case anywhere in the contract asserts a number of seconds.

The tree agrees, and more strongly. No assertion in either package touches a timing value at all:

```
$ grep -rnE 'assert[a-z_]*!\([^)]*\b(Instant|elapsed|Duration|timestamp|secs|millis)\b' \
      --include=*.rs mokiterions-core mokiterions-tui | wc -l
0
```

Where the vocabulary does appear:

```
$ grep -rncE '\b(Instant|SystemTime|elapsed|Duration|timestamp|::now)\b' --include=*.rs \
      mokiterions-core/src mokiterions-tui/src mokiterions-core/tests mokiterions-tui/tests
mokiterions-core/src/simulation.rs:4          all four are comments, three of them citing rule 11.4
mokiterions-tui/src/export.rs:1               a comment: "no wall-clock timestamp, no path"
mokiterions-tui/src/main.rs:29                the observer's frame pacer
mokiterions-core/tests/connector.rs:1         a comment, at line 403
mokiterions-core/tests/records.rs:2           two entries in a prohibition list
```

The 29 hits in `mokiterions-tui/src/main.rs` are one thing: the terminal host's real-time loop,
`FRAME_INTERVAL` of 33 ms and `INPUT_INTERVAL` of 16 ms and the `Instant`s that pace them. A frame
rate is not a pass condition; it is what a terminal does, and `SPEC-MOK-003` rule 1 is explicit
about the boundary --- clause 5: *"Wall-clock time is read only to decide **when** rule 1.2
advances and when rule 6 draws. It is never passed to the engine and never enters any
authoritative value."* Clause 2 is why the reading is taken from the advance rather than the
schedule, and the source says so at `mokiterions-tui/src/main.rs:268`: "falling behind slows the
run and never advances two ticks in zero elapsed time (rule 1.2)."

The two hits in `mokiterions-core/tests/records.rs` are the strongest form of this case, because
they are a check that the *product* carries no timing figure rather than a check that the contract
demands none. `every_key_in_the_stream_is_a_key_the_specification_names`
(`mokiterions-core/tests/records.rs:436`) asserts that none of twenty-one names appears as a key
anywhere in the record stream, and seven of the twenty-one are timing words:

    threshold, duration, elapsed, seconds, millis, timestamp, clock, wall

And `mokiterions-core/src/simulation.rs:16651` states the same in the engine's own test module:
"No case below asserts a duration, which is case **R5**: rule 11.4 admits no timestamp in a
transcript, so an elapsed figure is not something this repository retains."

**R5: PASS.** Three mentions in the contract, none of them a pass condition; zero timing
assertions in either package; and the record stream's key prohibition forbids seven timing names
outright.
