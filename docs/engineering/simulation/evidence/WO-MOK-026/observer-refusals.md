# Item 14: the observer's refusals — `VER-MOK-018` case `L32`

Measured at candidate `6e9ca13` on 2026-08-29 by `cases.sh`. The captures are under `cases/observer/`, with
digests in `cases-manifest.txt`.

`ADR-MOK-003` and `SPEC-MOK-004` rule 4 keep the observer a watcher: it replays and it does not spend.
`SPEC-MOK-007` rule 18.4 therefore has it refuse every option the live path added, and the refusal must name
the host that does own the option rather than reporting an unknown flag.

## The three the item names, and the two it does not

Item 14 names three — connector path, live selection and ceiling. The live path added **five** options, and
all five are refused on the same terms, so all five were measured. The two extra are recorded because a
refusal list with two silent members would be a gap nobody had looked for.

| option | exit | standard output | first line of standard error |
|---|---|---|---|
| `--connector-path <path>` | **`2`** | **0 bytes** | `configuration error: --connector-path belongs to the Mokiterions binary: …` |
| `--live` | **`2`** | **0 bytes** | `configuration error: --live belongs to the Mokiterions binary: …` |
| `--spend-ceiling 2` | **`2`** | **0 bytes** | `configuration error: --spend-ceiling belongs to the Mokiterions binary: …` |
| `--prices 125:13:1000:0` | **`2`** | **0 bytes** | `configuration error: --prices belongs to the Mokiterions binary: …` |
| `--transcript-output x.jsonl` | **`2`** | **0 bytes** | `configuration error: --transcript-output belongs to the Mokiterions binary: …` |

The message in full, for `--live`, and the other four differ only in the option they name:

```
configuration error: --live belongs to the Mokiterions binary: this program only replays --policy llm, so it starts no connector program, asks no model and spends nothing. Record a live run with that binary, then watch it back here with --transcript-path <path>
```

Then the program's usage text, 115 lines, on the same stream. Standard error is 6,518 to 6,531 bytes
depending on the option's own length; **standard output is empty in all five**.

## What the message does, and what it refuses to do

**It names the host that owns the option.** "belongs to the Mokiterions binary" — so the operator learns
where the capability lives rather than that this program has never heard of the flag.

**It says what this program is instead**: "this program only replays `--policy llm`, so it starts no connector
program, asks no model and spends nothing." That sentence appears **twice** in the output — once in the
refusal and once in the usage text — so the two cannot drift apart in one place while reading correctly in
the other.

**It offers a route, and the route is not a substitute source**: record with the engine, watch it back with
`--transcript-path`. The refusal does not silently fall back to `reference`, and it does not offer to run
`llm` without a connector.

**It is not the shared parser's unknown-option message.** The word `unknown` appears exactly once in the
6,518 bytes, on line 112, and it is in the documentation of the exit statuses — "2 when an option was
unknown, repeated, missing its value, or outside what it …". The refusal line itself never says it. Five
options the observer *deliberately* rejects are five options it knows about, and a message calling them
unknown would be false.

## None was accepted and ignored

Exit **`2`** rather than `0` is the whole of that claim: the run did not happen. An accepted-and-ignored
option would exit `0`, enter the terminal, and replay something — and the operator would have been told
nothing.

The observer's own suite carries the harder half, which no single invocation can show:
`no_invocation_this_program_accepts_carries_a_spend_ceiling` asserts `config.spend_ceiling == None` for
**every** invocation the observer accepts, against the engine's parser retaining `Some(200)` for the same
argument. So the field cannot be quietly populated by an accepted invocation either.

## Produced before the terminal was entered

`mokiterions-tui/src/main.rs`, the ordering:

```
166:fn main() -> ExitCode {
176:    let mut observer = match prepare(env::args().skip(1), &mut stdout, &mut stderr, viewport) {
186:    // `try_init` enters the alternate screen with raw input and installs a panic hook that
188:    let mut terminal = match ratatui::try_init() {
199:    let restoration = ratatui::try_restore();
```

`prepare` is at line 176 and `ratatui::try_init` at line 188: **the parse precedes the terminal, and the
refusal returns from `main` at 176's `match` without reaching 188.** There is exactly one **call** to
`ratatui::try_init` in the file — the other of the two occurrences is line 186's comment naming it — so there
is no second entry point a refusal could have passed through.

The measured consequence is visible in the captures: all five refusals wrote their message to standard error
as ordinary bytes: **each of the five captures contains 0 escape bytes (`0x1b`)**, so there is no
alternate-screen switch and no cursor movement anywhere in them. Standard output — which is where a terminal
session would have gone — is empty in all five. A refusal produced after `try_init`
would have left the screen switched and the message written into an alternate buffer the shell discards.

The observer's own test `a_refused_live_run_enters_no_terminal_and_spawns_nothing` asserts the same ordering
statically on every `cargo test`, together with the absence of any process spawn in the observer's tree
(measured in `spawn-and-passthrough.md` §3).

## The list cannot drift from the engine's

`mokiterions-tui/src/options.rs` declares `const LIVE_RUN_OPTIONS: [&str; 5]`, and
`the_refused_list_and_the_engines_own_options_agree` holds that declaration against two other things:
the list the test itself enumerates, **and the engine's own help text** — every refused option must have an
option entry in `mokiterions::cli::USAGE` and none in the observer's synopsis, and the observer's help must
name every one of them as the other binary's.

Neither list can be derived from the other: the engine's parser decides what an operator can type, and the
observer decides which of those it cannot honour. An option added to the engine's parser and not to this list
is accepted and ignored again — the shape of GitHub issue 40, and the defect four of these five options had
until 2026-08-29. An option in the list the engine does not accept is worse, because the refusal would then
claim the other binary accepts something it does not.

So this file's five is a measurement of a closed set rather than of the five somebody remembered.
