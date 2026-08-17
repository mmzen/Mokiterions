use std::fmt;
use std::io::{self, Write};

const WORLD_SIZE: u8 = 128;
const TERRITORY_HEIGHT: u8 = 64;
const ATTRIBUTE_MAX: u8 = 100;
const REGENERATION_INTERVAL: u64 = 10;
const REGENERATION_YIELD: usize = 2;
const SATIETY_DECAY: u8 = 1;
const ENERGY_DECAY: u8 = 1;
const PERCEPTION_RADIUS: u8 = 16;
const REFERENCE_SLEEP_THRESHOLD: u8 = 20;

/// Cells in one territory. Density is expressed relative to this, not to the world.
const CELLS_PER_TERRITORY: usize = WORLD_SIZE as usize * TERRITORY_HEIGHT as usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub seed: u64,
    pub tick_limit: u64,
    pub policy: Policy,
    pub density: Density,
    pub trace_actions: bool,
}

/// Resource density as an exact count of hundredths of a percent of a territory's
/// cells. Held as an integer rather than a float because `REQ-MOK-009` requires
/// byte-identical reproducibility, and because the resolved resource count must not
/// depend on floating-point rounding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Density {
    hundredths_of_percent: u32,
}

impl Density {
    /// The default density, `0.75%`, which resolves to 61 resources per territory.
    /// `REQ-MOK-014` states a survivor floor of five here.
    pub const DEFAULT: Self = Self {
        hundredths_of_percent: 75,
    };

    /// Parses a percentage with at most two decimal places, such as `0.75` or `1.5`.
    /// Rejects anything that does not resolve to at least one resource per territory,
    /// because a territory at zero resources can never regenerate under rule 15.
    pub fn parse(value: &str) -> Result<Self, String> {
        let (whole, fraction) = match value.split_once('.') {
            Some((whole, fraction)) => (whole, fraction),
            None => (value, ""),
        };
        if whole.is_empty() && fraction.is_empty() {
            return Err("expected a percentage such as 0.75".into());
        }
        if fraction.len() > 2 {
            return Err("at most two decimal places are accepted".into());
        }
        let digits = |text: &str| {
            if text.chars().all(|character| character.is_ascii_digit()) {
                Ok(())
            } else {
                Err("expected digits and at most one decimal point".to_string())
            }
        };
        digits(whole)?;
        digits(fraction)?;

        let whole: u32 = if whole.is_empty() {
            0
        } else {
            whole
                .parse()
                .map_err(|_| "value is too large".to_string())?
        };
        // "5" means 50 hundredths, "05" means 5; pad so both scale correctly.
        let fraction: u32 = match fraction.len() {
            0 => 0,
            1 => fraction.parse::<u32>().unwrap() * 10,
            _ => fraction.parse::<u32>().unwrap(),
        };
        let hundredths_of_percent = whole
            .checked_mul(100)
            .and_then(|scaled| scaled.checked_add(fraction))
            .ok_or("value is too large")?;

        let density = Self {
            hundredths_of_percent,
        };
        if hundredths_of_percent > 10_000 {
            return Err("must not exceed 100".into());
        }
        if density.resources_per_territory() == 0 {
            return Err(format!(
                "resolves to zero resources per territory; the smallest usable density is {}",
                Self::SMALLEST_USABLE
            ));
        }
        Ok(density)
    }

    /// The smallest density that resolves to at least one resource, reported in the
    /// rejection message so the operator is told the usable floor rather than left to
    /// find it.
    const SMALLEST_USABLE: &'static str = "0.02";

    /// Resolves the density to a resource count using integer arithmetic that
    /// truncates toward zero, as `SPEC-MOK-001` requires. `0.15%` yields 12, `0.75%`
    /// yields 61, and `1.50%` yields 122.
    fn resources_per_territory(self) -> usize {
        self.hundredths_of_percent as usize * CELLS_PER_TERRITORY / 10_000
    }
}

impl Default for Density {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl fmt::Display for Density {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{:02}",
            self.hundredths_of_percent / 100,
            self.hundredths_of_percent % 100
        )
    }
}

/// The operator-selectable decision source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Policy {
    Baseline,
    #[default]
    Reference,
}

impl Policy {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "baseline" => Some(Self::Baseline),
            "reference" => Some(Self::Reference),
            _ => None,
        }
    }
}

impl fmt::Display for Policy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Baseline => formatter.write_str("baseline"),
            Self::Reference => formatter.write_str("reference"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Territory {
    A,
    B,
}

impl Territory {
    const ALL: [Self; 2] = [Self::A, Self::B];
}

impl fmt::Display for Territory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => formatter.write_str("A"),
            Self::B => formatter.write_str("B"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

impl Coordinate {
    fn territory(self) -> Territory {
        if self.y < TERRITORY_HEIGHT {
            Territory::A
        } else {
            Territory::B
        }
    }

    /// Chebyshev distance: the greater of the two absolute coordinate differences.
    fn distance_to(self, other: Self) -> u8 {
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }

    /// The eight-way relative direction of `other` as seen from `self`, or `None`
    /// when the two coordinates are identical.
    fn direction_to(self, other: Self) -> Option<RelativeDirection> {
        RelativeDirection::from_offsets(
            other.x as i16 - self.x as i16,
            other.y as i16 - self.y as i16,
        )
    }

    fn moved(self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::North if self.y > 0 => Some(Self {
                x: self.x,
                y: self.y - 1,
            }),
            Direction::East if self.x < WORLD_SIZE - 1 => Some(Self {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::South if self.y < WORLD_SIZE - 1 => Some(Self {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::West if self.x > 0 => Some(Self {
                x: self.x - 1,
                y: self.y,
            }),
            _ => None,
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const ORDERED: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];
}

impl fmt::Display for Direction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::North => formatter.write_str("north"),
            Self::East => formatter.write_str("east"),
            Self::South => formatter.write_str("south"),
            Self::West => formatter.write_str("west"),
        }
    }
}

/// One of the eight relative directions reported by perception. Unlike `Direction`,
/// which is a movable cardinal step, this only describes where something lies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RelativeDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl RelativeDirection {
    fn from_offsets(offset_x: i16, offset_y: i16) -> Option<Self> {
        match (offset_x.signum(), offset_y.signum()) {
            (0, 0) => None,
            (0, -1) => Some(Self::North),
            (1, -1) => Some(Self::NorthEast),
            (1, 0) => Some(Self::East),
            (1, 1) => Some(Self::SouthEast),
            (0, 1) => Some(Self::South),
            (-1, 1) => Some(Self::SouthWest),
            (-1, 0) => Some(Self::West),
            (-1, -1) => Some(Self::NorthWest),
            _ => unreachable!("signum yields only -1, 0, or 1"),
        }
    }

    /// The cardinal step that reduces the horizontal difference, if any.
    fn horizontal(self) -> Option<Direction> {
        match self {
            Self::East | Self::NorthEast | Self::SouthEast => Some(Direction::East),
            Self::West | Self::NorthWest | Self::SouthWest => Some(Direction::West),
            Self::North | Self::South => None,
        }
    }

    /// The cardinal step that reduces the vertical difference, if any.
    fn vertical(self) -> Option<Direction> {
        match self {
            Self::North | Self::NorthEast | Self::NorthWest => Some(Direction::North),
            Self::South | Self::SouthEast | Self::SouthWest => Some(Direction::South),
            Self::East | Self::West => None,
        }
    }
}

impl fmt::Display for RelativeDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::North => "north",
            Self::NorthEast => "north_east",
            Self::East => "east",
            Self::SouthEast => "south_east",
            Self::South => "south",
            Self::SouthWest => "south_west",
            Self::West => "west",
            Self::NorthWest => "north_west",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoodClass {
    Low,
    Medium,
    High,
}

impl FoodClass {
    const ALL: [Self; 3] = [Self::Low, Self::Medium, Self::High];

    fn restoration(self) -> (u8, u8) {
        match self {
            Self::Low => (15, 5),
            Self::Medium => (30, 10),
            Self::High => (50, 20),
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Low => 0,
            Self::Medium => 1,
            Self::High => 2,
        }
    }

    /// Higher is more attractive. Used only by decision-source preference, never by
    /// world rules.
    fn calorie_rank(self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Medium => 1,
            Self::High => 2,
        }
    }
}

impl fmt::Display for FoodClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => formatter.write_str("low"),
            Self::Medium => formatter.write_str("medium"),
            Self::High => formatter.write_str("high"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Food {
    id: String,
    position: Coordinate,
    class: FoodClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mokiterion {
    id: String,
    position: Coordinate,
    health: u8,
    satiety: u8,
    energy: u8,
    alive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Wait,
    Sleep,
    Eat { food_id: String },
    Move { direction: Direction },
}

impl fmt::Display for Action {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wait => formatter.write_str("wait"),
            Self::Sleep => formatter.write_str("sleep"),
            Self::Eat { food_id } => write!(formatter, "eat:{food_id}"),
            Self::Move { direction } => write!(formatter, "move:{direction}"),
        }
    }
}

/// A food resource as perceived from a distance. It carries no reference to the
/// resource itself, so a decision source can read it but cannot reach it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PerceivedFood {
    id: String,
    class: FoodClass,
    direction: Option<RelativeDirection>,
    distance: u8,
}

/// Another living Mokiterion as perceived from a distance. No requirement consumes
/// this yet; it settles the observation contract once.
#[derive(Debug, Clone, PartialEq, Eq)]
struct PerceivedMokiterion {
    id: String,
    direction: Option<RelativeDirection>,
    distance: u8,
}

#[derive(Debug, Clone)]
struct Observation {
    tick: u64,
    agent_id: String,
    position: Coordinate,
    territory: Territory,
    health: u8,
    satiety: u8,
    energy: u8,
    co_located_food: Vec<String>,
    perceived_food: Vec<PerceivedFood>,
    perceived_mokiterions: Vec<PerceivedMokiterion>,
    valid_actions: Vec<Action>,
}

impl Observation {
    fn is_consistent(&self) -> bool {
        self.tick > 0
            && !self.agent_id.is_empty()
            && self.position.territory() == self.territory
            && self.health <= ATTRIBUTE_MAX
            && self.satiety <= ATTRIBUTE_MAX
            && self.energy <= ATTRIBUTE_MAX
            && !self.valid_actions.is_empty()
            && self.co_located_food.iter().all(|food_id| {
                self.valid_actions.contains(&Action::Eat {
                    food_id: food_id.clone(),
                })
            })
            && self.perceived_food.iter().all(|food| {
                food.distance <= PERCEPTION_RADIUS
                    && (food.distance == 0) == food.direction.is_none()
                    && (food.distance > 0) == !self.co_located_food.contains(&food.id)
            })
            && self.perceived_mokiterions.iter().all(|other| {
                other.distance <= PERCEPTION_RADIUS
                    && other.id != self.agent_id
                    && (other.distance == 0) == other.direction.is_none()
            })
            && is_sorted_by_distance_then_id(
                self.perceived_food
                    .iter()
                    .map(|food| (food.distance, food.id.as_str())),
            )
            && is_sorted_by_distance_then_id(
                self.perceived_mokiterions
                    .iter()
                    .map(|other| (other.distance, other.id.as_str())),
            )
    }

    /// Whether consuming this resource would waste part of its satiety restoration.
    ///
    /// `REQ-MOK-015` requires consuming when consuming is not wasteful, and `SPEC-MOK-001`
    /// rule 5 applies this one test to both eating and approaching. A fixed satiety
    /// threshold encoded neither faithfully: it made satiety 51..=100 dead buffer that could
    /// never fund travel. Applying the test to eating alone left the other half of the
    /// defect standing, because a Mokiterion that declined the resource underfoot stepped
    /// off, perceived it again as the nearest resource at a distance greater than zero, and
    /// stepped back. Screening approach targets by the same rule is what closes that cycle.
    fn fits(&self, food: &PerceivedFood) -> bool {
        self.satiety.saturating_add(food.class.restoration().0) <= ATTRIBUTE_MAX
    }

    /// The co-located resource worth consuming: the richest one that fits, then lowest
    /// identifier.
    fn best_fitting_co_located_food(&self) -> Option<&PerceivedFood> {
        self.perceived_food
            .iter()
            .filter(|food| food.distance == 0)
            .filter(|food| self.fits(food))
            .max_by(|left, right| {
                left.class
                    .calorie_rank()
                    .cmp(&right.class.calorie_rank())
                    .then_with(|| right.id.cmp(&left.id))
            })
    }

    /// The most attractive perceived resource that is not already underfoot and would not be
    /// wasted on arrival: nearest first, then highest calorie class, then lowest identifier.
    /// When every perceived resource would be wasted there is nothing worth walking to, and
    /// the reference source falls through to a search step.
    fn best_fitting_distant_food(&self) -> Option<&PerceivedFood> {
        self.perceived_food
            .iter()
            .filter(|food| food.distance > 0)
            .filter(|food| self.fits(food))
            .min_by(|left, right| {
                left.distance
                    .cmp(&right.distance)
                    .then_with(|| right.class.calorie_rank().cmp(&left.class.calorie_rank()))
                    .then_with(|| left.id.cmp(&right.id))
            })
    }

    fn allows(&self, action: &Action) -> bool {
        self.valid_actions.contains(action)
    }

    fn valid_moves(&self) -> Vec<Direction> {
        self.valid_actions
            .iter()
            .filter_map(|action| match action {
                Action::Move { direction } => Some(*direction),
                _ => None,
            })
            .collect()
    }
}

fn is_sorted_by_distance_then_id<'a, I>(entries: I) -> bool
where
    I: Iterator<Item = (u8, &'a str)>,
{
    let mut previous: Option<(u8, &str)> = None;
    for entry in entries {
        if let Some(earlier) = previous
            && earlier > entry
        {
            return false;
        }
        previous = Some(entry);
    }
    true
}

/// The bounded entropy capability handed to a decision source. It exposes selection
/// and nothing else, so a source can resolve a choice it cannot derive from
/// perception without reaching any part of the engine.
struct DecisionEntropy<'a> {
    stream: &'a mut SplitMix64,
    draws: u32,
}

impl<'a> DecisionEntropy<'a> {
    fn new(stream: &'a mut SplitMix64) -> Self {
        Self { stream, draws: 0 }
    }

    fn choose_index(&mut self, upper_bound: usize) -> usize {
        self.draws += 1;
        self.stream.choose_index(upper_bound)
    }
}

trait DecisionSource {
    /// The name reported in `decision_source_selected`, so a run is never ambiguous
    /// about which policy produced it.
    fn name(&self) -> &str;

    fn decide(&mut self, observation: &Observation, entropy: &mut DecisionEntropy<'_>) -> Action;
}

/// Selects uniformly among the currently valid actions. It is not held to the
/// population viability floor and is expected to starve.
#[derive(Default)]
struct BaselineDecisionSource;

impl DecisionSource for BaselineDecisionSource {
    fn name(&self) -> &str {
        "baseline"
    }

    fn decide(&mut self, observation: &Observation, entropy: &mut DecisionEntropy<'_>) -> Action {
        debug_assert!(observation.is_consistent());
        let choice = entropy.choose_index(observation.valid_actions.len());
        observation
            .valid_actions
            .get(choice)
            .cloned()
            .unwrap_or(Action::Wait)
    }
}

/// A development instrument, not autonomous behavior. It seeks and consumes food so
/// that world viability can be measured against a competent policy, and it draws
/// entropy only for a search step.
#[derive(Default)]
struct ReferenceDecisionSource;

impl DecisionSource for ReferenceDecisionSource {
    fn name(&self) -> &str {
        "reference"
    }

    fn decide(&mut self, observation: &Observation, entropy: &mut DecisionEntropy<'_>) -> Action {
        debug_assert!(observation.is_consistent());

        if let Some(food) = observation.best_fitting_co_located_food() {
            let eat = Action::Eat {
                food_id: food.id.clone(),
            };
            if observation.allows(&eat) {
                return eat;
            }
        }

        if observation.energy < REFERENCE_SLEEP_THRESHOLD && observation.allows(&Action::Sleep) {
            return Action::Sleep;
        }

        if let Some(target) = observation.best_fitting_distant_food()
            && let Some(direction) = target.direction
        {
            let preferred = direction.horizontal().or_else(|| direction.vertical());
            let alternate = direction.vertical().or_else(|| direction.horizontal());
            for candidate in [preferred, alternate].into_iter().flatten() {
                let step = Action::Move {
                    direction: candidate,
                };
                if observation.allows(&step) {
                    return step;
                }
            }
        }

        let moves = observation.valid_moves();
        debug_assert!(
            !moves.is_empty(),
            "every in-bounds position has at least two valid cardinal moves"
        );
        let choice = entropy.choose_index(moves.len());
        Action::Move {
            direction: moves[choice],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^ (value >> 31)
    }

    fn choose_index(&mut self, upper_bound: usize) -> usize {
        assert!(upper_bound > 0, "cannot choose from an empty collection");
        let bound = upper_bound as u64;
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let value = self.next_u64();
            if value >= threshold {
                return (value % bound) as usize;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminationReason {
    TickLimit,
    Extinction,
}

impl fmt::Display for TerminationReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TickLimit => formatter.write_str("tick_limit"),
            Self::Extinction => formatter.write_str("extinction"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunSummary {
    reason: TerminationReason,
    ticks: u64,
    survivors: usize,
    deaths: usize,
    territory_a: usize,
    territory_b: usize,
    food_a: [usize; 3],
    food_b: [usize; 3],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ActionResult {
    accepted: bool,
    detail: String,
}

/// Why regeneration declined to place a resource. `SPEC-MOK-001` fixes both words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegenerationSkipReason {
    /// The territory holds no resource, so rule 15 can never restock it again.
    Depleted,
    /// The territory already stands at the capacity the run's density implies.
    Capacity,
}

impl fmt::Display for RegenerationSkipReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Depleted => formatter.write_str("depleted"),
            Self::Capacity => formatter.write_str("capacity"),
        }
    }
}

/// The event vocabulary `SPEC-MOK-001` fixes. Eleven core types plus `action_trace`.
///
/// The type is enumerated rather than left as a string so that a consumer filtering or
/// mapping by type cannot invent a type the engine does not emit, and so that adding a
/// type is a compile error everywhere it is handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventType {
    WorldInitialized,
    FoodInitialized,
    AgentInitialized,
    DecisionSourceSelected,
    SurvivalChanged,
    AgentDied,
    FoodConsumed,
    FoodRegenerated,
    FoodRegenerationSkipped,
    TerritoryCrossed,
    SimulationEnded,
    ActionTrace,
}

impl EventType {
    /// Every type, in a stable order. A consumer cycling a filter through the vocabulary
    /// uses this, so the vocabulary cannot drift out of step with what is emitted.
    pub const ALL: [Self; 12] = [
        Self::WorldInitialized,
        Self::FoodInitialized,
        Self::AgentInitialized,
        Self::DecisionSourceSelected,
        Self::SurvivalChanged,
        Self::AgentDied,
        Self::FoodConsumed,
        Self::FoodRegenerated,
        Self::FoodRegenerationSkipped,
        Self::TerritoryCrossed,
        Self::SimulationEnded,
        Self::ActionTrace,
    ];

    /// The `event=` field of the text record. These strings are fixed by `SPEC-MOK-001`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorldInitialized => "world_initialized",
            Self::FoodInitialized => "food_initialized",
            Self::AgentInitialized => "agent_initialized",
            Self::DecisionSourceSelected => "decision_source_selected",
            Self::SurvivalChanged => "survival_changed",
            Self::AgentDied => "agent_died",
            Self::FoodConsumed => "food_consumed",
            Self::FoodRegenerated => "food_regenerated",
            Self::FoodRegenerationSkipped => "food_regeneration_skipped",
            Self::TerritoryCrossed => "territory_crossed",
            Self::SimulationEnded => "simulation_ended",
            Self::ActionTrace => "action_trace",
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// The typed payload of an event. It renders to the `result=` field of the text record
/// and is the same value a host reads structurally, so the record and the structure can
/// never disagree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventDetail {
    WorldInitialized {
        width: u8,
        height: u8,
        territories: u8,
    },
    FoodInitialized {
        class: FoodClass,
        position: Coordinate,
        territory: Territory,
    },
    AgentInitialized {
        position: Coordinate,
        territory: Territory,
        health: u8,
        satiety: u8,
        energy: u8,
    },
    DecisionSourceSelected {
        source: String,
    },
    SurvivalChanged {
        health: (u8, u8),
        satiety: (u8, u8),
        energy: (u8, u8),
    },
    AgentDied {
        health: u8,
    },
    FoodConsumed {
        food: String,
        class: FoodClass,
        satiety: (u8, u8),
        energy: (u8, u8),
    },
    FoodRegenerated {
        food: String,
        class: FoodClass,
        position: Coordinate,
    },
    FoodRegenerationSkipped {
        reason: RegenerationSkipReason,
        count: usize,
    },
    TerritoryCrossed {
        from: Territory,
        to: Territory,
    },
    SimulationEnded {
        reason: TerminationReason,
    },
    ActionTrace {
        proposal: Action,
        accepted: bool,
        detail: String,
        position: Coordinate,
        territory: Territory,
        health: u8,
        satiety: u8,
        energy: u8,
    },
}

impl EventDetail {
    pub fn event_type(&self) -> EventType {
        match self {
            Self::WorldInitialized { .. } => EventType::WorldInitialized,
            Self::FoodInitialized { .. } => EventType::FoodInitialized,
            Self::AgentInitialized { .. } => EventType::AgentInitialized,
            Self::DecisionSourceSelected { .. } => EventType::DecisionSourceSelected,
            Self::SurvivalChanged { .. } => EventType::SurvivalChanged,
            Self::AgentDied { .. } => EventType::AgentDied,
            Self::FoodConsumed { .. } => EventType::FoodConsumed,
            Self::FoodRegenerated { .. } => EventType::FoodRegenerated,
            Self::FoodRegenerationSkipped { .. } => EventType::FoodRegenerationSkipped,
            Self::TerritoryCrossed { .. } => EventType::TerritoryCrossed,
            Self::SimulationEnded { .. } => EventType::SimulationEnded,
            Self::ActionTrace { .. } => EventType::ActionTrace,
        }
    }
}

impl fmt::Display for EventDetail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorldInitialized {
                width,
                height,
                territories,
            } => write!(
                formatter,
                "width:{width},height:{height},territories:{territories}"
            ),
            Self::FoodInitialized {
                class,
                position,
                territory,
            } => write!(
                formatter,
                "class:{class},position:{position},territory:{territory}"
            ),
            Self::AgentInitialized {
                position,
                territory,
                health,
                satiety,
                energy,
            } => write!(
                formatter,
                "position:{position},territory:{territory},health:{health},satiety:{satiety},energy:{energy}"
            ),
            Self::DecisionSourceSelected { source } => write!(formatter, "source:{source}"),
            Self::SurvivalChanged {
                health,
                satiety,
                energy,
            } => write!(
                formatter,
                "health:{}->{},satiety:{}->{},energy:{}->{}",
                health.0, health.1, satiety.0, satiety.1, energy.0, energy.1
            ),
            Self::AgentDied { health } => write!(formatter, "health:{health}"),
            Self::FoodConsumed {
                food,
                class,
                satiety,
                energy,
            } => write!(
                formatter,
                "food:{food},class:{class},satiety:{}->{},energy:{}->{}",
                satiety.0, satiety.1, energy.0, energy.1
            ),
            Self::FoodRegenerated {
                food,
                class,
                position,
            } => write!(formatter, "food:{food},class:{class},position:{position}"),
            Self::FoodRegenerationSkipped { reason, count } => {
                write!(formatter, "reason:{reason},count:{count}")
            }
            Self::TerritoryCrossed { from, to } => write!(formatter, "from:{from},to:{to}"),
            Self::SimulationEnded { reason } => write!(formatter, "reason:{reason}"),
            Self::ActionTrace {
                proposal,
                accepted,
                detail,
                position,
                territory,
                health,
                satiety,
                energy,
            } => write!(
                formatter,
                "proposal:{proposal},status:{},detail:{detail},position:{position},territory:{territory},health:{health},satiety:{satiety},energy:{energy}",
                if *accepted { "accepted" } else { "rejected" }
            ),
        }
    }
}

/// One authoritative event. `Display` renders the exact `REQ-MOK-010` record line, so a
/// host that writes events and a host that reads them structurally see one order and one
/// content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub tick: u64,
    pub subject: String,
    pub detail: EventDetail,
}

impl Event {
    fn new(tick: u64, subject: impl Into<String>, detail: EventDetail) -> Self {
        Self {
            tick,
            subject: subject.into(),
            detail,
        }
    }

    pub fn event_type(&self) -> EventType {
        self.detail.event_type()
    }
}

impl fmt::Display for Event {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "tick={} subject={} event={} result={}",
            self.tick,
            self.subject,
            self.detail.event_type(),
            self.detail
        )
    }
}

/// The engine's verdict on one proposed action, as `REQ-MOK-004` defines it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionOutcome {
    Accepted,
    /// The ground the engine stated. It is an expected outcome of the authority
    /// boundary, not a fault.
    Rejected {
        ground: String,
    },
}

/// One decision opportunity from the most recently completed tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionSnapshot {
    pub agent_id: String,
    pub proposed: Action,
    pub outcome: DecisionOutcome,
    /// The action the engine applied, absent when the proposal was rejected.
    pub applied: Option<Action>,
}

/// A territory's standing resources at a completed-tick boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerritorySnapshot {
    pub id: Territory,
    pub standing: usize,
    pub low: usize,
    pub medium: usize,
    pub high: usize,
    pub capacity: usize,
    /// `SPEC-MOK-001` rule 15 makes regeneration conditional on at least one standing
    /// resource, so a standing count of zero is irreversible rather than merely low.
    pub permanently_depleted: bool,
}

/// One living Mokiterion at a completed-tick boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSnapshot {
    pub id: String,
    pub position: Coordinate,
    pub territory: Territory,
    pub health: u8,
    pub satiety: u8,
    pub energy: u8,
    /// The action the engine applied on the most recently completed tick. Absent before
    /// tick 1 completes and when the proposal was rejected.
    pub applied_action: Option<Action>,
}

/// One standing resource at a completed-tick boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceSnapshot {
    pub id: String,
    pub position: Coordinate,
    pub territory: Territory,
    pub class: FoodClass,
}

/// A complete, owned picture of authoritative state at a completed-tick boundary.
///
/// Every field is an owned value. There is no reference into engine state, no shared
/// handle, no interior mutability, and no method that mutates, so holding a snapshot
/// cannot influence the run it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldSnapshot {
    pub tick: u64,
    pub living_count: usize,
    pub deaths: usize,
    pub territories: [TerritorySnapshot; 2],
    /// Living Mokiterions only, in ascending identifier order, which is the order
    /// `SPEC-MOK-001` processes them in.
    pub agents: Vec<AgentSnapshot>,
    /// Standing resources only, in the engine's stable order.
    pub resources: Vec<ResourceSnapshot>,
    /// The most recently completed tick's decision opportunities, ascending identifier.
    /// Empty before tick 1 completes.
    pub decisions: Vec<DecisionSnapshot>,
}

/// What one call to [`Simulation::advance_tick`] produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TickOutcome {
    /// The tick's authoritative events, in authoritative order.
    pub events: Vec<Event>,
    pub finished: bool,
    /// The engine's termination reason, present exactly when `finished`.
    pub reason: Option<TerminationReason>,
}

pub struct Simulation {
    config: Config,
    tick: u64,
    agents: Vec<Mokiterion>,
    foods: Vec<Food>,
    entropy: SplitMix64,
    next_food_id: u32,
    /// Set once the run has ended. `SPEC-MOK-002` rule 1.4 requires a further advance to
    /// be refused rather than to restart or to extend a finished run.
    outcome: Option<TerminationReason>,
    /// The most recently completed tick's decision records. Presentation reads them; no
    /// rule does.
    decisions: Vec<DecisionSnapshot>,
    /// Present only while [`Simulation::advance_tick`] is collecting. The text-stream host
    /// leaves it absent, so a long run retains nothing it does not need.
    collected_events: Option<Vec<Event>>,
}

impl Simulation {
    pub fn new(config: Config) -> Result<Self, String> {
        if config.tick_limit == 0 {
            return Err("tick limit must be greater than zero".into());
        }

        let per_territory = config.density.resources_per_territory();

        let mut entropy = SplitMix64::new(config.seed);
        let mut foods: Vec<Food> = Vec::with_capacity(per_territory * 2);
        let mut next_food_id = 1;

        for territory in Territory::ALL {
            // Classes cycle rather than being drawn, so the class mix is exactly balanced at
            // every density and only placement consumes entropy.
            for class in FoodClass::ALL.iter().copied().cycle().take(per_territory) {
                let occupied: Vec<Coordinate> = foods
                    .iter()
                    .filter(|food| food.position.territory() == territory)
                    .map(|food| food.position)
                    .collect();
                let position = choose_free_coordinate(territory, &occupied, &mut entropy)?;
                foods.push(Food {
                    id: food_id(next_food_id),
                    position,
                    class,
                });
                next_food_id += 1;
            }
        }

        let mut agents = Vec::with_capacity(12);
        let mut occupied_agent_positions = Vec::with_capacity(12);
        for number in 1..=12 {
            let territory = if number <= 6 {
                Territory::A
            } else {
                Territory::B
            };
            let position =
                choose_free_coordinate(territory, &occupied_agent_positions, &mut entropy)?;
            occupied_agent_positions.push(position);
            agents.push(Mokiterion {
                id: format!("M{number:02}"),
                position,
                health: ATTRIBUTE_MAX,
                satiety: ATTRIBUTE_MAX,
                energy: ATTRIBUTE_MAX,
                alive: true,
            });
        }

        Ok(Self {
            config,
            tick: 0,
            agents,
            foods,
            entropy,
            next_food_id,
            outcome: None,
            decisions: Vec::new(),
            collected_events: None,
        })
    }

    pub fn run<W: Write>(&mut self, output: &mut W) -> io::Result<RunSummary> {
        match self.config.policy {
            Policy::Baseline => {
                let mut source = BaselineDecisionSource;
                self.run_with_source(output, &mut source)
            }
            Policy::Reference => {
                let mut source = ReferenceDecisionSource;
                self.run_with_source(output, &mut source)
            }
        }
    }

    fn run_with_source<W: Write, D: DecisionSource>(
        &mut self,
        output: &mut W,
        decision_source: &mut D,
    ) -> io::Result<RunSummary> {
        if self.tick != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "a simulation instance can only be run once",
            ));
        }

        for event in self.entity_initialization_events() {
            self.emit(output, event)?;
        }
        let event = Event::new(
            0,
            "world",
            EventDetail::DecisionSourceSelected {
                source: decision_source.name().to_string(),
            },
        );
        self.emit(output, event)?;

        loop {
            if let Some(reason) = self.step(output, decision_source)? {
                let summary = self.summary(reason);
                self.emit_summary(output, &summary)?;
                return Ok(summary);
            }
        }
    }

    /// Advances the simulation exactly one tick and returns that tick's events.
    ///
    /// This is the only operation on this type that changes simulation state, and it takes
    /// no host data, so a host's whole influence over a run is *when* it calls this.
    /// Advancing a finished run is refused with no state change, as `SPEC-MOK-002` rule 1.4
    /// requires.
    ///
    /// The error is the engine's own: `SPEC-MOK-001` rule 15 finding no free cell for a
    /// resource it must place. `Simulation::new` reports its own failures the same way, and
    /// `ARCH-MOK-001` requires ordinary `Result` propagation rather than a panic.
    pub fn advance_tick(&mut self) -> Result<TickOutcome, String> {
        if let Some(reason) = self.outcome {
            return Ok(TickOutcome {
                events: Vec::new(),
                finished: true,
                reason: Some(reason),
            });
        }

        match self.config.policy {
            Policy::Baseline => {
                let mut source = BaselineDecisionSource;
                self.advance_tick_with_source(&mut source)
            }
            Policy::Reference => {
                let mut source = ReferenceDecisionSource;
                self.advance_tick_with_source(&mut source)
            }
        }
    }

    fn advance_tick_with_source<D: DecisionSource>(
        &mut self,
        decision_source: &mut D,
    ) -> Result<TickOutcome, String> {
        self.collected_events = Some(Vec::new());
        let stepped = self.step(&mut io::sink(), decision_source);
        let events = self.collected_events.take().unwrap_or_default();
        match stepped {
            Ok(reason) => Ok(TickOutcome {
                events,
                finished: reason.is_some(),
                reason,
            }),
            // `io::sink` cannot fail, so a step's only error is the engine's own.
            Err(error) => Err(error.to_string()),
        }
    }

    /// Whether the run has ended. A finished run refuses to advance and stays inspectable.
    pub fn is_finished(&self) -> bool {
        self.outcome.is_some()
    }

    /// The engine's termination reason, present exactly once the run has ended. A host
    /// presents this value rather than deriving its own verdict.
    pub fn termination_reason(&self) -> Option<TerminationReason> {
        self.outcome
    }

    /// The configuration this run was constructed with, defaults resolved. A defaulted
    /// value and an explicitly supplied one are indistinguishable here, which is what
    /// `SPEC-MOK-002` rule 8.1 requires of a host presenting provenance.
    pub fn configuration(&self) -> Config {
        self.config
    }

    /// An owned picture of authoritative state at the current completed-tick boundary.
    ///
    /// Every value is owned. Nothing here references engine state, so holding a snapshot
    /// for any length of time cannot influence the run it came from.
    pub fn snapshot(&self) -> WorldSnapshot {
        let agents: Vec<AgentSnapshot> = self
            .agents
            .iter()
            .filter(|agent| agent.alive)
            .map(|agent| AgentSnapshot {
                id: agent.id.clone(),
                position: agent.position,
                territory: agent.position.territory(),
                health: agent.health,
                satiety: agent.satiety,
                energy: agent.energy,
                applied_action: self
                    .decisions
                    .iter()
                    .find(|decision| decision.agent_id == agent.id)
                    .and_then(|decision| decision.applied.clone()),
            })
            .collect();

        let resources: Vec<ResourceSnapshot> = self
            .foods
            .iter()
            .map(|food| ResourceSnapshot {
                id: food.id.clone(),
                position: food.position,
                territory: food.position.territory(),
                class: food.class,
            })
            .collect();

        let living_count = agents.len();
        WorldSnapshot {
            tick: self.tick,
            living_count,
            deaths: self.agents.len() - living_count,
            territories: [
                self.territory_snapshot(Territory::A),
                self.territory_snapshot(Territory::B),
            ],
            agents,
            resources,
            decisions: self.decisions.clone(),
        }
    }

    fn territory_snapshot(&self, territory: Territory) -> TerritorySnapshot {
        let counts = self.food_counts(territory);
        let standing = counts[0] + counts[1] + counts[2];
        TerritorySnapshot {
            id: territory,
            standing,
            low: counts[0],
            medium: counts[1],
            high: counts[2],
            capacity: self.config.density.resources_per_territory(),
            permanently_depleted: standing == 0,
        }
    }

    /// The tick-0 events: the world, every resource, every Mokiterion, and the selected
    /// decision source, in the order `SPEC-MOK-001` fixes.
    ///
    /// A host that did not call [`Simulation::run`] obtains them here rather than by
    /// reading the text stream, so the two hosts present one vocabulary.
    pub fn initialization_events(&self) -> Vec<Event> {
        let source = match self.config.policy {
            Policy::Baseline => BaselineDecisionSource.name().to_string(),
            Policy::Reference => ReferenceDecisionSource.name().to_string(),
        };
        let mut events = self.entity_initialization_events();
        events.push(Event::new(
            0,
            "world",
            EventDetail::DecisionSourceSelected { source },
        ));
        events
    }

    fn entity_initialization_events(&self) -> Vec<Event> {
        let mut events = Vec::with_capacity(1 + self.foods.len() + self.agents.len());
        events.push(Event::new(
            0,
            "world",
            EventDetail::WorldInitialized {
                width: WORLD_SIZE,
                height: WORLD_SIZE,
                territories: Territory::ALL.len() as u8,
            },
        ));
        for food in &self.foods {
            events.push(Event::new(
                0,
                food.id.clone(),
                EventDetail::FoodInitialized {
                    class: food.class,
                    position: food.position,
                    territory: food.position.territory(),
                },
            ));
        }
        for agent in &self.agents {
            events.push(Event::new(
                0,
                agent.id.clone(),
                EventDetail::AgentInitialized {
                    position: agent.position,
                    territory: agent.position.territory(),
                    health: agent.health,
                    satiety: agent.satiety,
                    energy: agent.energy,
                },
            ));
        }
        events
    }

    /// One complete tick: every living Mokiterion's turn in `SPEC-MOK-001` order, any
    /// regeneration, and the termination check.
    ///
    /// Both hosts call this, so neither can apply a tick the other would not, and a run is
    /// the same sequence whether it was watched or not.
    fn step<W: Write, D: DecisionSource>(
        &mut self,
        output: &mut W,
        decision_source: &mut D,
    ) -> io::Result<Option<TerminationReason>> {
        self.tick += 1;
        self.run_tick(output, decision_source)?;

        let extinct = self.agents.iter().all(|agent| !agent.alive);
        let tick_limit_reached = self.tick >= self.config.tick_limit;
        if extinct || tick_limit_reached {
            let reason = if extinct {
                TerminationReason::Extinction
            } else {
                TerminationReason::TickLimit
            };
            self.outcome = Some(reason);
            let event = Event::new(self.tick, "world", EventDetail::SimulationEnded { reason });
            self.emit(output, event)?;
            return Ok(Some(reason));
        }
        Ok(None)
    }

    /// Writes one authoritative event to the host's sink and retains it when a host is
    /// collecting. Every event passes through here, which is why a collected `TickOutcome`
    /// and the `REQ-MOK-010` record cannot disagree about order or content.
    fn emit<W: Write>(&mut self, output: &mut W, event: Event) -> io::Result<()> {
        writeln!(output, "{event}")?;
        if let Some(collected) = &mut self.collected_events {
            collected.push(event);
        }
        Ok(())
    }

    fn run_tick<W: Write, D: DecisionSource>(
        &mut self,
        output: &mut W,
        decision_source: &mut D,
    ) -> io::Result<()> {
        self.decisions.clear();
        for agent_index in 0..self.agents.len() {
            if !self.agents[agent_index].alive {
                continue;
            }

            let observation = self.observation(agent_index);
            let proposal = {
                let mut entropy = DecisionEntropy::new(&mut self.entropy);
                decision_source.decide(&observation, &mut entropy)
            };
            let result = self.apply_action(output, agent_index, &proposal)?;
            self.decisions.push(DecisionSnapshot {
                agent_id: self.agents[agent_index].id.clone(),
                outcome: if result.accepted {
                    DecisionOutcome::Accepted
                } else {
                    DecisionOutcome::Rejected {
                        ground: result.detail.clone(),
                    }
                },
                applied: result.accepted.then(|| proposal.clone()),
                proposed: proposal.clone(),
            });

            if self.config.trace_actions {
                self.emit_action_trace(output, agent_index, &proposal, &result)?;
            }

            self.apply_survival(output, agent_index)?;
        }

        if self.tick.is_multiple_of(REGENERATION_INTERVAL) {
            for territory in Territory::ALL {
                self.regenerate_food(output, territory)?;
            }
        }
        Ok(())
    }

    fn observation(&self, agent_index: usize) -> Observation {
        let agent = &self.agents[agent_index];
        let mut co_located_food: Vec<String> = self
            .foods
            .iter()
            .filter(|food| food.position == agent.position)
            .map(|food| food.id.clone())
            .collect();
        co_located_food.sort();

        let mut perceived_food: Vec<PerceivedFood> = self
            .foods
            .iter()
            .filter(|food| agent.position.distance_to(food.position) <= PERCEPTION_RADIUS)
            .map(|food| PerceivedFood {
                id: food.id.clone(),
                class: food.class,
                direction: agent.position.direction_to(food.position),
                distance: agent.position.distance_to(food.position),
            })
            .collect();
        perceived_food.sort_by(|left, right| {
            left.distance
                .cmp(&right.distance)
                .then_with(|| left.id.cmp(&right.id))
        });

        let mut perceived_mokiterions: Vec<PerceivedMokiterion> = self
            .agents
            .iter()
            .enumerate()
            .filter(|(index, other)| *index != agent_index && other.alive)
            .filter_map(|(_, other)| {
                let distance = agent.position.distance_to(other.position);
                if distance > PERCEPTION_RADIUS {
                    return None;
                }
                Some(PerceivedMokiterion {
                    id: other.id.clone(),
                    direction: agent.position.direction_to(other.position),
                    distance,
                })
            })
            .collect();
        perceived_mokiterions.sort_by(|left, right| {
            left.distance
                .cmp(&right.distance)
                .then_with(|| left.id.cmp(&right.id))
        });

        let mut valid_actions = vec![Action::Wait];
        if agent.energy < ATTRIBUTE_MAX {
            valid_actions.push(Action::Sleep);
        }
        valid_actions.extend(
            co_located_food
                .iter()
                .cloned()
                .map(|food_id| Action::Eat { food_id }),
        );
        valid_actions.extend(
            Direction::ORDERED
                .into_iter()
                .filter(|direction| agent.position.moved(*direction).is_some())
                .map(|direction| Action::Move { direction }),
        );

        Observation {
            tick: self.tick,
            agent_id: agent.id.clone(),
            position: agent.position,
            territory: agent.position.territory(),
            health: agent.health,
            satiety: agent.satiety,
            energy: agent.energy,
            co_located_food,
            perceived_food,
            perceived_mokiterions,
            valid_actions,
        }
    }

    fn apply_action<W: Write>(
        &mut self,
        output: &mut W,
        agent_index: usize,
        action: &Action,
    ) -> io::Result<ActionResult> {
        if !self.agents[agent_index].alive {
            return Ok(ActionResult {
                accepted: false,
                detail: "agent_dead".into(),
            });
        }

        match action {
            Action::Wait => Ok(ActionResult {
                accepted: true,
                detail: "waited".into(),
            }),
            Action::Sleep => {
                let agent = &mut self.agents[agent_index];
                if agent.energy == ATTRIBUTE_MAX {
                    return Ok(ActionResult {
                        accepted: false,
                        detail: "energy_full".into(),
                    });
                }
                let before = agent.energy;
                agent.energy = agent.energy.saturating_add(20).min(ATTRIBUTE_MAX);
                Ok(ActionResult {
                    accepted: true,
                    detail: format!("energy:{before}->{}", agent.energy),
                })
            }
            Action::Move { direction } => {
                let current_position = self.agents[agent_index].position;
                let Some(destination) = current_position.moved(*direction) else {
                    return Ok(ActionResult {
                        accepted: false,
                        detail: "out_of_bounds".into(),
                    });
                };
                let previous_territory = current_position.territory();
                let current_territory = destination.territory();
                self.agents[agent_index].position = destination;
                if previous_territory != current_territory {
                    let event = Event::new(
                        self.tick,
                        self.agents[agent_index].id.clone(),
                        EventDetail::TerritoryCrossed {
                            from: previous_territory,
                            to: current_territory,
                        },
                    );
                    self.emit(output, event)?;
                }
                Ok(ActionResult {
                    accepted: true,
                    detail: format!("position:{destination}"),
                })
            }
            Action::Eat { food_id } => {
                let agent_position = self.agents[agent_index].position;
                let Some(food_index) = self
                    .foods
                    .iter()
                    .position(|food| food.id == *food_id && food.position == agent_position)
                else {
                    return Ok(ActionResult {
                        accepted: false,
                        detail: "food_unavailable".into(),
                    });
                };

                let food = self.foods.remove(food_index);
                let (satiety_restored, energy_restored) = food.class.restoration();
                let event = {
                    let agent = &mut self.agents[agent_index];
                    let previous_satiety = agent.satiety;
                    let previous_energy = agent.energy;
                    agent.satiety = agent
                        .satiety
                        .saturating_add(satiety_restored)
                        .min(ATTRIBUTE_MAX);
                    agent.energy = agent
                        .energy
                        .saturating_add(energy_restored)
                        .min(ATTRIBUTE_MAX);
                    Event::new(
                        self.tick,
                        agent.id.clone(),
                        EventDetail::FoodConsumed {
                            food: food.id.clone(),
                            class: food.class,
                            satiety: (previous_satiety, agent.satiety),
                            energy: (previous_energy, agent.energy),
                        },
                    )
                };
                self.emit(output, event)?;
                Ok(ActionResult {
                    accepted: true,
                    detail: format!("food:{};class:{}", food.id, food.class),
                })
            }
        }
    }

    fn emit_action_trace<W: Write>(
        &mut self,
        output: &mut W,
        agent_index: usize,
        action: &Action,
        result: &ActionResult,
    ) -> io::Result<()> {
        let event = {
            let agent = &self.agents[agent_index];
            Event::new(
                self.tick,
                agent.id.clone(),
                EventDetail::ActionTrace {
                    proposal: action.clone(),
                    accepted: result.accepted,
                    detail: result.detail.clone(),
                    position: agent.position,
                    territory: agent.position.territory(),
                    health: agent.health,
                    satiety: agent.satiety,
                    energy: agent.energy,
                },
            )
        };
        self.emit(output, event)
    }

    fn apply_survival<W: Write>(&mut self, output: &mut W, agent_index: usize) -> io::Result<()> {
        let (event, died) = {
            let agent = &mut self.agents[agent_index];
            let previous_health = agent.health;
            let previous_satiety = agent.satiety;
            let previous_energy = agent.energy;

            agent.satiety = agent.satiety.saturating_sub(SATIETY_DECAY);
            agent.energy = agent.energy.saturating_sub(ENERGY_DECAY);
            if agent.satiety == 0 || agent.energy == 0 {
                agent.health = agent.health.saturating_sub(5);
            }

            let event = Event::new(
                self.tick,
                agent.id.clone(),
                EventDetail::SurvivalChanged {
                    health: (previous_health, agent.health),
                    satiety: (previous_satiety, agent.satiety),
                    energy: (previous_energy, agent.energy),
                },
            );

            let died = agent.health == 0;
            if died {
                agent.alive = false;
            }
            (event, died)
        };
        self.emit(output, event)?;

        if died {
            let event = Event::new(
                self.tick,
                self.agents[agent_index].id.clone(),
                EventDetail::AgentDied { health: 0 },
            );
            self.emit(output, event)?;
        }
        Ok(())
    }

    fn regenerate_food<W: Write>(
        &mut self,
        output: &mut W,
        territory: Territory,
    ) -> io::Result<()> {
        let capacity = self.config.density.resources_per_territory();
        let current_count = self
            .foods
            .iter()
            .filter(|food| food.position.territory() == territory)
            .count();

        if current_count == 0 || current_count >= capacity {
            let reason = if current_count == 0 {
                RegenerationSkipReason::Depleted
            } else {
                RegenerationSkipReason::Capacity
            };
            let event = Event::new(
                self.tick,
                territory.to_string(),
                EventDetail::FoodRegenerationSkipped {
                    reason,
                    count: current_count,
                },
            );
            return self.emit(output, event);
        }

        // Add the specified yield, or only as many as the remaining capacity allows.
        let additions = REGENERATION_YIELD.min(capacity - current_count);
        for _ in 0..additions {
            let class = FoodClass::ALL[self.entropy.choose_index(FoodClass::ALL.len())];
            let occupied: Vec<Coordinate> = self
                .foods
                .iter()
                .filter(|food| food.position.territory() == territory)
                .map(|food| food.position)
                .collect();
            let position = choose_free_coordinate(territory, &occupied, &mut self.entropy)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
            let id = food_id(self.next_food_id);
            self.next_food_id += 1;
            self.foods.push(Food {
                id: id.clone(),
                position,
                class,
            });
            let event = Event::new(
                self.tick,
                territory.to_string(),
                EventDetail::FoodRegenerated {
                    food: id,
                    class,
                    position,
                },
            );
            self.emit(output, event)?;
        }
        Ok(())
    }

    fn summary(&self, reason: TerminationReason) -> RunSummary {
        let survivors = self.agents.iter().filter(|agent| agent.alive).count();
        let territory_a = self
            .agents
            .iter()
            .filter(|agent| agent.alive && agent.position.territory() == Territory::A)
            .count();
        let territory_b = survivors - territory_a;
        RunSummary {
            reason,
            ticks: self.tick,
            survivors,
            deaths: self.agents.len() - survivors,
            territory_a,
            territory_b,
            food_a: self.food_counts(Territory::A),
            food_b: self.food_counts(Territory::B),
        }
    }

    fn food_counts(&self, territory: Territory) -> [usize; 3] {
        let mut counts = [0; 3];
        for food in self
            .foods
            .iter()
            .filter(|food| food.position.territory() == territory)
        {
            counts[food.class.index()] += 1;
        }
        counts
    }

    fn emit_summary<W: Write>(&self, output: &mut W, summary: &RunSummary) -> io::Result<()> {
        writeln!(
            output,
            "summary reason={} ticks={} survivors={} deaths={} territory_a={} territory_b={} food_a_low={} food_a_medium={} food_a_high={} food_b_low={} food_b_medium={} food_b_high={}",
            summary.reason,
            summary.ticks,
            summary.survivors,
            summary.deaths,
            summary.territory_a,
            summary.territory_b,
            summary.food_a[0],
            summary.food_a[1],
            summary.food_a[2],
            summary.food_b[0],
            summary.food_b[1],
            summary.food_b[2]
        )
    }
}

fn choose_free_coordinate(
    territory: Territory,
    occupied: &[Coordinate],
    entropy: &mut SplitMix64,
) -> Result<Coordinate, String> {
    for _ in 0..(WORLD_SIZE as usize * WORLD_SIZE as usize) {
        let x = entropy.choose_index(WORLD_SIZE as usize) as u8;
        let local_y = entropy.choose_index(TERRITORY_HEIGHT as usize) as u8;
        let y = match territory {
            Territory::A => local_y,
            Territory::B => local_y + TERRITORY_HEIGHT,
        };
        let coordinate = Coordinate { x, y };
        if !occupied.contains(&coordinate) {
            return Ok(coordinate);
        }
    }
    Err(format!(
        "no free coordinate remains in territory {territory}"
    ))
}

fn food_id(number: u32) -> String {
    format!("F{number:04}")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    /// The historical helper. It selects the baseline source so that the foundation
    /// tests keep testing the source they were written against, at the default density
    /// so that they test the shipped configuration.
    fn config(seed: u64, tick_limit: u64, trace_actions: bool) -> Config {
        Config {
            seed,
            tick_limit,
            policy: Policy::Baseline,
            density: Density::DEFAULT,
            trace_actions,
        }
    }

    fn reference_config(seed: u64, tick_limit: u64, trace_actions: bool) -> Config {
        Config {
            seed,
            tick_limit,
            policy: Policy::Reference,
            density: Density::DEFAULT,
            trace_actions,
        }
    }

    fn config_at(seed: u64, tick_limit: u64, density: &str) -> Config {
        Config {
            density: Density::parse(density).unwrap(),
            ..reference_config(seed, tick_limit, false)
        }
    }

    /// The seed set declared by `VER-MOK-002`, fixed so that viability cannot be
    /// demonstrated on a favourable seed chosen after the fact.
    const DECLARED_SEEDS: [u64; 5] = [0, 1, 42, 123, 777];

    /// The declared density from `VER-MOK-002` paired with the survivor floor `REQ-MOK-014`
    /// states for it. One density is declared and it is the default, so the floor is verified
    /// at the scarce density the system ships with. Survivors are not monotonic in density
    /// and a floor may not be interpolated or extrapolated from this point; other densities
    /// are swept as evidence in `density-curve.md` and carry no obligation.
    const DECLARED_FLOORS: [(&str, usize); 1] = [("0.75", 8)];

    fn decide_once(simulation: &Simulation, agent_index: usize) -> (Action, u32) {
        let observation = simulation.observation(agent_index);
        let mut stream = simulation.entropy;
        let mut entropy = DecisionEntropy::new(&mut stream);
        let action = ReferenceDecisionSource.decide(&observation, &mut entropy);
        (action, entropy.draws)
    }

    fn state_snapshot(
        simulation: &Simulation,
    ) -> (u64, Vec<Mokiterion>, Vec<Food>, SplitMix64, u32) {
        (
            simulation.tick,
            simulation.agents.clone(),
            simulation.foods.clone(),
            simulation.entropy,
            simulation.next_food_id,
        )
    }

    fn output_without_action_traces(output: &[u8]) -> String {
        let output = String::from_utf8(output.to_vec()).unwrap();
        let mut filtered = String::new();
        for line in output.lines() {
            if !line.contains("event=action_trace") {
                filtered.push_str(line);
                filtered.push('\n');
            }
        }
        filtered
    }

    #[test]
    fn initial_world_population_and_food_match_the_contract() {
        let simulation = Simulation::new(config(42, 100, false)).unwrap();
        let per_territory = simulation.config.density.resources_per_territory();

        assert_eq!(simulation.agents.len(), 12);
        assert_eq!(simulation.foods.len(), per_territory * 2);

        let ids: HashSet<&str> = simulation
            .agents
            .iter()
            .map(|agent| agent.id.as_str())
            .collect();
        let positions: HashSet<Coordinate> = simulation
            .agents
            .iter()
            .map(|agent| agent.position)
            .collect();
        assert_eq!(ids.len(), 12);
        assert_eq!(positions.len(), 12);

        for (index, agent) in simulation.agents.iter().enumerate() {
            let expected_territory = if index < 6 {
                Territory::A
            } else {
                Territory::B
            };
            assert_eq!(agent.position.territory(), expected_territory);
            assert_eq!(agent.health, 100);
            assert_eq!(agent.satiety, 100);
            assert_eq!(agent.energy, 100);
            assert!(agent.alive);
        }

        // Classes cycle, so each territory holds the resolved count spread as evenly as
        // the count allows: no class differs from another by more than one resource.
        for territory in Territory::ALL {
            let counts: Vec<usize> = FoodClass::ALL
                .iter()
                .map(|class| {
                    simulation
                        .foods
                        .iter()
                        .filter(|food| {
                            food.position.territory() == territory && food.class == *class
                        })
                        .count()
                })
                .collect();
            assert_eq!(counts.iter().sum::<usize>(), per_territory);
            let smallest = counts.iter().min().unwrap();
            let largest = counts.iter().max().unwrap();
            assert!(largest - smallest <= 1, "class mix is uneven: {counts:?}");
        }

        let food_positions: HashSet<Coordinate> =
            simulation.foods.iter().map(|food| food.position).collect();
        assert_eq!(food_positions.len(), per_territory * 2);
    }

    #[test]
    fn initialization_is_seeded_and_reproducible() {
        let first = Simulation::new(config(5, 100, false)).unwrap();
        let second = Simulation::new(config(5, 100, false)).unwrap();
        let different = Simulation::new(config(6, 100, false)).unwrap();

        assert_eq!(state_snapshot(&first), state_snapshot(&second));
        assert_ne!(first.agents, different.agents);
    }

    #[test]
    fn splitmix64_sequence_is_stable() {
        let mut entropy = SplitMix64::new(0);

        assert_eq!(entropy.next_u64(), 0xE220_A839_7B1D_CDAF);
        assert_eq!(entropy.next_u64(), 0x6E78_9E6A_A1B9_65F4);
        assert_eq!(entropy.next_u64(), 0x06C4_5D18_8009_454F);
    }

    #[test]
    fn repeated_runs_are_byte_identical() {
        let mut first = Simulation::new(config(42, 100, false)).unwrap();
        let mut second = Simulation::new(config(42, 100, false)).unwrap();
        let mut first_output = Vec::new();
        let mut second_output = Vec::new();

        let first_summary = first.run(&mut first_output).unwrap();
        let second_summary = second.run(&mut second_output).unwrap();

        assert_eq!(first_summary, second_summary);
        assert_eq!(state_snapshot(&first), state_snapshot(&second));
        assert_eq!(first_output, second_output);
    }

    #[test]
    fn action_tracing_is_optional_complete_and_observational() {
        let mut plain = Simulation::new(config(17, 5, false)).unwrap();
        let mut traced = Simulation::new(config(17, 5, true)).unwrap();
        let mut plain_output = Vec::new();
        let mut traced_output = Vec::new();

        let plain_summary = plain.run(&mut plain_output).unwrap();
        let traced_summary = traced.run(&mut traced_output).unwrap();

        assert_eq!(plain_summary, traced_summary);
        assert_eq!(state_snapshot(&plain), state_snapshot(&traced));
        assert_eq!(
            output_without_action_traces(&traced_output),
            String::from_utf8(plain_output).unwrap()
        );

        let traced_output = String::from_utf8(traced_output).unwrap();
        assert_eq!(traced_output.matches("event=action_trace").count(), 60);
        assert!(!traced_output.contains("status:rejected"));
    }

    #[test]
    fn invalid_move_does_not_mutate_action_state() {
        let mut simulation = Simulation::new(config(0, 1, false)).unwrap();
        simulation.agents[0].position = Coordinate { x: 0, y: 0 };
        let before_agent = simulation.agents[0].clone();
        let before_food = simulation.foods.clone();
        let mut output = Vec::new();

        let result = simulation
            .apply_action(
                &mut output,
                0,
                &Action::Move {
                    direction: Direction::North,
                },
            )
            .unwrap();

        assert!(!result.accepted);
        assert_eq!(result.detail, "out_of_bounds");
        assert_eq!(simulation.agents[0], before_agent);
        assert_eq!(simulation.foods, before_food);
        assert!(output.is_empty());
    }

    #[test]
    fn movement_crosses_territory_and_is_observable() {
        let mut simulation = Simulation::new(config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 10, y: 63 };
        let mut output = Vec::new();

        let result = simulation
            .apply_action(
                &mut output,
                0,
                &Action::Move {
                    direction: Direction::South,
                },
            )
            .unwrap();

        assert!(result.accepted);
        assert_eq!(simulation.agents[0].position, Coordinate { x: 10, y: 64 });
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("event=territory_crossed"));
        assert!(output.contains("result=from:A,to:B"));
    }

    #[test]
    fn sleep_restores_energy_without_exceeding_the_maximum() {
        let mut simulation = Simulation::new(config(0, 1, false)).unwrap();
        simulation.agents[0].energy = 90;

        let result = simulation
            .apply_action(&mut Vec::new(), 0, &Action::Sleep)
            .unwrap();

        assert!(result.accepted);
        assert_eq!(simulation.agents[0].energy, 100);
        assert_eq!(result.detail, "energy:90->100");
    }

    #[test]
    fn eating_is_atomic_bounded_and_single_use() {
        let mut simulation = Simulation::new(config(0, 1, false)).unwrap();
        let food = simulation
            .foods
            .iter()
            .find(|food| food.class == FoodClass::Medium)
            .unwrap()
            .clone();
        simulation.agents[0].position = food.position;
        simulation.agents[0].satiety = 80;
        simulation.agents[0].energy = 95;
        simulation.agents[1].position = food.position;
        let action = Action::Eat {
            food_id: food.id.clone(),
        };
        let mut output = Vec::new();

        let first = simulation.apply_action(&mut output, 0, &action).unwrap();
        let second_before = simulation.agents[1].clone();
        let second = simulation.apply_action(&mut output, 1, &action).unwrap();

        assert!(first.accepted);
        assert_eq!(simulation.agents[0].satiety, 100);
        assert_eq!(simulation.agents[0].energy, 100);
        assert!(!simulation.foods.iter().any(|item| item.id == food.id));
        assert!(!second.accepted);
        assert_eq!(simulation.agents[1], second_before);
        assert_eq!(
            String::from_utf8(output)
                .unwrap()
                .matches("event=food_consumed")
                .count(),
            1
        );
    }

    #[test]
    fn survival_decay_saturates_and_death_is_final() {
        let mut simulation = Simulation::new(config(0, 2, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].health = 5;
        simulation.agents[0].satiety = 1;
        simulation.agents[0].energy = 50;
        let mut output = Vec::new();

        simulation.apply_survival(&mut output, 0).unwrap();

        assert_eq!(simulation.agents[0].health, 0);
        assert_eq!(simulation.agents[0].satiety, 0);
        assert!(!simulation.agents[0].alive);
        assert_eq!(
            String::from_utf8(output)
                .unwrap()
                .matches("event=agent_died")
                .count(),
            1
        );
    }

    #[test]
    fn food_regenerates_only_in_nonempty_nonfull_territories() {
        let mut simulation = Simulation::new(config(9, 10, false)).unwrap();
        simulation.tick = 10;
        let retained = simulation
            .foods
            .iter()
            .find(|food| food.position.territory() == Territory::A)
            .unwrap()
            .clone();
        simulation.foods = vec![retained];
        let mut output = Vec::new();

        simulation
            .regenerate_food(&mut output, Territory::A)
            .unwrap();
        simulation
            .regenerate_food(&mut output, Territory::B)
            .unwrap();

        assert_eq!(
            simulation.food_counts(Territory::A).iter().sum::<usize>(),
            1 + REGENERATION_YIELD
        );
        assert_eq!(
            simulation.food_counts(Territory::B).iter().sum::<usize>(),
            0
        );
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("event=food_regenerated"));
        assert!(output.contains("event=food_regeneration_skipped result=reason:depleted"));
    }

    #[test]
    fn food_regeneration_respects_capacity() {
        let mut simulation = Simulation::new(config(9, 10, false)).unwrap();
        let capacity = simulation.config.density.resources_per_territory();
        simulation.tick = 10;
        simulation
            .foods
            .retain(|food| food.position.territory() == Territory::B);
        for number in 0..capacity {
            simulation.foods.push(Food {
                id: format!("FC{number:02}"),
                position: Coordinate {
                    x: number as u8,
                    y: 0,
                },
                class: FoodClass::Low,
            });
        }
        let mut output = Vec::new();

        simulation
            .regenerate_food(&mut output, Territory::A)
            .unwrap();

        assert_eq!(simulation.food_counts(Territory::A)[0], capacity);
        assert!(
            String::from_utf8(output)
                .unwrap()
                .contains("reason:capacity")
        );
    }

    struct InvalidNorthDecisionSource;

    impl DecisionSource for InvalidNorthDecisionSource {
        fn name(&self) -> &str {
            "invalid_north"
        }

        fn decide(
            &mut self,
            _observation: &Observation,
            _entropy: &mut DecisionEntropy<'_>,
        ) -> Action {
            Action::Move {
                direction: Direction::North,
            }
        }
    }

    #[test]
    fn untrusted_decisions_are_validated_and_traced() {
        let mut simulation = Simulation::new(config(0, 1, true)).unwrap();
        for agent in &mut simulation.agents {
            agent.position.y = 0;
        }
        let initial_positions: Vec<Coordinate> = simulation
            .agents
            .iter()
            .map(|agent| agent.position)
            .collect();
        let mut source = InvalidNorthDecisionSource;
        let mut output = Vec::new();

        simulation
            .run_with_source(&mut output, &mut source)
            .unwrap();

        assert_eq!(
            simulation
                .agents
                .iter()
                .map(|agent| agent.position)
                .collect::<Vec<_>>(),
            initial_positions
        );
        let output = String::from_utf8(output).unwrap();
        assert_eq!(output.matches("event=action_trace").count(), 12);
        assert_eq!(output.matches("status:rejected").count(), 12);
    }

    #[test]
    fn tick_limit_terminates_with_one_summary() {
        let mut simulation = Simulation::new(config(1, 1, false)).unwrap();
        let mut output = Vec::new();

        let summary = simulation.run(&mut output).unwrap();

        assert_eq!(summary.reason, TerminationReason::TickLimit);
        assert_eq!(summary.ticks, 1);
        assert_eq!(
            String::from_utf8(output)
                .unwrap()
                .matches("summary ")
                .count(),
            1
        );
    }

    #[test]
    fn extinction_takes_precedence_at_the_tick_limit() {
        let mut simulation = Simulation::new(config(1, 1, false)).unwrap();
        simulation.foods.clear();
        for agent in &mut simulation.agents {
            agent.health = 5;
            agent.satiety = 0;
        }
        let mut output = Vec::new();

        let summary = simulation.run(&mut output).unwrap();

        assert_eq!(summary.reason, TerminationReason::Extinction);
        assert_eq!(summary.survivors, 0);
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("event=simulation_ended result=reason:extinction"));
    }

    #[test]
    fn a_long_configured_run_is_bounded_and_does_not_panic() {
        let mut simulation = Simulation::new(config(123, 10_000, false)).unwrap();

        let summary = simulation.run(&mut io::sink()).unwrap();

        assert!(summary.ticks <= 10_000);
        assert_eq!(summary.survivors + summary.deaths, 12);
    }

    #[test]
    fn regeneration_adds_only_what_remaining_capacity_allows() {
        let mut simulation = Simulation::new(config(9, 10, false)).unwrap();
        let capacity = simulation.config.density.resources_per_territory();
        simulation.tick = 10;
        simulation
            .foods
            .retain(|food| food.position.territory() == Territory::B);
        for number in 0..(capacity - 1) {
            simulation.foods.push(Food {
                id: format!("FC{number:02}"),
                position: Coordinate {
                    x: number as u8,
                    y: 0,
                },
                class: FoodClass::Low,
            });
        }
        let mut output = Vec::new();

        simulation
            .regenerate_food(&mut output, Territory::A)
            .unwrap();

        assert_eq!(
            simulation.food_counts(Territory::A).iter().sum::<usize>(),
            capacity
        );
        assert_eq!(
            String::from_utf8(output)
                .unwrap()
                .matches("event=food_regenerated")
                .count(),
            1
        );
    }

    // REQ-MOK-013: perception.

    #[test]
    fn perception_reports_in_radius_food_with_class_direction_and_distance() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 40, y: 20 };
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position: Coordinate { x: 44, y: 20 },
            class: FoodClass::Medium,
        }];

        let observation = simulation.observation(0);

        assert_eq!(
            observation.perceived_food,
            vec![PerceivedFood {
                id: "F0001".into(),
                class: FoodClass::Medium,
                direction: Some(RelativeDirection::East),
                distance: 4,
            }]
        );
        assert!(observation.co_located_food.is_empty());
    }

    #[test]
    fn perception_reports_living_neighbours_and_never_the_observer() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 40, y: 20 };
        simulation.agents[1].position = Coordinate { x: 45, y: 24 };
        for agent in simulation.agents.iter_mut().skip(2) {
            agent.position = Coordinate { x: 120, y: 60 };
        }

        let observation = simulation.observation(0);

        assert_eq!(
            observation.perceived_mokiterions,
            vec![PerceivedMokiterion {
                id: "M02".into(),
                direction: Some(RelativeDirection::SouthEast),
                distance: 5,
            }]
        );
        assert!(
            !observation
                .perceived_mokiterions
                .iter()
                .any(|other| other.id == observation.agent_id)
        );
    }

    #[test]
    fn perception_excludes_distant_resources_and_dead_neighbours() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 40, y: 20 };
        simulation.agents[1].position = Coordinate { x: 41, y: 20 };
        simulation.agents[1].alive = false;
        for agent in simulation.agents.iter_mut().skip(2) {
            agent.position = Coordinate { x: 120, y: 60 };
        }
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position: Coordinate { x: 100, y: 20 },
            class: FoodClass::High,
        }];

        let observation = simulation.observation(0);

        assert!(observation.perceived_food.is_empty());
        assert!(observation.perceived_mokiterions.is_empty());
    }

    #[test]
    fn the_radius_boundary_is_inclusive_and_exclusive_by_one_cell() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 40, y: 40 };
        simulation.foods = vec![
            Food {
                id: "F0001".into(),
                position: Coordinate {
                    x: 40 + PERCEPTION_RADIUS,
                    y: 40,
                },
                class: FoodClass::Low,
            },
            Food {
                id: "F0002".into(),
                position: Coordinate {
                    x: 40 + PERCEPTION_RADIUS + 1,
                    y: 40,
                },
                class: FoodClass::Low,
            },
        ];

        let perceived = simulation.observation(0).perceived_food;

        assert_eq!(perceived.len(), 1);
        assert_eq!(perceived[0].id, "F0001");
        assert_eq!(perceived[0].distance, PERCEPTION_RADIUS);
    }

    #[test]
    fn perception_crosses_the_territory_boundary() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 30, y: 62 };
        simulation.agents[1].position = Coordinate { x: 30, y: 66 };
        for agent in simulation.agents.iter_mut().skip(2) {
            agent.position = Coordinate { x: 120, y: 10 };
        }
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position: Coordinate { x: 30, y: 70 },
            class: FoodClass::High,
        }];

        let observation = simulation.observation(0);

        assert_eq!(observation.territory, Territory::A);
        assert_eq!(observation.perceived_food.len(), 1);
        assert_eq!(
            observation.perceived_food[0].direction,
            Some(RelativeDirection::South)
        );
        assert_eq!(observation.perceived_mokiterions.len(), 1);
    }

    #[test]
    fn co_located_entities_are_reported_at_distance_zero_without_a_direction() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        let shared = Coordinate { x: 50, y: 30 };
        simulation.agents[0].position = shared;
        simulation.agents[1].position = shared;
        for agent in simulation.agents.iter_mut().skip(2) {
            agent.position = Coordinate { x: 120, y: 60 };
        }
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position: shared,
            class: FoodClass::Low,
        }];

        let observation = simulation.observation(0);

        assert_eq!(observation.co_located_food, vec!["F0001".to_string()]);
        assert_eq!(observation.perceived_food[0].distance, 0);
        assert_eq!(observation.perceived_food[0].direction, None);
        assert_eq!(observation.perceived_mokiterions[0].distance, 0);
        assert_eq!(observation.perceived_mokiterions[0].direction, None);
    }

    #[test]
    fn perception_order_is_stable_and_independent_of_collection_order() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 60, y: 30 };
        simulation.foods = vec![
            Food {
                id: "F0009".into(),
                position: Coordinate { x: 66, y: 30 },
                class: FoodClass::Low,
            },
            Food {
                id: "F0002".into(),
                position: Coordinate { x: 62, y: 30 },
                class: FoodClass::High,
            },
            Food {
                id: "F0001".into(),
                position: Coordinate { x: 60, y: 32 },
                class: FoodClass::Low,
            },
        ];

        let first = simulation.observation(0);
        simulation.foods.reverse();
        let reversed = simulation.observation(0);

        let ids: Vec<&str> = first
            .perceived_food
            .iter()
            .map(|food| food.id.as_str())
            .collect();
        // Distance first: the two resources two cells away precede the one six
        // cells away. Within equal distance, identifier order decides, so the
        // richer but higher-numbered F0002 still follows F0001.
        assert_eq!(ids, vec!["F0001", "F0002", "F0009"]);
        assert_eq!(first.perceived_food, reversed.perceived_food);
        assert!(first.is_consistent());
    }

    #[test]
    fn perception_is_symmetric_between_living_mokiterions() {
        let mut simulation = Simulation::new(reference_config(7, 1, false)).unwrap();
        simulation.tick = 1;

        for left in 0..simulation.agents.len() {
            let observed: Vec<(String, u8)> = simulation
                .observation(left)
                .perceived_mokiterions
                .into_iter()
                .map(|other| (other.id, other.distance))
                .collect();
            for (id, distance) in observed {
                let right = simulation
                    .agents
                    .iter()
                    .position(|agent| agent.id == id)
                    .unwrap();
                let reciprocal = simulation
                    .observation(right)
                    .perceived_mokiterions
                    .into_iter()
                    .find(|other| other.id == simulation.agents[left].id);
                assert_eq!(reciprocal.map(|other| other.distance), Some(distance));
            }
        }
    }

    #[test]
    fn building_an_observation_consumes_no_entropy_and_mutates_nothing() {
        let mut simulation = Simulation::new(reference_config(42, 5, false)).unwrap();
        simulation.tick = 1;
        let before = state_snapshot(&simulation);

        for index in 0..simulation.agents.len() {
            let _ = simulation.observation(index);
        }

        assert_eq!(state_snapshot(&simulation), before);
    }

    // REQ-MOK-014: population viability.

    #[test]
    fn density_resolves_to_the_specified_resource_count() {
        // The mapping is fixed by `SPEC-MOK-001`: hundredths of a percent times the cells
        // in one territory, divided by 10,000, truncating toward zero. These three counts
        // are the ones the specification states.
        assert_eq!(CELLS_PER_TERRITORY, 8_192);
        for (density, expected) in [("0.15", 12), ("0.75", 61), ("1.50", 122)] {
            assert_eq!(
                Density::parse(density).unwrap().resources_per_territory(),
                expected,
                "density {density}% must resolve to {expected} resources per territory"
            );
        }
        assert_eq!(Density::DEFAULT.resources_per_territory(), 61);

        // Written forms that denote the same density resolve identically, and the display
        // form round-trips.
        assert_eq!(Density::parse("1.5"), Density::parse("1.50"));
        assert_eq!(Density::parse(".75"), Density::parse("0.75"));
        assert_eq!(Density::DEFAULT.to_string(), "0.75");
    }

    #[test]
    fn a_density_resolving_to_no_resources_is_rejected() {
        // Truncation makes `0.01%` resolve to zero. That is invalid configuration rather
        // than an empty world, because an emptied territory can never regenerate.
        let error = Density::parse("0.01").unwrap_err();
        assert!(error.contains("zero resources"), "{error}");
        assert!(
            error.contains("0.02"),
            "the usable floor must be reported: {error}"
        );
        assert_eq!(Density::parse("0.02").unwrap().resources_per_territory(), 1);

        assert!(Density::parse("0").is_err());
        assert!(Density::parse("0.751").is_err());
        assert!(Density::parse("101").is_err());
        assert!(Density::parse("").is_err());
        assert!(Density::parse("-1").is_err());
    }

    #[test]
    fn density_binds_initialization_capacity_and_the_replenishment_target() {
        for density in ["0.02", "0.50", "1.50"] {
            let resolved = Density::parse(density).unwrap().resources_per_territory();
            let mut simulation = Simulation::new(config_at(7, 400, density)).unwrap();

            // Initialization: every territory begins at exactly the resolved count.
            for territory in Territory::ALL {
                assert_eq!(
                    simulation.food_counts(territory).iter().sum::<usize>(),
                    resolved,
                    "density {density}% must start each territory at {resolved}"
                );
            }

            // Capacity: across a whole run no territory ever exceeds the resolved count.
            simulation.run(&mut io::sink()).unwrap();
            for territory in Territory::ALL {
                assert!(
                    simulation.food_counts(territory).iter().sum::<usize>() <= resolved,
                    "density {density}% exceeded its capacity in territory {territory}"
                );
            }
        }

        // Replenishment target: a territory reduced below the resolved count is refilled
        // toward it and stops there rather than growing past it.
        let mut simulation = Simulation::new(config_at(7, 10, "0.50")).unwrap();
        let resolved = simulation.config.density.resources_per_territory();
        simulation
            .foods
            .retain(|food| food.position.territory() == Territory::B || food.id == "F0001");
        for tick in 1..=(resolved * REGENERATION_INTERVAL as usize) {
            simulation.tick = tick as u64 * REGENERATION_INTERVAL;
            simulation
                .regenerate_food(&mut io::sink(), Territory::A)
                .unwrap();
        }
        assert_eq!(
            simulation.food_counts(Territory::A).iter().sum::<usize>(),
            resolved,
            "replenishment must reach the resolved count and stop there"
        );
    }

    #[test]
    fn the_reference_source_sustains_the_population_at_every_declared_density() {
        for (density, floor) in DECLARED_FLOORS {
            for seed in DECLARED_SEEDS {
                let mut simulation = Simulation::new(config_at(seed, 1_000, density)).unwrap();
                let mut output = Vec::new();

                let summary = simulation.run(&mut output).unwrap();
                let consumed = String::from_utf8(output)
                    .unwrap()
                    .matches("event=food_consumed")
                    .count();

                assert_eq!(
                    summary.reason,
                    TerminationReason::TickLimit,
                    "seed {seed} at density {density}% ended in extinction"
                );
                assert!(
                    summary.survivors >= floor,
                    "seed {seed} at density {density}% left only {} survivors, below the stated floor of {floor}",
                    summary.survivors
                );
                assert!(
                    consumed > 0,
                    "seed {seed} at density {density}% consumed no food"
                );
            }
        }
    }

    #[test]
    fn attributes_stay_within_bounds_across_a_long_reference_run() {
        let mut simulation = Simulation::new(reference_config(42, 1_000, false)).unwrap();

        simulation.run(&mut io::sink()).unwrap();

        for agent in &simulation.agents {
            assert!(agent.health <= ATTRIBUTE_MAX);
            assert!(agent.satiety <= ATTRIBUTE_MAX);
            assert!(agent.energy <= ATTRIBUTE_MAX);
        }
    }

    // REQ-MOK-015: the reference decision source.

    #[test]
    fn the_reference_source_approaches_then_consumes_a_perceived_resource() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 40, y: 20 };
        simulation.agents[0].satiety = 40;
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position: Coordinate { x: 44, y: 20 },
            class: FoodClass::Medium,
        }];

        let (approach, draws) = decide_once(&simulation, 0);
        assert_eq!(
            approach,
            Action::Move {
                direction: Direction::East
            }
        );
        assert_eq!(draws, 0, "an approach must not consume entropy");

        simulation.agents[0].position = Coordinate { x: 44, y: 20 };
        let (arrival, draws) = decide_once(&simulation, 0);
        assert_eq!(
            arrival,
            Action::Eat {
                food_id: "F0001".into()
            }
        );
        assert_eq!(draws, 0, "eating must not consume entropy");
    }

    #[test]
    fn the_reference_source_prefers_the_horizontal_axis_on_a_diagonal_approach() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 40, y: 20 };
        simulation.agents[0].satiety = 40;
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position: Coordinate { x: 43, y: 17 },
            class: FoodClass::Low,
        }];

        let (action, _) = decide_once(&simulation, 0);

        assert_eq!(
            action,
            Action::Move {
                direction: Direction::East
            }
        );
    }

    #[test]
    fn the_reference_source_prefers_the_nearest_then_richest_resource() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 60, y: 30 };
        simulation.agents[0].satiety = 40;
        simulation.foods = vec![
            Food {
                id: "F0001".into(),
                position: Coordinate { x: 56, y: 30 },
                class: FoodClass::High,
            },
            Food {
                id: "F0002".into(),
                position: Coordinate { x: 62, y: 30 },
                class: FoodClass::Low,
            },
        ];

        let (nearest_wins, _) = decide_once(&simulation, 0);
        assert_eq!(
            nearest_wins,
            Action::Move {
                direction: Direction::East
            }
        );

        // At equal distance the richer resource wins instead.
        simulation.foods[1].position = Coordinate { x: 64, y: 30 };
        let (richest_wins, _) = decide_once(&simulation, 0);
        assert_eq!(
            richest_wins,
            Action::Move {
                direction: Direction::West
            }
        );
    }

    #[test]
    fn the_reference_source_does_not_consume_a_resource_it_does_not_need() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        let position = Coordinate { x: 50, y: 30 };
        simulation.agents[0].position = position;
        // One satiety point above the level at which a high-class resource still fits, so the
        // non-waste rule declines it. Derived from the food table rather than from a
        // threshold constant, because rule 5 states no threshold.
        simulation.agents[0].satiety = ATTRIBUTE_MAX - FoodClass::High.restoration().0 + 1;
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position,
            class: FoodClass::High,
        }];

        let (action, _) = decide_once(&simulation, 0);

        assert_ne!(
            action,
            Action::Eat {
                food_id: "F0001".into()
            }
        );
        assert!(simulation.foods.iter().any(|food| food.id == "F0001"));
    }

    /// Rule 5 applies the non-waste test to approaching as well as to eating. Only the search
    /// step consumes entropy, so the draw count distinguishes a deliberate approach from a
    /// search without naming how the source ranks candidates.
    #[test]
    fn the_reference_source_does_not_approach_a_resource_it_would_decline() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        let underfoot = Coordinate { x: 50, y: 30 };
        simulation.agents[0].position = underfoot;
        simulation.agents[0].satiety = ATTRIBUTE_MAX - FoodClass::High.restoration().0 + 1;

        // Both perceived resources would be clipped, so there is nothing worth walking to.
        simulation.foods = vec![
            Food {
                id: "F0001".into(),
                position: underfoot,
                class: FoodClass::High,
            },
            Food {
                id: "F0002".into(),
                position: Coordinate { x: 47, y: 30 },
                class: FoodClass::High,
            },
        ];

        let (declined, draws) = decide_once(&simulation, 0);
        assert!(
            matches!(declined, Action::Move { .. }),
            "a Mokiterion with nothing worth eating must keep moving, got {declined:?}"
        );
        assert_eq!(
            draws, 1,
            "with no resource worth approaching the step must be a search, not an approach"
        );

        // The filter is selective, not blanket: a resource that does fit is still approached
        // deterministically, on the horizontal axis, consuming no entropy.
        simulation.foods[1].class = FoodClass::Medium;
        let (approach, draws) = decide_once(&simulation, 0);
        assert_eq!(
            approach,
            Action::Move {
                direction: Direction::West
            }
        );
        assert_eq!(draws, 0, "approaching must not consume entropy");

        // The two-cell oscillation this rule exists to remove: standing one cell from a
        // resource it would decline, the source must not re-target that cell.
        simulation.foods.truncate(1);
        simulation.agents[0].position = Coordinate { x: 49, y: 30 };
        let (adjacent, draws) = decide_once(&simulation, 0);
        assert_eq!(
            draws, 1,
            "the cell just left must not be re-targeted, got {adjacent:?}"
        );
    }

    #[test]
    fn the_reference_source_sustains_itself_before_seeking_or_searching() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 40, y: 20 };
        simulation.agents[0].satiety = 40;
        simulation.agents[0].energy = REFERENCE_SLEEP_THRESHOLD - 1;
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position: Coordinate { x: 44, y: 20 },
            class: FoodClass::Medium,
        }];

        let (depleted, draws) = decide_once(&simulation, 0);
        assert_eq!(depleted, Action::Sleep);
        assert_eq!(draws, 0, "sleeping must not consume entropy");

        simulation.agents[0].energy = REFERENCE_SLEEP_THRESHOLD;
        let (rested, _) = decide_once(&simulation, 0);
        assert_eq!(
            rested,
            Action::Move {
                direction: Direction::East
            }
        );
    }

    #[test]
    fn the_reference_source_searches_rather_than_waiting_when_it_perceives_nothing() {
        let mut simulation = Simulation::new(reference_config(3, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = Coordinate { x: 40, y: 20 };
        simulation.foods.clear();

        let (action, draws) = decide_once(&simulation, 0);

        assert!(matches!(action, Action::Move { .. }), "got {action}");
        assert_ne!(action, Action::Wait);
        assert_eq!(draws, 1, "a search step consumes exactly one selection");
    }

    #[test]
    fn the_reference_source_cannot_mutate_authoritative_state() {
        let mut simulation = Simulation::new(reference_config(11, 1, false)).unwrap();
        simulation.tick = 1;
        let before = state_snapshot(&simulation);

        for index in 0..simulation.agents.len() {
            let (_action, _draws) = decide_once(&simulation, index);
        }

        assert_eq!(state_snapshot(&simulation), before);
    }

    #[test]
    fn perception_grants_no_ability_to_act_at_a_distance() {
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.agents[0].position = Coordinate { x: 40, y: 20 };
        simulation.agents[0].satiety = 40;
        for agent in simulation.agents.iter_mut().skip(1) {
            agent.position = Coordinate { x: 120, y: 100 };
        }
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position: Coordinate { x: 43, y: 20 },
            class: FoodClass::High,
        }];

        simulation.run(&mut io::sink()).unwrap();

        assert!(
            simulation.foods.iter().any(|food| food.id == "F0001"),
            "a perceived resource must only be removed by a co-located eat"
        );
    }

    #[test]
    fn both_sources_run_are_reported_and_are_byte_identically_reproducible() {
        for policy in [Policy::Baseline, Policy::Reference] {
            let configuration = Config {
                seed: 42,
                tick_limit: 200,
                policy,
                density: Density::DEFAULT,
                trace_actions: true,
            };
            let mut first = Simulation::new(configuration).unwrap();
            let mut second = Simulation::new(configuration).unwrap();
            let mut first_output = Vec::new();
            let mut second_output = Vec::new();

            let first_summary = first.run(&mut first_output).unwrap();
            let second_summary = second.run(&mut second_output).unwrap();

            assert_eq!(first_summary, second_summary);
            assert_eq!(state_snapshot(&first), state_snapshot(&second));
            assert_eq!(first_output, second_output);
            assert!(String::from_utf8(first_output).unwrap().contains(&format!(
                "event=decision_source_selected result=source:{policy}"
            )));
        }
    }

    #[test]
    fn the_reference_source_never_waits_and_never_runs_its_energy_to_zero() {
        let mut simulation = Simulation::new(reference_config(1, 500, true)).unwrap();
        let mut output = Vec::new();

        simulation.run(&mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(output.matches("proposal:wait").count(), 0);
        for agent in simulation.agents.iter().filter(|agent| agent.alive) {
            assert!(agent.energy > 0, "{} ran its energy to zero", agent.id);
        }
    }

    #[test]
    fn a_long_run_is_bounded_under_either_source() {
        for policy in [Policy::Baseline, Policy::Reference] {
            let mut simulation = Simulation::new(Config {
                seed: 123,
                tick_limit: 10_000,
                policy,
                density: Density::DEFAULT,
                trace_actions: false,
            })
            .unwrap();

            let summary = simulation.run(&mut io::sink()).unwrap();

            assert!(summary.ticks <= 10_000);
            assert_eq!(summary.survivors + summary.deaths, 12);
        }
    }
}
