//! The observer's presentation state and its response to operator input.
//!
//! Presentation state is not simulation state. Nothing here is persisted, and the only call
//! that changes simulation state is [`Observer::advance`], which calls the engine's
//! single-tick advance and nothing else (`SPEC-MOK-003` rule 12.1).

use std::collections::{BTreeMap, VecDeque};

#[cfg(test)]
use mokiterions::simulation::DecisionSnapshot;
use mokiterions::simulation::{
    AgentSnapshot, Config, Event, EventDetail, EventType, Simulation, TerminationReason,
    WorldSnapshot,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::export;
use crate::options::{self, Options};
use crate::spatial::{Viewport, Zoom};

/// The event buffer's capacity, fixed by `SPEC-MOK-003`.
pub const EVENT_CAPACITY: usize = 100_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Progression {
    Running,
    Held,
}

impl Progression {
    pub fn label(self) -> &'static str {
        match self {
            Self::Running => "RUNNING",
            Self::Held => "HELD",
        }
    }
}

/// The log filter. Filtering changes presentation only, and never the export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    None,
    Type(EventType),
    Subject(String),
}

impl Filter {
    pub fn matches(&self, event: &Event) -> bool {
        match self {
            Self::None => true,
            Self::Type(event_type) => event.event_type() == *event_type,
            Self::Subject(subject) => event.subject == *subject,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Type(event_type) => format!("event={event_type}"),
            Self::Subject(subject) => format!("subject={subject}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overlay {
    None,
    Roster,
    Log,
    Inspector,
    Help,
    Authority,
}

/// A retained window over the run's authoritative events.
///
/// Dropping the oldest record when full loses presentability, never authority: the engine
/// binary's text stream remains the unbounded record.
#[derive(Debug, Clone, Default)]
pub struct EventBuffer {
    records: VecDeque<Event>,
    truncated: bool,
}

impl EventBuffer {
    pub fn new() -> Self {
        Self {
            records: VecDeque::new(),
            truncated: false,
        }
    }

    pub fn push(&mut self, event: Event) {
        if self.records.len() == EVENT_CAPACITY {
            self.records.pop_front();
            self.truncated = true;
        }
        self.records.push_back(event);
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Present because a `len` without an `is_empty` is a lint, and read by the tests. The panes
    /// ask `presented()` whether they have anything to show, since a filter can empty a pane
    /// that has retained records.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Event> {
        self.records.iter()
    }
}

/// What the engine computed for a Mokiterion at the tick its death was applied.
///
/// Satiety and energy are read from the same tick's `survival_changed` payload rather than
/// invented, and stay absent if that record was never seen, because rule 10.7 forbids
/// presenting a value the engine did not compute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Death {
    pub id: String,
    pub tick: u64,
    pub health: u8,
    pub satiety: Option<u8>,
    pub energy: Option<u8>,
}

/// What the observer does after a key press.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KeyResponse {
    pub quit: bool,
    /// Set only where a rule requires a frame that the ordinary cadence would delay: rule
    /// 6.1's "always draws immediately after a single-step".
    pub force_draw: bool,
}

pub struct Observer {
    simulation: Simulation,
    config: Config,
    snapshot: WorldSnapshot,
    progression: Progression,
    speed: u32,
    selection: Option<String>,
    follow: bool,
    zoom: Zoom,
    camera: (u16, u16),
    filter: Filter,
    overlay: Overlay,
    events: EventBuffer,
    /// Offset of the highlighted record from the newest presented record.
    log_cursor: usize,
    deaths: Vec<Death>,
    latest_survival: BTreeMap<String, (u8, u8)>,
    export_path: Option<String>,
    notice: Option<String>,
    ended_early: bool,
    /// The canvas and log sizes of the most recent frame. Panning and paging are relative to
    /// what the operator last saw; neither reaches the engine.
    last_canvas: (u16, u16),
    last_log_rows: usize,
}

impl Observer {
    pub fn new(options: Options) -> Result<Self, String> {
        let simulation = Simulation::new(options.config)?;
        let mut observer = Self {
            snapshot: simulation.snapshot(),
            config: simulation.configuration(),
            simulation,
            progression: if options.start_paused {
                Progression::Held
            } else {
                Progression::Running
            },
            speed: options.speed,
            selection: None,
            follow: false,
            zoom: Zoom::Overview,
            camera: (0, 0),
            filter: Filter::None,
            overlay: Overlay::None,
            events: EventBuffer::new(),
            log_cursor: 0,
            deaths: Vec::new(),
            latest_survival: BTreeMap::new(),
            export_path: options.export_path,
            notice: None,
            ended_early: false,
            last_canvas: (32, 16),
            last_log_rows: 1,
        };
        let initialization = observer.simulation.initialization_events();
        observer.ingest(initialization);
        Ok(observer)
    }

    // ---- authoritative state -------------------------------------------------------

    /// The only call that changes simulation state.
    pub fn advance(&mut self) -> Result<(), String> {
        if self.simulation.is_finished() {
            return Ok(());
        }
        let outcome = self.simulation.advance_tick()?;
        self.ingest(outcome.events);
        self.snapshot = self.simulation.snapshot();
        Ok(())
    }

    fn ingest(&mut self, events: Vec<Event>) {
        for event in events {
            match &event.detail {
                EventDetail::SurvivalChanged {
                    satiety, energy, ..
                } => {
                    self.latest_survival
                        .insert(event.subject.clone(), (satiety.1, energy.1));
                }
                EventDetail::AgentDied { health } => {
                    let survival = self.latest_survival.get(&event.subject).copied();
                    self.deaths.push(Death {
                        id: event.subject.clone(),
                        tick: event.tick,
                        health: *health,
                        satiety: survival.map(|(satiety, _)| satiety),
                        energy: survival.map(|(_, energy)| energy),
                    });
                }
                _ => {}
            }
            self.events.push(event);
        }
        // A record arriving under a cursor held above the newest would silently shift what
        // the operator is reading; holding the cursor on its own record keeps the log stable.
        self.clamp_log_cursor();
    }

    pub fn snapshot(&self) -> &WorldSnapshot {
        &self.snapshot
    }

    pub fn config(&self) -> Config {
        self.config
    }

    pub fn is_finished(&self) -> bool {
        self.simulation.is_finished()
    }

    pub fn termination_reason(&self) -> Option<TerminationReason> {
        self.simulation.termination_reason()
    }

    // ---- presentation state --------------------------------------------------------

    pub fn progression(&self) -> Progression {
        self.progression
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn zoom(&self) -> Zoom {
        self.zoom
    }

    pub fn follow(&self) -> bool {
        self.follow
    }

    /// The camera as held, which the panning and following tests read. A frame states the
    /// visible world range instead, because that is what rule 2.3 obliges it to state.
    #[allow(dead_code)]
    pub fn camera(&self) -> (u16, u16) {
        self.camera
    }

    pub fn filter(&self) -> &Filter {
        &self.filter
    }

    pub fn overlay(&self) -> Overlay {
        self.overlay
    }

    pub fn events(&self) -> &EventBuffer {
        &self.events
    }

    pub fn selection(&self) -> Option<&str> {
        self.selection.as_deref()
    }

    pub fn notice(&self) -> Option<&str> {
        self.notice.as_deref()
    }

    pub fn set_notice(&mut self, notice: impl Into<String>) {
        self.notice = Some(notice.into());
    }

    pub fn ended_early(&self) -> bool {
        self.ended_early
    }

    pub fn mark_ended_early(&mut self) {
        self.ended_early = !self.simulation.is_finished();
    }

    /// The deaths the observer has seen. The roster and the footer state the engine's own totals;
    /// this list is what rule 10.6's inspector reads through [`Observer::death_of`], and what the
    /// tests read to find a death without assuming one.
    #[allow(dead_code)]
    pub fn deaths(&self) -> &[Death] {
        &self.deaths
    }

    pub fn death_of(&self, id: &str) -> Option<&Death> {
        self.deaths.iter().rev().find(|death| death.id == id)
    }

    pub fn log_cursor(&self) -> usize {
        self.log_cursor
    }

    /// The retained records the current filter presents, oldest first.
    pub fn presented(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|event| self.filter.matches(event))
            .collect()
    }

    /// The record the `t` control reports on: the one the log highlights.
    pub fn highlighted(&self) -> Option<&Event> {
        let presented = self.presented();
        let index = presented.len().checked_sub(self.log_cursor + 1)?;
        presented.get(index).copied()
    }

    pub fn selected_agent(&self) -> Option<&AgentSnapshot> {
        let selection = self.selection.as_deref()?;
        self.snapshot
            .agents
            .iter()
            .find(|agent| agent.id == selection)
    }

    /// How many living Mokiterions share the selected one's rendered cell, the selected one
    /// included. It depends on the zoom, because a cell covers eight world cells in overview.
    pub fn shared_cell_count(&self) -> usize {
        let Some(selected) = self.selected_agent() else {
            return 0;
        };
        let viewport = Viewport::resolve(self.zoom, self.last_canvas, self.camera);
        let Some(cell) = viewport.cell_of(
            self.zoom,
            selected.position.x.into(),
            selected.position.y.into(),
        ) else {
            return 1;
        };
        self.snapshot
            .agents
            .iter()
            .filter(|agent| {
                viewport.cell_of(self.zoom, agent.position.x.into(), agent.position.y.into())
                    == Some(cell)
            })
            .count()
    }

    /// Records the geometry of the frame just drawn, so panning and paging are relative to
    /// what the operator last saw.
    pub fn record_geometry(&mut self, canvas: (u16, u16), log_rows: usize) {
        self.last_canvas = canvas;
        self.last_log_rows = log_rows.max(1);
    }

    /// Rule 2.6's following, applied at draw time because it depends on the canvas size.
    pub fn apply_follow(&mut self, canvas: (u16, u16)) {
        self.last_canvas = canvas;
        if !self.follow {
            return;
        }
        let Some(agent) = self.selected_agent() else {
            return;
        };
        let (x, y) = (u16::from(agent.position.x), u16::from(agent.position.y));
        let viewport = Viewport::resolve(self.zoom, canvas, self.camera);
        let (limit_x, limit_y) = viewport.camera_limit();
        self.camera = (
            x.saturating_sub(viewport.width / 2).min(limit_x),
            y.saturating_sub(viewport.height / 2).min(limit_y),
        );
    }

    pub fn viewport(&self) -> Viewport {
        Viewport::resolve(self.zoom, self.last_canvas, self.camera)
    }

    // ---- operator input ------------------------------------------------------------

    /// Applies one key press. An unbound key is ignored: no action, no state change, no
    /// diagnostic (rule 6.4). No binding mutates world state except rule 1's advance.
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<KeyResponse, String> {
        if key.kind != KeyEventKind::Press {
            return Ok(KeyResponse::default());
        }
        let scrolling_log = self.overlay == Overlay::Log;
        let mut response = KeyResponse::default();

        match key.code {
            KeyCode::Char(' ') => {
                self.progression = match self.progression {
                    Progression::Running => Progression::Held,
                    Progression::Held => Progression::Running,
                };
            }
            KeyCode::Char('.') => {
                if self.progression == Progression::Held {
                    self.advance()?;
                    response.force_draw = true;
                }
            }
            KeyCode::Char('+') => self.speed = options::faster(self.speed),
            KeyCode::Char('-') => self.speed = options::slower(self.speed),
            KeyCode::Tab => self.step_selection(true),
            KeyCode::BackTab => self.step_selection(false),
            KeyCode::Esc => {
                if self.overlay == Overlay::None {
                    self.selection = None;
                } else {
                    self.overlay = Overlay::None;
                }
            }
            KeyCode::Char('f') => self.follow = !self.follow,
            KeyCode::Char('z') => self.zoom = self.zoom.toggled(),
            KeyCode::Char('h') | KeyCode::Left => self.pan(-1, 0),
            KeyCode::Char('l') | KeyCode::Right => self.pan(1, 0),
            KeyCode::Char('j') | KeyCode::Down => {
                if scrolling_log {
                    self.scroll_log(-1);
                } else {
                    self.pan(0, 1);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if scrolling_log {
                    self.scroll_log(1);
                } else {
                    self.pan(0, -1);
                }
            }
            KeyCode::PageDown => {
                if scrolling_log {
                    self.scroll_log(-(self.last_log_rows as isize));
                } else {
                    self.pan(0, self.viewport().height as i32);
                }
            }
            KeyCode::PageUp => {
                if scrolling_log {
                    self.scroll_log(self.last_log_rows as isize);
                } else {
                    self.pan(0, -(self.viewport().height as i32));
                }
            }
            KeyCode::Char('e') => self.cycle_type_filter(),
            KeyCode::Char('u') => {
                if let Some(selection) = self.selection.clone() {
                    self.filter = Filter::Subject(selection);
                    self.log_cursor = 0;
                }
            }
            KeyCode::Char('c') => {
                self.filter = Filter::None;
                self.log_cursor = 0;
            }
            KeyCode::Char('x') => self.export(),
            KeyCode::Char('t') => self.overlay = Overlay::Authority,
            KeyCode::Char('r') => self.overlay = Overlay::Roster,
            KeyCode::Char('L') => self.overlay = Overlay::Log,
            KeyCode::Char('i') => self.overlay = Overlay::Inspector,
            KeyCode::Char('?') => self.overlay = Overlay::Help,
            KeyCode::Char('q') => response.quit = true,
            _ => {}
        }
        Ok(response)
    }

    fn pan(&mut self, dx: i32, dy: i32) {
        let viewport = self.viewport();
        let (limit_x, limit_y) = viewport.camera_limit();
        self.camera = (
            (i32::from(viewport.origin_x) + dx).clamp(0, i32::from(limit_x)) as u16,
            (i32::from(viewport.origin_y) + dy).clamp(0, i32::from(limit_y)) as u16,
        );
    }

    /// Positive scrolls towards older records, which is upward on screen.
    fn scroll_log(&mut self, lines: isize) {
        let cursor = self.log_cursor as isize + lines;
        self.log_cursor = cursor.max(0) as usize;
        self.clamp_log_cursor();
    }

    fn clamp_log_cursor(&mut self) {
        let presented = self.presented().len();
        self.log_cursor = self.log_cursor.min(presented.saturating_sub(1));
    }

    fn cycle_type_filter(&mut self) {
        let next = match &self.filter {
            Filter::Type(current) => EventType::ALL
                .iter()
                .position(|event_type| event_type == current)
                .and_then(|index| EventType::ALL.get(index + 1).copied()),
            _ => EventType::ALL.first().copied(),
        };
        self.filter = match next {
            Some(event_type) => Filter::Type(event_type),
            None => Filter::None,
        };
        self.log_cursor = 0;
    }

    fn step_selection(&mut self, forward: bool) {
        let ids: Vec<&str> = self
            .snapshot
            .agents
            .iter()
            .map(|agent| agent.id.as_str())
            .collect();
        if ids.is_empty() {
            self.selection = None;
            return;
        }
        let next = match self.selection.as_deref() {
            None if forward => ids[0],
            None => ids[ids.len() - 1],
            Some(current) => match ids.iter().position(|id| *id == current) {
                Some(index) if forward => ids[(index + 1) % ids.len()],
                Some(index) => ids[(index + ids.len() - 1) % ids.len()],
                // The selection died. Rule 10.6: move to the nearest living Mokiterion in
                // roster order.
                None if forward => ids
                    .iter()
                    .find(|id| **id > current)
                    .copied()
                    .unwrap_or(ids[0]),
                None => ids
                    .iter()
                    .rev()
                    .find(|id| **id < current)
                    .copied()
                    .unwrap_or(ids[ids.len() - 1]),
            },
        };
        self.selection = Some(next.to_string());
    }

    /// Rule 9.4's export: every retained record, in authoritative order, ignoring any filter.
    fn export(&mut self) {
        let path = match &self.export_path {
            Some(path) => path.clone(),
            None => export::default_path(self.config.seed, self.snapshot.tick),
        };
        // The resolved path never reaches a frame: an operator-supplied path may be absolute,
        // and no absolute path may appear in a frame.
        let retained = self.events.len();
        match export::write_file(&path, &self.events) {
            Ok(()) => self.set_notice(format!("export: wrote {retained} records")),
            Err(error) => self.set_notice(format!("export failed: {error}")),
        }
    }

    #[cfg(test)]
    pub fn select_for_test(&mut self, id: &str) {
        self.selection = Some(id.to_string());
    }

    #[cfg(test)]
    pub fn set_overlay_for_test(&mut self, overlay: Overlay) {
        self.overlay = overlay;
    }

    /// Replaces the retained snapshot's decisions.
    ///
    /// `VER-MOK-005` requires the observer to be shown a snapshot whose outcome contradicts what
    /// a validation rule would produce, and to present the snapshot's outcome anyway. Neither
    /// shipped decision source can produce a rejection, since both propose only actions the
    /// engine has already declared valid, so that case is unreachable through a run and this hook
    /// is how it is reached. It is compiled out of the shipped binary.
    #[cfg(test)]
    pub fn replace_decisions_for_test(&mut self, decisions: Vec<DecisionSnapshot>) {
        self.snapshot.decisions = decisions;
    }

    /// Replaces the retained snapshot wholesale.
    ///
    /// `VER-MOK-005` requires a frame to be drawn on a world with no living Mokiterions, one with
    /// no standing resources, and one with neither. Extinction is reachable through a run, but a
    /// world stripped of every standing resource is not: rule 15 makes regeneration conditional on
    /// one remaining resource, and no shipped decision source consumes fast enough to empty a
    /// territory before the population dies. This hook is how that state is reached. It is
    /// compiled out of the shipped binary.
    #[cfg(test)]
    pub fn replace_snapshot_for_test(&mut self, snapshot: WorldSnapshot) {
        self.snapshot = snapshot;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::Startup;
    use crate::spatial::WORLD_SIZE;
    use ratatui::crossterm::event::{KeyEventState, KeyModifiers};

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
    fn the_log_cursor_scrolls_only_inside_the_log_overlay() {
        let mut observer = start(&["--start-paused"]);
        observer.advance().unwrap();
        observer.record_geometry((40, 20), 6);

        // Outside the overlay, `j` and `k` pan.
        send(&mut observer, KeyCode::Char('k'));
        assert_eq!(observer.log_cursor(), 0);

        send(&mut observer, KeyCode::Char('L'));
        send(&mut observer, KeyCode::Char('k'));
        assert_eq!(observer.log_cursor(), 1);
        send(&mut observer, KeyCode::PageUp);
        assert_eq!(observer.log_cursor(), 7);
        send(&mut observer, KeyCode::Char('j'));
        assert_eq!(observer.log_cursor(), 6);
        send(&mut observer, KeyCode::PageDown);
        assert_eq!(observer.log_cursor(), 0);

        // The cursor never leaves the retained records.
        observer.scroll_log(1_000_000);
        assert_eq!(observer.log_cursor(), observer.presented().len() - 1);
        observer.scroll_log(-1_000_000);
        assert_eq!(observer.log_cursor(), 0);
    }

    #[test]
    fn the_highlighted_record_is_the_newest_until_the_operator_scrolls() {
        let mut observer = start(&["--start-paused"]);
        observer.advance().unwrap();
        let presented = observer.presented().len();

        assert_eq!(
            observer.highlighted().map(|event| event.to_string()),
            observer.presented().last().map(|event| event.to_string())
        );

        observer.set_overlay_for_test(Overlay::Log);
        send(&mut observer, KeyCode::Char('k'));
        assert_eq!(
            observer.highlighted().map(|event| event.to_string()),
            Some(observer.presented()[presented - 2].to_string())
        );
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
    fn a_dead_selection_is_retained_and_the_next_control_finds_a_living_neighbour() {
        // The reference policy loses its first Mokiterion at tick 604 with eleven still living,
        // which is the state rule 10.6 describes. The baseline policy cannot serve here: it
        // starves the whole population on one tick, leaving no living neighbour to move to.
        let mut observer = start(&["--policy", "reference", "--ticks", "700", "--start-paused"]);
        while !observer.is_finished() && observer.deaths().is_empty() {
            observer.advance().unwrap();
        }
        let dead = observer.deaths()[0].id.clone();
        assert!(
            !observer.snapshot().agents.is_empty(),
            "the scenario needs a living neighbour"
        );
        observer.select_for_test(&dead);

        assert_eq!(observer.selection(), Some(dead.as_str()));
        assert!(observer.selected_agent().is_none());
        assert!(observer.death_of(&dead).is_some());

        send(&mut observer, KeyCode::Tab);
        let forward = observer.selection().unwrap().to_string();
        assert_ne!(forward, dead);
        assert!(observer.selected_agent().is_some());
        assert!(
            forward > dead,
            "forward moves to the next living identifier"
        );

        // Backwards from the same dead selection reaches the living Mokiterion below it.
        observer.select_for_test(&dead);
        send(&mut observer, KeyCode::BackTab);
        let backward = observer.selection().unwrap().to_string();
        assert!(backward < dead, "{backward} is not below {dead}");
        assert!(observer.selected_agent().is_some());
    }

    /// Extinction leaves nothing to select, which is stated by clearing the selection rather
    /// than by holding a Mokiterion that no longer exists.
    #[test]
    fn selection_clears_itself_when_no_living_mokiterion_remains() {
        let mut observer = start(&["--policy", "baseline", "--ticks", "400", "--start-paused"]);
        while !observer.is_finished() {
            observer.advance().unwrap();
        }
        assert_eq!(
            observer.termination_reason(),
            Some(TerminationReason::Extinction)
        );
        assert!(observer.snapshot().agents.is_empty());

        observer.select_for_test("M01");
        send(&mut observer, KeyCode::Tab);
        assert_eq!(observer.selection(), None);
        assert!(
            observer.death_of("M01").is_some(),
            "the death is still retained"
        );
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
}
