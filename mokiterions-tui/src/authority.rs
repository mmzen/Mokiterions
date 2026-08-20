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
            Policy::Individual => "REQ-MOK-033",
            Policy::Social => "REQ-MOK-048",
        },
        EventType::SurvivalChanged | EventType::AgentDied => "REQ-MOK-003",
        EventType::FoodConsumed => "REQ-MOK-006",
        EventType::FoodRegenerated | EventType::FoodRegenerationSkipped => "REQ-MOK-007",
        EventType::TerritoryCrossed => "REQ-MOK-005",
        // Rule 11's three added rows. `attack_resolved` maps to `REQ-MOK-044` for `attack` and
        // for `fight` alike, because both invoke one resolution and the record does not say
        // which verb produced it. `REQ-MOK-043` takes no row: it authorizes seven verbs while
        // adding no event type, and rule 11 clause 2's exhaustiveness runs from the event side.
        EventType::AttackResolved => "REQ-MOK-044",
        EventType::ThreatResolved => "REQ-MOK-046",
        EventType::SurrenderResolved => "REQ-MOK-047",
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
                    "REQ-MOK-008 baseline / REQ-MOK-015 reference / REQ-MOK-033 individual \
                     / REQ-MOK-048 social"
                        .to_string()
                }
                other => for_type(*other, Some(policy))
                    .map(str::to_string)
                    .unwrap_or_else(|| "mapping missing".to_string()),
            };
            (event_type.as_str(), identifier)
        })
        .collect()
}
