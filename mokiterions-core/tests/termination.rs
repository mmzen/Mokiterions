//! Public tier, `SPEC-MOK-002` rule 8: termination by tick limit, and the emitted summary.
//!
//! Relocated from `src/simulation.rs` under `WO-MOK-003`. Every test here needs only
//! `Simulation::new`, `Simulation::run`, and the rule 5 additions `TerminationReason` and
//! the four `RunSummary` accessors. Assertions are verbatim; a private field read became
//! an accessor call on the same value, as rule 12 permits.
//!
//! Rule 8 also names termination by extinction as this file's subject. That case stays in
//! `src/simulation.rs` because it clears the resource collection and sets agent health and
//! satiety directly to force extinction within one tick. Rule 6 forbids exposing either
//! collection, rule 7 therefore places the test in the internal tier, and rule 12 forbids
//! substituting a weaker public assertion for it.

use std::io;

use mokiterions::simulation::{Config, Density, Policy, Simulation, TerminationReason};

/// The historical helper, relocated with the tests that use it. It selects the baseline
/// source so that the foundation tests keep testing the source they were written against,
/// at the default density so that they test the shipped configuration.
fn config(seed: u64, tick_limit: u64, trace_actions: bool) -> Config {
    Config {
        seed,
        tick_limit,
        policy: Policy::Baseline,
        density: Density::DEFAULT,
        trace_actions,
    }
}

#[test]
fn tick_limit_terminates_with_one_summary() {
    let mut simulation = Simulation::new(config(1, 1, false)).unwrap();
    let mut output = Vec::new();

    let summary = simulation.run(&mut output).unwrap();

    assert_eq!(summary.reason(), TerminationReason::TickLimit);
    assert_eq!(summary.ticks(), 1);
    assert_eq!(
        String::from_utf8(output)
            .unwrap()
            .matches("summary ")
            .count(),
        1
    );
}

#[test]
fn a_long_configured_run_is_bounded_and_does_not_panic() {
    let mut simulation = Simulation::new(config(123, 10_000, false)).unwrap();

    let summary = simulation.run(&mut io::sink()).unwrap();

    assert!(summary.ticks() <= 10_000);
    assert_eq!(summary.survivors() + summary.deaths(), 12);
}

#[test]
fn a_long_run_is_bounded_under_either_source() {
    for policy in [Policy::Baseline, Policy::Reference] {
        let mut simulation = Simulation::new(Config {
            seed: 123,
            tick_limit: 10_000,
            policy,
            density: Density::DEFAULT,
            trace_actions: false,
        })
        .unwrap();

        let summary = simulation.run(&mut io::sink()).unwrap();

        assert!(summary.ticks() <= 10_000);
        assert_eq!(summary.survivors() + summary.deaths(), 12);
    }
}

/// `REQ-MOK-057`: the same bound under the social source, where the population can now fall by a
/// cause other than starvation.
///
/// A named sibling, on the same reasoning the test below records. Ten thousand ticks is where a
/// combat rule that leaked would show: an unbounded `fear` accumulation, a `health` subtraction
/// that wrapped instead of saturating, a suffered-attack record that grew without being cleared, or
/// a strike attributed to a Mokiterion that had already died. The survivor-plus-death identity is
/// the assertion that catches the last of those — a Mokiterion counted in neither column, or in
/// both, is bookkeeping that combat broke.
#[test]
fn a_long_run_is_bounded_under_the_social_source() {
    for seed in [0, 1, 42, 123, 777] {
        let mut simulation = Simulation::new(Config {
            seed,
            tick_limit: 10_000,
            policy: Policy::Social,
            density: Density::DEFAULT,
            trace_actions: false,
        })
        .unwrap();

        let summary = simulation.run(&mut io::sink()).unwrap();

        assert!(summary.ticks() <= 10_000, "seed {seed}");
        assert_eq!(summary.survivors() + summary.deaths(), 12, "seed {seed}");
    }
}

/// `REQ-MOK-033`, `REQ-MOK-034`: the same bound under the trait-aware source.
///
/// A named sibling rather than a third entry in the loop above: the test above is
/// `a_long_run_is_bounded_under_either_source` and its name would then be false. `WO-MOK-010`
/// does not rename inherited tests, because a rename is indistinguishable from a removal in the
/// census `VER-MOK-010` requires.
///
/// Ten thousand ticks is the run length that would expose an unbounded fear accumulation, a
/// saturating-arithmetic slip surviving only in release mode, or a tolerance that starves its
/// holder over a horizon a thousand ticks does not reach.
#[test]
fn a_long_run_is_bounded_under_the_trait_aware_source() {
    let mut simulation = Simulation::new(Config {
        seed: 123,
        tick_limit: 10_000,
        policy: Policy::Individual,
        density: Density::DEFAULT,
        trace_actions: false,
    })
    .unwrap();

    let summary = simulation.run(&mut io::sink()).unwrap();

    assert!(summary.ticks() <= 10_000);
    assert_eq!(summary.survivors() + summary.deaths(), 12);
}
