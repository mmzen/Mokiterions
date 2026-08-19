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
    /// The name the engine reported for each identifier, read from its own `agent_initialized`
    /// record and from nothing else. `REQ-MOK-041` admits no name table, no fallback and no
    /// derivation from an identifier here: an identifier absent from this map has no name to
    /// present, which is why the map is consulted rather than a name being constructed.
    names: BTreeMap<String, String>,
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
            names: BTreeMap::new(),
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
                // `REQ-MOK-041`: the presented name is the one the engine reported, taken from
                // the record it reported it in. The observer's own construction ingests these
                // before the first frame, so no pane is ever drawn with the map unpopulated.
                EventDetail::AgentInitialized { name, .. } => {
                    self.names.insert(event.subject.clone(), name.clone());
                }
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

    /// The name the engine reported for this identifier, or `None` if it reported none.
    ///
    /// `None` is unreachable in a run this observer initialized — `SPEC-MOK-001` rule 1 names
    /// every Mokiterion before tick 1 — and it is returned rather than filled in because
    /// `SPEC-MOK-003` rule 10.7 makes an uncomputed value absent. It is deliberately not the
    /// identifier: presenting an identifier as a name would be the derivation `REQ-MOK-041`
    /// forbids.
    ///
    /// **`pub(crate)` and not `pub`**, so `SPEC-MOK-004` rule 6's interface does not grow. Its only
    /// caller is the rendering module of this same crate; no host outside it needs the map, because
    /// the names are in the records `REQ-MOK-022` already retains. A test is never a reason to
    /// widen this, which rule 6's *Growth* clause states and rule 7 prohibits.
    pub(crate) fn name_of(&self, id: &str) -> Option<&str> {
        self.names.get(id).map(String::as_str)
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
}
