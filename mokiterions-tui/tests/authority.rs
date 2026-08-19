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

use mokiterions::simulation::{Event, EventDetail};
use mokiterions::simulation::{EventType, Policy};
use mokiterions_tui::authority::*;

#[test]
fn every_event_type_the_observer_can_present_has_an_entry() {
    for policy in [Policy::Baseline, Policy::Reference, Policy::Individual] {
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
        (EventType::SimulationEnded, "REQ-MOK-011"),
        (EventType::ActionTrace, "REQ-MOK-012"),
    ];
    for (event_type, identifier) in expected {
        assert_eq!(for_type(event_type, None), Some(identifier), "{event_type}");
    }
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

    // A source the observer does not know is reported as missing, never guessed.
    assert_eq!(for_event(&source("something-else")), None);
    assert_eq!(for_type(EventType::DecisionSourceSelected, None), None);
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
