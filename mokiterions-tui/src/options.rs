//! Start-up inputs, as `SPEC-MOK-002` fixes them.
//!
//! The four simulation inputs are not re-parsed here. The observer extracts its own three
//! inputs from the argument list and hands the remainder to `mokiterions_core::cli::parse`,
//! so "identical names, identical parsing, identical validation, identical defaults and
//! identical rejection behavior" holds by construction rather than by duplication.

use mokiterions_core::cli::{self, Command};
use mokiterions_core::simulation::Config;

/// The observer's own usage text. It is not the engine's, because the binary name and the
/// three additional inputs differ; every shared input keeps the engine's meaning.
pub const USAGE: &str = "Usage: mokiterions-tui [--seed <u64>] [--ticks <u64>]\n                       [--policy <baseline|reference>] [--density <percent>]\n                       [--speed <1|2|4|8|16|32|64>] [--start-paused]\n                       [--export <path>]\n       mokiterions-tui --help\n\nThe observer presents a running simulation in a terminal. It never mutates world\nstate: the operator's only influence over the simulation is when a tick is advanced.\n\n--seed, --ticks, --policy and --density carry exactly the meaning, the defaults and\nthe validation the mokiterions binary gives them.\n\n--speed is the number of ticks advanced per second while progression runs, and it\ndefaults to 8. --start-paused begins held before tick 1. --export supplies the path\nthe export control writes to; it is validated as a string only and is never opened\nuntil the operator asks for an export.\n\nAction tracing is always on in the observer, because the event log presents traced\nactions and the authority overlay maps them. Tracing does not change a run.\n\nPress ? inside the observer for the key bindings.\n";

/// The speed steps `SPEC-MOK-002` fixes, ascending. `+` and `-` step through this list.
pub const SPEED_STEPS: [u32; 7] = [1, 2, 4, 8, 16, 32, 64];

/// The default speed `SPEC-MOK-002` fixes.
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

#[cfg(test)]
mod tests {
    use super::*;
    use mokiterions_core::simulation::{Density, Policy};

    fn run(args: &[&str]) -> Options {
        match parse(args.to_vec()).unwrap() {
            Startup::Run(options) => options,
            Startup::Help => panic!("expected a run"),
        }
    }

    #[test]
    fn defaults_match_the_specified_values() {
        let options = run(&[]);

        assert_eq!(options.speed, 8);
        assert!(!options.start_paused);
        assert_eq!(options.export_path, None);
        assert_eq!(options.config.seed, 0);
        assert_eq!(options.config.tick_limit, 100);
        assert_eq!(options.config.policy, Policy::Reference);
        assert_eq!(options.config.density, Density::DEFAULT);
    }

    #[test]
    fn tracing_is_always_on_and_cannot_be_turned_off() {
        assert!(run(&[]).config.trace_actions);
        assert!(run(&["--trace-actions"]).config.trace_actions);
    }

    #[test]
    fn simulation_inputs_keep_the_engine_parser_and_its_rejections() {
        let options = run(&[
            "--seed",
            "42",
            "--ticks",
            "7",
            "--policy",
            "baseline",
            "--density",
            "1.5",
        ]);
        assert_eq!(options.config.seed, 42);
        assert_eq!(options.config.tick_limit, 7);
        assert_eq!(options.config.policy, Policy::Baseline);
        assert_eq!(options.config.density, Density::parse("1.50").unwrap());

        // Every engine rejection is the engine's, unchanged.
        assert!(parse(vec!["--ticks", "0"]).is_err());
        assert!(parse(vec!["--density", "0.01"]).is_err());
        assert!(parse(vec!["--policy", "random"]).is_err());
        assert!(parse(vec!["--seed", "1", "--seed", "2"]).is_err());
        assert!(parse(vec!["--unknown"]).is_err());
    }

    #[test]
    fn observer_inputs_are_validated() {
        assert_eq!(run(&["--speed", "64"]).speed, 64);
        assert!(run(&["--start-paused"]).start_paused);
        assert_eq!(
            run(&["--export", "events.log"]).export_path.as_deref(),
            Some("events.log")
        );

        assert!(parse(vec!["--speed", "3"]).is_err());
        assert!(parse(vec!["--speed", "0"]).is_err());
        assert!(parse(vec!["--speed", "128"]).is_err());
        assert!(parse(vec!["--speed", "abc"]).is_err());
        assert!(parse(vec!["--speed"]).is_err());
        assert!(parse(vec!["--speed", "--start-paused"]).is_err());
        assert!(parse(vec!["--speed", "8", "--speed", "16"]).is_err());
        assert!(parse(vec!["--start-paused", "--start-paused"]).is_err());
        assert!(parse(vec!["--export"]).is_err());
        assert!(parse(vec!["--export", ""]).is_err());
        assert!(parse(vec!["--export", "a", "--export", "b"]).is_err());
    }

    #[test]
    fn help_wins_over_every_other_input() {
        assert_eq!(parse(vec!["--help"]).unwrap(), Startup::Help);
        assert_eq!(
            parse(vec!["--speed", "16", "--help", "--start-paused"]).unwrap(),
            Startup::Help
        );
    }

    #[test]
    fn an_export_path_is_taken_verbatim_as_data() {
        // A path that looks like an option value, a flag or a traversal is a string.
        for path in ["-x", "a b/c.log", "../../events.log", "sub/dir/events.log"] {
            assert_eq!(
                run(&["--export", path]).export_path.as_deref(),
                Some(path),
                "{path}"
            );
        }
    }

    #[test]
    fn speed_steps_are_clamped_at_both_ends() {
        assert_eq!(faster(1), 2);
        assert_eq!(faster(32), 64);
        assert_eq!(faster(64), 64);
        assert_eq!(slower(64), 32);
        assert_eq!(slower(2), 1);
        assert_eq!(slower(1), 1);
    }
}
