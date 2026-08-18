//! Public tier: state.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Each was in `src/state.rs`'s
//! `#[cfg(test)] mod tests` block and reaches the code through items that were already public, so
//! the move changes the path and nothing else: the assertions are verbatim and no item was widened
//! to bring them out. The tests rule 10 keeps inline are the ones that name a private item of the
//! module or one of the `#[cfg(test)]` hooks, and they stay in `src/state.rs`.

use mokiterions::simulation::{Event, EventDetail, EventType, TerminationReason};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

// `use super::*` used to supply these. A test tier outside the crate gets none of the module's
// private imports, so every name a moved test uses is named here through the public interface.
use mokiterions_tui::options::{self, Startup};
use mokiterions_tui::spatial::{WORLD_SIZE, Zoom};
use mokiterions_tui::state::*;

fn start(args: &[&str]) -> Observer {
    match options::parse(args.to_vec()).unwrap() {
        Startup::Run(options) => Observer::new(options).unwrap(),
        Startup::Help => panic!("expected a run"),
    }
}

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn send(observer: &mut Observer, code: KeyCode) -> KeyResponse {
    observer.handle_key(press(code)).unwrap()
}

/// The world extent this crate consumes must be the extent the engine reports.
#[test]
fn the_world_extent_matches_the_engine() {
    let observer = start(&[]);
    let world = observer
        .events()
        .iter()
        .find(|event| matches!(event.detail, EventDetail::WorldInitialized { .. }))
        .expect("the engine emits world_initialized first");
    match world.detail {
        EventDetail::WorldInitialized { width, height, .. } => {
            assert_eq!(u16::from(width), WORLD_SIZE);
            assert_eq!(u16::from(height), WORLD_SIZE);
        }
        _ => unreachable!(),
    }
}

#[test]
fn initial_state_is_the_specified_one() {
    let observer = start(&[]);
    assert_eq!(observer.progression(), Progression::Running);
    assert_eq!(observer.speed(), 8);
    assert_eq!(observer.selection(), None);
    assert!(!observer.follow());
    assert_eq!(observer.zoom(), Zoom::Overview);
    assert_eq!(observer.camera(), (0, 0));
    assert_eq!(*observer.filter(), Filter::None);
    assert_eq!(observer.overlay(), Overlay::None);
    assert!(!observer.events().truncated());
    assert_eq!(observer.snapshot().tick, 0);

    assert_eq!(start(&["--start-paused"]).progression(), Progression::Held);
    assert_eq!(start(&["--speed", "32"]).speed(), 32);
}

/// The tick-0 records are the engine's own, not the observer's reconstruction.
#[test]
fn initialization_events_are_retained_in_authoritative_order() {
    let observer = start(&[]);
    let events = observer.presented();

    assert_eq!(events[0].event_type(), EventType::WorldInitialized);
    assert_eq!(
        events.last().unwrap().event_type(),
        EventType::DecisionSourceSelected
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type() == EventType::AgentInitialized)
            .count(),
        12
    );
    assert!(events.iter().all(|event| event.tick == 0));
}

#[test]
fn a_single_step_is_accepted_only_while_held_and_advances_exactly_one_tick() {
    let mut observer = start(&["--start-paused"]);
    assert_eq!(observer.snapshot().tick, 0);

    let response = send(&mut observer, KeyCode::Char('.'));
    assert!(response.force_draw, "stepping must never be invisible");
    assert_eq!(observer.snapshot().tick, 1);

    // Released, the step control is refused.
    send(&mut observer, KeyCode::Char(' '));
    assert_eq!(observer.progression(), Progression::Running);
    send(&mut observer, KeyCode::Char('.'));
    assert_eq!(observer.snapshot().tick, 1);

    // Holding again restores it, and it stays held.
    send(&mut observer, KeyCode::Char(' '));
    send(&mut observer, KeyCode::Char('.'));
    assert_eq!(observer.snapshot().tick, 2);
    assert_eq!(observer.progression(), Progression::Held);
}

#[test]
fn a_finished_run_refuses_to_advance_and_stays_inspectable() {
    let mut observer = start(&["--ticks", "3", "--start-paused"]);
    for _ in 0..3 {
        observer.advance().unwrap();
    }
    assert!(observer.is_finished());
    assert_eq!(
        observer.termination_reason(),
        Some(TerminationReason::TickLimit)
    );

    let tick = observer.snapshot().tick;
    let retained = observer.events().len();
    observer.advance().unwrap();
    send(&mut observer, KeyCode::Char('.'));

    assert_eq!(observer.snapshot().tick, tick);
    assert_eq!(observer.events().len(), retained);
    assert_eq!(observer.snapshot().agents.len(), 12);
}

#[test]
fn speed_steps_through_the_fixed_ladder_and_clamps() {
    let mut observer = start(&["--speed", "1"]);
    for expected in [2, 4, 8, 16, 32, 64, 64] {
        send(&mut observer, KeyCode::Char('+'));
        assert_eq!(observer.speed(), expected);
    }
    for expected in [32, 16, 8, 4, 2, 1, 1] {
        send(&mut observer, KeyCode::Char('-'));
        assert_eq!(observer.speed(), expected);
    }
}

#[test]
fn selection_cycles_in_roster_order_and_escape_clears_it() {
    let mut observer = start(&[]);
    send(&mut observer, KeyCode::Tab);
    assert_eq!(observer.selection(), Some("M01"));
    send(&mut observer, KeyCode::Tab);
    assert_eq!(observer.selection(), Some("M02"));
    send(&mut observer, KeyCode::BackTab);
    assert_eq!(observer.selection(), Some("M01"));
    send(&mut observer, KeyCode::BackTab);
    assert_eq!(observer.selection(), Some("M12"), "the cycle wraps");

    send(&mut observer, KeyCode::Esc);
    assert_eq!(observer.selection(), None);
}

#[test]
fn escape_closes_an_overlay_before_it_clears_a_selection() {
    let mut observer = start(&[]);
    send(&mut observer, KeyCode::Tab);
    send(&mut observer, KeyCode::Char('?'));
    assert_eq!(observer.overlay(), Overlay::Help);

    send(&mut observer, KeyCode::Esc);
    assert_eq!(observer.overlay(), Overlay::None);
    assert_eq!(observer.selection(), Some("M01"));

    send(&mut observer, KeyCode::Esc);
    assert_eq!(observer.selection(), None);
}

#[test]
fn every_overlay_has_its_bound_key() {
    let mut observer = start(&[]);
    for (code, overlay) in [
        (KeyCode::Char('r'), Overlay::Roster),
        (KeyCode::Char('L'), Overlay::Log),
        (KeyCode::Char('i'), Overlay::Inspector),
        (KeyCode::Char('?'), Overlay::Help),
        (KeyCode::Char('t'), Overlay::Authority),
    ] {
        send(&mut observer, code);
        assert_eq!(observer.overlay(), overlay);
    }
}

#[test]
fn panning_moves_one_world_cell_and_clamps_at_every_edge() {
    let mut observer = start(&[]);
    observer.record_geometry((40, 20), 6);
    send(&mut observer, KeyCode::Char('z')); // detail zoom: a 40 x 20 region
    assert_eq!(observer.zoom(), Zoom::Detail);

    send(&mut observer, KeyCode::Char('l'));
    assert_eq!(observer.camera(), (1, 0));
    send(&mut observer, KeyCode::Char('j'));
    assert_eq!(observer.camera(), (1, 1));
    send(&mut observer, KeyCode::Char('h'));
    send(&mut observer, KeyCode::Char('k'));
    assert_eq!(observer.camera(), (0, 0));

    // Already at the north-west corner, panning further changes nothing.
    send(&mut observer, KeyCode::Char('h'));
    send(&mut observer, KeyCode::Char('k'));
    assert_eq!(observer.camera(), (0, 0));

    // Paging moves one visible region and stops at the world's edge.
    send(&mut observer, KeyCode::PageDown);
    assert_eq!(observer.camera(), (0, 20));
    for _ in 0..20 {
        send(&mut observer, KeyCode::PageDown);
    }
    assert_eq!(observer.camera(), (0, WORLD_SIZE - 20));
}

#[test]
fn a_whole_world_overview_cannot_be_panned_off_the_world() {
    let mut observer = start(&[]);
    observer.record_geometry((67, 32), 8);
    for code in [KeyCode::Char('l'), KeyCode::Char('j'), KeyCode::PageDown] {
        send(&mut observer, code);
        assert_eq!(observer.camera(), (0, 0));
    }
}

#[test]
fn following_centres_the_selection_and_clamps_identically() {
    let mut observer = start(&[]);
    observer.record_geometry((40, 20), 6);
    send(&mut observer, KeyCode::Char('z'));

    // Following is ignored while nothing is selected.
    send(&mut observer, KeyCode::Char('f'));
    assert!(observer.follow());
    observer.apply_follow((40, 20));
    assert_eq!(observer.camera(), (0, 0));

    send(&mut observer, KeyCode::Tab);
    let position = observer.selected_agent().unwrap().position;
    observer.apply_follow((40, 20));
    let camera = observer.camera();
    let viewport = observer.viewport();
    assert!(
        viewport.contains(position.x.into(), position.y.into()),
        "the followed Mokiterion must be visible, camera {camera:?}"
    );
    assert!(camera.0 <= 88 && camera.1 <= 108, "clamped: {camera:?}");
}

#[test]
fn the_type_filter_cycles_the_whole_vocabulary_then_returns_to_none() {
    let mut observer = start(&[]);
    for expected in EventType::ALL {
        send(&mut observer, KeyCode::Char('e'));
        assert_eq!(*observer.filter(), Filter::Type(expected));
    }
    send(&mut observer, KeyCode::Char('e'));
    assert_eq!(*observer.filter(), Filter::None);
}

#[test]
fn filtering_changes_presentation_only() {
    let mut observer = start(&["--start-paused"]);
    observer.advance().unwrap();
    let retained = observer.events().len();

    send(&mut observer, KeyCode::Tab);
    send(&mut observer, KeyCode::Char('u'));
    assert_eq!(*observer.filter(), Filter::Subject("M01".to_string()));
    let presented = observer.presented();
    assert!(!presented.is_empty());
    assert!(presented.iter().all(|event| event.subject == "M01"));
    assert!(presented.len() < retained);
    assert_eq!(observer.events().len(), retained);

    send(&mut observer, KeyCode::Char('c'));
    assert_eq!(*observer.filter(), Filter::None);
    assert_eq!(observer.presented().len(), retained);
}

#[test]
fn a_subject_filter_needs_a_selection() {
    let mut observer = start(&[]);
    send(&mut observer, KeyCode::Char('u'));
    assert_eq!(*observer.filter(), Filter::None);
}

#[test]
fn an_unbound_key_changes_nothing() {
    let mut observer = start(&["--start-paused"]);
    send(&mut observer, KeyCode::Tab);
    let before = (
        observer.snapshot().tick,
        observer.speed(),
        observer.selection().map(str::to_string),
        observer.overlay(),
        observer.camera(),
        observer.events().len(),
    );

    for code in [
        KeyCode::Char('Q'),
        KeyCode::Char('9'),
        KeyCode::Char('/'),
        KeyCode::Home,
        KeyCode::Delete,
        KeyCode::F(5),
        KeyCode::Enter,
    ] {
        let response = send(&mut observer, code);
        assert!(!response.quit);
        assert!(!response.force_draw);
    }

    assert_eq!(
        (
            observer.snapshot().tick,
            observer.speed(),
            observer.selection().map(str::to_string),
            observer.overlay(),
            observer.camera(),
            observer.events().len(),
        ),
        before
    );
}

#[test]
fn a_key_release_is_not_a_press() {
    let mut observer = start(&["--start-paused"]);
    let release = KeyEvent {
        kind: KeyEventKind::Release,
        ..press(KeyCode::Char('.'))
    };
    observer.handle_key(release).unwrap();
    assert_eq!(observer.snapshot().tick, 0);
}

#[test]
fn quit_is_the_only_key_that_asks_to_exit() {
    let mut observer = start(&[]);
    assert!(send(&mut observer, KeyCode::Char('q')).quit);
}

#[test]
fn the_event_buffer_drops_the_oldest_record_and_says_so() {
    let mut buffer = EventBuffer::new();
    for tick in 0..(EVENT_CAPACITY as u64 + 5) {
        buffer.push(Event {
            tick,
            subject: "world".to_string(),
            detail: EventDetail::SimulationEnded {
                reason: TerminationReason::TickLimit,
            },
        });
    }
    assert_eq!(buffer.len(), EVENT_CAPACITY);
    assert!(buffer.truncated());
    assert_eq!(buffer.iter().next().unwrap().tick, 5);
    assert_eq!(
        buffer.iter().next_back().unwrap().tick,
        EVENT_CAPACITY as u64 + 4
    );
}

#[test]
fn a_death_carries_the_tick_and_the_engine_computed_final_values() {
    let mut observer = start(&["--policy", "baseline", "--ticks", "400", "--start-paused"]);
    while !observer.is_finished() && observer.deaths().is_empty() {
        observer.advance().unwrap();
    }
    let death = observer
        .deaths()
        .first()
        .expect("the baseline policy starves its population well inside 400 ticks")
        .clone();

    assert_eq!(death.health, 0);
    assert!(death.tick > 0);
    assert!(
        death.satiety.is_some(),
        "read from the same tick's survival record"
    );
    assert!(death.energy.is_some());
    assert!(observer.death_of(&death.id).is_some());

    // The roster no longer lists it, and the death total corroborates the disappearance.
    assert!(
        observer
            .snapshot()
            .agents
            .iter()
            .all(|agent| agent.id != death.id)
    );
    assert!(observer.snapshot().deaths >= 1);
}

#[test]
fn shared_cells_are_counted_at_the_rendered_granularity() {
    let mut observer = start(&[]);
    observer.record_geometry((67, 32), 8);
    send(&mut observer, KeyCode::Tab);
    assert!(observer.shared_cell_count() >= 1);

    // With nothing selected there is no cell to share.
    send(&mut observer, KeyCode::Esc);
    assert_eq!(observer.shared_cell_count(), 0);
}
