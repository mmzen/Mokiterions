//! Public tier: authority.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Every one of them was in
//! `src/authority.rs`'s `#[cfg(test)] mod tests` block and reached the code through items that were
//! already public, so the move changes the path and nothing else: the assertions are verbatim and
//! no item was widened to bring them out. `SPEC-MOK-004` rule 12 is the obligation and the
//! per-test comparison under `WO-MOK-006` is the evidence.
//!
//! `WO-MOK-010` added a third decision source, and with it a third arm to the mapping these tests
//! cover. Two cases below gained assertions for it: the exhaustiveness sweep now runs under each
//! of the three policies, and the source-dependent mapping names all three. Nothing was removed
//! or weakened — an untested arm in an exhaustiveness check is the one thing this file exists to
//! prevent.
//!
//! `WO-MOK-016` added a fourth source and three event types, on the same terms: the sweep runs
//! under four policies, the source-dependent mapping names four, and the specified-mapping case
//! carries the three added rows. Again nothing was removed or weakened.
//!
//! `WO-MOK-025` added a fifth source and no event type, on the same terms once more: the sweep runs
//! under five policies and the source-dependent mapping names five. One case arrives rather than
//! being folded into an existing one — `SPEC-MOK-007` rule 18.5 obliges two things, an entry for the
//! fifth source *and* the correction of a hard-coded four-source description, and the second is a
//! property of `table`'s row rather than of `for_type`'s answer. A repository that satisfied the
//! first and not the second would pass every case above.

use mokiterions::simulation::{Event, EventDetail};
use mokiterions::simulation::{EventType, Policy};
use mokiterions_tui::authority::*;

#[test]
fn every_event_type_the_observer_can_present_has_an_entry() {
    for policy in [
        Policy::Baseline,
        Policy::Reference,
        Policy::Individual,
        Policy::Social,
        Policy::Llm,
    ] {
        for event_type in EventType::ALL {
            let resolved = for_type(event_type, Some(policy));
            assert!(resolved.is_some(), "{event_type} has no authority");
            assert!(
                resolved.unwrap().starts_with("REQ-MOK-"),
                "{event_type} maps to {resolved:?}"
            );
        }
        assert_eq!(table(policy).len(), EventType::ALL.len());
    }
}

#[test]
fn the_mapping_is_the_specified_one() {
    let expected = [
        (EventType::WorldInitialized, "REQ-MOK-001"),
        (EventType::FoodInitialized, "REQ-MOK-001"),
        (EventType::AgentInitialized, "REQ-MOK-002"),
        (EventType::SurvivalChanged, "REQ-MOK-003"),
        (EventType::AgentDied, "REQ-MOK-003"),
        (EventType::FoodConsumed, "REQ-MOK-006"),
        (EventType::FoodRegenerated, "REQ-MOK-007"),
        (EventType::FoodRegenerationSkipped, "REQ-MOK-007"),
        (EventType::TerritoryCrossed, "REQ-MOK-005"),
        // Rule 11's three added rows. `attack_resolved` carries one identifier for `attack` and
        // for `fight` alike, because they are one resolution, and `REQ-MOK-052` has no row of its
        // own because it adds no event type.
        (EventType::AttackResolved, "REQ-MOK-053"),
        (EventType::ThreatResolved, "REQ-MOK-055"),
        (EventType::SurrenderResolved, "REQ-MOK-056"),
        (EventType::SimulationEnded, "REQ-MOK-011"),
        (EventType::ActionTrace, "REQ-MOK-012"),
    ];
    for (event_type, identifier) in expected {
        assert_eq!(for_type(event_type, None), Some(identifier), "{event_type}");
    }

    // The table above is the whole of rule 11 minus its one source-dependent row, so its length
    // plus that row is the vocabulary. A fourth added type would fail here rather than pass
    // untested.
    assert_eq!(expected.len() + 1, EventType::ALL.len());
}

#[test]
fn the_decision_source_maps_by_the_source_the_record_names() {
    let source = |name: &str| Event {
        tick: 0,
        subject: "world".to_string(),
        detail: EventDetail::DecisionSourceSelected {
            source: name.to_string(),
        },
    };

    assert_eq!(for_event(&source("baseline")), Some("REQ-MOK-008"));
    assert_eq!(for_event(&source("reference")), Some("REQ-MOK-015"));
    assert_eq!(for_event(&source("individual")), Some("REQ-MOK-033"));
    assert_eq!(for_event(&source("social")), Some("REQ-MOK-057"));
    // `SPEC-MOK-007` rule 18.5's identifier. `REQ-MOK-063` authorizes the source itself, which is
    // what this row answers; `REQ-MOK-067`'s replay determinism would name the observer's own
    // restriction instead of the record's authority.
    assert_eq!(for_event(&source("llm")), Some("REQ-MOK-063"));

    // A source the observer does not know is reported as missing, never guessed.
    assert_eq!(for_event(&source("something-else")), None);
    assert_eq!(for_type(EventType::DecisionSourceSelected, None), None);
}

/// `SPEC-MOK-007` rule 18.5's second obligation: the overlay's row names every source, and names it
/// with the identifier `for_type` gives.
///
/// The row is compared against `for_type` rather than against a written-out expectation, because a
/// written-out expectation is exactly the hard-coded description this rule corrects — a second copy
/// of the mapping, in a file that has no way to know when the first one moved.
#[test]
fn the_source_row_names_every_source_and_agrees_with_the_mapping() {
    let row = table(Policy::Baseline)
        .into_iter()
        .find(|(event_type, _)| *event_type == EventType::DecisionSourceSelected.as_str())
        .expect("rule 11.2: every event type has a row")
        .1;

    let sources = [
        ("baseline", Policy::Baseline),
        ("reference", Policy::Reference),
        ("individual", Policy::Individual),
        ("social", Policy::Social),
        ("llm", Policy::Llm),
    ];
    let lines: Vec<&str> = row.lines().collect();
    assert_eq!(lines.len(), sources.len(), "{row}");
    for (line, (name, policy)) in lines.iter().zip(sources) {
        let identifier =
            for_type(EventType::DecisionSourceSelected, Some(policy)).expect("a known source");
        assert_eq!(*line, format!("{identifier} {name}"), "{row}");
    }

    // The row does not vary with the run's own source: rule 11's mapping is static and presented in
    // full, so an operator watching a `social` replay can still see what authorizes the others.
    for policy in [Policy::Reference, Policy::Social, Policy::Llm] {
        let other = table(policy)
            .into_iter()
            .find(|(event_type, _)| *event_type == EventType::DecisionSourceSelected.as_str())
            .expect("rule 11.2: every event type has a row")
            .1;
        assert_eq!(other, row);
    }

    // Rule 11.2's exhaustiveness is a count of event types, and splitting one row across five lines
    // must not change it. This is the half of the rule the multi-line form could have broken.
    assert_eq!(table(Policy::Llm).len(), EventType::ALL.len());
}

#[test]
fn an_ordinary_record_resolves_from_its_own_payload() {
    let event = Event {
        tick: 7,
        subject: "M03".to_string(),
        detail: EventDetail::AgentDied { health: 0 },
    };
    assert_eq!(for_event(&event), Some("REQ-MOK-003"));
}
