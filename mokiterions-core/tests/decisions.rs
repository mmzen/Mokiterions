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

/// The declared verification seed set, fixed by `VER-MOK-002` and reused unchanged so that these
/// counts are comparable at matched seeds with the survivor measurements.
const DECLARED_SEEDS: [u64; 5] = [0, 1, 42, 123, 777];

/// The seven targeted verbs of `REQ-MOK-043`, in the order `SPEC-MOK-001` states them.
const TARGETED_VERBS: [&str; 7] = [
    "attack",
    "threaten",
    "fight",
    "retreat",
    "surrender",
    "approach",
    "avoid",
];

/// One 1,000-tick traced run under the `social` source at the default density.
fn social_run(seed: u64) -> String {
    let mut simulation = Simulation::new(Config {
        seed,
        tick_limit: 1_000,
        policy: Policy::Social,
        density: Density::DEFAULT,
        trace_actions: true,
    })
    .unwrap();
    let mut output = Vec::new();
    simulation.run(&mut output).unwrap();
    String::from_utf8(output).unwrap()
}

/// The verb each `action_trace` line proposed, paired with whether it was applied.
fn proposals(output: &str) -> Vec<(&str, bool)> {
    output
        .lines()
        .filter(|line| line.contains("event=action_trace"))
        .map(|line| {
            let rest = line
                .split_once("result=proposal:")
                .expect("every trace line states its proposal")
                .1;
            let verb = rest
                .split_once(',')
                .expect("a proposal is followed by more")
                .0;
            (verb, line.contains(",status:accepted,"))
        })
        .collect()
}

/// `REQ-MOK-043` and `VER-MOK-012` oracle 4: every one of the seven targeted verbs applies at least
/// once somewhere in the declared matrix.
///
/// A verb that never applies in any run is an unreachable rule, and nothing in a constructed-state
/// test would say so — those place two Mokiterions in contact by hand and prove only the
/// arithmetic. This is the counterpart oracle, and the contract states that neither may stand for
/// the other.
///
/// The whole table is built before anything is asserted, so that a failure names every verb that
/// never applied rather than the first one, and reports the counts the product owner needs.
#[test]
fn every_targeted_verb_applies_somewhere_in_the_declared_matrix() {
    let mut applied = [0usize; 7];
    let mut proposed = [0usize; 7];
    let mut waits = 0;

    for seed in DECLARED_SEEDS {
        for (verb, accepted) in proposals(&social_run(seed)) {
            if verb == "wait" {
                waits += 1;
            }
            if let Some(index) = TARGETED_VERBS.iter().position(|named| *named == verb) {
                proposed[index] += 1;
                if accepted {
                    applied[index] += 1;
                }
            }
        }
    }

    let table = TARGETED_VERBS
        .iter()
        .enumerate()
        .map(|(index, verb)| {
            format!(
                "  {verb:<9} proposed {:>6}, applied {:>6}",
                proposed[index], applied[index]
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("targeted verbs over the declared seeds under `social`:\n{table}");

    // `REQ-MOK-048` branches 2 and 5 delegate to rule 19, which never waits, so neither does this
    // source.
    assert_eq!(waits, 0, "the social source proposed a wait");

    let unreached: Vec<&str> = TARGETED_VERBS
        .iter()
        .enumerate()
        .filter(|(index, _)| applied[*index] == 0)
        .map(|(_, verb)| *verb)
        .collect();
    assert!(
        unreached.is_empty(),
        "these verbs never applied in any declared run, so the rules that define them are \
         unreachable as the source is ordered: {unreached:?}\n{table}"
    );
}

/// `REQ-MOK-043` and `VER-MOK-012` oracle 4: the acting order is untouched.
///
/// One opportunity per living Mokiterion per tick, in ascending identifier order, over whole runs.
/// The order is what `INT-MOK-009` records as the source of combat's one asymmetry, so it is
/// asserted directly rather than inferred from the outcomes it produces: a Mokiterion acting twice
/// would strike twice in a tick, and one acting out of order would answer an attack before it
/// landed.
#[test]
fn the_acting_order_is_one_ascending_pass_per_tick_under_the_social_source() {
    let output = social_run(42);
    let mut dead: Vec<String> = Vec::new();
    let mut ticks = 0;
    let mut current = 0u64;
    let mut acted: Vec<String> = Vec::new();
    // The population entitled to an opportunity in the tick being read, snapshotted at the tick
    // boundary. A Mokiterion that dies *during* a tick has already acted in it: rule 13 runs its
    // survival step after its own action, so its death record follows its own trace line inside the
    // same tick and takes nothing away from that tick's width. It opens at zero because tick 0 is
    // the initialization pass, which reports twelve Mokiterions and grants no opportunities.
    let mut entitled = 0;

    let close = |tick: u64, acted: &[String], entitled: usize| {
        let mut ascending = acted.to_vec();
        ascending.sort();
        assert_eq!(
            acted, ascending,
            "tick {tick} acted out of identifier order"
        );
        ascending.dedup();
        assert_eq!(
            acted.len(),
            ascending.len(),
            "tick {tick} gave a Mokiterion two opportunities"
        );
        assert_eq!(
            acted.len(),
            entitled,
            "tick {tick} gave {} opportunities to {entitled} living Mokiterions: {acted:?}",
            acted.len()
        );
    };

    for line in output.lines() {
        let Some((prefix, _)) = line.split_once(" event=") else {
            continue;
        };
        let tick: u64 = prefix
            .trim_start_matches("tick=")
            .split_once(" subject=")
            .expect("every record states a tick and a subject")
            .0
            .parse()
            .expect("the tick is an integer");
        let subject = prefix.rsplit_once("subject=").unwrap().1.to_string();

        if tick != current {
            close(current, &acted, entitled);
            acted.clear();
            entitled = 12 - dead.len();
            current = tick;
            ticks += 1;
        }
        if line.contains("event=action_trace") {
            // Checked here rather than at the tick boundary, because the claim is order-sensitive:
            // a Mokiterion killed by a striker earlier in the pass must take no opportunity later in
            // the same tick, and a set compared at the tick's end cannot tell that case from a death
            // that followed its holder's own action.
            assert!(
                !dead.contains(&subject),
                "tick {tick} gave the dead {subject} an opportunity"
            );
            acted.push(subject);
        } else if line.contains("event=agent_died") {
            dead.push(subject);
        }
    }
    close(current, &acted, entitled);

    assert!(ticks > 900, "only {ticks} ticks were observed");
    // The seed is one where the population does fall, so the width assertion above is exercised at
    // more than one value rather than being vacuously twelve throughout.
    assert!(
        !dead.is_empty(),
        "seed 42 lost nobody, so the narrowing of the pass went untested"
    );
}
