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

/// The `waste_tolerance` every `agent_initialized` record reports, in the order reported.
///
/// The trait is read back out of the authoritative event stream rather than off the simulation,
/// because the stream is what `REQ-MOK-031`'s reporting clause obliges and what an observer or an
/// analysis downstream would actually see.
fn reported_traits(output: &str) -> Vec<u8> {
    output
        .lines()
        .filter(|line| line.contains("event=agent_initialized"))
        .map(|line| {
            line.rsplit_once(",waste_tolerance:")
                .expect("every initialization record reports the trait")
                .1
                .trim_end()
                .parse()
                .expect("the trait is an integer")
        })
        .collect()
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

/// `REQ-MOK-034`: the trait-aware source holds the same floor as the reference source, on the
/// same declared seeds at the same declared density, over the same thousand ticks.
///
/// A source that declines resources must be shown not to decline its way into extinction, and
/// the floor is where `REQ-MOK-014` set it — this change is not permitted to buy individuality
/// with viability. The traits are read back from the stream and checked to span away from the
/// lower bound as well, because a run in which every Mokiterion happened to be derived a
/// tolerance of zero would pass the floor while behaving exactly like the reference source and
/// demonstrating nothing about rule 19.
#[test]
fn the_trait_aware_source_sustains_the_population_at_every_declared_density() {
    for (density, floor) in DECLARED_FLOORS {
        for seed in DECLARED_SEEDS {
            let mut simulation = Simulation::new(Config {
                policy: Policy::Individual,
                ..config_at(seed, 1_000, density)
            })
            .unwrap();
            let mut output = Vec::new();

            let summary = simulation.run(&mut output).unwrap();
            let output = String::from_utf8(output).unwrap();
            let consumed = output.matches("event=food_consumed").count();
            let traits = reported_traits(&output);

            assert_eq!(
                summary.reason(),
                TerminationReason::TickLimit,
                "seed {seed} at density {density}% ended in extinction under the trait-aware source"
            );
            assert!(
                summary.survivors() >= floor,
                "seed {seed} at density {density}% left only {} survivors under the trait-aware \
                 source, below the stated floor of {floor}",
                summary.survivors()
            );
            assert!(
                consumed > 0,
                "seed {seed} at density {density}% consumed no food under the trait-aware source"
            );

            assert_eq!(traits.len(), 12, "seed {seed} did not report twelve traits");
            // `SPEC-MOK-001`'s *Behavioral trait* as amended on 2026-08-19: `0..=40`. Written as a
            // literal because the bound is not part of the public interface and `SPEC-MOK-004`
            // rule 12 forbids widening an item to relocate or supply a test.
            assert!(
                traits.iter().all(|tolerance| *tolerance <= 40),
                "seed {seed} reported a trait outside the specified 0..=40: {traits:?}"
            );
            assert!(
                traits.iter().any(|tolerance| *tolerance > 0),
                "seed {seed} gave every Mokiterion a tolerance of zero, so the floor it held says \
                 nothing about rule 19: {traits:?}"
            );
            // The floor must be held by a population spread across the range, not by one huddled at
            // the lower bound where `REQ-MOK-033` makes this source the reference source.
            // `REQ-MOK-034` states this as a condition on the measurement itself.
            assert!(
                traits.iter().copied().max().unwrap() >= 20,
                "seed {seed}'s traits all sit in the range's lower half, so the floor it held is \
                 close to the reference source's own: {traits:?}"
            );
        }
    }
}
