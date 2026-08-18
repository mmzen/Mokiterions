//! Public tier, `SPEC-MOK-002` rule 8: the population floor at the declared density on the
//! declared seeds.
//!
//! Relocated from `src/simulation.rs` under `WO-MOK-003`. This test needs only
//! `Density::parse`, `Simulation::new`, `Simulation::run`, and the rule 5 additions
//! `TerminationReason` and the `RunSummary` reason and survivor accessors. Assertions are
//! verbatim, as rule 12 requires.

use mokiterions::simulation::{Config, Density, Policy, Simulation, TerminationReason};

/// The seed set declared by `VER-MOK-002`, fixed so that viability cannot be
/// demonstrated on a favourable seed chosen after the fact.
const DECLARED_SEEDS: [u64; 5] = [0, 1, 42, 123, 777];

/// The declared density from `VER-MOK-002` paired with the survivor floor `REQ-MOK-014`
/// states for it. One density is declared and it is the default, so the floor is verified
/// at the scarce density the system ships with. Survivors are not monotonic in density
/// and a floor may not be interpolated or extrapolated from this point; other densities
/// are swept as evidence in `density-curve.md` and carry no obligation.
const DECLARED_FLOORS: [(&str, usize); 1] = [("0.75", 8)];

fn config_at(seed: u64, tick_limit: u64, density: &str) -> Config {
    Config {
        seed,
        tick_limit,
        policy: Policy::Reference,
        density: Density::parse(density).unwrap(),
        trace_actions: false,
    }
}

#[test]
fn the_reference_source_sustains_the_population_at_every_declared_density() {
    for (density, floor) in DECLARED_FLOORS {
        for seed in DECLARED_SEEDS {
            let mut simulation = Simulation::new(config_at(seed, 1_000, density)).unwrap();
            let mut output = Vec::new();

            let summary = simulation.run(&mut output).unwrap();
            let consumed = String::from_utf8(output)
                .unwrap()
                .matches("event=food_consumed")
                .count();

            assert_eq!(
                summary.reason(),
                TerminationReason::TickLimit,
                "seed {seed} at density {density}% ended in extinction"
            );
            assert!(
                summary.survivors() >= floor,
                "seed {seed} at density {density}% left only {} survivors, below the stated floor of {floor}",
                summary.survivors()
            );
            assert!(
                consumed > 0,
                "seed {seed} at density {density}% consumed no food"
            );
        }
    }
}
