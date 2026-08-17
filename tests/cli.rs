//! Public tier, `SPEC-MOK-002` rule 8: argument parsing.
//!
//! Relocated from `src/cli.rs` under `WO-MOK-003`. Every test here needs only
//! `cli::parse`, `cli::Command`, `simulation::Config`, `simulation::Policy`, and
//! `simulation::Density`, all of which were already public. Assertions are verbatim; only
//! the path by which the code is reached changed, as rule 12 requires.

use mokiterions::cli::{Command, parse};
use mokiterions::simulation::{Config, Density, Policy};

#[test]
fn defaults_are_stable() {
    assert_eq!(
        parse(Vec::<String>::new()).unwrap(),
        Command::Run(Config {
            seed: 0,
            tick_limit: 100,
            policy: Policy::Reference,
            density: Density::DEFAULT,
            trace_actions: false,
        })
    );
}

#[test]
fn options_work_in_any_order() {
    assert_eq!(
        parse([
            "--trace-actions",
            "--ticks",
            "7",
            "--density",
            "1.5",
            "--policy",
            "baseline",
            "--seed",
            "42"
        ])
        .unwrap(),
        Command::Run(Config {
            seed: 42,
            tick_limit: 7,
            policy: Policy::Baseline,
            density: Density::parse("1.50").unwrap(),
            trace_actions: true,
        })
    );
}

#[test]
fn both_policies_are_selectable_and_reference_is_the_default() {
    let baseline = parse(["--policy", "baseline"]).unwrap();
    let reference = parse(["--policy", "reference"]).unwrap();
    let default = parse(Vec::<String>::new()).unwrap();

    assert_eq!(baseline, Command::Run(config_with(Policy::Baseline)));
    assert_eq!(reference, Command::Run(config_with(Policy::Reference)));
    assert_eq!(default, reference);
}

fn config_with(policy: Policy) -> Config {
    Config {
        seed: 0,
        tick_limit: 100,
        policy,
        density: Density::DEFAULT,
        trace_actions: false,
    }
}

#[test]
fn duplicates_and_missing_values_are_rejected() {
    assert!(parse(["--seed", "1", "--seed", "2"]).is_err());
    assert!(parse(["--ticks", "--trace-actions"]).is_err());
    assert!(parse(["--trace-actions", "--trace-actions"]).is_err());
    assert!(parse(["--unknown"]).is_err());
    assert!(parse(["--policy", "baseline", "--policy", "reference"]).is_err());
    assert!(parse(["--policy", "random"]).is_err());
    assert!(parse(["--policy"]).is_err());
    assert!(parse(["--density", "0.75", "--density", "1.50"]).is_err());
    assert!(parse(["--density"]).is_err());
}

#[test]
fn density_is_accepted_in_the_specified_forms_and_rejected_otherwise() {
    // The default is the declared default density, and it is written as a percentage.
    assert_eq!(
        parse(Vec::<String>::new()).unwrap(),
        Command::Run(config_with(Policy::Reference))
    );

    // Trailing zeros and omitted decimals denote the same density.
    assert_eq!(
        parse(["--density", "1.5"]).unwrap(),
        parse(["--density", "1.50"]).unwrap()
    );
    assert_eq!(
        parse(["--density", "2"]).unwrap(),
        parse(["--density", "2.00"]).unwrap()
    );

    // A density resolving to no resources is invalid configuration, not a valid empty
    // world, because an emptied territory can never regenerate.
    assert!(parse(["--density", "0.01"]).is_err());
    assert!(parse(["--density", "0"]).is_err());

    assert!(parse(["--density", "0.751"]).is_err());
    assert!(parse(["--density", "abc"]).is_err());
    assert!(parse(["--density", "1.2.3"]).is_err());
    assert!(parse(["--density", "-1"]).is_err());
    assert!(parse(["--density", "101"]).is_err());
}
