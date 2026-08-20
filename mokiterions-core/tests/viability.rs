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

/// `REQ-MOK-049`: under the social source at the declared density over a thousand ticks, at least
/// five of the twelve are left living and at least one death is attributable to combat, on every
/// declared seed.
///
/// Two-sided on purpose, and the second side is the one that is easy to lose. A source that
/// proposed no lethal action would pass a survivor floor perfectly, and the floor alone cannot tell
/// a world where combat is survivable from one where it never happens. `REQ-MOK-049` states the
/// lower bound as a *combat* death so that the requirement fails in both directions.
///
/// The whole curve is measured before anything is asserted. `VER-MOK-012` requires that: the
/// per-seed count is retained even on a seed that fails, "because the curve is what the floor is
/// ratified on and a suite that stops at the first failure produces no curve". So the table is
/// built first, printed, and asserted at the end, and each failure message carries the entire
/// table — the floor's value is the product owner's to ratify or amend on the first measured
/// curve, and this is the measurement that decision is made on.
///
/// The cause of death is read off the resolution record rather than off `agent_died`.
/// `REQ-MOK-044` puts it there deliberately, so that combat deaths can be told apart from
/// starvation without adding a cause field to `agent_died`; every death not reported by a
/// resolution is therefore a death by starvation or exhaustion, by elimination.
#[test]
fn the_social_source_keeps_the_world_habitable_and_combat_lethal() {
    /// `REQ-MOK-049`, transcribed. Three below `REQ-MOK-014`'s eight, which is the cost the owner
    /// accepted for combat and the reason `social` is not the default.
    const FLOOR: usize = 5;

    let mut curve = Vec::new();

    for (density, _) in DECLARED_FLOORS {
        for seed in DECLARED_SEEDS {
            let mut simulation = Simulation::new(Config {
                policy: Policy::Social,
                ..config_at(seed, 1_000, density)
            })
            .unwrap();
            let mut output = Vec::new();

            let summary = simulation.run(&mut output).unwrap();
            let output = String::from_utf8(output).unwrap();

            let combat = output.matches("target_died:yes").count();
            let deaths = output.matches("event=agent_died").count();
            curve.push((
                seed,
                summary.survivors(),
                combat,
                deaths - combat,
                summary.reason(),
            ));
        }
    }

    let table = curve
        .iter()
        .map(|(seed, survivors, combat, starved, reason)| {
            format!(
                "  seed {seed:>3}: {survivors:>2} survivors, {combat} combat deaths, \
                 {starved} other deaths, ended {reason:?}"
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("REQ-MOK-049 curve, `social`, default density, 1,000 ticks:\n{table}");

    let below: Vec<u64> = curve
        .iter()
        .filter(|(_, survivors, ..)| *survivors < FLOOR)
        .map(|(seed, ..)| *seed)
        .collect();
    let bloodless: Vec<u64> = curve
        .iter()
        .filter(|(_, _, combat, ..)| *combat == 0)
        .map(|(seed, ..)| *seed)
        .collect();

    assert!(
        below.is_empty(),
        "these declared seeds finished below the floor of {FLOOR}: {below:?}\n{table}"
    );
    assert!(
        bloodless.is_empty(),
        "these declared seeds recorded no death attributable to combat, so combat is present in \
         the rules and absent from the world: {bloodless:?}\n{table}"
    );
}

/// The Spearman rank correlation between a value's position in a series and the value itself, with
/// tied values taking the average of the ranks they span.
///
/// Averaged ties matter here rather than being a nicety: the series are small integer counts with
/// many repeats, and ranking ties by their order of appearance would manufacture a correlation with
/// identifier out of values that are equal.
fn rank_correlation(series: &[usize]) -> f64 {
    let ranks: Vec<f64> = series
        .iter()
        .map(|value| {
            let below = series.iter().filter(|other| *other < value).count() as f64;
            let equal = series.iter().filter(|other| *other == value).count() as f64;
            below + (equal + 1.0) / 2.0
        })
        .collect();
    let positions: Vec<f64> = (1..=series.len()).map(|position| position as f64).collect();

    let mean = |values: &[f64]| values.iter().sum::<f64>() / values.len() as f64;
    let (position_mean, rank_mean) = (mean(&positions), mean(&ranks));
    let covariance: f64 = positions
        .iter()
        .zip(&ranks)
        .map(|(position, rank)| (position - position_mean) * (rank - rank_mean))
        .sum();
    let position_spread: f64 = positions
        .iter()
        .map(|position| (position - position_mean).powi(2))
        .sum();
    let rank_spread: f64 = ranks.iter().map(|rank| (rank - rank_mean).powi(2)).sum();

    // A series of one repeated value correlates with nothing. It is not thereby symmetric — the
    // monotonicity test below is what rejects it — but the correlation of a flat series is
    // undefined, and reporting zero rather than a division by zero keeps the two checks separate.
    if position_spread == 0.0 || rank_spread == 0.0 {
        return 0.0;
    }
    covariance / (position_spread * rank_spread).sqrt()
}

/// Whether every lower position in a series does at least as well as every higher one.
fn monotone_non_increasing(series: &[usize]) -> bool {
    series.windows(2).all(|pair| pair[0] >= pair[1])
}

/// `VER-MOK-012` oracle 5, the outcome half: the identifier-monotonicity band.
///
/// `INT-MOK-009` records the risk that deterministic resolution plus an ascending-identifier acting
/// order advantages `M01`. The contract bounds it from two directions and states that neither is
/// sufficient alone. The mechanism half is exact and lives in the internal tier, where one
/// constructed encounter is resolved with the two identifiers exchanged and shown to yield identical
/// damage. This is the other half, and the contract is explicit that it is weak: five seeds and
/// twelve identifiers give it little power, its purpose is to fail loudly on a gross advantage —
/// `M01` surviving every seed while `M12` survives none — and it will not detect a small one.
///
/// It is written as the contract states it, on all three series and with both conditions, and the
/// whole table is printed either way. A correlation is only interpretable beside the number of
/// events it was computed from, so the totals are printed with it.
#[test]
fn no_identifier_series_is_monotone_in_identifier_or_correlated_beyond_the_band() {
    /// `VER-MOK-012`, *The stated monotonicity band*, transcribed.
    const BAND: f64 = 0.5;

    let mut survivals = [0usize; 12];
    let mut applied = [0usize; 12];
    let mut suffered = [0usize; 12];

    for seed in DECLARED_SEEDS {
        let mut simulation = Simulation::new(Config {
            policy: Policy::Social,
            ..config_at(seed, 1_000, DECLARED_FLOORS[0].0)
        })
        .unwrap();
        let mut output = Vec::new();
        simulation.run(&mut output).unwrap();
        let output = String::from_utf8(output).unwrap();

        for (index, slot) in survivals.iter_mut().enumerate() {
            let identifier = format!("M{:02}", index + 1);
            if !output.contains(&format!("subject={identifier} event=agent_died")) {
                *slot += 1;
            }
            // A `fight` resolves as a strike and reports the same event, so these are counts of
            // strikes dealt and strikes taken, which is what the series are for.
            applied[index] += output
                .matches(&format!("subject={identifier} event=attack_resolved"))
                .count();
            suffered[index] += output
                .matches(&format!(
                    "event=attack_resolved result=target:{identifier},"
                ))
                .count();
        }
    }

    let series = [
        ("survivals", &survivals),
        ("attacks applied", &applied),
        ("attacks suffered", &suffered),
    ];
    let table = series
        .iter()
        .map(|(name, values)| {
            format!(
                "  {name:<17} {values:?} total {}, correlation {:+.3}",
                values.iter().sum::<usize>(),
                rank_correlation(values.as_slice())
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("per-identifier series, M01 to M12, over the declared seeds under `social`:\n{table}");

    for (name, values) in series {
        assert!(
            !monotone_non_increasing(values.as_slice()),
            "the {name} series never rises with identifier, which is the advantage this oracle \
             exists to catch\n{table}"
        );
        let correlation = rank_correlation(values.as_slice());
        assert!(
            correlation.abs() <= BAND,
            "the {name} series correlates with identifier at {correlation:+.3}, outside the stated \
             band of ±{BAND}\n{table}"
        );
    }
}
