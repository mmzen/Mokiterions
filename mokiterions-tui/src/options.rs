//! Start-up inputs, as `SPEC-MOK-003` fixes them.
//!
//! The four simulation inputs are not re-parsed here. The observer extracts its own three
//! inputs from the argument list and hands the remainder to `mokiterions::cli::parse`,
//! so "identical names, identical parsing, identical validation, identical defaults and
//! identical rejection behavior" holds by construction rather than by duplication.

use mokiterions::cli::{self, Command};
use mokiterions::simulation::Config;

/// The observer's own usage text. It is not the engine's, because the binary name and the
/// three additional inputs differ; every shared input keeps the engine's meaning.
///
/// Amended 2026-08-22 under `WO-MOK-024`, to the shape the engine's text took in the same act:
/// one entry per input, each stating what the input does, what its accepted values mean and
/// what happens when it is omitted, and nothing left to prose the operator has to find. The
/// four inputs the engine owns carry the engine's own entries **verbatim**, byte for byte, and
/// `tests/options.rs` holds them equal to `mokiterions::cli::USAGE`, so the duplication cannot
/// become a divergence. What the engine accepts and this program does not act on is stated
/// rather than left silent.
///
/// Amended again 2026-08-29 under `WO-MOK-026` for `SPEC-MOK-007` rule 18.4.2, which makes the
/// option sets differ by host and requires each host's text to state which options are its own
/// (rule 18.2). The four the engine's binary target owns are named as that binary's, together with
/// what this program does when it is given one, because the entry for `--transcript-path` is shared
/// verbatim with that binary and says "use --live instead to make a new recording" — true of the
/// pair of programs and not of this one. The exit-status sentence gains the clause for that refusal.
/// Nothing in the five shared entries moved.
///
/// One literal per output line, concatenated at compile time, for the reason the engine's
/// constant gives: a single escaped literal of this length cannot be read or reviewed.
pub const USAGE: &str = concat!(
    "Usage: mokiterions-tui [--seed <number>] [--ticks <number>]\n",
    "                       [--policy <baseline|reference|individual|social|llm>]\n",
    "                       [--density <percent>] [--transcript-path <path>]\n",
    "                       [--speed <1|2|4|8|16|32|64>] [--start-paused]\n",
    "                       [--export <path>]\n",
    "       mokiterions-tui --help\n",
    "\n",
    "mokiterions-tui shows one Mokiterions run as it happens, in your terminal: the\n",
    "two territories, the roster of twelve Mokiterions, a live event log, and an\n",
    "inspector for the Mokiterion you select. It only watches. The one thing you\n",
    "decide is when the next turn is taken. It needs a terminal of at least 34\n",
    "columns by 22 rows.\n",
    "\n",
    "The run is set up exactly as it is for the Mokiterions binary. These five\n",
    "options are that binary's own, parsed and validated by the same code:\n",
    "\n",
    "  --seed <number>\n",
    "      Starting number for every random draw the run makes. Change it for a\n",
    "      different world; keep it to repeat one exactly. Default: 0.\n",
    "\n",
    "  --ticks <number>\n",
    "      How many turns to run. In one turn every living Mokiterion gets one\n",
    "      decision. Must be greater than zero. A run stops earlier only when no\n",
    "      Mokiterion is left alive. Default: 100.\n",
    "\n",
    "  --policy <baseline|reference|individual|social|llm>\n",
    "      How each Mokiterion chooses its next action. The first four are fixed sets\n",
    "      of rules: none of them learns anything or calls a model, and all four are\n",
    "      deterministic. The fifth asks a model and is neither. Default: reference.\n",
    "      baseline    Chooses at random among the actions that are legal for it\n",
    "                  this turn. The control case, for comparison.\n",
    "      reference   Walks toward the nearest food it can see and eats it, but\n",
    "                  refuses food whose value it would partly waste.\n",
    "      individual  Like reference, except each Mokiterion will waste a little,\n",
    "                  by an amount the seed fixes for it alone, so two Mokiterions\n",
    "                  in the same position can act differently.\n",
    "      social      Like individual while it sees nobody else. When it does see\n",
    "                  another Mokiterion it may strike back, attack, threaten,\n",
    "                  close in, or keep away, depending on how afraid it is.\n",
    "      llm         Asks a language model, one question per Mokiterion per turn,\n",
    "                  each in its own context with no memory of any other. The\n",
    "                  model is reached through a separate connector program you\n",
    "                  supply, never by this program itself, so what it answers is\n",
    "                  not fixed by the seed and two runs may differ. A recorded\n",
    "                  run replays exactly from its transcript. Given no connector,\n",
    "                  the run is refused rather than run some other way.\n",
    "\n",
    "  --density <percent>\n",
    "      How much food the world holds, as a percentage of one territory's 8192\n",
    "      cells, written with at most two decimal places. Default: 0.75. The one\n",
    "      value sets three things together: the food present at the start, the most\n",
    "      a territory can ever hold, and the level regrowth aims for. It must leave\n",
    "      at least one resource per territory and must not exceed 100. Runs are\n",
    "      comparable only with runs at the same density.\n",
    "\n",
    "  --transcript-path <path>\n",
    "      Read this run's decisions from a transcript of an earlier run, rather\n",
    "      than asking a model for them again. This is how --policy llm replays;\n",
    "      use --live instead to make a new recording. One of the two is needed\n",
    "      with --policy llm and they may not be given together, and both are\n",
    "      refused with any other policy, which needs no transcript and would\n",
    "      ignore one. The replayed run reproduces the recorded one exactly and\n",
    "      stops with an error rather than guessing if the transcript does not\n",
    "      match the run being replayed.\n",
    "\n",
    "These belong to the observer:\n",
    "\n",
    "  --speed <1|2|4|8|16|32|64>\n",
    "      Turns advanced per second while the run is playing. Only those seven\n",
    "      values are accepted, and + and - inside the observer step through the\n",
    "      same list. Default: 8.\n",
    "\n",
    "  --start-paused\n",
    "      Open held before turn 1 rather than playing. Off unless given; either way\n",
    "      the space bar holds and releases the run.\n",
    "\n",
    "  --export <path>\n",
    "      Where the export key writes the event log the observer has kept. Nothing\n",
    "      is written, opened or created until you press that key, so a path that\n",
    "      cannot be written is not refused here. Default: a name in the working\n",
    "      directory built from the seed and the turn reached.\n",
    "\n",
    "  --help\n",
    "      Print this text and exit without opening the terminal.\n",
    "\n",
    "Options may appear in any order, and each may appear at most once.\n",
    "\n",
    "Action tracing is always on here, because the event log presents traced actions\n",
    "and the authority overlay maps each one to the requirement that permits it.\n",
    "Tracing only observes, so it cannot change a run. The Mokiterions binary's own\n",
    "--trace-actions is therefore accepted and has nothing left to switch on. Its\n",
    "--events-path is accepted and then ignored: this program writes no record\n",
    "stream. Use the Mokiterions binary for a record stream, or the export key for\n",
    "the log.\n",
    "\n",
    "This program only replays --policy llm. It reads every decision from the\n",
    "transcript you name and never asks a model for one, because a single question\n",
    "to a model takes longer than the whole turn this display is drawn in: the\n",
    "picture would stop moving and the keys would stop answering. --transcript-path\n",
    "is therefore this program's only share of that policy, and it is required.\n",
    "\n",
    "So the five options that make a recording rather than read one belong to the\n",
    "Mokiterions binary alone: --connector-path, --live, --transcript-output,\n",
    "--spend-ceiling and --prices. That binary accepts them and this one refuses\n",
    "them, saying which, rather than accepting one and doing nothing with it.\n",
    "Record a run there, then watch it back here.\n",
    "\n",
    "Press ? inside the observer for the key bindings.\n",
    "\n",
    "Exit status: 0 when the observer closed normally or this text was printed,\n",
    "2 when an option was unknown, repeated, missing its value, or outside what it\n",
    "accepts, or belongs to the Mokiterions binary alone, or the options given do\n",
    "not go together, or the terminal is smaller than the floor above, and 1 when\n",
    "output could not be written or a transcript could not be read.\n",
);

/// The one engine option this program acts on, rather than passes through and ignores.
///
/// Spelled here because the engine's parser validates it and keeps nothing, so each host that acts
/// on it re-reads the raw argument — `SPEC-MOK-007` rule 18.4. The engine's own binary target holds
/// the same constant for the same reason; `tests/options.rs` holds all three spellings equal.
const TRANSCRIPT_PATH_OPTION: &str = "--transcript-path";

/// The engine binary's live-run options, which this program refuses rather than acts on.
///
/// `SPEC-MOK-007` rule 18.4.2 splits the option set by host: the engine's binary target acts on six,
/// "the terminal observer acts on `--transcript-path` and on nothing else", and given a connector
/// path, a live-mode selection or a spend ceiling the observer "refuses at start-up with the
/// usage-error status and states that this host replays only". Rule 18.4.4 puts the fourth in the
/// same list in the same words — `--transcript-output` "is the binary target's alone", and "rule
/// 18.4.2's refusal covers it as it covers `--connector-path` and `--spend-ceiling`".
///
/// The refusal is necessary rather than tidy. Rule 18.4.1 records that this program hands every
/// argument it does not recognise to the engine's parser, so an option that parser accepts arrives
/// here whether or not this program can do anything with it — which is how `--events-path` came to
/// be accepted and ignored, the defect GitHub issue 40 tracks and rule 18.4.1 refuses to reproduce.
/// Every one of these was accepted by the shared parser the moment it learned it, so without this
/// list an operator would receive no connector, no recording, no ceiling, no prices and no
/// diagnostic.
///
/// `--prices` joined the list in the commit that taught the shared parser to accept it, which is
/// what the previous revision of this comment said would happen. Rule 18.4.2 enumerates six options
/// as the engine binary's and names `--prices` among them, added 2026-08-29 under `WO-MOK-030` by
/// rule 14.3a; five of the six are refused here and `--transcript-path` is the one this host shares.
const LIVE_RUN_OPTIONS: [&str; 5] = [
    "--connector-path",
    "--live",
    "--transcript-output",
    "--spend-ceiling",
    "--prices",
];

/// The speed steps `SPEC-MOK-003` fixes, ascending. `+` and `-` step through this list.
pub const SPEED_STEPS: [u32; 7] = [1, 2, 4, 8, 16, 32, 64];

/// The default speed `SPEC-MOK-003` fixes.
pub const DEFAULT_SPEED: u32 = 8;

/// Resolved start-up inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// The engine configuration, with every default resolved by the engine's own parser.
    pub config: Config,
    pub speed: u32,
    pub start_paused: bool,
    pub export_path: Option<String>,
    /// The transcript this run replays, when one was named.
    ///
    /// Not one of the observer's own three inputs: `SPEC-MOK-007` rule 18.4 has the engine's
    /// shared parser recognise and validate it, and each host "re-reads the raw argument it is the
    /// one to act on". This is that re-read. It is a path and it is data — never opened here, and
    /// opened by the binary target, which is where `SPEC-MOK-004` rules 4 and 5 put a start-up
    /// refusal that has to reach the operator's own screen.
    ///
    /// `Some` exactly when the policy is `llm`, which is not this field's own invariant but the two
    /// parsers' together. The shared parser gives one half: rule 18.4.3 refuses a transcript under
    /// any other source. The other half was the shared parser's alone until `--live` existed —
    /// rule 13.2's check now accepts `llm` with a transcript **or** a live-mode selection, so `llm`
    /// with no transcript reaches this host and is refused here, by `LIVE_RUN_OPTIONS` under rule
    /// 18.4.2, which is what leaves rule 20.3 holding. Nothing re-checks either half — a second
    /// copy of a rule is a second thing to keep in step.
    pub transcript_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Startup {
    Help,
    Run(Options),
}

/// Parses the observer's argument list.
///
/// `--help` wins over everything else, as it does in the engine binary. Every other
/// rejection is reported as a message the caller writes to standard error before the
/// terminal is entered.
pub fn parse<I, S>(args: I) -> Result<Startup, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();
    let mut speed = None;
    let mut start_paused = false;
    let mut export_path = None;
    let mut engine_args: Vec<String> = Vec::with_capacity(args.len());
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--speed" => {
                if speed.is_some() {
                    return Err("--speed may appear at most once".into());
                }
                let value = option_value(&args, index, "--speed")?;
                let parsed = value
                    .parse::<u32>()
                    .map_err(|_| format!("invalid --speed value: {value}"))?;
                if !SPEED_STEPS.contains(&parsed) {
                    return Err(format!(
                        "invalid --speed value: {value}; expected one of 1, 2, 4, 8, 16, 32, 64"
                    ));
                }
                speed = Some(parsed);
                index += 2;
            }
            "--start-paused" => {
                if start_paused {
                    return Err("--start-paused may appear at most once".into());
                }
                start_paused = true;
                index += 1;
            }
            "--export" => {
                if export_path.is_some() {
                    return Err("--export may appear at most once".into());
                }
                let value = option_value(&args, index, "--export")?;
                if value.is_empty() {
                    return Err("invalid --export value: the path is empty".into());
                }
                // The path is data. It is validated as a string, never opened here, never
                // interpreted as code, and never read from.
                export_path = Some(value.to_string());
                index += 2;
            }
            // Rule 18.4.2, before the argument reaches the shared parser. Refusing here rather than
            // after `cli::parse` is what decides which of two applicable refusals the operator
            // reads, and the host's is the one worth reading: rule 18.4.3 would answer
            // `mokiterions-tui --spend-ceiling 2` with "only used by --policy llm", which is true
            // and would send the operator to select a source that leaves this program refusing the
            // option for a second reason. What is wrong with the invocation is the program, not the
            // policy, so this arm says so and the value is never examined.
            //
            // It is not an unknown option and is not called one — the shared parser accepts every
            // one of them — and it is not silently ignored, which is the whole point of rule
            // 18.4.1. It offers no substitute source either, for rule 20.3's reason.
            //
            // `--help` given alongside does not win, unlike the two combination checks in the
            // engine's parser, and for the same reason `--speed 3 --help` does not win here: this
            // is a rejection of one named option and every such rejection in this loop is
            // immediate. The operator is not left without the answer — a refusal writes the usage
            // text after the message, and rule 18.2's entry for each of them is what it now says.
            option if LIVE_RUN_OPTIONS.contains(&option) => {
                return Err(format!(
                    "{option} belongs to the Mokiterions binary: this program only replays --policy llm, so it starts no connector program, asks no model and spends nothing. Record a live run with that binary, then watch it back here with --transcript-path <path>"
                ));
            }
            _ => {
                engine_args.push(args[index].clone());
                index += 1;
            }
        }
    }

    // The engine's verdict first, then the re-read. A positional scan is exact only on a list the
    // engine's parser accepted — it consumes every value option as a pair and rejects a value
    // beginning with `--`, so the token can appear only at an option position — and `engine_args`
    // is precisely that list, with the observer's own three inputs already taken out of it. The
    // engine's binary target reads its two paths the same way and for the same reason.
    let transcript_path = argument_after(&engine_args, TRANSCRIPT_PATH_OPTION);

    match cli::parse(engine_args)? {
        Command::Help => Ok(Startup::Help),
        Command::Run(config) => Ok(Startup::Run(Options {
            config: Config {
                // The observer always traces. Rule 9.2 lets the operator filter the log to
                // `action_trace`, rule 11 maps that type to its authorizing requirement, and
                // the start-up input list is closed, so tracing cannot be an operator
                // choice. `SPEC-MOK-001` makes tracing observational, so this cannot
                // perturb a run.
                //
                // `spend_ceiling` was overridden to `None` here and no longer is. The override was
                // the shape of defect rule 18.4.1 refuses: `--spend-ceiling` reached the shared
                // parser, was validated, was carried into the configuration and was then discarded
                // by this line, so an operator who named a ceiling received neither a ceiling nor a
                // word about it. `LIVE_RUN_OPTIONS` refuses the option instead, which makes `Some`
                // unreachable in this host rather than erased — and a defensive second copy of a
                // rule is a second thing to keep in step with it.
                trace_actions: true,
                ..config
            },
            speed: speed.unwrap_or(DEFAULT_SPEED),
            start_paused,
            export_path,
            transcript_path,
        })),
    }
}

/// The value following the named option, from a list the engine's parser accepted.
///
/// The engine's binary target holds the same function for the same reason, and
/// `tests/options.rs` holds this spelling equal to the one that parser accepts.
fn argument_after(arguments: &[String], option: &str) -> Option<String> {
    let position = arguments.iter().position(|argument| argument == option)?;
    arguments.get(position + 1).cloned()
}

/// The engine's own missing-value rule: a value must be present and must not itself look
/// like an option.
fn option_value<'a>(args: &'a [String], index: usize, option: &str) -> Result<&'a str, String> {
    let value = args
        .get(index + 1)
        .ok_or_else(|| format!("missing value for {option}"))?;
    if value.starts_with("--") {
        return Err(format!("missing value for {option}"));
    }
    Ok(value)
}

/// The next faster speed step, clamped at the fastest.
pub fn faster(speed: u32) -> u32 {
    let index = SPEED_STEPS.iter().position(|step| *step == speed);
    match index {
        Some(index) => SPEED_STEPS[(index + 1).min(SPEED_STEPS.len() - 1)],
        None => DEFAULT_SPEED,
    }
}

/// The next slower speed step, clamped at the slowest.
pub fn slower(speed: u32) -> u32 {
    let index = SPEED_STEPS.iter().position(|step| *step == speed);
    match index {
        Some(index) => SPEED_STEPS[index.saturating_sub(1)],
        None => DEFAULT_SPEED,
    }
}
