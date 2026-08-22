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
/// Amended 2026-08-22 under `WO-MOK-020`, to the shape the engine's text took in the same act:
/// one entry per input, each stating what the input does, what its accepted values mean and
/// what happens when it is omitted, and nothing left to prose the operator has to find. The
/// four inputs the engine owns carry the engine's own entries **verbatim**, byte for byte, and
/// `tests/options.rs` holds them equal to `mokiterions::cli::USAGE`, so the duplication cannot
/// become a divergence. What the engine accepts and this program does not act on is stated
/// rather than left silent.
///
/// One literal per output line, concatenated at compile time, for the reason the engine's
/// constant gives: a single escaped literal of this length cannot be read or reviewed.
pub const USAGE: &str = concat!(
    "Usage: mokiterions-tui [--seed <number>] [--ticks <number>]\n",
    "                       [--policy <baseline|reference|individual|social>]\n",
    "                       [--density <percent>]\n",
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
    "The run is set up exactly as it is for the Mokiterions binary. These four\n",
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
    "  --policy <baseline|reference|individual|social>\n",
    "      Which fixed set of rules each Mokiterion uses to choose its next action.\n",
    "      None of the four learns anything or calls a model; all four are\n",
    "      deterministic. Default: reference.\n",
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
    "\n",
    "  --density <percent>\n",
    "      How much food the world holds, as a percentage of one territory's 8192\n",
    "      cells, written with at most two decimal places. Default: 0.75. The one\n",
    "      value sets three things together: the food present at the start, the most\n",
    "      a territory can ever hold, and the level regrowth aims for. It must leave\n",
    "      at least one resource per territory and must not exceed 100. Runs are\n",
    "      comparable only with runs at the same density.\n",
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
    "Press ? inside the observer for the key bindings.\n",
    "\n",
    "Exit status: 0 when the observer closed normally or this text was printed,\n",
    "2 when an option was unknown, repeated, missing its value, or outside what it\n",
    "accepts, or the terminal is smaller than the floor above, and 1 when output\n",
    "could not be written.\n",
);

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
            _ => {
                engine_args.push(args[index].clone());
                index += 1;
            }
        }
    }

    match cli::parse(engine_args)? {
        Command::Help => Ok(Startup::Help),
        Command::Run(config) => Ok(Startup::Run(Options {
            config: Config {
                // The observer always traces. Rule 9.2 lets the operator filter the log to
                // `action_trace`, rule 11 maps that type to its authorizing requirement, and
                // the start-up input list is closed, so tracing cannot be an operator
                // choice. `SPEC-MOK-001` makes tracing observational, so this cannot
                // perturb a run.
                trace_actions: true,
                ..config
            },
            speed: speed.unwrap_or(DEFAULT_SPEED),
            start_paused,
            export_path,
        })),
    }
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
