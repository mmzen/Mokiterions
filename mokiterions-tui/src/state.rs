//! The observer's presentation state and its response to operator input.
//!
//! Presentation state is not simulation state. Nothing here is persisted, and the only call
//! that changes simulation state is [`Observer::advance`], which calls the engine's
//! single-tick advance and nothing else (`SPEC-MOK-003` rule 12.1).

use std::collections::{BTreeMap, VecDeque};

#[cfg(test)]
use mokiterions::simulation::DecisionSnapshot;
use mokiterions::simulation::{
    Action, AgentSnapshot, Config, DecisionOutcome, Event, EventDetail, EventType, Simulation,
    TerminationReason, WorldSnapshot,
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
/// Satiety, energy and fear are read from the same tick's `survival_changed` payload rather
/// than invented, and stay absent if that record was never seen, because rule 10.7 forbids
/// presenting a value the engine did not compute. All three travel together under one
/// `Option` per field for that reason: none is defaulted when the record is missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Death {
    pub id: String,
    pub tick: u64,
    pub health: u8,
    pub satiety: Option<u8>,
    pub energy: Option<u8>,
    pub fear: Option<u8>,
}

/// One kind of the engine's action contract, which `SPEC-MOK-001` rule 21 closes at eleven.
///
/// The observer counts *kinds*, not actions: `eat:f07` and `eat:f11` are one kind, and the target
/// of a directed verb is not part of the count. A twelfth kind cannot be silently left uncounted
/// here, which is `VER-MOK-017` P3's obligation. [`ActionKind::of`] matches the contract without a
/// wildcard, so a new variant of the engine's enum fails to compile there; and [`ActionKind::ALL`]
/// is declared at length [`ActionKind::COUNT`], so a new variant of *this* enum that the array does
/// not carry fails to compile too. Neither check is a list copied from the contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ActionKind {
    Wait,
    Sleep,
    Eat,
    Move,
    Attack,
    Threaten,
    Fight,
    Retreat,
    Surrender,
    Approach,
    Avoid,
}

impl ActionKind {
    /// The eleven kinds the contract is closed at.
    pub(crate) const COUNT: usize = 11;

    /// Every kind, in the order the engine's enum declares them, which is the order the pane
    /// presents them in. The discriminants index [`Profile::applied`], so the order here and the
    /// order there are one order rather than two that agree.
    pub(crate) const ALL: [Self; Self::COUNT] = [
        Self::Wait,
        Self::Sleep,
        Self::Eat,
        Self::Move,
        Self::Attack,
        Self::Threaten,
        Self::Fight,
        Self::Retreat,
        Self::Surrender,
        Self::Approach,
        Self::Avoid,
    ];

    /// The kind of one action. No wildcard arm: a twelfth kind of the contract stops the build.
    fn of(action: &Action) -> Self {
        match action {
            Action::Wait => Self::Wait,
            Action::Sleep => Self::Sleep,
            Action::Eat { .. } => Self::Eat,
            Action::Move { .. } => Self::Move,
            Action::Attack { .. } => Self::Attack,
            Action::Threaten { .. } => Self::Threaten,
            Action::Fight { .. } => Self::Fight,
            Action::Retreat { .. } => Self::Retreat,
            Action::Surrender { .. } => Self::Surrender,
            Action::Approach { .. } => Self::Approach,
            Action::Avoid { .. } => Self::Avoid,
        }
    }

    /// The engine's own verb for this kind.
    ///
    /// Nine of the eleven are byte-identical to the engine's own `Display`. `eat` and `move` are
    /// the engine's verb without the payload its records carry after the colon, because a label
    /// heads a count of a kind and `eat:f07` is not a kind.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Wait => "wait",
            Self::Sleep => "sleep",
            Self::Eat => "eat",
            Self::Move => "move",
            Self::Attack => "attack",
            Self::Threaten => "threaten",
            Self::Fight => "fight",
            Self::Retreat => "retreat",
            Self::Surrender => "surrender",
            Self::Approach => "approach",
            Self::Avoid => "avoid",
        }
    }
}

/// The cumulative activity totals the observer retains for one Mokiterion (`REQ-MOK-061`).
///
/// Every field counts records the engine stated, accumulated once per completed tick inside
/// [`Observer::advance`]. **Nothing here is recomputed from the retained event buffer**: that
/// buffer drops its oldest record at [`EVENT_CAPACITY`] and marks itself `truncated`, so a total
/// read back from it would begin understating part-way through a long run while still presenting
/// as a figure — the one failure mode a profile pane cannot afford.
///
/// The counters are `u64` and saturate, and saturation is unreachable for any admissible run.
/// Per completed tick a Mokiterion contributes exactly one to `opportunities`, at most one across
/// the eleven `applied` cells and at most one to `rejected`; `crossings` and `killed` are bounded
/// by the events of a single tick. A run's tick count is bounded by `Config::tick_limit`, itself a
/// `u64`, so a total cannot exceed the number of ticks by more than a small constant factor and
/// `u64::MAX` is out of reach. `saturating_add` is the discipline *at* the limit, not the
/// mechanism the bound rests on.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Profile {
    /// One count per action kind, indexed by [`ActionKind`]'s discriminant. Counted on the
    /// *applied* action, because rule 10.3's proposal is what was asked for and the applied
    /// action is what the world did.
    applied: [u64; ActionKind::COUNT],
    rejected: u64,
    crossings: u64,
    killed: u64,
    /// Completed ticks on which the engine stated a decision for this Mokiterion. It is the
    /// denominator of the other totals rather than a total of its own, and `REQ-MOK-061`
    /// clause 2's identity is over it.
    opportunities: u64,
}

impl Profile {
    pub(crate) fn applied(&self, kind: ActionKind) -> u64 {
        self.applied[kind as usize]
    }

    pub(crate) fn rejected(&self) -> u64 {
        self.rejected
    }

    pub(crate) fn crossings(&self) -> u64 {
        self.crossings
    }

    pub(crate) fn killed(&self) -> u64 {
        self.killed
    }

    pub(crate) fn opportunities(&self) -> u64 {
        self.opportunities
    }

    fn count_applied(&mut self, kind: ActionKind) {
        let cell = &mut self.applied[kind as usize];
        *cell = cell.saturating_add(1);
    }

    /// Adds one Mokiterion's totals into a population total (`REQ-MOK-062`).
    ///
    /// The population figures are a summation over these records and are not accumulated a
    /// second time: one accumulation with one summation cannot disagree with itself.
    fn absorb(&mut self, other: &Self) {
        for (total, addend) in self.applied.iter_mut().zip(other.applied) {
            *total = total.saturating_add(addend);
        }
        self.rejected = self.rejected.saturating_add(other.rejected);
        self.crossings = self.crossings.saturating_add(other.crossings);
        self.killed = self.killed.saturating_add(other.killed);
        self.opportunities = self.opportunities.saturating_add(other.opportunities);
    }
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
    /// The satiety, energy and fear the engine last reported for each identifier, in that
    /// order. Fear is retained alongside the other two because rule 10.6 presents the final
    /// attribute values of a dead subject, and a dead subject has left the roster where
    /// rule 4 presents fear for the living.
    latest_survival: BTreeMap<String, (u8, u8, u8)>,
    /// The cumulative activity totals per identifier (`REQ-MOK-061`). A record is created when the
    /// engine's own initialization records name a Mokiterion and is **never removed**, so a dead
    /// Mokiterion's totals stay presentable under rule 10.6 and stay inside `REQ-MOK-062`'s sum.
    /// One record per initialized Mokiterion is the whole bound: it is fixed by the population and
    /// does not grow with ticks.
    profiles: BTreeMap<String, Profile>,
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
            profiles: BTreeMap::new(),
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
        // After the refresh, and only here: the snapshot the engine produces once a tick has been
        // applied is the one carrying that tick's decision records. Accumulating before the refresh
        // would count the previous tick a second time and the final tick never.
        self.accumulate_decisions();
        // Rule 10's extinction consequence. With no living Mokiterion there is nothing to hold
        // selected, so the selection is cleared here rather than at the next operator act, and a
        // run that ends in extinction presents the population's completed totals unprompted.
        if self.snapshot.agents.is_empty() {
            self.selection = None;
        }
        Ok(())
    }

    /// The decision-derived totals of the tick just completed.
    ///
    /// Two sources are read for this profile and each for what only it carries: the decision record
    /// states the verb that was applied and whether the proposal was rejected, and the tick's
    /// authoritative events state crossings and kills — those are accumulated in [`Observer::ingest`]
    /// instead, where the events arrive.
    ///
    /// A decision naming an identifier with no retained record is skipped rather than creating one.
    /// Creating one here would make the retained record count grow with ticks, which is exactly the
    /// bound `REQ-MOK-062` rests on, and would silently absorb a discrepancy that `VER-MOK-017` O3's
    /// identity is there to expose.
    fn accumulate_decisions(&mut self) {
        // The snapshot and the records are disjoint fields, split here so the loop can read one
        // while writing the other.
        let Self {
            snapshot, profiles, ..
        } = self;
        for decision in &snapshot.decisions {
            let Some(profile) = profiles.get_mut(&decision.agent_id) else {
                continue;
            };
            profile.opportunities = profile.opportunities.saturating_add(1);
            match &decision.outcome {
                DecisionOutcome::Accepted => {}
                DecisionOutcome::Rejected { .. } => {
                    profile.rejected = profile.rejected.saturating_add(1);
                }
            }
            // Absent exactly where the proposal was rejected, so the eleven verb totals plus the
            // rejected total is the opportunity count.
            if let Some(applied) = &decision.applied {
                profile.count_applied(ActionKind::of(applied));
            }
        }
    }

    fn ingest(&mut self, events: Vec<Event>) {
        for event in events {
            match &event.detail {
                // `REQ-MOK-041`: the presented name is the one the engine reported, taken from
                // the record it reported it in. The observer's own construction ingests these
                // before the first frame, so no pane is ever drawn with the map unpopulated.
                EventDetail::AgentInitialized { name, .. } => {
                    self.names.insert(event.subject.clone(), name.clone());
                    // `REQ-MOK-061`: the record exists from the moment the engine names its
                    // subject, so every total the pane presents afterwards is a measurement over
                    // the whole run and a zero is a measured zero.
                    self.profiles.entry(event.subject.clone()).or_default();
                }
                // The two totals only the event stream carries. The decision record states that
                // `move` was applied and that `attack` was applied; it does not state that the move
                // crossed the boundary or that the strike killed, so those are counted here.
                EventDetail::TerritoryCrossed { .. } => {
                    if let Some(profile) = self.profiles.get_mut(&event.subject) {
                        profile.crossings = profile.crossings.saturating_add(1);
                    }
                }
                EventDetail::AttackResolved { target_died, .. } => {
                    if *target_died && let Some(profile) = self.profiles.get_mut(&event.subject) {
                        profile.killed = profile.killed.saturating_add(1);
                    }
                }
                EventDetail::SurvivalChanged {
                    satiety,
                    energy,
                    fear,
                    ..
                } => {
                    self.latest_survival
                        .insert(event.subject.clone(), (satiety.1, energy.1, fear.1));
                }
                EventDetail::AgentDied { health } => {
                    let survival = self.latest_survival.get(&event.subject).copied();
                    self.deaths.push(Death {
                        id: event.subject.clone(),
                        tick: event.tick,
                        health: *health,
                        satiety: survival.map(|(satiety, _, _)| satiety),
                        energy: survival.map(|(_, energy, _)| energy),
                        fear: survival.map(|(_, _, fear)| fear),
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

    /// This Mokiterion's cumulative totals, or `None` if the engine never named it.
    ///
    /// `None` is unreachable for any identifier a run produced — `SPEC-MOK-001` rule 1 names every
    /// Mokiterion before tick 1 — and it is returned rather than defaulted because a record of
    /// zeros for a subject that was never measured is the invented value rule 10.7 forbids, and is
    /// indistinguishable on the pane from the measured zero `VER-MOK-017` O6 requires.
    ///
    /// **`pub(crate)` and not `pub`**, so `SPEC-MOK-004` rule 6's interface does not grow. No host
    /// outside this crate needs the totals; rule 6's *Growth* clause states that a test is never
    /// the requirement that widens an item, and `ARCH-MOK-002` prohibits widening one to reach it
    /// from a test. The exhaustive per-Mokiterion oracles therefore live in rule 10's internal tier
    /// and the frame-level ones in rule 9's public tier, which is where `VER-MOK-017`'s
    /// independence clause 3 puts them.
    pub(crate) fn profile_of(&self, id: &str) -> Option<&Profile> {
        self.profiles.get(id)
    }

    /// The population's totals: the sum over every retained record, the dead included
    /// (`REQ-MOK-062`).
    pub(crate) fn population_profile(&self) -> Profile {
        let mut total = Profile::default();
        for profile in self.profiles.values() {
            total.absorb(profile);
        }
        total
    }

    /// How many Mokiterions the engine's initialization records named, which is also the number of
    /// retained records: `VER-MOK-017` P4's bound is that these two are the same figure.
    pub(crate) fn initialized_count(&self) -> usize {
        self.profiles.len()
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
        // Rule 10.6's state, and both directions of it: a dead Mokiterion with a living
        // identifier on each side, so that Tab and BackTab are asserted on direction rather
        // than on the wrap. The baseline policy cannot serve here: it starves the whole
        // population on one tick, leaving no living neighbour to move to.
        //
        // The run is advanced until such a death appears rather than until the first one,
        // because *which* identifier dies first is a property of the world and not of the
        // observer. Under the reference policy it was an interior identifier at tick 604 until
        // `REQ-MOK-060` corrected the waste condition on 2026-08-21, and is `M01` now. Searching
        // for the state the rule describes keeps both assertions below exactly as written; a
        // scenario pinned to the first death would have had to accept a wrap, which is a weaker
        // claim about a different behavior.
        let mut observer = start(&["--policy", "reference", "--ticks", "700", "--start-paused"]);
        let dead = loop {
            let living: Vec<String> = observer
                .snapshot()
                .agents
                .iter()
                .map(|agent| agent.id.clone())
                .collect();
            let interior = observer
                .deaths()
                .iter()
                .map(|death| &death.id)
                .find(|dead| {
                    living.iter().any(|id| id < *dead) && living.iter().any(|id| id > *dead)
                });
            if let Some(dead) = interior {
                break dead.clone();
            }
            assert!(
                !observer.is_finished(),
                "the run ended without a dead Mokiterion holding living identifiers on both \
                 sides, so rule 10.6's state was never reached"
            );
            observer.advance().unwrap();
        };
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

    /// Rule 10.7's standing rule, asserted on the one attribute for which a run cannot
    /// produce the negative case: every death a run reaches is preceded by a
    /// `survival_changed` record for the same subject, so satiety, energy and fear are
    /// always present there. The absence branch is reachable only by ingesting a death for
    /// a subject the observer never saw a survival record for.
    ///
    /// This test lives beside the code because `ingest` is private and `ARCH-MOK-002`
    /// forbids widening an item's visibility to reach it from a test and fixes the observer's
    /// four `#[cfg(test)]` hooks at four. Both subjects are ingested in one call so that the
    /// assertion discriminates between the two branches rather than observing a uniform
    /// absence.
    ///
    /// Living here also puts the frame within reach, so the absence is asserted where rule 10.7
    /// binds — on the rendered pane — and not only on the derived value. The one part of rule
    /// 10.6 no frame assertion can carry is the suppression of the empty second line: the death
    /// branch returns with that line last, so a line the code declined to emit and the pane's own
    /// unwritten rows are the same cells. That guard is asserted by the code's shape, not measured.
    #[test]
    fn a_death_carries_no_attribute_the_engine_never_reported_for_its_subject() {
        let mut observer = start(&["--start-paused"]);

        let reported = "M01";
        let unreported = "M99";
        observer.ingest(vec![
            Event {
                tick: 7,
                subject: reported.to_string(),
                detail: EventDetail::SurvivalChanged {
                    health: (10, 0),
                    satiety: (40, 38),
                    energy: (30, 29),
                    fear: (55, 61),
                },
            },
            Event {
                tick: 7,
                subject: reported.to_string(),
                detail: EventDetail::AgentDied { health: 0 },
            },
            Event {
                tick: 7,
                subject: unreported.to_string(),
                detail: EventDetail::AgentDied { health: 0 },
            },
        ]);

        let carried = observer.death_of(reported).unwrap();
        assert_eq!(carried.satiety, Some(38));
        assert_eq!(carried.energy, Some(29));
        assert_eq!(
            carried.fear,
            Some(61),
            "fear is the value the engine reported, not the value it moved from"
        );

        let absent = observer.death_of(unreported).unwrap();
        assert_eq!(absent.health, 0, "the death record's own payload is read");
        assert_eq!(absent.satiety, None);
        assert_eq!(absent.energy, None);
        assert_eq!(
            absent.fear, None,
            "an unreported attribute is absent, never zero-filled"
        );

        // The same claim at the frame, which is where rule 10.7 is an obligation rather than a
        // property of a derived value. It is asserted here and not in a public-tier file for the
        // reason the state above exists at all: reaching it needs the private `ingest`.
        observer.select_for_test(unreported);
        let inspector = inspector_text(&mut observer, 160, 48);
        assert!(
            inspector.contains("final health 0"),
            "the death line is not presented:\n{inspector}"
        );
        for withheld in ["satiety", "energy", "fear"] {
            assert!(
                !inspector.contains(withheld),
                "`{withheld}` is presented for a subject the engine never reported it for, which \
                 rule 10.7 forbids as directly as a zero would:\n{inspector}"
            );
        }
    }

    // ---- `VER-MOK-017`'s independent count ------------------------------------------------
    //
    // Everything from here to the frame helper below is the oracle side of `VER-MOK-017` and the
    // cases that read it. Independence clause 1 obliges the count to be straight-line code over
    // the engine's own records rather than a second call into the accumulator, so nothing here
    // reads a total except to compare against it. Clause 2 obliges a second engine record wherever
    // one exists, which is why the verbs are counted from the tick's `action_trace` records and not
    // from the snapshot the observer itself reads: an off-by-one in where the accumulation sits
    // relative to the snapshot refresh would be reproduced exactly by an oracle reading the same
    // snapshot, and would pass.
    //
    // These cases live beside the code because the totals are `pub(crate)`. Rule 6's *Growth*
    // clause makes a test no reason to widen an item and `ARCH-MOK-002` prohibits widening one to
    // reach it from a test, so the exhaustive per-Mokiterion oracles are here and the frame-level
    // ones — O13, O16, O17, O18 and the static checks — are in the public tier, which is where
    // independence clause 3 puts them.

    /// The seeds the observer's and the engine's own suites already declare.
    const DECLARED_SEEDS: [u64; 5] = [0, 1, 42, 123, 777];

    /// One Mokiterion's totals as this contract counts them, from the engine's records alone.
    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    struct Counted {
        /// Accepted proposals, keyed by the engine's own verb text rather than by [`ActionKind`],
        /// so the oracle does not share the mapping it is checking.
        verbs: BTreeMap<String, u64>,
        rejected: u64,
        crossings: u64,
        killed: u64,
        opportunities: u64,
        /// Independence clause 2's second records: `food_consumed` and `attack_resolved`, counted
        /// whatever verb produced them.
        consumed: u64,
        strikes: u64,
    }

    impl Counted {
        fn verb(&self, verb: &str) -> u64 {
            self.verbs.get(verb).copied().unwrap_or(0)
        }
    }

    /// The engine's own verb for an action: its `Display` text up to any payload.
    ///
    /// `eat:f07` and `move:north` carry a payload the engine appends after a colon, and a payload
    /// is not a kind. The seven targeted verbs render bare and pass through unchanged.
    fn engine_verb(action: &Action) -> String {
        let rendered = action.to_string();
        match rendered.split_once(':') {
            Some((verb, _)) => verb.to_string(),
            None => rendered,
        }
    }

    /// Every identifier the engine named, taken from the roster and the retained deaths.
    ///
    /// Together those cover the whole initialized population, since a Mokiterion is either living
    /// or dead. Asserting that against the retained record count is O11's second clause.
    fn every_identifier(observer: &Observer) -> Vec<String> {
        let mut ids: Vec<String> = observer
            .snapshot()
            .agents
            .iter()
            .map(|agent| agent.id.clone())
            .collect();
        ids.extend(observer.deaths().iter().map(|death| death.id.clone()));
        ids.sort();
        ids.dedup();
        ids
    }

    /// Runs an observer to the end of its run and returns it.
    fn to_the_end(args: &[&str]) -> Observer {
        let mut observer = start(args);
        while !observer.is_finished() {
            observer.advance().unwrap();
        }
        observer
    }

    /// Drives an independently configured engine tick for tick beside an observer started on the
    /// same arguments, calling `check` after every completed tick, and returns both at the end.
    ///
    /// The independent engine runs with `trace_actions` forced on, because `SPEC-MOK-001` rule 7
    /// emits the `action_trace` record only under that flag and it is the record the verbs are
    /// counted from. The flag is read in exactly one place in the engine — the emission itself —
    /// so it adds records and moves no state; O7 asserts separately that the observer's own totals
    /// do not move with it either.
    fn in_lockstep(
        args: &[&str],
        mut check: impl FnMut(&Observer, &BTreeMap<String, Counted>),
    ) -> (Observer, BTreeMap<String, Counted>) {
        let mut observer = start(args);
        let mut options = match options::parse(args.to_vec()).unwrap() {
            Startup::Run(options) => options,
            Startup::Help => panic!("expected a run"),
        };
        options.config.trace_actions = true;
        let mut engine = Simulation::new(options.config).expect("the same configuration");
        let mut counted: BTreeMap<String, Counted> = engine
            .initialization_events()
            .into_iter()
            .filter(|event| matches!(event.detail, EventDetail::AgentInitialized { .. }))
            .map(|event| (event.subject, Counted::default()))
            .collect();
        assert!(!counted.is_empty(), "the engine named no Mokiterion");

        while !observer.is_finished() {
            observer.advance().unwrap();
            let outcome = engine.advance_tick().expect("the same tick");
            for event in &outcome.events {
                let Some(entry) = counted.get_mut(&event.subject) else {
                    continue;
                };
                match &event.detail {
                    EventDetail::ActionTrace {
                        proposal, accepted, ..
                    } => {
                        entry.opportunities += 1;
                        if *accepted {
                            *entry.verbs.entry(engine_verb(proposal)).or_default() += 1;
                        } else {
                            entry.rejected += 1;
                        }
                    }
                    EventDetail::TerritoryCrossed { .. } => entry.crossings += 1,
                    EventDetail::FoodConsumed { .. } => entry.consumed += 1,
                    EventDetail::AttackResolved { target_died, .. } => {
                        entry.strikes += 1;
                        if *target_died {
                            entry.killed += 1;
                        }
                    }
                    _ => {}
                }
            }
            assert_eq!(
                observer.snapshot().tick,
                engine.snapshot().tick,
                "the observed and the independent run left lockstep"
            );
            check(&observer, &counted);
        }
        assert!(
            engine.is_finished(),
            "the independent run outlived the observed one, so the two are not the same run"
        );
        (observer, counted)
    }

    /// O1: each of the fourteen totals against the independent count, and the opportunity count
    /// beside them.
    fn assert_totals_match(observer: &Observer, oracle: &BTreeMap<String, Counted>) {
        let tick = observer.snapshot().tick;
        assert_eq!(
            observer.initialized_count(),
            oracle.len(),
            "the retained record count is not the number of Mokiterions the engine named"
        );
        for (id, expected) in oracle {
            let profile = observer
                .profile_of(id)
                .unwrap_or_else(|| panic!("no retained record for {id} at tick {tick}"));
            for kind in ActionKind::ALL {
                assert_eq!(
                    profile.applied(kind),
                    expected.verb(kind.label()),
                    "{id}'s applied `{}` at tick {tick}",
                    kind.label()
                );
            }
            assert_eq!(
                profile.rejected(),
                expected.rejected,
                "{id}'s rejected proposals at tick {tick}"
            );
            assert_eq!(
                profile.crossings(),
                expected.crossings,
                "{id}'s territory crossings at tick {tick}"
            );
            assert_eq!(
                profile.killed(),
                expected.killed,
                "{id}'s killed total at tick {tick}"
            );
            assert_eq!(
                profile.opportunities(),
                expected.opportunities,
                "{id}'s decision opportunities at tick {tick}"
            );
        }
    }

    /// P1: no figure of `now` is below the same figure of `before`.
    fn assert_monotone(before: &Profile, now: &Profile, subject: &str) {
        for kind in ActionKind::ALL {
            assert!(
                now.applied(kind) >= before.applied(kind),
                "{subject}: applied `{}` fell from {} to {}",
                kind.label(),
                before.applied(kind),
                now.applied(kind)
            );
        }
        assert!(
            now.rejected() >= before.rejected(),
            "{subject}: the rejected total fell"
        );
        assert!(
            now.crossings() >= before.crossings(),
            "{subject}: the crossings total fell"
        );
        assert!(
            now.killed() >= before.killed(),
            "{subject}: the killed total fell"
        );
        assert!(
            now.opportunities() >= before.opportunities(),
            "{subject}: the decision-opportunity count fell"
        );
    }

    /// O1's two counted tables and their difference, printed so a retained transcript of
    /// `cargo test -- --nocapture` **is** the comparison rather than a summary of one already made.
    ///
    /// Columns are [`ActionKind::ALL`] in order, then the rejected, crossing, kill and decision
    /// totals. The `difference` row is signed and is all zeros wherever the assertions hold, which
    /// is what makes the retained table checkable without rerunning the assertions.
    fn print_tables(seed: &str, observer: &Observer, oracle: &BTreeMap<String, Counted>) {
        let heading: Vec<&str> = ActionKind::ALL
            .into_iter()
            .map(ActionKind::label)
            .chain(["rejected", "crossings", "killed", "decisions"])
            .collect();
        println!("--- seed {seed}: tick {} ---", observer.snapshot().tick);
        println!(
            "{:<14}{:<12}{}",
            "subject",
            "table",
            heading
                .iter()
                .map(|label| format!("{label:>11}"))
                .collect::<String>()
        );
        for (id, expected) in oracle {
            let profile = observer.profile_of(id).expect("a retained record");
            let accumulated: Vec<u64> = ActionKind::ALL
                .into_iter()
                .map(|kind| profile.applied(kind))
                .chain([
                    profile.rejected(),
                    profile.crossings(),
                    profile.killed(),
                    profile.opportunities(),
                ])
                .collect();
            let counted: Vec<u64> = ActionKind::ALL
                .into_iter()
                .map(|kind| expected.verb(kind.label()))
                .chain([
                    expected.rejected,
                    expected.crossings,
                    expected.killed,
                    expected.opportunities,
                ])
                .collect();
            for (table, figures) in [("accumulated", &accumulated), ("counted", &counted)] {
                println!(
                    "{id:<14}{table:<12}{}",
                    figures
                        .iter()
                        .map(|figure| format!("{figure:>11}"))
                        .collect::<String>()
                );
            }
            println!(
                "{:<14}{:<12}{}",
                "",
                "difference",
                accumulated
                    .iter()
                    .zip(&counted)
                    .map(|(left, right)| format!("{:>11}", i128::from(*left) - i128::from(*right)))
                    .collect::<String>()
            );
        }
        // P4 and the contract's *Memory* item: the retained state, reported beside the tick it was
        // reached at, so a reviewer can read boundedness off two captures of different lengths
        // rather than take a claim about it. The map's own node overhead is not counted, which is
        // why the figure is a lower bound and is labelled one.
        let identifiers: usize = observer.profiles.keys().map(String::len).sum();
        println!(
            "retained state at tick {}: {} records of {} bytes plus {} bytes of identifiers, at \
             least {} bytes, one record per initialized Mokiterion",
            observer.snapshot().tick,
            observer.profiles.len(),
            size_of::<Profile>(),
            identifiers,
            observer.profiles.len() * size_of::<Profile>() + identifiers
        );
        println!("--- end seed {seed} ---");
    }

    /// The integer the pane presents under `label`, or `None` where the label heads no figure.
    ///
    /// Matched on words, so a label is a word and not a substring: the decision record above the
    /// totals carries the same verbs, but there a verb is followed by a target, by a payload or by
    /// nothing and never by a bare integer, so the first place a label is followed by one is its
    /// own figure.
    ///
    /// A word is a run of ASCII alphanumerics, which is what the pane's own text is made of. Every
    /// other byte separates — whitespace, but also the block's border glyph, which is adjacent to
    /// the first label of every row and would otherwise be read as part of it.
    fn words(text: &str) -> Vec<&str> {
        text.split(|character: char| !character.is_ascii_alphanumeric())
            .filter(|word| !word.is_empty())
            .collect()
    }

    fn presented(text: &str, label: &str) -> Option<u64> {
        words(text)
            .windows(2)
            .find(|pair| pair[0] == label && pair[1].parse::<u64>().is_ok())
            .map(|pair| pair[1].parse().expect("checked in the predicate"))
    }

    /// O1. The comparison is made after **every** completed tick, which subsumes the matrix's
    /// "the final tick and three intermediate ticks".
    ///
    /// The social source is the one whose runs reach attacks, fights, kills and deaths, so the
    /// fourteen totals are compared against counts that are largely non-zero rather than against a
    /// field of zeros that any broken accumulator would also produce.
    #[test]
    fn every_total_equals_an_independent_count_on_every_declared_seed() {
        for seed in DECLARED_SEEDS {
            let seed = seed.to_string();
            let (observer, oracle) = in_lockstep(
                &[
                    "--policy",
                    "social",
                    "--seed",
                    &seed,
                    "--ticks",
                    "200",
                    "--start-paused",
                ],
                assert_totals_match,
            );
            assert!(
                observer.snapshot().tick > 0,
                "seed {seed} completed no tick, so nothing was compared"
            );
            assert!(
                oracle.values().any(|counted| counted.opportunities > 0),
                "seed {seed} produced no decision to count"
            );
            print_tables(&seed, &observer, &oracle);
        }
    }

    /// O2. Independence clause 2: where the engine states a fact twice, the total is checked
    /// against the record the observer did not read for it.
    #[test]
    fn each_total_the_engine_states_twice_agrees_with_its_second_record() {
        let (observer, oracle) = in_lockstep(
            &[
                "--policy",
                "social",
                "--seed",
                "123",
                "--ticks",
                "300",
                "--start-paused",
            ],
            |observer, oracle| {
                let tick = observer.snapshot().tick;
                for (id, counted) in oracle {
                    let profile = observer.profile_of(id).expect("a retained record");
                    assert_eq!(
                        profile.applied(ActionKind::Eat),
                        counted.consumed,
                        "{id}'s applied `eat` against its `food_consumed` records at tick {tick}"
                    );
                    assert_eq!(
                        profile.applied(ActionKind::Attack) + profile.applied(ActionKind::Fight),
                        counted.strikes,
                        "{id}'s applied strikes against its `attack_resolved` records at tick \
                         {tick}"
                    );
                    assert_eq!(
                        profile.crossings(),
                        counted.crossings,
                        "{id}'s crossings against its `territory_crossed` records at tick {tick}"
                    );
                }
            },
        );
        let population = observer.population_profile();
        assert!(
            population.applied(ActionKind::Eat) > 0,
            "no `eat` resolved, so the `food_consumed` cross-check compared zero to zero"
        );
        assert!(
            oracle.values().map(|counted| counted.strikes).sum::<u64>() > 0,
            "no strike resolved, so the `attack_resolved` cross-check compared zero to zero"
        );
        assert!(
            population.crossings() > 0,
            "no boundary was crossed, so the `territory_crossed` cross-check compared zero to zero"
        );
    }

    /// O3, over all four decision sources: the eleven verb totals plus the rejected total account
    /// for every decision opportunity, for every Mokiterion, at every completed tick.
    ///
    /// This is `REQ-MOK-061` clause 2's identity, and residual uncertainty 3 records that it rests
    /// on the engine stating a decision for every Mokiterion on every completed tick. Asserting it
    /// on every tick of four sources is how that assumption is exposed rather than trusted.
    #[test]
    fn the_verb_totals_and_the_rejections_account_for_every_opportunity() {
        for policy in ["baseline", "reference", "individual", "social"] {
            in_lockstep(
                &[
                    "--policy",
                    policy,
                    "--seed",
                    "42",
                    "--ticks",
                    "150",
                    "--start-paused",
                ],
                |observer, oracle| {
                    let tick = observer.snapshot().tick;
                    for id in oracle.keys() {
                        let profile = observer.profile_of(id).expect("a retained record");
                        let applied: u64 = ActionKind::ALL
                            .iter()
                            .map(|kind| profile.applied(*kind))
                            .sum();
                        assert_eq!(
                            applied + profile.rejected(),
                            profile.opportunities(),
                            "{id}'s verbs and rejections do not account for its opportunities at \
                             tick {tick} under `{policy}`"
                        );
                    }
                },
            );
        }
    }

    /// O4: the killed total is the engine's own fatal strikes, on a run in which both `attack` and
    /// `fight` occur — so a total counted from only one of the two verbs fails here.
    #[test]
    fn the_killed_total_is_the_engine_s_fatal_strikes_where_both_verbs_occur() {
        let (observer, _) = in_lockstep(
            &[
                "--policy",
                "social",
                "--seed",
                "123",
                "--ticks",
                "300",
                "--start-paused",
            ],
            |observer, oracle| {
                let tick = observer.snapshot().tick;
                for (id, counted) in oracle {
                    let profile = observer.profile_of(id).expect("a retained record");
                    assert_eq!(
                        profile.killed(),
                        counted.killed,
                        "{id}'s killed total at tick {tick}"
                    );
                }
            },
        );
        let population = observer.population_profile();
        assert!(
            population.applied(ActionKind::Attack) > 0,
            "the run reached no `attack`"
        );
        assert!(
            population.applied(ActionKind::Fight) > 0,
            "the run reached no `fight`, so O4's stated case was not reached"
        );
        assert!(
            population.killed() > 0,
            "the run reached no fatal strike, so the killed total compared zero to zero"
        );
    }

    /// O5: a dead Mokiterion's totals are frozen at the tick of its death and its record is never
    /// removed.
    ///
    /// The totals are captured at the tick the death appears, which is after that tick's own
    /// decision has been accumulated: a Mokiterion decides before it dies, and its last decision
    /// belongs to it.
    #[test]
    fn a_dead_mokiterion_s_totals_stop_moving_and_are_never_removed() {
        let mut frozen: BTreeMap<String, (u64, Profile)> = BTreeMap::new();
        let (observer, _) = in_lockstep(
            &[
                "--policy",
                "social",
                "--seed",
                "123",
                "--ticks",
                "400",
                "--start-paused",
            ],
            |observer, _| {
                let tick = observer.snapshot().tick;
                for (id, (died, totals)) in &frozen {
                    let now = observer.profile_of(id).unwrap_or_else(|| {
                        panic!("{id}'s retained record was removed by tick {tick}")
                    });
                    assert_eq!(
                        now, totals,
                        "{id} died on tick {died} and its totals moved by tick {tick}"
                    );
                }
                let newly: Vec<String> = observer
                    .deaths()
                    .iter()
                    .map(|death| death.id.clone())
                    .filter(|id| !frozen.contains_key(id))
                    .collect();
                for id in newly {
                    let totals = observer.profile_of(&id).expect("a retained record").clone();
                    frozen.insert(id, (tick, totals));
                }
            },
        );
        assert!(
            !frozen.is_empty(),
            "the run reached no death, so nothing was frozen"
        );
        for id in frozen.keys() {
            assert!(
                observer.profile_of(id).is_some(),
                "{id}'s record is absent at the end of the run"
            );
        }
    }

    /// O6: a zero total is a measurement and is presented; a value the engine does not compute
    /// stays absent from the pane entirely.
    ///
    /// The reference source proposes no targeted verb — `SPEC-MOK-001` rule 26 makes `social` the
    /// only source that proposes one — so `attack` is a measured zero here rather than a
    /// coincidence of the seed.
    #[test]
    fn a_kind_that_never_happened_presents_a_measured_zero_while_an_uncomputed_value_stays_absent()
    {
        let mut observer =
            to_the_end(&["--policy", "reference", "--ticks", "30", "--start-paused"]);
        observer.select_for_test("M01");
        let profile = observer.profile_of("M01").expect("a retained record");
        assert_eq!(
            profile.applied(ActionKind::Attack),
            0,
            "the reference source proposed an attack, which rule 26 says it cannot"
        );
        assert!(
            profile.opportunities() > 0,
            "nothing was measured, so a zero would not be a measurement"
        );

        let inspector = inspector_text(&mut observer, 160, 48);
        assert_eq!(
            presented(&inspector, "attack"),
            Some(0),
            "a measured zero is not presented as a figure:\n{inspector}"
        );
        // Rule 10.7's standing prohibition, on two of the values the amended item still names.
        // Matched as words rather than as substrings, because `damage` contains one of them.
        let present = words(&inspector);
        for withheld in ["age", "entropy"] {
            assert!(
                !present.contains(&withheld),
                "`{withheld}` reached the pane, and the engine computes no such value:\n{inspector}"
            );
        }
    }

    /// O7: the totals are read from the decision records, so `--trace-actions` cannot move them.
    #[test]
    fn no_total_moves_with_the_action_trace_flag() {
        let plain = to_the_end(&[
            "--policy",
            "social",
            "--seed",
            "42",
            "--ticks",
            "120",
            "--start-paused",
        ]);
        let traced = to_the_end(&[
            "--policy",
            "social",
            "--seed",
            "42",
            "--ticks",
            "120",
            "--start-paused",
            "--trace-actions",
        ]);

        assert_eq!(plain.initialized_count(), traced.initialized_count());
        assert_eq!(
            plain.population_profile(),
            traced.population_profile(),
            "the population totals move with the trace flag"
        );
        let ids = every_identifier(&plain);
        assert_eq!(ids, every_identifier(&traced));
        for id in &ids {
            assert_eq!(
                plain.profile_of(id),
                traced.profile_of(id),
                "{id}'s totals move with the trace flag"
            );
        }
    }

    /// O8 and P4: the totals are unaffected by the retained event buffer dropping its oldest
    /// record, because no total is ever computed from that buffer.
    ///
    /// The run is executed rather than reasoned about, which is what the contract's *long run*
    /// check requires. `individual` at this length is the shortest declared configuration that
    /// reaches `EVENT_CAPACITY`: it fills the buffer at tick 6476 and then runs on. The comparison
    /// is made every hundredth tick past the boundary and once at the end, rather than on every
    /// tick, because the point of this case is its length.
    #[test]
    fn totals_survive_the_event_buffer_dropping_its_oldest_record() {
        let (observer, oracle) = in_lockstep(
            &[
                "--policy",
                "individual",
                "--ticks",
                "6600",
                "--start-paused",
            ],
            |observer, oracle| {
                if observer.events().truncated() && observer.snapshot().tick.is_multiple_of(100) {
                    assert_totals_match(observer, oracle);
                }
            },
        );
        assert!(
            observer.events().truncated(),
            "the run retained only {} records, so O8's case was never reached",
            observer.events().len()
        );
        assert_eq!(observer.events().len(), EVENT_CAPACITY);
        assert_totals_match(&observer, &oracle);
        println!(
            "retained events {} of capacity {EVENT_CAPACITY}, truncated {}, tick {}",
            observer.events().len(),
            observer.events().truncated(),
            observer.snapshot().tick
        );
        print_tables("individual, 6600 ticks", &observer, &oracle);

        // P4: the retained state is one record per initialized Mokiterion after a run long enough
        // to have dropped tens of thousands of records, and does not grow with ticks.
        assert_eq!(
            observer.initialized_count(),
            every_identifier(&observer).len(),
            "the retained record count is not the initialized count after a long run"
        );
    }

    /// O9 and P2: every population total is the sum over the Mokiterions, at every completed tick
    /// and not only at the end, in a run with at least one death.
    ///
    /// The sum is taken over the *oracle's* counts rather than over the observer's records, so the
    /// summation under test is not used to produce its own expected value.
    #[test]
    fn every_population_total_is_the_independent_sum_at_every_tick() {
        let (observer, oracle) = in_lockstep(
            &[
                "--policy",
                "social",
                "--seed",
                "123",
                "--ticks",
                "300",
                "--start-paused",
            ],
            |observer, oracle| {
                let tick = observer.snapshot().tick;
                let population = observer.population_profile();
                for kind in ActionKind::ALL {
                    let expected: u64 = oracle
                        .values()
                        .map(|counted| counted.verb(kind.label()))
                        .sum();
                    assert_eq!(
                        population.applied(kind),
                        expected,
                        "the population's applied `{}` at tick {tick}",
                        kind.label()
                    );
                }
                assert_eq!(
                    population.rejected(),
                    oracle.values().map(|counted| counted.rejected).sum::<u64>(),
                    "the population's rejected total at tick {tick}"
                );
                assert_eq!(
                    population.crossings(),
                    oracle
                        .values()
                        .map(|counted| counted.crossings)
                        .sum::<u64>(),
                    "the population's crossings total at tick {tick}"
                );
                assert_eq!(
                    population.killed(),
                    oracle.values().map(|counted| counted.killed).sum::<u64>(),
                    "the population's killed total at tick {tick}"
                );
                assert_eq!(
                    population.opportunities(),
                    oracle
                        .values()
                        .map(|counted| counted.opportunities)
                        .sum::<u64>(),
                    "the population's decision-opportunity count at tick {tick}"
                );
            },
        );
        assert!(
            observer.snapshot().deaths > 0,
            "the run reached no death, so the dead members were never in the sum"
        );
        assert_eq!(oracle.len(), observer.initialized_count());
    }

    /// O10 and P1: no total ever decreases, for any Mokiterion or for the population, on any
    /// declared seed. A dead member leaving the sum would show here as a fall.
    #[test]
    fn no_total_ever_decreases_on_any_declared_seed() {
        for seed in DECLARED_SEEDS {
            let seed = seed.to_string();
            let mut previous: BTreeMap<String, Profile> = BTreeMap::new();
            let mut previous_population = Profile::default();
            let (observer, _) = in_lockstep(
                &[
                    "--policy",
                    "social",
                    "--seed",
                    &seed,
                    "--ticks",
                    "200",
                    "--start-paused",
                ],
                |observer, oracle| {
                    let tick = observer.snapshot().tick;
                    for id in oracle.keys() {
                        let now = observer.profile_of(id).expect("a retained record");
                        let before = previous.get(id).cloned().unwrap_or_default();
                        assert_monotone(&before, now, &format!("seed {seed}, {id} at tick {tick}"));
                        previous.insert(id.clone(), now.clone());
                    }
                    let population = observer.population_profile();
                    assert_monotone(
                        &previous_population,
                        &population,
                        &format!("seed {seed}, the population at tick {tick}"),
                    );
                    previous_population = population;
                },
            );
            assert!(
                observer.snapshot().deaths > 0,
                "seed {seed} reached no death, so retention across a death was not exercised"
            );
        }
    }

    /// O11: the tick, the living count and the death count the pane presents are the engine's own
    /// snapshot values, and the initialized count is the number of Mokiterions it named.
    #[test]
    fn the_population_pane_states_the_engine_s_own_tick_living_and_death_counts() {
        let mut observer = to_the_end(&[
            "--policy",
            "social",
            "--seed",
            "123",
            "--ticks",
            "40",
            "--start-paused",
        ]);
        assert!(
            observer.selection().is_none(),
            "the population state needs no operator act to reach from a fresh run"
        );
        let tick = observer.snapshot().tick;
        let living = u64::try_from(observer.snapshot().living_count).unwrap();
        let deaths = u64::try_from(observer.snapshot().deaths).unwrap();
        let initialized = observer.initialized_count();
        assert_eq!(
            initialized,
            every_identifier(&observer).len(),
            "the initialized count is not the number of Mokiterions the engine named"
        );

        let inspector = inspector_text(&mut observer, 160, 48);
        assert_eq!(presented(&inspector, "tick"), Some(tick), "{inspector}");
        assert_eq!(presented(&inspector, "living"), Some(living), "{inspector}");
        assert_eq!(presented(&inspector, "deaths"), Some(deaths), "{inspector}");
        assert_eq!(
            presented(&inspector, "initialized"),
            Some(u64::try_from(initialized).unwrap()),
            "{inspector}"
        );
        assert!(
            deaths > 0,
            "the run reached no death, so the split was zero"
        );
    }

    /// O12: the deaths attributed to a strike are the population's killed total, and the remainder
    /// is the engine's death count less that total — never negative, and carrying no cause,
    /// because the engine attributed none.
    #[test]
    fn the_death_split_accounts_for_every_death_and_names_no_cause_it_cannot() {
        let mut observer = to_the_end(&[
            "--policy",
            "social",
            "--seed",
            "123",
            "--ticks",
            "400",
            "--start-paused",
        ]);
        let population = observer.population_profile();
        let deaths = u64::try_from(observer.snapshot().deaths).unwrap();
        assert!(deaths > 0, "the run reached no death");
        assert!(
            population.killed() > 0,
            "the run reached no fatal strike, so the split had only one side"
        );
        assert!(
            population.killed() <= deaths,
            "more deaths were attributed to a strike ({}) than the engine reported ({deaths})",
            population.killed()
        );

        let inspector = inspector_text(&mut observer, 160, 48);
        assert_eq!(
            presented(&inspector, "strike"),
            Some(population.killed()),
            "{inspector}"
        );
        assert_eq!(
            presented(&inspector, "unattributed"),
            Some(deaths - population.killed()),
            "{inspector}"
        );
    }

    /// O14 and acceptance scenario 5: extinction clears the selection itself, and the final frame
    /// presents the population's completed totals with no operator act.
    #[test]
    fn extinction_clears_the_selection_and_presents_the_completed_totals_unprompted() {
        let mut observer = start(&["--policy", "baseline", "--ticks", "400", "--start-paused"]);
        observer.advance().unwrap();
        observer.select_for_test("M01");
        assert_eq!(observer.selection(), Some("M01"));
        while !observer.is_finished() {
            observer.advance().unwrap();
        }
        assert_eq!(
            observer.termination_reason(),
            Some(TerminationReason::Extinction)
        );
        assert_eq!(
            observer.selection(),
            None,
            "the selection outlived the population, and no operator act cleared it"
        );
        assert_eq!(observer.snapshot().living_count, 0);

        let inspector = inspector_text(&mut observer, 160, 48);
        println!("--- inspector interior: 160x48, extinction ---\n{inspector}\n--- end ---");
        assert!(inspector.contains("nothing selected"), "{inspector}");
        assert!(inspector.contains("population activity"), "{inspector}");
        assert_eq!(presented(&inspector, "living"), Some(0), "{inspector}");
        let decisions = observer.population_profile().opportunities();
        assert!(decisions > 0, "the run completed no decision");
        assert_eq!(
            presented(&inspector, "decisions"),
            Some(decisions),
            "the completed totals were withheld because the run ended:\n{inspector}"
        );
    }

    /// O15: before the first completed tick, both selection states state that no tick has
    /// completed and present no figure.
    #[test]
    fn before_the_first_completed_tick_the_pane_states_so_and_presents_no_figure() {
        let mut observer = start(&["--start-paused"]);
        assert_eq!(observer.snapshot().tick, 0);

        // Not fifteen zeros: rule 10 makes a zero a measurement, and nothing has been measured.
        let withheld = [
            "wait",
            "move",
            "attack",
            "rejected",
            "crossings",
            "killed",
            "decisions",
            "tick",
            "living",
            "initialized",
            "deaths",
        ];

        let nothing = inspector_text(&mut observer, 160, 48);
        assert!(nothing.contains("nothing selected"), "{nothing}");
        assert!(nothing.contains("no tick has completed"), "{nothing}");
        for label in withheld {
            assert!(
                presented(&nothing, label).is_none(),
                "`{label}` heads a figure before any tick completed:\n{nothing}"
            );
        }

        observer.select_for_test("M01");
        let selected = inspector_text(&mut observer, 160, 48);
        assert!(selected.contains("no tick has completed"), "{selected}");
        for label in withheld {
            assert!(
                presented(&selected, label).is_none(),
                "`{label}` heads a figure before any tick completed:\n{selected}"
            );
        }
    }

    /// P3: every one of the eleven kinds of the closed contract has a total on the pane, in both
    /// selection states.
    ///
    /// The list is read from [`ActionKind::ALL`], which is the array the counts are indexed by, so
    /// a twelfth kind that reached the enum without reaching the pane fails here. The discriminant
    /// order is asserted against the array's order in the same case, because the two being one
    /// order rather than two that agree is what makes the index safe.
    #[test]
    fn every_kind_of_the_action_contract_reaches_the_pane_under_its_own_label() {
        assert_eq!(ActionKind::ALL.len(), ActionKind::COUNT);
        for (index, kind) in ActionKind::ALL.iter().enumerate() {
            assert_eq!(
                index, *kind as usize,
                "`ALL`'s order is not the discriminant order that indexes the counts"
            );
        }

        let mut observer = to_the_end(&[
            "--policy",
            "social",
            "--seed",
            "123",
            "--ticks",
            "40",
            "--start-paused",
        ]);
        let nothing = inspector_text(&mut observer, 160, 48);
        for kind in ActionKind::ALL {
            assert!(
                presented(&nothing, kind.label()).is_some(),
                "`{}` heads no figure with nothing selected:\n{nothing}",
                kind.label()
            );
        }

        observer.select_for_test("M01");
        let selected = inspector_text(&mut observer, 160, 48);
        for kind in ActionKind::ALL {
            assert!(
                presented(&selected, kind.label()).is_some(),
                "`{}` heads no figure for the selected Mokiterion:\n{selected}",
                kind.label()
            );
        }
    }

    /// Acceptance scenario 1: two decision sources produce profiles that differ, in the direction
    /// their own specification states.
    ///
    /// The direction is not inferred from the arithmetic. `SPEC-MOK-001` rule 26 and its *Event
    /// vocabulary* both state that `social` is the only source that proposes a targeted verb and
    /// that no targeted record ever appears under the other three, so the seven targeted totals
    /// separate the two sources and the four core verbs do not.
    #[test]
    fn two_decision_sources_produce_profiles_that_differ_in_the_stated_direction() {
        let profile_under = |policy: &str| {
            to_the_end(&[
                "--policy",
                policy,
                "--seed",
                "123",
                "--ticks",
                "120",
                "--start-paused",
            ])
            .population_profile()
        };
        let reference = profile_under("reference");
        let social = profile_under("social");
        assert_ne!(
            reference, social,
            "two sources produced indistinguishable profiles, which is the purpose of the pane \
             unmet"
        );

        let targeted = [
            ActionKind::Attack,
            ActionKind::Threaten,
            ActionKind::Fight,
            ActionKind::Retreat,
            ActionKind::Surrender,
            ActionKind::Approach,
            ActionKind::Avoid,
        ];
        let under_reference: u64 = targeted.iter().map(|kind| reference.applied(*kind)).sum();
        let under_social: u64 = targeted.iter().map(|kind| social.applied(*kind)).sum();
        assert_eq!(
            under_reference, 0,
            "a targeted verb was applied under `reference`, which rule 26 forbids"
        );
        assert!(
            under_social > under_reference,
            "`social` applied no more targeted verbs than `reference`"
        );
        assert!(
            social.killed() > 0 && reference.killed() == 0,
            "the killed totals do not separate the two sources as their sources' behaviour does"
        );
    }

    /// The inspector pane's rows at one viewport, joined.
    ///
    /// `tests/verification.rs` has its own copy of this projection. Restating it across the two
    /// tiers is what `SPEC-MOK-004` rule 9's split already requires, since an integration test
    /// links the crate's public interface and a helper inside the crate is not part of it.
    fn inspector_text(observer: &mut Observer, width: u16, height: u16) -> String {
        use ratatui::Terminal;
        use ratatui::backend::TestBackend;

        let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test backend");
        terminal
            .draw(|target| crate::render::draw(target, observer))
            .expect("drawing into a buffer");
        let buffer = terminal.backend().buffer().clone();
        let area = crate::layout::resolve(*buffer.area())
            .inspector
            .expect("this viewport presents the inspector");
        (area.y..area.y + area.height)
            .map(|y| {
                (area.x..area.x + area.width)
                    .map(|x| buffer.cell((x, y)).expect("inside the area").symbol())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
