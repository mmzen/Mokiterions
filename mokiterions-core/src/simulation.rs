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
const FEAR_INCREASE: u8 = 10;
const FEAR_DECREASE: u8 = 5;

/// `SPEC-MOK-001`'s *Contact* section and rule 20: the contact radius, in the same
/// Chebyshev distance perception uses. Contact adds a radius and no second notion of
/// distance, which is why this is the only constant the relation needs.
const CONTACT_RADIUS: u8 = 1;

/// Rule 22's constant term. It is what makes the minimum damage `10` by construction
/// rather than by a clamp, so no resolution can deal `0`.
const STRIKE_BASE_DAMAGE: u8 = 10;

/// Rule 22's divisor over the striker's `energy + health`. With both terms bounded by
/// [`ATTRIBUTE_MAX`] the variable part is `0..=20`, so damage is `10..=30`.
const STRIKE_CONDITION_DIVISOR: u16 = 10;

/// Rule 22's flat `energy` cost, paid whether or not the target dies.
const STRIKE_ENERGY_COST: u8 = 5;

/// Rule 23's increase, a constant of its own beside [`FEAR_INCREASE`] and
/// [`FEAR_DECREASE`] rather than a multiple of either, and derived from neither
/// Mokiterion's attributes.
const THREAT_FEAR_INCREASE: u8 = 30;

/// Rule 26's first answer threshold: at or above this `fear` the answer is `surrender`.
const SURRENDER_FEAR_THRESHOLD: u8 = 60;

/// Rule 26's second answer threshold: at or above this `fear` the answer is `retreat`,
/// and below it `fight`.
const RETREAT_FEAR_THRESHOLD: u8 = 30;

/// Rule 26's engagement threshold: below this `fear` a Mokiterion attacks or approaches,
/// and at or above it threatens or avoids. It reads "engage unless nearly saturated": only a
/// Mokiterion within `5` of [`ATTRIBUTE_MAX`] declines.
///
/// It was `30`, equal to [`RETREAT_FEAR_THRESHOLD`], and was named separately against exactly
/// the amendment that has since happened — `REQ-MOK-048`'s of 2026-08-20 moved this one and
/// left that one alone. **The value is measured rather than derived**, and the specification
/// records that as a cost: at `30` no approach could ever complete, because rule 12 drives
/// `fear` from company perceived at [`PERCEPTION_RADIUS`] while engagement needs contact at
/// [`CONTACT_RADIUS`], so the gate closed on the third perceiving tick and closing sixteen
/// squares takes fifteen. `90` fails `REQ-MOK-049` on one declared seed and `100` holds with
/// less survivor margin; `95` is the measured point where both of that requirement's bounds
/// first hold together on all five.
const ENGAGEMENT_FEAR_THRESHOLD: u8 = 95;

/// The constant mixed with the run seed to derive a Mokiterion's trait, fixed by
/// `SPEC-MOK-001`'s *Behavioral trait*. It is a salt, not a generator parameter: the
/// generator's own multipliers are in [`SplitMix64`].
const TRAIT_SALT: u64 = 0xC2B2_AE3D_27D4_EB4F;

/// Cells in one territory. Density is expressed relative to this, not to the world.
///
/// Public under `SPEC-MOK-002` rule 5: a fixed world dimension already implied by
/// `SPEC-MOK-001`. It is the only simulation constant rule 6 admits to the interface.
pub const CELLS_PER_TERRITORY: usize = WORLD_SIZE as usize * TERRITORY_HEIGHT as usize;

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
    ///
    /// Public under `SPEC-MOK-002` rule 5: a pure function of a value, taking `self` by
    /// copy and returning a count the program already reports.
    pub fn resources_per_territory(self) -> usize {
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
    Individual,
    /// Rule 26's source. It is not the default and is not proposed as one: the survivor
    /// floor `REQ-MOK-049` states for it is three below `REQ-MOK-014`'s for the default.
    Social,
}

impl Policy {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "baseline" => Some(Self::Baseline),
            "reference" => Some(Self::Reference),
            "individual" => Some(Self::Individual),
            "social" => Some(Self::Social),
            _ => None,
        }
    }
}

impl fmt::Display for Policy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Baseline => formatter.write_str("baseline"),
            Self::Reference => formatter.write_str("reference"),
            Self::Individual => formatter.write_str("individual"),
            Self::Social => formatter.write_str("social"),
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

    /// The opposite cardinal step. Rule 21 states that `avoid` and `retreat` "use the same axis
    /// choice in the opposite direction" as `approach`, so the three verbs share one axis rule
    /// and differ by this function alone.
    fn reversed(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
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

/// One attack that has landed on a Mokiterion since its previous decision opportunity:
/// who struck, and for how much.
///
/// Rule 25's window is a `Vec` of these hanging off the Mokiterion that suffered them, in
/// the order the attacks resolved. It is transient per-Mokiterion state and not an
/// attribute: it is not bounded in `0..=100`, it is not reported in `agent_initialized` or
/// `survival_changed`, and no structure indexed by pairs of Mokiterions exists. The record
/// carries what happened to the sufferer and never the condition of whoever did it, which
/// is how a defender answers without reading another Mokiterion's strength.
///
/// The type stays private under `SPEC-MOK-002` rule 6. Where the trace line reports the
/// record, [`EventDetail::ActionTrace`] carries it as pairs of already-public values, so
/// that reporting it adds no item to the interface.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SufferedAttack {
    attacker: String,
    damage: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mokiterion {
    id: String,
    /// `SPEC-MOK-001`'s *Name*: fixed for the run and identical in every run, assigned by table
    /// lookup on the identifier's number. Borrowed from [`NAMES`] rather than owned because no
    /// name is ever constructed, edited or freed.
    name: &'static str,
    position: Coordinate,
    health: u8,
    satiety: u8,
    energy: u8,
    /// Rule 12's fourth bounded attribute, and no longer inert. Rule 26's source reads it at
    /// every decision opportunity, and it has two writers within one tick: rule 12 at this
    /// Mokiterion's own turn and rule 23's threat at a threatener's. Both saturate within
    /// `0..=100`, neither is suppressed and neither is deferred.
    fear: u8,
    /// The one behavioral trait of `SPEC-MOK-001`'s *Behavioral trait*, derived at
    /// initialization and never written again. Only the trait-aware source reads it.
    waste_tolerance: u8,
    /// Rule 25's suffered-attack window. It opens inside rule 22 when damage resolves against
    /// this Mokiterion and closes at this Mokiterion's own next decision opportunity, whether
    /// the opportunity answered or not. It starts empty and, under `baseline`, `reference` and
    /// `individual`, stays empty for the whole run, because no targeted verb is proposed
    /// under those sources.
    suffered: Vec<SufferedAttack>,
    alive: bool,
}

/// The twelve names of `SPEC-MOK-001`'s *Name*, in identifier order: index `n - 1` is `Mn`'s.
///
/// The table is the product owner's decision, recorded in `WO-MOK-011`, and this array is the
/// whole of it. The specification fixes three properties this literal satisfies by inspection and
/// [`the_names_are_the_specified_twelve`](tests::the_names_are_the_specified_twelve) asserts: the
/// names are pairwise distinct, their first characters are pairwise distinct, and every name is
/// one to five ASCII letters. `the_names_are_the_specified_twelve` asserts all three, so a later
/// edit to this literal cannot quietly break one of them.
const NAMES: [&str; 12] = [
    "Zug", "Krul", "Quib", "Sput", "Trok", "Womp", "Hozz", "Nurb", "Vonk", "Gorm", "Xob", "Drix",
];

/// The name of the Mokiterion numbered `number`, which must be in `1..=12`.
///
/// **This performs no draw.** It reads neither the seed nor the configuration and touches no
/// generator, which is why naming leaves every run predating it byte-identical apart from the one
/// field added to the `agent_initialized` record. It is a table lookup, and `SPEC-MOK-001`'s *Time
/// and entropy* records that it is not an exception to the single shared stream because it is not
/// a draw at all.
fn name_of(number: u8) -> &'static str {
    NAMES[usize::from(number) - 1]
}

/// The inclusive upper bound of the `waste_tolerance` range, from `SPEC-MOK-001`'s *Behavioral
/// trait* as amended on 2026-08-19.
///
/// It was `ATTRIBUTE_MAX` until the sweep in `evidence/WO-MOK-010/escalation.md` showed the full
/// range missing `REQ-MOK-034`'s survivor floor on three of five declared seeds, with a fifty-seed
/// mean of 7.40 against a floor of 8. Narrowing to `40` leaves a mean of 9.94 and a 4% miss rate,
/// against the reference source's own 6%.
///
/// **This is a bound of its own and not the attribute bound**, even though the two happened to
/// coincide in the first form. The trait is not an attribute: it is not clipped, not decayed and
/// not reported per tick, and tying it to `ATTRIBUTE_MAX` would make a later change to either one
/// silently move the other.
const WASTE_TOLERANCE_MAX: u8 = 40;

/// The `waste_tolerance` of the Mokiterion numbered `number`, in `0..=`[`WASTE_TOLERANCE_MAX`].
///
/// `SPEC-MOK-001`'s *Behavioral trait* fixes the whole derivation: a generator whose initial
/// state is the seed exclusive-ored with the identifier's number times [`TRAIT_SALT`], and one
/// unbiased bounded selection from it.
///
/// **The generator is constructed here and dropped here.** It neither reads nor advances the
/// shared stream, which is the property that leaves every run predating this revision
/// unchanged. Integer arithmetic only, as the trait's every use is.
fn derive_waste_tolerance(seed: u64, number: u8) -> u8 {
    let mut generator = SplitMix64::new(seed ^ u64::from(number).wrapping_mul(TRAIT_SALT));
    generator.choose_index(usize::from(WASTE_TOLERANCE_MAX) + 1) as u8
}

/// The action contract, closed at eleven kinds by rule 21.
///
/// The seven targeted forms all name the other Mokiterion in one field called `target`,
/// which `SPEC-MOK-001`'s *Data and interface contracts* fixes: `retreat` retreats *from*
/// its target and `surrender` surrenders *to* its target, so two directional names were
/// available, and one was chosen because rule 6 checks the same three things about the
/// named Mokiterion in all seven cases. A `target` is an identifier and never a reference,
/// so naming one grants a source no more reach than perceiving one does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Wait,
    Sleep,
    Eat { food_id: String },
    Move { direction: Direction },
    Attack { target: String },
    Threaten { target: String },
    Fight { target: String },
    Retreat { target: String },
    Surrender { target: String },
    Approach { target: String },
    Avoid { target: String },
}

impl Action {
    /// The identifier this proposal names, where it names one.
    ///
    /// Rule 7's trace reports the target in a field of its own rather than inside
    /// `proposal`, and it reads it from here, so the verb and the target on a trace line
    /// cannot disagree about which action they describe.
    fn target(&self) -> Option<&str> {
        match self {
            Self::Wait | Self::Sleep | Self::Eat { .. } | Self::Move { .. } => None,
            Self::Attack { target }
            | Self::Threaten { target }
            | Self::Fight { target }
            | Self::Retreat { target }
            | Self::Surrender { target }
            | Self::Approach { target }
            | Self::Avoid { target } => Some(target.as_str()),
        }
    }
}

impl fmt::Display for Action {
    /// The four core verbs render exactly as they always have, `move`'s direction included:
    /// `CAP-MOK-009` holds every line they appear on byte-identical. A targeted verb renders
    /// as the bare verb, because its target is a field of its own beside `proposal`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wait => formatter.write_str("wait"),
            Self::Sleep => formatter.write_str("sleep"),
            Self::Eat { food_id } => write!(formatter, "eat:{food_id}"),
            Self::Move { direction } => write!(formatter, "move:{direction}"),
            Self::Attack { .. } => formatter.write_str("attack"),
            Self::Threaten { .. } => formatter.write_str("threaten"),
            Self::Fight { .. } => formatter.write_str("fight"),
            Self::Retreat { .. } => formatter.write_str("retreat"),
            Self::Surrender { .. } => formatter.write_str("surrender"),
            Self::Approach { .. } => formatter.write_str("approach"),
            Self::Avoid { .. } => formatter.write_str("avoid"),
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

/// Another living Mokiterion as perceived from a distance. Rule 26 is the first rule to consume
/// it: the source reads the identifier to name a target and the distance to evaluate rule 20's
/// predicate. It carries no attribute of the perceived Mokiterion — not its `health`, its
/// `energy` or its `fear` — so no decision here can be taken on another Mokiterion's condition,
/// which is what keeps rule 22's damage a function of the striker alone.
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
    /// The acting Mokiterion's own `fear`, carried under `REQ-MOK-045`. It replaces the
    /// refusal that stood here for two revisions: the field was absent for as long as no
    /// rule and no source read it, and rule 26's source reads it at every opportunity. The
    /// value is the one standing at the start of this opportunity — after the previous tick's
    /// rule 12 write, plus any threat applied by an earlier-acting Mokiterion in this tick.
    fear: u8,
    /// Rule 3 carries the acting Mokiterion's trait so that a source can read it without
    /// reaching into authoritative state.
    waste_tolerance: u8,
    /// Rule 25's record as it stands at this opportunity, in the order the attacks resolved.
    /// A copy, like every other field here: a source reads what happened to the observer and
    /// cannot reach whoever did it.
    suffered: Vec<SufferedAttack>,
    co_located_food: Vec<String>,
    perceived_food: Vec<PerceivedFood>,
    perceived_mokiterions: Vec<PerceivedMokiterion>,
    /// The core proposals of `REQ-MOK-005` and nothing else. **A targeted action never
    /// appears here**, and rule 3 states why: rule 4's baseline consumes one entropy
    /// selection over this list's length, so a longer list would move that selection and
    /// diverge every run ever recorded under `baseline`. The consequence is that this list is
    /// no longer everything a source may legitimately propose — rule 6 is — and a reader who
    /// takes it as the whole contract will be wrong about the social source.
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
            && self.fear <= ATTRIBUTE_MAX
            && self.waste_tolerance <= WASTE_TOLERANCE_MAX
            && !self.valid_actions.is_empty()
            // Rule 25's record never names the observer, and rule 22's minimum is the
            // constant term, so an entry dealing less than that could only come from a
            // resolution this engine does not have.
            && self.suffered.iter().all(|attack| {
                attack.attacker != self.agent_id
                    && !attack.attacker.is_empty()
                    && attack.damage >= STRIKE_BASE_DAMAGE
            })
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

    /// Whether consuming this resource wastes no more of its satiety restoration than the
    /// acting Mokiterion's own tolerance admits.
    ///
    /// Rule 19's tolerant test verbatim: the resource fits outright, or the part the
    /// attribute maximum would clip is at most `T * R / 100` with the division truncating
    /// toward zero. `u16` throughout, so no intermediate saturates and no float appears. At
    /// tolerance `0` the second clause reads `S + R - 100 <= 0`, which is [`Self::fits`], so
    /// the two agree on every observation at that tolerance.
    ///
    /// [`Self::fits`] and the two selectors beside it are deliberately left as they are
    /// rather than generalized to take a tolerance. They are the control this change is
    /// measured against, and a shared implementation would make the reference source's
    /// behavior depend on code written for the trait-aware one. The cost is the ordering
    /// below, repeated once.
    fn fits_within_tolerance(&self, food: &PerceivedFood) -> bool {
        let restored = u16::from(food.class.restoration().0);
        let resulting = u16::from(self.satiety) + restored;
        let maximum = u16::from(ATTRIBUTE_MAX);
        if resulting <= maximum {
            return true;
        }
        resulting - maximum <= u16::from(self.waste_tolerance) * restored / 100
    }

    /// The co-located resource rule 19 case 1 selects: the richest one the tolerance admits,
    /// then lowest identifier.
    fn best_tolerated_co_located_food(&self) -> Option<&PerceivedFood> {
        self.perceived_food
            .iter()
            .filter(|food| food.distance == 0)
            .filter(|food| self.fits_within_tolerance(food))
            .max_by(|left, right| {
                left.class
                    .calorie_rank()
                    .cmp(&right.class.calorie_rank())
                    .then_with(|| right.id.cmp(&left.id))
            })
    }

    /// The target rule 19 case 3 selects: nearest first, then highest calorie class, then
    /// lowest identifier, among those the tolerance admits. Case 1 and case 3 screen by the
    /// same test, which is what keeps the resource just left from being re-targeted.
    fn best_tolerated_distant_food(&self) -> Option<&PerceivedFood> {
        self.perceived_food
            .iter()
            .filter(|food| food.distance > 0)
            .filter(|food| self.fits_within_tolerance(food))
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

    /// The Mokiterion rule 26 branch 4 engages: the nearest one in contact, then the lowest
    /// identifier.
    ///
    /// Rule 3 already sorted the list by ascending distance and then identifier and already
    /// excluded the observer and the dead, so rule 26's tie-break is the first match rather
    /// than a search of its own. The distance is rule 20's predicate read off a copy: the
    /// source evaluates it against what it was shown, and rule 6 evaluates it again against
    /// authoritative state before anything applies.
    fn nearest_in_contact(&self) -> Option<&PerceivedMokiterion> {
        self.perceived_mokiterions
            .iter()
            .find(|other| other.distance <= CONTACT_RADIUS)
    }

    /// The Mokiterion rule 26 branch 5 closes on or avoids: the nearest one perceived at a
    /// distance of `2` or more, then the lowest identifier.
    fn nearest_beyond_contact(&self) -> Option<&PerceivedMokiterion> {
        self.perceived_mokiterions
            .iter()
            .find(|other| other.distance > CONTACT_RADIUS)
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

/// Rule 19's cases 1 and 2: eat the richest tolerated resource underfoot, else sleep when
/// exhausted. Returns `None` when neither applies.
///
/// It is separated from cases 3 and 4 because rule 26 branch 2 needs exactly this half.
/// That rule forbids testing its branch 2 by delegating to rule 19 and inspecting the
/// result — case 4 takes an entropy selection, so a delegation reaching it would consume a
/// draw the social source then discarded whenever a later branch fired, silently moving the
/// shared stream for a decision that was never used. **This function draws nothing**, which
/// is what makes the delegation legitimate rather than a reordering of the branches.
fn tolerant_survival_choice(observation: &Observation) -> Option<Action> {
    if let Some(food) = observation.best_tolerated_co_located_food() {
        let eat = Action::Eat {
            food_id: food.id.clone(),
        };
        if observation.allows(&eat) {
            return Some(eat);
        }
    }

    // Rule 19 case 2 states the same threshold rule 5 case 2 does, so the two share one
    // constant and cannot drift apart.
    if observation.energy < REFERENCE_SLEEP_THRESHOLD && observation.allows(&Action::Sleep) {
        return Some(Action::Sleep);
    }

    None
}

/// Rule 19's case 3 alone: step toward the most attractive tolerated resource. Returns `None`
/// where no such resource is perceived, or where neither of the two admissible axes yields a
/// step rule 6 accepts.
///
/// It is separated from case 4 for the reason [`tolerant_survival_choice`] is separated from
/// both, and the separation carries more weight here. Rule 26 branch 3 needs exactly this
/// half: `REQ-MOK-048`'s amendment of 2026-08-20 hoists case 3 above that rule's two social
/// branches, and hoisting is only legitimate because **this function draws nothing**. Lifting
/// case 4 with it would have moved the shared stream for every social decision.
fn tolerant_seek_choice(observation: &Observation) -> Option<Action> {
    if let Some(target) = observation.best_tolerated_distant_food()
        && let Some(direction) = target.direction
    {
        let preferred = direction.horizontal().or_else(|| direction.vertical());
        let alternate = direction.vertical().or_else(|| direction.horizontal());
        for candidate in [preferred, alternate].into_iter().flatten() {
            let step = Action::Move {
                direction: candidate,
            };
            if observation.allows(&step) {
                return Some(step);
            }
        }
    }

    None
}

/// Rule 19's case 4 alone: the search step. This is the only part of rule 19 that draws, and
/// it draws exactly once.
fn tolerant_search_choice(observation: &Observation, entropy: &mut DecisionEntropy<'_>) -> Action {
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

/// Rule 19's cases 3 and 4 in rule 19's own order, which is what the trait-aware source wants.
///
/// Kept as a composition of the two halves rather than the other way round, so that splitting
/// the halves for rule 26's benefit cannot move `--policy individual`: the fall-through from
/// case 3 to case 4 is expressed once, here, and both callers read it from the same place.
fn tolerant_movement_choice(
    observation: &Observation,
    entropy: &mut DecisionEntropy<'_>,
) -> Action {
    tolerant_seek_choice(observation)
        .unwrap_or_else(|| tolerant_search_choice(observation, entropy))
}

/// Rule 19's trait-aware source. It is the reference source's order of business with one
/// substitution: where that source asks whether a resource fits, this asks whether the acting
/// Mokiterion's own tolerance admits it. Two Mokiterions in identical situations therefore
/// propose differently, which is the whole point of `CAP-MOK-006`.
///
/// It reads the trait and nothing else new. Reading the trait grants it no authority: it
/// proposes, and the engine decides under rule 6 with no relaxation.
#[derive(Default)]
struct IndividualDecisionSource;

impl DecisionSource for IndividualDecisionSource {
    fn name(&self) -> &str {
        "individual"
    }

    fn decide(&mut self, observation: &Observation, entropy: &mut DecisionEntropy<'_>) -> Action {
        debug_assert!(observation.is_consistent());
        tolerant_survival_choice(observation)
            .unwrap_or_else(|| tolerant_movement_choice(observation, entropy))
    }
}

/// Rule 26's source, selected by `--policy social`. It reads the acting Mokiterion's own
/// `fear` and its suffered-attack record, both carried on the rule 3 observation, and returns
/// the first applicable branch.
///
/// **The branch order is normative and this function's order is it.** Branches 1, 4 and 5
/// return without a draw; branch 2 returns `eat` or `sleep` and branch 3 a seek step, which
/// rule 19 reaches without a draw; only branch 6 can draw, and it draws exactly what rule 19
/// draws. So this source takes at most one draw per opportunity and never for a social
/// decision.
///
/// Branches 2, 3 and 6 delegate to rule 19 rather than restating it, so a Mokiterion under
/// this source that never meets another behaves exactly as it would under `--policy
/// individual`, trait and all. It introduces no survival constant of its own: the three
/// constants are the two answer thresholds and the engagement threshold, and nothing else.
///
/// Reading `fear` and the record grants the source no authority: it proposes, and the engine
/// decides under rule 6 with no relaxation. That matters most in branch 1, which answers the
/// first attack in the record **whether or not the answer can succeed** — the attacker may
/// have died at a third Mokiterion's hands, and a `fight` may name an attacker that has
/// stepped out of contact. Rule 6 rejects those, the opportunity is spent, and rule 25 clears
/// the record regardless. That is rule 25's asymmetric latency made observable on the trace
/// line rather than hidden, and the technical owner accepted its cost on 2026-08-20.
#[derive(Default)]
struct SocialDecisionSource;

impl DecisionSource for SocialDecisionSource {
    fn name(&self) -> &str {
        "social"
    }

    fn decide(&mut self, observation: &Observation, entropy: &mut DecisionEntropy<'_>) -> Action {
        debug_assert!(observation.is_consistent());

        // Branch 1: an unanswered attack is answered. The record is in resolution order, so
        // "the first attack" is deterministic rather than a choice.
        if let Some(attack) = observation.suffered.first() {
            let target = attack.attacker.clone();
            return if observation.fear >= SURRENDER_FEAR_THRESHOLD {
                Action::Surrender { target }
            } else if observation.fear >= RETREAT_FEAR_THRESHOLD {
                Action::Retreat { target }
            } else {
                Action::Fight { target }
            };
        }

        // Branch 2: survival comes before society. Rule 19's cases 1 and 2, drawing nothing.
        if let Some(action) = tolerant_survival_choice(observation) {
            return action;
        }

        // Branch 3: food perceived outranks company perceived. Rule 19's case 3, drawing
        // nothing — which is what makes its position here a reordering of decisions rather
        // than of the shared stream.
        if let Some(action) = tolerant_seek_choice(observation) {
            return action;
        }

        // Branch 4: a Mokiterion in contact is engaged.
        if let Some(other) = observation.nearest_in_contact() {
            let target = other.id.clone();
            return if observation.fear < ENGAGEMENT_FEAR_THRESHOLD {
                Action::Attack { target }
            } else {
                Action::Threaten { target }
            };
        }

        // Branch 5: a Mokiterion merely perceived is closed on or avoided.
        if let Some(other) = observation.nearest_beyond_contact() {
            let target = other.id.clone();
            return if observation.fear < ENGAGEMENT_FEAR_THRESHOLD {
                Action::Approach { target }
            } else {
                Action::Avoid { target }
            };
        }

        // Branch 6: otherwise rule 19's case 4 decides. The only branch that draws.
        tolerant_search_choice(observation, entropy)
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

/// Why a run ended.
///
/// Public under `SPEC-MOK-002` rule 5: a reported outcome, already emitted in the
/// `simulation_ended` event and the summary line, and required by `RunSummary::reason`.
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

/// Read-only accessors, public under `SPEC-MOK-002` rule 5. Each returns an owned copy of
/// a value the summary line already prints, so a summary cannot be used as a window into
/// live state. The fields stay private and the type stays opaque.
///
/// Rule 5 also authorizes accessors for population per territory and for the resource
/// counts per territory by calorie class. Neither is added: the enumeration is a ceiling
/// rather than a checklist, and no relocated test requires them.
impl RunSummary {
    pub fn reason(&self) -> TerminationReason {
        self.reason
    }

    pub fn ticks(&self) -> u64 {
        self.ticks
    }

    pub fn survivors(&self) -> usize {
        self.survivors
    }

    pub fn deaths(&self) -> usize {
        self.deaths
    }
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

/// The event vocabulary `SPEC-MOK-001` fixes. Fourteen core types plus `action_trace`.
///
/// The type is enumerated rather than left as a string so that a consumer filtering or
/// mapping by type cannot invent a type the engine does not emit, and so that adding a
/// type is a compile error everywhere it is handled.
///
/// The three resolution types exist on one rule: a new type exists where a resolution moves
/// a *second* Mokiterion's state, because that is the transition no other record can carry.
/// `attack_resolved` is shared by `attack` and `fight`, which are one resolution invoked by
/// either verb. `approach`, `avoid` and `retreat` take none, because each mutates only the
/// actor and rule 7's trace and rule 8's crossing event already carry everything they change.
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
    AttackResolved,
    ThreatResolved,
    SurrenderResolved,
    SimulationEnded,
    ActionTrace,
}

impl EventType {
    /// Every type, in a stable order. A consumer cycling a filter through the vocabulary
    /// uses this, so the vocabulary cannot drift out of step with what is emitted. The three
    /// added types are inserted where `SPEC-MOK-001` lists them, after `territory_crossed`,
    /// so that no existing type moves relative to any other.
    pub const ALL: [Self; 15] = [
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
        Self::AttackResolved,
        Self::ThreatResolved,
        Self::SurrenderResolved,
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
            Self::AttackResolved => "attack_resolved",
            Self::ThreatResolved => "threat_resolved",
            Self::SurrenderResolved => "surrender_resolved",
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
        /// Reported once, here, and on no other record kind, because the name cannot change
        /// during a run. First in the detail list, which `SPEC-MOK-001`'s *Data and interface
        /// contracts* fixes: two test suites parse this record positionally, from the front for
        /// the count of initialized Mokiterions and from the back for `waste_tolerance`.
        name: String,
        position: Coordinate,
        territory: Territory,
        health: u8,
        satiety: u8,
        energy: u8,
        fear: u8,
        /// Reported once, here, because the trait cannot change during a run.
        waste_tolerance: u8,
    },
    DecisionSourceSelected {
        source: String,
    },
    SurvivalChanged {
        health: (u8, u8),
        satiety: (u8, u8),
        energy: (u8, u8),
        fear: (u8, u8),
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
    /// Rule 22's one resolution, reported for `attack` and for `fight` alike. The subject is
    /// the striker; this record does not distinguish the two verbs and is not meant to, since
    /// they are one resolution. A reader tells them apart by the pairing and directly from
    /// rule 7's trace, whose `proposal` field carries the verb.
    AttackResolved {
        target: String,
        /// The damage rule 22 computed. It is reported alongside the transition because
        /// saturation at zero makes it unrecoverable from the transition alone.
        damage: u8,
        target_health: (u8, u8),
        striker_energy: (u8, u8),
        /// The death this strike caused, so that death by combat is distinguishable in the
        /// stream from death by starvation without adding a cause field to `agent_died`.
        target_died: bool,
    },
    /// Rule 23's resolution. Nothing but the target's `fear` changes.
    ThreatResolved {
        target: String,
        /// The increase rule 23 *applied*, which is `0` against a target already at
        /// `ATTRIBUTE_MAX`, on the same terms as rule 24's `transferred`. `REQ-MOK-046`
        /// requires the applied amount rather than the constant attempted.
        increase: u8,
        target_fear: (u8, u8),
    },
    /// Rule 24's resolution.
    SurrenderResolved {
        recipient: String,
        /// What reached the recipient.
        transferred: u8,
        /// What the forfeit lost to a full recipient, so that a run's own output shows where
        /// non-conservation occurred rather than leaving it to be inferred.
        discarded: u8,
        subject_satiety: (u8, u8),
        recipient_satiety: (u8, u8),
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
        /// Rule 7 places the trace before survival decay, so this is the value held before
        /// rule 12's update for this tick.
        fear: u8,
        /// Rule 25's record as the source read it, before rule 25 clears it. Rendered only
        /// when non-empty, which `CAP-MOK-009` requires: a field appended unconditionally
        /// would change every `action_trace` line of every `baseline` run.
        ///
        /// Pairs of an identifier and a damage rather than the engine's own
        /// [`SufferedAttack`], because `SPEC-MOK-002`'s enumerated growth adds no item to the
        /// public interface and both halves of a pair are already public values.
        suffered: Vec<(String, u8)>,
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
            Self::AttackResolved { .. } => EventType::AttackResolved,
            Self::ThreatResolved { .. } => EventType::ThreatResolved,
            Self::SurrenderResolved { .. } => EventType::SurrenderResolved,
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
                name,
                position,
                territory,
                health,
                satiety,
                energy,
                fear,
                waste_tolerance,
            } => write!(
                formatter,
                "name:{name},position:{position},territory:{territory},health:{health},satiety:{satiety},energy:{energy},fear:{fear},waste_tolerance:{waste_tolerance}"
            ),
            Self::DecisionSourceSelected { source } => write!(formatter, "source:{source}"),
            Self::SurvivalChanged {
                health,
                satiety,
                energy,
                fear,
            } => write!(
                formatter,
                "health:{}->{},satiety:{}->{},energy:{}->{},fear:{}->{}",
                health.0, health.1, satiety.0, satiety.1, energy.0, energy.1, fear.0, fear.1
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
            Self::AttackResolved {
                target,
                damage,
                target_health,
                striker_energy,
                target_died,
            } => write!(
                formatter,
                "target:{target},damage:{damage},target_health:{}->{},striker_energy:{}->{},target_died:{}",
                target_health.0,
                target_health.1,
                striker_energy.0,
                striker_energy.1,
                if *target_died { "yes" } else { "no" }
            ),
            Self::ThreatResolved {
                target,
                increase,
                target_fear,
            } => write!(
                formatter,
                "target:{target},increase:{increase},target_fear:{}->{}",
                target_fear.0, target_fear.1
            ),
            Self::SurrenderResolved {
                recipient,
                transferred,
                discarded,
                subject_satiety,
                recipient_satiety,
            } => write!(
                formatter,
                "recipient:{recipient},transferred:{transferred},discarded:{discarded},subject_satiety:{}->{},recipient_satiety:{}->{}",
                subject_satiety.0, subject_satiety.1, recipient_satiety.0, recipient_satiety.1
            ),
            Self::SimulationEnded { reason } => write!(formatter, "reason:{reason}"),
            // Two conditionally-present fields, which is why a parser of this line may not
            // assume a fixed field count and must address details by name. `target` is
            // inserted after `proposal` and read from the proposal itself, so the verb and
            // the target cannot disagree; `suffered` is appended last.
            Self::ActionTrace {
                proposal,
                accepted,
                detail,
                position,
                territory,
                health,
                satiety,
                energy,
                fear,
                suffered,
            } => {
                write!(formatter, "proposal:{proposal}")?;
                if let Some(target) = proposal.target() {
                    write!(formatter, ",target:{target}")?;
                }
                write!(
                    formatter,
                    ",status:{},detail:{detail},position:{position},territory:{territory},health:{health},satiety:{satiety},energy:{energy},fear:{fear}",
                    if *accepted { "accepted" } else { "rejected" }
                )?;
                if !suffered.is_empty() {
                    let entries: Vec<String> = suffered
                        .iter()
                        .map(|(attacker, damage)| format!("{attacker}:{damage}"))
                        .collect();
                    write!(formatter, ",suffered:{}", entries.join(";"))?;
                }
                Ok(())
            }
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
    pub fear: u8,
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
    /// Set once the run has ended. `SPEC-MOK-003` rule 1.4 requires a further advance to
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
                // Rule 1 assigns the name at the point the agent is created, by table lookup on
                // the identifier's number. It draws nothing, so it cannot move the placement
                // draws above and every run predating naming is unchanged.
                name: name_of(number),
                position,
                health: ATTRIBUTE_MAX,
                satiety: ATTRIBUTE_MAX,
                energy: ATTRIBUTE_MAX,
                // Rule 12's attribute starts at zero: no Mokiterion begins afraid.
                fear: 0,
                // Rule 1 derives the trait at the point the agent is created. The derivation
                // has a generator of its own, so it cannot move the placement draws above.
                waste_tolerance: derive_waste_tolerance(config.seed, number),
                // Rule 25's window starts empty: nobody has been struck before tick 1.
                suffered: Vec::new(),
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
            Policy::Individual => {
                let mut source = IndividualDecisionSource;
                self.run_with_source(output, &mut source)
            }
            Policy::Social => {
                let mut source = SocialDecisionSource;
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
    /// Advancing a finished run is refused with no state change, as `SPEC-MOK-003` rule 1.4
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
            Policy::Individual => {
                let mut source = IndividualDecisionSource;
                self.advance_tick_with_source(&mut source)
            }
            Policy::Social => {
                let mut source = SocialDecisionSource;
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
    /// `SPEC-MOK-003` rule 8.1 requires of a host presenting provenance.
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
                fear: agent.fear,
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
            Policy::Individual => IndividualDecisionSource.name().to_string(),
            Policy::Social => SocialDecisionSource.name().to_string(),
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
                    name: agent.name.to_string(),
                    position: agent.position,
                    territory: agent.position.territory(),
                    health: agent.health,
                    satiety: agent.satiety,
                    energy: agent.energy,
                    fear: agent.fear,
                    waste_tolerance: agent.waste_tolerance,
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
            // Rule 12 updates `fear` from *this* observation's perceived-Mokiterion list, so
            // the driver is read here and carried, not re-perceived after the action.
            let perceived_company = !observation.perceived_mokiterions.is_empty();
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

            // Rule 25 closes the window when the opportunity is taken rather than when it is
            // used: after the source has been consulted and the proposal applied or rejected,
            // whether it answered, proposed something else, or was rejected. The clearing sits
            // here, past the trace and before rule 12, and it is positioned identically
            // whether or not the flag is set — a clearing after an emission that only
            // sometimes happens would make `--trace-actions` change simulation state.
            self.agents[agent_index].suffered.clear();

            self.apply_survival(output, agent_index, perceived_company)?;
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
            fear: agent.fear,
            waste_tolerance: agent.waste_tolerance,
            suffered: agent.suffered.clone(),
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
            Action::Move { direction } => self.apply_move(output, agent_index, *direction),
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
            Action::Attack { .. }
            | Action::Threaten { .. }
            | Action::Fight { .. }
            | Action::Retreat { .. }
            | Action::Surrender { .. }
            | Action::Approach { .. }
            | Action::Avoid { .. } => self.apply_targeted_action(output, agent_index, action),
        }
    }

    /// Rule 8's move, applied once and reached by two routes: a `move` proposal and rule 21's
    /// three targeted moves, which are "rule 8 moves and nothing more". One implementation is
    /// why a targeted move cannot acquire a cost, a second crossing rule or a different
    /// bounds test.
    fn apply_move<W: Write>(
        &mut self,
        output: &mut W,
        agent_index: usize,
        direction: Direction,
    ) -> io::Result<ActionResult> {
        let current_position = self.agents[agent_index].position;
        let Some(destination) = current_position.moved(direction) else {
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

    /// Rule 20's predicate: two living Mokiterions at a Chebyshev distance of at most `1`.
    ///
    /// Recomputed from current positions and never stored. No field, collection or
    /// observation entry records that two Mokiterions are in contact, and nothing here reaches
    /// for one: it is this arithmetic, evaluated when a rule asks.
    fn in_contact(&self, left_index: usize, right_index: usize) -> bool {
        left_index != right_index
            && self.agents[left_index].alive
            && self.agents[right_index].alive
            && self.agents[left_index]
                .position
                .distance_to(self.agents[right_index].position)
                <= CONTACT_RADIUS
    }

    /// Rule 6's targeted checks 1 to 4, in the order that rule fixes, against authoritative
    /// state and never against the observation the source read. The first unmet condition is
    /// the rejection reason. Check 5 — that the resulting move is valid under rule 8 — belongs
    /// to the three verbs that move and is applied where the move is computed.
    ///
    /// A rejected targeted proposal mutates neither Mokiterion, which this function makes
    /// structural: it takes `&self` and returns an index, so nothing can have changed by the
    /// time a rejection is returned.
    fn validate_targeted(&self, agent_index: usize, action: &Action) -> Result<usize, String> {
        let Some(target) = action.target() else {
            debug_assert!(
                false,
                "validate_targeted is reached only by a targeted verb"
            );
            return Err("target_missing".into());
        };

        // 1. The target exists, lives, and is not the actor — for all seven verbs. A dead
        //    Mokiterion is no longer a valid target, including for a `fight` answering the
        //    attack it made while it lived.
        let Some(target_index) = self.agents.iter().position(|other| other.id == target) else {
            return Err("target_unknown".into());
        };
        if !self.agents[target_index].alive {
            return Err("target_dead".into());
        }
        if target_index == agent_index {
            return Err("target_is_actor".into());
        }

        let distance = self.agents[agent_index]
            .position
            .distance_to(self.agents[target_index].position);

        // 2. The target is perceived — for `approach` and `avoid`.
        if matches!(action, Action::Approach { .. } | Action::Avoid { .. })
            && distance > PERCEPTION_RADIUS
        {
            return Err("target_not_perceived".into());
        }

        // 3. The target is in contact — for `threaten`, `attack` and `fight`. `fight` carries
        //    this precondition as well as the record's, because it is a strike resolving
        //    through the same function `attack` resolves through: an attacker that struck and
        //    then stepped out of contact cannot be fought, and this is the rejection that says
        //    so.
        if matches!(
            action,
            Action::Threaten { .. } | Action::Attack { .. } | Action::Fight { .. }
        ) && !self.in_contact(agent_index, target_index)
        {
            return Err("target_not_in_contact".into());
        }

        // 4. The target is named in the actor's suffered-attack record — for `fight`,
        //    `retreat` and `surrender`. Read from authoritative state, which at this point in
        //    the tick still holds the record the source read: rule 25 clears it after rule 7.
        if matches!(
            action,
            Action::Fight { .. } | Action::Retreat { .. } | Action::Surrender { .. }
        ) && !self.agents[agent_index]
            .suffered
            .iter()
            .any(|attack| attack.attacker == target)
        {
            return Err("target_not_in_record".into());
        }

        Ok(target_index)
    }

    /// Rule 21: a validated targeted proposal applied at the acting Mokiterion's own
    /// opportunity, exactly once, against exactly one target, with no effect deferred to a
    /// later tick.
    ///
    /// No selection among Mokiterions happens here. The proposal arrives naming one target;
    /// choosing between candidates is rule 26's, and applying is this function's.
    fn apply_targeted_action<W: Write>(
        &mut self,
        output: &mut W,
        agent_index: usize,
        action: &Action,
    ) -> io::Result<ActionResult> {
        let target_index = match self.validate_targeted(agent_index, action) {
            Ok(index) => index,
            Err(detail) => {
                return Ok(ActionResult {
                    accepted: false,
                    detail,
                });
            }
        };

        match action {
            Action::Approach { .. } => {
                self.apply_targeted_move(output, agent_index, target_index, false)
            }
            Action::Avoid { .. } | Action::Retreat { .. } => {
                self.apply_targeted_move(output, agent_index, target_index, true)
            }
            Action::Attack { .. } | Action::Fight { .. } => {
                self.resolve_strike(output, agent_index, target_index)
            }
            Action::Threaten { .. } => self.resolve_threat(output, agent_index, target_index),
            Action::Surrender { .. } => self.resolve_surrender(output, agent_index, target_index),
            _ => {
                debug_assert!(
                    false,
                    "apply_targeted_action is reached only by a targeted verb"
                );
                Ok(ActionResult {
                    accepted: false,
                    detail: "target_missing".into(),
                })
            }
        }
    }

    /// Rule 21's `approach`, `avoid` and `retreat`: one cell, one cardinal axis, no additional
    /// energy cost, and a crossing of `y=63/64` handled exactly as any move handles it.
    ///
    /// The axis choice is rule 5 case 3's, unchanged: the horizontal axis while the target's
    /// direction has an easterly or westerly component, otherwise the vertical, and the other
    /// axis when the preferred one is invalid. `away` reverses each step, so the three verbs
    /// share one axis rule and differ only in sign. Where both axes are invalid the proposal is
    /// rejected under rule 6's fifth condition as an invalid move, which is rule 8's own
    /// reason reached by a targeted verb.
    fn apply_targeted_move<W: Write>(
        &mut self,
        output: &mut W,
        agent_index: usize,
        target_index: usize,
        away: bool,
    ) -> io::Result<ActionResult> {
        let position = self.agents[agent_index].position;
        let target_position = self.agents[target_index].position;

        let candidates: Vec<Direction> = match position.direction_to(target_position) {
            Some(direction) => {
                let preferred = direction.horizontal().or_else(|| direction.vertical());
                let alternate = direction.vertical().or_else(|| direction.horizontal());
                [preferred, alternate]
                    .into_iter()
                    .flatten()
                    .map(|step| if away { step.reversed() } else { step })
                    .collect()
            }
            // Distance zero has no direction. Moving away from it still has to mean
            // something, because rejecting it would make co-location an inescapable state, so
            // rule 21 fixes north, then east, then south, then west — rule 5 case 4's own
            // cardinal order, reused so that this case introduces no new ordering, and taken
            // as the first valid direction rather than a selection among them, so it draws
            // nothing. Approaching a Mokiterion already underfoot has no such reading and no
            // resulting move, so rule 6's fifth condition is unmet.
            None if away => Direction::ORDERED.to_vec(),
            None => {
                return Ok(ActionResult {
                    accepted: false,
                    detail: "target_co_located".into(),
                });
            }
        };

        for candidate in candidates {
            if position.moved(candidate).is_some() {
                return self.apply_move(output, agent_index, candidate);
            }
        }

        Ok(ActionResult {
            accepted: false,
            detail: "out_of_bounds".into(),
        })
    }

    /// Rule 22's one resolution, invoked by `attack` and by `fight` alike.
    ///
    /// Damage is `10 + (striker.energy + striker.health) / 10`, evaluated at the moment of
    /// resolution from the striker's own condition and from nothing else — not the target's
    /// attributes, not the tick, not either identifier, not any trait, not any population
    /// aggregate. That is what makes a weakened Mokiterion a weaker attacker without any
    /// Mokiterion reading another's strength. Both terms are bounded by [`ATTRIBUTE_MAX`], so
    /// the range is `10..=30` and no resolution deals `0` by construction rather than by a
    /// clamp. Integer arithmetic only, in `u16`, where no intermediate overflows, and no
    /// entropy is drawn by any part of this.
    fn resolve_strike<W: Write>(
        &mut self,
        output: &mut W,
        striker_index: usize,
        target_index: usize,
    ) -> io::Result<ActionResult> {
        let striker = &self.agents[striker_index];
        let condition = u16::from(striker.energy) + u16::from(striker.health);
        let damage =
            u8::try_from(u16::from(STRIKE_BASE_DAMAGE) + condition / STRIKE_CONDITION_DIVISOR)
                .unwrap_or(u8::MAX);
        debug_assert!((STRIKE_BASE_DAMAGE..=30).contains(&damage));

        let striker_energy_before = striker.energy;
        self.agents[striker_index].energy =
            striker_energy_before.saturating_sub(STRIKE_ENERGY_COST);
        let striker_energy_after = self.agents[striker_index].energy;
        let striker_id = self.agents[striker_index].id.clone();

        let target = &mut self.agents[target_index];
        let target_health_before = target.health;
        target.health = target_health_before.saturating_sub(damage);
        let target_health_after = target.health;
        // Rule 25 opens the window here, carrying the striker and the damage in resolution
        // order. It is pushed whether or not the target survives: rule 13 states that a
        // Mokiterion dying of an attack never reads it and the record dies with it, so this is
        // one path rather than two.
        target.suffered.push(SufferedAttack {
            attacker: striker_id,
            damage,
        });
        let died = target_health_after == 0;
        if died {
            target.alive = false;
        }
        let target_id = target.id.clone();

        let event = Event::new(
            self.tick,
            self.agents[striker_index].id.clone(),
            EventDetail::AttackResolved {
                target: target_id,
                damage,
                target_health: (target_health_before, target_health_after),
                striker_energy: (striker_energy_before, striker_energy_after),
                target_died: died,
            },
        );
        self.emit(output, event)?;

        // Rule 13's path, event and finality. There is no second death and no combat-specific
        // death event; what differs is only that this one happens inside another Mokiterion's
        // turn, so the target may die at a point in the tick where it has not yet acted.
        if died {
            let event = Event::new(
                self.tick,
                self.agents[target_index].id.clone(),
                EventDetail::AgentDied { health: 0 },
            );
            self.emit(output, event)?;
        }

        Ok(ActionResult {
            accepted: true,
            detail: format!("damage:{damage}"),
        })
    }

    /// Rule 23: the target's `fear` rises by [`THREAT_FEAR_INCREASE`] and nothing else
    /// changes — not the target's `health`, `satiety`, `energy` or position, and not the
    /// threatener's anything. No damage, no cost, no transfer, no movement, no death, and no
    /// suffered-attack window: a threatened Mokiterion has not been attacked, so it may not
    /// answer. The only thing a threat costs its maker is the opportunity it spends.
    ///
    /// The reported `increase` is the **effective** one, which is `0` where the target already
    /// stood at [`ATTRIBUTE_MAX`], not the nominal [`THREAT_FEAR_INCREASE`]. `REQ-MOK-046`
    /// requires the increase *applied*, on the same terms as rule 24's `transferred`; a
    /// saturated threat succeeds and reports that it moved nothing. The nominal constant stays
    /// recoverable from the pair of `fear` values the same event carries.
    fn resolve_threat<W: Write>(
        &mut self,
        output: &mut W,
        threatener_index: usize,
        target_index: usize,
    ) -> io::Result<ActionResult> {
        let target = &mut self.agents[target_index];
        let fear_before = target.fear;
        target.fear = fear_before
            .saturating_add(THREAT_FEAR_INCREASE)
            .min(ATTRIBUTE_MAX);
        let fear_after = target.fear;
        let target_id = target.id.clone();
        let increase = fear_after - fear_before;

        let event = Event::new(
            self.tick,
            self.agents[threatener_index].id.clone(),
            EventDetail::ThreatResolved {
                target: target_id,
                increase,
                target_fear: (fear_before, fear_after),
            },
        );
        self.emit(output, event)?;

        Ok(ActionResult {
            accepted: true,
            detail: format!("increase:{increase}"),
        })
    }

    /// Rule 24: the surrendering Mokiterion forfeits `satiety / 2` of its own `satiety`,
    /// derived only from its own `satiety` and from nothing else. The recipient rises by the
    /// same amount, saturating at [`ATTRIBUTE_MAX`], and any excess is destroyed rather than
    /// returned, banked or converted.
    ///
    /// No damage is dealt to either party, no `energy` is paid, neither moves, and no `fear` is
    /// written in either direction. A surrender below `satiety` `2` transfers `0` and still
    /// succeeds: a Mokiterion with nothing to give gives nothing, and it has still declined to
    /// fight. Death by starvation remains rule 12's, so a `satiety` reduced to zero here kills
    /// through the survival path and not through this one.
    fn resolve_surrender<W: Write>(
        &mut self,
        output: &mut W,
        subject_index: usize,
        recipient_index: usize,
    ) -> io::Result<ActionResult> {
        let subject_satiety_before = self.agents[subject_index].satiety;
        let forfeit = subject_satiety_before / 2;
        self.agents[subject_index].satiety = subject_satiety_before.saturating_sub(forfeit);
        let subject_satiety_after = self.agents[subject_index].satiety;

        let recipient = &mut self.agents[recipient_index];
        let recipient_satiety_before = recipient.satiety;
        recipient.satiety = recipient_satiety_before
            .saturating_add(forfeit)
            .min(ATTRIBUTE_MAX);
        let recipient_satiety_after = recipient.satiety;
        let recipient_id = recipient.id.clone();

        let transferred = recipient_satiety_after - recipient_satiety_before;
        let discarded = forfeit - transferred;

        let event = Event::new(
            self.tick,
            self.agents[subject_index].id.clone(),
            EventDetail::SurrenderResolved {
                recipient: recipient_id,
                transferred,
                discarded,
                subject_satiety: (subject_satiety_before, subject_satiety_after),
                recipient_satiety: (recipient_satiety_before, recipient_satiety_after),
            },
        );
        self.emit(output, event)?;

        Ok(ActionResult {
            accepted: true,
            detail: format!("transferred:{transferred}"),
        })
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
                    fear: agent.fear,
                    // Rule 25's record as it stood at this opportunity, which is why the trace
                    // is emitted before the clearing and not after: the field reports the
                    // record the source read, so a reader can see the input the answer branch
                    // acted on. It is empty under every source but rule 26's, and rule 17's
                    // rendering omits it when empty, so no existing stream gains a byte.
                    suffered: agent
                        .suffered
                        .iter()
                        .map(|attack| (attack.attacker.clone(), attack.damage))
                        .collect(),
                },
            )
        };
        self.emit(output, event)
    }

    /// Rule 12: survival decay, then rule 12's `fear` update from the same tick's rule 3
    /// observation. `perceived_company` is whether that observation's perceived-Mokiterion
    /// list held at least one entry — the whole driver, with no distance constant of its own,
    /// because rule 3's list is already bounded by the perception radius.
    fn apply_survival<W: Write>(
        &mut self,
        output: &mut W,
        agent_index: usize,
        perceived_company: bool,
    ) -> io::Result<()> {
        let (event, died) = {
            let agent = &mut self.agents[agent_index];
            let previous_health = agent.health;
            let previous_satiety = agent.satiety;
            let previous_energy = agent.energy;
            let previous_fear = agent.fear;

            agent.satiety = agent.satiety.saturating_sub(SATIETY_DECAY);
            agent.energy = agent.energy.saturating_sub(ENERGY_DECAY);
            if agent.satiety == 0 || agent.energy == 0 {
                agent.health = agent.health.saturating_sub(5);
            }
            // Saturation at both bounds is a normal outcome here, not an error: a Mokiterion
            // in lasting company sits at 100 and one lastingly alone sits at 0.
            agent.fear = if perceived_company {
                agent.fear.saturating_add(FEAR_INCREASE).min(ATTRIBUTE_MAX)
            } else {
                agent.fear.saturating_sub(FEAR_DECREASE)
            };

            let event = Event::new(
                self.tick,
                agent.id.clone(),
                EventDetail::SurvivalChanged {
                    health: (previous_health, agent.health),
                    satiety: (previous_satiety, agent.satiety),
                    energy: (previous_energy, agent.energy),
                    fear: (previous_fear, agent.fear),
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

        // The third argument is rule 12's `fear` driver, whose own saturation is asserted in
        // `fear_saturates_at_both_bounds_and_is_reported_every_tick`. Passing `false` keeps
        // this test's subject the decay of the three attributes it was written for.
        simulation.apply_survival(&mut output, 0, false).unwrap();

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
    // ---- WO-MOK-010: the trait, fear, and the trait-aware source -------------------------

    /// The verification seed set `VER-MOK-002` declares, reused unchanged by `VER-MOK-010` so
    /// that this change's measurements and the control's are taken on the same worlds.
    const DECLARED_SEEDS: [u64; 5] = [0, 1, 42, 123, 777];

    /// The twelve `waste_tolerance` values per declared seed, `M01` first, in the order of
    /// [`DECLARED_SEEDS`].
    ///
    /// `VER-MOK-010` requires a recorded expectation checked into the suite rather than a
    /// re-derivation: a re-derivation restates the implementation and would follow it wherever
    /// it went. These values were computed from `SPEC-MOK-001`'s *Behavioral trait* by a
    /// separate implementation of SplitMix64 and agree with this one on all sixty values.
    ///
    /// Re-recorded on 2026-08-19 when *Behavioral trait* narrowed the range to `0..=40`. The
    /// independent derivation was re-run at the amended bound and agreed again on all sixty; the
    /// negative control in `evidence/WO-MOK-010/negative-control/oracle-2.txt` shows why a
    /// re-derived expectation would not have been worth having.
    const RECORDED_TRAITS: [[u8; 12]; 5] = [
        [6, 8, 8, 5, 4, 32, 15, 10, 39, 18, 20, 37],
        [26, 3, 22, 39, 39, 37, 2, 17, 15, 16, 28, 0],
        [11, 40, 4, 24, 21, 13, 7, 40, 24, 15, 10, 23],
        [20, 33, 40, 13, 35, 19, 40, 35, 24, 0, 19, 4],
        [36, 3, 7, 10, 30, 18, 36, 24, 0, 22, 8, 38],
    ];

    /// The densities `VER-MOK-002` sweeps for resource counts, with the number of values the
    /// shared stream has produced by the end of initialization at each declared seed.
    ///
    /// These counts are what `VER-MOK-010` oracle 2 pins. Initialization places
    /// `2 x resources_per_territory + 12` entities, each from two draws, plus two more for every
    /// coordinate rejected as occupied; at `0.15%` that is `2 x (2 x 12 + 12) = 72` on every
    /// seed, with no rejection anywhere and therefore no slack in which a thirteenth draw could
    /// hide. Twelve trait derivations happen in the middle of the agent placements and contribute
    /// none of these values.
    const INITIALIZATION_DRAWS: [(&str, [u64; 5]); 3] = [
        ("0.15", [72, 72, 72, 72, 72]),
        ("0.75", [270, 268, 268, 268, 268]),
        ("1.50", [516, 516, 516, 514, 516]),
    ];

    fn individual_config(seed: u64, tick_limit: u64, trace_actions: bool) -> Config {
        Config {
            seed,
            tick_limit,
            policy: Policy::Individual,
            density: Density::DEFAULT,
            trace_actions,
        }
    }

    fn decide_individual_once(simulation: &Simulation, agent_index: usize) -> (Action, u32) {
        let observation = simulation.observation(agent_index);
        let mut stream = simulation.entropy;
        let mut entropy = DecisionEntropy::new(&mut stream);
        let action = IndividualDecisionSource.decide(&observation, &mut entropy);
        (action, entropy.draws)
    }

    fn traits_of(simulation: &Simulation) -> Vec<u8> {
        simulation
            .agents
            .iter()
            .map(|agent| agent.waste_tolerance)
            .collect()
    }

    /// How many values the shared stream has produced since it was seeded.
    ///
    /// `SplitMix64` advances its state by one fixed increment per value, so the state alone
    /// carries the count. Recovering it by stepping forward from the seed needs no inverse and
    /// no constant this file does not already hold.
    fn shared_stream_draws(simulation: &Simulation) -> u64 {
        let mut probe = SplitMix64::new(simulation.config.seed);
        for count in 0..10_000 {
            if probe.state == simulation.entropy.state {
                return count;
            }
            probe.next_u64();
        }
        panic!("the shared stream is more than 10,000 values past its seed");
    }

    /// The `fear` transition the named subject's `survival_changed` line reports, per line.
    fn reported_fear_transitions(output: &str, subject: &str) -> Vec<(u8, u8)> {
        output
            .lines()
            .filter(|line| line.contains(&format!("subject={subject} ")))
            .filter(|line| line.contains("event=survival_changed"))
            .map(|line| {
                let field = line
                    .split(",fear:")
                    .nth(1)
                    .expect("every survival_changed line reports fear");
                let (from, to) = field.split_once("->").expect("a transition has two ends");
                (
                    from.parse().expect("the earlier value is an integer"),
                    to.trim_end()
                        .parse()
                        .expect("the later value is an integer"),
                )
            })
            .collect()
    }

    /// Rule 12's driver, constructed: the acting Mokiterion at `origin` with the listed
    /// companions inside its perception and every other Mokiterion far outside it.
    fn with_companions(simulation: &mut Simulation, origin: Coordinate, companions: &[Coordinate]) {
        simulation.agents[0].position = origin;
        // The other territory, past the boundary and past the radius from `origin`, at distinct
        // coordinates so that nothing here depends on shared-cell behavior.
        for (index, agent) in simulation.agents.iter_mut().enumerate().skip(1) {
            agent.position = Coordinate {
                x: u8::try_from(index).expect("twelve fits"),
                y: 120,
            };
        }
        for (offset, position) in companions.iter().enumerate() {
            simulation.agents[offset + 1].position = *position;
        }
        simulation.foods.clear();
    }

    /// Oracle 3's placement dimension: the resource underfoot, then each of the eight relative
    /// directions at distance `1` and at the perception radius, then no resource at all.
    fn enumerated_placements(origin: Coordinate) -> Vec<Option<Coordinate>> {
        const OFFSETS: [(i16, i16); 8] = [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];
        let mut placements = vec![Some(origin)];
        for distance in [1, i16::from(PERCEPTION_RADIUS)] {
            for (offset_x, offset_y) in OFFSETS {
                placements.push(Some(Coordinate {
                    x: u8::try_from(i16::from(origin.x) + offset_x * distance)
                        .expect("the origin is far from every edge"),
                    y: u8::try_from(i16::from(origin.y) + offset_y * distance)
                        .expect("the origin is far from every edge"),
                }));
            }
        }
        placements.push(None);
        placements
    }

    /// `VER-MOK-010` oracle 2: the shared entropy stream's own position, either side of trait
    /// derivation and after initialization as a whole.
    ///
    /// The derivation takes no stream, so the before-and-after pair alone could not fail. The
    /// recorded draw counts are what makes this test able to fail: a derivation that drew from
    /// the shared stream — the design `REQ-MOK-031` forbids — would raise every count by twelve.
    /// The counts themselves are validated by oracle 1, which found the whole event stream
    /// byte-identical to the pre-change capture on all five seeds, so they are the pre-change
    /// positions and not merely this build's own.
    #[test]
    fn trait_derivation_leaves_the_shared_stream_where_it_found_it() {
        for (density, counts) in INITIALIZATION_DRAWS {
            for (index, seed) in DECLARED_SEEDS.into_iter().enumerate() {
                let simulation = Simulation::new(Config {
                    density: Density::parse(density).unwrap(),
                    ..individual_config(seed, 1, false)
                })
                .unwrap();

                assert_eq!(
                    shared_stream_draws(&simulation),
                    counts[index],
                    "the shared stream is not where initialization at seed {seed}, density \
                     {density}% left it before this change"
                );

                // The direct before-and-after form the contract names, for every Mokiterion.
                let before = simulation.entropy;
                for number in 1..=12u8 {
                    let derived = derive_waste_tolerance(seed, number);
                    assert_eq!(
                        simulation.entropy, before,
                        "deriving M{number:02}'s trait at seed {seed} moved the shared stream"
                    );
                    assert!(derived <= WASTE_TOLERANCE_MAX);
                }
                assert_eq!(simulation.entropy, before);
            }
        }
    }

    /// `REQ-MOK-031`: the twelve values, their range, their spread, and their reproducibility
    /// across two independent initializations at one seed.
    #[test]
    fn the_twelve_traits_are_the_recorded_ones_and_are_neither_uniform_nor_out_of_range() {
        for (index, seed) in DECLARED_SEEDS.into_iter().enumerate() {
            let first = Simulation::new(individual_config(seed, 1, false)).unwrap();
            let second = Simulation::new(individual_config(seed, 1, false)).unwrap();
            let values = traits_of(&first);

            assert_eq!(
                values,
                RECORDED_TRAITS[index].to_vec(),
                "seed {seed} no longer derives the recorded traits"
            );
            assert_eq!(
                values,
                traits_of(&second),
                "seed {seed} is not reproducible"
            );
            assert!(
                values.iter().all(|value| *value <= WASTE_TOLERANCE_MAX),
                "seed {seed} derived a value outside 0..={WASTE_TOLERANCE_MAX}: {values:?}"
            );
            let distinct: HashSet<u8> = values.iter().copied().collect();
            assert!(
                distinct.len() > 1,
                "seed {seed} gave all twelve the same trait, which is no individuality at all"
            );
        }

        // Both endpoints of the specified range are attained somewhere in the declared set, so the
        // range is the inclusive one *Behavioral trait* states and not one narrower by a value at
        // either end. This is what an off-by-one in the bounded selection would break, and it is
        // also the check that a later narrowing of the range cannot pass by accident.
        let all: Vec<u8> = RECORDED_TRAITS.iter().flatten().copied().collect();
        assert!(all.contains(&0), "no declared seed derives the lower bound");
        assert!(
            all.contains(&WASTE_TOLERANCE_MAX),
            "no declared seed derives the upper bound of {WASTE_TOLERANCE_MAX}, so the range the \
             tests exercise is narrower than the one specified"
        );
    }

    /// `REQ-MOK-031`: the derivation reads the seed, not the identifier alone. Were it otherwise
    /// every world would hold the same twelve personalities.
    #[test]
    fn the_trait_reads_the_seed_and_not_only_the_identifier() {
        let first = derive_waste_tolerance(DECLARED_SEEDS[0], 1);
        assert!(
            DECLARED_SEEDS
                .into_iter()
                .any(|seed| derive_waste_tolerance(seed, 1) != first),
            "M01 holds the same trait at every declared seed"
        );
    }

    /// `REQ-MOK-031`: fixed for the run, and a property of the Mokiterion rather than of the
    /// configuration. The same twelve values under every source, at every swept density, at
    /// every tick limit, and unchanged after a thousand ticks of living and dying.
    #[test]
    fn the_trait_is_fixed_for_the_run_and_independent_of_every_configuration() {
        let expected = RECORDED_TRAITS[2].to_vec();
        let seed = DECLARED_SEEDS[2];

        for policy in [
            Policy::Baseline,
            Policy::Reference,
            Policy::Individual,
            Policy::Social,
        ] {
            for density in ["0.15", "0.75", "1.50"] {
                for tick_limit in [1, 37] {
                    let simulation = Simulation::new(Config {
                        seed,
                        tick_limit,
                        policy,
                        density: Density::parse(density).unwrap(),
                        trace_actions: false,
                    })
                    .unwrap();
                    assert_eq!(
                        traits_of(&simulation),
                        expected,
                        "{policy} at density {density}% over {tick_limit} ticks derived other \
                         traits"
                    );
                }
            }
        }

        // A thousand ticks later the values are the same ones, for the dead as for the living:
        // the field has one writer and it runs once, at initialization.
        let mut simulation = Simulation::new(individual_config(seed, 1_000, false)).unwrap();
        simulation.run(&mut io::sink()).unwrap();
        assert_eq!(traits_of(&simulation), expected);
    }

    /// `VER-MOK-010` oracle 3: at the trait's lower bound the trait-aware source proposes what
    /// the reference source proposes, over an enumerated situation set rather than a sampled one.
    ///
    /// The set is the product of five dimensions:
    ///
    /// - thirteen satiety values straddling every clipping boundary the food table produces —
    ///   `85` for the low class, `70` for the medium and `50` for the high — with one value below,
    ///   at and above each, plus both ends of the range and one value between the boundaries;
    /// - the three calorie classes;
    /// - eighteen placements: underfoot, then each of the eight relative directions at distance
    ///   `1` and at the perception radius, then nothing perceived at all, which is the case that
    ///   makes the search fallback reachable;
    /// - two energy values, one below rule 19 case 2's threshold and one at it, so the sleep
    ///   branch is taken in half the set and skipped in the other;
    /// - two companion states, with and without a low-class resource underfoot, so case 1's
    ///   tie-break and case 3's fallthrough are both reached.
    ///
    /// Its size is asserted below so that the set cannot silently shrink. Both sources are given
    /// the identical observation and a copy of the identical stream, so the search step's
    /// selection is comparable too, and the streams are compared after the proposal: two sources
    /// that agreed on the action while consuming different amounts of entropy would diverge on
    /// the next tick.
    #[test]
    fn at_tolerance_zero_the_trait_aware_source_proposes_what_the_reference_source_proposes() {
        const SATIETIES: [u8; 13] = [0, 49, 50, 51, 60, 69, 70, 71, 84, 85, 86, 99, 100];

        let mut simulation = Simulation::new(individual_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        // Far from every edge, so every placement below is in bounds and in territory A, and
        // far from the other eleven, so that only resources are perceived.
        let origin = Coordinate { x: 60, y: 30 };
        with_companions(&mut simulation, origin, &[]);
        simulation.agents[0].waste_tolerance = 0;
        let placements = enumerated_placements(origin);

        let mut cases = 0usize;
        for satiety in SATIETIES {
            for class in FoodClass::ALL {
                for placement in &placements {
                    for energy in [REFERENCE_SLEEP_THRESHOLD - 1, REFERENCE_SLEEP_THRESHOLD] {
                        for companion in [false, true] {
                            simulation.agents[0].satiety = satiety;
                            simulation.agents[0].energy = energy;
                            simulation.foods.clear();
                            if let Some(position) = *placement {
                                simulation.foods.push(Food {
                                    id: "F0001".into(),
                                    position,
                                    class,
                                });
                            }
                            if companion {
                                simulation.foods.push(Food {
                                    id: "F0002".into(),
                                    position: origin,
                                    class: FoodClass::Low,
                                });
                            }

                            let observation = simulation.observation(0);
                            let mut reference_stream = simulation.entropy;
                            let mut individual_stream = simulation.entropy;
                            let reference = ReferenceDecisionSource.decide(
                                &observation,
                                &mut DecisionEntropy::new(&mut reference_stream),
                            );
                            let individual = IndividualDecisionSource.decide(
                                &observation,
                                &mut DecisionEntropy::new(&mut individual_stream),
                            );

                            let case = format!(
                                "satiety {satiety}, {class} class, placement {placement:?}, \
                                 energy {energy}, companion {companion}"
                            );
                            assert_eq!(individual, reference, "{case}");
                            assert_eq!(individual_stream, reference_stream, "{case}");
                            cases += 1;
                        }
                    }
                }
            }
        }

        assert_eq!(cases, SATIETIES.len() * 3 * placements.len() * 2 * 2);
        assert_eq!(cases, 2_808, "the enumerated situation set changed size");
    }

    /// `REQ-MOK-033`: a trait difference alone changes the proposal, in both of rule 19's worked
    /// cases as amended on 2026-08-19.
    ///
    /// The medium-class case is the interior one: at satiety `80` a medium-class resource restores
    /// `30` and wastes `10`, which the tolerant test admits when `10 <= T * 30 / 100`, so at
    /// `T = 34` (`1020 / 100 = 10`) and not at `T = 33` (`990 / 100 = 9`). **The pair either side
    /// of `34` is what pins the division as truncating rather than rounding**, and neither value is
    /// near the range's ends, so it survives a further narrowing.
    ///
    /// The high-class case sits exactly at the range's upper bound: at satiety `70` the waste is
    /// `20` and `40 * 50 / 100 = 20` admits it while `39 * 50 / 100 = 19` does not. A resource of
    /// that class at satiety `80` is declined at every reachable tolerance, which is the effect the
    /// narrowing was made to produce, so that is asserted too.
    #[test]
    fn a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten() {
        let underfoot = Coordinate { x: 60, y: 30 };
        let expected = Action::Eat {
            food_id: "F0001".into(),
        };

        for (class, satiety, admits, declines) in [
            (FoodClass::Medium, 80u8, 34u8, 33u8),
            (
                FoodClass::High,
                70,
                WASTE_TOLERANCE_MAX,
                WASTE_TOLERANCE_MAX - 1,
            ),
        ] {
            let mut simulation = Simulation::new(individual_config(0, 1, false)).unwrap();
            simulation.tick = 1;
            for agent in &mut simulation.agents {
                agent.position = underfoot;
                agent.satiety = satiety;
                agent.energy = ATTRIBUTE_MAX;
            }
            simulation.foods = vec![Food {
                id: "F0001".into(),
                position: underfoot,
                class,
            }];
            simulation.agents[0].waste_tolerance = admits;
            simulation.agents[1].waste_tolerance = declines;
            simulation.agents[2].waste_tolerance = 0;
            simulation.agents[3].waste_tolerance = admits;

            let (tolerant, draws) = decide_individual_once(&simulation, 0);
            assert_eq!(
                tolerant, expected,
                "tolerance {admits} declined a {class}-class resource at satiety {satiety}"
            );
            assert_eq!(draws, 0, "eating consumes no entropy");

            for index in [1, 2] {
                let tolerance = simulation.agents[index].waste_tolerance;
                let (rejected, draws) = decide_individual_once(&simulation, index);
                assert!(
                    matches!(rejected, Action::Move { .. }),
                    "tolerance {tolerance} accepted the waste of a {class}-class resource at \
                     satiety {satiety}, proposing {rejected}"
                );
                assert_eq!(draws, 1, "a Mokiterion with nothing to eat searches");
            }

            // Equal traits in situations identical in everything rule 19 reads propose identically.
            let (same, _) = decide_individual_once(&simulation, 3);
            assert_eq!(same, tolerant);
        }

        // The narrowing's intended effect: a high-class resource at satiety 80 wastes 30, and no
        // tolerance the amended range can produce admits it, because `40 * 50 / 100 = 20 < 30`.
        let mut simulation = Simulation::new(individual_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        with_companions(&mut simulation, underfoot, &[]);
        simulation.agents[0].satiety = 80;
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position: underfoot,
            class: FoodClass::High,
        }];
        for tolerance in 0..=WASTE_TOLERANCE_MAX {
            simulation.agents[0].waste_tolerance = tolerance;
            let (proposal, _) = decide_individual_once(&simulation, 0);
            assert!(
                matches!(proposal, Action::Move { .. }),
                "tolerance {tolerance} ate a high-class resource at satiety 80, wasting 30, which \
                 the amended range was narrowed to prevent"
            );
        }
    }

    /// Rule 19's *The test governs cases 1 and 3 alike*, in the form rule 5's own test is held
    /// to: the resource a tolerance declines underfoot must not become the target of the next
    /// step. Only the search step consumes entropy, so the draw count separates a deliberate
    /// approach from a search without naming how either source ranks candidates.
    #[test]
    fn the_tolerant_test_governs_seeking_as_well_as_eating() {
        let mut simulation = Simulation::new(individual_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        let underfoot = Coordinate { x: 50, y: 30 };
        with_companions(&mut simulation, underfoot, &[]);
        // Rule 19's medium-class worked case, whose boundary at 34 is interior to the amended
        // range: tolerance 33 declines a waste of 10 and 34 admits it.
        simulation.agents[0].satiety = 80;
        simulation.agents[0].waste_tolerance = 33;
        simulation.foods = vec![Food {
            id: "F0001".into(),
            position: underfoot,
            class: FoodClass::Medium,
        }];

        let (declined, draws) = decide_individual_once(&simulation, 0);
        assert!(matches!(declined, Action::Move { .. }), "got {declined}");
        assert_eq!(draws, 1, "with nothing worth eating the step is a search");

        // Standing one cell away, the resource it just declined must not be re-targeted.
        simulation.agents[0].position = Coordinate { x: 49, y: 30 };
        let (adjacent, draws) = decide_individual_once(&simulation, 0);
        assert_eq!(
            draws, 1,
            "the cell just left must not be re-targeted, got {adjacent}"
        );

        // The tolerance is a filter and not a blanket: one point higher and the same resource is
        // both approached and eaten, neither of which consumes entropy.
        simulation.agents[0].waste_tolerance = 34;
        let (approach, draws) = decide_individual_once(&simulation, 0);
        assert_eq!(
            approach,
            Action::Move {
                direction: Direction::East
            }
        );
        assert_eq!(draws, 0, "approaching must not consume entropy");

        simulation.agents[0].position = underfoot;
        let (eaten, draws) = decide_individual_once(&simulation, 0);
        assert_eq!(
            eaten,
            Action::Eat {
                food_id: "F0001".into()
            }
        );
        assert_eq!(draws, 0);
    }

    /// `REQ-MOK-032`: the driver is rule 3's perceived-Mokiterion list, so the boundary is
    /// perception's own. Constructed at Chebyshev distance `16` and at `17`, one cell apart: an
    /// off-by-one here is an off-by-one in perception.
    #[test]
    fn fear_rises_at_the_perception_boundary_and_decays_one_cell_beyond_it() {
        for (distance, expected) in [
            (PERCEPTION_RADIUS, (20, 20 + FEAR_INCREASE)),
            (PERCEPTION_RADIUS + 1, (20, 20 - FEAR_DECREASE)),
        ] {
            let mut simulation = Simulation::new(individual_config(0, 1, false)).unwrap();
            simulation.tick = 1;
            let origin = Coordinate { x: 60, y: 30 };
            with_companions(
                &mut simulation,
                origin,
                &[Coordinate {
                    x: origin.x + distance,
                    y: origin.y,
                }],
            );
            simulation.agents[0].fear = 20;
            let mut output = Vec::new();

            simulation
                .run_tick(&mut output, &mut IndividualDecisionSource)
                .unwrap();

            let output = String::from_utf8(output).unwrap();
            assert_eq!(
                reported_fear_transitions(&output, "M01"),
                vec![expected],
                "at Chebyshev distance {distance}"
            );
        }
    }

    /// `REQ-MOK-032`: the step is fixed. How many are perceived, how far away and in which
    /// direction do not enter it, because the update reads whether rule 3's list is empty and
    /// nothing else about it.
    #[test]
    fn fear_ignores_how_many_are_perceived_how_far_and_in_which_direction() {
        let origin = Coordinate { x: 60, y: 30 };
        let cases: [(&str, Vec<Coordinate>); 5] = [
            (
                "one companion one cell east",
                vec![Coordinate { x: 61, y: 30 }],
            ),
            (
                "one companion at the radius",
                vec![Coordinate { x: 76, y: 30 }],
            ),
            (
                "one companion to the north west",
                vec![Coordinate { x: 55, y: 25 }],
            ),
            ("one companion due south", vec![Coordinate { x: 60, y: 38 }]),
            (
                "four companions at four distances",
                vec![
                    Coordinate { x: 61, y: 30 },
                    Coordinate { x: 60, y: 25 },
                    Coordinate { x: 48, y: 42 },
                    Coordinate { x: 76, y: 46 },
                ],
            ),
        ];

        for (case, companions) in cases {
            let mut simulation = Simulation::new(individual_config(0, 1, false)).unwrap();
            simulation.tick = 1;
            with_companions(&mut simulation, origin, &companions);
            simulation.agents[0].fear = 20;
            let mut output = Vec::new();

            simulation
                .run_tick(&mut output, &mut IndividualDecisionSource)
                .unwrap();

            let output = String::from_utf8(output).unwrap();
            assert_eq!(
                reported_fear_transitions(&output, "M01"),
                vec![(20, 20 + FEAR_INCREASE)],
                "{case}"
            );
        }
    }

    /// `REQ-MOK-032`: saturation at both bounds over many consecutive ticks, and a transition
    /// reported on every one of them — including `0->0`, which is the lower bound holding rather
    /// than a missing update. In a debug build a wrap would be a panic, so this also covers the
    /// requirement's no-wrap clause.
    #[test]
    fn fear_saturates_at_both_bounds_and_is_reported_every_tick() {
        let mut simulation = Simulation::new(individual_config(0, 40, false)).unwrap();
        simulation.tick = 1;
        let mut output = Vec::new();

        // Ten increments of ten reach the upper bound; the eleventh tick must hold there.
        for _ in 0..11 {
            simulation.apply_survival(&mut output, 0, true).unwrap();
            assert!(simulation.agents[0].fear <= ATTRIBUTE_MAX);
        }
        assert_eq!(simulation.agents[0].fear, ATTRIBUTE_MAX);

        // Twenty decrements of five reach the lower bound; the twenty-first must hold there
        // rather than wrap to 251.
        for _ in 0..21 {
            simulation.apply_survival(&mut output, 0, false).unwrap();
            assert!(simulation.agents[0].fear <= ATTRIBUTE_MAX);
        }
        assert_eq!(simulation.agents[0].fear, 0);

        let output = String::from_utf8(output).unwrap();
        let transitions = reported_fear_transitions(&output, "M01");
        assert_eq!(transitions.len(), 32, "every tick reports a transition");
        assert_eq!(
            transitions
                .iter()
                .filter(|pair| **pair == (100, 100))
                .count(),
            1,
            "the upper bound holds exactly once in this sequence"
        );
        assert_eq!(
            transitions.iter().filter(|pair| **pair == (0, 0)).count(),
            1,
            "the lower bound holds exactly once in this sequence"
        );
        assert!(
            simulation.agents[0].alive,
            "this test must exercise fear, not death"
        );
    }

    /// `REQ-MOK-032`: rule 7 places the trace before survival decay, so the traced value is the
    /// one the survival record then changes.
    #[test]
    fn the_trace_reports_the_fear_the_survival_record_then_changes() {
        let mut simulation = Simulation::new(individual_config(0, 1, true)).unwrap();
        simulation.tick = 1;
        with_companions(
            &mut simulation,
            Coordinate { x: 60, y: 30 },
            &[Coordinate { x: 61, y: 30 }],
        );
        simulation.agents[0].fear = 20;
        let mut output = Vec::new();

        simulation
            .run_tick(&mut output, &mut IndividualDecisionSource)
            .unwrap();

        let output = String::from_utf8(output).unwrap();
        let trace = output
            .lines()
            .find(|line| line.contains("subject=M01 ") && line.contains("event=action_trace"))
            .expect("tracing is on");
        assert!(trace.ends_with(",fear:20"), "{trace}");
        assert_eq!(
            reported_fear_transitions(&output, "M01"),
            vec![(20, 30)],
            "{output}"
        );
    }

    /// `REQ-MOK-032`: a dead Mokiterion has no fear to report and reports nothing at all.
    #[test]
    fn a_dead_mokiterion_reports_no_fear_and_no_decision() {
        let mut simulation = Simulation::new(individual_config(0, 2, true)).unwrap();
        simulation.tick = 1;
        with_companions(
            &mut simulation,
            Coordinate { x: 60, y: 30 },
            &[Coordinate { x: 61, y: 30 }],
        );
        simulation.agents[0].health = 5;
        simulation.agents[0].satiety = 1;
        let mut output = Vec::new();

        simulation.apply_survival(&mut output, 0, true).unwrap();
        assert!(!simulation.agents[0].alive);
        assert!(
            String::from_utf8(output)
                .unwrap()
                .contains("event=agent_died")
        );

        simulation.tick = 2;
        let mut output = Vec::new();
        simulation
            .run_tick(&mut output, &mut IndividualDecisionSource)
            .unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(!output.contains("subject=M01 "), "{output}");
        assert!(output.contains("subject=M02 "));
        assert!(
            !simulation
                .snapshot()
                .agents
                .iter()
                .any(|agent| agent.id == "M01")
        );
    }

    /// `REQ-MOK-033`: the third source is reported once, its runs reproduce byte for byte, and
    /// they differ from the reference source's, as a source with behavior of its own must.
    #[test]
    fn individual_runs_are_reported_and_byte_identically_reproducible() {
        for seed in DECLARED_SEEDS {
            let mut first = Simulation::new(individual_config(seed, 200, true)).unwrap();
            let mut second = Simulation::new(individual_config(seed, 200, true)).unwrap();
            let mut reference = Simulation::new(reference_config(seed, 200, true)).unwrap();
            let mut first_output = Vec::new();
            let mut second_output = Vec::new();
            let mut reference_output = Vec::new();

            let summary = first.run(&mut first_output).unwrap();
            assert_eq!(summary, second.run(&mut second_output).unwrap());
            reference.run(&mut reference_output).unwrap();

            assert_eq!(state_snapshot(&first), state_snapshot(&second));
            assert_eq!(
                first_output, second_output,
                "seed {seed} is not reproducible"
            );
            assert_ne!(
                first_output, reference_output,
                "seed {seed} produced the reference source's stream exactly"
            );

            let text = String::from_utf8(first_output).unwrap();
            assert_eq!(
                text.matches("event=decision_source_selected result=source:individual")
                    .count(),
                1
            );
        }
    }

    // ---- WO-MOK-011: the name ------------------------------------------------------------

    /// `REQ-MOK-040`: the table's domain and its two distinctness properties.
    ///
    /// The names themselves are the product owner's decision and are asserted as the literal they
    /// are, so an edit to [`NAMES`] fails here rather than silently changing what every run
    /// reports. The properties are asserted separately from the literal, because it is the
    /// properties the observer's glyph assignment and the roster's column budget depend on.
    #[test]
    fn the_names_are_the_specified_twelve() {
        const SPECIFIED: [&str; 12] = [
            "Zug", "Krul", "Quib", "Sput", "Trok", "Womp", "Hozz", "Nurb", "Vonk", "Gorm", "Xob",
            "Drix",
        ];
        assert_eq!(NAMES, SPECIFIED);

        // Total on `1..=12` and injective: every identifier gets a name and no two share one.
        let assigned: Vec<&str> = (1..=12u8).map(name_of).collect();
        assert_eq!(assigned, SPECIFIED.to_vec());
        let distinct: std::collections::BTreeSet<&str> = assigned.iter().copied().collect();
        assert_eq!(distinct.len(), 12, "two Mokiterions share a name");

        // The first characters are distinct, which is what `SPEC-MOK-003` rule 2's glyph
        // assignment rests on and what makes twelve glyphs tell twelve subjects apart.
        let initials: std::collections::BTreeSet<char> = assigned
            .iter()
            .map(|name| name.chars().next().expect("no name is empty"))
            .collect();
        assert_eq!(initials.len(), 12, "two names share a first character");

        for name in assigned {
            assert!(
                (1..=5).contains(&name.len()),
                "{name} is outside the one-to-five character bound"
            );
            assert!(
                name.chars()
                    .all(|character| character.is_ascii_alphabetic()),
                "{name} carries a character outside the ASCII letters"
            );
            // A name is not an identifier: nothing that reads one can be satisfied by the other.
            assert!(
                !(1..=12u8).any(|number| format!("M{number:02}") == name),
                "{name} is also an identifier"
            );
        }

        // The table holds exactly as many entries as a run creates Mokiterions. The population is
        // read from a run rather than restated as a literal, because the engine states it as the
        // range the initialization loop walks and has no constant to name: an edit that created a
        // thirteenth Mokiterion would panic in `name_of`, and one that added a thirteenth name
        // without a Mokiterion to hold it would fail here.
        let simulation = Simulation::new(individual_config(42, 1, false)).unwrap();
        assert_eq!(NAMES.len(), simulation.agents.len());
    }

    /// `REQ-MOK-040`: a name is the same value at both ends of a run, including for a Mokiterion
    /// that died during it.
    ///
    /// That no second report is emitted is `tests/naming.rs`'s subject. Immutability is this tier's,
    /// because it is a claim about the value the engine holds rather than about what it printed: the
    /// field has one writer, at initialization, and a run that wrote it again would satisfy every
    /// public-tier assertion here and fail this one.
    #[test]
    fn a_name_is_the_same_value_at_both_ends_of_a_run() {
        for policy in [
            Policy::Baseline,
            Policy::Reference,
            Policy::Individual,
            Policy::Social,
        ] {
            let mut simulation = Simulation::new(Config {
                policy,
                ..individual_config(42, 1_000, true)
            })
            .unwrap();
            let before: Vec<(String, &str)> = simulation
                .agents
                .iter()
                .map(|agent| (agent.id.clone(), agent.name))
                .collect();

            let mut sink = Vec::new();
            simulation.run(&mut sink).expect("the run completes");

            let after: Vec<(String, &str)> = simulation
                .agents
                .iter()
                .map(|agent| (agent.id.clone(), agent.name))
                .collect();
            assert_eq!(before, after, "a name moved during a {policy} run");

            // Death does not release a name: the dead keep theirs, which is what lets the observer
            // go on naming a subject it can no longer see.
            let dead: Vec<&str> = simulation
                .agents
                .iter()
                .filter(|agent| !agent.alive)
                .map(|agent| agent.name)
                .collect();
            for name in dead {
                assert!(NAMES.contains(&name));
            }
        }
    }

    /// `REQ-MOK-040`: naming performs no draw, and reads neither the seed nor the configuration.
    ///
    /// The recorded stream positions are [`INITIALIZATION_DRAWS`], the same expectations
    /// `VER-MOK-007` oracle 2 pinned before naming existed. A name obtained by drawing — from the
    /// shared stream or from a side generator seeded by the run — would move them, or would make
    /// the assignment differ between two seeds.
    #[test]
    fn naming_draws_nothing_and_reads_neither_the_seed_nor_the_configuration() {
        let reference: Vec<&str> = (1..=12u8).map(name_of).collect();

        for (density, counts) in INITIALIZATION_DRAWS {
            for (index, seed) in DECLARED_SEEDS.into_iter().enumerate() {
                for policy in [
                    Policy::Baseline,
                    Policy::Reference,
                    Policy::Individual,
                    Policy::Social,
                ] {
                    let simulation = Simulation::new(Config {
                        density: Density::parse(density).unwrap(),
                        policy,
                        ..individual_config(seed, 1, false)
                    })
                    .unwrap();

                    // The stream is exactly where it was before naming was added.
                    assert_eq!(
                        shared_stream_draws(&simulation),
                        counts[index],
                        "naming moved the shared stream at seed {seed}, density {density}%"
                    );

                    // The same twelve names, in the same order, whatever the run is.
                    let names: Vec<&str> =
                        simulation.agents.iter().map(|agent| agent.name).collect();
                    assert_eq!(
                        names, reference,
                        "the assignment changed at seed {seed}, density {density}%"
                    );
                    // And each is the one belonging to its own identifier.
                    for (number, agent) in (1..=12u8).zip(&simulation.agents) {
                        assert_eq!(agent.id, format!("M{number:02}"));
                        assert_eq!(agent.name, name_of(number));
                    }
                }
            }
        }
    }

    /// `REQ-MOK-040`: the record the engine writes carries the name it assigned, so the assignment
    /// and the report cannot drift apart.
    ///
    /// Only the *link* is asserted here, from a private field to a public record. What the record's
    /// text is, that the name is reported once and nowhere else, and that `waste_tolerance` is
    /// still last, are all writable through rule 5's interface and are therefore
    /// `tests/naming.rs`'s under rule 7's placement rule.
    #[test]
    fn the_reported_record_carries_the_name_the_agent_holds() {
        let simulation = Simulation::new(individual_config(42, 1, false)).unwrap();
        let reported: Vec<String> = simulation
            .initialization_events()
            .iter()
            .filter_map(|event| match &event.detail {
                EventDetail::AgentInitialized { name, .. } => Some(name.clone()),
                _ => None,
            })
            .collect();

        let held: Vec<String> = simulation
            .agents
            .iter()
            .map(|agent| agent.name.to_string())
            .collect();
        assert_eq!(reported, held);
        assert_eq!(reported.len(), 12);
    }

    // ---- WO-MOK-012: contact, conflict and society ----------------------------------------

    fn social_config(seed: u64, tick_limit: u64, trace_actions: bool) -> Config {
        Config {
            policy: Policy::Social,
            ..individual_config(seed, tick_limit, trace_actions)
        }
    }

    fn decide_social_once(simulation: &Simulation, agent_index: usize) -> (Action, u32) {
        let observation = simulation.observation(agent_index);
        let mut stream = simulation.entropy;
        let mut entropy = DecisionEntropy::new(&mut stream);
        let action = SocialDecisionSource.decide(&observation, &mut entropy);
        (action, entropy.draws)
    }

    /// A constructed encounter: `M01` and `M02` where the test puts them, the other ten
    /// Mokiterions beyond every perception radius involved, and no resource anywhere.
    ///
    /// The resources are removed rather than worked around. Rule 26 puts survival ahead of
    /// society, so a resource underfoot is a legitimate answer that would displace the branch
    /// under test; a world with none makes the branch that fires the branch the test is about.
    /// `tick` is `1` because rule 3's consistency test requires a started run.
    fn encounter(seed: u64, first: Coordinate, second: Coordinate) -> Simulation {
        let mut simulation = Simulation::new(social_config(seed, 1_000, false)).unwrap();
        simulation.tick = 1;
        simulation.foods.clear();
        simulation.agents[0].position = first;
        simulation.agents[1].position = second;
        for (index, agent) in simulation.agents.iter_mut().enumerate().skip(2) {
            agent.position = Coordinate {
                x: index as u8 * 4,
                y: WORLD_SIZE - 8,
            };
        }
        simulation
    }

    /// The seven targeted verbs, all naming `target`. Rule 6's first three conditions hold for
    /// all seven, so the assertions about them are made for all seven rather than for a
    /// representative one.
    fn targeted_proposals(target: &str) -> [Action; 7] {
        let target = target.to_string();
        [
            Action::Attack {
                target: target.clone(),
            },
            Action::Threaten {
                target: target.clone(),
            },
            Action::Fight {
                target: target.clone(),
            },
            Action::Retreat {
                target: target.clone(),
            },
            Action::Surrender {
                target: target.clone(),
            },
            Action::Approach {
                target: target.clone(),
            },
            Action::Avoid { target },
        ]
    }

    /// Rule 25's window, opened by hand where a test's subject is what reads it rather than
    /// what wrote it. The damage is rule 22's minimum, which rule 3's consistency test
    /// requires of every entry.
    fn suffered_from(attacker: &str) -> Vec<SufferedAttack> {
        vec![SufferedAttack {
            attacker: attacker.to_string(),
            damage: STRIKE_BASE_DAMAGE,
        }]
    }

    /// Rule 20: contact is a Chebyshev distance of at most one between two living
    /// Mokiterions, evaluated when a rule asks it.
    ///
    /// Nothing is set up before the probes and nothing is torn down between them, because
    /// there is no contact state to set up: each probe moves a position and asks again without
    /// telling the engine anything, which a stored contact set would fail.
    #[test]
    fn contact_is_recomputed_from_position_and_needs_two_living_neighbors() {
        let mut simulation = encounter(0, Coordinate { x: 10, y: 10 }, Coordinate { x: 10, y: 10 });

        for (offset_x, offset_y, expected) in [
            (0u8, 0u8, true),
            (1, 0, true),
            (0, 1, true),
            (1, 1, true),
            (2, 0, false),
            (0, 2, false),
            (1, 2, false),
            (2, 2, false),
        ] {
            simulation.agents[1].position = Coordinate {
                x: 10 + offset_x,
                y: 10 + offset_y,
            };
            assert_eq!(
                simulation.in_contact(0, 1),
                expected,
                "offset {offset_x},{offset_y}"
            );
            // Chebyshev distance is symmetric, so the predicate is.
            assert_eq!(
                simulation.in_contact(1, 0),
                expected,
                "offset {offset_x},{offset_y}, reversed"
            );
        }

        // The metric is the one `SPEC-MOK-001` names and not a near neighbor of it: a diagonal
        // step is in contact, which a Manhattan reading at radius one would exclude, and a
        // knight's move is not, which a Euclidean reading at radius two would include.
        simulation.agents[1].position = Coordinate { x: 11, y: 11 };
        assert!(simulation.in_contact(0, 1));
        simulation.agents[1].position = Coordinate { x: 12, y: 11 };
        assert!(!simulation.in_contact(0, 1));

        // No Mokiterion is in contact with itself, at any distance from itself.
        assert!(!simulation.in_contact(0, 0));

        // And the dead are in contact with nobody, either way round.
        simulation.agents[1].position = simulation.agents[0].position;
        assert!(simulation.in_contact(0, 1));
        simulation.agents[1].alive = false;
        assert!(!simulation.in_contact(0, 1));
        assert!(!simulation.in_contact(1, 0));
        simulation.agents[1].alive = true;
        simulation.agents[0].alive = false;
        assert!(!simulation.in_contact(0, 1));
    }

    /// Rule 6's targeted conditions, each named by the ground the engine states.
    ///
    /// Every assertion goes through [`Simulation::validate_targeted`], which is where the
    /// order lives, and the cases where two conditions are unmet at once are what pin that
    /// order rather than merely exercising it.
    #[test]
    fn every_targeted_rejection_names_the_first_unmet_condition() {
        let mut simulation = encounter(0, Coordinate { x: 10, y: 10 }, Coordinate { x: 40, y: 10 });

        // 1. The target exists, lives and is not the actor. All seven verbs carry these three,
        //    and a dead target is reported as dead even though it is also out of contact and
        //    out of the record.
        for verb in targeted_proposals("M99") {
            assert_eq!(
                simulation.validate_targeted(0, &verb),
                Err("target_unknown".into()),
                "{verb}"
            );
        }
        for verb in targeted_proposals("M01") {
            assert_eq!(
                simulation.validate_targeted(0, &verb),
                Err("target_is_actor".into()),
                "{verb}"
            );
        }
        simulation.agents[1].alive = false;
        for verb in targeted_proposals("M02") {
            assert_eq!(
                simulation.validate_targeted(0, &verb),
                Err("target_dead".into()),
                "{verb}"
            );
        }
        simulation.agents[1].alive = true;

        // 2. `approach` and `avoid` require perception, at the radius itself and not one cell
        //    beyond it. `retreat` and `surrender` answer an attacker that may since have
        //    walked away, so neither reads perception: both are rejected here for the record
        //    alone, at both distances.
        for distance in [PERCEPTION_RADIUS, PERCEPTION_RADIUS + 1] {
            simulation.agents[1].position = Coordinate {
                x: 10 + distance,
                y: 10,
            };
            for verb in [
                Action::Approach {
                    target: "M02".into(),
                },
                Action::Avoid {
                    target: "M02".into(),
                },
            ] {
                let outcome = simulation.validate_targeted(0, &verb);
                if distance > PERCEPTION_RADIUS {
                    assert_eq!(outcome, Err("target_not_perceived".into()), "{verb}");
                } else {
                    assert_eq!(outcome, Ok(1), "{verb}");
                }
            }
            for verb in [
                Action::Retreat {
                    target: "M02".into(),
                },
                Action::Surrender {
                    target: "M02".into(),
                },
            ] {
                assert_eq!(
                    simulation.validate_targeted(0, &verb),
                    Err("target_not_in_record".into()),
                    "{verb}"
                );
            }
        }

        // 3. `threaten`, `attack` and `fight` require contact. The record is populated here,
        //    so `fight` at distance two is the case that fixes the order between condition 3
        //    and condition 4: an attacker that struck and stepped away cannot be fought.
        simulation.agents[0].suffered = suffered_from("M02");
        for distance in [CONTACT_RADIUS, CONTACT_RADIUS + 1] {
            simulation.agents[1].position = Coordinate {
                x: 10 + distance,
                y: 10,
            };
            for verb in [
                Action::Attack {
                    target: "M02".into(),
                },
                Action::Threaten {
                    target: "M02".into(),
                },
                Action::Fight {
                    target: "M02".into(),
                },
            ] {
                let outcome = simulation.validate_targeted(0, &verb);
                if distance > CONTACT_RADIUS {
                    assert_eq!(outcome, Err("target_not_in_contact".into()), "{verb}");
                } else {
                    assert_eq!(outcome, Ok(1), "{verb}");
                }
            }
        }

        // 4. `fight`, `retreat` and `surrender` require the record, and the record is read by
        //    attacker: an entry naming a third Mokiterion licenses no answer to this one.
        simulation.agents[1].position = Coordinate { x: 11, y: 10 };
        for record in [Vec::new(), suffered_from("M03")] {
            simulation.agents[0].suffered = record;
            for verb in [
                Action::Fight {
                    target: "M02".into(),
                },
                Action::Retreat {
                    target: "M02".into(),
                },
                Action::Surrender {
                    target: "M02".into(),
                },
            ] {
                assert_eq!(
                    simulation.validate_targeted(0, &verb),
                    Err("target_not_in_record".into()),
                    "{verb}"
                );
            }
            // And the record is asked of nothing else: the same position and the same empty
            // record admit an attack.
            assert_eq!(
                simulation.validate_targeted(
                    0,
                    &Action::Attack {
                        target: "M02".into()
                    }
                ),
                Ok(1)
            );
        }
    }

    /// Rule 6: a rejected targeted proposal changes no state and reports no event.
    ///
    /// Every ground the engine can state is applied against one world, and the whole of
    /// authoritative state is compared after each — both Mokiterions, every resource, the
    /// resource counter and the shared stream. The stream especially: a rejection that had
    /// drawn would leave every subsequent tick of the run somewhere else.
    #[test]
    fn a_rejected_targeted_proposal_mutates_nothing_and_reports_nothing() {
        let mut simulation =
            encounter(7, Coordinate { x: 127, y: 10 }, Coordinate { x: 97, y: 10 });
        // `M03` underfoot, `M04` due west and perceived, `M05` dead.
        simulation.agents[2].position = Coordinate { x: 127, y: 10 };
        simulation.agents[3].position = Coordinate { x: 120, y: 10 };
        simulation.agents[4].alive = false;
        let before = state_snapshot(&simulation);

        for (action, ground) in [
            (
                Action::Attack {
                    target: "M99".into(),
                },
                "target_unknown",
            ),
            (
                Action::Attack {
                    target: "M01".into(),
                },
                "target_is_actor",
            ),
            (
                Action::Attack {
                    target: "M05".into(),
                },
                "target_dead",
            ),
            (
                Action::Approach {
                    target: "M02".into(),
                },
                "target_not_perceived",
            ),
            (
                Action::Avoid {
                    target: "M02".into(),
                },
                "target_not_perceived",
            ),
            (
                Action::Threaten {
                    target: "M04".into(),
                },
                "target_not_in_contact",
            ),
            (
                Action::Fight {
                    target: "M04".into(),
                },
                "target_not_in_contact",
            ),
            (
                Action::Retreat {
                    target: "M04".into(),
                },
                "target_not_in_record",
            ),
            (
                Action::Surrender {
                    target: "M04".into(),
                },
                "target_not_in_record",
            ),
            (
                Action::Approach {
                    target: "M03".into(),
                },
                "target_co_located",
            ),
            (
                Action::Avoid {
                    target: "M04".into(),
                },
                "out_of_bounds",
            ),
        ] {
            let mut output = Vec::new();
            let result = simulation.apply_action(&mut output, 0, &action).unwrap();

            assert!(!result.accepted, "{action} was accepted");
            assert_eq!(result.detail, ground, "{action}");
            assert!(output.is_empty(), "{action} reported an event");
            assert_eq!(state_snapshot(&simulation), before, "{action} moved state");
        }
    }

    /// Rule 22: the damage a strike deals is `10 + (energy + health) / 10`, read from the
    /// striker's own condition at the moment of resolution, and the cost is the flat constant.
    ///
    /// The table is a boundary table and not a sample: both ends of the `10..=30` range, the
    /// division's own boundary either side of a carry, and the same condition reached through
    /// different attributes, which is what shows the sum is the input rather than either term.
    #[test]
    fn strike_damage_is_the_strikers_own_condition_and_the_cost_is_flat() {
        // striker energy, striker health, damage
        for (energy, health, damage) in [
            (0u8, 1u8, 10u8),
            (0, 9, 10),
            (0, 10, 11),
            (5, 5, 11),
            (50, 50, 20),
            (0, 100, 20),
            (100, 1, 20),
            (99, 100, 29),
            (100, 100, 30),
        ] {
            let mut simulation =
                encounter(1, Coordinate { x: 10, y: 10 }, Coordinate { x: 11, y: 10 });
            simulation.agents[0].energy = energy;
            simulation.agents[0].health = health;
            let mut output = Vec::new();

            let result = simulation
                .apply_action(
                    &mut output,
                    0,
                    &Action::Attack {
                        target: "M02".into(),
                    },
                )
                .unwrap();

            let cost = energy.saturating_sub(STRIKE_ENERGY_COST);
            assert!(result.accepted, "{energy},{health}");
            assert_eq!(
                result.detail,
                format!("damage:{damage}"),
                "{energy},{health}"
            );
            assert_eq!(simulation.agents[1].health, ATTRIBUTE_MAX - damage);
            // The cost is the constant, saturating at zero, and it is not the damage.
            assert_eq!(simulation.agents[0].energy, cost, "{energy},{health}");
            // The striker loses nothing else, and the target nothing but health.
            assert_eq!(simulation.agents[0].health, health);
            assert_eq!(simulation.agents[0].satiety, ATTRIBUTE_MAX);
            assert_eq!(simulation.agents[0].fear, 0);
            assert_eq!(simulation.agents[1].satiety, ATTRIBUTE_MAX);
            assert_eq!(simulation.agents[1].energy, ATTRIBUTE_MAX);
            assert_eq!(simulation.agents[1].fear, 0);
            assert_eq!(simulation.agents[1].position, Coordinate { x: 11, y: 10 });
            assert!(simulation.agents[1].alive);

            // Rule 25's window opens on the target and on nobody else: a strike is not an
            // attack on the Mokiterion that made it.
            assert_eq!(
                simulation.agents[1].suffered,
                vec![SufferedAttack {
                    attacker: "M01".to_string(),
                    damage
                }]
            );
            assert!(simulation.agents[0].suffered.is_empty());

            assert_eq!(
                String::from_utf8(output).unwrap().trim_end(),
                format!(
                    "tick=1 subject=M01 event=attack_resolved result=target:M02,damage:{damage},target_health:100->{},striker_energy:{energy}->{cost},target_died:no",
                    ATTRIBUTE_MAX - damage
                )
            );
        }

        // The same striker deals the same damage to a target in any condition. That is what
        // rule 22 means by naming only the striker's attributes, and what rule 3's perception
        // record enforces by carrying none of the target's.
        let mut damages = Vec::new();
        for (target_health, target_energy, target_fear) in
            [(100u8, 100u8, 0u8), (60, 3, 90), (31, 0, 100)]
        {
            let mut simulation =
                encounter(1, Coordinate { x: 10, y: 10 }, Coordinate { x: 11, y: 10 });
            simulation.agents[0].energy = 70;
            simulation.agents[0].health = 40;
            simulation.agents[1].health = target_health;
            simulation.agents[1].energy = target_energy;
            simulation.agents[1].fear = target_fear;

            let result = simulation
                .apply_action(
                    &mut Vec::new(),
                    0,
                    &Action::Attack {
                        target: "M02".into(),
                    },
                )
                .unwrap();

            assert!(result.accepted);
            damages.push(result.detail);
        }
        assert_eq!(damages, vec!["damage:21".to_string(); 3]);
    }

    /// Rule 22's saturation and rule 13's death, reached from inside another Mokiterion's
    /// turn.
    #[test]
    fn a_strike_that_empties_health_kills_through_rule_13_and_no_other_path() {
        let mut simulation = encounter(2, Coordinate { x: 10, y: 10 }, Coordinate { x: 11, y: 11 });
        simulation.agents[1].health = 4;
        let mut output = Vec::new();

        let result = simulation
            .apply_action(
                &mut output,
                0,
                &Action::Attack {
                    target: "M02".into(),
                },
            )
            .unwrap();

        assert!(result.accepted);
        assert_eq!(result.detail, "damage:30");
        // Health saturates at zero rather than wrapping, and death is immediate.
        assert_eq!(simulation.agents[1].health, 0);
        assert!(!simulation.agents[1].alive);

        // Rule 13's own event, once, with no combat-specific death record beside it.
        assert_eq!(
            String::from_utf8(output)
                .unwrap()
                .lines()
                .collect::<Vec<&str>>(),
            vec![
                "tick=1 subject=M01 event=attack_resolved result=target:M02,damage:30,target_health:4->0,striker_energy:100->95,target_died:yes",
                "tick=1 subject=M02 event=agent_died result=health:0",
            ]
        );

        // The window the strike opened is written whether or not the target survived, and the
        // dead take no further opportunity to read it.
        assert_eq!(simulation.agents[1].suffered.len(), 1);

        // A dead Mokiterion is no longer a target of anything, the strike that killed it
        // included.
        for verb in targeted_proposals("M02") {
            assert_eq!(
                simulation.validate_targeted(0, &verb),
                Err("target_dead".into()),
                "{verb}"
            );
        }
    }

    /// Rule 23: a threat moves the target's `fear` by the constant, saturating at the
    /// attribute bound, and moves nothing else about either party.
    ///
    /// The saturating rows are the ones with a decision in them. A target already at the
    /// maximum is threatened *validly* and reports an increase of `0`: `REQ-MOK-046` requires
    /// the increase applied rather than the constant attempted, so a rejection there would be
    /// wrong and a reported `30` would be a lie about the transition beside it.
    #[test]
    fn a_threat_moves_the_targets_fear_and_nothing_else() {
        for (before, after) in [
            (0u8, THREAT_FEAR_INCREASE),
            (69, 99),
            (70, ATTRIBUTE_MAX),
            (85, ATTRIBUTE_MAX),
            (ATTRIBUTE_MAX, ATTRIBUTE_MAX),
        ] {
            let increase = after - before;
            let mut simulation =
                encounter(3, Coordinate { x: 10, y: 10 }, Coordinate { x: 10, y: 11 });
            simulation.agents[1].fear = before;
            let threatener = simulation.agents[0].clone();
            let target = simulation.agents[1].clone();
            let mut output = Vec::new();

            let result = simulation
                .apply_action(
                    &mut output,
                    0,
                    &Action::Threaten {
                        target: "M02".into(),
                    },
                )
                .unwrap();

            assert!(result.accepted, "at fear {before}");
            assert_eq!(result.detail, format!("increase:{increase}"));
            assert_eq!(
                simulation.agents[1],
                Mokiterion {
                    fear: after,
                    ..target
                },
                "at fear {before}"
            );
            // The only thing a threat costs its maker is the opportunity it spent.
            assert_eq!(simulation.agents[0], threatener, "at fear {before}");

            // Below the bound the effective increase is the constant, so the unsaturated rows
            // also pin the constant itself at `30` rather than at rule 12's `10`.
            if before <= ATTRIBUTE_MAX - THREAT_FEAR_INCREASE {
                assert_eq!(increase, THREAT_FEAR_INCREASE, "at fear {before}");
            }

            assert_eq!(
                String::from_utf8(output).unwrap().trim_end(),
                format!(
                    "tick=1 subject=M01 event=threat_resolved result=target:M02,increase:{increase},target_fear:{before}->{after}"
                )
            );

            // A threatened Mokiterion has not been attacked, so rule 6's record condition
            // still refuses it an answer.
            assert_eq!(
                simulation.validate_targeted(
                    1,
                    &Action::Fight {
                        target: "M01".into()
                    }
                ),
                Err("target_not_in_record".into())
            );
        }
    }

    /// Rule 24: a surrender forfeits half of the surrendering Mokiterion's own `satiety`, the
    /// recipient takes what fits, and the remainder is destroyed rather than banked.
    #[test]
    fn a_surrender_forfeits_half_its_own_satiety_and_discards_what_does_not_fit() {
        // subject satiety, recipient satiety, transferred, discarded
        for (subject_before, recipient_before, transferred, discarded) in [
            (80u8, 30u8, 40u8, 0u8),
            (20, 50, 10, 0),
            // The halving truncates toward zero, so an odd satiety keeps the odd unit.
            (81, 30, 40, 0),
            (100, 50, 50, 0),
            (100, 90, 10, 40),
            (100, 100, 0, 50),
            // The cheapest answer is available to the Mokiterion least able to pay. That is
            // what a proportion decides, and it is not a rounding defect: `2` and `3` both
            // give one unit, `1` and `0` give none, and all four succeed.
            (3, 50, 1, 0),
            (2, 50, 1, 0),
            (1, 50, 0, 0),
            (0, 50, 0, 0),
        ] {
            // Out of contact, which rule 6 does not ask of a surrender: the attacker in the
            // record may have stepped away since it struck.
            let mut simulation =
                encounter(4, Coordinate { x: 20, y: 20 }, Coordinate { x: 22, y: 20 });
            simulation.agents[0].satiety = subject_before;
            simulation.agents[0].suffered = suffered_from("M02");
            simulation.agents[1].satiety = recipient_before;
            let subject = simulation.agents[0].clone();
            let recipient = simulation.agents[1].clone();
            let mut output = Vec::new();

            let result = simulation
                .apply_action(
                    &mut output,
                    0,
                    &Action::Surrender {
                        target: "M02".into(),
                    },
                )
                .unwrap();

            let forfeit = subject_before / 2;
            assert!(result.accepted, "at satiety {subject_before}");
            assert_eq!(result.detail, format!("transferred:{transferred}"));
            assert_eq!(
                forfeit,
                transferred + discarded,
                "at satiety {subject_before}"
            );

            // No damage to either party, no energy paid, no movement, no `fear` written and no
            // window opened. A surrender is not an attack.
            assert_eq!(
                simulation.agents[0],
                Mokiterion {
                    satiety: subject_before - forfeit,
                    ..subject
                },
                "at satiety {subject_before}"
            );
            assert_eq!(
                simulation.agents[1],
                Mokiterion {
                    satiety: recipient_before + transferred,
                    ..recipient
                },
                "at satiety {subject_before}"
            );
            // A surrender that empties `satiety` does not itself kill: starvation is rule
            // 12's, and it is reached through the survival path or not at all.
            assert!(simulation.agents[0].alive);

            assert_eq!(
                String::from_utf8(output).unwrap().trim_end(),
                format!(
                    "tick=1 subject=M01 event=surrender_resolved result=recipient:M02,transferred:{transferred},discarded:{discarded},subject_satiety:{subject_before}->{},recipient_satiety:{recipient_before}->{}",
                    subject_before - forfeit,
                    recipient_before + transferred
                )
            );
        }
    }

    /// Rule 21: a targeted move is one rule 8 move on rule 5 case 3's axis, with `avoid` and
    /// `retreat` taking the same axis in the opposite direction.
    #[test]
    fn a_targeted_move_is_a_rule_8_move_on_rule_5s_axis() {
        let origin = Coordinate { x: 30, y: 30 };
        // target offset, the step `approach` takes, the step `avoid` and `retreat` take
        const TABLE: [(i16, i16, Direction, Direction); 8] = [
            (10, 0, Direction::East, Direction::West),
            (-10, 0, Direction::West, Direction::East),
            (0, -10, Direction::North, Direction::South),
            (0, 10, Direction::South, Direction::North),
            // A diagonal takes the horizontal axis first, which is rule 5 case 3's rule and
            // not one of rule 21's own.
            (10, -10, Direction::East, Direction::West),
            (10, 10, Direction::East, Direction::West),
            (-10, 10, Direction::West, Direction::East),
            (-10, -10, Direction::West, Direction::East),
        ];

        for (offset_x, offset_y, toward, away) in TABLE {
            let target_position = Coordinate {
                x: (i16::from(origin.x) + offset_x) as u8,
                y: (i16::from(origin.y) + offset_y) as u8,
            };
            for (action, step) in [
                (
                    Action::Approach {
                        target: "M02".into(),
                    },
                    toward,
                ),
                (
                    Action::Avoid {
                        target: "M02".into(),
                    },
                    away,
                ),
                (
                    Action::Retreat {
                        target: "M02".into(),
                    },
                    away,
                ),
            ] {
                let mut simulation = encounter(3, origin, target_position);
                simulation.agents[0].suffered = suffered_from("M02");
                let actor = simulation.agents[0].clone();
                let target = simulation.agents[1].clone();
                let destination = origin.moved(step).unwrap();
                let mut output = Vec::new();

                let result = simulation.apply_action(&mut output, 0, &action).unwrap();

                assert!(result.accepted, "{action} at {offset_x},{offset_y}");
                assert_eq!(result.detail, format!("position:{destination}"));
                // One cell, and no cost of any kind: this is rule 8's move, reached by a
                // targeted verb, and rule 8 charges nothing.
                assert_eq!(
                    simulation.agents[0],
                    Mokiterion {
                        position: destination,
                        ..actor
                    },
                    "{action} at {offset_x},{offset_y}"
                );
                // The target is untouched, an `approach` that closed on it included.
                assert_eq!(simulation.agents[1], target);
                assert!(
                    output.is_empty(),
                    "a move within a territory reported an event"
                );
            }
        }

        // The alternate axis, where the preferred one leaves the world. Only a fleeing
        // Mokiterion reaches it: an `approach` whose horizontal step is out of bounds would
        // need its target beyond the same edge, so for `approach` rule 6's fifth condition is
        // unmet by co-location alone.
        let mut simulation =
            encounter(3, Coordinate { x: 127, y: 10 }, Coordinate { x: 115, y: 5 });
        let result = simulation
            .apply_action(
                &mut Vec::new(),
                0,
                &Action::Avoid {
                    target: "M02".into(),
                },
            )
            .unwrap();
        assert!(result.accepted);
        assert_eq!(simulation.agents[0].position, Coordinate { x: 127, y: 11 });

        // Both axes out of the world: rule 6's fifth condition, stated as rule 8's own reason.
        let mut simulation = encounter(
            3,
            Coordinate { x: 127, y: 10 },
            Coordinate { x: 115, y: 10 },
        );
        let before = state_snapshot(&simulation);
        let result = simulation
            .apply_action(
                &mut Vec::new(),
                0,
                &Action::Avoid {
                    target: "M02".into(),
                },
            )
            .unwrap();
        assert!(!result.accepted);
        assert_eq!(result.detail, "out_of_bounds");
        assert_eq!(state_snapshot(&simulation), before);

        // Co-location: an `approach` has nowhere to go, while an `avoid` takes rule 5 case 4's
        // cardinal order so that co-location stays escapable.
        let mut simulation = encounter(3, origin, origin);
        let before = state_snapshot(&simulation);
        let result = simulation
            .apply_action(
                &mut Vec::new(),
                0,
                &Action::Approach {
                    target: "M02".into(),
                },
            )
            .unwrap();
        assert!(!result.accepted);
        assert_eq!(result.detail, "target_co_located");
        assert_eq!(state_snapshot(&simulation), before);

        let result = simulation
            .apply_action(
                &mut Vec::new(),
                0,
                &Action::Avoid {
                    target: "M02".into(),
                },
            )
            .unwrap();
        assert!(result.accepted);
        assert_eq!(simulation.agents[0].position, Coordinate { x: 30, y: 29 });

        // North first, then east where north leaves the world. It is the first valid direction
        // and not a selection among them, so it draws nothing.
        let mut simulation = encounter(3, Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 });
        simulation.agents[0].suffered = suffered_from("M02");
        let stream = simulation.entropy;
        let result = simulation
            .apply_action(
                &mut Vec::new(),
                0,
                &Action::Retreat {
                    target: "M02".into(),
                },
            )
            .unwrap();
        assert!(result.accepted);
        assert_eq!(simulation.agents[0].position, Coordinate { x: 1, y: 0 });
        assert_eq!(simulation.entropy, stream);

        // And a targeted move crosses `y=63/64` exactly as any move does, because both routes
        // reach the one implementation.
        let mut simulation = encounter(3, Coordinate { x: 10, y: 63 }, Coordinate { x: 10, y: 55 });
        let mut output = Vec::new();
        let result = simulation
            .apply_action(
                &mut output,
                0,
                &Action::Avoid {
                    target: "M02".into(),
                },
            )
            .unwrap();
        assert!(result.accepted);
        assert_eq!(simulation.agents[0].position, Coordinate { x: 10, y: 64 });
        assert_eq!(
            String::from_utf8(output).unwrap().trim_end(),
            "tick=1 subject=M01 event=territory_crossed result=from:A,to:B"
        );
    }

    /// Rule 25's window: opened by a strike in resolution order, read at the sufferer's own
    /// next opportunity, and closed there whether that opportunity answered or not.
    ///
    /// The asymmetry is the second half's subject. `M01` acts before `M02` in a tick, so the
    /// strike `M02` lands in reply reaches `M01`'s record after `M01`'s opportunity has passed
    /// and waits a whole tick, while the strike `M01` landed is read and cleared inside the
    /// same tick. The technical owner accepted that latency on 2026-08-20.
    #[test]
    fn the_suffered_window_opens_in_resolution_order_and_closes_at_the_next_opportunity() {
        let mut simulation = encounter(8, Coordinate { x: 20, y: 20 }, Coordinate { x: 21, y: 20 });
        simulation.agents[2].position = Coordinate { x: 22, y: 20 };

        for striker in [0, 2] {
            simulation
                .apply_action(
                    &mut Vec::new(),
                    striker,
                    &Action::Attack {
                        target: "M02".into(),
                    },
                )
                .unwrap();
        }

        // One entry per strike, in the order they resolved, carrying who struck and for how
        // much and nothing about the striker's condition.
        assert_eq!(
            simulation.agents[1].suffered,
            vec![
                SufferedAttack {
                    attacker: "M01".to_string(),
                    damage: 30
                },
                SufferedAttack {
                    attacker: "M03".to_string(),
                    damage: 30
                },
            ]
        );
        // Rule 3 carries the whole record to the source, which answers the first of them.
        assert_eq!(
            simulation.observation(1).suffered,
            simulation.agents[1].suffered
        );
        assert_eq!(
            decide_social_once(&simulation, 1).0,
            Action::Fight {
                target: "M01".to_string()
            }
        );

        // The closing, observed across a whole tick from a clean world.
        let mut simulation = encounter(8, Coordinate { x: 20, y: 20 }, Coordinate { x: 21, y: 20 });
        simulation.config.trace_actions = true;
        let mut output = Vec::new();
        simulation
            .run_tick(&mut output, &mut SocialDecisionSource)
            .unwrap();

        assert!(
            simulation.agents[1].suffered.is_empty(),
            "M02's record outlived the opportunity that read it"
        );
        assert_eq!(
            simulation.agents[0].suffered,
            vec![SufferedAttack {
                attacker: "M02".to_string(),
                damage: 27
            }]
        );

        let output = String::from_utf8(output).unwrap();
        let line = |subject: &str| {
            output
                .lines()
                .find(|line| line.contains(&format!("subject={subject} event=action_trace")))
                .unwrap_or_else(|| panic!("{subject} was not traced"))
                .to_string()
        };
        // The trace reports the record the source read, before rule 25 clears it, with
        // `target` after `proposal` and `suffered` last.
        assert_eq!(
            line("M02"),
            "tick=1 subject=M02 event=action_trace result=proposal:fight,target:M01,status:accepted,detail:damage:27,position:21:20,territory:A,health:70,satiety:100,energy:95,fear:0,suffered:M01:30"
        );
        // `M01`'s own line, taken before the reply landed, carries no record at all: the field
        // is rendered only when it is non-empty, which is what leaves every line of every run
        // under the other three sources byte-identical.
        let first = line("M01");
        assert!(
            first.contains("proposal:attack,target:M02,status:accepted,detail:damage:30"),
            "{first}"
        );
        assert!(!first.contains("suffered:"), "{first}");
    }

    /// Rule 26 branch 1 and its two thresholds: an unanswered attack is answered, and the
    /// answer is fixed by the answerer's own `fear`.
    ///
    /// Each threshold is asserted at its own value and one below it, because the specification
    /// states them as `>=` and an off-by-one there is a different rule.
    #[test]
    fn branch_one_answers_the_first_attack_at_the_specified_fear_thresholds() {
        for (fear, expected) in [
            (0u8, "fight"),
            (RETREAT_FEAR_THRESHOLD - 1, "fight"),
            (RETREAT_FEAR_THRESHOLD, "retreat"),
            (SURRENDER_FEAR_THRESHOLD - 1, "retreat"),
            (SURRENDER_FEAR_THRESHOLD, "surrender"),
            (ATTRIBUTE_MAX, "surrender"),
        ] {
            let mut simulation =
                encounter(5, Coordinate { x: 20, y: 20 }, Coordinate { x: 21, y: 20 });
            simulation.agents[0].fear = fear;
            simulation.agents[0].suffered = suffered_from("M02");

            let (action, draws) = decide_social_once(&simulation, 0);

            assert_eq!(action.to_string(), expected, "at fear {fear}");
            assert_eq!(action.target(), Some("M02"), "at fear {fear}");
            // An answer is derived and never selected, so branch 1 draws nothing.
            assert_eq!(draws, 0, "at fear {fear}");
        }
    }

    /// Rule 26 branch 1's target and its precedence: the first attack in the record, in
    /// resolution order, ahead of every other branch however good their alternatives are.
    #[test]
    fn branch_one_answers_the_earliest_attacker_and_precedes_every_other_branch() {
        let mut simulation = encounter(5, Coordinate { x: 20, y: 20 }, Coordinate { x: 21, y: 20 });
        // Two attackers, in the order rule 22 wrote them, the second dealing more damage from
        // closer in: neither is a tie-break rule 26 knows about.
        simulation.agents[2].position = Coordinate { x: 20, y: 20 };
        simulation.agents[0].suffered = vec![
            SufferedAttack {
                attacker: "M02".to_string(),
                damage: 11,
            },
            SufferedAttack {
                attacker: "M03".to_string(),
                damage: 30,
            },
        ];
        // And every later branch has something to say: a rich resource underfoot, no energy,
        // a Mokiterion in contact and another perceived beyond it.
        simulation.agents[0].satiety = 0;
        simulation.agents[0].energy = 0;
        simulation.agents[3].position = Coordinate { x: 28, y: 20 };
        simulation.foods.push(Food {
            id: "F900".to_string(),
            position: simulation.agents[0].position,
            class: FoodClass::High,
        });

        let (action, draws) = decide_social_once(&simulation, 0);

        assert_eq!(
            action,
            Action::Fight {
                target: "M02".to_string()
            }
        );
        assert_eq!(draws, 0);
    }

    /// Rule 26 branch 2: survival comes before society, and it is rule 19's own first two
    /// cases rather than a survival rule of this source's.
    #[test]
    fn branch_two_puts_survival_before_society_and_draws_nothing() {
        // Case 1, with a Mokiterion in contact and another perceived beyond it. Satiety is
        // emptied so that the resource is tolerated at every seed's trait.
        let mut simulation = encounter(6, Coordinate { x: 20, y: 20 }, Coordinate { x: 21, y: 20 });
        simulation.agents[2].position = Coordinate { x: 30, y: 20 };
        simulation.agents[0].satiety = 0;
        simulation.foods.push(Food {
            id: "F900".to_string(),
            position: simulation.agents[0].position,
            class: FoodClass::High,
        });

        let (action, draws) = decide_social_once(&simulation, 0);
        assert_eq!(
            action,
            Action::Eat {
                food_id: "F900".to_string()
            }
        );
        assert_eq!(draws, 0);

        // Case 2, with the same company. The threshold is rule 5 case 2's own constant.
        let mut simulation = encounter(6, Coordinate { x: 20, y: 20 }, Coordinate { x: 21, y: 20 });
        simulation.agents[2].position = Coordinate { x: 30, y: 20 };
        simulation.agents[0].energy = REFERENCE_SLEEP_THRESHOLD - 1;

        let (action, draws) = decide_social_once(&simulation, 0);
        assert_eq!(action, Action::Sleep);
        assert_eq!(draws, 0);

        // At the threshold itself survival has nothing to say and society answers.
        simulation.agents[0].energy = REFERENCE_SLEEP_THRESHOLD;
        let (action, draws) = decide_social_once(&simulation, 0);
        assert_eq!(
            action,
            Action::Attack {
                target: "M02".to_string()
            }
        );
        assert_eq!(draws, 0);
    }

    /// Rule 26 branches 4 and 5: contact is engaged and distance is closed on or fled, both
    /// sides of the choice turning on the one engagement threshold.
    ///
    /// The two branches were numbered 3 and 4 when this test was written and are 4 and 5 after
    /// the 2026-08-20 amendment hoisted rule 19's case 3 above them. The name follows the rule
    /// rather than the history: nothing about what is asserted has changed, and a name that
    /// disagreed with the specification's numbering would cost every later reader the mapping.
    #[test]
    fn branches_four_and_five_choose_by_distance_then_by_the_engagement_threshold() {
        for distance in [0, CONTACT_RADIUS, CONTACT_RADIUS + 1, 8, PERCEPTION_RADIUS] {
            for (fear, engaged, afraid) in [
                (0u8, "attack", "approach"),
                (ENGAGEMENT_FEAR_THRESHOLD - 1, "attack", "approach"),
                (ENGAGEMENT_FEAR_THRESHOLD, "threaten", "avoid"),
                (ATTRIBUTE_MAX, "threaten", "avoid"),
            ] {
                let mut simulation = encounter(
                    7,
                    Coordinate { x: 20, y: 20 },
                    Coordinate {
                        x: 20 + distance,
                        y: 20,
                    },
                );
                simulation.agents[0].fear = fear;

                let (action, draws) = decide_social_once(&simulation, 0);

                let expected = if distance <= CONTACT_RADIUS {
                    engaged
                } else {
                    afraid
                };
                assert_eq!(
                    action.to_string(),
                    expected,
                    "distance {distance}, fear {fear}"
                );
                assert_eq!(
                    action.target(),
                    Some("M02"),
                    "distance {distance}, fear {fear}"
                );
                // Naming a Mokiterion is reading a sorted list, not a selection.
                assert_eq!(draws, 0, "distance {distance}, fear {fear}");
            }
        }

        // Contact is engaged ahead of distance, and among equals the lowest identifier is
        // named: rule 3 sorted the list by distance and then identifier, so branch 4's
        // tie-break is the first entry rather than a search of its own.
        let mut simulation = encounter(7, Coordinate { x: 20, y: 20 }, Coordinate { x: 25, y: 20 });
        simulation.agents[2].position = Coordinate { x: 21, y: 20 };
        simulation.agents[3].position = Coordinate { x: 19, y: 20 };
        assert_eq!(
            decide_social_once(&simulation, 0).0,
            Action::Attack {
                target: "M03".to_string()
            }
        );

        // One cell beyond perception there is no company at all, and branch 6 hands the
        // opportunity to rule 19, which searches and draws its one selection.
        let simulation = encounter(
            7,
            Coordinate { x: 20, y: 20 },
            Coordinate {
                x: 20 + PERCEPTION_RADIUS + 1,
                y: 20,
            },
        );
        let (action, draws) = decide_social_once(&simulation, 0);
        assert!(matches!(action, Action::Move { .. }), "{action}");
        assert_eq!(draws, 1);
    }

    /// Rule 26's entropy discipline: at most one draw per opportunity, and never for a
    /// targeted proposal.
    ///
    /// The property is structural — branches 1, 4 and 5 derive their answer, branches 2 and 3
    /// delegate to the two halves of rule 19 that cannot draw, and branch 6 to the half that
    /// draws exactly once — and this walks real runs at every declared seed to check that the
    /// structure holds at every opportunity a run actually presents.
    #[test]
    fn the_social_source_draws_at_most_once_and_never_for_a_targeted_proposal() {
        for seed in DECLARED_SEEDS {
            let mut simulation = Simulation::new(social_config(seed, 60, false)).unwrap();
            // Rule 3's consistency test requires a started run, so the first tick is taken
            // before the first opportunity is inspected.
            simulation.advance_tick().unwrap();
            while !simulation.is_finished() {
                for index in 0..simulation.agents.len() {
                    if !simulation.agents[index].alive {
                        continue;
                    }
                    let (action, draws) = decide_social_once(&simulation, index);
                    assert!(draws <= 1, "{draws} draws at seed {seed}");
                    if action.target().is_some() {
                        assert_eq!(draws, 0, "{action} drew at seed {seed}");
                    }
                }
                simulation.advance_tick().unwrap();
            }
        }
    }

    /// Rule 26's fallback is rule 19 and not a variation on it: at an opportunity with no
    /// company and an empty record, the social source proposes what the trait-aware source
    /// proposes, and takes the same number of draws doing it.
    ///
    /// This is the per-observation form of the claim. The run-level form — that the two
    /// sources agree byte for byte on a world where nobody ever meets anybody — is not
    /// available, because a run dense enough to reach termination is a run where they meet.
    #[test]
    fn with_no_company_and_an_empty_record_the_social_source_is_the_individual_source() {
        let mut compared = 0usize;
        for seed in DECLARED_SEEDS {
            let mut simulation = Simulation::new(social_config(seed, 60, false)).unwrap();
            simulation.advance_tick().unwrap();
            while !simulation.is_finished() {
                for index in 0..simulation.agents.len() {
                    if !simulation.agents[index].alive {
                        continue;
                    }
                    let observation = simulation.observation(index);
                    // Rule 3's list is the four core kinds and nothing else, whatever the
                    // source: rule 4's baseline takes one selection over its length, so a
                    // targeted entry here would move every `baseline` run ever recorded.
                    assert!(
                        observation
                            .valid_actions
                            .iter()
                            .all(|action| action.target().is_none()),
                        "a targeted proposal reached the valid-action list"
                    );
                    if !observation.perceived_mokiterions.is_empty()
                        || !observation.suffered.is_empty()
                    {
                        continue;
                    }
                    assert_eq!(
                        decide_social_once(&simulation, index),
                        decide_individual_once(&simulation, index),
                        "the two sources parted at seed {seed}, tick {}",
                        simulation.tick
                    );
                    compared += 1;
                }
                simulation.advance_tick().unwrap();
            }
        }
        // The comparison is worth nothing if its condition never held.
        assert!(compared > 0, "no opportunity without company was compared");
    }

    /// Rule 25 under the three sources that predate it: the window never opens, so
    /// `CAP-MOK-009`'s byte-identity has nothing here to preserve it against.
    #[test]
    fn no_source_but_rule_26s_opens_the_suffered_window() {
        for policy in [Policy::Baseline, Policy::Reference, Policy::Individual] {
            for seed in DECLARED_SEEDS {
                let mut simulation = Simulation::new(Config {
                    policy,
                    ..individual_config(seed, 200, true)
                })
                .unwrap();
                let mut output = Vec::new();
                simulation.run(&mut output).expect("the run completes");

                let output = String::from_utf8(output).unwrap();
                for event in ["attack_resolved", "threat_resolved", "surrender_resolved"] {
                    assert!(
                        !output.contains(event),
                        "{policy} at seed {seed} emitted {event}"
                    );
                }
                // The trace renders the record only when it is non-empty, so its absence from
                // every line is the record's absence from every opportunity.
                assert!(
                    !output.contains(",suffered:"),
                    "{policy} at seed {seed} traced a record"
                );
                assert!(
                    simulation
                        .agents
                        .iter()
                        .all(|agent| agent.suffered.is_empty()),
                    "{policy} at seed {seed} left a record standing"
                );
            }
        }
    }

    /// Rule 21's closed vocabulary: eleven kinds, seven of which name a target and four of
    /// which cannot.
    #[test]
    fn the_action_vocabulary_is_eleven_kinds_and_seven_name_a_target() {
        let core = [
            Action::Wait,
            Action::Sleep,
            Action::Eat {
                food_id: "F001".to_string(),
            },
            Action::Move {
                direction: Direction::North,
            },
        ];
        for action in &core {
            assert_eq!(action.target(), None, "{action}");
        }
        // The four render exactly as they did before rule 21, `move`'s direction included.
        assert_eq!(
            core.iter().map(Action::to_string).collect::<Vec<String>>(),
            ["wait", "sleep", "eat:F001", "move:north"]
        );

        let targeted = targeted_proposals("M07");
        for action in &targeted {
            assert_eq!(action.target(), Some("M07"), "{action}");
        }
        // A targeted verb renders as the bare verb, because rule 7 puts the target in a field
        // of its own and reads it from `target` above, so the two cannot disagree.
        assert_eq!(
            targeted
                .iter()
                .map(Action::to_string)
                .collect::<Vec<String>>(),
            [
                "attack",
                "threaten",
                "fight",
                "retreat",
                "surrender",
                "approach",
                "avoid"
            ]
        );

        assert_eq!(core.len() + targeted.len(), 11);
    }

    /// `REQ-MOK-044`'s no-entropy constraint, directly: the shared stream stands exactly where
    /// it stood, either side of every resolution and every targeted move.
    ///
    /// This is the form of the claim that a byte-identical capture cannot make. A draw taken
    /// and afterwards restored leaves every recorded stream identical and is still wrong,
    /// because the next change to the code would expose it; and the accepted and the rejected
    /// path are both checked, because a rejection that drew would diverge a run just as far.
    #[test]
    fn no_resolution_and_no_targeted_move_touches_the_shared_stream() {
        for (first, second, record) in [
            // In contact, so all seven are reachable; the record admits the three answers.
            (
                Coordinate { x: 40, y: 40 },
                Coordinate { x: 41, y: 40 },
                true,
            ),
            // Out of contact but perceived: the moves apply and the three contact verbs are
            // rejected.
            (
                Coordinate { x: 40, y: 40 },
                Coordinate { x: 48, y: 40 },
                true,
            ),
            // Co-located, which is where the move fallbacks and `target_co_located` live.
            (
                Coordinate { x: 40, y: 40 },
                Coordinate { x: 40, y: 40 },
                true,
            ),
            // No record: `fight`, `retreat` and `surrender` are rejected instead.
            (
                Coordinate { x: 40, y: 40 },
                Coordinate { x: 41, y: 40 },
                false,
            ),
        ] {
            for action in targeted_proposals("M02") {
                let mut simulation = encounter(9, first, second);
                if record {
                    simulation.agents[0].suffered = suffered_from("M02");
                }
                let before = simulation.entropy;

                simulation
                    .apply_action(&mut Vec::new(), 0, &action)
                    .unwrap();

                assert_eq!(simulation.entropy, before, "{action} moved the stream");
            }
        }
    }

    /// `INT-MOK-009`'s recorded risk, at the mechanism: exchanging the two identifiers changes
    /// nothing about the outcome.
    ///
    /// Deterministic resolution in ascending identifier order is what makes this worth
    /// asserting. Damage is a function of the striker's `energy` and `health` and of nothing
    /// else, so the same encounter struck the other way round must produce the same numbers —
    /// any difference would mean an identifier had reached the arithmetic.
    #[test]
    fn exchanging_the_two_identifiers_changes_no_outcome() {
        // striker, target: the roles, not the identifiers.
        let outcome = |striker: usize, target: usize| {
            let mut simulation =
                encounter(9, Coordinate { x: 50, y: 50 }, Coordinate { x: 51, y: 50 });
            simulation.agents[striker].energy = 70;
            simulation.agents[striker].health = 40;
            simulation.agents[target].health = ATTRIBUTE_MAX;
            simulation.agents[target].energy = 35;
            let target_id = simulation.agents[target].id.clone();

            let result = simulation
                .apply_action(
                    &mut Vec::new(),
                    striker,
                    &Action::Attack { target: target_id },
                )
                .unwrap();

            (
                result.detail,
                simulation.agents[striker].energy,
                simulation.agents[striker].health,
                simulation.agents[target].health,
                simulation.agents[target].suffered[0].damage,
            )
        };

        // `M01` striking `M02`, and `M02` striking `M01`: the striker is the lower identifier
        // in one and the higher in the other.
        assert_eq!(outcome(0, 1), outcome(1, 0));
        assert_eq!(outcome(0, 1).0, "damage:21");
    }

    /// `REQ-MOK-044`'s lethality, counted: four strikes at the maximum and ten at the minimum
    /// empty a full-health Mokiterion.
    ///
    /// This is the arithmetic `REQ-MOK-049`'s floor was lowered against, so it is asserted
    /// rather than left implied by the damage range.
    #[test]
    fn a_full_health_mokiterion_falls_in_four_strikes_at_most_and_ten_at_least() {
        // striker energy, striker health, damage, strikes to empty a full-health target
        for (energy, health, damage, strikes) in [(100u8, 100u8, 30u8, 4u32), (0, 1, 10, 10)] {
            let mut simulation =
                encounter(9, Coordinate { x: 50, y: 50 }, Coordinate { x: 51, y: 50 });
            let mut counted = 0;

            while simulation.agents[1].alive {
                // The striker's condition is held, because a striker whose own `energy` fell
                // would deal less each time and the count would measure two rules at once.
                simulation.agents[0].energy = energy;
                simulation.agents[0].health = health;
                let result = simulation
                    .apply_action(
                        &mut Vec::new(),
                        0,
                        &Action::Attack {
                            target: "M02".into(),
                        },
                    )
                    .unwrap();
                assert_eq!(result.detail, format!("damage:{damage}"));
                counted += 1;
                assert!(counted <= 10, "a full-health target survived ten strikes");
            }

            assert_eq!(counted, strikes, "at {energy},{health}");
            assert_eq!(simulation.agents[1].health, 0);
            // Every strike is recorded, the last one included, and the record outlives its
            // holder without being read.
            assert_eq!(simulation.agents[1].suffered.len(), strikes as usize);
        }
    }

    /// Rule 22 and rule 13 together, across a whole tick: a Mokiterion killed at another's
    /// opportunity takes no opportunity of its own in that tick.
    #[test]
    fn a_mokiterion_killed_mid_tick_takes_no_further_opportunity() {
        let mut simulation =
            encounter(10, Coordinate { x: 60, y: 60 }, Coordinate { x: 61, y: 60 });
        simulation.agents[1].health = 4;
        simulation.config.trace_actions = true;
        let mut output = Vec::new();

        simulation
            .run_tick(&mut output, &mut SocialDecisionSource)
            .unwrap();

        assert!(!simulation.agents[1].alive);
        let output = String::from_utf8(output).unwrap();
        // No opportunity, so no trace line and no rule 12 write: the dead neither act nor
        // decay, and the strike that killed it is the last record naming it as a subject.
        assert!(
            !output.contains("subject=M02 event=action_trace"),
            "the dead took an opportunity"
        );
        assert!(
            !output.contains("subject=M02 event=survival_changed"),
            "the dead decayed"
        );
        assert_eq!(output.matches("subject=M02 event=agent_died").count(), 1);
    }

    /// Rule 22 and rule 13: both Mokiterions of an encounter may die within one tick, and a
    /// strike is attributed to the Mokiterion that made it and never to one already dead.
    #[test]
    fn both_parties_may_die_within_one_tick() {
        let mut simulation =
            encounter(10, Coordinate { x: 60, y: 60 }, Coordinate { x: 61, y: 60 });
        simulation.agents[2].position = Coordinate { x: 59, y: 60 };
        // `M01`'s own damage falls with its own `health`, so the target it can kill is set from
        // what a striker in that condition actually deals: `10 + (100 + 30) / 10`.
        simulation.agents[0].health = 30;
        simulation.agents[1].health = 23;
        let mut output = Vec::new();

        // `M01` kills `M02` at its own opportunity, and `M03` kills `M01` at its own.
        for (striker, target) in [(0, "M02"), (2, "M01")] {
            simulation
                .apply_action(
                    &mut output,
                    striker,
                    &Action::Attack {
                        target: target.to_string(),
                    },
                )
                .unwrap();
        }

        assert!(!simulation.agents[0].alive);
        assert!(!simulation.agents[1].alive);
        assert!(simulation.agents[2].alive);

        let output = String::from_utf8(output).unwrap();
        assert_eq!(output.matches("event=agent_died").count(), 2);
        // Each resolution names its own striker as subject. `M01` is the subject of one and
        // the target of the other, and it is dead only for the second.
        assert_eq!(
            output.matches("subject=M01 event=attack_resolved").count(),
            1
        );
        assert_eq!(
            output.matches("subject=M03 event=attack_resolved").count(),
            1
        );

        // And once dead, `M01` is no longer a striker: rule 26 gives it no opportunity, and
        // rule 6 refuses every proposal naming it.
        for verb in targeted_proposals("M01") {
            assert_eq!(
                simulation.validate_targeted(2, &verb),
                Err("target_dead".into()),
                "{verb}"
            );
        }
    }

    /// Rule 23 and rule 12 compose in turn order within a tick, and a threat outlasts the tick
    /// that made it.
    ///
    /// The `-5` composition needs a construction that looks contrived and is not. A threatener
    /// is in contact with its target, and contact implies perception, so the target's own rule
    /// 12 write in that same tick is always the `+10`. The `-5` composes in the same tick only
    /// where the threatener stops being perceived before the target's opportunity arrives —
    /// which is reachable exactly one way: the threatener dies of its own survival step in
    /// between. That is the second half below, and it is the only same-tick `-5` the rules
    /// admit.
    #[test]
    fn a_threat_composes_with_rule_12_in_turn_order_and_outlasts_its_tick() {
        // The `+10` case. `M01` is given rule 26's threatening `fear` so that the tick's own
        // decision produces the threat rather than a test bypassing the source.
        let mut simulation =
            encounter(11, Coordinate { x: 70, y: 70 }, Coordinate { x: 70, y: 71 });
        simulation.agents[0].fear = ENGAGEMENT_FEAR_THRESHOLD;
        let mut output = Vec::new();

        simulation
            .run_tick(&mut output, &mut SocialDecisionSource)
            .unwrap();

        // `M02` carries the composition, and it is the half that proves the ordering: it was
        // threatened inside `M01`'s turn for `THREAT_FEAR_INCREASE`, and rule 12 then added `10`
        // for the company it kept at its own. Unsaturated, so both writes are visible in the sum.
        assert_eq!(simulation.agents[1].fear, THREAT_FEAR_INCREASE + 10);

        // `M01` carries only saturation, and that is a consequence of `REQ-MOK-048`'s amendment
        // of 2026-08-20 rather than a weakening of this test. Rule 26 threatens at or above
        // `ENGAGEMENT_FEAR_THRESHOLD`, which that amendment moved to `95`, so **every** threatener
        // is within `FEAR_DECREASE` of [`ATTRIBUTE_MAX`] and its own rule 12 write saturates.
        // An unsaturated threatener composing with rule 12 is no longer a reachable construction
        // in either direction, so it is asserted as the bound it now is.
        assert_eq!(simulation.agents[0].fear, ATTRIBUTE_MAX);
        assert!(
            u16::from(ENGAGEMENT_FEAR_THRESHOLD) + u16::from(FEAR_INCREASE)
                > u16::from(ATTRIBUTE_MAX),
            "the line above would be an arithmetic coincidence if a threatener could stay below \
             the ceiling"
        );

        // And `M02` answered the threat with `attack` rather than with a threat of its own, which
        // is the same amendment read from the other side: at a gate of `95` a Mokiterion at
        // `THREAT_FEAR_INCREASE` is calm, so one threat no longer makes its target threaten back.
        // `SPEC-MOK-001` rule 26 records that as the cost of the value.
        let output = String::from_utf8(output).unwrap();
        assert_eq!(
            output.matches("subject=M02 event=attack_resolved").count(),
            1,
            "the target of the threat was to answer with a strike at this gate"
        );

        // The `-5` case, at the bound where the threat itself applies nothing.
        let mut simulation =
            encounter(11, Coordinate { x: 70, y: 70 }, Coordinate { x: 70, y: 71 });
        simulation.agents[0].fear = ENGAGEMENT_FEAR_THRESHOLD;
        simulation.agents[0].health = 5;
        simulation.agents[0].satiety = 1;
        simulation.agents[1].fear = ATTRIBUTE_MAX;

        simulation
            .run_tick(&mut Vec::new(), &mut SocialDecisionSource)
            .unwrap();

        assert!(
            !simulation.agents[0].alive,
            "the threatener was to die of its own decay"
        );
        assert_eq!(simulation.agents[1].fear, ATTRIBUTE_MAX - FEAR_DECREASE);

        // And the threat outlasts the tick: `30` takes six quiet ticks to shed at rule 12's
        // `5`, which is why rule 23's constant is three times rule 12's rather than equal to
        // it.
        let mut simulation =
            encounter(11, Coordinate { x: 70, y: 70 }, Coordinate { x: 70, y: 71 });
        simulation
            .apply_action(
                &mut Vec::new(),
                0,
                &Action::Threaten {
                    target: "M02".into(),
                },
            )
            .unwrap();
        assert_eq!(simulation.agents[1].fear, THREAT_FEAR_INCREASE);
        // The threatener leaves, so that what is measured afterwards is rule 12's decrease and
        // not a second encounter.
        simulation.agents[0].position = Coordinate { x: 100, y: 100 };

        for expected in [25, 20, 15, 10, 5, 0] {
            simulation
                .run_tick(&mut Vec::new(), &mut SocialDecisionSource)
                .unwrap();
            assert_eq!(
                simulation.agents[1].fear, expected,
                "at tick {}",
                simulation.tick
            );
        }
    }

    /// Rule 24 grants no immunity: a Mokiterion that surrendered may be struck afterwards and
    /// may surrender again on the next tick, paying again.
    #[test]
    fn a_surrender_buys_no_immunity_and_is_paid_again() {
        let mut simulation =
            encounter(12, Coordinate { x: 80, y: 80 }, Coordinate { x: 81, y: 80 });
        simulation.agents[0].satiety = 100;
        simulation.agents[1].satiety = 0;
        simulation.agents[0].suffered = suffered_from("M02");

        let first = simulation
            .apply_action(
                &mut Vec::new(),
                0,
                &Action::Surrender {
                    target: "M02".into(),
                },
            )
            .unwrap();
        assert_eq!(first.detail, "transferred:50");
        assert_eq!(simulation.agents[0].satiety, 50);

        // The record is not consumed by the answer — rule 25's window closes at the next
        // opportunity, not at the action — and nothing about having surrendered protects the
        // surrendering Mokiterion from being struck.
        let struck = simulation
            .apply_action(
                &mut Vec::new(),
                1,
                &Action::Attack {
                    target: "M01".into(),
                },
            )
            .unwrap();
        assert!(struck.accepted);
        assert_eq!(simulation.agents[0].health, ATTRIBUTE_MAX - 30);

        // And the next tick's answer is paid at the new satiety, from the record the strike
        // above wrote.
        simulation.tick += 1;
        simulation.agents[0].suffered = suffered_from("M02");
        let second = simulation
            .apply_action(
                &mut Vec::new(),
                0,
                &Action::Surrender {
                    target: "M02".into(),
                },
            )
            .unwrap();
        assert_eq!(second.detail, "transferred:25");
        assert_eq!(simulation.agents[0].satiety, 25);
    }

    /// Rule 21: a resolution touches its two parties and nobody else.
    ///
    /// Every other Mokiterion in the world is compared before and after, for all seven verbs.
    /// A resolution that reached a third party — by index arithmetic, by a stale borrow, or by
    /// applying twice — is what this excludes.
    #[test]
    fn a_resolution_touches_nobody_but_its_two_parties() {
        for action in targeted_proposals("M02") {
            let mut simulation =
                encounter(13, Coordinate { x: 90, y: 90 }, Coordinate { x: 91, y: 90 });
            simulation.agents[0].suffered = suffered_from("M02");
            let bystanders: Vec<Mokiterion> = simulation.agents[2..].to_vec();
            let foods = simulation.foods.clone();

            let result = simulation
                .apply_action(&mut Vec::new(), 0, &action)
                .unwrap();

            assert!(result.accepted, "{action}");
            assert_eq!(simulation.agents[2..], bystanders[..], "{action}");
            // And no resolution moves a resource: rule 24 transfers `satiety` between
            // Mokiterions and touches nothing in the world.
            assert_eq!(simulation.foods, foods, "{action}");
        }
    }

    /// Rule 20 across the territory boundary: contact is a distance and knows nothing about
    /// territories, and a strike across the line emits its own event and no crossing.
    #[test]
    fn contact_and_resolution_span_the_territory_boundary() {
        let mut simulation =
            encounter(13, Coordinate { x: 20, y: 63 }, Coordinate { x: 20, y: 64 });
        assert_eq!(simulation.agents[0].position.territory(), Territory::A);
        assert_eq!(simulation.agents[1].position.territory(), Territory::B);
        assert!(simulation.in_contact(0, 1));

        let mut output = Vec::new();
        simulation
            .apply_action(
                &mut output,
                0,
                &Action::Attack {
                    target: "M02".into(),
                },
            )
            .unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(output.lines().count(), 1);
        assert!(output.contains("event=attack_resolved"), "{output}");
        assert!(!output.contains("territory_crossed"), "{output}");
    }

    /// Rule 25's latency in both directions, over whole ticks.
    ///
    /// A defender whose identifier is above its attacker's answers inside the same tick; one
    /// whose identifier is below it answers on the next. Both are consequences of the acting
    /// order rather than of rule 25, and `INT-MOK-009` records the asymmetry as accepted.
    #[test]
    fn a_defender_below_its_attackers_identifier_answers_on_the_next_tick() {
        let mut simulation =
            encounter(14, Coordinate { x: 30, y: 30 }, Coordinate { x: 31, y: 30 });
        simulation.config.trace_actions = true;

        // Tick one: `M01` strikes, `M02` answers within the tick, and the reply lands in
        // `M01`'s record after `M01`'s opportunity has passed.
        simulation
            .run_tick(&mut Vec::new(), &mut SocialDecisionSource)
            .unwrap();
        assert_eq!(simulation.agents[0].suffered.len(), 1);

        // Tick two: `M01` reads that record at its own next opportunity. Its `fear` stands at
        // rule 12's one write, which is below rule 26's retreat threshold, so the answer is a
        // fight.
        let mut output = Vec::new();
        simulation
            .run_tick(&mut output, &mut SocialDecisionSource)
            .unwrap();

        let output = String::from_utf8(output).unwrap();
        let line = output
            .lines()
            .find(|line| line.contains("subject=M01 event=action_trace"))
            .expect("M01 was traced");
        assert!(line.contains("proposal:fight,target:M02"), "{line}");
        assert!(line.contains(",suffered:M02:"), "{line}");
        // Read, and closed. The record is not empty at the end of this tick, because `M02`
        // answered the fight and that reply is a new attack written after the clearing — which
        // is the same one-tick latency seen from the other side. What the clearing rules out is
        // accumulation: two ticks of strikes never stand in one window.
        assert_eq!(simulation.agents[0].suffered.len(), 1);
        assert_eq!(simulation.agents[0].suffered[0].attacker, "M02");
    }

    /// Rule 25: answering one attack closes the window on all of them.
    #[test]
    fn answering_one_attack_closes_the_window_on_every_attack_in_it() {
        let mut simulation =
            encounter(14, Coordinate { x: 40, y: 40 }, Coordinate { x: 41, y: 40 });
        // `M03` struck and stepped away, which is what makes this test able to see anything:
        // left in contact it would simply strike again at its own opportunity, and a fresh
        // entry naming it is indistinguishable from an entry that survived the window.
        simulation.agents[2].position = Coordinate { x: 36, y: 40 };
        simulation.agents[0].suffered = vec![
            SufferedAttack {
                attacker: "M02".to_string(),
                damage: 12,
            },
            SufferedAttack {
                attacker: "M03".to_string(),
                damage: 30,
            },
        ];

        let mut output = Vec::new();
        simulation
            .run_tick(&mut output, &mut SocialDecisionSource)
            .unwrap();

        // `M01` answered `M02` and the entry naming `M03` is gone with it: rule 25 closes the
        // window on the opportunity, not on the entry.
        let output = String::from_utf8(output).unwrap();
        assert!(
            output.contains("subject=M01 event=attack_resolved result=target:M02"),
            "{output}"
        );
        assert!(
            !output.contains("subject=M01 event=attack_resolved result=target:M03"),
            "{output}"
        );
        // Both entries are gone, and the one never answered is gone for good: rule 25 closes
        // the window on the opportunity rather than on the entry, so an unanswered attack is
        // forgotten rather than banked.
        assert!(
            simulation.agents[0]
                .suffered
                .iter()
                .all(|attack| attack.attacker != "M03"),
            "the unanswered entry survived the opportunity"
        );
    }
}
