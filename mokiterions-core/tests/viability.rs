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
        spend_ceiling: None,
        prices: None,
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

/// `REQ-MOK-058`: under the social source at the declared density over a thousand ticks, at least
/// five of the twelve are left living and at least one death is attributable to combat, on every
/// declared seed.
///
/// Two-sided on purpose, and the second side is the one that is easy to lose. A source that
/// proposed no lethal action would pass a survivor floor perfectly, and the floor alone cannot tell
/// a world where combat is survivable from one where it never happens. `REQ-MOK-058` states the
/// lower bound as a *combat* death so that the requirement fails in both directions.
///
/// The whole curve is measured before anything is asserted. `VER-MOK-016` requires that: the
/// per-seed count is retained even on a seed that fails, "because the curve is what the floor is
/// ratified on and a suite that stops at the first failure produces no curve". So the table is
/// built first, printed, and asserted at the end, and each failure message carries the entire
/// table — the floor's value is the product owner's to ratify or amend on the first measured
/// curve, and this is the measurement that decision is made on.
///
/// The cause of death is read off the resolution record rather than off `agent_died`.
/// `REQ-MOK-053` puts it there deliberately, so that combat deaths can be told apart from
/// starvation without adding a cause field to `agent_died`; every death not reported by a
/// resolution is therefore a death by starvation or exhaustion, by elimination.
#[test]
fn the_social_source_keeps_the_world_habitable_and_combat_lethal() {
    /// `REQ-MOK-058`, transcribed. Three below `REQ-MOK-014`'s eight, which is the cost the owner
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
    println!("REQ-MOK-058 curve, `social`, default density, 1,000 ticks:\n{table}");

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

/// The three per-identifier series `VER-MOK-016` oracle 5 computes, summed over `seeds`.
///
/// Survival is read as the absence of an `agent_died` record rather than off a summary count,
/// because the count says how many lived and this needs to know which. The strike counts come from
/// the same stream for the same reason. A `fight` resolves as a strike and reports the same event,
/// so `applied` and `suffered` are strikes dealt and strikes taken however they were proposed.
fn identifier_series(seeds: &[u64]) -> ([usize; 12], [usize; 12], [usize; 12]) {
    let mut survivals = [0usize; 12];
    let mut applied = [0usize; 12];
    let mut suffered = [0usize; 12];

    for &seed in seeds {
        let mut simulation = Simulation::new(Config {
            policy: Policy::Social,
            ..config_at(seed, 1_000, DECLARED_FLOORS[0].0)
        })
        .unwrap();
        let mut output = Vec::new();
        simulation.run(&mut output).unwrap();
        let output = String::from_utf8(output).unwrap();

        for index in 0..12 {
            let identifier = format!("M{:02}", index + 1);
            if !output.contains(&format!("subject={identifier} event=agent_died")) {
                survivals[index] += 1;
            }
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

    (survivals, applied, suffered)
}

/// A per-identifier series folded onto turn position within a Mokiterion's own territory.
///
/// `SPEC-MOK-001` places six Mokiterions in each territory by identifier — `M01` to `M06` in A,
/// `M07` to `M12` in B — and contact is overwhelmingly within a territory, so position `index % 6`
/// is the covariate turn order's effect actually runs along. Against identifier `1..=12` the same
/// effect is a sawtooth that resets at `M07`, which is why `VER-MOK-016` measures it here instead.
fn by_turn_position(series: &[usize; 12]) -> [usize; 6] {
    let mut pooled = [0usize; 6];
    for (index, value) in series.iter().enumerate() {
        pooled[index % 6] += value;
    }
    pooled
}

/// `VER-MOK-016` oracle 5, the outcome half, part one: the gross-advantage tripwire.
///
/// `INT-MOK-010` records the risk that deterministic resolution plus an ascending-identifier acting
/// order advantages `M01`. The contract bounds it from two directions and states that neither is
/// sufficient alone. The mechanism half is exact and lives in the internal tier, where one
/// constructed encounter is resolved with the two identifiers exchanged and shown to yield identical
/// damage. This is one half of the other direction, and the contract is explicit about what five
/// seeds support: this and no more. Its purpose is to fail loudly on a gross advantage — `M01`
/// surviving every seed while `M12` survives none — and it will not detect a small one. The small
/// one is `survival_by_turn_position_stays_inside_the_stated_bound`, on the set declared for it.
///
/// **Both rank correlations are printed and neither is asserted on.** The contract's amendment of
/// 2026-08-20 removed the `±0.5` band it used to assert here: a correlation over twelve points is
/// underpowered at five seeds and saturated at a thousand, so it was measured to be able to fail on
/// noise and pass on a real advantage. It stays in the output because the measurement is a
/// deliverable, and a correlation is only interpretable beside the number of events it came from.
#[test]
fn no_identifier_series_is_monotone_in_identifier() {
    let (survivals, applied, suffered) = identifier_series(&DECLARED_SEEDS);
    let series = [
        ("survivals", &survivals),
        ("attacks applied", &applied),
        ("attacks suffered", &suffered),
    ];
    let table = series
        .iter()
        .map(|(name, values)| {
            format!(
                "  {name:<17} {values:?} total {}, correlation vs identifier {:+.3}, \
                 vs turn position {:+.3}",
                values.iter().sum::<usize>(),
                rank_correlation(values.as_slice()),
                rank_correlation(&by_turn_position(values))
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
    }
}

/// `VER-MOK-016` oracle 5, the outcome half, part two: the turn-position survival bound.
///
/// Survival is what an advantage means, so survival is what is bounded. The measured advantage runs
/// to *later*-acting Mokiterions rather than to `M01`, which is the opposite of the direction
/// `INT-MOK-010` anticipated, and the ratio is taken over the extremes of all six positions rather
/// than of the last to the first so that it bounds any pair and cannot stop catching a reversal.
///
/// This runs its own declared seed set because the bound cannot be measured on five. Over five
/// disjoint groups of 200 the ratio holds between `1.010` and `1.137` and every group agrees on the
/// direction; at 100 and at 50 the groups straddle `1.0` and disagree about which way it runs. The
/// set carries no survivor floor and no lethality bound — `REQ-MOK-058`'s obligations stay on
/// `DECLARED_SEEDS`, and nothing here is comparable with `REQ-MOK-014`'s or `REQ-MOK-034`'s.
#[test]
fn survival_by_turn_position_stays_inside_the_stated_bound() {
    /// `VER-MOK-016`, *Part two: the turn-position survival bound*, transcribed.
    ///
    /// Not read off the curve it bounds. `REQ-MOK-034` binds the trait-aware source at eight of
    /// twelve and `REQ-MOK-058` binds this source at five, so three of twelve — a quarter of the
    /// population — is the survivor cost combat was accepted to impose. An advantage worth more
    /// than that whole quarter is structural rather than residual.
    const BOUND: f64 = 1.25;
    /// The declared diagnostic seed set: the 200 seeds `0` through `199` inclusive.
    const DIAGNOSTIC_SEEDS: std::ops::RangeInclusive<u64> = 0..=199;

    let seeds: Vec<u64> = DIAGNOSTIC_SEEDS.collect();
    let (survivals, applied, suffered) = identifier_series(&seeds);

    let pooled = by_turn_position(&survivals);
    // Each turn position holds two identifiers, one per territory, on every seed.
    let opportunities = (seeds.len() * 2) as f64;
    let rates: Vec<f64> = pooled
        .iter()
        .map(|count| *count as f64 / opportunities)
        .collect();

    let table = format!(
        "  survival rate by turn position in own territory, over {} seeds:\n    {}\n  \
         strikes applied {:?} correlation {:+.3}\n  strikes suffered {:?} correlation {:+.3}",
        seeds.len(),
        rates
            .iter()
            .enumerate()
            .map(|(position, rate)| format!("{}:{rate:.4}", position + 1))
            .collect::<Vec<String>>()
            .join("  "),
        by_turn_position(&applied),
        rank_correlation(&by_turn_position(&applied)),
        by_turn_position(&suffered),
        rank_correlation(&by_turn_position(&suffered)),
    );
    println!("{table}");

    let highest = rates.iter().copied().fold(f64::MIN, f64::max);
    let lowest = rates.iter().copied().fold(f64::MAX, f64::min);
    assert!(lowest > 0.0, "no turn position survived at all\n{table}");
    let ratio = highest / lowest;
    println!("  ratio highest to lowest {ratio:.4}, bound {BOUND}");

    assert!(
        ratio < BOUND,
        "survival by turn position spans a ratio of {ratio:.4}, outside the stated bound of \
         {BOUND}: the acting order hands an advantage larger than the survivor cost combat was \
         accepted to impose\n{table}"
    );
}
