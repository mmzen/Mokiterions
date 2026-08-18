//! Public tier: options.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Every one of them was in
//! `src/options.rs`'s `#[cfg(test)] mod tests` block and reached the code through items that were
//! already public, so the move changes the path and nothing else: the assertions are verbatim and
//! no item was widened to bring them out. `SPEC-MOK-004` rule 12 is the obligation and the
//! per-test comparison under `WO-MOK-006` is the evidence.

use mokiterions::simulation::{Density, Policy};
use mokiterions_tui::options::*;

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
