//! `SPEC-MOK-003` rule 11's authority mapping.
//!
//! The mapping names identifiers only. It never restates requirement text, which could drift
//! from the artifact that holds it. Where an identifier cannot be resolved the caller states
//! that the mapping is missing rather than presenting a plausible one.

use mokiterions::simulation::{Event, EventDetail, EventType, Policy};

/// The requirement authorizing the inspector's proposal-and-outcome presentation.
pub const DECISION_AUTHORITY: &str = "REQ-MOK-004";

/// The requirement authorizing perceived-entity information.
pub const PERCEPTION_AUTHORITY: &str = "REQ-MOK-013";

/// The authorizing requirement for an event type.
///
/// `decision_source_selected` resolves against the source named in the record, which is why
/// the source is a parameter rather than an assumption.
pub fn for_type(event_type: EventType, source: Option<Policy>) -> Option<&'static str> {
    Some(match event_type {
        EventType::WorldInitialized | EventType::FoodInitialized => "REQ-MOK-001",
        EventType::AgentInitialized => "REQ-MOK-002",
        EventType::DecisionSourceSelected => match source? {
            Policy::Baseline => "REQ-MOK-008",
            Policy::Reference => "REQ-MOK-015",
        },
        EventType::SurvivalChanged | EventType::AgentDied => "REQ-MOK-003",
        EventType::FoodConsumed => "REQ-MOK-006",
        EventType::FoodRegenerated | EventType::FoodRegenerationSkipped => "REQ-MOK-007",
        EventType::TerritoryCrossed => "REQ-MOK-005",
        EventType::SimulationEnded => "REQ-MOK-011",
        EventType::ActionTrace => "REQ-MOK-012",
    })
}

/// The authorizing requirement for one record, resolved from the record's own payload.
///
/// A `decision_source_selected` record naming a source the observer does not know yields
/// `None`, so rule 11.2's "state that the mapping is missing" is a reachable path rather than
/// an unreachable branch.
pub fn for_event(event: &Event) -> Option<&'static str> {
    let source = match &event.detail {
        EventDetail::DecisionSourceSelected { source } => Some(Policy::parse(source)?),
        _ => None,
    };
    for_type(event.event_type(), source)
}

/// The mapping as rule 11's table presents it, for the authority overlay.
pub fn table(policy: Policy) -> Vec<(&'static str, String)> {
    EventType::ALL
        .iter()
        .map(|event_type| {
            let identifier = match event_type {
                EventType::DecisionSourceSelected => {
                    "REQ-MOK-008 baseline / REQ-MOK-015 reference".to_string()
                }
                other => for_type(*other, Some(policy))
                    .map(str::to_string)
                    .unwrap_or_else(|| "mapping missing".to_string()),
            };
            (event_type.as_str(), identifier)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mokiterions::simulation::{Event, EventDetail};

    #[test]
    fn every_event_type_the_observer_can_present_has_an_entry() {
        for event_type in EventType::ALL {
            let resolved = for_type(event_type, Some(Policy::Reference));
            assert!(resolved.is_some(), "{event_type} has no authority");
            assert!(
                resolved.unwrap().starts_with("REQ-MOK-"),
                "{event_type} maps to {resolved:?}"
            );
        }
        assert_eq!(table(Policy::Reference).len(), EventType::ALL.len());
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
}
