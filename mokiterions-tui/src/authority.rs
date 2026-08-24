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
            Policy::Social => "REQ-MOK-057",
            // `SPEC-MOK-007` rule 18.5 names this row. It is `REQ-MOK-063`, the requirement for
            // the source itself, and not `REQ-MOK-067`'s replay determinism: the row answers
            // "what authorizes this source to exist", and the observer reaches this source in
            // replay only, so mapping it to the replay requirement would name the observer's
            // restriction rather than the record's authority.
            Policy::Llm => "REQ-MOK-063",
        },
        EventType::SurvivalChanged | EventType::AgentDied => "REQ-MOK-003",
        EventType::FoodConsumed => "REQ-MOK-006",
        EventType::FoodRegenerated | EventType::FoodRegenerationSkipped => "REQ-MOK-007",
        EventType::TerritoryCrossed => "REQ-MOK-005",
        // Rule 11's three added rows. `attack_resolved` maps to `REQ-MOK-053` for `attack` and
        // for `fight` alike, because both invoke one resolution and the record does not say
        // which verb produced it. `REQ-MOK-052` takes no row: it authorizes seven verbs while
        // adding no event type, and rule 11 clause 2's exhaustiveness runs from the event side.
        EventType::AttackResolved => "REQ-MOK-053",
        EventType::ThreatResolved => "REQ-MOK-055",
        EventType::SurrenderResolved => "REQ-MOK-056",
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
///
/// The source-dependent row is **derived from [`for_type`]** rather than written out. Until
/// `SPEC-MOK-007` rule 18.5 it was a hand-written string naming four sources and repeating their
/// four identifiers, and a hand-written copy of a mapping is what goes stale the moment the mapping
/// grows: rule 11's clause 2 asks for exhaustiveness, and a duplicate is exhaustive only for as long
/// as someone remembers it exists. Derived, this row and [`for_event`] cannot disagree, and rule
/// 18.5's "hard-coded four-source description" has no second copy left to correct.
///
/// One thing the derivation does not buy, stated rather than left to be discovered: `SOURCES` below
/// is still a list, so a sixth decision source has to be added to it. What has changed is that
/// forgetting is no longer silent — [`for_type`]'s `match` over `Policy` is exhaustive, so a sixth
/// variant stops the build in the one function whose comment points here.
///
/// The row's identifier carries **one line per source**, joined by newlines. Five sources on one
/// line is 135 columns, and the overlay renders a paragraph that truncates rather than wraps, so on
/// a 132-column terminal the row would show four sources and say nothing about the fifth — the
/// plausible-but-incomplete presentation rule 11.2 exists to forbid. The row *count* is unchanged at
/// one per event type; only its height is.
pub fn table(policy: Policy) -> Vec<(&'static str, String)> {
    /// Every value `Policy` admits, in `SPEC-MOK-007` rule 18.1's order: the four existing values
    /// unchanged, with the fifth after them.
    ///
    /// Local, and deliberately. `SPEC-MOK-004` rule 6 counts one item per `pub` declaration and its
    /// **Growth** clause admits a new one only when an approved requirement needs it; nothing needs
    /// this outside this function, and the engine's `Policy` carries no `ALL` of its own that this
    /// could read instead — `WO-MOK-025` fixes the engine's public surface and adding one there is
    /// not in it.
    const SOURCES: [Policy; 5] = [
        Policy::Baseline,
        Policy::Reference,
        Policy::Individual,
        Policy::Social,
        Policy::Llm,
    ];

    EventType::ALL
        .iter()
        .map(|event_type| {
            let identifier = match event_type {
                EventType::DecisionSourceSelected => SOURCES
                    .iter()
                    .map(|source| match for_type(*event_type, Some(*source)) {
                        Some(identifier) => format!("{identifier} {source}"),
                        // Unreachable while `for_type`'s arm is exhaustive over `Policy`, and
                        // written anyway on rule 11.2's terms: the same words the other rows use
                        // for a gap, never a plausible identifier.
                        None => format!("mapping missing {source}"),
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
                other => for_type(*other, Some(policy))
                    .map(str::to_string)
                    .unwrap_or_else(|| "mapping missing".to_string()),
            };
            (event_type.as_str(), identifier)
        })
        .collect()
}
