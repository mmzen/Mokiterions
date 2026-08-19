//! Public tier, `SPEC-MOK-002` rule 8: what a decision source proposes over a whole run, and
//! whether the engine's own validation accepts it.
//!
//! A sixth file, added under `WO-MOK-010` on rule 8's own terms — "a further file may be added
//! when a further public subject appears". The subject is new: the five files rule 8 lists cover
//! argument parsing, the process boundary, resolved density, termination and the population
//! floor, and none of them covers the proposals a source makes.
//!
//! The test below was first written inline in `src/simulation.rs`, beside the source it covers.
//! That was a rule 7 misclassification: rule 7 fixes the tier by the access a test requires and
//! states that "the subject it covers does not decide the tier", and this test requires only
//! `Config`, `Policy`, `Density`, `Simulation::new` and `Simulation::run`. Leaving it inline is
//! exactly the "left inline for convenience when rule 5 suffices" that rule 7 forbids. Its
//! assertions are verbatim from that first form, as rule 12 requires; only the construction of
//! `Config` changed, from a private test helper to the public struct literal the other files in
//! this directory use.

use mokiterions::simulation::{Config, Density, Policy, Simulation};

/// `REQ-MOK-033`: the source never proposes `wait`, and every proposal it makes is one the
/// observation already listed as valid, so rule 6 rejects none of them and is not relaxed to
/// admit them.
#[test]
fn the_trait_aware_source_never_waits_and_proposes_only_valid_actions() {
    let mut simulation = Simulation::new(Config {
        seed: 1,
        tick_limit: 500,
        policy: Policy::Individual,
        density: Density::DEFAULT,
        trace_actions: true,
    })
    .unwrap();
    let mut output = Vec::new();

    simulation.run(&mut output).unwrap();

    let output = String::from_utf8(output).unwrap();
    assert_eq!(output.matches("proposal:wait").count(), 0);
    assert_eq!(output.matches("status:rejected").count(), 0);
    assert!(output.matches("status:accepted").count() > 1_000);
}
