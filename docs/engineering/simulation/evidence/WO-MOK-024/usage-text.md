# WO-MOK-024 evidence: both usage texts before and after

`WO-MOK-024`'s *Evidence to record* requires both texts as rendered, the width check, and the three
implicit facts named in its *Objective* traced to where each is now stated. This is that record.

These are observations of the working tree. They approve nothing, create no candidate commit, and
authorize no release. `VER-MOK-004` is the verification contract they serve; the accountable assurance
decision is the owner's act. **Updated 2026-08-22**: this paragraph read "`WO-MOK-024` is `draft` with five
**OUTSTANDING** amendments, so no verification record exists yet", which was true when the texts were
captured. The owner has since ratified those five and directed the transition to `implemented`, and
`VREC-MOK-022` exists at status `ready` — prepared, not accepted. A **sixth** amendment, to `VER-MOK-004`
itself, was found afterwards while executing that contract and was ratified later the same day; the width
figures in *Measurements* below are the ones its replacement row asks for, and none of them changed to
suit it.

## Commit binding

| Fact | Value |
| --- | --- |
| Pre-change text rendered from | `f7b1c452039dc2f03010ca8b8cc81e73c54727c0`, the branch point and the tip of `master` |
| Method for the pre-change text | `git worktree add` at that commit, then `cargo run -- --help` in the clean tree. The text was **not** reconstructed from a diff. |
| Post-change text rendered from | the working tree of `feature/help-output-clarity` that became the commit |
| Implementation branch | `feature/help-output-clarity` |
| Candidate commit | the commit `VREC-MOK-022` binds; its `commit` field is the authority. This row read "none yet; this work order is `draft`" when the texts were captured, which was true then and is corrected rather than back-dated. |
| Toolchain | rustc 1.97.1 (8bab26f4f 2026-07-14), cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable |

Both programs were invoked as `cargo run -q -p <package> -- --help` and both exited `0`. The four
rendered files are `before/engine-usage.txt`, `before/observer-usage.txt`, `after/engine-usage.txt` and
`after/observer-usage.txt`; the line-by-line diffs are `engine-usage.diff.txt` and
`observer-usage.diff.txt`.

## A note on line endings

The repository is worked on under Windows with `core.autocrlf = true`, so a text file retained here is
stored `LF` and appears in a Windows working tree `CRLF`. That conversion does not reach the measurement
below, because the measurement that matters was taken from the program's own bytes: **both texts contain
zero carriage returns**, on this machine and on any other, because both constants are built from one
string literal per line with an explicit `\n` rather than from a multi-line literal that would inherit
its line endings from the checkout. The observer's constant was a single escaped literal before this
change and is now one literal per line, so it acquires that property here; the engine's already had it.

## Measurements

| Measure | Engine before | Engine after | Observer before | Observer after |
| --- | ---: | ---: | ---: | ---: |
| Lines | 38 | 71 | 22 | 84 |
| Non-blank lines | 34 | 59 | 17 | 69 |
| Bytes | 2196 | 3601 | 1114 | 4208 |
| Widest line | 81 | **80** | 84 | **79** |
| Lines past column 80 | 2 | **0** | 6 | **0** |
| Options-block entries | 7 | 7 | **0** | 8 |
| `Default: ` statements | 4 | 4 | 0 | 6 |
| `defaults to` in prose | 0 | 0 | 1 | **0** |
| States what is printed | no | **yes** | no | no — it draws a terminal |
| States the exit codes | no | **yes** | no | **yes** |
| Carriage-return bytes | 0 | 0 | 0 | 0 |

Three of these rows carry the change and are worth reading rather than scanning.

**The observer had no options block at all.** Its `--seed`, `--ticks`, `--policy` and `--density` were
described by one sentence saying they "carry exactly the meaning, the defaults and the validation the
Mokiterions binary gives them" — a pointer to a text the operator would have to go and print. It named
none of the four defaults, and its own three options were described in a run-on paragraph that stated one
default in prose and none of `--speed`'s seven accepted values as a set. Its `--help` was in the synopsis
and nowhere else. That is why its entry count moves from 0 to 8 and its `Default: ` count from 0 to 6.

**The engine's `Default: ` count does not move.** It is 4 before and 4 after, and that is the correct
outcome rather than a missed one: `REQ-MOK-018` requires the default the program applies for **every
option that has one**, and the two options whose default is an absence — `--trace-actions` and
`--events-path` — state it as an absence rather than as a value, exactly as they did before, because
printing `Default: false` would invite `--trace-actions false`, which the program rejects. Six declared
defaults, four of them values. `tests/cli.rs` asserts the count is 4 and that each occurrence is on one
raw line, and that assertion was not touched.

**Both texts now fit 80 columns; neither did before.** The two engine lines past 80 were in the retired
trailing prose, so they left with it. The observer's six were rewrapped, five of them after the first
rendering of this evidence showed them at 81 to 82 columns — the bound is the one `REQ-MOK-018`'s amended
*Verbosity stays bounded* clause states, and a text authored in the same act as that clause should not
have been the thing that violated it. The rewrap moved no word and changed no claim; `observer-usage.diff.txt`
shows it as reflowed paragraph lines.

## The three implicit facts, traced

`WO-MOK-024`'s *Objective* names three things an operator could not learn from the engine's help. Each is
traced here to where it is now stated and to the approved artifact that governs it. **No fact below was
invented by the implementation**: every one was already approved somewhere the operator could not see.

### 1. What each `--policy` value does

Before: the four names appeared in the placeholder at `before/engine-usage.txt:11`, and their behavior in
a 10-line paragraph at `:24-33` — twelve lines further down, past the end of the options block — in the
specification's own vocabulary — "seeks and consumes perceived food so
that world viability can be measured", "in proportion to its own waste tolerance, which is derived from
the seed and its identifier".

After: `after/engine-usage.txt:28-37`, four indented sub-entries inside the `--policy` entry.

| Value | Stated as | Governed by |
| --- | --- | --- |
| `baseline` | "Chooses at random among the actions that are legal for it this turn. The control case, for comparison." | `SPEC-MOK-001` *Decision sources*, baseline; the retired prose's "selects uniformly among valid actions" |
| `reference` | "Walks toward the nearest food it can see and eats it, but refuses food whose value it would partly waste." | `SPEC-MOK-001` rule 5 cases 1 and 3, and its non-waste condition |
| `individual` | "Like reference, except each Mokiterion will waste a little, by an amount the seed fixes for it alone, so two Mokiterions in the same position can act differently." | `SPEC-MOK-001` *Behavioral trait*: `waste_tolerance` derived from the seed and the identifier |
| `social` | "Like individual while it sees nobody else. When it does see another Mokiterion it may strike back, attack, threaten, close in, or keep away, depending on how afraid it is." | `SPEC-MOK-001` rules for the social source, and `fear` |

The five verbs in `social`'s description are the retired prose's own five, in its own order: answer an
attack just suffered, attack, threaten, close on, keep away. Nothing was added to the set and nothing
dropped from it.

### 2. What `--density` is a percentage of, and what the one value binds

Before: the placeholder read `<percent>` and the entry read "Resource density per territory, at most two
decimal places. Default: 0.75." What it is a percentage *of* was four lines at the very bottom of the
text, and that paragraph ended "Only the densities declared in the requirements carry a population
viability floor" — a sentence that cannot be read at all without the requirements it points at.

After: `after/engine-usage.txt:39-45`. Five facts, all pre-existing:

| Fact now stated | Source |
| --- | --- |
| a percentage of one territory's **8192** cells | `SPEC-MOK-001` *World*: `0..=127` square, Territory A `y 0..=63`; `128 × 64 = 8192` |
| at most two decimal places | `SPEC-MOK-001` *Inputs* `--density` bullet, unchanged |
| `Default: 0.75` | same bullet, unchanged |
| the one value sets the starting food, the ceiling, and the level regrowth aims for | the retired prose, relocated; `SPEC-MOK-001` *Resource density* |
| must leave at least one resource per territory, and must not exceed 100 | same *Inputs* bullet. **These two constraints were in the specification and not in the help text at all.** |
| runs are comparable only with runs at the same density | `SPEC-MOK-001` *Resource density*, closing paragraph |

The retired sentence about the viability floor is **not** restated, and that is deliberate: it is a fact
about which densities carry an obligation, not about what the option does, and it was unreadable without
`REQ-MOK-014`. `REQ-MOK-018`'s *Required response* asks for effect, default, constraint and value
meanings; a pointer to a requirement is none of the four.

### 3. What the program writes, and what its exit codes mean

Before: stated nowhere in the text. Both are fixed by `SPEC-MOK-001` *Outputs*.

After: `after/engine-usage.txt:64-71`, two short paragraphs after the order-and-repetition sentence.

| Statement | `SPEC-MOK-001` *Outputs* line it restates |
| --- | --- |
| one line naming the chosen policy, first | "The selected decision source is reported exactly once on standard output, before agent processing begins" |
| then one line per notable event, then a closing summary, all to standard output | "Deterministic simulation events and the final summary are written to standard output" |
| a configuration error goes to standard error, followed by this text | "Usage and configuration errors are written to standard error", and *Help output*'s two emission paths |
| `0` when the run finished or this text was printed | "Successful help or simulation completion exits with code `0`" |
| `2` when an option was unknown, repeated, missing its value, or outside what it accepts | "Invalid configuration exits with code `2`" |
| `1` when output could not be written | "An unrecoverable runtime or output failure exits with code `1`" |

The three codes are the whole set, which *Outputs* states in as many words. The text adds no fourth and
narrows none of the three: `2`'s four listed causes are the four the parser produces, and `1` is stated
by its consequence rather than by an enumeration, because a runtime failure is not a closed list.

## What the observer's text now discloses that its previous text did not

Two of these are the reason `SPEC-MOK-003` needs the amendment `WO-MOK-024` lists fourth.

| Disclosure | Where | Why it was silent before |
| --- | --- | --- |
| `--trace-actions` is accepted and has nothing to switch on | `after/observer-usage.txt:73-74` | The observer traces unconditionally, so the forwarded flag is a no-op. True since the observer was built; written down nowhere. |
| `--events-path` is accepted and then **ignored**; this program writes no record stream | `after/observer-usage.txt:74-77` | The `_ =>` forwarding accepts and validates it, and nothing downstream acts on it. **This is a defect, not a design.** Tracked as GitHub issue 40, deferred by the owner on 2026-08-22, and disclosed in the text until it is closed. |
| the terminal floor of 34 columns by 22 rows, and that a smaller terminal exits `2` | `after/observer-usage.txt:11-12, 81-84` | `SPEC-MOK-003` rule 5 fixes the floor and the exit `2` refusal; the operator whose terminal is refused learned neither. |
| `--speed`'s seven values are the whole accepted set, and `+`/`-` step the same list | `after/observer-usage.txt:51-54` | The synopsis carried the seven; that they are exhaustive, and that the keys share the list, was not stated. |
| `--export`'s default is a name in the working directory built from the seed and the turn reached | `after/observer-usage.txt:60-64` | `SPEC-MOK-003` *Start-up inputs* declares the default path exists; the text said only that the option "supplies the path". The name is `export.rs:15`'s `mokiterions-events-seed{seed}-ticks{tick}.log`, which the text describes rather than reproduces, so that the format is stated in one place. |
| nothing is opened or created until the export key is pressed | `after/observer-usage.txt:61-63` | Was stated, and is kept, because it is why an unwritable path is not refused at start-up. `SPEC-MOK-003` line 122. |

**`--trace-actions` and `--events-path` are named in the observer's prose and deliberately not in its
synopsis.** The synopsis is a list of what to reach for; neither of these does anything in this program,
and one of them is a defect. Putting them there would advertise them.

## Both texts as rendered

### The engine, after

```
Usage: Mokiterions [--seed <number>] [--ticks <number>]
                   [--policy <baseline|reference|individual|social>]
                   [--density <percent>] [--trace-actions]
                   [--events-path <path>]
       Mokiterions --help

Mokiterions simulates a small closed world. Twelve creatures, each also called a
Mokiterion, live on a 128 by 128 grid split into two territories, look for food,
eat it, and can die. Nothing is learned and nothing is random beyond the seed:
the same options always produce exactly the same run. Given no options at all it
runs 100 turns of the default world and prints what happened.

Options:

  --seed <number>
      Starting number for every random draw the run makes. Change it for a
      different world; keep it to repeat one exactly. Default: 0.

  --ticks <number>
      How many turns to run. In one turn every living Mokiterion gets one
      decision. Must be greater than zero. A run stops earlier only when no
      Mokiterion is left alive. Default: 100.

  --policy <baseline|reference|individual|social>
      Which fixed set of rules each Mokiterion uses to choose its next action.
      None of the four learns anything or calls a model; all four are
      deterministic. Default: reference.
      baseline    Chooses at random among the actions that are legal for it
                  this turn. The control case, for comparison.
      reference   Walks toward the nearest food it can see and eats it, but
                  refuses food whose value it would partly waste.
      individual  Like reference, except each Mokiterion will waste a little,
                  by an amount the seed fixes for it alone, so two Mokiterions
                  in the same position can act differently.
      social      Like individual while it sees nobody else. When it does see
                  another Mokiterion it may strike back, attack, threaten,
                  close in, or keep away, depending on how afraid it is.

  --density <percent>
      How much food the world holds, as a percentage of one territory's 8192
      cells, written with at most two decimal places. Default: 0.75. The one
      value sets three things together: the food present at the start, the most
      a territory can ever hold, and the level regrowth aims for. It must leave
      at least one resource per territory and must not exceed 100. Runs are
      comparable only with runs at the same density.

  --trace-actions
      Also print one trace line for every living Mokiterion every turn, giving
      the action it proposed and whether the engine accepted it. Off unless
      given. Tracing only observes: the same seed produces the same run either
      way.

  --events-path <path>
      Also write a machine-readable record stream to this file, replacing any
      file already there. No record stream is written unless given. Standard
      output is byte-for-byte the same whether or not this option is used, and
      nothing ever reads the file back.

  --help
      Print this text and exit without running a simulation.

Options may appear in any order, and each may appear at most once.

What is printed: one line naming the chosen policy, then one line per notable
event as the run proceeds, then a closing summary of how the run ended. All of
it goes to standard output. A configuration error goes to standard error,
followed by this text.

Exit status: 0 when the run finished or this text was printed, 2 when an option
was unknown, repeated, missing its value, or outside what it accepts, and 1 when
output could not be written.
```

### The engine, before

```
Usage: Mokiterions [--seed <u64>] [--ticks <u64>]
                   [--policy <baseline|reference|individual|social>]
                   [--density <percent>] [--trace-actions]
                   [--events-path <path>]
       Mokiterions --help

Options:
  --seed <u64>                   Entropy stream seed. Default: 0.
  --ticks <u64>                  Ticks to run; must be greater than zero.
                                 Default: 100.
  --policy <baseline|reference|individual|social>
                                 Decision source. Default: reference.
  --density <percent>            Resource density per territory, at most two
                                 decimal places. Default: 0.75.
  --trace-actions                Emit one action trace per living-agent decision
                                 opportunity. Off unless given.
  --events-path <path>           Write the structured record stream to the named
                                 file, replacing any file already there. No
                                 record stream is written unless given.
  --help                         Print this usage and exit without running.

Options may appear in any order and at most once.

The reference policy is a deterministic development instrument, not autonomous
behavior. It seeks and consumes perceived food so that world viability can be
measured. The baseline policy selects uniformly among valid actions. The
individual policy seeks and consumes as the reference policy does, except that
each Mokiterion also accepts food it would partly waste, in proportion to its own
waste tolerance, which is derived from the seed and its identifier. The social
policy behaves as the individual policy does while no other Mokiterion is
perceived, and otherwise answers an attack it has just suffered, attacks or
threatens a Mokiterion in contact, or closes on or keeps away from a more distant
one, according to how afraid it is.

--density is the percentage of a territory's cells that hold a resource. It sets
the initial endowment, the territory capacity, and the replenishment target
together. Only the densities declared in the requirements carry a population
viability floor.
```

### The observer, after

```
Usage: mokiterions-tui [--seed <number>] [--ticks <number>]
                       [--policy <baseline|reference|individual|social>]
                       [--density <percent>]
                       [--speed <1|2|4|8|16|32|64>] [--start-paused]
                       [--export <path>]
       mokiterions-tui --help

mokiterions-tui shows one Mokiterions run as it happens, in your terminal: the
two territories, the roster of twelve Mokiterions, a live event log, and an
inspector for the Mokiterion you select. It only watches. The one thing you
decide is when the next turn is taken. It needs a terminal of at least 34
columns by 22 rows.

The run is set up exactly as it is for the Mokiterions binary. These four
options are that binary's own, parsed and validated by the same code:

  --seed <number>
      Starting number for every random draw the run makes. Change it for a
      different world; keep it to repeat one exactly. Default: 0.

  --ticks <number>
      How many turns to run. In one turn every living Mokiterion gets one
      decision. Must be greater than zero. A run stops earlier only when no
      Mokiterion is left alive. Default: 100.

  --policy <baseline|reference|individual|social>
      Which fixed set of rules each Mokiterion uses to choose its next action.
      None of the four learns anything or calls a model; all four are
      deterministic. Default: reference.
      baseline    Chooses at random among the actions that are legal for it
                  this turn. The control case, for comparison.
      reference   Walks toward the nearest food it can see and eats it, but
                  refuses food whose value it would partly waste.
      individual  Like reference, except each Mokiterion will waste a little,
                  by an amount the seed fixes for it alone, so two Mokiterions
                  in the same position can act differently.
      social      Like individual while it sees nobody else. When it does see
                  another Mokiterion it may strike back, attack, threaten,
                  close in, or keep away, depending on how afraid it is.

  --density <percent>
      How much food the world holds, as a percentage of one territory's 8192
      cells, written with at most two decimal places. Default: 0.75. The one
      value sets three things together: the food present at the start, the most
      a territory can ever hold, and the level regrowth aims for. It must leave
      at least one resource per territory and must not exceed 100. Runs are
      comparable only with runs at the same density.

These belong to the observer:

  --speed <1|2|4|8|16|32|64>
      Turns advanced per second while the run is playing. Only those seven
      values are accepted, and + and - inside the observer step through the
      same list. Default: 8.

  --start-paused
      Open held before turn 1 rather than playing. Off unless given; either way
      the space bar holds and releases the run.

  --export <path>
      Where the export key writes the event log the observer has kept. Nothing
      is written, opened or created until you press that key, so a path that
      cannot be written is not refused here. Default: a name in the working
      directory built from the seed and the turn reached.

  --help
      Print this text and exit without opening the terminal.

Options may appear in any order, and each may appear at most once.

Action tracing is always on here, because the event log presents traced actions
and the authority overlay maps each one to the requirement that permits it.
Tracing only observes, so it cannot change a run. The Mokiterions binary's own
--trace-actions is therefore accepted and has nothing left to switch on. Its
--events-path is accepted and then ignored: this program writes no record
stream. Use the Mokiterions binary for a record stream, or the export key for
the log.

Press ? inside the observer for the key bindings.

Exit status: 0 when the observer closed normally or this text was printed,
2 when an option was unknown, repeated, missing its value, or outside what it
accepts, or the terminal is smaller than the floor above, and 1 when output
could not be written.
```

### The observer, before

```
Usage: mokiterions-tui [--seed <u64>] [--ticks <u64>]
                       [--policy <baseline|reference|individual|social>]
                       [--density <percent>]
                       [--speed <1|2|4|8|16|32|64>] [--start-paused]
                       [--export <path>]
       mokiterions-tui --help

The observer presents a running simulation in a terminal. It never mutates world
state: the operator's only influence over the simulation is when a tick is advanced.

--seed, --ticks, --policy and --density carry exactly the meaning, the defaults and
the validation the Mokiterions binary gives them.

--speed is the number of ticks advanced per second while progression runs, and it
defaults to 8. --start-paused begins held before tick 1. --export supplies the path
the export control writes to; it is validated as a string only and is never opened
until the operator asks for an export.

Action tracing is always on in the observer, because the event log presents traced
actions and the authority overlay maps them. Tracing does not change a run.

Press ? inside the observer for the key bindings.
```

## The four shared entries

`--seed`, `--ticks`, `--policy` and `--density` appear in both texts and are **byte-identical**, which is
the claim `mokiterions-tui/tests/options.rs::the_shared_entries_are_the_engines_own_words` holds. It
reads each entry out of the observer's constant — from the line opening `  --<option> ` through the last
line indented six columns — and requires `mokiterions::cli::USAGE` to contain that exact string. Four
entries, 28 lines, 1799 bytes held equal across two packages: `--seed` 3 lines and 158 bytes, `--ticks`
4 and 214, `--policy` 14 and 963, `--density` 7 and 464.

The alternative was to share one literal. It was rejected: `concat!` accepts literals and not `const`
items, so the only sharing mechanism is a macro, and a macro the observer can invoke is a public item of
the engine. `SPEC-MOK-002` rules 5 and 6 close that interface to value-only items and make any growth of
it an amendment in the same act. A presentation concern does not justify one. The duplication is
therefore deliberate, and the test is what makes it safe: it fails on a one-character divergence and
names the option, which `drift-demonstration.txt` records rather than asserts.

## What did not change

- **No engine source is in the diff.** `mokiterions-core/src/simulation.rs` is untouched, so the claim
  that no run's output, entropy sequence or exit code moved is not a claim about a careful edit — there
  is no edit to be careful about. The changed files are two usage constants, two test files, and
  documents.
- **No assertion in `mokiterions-core/tests/cli.rs` moved.** One helper did: `options_block()` now ends
  the block at the first line carrying text in column one rather than at the first blank line, because
  entries are separated by blank lines. All eighteen tests in that file pass against the new text with
  every assertion as `VREC-MOK-004` bound it. Relaxing one was the available shortcut and `WO-MOK-010`'s
  precedent forbids it.
- **Neither public interface grew.** Both constants keep the name `USAGE`, the type `&'static str` and
  `pub` visibility. No item was added to either package.
- **No dependency was added**, and no build script. Stable Rust cannot format an integer in a constant,
  and `REQ-MOK-018`'s *Constraints* forbid importing a crate that can; the printed-equals-applied
  equality is held by a test, as that clause requires.
