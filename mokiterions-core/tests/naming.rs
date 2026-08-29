//! Public tier, `SPEC-MOK-002` rule 8: the names the engine reports, and where in the record it
//! reports them.
//!
//! A further public subject appeared, which rule 8's closing sentence admits as a further file.
//! Everything here is writable through rule 5's interface — `Simulation::new`, `Simulation::run`
//! and the text stream those produce — so rule 7's placement rule puts it here rather than inline.
//! Nothing is widened for it: the name table and the assignment function stay private, and the
//! expectation below is `SPEC-MOK-001`'s *Name* written out as a literal.
//!
//! Reproducing the twelve names here is deliberate and is not duplication of the implementation. A
//! test that re-derived them from the engine would follow the engine wherever it went, which is the
//! reasoning `VER-MOK-007` recorded for the trait expectations and which holds identically for a
//! value the specification fixes outright.

use mokiterions::simulation::{Config, Density, Policy, Simulation};

/// The verification seed set `VER-MOK-002` declares, reused by `VER-MOK-011`.
const DECLARED_SEEDS: [u64; 5] = [0, 1, 42, 123, 777];

/// `SPEC-MOK-001`'s *Name*: the name of `M01` through `M12`, in identifier order.
const SPECIFIED_NAMES: [&str; 12] = [
    "Zug", "Krul", "Quib", "Sput", "Trok", "Womp", "Hozz", "Nurb", "Vonk", "Gorm", "Xob", "Drix",
];

fn config_at(seed: u64, tick_limit: u64, policy: Policy, trace_actions: bool) -> Config {
    Config {
        seed,
        tick_limit,
        policy,
        density: Density::DEFAULT,
        trace_actions,
        spend_ceiling: None,
        prices: None,
    }
}

fn run(config: Config) -> String {
    let mut simulation = Simulation::new(config).expect("the declared configuration is valid");
    let mut output = Vec::new();
    simulation.run(&mut output).expect("the run completes");
    String::from_utf8(output).expect("the stream is UTF-8")
}

/// The `name` every `agent_initialized` record reports, in the order reported.
///
/// Read from the front of the detail list, which is where `SPEC-MOK-001`'s *Data and interface
/// contracts* fixes it, so this helper fails if the field moves rather than finding it elsewhere.
fn reported_names(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|line| line.contains("event=agent_initialized"))
        .map(|line| {
            line.split_once("result=name:")
                .expect("every initialization record reports the name first")
                .1
                .split_once(',')
                .expect("a field is followed by the next")
                .0
                .to_string()
        })
        .collect()
}

/// `REQ-MOK-040`: the twelve names the specification fixes, assigned to the twelve identifiers,
/// identically at every seed, density, decision source and trace setting.
#[test]
fn every_run_reports_the_specified_twelve_names_in_identifier_order() {
    for seed in DECLARED_SEEDS {
        for policy in [
            Policy::Baseline,
            Policy::Reference,
            Policy::Individual,
            Policy::Social,
        ] {
            for density in ["0.15", "0.75", "1.50"] {
                for trace_actions in [false, true] {
                    let output = run(Config {
                        density: Density::parse(density).unwrap(),
                        ..config_at(seed, 1, policy, trace_actions)
                    });

                    assert_eq!(
                        reported_names(&output),
                        SPECIFIED_NAMES.to_vec(),
                        "seed {seed} at density {density}% did not report the specified names"
                    );

                    // Each name is on its own subject's record, so no name is presented against
                    // another Mokiterion's values.
                    for (index, name) in SPECIFIED_NAMES.iter().enumerate() {
                        let expected = format!(
                            "subject=M{:02} event=agent_initialized result=name:{name},position:",
                            index + 1
                        );
                        assert_eq!(
                            output.matches(expected.as_str()).count(),
                            1,
                            "seed {seed} at density {density}% did not report {name} once against \
                             M{:02}, first in the record",
                            index + 1
                        );
                    }
                }
            }
        }
    }
}

/// `REQ-MOK-040`: the name is reported once per Mokiterion and on no other record kind.
///
/// The count over a whole run is what makes "and nowhere else" checkable — a name leaking onto a
/// second record kind raises it above twelve. The run is long enough and the trace is on, so every
/// record kind the rules emit appears in the text being counted.
#[test]
fn a_name_is_reported_once_and_on_no_other_record() {
    for seed in DECLARED_SEEDS {
        let output = run(config_at(seed, 200, Policy::Individual, true));

        assert_eq!(
            output.matches("result=name:").count(),
            12,
            "seed {seed} does not report exactly twelve names"
        );
        assert_eq!(
            output.matches("name:").count(),
            12,
            "seed {seed} writes `name:` somewhere other than the twelve initialization records"
        );
        // Records other than `agent_initialized` are unchanged, so a subject's later records carry
        // its identifier and no name.
        for line in output
            .lines()
            .filter(|line| !line.contains("event=agent_initialized"))
        {
            assert!(!line.contains("name:"), "{line}");
        }
    }
}

/// `REQ-MOK-040` and `SPEC-MOK-001`'s *Data and interface contracts*: the name is the first detail
/// and `waste_tolerance` is still the last, because two suites parse this record by position.
///
/// `tests/process.rs` counts `,fear:0,waste_tolerance:` from the front of the attribute list and
/// this file's sibling `tests/viability.rs` reads the trait with `rsplit_once`. Both survive a field
/// added at the head, and that is a property of the record's shape rather than of those files, so it
/// is asserted here where the shape is the subject.
#[test]
fn the_initialization_record_keeps_the_name_first_and_the_trait_last() {
    let output = run(config_at(42, 1, Policy::Individual, false));
    let records: Vec<&str> = output
        .lines()
        .filter(|line| line.contains("event=agent_initialized"))
        .collect();
    assert_eq!(records.len(), 12);

    for record in records {
        let details = record
            .split_once("result=")
            .expect("every record has a detail list")
            .1;
        assert!(details.starts_with("name:"), "{record}");

        let fields: Vec<&str> = details.split(',').collect();
        assert_eq!(fields.len(), 8, "{record}");
        let keys: Vec<&str> = fields
            .iter()
            .map(|field| field.split_once(':').expect("a field is a pair").0)
            .collect();
        assert_eq!(
            keys,
            vec![
                "name",
                "position",
                "territory",
                "health",
                "satiety",
                "energy",
                "fear",
                "waste_tolerance"
            ],
            "{record}"
        );

        // The two positional reads the suites perform, verbatim in form.
        assert_eq!(record.matches(",fear:0,waste_tolerance:").count(), 1);
        assert!(
            record
                .rsplit_once(",waste_tolerance:")
                .expect("the trait is last")
                .1
                .parse::<u8>()
                .is_ok(),
            "{record}"
        );
    }
}
