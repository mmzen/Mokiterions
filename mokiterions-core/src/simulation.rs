use std::fmt;
use std::io::{self, BufRead, Write};

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
/// the amendment that has since happened — `REQ-MOK-057`'s of 2026-08-20 moved this one and
/// left that one alone. **The value is measured rather than derived**, and the specification
/// records that as a cost: at `30` no approach could ever complete, because rule 12 drives
/// `fear` from company perceived at [`PERCEPTION_RADIUS`] while engagement needs contact at
/// [`CONTACT_RADIUS`], so the gate closed on the third perceiving tick and closing sixteen
/// squares takes fifteen. `90` fails `REQ-MOK-058` on one declared seed and `100` holds with
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
    /// The spend ceiling of `SPEC-MOK-007` rule 14.6, in minor units, or `None` when none was
    /// declared. `--spend-ceiling` is the option, fixed by rule 18.4.0.
    ///
    /// **This is the one new option value the configuration retains, and the asymmetry is the
    /// specification's rather than a convenience.** Rule 18.4 has the shared parser validate a
    /// path and then discard it, so that no path reaches the simulation's rules by travelling
    /// inside the configuration; `VER-MOK-018` case `S6a` scopes that prohibition precisely — the
    /// configuration "gains no field **for either new path**". A ceiling is not a path. It is a
    /// quantity the run itself must act on: rule 14.6 stops the run *before* an exchange when the
    /// accumulated cost has reached it, and rule 15.2 reports it in the run record. Neither is
    /// something a host can do on the library's behalf without the library knowing the number.
    ///
    /// Minor units, as an integer, because rule 14.2 fixes cost as integer arithmetic in a stated
    /// minor unit and `SPEC-MOK-006` forbids a floating-point value in a stream. The parser turns
    /// the operator's decimal into this integer, on `Density`'s precedent, so no float exists at
    /// any point.
    pub spend_ceiling: Option<u64>,
    /// The unit prices of `SPEC-MOK-007` rule 14.3a, or `None` when none were declared.
    /// `--prices` is the option.
    ///
    /// The second value the configuration retains, and the reasoning above transfers without
    /// change: prices are not a path, and rule 14.2's cost "is computed from the reported counts and
    /// the unit prices declared for the run" by the run itself. Rule 14.6 stops the run *before* an
    /// exchange once the accumulated cost reaches the ceiling, and rule 15.2 reports the cost — so
    /// the library needs the ceiling and the multipliers that turn token counts into it, and a host
    /// can supply neither on its behalf.
    ///
    /// `None` on every replay and on the four deterministic sources, per rule 14.8: "a replay spends
    /// nothing, computes no ratio and has no ceiling". `Some` on every live run, because rule 14.3
    /// forbids a compiled-in default and `cli::parse` therefore refuses `--live` without it.
    pub prices: Option<UnitPrices>,
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

/// The provider's unit prices for one live run, in **US cents per million tokens**.
///
/// `SPEC-MOK-007` rule 14.3 makes these "inputs of the run, not compiled-in constants" — "the
/// provider's prices are the provider's to change" — and rule 14.3a, amended 2026-08-29 under
/// `WO-MOK-030`, fixes how they arrive: four integers, colon-separated, in the order below.
/// `--prices 125:13:1000:0` is the rule's own example.
///
/// Integers rather than a decimal, in the rule's words, "so rule 14.2's arithmetic is integer
/// arithmetic from the input onward and no rounding enters at the edge". Cents per **million**
/// tokens rather than per token because a per-token price in cents is not an integer for any
/// provider: the four figures above are $1.25, $0.13, $10.00 and $0.00 per million.
///
/// A named type rather than `[u64; 4]`, which is this work order's *Authorized decision envelope*
/// naming "the unit-price representation and the cost unit" as the implementation's to decide. The
/// four differ by nearly two orders of magnitude and three of them are plausible values for each
/// other's position, so a positional array puts a silent eighty-fold cost error one transposition
/// away — and rule 14.3a chose integers precisely to keep arithmetic error out of the edge. It costs
/// one type and one function in `SPEC-MOK-002` rule 5's census, which its 2026-08-29 row records,
/// and it is `Density`'s shape exactly: an operator-supplied quantity, parsed and validated once,
/// held in [`Config`], computed with as an integer.
///
/// No `Default`. Rule 14.3 forbids a compiled-in default, so there is no value to give one, and a
/// live run with no declared prices is refused by `cli::parse` rather than run at a guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitPrices {
    /// Uncached prompt tokens.
    pub prompt: u64,
    /// Prompt tokens the provider reported as served from its cache.
    pub cached: u64,
    /// Output tokens.
    pub output: u64,
    /// Reasoning tokens. `0` is a legal price and is the rule's own example, because
    /// `SPEC-MOK-007` fixes the reasoning level at `none` for this stage.
    pub reasoning: u64,
}

impl UnitPrices {
    /// Parses rule 14.3a's compact form: exactly four non-negative integers separated by `:`, in
    /// the order prompt, cached, output, reasoning.
    ///
    /// Every rejection names what was expected rather than what was wrong, on `Density::parse`'s
    /// precedent, because an operator who mistyped a price list needs the shape and not a diagnosis.
    /// The count is checked before the digits so that `125:13:1000` is answered with "four" rather
    /// than with a complaint about a field that is not there.
    pub fn parse(value: &str) -> Result<Self, String> {
        let fields: Vec<&str> = value.split(':').collect();
        if fields.len() != 4 {
            return Err(format!(
                "expected four prices separated by colons, in the order prompt, cached, output, reasoning, such as 125:13:1000:0; {} were given",
                fields.len()
            ));
        }
        let mut prices = [0u64; 4];
        for (field, price) in fields.iter().zip(prices.iter_mut()) {
            if field.is_empty() || !field.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(format!(
                    "expected a whole number of cents per million tokens, such as 125; found {field}"
                ));
            }
            *price = field
                .parse()
                .map_err(|_| format!("the price {field} is larger than this program can hold"))?;
        }
        Ok(Self {
            prompt: prices[0],
            cached: prices[1],
            output: prices[2],
            reasoning: prices[3],
        })
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
    /// floor `REQ-MOK-058` states for it is three below `REQ-MOK-014`'s for the default.
    Social,
    /// `SPEC-MOK-007`'s source, and the one that is unlike the other four.
    ///
    /// The four above are functions of the observation and of `REQ-MOK-009`'s entropy stream, and
    /// are deterministic at a seed. This one asks something outside the engine, through
    /// [`Proposer`], and the engine neither knows nor can know what: rule 1.1's interface
    /// names no provider, no transport, no model and no mode. It is deterministic only in
    /// replay, where a host has connected a port backed by a recorded transcript.
    ///
    /// Selecting it with no port supplied is an invalid configuration and the run refuses, per
    /// rule 20.8 and [`MISSING_DECISION_PORT`]. That refusal is the one check of rule 13 the
    /// library makes rather than a host: this variant is reachable from any caller of
    /// [`Policy::parse`], and a host that accepts `llm` on its command line and then omits the
    /// port must not quietly run something else.
    Llm,
}

impl Policy {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "baseline" => Some(Self::Baseline),
            "reference" => Some(Self::Reference),
            "individual" => Some(Self::Individual),
            "social" => Some(Self::Social),
            "llm" => Some(Self::Llm),
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
            Self::Llm => formatter.write_str("llm"),
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

    /// The slot this territory occupies in a per-territory array, matching [`Self::ALL`]'s
    /// order so that an array built by index and a record written by iteration cannot
    /// disagree about which territory a figure belongs to.
    fn index(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
        }
    }
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
    /// `SPEC-MOK-006`'s *State model*: absent until this Mokiterion dies and thereafter the
    /// tick at which it died. Written in the same statement sequence as the `agent_died`
    /// event, read only by the run record, and reported as `null` for a survivor because
    /// tick `0` is a legitimate death tick and a sentinel would collide with it.
    ///
    /// No rule reads it. It is not an attribute, is not bounded, is not decayed and does not
    /// reach the text stream, so it cannot move a decision, a draw or a byte of output.
    died_at: Option<u64>,
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
    /// `CAP-MOK-010` holds every line they appear on byte-identical. A targeted verb renders
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
    /// The acting Mokiterion's own `fear`, carried under `REQ-MOK-054`. It replaces the
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

    /// The non-waste condition of `SPEC-MOK-001` rule 5, at an arbitrary waste tolerance, and
    /// the only place its arithmetic is written.
    ///
    /// The resource fits outright, or the part the attribute maximum would clip is at most the
    /// allowance. The allowance is the larger of two terms and both truncate toward zero:
    ///
    /// * `R * R / 100`, which the resource earns for itself and which no tolerance is needed
    ///   to obtain — `2` satiety for low class, `9` for medium, `25` for high. This is the term
    ///   `REQ-MOK-060` added, and it is rule 19's own arithmetic at a tolerance equal to the
    ///   restoration itself, so the corrected condition introduces no constant the
    ///   specification did not already have.
    /// * `T * R / 100`, rule 19's per-Mokiterion tolerance, which is `0` for rule 5's callers.
    ///
    /// `u16` throughout, so no intermediate saturates and no float appears; the widest product
    /// reachable is `50 * 50`.
    ///
    /// **This function is why rule 19's `T = 0` identity with rule 5 holds structurally rather
    /// than by coincidence.** `REQ-MOK-033` obliges the trait-aware source at tolerance `0` to
    /// decide exactly as the reference source does, and `SPEC-MOK-001`'s amendment of
    /// 2026-08-21 preserves that obligation by restating rule 19's first clause as a reference
    /// to rule 5's condition rather than the literal `S + R <= 100`. At `T = 0` the second term
    /// is `0`, the first is unchanged, and the two predicates below are the same call. An
    /// implementation that duplicated the arithmetic could satisfy the obligation today and
    /// break it on the next edit to either copy.
    fn fits_within(&self, food: &PerceivedFood, tolerance: u8) -> bool {
        let restored = u16::from(food.class.restoration().0);
        let resulting = u16::from(self.satiety) + restored;
        let maximum = u16::from(ATTRIBUTE_MAX);
        if resulting <= maximum {
            return true;
        }
        let earned = restored * restored / 100;
        let tolerated = u16::from(tolerance) * restored / 100;
        resulting - maximum <= earned.max(tolerated)
    }

    /// Whether consuming this resource would waste more of its satiety restoration than the
    /// resource itself earns.
    ///
    /// `REQ-MOK-015` requires consuming when consuming is not wasteful, and `SPEC-MOK-001`
    /// rule 5 applies this one test to both eating and approaching. A fixed satiety
    /// threshold encoded neither faithfully: it made satiety 51..=100 dead buffer that could
    /// never fund travel. Applying the test to eating alone left the other half of the
    /// defect standing, because a Mokiterion that declined the resource underfoot stepped
    /// off, perceived it again as the nearest resource at a distance greater than zero, and
    /// stepped back. Screening approach targets by the same rule is what closes that cycle.
    ///
    /// The test admitted nothing above satiety `85`, `70` and `50` for low, medium and high
    /// class until `REQ-MOK-060`, and now admits up to `87`, `79` and `75`. That requirement
    /// is a ceiling on a territory's class composition, and this is the whole of the mechanism
    /// it permits: the resource nobody could eat was the resource that stayed standing, so
    /// high class accumulated to 45 of 61 by tick 1,000. Rule 4's baseline candidate list
    /// applies no waste condition at all and so is not reached from here, which is what keeps
    /// `INT-MOK-010`'s byte-identity promise for `baseline` across the correction.
    fn fits(&self, food: &PerceivedFood) -> bool {
        self.fits_within(food, 0)
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
    /// resource earns or the acting Mokiterion's own tolerance admits, whichever is larger.
    ///
    /// Rule 19's tolerant test, and [`Self::fits`] is the same test at tolerance `0`.
    ///
    /// The two predicates were deliberately separate implementations until 2026-08-21, so
    /// that the reference source's behavior did not depend on code written for the
    /// trait-aware one. `REQ-MOK-060` reverses that judgment and states the reason: it
    /// requires the corrected condition expressed once rather than duplicated across the two
    /// sources, because the correction has to reach `reference`, `individual` and `social`
    /// alike and two copies is the shape in which it reaches only some of them. The control
    /// the separation protected is now the pre-change capture, which is a stronger control
    /// than a second copy of an expression.
    ///
    /// The trait is masked wherever the resource's own `R * R / 100` already exceeds
    /// `T * R / 100`, which is `T <= 19` for low class, `T <= 33` for medium and every
    /// tolerance in `0..=40` for high. It remains live above those, and the declared seeds
    /// still derive tolerances that reach them.
    fn fits_within_tolerance(&self, food: &PerceivedFood) -> bool {
        self.fits_within(food, self.waste_tolerance)
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

    /// Announces the run to the source, once, before its first decision.
    ///
    /// The roster is every Mokiterion the run created, in ascending identifier order, as two values
    /// each — which is the whole of `SPEC-MOK-007` rule 5.2's block B and is why the parameter is
    /// [`RosterEntry`] rather than the agents themselves. `PortDecisionSource` writes the
    /// transcript's prefix head from it; the four deterministic sources take the default and do
    /// nothing, which is what keeps rule 16's non-perturbation obligation a property of the call
    /// graph.
    ///
    /// **Called by the whole-run host and not by the observer host.** `Simulation::run_with_source`
    /// runs once per instance and refuses a second call, so "once" is structural there. The
    /// observer constructs a fresh source every tick, so a call from `advance_tick_with_source`
    /// would write the head every tick; it needs none, because rule 20.1 makes it the replay host
    /// and rule 11.8 leaves a replay writing no transcript.
    fn open(&mut self, _roster: &[RosterEntry<'_>]) -> io::Result<()> {
        Ok(())
    }

    /// Whether this source has stopped spending, asked before each opportunity is composed.
    ///
    /// The private half of [`Proposer::halted`], and the reason it exists on this trait too is the
    /// reason [`DecisionSource::failure`] does: the run loop drives a `DecisionSource` and knows
    /// nothing of a port. Only `PortDecisionSource` overrides it; the four deterministic sources take
    /// the default, so no run that existed before the port does can reach `SPEC-MOK-007` rule 14.6's
    /// stop, which is rule 16's non-perturbation obligation stated once more as a call graph.
    ///
    /// Defaulted to `false` on the same ground as the public method's default: a source that spends
    /// nothing cannot reach a spending ceiling.
    fn halted(&self) -> bool {
        false
    }

    fn decide(&mut self, observation: &Observation, entropy: &mut DecisionEntropy<'_>) -> Action;

    /// A failure the source reached while deciding, which ends the run.
    ///
    /// [`DecisionSource::decide`] returns an `Action` and cannot fail, and that is right for the
    /// four deterministic sources: a rule-based source always has an action. A port-backed one does
    /// not — `SPEC-MOK-007` rule 19.6 makes a failure to write the transcript fatal, and rules 12.3
    /// and 12.4 make a mismatched or exhausted transcript fatal — so the failure is latched during
    /// the decision and collected here, at a point where it can be returned rather than absorbed.
    /// It is taken rather than borrowed, so that the same failure is reported once by the run and
    /// the port stays the only party that repeats it.
    ///
    /// The alternative was `decide` returning `Result`, which would put a `?` on four sources that
    /// cannot fail and would change five call sites and every test helper that decides once, for a
    /// failure mode only the fifth source has.
    fn failure(&mut self) -> Option<io::Error> {
        None
    }
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
/// half: `REQ-MOK-057`'s amendment of 2026-08-20 hoists case 3 above that rule's two social
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

/// `SPEC-MOK-007` rule 20.8's refusal, in one sentence held in one place because two doors
/// state it and a third checks it for its own exit code.
pub(crate) const MISSING_DECISION_PORT: &str =
    "policy llm requires a decision port and none was supplied";

/// The `llm` source's name, as `SPEC-MOK-006` rule 3.2 admits it into `result.source`.
///
/// The four existing sources each answer with their own literal and nothing needs the string
/// before one exists. This one does: [`Simulation::initialization_events`] reports the selected
/// source and cannot build a [`PortDecisionSource`], having no port. Holding the string here
/// makes `config.policy`'s rendering, `result.source`'s value and that report one string rather
/// than three that agree today.
const LLM_SOURCE_NAME: &str = "llm";

/// Block A: the world's rules as prose, byte-identical across every request of every run.
///
/// `SPEC-MOK-007` rule 4 fixes what this states and rule 3.3 fixes that it never varies. It is
/// the leading span a provider's prompt cache matches, so a name, a tick, a count or a
/// whitespace difference inside it would destroy the shared prefix for every request of the run
/// at once. Rule 4.5 restates that as a content rule — no identity, no tick, no seed, no count
/// of anything that varies — so the property is checked when this text is edited rather than
/// only when a cache ratio regresses.
///
/// **It contains no strategy, no goal, no preference and no advice**, which is rule 4.4 and is
/// the reason `INT-MOK-011` sets no viability floor for this source: a block A that told the
/// model to survive would measure the instruction rather than the model. Every sentence below
/// states a mechanism the engine implements, and stops there. Nothing says that health is
/// better high, that combat is risky, or that any action is preferable to any other.
///
/// Rule 4.2 makes it a restatement for a reader and not a second authority: where it and
/// `SPEC-MOK-001` disagree, `SPEC-MOK-001` governs and this text is wrong and is corrected.
/// Rule 4.6 is why the closing section states the answer's grammar — a response has to be
/// well-formed from block A alone.
///
/// One literal per line, concatenated at compile time, on [`crate::cli::USAGE`]'s precedent and
/// for its reason: a multi-line literal would take its line endings from however the file was
/// checked out, and rule 3.3 does not admit bytes that depend on a checkout.
const SHARED_RULES: &str = concat!(
    "You are one Mokiterion. You will be given what you can see and the actions you may\n",
    "take, and you answer with exactly one of them.\n",
    "\n",
    "THE WORLD\n",
    "The world is a grid 128 cells wide and 128 cells tall. A cell has an x coordinate from\n",
    "0 to 127 and a y coordinate from 0 to 127. x rises toward the east and y rises toward\n",
    "the south. The grid is split into two territories: territory A is every cell with y\n",
    "from 0 to 63, territory B is every cell with y from 64 to 127. The boundary is a line\n",
    "on the map and not a barrier, and crossing it is an ordinary move. Nothing exists\n",
    "outside the grid, and a move that would leave it does not happen.\n",
    "\n",
    "Time passes in turns called ticks. In one tick every living Mokiterion is asked for one\n",
    "action, one after another, and each action is resolved before the next Mokiterion is\n",
    "asked. Food resources occupy cells. Any number of Mokiterions and resources may share\n",
    "one cell.\n",
    "\n",
    "YOUR ATTRIBUTES\n",
    "health, satiety, energy and fear are each an integer from 0 to 100.\n",
    "  health   A Mokiterion whose health reaches 0 is dead and acts no further. Nothing\n",
    "           raises health.\n",
    "  satiety  Falls by 1 at the end of every tick. Eating raises it.\n",
    "  energy   Falls by 1 at the end of every tick. Sleeping and eating raise it.\n",
    "  fear     Rises by 10 at the end of a tick in which another Mokiterion was perceived,\n",
    "           and falls by 5 at the end of a tick in which none was. Being threatened\n",
    "           raises it by 30 at the moment of the threat.\n",
    "At the end of a tick in which either satiety or energy is 0, health falls by 5.\n",
    "\n",
    "waste_tolerance is an integer from 0 to 40. It never changes and it permits and forbids\n",
    "nothing; it is stated because it is part of what a Mokiterion is.\n",
    "\n",
    "PERCEPTION\n",
    "A Mokiterion perceives every resource and every living Mokiterion at a distance of 16\n",
    "or less, where the distance between two cells is the larger of the two coordinate\n",
    "differences. Of each one it is given the identifier, the compass direction toward it\n",
    "and that distance. A distance of 0 means the same cell. Two Mokiterions at a distance\n",
    "of 1 or less — the same cell, or one of the eight cells around it — are in contact.\n",
    "Nothing else about another Mokiterion is perceived: not its health, not its satiety,\n",
    "not its energy, not its fear.\n",
    "\n",
    "RESOURCES\n",
    "A resource has a class, which is low, medium or high. Eating one raises satiety by 15,\n",
    "30 or 50 and energy by 5, 10 or 20 for those three classes. Neither attribute passes\n",
    "100, and whatever would have gone above 100 is lost. An eaten resource is removed from\n",
    "the world.\n",
    "\n",
    "THE FOUR CORE ACTIONS\n",
    "  wait                Nothing happens.\n",
    "  sleep               energy rises by 20, to no more than 100. Not permitted while\n",
    "                      energy is already 100.\n",
    "  eat <resource>      The resource must be in this Mokiterion's own cell. It is eaten\n",
    "                      and removed from the world.\n",
    "  move <direction>    One cell north, east, south or west. Not permitted where the\n",
    "                      destination would be outside the grid.\n",
    "\n",
    "THE SEVEN TARGETED ACTIONS\n",
    "Each names exactly one other living Mokiterion by its identifier. A step toward or away\n",
    "from a target moves on one axis: east or west while the target lies to the east or the\n",
    "west, otherwise north or south, and on the other axis when the first would leave the\n",
    "grid.\n",
    "  attack <who>        The target must be in contact. Its health falls by 10 plus a\n",
    "                      tenth of the sum of this Mokiterion's own energy and health, so\n",
    "                      by 10 to 30. This Mokiterion's energy falls by 5. A target whose\n",
    "                      health reaches 0 is dead.\n",
    "  threaten <who>      The target must be in contact. Its fear rises by 30, to no more\n",
    "                      than 100. Nothing else changes.\n",
    "  fight <who>         An attack, resolved identically, permitted only against a\n",
    "                      Mokiterion that struck this one since its previous action, and\n",
    "                      only while that Mokiterion is in contact.\n",
    "  retreat <who>       One cell away from the target. Permitted only against a\n",
    "                      Mokiterion that struck this one since its previous action.\n",
    "  surrender <who>     Permitted only against a Mokiterion that struck this one since\n",
    "                      its previous action. Half of this Mokiterion's satiety, rounded\n",
    "                      down, is taken from it and added to the target's, to no more than\n",
    "                      100; whatever would have gone above 100 is lost.\n",
    "  approach <who>      One cell toward the target. The target must be perceived and must\n",
    "                      not be in the same cell.\n",
    "  avoid <who>         One cell away from the target. The target must be perceived.\n",
    "\n",
    "REJECTION\n",
    "Every proposed action is checked against the world before it is applied, against the\n",
    "conditions stated with it above. A rejected action changes nothing. The tick then moves\n",
    "on, and the end-of-tick changes to satiety, energy, fear and health happen whether the\n",
    "action was applied or rejected. The actions offered already exclude those whose\n",
    "conditions what you are given shows to be unmet.\n",
    "\n",
    "YOUR ANSWER\n",
    "Answer with exactly one action from the set you are given. Name the verb, and where the\n",
    "verb takes a resource, a direction or another Mokiterion, name exactly one and write it\n",
    "exactly as that set writes it. Give no second action, no alternative, no explanation,\n",
    "no reason and no confidence.\n",
);

/// One decision opportunity, composed for a decision source that does not live in this engine.
///
/// `SPEC-MOK-007` rule 2.1: one tick, one living Mokiterion, one observation. Rule 2.2 fixes
/// that it carries four parts and nothing else, and rule 3.1 fixes their order, which
/// [`DecisionRequest::blocks`] is the single statement of. The order is not cosmetic: a
/// provider's prompt cache matches the longest identical leading span, blocks A and B do not
/// vary within a run, and blocks C and D sit last where varying costs nothing.
///
/// **It carries values only** — rule 1.3. There is no reference into engine state, no mutable
/// borrow and no handle, so holding one for any length of time cannot influence the run it came
/// from. Rules 2.3 and 2.4 are discharged by construction rather than by a check: there is no
/// part in which another Mokiterion's condition, a population aggregate, an earlier request, an
/// earlier response or a conversation identifier could be placed.
///
/// A host receives one of these and never builds one. That is what keeps the private
/// `Observation` and the private decision-source abstraction private while this source is
/// public, which is rule 20.6 and `ADR-MOK-001`'s trust boundary left where it was.
///
/// Block A is a `&'static str` and the other three are owned, which is deliberate rather than
/// an optimisation: the type then cannot hold a block A that differs from
/// [`SHARED_RULES`], so rule 3.3's byte-identity across a run is a property of the type and
/// not of a copy made correctly every time.
///
/// The two scalars beside the four blocks are the opportunity's identity, and they are held as
/// fields rather than read back out of block B's and block C's text. Rule 11.3 binds a transcript
/// record to "the tick and the acting Mokiterion", and rule 12.3 checks a record against "the
/// opportunity the engine has reached" — a check a port makes, from the request, because the
/// engine holds the opportunity and the port holds the record and neither holds both. The
/// alternative was for [`ReplayPort`] to parse `identifier:` out of block B and `tick:` out of
/// block C, which would make a change to either block's wording silently break the one check that
/// stands between a mismatched transcript and a plausible wrong run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRequest {
    tick: u64,
    actor_id: String,
    shared_rules: &'static str,
    actor: String,
    observation: String,
    permitted_set: String,
}

impl DecisionRequest {
    /// The four parts in rule 3.1's order, which is stated here and nowhere else.
    ///
    /// Rule 3.6 refuses an implementation that "composes the parts in this order and then
    /// serialises them through a structure whose field order is not guaranteed". A caller that
    /// wants the request's bytes takes them from here; a caller that wants to send blocks A and
    /// B as a cacheable prefix and C and D after it takes the first two and the last two of the
    /// same array, and cannot reorder them by accident.
    pub fn blocks(&self) -> [&str; 4] {
        [
            self.shared_rules,
            self.actor.as_str(),
            self.observation.as_str(),
            self.permitted_set.as_str(),
        ]
    }

    /// The tick this opportunity belongs to, the first half of rule 12.3's match.
    ///
    /// Also present inside block C as text. The duplication is the point: the block is what a
    /// model reads and this is what a program compares, and a comparison that had to find the
    /// figure inside prose would be a comparison held together by the prose staying put.
    pub fn tick(&self) -> u64 {
        self.tick
    }

    /// The acting Mokiterion's identifier, the second half of rule 12.3's match.
    ///
    /// Also present inside block B as text, for the reason [`DecisionRequest::tick`] gives.
    pub fn actor_id(&self) -> &str {
        &self.actor_id
    }

    /// Block A, rule 4: the shared rules, identical in every request of every run.
    pub fn shared_rules(&self) -> &str {
        self.shared_rules
    }

    /// Block B, rule 5: the acting Mokiterion's identifier and its `waste_tolerance`, and
    /// nothing else. Identical in every request for that Mokiterion in a run.
    pub fn actor(&self) -> &str {
        &self.actor
    }

    /// Block C, rule 6: the observation's varying fields, in the order rule 6.1 fixes.
    pub fn observation(&self) -> &str {
        &self.observation
    }

    /// Block D, rule 7: every action the specification permits at this opportunity.
    pub fn permitted_set(&self) -> &str {
        &self.permitted_set
    }

    /// Composes the four blocks from one observation and from nothing else.
    ///
    /// Rule 2.5 is what this being a pure function of the observation buys: the request is a
    /// run input, identical across two runs of the same seed, tick limit, density and tracing
    /// selection, which is what later lets a replay detect a transcript from a different
    /// configuration.
    fn compose(observation: &Observation) -> Self {
        Self {
            tick: observation.tick,
            actor_id: observation.agent_id.clone(),
            shared_rules: SHARED_RULES,
            actor: actor_block(observation),
            observation: observation_block(observation),
            permitted_set: permitted_set_block(observation),
        }
    }
}

/// The four token counts a provider reported for one exchange, each present or absent.
///
/// **`Option<u64>` and not `u64`, because `SPEC-MOK-007` rule 11.5 fixes the difference**: "a
/// reported count that the provider did not report is recorded as **absent**, not as zero", and
/// rule 14.5 is what depends on telling them apart. A run whose provider reported no cached-token
/// figure "cannot compute the ratio, and that is a failure to evaluate rather than a pass"; a run
/// that reported a cached figure of zero computed a ratio of zero and failed `REQ-MOK-070` on the
/// merits. Four absences and four zeros are the same bytes in any representation that loses this,
/// and the two outcomes are not the same outcome.
///
/// A named type rather than four more fields on [`Proposal`], for the reason [`UnitPrices`] is one
/// and measured the same way: the four are unlabelled integers of similar magnitude in a fixed
/// order, and rule 14's cost arithmetic and `REQ-MOK-070`'s cache ratio are both computed from
/// them, so a transposed pair is a wrong cost figure *and* a wrong ratio with nothing to catch
/// either. `SPEC-MOK-002` rule 5's 2026-08-29 row enumerates the type and its four fields.
///
/// [`Default`] is derived and its value is four **absent** counts, which is rule 11.3.1's reading
/// of an empty usage — "an empty usage is four **absent** counts rather than four zeros" — rather
/// than a convenience.
///
/// `accounting::ExchangeUsage` is an alias of this type and not a second one, so rule 14's cost
/// arithmetic reads the counts the port reported rather than a copy of them.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReportedUsage {
    /// Prompt tokens as the provider reported them, rule 10.4a's `usage.prompt`, and the
    /// denominator of `REQ-MOK-070`'s ratio.
    pub prompt: Option<u64>,
    /// Prompt tokens the provider reported as served from its cache, rule 10.4a's
    /// `usage.cached_prompt`, and that ratio's numerator.
    pub cached_prompt: Option<u64>,
    /// Output tokens, as reported.
    pub output: Option<u64>,
    /// Reasoning tokens, as reported. Rule 8.5 fixes the level a run may use at `none`, so this is
    /// where a run shows it got what it asked for. It is a **count** and not the level: rule 10.4a
    /// puts the level at the response's top level, as a string, and names both `reasoning`.
    pub reasoning: Option<u64>,
}

/// What a port obtained for one decision opportunity: the proposal, and the evidence of the
/// exchange it came from.
///
/// `SPEC-MOK-007` rule 1.1, as rule 1.1a grew it on 2026-08-29. **The proposal is `action` and
/// nothing else here is one.** Rule 1.4a states that plainly: the engine decides nothing from the
/// other two fields, so rule 9's validation is reached by exactly the value it was reached by
/// before this type existed. `response` is consumed by the transcript and by nothing else, and
/// `usage` by rule 14's accounting and by nothing else.
///
/// **The two evidence fields exist because rule 11.3 obliges the record to carry them and rule 11.1
/// obliges the engine to author it.** That rule asks for "the response as received, in full, or the
/// error", the four reported counts, and "the action the response was parsed into, or the fact that
/// it was not parsed **and why**" — and the port is the engine's only contact with whatever
/// answered, so a return of `Option<Action>` alone left the engine obliged to write three things it
/// had no route to. This module recorded that under `WO-MOK-025` as "a pre-existing tension between
/// rule 1.1's port shape and rule 11.3's field list", naming `WO-MOK-026` as where the return type
/// would have to grow. It grew here.
///
/// A port that reports a response contradicting its own action changes what the record says about
/// the exchange and changes no decision, which rule 1.4a records as the honest outcome rather than
/// as an oversight: rule 10.7 makes the connector untrusted in whole, and a transcript that recorded
/// a response the engine had silently overruled would hide exactly the disagreement a reader needs
/// to see.
///
/// **A fourth field arrived on 2026-08-29 for rule 19.5's retry**, and it is evidence of the same
/// kind rather than a second mechanism: [`Self::earlier_attempts`] carries the exchanges the
/// opportunity spent before this one, because rule 11.2 gives each attempt its own record and rule
/// 11.1 leaves the authoring of every record with the engine. It is empty for every port that does
/// not retry, which is every port but a live connector, and its own documentation states what the
/// engine does with it. `SPEC-MOK-002` rule 5's amendment of that date enumerates the field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    /// The proposal, or the fact that none was obtained. Rule 1.4's value, unchanged, and still the
    /// only field the engine acts on.
    pub action: Option<Action>,
    /// The response as received, in full, or the error, in the port's own bytes. `None` where there
    /// was no response at all — a port that spoke to nothing, which is every replay of a transcript
    /// recorded before a provider was ever called.
    pub response: Option<String>,
    /// The provider's reported counts. Four absences for a port with no provider behind it, which
    /// rule 11.5 distinguishes from four zeros and rule 14.5 acts on.
    pub usage: ReportedUsage,
    /// The exchanges this opportunity spent **before** the one this proposal came from, oldest
    /// first, and empty for the single-attempt case that is every exchange of every port but a
    /// retrying one.
    ///
    /// Rule 19.5 retries a failed exchange a bounded number of times and rule 11.2 gives each
    /// attempt "its own record, because it was its own billed exchange" — while rule 11.1 puts the
    /// authoring of every record in the engine. The attempts therefore have to reach the engine,
    /// and this is the field they reach it by. The engine writes one record per element, in this
    /// order, before the record for the proposal itself.
    ///
    /// **The list is flat and every element's own list is empty.** An element is one exchange, not
    /// a sub-run: a port fills this by pushing each attempt it abandons, so nesting would mean an
    /// attempt of an attempt, which rule 19.5 has no notion of.
    ///
    /// **No element carries an action**, because an attempt that returned one would have ended the
    /// retrying. That is an invariant of how a port fills the field rather than a shape the type
    /// enforces, and nothing depends on it: rule 1.4 makes [`Self::action`] of *this* proposal the
    /// whole of what the engine decides from, and an element's action would be read by nothing.
    ///
    /// **None of these records is rule 9.5's fallback and none is marked as one.** An attempt is not
    /// a decision; the opportunity's decision is this proposal's, and it is a fallback exactly when
    /// this proposal carries no action. Rule 15.4's count reconciles against the records marked as
    /// fallbacks, which `VER-MOK-018` case **P5** checks, so a run that retried three times and then
    /// succeeded moves that count by nothing and writes four records.
    ///
    /// Every element was billed. Rule 11.2's reason for the record is the reason for the field: the
    /// attempts are what the run paid for, and a retry that vanished from the evidence would be an
    /// exchange the transcript could not account for.
    pub earlier_attempts: Vec<Proposal>,
}

impl Proposal {
    /// Nothing obtained, nothing received and nothing reported: rule 9.5's case at its emptiest.
    ///
    /// A named function rather than a derived `Default`, because "the default proposal" names
    /// nothing a reader can check against a rule where "nothing was obtained" is rule 9.5's own
    /// words. [`ReportedUsage`] derives `Default` instead, four absent counts being a value rule
    /// 11.3.1 gives a meaning to.
    pub fn nothing() -> Self {
        Self {
            action: None,
            response: None,
            usage: ReportedUsage::default(),
            earlier_attempts: Vec::new(),
        }
    }
}

/// The engine's one interface for obtaining a proposal from outside itself.
///
/// `SPEC-MOK-007` rule 1.1: it takes a request by value and returns either a proposal or the
/// fact that none was obtained. **It names no provider, no transport, no model, no credential,
/// no file and no mode**, and there is no branch on live-versus-replay anywhere in this target.
/// The difference between recording a run and replaying one is entirely a difference in what
/// the host connected, which is what makes `REQ-MOK-067`'s byte-identity structural instead of
/// a second implementation to be kept in agreement with the first.
///
/// `None` is not an error type and carries no diagnosis. Rule 9.5 fixes what the engine does
/// with it — propose `wait`, the least consequential action and the one available at every
/// opportunity — and rule 9.7 forbids the alternative of borrowing another source's selection,
/// which would make the run a mixture of two sources under one label.
///
/// **The host builds this, owns it for the whole run, and lends it per tick.** The engine
/// builds none, holds none and closes none: what sits behind the port is a resource this target
/// is forbidden to hold, and rule 20.4.1 records the failure caller-ownership makes
/// unavailable — a port rebuilt each tick still compiles and still runs, and resets the
/// transcript cursor, the accumulated cost and the fallback count every tick.
///
/// `&mut self` for that reason: an implementation is expected to carry state across a run.
pub trait Proposer {
    /// Whether this port has stopped spending, asked **before** each exchange is composed.
    ///
    /// Rule 14.6's check, at the only place it can honestly be made: the port is what spends, so the
    /// port is what knows whether the declared ceiling has been reached, and the engine — which by
    /// rule 14's *State model* may read no accounting figure at all — asks a question rather than
    /// inspecting a number. A port that answers `true` is not asked for a proposal at that
    /// opportunity: the request is not composed, nothing is written to it, and the run ends. That is
    /// what makes the check *precede* the spending rather than trail it by one exchange.
    ///
    /// **Defaulted to `false`, and the default is rule 14.8.** A replay spends nothing and has no
    /// ceiling, so [`ReplayPort`] takes this default rather than implementing it, and so does any
    /// port with nothing behind it. A port that never spends can never halt, and stating that by
    /// omission is the same shape [`Proposer::record`]'s replay behaviour already has.
    ///
    /// `&self` and not `&mut self`: asking must not move a figure. A method that could bill, count or
    /// advance a cursor while answering would make the number the answer depends on a function of how
    /// often the engine asked, and the engine asks once per opportunity today and is not held to
    /// that by anything.
    ///
    /// This is not rule 1.1a's declined second method. That one would have returned the *previous*
    /// exchange's evidence, so its correctness rested on a temporal contract between two calls that
    /// no type enforces. This one is asked before an exchange exists, its answer concerns no
    /// exchange, and a port that answered it wrongly stops a run early or spends past a ceiling —
    /// both visible in the run's own figures rather than silently written into a record.
    fn halted(&self) -> bool {
        false
    }

    /// Proposes one action for one decision opportunity, or reports that none was obtained,
    /// together with the evidence of the exchange it came from.
    ///
    /// [`Proposal::action`] is rule 1.4's value and the only field the engine decides from. The
    /// other two are rule 11.3's obligations travelling to the engine that has to author the
    /// record, and rule 1.4a states what they are not: not an answer, and not consulted by any
    /// decision. A port with no provider behind it returns [`Proposal::nothing`] or a proposal whose
    /// evidence fields are empty, and neither is an error.
    fn propose(&mut self, request: DecisionRequest) -> Proposal;

    /// Takes one transcript record, already authored, and does whatever the host wants done with
    /// it.
    ///
    /// Rule 11.1 splits the transcript in two: **the engine authors every record and the host owns
    /// the destination.** This is the seam. The engine formats the line — the framing, the field
    /// set, the escaping, the version — hands it over complete with no trailing newline, and
    /// learns nothing about where it went. A recording host appends it to the stream it opened. A
    /// replay host discards it, because rule 11.8's "a replay writes no transcript" is a statement
    /// about the destination and not about the authoring, and rule 12.1's "the same code path a
    /// live run uses" is only true if the authoring happens either way.
    ///
    /// **This is also the port's one opportunity to stop the run**, and both hosts need it. A
    /// recording host reports a failure to write, which rule 19.6 makes fatal: a live run whose
    /// exchanges were spent and not recorded has produced cost and no evidence. A replay host
    /// reports rule 12.3's mismatch and rule 12.4's exhaustion, which it detected in
    /// [`Proposer::propose`] and could not report from there, because a proposal's absence is rule
    /// 9.5's ordinary fallback and a mismatched transcript is not. The engine calls this once per
    /// exchange, after the proposal and **before the proposed action is applied**, so an error here
    /// ends the run with that opportunity unresolved rather than half-resolved.
    ///
    /// The engine does not interpret the error, and the error is the port's own words. Rule 19.7
    /// is the port's to keep: no credential, and no path, because the engine resolved none and has
    /// nothing to say about the one the host opened.
    fn record(&mut self, record: &str) -> io::Result<()>;
}

/// Block B, rule 5: the identifier and the trait constant, and rule 5.2's nothing else.
///
/// Rule 3.4 is why `waste_tolerance` is here and `health`, `satiety`, `energy` and `fear` are
/// not: this block must not vary within a run, and a trait constant does not.
fn actor_block(observation: &Observation) -> String {
    actor_block_of(&observation.agent_id, observation.waste_tolerance)
}

/// [`actor_block`] from the two values it reads and from nothing else.
///
/// Split out for the transcript's prefix head, which states blocks A and B once per Mokiterion
/// before the run's first exchange and therefore before any observation has been composed. Rule
/// 5.2's "nothing else" is what makes the split possible at all: a block that read a varying
/// attribute could not be written ahead of the tick it belonged to. The two callers share this
/// body rather than each formatting the block, so a prefix in the head and the prefix in every
/// request for that Mokiterion cannot differ.
fn actor_block_of(agent_id: &str, waste_tolerance: u8) -> String {
    format!("YOU\nidentifier: {agent_id}\nwaste_tolerance: {waste_tolerance}\n")
}

/// Block C, rule 6.1's fields in rule 6.1's order.
///
/// Every list states its emptiness rather than omitting its line, which is rule 6.5, and an
/// absent relative direction states `same_cell` rather than being omitted or given a sentinel,
/// which is rule 6.3 adopting `SPEC-MOK-006` rule 4.4's principle that an absence is stated as
/// an absence.
///
/// **There is no aggregate here and no count of anything**, rule 6.6: not a population figure,
/// not a mean, not a ranking, and not a length of any of the lists below. `REQ-MOK-059` already
/// forbids the engine to read a population-level aggregate, and rule 6.6 forbids composing one
/// out of what it may read — including out of a list whose entries are all present anyway.
fn observation_block(observation: &Observation) -> String {
    let mut block = format!(
        "WHAT YOU SEE\ntick: {}\nposition: {} in territory {}\nhealth: {}\nsatiety: {}\nenergy: {}\nfear: {}\n",
        observation.tick,
        observation.position,
        observation.territory,
        observation.health,
        observation.satiety,
        observation.energy,
        observation.fear
    );

    // Rule 6.4: the engine's own one-tick memory, rendered as part of the observation and in
    // the order the attacks resolved. An attacker's identifier renders; nothing about an
    // attacker's condition renders, because nothing about it is carried.
    block.push_str("attacks suffered since your previous action:\n");
    if observation.suffered.is_empty() {
        block.push_str("  none\n");
    } else {
        for attack in &observation.suffered {
            block.push_str(&format!("  {} for {}\n", attack.attacker, attack.damage));
        }
    }

    block.push_str("resources in your cell:\n");
    if observation.co_located_food.is_empty() {
        block.push_str("  none\n");
    } else {
        for food_id in &observation.co_located_food {
            block.push_str(&format!("  {food_id}\n"));
        }
    }

    block.push_str("resources perceived:\n");
    if observation.perceived_food.is_empty() {
        block.push_str("  none\n");
    } else {
        for food in &observation.perceived_food {
            block.push_str(&format!(
                "  {} class {} direction {} distance {}\n",
                food.id,
                food.class,
                relative_direction_form(food.direction),
                food.distance
            ));
        }
    }

    // Rule 6.2: exactly the three values the observation carries, because no attribute of a
    // perceived Mokiterion is available to render.
    block.push_str("mokiterions perceived:\n");
    if observation.perceived_mokiterions.is_empty() {
        block.push_str("  none\n");
    } else {
        for other in &observation.perceived_mokiterions {
            block.push_str(&format!(
                "  {} direction {} distance {}\n",
                other.id,
                relative_direction_form(other.direction),
                other.distance
            ));
        }
    }

    block
}

/// Rule 6.3's stated word for the co-located case.
fn relative_direction_form(direction: Option<RelativeDirection>) -> String {
    match direction {
        Some(direction) => direction.to_string(),
        None => "same_cell".to_string(),
    }
}

/// Block D, rule 7: every action the specification permits this Mokiterion to propose at this
/// opportunity, with each targeted action named against each target it may name.
///
/// **It is composed beside the observation's core-proposal list and never by extending it**,
/// which is rule 7.2 and `SPEC-MOK-001` rule 3's own reason: rule 4's baseline consumes one
/// entropy selection over that list's length, so a longer list would move that selection and
/// every run ever recorded under `baseline` would diverge. The core proposals are read from it;
/// the seven targeted verbs are enumerated here and never pushed onto it.
///
/// **Rule 7.4 is what each loop below implements.** A verb is enumerated against a target only
/// where the preconditions `SPEC-MOK-001` rule 6 states for it are met by what the observation
/// carries, so block D never offers an action the engine would reject on a ground block D could
/// have known about — an `approach` against a Mokiterion in the same cell is the case that
/// makes the difference visible, and it is excluded. What rule 7.5 keeps *in* is an action the
/// engine may still reject on a ground the observation does not carry; that is an ordinary
/// rejected proposal, and rule 9.6 keeps it out of the fallback count.
///
/// Rule 7.7: the order is derived from the observation's order and is therefore fixed. The core
/// proposals come in the order the observation carries them, and the seven verbs come in
/// `SPEC-MOK-001` rule 21's order, each against the perceived Mokiterions in the observation's
/// own ascending distance-then-identifier order.
fn permitted_set_block(observation: &Observation) -> String {
    let mut block = String::from("ACTIONS YOU MAY TAKE\n");
    for action in &observation.valid_actions {
        push_permitted(&mut block, action);
    }

    let in_contact = |other: &PerceivedMokiterion| other.distance <= CONTACT_RADIUS;
    let struck_me = |other: &PerceivedMokiterion| {
        observation
            .suffered
            .iter()
            .any(|attack| attack.attacker == other.id)
    };
    let others = || observation.perceived_mokiterions.iter();

    for other in others().filter(|other| in_contact(other)) {
        push_permitted(
            &mut block,
            &Action::Attack {
                target: other.id.clone(),
            },
        );
    }
    for other in others().filter(|other| in_contact(other)) {
        push_permitted(
            &mut block,
            &Action::Threaten {
                target: other.id.clone(),
            },
        );
    }
    for other in others().filter(|other| in_contact(other) && struck_me(other)) {
        push_permitted(
            &mut block,
            &Action::Fight {
                target: other.id.clone(),
            },
        );
    }
    for other in others().filter(|other| struck_me(other)) {
        push_permitted(
            &mut block,
            &Action::Retreat {
                target: other.id.clone(),
            },
        );
    }
    for other in others().filter(|other| struck_me(other)) {
        push_permitted(
            &mut block,
            &Action::Surrender {
                target: other.id.clone(),
            },
        );
    }
    for other in others().filter(|other| other.distance > 0) {
        push_permitted(
            &mut block,
            &Action::Approach {
                target: other.id.clone(),
            },
        );
    }
    for other in others() {
        push_permitted(
            &mut block,
            &Action::Avoid {
                target: other.id.clone(),
            },
        );
    }

    block
}

fn push_permitted(block: &mut String, action: &Action) {
    block.push_str("  ");
    block.push_str(&permitted_form(action));
    block.push('\n');
}

/// The port's rendering of an action: the verb, then its one parameter where it has one,
/// separated by a space.
///
/// **This is not [`Action`]'s `Display`, and the two are deliberately different.** That one is
/// the `REQ-MOK-010` text stream's, where `CAP-MOK-010` holds every line a core verb appears on
/// byte-identical and where a targeted verb renders as the bare verb because its target is a
/// field of its own beside `proposal`. A request that dropped the target would name no action at
/// all, so this rendering is the port's own and is the single place the port's grammar is
/// written: block A describes this form, block D enumerates in it, and rule 8.2's closed
/// grammar is checked against it.
fn permitted_form(action: &Action) -> String {
    match action {
        Action::Wait => "wait".to_string(),
        Action::Sleep => "sleep".to_string(),
        Action::Eat { food_id } => format!("eat {food_id}"),
        Action::Move { direction } => format!("move {direction}"),
        Action::Attack { target } => format!("attack {target}"),
        Action::Threaten { target } => format!("threaten {target}"),
        Action::Fight { target } => format!("fight {target}"),
        Action::Retreat { target } => format!("retreat {target}"),
        Action::Surrender { target } => format!("surrender {target}"),
        Action::Approach { target } => format!("approach {target}"),
        Action::Avoid { target } => format!("avoid {target}"),
    }
}

// ---------------------------------------------------------------------------------------
// The transcript, `SPEC-MOK-007` rules 11 and 12.
//
// Two record kinds, one per line, written by `format!` for the reason the record stream above is
// written by `write!`: the engine's dependency table is empty and no rule permits a crate here.
//
// A **prefix** record states blocks A and B for one Mokiterion, once, before the run's first
// exchange. An **exchange** record states blocks C and D, the digest of that Mokiterion's prefix,
// and what the exchange yielded. Rule 3.4 is what makes the split sound — neither of the first two
// blocks varies within a run for a given Mokiterion — and the arithmetic is why it was chosen.
//
// The figures are measured, by `a_record_carries_the_variable_part_and_the_head_carries_the_rest`
// on a 20-tick run of twelve Mokiterions at seed 42. Block A is 5,385 bytes and a prefix record is
// 5,620, so the head is 67,450 bytes once; an exchange record averages 996. A record carrying the
// request "in full" would therefore average 6,616 bytes, of which 5,620 would be repetition — that
// run's 240 exchanges would be 1.6 MB instead of the measured 306,552, and a 1,000-tick run's
// estimated 10,954 exchanges would be an estimated 72 MB.
//
// **Rule 11.7 no longer estimates.** Its band — "an estimated 4.7 MB for a 1,000-tick run, an
// estimated 100 to 260 KB for a 20-to-50-tick run" — was withdrawn on 2026-08-24 for measured
// figures, and rule 11.7.1 records the withdrawn wording rather than deleting it, because the
// measurement exceeded both ends of it. The rule's own figures are the committed transcript's, at
// seed 0: 305,568 bytes and a mean exchange record of 1,078. The figures above are this module's,
// at seed 42; the two runs differ and both are measured, so neither supersedes the other.
//
// **The split does not reach the withdrawn band either**: 20 ticks measures 299 KB here and 1,000
// ticks extrapolates to an estimated 11 MB, because blocks C and D are larger than the estimate
// assumed. That is a size to record, not a reason to abbreviate a transcript — rule 11.7's first
// sentence forbids exactly that, and the split is the whole of what can be saved without dropping
// a byte anyone recorded.
//
// A digest in each exchange is what keeps the prefix load-bearing rather than decorative: rule
// 12.3's mismatch check is extended to it, so an edit to block A invalidates every transcript
// taken before the edit, loudly, at the first exchange.
//
// **The digest is FNV-1a 64 and is not cryptographic**, which is stated rather than implied. Its
// job is drift detection between a prefix and the run that replays against it, over bytes that sit
// in the same file as the digest; an adversary who can edit the prefix can edit the digest beside
// it, and no digest of any strength changes that. The alternatives were a crate, which
// `SPEC-MOK-006` rule 12.4 forbids, and roughly eighty lines of hand-written SHA-256, which would
// be a second thing to verify for no property this needs.
//
// **Rule 11.4's closed alphabet does not hold here and cannot.** The record stream's alphabet is
// `A-Z a-z 0-9 _ . - + : ; >` and blocks A to D are English prose: block A alone carries 1,282
// spaces, 44 commas, 9 less-than signs, 5 apostrophes and 2 em dashes, none of which is in that
// alphabet, and all four blocks are multi-line. Note that `.` and `>` are in it and `<` is not,
// which is why the list is measured rather than described. Rule 11.4
// adopts `SPEC-MOK-006`'s constraints, and `SPEC-MOK-006` rule 3.4 names this exact branch — a
// value outside the enumeration "must either be added to that enumeration or arrive together with
// an escaping function and its own verification". This is that function, and
// `every_block_survives_the_escaping_unchanged`, `the_escaping_survives_the_framing_and_round_trips`
// and `an_escape_this_module_never_writes_is_not_read_generously` are that verification.
// The transcript is still comparable with `cmp` and still carries no floating-point value, no
// timestamp and no path, which is the rest of rule 11.4 and the part that made it a rule.
//
// **How the response and the four counts reach a record, and what an empty one still says.** Rule
// 11.3 asks for the response as received in full and for the provider's four reported token counts,
// and until `WO-MOK-026` neither was obtainable: `Proposer::propose` returned `Option<Action>`, so
// the engine never saw a response and was never told a count. This module recorded that under
// `WO-MOK-025` as a tension between rule 1.1's port shape and rule 11.3's field list, and named this
// work order as where the return type would have to grow. It grew: `SPEC-MOK-007` rule 1.1a of
// 2026-08-29 puts the evidence on [`Proposal`] beside the action, and both fields are carried here
// from whatever the port obtained, escaped by the same function every other string value goes
// through.
//
// A port with nothing behind it still authors a record, and the empty values are **absences, not
// zeros** — rule 11.5. `response` is `null` where the port received nothing; `usage` is `null` where
// **all four** counts are absent, and otherwise an object whose four members are each an integer or
// `null`. `null` for four absences rather than an object of four nulls, because the specification's
// own "an exchange that yielded nothing" example writes it that way, and because it makes every
// transcript recorded before a provider existed replay byte-identically under rule 12.6 rather than
// needing a version bump for a shape no reader would read differently.
// ---------------------------------------------------------------------------------------

/// The transcript's own version integer, on every record of both kinds.
///
/// Separate from [`RECORD_SCHEMA_VERSION`] and deliberately so: the two streams are read by
/// different things for different reasons and neither's compatibility question is the other's. A
/// reader that meets a version it does not know refuses rather than guessing, which is the whole
/// purpose of putting the integer on every line instead of only in a head record — a transcript
/// truncated at any line boundary is still self-describing.
const TRANSCRIPT_VERSION: u32 = 1;

/// FNV-1a's 64-bit offset basis and prime, from the algorithm's own definition.
const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// FNV-1a 64, continued from an existing state so that a digest over two blocks needs no
/// concatenation of them. `wrapping_mul` is the algorithm, not an overflow accommodation.
fn fnv1a64(state: u64, bytes: &[u8]) -> u64 {
    let mut hash = state;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// The digest of one Mokiterion's cacheable prefix: blocks A and B, in rule 3.1's order.
///
/// The algorithm is named in the value rather than left to a reader of this file, so that a
/// transcript read years from now says what produced its own digests.
fn prefix_digest(shared_rules: &str, actor: &str) -> String {
    let hash = fnv1a64(FNV_OFFSET_BASIS, shared_rules.as_bytes());
    format!("fnv1a64:{:016x}", fnv1a64(hash, actor.as_bytes()))
}

/// One block, as a transcript record's string value.
///
/// Three characters have to be escaped for the framing to survive — the backslash, the quotation
/// mark and the newline — and the tab, the carriage return and every remaining control character
/// are escaped as well. Those last are not in any block today and the test beside this says so;
/// they are handled anyway, so that this function is total over `&str` rather than total over the
/// blocks as they happen to be worded, which is the difference between an escaping function and a
/// transformation that works until a block gains a character.
fn escape_transcript_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len() + text.len() / 16);
    for character in text.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            control if control.is_control() => {
                escaped.push_str(&format!("\\u{:04x}", control as u32));
            }
            other => escaped.push(other),
        }
    }
    escaped
}

/// [`escape_transcript_text`]'s inverse. `None` on an escape this module never writes, because a
/// transcript it cannot read is a failure and not a line to interpret generously.
fn unescape_transcript_text(text: &str) -> Option<String> {
    let mut plain = String::with_capacity(text.len());
    let mut characters = text.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            plain.push(character);
            continue;
        }
        match characters.next()? {
            '\\' => plain.push('\\'),
            '"' => plain.push('"'),
            'n' => plain.push('\n'),
            'r' => plain.push('\r'),
            't' => plain.push('\t'),
            'u' => {
                let digits: String = characters.by_ref().take(4).collect();
                if digits.chars().count() != 4 {
                    return None;
                }
                plain.push(char::from_u32(u32::from_str_radix(&digits, 16).ok()?)?);
            }
            _ => return None,
        }
    }
    Some(plain)
}

/// One string field's value, unescaped, from a record this module wrote.
///
/// **This is a field reader and not a parser, and the distinction is load-bearing.** It finds
/// `"<field>":"` and reads to the next unescaped quotation mark, which is exact here for one
/// reason: [`escape_transcript_text`] leaves no unescaped quotation mark inside any value, so the
/// needle cannot occur within one. `\"tick\":` is not `"tick":`. A record from anywhere else may
/// well defeat this, and that is what rule 12.3's checks are for — a field this cannot read is
/// absent, and an absent field the replay needs is a named failure rather than a default.
fn transcript_string(line: &str, field: &str) -> Option<String> {
    let needle = format!("\"{field}\":\"");
    let start = line.find(&needle)? + needle.len();
    let value = &line[start..];
    let mut escaped = false;
    let mut end = None;
    for (offset, character) in value.char_indices() {
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == '"' {
            end = Some(offset);
            break;
        }
    }
    unescape_transcript_text(&value[..end?])
}

/// One unsigned field's value. `None` when the field is absent or is not a run of digits.
fn transcript_number(line: &str, field: &str) -> Option<u64> {
    let needle = format!("\"{field}\":");
    let start = line.find(&needle)? + needle.len();
    let digits: String = line[start..]
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect();
    digits.parse().ok()
}

/// One boolean field's value. `None` when the field is absent or is neither literal.
fn transcript_flag(line: &str, field: &str) -> Option<bool> {
    let needle = format!("\"{field}\":");
    let start = line.find(&needle)? + needle.len();
    let value = &line[start..];
    if value.starts_with("true") {
        Some(true)
    } else if value.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

/// An action as a transcript record states it: the verb, and its one parameter where it has one.
///
/// **The parameter is called `parameter` and not `target`**, which departs from the abbreviated
/// example under rule 11.3 and does so deliberately. Nine of the eleven verbs take a target; `eat`
/// takes a resource identifier and `move` takes a direction, and [`Action::target`] returns `None`
/// for both. A field named `target` would therefore be false on exactly the records where it was
/// most likely to be read as one, and rule 11.3's own words are "the action the response was parsed
/// into" rather than a field list.
///
/// The rendering is [`permitted_form`]'s, split at its one space. Block D enumerates in that
/// grammar and rule 8.2's closed grammar is checked against it, so an action a transcript states
/// is an action block D could have offered, spelled the way it was offered.
fn action_parts(action: &Action) -> (&'static str, Option<String>) {
    match action {
        Action::Wait => ("wait", None),
        Action::Sleep => ("sleep", None),
        Action::Eat { food_id } => ("eat", Some(food_id.clone())),
        Action::Move { direction } => ("move", Some(direction.to_string())),
        Action::Attack { target } => ("attack", Some(target.clone())),
        Action::Threaten { target } => ("threaten", Some(target.clone())),
        Action::Fight { target } => ("fight", Some(target.clone())),
        Action::Retreat { target } => ("retreat", Some(target.clone())),
        Action::Surrender { target } => ("surrender", Some(target.clone())),
        Action::Approach { target } => ("approach", Some(target.clone())),
        Action::Avoid { target } => ("avoid", Some(target.clone())),
    }
}

/// [`action_parts`]'s inverse, and `None` for anything the grammar does not admit — an unknown
/// verb, a verb given a parameter it does not take, a verb missing the one it does.
///
/// `move`'s direction is resolved by rendering each of the four rather than by a table of its own,
/// so the two directions of this conversion cannot disagree about how a direction is spelled.
fn action_from_parts(verb: &str, parameter: Option<&str>) -> Option<Action> {
    let named = |target: Option<&str>| target.map(str::to_string);
    match (verb, parameter) {
        ("wait", None) => Some(Action::Wait),
        ("sleep", None) => Some(Action::Sleep),
        ("eat", Some(food_id)) => Some(Action::Eat {
            food_id: food_id.to_string(),
        }),
        ("move", Some(name)) => Direction::ORDERED
            .into_iter()
            .find(|direction| direction.to_string() == name)
            .map(|direction| Action::Move { direction }),
        ("attack", target) => named(target).map(|target| Action::Attack { target }),
        ("threaten", target) => named(target).map(|target| Action::Threaten { target }),
        ("fight", target) => named(target).map(|target| Action::Fight { target }),
        ("retreat", target) => named(target).map(|target| Action::Retreat { target }),
        ("surrender", target) => named(target).map(|target| Action::Surrender { target }),
        ("approach", target) => named(target).map(|target| Action::Approach { target }),
        ("avoid", target) => named(target).map(|target| Action::Avoid { target }),
        _ => None,
    }
}

/// One Mokiterion's invariant identity, as the transcript's prefix head needs it.
///
/// Two values and nothing else, which is exactly rule 5.2's content for block B. It is
/// deliberately not a `&Mokiterion`: a decision source that held one could read a position, a
/// health, an energy and a fear, and `ADR-MOK-001`'s boundary and `SPEC-MOK-002` rule 6 both refuse
/// that. What a source receives here it can compose a prefix from and can compose nothing else
/// from.
struct RosterEntry<'a> {
    id: &'a str,
    waste_tolerance: u8,
}

/// One prefix record: blocks A and B for one Mokiterion, with their digest.
///
/// `blocks` is an array of the two rather than two named fields, because rule 3.6 fixes the order
/// in one place and an array carries it. A reader that wants the prefix as sent concatenates the
/// array; there is no second statement of which comes first.
fn prefix_record(entry: &RosterEntry<'_>) -> String {
    let actor = actor_block_of(entry.id, entry.waste_tolerance);
    format!(
        "{{\"transcript\":\"prefix\",\"version\":{TRANSCRIPT_VERSION},\"actor\":\"{}\",\
         \"digest\":\"{}\",\"blocks\":[\"{}\",\"{}\"]}}",
        entry.id,
        prefix_digest(SHARED_RULES, &actor),
        escape_transcript_text(SHARED_RULES),
        escape_transcript_text(&actor),
    )
}

/// One exchange record: the opportunity, the prefix it was sent against, blocks C and D, and what
/// came back.
///
/// `prefix` repeats `actor` on purpose. The two must be equal, so the replay compares them, and a
/// redundant field that is checked is a check rather than a hazard.
///
/// `fallback` is the field rule 12.7 replays from, and it is why the proposal's absence is recorded
/// as its own fact instead of being inferred from the action. `wait` is a proposal a source may
/// legitimately make, so a record showing `wait` with no flag would be indistinguishable from an
/// exchange that yielded nothing — and the two must replay differently, because one moves
/// `REQ-MOK-074`'s count and the other does not.
fn exchange_record(request: &DecisionRequest, proposal: &Proposal) -> String {
    exchange_line(request, proposal, proposal.action.is_none())
}

/// One exchange record for a retried attempt: rule 11.2's "a retry is its own record, because it was
/// its own billed exchange".
///
/// Identical to [`exchange_record`] in every field but one. **`fallback` is `false` whatever the
/// attempt returned**, and that is rule 11.3's own wording rather than a convenience: the field
/// states "whether the **decision** was rule 9.5's fallback", and an abandoned attempt is not a
/// decision — the opportunity's decision is the proposal the retrying ended with. Rule 15.4's count
/// is reconciled against the records marked as fallbacks, which case **P5** checks, so marking an
/// attempt would make a run that retried once and then succeeded report a fallback it never took.
///
/// The request is the same request. Rule 19.5 retries *the exchange*, not the opportunity, so blocks
/// C and D, the tick, the actor and the prefix digest are the ones already composed; a retry that
/// recomposed them would be a second opportunity wearing the first one's tick.
fn attempt_record(request: &DecisionRequest, attempt: &Proposal) -> String {
    exchange_line(request, attempt, false)
}

/// The bytes both record kinds share, with `fallback` supplied by the caller rather than derived.
fn exchange_line(request: &DecisionRequest, proposal: &Proposal, fallback: bool) -> String {
    let [shared_rules, actor, observation, permitted_set] = request.blocks();
    let (verb, parameter) = match &proposal.action {
        Some(action) => action_parts(action),
        None => action_parts(&Action::Wait),
    };

    let mut record = format!(
        "{{\"transcript\":\"exchange\",\"version\":{TRANSCRIPT_VERSION},\"tick\":{},\
         \"actor\":\"{}\",\"prefix\":\"{}\",\"prefix_digest\":\"{}\",\"observation\":\"{}\",\
         \"permitted\":\"{}\",\"response\":{},\"usage\":{},\"action\":{{\"verb\":\"{verb}\"",
        request.tick(),
        request.actor_id(),
        request.actor_id(),
        prefix_digest(shared_rules, actor),
        escape_transcript_text(observation),
        escape_transcript_text(permitted_set),
        transcript_response_value(proposal.response.as_deref()),
        transcript_usage_value(&proposal.usage),
    );
    if let Some(parameter) = parameter {
        record.push_str(&format!(",\"parameter\":\"{parameter}\""));
    }
    record.push_str(&format!("}},\"fallback\":{fallback}}}"));
    record
}

/// The `response` field's value: the bytes the port received, escaped, or the format's absence.
///
/// Escaped by [`escape_transcript_text`] and by nothing of its own, which is what makes rule
/// 11.4.1's round-trip obligation hold for a response as much as for a block. It matters more here
/// than anywhere else in this module: every other string value on a record is composed by this
/// library from its own vocabulary, and this one arrives from a program rule 10.7 declares untrusted
/// in whole. A response is recorded **as received**, in full — not summarised, not truncated, not
/// re-serialised from whatever the host managed to parse out of it — because rule 11.3's purpose is
/// that a reader can see what the engine was actually sent, including the ill-formed line that
/// became a fallback.
fn transcript_response_value(response: Option<&str>) -> String {
    match response {
        Some(response) => format!("\"{}\"", escape_transcript_text(response)),
        None => "null".to_string(),
    }
}

/// The `usage` field's value: rule 11.5's four counts, each present or absent.
///
/// `null` when **all four** are absent, and otherwise an object stating all four with `null` for the
/// ones that are. Two levels of absence for one reason: rule 11.5 fixes that an unreported count is
/// absent and not zero, and both "the provider reported nothing" and "the provider reported three of
/// the four" have to survive to a reader. Collapsing the first to an object of four nulls would
/// change every record produced before a provider existed, which rule 12.6's byte-identity would
/// then read as drift; collapsing the second by omitting members would leave a reader unable to tell
/// an absent count from a field this module forgot to write.
fn transcript_usage_value(usage: &ReportedUsage) -> String {
    let counts = [
        usage.prompt,
        usage.cached_prompt,
        usage.output,
        usage.reasoning,
    ];
    if counts.iter().all(Option::is_none) {
        return "null".to_string();
    }
    let [prompt, cached_prompt, output, reasoning] = counts.map(transcript_count_value);
    format!(
        "{{\"prompt\":{prompt},\"cached_prompt\":{cached_prompt},\"output\":{output},\
         \"reasoning\":{reasoning}}}"
    )
}

/// One reported count as a record states it: the integer, or the format's absence.
fn transcript_count_value(count: Option<u64>) -> String {
    match count {
        Some(count) => count.to_string(),
        None => "null".to_string(),
    }
}

/// Distinguishes a transcript failure from a record-sink failure and from the engine's own,
/// which rule 19.4 requires of the diagnostic and `sink_error` is the precedent for.
///
/// The form carries that the transcript was what failed and the port's own reason, and it carries
/// no path: rule 19.7, and the engine has no path to carry.
fn transcript_error(error: io::Error) -> io::Error {
    io::Error::new(error.kind(), format!("transcript: {error}"))
}

/// A [`Proposer`] that answers from a transcript instead of from a model, `SPEC-MOK-007` rule 12.
///
/// **This is the engine library's own item and rule 12.1.1 says why**: both hosts replay, this is
/// the one place where they do the same work, and a reader written once per host would be two
/// copies to keep in agreement. Each host supplies only the open reader — it resolves the path, it
/// opens the file, it owns the handle for the whole run, and this type performs no filesystem
/// operation of its own, which is rule 20.4 and what keeps `SPEC-MOK-001`'s "the library target
/// interprets no path at all" true.
///
/// It reaches nothing. Rule 12.2: no provider call, no socket, no spawned connector, no credential
/// read, whether or not one is present in the environment. `R: BufRead` is the whole of its
/// contact with the outside world, and a `&[u8]` satisfies it, which is what the tests use.
///
/// **The cursor is why rule 20.4.1 exists.** A port rebuilt each tick would restart at the first
/// record and replay tick 1 forever while reporting no drift, so the host builds this once and
/// lends it per tick.
///
/// A failure — rule 12.3's mismatch or rule 12.4's exhaustion — is latched and permanent. Once one
/// has been reached this port proposes nothing further and reports the same reason for every
/// subsequent exchange, which is rule 12.3's "produces no further ticks" as a property of the port
/// rather than as an obligation on a host to notice.
pub struct ReplayPort<R: BufRead> {
    reader: R,
    /// Every prefix digest the head declared, by actor identifier, in the head's order.
    prefixes: Vec<(String, String)>,
    /// The head is read once, at the first proposal, because there is no other moment the engine
    /// gives a port.
    head_read: bool,
    /// Reading the head consumes one line too many. That line is the first exchange and waits here.
    pending: Option<String>,
    /// Rule 12.3's and 12.4's failure, latched.
    failure: Option<String>,
    /// Exchanges served, so that rule 12.4's diagnosis can say how far the transcript went.
    served: u64,
}

impl<R: BufRead> ReplayPort<R> {
    /// Wraps an already-open reader. Nothing is read here: a run refused before its first tick
    /// must not have consumed a byte, and rule 20.8's refusal happens after construction.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            prefixes: Vec::new(),
            head_read: false,
            pending: None,
            failure: None,
            served: 0,
        }
    }

    /// The next line that is not blank, or `None` at the transcript's end.
    ///
    /// The trailing newline is trimmed in both spellings, so a transcript checked out with `CRLF`
    /// endings replays identically to one checked out with `LF`. Nothing else about a line is
    /// normalized: a record's own bytes are what rule 12.3 compares.
    fn next_content_line(&mut self) -> io::Result<Option<String>> {
        loop {
            let mut line = String::new();
            if self.reader.read_line(&mut line)? == 0 {
                return Ok(None);
            }
            let line = line.trim_end_matches(['\n', '\r']).to_string();
            if !line.trim().is_empty() {
                return Ok(Some(line));
            }
        }
    }

    /// Reads the prefix head, stopping at the first line that is not a prefix record.
    ///
    /// Returns that line, which is the first exchange, or `None` for a transcript that held only a
    /// head. A transcript with no head at all is not an error here — rule 12.3's digest comparison
    /// is what refuses it, and it refuses it naming the prefix the run needed.
    fn read_head(&mut self) -> io::Result<Option<String>> {
        loop {
            let Some(line) = self.next_content_line()? else {
                return Ok(None);
            };
            if transcript_string(&line, "transcript").as_deref() != Some("prefix") {
                return Ok(Some(line));
            }
            let actor = transcript_string(&line, "actor");
            let digest = transcript_string(&line, "digest");
            if let (Some(actor), Some(digest)) = (actor, digest) {
                self.prefixes.push((actor, digest));
            }
        }
    }

    /// The next exchange record, reading the head first if it has not been read.
    fn next_exchange(&mut self) -> io::Result<Option<String>> {
        if !self.head_read {
            self.head_read = true;
            self.pending = self.read_head()?;
        }
        match self.pending.take() {
            Some(record) => Ok(Some(record)),
            None => self.next_content_line(),
        }
    }

    /// Latches a failure and returns the empty proposal the engine expects from
    /// [`Proposer::propose`]. The reason reaches the run through [`Proposer::record`].
    ///
    /// [`Proposal::nothing`] and not the recorded evidence: the record whose evidence it would be is
    /// the one this port has just refused, and re-authoring its response under a failed replay would
    /// put a recorded exchange's own words on a record the run is about to abandon. Nothing is lost —
    /// rule 11.8 discards a replay's records, and the run ends on the [`Proposer::record`] call that
    /// immediately follows.
    fn fail(&mut self, reason: String) -> Proposal {
        if self.failure.is_none() {
            self.failure = Some(reason);
        }
        Proposal::nothing()
    }

    /// The evidence a recorded exchange carried, read back out so that the replay's re-authored
    /// record states it again.
    ///
    /// **Rule 12.6's byte-identity is what makes this necessary.** A replay authors its records
    /// through the same lines that authored the recording's, so the response and the four counts have
    /// to reach [`exchange_record`] on a replay exactly as they reached it on the recording, or every
    /// record of a live run's replay would differ from the record it replays in two fields. The
    /// response round-trips through [`unescape_transcript_text`] here and back through
    /// [`escape_transcript_text`] there, which rule 11.4.1's round-trip obligation makes exact.
    ///
    /// A field this cannot read is **absent**, and that is not a mismatch: [`Self::mismatch`] checks
    /// what rule 12.3 asks it to check, the evidence is not part of it, and a transcript recorded
    /// before a provider existed states both fields as `null` legitimately. The four counts are read
    /// unconditionally rather than only inside a `usage` object, because a record's `"usage":null`
    /// contains no count for any of the four needles to find.
    fn evidence(record: &str) -> (Option<String>, ReportedUsage) {
        (
            transcript_string(record, "response"),
            ReportedUsage {
                prompt: transcript_number(record, "prompt"),
                cached_prompt: transcript_number(record, "cached_prompt"),
                output: transcript_number(record, "output"),
                reasoning: transcript_number(record, "reasoning"),
            },
        )
    }

    /// Rule 12.3's check, in full: the tick, the acting Mokiterion, the prefix the record names,
    /// and the digest of the prefix the engine is actually sending.
    ///
    /// The digest is the half no rule spelled out, and it is the half that catches an edit to
    /// block A. Without it a transcript recorded under different shared rules would replay
    /// silently, every tick and actor matching, against prompts nobody ever sent.
    fn mismatch(&self, record: &str, request: &DecisionRequest) -> Option<String> {
        let opportunity = format!("tick {} actor {}", request.tick(), request.actor_id());

        let Some(version) = transcript_number(record, "version") else {
            return Some(format!("{opportunity}: record states no version"));
        };
        if version != u64::from(TRANSCRIPT_VERSION) {
            return Some(format!(
                "{opportunity}: record is transcript version {version}, this engine reads \
                 {TRANSCRIPT_VERSION}"
            ));
        }

        match transcript_number(record, "tick") {
            Some(tick) if tick == request.tick() => {}
            Some(tick) => {
                return Some(format!("{opportunity}: record is for tick {tick}"));
            }
            None => return Some(format!("{opportunity}: record states no tick")),
        }

        match transcript_string(record, "actor").as_deref() {
            Some(actor) if actor == request.actor_id() => {}
            Some(actor) => {
                return Some(format!("{opportunity}: record is for actor {actor}"));
            }
            None => return Some(format!("{opportunity}: record states no actor")),
        }

        match transcript_string(record, "prefix").as_deref() {
            Some(prefix) if prefix == request.actor_id() => {}
            Some(prefix) => {
                return Some(format!(
                    "{opportunity}: record names prefix {prefix} against actor {}",
                    request.actor_id()
                ));
            }
            None => return Some(format!("{opportunity}: record states no prefix")),
        }

        let [shared_rules, actor, ..] = request.blocks();
        let expected = prefix_digest(shared_rules, actor);
        match transcript_string(record, "prefix_digest") {
            Some(digest) if digest == expected => {}
            Some(digest) => {
                return Some(format!(
                    "{opportunity}: record was recorded against prefix {digest}, this run sends \
                     {expected}. The shared rules or the actor block changed since the recording, \
                     so the transcript answers prompts this run does not ask"
                ));
            }
            None => return Some(format!("{opportunity}: record states no prefix digest")),
        }

        let declared = self
            .prefixes
            .iter()
            .find(|(actor, _)| actor == request.actor_id());
        match declared {
            Some((_, digest)) if *digest == expected => {}
            Some((_, digest)) => {
                return Some(format!(
                    "{opportunity}: the head declares prefix {digest} for this actor and the \
                     record was taken against {expected}"
                ));
            }
            None => {
                return Some(format!(
                    "{opportunity}: the transcript's head declares no prefix for this actor"
                ));
            }
        }

        None
    }
}

impl<R: BufRead> Proposer for ReplayPort<R> {
    fn propose(&mut self, request: DecisionRequest) -> Proposal {
        if self.failure.is_some() {
            return Proposal::nothing();
        }

        let record = match self.next_exchange() {
            Ok(Some(record)) => record,
            // Rule 12.4: the transcript ended before the run did. The opportunity is named, the
            // run does not shorten, rule 9.5's fallback is not applied and no rule-based proposal
            // is substituted — every one of those would produce a plausible wrong run.
            Ok(None) => {
                return self.fail(format!(
                    "the transcript ended after {} exchange(s) and cannot satisfy tick {} actor \
                     {}. A replay does not shorten the run and does not fall back",
                    self.served,
                    request.tick(),
                    request.actor_id()
                ));
            }
            Err(error) => {
                return self.fail(format!("cannot read the transcript: {error}"));
            }
        };

        if let Some(reason) = self.mismatch(&record, &request) {
            return self.fail(reason);
        }
        self.served += 1;

        // The recorded exchange's own evidence, re-authored with the replayed record. It travels
        // whether or not the record was a fallback, because the recorded fallback carried a response
        // too — an ill-formed one, which is the one a reader most needs to still be there.
        let (response, usage) = Self::evidence(&record);

        // Rule 12.7: an exchange that yielded nothing in the recorded run yields nothing here, so
        // the engine takes rule 9.5's fallback again and the count moves again. A replay reproduces
        // the run that happened, contamination included.
        if transcript_flag(&record, "fallback") != Some(false) {
            return Proposal {
                action: None,
                response,
                usage,
                // A replay attempts nothing: it reads one record per opportunity, so there is no
                // earlier attempt for it to carry even where the recorded run made several. See
                // the note on [`Proposal::earlier_attempts`] — a transcript that holds a retried
                // exchange is not replayable at all, and this field being empty is not what makes
                // that so.
                earlier_attempts: Vec::new(),
            };
        }

        let verb = transcript_string(&record, "verb");
        let parameter = transcript_string(&record, "parameter");
        match verb
            .as_deref()
            .and_then(|verb| action_from_parts(verb, parameter.as_deref()))
        {
            Some(action) => Proposal {
                action: Some(action),
                response,
                usage,
                earlier_attempts: Vec::new(),
            },
            None => self.fail(format!(
                "tick {} actor {}: the record states no action this engine's grammar admits",
                request.tick(),
                request.actor_id()
            )),
        }
    }

    /// Rule 11.8: a replay writes no transcript. It has one; it is reading it.
    ///
    /// The engine authors the record anyway, because rule 12.1 requires the same code path, and it
    /// is discarded here. This is also where a latched failure reaches the run, for the reason
    /// [`Proposer::record`] gives: a mismatch cannot be reported from `propose`, whose `None` means
    /// rule 9.5's ordinary fallback. The failure is not cleared, so a host that ignores the error
    /// and advances again meets it again on the first exchange of the next tick.
    fn record(&mut self, _record: &str) -> io::Result<()> {
        match &self.failure {
            Some(reason) => Err(io::Error::new(io::ErrorKind::InvalidData, reason.clone())),
            None => Ok(()),
        }
    }
}

// ---------------------------------------------------------------------------------------
// `SPEC-MOK-007` rule 10: the connector's line protocol, and the port that speaks it.
//
// **The library speaks the protocol and does not start the program that answers it.** Rule 10.1
// makes the connector "an operator-named executable spawned as a child", and `SPEC-MOK-006` rule
// 1.2 puts every path resolution and every process in the binary target — so what lives here is
// the composition of a request, the reading of one response line and the grammar check, over two
// already-connected streams a host owns. Nothing below opens, spawns, resolves or reads an
// environment variable, which is `WO-MOK-026` stop-and-escalate condition 2 met rather than
// escalated.
//
// **The connector's output is untrusted in whole**, rule 10.7, and that governs every line of the
// reader below: a response is parsed without panicking on any input, without recursing without a
// bound, and without a single figure being believed further than rule 10.8 admits. Rule 10.8 states
// the limit plainly — the ceiling protects against an honest connector and not a dishonest one —
// so the reader's job is to be total over its input, not to be safe against a liar.
// ---------------------------------------------------------------------------------------

/// The protocol version this engine speaks and the only one it reads.
///
/// On every request, and required on every response: a response stating another version fails the
/// grammar check and becomes rule 9.5's counted fallback, exactly as an unknown verb does. The
/// integer is on the wire in both directions rather than agreed once at start-up, because the
/// connector is a program this repository did not build and a version negotiated out of band is a
/// version nobody can check.
const CONNECTOR_PROTOCOL: u64 = 1;

/// Rule 19.5's bound: how many times a failed exchange is tried **again**, so an exchange is
/// attempted at most four times.
///
/// The number is the rule's, as amended 2026-08-29 under `WO-MOK-030`, and not this module's — the
/// rule gave none until then, and it records why three: an exchange costs an **estimated** $0.0001,
/// so every exchange of a two-hundred-exchange run retrying to exhaustion is an **estimated** four
/// cents. It sits here as a retry count rather than as an attempt count because that is the figure
/// the rule states, and a constant one greater than the rule's would be an off-by-one nobody could
/// see by reading either.
const RETRY_BOUND: usize = 3;

/// The eleven verbs, as the request's `schema` enumerates them for the provider's structured-output
/// facility.
///
/// Rule 8.2's closed grammar, in the order `docs/CONNECTOR_PROTOCOL.md` lists it. It is a second
/// statement of the verb set — [`action_from_parts`] is the first — and it is held equal to that one
/// by a test rather than by care, because the two disagreeing would mean the schema offered a
/// provider a verb this engine then refused, which is a paid exchange spent on a counted fallback.
const RESPONSE_VERBS: [&str; 11] = [
    "wait",
    "sleep",
    "eat",
    "move",
    "attack",
    "fight",
    "threaten",
    "retreat",
    "surrender",
    "approach",
    "avoid",
];

/// A [`Proposer`] that answers from a connector the host spawned, `SPEC-MOK-007` rules 10 and 13.
///
/// **It holds streams and not a process.** `R` is the child's standard output and `W` its standard
/// input, both already connected by the host that spawned it; the transcript is the sink that host
/// opened. This type never learns the connector's path, never learns whether a credential exists,
/// and cannot start, signal or reap anything — rule 13.4's "neither host reads the credential and
/// the library target reaches it by no route", stated as a type that has no route to reach it by.
///
/// **The two gates are not checked here and that is rule 13.1.** The live selection is the host's
/// condition and the credential is the connector's, and "neither component can satisfy the other's
/// condition". A gate this type checked would be a gate one component could pass alone. What this
/// type does is speak to whatever the host connected: with no live selection the host spawns nothing
/// and builds none of these (rule 13.2), and with no credential the connector answers rule 13.3's
/// error on the first exchange and this type records it as the error it is.
///
/// It is built once and lent per tick, rule 20.4.1. The reason is sharper here than for
/// [`ReplayPort`]: a port rebuilt each tick would hand the child a request on a stream nobody was
/// reading, and it would reset the accumulated cost the ceiling of rule 14.6 is checked against — a
/// run that never stops spending. That second half is no longer hypothetical: the account is a field
/// of this type, so a rebuild is a reset in the literal sense the rule describes, which is what case
/// **L30** measures.
///
/// **One attempt per exchange, in this stage.** Rule 19.5's retry bound is `WO-MOK-026`'s item 12
/// and is not here yet, so a `transport` or `provider` error becomes a counted fallback where rule
/// 19.5a asks for it to be retried first. `malformed` and `refused` are already correct — rule 19.5a
/// makes both immediate counted fallbacks — and every kind's record is written either way, so the
/// evidence a retry would add to is being kept from the first exchange.
pub struct ConnectorPort<'sink, R: BufRead, W: Write> {
    /// The child's standard output: one response object per line.
    responses: R,
    /// The child's standard input: one request object per line.
    requests: W,
    /// The transcript the host opened. Rule 11.1's destination, owned by the host and written
    /// through here because [`Proposer::record`] is the seam the engine hands a finished record to.
    transcript: &'sink mut dyn Write,
    /// Rule 8.4's response schema, built once in [`ConnectorPort::new`].
    ///
    /// Once and not per request, and the reason is rule 3.3 rather than efficiency: the schema is
    /// not part of the prompt, but a schema rebuilt per exchange is a value that *could* vary within
    /// a run, and a request field that varies for no reason is a field a later reader has to prove
    /// constant.
    schema: String,
    /// Rules 14.1 and 9.5's accumulators, in the party rule 20.4.1 puts them in.
    ///
    /// It is `accounting::RunAccount` and therefore no part of this type's public surface: the field
    /// is private and the type is private to `simulation`, which is why `SPEC-MOK-002` rule 5's
    /// census for this port is one item and two declarations with no field among them. The engine
    /// never reads it — rule 14's *State model* says none of it may influence a decision, and a
    /// figure the engine cannot see cannot.
    account: accounting::RunAccount,
}

impl<'sink, R: BufRead, W: Write> ConnectorPort<'sink, R, W> {
    /// Wraps the child's two streams and the host's transcript, and takes the run's declared prices
    /// and ceiling. Nothing is written and nothing is read: a run refused before its first tick must
    /// not have spoken to the connector, and rule 20.8's refusal happens after construction.
    ///
    /// **The prices and the ceiling arrive here rather than being read here**, rule 14.3: they are
    /// inputs of the run, and this type has no route to an environment variable, a file or a
    /// constant holding either. The ceiling is in whole US cents, rule 14.2, which is what
    /// `--spend-ceiling` parses to; `None` is a run with none declared, which for a live run rule
    /// 13.5 forbids and which this type does not police because rule 13.1 leaves the live selection's
    /// conditions to the host.
    ///
    /// They are parameters of `new` and not of `propose` because rule 20.4.1 builds this once and
    /// lends it per tick. Prices arriving per exchange could differ between exchanges of one run,
    /// which is a cost figure no reader could reconstruct from the record.
    pub fn new(
        responses: R,
        requests: W,
        transcript: &'sink mut dyn Write,
        prices: UnitPrices,
        ceiling: Option<u64>,
    ) -> Self {
        Self {
            responses,
            requests,
            transcript,
            schema: response_schema(),
            account: accounting::RunAccount::declared(
                accounting::PerTokenPrices::declared(&prices),
                ceiling,
            ),
        }
    }

    /// One request object, on one line, in the field order `docs/CONNECTOR_PROTOCOL.md` documents.
    ///
    /// **The prompt is [`DecisionRequest::blocks`] concatenated and nothing else.** Rule 3.6 refuses
    /// an implementation that "composes the parts in this order and then serialises them through a
    /// structure whose field order is not guaranteed", and this is that rule discharged: the order
    /// comes from the one function that states it, the four blocks are joined with no separator of
    /// this module's invention, and rule 3.3's byte-identical block A is byte-identical here because
    /// [`DecisionRequest`] holds it as a `&'static str`.
    ///
    /// **The request names no model, no reasoning level and no credential**, rules 10.3a and 10.3.
    /// There are five fields and the type has no field to put a sixth in.
    ///
    /// The escaping is [`escape_transcript_text`]'s, which is a JSON string escaper and is reused
    /// rather than reimplemented: a second escaper would be a second place for the framing of rule
    /// 10.2 — "no newline within an object" — to be got wrong, and the blocks are full of newlines.
    fn request_line(&self, request: &DecisionRequest) -> String {
        format!(
            "{{\"protocol\":{CONNECTOR_PROTOCOL},\"tick\":{},\"actor\":\"{}\",\"prompt\":\"{}\",\
             \"schema\":{}}}",
            request.tick(),
            escape_transcript_text(request.actor_id()),
            escape_transcript_text(&request.blocks().concat()),
            self.schema,
        )
    }

    /// Writes one request and reads one response, rule 10.2's one-for-one ordering.
    ///
    /// Flushed before the read, because the child cannot answer a request sitting in this process's
    /// buffer and the read would then block until one of the two gave up.
    ///
    /// End of input is an error and not an empty response: rule 10.2 fixes exactly one response per
    /// request, so a connector that closed its output has stopped answering rather than answered
    /// nothing. The canned connector's `close` directive is that case on purpose.
    fn exchange(&mut self, request: &DecisionRequest) -> io::Result<String> {
        let line = self.request_line(request);
        self.requests.write_all(line.as_bytes())?;
        self.requests.write_all(b"\n")?;
        self.requests.flush()?;

        let mut response = String::new();
        if self.responses.read_line(&mut response)? == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "the connector closed its output without answering",
            ));
        }
        Ok(response.trim_end_matches(['\n', '\r']).to_string())
    }

    /// One response line to one proposal, with the line itself as the evidence either way.
    ///
    /// **The response travels whole and unconditionally**, rule 11.3's "the response as received, in
    /// full, or the error". The ill-formed line is the one a reader most needs, so it is recorded
    /// exactly as it arrived — not summarised, not truncated and not re-serialised from whatever this
    /// function managed to read out of it.
    ///
    /// **The counts travel even when the action does not.** An exchange whose response failed the
    /// grammar check was still billed, so rule 14's arithmetic must see its usage and rule 11.3's
    /// record must state it. The two halves are therefore read independently: [`Self::usage`] does
    /// not consult the action and [`Self::action`] does not consult the counts beyond rule 10.4a's
    /// requirement that a `usage` accompany an action at all.
    /// The second half of the return is rule 19.5a's question: whether this response asks for
    /// another attempt. It is answered here rather than by a second parse of the same line, and the
    /// answer is about *this* response only — the bound belongs to [`Self::propose`], which is the
    /// only place that knows how many attempts an opportunity has already spent.
    fn interpret(line: &str) -> (Proposal, bool) {
        let response = Some(line.to_string());
        let Some(parsed) = json::parse(line) else {
            // Rule 19.5a's `malformed`: an immediate counted fallback, and the line is the whole of
            // why. Not a parse *error* to report — the engine does not interpret a port's diagnosis,
            // and a reader of the transcript has the bytes this reader refused. **Not retried**: a
            // line this reader could not parse would be answered by the same request, and rule 19.5a
            // says so of the kind a connector reports as `malformed` for the same reason.
            return (
                Proposal {
                    action: None,
                    response,
                    usage: ReportedUsage::default(),
                    earlier_attempts: Vec::new(),
                },
                false,
            );
        };
        let retry = Self::asks_for_another_attempt(&parsed);
        (
            Proposal {
                action: Self::action(&parsed),
                response,
                usage: Self::usage(&parsed),
                earlier_attempts: Vec::new(),
            },
            retry,
        )
    }

    /// Rule 19.5a's vocabulary, as far as this engine acts on it: `transport` and `provider` are
    /// retried, `malformed` and `refused` are not.
    ///
    /// The two that are retried are the two the rule calls ordinarily transient — the connector could
    /// not reach the provider, or the provider answered and failed, "a rate limit and a server error
    /// being this shape". The two that are not are the two where the same request would produce the
    /// same answer, so a retry would buy nothing and cost an exchange.
    ///
    /// **A kind outside the four is not retried**, which is rule 19.5a's own instruction to treat it
    /// as `malformed`: the connector is untrusted under rule 10.7 "and that includes its error
    /// vocabulary", so an unrecognised kind must not be able to make this engine spend four times.
    ///
    /// **A response that carries an action is never retried whatever else it carries.** Rule 10.4
    /// admits exactly one of `action` and `error`, so a response carrying both fails the grammar
    /// check in [`Self::action`] and becomes a fallback; retrying it would be this engine deciding
    /// which of the two the connector meant, on the strength of the half it is told to distrust.
    fn asks_for_another_attempt(response: &json::Value) -> bool {
        if response.member("action").is_some() {
            return false;
        }
        matches!(
            response
                .member("error")
                .and_then(|error| error.member("kind"))
                .and_then(json::Value::text),
            Some("transport" | "provider")
        )
    }

    /// Rule 10.4a's four counts, as reported, and rule 11.5's absence for everything else.
    ///
    /// A count that is missing, is not a number, is negative, carries a fraction or carries an
    /// exponent is **absent**. Rule 11.5 fixes that an unreported count is absent and not zero, and
    /// rule 10.8 is why the same treatment covers a count that is present and unusable: this figure
    /// comes from a program rule 10.7 declares untrusted, and a `-1` coerced to `0` would be a
    /// cost of zero for an exchange that was billed. Rule 14.5 is what acts on the absence — an
    /// uncomputable ratio is a failure to evaluate, which is exactly the outcome wanted.
    ///
    /// A `usage` that is present and is not an object yields four absences rather than a rejection,
    /// because [`Self::action`] already requires the member to be present and the two checks are
    /// deliberately independent.
    fn usage(response: &json::Value) -> ReportedUsage {
        let Some(usage) = response.member("usage") else {
            return ReportedUsage::default();
        };
        ReportedUsage {
            prompt: usage.member("prompt").and_then(json::Value::count),
            cached_prompt: usage.member("cached_prompt").and_then(json::Value::count),
            output: usage.member("output").and_then(json::Value::count),
            reasoning: usage.member("reasoning").and_then(json::Value::count),
        }
    }

    /// Rule 10.4's grammar check, in full. `None` is rule 9.5's counted fallback and carries no
    /// diagnosis, per rule 1.4.
    ///
    /// The checks are in the order a reader of rule 10.4 meets them, and each one's absence would be
    /// a different silent wrong run:
    ///
    /// - **The protocol version.** A connector written against a later protocol may mean something
    ///   else by every field below, so a version this engine does not read is not a response to
    ///   interpret generously.
    /// - **Exactly one of `action` and `error`.** Rule 10.4's own words. A response carrying both is
    ///   two answers, and picking either would be this engine deciding which one the connector meant.
    /// - **`model` and `reasoning` accompany the action**, rule 10.4c: "a response reporting neither
    ///   fails the grammar check and becomes a counted fallback". An empty `model` is not a name, so
    ///   it fails here too — a run record naming the empty string would claim to name what answered.
    /// - **A `usage` member is present.** Rule 10.4a puts it on every response that carries an
    ///   action, so an action arriving without one is not the documented response. Its *contents* are
    ///   [`Self::usage`]'s and no count is required, which is rule 11.5 and not a relaxation of this.
    /// - **The verb and its parameter**, through [`action_from_parts`], which is the same function
    ///   [`ReplayPort`] parses a transcript with. One grammar, one implementation: a connector's
    ///   answer and a recorded answer are admitted by the same eleven cases or by neither.
    fn action(response: &json::Value) -> Option<Action> {
        if response.member("protocol").and_then(json::Value::count) != Some(CONNECTOR_PROTOCOL) {
            return None;
        }
        let action = response.member("action")?;
        if response.member("error").is_some() {
            return None;
        }
        if response
            .member("model")
            .and_then(json::Value::text)
            .is_none_or(str::is_empty)
        {
            return None;
        }
        response.member("reasoning").and_then(json::Value::text)?;
        response.member("usage")?;

        let verb = action.member("verb").and_then(json::Value::text)?;
        // A `parameter` that is present and is not a string is a defect and not an absence: the
        // difference between `avoid M07` and `avoid` is the difference between two admitted actions,
        // so a value this reader cannot read must not become the one without a parameter.
        let parameter = match action.member("parameter") {
            Some(value) => Some(value.text()?),
            None => None,
        };
        action_from_parts(verb, parameter)
    }

    /// One exchange, billed: rules 14.1 and 11.2, where an attempt is what gets billed and recorded.
    fn attempt(&mut self, request: &DecisionRequest) -> (Proposal, bool) {
        let (proposal, asks_for_another) = match self.exchange(request) {
            Ok(line) => Self::interpret(&line),
            // A pipe that failed is the connector's error, so it is recorded as one: rule 11.3
            // asks for "the response as received, in full, **or the error**", and this is the
            // error. It becomes rule 9.5's counted fallback rather than ending the run, per rule
            // 9.8 — and once the child is gone every later exchange reaches here too, so a run
            // whose connector died reports itself as a run of fallbacks rather than as a run.
            //
            // Rule 19.7 is kept by the message: an `io::Error` from a pipe names no credential and
            // no path, and this target resolved neither.
            //
            // **Not retried, and the pipe is why.** Rule 19.5a's four kinds are kinds a connector
            // *reports*, and a connector that cannot be written to or read from has reported nothing:
            // the child is gone, so three more attempts would write three more identical records for
            // one dead process. The canned connector's `close` directive is exactly this case. Which
            // transport failures are retried is this work order's envelope, and this is the line it
            // decides.
            Err(error) => (
                Proposal {
                    action: None,
                    response: Some(format!("connector: {error}")),
                    usage: ReportedUsage::default(),
                    earlier_attempts: Vec::new(),
                },
                false,
            ),
        };

        // Rule 14.1, booked here because this is where the exchange happened and rule 20.4.1 puts the
        // accumulators in this type.
        //
        // **Every exchange is billed, whatever it returned.** Rule 10.4c's response arrived, carried
        // usage, and failed the grammar check on a missing model or reasoning level — `Self::interpret`
        // returns no action and the usage it reported, and that exchange was paid for. Billing it as an
        // absence would understate the run. The exchanges that genuinely reported nothing — the pipe
        // failure above, rule 10.4's error response and every abandoned attempt — reach
        // `after_exchange` with four absences, and rule 11.5 makes an absence contribute zero, so the
        // one statement is right for all of them.
        //
        // Rule 9.5's count is **not** here, and rule 19.5 is why: an attempt that fails and is retried
        // is not a fallback, so the count belongs to the opportunity and is taken in `propose` once the
        // retrying has ended. A rejection the engine decides afterwards cannot reach either place,
        // which is rule 9.6: the port sees what the connector answered and never learns what the world
        // did with it.
        self.account = self.account.after_exchange(&proposal.usage);
        (proposal, asks_for_another)
    }
}

impl<R: BufRead, W: Write> Proposer for ConnectorPort<'_, R, W> {
    /// Rule 14.6: whether the accumulated cost has reached the declared ceiling.
    ///
    /// The whole of the answer is [`accounting::RunAccount::ceiling_reached`], compared in the unit
    /// the cost is accumulated in rather than in the cent the ceiling was declared in, so a run whose
    /// spending has passed a one-cent ceiling by a fraction stops on the fraction. A port built with
    /// no ceiling answers `false` for ever, which is the same run rule 13.5 forbids the host to start
    /// and this type does not police.
    ///
    /// Nothing is written and nothing is read: asking must not speak to the child. That is what lets
    /// the engine ask before every opportunity, including the one where the answer is `true` and the
    /// child is left with an empty input queue and a live process the host will close.
    fn halted(&self) -> bool {
        self.account.ceiling_reached()
    }

    /// Rule 19.5's bounded retry, and rule 11.2's record for every attempt of it.
    ///
    /// One attempt is the whole of this method for every exchange that succeeds and for every failure
    /// rule 19.5a does not retry, which is the case the loop is written to cost nothing:
    /// `earlier_attempts` stays an empty `Vec`, which allocates nothing.
    ///
    /// **Three things end the retrying, and they are checked in this order**: the bound, the kind, and
    /// the ceiling.
    ///
    /// - [`RETRY_BOUND`] attempts have already been abandoned, so this exchange has been attempted
    ///   four times and rule 19.5's bound is spent. Rule 9.5 then applies to the opportunity, which
    ///   is the `counting_fallback` below and **not** an error: rule 19.5 says the run continues, and
    ///   `VER-MOK-018` case **R2** checks that it does.
    /// - The response is not one rule 19.5a retries — it succeeded, or it is `malformed`, `refused`,
    ///   an unrecognised kind, or a pipe that failed.
    /// - The ceiling has been reached. **This is rule 14.6's check standing before each attempt**, and
    ///   it is the half of it that the engine cannot make: the engine asks [`Proposer::halted`] once
    ///   per opportunity, before this method is entered, and every attempt after the first is inside
    ///   it. Without this line a bound of three could spend three exchanges past a ceiling, which is
    ///   the reading rule 19.5 forecloses when it says "rule 14.6's check runs before each attempt, so
    ///   the bound cannot breach a ceiling whatever it is set to".
    ///
    /// **There is no delay between attempts, and that is a decision this work order's envelope leaves
    /// here rather than an omission.** The envelope delegates "the retry count, the backoff shape and
    /// which transport failures are retried"; the count is rule 19.5's own and is
    /// [`RETRY_BOUND`], and the shape is none. A delay would leave no trace in the evidence — rule
    /// 11.4 admits no timestamp in a transcript, so nothing this repository retains could show that a
    /// backoff happened or how long it was — and case **R5** forbids making any wall-clock figure a
    /// pass condition, so a backoff would be behaviour no verification case could check. It would also
    /// stretch a run's wall clock by an amount no rule bounds, where rule 19.5 bounds attempts. A
    /// successor that wants one needs a rule fixing it, because it changes what a run costs in time.
    fn propose(&mut self, request: DecisionRequest) -> Proposal {
        let mut earlier_attempts = Vec::new();
        let mut proposal = loop {
            let (proposal, asks_for_another) = self.attempt(&request);
            if earlier_attempts.len() == RETRY_BOUND
                || !asks_for_another
                || self.account.ceiling_reached()
            {
                break proposal;
            }
            earlier_attempts.push(proposal);
        };
        proposal.earlier_attempts = earlier_attempts;

        // Rule 9.5, counted once for the opportunity and not once per attempt. The billing happened
        // in `attempt`, per exchange, because that is what rule 11.2 calls each attempt; this is the
        // *decision*, and an opportunity has one however many exchanges it took.
        if proposal.action.is_none() {
            self.account = self.account.counting_fallback();
        }
        proposal
    }

    /// Rule 11.2's one line per exchange, appended to the host's transcript and flushed.
    ///
    /// **Flushed on every record, and that is not caution.** A live run's records are evidence that
    /// was paid for, and a buffer holding the last few exchanges of a run that then died would lose
    /// exactly the records nobody can regenerate without spending again. Rule 19.6 makes a failure
    /// here end the run, which is the other half of the same reasoning: an exchange spent and not
    /// recorded has produced cost and no evidence.
    ///
    /// The newline is written as one byte, on every platform. `SPEC-MOK-004`'s retained evidence is
    /// `-text` and a transcript is compared byte for byte under rule 12.6, so a record terminated
    /// differently on Windows would make a replay of a Windows recording fail on a line ending.
    fn record(&mut self, record: &str) -> io::Result<()> {
        self.transcript.write_all(record.as_bytes())?;
        self.transcript.write_all(b"\n")?;
        self.transcript.flush()
    }
}

/// Rule 8.4's schema: the response `action` object, for the provider's structured-output facility.
///
/// Built rather than written out as a literal, so that [`RESPONSE_VERBS`] is the one statement of
/// the verb set on this side and a twelfth verb cannot be added to the enumeration without appearing
/// here. The shape is the one `docs/CONNECTOR_PROTOCOL.md` publishes, which rule 10.4b makes
/// normative as documented: `parameter` is not required, because seven of the eleven verbs take one
/// and four do not.
fn response_schema() -> String {
    let verbs: Vec<String> = RESPONSE_VERBS
        .iter()
        .map(|verb| format!("\"{verb}\""))
        .collect();
    format!(
        "{{\"type\":\"object\",\"required\":[\"verb\"],\"properties\":{{\"verb\":{{\"enum\":[{}]}},\
         \"parameter\":{{\"type\":\"string\"}}}}}}",
        verbs.join(",")
    )
}

/// A reader for the connector's responses: `SPEC-MOK-007` rule 10.7's untrusted input, read without
/// a crate.
///
/// **It exists because rule 10.1 forbids the alternative.** "Neither package acquires a crate" for
/// this binding, so the responses of a program written by somebody else are read by code in this
/// file. `REQ-MOK-050`, `SPEC-MOK-002` rule 13 and `ARCH-MOK-001`'s conformance check all rest on
/// that, and this module is what it costs.
///
/// Three properties are the whole of its defence, and each is a line of code rather than a habit:
///
/// - **It is total over `&str`.** Every path returns `Option`, nothing indexes without a bound and
///   nothing unwraps. A response is a line from a child process, so a panic here would be a run
///   ended by the connector's choice of bytes.
/// - **Nesting is bounded** by [`MAX_DEPTH`]. A recursive-descent reader with no bound is a stack
///   overflow one line of `[[[[…` away, and a stack overflow is not a `Result`.
/// - **No floating-point type appears.** A number is held as the text it arrived as and is converted
///   only where an unsigned integer is what a field means, so `1e999`, `1.5` and `-1` are each a
///   count this engine does not have rather than a count it invented. `SPEC-MOK-007` rule 14.2's
///   integer arithmetic and case **P6**'s claim about the accounting both depend on no `f64` ever
///   holding a figure that reaches them.
///
/// It reads what rule 10.4's responses are made of and no more: objects, arrays, strings, numbers
/// and the three literals. It is not a general JSON library and is not offered as one — it is
/// private, and the one caller is [`ConnectorPort`].
mod json {
    /// The nesting this reader admits, counted in containers.
    ///
    /// Rule 10.4's response nests two deep — the object, then `action` or `usage`. Sixteen is far
    /// past anything the protocol describes and far short of anything that troubles a stack, which
    /// is the point: the bound exists to make a hostile line a refused line, not to be a limit a
    /// conforming connector could meet.
    const MAX_DEPTH: u32 = 16;

    /// One JSON value, in the shapes rule 10.4's responses are made of.
    ///
    /// `Number` holds the text it arrived as, for the reason the module comment gives: this crate
    /// holds no floating-point value, and a number's meaning is decided by the field it is read for
    /// rather than by this type.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(super) enum Value {
        Null,
        Bool(bool),
        Number(String),
        Text(String),
        List(Vec<Value>),
        /// Members in the order they arrived, with duplicates kept. [`Value::member`] returns the
        /// first, which is a choice and is stated: JSON fixes no meaning for a repeated key, so a
        /// reader that took the last would let a connector append an override after a value the
        /// operator's provider produced.
        Map(Vec<(String, Value)>),
    }

    impl Value {
        /// One member of an object, or `None` for an absent member and for every non-object.
        ///
        /// The two cases are deliberately one: every caller in [`super::ConnectorPort`] is asking
        /// "is this field here and usable", and a `usage` that arrived as a string is a field that
        /// is not usable in exactly the way an absent one is not.
        pub(super) fn member(&self, name: &str) -> Option<&Value> {
            match self {
                Self::Map(members) => members
                    .iter()
                    .find(|(key, _)| key == name)
                    .map(|(_, value)| value),
                _ => None,
            }
        }

        /// The value as a string, and `None` for every other shape.
        pub(super) fn text(&self) -> Option<&str> {
            match self {
                Self::Text(text) => Some(text.as_str()),
                _ => None,
            }
        }

        /// The value as an unsigned integer count, and `None` for anything else at all.
        ///
        /// A run of ASCII digits and nothing else: no sign, no fraction, no exponent, and no
        /// leading `+`. `u64::from_str` refuses the rest, and where it refuses the count is
        /// **absent** under rule 11.5 rather than coerced — a negative count, a fractional count and
        /// `1e999` are three things a provider did not report.
        pub(super) fn count(&self) -> Option<u64> {
            match self {
                Self::Number(text) => text.parse().ok(),
                _ => None,
            }
        }
    }

    /// One line to one value, or `None` for anything this reader will not read.
    ///
    /// Trailing content after the value is a refusal and not something to ignore: rule 10.2 puts one
    /// object on a line, so a line carrying an object and then more bytes is not that.
    pub(super) fn parse(line: &str) -> Option<Value> {
        let mut reader = Reader {
            characters: line.chars().collect(),
            at: 0,
        };
        let value = reader.value(0)?;
        reader.skip_whitespace();
        if reader.at != reader.characters.len() {
            return None;
        }
        Some(value)
    }

    /// A cursor over the line's characters.
    ///
    /// Characters and not bytes, so that a multi-byte character in a message can never be sliced
    /// through the middle. A response line is a few hundred characters, so the collection costs
    /// nothing worth a byte-index reader's care.
    struct Reader {
        characters: Vec<char>,
        at: usize,
    }

    impl Reader {
        fn peek(&self) -> Option<char> {
            self.characters.get(self.at).copied()
        }

        fn take(&mut self) -> Option<char> {
            let character = self.peek()?;
            self.at += 1;
            Some(character)
        }

        /// JSON's four whitespace characters and no others. A vertical tab or a form feed between
        /// tokens is not whitespace here, because it is not whitespace in the grammar.
        fn skip_whitespace(&mut self) {
            while matches!(self.peek(), Some(' ' | '\t' | '\n' | '\r')) {
                self.at += 1;
            }
        }

        fn expect(&mut self, expected: char) -> Option<()> {
            (self.take()? == expected).then_some(())
        }

        fn literal(&mut self, text: &str) -> Option<()> {
            for expected in text.chars() {
                self.expect(expected)?;
            }
            Some(())
        }

        fn value(&mut self, depth: u32) -> Option<Value> {
            if depth > MAX_DEPTH {
                return None;
            }
            self.skip_whitespace();
            match self.peek()? {
                '{' => self.map(depth),
                '[' => self.list(depth),
                '"' => self.text().map(Value::Text),
                't' => self.literal("true").map(|()| Value::Bool(true)),
                'f' => self.literal("false").map(|()| Value::Bool(false)),
                'n' => self.literal("null").map(|()| Value::Null),
                '-' | '0'..='9' => self.number(),
                _ => None,
            }
        }

        fn map(&mut self, depth: u32) -> Option<Value> {
            self.expect('{')?;
            let mut members = Vec::new();
            self.skip_whitespace();
            if self.peek()? == '}' {
                self.at += 1;
                return Some(Value::Map(members));
            }
            loop {
                self.skip_whitespace();
                let key = self.text()?;
                self.skip_whitespace();
                self.expect(':')?;
                members.push((key, self.value(depth + 1)?));
                self.skip_whitespace();
                match self.take()? {
                    ',' => continue,
                    '}' => return Some(Value::Map(members)),
                    _ => return None,
                }
            }
        }

        fn list(&mut self, depth: u32) -> Option<Value> {
            self.expect('[')?;
            let mut entries = Vec::new();
            self.skip_whitespace();
            if self.peek()? == ']' {
                self.at += 1;
                return Some(Value::List(entries));
            }
            loop {
                entries.push(self.value(depth + 1)?);
                self.skip_whitespace();
                match self.take()? {
                    ',' => continue,
                    ']' => return Some(Value::List(entries)),
                    _ => return None,
                }
            }
        }

        /// A string, with JSON's escapes and with a surrogate pair joined into the character it
        /// denotes.
        ///
        /// A lone surrogate is refused rather than replaced. `char::from_u32` cannot hold one, and
        /// the two alternatives are a replacement character — which would silently change a message
        /// — and a panic, which rule 10.7 makes unavailable. A response carrying one is a response
        /// this reader does not read, and rule 9.5's fallback is what follows.
        fn text(&mut self) -> Option<String> {
            self.expect('"')?;
            let mut text = String::new();
            loop {
                match self.take()? {
                    '"' => return Some(text),
                    '\\' => match self.take()? {
                        '"' => text.push('"'),
                        '\\' => text.push('\\'),
                        '/' => text.push('/'),
                        'b' => text.push('\u{8}'),
                        'f' => text.push('\u{c}'),
                        'n' => text.push('\n'),
                        'r' => text.push('\r'),
                        't' => text.push('\t'),
                        'u' => text.push(self.escaped_character()?),
                        _ => return None,
                    },
                    // Rule 10.2: no object contains a raw newline, and a control character inside a
                    // string is outside JSON's grammar besides. Refused rather than accepted, so a
                    // response cannot smuggle a line break past the framing.
                    control if control.is_control() => return None,
                    character => text.push(character),
                }
            }
        }

        /// One `\u` escape, and the second half of a surrogate pair where the first half needs one.
        fn escaped_character(&mut self) -> Option<char> {
            let first = self.hex_quad()?;
            // The high half of a surrogate pair. The low half must follow, spelled as its own
            // escape, which is how JSON carries a character outside the basic multilingual plane.
            if (0xd800..0xdc00).contains(&first) {
                self.expect('\\')?;
                self.expect('u')?;
                let second = self.hex_quad()?;
                if !(0xdc00..0xe000).contains(&second) {
                    return None;
                }
                let combined = 0x1_0000 + ((first - 0xd800) << 10) + (second - 0xdc00);
                return char::from_u32(combined);
            }
            char::from_u32(first)
        }

        fn hex_quad(&mut self) -> Option<u32> {
            let mut value = 0u32;
            for _ in 0..4 {
                value = value * 16 + self.take()?.to_digit(16)?;
            }
            Some(value)
        }

        /// A number, kept as the text it arrived as and validated against JSON's grammar.
        ///
        /// Validated rather than merely collected, so that `count` refusing a value means "this is
        /// not an unsigned integer" and never "this reader stopped early". `01`, `1.`, `.5`, `1e`
        /// and a bare `-` are each refused here, where refusing the whole line is the right answer,
        /// rather than becoming a truncated value in a field.
        fn number(&mut self) -> Option<Value> {
            let start = self.at;
            if self.peek() == Some('-') {
                self.at += 1;
            }
            match self.take()? {
                // A leading zero admits no further digit, which is JSON's grammar and not a
                // strictness of this reader's.
                '0' => {}
                '1'..='9' => self.skip_digits(),
                _ => return None,
            }
            if self.peek() == Some('.') {
                self.at += 1;
                self.digits()?;
            }
            if matches!(self.peek(), Some('e' | 'E')) {
                self.at += 1;
                if matches!(self.peek(), Some('+' | '-')) {
                    self.at += 1;
                }
                self.digits()?;
            }
            Some(Value::Number(
                self.characters[start..self.at].iter().collect(),
            ))
        }

        /// At least one digit, for the two places JSON's grammar requires one.
        fn digits(&mut self) -> Option<()> {
            if !self.peek()?.is_ascii_digit() {
                return None;
            }
            self.skip_digits();
            Some(())
        }

        fn skip_digits(&mut self) {
            while self
                .peek()
                .is_some_and(|character| character.is_ascii_digit())
            {
                self.at += 1;
            }
        }
    }
}

/// The private adapter of `SPEC-MOK-007` rule 20.6: it implements the engine's own decision-source
/// abstraction in terms of the public port.
///
/// It exists so that the abstraction stays private. That abstraction takes [`Observation`],
/// which carries `ADR-MOK-001`'s trust boundary, and publishing it in order to reach this source
/// would export the boundary itself. `SPEC-MOK-002` rule 6 keeps both names on its prohibited
/// list, and this type is the whole of what stands between them and [`Proposer`].
///
/// **It draws no entropy**, rule 20.7. It receives the handle the abstraction passes every
/// source and never touches it, so `REQ-MOK-009`'s stream does not move. One draw here would
/// shift every subsequent draw in the run and the four existing sources would then behave
/// differently at the same seed, which is exactly what `REQ-MOK-068`'s byte-identity comparison
/// exists to catch.
///
/// It also **authors every transcript record**, which is rule 11.1's other half. The port owns the
/// destination and this owns the content: the framing, the field set, the escaping and the version
/// are all decided here, once, for both hosts and for a live run and a replay alike. That is what
/// makes rule 12.6's byte-identity structural — a replay's records are authored by the same lines
/// that authored the recording's, so there is no second writer to hold in agreement with the first.
struct PortDecisionSource<'port> {
    port: &'port mut dyn Proposer,
    /// The port's failure, latched from the decision that met it. See [`DecisionSource::failure`].
    failure: Option<io::Error>,
}

impl<'port> PortDecisionSource<'port> {
    fn new(port: &'port mut dyn Proposer) -> Self {
        Self {
            port,
            failure: None,
        }
    }

    /// Latches the first failure and keeps it. A later one is dropped: the run ends on the first,
    /// so a second is a consequence of the first rather than news, and reporting the later one
    /// would name the wrong opportunity.
    fn latch(&mut self, error: io::Error) {
        if self.failure.is_none() {
            self.failure = Some(error);
        }
    }
}

impl DecisionSource for PortDecisionSource<'_> {
    fn name(&self) -> &str {
        LLM_SOURCE_NAME
    }

    /// Rule 14.6's question, forwarded and nothing more.
    ///
    /// This adapter holds no accounting figure and computes none — rule 14's *State model* keeps every
    /// accumulator in the port, rule 20.4.1 keeps the port in the host, and an adapter that cached the
    /// answer would be a third party to a two-party fact.
    fn halted(&self) -> bool {
        self.port.halted()
    }

    /// Rule 11.1's prefix head: blocks A and B for every Mokiterion the run created, in ascending
    /// identifier order, before the first exchange.
    ///
    /// Every Mokiterion gets one whether or not it ever decides. A run can kill a Mokiterion before
    /// it acts, and a head that only declared the ones that acted would make the transcript's own
    /// contents depend on the run's outcome — so a reader could not tell a Mokiterion that never
    /// decided from a prefix somebody dropped.
    fn open(&mut self, roster: &[RosterEntry<'_>]) -> io::Result<()> {
        for entry in roster {
            self.port.record(&prefix_record(entry))?;
        }
        Ok(())
    }

    /// Rule 9.5's fallback: where no proposal was obtained the source proposes `wait`.
    ///
    /// `wait` and not a substitute from another source, per rule 9.7, and not an abort, per rule
    /// 9.8 — a run whose transport hiccupped once has real ticks and a replayable transcript.
    ///
    /// **The occurrence is recorded here and counted elsewhere.** This authors the record and its
    /// `fallback` flag; rule 9.5's count and rule 15.4's mark live in
    /// [`accounting::RunAccount`], which rule 20.4.1 puts in the port's hands. Two parties, and
    /// deliberately: case **P5** requires the run record's count to equal the number of records
    /// marked as fallbacks, and a count this method incremented in the same statement that wrote the
    /// flag would satisfy P5 by construction, leaving the property nothing to catch.
    ///
    /// What the *engine* does not decide is the **cause**. [`Proposal::action`]'s absence carries no
    /// diagnosis, so an exchange that yielded no response and a response whose action the enumeration
    /// does not admit arrive here identically — case **L22** distinguishes them and both distinctions
    /// are made on the port's side of rule 1.1. The cause reaches the transcript **with the response
    /// itself**, in the field rule 1.1a grew the port's return to carry: the engine records the two
    /// cases differently without telling them apart, which is rule 1.4a's point about what the
    /// evidence is and is not.
    ///
    /// A proposal the engine then rejects is not this case and is not a fallback, rule 9.6. The port
    /// is never told that a rejection happened, which is that rule as a call graph: there is no
    /// route by which the rejection could reach the count.
    ///
    /// One exchange, one record, in that order and unconditionally: rule 11.2's "one line per
    /// exchange" includes the exchange that yielded nothing, which the specification's own second
    /// example writes out. The record is handed over **before the proposal is applied**, so a
    /// transcript failure or a replay mismatch stops the run at an opportunity that is recorded and
    /// unresolved rather than applied and unrecorded.
    ///
    /// The request is cloned because rule 1.1 hands it to the port by value while the record is
    /// authored from it. The clone copies three owned strings; block A is a `&'static str` and is
    /// not copied, which is one of the reasons [`DecisionRequest`] holds it as one.
    fn decide(&mut self, observation: &Observation, _entropy: &mut DecisionEntropy<'_>) -> Action {
        debug_assert!(observation.is_consistent());
        let request = DecisionRequest::compose(observation);
        let proposal = self.port.propose(request.clone());
        // Rule 11.2's record for every attempt the port abandoned, in the order it made them and
        // before the one it ended with. The engine authors these as it authors every record, rule
        // 11.1, and it learns of them the only way rule 1.1 leaves open: they arrived with the
        // proposal, in the field rule 19.5's retry grew it by.
        //
        // A write failure on any of them is latched exactly as one on the record below is. The run
        // then ends under rule 19.6 with the attempts before it recorded and the opportunity's own
        // record missing, which is the honest shape: those exchanges were spent.
        for attempt in &proposal.earlier_attempts {
            if let Err(error) = self.port.record(&attempt_record(&request, attempt)) {
                self.latch(error);
            }
        }
        if let Err(error) = self.port.record(&exchange_record(&request, &proposal)) {
            self.latch(error);
        }
        // Rule 1.4a: the action and nothing else. The response and the counts went to the record
        // above and reach no decision from here — there is no route by which they could.
        proposal.action.unwrap_or(Action::Wait)
    }

    fn failure(&mut self) -> Option<io::Error> {
        self.failure.take()
    }
}

/// `SPEC-MOK-007` rules 14 and 15: what a live run accumulates, and the run record it reports.
///
/// **One module with one `#[allow(dead_code)]` on it, because nothing in this crate calls any of it
/// yet.** `WO-MOK-025`'s item 7 builds the arithmetic and exercises it against declared prices and
/// synthetic usage; there is no live run to feed it a provider's usage, because the connector, live
/// mode and the ceiling option are `WO-MOK-026`'s and this stage is expressly out of scope for all
/// three. The alternative was to have the recording path emit a run record now, and rule 15.6
/// forbids it: a replay reports no run record, [`PortDecisionSource`] cannot tell a replaying port
/// from a live one — rule 1.1 leaves `propose` returning the same `Option<Action>` either way — and
/// a record written by a party that had to guess would be an account of spending that never
/// happened. `mokiterions-tui`'s `is_empty` and `camera` carry the same attribute for the same
/// reason: an item the tests read and the product does not yet.
///
/// A module rather than an attribute per item, so the exemption is stated once and in one place —
/// and because **case P6's second half is a claim about this region**: no floating-point type
/// appears in the accounting code. Every figure below is a `u64`, every operation on one is integer,
/// and a reader can check that by reading one module instead of trusting a grep.
///
/// Nothing here is public. `WO-MOK-025`'s constraint is that the public surface grows by exactly one
/// interface and one request type, and the host that needs these types is the host that gains live
/// mode. Every operation takes `self` by value and returns a new account rather than mutating one,
/// which is also why `SPEC-MOK-002` rule 5's public-surface check cannot be disturbed by any of it:
/// there is no `&mut self` receiver here to match, in this stage or in the stage that publishes it.
#[allow(dead_code)]
mod accounting {
    use std::fmt;

    use super::{Config, LLM_SOURCE_NAME, TerminationReason, escape_transcript_text};

    /// Basis points — hundredths of a percent — in one whole. Rule 14.4's ratio is reported in
    /// them, and rule 14.5's floor of 0.85 is `8_500` of them: the coarsest unit that states that
    /// floor exactly, with no decimal separator anywhere in the figure or in its computation.
    const BASIS_POINTS: u64 = 10_000;

    /// One US cent, in the unit this module accumulates in. See [`PerTokenPrices`].
    const MICROCENTS_PER_CENT: u64 = 1_000_000;

    /// The unit prices declared for a live run, in **microcents per token** — a microcent being
    /// 10^-6 of a US cent.
    ///
    /// **Rule 14.2's stated minor unit is the US cent, and every figure this specification states
    /// is in cents**: `--spend-ceiling`'s amount, rule 15.2's accumulated cost and declared ceiling,
    /// and `SPEC-MOK-006` rule 8.9's `cost` and `ceiling`. What is held here is finer than that, and
    /// the reason is that the cent cannot carry one exchange's cost. Measured at rule 14.3a's own
    /// example prices and this stage's own exchange — `--prices 125:13:1000:0` against a
    /// 1,000-token prompt of which 900 were cached and an 8-token response — one exchange costs
    /// **0.0322 cents**, and `WO-MOK-026` item 13 puts a 200-exchange run at an estimated 2 to 3
    /// cents in total. An accumulation truncated to whole cents per exchange would therefore add
    /// zero every time: rule 14.6's ceiling would never be reached and rule 15.2's cost would be
    /// reported as `0` for a run that spent money. So the accumulation is exact and only the
    /// reported figure is a cent, which is [`RunAccount::cost_cents`].
    ///
    /// **The microcent is chosen so that no arithmetic happens at the edge.** Rule 14.3a declares
    /// prices in *cents per million tokens*, and one cent per million tokens **is** one microcent
    /// per token — the same integer, with no multiplication, no division and therefore no rounding
    /// between what the operator typed and what this module computes with. That is rule 14.3a's
    /// closing sentence discharged by the unit rather than by care: "integers rather than a decimal,
    /// so rule 14.2's arithmetic is integer arithmetic from the input onward and no rounding enters
    /// at the edge". [`PerTokenPrices::declared`] is consequently a renaming of four fields.
    ///
    /// Two units were measured and rejected. **Cents per token** rounds every published price to
    /// zero. **A price held per million tokens** pushes a division into every exchange's cost, where
    /// the truncation accumulates once per exchange and rule 14.6's ceiling would then bound the run
    /// at a figure depending on how many exchanges it happened to make.
    ///
    /// Rule 14.3: these are **inputs of the run**. No constant here holds a provider's number,
    /// because the provider's prices are the provider's to change and a price compiled into the
    /// engine would be a figure a release declared rather than a run.
    ///
    /// **The name is not [`UnitPrices`](super::UnitPrices)'s, and that is deliberate.** The public
    /// type of that name carries the operator's four declared prices in the order rule 14.3a fixes;
    /// this one carries the same four in the order the *arithmetic* reads them, where the first is
    /// the price of a prompt token the cache did **not** serve. Two types of one name in one file,
    /// differing in which of the two the first field means, is a transposition nobody would see.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub(super) struct PerTokenPrices {
        /// Per prompt token the provider did **not** serve from its cache.
        ///
        /// The uncached share and not the whole prompt count, which is a distinction rule 14.1's
        /// four totals do not draw and a cost computed from them must: a prompt token served from
        /// the cache is billed once, at the cached price, and an implementation billing the prompt
        /// total *and* the cached total would charge the cache twice while still passing rule 14.4's
        /// ratio.
        pub(super) uncached_prompt: u64,
        pub(super) cached_prompt: u64,
        pub(super) output: u64,
        pub(super) reasoning: u64,
    }

    impl PerTokenPrices {
        /// The operator's declared prices, in the order this module's arithmetic reads them.
        ///
        /// **No arithmetic, by construction of the unit.** Rule 14.3a declares cents per million
        /// tokens and this module holds microcents per token, which are the same integer, so the four
        /// values cross unchanged and no rounding can enter here. The whole of the conversion is that
        /// [`UnitPrices::prompt`](super::UnitPrices::prompt) is the price of a prompt token the cache
        /// did not serve — which rule 14.3a's ordering implies and does not say, and which the field
        /// name here says outright.
        pub(super) fn declared(prices: &super::UnitPrices) -> Self {
            Self {
                uncached_prompt: prices.prompt,
                cached_prompt: prices.cached,
                output: prices.output,
                reasoning: prices.reasoning,
            }
        }
    }

    /// One exchange's usage, as the provider reported it: [`ReportedUsage`] under the name this
    /// module's arithmetic reads it by.
    ///
    /// **An alias and not a second struct, which is the whole point.** This module declared its own
    /// four-`Option<u64>` type under `WO-MOK-025`, when the port's return could not carry a count and
    /// nothing but a test could supply one. `SPEC-MOK-007` rule 1.1a of 2026-08-29 put the four on
    /// the port's return, and two structurally identical types would then need a conversion between
    /// them — four unlabelled integers of similar magnitude copied field by field, where a transposed
    /// pair is a wrong cost figure *and* a wrong cache ratio with nothing in the type system to catch
    /// either. The alias means the four values exist once and the conversion does not exist at all.
    ///
    /// Every count is optional because rule 11.5 makes a count the provider did not report
    /// **absent, not zero**, while rule 15.3 makes a reported zero a positive statement. The two
    /// stay distinguishable only if the absence has a representation, and `Option<u64>` is it.
    pub(super) type ExchangeUsage = super::ReportedUsage;

    /// The three usages `SPEC-MOK-007` names, as named constructors.
    ///
    /// They live here rather than beside [`ReportedUsage`] because each one states a *rule's* case
    /// rather than a shape, and the rules are this module's: 14.1's billed exchange, 9.4's exchange
    /// that reported nothing, and 14.5's exchange that reported all but the cached figure. They are
    /// `pub(super)` and are therefore no part of the public surface `SPEC-MOK-002` rule 5 enumerates,
    /// which lists the type and its four fields and nothing else.
    impl ExchangeUsage {
        /// What a connector produces when the provider reported all four counts.
        pub(super) fn reported(
            prompt: u64,
            cached_prompt: u64,
            output: u64,
            reasoning: u64,
        ) -> Self {
            Self {
                prompt: Some(prompt),
                cached_prompt: Some(cached_prompt),
                output: Some(output),
                reasoning: Some(reasoning),
            }
        }

        /// What rule 9.4's exchange produces: the transport failed after the run's retries, the
        /// provider returned an error, or the provider returned nothing — so there is no usage to
        /// report. It is [`Default`] deliberately. An exchange nobody reported anything about is
        /// four absences and not four zeros.
        pub(super) fn unreported() -> Self {
            Self::default()
        }

        /// A response that arrived with its cached-prompt figure omitted. Rule 14.5's third sentence
        /// is about exactly this exchange and not about rule 9.4's: the exchange succeeded and was
        /// billed, and what is missing is one figure rather than the response.
        ///
        /// The reasoning count is a reported `0` rather than an absence, because rule 8.5 fixes the
        /// reasoning level at `none` and rule 15.3 wants that stated positively.
        pub(super) fn without_cached_figure(prompt: u64, output: u64) -> Self {
            Self {
                prompt: Some(prompt),
                cached_prompt: None,
                output: Some(output),
                reasoning: Some(0),
            }
        }
    }

    /// The three accumulators of `SPEC-MOK-007`'s *State model*, and the exchange count rule 14.5
    /// needs in order to interpret one of them, as a single value.
    ///
    /// **The port holds it.** Rule 20.4.1 is explicit — a port rebuilt for each call "resets the
    /// transcript cursor, the accumulated cost and the fallback count" — and case **L30** is the
    /// test written for that sentence, which only discriminates if the cost lives here rather than
    /// on [`Simulation`](super::Simulation), whose instance outlives every tick either way. Rule
    /// 15.6 says the same thing from the other side: the engine cannot know whether a run is live,
    /// so it cannot decide whether a run record is owed, and the party that does know is the host
    /// that built the connector-backed port and reaches this through the port it owns.
    ///
    /// Rule 9.5's count is here for the same reason and one more. Case **P5** compares it against
    /// the transcript records the *engine* authored; two figures derived independently make that a
    /// check, where a count incremented by the same statement that wrote the flag would make it a
    /// restatement that cannot fail.
    ///
    /// **A value, not a mutable accumulator.** Every operation consumes `self` and returns the
    /// account that follows it. Rule 14.6 puts the ceiling check *before* the spend, so the shape a
    /// caller needs is "has the account already reached its ceiling, and what would it be after
    /// this exchange" — two questions about two values rather than one about a thing being mutated
    /// underneath them.
    ///
    /// Nothing here is read by any rule that composes a request or interprets a response, which is
    /// the *State model*'s own sentence and the whole reason none of it can influence a decision.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub(super) struct RunAccount {
        prices: PerTokenPrices,
        /// Rule 14.6's declared ceiling, **in microcents**, absent where none was declared. A replay
        /// declares none, rule 14.8.
        ///
        /// Converted once, in [`Self::declared`], from the cents the operator wrote. Held in the
        /// accumulation's own unit rather than in cents, because the comparison rule 14.6 makes is
        /// against the accumulated cost and a comparison between two units is the one place a
        /// factor of a million goes unnoticed.
        ceiling: Option<u64>,
        /// Every exchange issued, including one that yielded nothing: rule 11.2's "a retry is its
        /// own record, because it was its own billed exchange" is the same accounting from the
        /// transcript's side. Rule 14.5's threshold of 200 counts these.
        exchanges: u64,
        prompt_tokens: u64,
        cached_prompt_tokens: u64,
        output_tokens: u64,
        reasoning_tokens: u64,
        /// Set once an exchange reported a prompt count and no cached-prompt count, and never
        /// cleared. Rule 14.5: the ratio then cannot be computed, and that is a failure to evaluate
        /// rather than a pass — so one such exchange has to survive to the end of the run.
        cache_unreported: bool,
        /// The accumulated cost, **in microcents**. [`Self::cost_cents`] is rule 14.2's figure.
        cost: u64,
        fallbacks: u64,
    }

    impl RunAccount {
        /// A run's opening account: the prices it declared, the ceiling it declared, and nothing
        /// spent.
        ///
        /// The ceiling arrives in **cents**, which is what `--spend-ceiling` parses to and what rule
        /// 14.2 states, and is converted here to the accumulation's unit. Saturating, because a
        /// ceiling so large that a million times it does not fit in a `u64` is a ceiling no run could
        /// reach, and `u64::MAX` is that same unreachable ceiling — where a wrap would turn a very
        /// large declared ceiling into a very small effective one, which is the one arithmetic
        /// failure here that would stop a run early rather than late.
        pub(super) fn declared(prices: PerTokenPrices, ceiling: Option<u64>) -> Self {
            Self {
                prices,
                ceiling: ceiling.map(|cents| cents.saturating_mul(MICROCENTS_PER_CENT)),
                ..Self::default()
            }
        }

        /// One exchange's cost, rule 14.2, from the reported counts and the declared prices.
        ///
        /// An unreported count contributes nothing, which is the only arithmetic rule 11.5 admits:
        /// a figure nobody reported cannot be billed, and substituting an estimate is what the
        /// specification's cache-ratio counterexample refuses on the other side of the same
        /// boundary. Rule 10.8 records the standing limit — a connector that under-reports usage
        /// spends past the ceiling and the run cannot tell — and this is where that limit lives.
        ///
        /// Every operation saturates, and `min` clamps a cached count above the prompt count.
        /// Rule 10.7 makes the provider's figures untrusted input, and a cached count larger than
        /// the prompt count is arithmetic nonsense that would otherwise underflow the uncached
        /// share into a bill of roughly eighteen quintillion tokens.
        pub(super) fn cost_of(&self, usage: &ExchangeUsage) -> u64 {
            let prompt = usage.prompt.unwrap_or(0);
            let cached = usage.cached_prompt.unwrap_or(0).min(prompt);
            let uncached = prompt - cached;
            uncached
                .saturating_mul(self.prices.uncached_prompt)
                .saturating_add(cached.saturating_mul(self.prices.cached_prompt))
                .saturating_add(usage.output.unwrap_or(0).saturating_mul(self.prices.output))
                .saturating_add(
                    usage
                        .reasoning
                        .unwrap_or(0)
                        .saturating_mul(self.prices.reasoning),
                )
        }

        /// Rule 14.1: the account after one exchange, with the four totals and the accumulated cost
        /// advanced by what that exchange reported.
        pub(super) fn after_exchange(self, usage: &ExchangeUsage) -> Self {
            let cost = self.cost_of(usage);
            Self {
                exchanges: self.exchanges.saturating_add(1),
                prompt_tokens: self.prompt_tokens.saturating_add(usage.prompt.unwrap_or(0)),
                cached_prompt_tokens: self
                    .cached_prompt_tokens
                    .saturating_add(usage.cached_prompt.unwrap_or(0)),
                output_tokens: self.output_tokens.saturating_add(usage.output.unwrap_or(0)),
                reasoning_tokens: self
                    .reasoning_tokens
                    .saturating_add(usage.reasoning.unwrap_or(0)),
                cache_unreported: self.cache_unreported
                    || (usage.prompt.is_some() && usage.cached_prompt.is_none()),
                cost: self.cost.saturating_add(cost),
                ..self
            }
        }

        /// Rule 9.5's occurrence, counted, and **nothing else**.
        ///
        /// It moves the fallback count and no other figure. The exchange itself is booked by
        /// [`Self::after_exchange`], which every exchange goes through whether or not it yielded a
        /// proposal, so a fallback is `self.after_exchange(&usage).counting_fallback()` and the
        /// exchange count moves exactly once.
        ///
        /// **This shape replaces one that billed the fallback as an unreported exchange**, and the
        /// reason is rule 10.4c. That earlier shape's own comment — "the usage it reported — none, by
        /// definition" — was true of rule 9.4's exchange, where the transport failed or the provider
        /// returned nothing, and it is false of rule 10.4c's, where a response arrived carrying
        /// usage and an action naming neither the model nor the reasoning level. That exchange fails
        /// the grammar check, becomes a counted fallback, and **was billed**:
        /// [`ConnectorPort::interpret`](super::ConnectorPort::interpret) is explicit that "the counts
        /// travel even when the action does not". Billing it as an absence would have understated
        /// every such run's cost, and rule 10.8 does not cover that — it covers a connector that
        /// under-reports, not a host that discards what was reported.
        ///
        /// What this is *not* is rule 9.6's rejected proposal: a source that answered and a world
        /// that refused the answer leave this count alone, and the port never learns that the
        /// refusal happened.
        pub(super) fn counting_fallback(self) -> Self {
            Self {
                fallbacks: self.fallbacks.saturating_add(1),
                ..self
            }
        }

        /// Rule 14.6: whether the accumulated cost has reached the declared ceiling. Reached, not
        /// exceeded — the check is made before the exchange is issued, so a ceiling equal to the
        /// cost of two exchanges admits exactly two.
        ///
        /// A run with no declared ceiling never stops here, which is rule 14.8's replay and rule
        /// 13.5's obligation seen from the account: whether a live run may run without a ceiling is
        /// a question for the mode that declares one, not for the arithmetic.
        pub(super) fn ceiling_reached(&self) -> bool {
            matches!(self.ceiling, Some(ceiling) if self.cost >= ceiling)
        }

        /// Rule 14.4's cache ratio in basis points, or `None` where rule 14.5 says it cannot be
        /// computed: an exchange reported a prompt count with no cached-prompt figure, or no prompt
        /// token was reported at all and the denominator is zero.
        ///
        /// `None` rather than a number, because rule 14.5 calls an uncomputable ratio "a failure to
        /// evaluate rather than a pass". A `0` would read as a run that cached nothing and a
        /// `10_000` as one that cached everything, and both are answers where the specification
        /// requires the absence of one.
        ///
        /// **From the reported figures only.** The specification's counterexample here is an
        /// implementation that computes this from its own block lengths, passes rule 14.5 and pays
        /// full price. This crate has no token estimator, and the two inputs below are both
        /// provider-reported totals.
        pub(super) fn cache_ratio_basis_points(&self) -> Option<u64> {
            if self.cache_unreported || self.prompt_tokens == 0 {
                return None;
            }
            Some(
                self.cached_prompt_tokens
                    .min(self.prompt_tokens)
                    .saturating_mul(BASIS_POINTS)
                    / self.prompt_tokens,
            )
        }

        /// The accumulated cost in the unit it is accumulated in, **microcents**.
        ///
        /// Rule 14.2's reportable figure is [`Self::cost_cents`]. This one exists because a test that
        /// asserted only the cent figure could not distinguish a run that spent 0.03 cents from one
        /// that spent nothing, and rule 14.6's ceiling is compared here.
        pub(super) fn cost(&self) -> u64 {
            self.cost
        }

        /// Rule 14.2's accumulated cost, in **whole US cents**.
        ///
        /// **Truncated, and the truncation is the point.** Rule 14.6 compares the accumulated cost
        /// against the ceiling in the accumulation's own unit, so truncating the *reported* figure
        /// keeps one invariant checkable from the outside: a run that stopped at its ceiling reports
        /// a cost at or above that ceiling, and a run that did not reports one below it. Rounding up
        /// would break exactly that — a run one microcent short of a 2-cent ceiling would report
        /// `2`, indistinguishable from the run that stopped, and `REQ-MOK-071` is about telling those
        /// two apart.
        ///
        /// The sub-cent remainder is not lost to a reader: rule 14.1's four token totals and rule
        /// 14.3a's declared prices are both reported, and the exact figure is their product. That is
        /// rule 14.3's own reasoning — the prices are inputs of the run, so a record that states them
        /// beside the counts has stated the cost to any precision.
        pub(super) fn cost_cents(&self) -> u64 {
            self.cost / MICROCENTS_PER_CENT
        }

        /// Rule 14.6's declared ceiling in **whole US cents**, which is the figure the operator wrote.
        ///
        /// Exact for every ceiling [`Self::declared`]'s conversion did not clamp, being that
        /// conversion undone. A ceiling large enough to have clamped comes back smaller than it was
        /// declared, and that is a ceiling of more than eighteen trillion cents — unreachable before
        /// and after, so the only figure it misstates is one no run could have spent against.
        pub(super) fn ceiling_cents(&self) -> Option<u64> {
            self.ceiling
                .map(|microcents| microcents / MICROCENTS_PER_CENT)
        }

        pub(super) fn fallbacks(&self) -> u64 {
            self.fallbacks
        }

        pub(super) fn exchanges(&self) -> u64 {
            self.exchanges
        }
    }

    /// How a live run ended, as rule 15.2's last field states it.
    ///
    /// A type of its own rather than a third [`TerminationReason`] variant, for three reasons that
    /// point the same way. That enum is public and both hosts match it, and `WO-MOK-025` moves
    /// nothing else on the public surface. It is written into `SPEC-MOK-006`'s run record as
    /// `result.reason`, so a new value there is a domain change and a `schema` increment, which this
    /// stage is out of scope for. And a ceiling stop is not a simulation outcome at all: no rule of
    /// `SPEC-MOK-001` produced it, and the world it stopped is neither extinct nor finished.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(super) enum RunEnd {
        /// The two endings every run has had since `SPEC-MOK-001`, carried as they are reported
        /// everywhere else so that one spelling of `tick_limit` serves every stream.
        Completed(TerminationReason),
        /// Rule 14.7's orderly stop, which rule 15.5 requires the record to say in so many words,
        /// and rule 19.3 gives an exit status distinct from a clean completion and from an error.
        Ceiling,
    }

    impl fmt::Display for RunEnd {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Completed(reason) => write!(formatter, "{reason}"),
                Self::Ceiling => formatter.write_str("ceiling"),
            }
        }
    }

    /// Rule 15's run record: every accounting figure this specification produces, in one place.
    ///
    /// It borrows the run's configuration and its account rather than copying figures out of them,
    /// so that the record cannot state a figure the account does not hold. **Where it is written is
    /// the host's**, exactly as rule 11.1 leaves the transcript's destination to the host: the
    /// *Outputs* section lists this as a fourth output and no rule fixes a destination for it, and
    /// the two streams that do have destinations are both closed to it — rule 12.6 requires a
    /// replay to reproduce the recorded run's standard output and record-stream bytes, and rule 15.6
    /// requires a replay to report no run record, so a run record in either stream would make those
    /// two rules contradict each other. [`Self::render`] therefore returns one unframed line, on
    /// [`exchange_record`](super::exchange_record)'s precedent.
    pub(super) struct RunRecord<'a> {
        config: &'a Config,
        /// Rule 15.2's model identifier and reasoning level. Inputs of the run under the *Inputs*
        /// section's "in live mode only", on rule 14.3's principle: a model named by a constant here
        /// would be a model this engine chose.
        model: &'a str,
        reasoning: &'a str,
        account: &'a RunAccount,
        /// Rule 15.2's tick reached, which rule 15.5 requires again for a ceiling stop, "so that a
        /// figure is never quoted at a horizon the run did not reach".
        tick_reached: u64,
        ended: RunEnd,
    }

    impl<'a> RunRecord<'a> {
        pub(super) fn new(
            config: &'a Config,
            model: &'a str,
            reasoning: &'a str,
            account: &'a RunAccount,
            tick_reached: u64,
            ended: RunEnd,
        ) -> Self {
            Self {
                config,
                model,
                reasoning,
                account,
                tick_reached,
                ended,
            }
        }

        /// The record as one line.
        ///
        /// Every field rule 15.2 enumerates is present unconditionally, and every figure is an
        /// integer or `null`: rule 15.3's positive zero means a clean run states `"fallbacks":0` and
        /// `"reasoning":0` rather than leaving a reader to infer them from a silence, and rule 11.5's
        /// absence is `null` rather than `0` for the two figures that can be missing. The exchange
        /// count is here on rule 15.1's authority — it is an accounting figure this specification
        /// produces, at rule 14.5's threshold of 200 — rather than on rule 15.2's enumeration.
        ///
        /// **The two money figures are named for their unit and the unit is rule 14.2's.** They were
        /// `cost_nanodollars` and `ceiling_nanodollars` while `WO-MOK-025` accumulated in
        /// nanodollars; rule 14.2 as amended 2026-08-29 states the minor unit, and it is the US cent,
        /// so they are `cost_cents` and `ceiling_cents`. The rename is a rename of the *reported*
        /// figures only — [`PerTokenPrices`] records why the accumulation behind them is finer — and
        /// naming a field for a unit it does not carry is the failure this pair exists to avoid.
        ///
        /// The seed, tick limit, density and tracing selection are spelled as
        /// [`write_header_record`](super::write_header_record) spells them, including the density as
        /// a quoted two-decimal string. That is the one decimal separator in the line and it is an
        /// echoed input rather than an accounting figure: `SPEC-MOK-006` rule 4.1 prohibits a
        /// floating-point *value*, which a string is not, and a second spelling of a density across
        /// two streams would be worse than the quoting.
        pub(super) fn render(&self) -> String {
            format!(
                "{{\"run_record\":\"{LLM_SOURCE_NAME}\",\"seed\":{},\"ticks\":{},\"density\":\"{}\",\
                 \"trace_actions\":{},\"model\":\"{}\",\"reasoning\":\"{}\",\"exchanges\":{},\
                 \"tokens\":{{\"prompt\":{},\"cached_prompt\":{},\"output\":{},\"reasoning\":{}}},\
                 \"cache_ratio_basis_points\":{},\"cost_cents\":{},\
                 \"ceiling_cents\":{},\"fallbacks\":{},\"unfit_to_publish\":{},\
                 \"tick_reached\":{},\"ended\":\"{}\"}}",
                self.config.seed,
                self.config.tick_limit,
                self.config.density,
                self.config.trace_actions,
                escape_transcript_text(self.model),
                escape_transcript_text(self.reasoning),
                self.account.exchanges,
                self.account.prompt_tokens,
                self.account.cached_prompt_tokens,
                self.account.output_tokens,
                self.account.reasoning_tokens,
                optional_figure(self.account.cache_ratio_basis_points()),
                self.account.cost_cents(),
                optional_figure(self.account.ceiling_cents()),
                self.account.fallbacks,
                // Rule 15.4's mark, and a property of the record rather than of a summary written
                // afterwards. It is derived here from the count rather than set by a caller,
                // because a mark a caller could forget is a mark that a run under pressure would
                // not carry.
                self.account.fallbacks > 0,
                self.tick_reached,
                self.ended,
            )
        }
    }

    /// A figure that may be absent: the integer, or `null`.
    ///
    /// `SPEC-MOK-006` rule 4.4's distinction and rule 11.5's — a figure nobody reported is not a
    /// figure that was zero — and the same choice `died_at` makes in the other run record.
    fn optional_figure(figure: Option<u64>) -> String {
        match figure {
            Some(figure) => figure.to_string(),
            None => String::from("null"),
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

/// How a recorded run ended: as a simulation, or at `SPEC-MOK-007` rule 14.6's ceiling.
///
/// `pub(crate)` and not `pub`, on `Simulation::run_recording`'s precedent and for the same reason:
/// rule 20.5.2 makes that entry point the crate-private carrier between the binary target and the
/// engine, this is what it now carries back, and `SPEC-MOK-002` rule 5's public interface grows by
/// nothing here. `Simulation::run` keeps its `RunSummary` and its exact signature, so every host
/// that never lends a port sees no change at all.
///
/// A ceiling stop is **not** a third [`TerminationReason`]. That enum is public, both hosts match on
/// it, and it is written into `SPEC-MOK-006`'s run record as `reason` — so a third value there is a
/// domain change and a `schema` increment that this work order is out of scope for. It is also not a
/// simulation outcome: no rule of `SPEC-MOK-001` produced it, and the world it stopped is neither
/// extinct nor finished. [`accounting::RunEnd`] states the same separation on the accounting side and
/// states it first; this type is the run loop's half of it.
/// `Debug` because `Result::unwrap_err` requires it of the success type, and four existing tests
/// assert on a run that failed to write its record sink.
#[derive(Debug)]
pub(crate) enum RunOutcome {
    /// The run reached one of `SPEC-MOK-001`'s two endings and has a summary.
    Completed(RunSummary),
    /// Rule 14.6's stop. The tick reached is rule 15.5's figure and rule 14.7's boundary — the
    /// streams are complete and readable to it — and it is the tick that was **in progress** when the
    /// ceiling was met, so the last tick with a metrics record is the one before it.
    Ceiling { tick_reached: u64 },
}

/// What one tick's opportunities came to.
///
/// Private, and a two-variant enum rather than a `bool`, because the caller acts differently on each
/// and a `bool` at a call site says nothing about which way is which.
enum TickFlow {
    /// Every living Mokiterion was given its opportunity.
    Completed,
    /// Rule 14.6's ceiling was reached before an opportunity could be issued. That opportunity and
    /// every later one in the tick was not taken, no action was applied for it, and the tick has no
    /// metrics record because it is not a completed tick — rule 7.1 of `SPEC-MOK-006` gives one to a
    /// tick that *terminates* the run, which this is not.
    CeilingReached,
}

/// What one step came to: [`TickFlow`] with the termination check folded in.
enum StepFlow {
    /// The tick completed and the run continues.
    Continued,
    /// The tick completed and terminated the run.
    Ended(TerminationReason),
    /// [`TickFlow::CeilingReached`], carried out unchanged. The termination check is not reached,
    /// because a tick that was cut short cannot be asked whether it emptied the world.
    CeilingReached,
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

impl RegenerationSkipReason {
    /// Every reason, in the order `SPEC-MOK-006` rule 8.2's `regeneration_skipped` object
    /// states them. Private: it exists to key the two skip counters and to enumerate the
    /// domain for the alphabet check, and neither is a public concern.
    const ALL: [Self; 2] = [Self::Depleted, Self::Capacity];

    /// The counter this reason increments, matching [`Self::ALL`]'s order the way
    /// [`FoodClass::index`] matches [`FoodClass::ALL`]'s.
    fn index(self) -> usize {
        match self {
            Self::Depleted => 0,
            Self::Capacity => 1,
        }
    }
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
        /// `ATTRIBUTE_MAX`, on the same terms as rule 24's `transferred`. `REQ-MOK-055`
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
        /// when non-empty, which `CAP-MOK-010` requires: a field appended unconditionally
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

/// The schema version of the structured record stream, `SPEC-MOK-006` rule 10.
///
/// It is the stream's own version and not the engine's: `engine` in the header record
/// identifies the producer and this identifies the contract, so an engine release that
/// changes no byte any conforming writer produces does not move it. Rule 10.2 fixes what
/// does move it, and it is a declared compatibility surface of this product.
///
/// `2` since 2026-08-23, when the repository owner ratified `SPEC-MOK-006`'s 2026-08-21
/// amendment row. That row is the first exercise of rule 10.2: the merge that brought
/// `CAP-MOK-010`'s combat and this stream into one tree added fourteen field names and
/// thirteen domain members, and the stream carried them for three commits while still
/// declaring `1`. The row states the divergence itself and left the increment to the
/// owner, because moving a declared compatibility surface is not an implementation
/// agent's decision.
///
/// `3` in the same change that adds [`Policy::Llm`], and for a smaller reason than the move to
/// `2`: rule 10.2 moves this when "a value's domain in rule 3.2 gains or loses a member", and
/// `llm` joins two of them — `config.policy` in the header record and `result.source` in the
/// decision record. No field is added, no field is removed and no field's meaning changes, so a
/// reader of a `3` stream that ignores the two new members reads a `2` stream correctly. The
/// increment is still owed: rule 10.2 does not grade a domain change by how much a reader
/// notices, and `ADR-MOK-007` requires the increment to be "one more than whatever value the
/// ratification leaves standing" rather than a number chosen when this was written. The owner
/// declined folding the two moves into a single one, which is why this is `3` and not `2`.
const RECORD_SCHEMA_VERSION: u32 = 3;

/// The two streams a run writes: the text stream `SPEC-MOK-001` fixes, and the optional
/// structured record stream `SPEC-MOK-006` fixes.
///
/// The pair is threaded through the tick rather than held on [`Simulation`], because a sink
/// belongs to the caller for the duration of one call and holding it would make it state.
/// The text stream is a type parameter, as it was before there was a second stream; the
/// record sink is a trait object so that a caller with no sink passes `None` and leaves
/// nothing to infer.
///
/// `'sink` is the trait object's own lifetime and is deliberately separate from `'a`, the
/// lifetime of the borrows this struct holds. `&mut dyn Write` is invariant in the trait
/// object's lifetime, so a single lifetime would force `execute`'s two independent borrows to
/// be provably equal in length, which they are not and which no caller should have to
/// arrange. With the two separated, `'a` shortens freely and `'sink` stays put.
struct Sinks<'a, 'sink, W: Write> {
    text: &'a mut W,
    records: Option<&'a mut (dyn Write + 'sink)>,
}

impl<'sink, W: Write> Sinks<'_, 'sink, W> {
    /// The record sink, reborrowed for one record. `None` is the whole of rule 1.1's
    /// "no sink-related code path runs": every producer below asks for the sink first and
    /// computes nothing when the answer is `None`.
    fn records(&mut self) -> Option<&mut (dyn Write + 'sink)> {
        self.records.as_deref_mut()
    }
}

/// Distinguishes a record-sink failure from a text-stream failure, which `SPEC-MOK-006`
/// rule 13.5 requires of the diagnostic.
///
/// The form is fixed and carries two things: that the sink was the stream that failed, and
/// the platform's own reason. It carries no path, because the library target never learns
/// one.
fn sink_error(error: io::Error) -> io::Error {
    io::Error::new(error.kind(), format!("record sink: {error}"))
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
    /// `SPEC-MOK-006`'s *State model*: the run's cumulative counters, one per phenomenon the
    /// run record states and no text line does.
    ///
    /// Each is incremented at exactly the point its event is emitted, in the same statement
    /// sequence, so a counter and the event stream cannot disagree. Each saturates: `u64`
    /// cannot be exhausted by any run the tick limit admits, and saturating arithmetic makes
    /// that a stated property rather than an assumption. **They exist whether or not a sink
    /// is configured** — a counter that existed only under an option would make the option a
    /// behavior change — and no counter is derived from a draw, participates in a rule, a
    /// decision, a proposal, a validation or an applied action, or reaches the text stream.
    ///
    /// All are private and the type exposes no accessor for any of them, so `SPEC-MOK-002`
    /// rule 6 needs no relaxation to admit them.
    /// Every territory crossing over the run.
    crossings: u64,
    /// Every consumption over the run, by resource class, keyed by [`FoodClass::index`].
    consumed: [u64; 3],
    /// Every resource the run regenerated.
    regenerated: u64,
    /// Every skipped regeneration, by reason, keyed by [`RegenerationSkipReason::index`].
    regeneration_skipped: [u64; 2],
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
                died_at: None,
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
            crossings: 0,
            consumed: [0; 3],
            regenerated: 0,
            regeneration_skipped: [0; 2],
        })
    }

    /// Runs to termination, writing the `SPEC-MOK-001` text record to `output`.
    ///
    /// The signature `SPEC-MOK-002` rule 5 enumerates, unchanged by the record stream and
    /// unchanged by the decision port: `SPEC-MOK-007` rule 20.5.1 fixes that this is **not**
    /// amended, and it delegates with the port absent. A host that wants either calls the
    /// entry point that takes it; a host that wants neither passes nothing extra, and its call
    /// site is the same character for character as it was before either existed.
    ///
    /// The consequence is that a `Policy::Llm` run cannot be started through here: it is an
    /// invalid configuration with no port, and this reports [`MISSING_DECISION_PORT`] rather
    /// than substituting a source. That is rule 20.8 and not an oversight of rule 20.5.1.
    ///
    /// It reports a [`RunSummary`] and not a [`RunOutcome`], and can, because with no port there is
    /// nothing that spends: [`DecisionSource::halted`]'s default is `false` for all four deterministic
    /// sources and `Policy::Llm` is refused above, so rule 14.6's ceiling is unreachable from here.
    /// The arm is still written out rather than asserted away — `ARCH-MOK-001` wants ordinary `Result`
    /// propagation rather than a panic, and a refusal a later reader can read beats an
    /// `unreachable!()` a later change can reach.
    pub fn run<W: Write>(&mut self, output: &mut W) -> io::Result<RunSummary> {
        match self.run_recording(output, None, None)? {
            RunOutcome::Completed(summary) => Ok(summary),
            RunOutcome::Ceiling { tick_reached } => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "a run with no decision port reported a spend ceiling at tick {tick_reached}"
                ),
            )),
        }
    }

    /// [`Simulation::run`], additionally projecting every record `SPEC-MOK-006` fixes onto
    /// `records` when a sink is supplied, and obtaining proposals through `port` when a
    /// decision port is.
    ///
    /// `pub(crate)` rather than `pub`: the only caller is `execute`, the sink and the port are
    /// both the *host's* to build, and `SPEC-MOK-006` rule 12.2 grows the public interface by
    /// nothing beyond `execute`'s parameters. `SPEC-MOK-007` rule 20.5.2 names this the
    /// crate-private carrier of the port down the call chain and discloses that it takes it, so
    /// that the parameter's appearance here is a stated part of the change and not something a
    /// reader finds. `pub(crate) fn` is not `pub fn`, so rule 5's public-surface grep does not
    /// match this line and still returns exactly `run` and `advance_tick`.
    ///
    /// With neither this is [`Simulation::run`] exactly — the same text bytes, the same draws,
    /// the same outcome — which is rule 11 stated as a call graph rather than as a promise.
    pub(crate) fn run_recording<W: Write>(
        &mut self,
        output: &mut W,
        records: Option<&mut dyn Write>,
        port: Option<&mut dyn Proposer>,
    ) -> io::Result<RunOutcome> {
        let mut sinks = Sinks {
            text: output,
            records,
        };
        match self.config.policy {
            Policy::Baseline => {
                let mut source = BaselineDecisionSource;
                self.run_with_source(&mut sinks, &mut source)
            }
            Policy::Reference => {
                let mut source = ReferenceDecisionSource;
                self.run_with_source(&mut sinks, &mut source)
            }
            Policy::Individual => {
                let mut source = IndividualDecisionSource;
                self.run_with_source(&mut sinks, &mut source)
            }
            Policy::Social => {
                let mut source = SocialDecisionSource;
                self.run_with_source(&mut sinks, &mut source)
            }
            // Rule 20.8: this source with no port is an invalid configuration and the run
            // refuses. It does not substitute a source, does not proceed with no decisions, and
            // does not treat the absence as rule 9's fallback — a run of `wait` for a thousand
            // ticks would produce a stream a reader could mistake for a measurement.
            //
            // Rule 20.9 is the other half, and it is the four arms above: a port supplied while
            // one of them is selected is ignored exactly as an absent sink is, and is not an
            // error. `port` is dropped there, which is that rule and not a leak.
            Policy::Llm => match port {
                Some(port) => {
                    let mut source = PortDecisionSource::new(port);
                    self.run_with_source(&mut sinks, &mut source)
                }
                None => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    MISSING_DECISION_PORT,
                )),
            },
        }
    }

    fn run_with_source<W: Write, D: DecisionSource>(
        &mut self,
        sinks: &mut Sinks<'_, '_, W>,
        decision_source: &mut D,
    ) -> io::Result<RunOutcome> {
        if self.tick != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "a simulation instance can only be run once",
            ));
        }

        // `SPEC-MOK-007` rule 11.1's prefix head, before the first exchange and therefore before
        // the first tick. It is written through the source and reaches only whatever the host
        // connected: it touches neither stream below, so a run's text bytes and record bytes are
        // the same with a transcript and without one. The four deterministic sources take
        // `DecisionSource::open`'s default and this loop hands them a roster they ignore.
        //
        // The borrow is scoped, because the roster borrows the agents and everything after this
        // mutates them.
        {
            let mut roster: Vec<RosterEntry<'_>> = self
                .agents
                .iter()
                .map(|agent| RosterEntry {
                    id: &agent.id,
                    waste_tolerance: agent.waste_tolerance,
                })
                .collect();
            // Imposed here rather than inherited, for the reason `write_run_record` gives about
            // rule 8.4's agent list: an ordering that came from a collection's iteration order is a
            // determinism defect waiting to manifest, and the head's order is part of the bytes.
            roster.sort_by(|left, right| left.id.cmp(right.id));
            decision_source.open(&roster).map_err(transcript_error)?;
        }

        // Rule 5.1: the header is first in the stream, before the first tick and before any
        // other record. A refused run is refused above and writes nothing at all.
        if let Some(sink) = sinks.records() {
            write_header_record(sink, &self.config).map_err(sink_error)?;
        }

        for event in self.entity_initialization_events() {
            self.emit(sinks, event)?;
        }
        let event = Event::new(
            0,
            "world",
            EventDetail::DecisionSourceSelected {
                source: decision_source.name().to_string(),
            },
        );
        self.emit(sinks, event)?;

        loop {
            match self.step(sinks, decision_source)? {
                StepFlow::Continued => {}
                StepFlow::Ended(reason) => {
                    let summary = self.summary(reason);
                    self.emit_summary(&mut *sinks.text, &summary)?;
                    // Rule 8.1: one run record per run, last in the stream, after the final
                    // tick's metrics record. Rule 9.3 pairs it with the summary line above, which
                    // is why it is written here and not inside `emit_summary`: the text stream has
                    // one authority and this record is the other stream's counterpart to it.
                    if let Some(sink) = sinks.records() {
                        self.write_run_record(sink, &summary).map_err(sink_error)?;
                    }
                    return Ok(RunOutcome::Completed(summary));
                }
                // Rule 14.6's stop, leaving both streams exactly as the cut-short tick left them.
                // No summary line, because `RunSummary` reports a `TerminationReason` and there is
                // none — a run that stopped spending is not a run that ended, and writing the
                // twelve-figure summary of a horizon the run did not reach is precisely what rule
                // 15.5 exists to prevent.
                StepFlow::CeilingReached => {
                    return Ok(RunOutcome::Ceiling {
                        tick_reached: self.tick,
                    });
                }
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
    ///
    /// `port` is `SPEC-MOK-007` rule 20.5's second door, and it is lent per tick rather
    /// than held because rule 20.4 puts the port's whole lifetime in the host: **the host builds
    /// it once, owns it for the run, and lends it here.** Rule 20.4.1 is why that matters and is
    /// not a preference — a port rebuilt for each call compiles and runs, and resets the
    /// transcript cursor, the accumulated cost and the fallback count every tick, so a replay
    /// would return the transcript's first entry a thousand times and report no drift.
    ///
    /// A host with no port passes `None` and needs no type annotation for a port it does not
    /// have, which is the shape `SPEC-MOK-002` rule 4 already fixed for the record sink.
    ///
    /// This signature is one line deliberately, and the interface is named for its width.
    /// `SPEC-MOK-002` rule 5 detects a third public mutating entry point by matching the
    /// declaration keyword and the receiver on one line, so a signature rustfmt wraps drops
    /// this method out of the check's own result — the check would then pass while reporting
    /// one door instead of two, which is a weakened check rather than a failing one.
    /// `DecisionPort` reaches 109 columns here, and 104 with the shortest sensible parameter
    /// name, both past rustfmt's 100. The interface's shape is `SPEC-MOK-007` rule 1.1's and
    /// its spelling is nobody's: no artifact fixes the identifier, and rule 1.1's own words are
    /// "obtains a **proposal**", which is what `propose` is called. The word *port* is the
    /// artifacts' and is kept everywhere it costs no width — this parameter, these doc
    /// comments, `PortDecisionSource` and `MISSING_DECISION_PORT`.
    pub fn advance_tick(&mut self, port: Option<&mut dyn Proposer>) -> Result<TickOutcome, String> {
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
            // Rule 20.8, on the observer host's door, and the check sits in the arm rather than
            // above the dispatch on purpose: a fresh `Simulation` has no outcome, so the first
            // call reaches here, which is rule 20.8's "refused on the first tick" literally. A
            // finished run reports finished above, and it can only have finished by having had
            // a port every tick, so that path never reaches a refusal it would be wrong to
            // report.
            Policy::Llm => match port {
                Some(port) => {
                    let mut source = PortDecisionSource::new(port);
                    self.advance_tick_with_source(&mut source)
                }
                None => Err(MISSING_DECISION_PORT.to_string()),
            },
        }
    }

    fn advance_tick_with_source<D: DecisionSource>(
        &mut self,
        decision_source: &mut D,
    ) -> Result<TickOutcome, String> {
        self.collected_events = Some(Vec::new());
        // The observer host reads events structurally and writes neither stream, so both
        // sinks are absent here: the text stream goes nowhere and no record is produced.
        let mut discarded = io::sink();
        let mut sinks = Sinks {
            text: &mut discarded,
            records: None,
        };
        let stepped = self.step(&mut sinks, decision_source);
        let events = self.collected_events.take().unwrap_or_default();
        match stepped {
            Ok(StepFlow::Continued) => Ok(TickOutcome {
                events,
                finished: false,
                reason: None,
            }),
            Ok(StepFlow::Ended(reason)) => Ok(TickOutcome {
                events,
                finished: true,
                reason: Some(reason),
            }),
            // Rule 14.8 is why this is an error and not a fourth shape of [`TickOutcome`]: **a replay
            // has no ceiling**, rule 20.1 makes this entry point the observer host's, and rule 18.4.2
            // gives that host no live-mode selection to make — so a port reaching here that has
            // stopped spending is a port this door was never meant to be given. Reporting it as an
            // error says so; `TickOutcome { finished: true, reason: None }` is representable and would
            // say instead that the run ended for no reason, to a host whose `is_finished` reads
            // `self.outcome` and would answer `false` at the same moment. `TickOutcome` is public and
            // `SPEC-MOK-002` rule 5's interface does not grow here to carry a case rule 14.8 excludes.
            Ok(StepFlow::CeilingReached) => Err(format!(
                "the decision port reported its spend ceiling reached at tick {}, \
                 and this entry point has no spending to bound",
                self.tick
            )),
            // `io::sink` cannot fail, so a step's error is the engine's own or the port's:
            // `SPEC-MOK-007` rules 12.3 and 12.4 reach here through `DecisionSource::failure`, and
            // rule 19.4 requires the message to name the opportunity, which the port's words do.
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
            // Named rather than asked, because the source that would answer needs a port and
            // this method takes none: it reports the *selected* source before any tick, and the
            // selection is the configuration's, not the port's. `LLM_SOURCE_NAME` is the one
            // place the string lives, so this and the record the run writes cannot disagree.
            Policy::Llm => LLM_SOURCE_NAME.to_string(),
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
        sinks: &mut Sinks<'_, '_, W>,
        decision_source: &mut D,
    ) -> io::Result<StepFlow> {
        self.tick += 1;
        // Rule 14.7's orderly stop, and what makes it orderly is everything this branch does not do.
        // No metrics record for the tick that was cut short, because it is not a completed tick; no
        // `simulation_ended` event, because the simulation did not end and `SPEC-MOK-006` admits no
        // reason it could carry; no run record, because rule 8.1 of that specification gives one to a
        // run and this one has no outcome to state. What was written stays written: rule 13.4's
        // removal is for a *partial* stream nobody should read as a whole run, and this stream is
        // complete and readable to the tick reached, which is what rule 14.7 asks for in so many
        // words. `self.outcome` also stays `None`, so a host that asks whether the run finished is
        // told the truth — it did not, it stopped.
        if let TickFlow::CeilingReached = self.run_tick(sinks, decision_source)? {
            return Ok(StepFlow::CeilingReached);
        }

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
            self.emit(sinks, event)?;
            // Rule 7.1: a tick that terminates the run is a completed tick and carries its
            // metrics record, after that tick's `simulation_ended` event.
            self.write_metrics(sinks)?;
            return Ok(StepFlow::Ended(reason));
        }
        self.write_metrics(sinks)?;
        Ok(StepFlow::Continued)
    }

    /// Writes one authoritative event to the host's sink, projects it onto the record stream
    /// when one is configured, and retains it when a host is collecting.
    ///
    /// **Every event passes through here, and this is the record stream's only emission
    /// site.** That is why a collected `TickOutcome`, the `REQ-MOK-010` text record and the
    /// `SPEC-MOK-006` record stream cannot disagree about order or content, and why
    /// `REQ-MOK-042`'s one-to-one correspondence is structural rather than maintained. A
    /// second emission site would make rule 9.3 a thing to keep true; adding one is a defect.
    ///
    /// The text stream is written first, so a run whose record sink fails has already written
    /// the text line the record would have accompanied, and the two streams are never
    /// inconsistent in the surviving direction.
    fn emit<W: Write>(&mut self, sinks: &mut Sinks<'_, '_, W>, event: Event) -> io::Result<()> {
        writeln!(sinks.text, "{event}")?;
        if let Some(sink) = sinks.records() {
            write_event_record(sink, &event).map_err(sink_error)?;
        }
        if let Some(collected) = &mut self.collected_events {
            collected.push(event);
        }
        Ok(())
    }

    fn run_tick<W: Write, D: DecisionSource>(
        &mut self,
        sinks: &mut Sinks<'_, '_, W>,
        decision_source: &mut D,
    ) -> io::Result<TickFlow> {
        self.decisions.clear();
        for agent_index in 0..self.agents.len() {
            if !self.agents[agent_index].alive {
                continue;
            }

            // `SPEC-MOK-007` rule 14.6, and its whole substance is where these three lines sit.
            // **Before** the observation is composed, before the request is built, before anything is
            // said to the source: the check precedes the spending, so a declared ceiling bounds a run
            // rather than being overshot by one exchange. Moving it below `decide` would compile, pass
            // a test that only compared the final cost against the ceiling, and spend one exchange
            // past every ceiling ever declared.
            //
            // Per opportunity and not per tick, because an exchange is issued per acting Mokiterion:
            // a tick of twelve is twelve exchanges, and a check at the tick boundary would admit
            // eleven of them past the ceiling. `VER-MOK-018` case L30 is the measurement that makes
            // the distinction matter — at that configuration a ceiling of eighteen falls inside the
            // second tick.
            //
            // The four deterministic sources take `DecisionSource::halted`'s default and never enter
            // this branch, so `REQ-MOK-068`'s byte-identity is untouched: no draw is taken here and no
            // record is written, whichever way the answer goes.
            if decision_source.halted() {
                return Ok(TickFlow::CeilingReached);
            }

            let observation = self.observation(agent_index);
            // Rule 12 updates `fear` from *this* observation's perceived-Mokiterion list, so
            // the driver is read here and carried, not re-perceived after the action.
            let perceived_company = !observation.perceived_mokiterions.is_empty();
            let proposal = {
                let mut entropy = DecisionEntropy::new(&mut self.entropy);
                decision_source.decide(&observation, &mut entropy)
            };
            // `SPEC-MOK-007` rules 12.3, 12.4 and 19.6, collected before the proposal is applied.
            // A replay that met a mismatched or exhausted transcript, and a live run that could not
            // write one, both end here: the proposal above is discarded, no action is applied for
            // this opportunity, no further opportunity is reached, and rule 12.3's "produces no
            // further ticks" therefore holds for the tick in progress as well as for the next.
            // The four deterministic sources take the default and this is `None` for every one of
            // them, so no run that existed before the port does can reach it.
            //
            // Wrapped like `open`'s and on `sink_error`'s precedent, so that a reader can tell a
            // transcript failure from the engine's own `SPEC-MOK-001` rule 15 placement failure.
            // The port's own words survive the wrap, which is what leaves rule 19.4's naming of the
            // opportunity and the mismatch intact.
            if let Some(error) = decision_source.failure() {
                return Err(transcript_error(error));
            }
            let result = self.apply_action(sinks, agent_index, &proposal)?;
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
                self.emit_action_trace(sinks, agent_index, &proposal, &result)?;
            }

            // Rule 25 closes the window when the opportunity is taken rather than when it is
            // used: after the source has been consulted and the proposal applied or rejected,
            // whether it answered, proposed something else, or was rejected. The clearing sits
            // here, past the trace and before rule 12, and it is positioned identically
            // whether or not the flag is set — a clearing after an emission that only
            // sometimes happens would make `--trace-actions` change simulation state.
            self.agents[agent_index].suffered.clear();

            self.apply_survival(sinks, agent_index, perceived_company)?;
        }

        if self.tick.is_multiple_of(REGENERATION_INTERVAL) {
            for territory in Territory::ALL {
                self.regenerate_food(sinks, territory)?;
            }
        }
        Ok(TickFlow::Completed)
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
        sinks: &mut Sinks<'_, '_, W>,
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
            Action::Move { direction } => self.apply_move(sinks, agent_index, *direction),
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
                let class_index = food.class.index();
                self.consumed[class_index] = self.consumed[class_index].saturating_add(1);
                self.emit(sinks, event)?;
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
            | Action::Avoid { .. } => self.apply_targeted_action(sinks, agent_index, action),
        }
    }

    /// Rule 8's move, applied once and reached by two routes: a `move` proposal and rule 21's
    /// three targeted moves, which are "rule 8 moves and nothing more". One implementation is
    /// why a targeted move cannot acquire a cost, a second crossing rule or a different
    /// bounds test.
    fn apply_move<W: Write>(
        &mut self,
        sinks: &mut Sinks<'_, '_, W>,
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
            // The counter moves in the same statement sequence as the event it counts, so
            // rule 8.6's equality between the two is a property of this block rather than of
            // a later reconciliation. Both routes to a crossing pass through here, so a
            // targeted move's crossing is counted on the same terms as a `move` proposal's.
            self.crossings = self.crossings.saturating_add(1);
            self.emit(sinks, event)?;
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
        sinks: &mut Sinks<'_, '_, W>,
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
                self.apply_targeted_move(sinks, agent_index, target_index, false)
            }
            Action::Avoid { .. } | Action::Retreat { .. } => {
                self.apply_targeted_move(sinks, agent_index, target_index, true)
            }
            Action::Attack { .. } | Action::Fight { .. } => {
                self.resolve_strike(sinks, agent_index, target_index)
            }
            Action::Threaten { .. } => self.resolve_threat(sinks, agent_index, target_index),
            Action::Surrender { .. } => self.resolve_surrender(sinks, agent_index, target_index),
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
        sinks: &mut Sinks<'_, '_, W>,
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
                return self.apply_move(sinks, agent_index, candidate);
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
        sinks: &mut Sinks<'_, '_, W>,
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
        self.emit(sinks, event)?;

        // Rule 13's path, event and finality. There is no second death and no combat-specific
        // death event; what differs is only that this one happens inside another Mokiterion's
        // turn, so the target may die at a point in the tick where it has not yet acted.
        if died {
            let event = Event::new(
                self.tick,
                self.agents[target_index].id.clone(),
                EventDetail::AgentDied { health: 0 },
            );
            self.emit(sinks, event)?;
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
    /// stood at [`ATTRIBUTE_MAX`], not the nominal [`THREAT_FEAR_INCREASE`]. `REQ-MOK-055`
    /// requires the increase *applied*, on the same terms as rule 24's `transferred`; a
    /// saturated threat succeeds and reports that it moved nothing. The nominal constant stays
    /// recoverable from the pair of `fear` values the same event carries.
    fn resolve_threat<W: Write>(
        &mut self,
        sinks: &mut Sinks<'_, '_, W>,
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
        self.emit(sinks, event)?;

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
        sinks: &mut Sinks<'_, '_, W>,
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
        self.emit(sinks, event)?;

        Ok(ActionResult {
            accepted: true,
            detail: format!("transferred:{transferred}"),
        })
    }

    fn emit_action_trace<W: Write>(
        &mut self,
        sinks: &mut Sinks<'_, '_, W>,
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
        self.emit(sinks, event)
    }

    /// Rule 12: survival decay, then rule 12's `fear` update from the same tick's rule 3
    /// observation. `perceived_company` is whether that observation's perceived-Mokiterion
    /// list held at least one entry — the whole driver, with no distance constant of its own,
    /// because rule 3's list is already bounded by the perception radius.
    fn apply_survival<W: Write>(
        &mut self,
        sinks: &mut Sinks<'_, '_, W>,
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
        self.emit(sinks, event)?;

        if died {
            // The death tick is recorded in the same statement sequence as the event that
            // reports the death, and is written exactly once because `alive` is already
            // `false` by the time this tick's loop could reach this Mokiterion again.
            self.agents[agent_index].died_at = Some(self.tick);
            let event = Event::new(
                self.tick,
                self.agents[agent_index].id.clone(),
                EventDetail::AgentDied { health: 0 },
            );
            self.emit(sinks, event)?;
        }
        Ok(())
    }

    fn regenerate_food<W: Write>(
        &mut self,
        sinks: &mut Sinks<'_, '_, W>,
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
            let reason_index = reason.index();
            self.regeneration_skipped[reason_index] =
                self.regeneration_skipped[reason_index].saturating_add(1);
            return self.emit(sinks, event);
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
            self.regenerated = self.regenerated.saturating_add(1);
            self.emit(sinks, event)?;
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

    /// One metrics record for the tick just completed, `SPEC-MOK-006` rule 7.
    ///
    /// Every figure describes the state at the end of the tick the record names, and all of
    /// them are read in the single pass below, so rule 7.3's "no figure is read at a different
    /// point in the tick from any other" is a property of the loop rather than a convention.
    /// Nothing here draws against the entropy stream and nothing here is retained.
    ///
    /// The sink is asked for first: with no sink this returns before reading any state, which
    /// is rule 1.1's "no sink-related code path runs". The record itself is written by
    /// [`Simulation::metrics_record`], so that every failure inside it is labelled once here
    /// rather than at each `write!` — rule 13.5 wants one form of diagnostic, not several.
    fn write_metrics<W: Write>(&self, sinks: &mut Sinks<'_, '_, W>) -> io::Result<()> {
        let Some(sink) = sinks.records() else {
            return Ok(());
        };
        self.metrics_record(sink).map_err(sink_error)
    }

    /// The metrics record's bytes. See [`Simulation::write_metrics`] for the rules it serves.
    fn metrics_record(&self, sink: &mut dyn Write) -> io::Result<()> {
        let mut living = 0usize;
        let mut population = [0usize; 2];
        // Sums are `u64` over twelve `u8` terms, so no accumulation here can overflow in any
        // build. Rule 4.2: a sum, and in the same record the count that divides it.
        let mut health_sum = 0u64;
        let mut satiety_sum = 0u64;
        let mut energy_sum = 0u64;
        let mut fear_sum = 0u64;
        // Rule 7.5's one extremum per attribute: `min` for the three whose depletion threatens
        // survival, `max` for the one whose accumulation is its own direction of harm.
        let mut health_min: Option<u8> = None;
        let mut satiety_min: Option<u8> = None;
        let mut energy_min: Option<u8> = None;
        let mut fear_max: Option<u8> = None;
        for agent in self.agents.iter().filter(|agent| agent.alive) {
            living += 1;
            population[agent.position.territory().index()] += 1;
            health_sum += u64::from(agent.health);
            satiety_sum += u64::from(agent.satiety);
            energy_sum += u64::from(agent.energy);
            fear_sum += u64::from(agent.fear);
            health_min = Some(health_min.map_or(agent.health, |seen| seen.min(agent.health)));
            satiety_min = Some(satiety_min.map_or(agent.satiety, |seen| seen.min(agent.satiety)));
            energy_min = Some(energy_min.map_or(agent.energy, |seen| seen.min(agent.energy)));
            fear_max = Some(fear_max.map_or(agent.fear, |seen| seen.max(agent.fear)));
        }
        // Rule 7.4: the two sum to the roster size at every tick.
        let deaths = self.agents.len() - living;

        write!(
            sink,
            "{{\"record\":\"metrics\",\"tick\":{},\"living\":{living},\"deaths\":{deaths},\"population\":{{\"A\":{},\"B\":{}}}",
            self.tick, population[0], population[1]
        )?;
        write_attribute(sink, "health", health_sum, "min", health_min)?;
        write_attribute(sink, "satiety", satiety_sum, "min", satiety_min)?;
        write_attribute(sink, "energy", energy_sum, "min", energy_min)?;
        write_attribute(sink, "fear", fear_sum, "max", fear_max)?;
        write!(sink, ",\"territories\":{{")?;
        for (position, territory) in Territory::ALL.into_iter().enumerate() {
            if position > 0 {
                write!(sink, ",")?;
            }
            self.write_metrics_territory(sink, territory)?;
        }
        writeln!(sink, "}}}}")
    }

    /// One territory's object inside a metrics record, rule 7.6.
    ///
    /// `capacity` does not vary within a run and is stated every tick regardless, so that a
    /// single record is interpretable alone. `depleted` is stated beside `standing` even
    /// though this engine derives the one from the other, because a consumer must not have to
    /// know which derivation the engine uses.
    fn write_metrics_territory(
        &self,
        sink: &mut dyn Write,
        territory: Territory,
    ) -> io::Result<()> {
        let counts = self.food_counts(territory);
        let standing = counts[0] + counts[1] + counts[2];
        write!(
            sink,
            "\"{territory}\":{{\"standing\":{standing},\"low\":{},\"medium\":{},\"high\":{},\"capacity\":{},\"depleted\":{}}}",
            counts[0],
            counts[1],
            counts[2],
            self.config.density.resources_per_territory(),
            standing == 0
        )
    }

    /// The run record, `SPEC-MOK-006` rule 8: the twelve figures the summary line carries,
    /// plus the five facts the cumulative counters hold and no text line states.
    fn write_run_record(&self, sink: &mut dyn Write, summary: &RunSummary) -> io::Result<()> {
        write!(
            sink,
            "{{\"record\":\"run\",\"reason\":\"{}\",\"ticks\":{},\"survivors\":{},\"deaths\":{},\"crossings\":{}",
            summary.reason, summary.ticks, summary.survivors, summary.deaths, self.crossings
        )?;

        write!(sink, ",\"consumed\":{{")?;
        for (position, class) in FoodClass::ALL.into_iter().enumerate() {
            if position > 0 {
                write!(sink, ",")?;
            }
            write!(sink, "\"{class}\":{}", self.consumed[class.index()])?;
        }
        write!(sink, "}},\"regenerated\":{}", self.regenerated)?;

        // Rule 8.5: the two skip reasons stay distinguished. Collapsing them would lose the
        // difference between a world at capacity and a world that can never restock.
        write!(sink, ",\"regeneration_skipped\":{{")?;
        for (position, reason) in RegenerationSkipReason::ALL.into_iter().enumerate() {
            if position > 0 {
                write!(sink, ",")?;
            }
            write!(
                sink,
                "\"{reason}\":{}",
                self.regeneration_skipped[reason.index()]
            )?;
        }
        write!(sink, "}}")?;

        // Rule 8.3: `final` carries the per-territory figures the summary line carries, so
        // the summary line is reconstructible from this record alone.
        write!(
            sink,
            ",\"final\":{{\"territories\":{{\"A\":{{\"population\":{},\"low\":{},\"medium\":{},\"high\":{}}},\"B\":{{\"population\":{},\"low\":{},\"medium\":{},\"high\":{}}}}}}}",
            summary.territory_a,
            summary.food_a[0],
            summary.food_a[1],
            summary.food_a[2],
            summary.territory_b,
            summary.food_b[0],
            summary.food_b[1],
            summary.food_b[2]
        )?;

        // Rule 8.4: one entry per Mokiterion the run created, living or dead, in ascending
        // identifier order. **The order is imposed here rather than inherited** from the
        // roster's traversal, because an ordering that came from a collection's iteration
        // order is a determinism defect waiting to manifest, and rule 11.5 names this as the
        // one place a traversal order would otherwise have been visible.
        let mut roster: Vec<&Mokiterion> = self.agents.iter().collect();
        roster.sort_by(|left, right| left.id.cmp(&right.id));
        write!(sink, ",\"agents\":[")?;
        for (position, agent) in roster.into_iter().enumerate() {
            if position > 0 {
                write!(sink, ",")?;
            }
            write!(
                sink,
                "{{\"id\":\"{}\",\"name\":\"{}\",\"territory\":\"{}\",\"died_at\":",
                agent.id,
                agent.name,
                agent.position.territory()
            )?;
            // Rule 4.4: `null`, never `0` and never omitted. Tick `0` is a legitimate death
            // tick, so a sentinel would be indistinguishable from a measurement.
            match agent.died_at {
                Some(tick) => write!(sink, "{tick}")?,
                None => write!(sink, "null")?,
            }
            write!(sink, "}}")?;
        }
        writeln!(sink, "]}}")
    }

    /// The shared entropy stream's state, for `VER-MOK-012` oracle 4.
    ///
    /// An owned `u64` and nothing else: `SPEC-MOK-002` rule 6 forbids a reference into the
    /// entropy state in *any* build configuration, so a test build gets a copy too.
    /// `#[cfg(test)]` in the merged tree, and named only by tests in this module — the
    /// public tier cannot reach it, which is the point.
    #[cfg(test)]
    fn entropy_state(&self) -> u64 {
        self.entropy.state
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

// ---------------------------------------------------------------------------------------
// The structured record stream, `SPEC-MOK-006`.
//
// Every function below writes one complete line, or part of one, and retains nothing. None
// of them takes `&mut self`, none draws against the entropy stream, and none is reachable
// from a run that was given no sink — `Sinks::records` returns `None` and each caller
// returns before reaching here, which is rule 1.1.
//
// The records are written by `write!`, not by a serializer, and that is `ARCH-MOK-001`'s
// standing decision rather than an omission: the engine's dependency table is empty and
// `SPEC-MOK-006` rule 12.4 keeps it empty. What makes hand-writing safe here is rule 3.3's
// closed value alphabet — `A-Z a-z 0-9 _ . - + : ; >` and nothing else, in particular no
// quotation mark, no backslash and no code point below U+0020. **There is therefore no
// escaping function, and the absence is deliberate.** A value outside that alphabet could
// not be written correctly by these functions, so any change that admits one must add
// escaping in the same act; `tests/records.rs` enumerates the alphabet exhaustively so that
// such a change fails a test rather than corrupting a stream.
// ---------------------------------------------------------------------------------------

/// The header record, `SPEC-MOK-006` rule 5: one per run, before the first tick.
///
/// Rule 5.4 states the configuration as resolved, never as given, so a stream carries the
/// run that happened rather than the arguments that asked for it. Rule 5.5 keeps the sink's
/// own path out of it, in every form — a stream is a description of a run, and where the
/// stream was put is not part of that run.
fn write_header_record(sink: &mut dyn Write, config: &Config) -> io::Result<()> {
    writeln!(
        sink,
        "{{\"record\":\"header\",\"schema\":{RECORD_SCHEMA_VERSION},\"engine\":\"{}\",\"config\":{{\"seed\":{},\"ticks\":{},\"policy\":\"{}\",\"density\":\"{}\",\"trace_actions\":{}}}}}",
        env!("CARGO_PKG_VERSION"),
        config.seed,
        config.tick_limit,
        config.policy,
        // Rule 4.3: a string, holding the same two-decimal rendering the text stream and the
        // help text carry. A density is a decimal quantity and rule 4.1 admits no decimal
        // number, so the choice is between a string and a pair of integers, and the string is
        // the figure a reader already recognizes.
        config.density,
        config.trace_actions
    )
}

/// One event record, `SPEC-MOK-006` rule 6: one per emitted text event line, in the same
/// order, with the same identity, the same event type and the same values.
///
/// The record is written from the same [`Event`] the text line is written from, at the same
/// point, which is what makes `REQ-MOK-042`'s correspondence structural rather than
/// maintained.
fn write_event_record(sink: &mut dyn Write, event: &Event) -> io::Result<()> {
    write!(
        sink,
        "{{\"record\":\"event\",\"tick\":{},\"subject\":\"{}\",\"event\":\"{}\",\"result\":{{",
        event.tick,
        event.subject,
        event.detail.event_type()
    )?;
    write_event_result(sink, &event.detail)?;
    writeln!(sink, "}}}}")
}

/// The `result` object's fields for one event, in the order the text line states them.
///
/// This match mirrors [`EventDetail`]'s `Display` field for field. It is exhaustive by
/// construction, so a thirteenth variant fails to compile here rather than reaching a stream
/// without its fields — rule 6.3's guarantee that every field of every event kind appears.
/// Rule 6.7's `"result":{}` for a fieldless event needs no arm: no variant is fieldless
/// today, and one added later would write nothing between the braces the caller wrote.
fn write_event_result(sink: &mut dyn Write, detail: &EventDetail) -> io::Result<()> {
    match detail {
        EventDetail::WorldInitialized {
            width,
            height,
            territories,
        } => write!(
            sink,
            "\"width\":{width},\"height\":{height},\"territories\":{territories}"
        ),
        EventDetail::FoodInitialized {
            class,
            position,
            territory,
        } => {
            write!(sink, "\"class\":\"{class}\",\"position\":")?;
            write_coordinate(sink, *position)?;
            write!(sink, ",\"territory\":\"{territory}\"")
        }
        EventDetail::AgentInitialized {
            name,
            position,
            territory,
            health,
            satiety,
            energy,
            fear,
            waste_tolerance,
        } => {
            write!(sink, "\"name\":\"{name}\",\"position\":")?;
            write_coordinate(sink, *position)?;
            write!(
                sink,
                ",\"territory\":\"{territory}\",\"health\":{health},\"satiety\":{satiety},\"energy\":{energy},\"fear\":{fear},\"waste_tolerance\":{waste_tolerance}"
            )
        }
        EventDetail::DecisionSourceSelected { source } => write!(sink, "\"source\":\"{source}\""),
        EventDetail::SurvivalChanged {
            health,
            satiety,
            energy,
            fear,
        } => {
            write!(sink, "\"health\":")?;
            write_transition(sink, *health)?;
            write!(sink, ",\"satiety\":")?;
            write_transition(sink, *satiety)?;
            write!(sink, ",\"energy\":")?;
            write_transition(sink, *energy)?;
            write!(sink, ",\"fear\":")?;
            write_transition(sink, *fear)
        }
        EventDetail::AgentDied { health } => write!(sink, "\"health\":{health}"),
        EventDetail::FoodConsumed {
            food,
            class,
            satiety,
            energy,
        } => {
            write!(
                sink,
                "\"food\":\"{food}\",\"class\":\"{class}\",\"satiety\":"
            )?;
            write_transition(sink, *satiety)?;
            write!(sink, ",\"energy\":")?;
            write_transition(sink, *energy)
        }
        EventDetail::FoodRegenerated {
            food,
            class,
            position,
        } => {
            write!(
                sink,
                "\"food\":\"{food}\",\"class\":\"{class}\",\"position\":"
            )?;
            write_coordinate(sink, *position)
        }
        EventDetail::FoodRegenerationSkipped { reason, count } => {
            write!(sink, "\"reason\":\"{reason}\",\"count\":{count}")
        }
        EventDetail::TerritoryCrossed { from, to } => {
            write!(sink, "\"from\":\"{from}\",\"to\":\"{to}\"")
        }
        // Rule 22's resolution, for `attack` and `fight` alike. `damage` is stated beside the
        // transition rather than left to be recovered from it, because saturation at zero makes
        // the subtraction lossy — which is rule 4.2's sum-and-count reasoning applied to a
        // transition rather than to an average.
        EventDetail::AttackResolved {
            target,
            damage,
            target_health,
            striker_energy,
            target_died,
        } => {
            write!(
                sink,
                "\"target\":\"{target}\",\"damage\":{damage},\"target_health\":"
            )?;
            write_transition(sink, *target_health)?;
            write!(sink, ",\"striker_energy\":")?;
            write_transition(sink, *striker_energy)?;
            // The text stream's own `yes`/`no`, as a string, and not the `bool` the engine
            // holds. This follows `status` below, which is `"accepted"` here rather than `true`
            // for the same reason: rule 6.3 makes every scalar in this stream the text
            // stream's own rendering of the same value, and a boolean here would be the only
            // scalar in the stream that is not. Rule 4.5's two booleans cost nothing to that
            // rule because neither `config.trace_actions` nor a territory's `depleted` appears
            // in the text stream at all — this is the first `bool` on a field that does, so it
            // is the first place the two rules could disagree, and rule 6.3 wins.
            write!(
                sink,
                ",\"target_died\":\"{}\"",
                if *target_died { "yes" } else { "no" }
            )
        }
        // Rule 23's resolution. `increase` is the amount *applied*, so `0` against a target
        // already at `ATTRIBUTE_MAX`; the transition states the same fact from the other side
        // and neither is derived from the other here.
        EventDetail::ThreatResolved {
            target,
            increase,
            target_fear,
        } => {
            write!(
                sink,
                "\"target\":\"{target}\",\"increase\":{increase},\"target_fear\":"
            )?;
            write_transition(sink, *target_fear)
        }
        // Rule 24's resolution. `transferred` and `discarded` are both stated because their sum
        // is what the forfeit cost and neither alone shows where conservation failed.
        EventDetail::SurrenderResolved {
            recipient,
            transferred,
            discarded,
            subject_satiety,
            recipient_satiety,
        } => {
            write!(
                sink,
                "\"recipient\":\"{recipient}\",\"transferred\":{transferred},\"discarded\":{discarded},\"subject_satiety\":"
            )?;
            write_transition(sink, *subject_satiety)?;
            write!(sink, ",\"recipient_satiety\":")?;
            write_transition(sink, *recipient_satiety)
        }
        EventDetail::SimulationEnded { reason } => write!(sink, "\"reason\":\"{reason}\""),
        EventDetail::ActionTrace {
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
            write!(sink, "\"proposal\":")?;
            write_proposal(sink, proposal)?;
            // Rule 6.4: the text line's `status:accepted` is the string `"accepted"` here, not
            // `true`. Rule 4.5 admits exactly two booleans and this is not one of them, because
            // a verdict that gains a third outcome must not have to change a field's type.
            //
            // `detail` is the engine's own ground, verbatim. It is on the alphabet because
            // `SPEC-MOK-001` composes it from the same words and separators every other value
            // uses, which is why it needs no treatment a class name does not.
            write!(
                sink,
                ",\"status\":\"{}\",\"detail\":\"{detail}\",\"position\":",
                if *accepted { "accepted" } else { "rejected" }
            )?;
            write_coordinate(sink, *position)?;
            write!(
                sink,
                ",\"territory\":\"{territory}\",\"health\":{health},\"satiety\":{satiety},\"energy\":{energy},\"fear\":{fear}"
            )?;
            // Rule 25's record, always present and empty where nothing was suffered. The text
            // line omits it when empty and this stream does not, which rule 4.4 requires: the
            // absence of an attack is an empty record rather than a missing fact, and a
            // consumer that had to distinguish "no attacks" from "field not written" would be
            // reading the writer's convenience instead of the world. Rule 6.6's reconstruction
            // carries the mapping in the one direction it runs — an empty array renders no
            // `suffered` clause on the text line.
            write!(sink, ",\"suffered\":[")?;
            for (index, (attacker, damage)) in suffered.iter().enumerate() {
                if index > 0 {
                    write!(sink, ",")?;
                }
                write!(sink, "{{\"attacker\":\"{attacker}\",\"damage\":{damage}}}")?;
            }
            write!(sink, "]")
        }
    }
}

/// Rule 6.5's first composite shape: a coordinate, `x:y` in the text stream.
///
/// An object rather than the text's colon-joined pair, because a consumer that wants one axis
/// should not have to split a string, and rule 6.5 requires exactly this.
fn write_coordinate(sink: &mut dyn Write, position: Coordinate) -> io::Result<()> {
    write!(sink, "{{\"x\":{},\"y\":{}}}", position.x, position.y)
}

/// Rule 6.5's second composite shape: a before-and-after pair, `before->after` in the text
/// stream. `from` and `to` are the two field names rule 6.5 fixes.
///
/// Rule 4.2 forbids the delta this pair implies. Both endpoints are stated and neither is
/// subtracted from the other here, so a consumer that wants the difference computes it and
/// owns it.
fn write_transition(sink: &mut dyn Write, transition: (u8, u8)) -> io::Result<()> {
    write!(
        sink,
        "{{\"from\":{},\"to\":{}}}",
        transition.0, transition.1
    )
}

/// Rule 6.5's third composite shape: a proposed action, `wait`, `sleep`, `eat:<food>` or
/// `move:<direction>` in the text stream.
///
/// The action word under `action`, then the one further value the action carries where it
/// carries one, under the name that value has. Rule 6.6's reconstruction walk reverses
/// exactly this: the action word, then a colon and the remaining value if there is one.
fn write_proposal(sink: &mut dyn Write, proposal: &Action) -> io::Result<()> {
    match proposal {
        Action::Wait => write!(sink, "{{\"action\":\"wait\"}}"),
        Action::Sleep => write!(sink, "{{\"action\":\"sleep\"}}"),
        Action::Eat { food_id } => write!(sink, "{{\"action\":\"eat\",\"food\":\"{food_id}\"}}"),
        Action::Move { direction } => {
            write!(
                sink,
                "{{\"action\":\"move\",\"direction\":\"{direction}\"}}"
            )
        }
        // The seven targeted verbs of `CAP-MOK-010`, each carrying its one further value under
        // the name that value has, exactly as `eat` carries `food`. The text stream renders the
        // target as a field beside `proposal` rather than inside it, so that the verb and the
        // target cannot disagree on a line a parser reads positionally; here they cannot
        // disagree for a stronger reason — both are read from the same `Action` in one arm.
        Action::Attack { target } => {
            write!(sink, "{{\"action\":\"attack\",\"target\":\"{target}\"}}")
        }
        Action::Threaten { target } => {
            write!(sink, "{{\"action\":\"threaten\",\"target\":\"{target}\"}}")
        }
        Action::Fight { target } => {
            write!(sink, "{{\"action\":\"fight\",\"target\":\"{target}\"}}")
        }
        Action::Retreat { target } => {
            write!(sink, "{{\"action\":\"retreat\",\"target\":\"{target}\"}}")
        }
        Action::Surrender { target } => {
            write!(sink, "{{\"action\":\"surrender\",\"target\":\"{target}\"}}")
        }
        Action::Approach { target } => {
            write!(sink, "{{\"action\":\"approach\",\"target\":\"{target}\"}}")
        }
        Action::Avoid { target } => {
            write!(sink, "{{\"action\":\"avoid\",\"target\":\"{target}\"}}")
        }
    }
}

/// One attribute's `{"sum":…,"<extremum>":…}` object inside a metrics record, rule 7.5.
///
/// The extremum is `null` over an empty living population, under rule 4.4. A sentinel is not
/// available: `0` is a legitimate health, satiety and energy for a living Mokiterion, and `0`
/// is the ordinary fear of an unthreatened one, so any in-band value would be
/// indistinguishable from a measurement. Rule 4.2 forbids the mean the sum would otherwise
/// invite, which is why the sum appears beside `living` and not divided by it.
fn write_attribute(
    sink: &mut dyn Write,
    name: &str,
    sum: u64,
    extremum_name: &str,
    extremum: Option<u8>,
) -> io::Result<()> {
    write!(sink, ",\"{name}\":{{\"sum\":{sum},\"{extremum_name}\":")?;
    match extremum {
        Some(value) => write!(sink, "{value}")?,
        None => write!(sink, "null")?,
    }
    write!(sink, "}}")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::accounting::{ExchangeUsage, PerTokenPrices, RunAccount, RunEnd, RunRecord};
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
            spend_ceiling: None,
            prices: None,
        }
    }

    /// The stream pair over a text stream alone.
    ///
    /// Every test written before `SPEC-MOK-006` asserts about text and about nothing else, so
    /// `None` here leaves each of them asserting exactly what it asserted before: no sink, and
    /// by rule 1.1 no sink-related code path. The tests that do exercise a sink build the pair
    /// themselves, so that a test which passes a sink says so at the call.
    fn text_only<W: Write>(text: &mut W) -> Sinks<'_, '_, W> {
        Sinks {
            text,
            records: None,
        }
    }

    fn reference_config(seed: u64, tick_limit: u64, trace_actions: bool) -> Config {
        Config {
            seed,
            tick_limit,
            policy: Policy::Reference,
            density: Density::DEFAULT,
            trace_actions,
            spend_ceiling: None,
            prices: None,
        }
    }

    fn config_at(seed: u64, tick_limit: u64, density: &str) -> Config {
        Config {
            density: Density::parse(density).unwrap(),
            ..reference_config(seed, tick_limit, false)
        }
    }

    /// The highest satiety at which `SPEC-MOK-001` rule 5's corrected non-waste condition still
    /// admits a resource of this class: `100 - R + R * R / 100`, which is `87`, `79` and `75`.
    ///
    /// Computed from the food table and the allowance rather than written as three literals, so
    /// that a scenario reading "one point above the level that still fits" stays true if either
    /// moves. The three values themselves are asserted as literals against the specification in
    /// `the_corrected_non_waste_condition_admits_the_specified_boundaries`, which is where a
    /// drift in this helper's own arithmetic is caught.
    fn highest_admitted_satiety(class: FoodClass) -> u8 {
        let restored = u16::from(class.restoration().0);
        let allowance = restored * restored / 100;
        u8::try_from(u16::from(ATTRIBUTE_MAX) - restored + allowance).unwrap()
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
                &mut text_only(&mut output),
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
                &mut text_only(&mut output),
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
            .apply_action(&mut text_only(&mut Vec::new()), 0, &Action::Sleep)
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

        let first = simulation
            .apply_action(&mut text_only(&mut output), 0, &action)
            .unwrap();
        let second_before = simulation.agents[1].clone();
        let second = simulation
            .apply_action(&mut text_only(&mut output), 1, &action)
            .unwrap();

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
        simulation
            .apply_survival(&mut text_only(&mut output), 0, false)
            .unwrap();

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
            .regenerate_food(&mut text_only(&mut output), Territory::A)
            .unwrap();
        simulation
            .regenerate_food(&mut text_only(&mut output), Territory::B)
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
            .regenerate_food(&mut text_only(&mut output), Territory::A)
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
            .run_with_source(&mut text_only(&mut output), &mut source)
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
            .regenerate_food(&mut text_only(&mut output), Territory::A)
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
                .regenerate_food(&mut text_only(&mut io::sink()), Territory::A)
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
        // non-waste rule declines it. Derived from the food table and the allowance rather than
        // from a threshold constant, because rule 5 states no threshold. The level was `51`
        // until `REQ-MOK-060` and is `76` now, because the condition grants a high-class
        // resource `50 * 50 / 100 = 25` satiety of allowance above the maximum.
        simulation.agents[0].satiety = highest_admitted_satiety(FoodClass::High) + 1;
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
        simulation.agents[0].satiety = highest_admitted_satiety(FoodClass::High) + 1;

        // Both perceived resources would be clipped beyond the allowance, so there is nothing
        // worth walking to.
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

    /// `REQ-MOK-060`: rule 5's corrected non-waste condition, at every boundary the specification
    /// states, for eating and for approaching alike.
    ///
    /// The allowance and the three satieties are written as literals transcribed from
    /// `SPEC-MOK-001`'s amendment of 2026-08-21 rather than computed, because the purpose here is to
    /// hold the engine to the specification's numbers. `highest_admitted_satiety`, which the
    /// scenario tests derive their satieties from, is checked against the same literals in the same
    /// loop, so a drift in its arithmetic cannot silently move those scenarios.
    ///
    /// Each class is asserted on both sides of its boundary and in both of rule 5's cases. The
    /// inclusive side is the one that matters: `REQ-MOK-060`'s whole mechanism is that a resource
    /// wasting exactly its own allowance is taken rather than left standing, and a condition
    /// written with `<` instead of `<=` would pass every other check in this suite.
    #[test]
    fn the_corrected_non_waste_condition_admits_the_specified_boundaries() {
        let underfoot = Coordinate { x: 50, y: 30 };
        let eat = Action::Eat {
            food_id: "F0001".into(),
        };

        for (class, allowance, highest) in [
            (FoodClass::Low, 2u16, 87u8),
            (FoodClass::Medium, 9, 79),
            (FoodClass::High, 25, 75),
        ] {
            let restored = u16::from(class.restoration().0);
            assert_eq!(
                restored * restored / 100,
                allowance,
                "{class} class: the allowance R * R / 100 is not the specified {allowance}"
            );
            assert_eq!(
                highest_admitted_satiety(class),
                highest,
                "{class} class: the highest admitted satiety is not the specified {highest}"
            );
            // The allowance is exactly what is wasted at the boundary, which is the sense in
            // which the condition is stated on the waste and not on the satiety.
            assert_eq!(
                u16::from(highest) + restored - u16::from(ATTRIBUTE_MAX),
                allowance
            );

            let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
            simulation.tick = 1;
            simulation.agents[0].position = underfoot;
            simulation.foods = vec![Food {
                id: "F0001".into(),
                position: underfoot,
                class,
            }];

            // Case 1, eating: inclusive at the boundary, excluded one point above it.
            simulation.agents[0].satiety = highest;
            let (admitted, draws) = decide_once(&simulation, 0);
            assert_eq!(
                admitted, eat,
                "{class} class at satiety {highest} was declined, wasting exactly the allowance \
                 of {allowance}, which rule 5's condition admits inclusively"
            );
            assert_eq!(draws, 0, "eating must not consume entropy");

            simulation.agents[0].satiety = highest + 1;
            let (declined, draws) = decide_once(&simulation, 0);
            assert!(
                matches!(declined, Action::Move { .. }),
                "{class} class at satiety {} wastes {} against an allowance of {allowance} and \
                 must be declined, got {declined}",
                highest + 1,
                allowance + 1
            );
            assert_eq!(
                draws, 1,
                "with nothing worth eating the step must be a search"
            );

            // Case 3, approaching: the same test on the same two satieties, one cell away. Rule
            // 5 screens both cases by one condition, so a correction applied to eating alone
            // would restore the two-cell oscillation the condition exists to prevent.
            simulation.agents[0].position = Coordinate { x: 49, y: 30 };
            simulation.agents[0].satiety = highest;
            let (approach, draws) = decide_once(&simulation, 0);
            assert_eq!(
                approach,
                Action::Move {
                    direction: Direction::East
                },
                "{class} class at satiety {highest} one cell west was not approached"
            );
            assert_eq!(draws, 0, "approaching must not consume entropy");

            simulation.agents[0].satiety = highest + 1;
            let (unapproached, draws) = decide_once(&simulation, 0);
            assert_eq!(
                draws,
                1,
                "{class} class at satiety {} must not be approached, got {unapproached}",
                highest + 1
            );
        }

        // The first clause still stands on its own: a low-class resource at satiety 80 fits
        // outright, needing none of the allowance, while a high-class one at the same satiety
        // exceeds even the largest allowance in the table.
        let mut simulation = Simulation::new(reference_config(0, 1, false)).unwrap();
        simulation.tick = 1;
        simulation.agents[0].position = underfoot;
        simulation.agents[0].satiety = 80;
        for (class, eaten) in [(FoodClass::Low, true), (FoodClass::High, false)] {
            simulation.foods = vec![Food {
                id: "F0001".into(),
                position: underfoot,
                class,
            }];
            let (action, _) = decide_once(&simulation, 0);
            assert_eq!(
                action == eat,
                eaten,
                "{class} class at satiety 80 was decided the wrong way, got {action}"
            );
        }
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
                spend_ceiling: None,
                prices: None,
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

    /// The densities `WO-MOK-019`'s evidence capture sweeps: the default, the `1.50%` sweep
    /// `VER-MOK-002` declares, and `0.15%`.
    ///
    /// `VER-MOK-012` requires the default and `1.50%`. `0.15%` is added because each density is a
    /// distinct world rather than the same world with more food — initialization performs a
    /// different number of coordinate draws — so a draw taken per resource would show at a scarce
    /// density and not at a generous one. The byte captures sweep the same three.
    const SWEPT_DENSITIES: [&str; 3] = ["0.15", "0.75", "1.50"];

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
            spend_ceiling: None,
            prices: None,
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
                        spend_ceiling: None,
                        prices: None,
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
    /// - twenty-one satiety values straddling every clipping boundary the food table produces —
    ///   `87` for the low class, `79` for the medium and `75` for the high, as `REQ-MOK-060`
    ///   corrected them on 2026-08-21 — with one value below, at and above each, plus both ends of
    ///   the range and one value between the boundaries. The three triples the *uncorrected*
    ///   condition produced, `84..=86`, `69..=71` and `49..=51`, are retained rather than replaced:
    ///   they are where a regression to the omitted allowance would first show, and this test is
    ///   the one that would have to keep holding through it;
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
        const SATIETIES: [u8; 21] = [
            0, 49, 50, 51, 60, 69, 70, 71, 74, 75, 76, 78, 79, 80, 84, 85, 86, 87, 88, 99, 100,
        ];

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
        assert_eq!(cases, 4_536, "the enumerated situation set changed size");
    }

    /// `REQ-MOK-033`: a trait difference alone changes the proposal, in both of rule 19's worked
    /// cases as amended on 2026-08-19 and again on 2026-08-21.
    ///
    /// The medium-class case is unmoved by `REQ-MOK-060` and is the interior one: at satiety `80` a
    /// medium-class resource restores `30` and wastes `10`. Rule 5's own allowance is
    /// `30 * 30 / 100 = 9`, which does not admit it, so the tolerance still decides — at `T = 34`
    /// (`1020 / 100 = 10`) and not at `T = 33` (`990 / 100 = 9`). **The pair either side of `34` is
    /// what pins the division as truncating rather than rounding**, and neither value is near the
    /// range's ends.
    ///
    /// **The high-class pair at satiety `70` was the second case until 2026-08-21 and is not a pair
    /// any more.** Rule 5's corrected allowance for that class is `50 * 50 / 100 = 25`, which admits
    /// the waste of `20` outright, so `39` and `40` both eat and neither the tolerance nor the
    /// truncation is observable there. The replacement is the low-class case at satiety `88`, where
    /// the waste is `3`, rule 5's allowance is `15 * 15 / 100 = 2`, and the tolerance decides at
    /// `T = 20` (`300 / 100 = 3`) against `T = 19` (`285 / 100 = 2`). That pair pins the truncation
    /// a second time, at the other end of the food table, and `19` is where rounding would have
    /// admitted it.
    ///
    /// Two effects are asserted after the loop because each is a claim about the whole reachable
    /// range rather than about a pair. A high-class resource at satiety `80` is declined at every
    /// tolerance, which is what the 2026-08-19 narrowing was made to produce and what `REQ-MOK-060`
    /// had to leave standing; the same resource at satiety `70` is now eaten at every tolerance and
    /// by the reference source alike, which is what `REQ-MOK-060` moved.
    #[test]
    fn a_trait_difference_alone_decides_whether_a_clipped_resource_is_eaten() {
        let underfoot = Coordinate { x: 60, y: 30 };
        let expected = Action::Eat {
            food_id: "F0001".into(),
        };

        for (class, satiety, admits, declines) in [
            (FoodClass::Medium, 80u8, 34u8, 33u8),
            (FoodClass::Low, 88, 20, 19),
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

        // The narrowing's intended effect, which REQ-MOK-060 had to leave standing: a high-class
        // resource at satiety 80 wastes 30, and no tolerance the amended range can produce admits
        // it, because neither `40 * 50 / 100 = 20` nor rule 5's own `50 * 50 / 100 = 25` reaches 30.
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

        // REQ-MOK-060's own effect at the same place: at satiety 70 the waste of 20 is inside rule
        // 5's allowance of 25, so the resource is eaten at every tolerance and by the reference
        // source too. This is the entry SPEC-MOK-001's *Acceptance examples* asserted the opposite
        // of until 2026-08-21, and asserting it across the whole range is what records that the
        // trait is masked here rather than merely agreeing at the two values a pair would name.
        simulation.agents[0].satiety = 70;
        for tolerance in 0..=WASTE_TOLERANCE_MAX {
            simulation.agents[0].waste_tolerance = tolerance;
            let (proposal, draws) = decide_individual_once(&simulation, 0);
            assert_eq!(
                proposal, expected,
                "tolerance {tolerance} declined a high-class resource at satiety 70, wasting 20, \
                 which rule 5's corrected allowance of 25 admits without any tolerance at all"
            );
            assert_eq!(draws, 0, "eating consumes no entropy");
        }
        let (reference, draws) = decide_once(&simulation, 0);
        assert_eq!(
            reference, expected,
            "the reference source declined a high-class resource at satiety 70, which is the \
             decision REQ-MOK-060 corrected"
        );
        assert_eq!(draws, 0, "eating consumes no entropy");
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
                .run_tick(&mut text_only(&mut output), &mut IndividualDecisionSource)
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
                .run_tick(&mut text_only(&mut output), &mut IndividualDecisionSource)
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
            simulation
                .apply_survival(&mut text_only(&mut output), 0, true)
                .unwrap();
            assert!(simulation.agents[0].fear <= ATTRIBUTE_MAX);
        }
        assert_eq!(simulation.agents[0].fear, ATTRIBUTE_MAX);

        // Twenty decrements of five reach the lower bound; the twenty-first must hold there
        // rather than wrap to 251.
        for _ in 0..21 {
            simulation
                .apply_survival(&mut text_only(&mut output), 0, false)
                .unwrap();
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
            .run_tick(&mut text_only(&mut output), &mut IndividualDecisionSource)
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

        simulation
            .apply_survival(&mut text_only(&mut output), 0, true)
            .unwrap();
        assert!(!simulation.agents[0].alive);
        assert!(
            String::from_utf8(output)
                .unwrap()
                .contains("event=agent_died")
        );

        simulation.tick = 2;
        let mut output = Vec::new();
        simulation
            .run_tick(&mut text_only(&mut output), &mut IndividualDecisionSource)
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

    // ---- WO-MOK-016: contact, conflict and society ----------------------------------------

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
            let result = simulation
                .apply_action(&mut text_only(&mut output), 0, &action)
                .unwrap();

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
                    &mut text_only(&mut output),
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
                    &mut text_only(&mut Vec::new()),
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
                &mut text_only(&mut output),
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
    /// maximum is threatened *validly* and reports an increase of `0`: `REQ-MOK-055` requires
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
                    &mut text_only(&mut output),
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
                    &mut text_only(&mut output),
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

                let result = simulation
                    .apply_action(&mut text_only(&mut output), 0, &action)
                    .unwrap();

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
                &mut text_only(&mut Vec::new()),
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
                &mut text_only(&mut Vec::new()),
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
                &mut text_only(&mut Vec::new()),
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
                &mut text_only(&mut Vec::new()),
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
                &mut text_only(&mut Vec::new()),
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
                &mut text_only(&mut output),
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
                    &mut text_only(&mut Vec::new()),
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
            .run_tick(&mut text_only(&mut output), &mut SocialDecisionSource)
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
            simulation.advance_tick(None).unwrap();
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
                simulation.advance_tick(None).unwrap();
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
            simulation.advance_tick(None).unwrap();
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
                simulation.advance_tick(None).unwrap();
            }
        }
        // The comparison is worth nothing if its condition never held.
        assert!(compared > 0, "no opportunity without company was compared");
    }

    /// Rule 25 under the three sources that predate it: the window never opens, so
    /// `CAP-MOK-010`'s byte-identity has nothing here to preserve it against.
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

    // -----------------------------------------------------------------------------------
    // The structured record stream, `SPEC-MOK-006`.
    //
    // These are the internal tier: they assert about the private record writers, the private
    // cumulative counters and the entropy state, none of which the public interface exposes
    // and none of which rule 12.2 or 12.3 permits it to. `tests/records.rs` asserts the same
    // stream from outside, over the built binary, and the two tiers are deliberately not
    // redundant — this one reaches the shapes a run cannot be made to produce.
    // -----------------------------------------------------------------------------------

    /// Rule 6.2 and rule 6.5: the exact bytes of one event record, for every event kind.
    ///
    /// All fifteen kinds, and two of the rows are shapes no shipped decision source produces —
    /// a rejected proposal and a `depleted` regeneration skip. A shape only a contrived state
    /// reaches is exactly the shape a capture cannot cover, so it is covered here instead.
    ///
    /// The rows are asserted against [`EventType::ALL`], once each, so a sixteenth event kind
    /// fails this test as well as failing to compile in `write_event_result`.
    #[test]
    fn every_event_kind_has_its_exact_record_shape() {
        let cases: [(u64, &str, EventDetail, &str); 15] = [
            (
                0,
                "world",
                EventDetail::WorldInitialized {
                    width: 128,
                    height: 128,
                    territories: 2,
                },
                "{\"record\":\"event\",\"tick\":0,\"subject\":\"world\",\"event\":\"world_initialized\",\"result\":{\"width\":128,\"height\":128,\"territories\":2}}\n",
            ),
            (
                0,
                "F0001",
                EventDetail::FoodInitialized {
                    class: FoodClass::High,
                    position: Coordinate { x: 41, y: 63 },
                    territory: Territory::A,
                },
                "{\"record\":\"event\",\"tick\":0,\"subject\":\"F0001\",\"event\":\"food_initialized\",\"result\":{\"class\":\"high\",\"position\":{\"x\":41,\"y\":63},\"territory\":\"A\"}}\n",
            ),
            (
                0,
                "M01",
                EventDetail::AgentInitialized {
                    name: "Zug".to_string(),
                    position: Coordinate { x: 89, y: 34 },
                    territory: Territory::A,
                    health: 100,
                    satiety: 100,
                    energy: 100,
                    fear: 0,
                    waste_tolerance: 6,
                },
                "{\"record\":\"event\",\"tick\":0,\"subject\":\"M01\",\"event\":\"agent_initialized\",\"result\":{\"name\":\"Zug\",\"position\":{\"x\":89,\"y\":34},\"territory\":\"A\",\"health\":100,\"satiety\":100,\"energy\":100,\"fear\":0,\"waste_tolerance\":6}}\n",
            ),
            (
                0,
                "world",
                EventDetail::DecisionSourceSelected {
                    source: "individual".to_string(),
                },
                "{\"record\":\"event\",\"tick\":0,\"subject\":\"world\",\"event\":\"decision_source_selected\",\"result\":{\"source\":\"individual\"}}\n",
            ),
            (
                3,
                "M04",
                EventDetail::SurvivalChanged {
                    health: (100, 99),
                    satiety: (97, 96),
                    energy: (97, 96),
                    fear: (0, 10),
                },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"M04\",\"event\":\"survival_changed\",\"result\":{\"health\":{\"from\":100,\"to\":99},\"satiety\":{\"from\":97,\"to\":96},\"energy\":{\"from\":97,\"to\":96},\"fear\":{\"from\":0,\"to\":10}}}\n",
            ),
            (
                3,
                "M04",
                EventDetail::AgentDied { health: 0 },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"M04\",\"event\":\"agent_died\",\"result\":{\"health\":0}}\n",
            ),
            (
                3,
                "M04",
                EventDetail::FoodConsumed {
                    food: "F0002".to_string(),
                    class: FoodClass::Medium,
                    satiety: (60, 90),
                    energy: (70, 80),
                },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"M04\",\"event\":\"food_consumed\",\"result\":{\"food\":\"F0002\",\"class\":\"medium\",\"satiety\":{\"from\":60,\"to\":90},\"energy\":{\"from\":70,\"to\":80}}}\n",
            ),
            (
                3,
                "A",
                EventDetail::FoodRegenerated {
                    food: "F0123".to_string(),
                    class: FoodClass::Low,
                    position: Coordinate { x: 7, y: 8 },
                },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"A\",\"event\":\"food_regenerated\",\"result\":{\"food\":\"F0123\",\"class\":\"low\",\"position\":{\"x\":7,\"y\":8}}}\n",
            ),
            (
                3,
                "B",
                EventDetail::FoodRegenerationSkipped {
                    reason: RegenerationSkipReason::Depleted,
                    count: 0,
                },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"B\",\"event\":\"food_regeneration_skipped\",\"result\":{\"reason\":\"depleted\",\"count\":0}}\n",
            ),
            (
                3,
                "M04",
                EventDetail::TerritoryCrossed {
                    from: Territory::A,
                    to: Territory::B,
                },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"M04\",\"event\":\"territory_crossed\",\"result\":{\"from\":\"A\",\"to\":\"B\"}}\n",
            ),
            (
                3,
                "M04",
                EventDetail::AttackResolved {
                    target: "M07".to_string(),
                    damage: 12,
                    target_health: (100, 88),
                    striker_energy: (60, 52),
                    target_died: false,
                },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"M04\",\"event\":\"attack_resolved\",\"result\":{\"target\":\"M07\",\"damage\":12,\"target_health\":{\"from\":100,\"to\":88},\"striker_energy\":{\"from\":60,\"to\":52},\"target_died\":\"no\"}}\n",
            ),
            (
                3,
                "M04",
                EventDetail::ThreatResolved {
                    target: "M07".to_string(),
                    increase: 10,
                    target_fear: (5, 15),
                },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"M04\",\"event\":\"threat_resolved\",\"result\":{\"target\":\"M07\",\"increase\":10,\"target_fear\":{\"from\":5,\"to\":15}}}\n",
            ),
            (
                3,
                "M04",
                EventDetail::SurrenderResolved {
                    recipient: "M07".to_string(),
                    transferred: 20,
                    discarded: 5,
                    subject_satiety: (25, 0),
                    recipient_satiety: (80, 100),
                },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"M04\",\"event\":\"surrender_resolved\",\"result\":{\"recipient\":\"M07\",\"transferred\":20,\"discarded\":5,\"subject_satiety\":{\"from\":25,\"to\":0},\"recipient_satiety\":{\"from\":80,\"to\":100}}}\n",
            ),
            (
                3,
                "world",
                EventDetail::SimulationEnded {
                    reason: TerminationReason::Extinction,
                },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"world\",\"event\":\"simulation_ended\",\"result\":{\"reason\":\"extinction\"}}\n",
            ),
            (
                3,
                "M04",
                EventDetail::ActionTrace {
                    proposal: Action::Move {
                        direction: Direction::North,
                    },
                    accepted: false,
                    detail: "out_of_bounds".to_string(),
                    position: Coordinate { x: 41, y: 63 },
                    territory: Territory::A,
                    health: 100,
                    satiety: 97,
                    energy: 97,
                    fear: 0,
                    suffered: Vec::new(),
                },
                "{\"record\":\"event\",\"tick\":3,\"subject\":\"M04\",\"event\":\"action_trace\",\"result\":{\"proposal\":{\"action\":\"move\",\"direction\":\"north\"},\"status\":\"rejected\",\"detail\":\"out_of_bounds\",\"position\":{\"x\":41,\"y\":63},\"territory\":\"A\",\"health\":100,\"satiety\":97,\"energy\":97,\"fear\":0,\"suffered\":[]}}\n",
            ),
        ];

        assert_eq!(cases.len(), EventType::ALL.len());
        for kind in EventType::ALL {
            assert_eq!(
                cases
                    .iter()
                    .filter(|(_, _, detail, _)| detail.event_type() == kind)
                    .count(),
                1,
                "exactly one row per event kind, and {kind} is not covered once"
            );
        }

        for (tick, subject, detail, expected) in cases {
            let kind = detail.event_type();
            let event = Event::new(tick, subject, detail);
            let mut records = Vec::new();
            write_event_record(&mut records, &event).unwrap();
            assert_eq!(String::from_utf8(records).unwrap(), expected, "{kind}");
        }
    }

    /// Rule 6.5's third composite shape, for all eleven actions.
    ///
    /// The match below is exhaustive without a wildcard, so a twelfth action fails to compile
    /// here as well as in `write_proposal`, and the count keeps the table honest.
    ///
    /// The seven targeted verbs each carry a target and nothing else, and the target is a field
    /// *of* the proposal object rather than a sibling of it. The text stream places it beside
    /// `proposal=` because that line has no nesting to place it in; rule 6.5's "the one value it
    /// carries" is what decides it here, on the same terms as `eat`'s `food` and `move`'s
    /// `direction`.
    #[test]
    fn every_proposal_shape_is_the_action_word_and_the_one_value_it_carries() {
        let cases = [
            (Action::Wait, "{\"action\":\"wait\"}"),
            (Action::Sleep, "{\"action\":\"sleep\"}"),
            (
                Action::Eat {
                    food_id: "F0002".to_string(),
                },
                "{\"action\":\"eat\",\"food\":\"F0002\"}",
            ),
            (
                Action::Move {
                    direction: Direction::North,
                },
                "{\"action\":\"move\",\"direction\":\"north\"}",
            ),
            (
                Action::Approach {
                    target: "M01".to_string(),
                },
                "{\"action\":\"approach\",\"target\":\"M01\"}",
            ),
            (
                Action::Avoid {
                    target: "M02".to_string(),
                },
                "{\"action\":\"avoid\",\"target\":\"M02\"}",
            ),
            (
                Action::Threaten {
                    target: "M03".to_string(),
                },
                "{\"action\":\"threaten\",\"target\":\"M03\"}",
            ),
            (
                Action::Attack {
                    target: "M04".to_string(),
                },
                "{\"action\":\"attack\",\"target\":\"M04\"}",
            ),
            (
                Action::Fight {
                    target: "M05".to_string(),
                },
                "{\"action\":\"fight\",\"target\":\"M05\"}",
            ),
            (
                Action::Retreat {
                    target: "M06".to_string(),
                },
                "{\"action\":\"retreat\",\"target\":\"M06\"}",
            ),
            (
                Action::Surrender {
                    target: "M07".to_string(),
                },
                "{\"action\":\"surrender\",\"target\":\"M07\"}",
            ),
        ];

        for (action, expected) in &cases {
            match action {
                Action::Wait
                | Action::Sleep
                | Action::Eat { .. }
                | Action::Move { .. }
                | Action::Approach { .. }
                | Action::Avoid { .. }
                | Action::Threaten { .. }
                | Action::Attack { .. }
                | Action::Fight { .. }
                | Action::Retreat { .. }
                | Action::Surrender { .. } => {}
            }
            let mut records = Vec::new();
            write_proposal(&mut records, action).unwrap();
            assert_eq!(String::from_utf8(records).unwrap(), *expected);
        }

        // Every direction, so that the one action carrying a further closed domain covers it.
        for direction in Direction::ORDERED {
            let mut records = Vec::new();
            write_proposal(&mut records, &Action::Move { direction }).unwrap();
            assert_eq!(
                String::from_utf8(records).unwrap(),
                format!("{{\"action\":\"move\",\"direction\":\"{direction}\"}}")
            );
        }
    }

    /// Rule 5: the header states the configuration as resolved, and states no path.
    #[test]
    fn the_header_record_states_the_resolved_configuration_and_never_a_path() {
        let mut records = Vec::new();
        write_header_record(
            &mut records,
            &Config {
                seed: 777,
                tick_limit: 1000,
                policy: Policy::Individual,
                density: Density::parse("1.50").unwrap(),
                trace_actions: true,
                spend_ceiling: None,
                prices: None,
            },
        )
        .unwrap();
        let record = String::from_utf8(records).unwrap();

        assert_eq!(
            record,
            "{\"record\":\"header\",\"schema\":3,\"engine\":\"0.1.0\",\
             \"config\":{\"seed\":777,\"ticks\":1000,\"policy\":\"individual\",\
             \"density\":\"1.50\",\"trace_actions\":true}}\n"
        );
        // Rule 5.5 in the only form a test can state it: the writer is given no path, so a
        // path cannot appear. The separators are asserted anyway, because the cheapest way for
        // one to arrive later is a well-meant addition to this record.
        assert!(!record.contains('/'));
        assert!(!record.contains('\\'));
        // Rule 4.3: the density is the two-decimal rendering, as a string, and rule 4.1 leaves
        // every other figure a bare integer.
        assert!(record.contains("\"density\":\"1.50\""));
        assert!(!record.contains("\"density\":1"));
    }

    /// Rule 7.5 and rule 4.4 over an empty living population.
    ///
    /// Zero is a legitimate health, satiety and energy for a living Mokiterion, and zero is the
    /// ordinary fear of an unthreatened one, so no in-band value can mean "there is no
    /// population". `null` is the only correct rendering. The whole record is asserted rather
    /// than the four fields, so a change that also moved a neighbouring field is caught here.
    #[test]
    fn a_metrics_record_over_an_empty_living_population_reports_null_extrema() {
        let mut simulation = Simulation::new(config(0, 10, false)).unwrap();
        simulation.tick = 7;
        for agent in &mut simulation.agents {
            agent.alive = false;
        }
        simulation.foods.clear();

        let mut records = Vec::new();
        simulation.metrics_record(&mut records).unwrap();

        assert_eq!(
            String::from_utf8(records).unwrap(),
            "{\"record\":\"metrics\",\"tick\":7,\"living\":0,\"deaths\":12,\
             \"population\":{\"A\":0,\"B\":0},\
             \"health\":{\"sum\":0,\"min\":null},\"satiety\":{\"sum\":0,\"min\":null},\
             \"energy\":{\"sum\":0,\"min\":null},\"fear\":{\"sum\":0,\"max\":null},\
             \"territories\":{\
             \"A\":{\"standing\":0,\"low\":0,\"medium\":0,\"high\":0,\"capacity\":61,\"depleted\":true},\
             \"B\":{\"standing\":0,\"low\":0,\"medium\":0,\"high\":0,\"capacity\":61,\"depleted\":true}}}\n"
        );
    }

    /// Rule 8.6: every cumulative figure equals the number of corresponding event records.
    ///
    /// The counts are taken from the **text** stream, which predates the counters and which
    /// `SPEC-MOK-001` makes authoritative, so this compares the new state against the older
    /// authority rather than against itself. A counter incremented at a different point in the
    /// tick from its event, incremented twice, or attributed to the wrong class fails here.
    #[test]
    fn every_cumulative_counter_equals_its_event_count_in_the_text_stream() {
        for seed in [0, 1, 42, 123, 777] {
            let mut simulation = Simulation::new(reference_config(seed, 300, false)).unwrap();
            let mut text = Vec::new();
            simulation.run(&mut text).unwrap();
            let text = String::from_utf8(text).unwrap();

            let occurrences = |event: &str, containing: &str| {
                text.lines()
                    .filter(|line| line.contains(event) && line.contains(containing))
                    .count() as u64
            };

            assert_eq!(
                simulation.crossings,
                occurrences("event=territory_crossed", ""),
                "crossings, seed {seed}"
            );
            for class in FoodClass::ALL {
                assert_eq!(
                    simulation.consumed[class.index()],
                    occurrences("event=food_consumed", &format!(",class:{class},")),
                    "consumed {class}, seed {seed}"
                );
            }
            assert_eq!(
                simulation.regenerated,
                occurrences("event=food_regenerated", ""),
                "regenerated, seed {seed}"
            );
            for reason in RegenerationSkipReason::ALL {
                assert_eq!(
                    simulation.regeneration_skipped[reason.index()],
                    occurrences(
                        "event=food_regeneration_skipped",
                        &format!("result=reason:{reason},")
                    ),
                    "regeneration_skipped {reason}, seed {seed}"
                );
            }

            // The per-Mokiterion death tick, on the same principle: the tick the run record
            // will state is the tick the text stream already stated.
            for agent in &simulation.agents {
                let died_at = text
                    .lines()
                    .find(|line| line.contains(&format!("subject={} event=agent_died", agent.id)))
                    .map(|line| {
                        line.trim_start_matches("tick=")
                            .split(' ')
                            .next()
                            .unwrap()
                            .parse::<u64>()
                            .unwrap()
                    });
                assert_eq!(agent.died_at, died_at, "{} died_at, seed {seed}", agent.id);
                assert_eq!(agent.alive, died_at.is_none(), "{}", agent.id);
            }
        }
    }

    /// The per-tick entropy sequence of each of the twenty configurations `REQ-MOK-068` covers, as
    /// measured at the base commit `cc54185`: the seed, the source, the number of tick boundaries,
    /// and an order-sensitive fold of every boundary state.
    ///
    /// Every value here was measured, none inferred. `print_base_commit_entropy_literals` is the
    /// instrument that printed it and `docs/engineering/simulation/evidence/WO-MOK-025/base/`
    /// retains the full per-boundary capture the same run produced.
    ///
    /// It is a base-commit figure and therefore frozen: a later change that moves one of these
    /// numbers has changed an existing source's entropy consumption, which is the failure
    /// `INT-MOK-010` promises against and `WO-MOK-025` stop condition 1 says not to work around.
    const BASE_COMMIT_ENTROPY: [(u64, Policy, usize, u64); 20] = [
        (0, Policy::Baseline, 121, 0xcda87ab9a4f25e4f),
        (0, Policy::Reference, 1002, 0xa293b8339d738ece),
        (0, Policy::Individual, 1002, 0x2e5e2007476de58d),
        (0, Policy::Social, 1002, 0xa55cd2954c2862e8),
        (1, Policy::Baseline, 121, 0x404595b2bcb1bbe6),
        (1, Policy::Reference, 1002, 0xaf7e27eb201edede),
        (1, Policy::Individual, 1002, 0x28b3b59e2e12143a),
        (1, Policy::Social, 1002, 0x084b84b3a501749d),
        (42, Policy::Baseline, 144, 0x85734ce1ccbf1fb2),
        (42, Policy::Reference, 1002, 0x929d7a4bb284a75a),
        (42, Policy::Individual, 1002, 0xdaa8511eca8bb837),
        (42, Policy::Social, 1002, 0x0c2f708da471ee93),
        (123, Policy::Baseline, 170, 0xadd3f3a9e0663338),
        (123, Policy::Reference, 1002, 0x76f23f60eace018c),
        (123, Policy::Individual, 1002, 0x751c89e7f2916755),
        (123, Policy::Social, 1002, 0xb88c3a3e3acc34a5),
        (777, Policy::Baseline, 136, 0x95c2001684ce93ca),
        (777, Policy::Reference, 1002, 0x30f44986c7380cb7),
        (777, Policy::Individual, 1002, 0xa119b216c00c6aca),
        (777, Policy::Social, 1002, 0xea2fb88317fa8b31),
    ];

    /// `REQ-MOK-068` and `VER-MOK-018` case **L9**, the entropy half: the per-tick entropy state
    /// of each of the four existing decision sources, at every declared seed and the default
    /// density, printed one line per tick boundary.
    ///
    /// This is an **instrument, not an assertion**. What L9 obliges is that these figures equal
    /// the base-commit captures for the whole run, and the base commit is `cc54185` — the commit
    /// that carried `WO-MOK-025`'s transition to `in_progress` and no code change. A test cannot
    /// compare against a tree it is not in, so the comparison is external: this test's output is
    /// reduced to one line per configuration by
    /// `docs/engineering/simulation/evidence/WO-MOK-025/entropy-manifest.sh` and the two manifests
    /// are compared with `diff`. `the_four_existing_sources_draw_what_the_base_commit_drew` below
    /// is the in-crate half, so the obligation is also checked on every push with no external
    /// file.
    ///
    /// It names only the four sources that exist at the base commit, deliberately: the same test
    /// body has to compile in a worktree at `cc54185` in order to produce the base capture at all,
    /// and a reference to this initiative's fifth source would make that impossible. The
    /// instrument is retained as a patch beside the capture it produced.
    ///
    /// No sink is configured. `REQ-MOK-045` and rule 11.2 already hold a sink entropy-neutral at
    /// every boundary, `a_record_sink_moves_no_entropy_draw_at_any_tick_boundary` is the test, and
    /// capturing both modes here would double the output to establish something already
    /// established.
    #[test]
    #[ignore = "instrument: prints ~20,000 lines for an external capture, run it by name"]
    fn the_four_existing_sources_entropy_state_at_every_tick_boundary() {
        for seed in DECLARED_SEEDS {
            let base = Config {
                seed,
                tick_limit: 1000,
                policy: Policy::Baseline,
                density: Density::DEFAULT,
                trace_actions: false,
                spend_ceiling: None,
                prices: None,
            };
            print_entropy_trace(
                Config {
                    policy: Policy::Baseline,
                    ..base
                },
                BaselineDecisionSource,
            );
            print_entropy_trace(
                Config {
                    policy: Policy::Reference,
                    ..base
                },
                ReferenceDecisionSource,
            );
            print_entropy_trace(
                Config {
                    policy: Policy::Individual,
                    ..base
                },
                IndividualDecisionSource,
            );
            print_entropy_trace(
                Config {
                    policy: Policy::Social,
                    ..base
                },
                SocialDecisionSource,
            );
        }
    }

    /// One line per tick boundary of one configuration, for the instrument above.
    ///
    /// The line carries the configuration on every line rather than under a header, so that the
    /// output can be reduced per configuration by a filter that needs no state.
    fn print_entropy_trace<D: DecisionSource>(config: Config, source: D) {
        let (states, _) = entropy_trace(config, source, false);
        for (boundary, state) in states.iter().enumerate() {
            println!(
                "seed={} density={} policy={} boundary={boundary} state={state:#018x}",
                config.seed, config.density, config.policy,
            );
        }
    }

    /// `REQ-MOK-068` and `VER-MOK-018` case **L9**, the entropy half as an automated check.
    ///
    /// The instrument above produces the retained capture; this is the same measurement reduced to
    /// two numbers per configuration and compared against the numbers measured at the base commit
    /// `cc54185`. It runs on every push, needs no external file and no network, and fails naming
    /// the configuration that moved.
    ///
    /// **Why two numbers and not a digest.** This package declares no dependencies, so there is no
    /// hash function available and one written here would be a constant the specification does not
    /// have. `fold_states` is order-sensitive over the whole sequence, which is what distinguishes
    /// this from an end-state comparison: two runs that drew differently in the middle and
    /// reconverged agree on the final state and disagree here. The boundary count is carried
    /// beside it because a sequence that is a prefix of another folds differently but for a reason
    /// worth naming separately — a run that ended early.
    ///
    /// **Where the expected values come from.** They were measured at `cc54185`, in a worktree at
    /// that commit with this test's own body applied and nothing else, and the same worktree
    /// produced the retained capture. They are not copied from any artifact.
    #[test]
    fn the_four_existing_sources_draw_what_the_base_commit_drew() {
        for (seed, policy, boundaries, fold) in BASE_COMMIT_ENTROPY {
            let config = Config {
                seed,
                tick_limit: 1000,
                policy,
                density: Density::DEFAULT,
                trace_actions: false,
                spend_ceiling: None,
                prices: None,
            };
            let states = match policy {
                Policy::Baseline => entropy_trace(config, BaselineDecisionSource, false).0,
                Policy::Reference => entropy_trace(config, ReferenceDecisionSource, false).0,
                Policy::Individual => entropy_trace(config, IndividualDecisionSource, false).0,
                Policy::Social => entropy_trace(config, SocialDecisionSource, false).0,
                // The table is the four existing sources' and nothing else. `REQ-MOK-068` is a
                // statement about them, and the `llm` source has no base-commit figure because
                // it did not exist at the base commit. It also has no source to trace without a
                // port — which rule 20.7 makes moot, since it draws nothing and would add a row
                // of zeroes.
                Policy::Llm => panic!("the base-commit entropy table names no llm row"),
            };
            assert_eq!(
                states.len(),
                boundaries,
                "seed {seed} policy {policy}: tick boundary count moved from the base commit"
            );
            assert_eq!(
                fold_states(&states),
                fold,
                "seed {seed} policy {policy}: the per-boundary entropy sequence moved from the \
                 base commit"
            );
        }
    }

    /// The instrument that produced `BASE_COMMIT_ENTROPY`, retained so the constant above is
    /// re-derivable rather than trusted.
    ///
    /// It prints the array in Rust source form. Run it in a worktree at the base commit with this
    /// module's test body applied and paste the output; that is how the constant was written, and
    /// running it at any later commit is how a reader checks whether it still holds.
    #[test]
    #[ignore = "instrument: prints the base-commit constant in source form, run it by name"]
    fn print_base_commit_entropy_literals() {
        println!("    const BASE_COMMIT_ENTROPY: [(u64, Policy, usize, u64); 20] = [");
        for seed in DECLARED_SEEDS {
            for policy in [
                Policy::Baseline,
                Policy::Reference,
                Policy::Individual,
                Policy::Social,
            ] {
                let config = Config {
                    seed,
                    tick_limit: 1000,
                    policy,
                    density: Density::DEFAULT,
                    trace_actions: false,
                    spend_ceiling: None,
                    prices: None,
                };
                let states = match policy {
                    Policy::Baseline => entropy_trace(config, BaselineDecisionSource, false).0,
                    Policy::Reference => entropy_trace(config, ReferenceDecisionSource, false).0,
                    Policy::Individual => entropy_trace(config, IndividualDecisionSource, false).0,
                    Policy::Social => entropy_trace(config, SocialDecisionSource, false).0,
                    // As above: the instrument regenerates the four existing sources' table.
                    Policy::Llm => panic!("the base-commit entropy table names no llm row"),
                };
                println!(
                    "        ({}, Policy::{:?}, {}, {:#018x}),",
                    seed,
                    policy,
                    states.len(),
                    fold_states(&states),
                );
            }
        }
        println!("    ];");
    }

    /// An order-sensitive fold over a boundary sequence, for the check above.
    ///
    /// Rotation before the exclusive-or is what makes it order-sensitive: without it, two
    /// sequences that are permutations of one another would fold equal.
    fn fold_states(states: &[u64]) -> u64 {
        states.iter().fold(0u64, |accumulator, state| {
            accumulator.rotate_left(7) ^ state
        })
    }

    /// `REQ-MOK-045` and rule 11.2: a record sink moves no entropy draw.
    ///
    /// The state is compared at **every** tick boundary rather than only at the end, because two
    /// runs that drew differently in the middle and reconverged would pass an end-state
    /// comparison. `SplitMix64` holds one `u64` and advances it by a fixed gamma, so equal
    /// states after equal tick counts is equal draw counts — the comparison is exact, not
    /// statistical.
    fn assert_a_sink_is_entropy_neutral<D: DecisionSource>(config: Config, source: impl Fn() -> D) {
        let (plain_states, plain_text) = entropy_trace(config, source(), false);
        let (recorded_states, recorded_text) = entropy_trace(config, source(), true);

        assert!(plain_states.len() > 2, "the trace must cover several ticks");
        assert_eq!(plain_states, recorded_states, "{:?}", config.policy);
        assert_eq!(plain_text, recorded_text, "{:?}", config.policy);

        // `VER-MOK-012` retains this comparison per tick, per seed, per policy, with and without a
        // sink. It is printed from inside the assertion rather than measured again elsewhere, so the
        // retained figures cannot disagree with the figures that were asserted. Boundary `0` is
        // before initialization, `1` is after it, and boundary `n + 1` is after tick `n`.
        for (boundary, (plain, recorded)) in plain_states.iter().zip(&recorded_states).enumerate() {
            println!(
                "seed={} density={} policy={} trace={} boundary={boundary} \
                 nosink={plain:#018x} sink={recorded:#018x} {}",
                config.seed,
                config.density,
                config.policy,
                config.trace_actions,
                if plain == recorded {
                    "equal"
                } else {
                    "DIFFERS"
                },
            );
        }
    }

    /// The entropy state once initialization is complete, with and without a sink.
    ///
    /// Initialization only: the additivity property is about the state a run *starts* from, and
    /// running further would measure the decision source as well as the seed and the density.
    fn state_after_initialization(config: Config, record: bool) -> u64 {
        let mut simulation = Simulation::new(config).unwrap();
        let mut text = Vec::new();
        let mut records = Vec::new();
        {
            let mut sinks = Sinks {
                text: &mut text,
                records: if record {
                    Some(&mut records as &mut dyn Write)
                } else {
                    None
                },
            };
            if let Some(sink) = sinks.records() {
                write_header_record(sink, &config).unwrap();
            }
            for event in simulation.entity_initialization_events() {
                simulation.emit(&mut sinks, event).unwrap();
            }
        }
        assert_eq!(record, !records.is_empty());
        simulation.entropy_state()
    }

    /// The entropy state at every tick boundary of a run, and the text the run wrote.
    ///
    /// The header is written here too when a sink is present, because it is part of the sink
    /// path and a trace that skipped it would leave that path unexercised.
    fn entropy_trace<D: DecisionSource>(
        config: Config,
        mut source: D,
        record: bool,
    ) -> (Vec<u64>, Vec<u8>) {
        let mut simulation = Simulation::new(config).unwrap();
        let mut text = Vec::new();
        let mut records = Vec::new();
        let mut states = vec![simulation.entropy_state()];
        {
            let mut sinks = Sinks {
                text: &mut text,
                records: if record {
                    Some(&mut records as &mut dyn Write)
                } else {
                    None
                },
            };
            if let Some(sink) = sinks.records() {
                write_header_record(sink, &config).unwrap();
            }
            for event in simulation.entity_initialization_events() {
                simulation.emit(&mut sinks, event).unwrap();
            }
            states.push(simulation.entropy_state());
            while simulation.outcome.is_none() {
                simulation.step(&mut sinks, &mut source).unwrap();
                states.push(simulation.entropy_state());
            }
        }
        assert_eq!(record, !records.is_empty());
        (states, text)
    }

    #[test]
    fn a_record_sink_moves_no_entropy_draw_at_any_tick_boundary() {
        for seed in DECLARED_SEEDS {
            for trace_actions in [false, true] {
                let base = Config {
                    seed,
                    tick_limit: 150,
                    policy: Policy::Baseline,
                    density: Density::DEFAULT,
                    trace_actions,
                    spend_ceiling: None,
                    prices: None,
                };
                assert_a_sink_is_entropy_neutral(
                    Config {
                        policy: Policy::Baseline,
                        ..base
                    },
                    || BaselineDecisionSource,
                );
                assert_a_sink_is_entropy_neutral(
                    Config {
                        policy: Policy::Reference,
                        ..base
                    },
                    || ReferenceDecisionSource,
                );
                assert_a_sink_is_entropy_neutral(
                    Config {
                        policy: Policy::Individual,
                        ..base
                    },
                    || IndividualDecisionSource,
                );
            }
        }
    }

    /// Rule 11.1's additivity: the state a run starts from is the seed and the density, and nothing
    /// else.
    ///
    /// The sink configuration, the policy and the tracing setting are all varied against a fixed
    /// seed and density, and the state after initialization must be one value across all twelve
    /// combinations. This is the property that makes the pre-change comparison meaningful: if the
    /// starting state depended on the policy, an unchanged output at one policy would say nothing
    /// about another.
    ///
    /// The swept densities are `VER-MOK-002`'s, because the record path resolves capacity from the
    /// density and a draw taken per resource would show at a density the default does not reach.
    #[test]
    fn the_entropy_state_after_initialization_is_the_seed_and_the_density_alone() {
        for seed in DECLARED_SEEDS {
            for density in SWEPT_DENSITIES {
                let mut observed = Vec::new();
                for policy in [Policy::Baseline, Policy::Reference, Policy::Individual] {
                    for trace_actions in [false, true] {
                        for record in [false, true] {
                            let config = Config {
                                seed,
                                tick_limit: 1000,
                                policy,
                                density: Density::parse(density).unwrap(),
                                trace_actions,
                                spend_ceiling: None,
                                prices: None,
                            };
                            observed.push((
                                (policy, trace_actions, record),
                                state_after_initialization(config, record),
                            ));
                        }
                    }
                }

                let (_, first) = observed[0];
                for (combination, state) in &observed {
                    assert_eq!(
                        *state, first,
                        "seed {seed} density {density} moved at {combination:?}"
                    );
                }
                println!(
                    "seed={seed} density={density} after_initialization={first:#018x} \
                     combinations={} all_equal=yes",
                    observed.len(),
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

    /// `REQ-MOK-053`'s no-entropy constraint, directly: the shared stream stands exactly where
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
                    .apply_action(&mut text_only(&mut Vec::new()), 0, &action)
                    .unwrap();

                assert_eq!(simulation.entropy, before, "{action} moved the stream");
            }
        }
    }

    /// The state at the thousandth tick, printed per declared seed, swept density and policy.
    ///
    /// The figures `VER-MOK-012` compares against the pre-change build. Unlike the state after
    /// initialization this one *is* a function of the policy, because deciding draws; so the sink
    /// comparison is made within a combination and the policy is part of the row's identity.
    #[test]
    fn the_entropy_state_at_the_thousandth_tick_is_unmoved_by_a_sink() {
        for seed in DECLARED_SEEDS {
            for density in SWEPT_DENSITIES {
                for policy in [Policy::Baseline, Policy::Reference, Policy::Individual] {
                    let config = Config {
                        seed,
                        tick_limit: 1000,
                        policy,
                        density: Density::parse(density).unwrap(),
                        trace_actions: false,
                        spend_ceiling: None,
                        prices: None,
                    };

                    let mut plain = Simulation::new(config).unwrap();
                    let mut plain_text = Vec::new();
                    let plain_summary = plain.run(&mut plain_text).unwrap();

                    let mut recorded = Simulation::new(config).unwrap();
                    let mut recorded_text = Vec::new();
                    let mut records = Vec::new();
                    let recorded_summary = completed(
                        recorded
                            .run_recording(&mut recorded_text, Some(&mut records), None)
                            .unwrap(),
                    );

                    assert_eq!(plain_text, recorded_text, "{seed} {density} {policy}");
                    assert_eq!(plain_summary, recorded_summary);
                    assert_eq!(plain.entropy_state(), recorded.entropy_state());

                    println!(
                        "seed={seed} density={density} policy={policy} ticks={} reason={} \
                         nosink={:#018x} sink={:#018x} equal",
                        plain_summary.ticks,
                        plain_summary.reason,
                        plain.entropy_state(),
                        recorded.entropy_state(),
                    );
                }
            }
        }
    }

    /// Rule 11.1 across the whole run, including the header and the run record.
    ///
    /// [`Simulation::run`] and [`Simulation::run_recording`] with no sink are the same call, and
    /// with a sink the text bytes and the final entropy state are unchanged. Every policy, and
    /// tracing on, because tracing multiplies the event records by an order of magnitude and is
    /// therefore where a projection that cost a draw would show first.
    #[test]
    fn a_recorded_run_writes_the_same_text_bytes_as_an_unrecorded_one() {
        for policy in [Policy::Baseline, Policy::Reference, Policy::Individual] {
            for trace_actions in [false, true] {
                let config = Config {
                    seed: 123,
                    tick_limit: 200,
                    policy,
                    density: Density::DEFAULT,
                    trace_actions,
                    spend_ceiling: None,
                    prices: None,
                };

                let mut plain = Simulation::new(config).unwrap();
                let mut plain_text = Vec::new();
                let plain_summary = plain.run(&mut plain_text).unwrap();

                let mut recorded = Simulation::new(config).unwrap();
                let mut recorded_text = Vec::new();
                let mut records = Vec::new();
                let recorded_summary = completed(
                    recorded
                        .run_recording(&mut recorded_text, Some(&mut records), None)
                        .unwrap(),
                );

                assert_eq!(plain_text, recorded_text, "{policy:?} {trace_actions}");
                assert_eq!(plain_summary, recorded_summary);
                assert_eq!(plain.entropy_state(), recorded.entropy_state());
                assert!(!records.is_empty());
            }
        }
    }

    /// `INT-MOK-010`'s recorded risk, at the mechanism: exchanging the two identifiers changes
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
                    &mut text_only(&mut Vec::new()),
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

    /// `REQ-MOK-053`'s lethality, counted: four strikes at the maximum and ten at the minimum
    /// empty a full-health Mokiterion.
    ///
    /// This is the arithmetic `REQ-MOK-058`'s floor was lowered against, so it is asserted
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
                        &mut text_only(&mut Vec::new()),
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
            .run_tick(&mut text_only(&mut output), &mut SocialDecisionSource)
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
                    &mut text_only(&mut output),
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
            .run_tick(&mut text_only(&mut output), &mut SocialDecisionSource)
            .unwrap();

        // `M02` carries the composition, and it is the half that proves the ordering: it was
        // threatened inside `M01`'s turn for `THREAT_FEAR_INCREASE`, and rule 12 then added `10`
        // for the company it kept at its own. Unsaturated, so both writes are visible in the sum.
        assert_eq!(simulation.agents[1].fear, THREAT_FEAR_INCREASE + 10);

        // `M01` carries only saturation, and that is a consequence of `REQ-MOK-057`'s amendment
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
            .run_tick(&mut text_only(&mut Vec::new()), &mut SocialDecisionSource)
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
                &mut text_only(&mut Vec::new()),
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
                .run_tick(&mut text_only(&mut Vec::new()), &mut SocialDecisionSource)
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
                &mut text_only(&mut Vec::new()),
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
                &mut text_only(&mut Vec::new()),
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
                &mut text_only(&mut Vec::new()),
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
                .apply_action(&mut text_only(&mut Vec::new()), 0, &action)
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
                &mut text_only(&mut output),
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
    /// order rather than of rule 25, and `INT-MOK-010` records the asymmetry as accepted.
    #[test]
    fn a_defender_below_its_attackers_identifier_answers_on_the_next_tick() {
        let mut simulation =
            encounter(14, Coordinate { x: 30, y: 30 }, Coordinate { x: 31, y: 30 });
        simulation.config.trace_actions = true;

        // Tick one: `M01` strikes, `M02` answers within the tick, and the reply lands in
        // `M01`'s record after `M01`'s opportunity has passed.
        simulation
            .run_tick(&mut text_only(&mut Vec::new()), &mut SocialDecisionSource)
            .unwrap();
        assert_eq!(simulation.agents[0].suffered.len(), 1);

        // Tick two: `M01` reads that record at its own next opportunity. Its `fear` stands at
        // rule 12's one write, which is below rule 26's retreat threshold, so the answer is a
        // fight.
        let mut output = Vec::new();
        simulation
            .run_tick(&mut text_only(&mut output), &mut SocialDecisionSource)
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
            .run_tick(&mut text_only(&mut output), &mut SocialDecisionSource)
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

    /// The *State model*'s counters are written to and read only by the run record.
    ///
    /// Asserted behaviorally rather than by reading the source: the two runs are identical
    /// except that one starts with every counter pre-loaded with a value no run reaches. Were
    /// any rule to consult a counter, the runs would diverge in text or in draws. The run
    /// records are expected to differ, and that difference is what shows the pre-loading
    /// reached the one place a counter is read.
    #[test]
    fn no_rule_reads_a_cumulative_counter() {
        let config = reference_config(42, 200, true);
        let mut plain = Simulation::new(config).unwrap();
        let mut loaded = Simulation::new(config).unwrap();
        loaded.crossings = 900_000;
        loaded.consumed = [900_001, 900_002, 900_003];
        loaded.regenerated = 900_004;
        loaded.regeneration_skipped = [900_005, 900_006];

        let mut plain_text = Vec::new();
        let mut plain_records = Vec::new();
        plain
            .run_recording(&mut plain_text, Some(&mut plain_records), None)
            .unwrap();
        let mut loaded_text = Vec::new();
        let mut loaded_records = Vec::new();
        loaded
            .run_recording(&mut loaded_text, Some(&mut loaded_records), None)
            .unwrap();

        assert_eq!(plain_text, loaded_text);
        assert_eq!(plain.entropy_state(), loaded.entropy_state());

        let plain_records = String::from_utf8(plain_records).unwrap();
        let loaded_records = String::from_utf8(loaded_records).unwrap();
        let plain_lines: Vec<&str> = plain_records.lines().collect();
        let loaded_lines: Vec<&str> = loaded_records.lines().collect();
        assert_eq!(
            plain_lines[..plain_lines.len() - 1],
            loaded_lines[..loaded_lines.len() - 1],
            "only the run record may differ"
        );
        assert_ne!(plain_lines.last(), loaded_lines.last());
        assert!(
            loaded_lines
                .last()
                .unwrap()
                .contains(&format!("\"crossings\":{}", 900_000 + plain.crossings))
        );
    }

    /// Every counter saturates.
    ///
    /// A counter at its maximum is not a state any run reaches, and it must still not be the
    /// state that ends one: an overflow panic in a debug build would turn a statistic into a
    /// crash, and the statistic is the least important thing in the process.
    #[test]
    fn a_saturated_counter_neither_wraps_nor_panics() {
        let mut simulation = Simulation::new(reference_config(42, 80, false)).unwrap();
        simulation.crossings = u64::MAX;
        simulation.consumed = [u64::MAX; 3];
        simulation.regenerated = u64::MAX;
        simulation.regeneration_skipped = [u64::MAX; 2];

        let mut text = Vec::new();
        let mut records = Vec::new();
        simulation
            .run_recording(&mut text, Some(&mut records), None)
            .unwrap();

        assert_eq!(simulation.crossings, u64::MAX);
        assert_eq!(simulation.consumed, [u64::MAX; 3]);
        assert_eq!(simulation.regenerated, u64::MAX);
        assert_eq!(simulation.regeneration_skipped, [u64::MAX; 2]);
    }

    /// A sink that fails on its first write. Rules 13.3 and 13.5, and rule 13.2's principle
    /// that a run which cannot be recorded is not run: the header is the first record, so the
    /// failure precedes the first tick and the text stream stays empty.
    #[test]
    fn a_record_sink_that_fails_at_once_ends_the_run_before_any_text_is_written() {
        let mut simulation = Simulation::new(reference_config(0, 10, false)).unwrap();
        let mut text = Vec::new();
        let mut sink = FailingSink::after(0);
        let error = simulation
            .run_recording(&mut text, Some(&mut sink), None)
            .unwrap_err();

        // Rule 13.5: distinguishable from a text-stream failure, deterministic in form, and
        // carrying the sink's identity and the platform's reason and nothing else. The library
        // knows no path, so no path can appear.
        assert_eq!(error.to_string(), "record sink: closed");
        assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
        assert!(text.is_empty());
        assert_eq!(simulation.tick, 0);
    }

    /// A sink that fails partway through. Rule 8.9: no run record for a run that failed a
    /// write, so a truncated stream cannot be read as a complete run even before the host
    /// removes the file.
    #[test]
    fn a_record_sink_that_fails_mid_run_leaves_no_run_record() {
        let mut simulation = Simulation::new(reference_config(0, 50, false)).unwrap();
        let mut text = Vec::new();
        let mut sink = FailingSink::after(2000);
        let error = simulation
            .run_recording(&mut text, Some(&mut sink), None)
            .unwrap_err();

        assert_eq!(error.to_string(), "record sink: closed");
        let written = String::from_utf8(sink.written).unwrap();
        assert!(written.starts_with("{\"record\":\"header\""));
        assert!(!written.contains("\"record\":\"run\""));
        // The text stream got as far as the record stream did, and no further: the run did not
        // reach its summary line, so nothing downstream can mistake it for a completed run.
        assert!(!String::from_utf8(text).unwrap().contains("summary reason="));
    }

    /// A sink that accepts writes until it has taken a fixed number of bytes and then fails
    /// every write, retaining what it took.
    ///
    /// Bytes rather than calls: one `write!` produces one call per formatting fragment, so a
    /// call budget would be a budget over an implementation detail of `core::fmt`. `flush`
    /// succeeds, so the failure this produces is unambiguously a write failure.
    struct FailingSink {
        budget: usize,
        written: Vec<u8>,
    }

    impl FailingSink {
        fn after(budget: usize) -> Self {
            Self {
                budget,
                written: Vec::new(),
            }
        }
    }

    impl Write for FailingSink {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            if self.written.len() >= self.budget {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed"));
            }
            self.written.extend_from_slice(buffer);
            Ok(buffer.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// Rule 3.3's closed value alphabet, enumerated over every domain that reaches a record.
    ///
    /// The union is `A-Z a-z 0-9 _ . - + : ; >`. **No quotation mark, no backslash, no code
    /// point below U+0020 — and that is the entire reason the writers in this module need no
    /// escaping function.** Each domain is enumerated completely and its size asserted, so a
    /// fifth direction or a sixteenth event type fails here rather than reaching a stream a
    /// consumer cannot parse. The matches are exhaustive without a wildcard, so for the domains
    /// that have no `ALL` a new variant fails to compile instead.
    #[test]
    fn every_closed_domain_that_reaches_a_record_is_on_the_alphabet() {
        fn on_alphabet(value: &str) -> bool {
            !value.is_empty()
                && value.chars().all(|character| {
                    character.is_ascii_alphanumeric()
                        || matches!(character, '_' | '.' | '-' | '+' | ':' | ';' | '>')
                })
        }

        /// One member, asserted and recorded in the same call.
        ///
        /// `VER-MOK-012` retains this enumeration with each domain's members, its size and the
        /// emitted bytes. Recording it here rather than in a separate pass is what stops the
        /// retained list from naming a member the assertion never saw.
        fn member(
            evidence: &mut Vec<(&'static str, String, String)>,
            domain: &'static str,
            value: &str,
        ) {
            assert!(on_alphabet(value), "{domain}: {value:?}");
            let bytes = value
                .as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<Vec<_>>()
                .join(" ");
            evidence.push((domain, value.to_string(), bytes));
        }

        // The alphabet's own definition, checked against the characters it must exclude, so
        // that a later widening of `on_alphabet` cannot pass silently.
        for excluded in ['"', '\\', '\n', '\r', '\t', ' ', ',', '{', '}', '[', ']'] {
            assert!(
                !on_alphabet(&excluded.to_string()),
                "{excluded:?} is not on the alphabet"
            );
        }

        let mut evidence = Vec::new();
        let mut sizes: Vec<(&str, usize)> = Vec::new();

        assert_eq!(EventType::ALL.len(), 15);
        sizes.push(("event", EventType::ALL.len()));
        for kind in EventType::ALL {
            member(&mut evidence, "event", kind.as_str());
        }

        assert_eq!(Territory::ALL.len(), 2);
        sizes.push(("territory", Territory::ALL.len()));
        for territory in Territory::ALL {
            member(&mut evidence, "territory", &territory.to_string());
        }

        assert_eq!(FoodClass::ALL.len(), 3);
        sizes.push(("class", FoodClass::ALL.len()));
        for class in FoodClass::ALL {
            member(&mut evidence, "class", &class.to_string());
        }

        assert_eq!(RegenerationSkipReason::ALL.len(), 2);
        sizes.push(("reason.skip", RegenerationSkipReason::ALL.len()));
        for reason in RegenerationSkipReason::ALL {
            member(&mut evidence, "reason.skip", &reason.to_string());
        }

        assert_eq!(Direction::ORDERED.len(), 4);
        sizes.push(("direction", Direction::ORDERED.len()));
        for direction in Direction::ORDERED {
            member(&mut evidence, "direction", &direction.to_string());
        }

        sizes.push(("reason.termination", 2));
        for reason in [TerminationReason::TickLimit, TerminationReason::Extinction] {
            match reason {
                TerminationReason::TickLimit | TerminationReason::Extinction => {}
            }
            member(&mut evidence, "reason.termination", &reason.to_string());
        }

        sizes.push(("policy", 5));
        for policy in [
            Policy::Baseline,
            Policy::Reference,
            Policy::Individual,
            Policy::Social,
            Policy::Llm,
        ] {
            match policy {
                Policy::Baseline
                | Policy::Reference
                | Policy::Individual
                | Policy::Social
                | Policy::Llm => {}
            }
            member(&mut evidence, "policy", &policy.to_string());
        }

        // The `status` and `target_died` fields' two values each, and the `source` field's
        // five, which are the three string domains that have no type of their own. `target_died`
        // is one of them because rule 6.3 carries it as the text stream's own word rather than
        // as the `bool` the engine holds, on the same terms as `status`.
        sizes.push(("status", 2));
        for status in ["accepted", "rejected"] {
            member(&mut evidence, "status", status);
        }
        sizes.push(("target_died", 2));
        for died in [true, false] {
            member(
                &mut evidence,
                "target_died",
                if died { "yes" } else { "no" },
            );
        }
        sizes.push(("source", 5));
        for source in [
            BaselineDecisionSource.name(),
            ReferenceDecisionSource.name(),
            IndividualDecisionSource.name(),
            SocialDecisionSource.name(),
            // Asked of the source itself like the other four, through a port that is never
            // consulted: `name` does not reach it, and a source constructed only to be named is
            // the closest this enumeration can come to the four above.
            PortDecisionSource::new(&mut ForbiddenPort).name(),
        ] {
            member(&mut evidence, "source", source);
        }

        // Every identifier a `subject`, `id`, `name`, `food` or `detail` field can carry, from
        // the widest configuration the requirements declare.
        assert_eq!(NAMES.len(), 12);
        sizes.push(("name", NAMES.len()));
        for name in NAMES {
            member(&mut evidence, "name", name);
        }
        let simulation = Simulation::new(Config {
            seed: 0,
            tick_limit: 1,
            policy: Policy::Reference,
            density: Density::parse("1.50").unwrap(),
            trace_actions: true,
            spend_ceiling: None,
            prices: None,
        })
        .unwrap();
        sizes.push(("subject.agent", simulation.agents.len()));
        for agent in &simulation.agents {
            member(&mut evidence, "subject.agent", &agent.id);
            assert!(on_alphabet(agent.name), "{}", agent.name);
        }
        sizes.push(("subject.food", simulation.foods.len()));
        for food in &simulation.foods {
            member(&mut evidence, "subject.food", &food.id);
        }

        // Every density the requirements declare, and the two bounds, since rule 4.3 puts this
        // rendering into the header as a string.
        sizes.push(("density", 5));
        for density in ["0.02", "0.15", "0.75", "1.50", "100.00"] {
            member(
                &mut evidence,
                "density",
                &Density::parse(density).unwrap().to_string(),
            );
        }

        // Retained as oracle 5's evidence. The `subject.food` domain is a run's resource
        // identifiers at the widest declared density rather than a closed set the specification
        // names, so its size is the count observed and is reported as such.
        for (domain, size) in &sizes {
            let observed = evidence.iter().filter(|(name, ..)| name == domain).count();
            println!("domain={domain} size={size} enumerated={observed}");
        }
        for (domain, value, bytes) in &evidence {
            println!("domain={domain} member={value} bytes={bytes}");
        }
        println!(
            "domains={} members={} off_alphabet=0",
            sizes.len(),
            evidence.len(),
        );
    }

    // -----------------------------------------------------------------------------------
    // `SPEC-MOK-007`: the decision port, the request it carries, and rule 20.8's refusal.
    //
    // Every port here is an in-process value. Nothing in this module spawns a process, opens a
    // connection, reads an environment variable or names a model, which is `WO-MOK-025`'s
    // *Out of scope* stated as a property of the tests rather than only of the product.
    // -----------------------------------------------------------------------------------

    /// A port that answers from a script and keeps every request it was given.
    ///
    /// It is `SPEC-MOK-007` rule 13.7's shape at its simplest: the source is exercised without a
    /// provider, without a credential and without a transport, because rule 1.1 leaves the
    /// engine unable to tell the difference. `answers` is consumed front to back and an
    /// exhausted script answers `None`, which is rule 9's no-proposal case.
    ///
    /// It also **holds the accounting**, because rule 20.4.1 puts it in the port and because
    /// `VER-MOK-018`'s cases L18, L19 and L30 all specify "a stubbed port with declared unit prices
    /// and synthetic usage" — which is this, with the prices and the usage declared by the test
    /// rather than by a provider. A live connector will do exactly what `propose` does below with
    /// the figures a response reported.
    #[derive(Default)]
    struct ScriptedPort {
        answers: Vec<Option<Action>>,
        /// The answer an exhausted script falls back to, so that a test can have a run in which
        /// *every* exchange yielded a proposal. Without it a script shorter than the run's
        /// opportunity count turns into a run of fallbacks partway through, and rule 9.6's
        /// distinction cannot be measured against a clean run.
        standing: Option<Action>,
        seen: Vec<DecisionRequest>,
        /// Every record the engine authored, in the order it authored them. This is the transcript
        /// a recording host would have written, held in memory: rule 11.1 puts the destination in
        /// the host's hands and a `Vec<String>` is as legitimate a destination as a file, which is
        /// what lets the whole of rule 11 be exercised without a filesystem operation.
        written: Vec<String>,
        /// The record number, counting from one, at which `record` reports a write failure. Rule
        /// 19.6's path: a run whose exchanges were spent and not recorded ends with an error.
        fail_at: Option<usize>,
        /// Rules 14.1 and 9.5's accumulators. A default account declares no prices and no ceiling,
        /// so every port built before this existed bills zero and behaves exactly as it did.
        account: RunAccount,
        /// The usage this port reports for an exchange that yielded a proposal. Synthetic by
        /// construction: no provider reported it and nothing here estimates it.
        ///
        /// It is billed *and* reported, which a real connector will also do with one figure: rule
        /// 14's accounting and rule 11.3's record read the same counts, and a fixture that declared
        /// them twice could pass case **P4** while a connector that disagreed with itself failed it.
        usage: ExchangeUsage,
        /// The response rule 1.1a has the port return beside the proposal, where the test declares
        /// one. `None` — no response at all — for every port built before this existed, so their
        /// records still state `"response":null` byte for byte.
        response: Option<String>,
    }

    impl ScriptedPort {
        /// A port that never obtains a proposal, so every decision takes rule 9.5's `wait`.
        fn silent() -> Self {
            Self::default()
        }

        fn answering(answers: Vec<Option<Action>>) -> Self {
            Self {
                answers,
                ..Self::default()
            }
        }

        /// A port that answers the same action at every opportunity and never runs out.
        fn always(action: Action) -> Self {
            Self {
                standing: Some(action),
                ..Self::default()
            }
        }

        /// Declares the run's prices, its ceiling and the usage each answered exchange reports —
        /// rule 14.3's inputs, arriving as inputs.
        ///
        /// The prices arrive as [`PerTokenPrices`] and not as the public [`UnitPrices`] that
        /// [`ConnectorPort::new`](super::ConnectorPort::new) takes, which is deliberate: the
        /// conversion between the two is
        /// [`PerTokenPrices::declared`](super::accounting::PerTokenPrices::declared) and it has a
        /// case of its own. A fixture that went through it would make every assertion below depend on
        /// that one function being right, and the field this file's figures name outright — the
        /// *uncached* prompt price — is the field the conversion exists to name.
        fn billing(
            self,
            prices: PerTokenPrices,
            ceiling: Option<u64>,
            usage: ExchangeUsage,
        ) -> Self {
            Self {
                account: RunAccount::declared(prices, ceiling),
                usage,
                ..self
            }
        }

        /// Declares the response the port received, rule 1.1a's other field.
        ///
        /// Separate from [`Self::billing`] because the two are independent: a response arrives from a
        /// connector whether or not the run declared prices, and rule 14.8 makes the prices live-only
        /// while rule 11.3's record field is owed by every exchange.
        fn receiving(self, response: &str) -> Self {
            Self {
                response: Some(response.to_string()),
                ..self
            }
        }

        /// A port whose `record` fails at the given record number, counting from one.
        fn failing_to_record_at(record_number: usize) -> Self {
            Self {
                fail_at: Some(record_number),
                ..Self::default()
            }
        }

        /// The transcript as a stream: one record per line, with a trailing newline, which is the
        /// framing rule 11.2 fixes and the one [`ReplayPort`] reads.
        fn transcript(&self) -> String {
            self.written
                .iter()
                .map(|record| format!("{record}\n"))
                .collect()
        }

        fn exchanges(&self) -> Vec<&String> {
            self.written
                .iter()
                .filter(|record| record.contains("\"transcript\":\"exchange\""))
                .collect()
        }

        fn prefixes(&self) -> Vec<&String> {
            self.written
                .iter()
                .filter(|record| record.contains("\"transcript\":\"prefix\""))
                .collect()
        }
    }

    impl Proposer for ScriptedPort {
        /// Rule 14.6, answered from this port's own account exactly as
        /// [`ConnectorPort::halted`](super::ConnectorPort::halted) answers from its own.
        ///
        /// The two implementations are one line each and the line is the same line, deliberately:
        /// the fixture must not be able to stop at a ceiling the connector would run past, or the
        /// run-level cases below would be measuring the fixture. A port built without
        /// [`Self::billing`] has a default account with no ceiling and answers `false` for ever,
        /// which is what leaves every test that predates rule 14.6 running exactly as it did.
        fn halted(&self) -> bool {
            self.account.ceiling_reached()
        }

        fn propose(&mut self, request: DecisionRequest) -> Proposal {
            let answer = match self.answers.get(self.seen.len()) {
                Some(answer) => answer.clone(),
                None => self.standing.clone(),
            };
            self.seen.push(request);
            // Rule 14.1 and rule 9.5's count, in the party rule 20.4.1 puts them in and at the
            // moment the exchange happened, in the two statements
            // [`ConnectorPort::propose`](super::ConnectorPort::propose) uses: every exchange is
            // billed, and an unanswered one then counts a fallback. This double bills the declared
            // usage for an answered exchange and four absences for an unanswered one, because *this*
            // port's unanswered exchange is rule 9.4's — it reports no usage, which is not true of
            // rule 10.4c's and is why the count and the bill are two statements rather than one.
            // A rejection the engine decides later cannot reach here, which is rule 9.6.
            self.account = match &answer {
                Some(_) => self.account.after_exchange(&self.usage),
                None => self
                    .account
                    .after_exchange(&ExchangeUsage::unreported())
                    .counting_fallback(),
            };
            // Rule 1.1a's evidence. An answered exchange reports what the port declares; an
            // unanswered one reports the response only, because rule 11.3's `response` field is
            // where an exchange that yielded no action says why — and the same declared response
            // travelling with both is what lets a test assert that the record differs in the
            // `action` and `fallback` fields alone.
            let usage = match &answer {
                Some(_) => self.usage,
                None => ReportedUsage::default(),
            };
            Proposal {
                action: answer,
                response: self.response.clone(),
                usage,
                // This port answers from a list and retries nothing, so rule 19.5's field stays
                // empty: it is the shape every port but a retrying one has.
                earlier_attempts: Vec::new(),
            }
        }

        fn record(&mut self, record: &str) -> io::Result<()> {
            self.written.push(record.to_string());
            if self.fail_at == Some(self.written.len()) {
                return Err(io::Error::other("the destination refused the record"));
            }
            Ok(())
        }
    }

    /// A port that must never be consulted. Rule 20.9's test needs one: a port that is *ignored*
    /// cannot be distinguished from a port that is called and whose answer is discarded, unless
    /// being called is itself the failure.
    struct ForbiddenPort;

    impl Proposer for ForbiddenPort {
        /// Panics for the third time, and this one closes the half rule 14.6 opened: an ignored port
        /// must not be *asked* anything either. The question is asked before every opportunity, so a
        /// source that asked it without checking whose port it held would consult an ignored port at
        /// every tick of every run — while proposing nothing and writing nothing, which is a
        /// consultation no other assertion in this file could see.
        fn halted(&self) -> bool {
            panic!("rule 20.9: a port supplied under another source must be ignored, not asked");
        }

        fn propose(&mut self, _request: DecisionRequest) -> Proposal {
            panic!(
                "rule 20.9: a port supplied under another source must be ignored, not consulted"
            );
        }

        /// Panics for the same reason `propose` does, and covers the half rule 20.9 would
        /// otherwise leave open: an ignored port must receive no *record* either. A source that
        /// wrote the prefix head before checking whose port it held would have written a
        /// transcript for a run that never consults one, and only a panic here can tell that
        /// apart from silence.
        fn record(&mut self, _record: &str) -> io::Result<()> {
            panic!("rule 20.9: a port supplied under another source must receive no record");
        }
    }

    /// `SPEC-MOK-001` rule 21's seven verbs against one target, in rule 21's order.
    ///
    /// Written out rather than derived, so that an eighth targeted verb makes this list wrong in
    /// a way a reader sees rather than making the comparison it feeds quietly narrower.
    fn every_targeted_action(target: &str) -> [Action; 7] {
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

    /// A proposal carrying an action and no evidence, which is what a port with nothing behind it
    /// returns and what every case predating rule 1.1a asserted against.
    fn proposed(action: Action) -> Proposal {
        Proposal {
            action: Some(action),
            ..Proposal::nothing()
        }
    }

    /// The summary of a run that reached one of `SPEC-MOK-001`'s two endings.
    ///
    /// Every test that predates `SPEC-MOK-007` rule 14.6 asserts on a summary and cannot reach the
    /// ceiling — a deterministic source takes `DecisionSource::halted`'s default, and a scripted port
    /// declares no ceiling unless the test says so. This states that in one place instead of at each
    /// call site, and it panics rather than defaulting, so a run that stopped short fails the test it
    /// was borrowed by rather than being read as a run that finished.
    fn completed(outcome: RunOutcome) -> RunSummary {
        match outcome {
            RunOutcome::Completed(summary) => summary,
            RunOutcome::Ceiling { tick_reached } => {
                panic!("the run stopped at its spend ceiling at tick {tick_reached}")
            }
        }
    }

    fn llm_config(seed: u64, tick_limit: u64, trace_actions: bool) -> Config {
        Config {
            seed,
            tick_limit,
            policy: Policy::Llm,
            density: Density::DEFAULT,
            trace_actions,
            spend_ceiling: None,
            prices: None,
        }
    }

    /// Rule 3.1's order, and rule 3.6's reason for stating it in exactly one place.
    ///
    /// The assertion is on `blocks()` rather than on a concatenation, because the failure rule
    /// 3.6 describes is a *reordering*, and a concatenation of the four in the wrong order is a
    /// string a test of the whole prompt would still accept if it were assembled the same wrong
    /// way twice.
    #[test]
    fn the_request_carries_four_blocks_in_the_cacheable_order() {
        let simulation = Simulation::new(llm_config(42, 10, false)).unwrap();
        let request = DecisionRequest::compose(&simulation.observation(0));

        let blocks = request.blocks();
        assert_eq!(blocks.len(), 4);
        assert_eq!(blocks[0], request.shared_rules());
        assert_eq!(blocks[1], request.actor());
        assert_eq!(blocks[2], request.observation());
        assert_eq!(blocks[3], request.permitted_set());

        // Rule 3.1's assignment of content to position. Block A is the constant, B names the
        // actor, C is headed by the tick and D by the actions — so a rotation of the four, which
        // would keep every assertion above true, fails here.
        assert_eq!(blocks[0], SHARED_RULES);
        assert!(blocks[1].starts_with("YOU\n"), "{}", blocks[1]);
        assert!(
            blocks[2].starts_with("WHAT YOU SEE\ntick:"),
            "{}",
            blocks[2]
        );
        assert!(
            blocks[3].starts_with("ACTIONS YOU MAY TAKE\n"),
            "{}",
            blocks[3]
        );
    }

    /// Rule 3.3: block A is byte-identical across every request of a run, including across
    /// Mokiterions and across ticks, and rule 3.5's cache estimate rests on nothing else.
    ///
    /// Rule 5.3 is measured in the same pass: block B is byte-identical per Mokiterion per run,
    /// which is what puts it inside the cacheable prefix rather than beside the observation.
    #[test]
    fn block_a_is_one_string_for_the_run_and_block_b_is_one_per_mokiterion() {
        let mut port = ScriptedPort::silent();
        let mut simulation = Simulation::new(llm_config(42, 20, false)).unwrap();
        simulation
            .run_recording(&mut io::sink(), None, Some(&mut port))
            .unwrap();
        assert!(
            port.seen.len() > 100,
            "too few requests to be a measurement"
        );

        let mut actors: HashSet<&str> = HashSet::new();
        for request in &port.seen {
            assert_eq!(request.shared_rules(), SHARED_RULES);
            actors.insert(request.actor());
        }
        // Twelve Mokiterions, so twelve distinct block Bs and no more: a block B that carried a
        // tick or an attribute would produce one per opportunity instead, and this would be the
        // request count.
        assert_eq!(actors.len(), 12, "{actors:?}");
    }

    /// Rules 4.4 and 4.5, as the two prohibitions that make the source measure a model rather
    /// than an instruction.
    ///
    /// Rule 4.4 is the one this asserts by vocabulary, which is a weaker instrument than the
    /// rule and is used deliberately: advice can be written in words no list anticipates, so the
    /// list catches the *ordinary* way it arrives — a sentence that says one action is better
    /// than another — and the review of block A's text is the rest of the check.
    #[test]
    fn block_a_gives_no_strategy_and_names_nothing_that_varies() {
        let lowered = SHARED_RULES.to_lowercase();
        for advice in [
            "should",
            "better",
            "best",
            "prefer",
            "goal",
            "strategy",
            "survive",
            "recommend",
            "important",
            "aim to",
            "in order to",
            "so that you",
            "risky",
            "safe",
        ] {
            assert!(
                !lowered.contains(advice),
                "block A advises: it contains {advice:?}"
            );
        }

        // Rule 4.5. The identifiers of an actual run are the exact strings that must not be in
        // it, so they are taken from one rather than guessed at.
        let simulation = Simulation::new(llm_config(777, 5, false)).unwrap();
        for agent in &simulation.agents {
            assert!(
                !SHARED_RULES.contains(&agent.id),
                "block A names {}",
                agent.id
            );
        }
        for food in &simulation.foods {
            assert!(
                !SHARED_RULES.contains(&food.id),
                "block A names {}",
                food.id
            );
        }
        assert!(!lowered.contains("seed"), "block A names the seed");
        assert!(!lowered.contains("777"), "block A carries a run's figure");

        // Rule 4.3's ranges and rule 4.1's verb list, which block A must state because rule 8's
        // grammar is unreadable without them. This is the positive half: the checks above would
        // all pass on an empty string.
        for required in [
            "0 to 100",
            "0 to 40",
            "distance of 16",
            "wait",
            "sleep",
            "eat <resource>",
            "move <direction>",
            "attack <who>",
            "threaten <who>",
            "fight <who>",
            "retreat <who>",
            "surrender <who>",
            "approach <who>",
            "avoid <who>",
        ] {
            assert!(
                SHARED_RULES.contains(required),
                "block A omits {required:?}"
            );
        }
    }

    /// Rule 6: block C's fields, in rule 6.1's order, with rule 6.5's stated emptiness and rule
    /// 6.6's absence of any aggregate.
    #[test]
    fn block_c_states_the_observation_in_order_and_states_its_empty_lists() {
        let simulation = Simulation::new(llm_config(42, 10, false)).unwrap();
        let block = observation_block(&simulation.observation(0));

        let order = [
            "tick:",
            "position:",
            "health:",
            "satiety:",
            "energy:",
            "fear:",
            "attacks suffered since your previous action:",
            "resources in your cell:",
            "resources perceived:",
            "mokiterions perceived:",
        ];
        let mut cursor = 0;
        for field in order {
            let at = block[cursor..]
                .find(field)
                .unwrap_or_else(|| panic!("block C omits or misplaces {field:?}:\n{block}"));
            cursor += at + field.len();
        }

        // Rule 6.5. At tick 1 nothing has attacked anyone, so this is the case the rule is about.
        assert!(
            block.contains("attacks suffered since your previous action:\n  none\n"),
            "{block}"
        );

        // Rule 6.6. No count of anything appears, and the list headings are the place one would
        // arrive: `resources perceived: 4` reads naturally and is exactly what the rule forbids.
        for heading in [
            "resources in your cell:",
            "resources perceived:",
            "mokiterions perceived:",
        ] {
            let line = block
                .lines()
                .find(|line| line.starts_with(heading))
                .unwrap_or_else(|| panic!("{heading:?} is not a line of its own:\n{block}"));
            assert_eq!(line, heading, "the heading carries an aggregate: {line:?}");
        }
        for aggregate in ["living:", "population", "count", "total", "average", "mean"] {
            assert!(
                !block.to_lowercase().contains(aggregate),
                "block C carries an aggregate: {aggregate:?}"
            );
        }
    }

    /// Rule 6.3: a co-located entity's relative direction is a stated word, and neither an
    /// omission nor a sentinel.
    #[test]
    fn block_c_states_the_same_cell_as_a_word() {
        assert_eq!(relative_direction_form(None), "same_cell");
        assert_eq!(
            relative_direction_form(Some(RelativeDirection::NorthEast)),
            "north_east"
        );

        // The distinction the rule exists to force: `direction ` followed by nothing, or by a
        // number, would both be read as data by a parser and as an error by a reader.
        let mut observation = Simulation::new(llm_config(42, 10, false))
            .unwrap()
            .observation(0);
        observation.perceived_mokiterions = vec![PerceivedMokiterion {
            id: "M11".to_string(),
            direction: None,
            distance: 0,
        }];
        let block = observation_block(&observation);
        assert!(
            block.contains("  M11 direction same_cell distance 0\n"),
            "{block}"
        );
    }

    /// Rule 7, and rule 7.4 in particular: every targeted verb is enumerated against exactly the
    /// targets whose preconditions the observation shows to be met.
    ///
    /// The observation is built by hand because the case that decides the rule — a perceived
    /// Mokiterion in the same cell, which `approach` may not name and `avoid` may — occurs in a
    /// real run at a tick no test should have to search for.
    #[test]
    fn block_d_enumerates_a_targeted_verb_only_where_its_preconditions_are_met() {
        let mut observation = Simulation::new(llm_config(42, 10, false))
            .unwrap()
            .observation(0);
        observation.valid_actions = vec![Action::Wait, Action::Sleep];
        observation.perceived_mokiterions = vec![
            // Same cell: in contact, and the one target `approach` may not name.
            PerceivedMokiterion {
                id: "M01".to_string(),
                direction: None,
                distance: 0,
            },
            // In contact and one cell away, so every contact verb applies and `approach` does.
            PerceivedMokiterion {
                id: "M02".to_string(),
                direction: Some(RelativeDirection::East),
                distance: 1,
            },
            // Perceived but not in contact: `approach` and `avoid` only.
            PerceivedMokiterion {
                id: "M03".to_string(),
                direction: Some(RelativeDirection::South),
                distance: 9,
            },
        ];
        // `M02` struck the observer and `M03` did not, so the three answer-to-an-attack verbs
        // separate the two. `M03`'s entry additionally shows that `retreat` and `surrender` need
        // the attack and not the contact.
        observation.suffered = vec![
            SufferedAttack {
                attacker: "M02".to_string(),
                damage: 12,
            },
            SufferedAttack {
                attacker: "M03".to_string(),
                damage: 20,
            },
        ];

        assert_eq!(
            permitted_set_block(&observation),
            concat!(
                "ACTIONS YOU MAY TAKE\n",
                "  wait\n",
                "  sleep\n",
                "  attack M01\n",
                "  attack M02\n",
                "  threaten M01\n",
                "  threaten M02\n",
                "  fight M02\n",
                "  retreat M02\n",
                "  retreat M03\n",
                "  surrender M02\n",
                "  surrender M03\n",
                "  approach M02\n",
                "  approach M03\n",
                "  avoid M01\n",
                "  avoid M02\n",
                "  avoid M03\n",
            )
        );
    }

    /// Rule 7.4 again, against the engine's own validator rather than against an expectation.
    ///
    /// The test above fixes the enumeration; this one checks it agrees with the rule that will
    /// judge the proposal. Every targeted action block D offers is put to `validate_targeted`,
    /// and every one it does not offer is put to it as well — the second half is what catches an
    /// enumeration that is merely *safe*, by being empty, rather than complete.
    #[test]
    fn block_d_offers_exactly_what_the_engine_would_accept_from_the_observation() {
        let mut simulation = Simulation::new(llm_config(1, 400, true)).unwrap();
        let mut port = ScriptedPort::silent();
        let mut examined = 0;

        // A live world rather than a crafted observation, so the cases are the ones a run
        // actually reaches. One tick at a time, because the observation and the validator have to
        // be read at the same instant for the comparison to mean anything.
        for _ in 0..400 {
            if simulation.is_finished() {
                break;
            }
            for index in 0..simulation.agents.len() {
                if simulation.agents[index].health == 0 {
                    continue;
                }
                let observation = simulation.observation(index);
                let offered: HashSet<String> = permitted_set_block(&observation)
                    .lines()
                    .skip(1)
                    .map(|line| line.trim().to_string())
                    .collect();

                for other in &observation.perceived_mokiterions {
                    for action in every_targeted_action(&other.id) {
                        let accepted = simulation.validate_targeted(index, &action).is_ok();
                        let entry = permitted_form(&action);
                        assert_eq!(
                            offered.contains(&entry),
                            accepted,
                            "tick {}: {entry} is {} by the validator and {} by block D",
                            observation.tick,
                            if accepted { "accepted" } else { "rejected" },
                            if offered.contains(&entry) {
                                "offered"
                            } else {
                                "withheld"
                            }
                        );
                        examined += 1;
                    }
                }
            }
            simulation
                .advance_tick(Some(&mut port))
                .expect("the port is present");
        }

        assert!(
            examined > 500,
            "only {examined} verb-target pairs were reached, too few to be a measurement"
        );
    }

    /// Rule 20.7: the adapter draws nothing from `REQ-MOK-009`'s stream.
    ///
    /// Asserted on the draw counter and on the stream's own state, which are two readings of one
    /// fact: `SplitMix64` advances by a fixed odd constant per draw, so an unmoved state is an
    /// unmoved draw count whether or not the counter was incremented honestly.
    #[test]
    fn the_port_source_draws_no_entropy() {
        // One tick first, because rule 3's consistency invariant is a property of a started run
        // and `decide` asserts it in a debug build like every other source's does.
        let mut simulation = Simulation::new(llm_config(42, 10, false)).unwrap();
        simulation
            .advance_tick(Some(&mut ScriptedPort::silent()))
            .unwrap();
        let observation = simulation.observation(0);

        let mut port = ScriptedPort::answering(vec![Some(Action::Sleep)]);
        let mut source = PortDecisionSource::new(&mut port);
        let mut stream = simulation.entropy;
        let before = stream;
        let mut entropy = DecisionEntropy::new(&mut stream);

        assert_eq!(source.decide(&observation, &mut entropy), Action::Sleep);
        assert_eq!(entropy.draws, 0);
        assert_eq!(stream, before);

        // And the same on the fallback path, which is the branch a later change is most likely
        // to reach for a number.
        let mut port = ScriptedPort::silent();
        let mut source = PortDecisionSource::new(&mut port);
        let mut stream = simulation.entropy;
        let before = stream;
        let mut entropy = DecisionEntropy::new(&mut stream);
        assert_eq!(source.decide(&observation, &mut entropy), Action::Wait);
        assert_eq!(entropy.draws, 0);
        assert_eq!(stream, before);
    }

    /// Rule 9.5: no proposal obtained is `wait`, and rule 9.7: never another source's selection.
    #[test]
    fn no_proposal_obtained_is_wait_and_nothing_else() {
        let mut port = ScriptedPort::silent();
        let mut text = Vec::new();
        let mut simulation = Simulation::new(llm_config(42, 30, true)).unwrap();
        simulation
            .run_recording(&mut text, None, Some(&mut port))
            .unwrap();
        let text = String::from_utf8(text).unwrap();

        let traces: Vec<&str> = text
            .lines()
            .filter(|line| line.contains("event=action_trace"))
            .collect();
        assert!(!traces.is_empty());
        for trace in &traces {
            assert!(
                trace.contains("result=proposal:wait"),
                "a fallback proposed something other than wait: {trace}"
            );
        }
        assert_eq!(traces.len(), port.seen.len());
    }

    /// Rule 20.8, on both doors, and rule 20.5.1's consequence for `run`.
    #[test]
    fn this_source_with_no_port_refuses_and_runs_nothing() {
        // `run`, which rule 20.5.1 leaves unamended and which therefore never has a port.
        let mut text = Vec::new();
        let error = Simulation::new(llm_config(42, 10, false))
            .unwrap()
            .run(&mut text)
            .expect_err("rule 20.8: no port is an invalid configuration");
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert_eq!(error.to_string(), MISSING_DECISION_PORT);
        // Nothing ran: no header, no event, no summary. A refusal that had produced a stream
        // would be a run somebody could read.
        assert!(text.is_empty(), "{}", String::from_utf8_lossy(&text));

        // `advance_tick`, on the first call, with no prior state change.
        let mut simulation = Simulation::new(llm_config(42, 10, false)).unwrap();
        assert_eq!(
            simulation.advance_tick(None),
            Err(MISSING_DECISION_PORT.to_string())
        );
        assert_eq!(simulation.tick, 0);
        assert!(!simulation.is_finished());
        // Refused again on the second call rather than becoming something else on it.
        assert_eq!(
            simulation.advance_tick(None),
            Err(MISSING_DECISION_PORT.to_string())
        );
        assert_eq!(simulation.tick, 0);
    }

    /// Rule 20.9: a port supplied under one of the four existing sources is ignored, exactly as
    /// an absent sink is, and is not an error.
    #[test]
    fn a_port_under_another_source_is_ignored_and_is_not_an_error() {
        for policy in [
            Policy::Baseline,
            Policy::Reference,
            Policy::Individual,
            Policy::Social,
        ] {
            let mut without = Vec::new();
            let mut with = Vec::new();
            let config = Config {
                seed: 42,
                tick_limit: 40,
                policy,
                density: Density::DEFAULT,
                trace_actions: true,
                spend_ceiling: None,
                prices: None,
            };

            Simulation::new(config)
                .unwrap()
                .run_recording(&mut without, None, None)
                .unwrap();
            Simulation::new(config)
                .unwrap()
                .run_recording(&mut with, None, Some(&mut ForbiddenPort))
                .unwrap();

            assert_eq!(without, with, "{policy}: the port changed the run");
        }
    }

    /// Rule 20.4.1's failure, stated as the difference it makes, because the rule is about a
    /// shape the compiler accepts either way.
    ///
    /// A port built once and lent every tick sees the whole run; one rebuilt per tick sees one
    /// decision and forgets it. Here the state is the request log, and a per-tick port would
    /// leave it holding one tick's requests rather than the run's.
    #[test]
    fn a_port_lent_for_the_run_accumulates_across_ticks() {
        let mut port = ScriptedPort::silent();
        let mut simulation = Simulation::new(llm_config(42, 10, false)).unwrap();
        let mut ticks = 0;
        while !simulation.is_finished() {
            simulation.advance_tick(Some(&mut port)).unwrap();
            ticks += 1;
        }
        assert_eq!(ticks, 10);

        // Twelve Mokiterions alive for ten ticks with nothing but `wait`, so a request per
        // Mokiterion per tick and none lost between ticks.
        assert_eq!(port.seen.len(), 120);
        let mut ticks_seen: Vec<u64> = port
            .seen
            .iter()
            .map(|request| {
                request
                    .observation()
                    .lines()
                    .nth(1)
                    .and_then(|line| line.strip_prefix("tick: "))
                    .and_then(|value| value.parse().ok())
                    .expect("block C states the tick")
            })
            .collect();
        ticks_seen.dedup();
        assert_eq!(ticks_seen, (1..=10).collect::<Vec<u64>>());
    }

    /// Rule 2.5: the request is a run input, identical across two runs of the same configuration.
    ///
    /// This is what later lets a replay detect a transcript recorded against a different
    /// configuration, so it is the property and not the transcript that is asserted here.
    #[test]
    fn the_same_configuration_composes_the_same_requests() {
        let mut first = ScriptedPort::silent();
        let mut second = ScriptedPort::silent();
        Simulation::new(llm_config(123, 25, false))
            .unwrap()
            .run_recording(&mut io::sink(), None, Some(&mut first))
            .unwrap();
        Simulation::new(llm_config(123, 25, false))
            .unwrap()
            .run_recording(&mut io::sink(), None, Some(&mut second))
            .unwrap();

        assert!(!first.seen.is_empty());
        assert_eq!(first.seen, second.seen);

        // A different seed is a different run, so equal requests there would mean the request
        // carries nothing of the world.
        let mut other = ScriptedPort::silent();
        Simulation::new(llm_config(777, 25, false))
            .unwrap()
            .run_recording(&mut io::sink(), None, Some(&mut other))
            .unwrap();
        assert_ne!(first.seen, other.seen);
    }

    /// Rules 8.2 and 8.3 through the rendering they are checked against, and the reason it is not
    /// [`Action`]'s `Display`: that one drops a targeted verb's target, which `CAP-MOK-010` holds
    /// it to for the text stream and which would make a request name no action at all.
    #[test]
    fn the_port_renders_a_targeted_action_with_its_target() {
        assert_eq!(permitted_form(&Action::Wait), "wait");
        assert_eq!(permitted_form(&Action::Sleep), "sleep");
        assert_eq!(
            permitted_form(&Action::Eat {
                food_id: "F0012".to_string()
            }),
            "eat F0012"
        );
        assert_eq!(
            permitted_form(&Action::Move {
                direction: Direction::North
            }),
            "move north"
        );

        let attack = Action::Attack {
            target: "M07".to_string(),
        };
        assert_eq!(permitted_form(&attack), "attack M07");
        assert_eq!(attack.to_string(), "attack");
    }

    /// The record stream's declared version, and the two domains rule 3.2 gains a member in.
    ///
    /// `SPEC-MOK-006` rule 10.2 moves `schema` when a rule 3.2 domain gains a member, so the
    /// increment and the members are one fact and are asserted together: a stream that carried
    /// `llm` under `schema` 2 would be non-conformant, and so would one that declared 3 without
    /// it.
    #[test]
    fn the_llm_source_reaches_both_record_domains_under_schema_three() {
        assert_eq!(RECORD_SCHEMA_VERSION, 3);
        assert_eq!(Policy::Llm.to_string(), LLM_SOURCE_NAME);
        assert_eq!(Policy::parse("llm"), Some(Policy::Llm));

        let mut port = ScriptedPort::silent();
        let mut records = Vec::new();
        let mut simulation = Simulation::new(llm_config(42, 5, true)).unwrap();
        simulation
            .run_recording(&mut io::sink(), Some(&mut records), Some(&mut port))
            .unwrap();
        let records = String::from_utf8(records).unwrap();

        let header = records.lines().next().expect("a header record");
        assert!(header.contains("\"schema\":3"), "{header}");
        assert!(header.contains("\"policy\":\"llm\""), "{header}");
        assert!(
            records.contains("\"event\":\"decision_source_selected\""),
            "no decision-source event"
        );
        assert!(
            records.contains("\"source\":\"llm\""),
            "result.source does not carry the new member"
        );
    }

    // -----------------------------------------------------------------------------------
    // `SPEC-MOK-007` rules 11 and 12: the transcript the engine authors, and the replay that
    // reads one back.
    //
    // Every transcript here is a `String` and every replay reads a `&[u8]`. No test below opens
    // a file, resolves a path or names a provider, so rule 12.2's "no provider call, no socket,
    // no spawned connector, no credential read" is a property of these tests as well as of the
    // product, and `WO-MOK-025`'s *Out of scope* holds over the suite.
    // -----------------------------------------------------------------------------------

    /// A recorded run: the port that kept its transcript, and the two streams it produced.
    ///
    /// The port is moved out after the run rather than inspected during it, which is the only
    /// arrangement rule 20.4 admits: the host owns the port for the whole run and the engine
    /// borrows it, so nothing can read the transcript while it is being written.
    struct Recording {
        port: ScriptedPort,
        text: Vec<u8>,
        records: Vec<u8>,
    }

    /// One run of the model-backed source against `script`, with both streams and the transcript.
    ///
    /// `trace_actions` is on for every recording here, because rule 12.6 claims byte-identity for
    /// the matched configuration *including* the tracing selection, and the traced stream is the
    /// one that states each applied action — so a replay that produced a different action fails on
    /// the bytes rather than on a summary that happened to agree.
    fn record_a_run(seed: u64, tick_limit: u64, script: Vec<Option<Action>>) -> Recording {
        record_a_run_with(seed, tick_limit, ScriptedPort::answering(script))
    }

    /// [`record_a_run`] against a port the case built, for the cases that declare what the port
    /// reports as well as what it proposes.
    fn record_a_run_with(seed: u64, tick_limit: u64, port: ScriptedPort) -> Recording {
        let mut port = port;
        let mut text = Vec::new();
        let mut records = Vec::new();
        Simulation::new(llm_config(seed, tick_limit, true))
            .unwrap()
            .run_recording(&mut text, Some(&mut records), Some(&mut port))
            .expect("a recorded run completes");
        Recording {
            port,
            text,
            records,
        }
    }

    /// The same configuration, decided from `transcript` instead of from a script.
    fn replay(seed: u64, tick_limit: u64, transcript: &str) -> io::Result<(Vec<u8>, Vec<u8>)> {
        let mut port = ReplayPort::new(transcript.as_bytes());
        let mut text = Vec::new();
        let mut records = Vec::new();
        Simulation::new(llm_config(seed, tick_limit, true))
            .unwrap()
            .run_recording(&mut text, Some(&mut records), Some(&mut port))?;
        Ok((text, records))
    }

    /// A script that reaches every shape a record can carry: both parameterless verbs, all four
    /// directions, a resource parameter, a target parameter, and rule 9.5's no-proposal case.
    ///
    /// Cycled rather than listed, so a run of any length is covered and the script never runs out
    /// where a case still wants a proposal. Ten entries against twelve Mokiterions, so the phase
    /// shifts each tick and no Mokiterion is handed the same verb every time.
    fn varied_script(length: usize) -> Vec<Option<Action>> {
        let cycle = [
            Some(Action::Sleep),
            Some(Action::Move {
                direction: Direction::North,
            }),
            None,
            Some(Action::Wait),
            Some(Action::Move {
                direction: Direction::East,
            }),
            Some(Action::Eat {
                food_id: "F0001".to_string(),
            }),
            Some(Action::Move {
                direction: Direction::South,
            }),
            None,
            Some(Action::Move {
                direction: Direction::West,
            }),
            Some(Action::Attack {
                target: "M02".to_string(),
            }),
        ];
        (0..length)
            .map(|index| cycle[index % cycle.len()].clone())
            .collect()
    }

    /// The transcript with one record edited, by line index.
    ///
    /// By index and not by search: the two record kinds share field names on purpose, so a
    /// replacement over the whole transcript would edit the head as well when the case is about an
    /// exchange, and the case would then pass for the wrong reason.
    fn with_line_edited(
        transcript: &str,
        line_index: usize,
        edit: impl Fn(&str) -> String,
    ) -> String {
        transcript
            .lines()
            .enumerate()
            .map(|(index, line)| {
                if index == line_index {
                    format!("{}\n", edit(line))
                } else {
                    format!("{line}\n")
                }
            })
            .collect()
    }

    /// One rule 12.3 case: what a case edits in the first exchange, and what the failure must say.
    ///
    /// A named struct rather than a tuple, because a three-element tuple of a name, a closure and a
    /// message reads identically whichever way round the two strings go.
    struct MismatchCase<'case> {
        name: &'case str,
        edit: &'case dyn Fn(&str) -> String,
        expected: &'case str,
    }

    /// A `BufRead` that counts what was taken from it.
    ///
    /// It exists so that "reads nothing" can be a measurement rather than a reading of the code,
    /// and it doubles as the demonstration that `R: BufRead` is the whole of the replay port's
    /// contact with the outside world: this type performs no filesystem operation either.
    struct CountingReader<'bytes> {
        inner: io::Cursor<&'bytes [u8]>,
        reads: usize,
    }

    impl<'bytes> CountingReader<'bytes> {
        fn new(bytes: &'bytes [u8]) -> Self {
            Self {
                inner: io::Cursor::new(bytes),
                reads: 0,
            }
        }
    }

    impl io::Read for CountingReader<'_> {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            self.reads += 1;
            self.inner.read(buffer)
        }
    }

    impl BufRead for CountingReader<'_> {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            self.reads += 1;
            self.inner.fill_buf()
        }

        fn consume(&mut self, amount: usize) {
            self.inner.consume(amount);
        }
    }

    /// The escaping round trip, and the framing property that makes the record readable at all.
    #[test]
    fn the_escaping_survives_the_framing_and_round_trips() {
        for text in [
            "",
            "plain",
            "a backslash \\ and a quotation mark \"",
            "a newline\nand a tab\tand a carriage return\r",
            "a bell \u{0007} and an escape \u{001b}",
            "\\\"\\n",
            "an em dash — and a full stop.",
        ] {
            let escaped = escape_transcript_text(text);
            assert_eq!(
                unescape_transcript_text(&escaped).as_deref(),
                Some(text),
                "{text:?} did not round trip through {escaped:?}"
            );

            // One record is one line, rule 11.2, so nothing that ends a line survives escaping.
            assert!(!escaped.contains('\n'), "{escaped:?}");
            assert!(!escaped.contains('\r'), "{escaped:?}");
            assert!(!escaped.contains('\t'), "{escaped:?}");
            // And every quotation mark is escaped, which is what makes the field reader exact.
            let mut previous = None;
            for character in escaped.chars() {
                if character == '"' {
                    assert_eq!(previous, Some('\\'), "unescaped quote in {escaped:?}");
                }
                previous = Some(character);
            }
        }
    }

    /// Rule 11.4's closed-alphabet clause, measured over the blocks as they are actually worded.
    ///
    /// This is the test the escaping function arrives with, which is `SPEC-MOK-006` rule 3.4's own
    /// branch for a value that cannot be brought onto the enumerated alphabet: block A carries
    /// spaces, commas, less-than signs, apostrophes and em dashes, none of which is in that
    /// alphabet, and all four blocks are multi-line, so the alphabet does not hold and cannot.
    /// A full stop is *in* the alphabet and so is `>`, which is why this list names what was
    /// measured rather than what prose usually contains. What holds instead is stated here — the
    /// four blocks pass through the escaping unchanged in meaning and unchanged in framing.
    #[test]
    fn every_block_survives_the_escaping_unchanged() {
        let simulation = Simulation::new(llm_config(42, 10, false)).unwrap();
        let request = DecisionRequest::compose(&simulation.observation(0));

        for block in request.blocks() {
            let escaped = escape_transcript_text(block);
            assert_eq!(unescape_transcript_text(&escaped).as_deref(), Some(block));
            assert_eq!(escaped.lines().count(), 1);
            assert!(!escaped.contains('"'), "a block carried a quotation mark");

            // The `\u` branch is unreached by the blocks as worded today, and this is the
            // assertion that says so. It is handled anyway, so that the function is total over
            // `&str` rather than total over the wording — the difference between an escaping
            // function and a transformation that works until a block gains a character.
            for character in block.chars() {
                assert!(
                    !character.is_control() || character == '\n',
                    "block carries the control character {:?}",
                    character
                );
            }
        }
    }

    /// An escape this module never writes is a failure rather than a line to interpret.
    #[test]
    fn an_escape_this_module_never_writes_is_not_read_generously() {
        for broken in ["\\q", "\\", "ends with \\", "\\u00", "\\uZZZZ", "\\u{0041}"] {
            assert_eq!(
                unescape_transcript_text(broken),
                None,
                "{broken:?} was read rather than refused"
            );
        }
    }

    /// The field reader's one load-bearing property: a field name inside a value is not a field.
    ///
    /// `\"tick\":` is not `"tick":`, and the reason it can never be is that
    /// [`escape_transcript_text`] leaves no unescaped quotation mark inside any value. A record
    /// this cannot read yields `None`, and rule 12.3 turns that into a named failure rather than a
    /// default — which is why the planted values below must not be found rather than merely not
    /// preferred.
    #[test]
    fn a_field_name_inside_a_value_is_not_a_field() {
        let planted = format!(
            "{{\"transcript\":\"exchange\",\"version\":1,\"tick\":7,\"observation\":\"{}\"}}",
            escape_transcript_text("\"tick\":999 \"actor\":\"M99\" \"fallback\":true")
        );
        assert_eq!(transcript_number(&planted, "tick"), Some(7));
        assert_eq!(transcript_string(&planted, "actor"), None);
        assert_eq!(transcript_flag(&planted, "fallback"), None);
        assert_eq!(
            transcript_string(&planted, "observation").as_deref(),
            Some("\"tick\":999 \"actor\":\"M99\" \"fallback\":true")
        );

        // And on a real record, where block C's own `tick:` line and block B's `identifier:` line
        // are the values a reader would most plausibly trip over.
        let simulation = Simulation::new(llm_config(42, 10, false)).unwrap();
        let request = DecisionRequest::compose(&simulation.observation(0));
        let record = exchange_record(&request, &proposed(Action::Sleep));
        assert!(request.observation().contains("tick: "));
        assert_eq!(transcript_number(&record, "tick"), Some(request.tick()));
        assert_eq!(
            transcript_string(&record, "actor").as_deref(),
            Some(request.actor_id())
        );
        assert_eq!(transcript_flag(&record, "fallback"), Some(false));

        // An absent field is `None` and not a zero, a false or an empty string, which is what
        // lets rule 12.3 tell "no tick" from "tick 0".
        assert_eq!(transcript_number(&record, "usage_prompt"), None);
        assert_eq!(transcript_string(&record, "response_text"), None);
        assert_eq!(transcript_flag(&record, "ceiling"), None);
    }

    /// Rule 8.2's closed grammar, both ways, over all eleven verbs.
    #[test]
    fn the_action_grammar_round_trips_and_refuses_what_it_does_not_admit() {
        let mut every = vec![Action::Wait, Action::Sleep];
        every.push(Action::Eat {
            food_id: "F0007".to_string(),
        });
        for direction in Direction::ORDERED {
            every.push(Action::Move { direction });
        }
        every.extend(every_targeted_action("M07"));

        let mut verbs: HashSet<&str> = HashSet::new();
        for action in &every {
            let (verb, parameter) = action_parts(action);
            verbs.insert(verb);
            assert_eq!(
                action_from_parts(verb, parameter.as_deref()).as_ref(),
                Some(action),
                "{verb} did not round trip"
            );

            // The verb and the parameter are block D's own rendering, split at its one space, so
            // an action a transcript states is an action block D could have offered — spelled the
            // way it was offered rather than in a second spelling kept in agreement by hand.
            let offered = permitted_form(action);
            let mut parts = offered.splitn(2, ' ');
            assert_eq!(parts.next(), Some(verb));
            assert_eq!(parts.next().map(str::to_string), parameter);
        }
        assert_eq!(verbs.len(), 11, "{verbs:?}");

        // Rule 8.2 again, as the refusals: an unknown verb, a verb given a parameter it does not
        // take, a verb missing the one it does, and a direction outside the four.
        for (verb, parameter) in [
            ("dance", None),
            ("", None),
            ("Wait", None),
            ("wait", Some("M07")),
            ("sleep", Some("M07")),
            ("eat", None),
            ("move", None),
            ("move", Some("northeast")),
            ("move", Some("NORTH")),
            ("attack", None),
        ] {
            assert_eq!(
                action_from_parts(verb, parameter),
                None,
                "{verb} {parameter:?} was admitted"
            );
        }
    }

    /// Rule 11.1's prefix head: one record per Mokiterion, before the first exchange.
    #[test]
    fn the_head_declares_every_mokiterion_once_in_identifier_order() {
        let recording = record_a_run(42, 20, varied_script(400));
        let prefixes = recording.port.prefixes();
        assert_eq!(prefixes.len(), 12);

        // The head is the transcript's first twelve records and nothing is interleaved with it,
        // which is what lets a reader take the prefixes without reading the whole file.
        for (index, prefix) in prefixes.iter().enumerate() {
            assert_eq!(*prefix, &recording.port.written[index]);
        }

        // Ascending identifier order, imposed by the roster rather than inherited from the agent
        // vector's order. Every Mokiterion the run created is declared, whether or not it ever
        // decided: the head is built from the roster, so the transcript's contents cannot depend
        // on the run's outcome.
        let declared: Vec<String> = prefixes
            .iter()
            .map(|prefix| transcript_string(prefix, "actor").expect("a prefix names its actor"))
            .collect();
        let mut sorted = declared.clone();
        sorted.sort();
        assert_eq!(declared, sorted, "the head is not in identifier order");

        let simulation = Simulation::new(llm_config(42, 20, true)).unwrap();
        let mut expected: Vec<String> = simulation.agents.iter().map(|a| a.id.clone()).collect();
        expected.sort();
        assert_eq!(declared, expected);

        // The head's two blocks and its digest are the prefix the exchanges were actually sent
        // against. The arguments come from a request the engine composed, not from a second call
        // to the same helpers, so a head that declared a prefix nobody sent fails here.
        let request = &recording.port.seen[0];
        let head = prefixes
            .iter()
            .find(|prefix| {
                transcript_string(prefix, "actor").as_deref() == Some(request.actor_id())
            })
            .expect("the head declares the first actor");
        assert!(head.contains(&escape_transcript_text(request.shared_rules())));
        assert!(head.contains(&escape_transcript_text(request.actor())));
        assert_eq!(
            transcript_string(head, "digest"),
            Some(prefix_digest(request.shared_rules(), request.actor()))
        );
        assert_eq!(
            transcript_number(head, "version"),
            Some(u64::from(TRANSCRIPT_VERSION))
        );
        assert_eq!(head.lines().count(), 1);

        // Twelve distinct digests, because block B differs per Mokiterion. One digest for all
        // twelve would mean the prefix carried nothing of the actor and rule 12.3's digest check
        // would then pass for a record taken against any of them.
        let digests: HashSet<String> = prefixes
            .iter()
            .map(|prefix| transcript_string(prefix, "digest").expect("a digest"))
            .collect();
        assert_eq!(digests.len(), 12);
    }

    /// Rule 11.2: one record per exchange, in the order the run made them, bound to its
    /// opportunity — and nothing of the transcript in either of the run's own streams.
    #[test]
    fn one_exchange_is_one_record_naming_its_opportunity() {
        let recording = record_a_run(42, 20, varied_script(400));
        let exchanges = recording.port.exchanges();
        assert_eq!(exchanges.len(), recording.port.seen.len());
        assert_eq!(recording.port.written.len(), 12 + exchanges.len());
        assert!(exchanges.len() > 200, "{} exchanges", exchanges.len());

        for (record, request) in exchanges.iter().zip(&recording.port.seen) {
            let record = record.as_str();
            assert_eq!(record.lines().count(), 1);
            assert_eq!(
                transcript_number(record, "version"),
                Some(u64::from(TRANSCRIPT_VERSION))
            );
            assert_eq!(transcript_number(record, "tick"), Some(request.tick()));
            assert_eq!(
                transcript_string(record, "actor").as_deref(),
                Some(request.actor_id())
            );
            assert_eq!(
                transcript_string(record, "prefix").as_deref(),
                Some(request.actor_id())
            );
            assert_eq!(
                transcript_string(record, "prefix_digest"),
                Some(prefix_digest(request.shared_rules(), request.actor()))
            );
            assert_eq!(
                transcript_string(record, "observation").as_deref(),
                Some(request.observation())
            );
            assert_eq!(
                transcript_string(record, "permitted").as_deref(),
                Some(request.permitted_set())
            );

            // What makes the split sound, and rule 3.4 is why it is sound: blocks A and B are
            // invariant, so a record repeats neither. A record that carried them would put a
            // measured 5,620 bytes of repetition behind a measured mean of 996.
            assert!(!record.contains(&escape_transcript_text(request.shared_rules())));
            assert!(!record.contains(&escape_transcript_text(request.actor())));
        }

        // The transcript reaches neither of the run's streams. `open` runs before the header
        // record, so a prefix that leaked would be the record stream's first line.
        let text = String::from_utf8(recording.text.clone()).unwrap();
        let records = String::from_utf8(recording.records.clone()).unwrap();
        assert!(!text.contains("\"transcript\":"));
        assert!(!records.contains("\"transcript\":"));
    }

    /// Rules 11.5 and 11.6: what a record does not carry.
    ///
    /// Rule 11.3's response text and four token counts are unobtainable from a port whose
    /// `propose` returns `Option<Action>`, so both fields are recorded **absent** — `null`, not
    /// zero, which rule 11.5 requires and rule 14.5 depends on. They arrive with the connector in
    /// `WO-MOK-026`; a `0` written today would be a count nobody reported.
    #[test]
    fn a_record_states_no_response_no_usage_and_no_credential() {
        let recording = record_a_run(42, 10, varied_script(200));
        for record in recording.port.exchanges() {
            assert!(record.contains("\"response\":null"), "{record}");
            assert!(record.contains("\"usage\":null"), "{record}");
            // Absent, not zero: there is no per-count field for a zero to arrive in. Each needle
            // begins with the quotation mark that opens a field name, and
            // `escape_transcript_text` leaves no unescaped quotation mark inside any value, so a
            // match here can only be a field and never block C's or D's text.
            for count in [
                "\"prompt",
                "\"cached",
                "\"output",
                "\"reasoning",
                "\"usage_",
            ] {
                assert!(
                    !record.contains(count),
                    "a token count field leaked: {record}"
                );
            }
        }

        // Rule 11.6, over the whole transcript rather than over the fields, because the clause is
        // about what a retained file contains and not about a field list.
        let lowered = recording.port.transcript().to_lowercase();
        for forbidden in [
            "authorization",
            "bearer",
            "api_key",
            "apikey",
            "api-key",
            "credential",
            "secret",
            "token",
            "password",
            "openai",
            "gpt-",
            "sk-",
            "http",
        ] {
            assert!(
                !lowered.contains(forbidden),
                "the transcript contains {forbidden:?}"
            );
        }
    }

    /// Rule 12.7's basis: a proposed `wait` and a fallback `wait` are different facts.
    ///
    /// Recorded as its own field rather than inferred from the action, because `wait` is a
    /// proposal a source may legitimately make. The two must replay differently — one moves
    /// `REQ-MOK-074`'s count and the other does not — so a record showing `wait` with no flag
    /// would be indistinguishable from an exchange that yielded nothing.
    #[test]
    fn a_proposed_wait_and_a_fallback_wait_are_distinguishable() {
        let script: Vec<Option<Action>> = (0..12).map(|_| Some(Action::Wait)).collect();
        let recording = record_a_run(42, 3, script);
        let exchanges = recording.port.exchanges();
        assert!(exchanges.len() >= 24, "{} exchanges", exchanges.len());

        for (index, record) in exchanges.iter().enumerate() {
            let record = record.as_str();
            assert!(record.contains("\"verb\":\"wait\""), "{record}");
            assert!(!record.contains("\"parameter\""), "{record}");
            assert_eq!(
                transcript_flag(record, "fallback"),
                Some(index >= 12),
                "exchange {index} states the wrong fallback"
            );
        }
    }

    /// Rule 19.6: a transcript that cannot be written ends the run with an error.
    ///
    /// A live run whose exchanges were spent and not recorded has produced cost and no evidence,
    /// which is the one failure worth aborting for. The diagnostic names the transcript, on
    /// `sink_error`'s precedent, so a reader can tell it from the engine's own failure.
    #[test]
    fn a_transcript_that_cannot_be_written_ends_the_run() {
        // The thirteenth record is the first exchange: the head is written, the first opportunity
        // is recorded, and the write is then refused.
        let mut port = ScriptedPort::failing_to_record_at(13);
        let mut text = Vec::new();
        let error = Simulation::new(llm_config(42, 10, true))
            .unwrap()
            .run_recording(&mut text, None, Some(&mut port))
            .expect_err("rule 19.6: a transcript that cannot be written ends the run");
        assert!(error.to_string().starts_with("transcript: "), "{error}");
        assert_eq!(port.written.len(), 13);

        // The run stopped at the opportunity and not after the tick, so it produced no summary.
        let text = String::from_utf8(text).unwrap();
        assert!(!text.contains("summary reason="), "{text}");

        // A head that cannot be written refuses before the header record, so the run's own
        // streams hold nothing at all — the same shape rule 20.8's refusal has.
        let mut port = ScriptedPort::failing_to_record_at(1);
        let mut text = Vec::new();
        let mut records = Vec::new();
        let error = Simulation::new(llm_config(42, 10, true))
            .unwrap()
            .run_recording(&mut text, Some(&mut records), Some(&mut port))
            .expect_err("a head that cannot be written is the same failure");
        assert!(error.to_string().starts_with("transcript: "), "{error}");
        assert_eq!(port.written.len(), 1);
        assert!(text.is_empty(), "{}", String::from_utf8_lossy(&text));
        assert!(records.is_empty(), "{}", String::from_utf8_lossy(&records));
    }

    /// Rule 12.6: a replay of a matched configuration produces the recorded run's bytes.
    ///
    /// Both streams, byte for byte, with tracing on — so every applied action is compared and not
    /// only the totals. This is the property rules 12.1 and 12.1.1 exist to make structural: the
    /// records are authored by the same lines in both directions, so there is no second writer to
    /// hold in agreement with the first.
    #[test]
    fn a_replay_reproduces_the_recorded_runs_bytes() {
        let recording = record_a_run(42, 20, varied_script(400));
        let transcript = recording.port.transcript();

        // Not a vacuous comparison: the recorded run proposed something other than the fallback at
        // every shape a record can carry, so a replay that fell back everywhere would differ at
        // these lines rather than agreeing on a summary.
        let text = String::from_utf8(recording.text.clone()).unwrap();
        for proposed in [
            "proposal:wait",
            "proposal:sleep",
            "proposal:move",
            "proposal:eat",
            "proposal:attack,target:M02",
        ] {
            assert!(
                text.contains(proposed),
                "the recording never made {proposed}"
            );
        }
        assert!(!recording.records.is_empty());

        let (replayed_text, replayed_records) =
            replay(42, 20, &transcript).expect("a matched replay completes");
        assert_eq!(replayed_text, recording.text);
        assert_eq!(replayed_records, recording.records);

        // The framing is read and not the file's line endings: a transcript checked out with
        // `CRLF` replays identically, which is what `.gitattributes` cannot promise for a file a
        // host opens in text mode on Windows.
        let (crlf_text, crlf_records) =
            replay(42, 20, &transcript.replace('\n', "\r\n")).expect("a CRLF transcript replays");
        assert_eq!(crlf_text, recording.text);
        assert_eq!(crlf_records, recording.records);

        // Blank lines are skipped rather than read as records, so a transcript concatenated from
        // two captures with a separating newline is still one transcript.
        let (spaced_text, _) =
            replay(42, 20, &transcript.replace('\n', "\n\n")).expect("blank lines are skipped");
        assert_eq!(spaced_text, recording.text);

        // And a different seed is a different run, so the same transcript must not satisfy it.
        // Rule 12.3's whole purpose: a transcript from another configuration is detected rather
        // than producing a plausible wrong run.
        let error = replay(777, 20, &transcript).expect_err("rule 12.3");
        assert!(error.to_string().starts_with("transcript: "), "{error}");
    }

    /// A port that forwards to another and keeps every record the engine handed it.
    ///
    /// Rule 11.8 has a replay write no transcript, so the records a replay *authors* have no
    /// destination and cannot otherwise be seen. Rule 12.6's byte-identity is a claim about exactly
    /// those bytes, and the streams the cases above compare are the text and the record stream
    /// rather than the transcript itself.
    struct RecordKeeper<P: Proposer> {
        inner: P,
        written: Vec<String>,
    }

    impl<P: Proposer> RecordKeeper<P> {
        fn new(inner: P) -> Self {
            Self {
                inner,
                written: Vec::new(),
            }
        }

        /// What a recording host would have written, in [`ScriptedPort::transcript`]'s framing so
        /// that the two are comparable as whole files.
        fn transcript(&self) -> String {
            self.written
                .iter()
                .map(|record| format!("{record}\n"))
                .collect()
        }
    }

    impl<P: Proposer> Proposer for RecordKeeper<P> {
        /// Forwarded, on `LentPort`'s reasoning in the observer host: a wrapper that answered rule
        /// 14.6's question itself would be a wrapper that changed what the run does.
        fn halted(&self) -> bool {
            self.inner.halted()
        }

        fn propose(&mut self, request: DecisionRequest) -> Proposal {
            self.inner.propose(request)
        }

        fn record(&mut self, record: &str) -> io::Result<()> {
            self.written.push(record.to_string());
            self.inner.record(record)
        }
    }

    /// Rule 11.3 and rule 1.1a: the record carries the response as received and the provider's four
    /// reported counts, because the port's return now carries them to the engine that authors it.
    #[test]
    fn a_record_carries_the_response_and_the_reported_counts() {
        let response = "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"},\"model\":\"m\",\
                        \"reasoning\":\"none\",\"usage\":{\"prompt\":6000,\"cached_prompt\":5400,\
                        \"output\":40,\"reasoning\":0}}";
        let recording = record_a_run_with(
            42,
            3,
            ScriptedPort::always(Action::Sleep)
                .billing(declared_prices(), None, synthetic_usage())
                .receiving(response),
        );

        let exchanges = recording.port.exchanges();
        assert!(!exchanges.is_empty());
        for record in &exchanges {
            assert!(
                record.contains(
                    "\"usage\":{\"prompt\":6000,\"cached_prompt\":5400,\"output\":40,\
                     \"reasoning\":0}"
                ),
                "{record}"
            );
            // "As received, in full", which for a response carrying quotation marks means the value
            // is escaped and is therefore a string rather than a nested object. The record's own
            // `usage` is the only object a field reader can find, and the four counts below are
            // read past the response to prove it.
            assert_eq!(
                transcript_string(record, "response").as_deref(),
                Some(response)
            );
            assert_eq!(transcript_number(record, "prompt"), Some(6_000));
            assert_eq!(transcript_number(record, "cached_prompt"), Some(5_400));
            assert_eq!(transcript_number(record, "output"), Some(40));
            assert_eq!(transcript_number(record, "reasoning"), Some(0));
        }

        // The counts billed and the counts recorded are the same figures, which is case **P4** from
        // the record's side: the port reports one usage and it reaches both.
        assert_eq!(
            recording.port.account.cost(),
            SYNTHETIC_COST * exchanges.len() as u64
        );
    }

    /// Rule 11.5: an unreported count is **absent, not zero**, at both levels of absence.
    #[test]
    fn an_unreported_count_is_absent_and_not_zero() {
        // All four absent: the field itself is the format's absence. This is how every record
        // authored before a provider existed states it, and it is why those transcripts replay
        // byte-identically under rule 12.6 rather than needing a version bump.
        let recording = record_a_run_with(42, 2, ScriptedPort::always(Action::Wait));
        let empty = recording.port.exchanges();
        assert!(!empty.is_empty());
        for record in &empty {
            assert!(
                record.contains("\"response\":null,\"usage\":null,"),
                "{record}"
            );
        }

        // One of the four absent: the object states all four and the missing one is `null`, so a
        // reader can tell an unreported count from a field this module forgot to write. Rule 14.5
        // acts on exactly this record — the exchange succeeded and was billed, and what is missing
        // is one figure rather than the response.
        let recording = record_a_run_with(
            42,
            2,
            ScriptedPort::always(Action::Wait).billing(
                declared_prices(),
                None,
                ExchangeUsage::without_cached_figure(6_000, 40),
            ),
        );
        let partial = recording.port.exchanges();
        assert!(!partial.is_empty());
        for record in &partial {
            assert!(
                record.contains(
                    "\"usage\":{\"prompt\":6000,\"cached_prompt\":null,\"output\":40,\
                     \"reasoning\":0}"
                ),
                "{record}"
            );
            assert_eq!(transcript_number(record, "cached_prompt"), None);
            assert_eq!(transcript_number(record, "prompt"), Some(6_000));
        }
        // And the absence survived to the end of the run, which is rule 14.5's failure to evaluate
        // rather than a ratio computed from a zero.
        assert_eq!(recording.port.account.cache_ratio_basis_points(), None);
    }

    /// Rule 11.4.1's round trip, over the one string value this module does not compose itself.
    ///
    /// A response arrives from a program rule 10.7 declares untrusted in whole, so the input worth
    /// testing is the one that attacks the framing: quotation marks, a backslash, a newline, control
    /// characters, and the field names the record's own readers look for. The counts and the flag
    /// asserted below are written *after* the response on the record, so a response that escaped its
    /// value would be found first and these would read its planted figures.
    #[test]
    fn a_response_survives_the_framing_and_names_no_field() {
        let hostile =
            "\"usage\":{\"prompt\":999},\"fallback\":true, a \\ backslash\nand \u{7}\u{1b} too";
        let recording = record_a_run_with(
            42,
            2,
            ScriptedPort::always(Action::Sleep)
                .billing(declared_prices(), None, ExchangeUsage::reported(1, 0, 2, 3))
                .receiving(hostile),
        );

        let exchanges = recording.port.exchanges();
        assert!(!exchanges.is_empty());
        for record in &exchanges {
            assert_eq!(
                transcript_string(record, "response").as_deref(),
                Some(hostile)
            );
            assert_eq!(transcript_number(record, "prompt"), Some(1));
            assert_eq!(transcript_number(record, "cached_prompt"), Some(0));
            assert_eq!(transcript_flag(record, "fallback"), Some(false));
            // Rule 11.2's one line per exchange, which a raw newline in a response would break.
            assert!(!record.contains('\n'), "{record}");
        }
    }

    /// Rule 12.6 over rule 1.1a's two new fields: a replay re-authors the evidence it read.
    ///
    /// This is the half of byte-identity these fields could break, and the only half that is not
    /// structural. Every other field on a replayed record is composed from the run — the tick, the
    /// actor, the blocks, the digest — so a replay that reached the same opportunity states them
    /// again by construction. The response and the counts came from outside the engine, and they
    /// appear on a replayed record only because [`ReplayPort`] read them back out of the record it
    /// replaced. The whole transcript is compared, head included.
    #[test]
    fn a_replay_re_authors_the_evidence_the_recording_carried() {
        let response = "{\"note\":\"a \\\\ backslash, a \\\" quote and a\nnewline\",\
                        \"usage\":{\"prompt\":9}}";
        let recording = record_a_run_with(
            42,
            10,
            ScriptedPort::answering(varied_script(200))
                .billing(declared_prices(), None, synthetic_usage())
                .receiving(response),
        );
        let transcript = recording.port.transcript();

        // Not a vacuous comparison. The recording carried both fields, on records of both kinds:
        // rule 12.7's recorded fallback carries a response too, and it is the ill-formed response
        // that a reader most needs to still be there.
        assert!(
            transcript.contains("\"cached_prompt\":5400"),
            "{transcript}"
        );
        let recorded = recording.port.exchanges();
        assert!(
            recorded
                .iter()
                .any(|record| record.contains("\"fallback\":true"))
        );
        for record in &recorded {
            assert_eq!(
                transcript_string(record, "response").as_deref(),
                Some(response)
            );
        }

        let mut keeper = RecordKeeper::new(ReplayPort::new(transcript.as_bytes()));
        let mut text = Vec::new();
        let mut records = Vec::new();
        Simulation::new(llm_config(42, 10, true))
            .unwrap()
            .run_recording(&mut text, Some(&mut records), Some(&mut keeper))
            .expect("a matched replay completes");

        assert_eq!(text, recording.text);
        assert_eq!(records, recording.records);
        assert_eq!(keeper.transcript(), transcript);
    }

    /// Rule 20.4: the port performs no filesystem operation, and reads nothing at construction.
    ///
    /// A run refused before its first tick must not have consumed a byte, and rule 20.8's refusal
    /// happens after the port is built — so construction reading the head would leave a refused
    /// run having read a file.
    #[test]
    fn a_replay_reads_nothing_until_the_run_needs_a_record() {
        let recording = record_a_run(42, 5, varied_script(100));
        let transcript = recording.port.transcript();
        let mut counter = CountingReader::new(transcript.as_bytes());

        {
            let mut port = ReplayPort::new(&mut counter);
            assert_eq!(port.served, 0);
            // Rule 11.8: a record handed to a replay is accepted and changes nothing, including
            // before anything has been read.
            assert!(port.record("{\"transcript\":\"exchange\"}").is_ok());
        }
        assert_eq!(counter.reads, 0, "construction read the transcript");

        // The same reader, now lent to a whole run: it is read, and it is the only thing the port
        // ever touched. `&mut CountingReader` satisfies `R: BufRead`, which is the genericity rule
        // 12.1.1 relies on — each host supplies its own open stream and nothing else.
        {
            let mut port = ReplayPort::new(&mut counter);
            let mut text = Vec::new();
            Simulation::new(llm_config(42, 5, true))
                .unwrap()
                .run_recording(&mut text, None, Some(&mut port))
                .expect("the replay completes");
            assert_eq!(text, recording.text);
            assert!(port.served > 0);
        }
        assert!(counter.reads > 0, "the run read nothing");
    }

    /// Rule 12.3, case by case: before using a record the replay checks it is this opportunity's.
    ///
    /// Each case edits exactly one field of the first exchange and asserts the run fails naming
    /// both the opportunity, which rule 19.4 requires, and the mismatch. The edit is asserted to
    /// have changed the transcript, so a case whose needle stopped matching fails rather than
    /// passing against an unmodified file.
    #[test]
    fn a_replay_refuses_a_record_that_is_not_the_opportunity_reached() {
        let recording = record_a_run(42, 5, varied_script(100));
        let transcript = recording.port.transcript();
        let first_exchange = recording.port.prefixes().len();
        let request = &recording.port.seen[0];
        let actor = request.actor_id().to_string();
        let digest = prefix_digest(request.shared_rules(), request.actor());
        let actor_field = format!("\"actor\":\"{actor}\",");
        let prefix_field = format!("\"prefix\":\"{actor}\",");
        let digest_field = format!("\"prefix_digest\":\"{digest}\"");

        let cases = [
            MismatchCase {
                name: "another tick",
                edit: &|line: &str| line.replace("\"tick\":1,", "\"tick\":2,"),
                expected: "record is for tick 2",
            },
            MismatchCase {
                name: "no tick",
                edit: &|line: &str| line.replace("\"tick\":1,", ""),
                expected: "record states no tick",
            },
            MismatchCase {
                name: "another actor",
                edit: &|line: &str| line.replace(&actor_field, "\"actor\":\"M09\","),
                expected: "record is for actor M09",
            },
            MismatchCase {
                name: "no actor",
                edit: &|line: &str| line.replace(&actor_field, ""),
                expected: "record states no actor",
            },
            MismatchCase {
                name: "another prefix",
                edit: &|line: &str| line.replace(&prefix_field, "\"prefix\":\"M09\","),
                expected: "record names prefix M09",
            },
            MismatchCase {
                name: "no prefix",
                edit: &|line: &str| line.replace(&prefix_field, ""),
                expected: "record states no prefix",
            },
            MismatchCase {
                name: "another prefix digest",
                edit: &|line: &str| {
                    line.replace(
                        &digest_field,
                        "\"prefix_digest\":\"fnv1a64:0000000000000000\"",
                    )
                },
                expected: "changed since the recording",
            },
            MismatchCase {
                name: "another transcript version",
                edit: &|line: &str| line.replace("\"version\":1,", "\"version\":2,"),
                expected: "record is transcript version 2",
            },
            MismatchCase {
                name: "no transcript version",
                edit: &|line: &str| line.replace("\"version\":1,", ""),
                expected: "record states no version",
            },
            MismatchCase {
                name: "a verb outside the grammar",
                edit: &|line: &str| line.replace("\"verb\":\"sleep\"", "\"verb\":\"dance\""),
                expected: "no action this engine's grammar admits",
            },
        ];
        assert_eq!(cases.len(), 10);

        for case in cases {
            let name = case.name;
            let edited = with_line_edited(&transcript, first_exchange, case.edit);
            assert_ne!(edited, transcript, "{name}: the edit changed nothing");

            let error = replay(42, 5, &edited).expect_err(name);
            let message = error.to_string();
            assert!(message.contains(case.expected), "{name}: {message}");
            assert!(message.starts_with("transcript: "), "{name}: {message}");
            // Rule 19.4: the opportunity, named.
            assert!(message.contains("tick 1 actor "), "{name}: {message}");
        }
    }

    /// The digest half of rule 12.3, which no rule spelled out and which is the half that catches
    /// an edit to block A.
    ///
    /// Without it a transcript recorded under different shared rules would replay silently, every
    /// tick and actor matching, against prompts nobody ever sent. Two cases: a head that declares
    /// a prefix the records were not taken against, and no head at all.
    #[test]
    fn a_replay_refuses_a_transcript_recorded_against_another_prefix() {
        let recording = record_a_run(42, 5, varied_script(100));
        let transcript = recording.port.transcript();
        let request = &recording.port.seen[0];
        let head_line = recording
            .port
            .prefixes()
            .iter()
            .position(|prefix| {
                transcript_string(prefix, "actor").as_deref() == Some(request.actor_id())
            })
            .expect("the head declares the first actor");

        // The head's declared digest, altered. Every field a rule spells out still matches — the
        // tick, the actor, the prefix — and only the digest catches it.
        let declared = prefix_digest(request.shared_rules(), request.actor());
        let edited = with_line_edited(&transcript, head_line, |line| {
            line.replace(
                &format!("\"digest\":\"{declared}\""),
                "\"digest\":\"fnv1a64:ffffffffffffffff\"",
            )
        });
        assert_ne!(edited, transcript);
        let message = replay(42, 5, &edited).expect_err("rule 12.3").to_string();
        assert!(message.contains("the head declares prefix"), "{message}");
        assert!(message.contains("tick 1 actor "), "{message}");

        // No head at all. The exchanges are untouched and still name their opportunity, so the
        // failure has to come from the prefix the run needed and the transcript never declared.
        let headless: String = transcript
            .lines()
            .filter(|line| transcript_string(line, "transcript").as_deref() != Some("prefix"))
            .map(|line| format!("{line}\n"))
            .collect();
        let message = replay(42, 5, &headless).expect_err("rule 12.3").to_string();
        assert!(
            message.contains("declares no prefix for this actor"),
            "{message}"
        );
    }

    /// Rule 12.4: the transcript ended before the run did.
    ///
    /// The run does not shorten, rule 9.5's fallback is not applied and no rule-based proposal is
    /// substituted — every one of those would produce a plausible wrong run.
    #[test]
    fn a_transcript_that_ends_early_fails_and_names_the_opportunity() {
        let recording = record_a_run(42, 20, varied_script(400));
        let transcript = recording.port.transcript();
        let head = recording.port.prefixes().len();

        // Kept: the head and the first two ticks' exchanges, counted from the run's own requests
        // rather than assumed, so the case does not depend on how many Mokiterions were alive.
        let kept = recording
            .port
            .seen
            .iter()
            .take_while(|request| request.tick() <= 2)
            .count();
        assert!(kept > 0);
        let truncated: String = transcript
            .lines()
            .take(head + kept)
            .map(|line| format!("{line}\n"))
            .collect();

        let message = replay(42, 20, &truncated)
            .expect_err("rule 12.4")
            .to_string();
        assert!(
            message.contains(&format!("the transcript ended after {kept} exchange(s)")),
            "{message}"
        );
        assert!(message.contains("tick 3 actor "), "{message}");
        assert!(message.contains("does not shorten the run"), "{message}");

        // A transcript that is nothing but a head is the same failure at the first opportunity.
        let head_only: String = transcript
            .lines()
            .take(head)
            .map(|line| format!("{line}\n"))
            .collect();
        let message = replay(42, 20, &head_only)
            .expect_err("rule 12.4")
            .to_string();
        assert!(
            message.contains("the transcript ended after 0 exchange(s)"),
            "{message}"
        );
        assert!(message.contains("tick 1 actor "), "{message}");
    }

    /// Rule 12.5: a transcript longer than the run needs leaves the surplus unread.
    ///
    /// The tail planted here is deliberately one a reader would fail on — a record for a tick past
    /// the horizon, and a line that is not a record at all — so a pass means unread rather than
    /// tolerated.
    #[test]
    fn a_surplus_tail_is_unread_and_the_run_is_unaffected() {
        let recording = record_a_run(42, 20, varied_script(400));
        let mut longer = recording.port.transcript();
        longer.push_str("{\"transcript\":\"exchange\",\"version\":9,\"tick\":9999}\n");
        longer.push_str("not a record at all\n");

        let (text, records) = replay(42, 20, &longer).expect("rule 12.5");
        assert_eq!(text, recording.text);
        assert_eq!(records, recording.records);
    }

    /// Rule 12.7: a record whose exchange yielded nothing replays as the fallback.
    ///
    /// The flag governs and the action does not. Asserted by editing a fallback record's verb to
    /// something the run would visibly have applied: the replay must ignore it. The converse is
    /// asserted in the same pass, because otherwise the first half would also pass if the verb
    /// were ignored everywhere.
    #[test]
    fn a_recorded_fallback_replays_as_the_fallback() {
        let recording = record_a_run(42, 5, varied_script(100));
        let transcript = recording.port.transcript();
        let head = recording.port.prefixes().len();
        let exchanges = recording.port.exchanges();

        let fallback = exchanges
            .iter()
            .position(|record| transcript_flag(record, "fallback") == Some(true))
            .expect("the script yields nothing somewhere");
        let edited = with_line_edited(&transcript, head + fallback, |line| {
            line.replace("\"verb\":\"wait\"", "\"verb\":\"sleep\"")
        });
        assert_ne!(edited, transcript);
        let (text, records) = replay(42, 5, &edited).expect("rule 12.7");
        assert_eq!(text, recording.text);
        assert_eq!(records, recording.records);

        // And a record that did carry a proposal is replayed from its verb, so the assertion above
        // is about the flag rather than about the verb being unread.
        let proposed = exchanges
            .iter()
            .position(|record| {
                transcript_flag(record, "fallback") == Some(false)
                    && record.contains("\"verb\":\"sleep\"")
            })
            .expect("the script proposes sleep somewhere");
        let edited = with_line_edited(&transcript, head + proposed, |line| {
            line.replace("\"verb\":\"sleep\"", "\"verb\":\"wait\"")
        });
        let (text, _) = replay(42, 5, &edited).expect("a grammatical verb replays");
        assert_ne!(text, recording.text);
    }

    /// Rule 12.3's "produces no further ticks", as a property of the port.
    ///
    /// Through the observer's door, which is where a host can ignore an error and call again. The
    /// failure is latched and not cleared, so the same reason is reported for every subsequent
    /// exchange rather than the run resuming at the next tick.
    #[test]
    fn a_replay_failure_is_latched_and_produces_no_further_ticks() {
        let recording = record_a_run(42, 10, varied_script(200));
        let head = recording.port.prefixes().len();
        let broken = with_line_edited(&recording.port.transcript(), head, |line| {
            line.replace("\"tick\":1,", "\"tick\":4,")
        });

        let mut port = ReplayPort::new(broken.as_bytes());
        let mut simulation = Simulation::new(llm_config(42, 10, true)).unwrap();
        let first = simulation
            .advance_tick(Some(&mut port))
            .expect_err("rule 12.3");
        assert!(first.contains("record is for tick 4"), "{first}");

        for attempt in 0..3 {
            let again = simulation
                .advance_tick(Some(&mut port))
                .expect_err("the failure is latched");
            assert_eq!(again, first, "attempt {attempt} reported something else");
        }
        // Not finished, because it did not end: a latched failure is a refusal to continue and
        // not a termination reason a host could present as a result.
        assert!(!simulation.is_finished());
        assert!(simulation.termination_reason().is_none());
    }

    /// Rule 11.8: a replay writes no transcript. It has one; it is reading it.
    #[test]
    fn a_replay_writes_no_transcript() {
        let recording = record_a_run(42, 5, varied_script(100));
        let transcript = recording.port.transcript();
        let mut port = ReplayPort::new(transcript.as_bytes());
        Simulation::new(llm_config(42, 5, true))
            .unwrap()
            .run_recording(&mut io::sink(), None, Some(&mut port))
            .expect("the replay completes");

        // Every record the run authored was handed to this port and accepted, and it is still
        // prepared to accept another: nothing accumulated, because there is nowhere for it to.
        // The engine authored them because rule 12.1 requires the same code path, which is what
        // makes rule 12.6's byte-identity structural rather than maintained.
        assert!(port.record("{\"transcript\":\"exchange\"}").is_ok());
        assert!(port.failure.is_none());
        assert_eq!(port.served, recording.port.exchanges().len() as u64);
    }

    /// Rule 11.7's size, measured rather than restated from the estimate.
    ///
    /// Rule 11.7 **carried** an estimated 4.7 MB for a 1,000-tick run until 2026-08-24, which over
    /// rule 19.5's estimated 10,954 exchanges implied about 429 bytes per record, while rule 11.3
    /// asks for "the request as sent, in full" — and block A alone is 5,385 bytes. The two could not
    /// both hold, and this test is where that was found. The split closes most of the gap and does
    /// not close all of it: the figures printed below are above the withdrawn band, and rule 11.7.1
    /// records that band rather than deleting it. The rule's own stated figures are now the committed
    /// transcript's, at seed 0; these are this run's, at seed 42. **This is retained evidence**,
    /// which is why it prints rather than only asserts.
    ///
    /// The assertion is the property, not the figure: an exchange record is a fraction of the prefix
    /// it was sent against, so a transcript grows with a run's exchanges and not with block A.
    #[test]
    fn a_record_carries_the_variable_part_and_the_head_carries_the_rest() {
        let recording = record_a_run(42, 20, varied_script(400));
        let transcript = recording.port.transcript();
        let head: usize = recording
            .port
            .prefixes()
            .iter()
            .map(|prefix| prefix.len() + 1)
            .sum();
        let exchanges = recording.port.exchanges();
        let mean = (transcript.len() - head) / exchanges.len();

        assert_eq!(SHARED_RULES.len(), 5_385);
        assert!(
            mean < SHARED_RULES.len() / 4,
            "a record is {mean} bytes against block A's {}",
            SHARED_RULES.len()
        );
        println!(
            "shared_rules_bytes={} head_bytes={head} exchanges={} transcript_bytes={} \
             mean_record_bytes={mean}",
            SHARED_RULES.len(),
            exchanges.len(),
            transcript.len()
        );
    }

    // -----------------------------------------------------------------------------------
    // `SPEC-MOK-007` rules 9.5, 9.6, 14 and 15: the fallback's accounting, the cost arithmetic
    // and the run record.
    //
    // Every price and every usage figure below is **declared by the test**. Nothing here reaches a
    // provider, and no figure is a measurement of one: rule 14.3 makes prices inputs of a run, and
    // `WO-MOK-025` item 7 says in so many words that no real usage exists yet. What is being checked
    // is the arithmetic and the record, against figures whose expected results can be computed
    // longhand in the assertion.
    // -----------------------------------------------------------------------------------

    /// Prices in the shape a provider publishes them, converted once: `$1.25`, `$0.125` and `$10.00`
    /// per million tokens are `125`, `13` and `1_000` **cents per million tokens**, which rule 14.3a
    /// declares and which are the same integers as microcents per token. The reasoning price matches
    /// the output price, which is how a provider that bills reasoning tokens bills them.
    ///
    /// The cached figure is `13` and not `12.5`, because rule 14.3a's four values are integers: the
    /// operator declares a whole number of cents per million tokens and there is no finer thing to
    /// declare. `WO-MOK-026` moved these figures from nanodollars per token, where the same three
    /// prices were `1_250`, `125` and `10_000`, when rule 14.2 as amended 2026-08-29 stated the minor
    /// unit as the US cent.
    ///
    /// Round numbers, and deliberately: a case about integer arithmetic should fail on the
    /// arithmetic rather than on a reader's inability to check the expected figure.
    fn declared_prices() -> PerTokenPrices {
        PerTokenPrices {
            uncached_prompt: 125,
            cached_prompt: 13,
            output: 1_000,
            reasoning: 1_000,
        }
    }

    /// One exchange's synthetic usage: a 6,000-token prompt of which 5,400 were served from the
    /// cache, a 40-token response and no reasoning tokens, which is rule 8.5's level stated as a
    /// reported zero rather than as an absence.
    ///
    /// The cache share is nine tenths, so [`RunAccount::cache_ratio_basis_points`] has a figure
    /// above rule 14.5's floor to report and the assertion is a comparison rather than a tautology.
    fn synthetic_usage() -> ExchangeUsage {
        ExchangeUsage::reported(6_000, 5_400, 40, 0)
    }

    /// [`synthetic_usage`] at [`declared_prices`], computed longhand: 600 uncached prompt tokens,
    /// 5,400 cached ones and 40 output tokens, in **microcents**.
    ///
    /// 185,200 microcents is 0.1852 of a cent, which is the figure that decides the unit: one
    /// exchange of this size costs a fifth of a cent, so an accumulation in whole cents would add
    /// zero here and rule 14.6's ceiling would never move. See [`PerTokenPrices`].
    const SYNTHETIC_COST: u64 = 600 * 125 + 5_400 * 13 + 40 * 1_000;

    /// One exchange's usage costing exactly one whole US cent at [`declared_prices`]: a
    /// 1,000-token prompt served entirely from the provider's cache at 13 microcents each, and 987
    /// output tokens at 1,000 each — 13,000 plus 987,000 microcents.
    ///
    /// **It exists because a ceiling cannot be finer than a cent and an exchange can.** Rule 14.2's
    /// minor unit is the US cent, so rule 14.6's ceiling is declared in whole cents, while one
    /// exchange of [`synthetic_usage`] costs 0.1852 of one. Cases **L19** and **L30** both state a
    /// ceiling as a multiple of an exchange's cost, and that claim cannot be written down at all
    /// unless an exchange costs a whole number of cents. This is the usage those cases spend, so
    /// that their figures are the figures the cases name rather than figures chosen to divide.
    fn whole_cent_usage() -> ExchangeUsage {
        ExchangeUsage::reported(1_000, 1_000, 987, 0)
    }

    /// [`whole_cent_usage`] at [`declared_prices`], in microcents. One whole cent, by construction,
    /// and written as the product and sum rather than as `1_000_000` so that a change to either
    /// input fails here rather than silently making the name a lie.
    const WHOLE_CENT: u64 = 1_000 * 13 + 987 * 1_000;

    /// Rules 9.5 and 9.8: the fallback is `wait`, it is counted, the run continues, and the record
    /// marks it.
    ///
    /// The twin run is the point. A port that never answers and a port that answers `wait` at every
    /// opportunity produce the *same* run — the same text bytes, and transcripts differing in one
    /// field — so the only thing distinguishing a contaminated run from a clean one is the flag, the
    /// count and the mark. That is rules 9.5, 9.6 and 15.4 as one comparison, and it is why rule
    /// 12.7 replays from the flag instead of from the verb.
    ///
    /// The cost is the other half. An exchange that yielded nothing reported no usage, so it moves
    /// the exchange count and bills nothing, while the twin run bills every exchange: rule 11.5's
    /// absence is not a zero, and rule 14.1 adds what was reported rather than what was expected.
    #[test]
    fn the_fallback_is_wait_counted_and_marks_the_run() {
        let config = llm_config(42, 6, true);
        let mut silent = ScriptedPort::silent().billing(declared_prices(), None, synthetic_usage());
        let mut text = Vec::new();
        let summary = completed(
            Simulation::new(config)
                .unwrap()
                .run_recording(&mut text, None, Some(&mut silent))
                .expect("rule 9.8: a run that fell back is not aborted"),
        );

        // Rule 9.8: the run's ticks are real and it reached its horizon.
        assert_eq!(summary.reason(), TerminationReason::TickLimit);
        assert_eq!(summary.ticks(), 6);

        let exchanges = silent.exchanges().len() as u64;
        assert!(
            exchanges >= 60,
            "{exchanges} exchanges is too few to measure"
        );
        for record in silent.exchanges() {
            assert_eq!(transcript_flag(record, "fallback"), Some(true), "{record}");
            assert!(record.contains("\"verb\":\"wait\""), "{record}");
        }
        assert_eq!(silent.account.fallbacks(), exchanges);
        assert_eq!(silent.account.exchanges(), exchanges);
        // Rule 11.5 and rule 14.1: nothing was reported, so nothing is billed, at prices that would
        // have billed every one of these exchanges had a proposal come back.
        assert_eq!(silent.account.cost(), 0);
        assert_eq!(silent.account.cache_ratio_basis_points(), None);

        // Rule 9.5's `wait` is applied and accepted at every opportunity, which is why it is the
        // fallback: no opportunity in this run rejected anything.
        let text = String::from_utf8(text).unwrap();
        let traces = text.matches("event=action_trace").count() as u64;
        assert_eq!(traces, exchanges);
        assert_eq!(text.matches("proposal:wait").count() as u64, exchanges);
        assert!(!text.contains("status:rejected"), "{text}");

        // Rule 15.4: the mark is a property of the record, derived from the count.
        let config = llm_config(42, 6, true);
        let record = RunRecord::new(
            &config,
            "a-model-identifier",
            "none",
            &silent.account,
            summary.ticks(),
            RunEnd::Completed(summary.reason()),
        )
        .render();
        assert!(record.contains("\"unfit_to_publish\":true"), "{record}");
        assert!(
            record.contains(&format!("\"fallbacks\":{exchanges}")),
            "{record}"
        );

        // The twin: every exchange answered, and answered with the same action the fallback takes.
        let mut proposing =
            ScriptedPort::always(Action::Wait).billing(declared_prices(), None, synthetic_usage());
        let mut proposed_text = Vec::new();
        Simulation::new(llm_config(42, 6, true))
            .unwrap()
            .run_recording(&mut proposed_text, None, Some(&mut proposing))
            .expect("a run in which every exchange answered");
        assert_eq!(String::from_utf8(proposed_text).unwrap(), text);

        // Two differences between the transcripts, and exactly two, each one a rule. Rule 12.7's
        // flag; and rule 11.5's usage, because an exchange that yielded nothing reported no counts —
        // so its record states `"usage":null` where the answered twin's states what the port
        // reported. Both are asserted present before being substituted away, so the second
        // difference is checked rather than tolerated, and the substitutions are then the *whole* of
        // it: the blocks, the digests, the prefix digests, the actions and the ticks are identical.
        // That is what leaves the flag, the count and the mark as the only things distinguishing a
        // contaminated run from a clean one.
        const REPORTED: &str = "\"usage\":{\"prompt\":6000,\"cached_prompt\":5400,\"output\":40,\
                                \"reasoning\":0}";
        assert!(proposing.transcript().contains(REPORTED));
        assert!(silent.transcript().contains("\"usage\":null"));
        assert_eq!(
            proposing.transcript(),
            silent
                .transcript()
                .replace("\"fallback\":true", "\"fallback\":false")
                .replace("\"usage\":null", REPORTED)
        );

        // Rule 15.3: a clean run states its zero rather than omitting it, and rule 15.4's mark is
        // absent because the count is.
        assert_eq!(proposing.account.fallbacks(), 0);
        assert_eq!(proposing.account.exchanges(), exchanges);
        assert_eq!(proposing.account.cost(), SYNTHETIC_COST * exchanges);
        let config = llm_config(42, 6, true);
        let record = RunRecord::new(
            &config,
            "a-model-identifier",
            "none",
            &proposing.account,
            summary.ticks(),
            RunEnd::Completed(summary.reason()),
        )
        .render();
        assert!(record.contains("\"fallbacks\":0"), "{record}");
        assert!(record.contains("\"unfit_to_publish\":false"), "{record}");
    }

    /// Rule 9.6 and property **P5**: a rejected proposal is not a fallback, and the count agrees
    /// with the transcript.
    ///
    /// The script reaches rule 7.5's gap — an action block D may enumerate and the engine still
    /// rejects, on a ground the observation did not carry — and the assertion refuses to pass unless
    /// that gap was actually reached, because a run with no rejection in it would satisfy the count
    /// half of this case vacuously.
    ///
    /// The two figures compared for **P5** are derived independently: the flag is written by
    /// `exchange_record` from the proposal's absence, and the count is moved by the port that
    /// obtained nothing. A count incremented beside the flag would make this property unable to
    /// fail.
    #[test]
    fn a_rejected_proposal_is_not_a_fallback_and_the_count_matches_the_transcript() {
        let mut port = ScriptedPort::answering(varied_script(400)).billing(
            declared_prices(),
            None,
            synthetic_usage(),
        );
        let mut text = Vec::new();
        Simulation::new(llm_config(42, 5, true))
            .unwrap()
            .run_recording(&mut text, None, Some(&mut port))
            .expect("a run whose proposals are sometimes refused");
        let text = String::from_utf8(text).unwrap();

        let rejections = text.matches("status:rejected").count();
        assert!(
            rejections > 0,
            "no proposal was rejected, so this case measures nothing"
        );

        let exchanges = port.exchanges().len() as u64;
        let flagged = port
            .exchanges()
            .iter()
            .filter(|record| transcript_flag(record, "fallback") == Some(true))
            .count() as u64;
        assert_eq!(port.account.fallbacks(), flagged);
        assert!(
            flagged > 0,
            "the script yields nothing at two of ten entries"
        );
        assert!(flagged < exchanges, "every exchange cannot be a fallback");
        // The count the script *dictates*: two of its ten entries yield nothing, and the entries are
        // consumed in order. Derived from the script rather than written as a constant, because a run
        // in which a Mokiterion died would consume fewer entries and a constant would then be
        // measuring the population instead of the count. A count that had absorbed the rejections
        // would be far above this figure.
        let dictated = varied_script(400)[..exchanges as usize]
            .iter()
            .filter(|entry| entry.is_none())
            .count() as u64;
        assert_eq!(flagged, dictated);

        // Rule 14.1 from the same run: a rejected proposal was an answered exchange and is billed,
        // and only the exchanges that yielded nothing are not.
        assert_eq!(port.account.exchanges(), exchanges);
        assert_eq!(port.account.cost(), SYNTHETIC_COST * (exchanges - flagged));
    }

    /// Rules 14.1, 14.2 and 14.3: cost is integer arithmetic over the run's declared prices.
    ///
    /// Every expected figure is written out as a product and a sum, so that a change to the
    /// arithmetic fails against a number a reader can verify rather than against a constant this
    /// file also produced.
    #[test]
    fn cost_is_integer_arithmetic_over_the_declared_prices() {
        let account = RunAccount::declared(declared_prices(), None);
        assert_eq!(account.cost_of(&synthetic_usage()), SYNTHETIC_COST);
        assert_eq!(account.cost_of(&synthetic_usage()), 185_200);

        // Rule 11.5: an unreported count contributes nothing, and an exchange that reported nothing
        // at all costs nothing rather than costing a prompt's worth of default.
        assert_eq!(account.cost_of(&ExchangeUsage::unreported()), 0);
        // The cached share is billed once, at the cached price. An implementation that billed the
        // prompt total *and* the cached total would report 828_000 here.
        assert_eq!(
            account.cost_of(&ExchangeUsage::reported(6_000, 6_000, 0, 0)),
            6_000 * 13
        );
        // Rule 10.7: the provider's figures are untrusted. A cached count above the prompt count is
        // nonsense, and the arithmetic neither panics nor underflows into a bill of quintillions.
        assert_eq!(
            account.cost_of(&ExchangeUsage::reported(10, 10_000, 0, 0)),
            10 * 13
        );

        // Rule 14.1 accumulates, and rule 14.2's integer arithmetic accumulates without drift: ten
        // exchanges cost exactly ten times one, which a per-exchange division would not.
        let mut ten = RunAccount::declared(declared_prices(), None);
        for _ in 0..10 {
            ten = ten.after_exchange(&synthetic_usage());
        }
        assert_eq!(ten.cost(), SYNTHETIC_COST * 10);
        assert_eq!(ten.exchanges(), 10);

        // Rule 14.3: the prices are the run's. The same usage under prices doubled costs double,
        // because nothing here is compiled in.
        let doubled = PerTokenPrices {
            uncached_prompt: 250,
            cached_prompt: 26,
            output: 2_000,
            reasoning: 2_000,
        };
        assert_eq!(
            RunAccount::declared(doubled, None).cost_of(&synthetic_usage()),
            SYNTHETIC_COST * 2
        );
    }

    /// Rules 14.4 and 14.5: the ratio is computed from the reported figures, and an unreported
    /// cached figure leaves it uncomputable for the rest of the run.
    #[test]
    fn the_cache_ratio_is_the_reported_figures_or_nothing() {
        let mut account = RunAccount::declared(declared_prices(), None);
        assert_eq!(account.cache_ratio_basis_points(), None, "nothing reported");

        for _ in 0..3 {
            account = account.after_exchange(&synthetic_usage());
        }
        // 16,200 of 18,000 prompt tokens, which is nine tenths and above rule 14.5's 8_500 floor.
        assert_eq!(account.cache_ratio_basis_points(), Some(9_000));

        // Rule 14.5's third sentence: one exchange that reported a prompt count with no cached
        // figure, and the ratio cannot be computed. It stays uncomputable when later exchanges
        // report both, because a ratio over the run cannot be assembled from the exchanges that
        // happened to be complete.
        let incomplete = account.after_exchange(&ExchangeUsage::without_cached_figure(6_000, 40));
        assert_eq!(incomplete.cache_ratio_basis_points(), None);
        assert_eq!(
            incomplete
                .after_exchange(&synthetic_usage())
                .cache_ratio_basis_points(),
            None
        );

        // An exchange that reported neither figure is not rule 14.5's case: an exchange the provider
        // never answered says nothing about the provider's cache. Nor is counting a fallback, which
        // moves one figure and it is not one of these two.
        assert_eq!(
            account
                .after_exchange(&ExchangeUsage::unreported())
                .counting_fallback()
                .cache_ratio_basis_points(),
            Some(9_000)
        );
    }

    /// Rule 14.6 and case **L19**: the ceiling is checked before the exchange, so a ceiling equal to
    /// the cost of two exchanges admits exactly two.
    ///
    /// The loop below is the shape rule 14.6 fixes for a caller — ask the account, then spend — and
    /// it is where a live run's ceiling stop will be made. `WO-MOK-025` is out of scope for the stop
    /// itself: `REQ-MOK-071` binds a live run, the option that declares a ceiling is `WO-MOK-026`'s,
    /// and only the arithmetic is in scope here.
    ///
    /// L19's first clause — "no exchange is issued whose cost would cross the ceiling" — is rule
    /// 14.6's check exactly when the ceiling is a whole multiple of an exchange's cost, which
    /// [`whole_cent_usage`] makes this one. It cannot be more than that in general: the cost of the
    /// next exchange depends on the usage that exchange will report, and a check made before it is
    /// issued has no such figure.
    #[test]
    fn the_ceiling_is_checked_before_the_exchange_and_admits_exactly_two() {
        let usage = whole_cent_usage();
        let mut account = RunAccount::declared(declared_prices(), Some(2));
        let mut issued = 0;
        while !account.ceiling_reached() && issued < 10 {
            account = account.after_exchange(&usage);
            issued += 1;
        }
        assert_eq!(issued, 2);
        assert_eq!(account.cost(), WHOLE_CENT * 2);
        assert_eq!(account.cost_cents(), 2);

        // The check is on the accumulated cost and not on the exchange count: two exchanges under a
        // three-cent ceiling have not reached it and the third is issued.
        let under = RunAccount::declared(declared_prices(), Some(3))
            .after_exchange(&usage)
            .after_exchange(&usage);
        assert!(!under.ceiling_reached());

        // **The comparison is made in the accumulation's unit and not in the ceiling's**, which is
        // the whole reason the accumulation is finer than the ceiling. One cent buys six exchanges of
        // `synthetic_usage`'s size: five cost 926,000 microcents and the sixth crosses. An
        // implementation that compared truncated cents against the declared cents would have admitted
        // the first five at a reported cost of `0` — and, at a ceiling declared in cents against
        // exchanges costing a fifth of one, would have admitted them for ever.
        let mut cheap = RunAccount::declared(declared_prices(), Some(1));
        let mut issued = 0;
        while !cheap.ceiling_reached() && issued < 100 {
            cheap = cheap.after_exchange(&synthetic_usage());
            issued += 1;
        }
        assert_eq!(issued, 6);
        assert_eq!(cheap.cost(), SYNTHETIC_COST * 6);
        assert_eq!(cheap.cost_cents(), 1);

        // Rule 14.8: a run with no declared ceiling never stops here, whatever it has spent.
        let uncapped = RunAccount::declared(declared_prices(), None).after_exchange(&usage);
        assert!(!uncapped.ceiling_reached());
        assert_eq!(uncapped.cost(), WHOLE_CENT);
    }

    /// Case **L30**'s second half: the account belongs to the port the host lends, so a stubbed
    /// run's cost rises across ticks and reaches a ceiling.
    ///
    /// **The figure is not the case's.** L30 names "a ceiling set to the cost of two exchanges", and
    /// a tick here holds twelve opportunities, so a two-exchange ceiling is reached inside the first
    /// tick and a port rebuilt every tick would reach it too — the case's own discriminator ("the
    /// accumulated cost stays at zero, so the ceiling never triggers") would be lost. Eighteen
    /// exchanges is one and a half ticks: reached in the second tick when the port is lent, and
    /// unreachable when it is rebuilt. The case's substance is checked and its illustrative figure
    /// is not.
    ///
    /// **Two ticks and not three, as of rule 14.6's check.** Until the check existed this drove three
    /// ticks and recorded a cost of twelve, twenty-four and thirty-six cents — a run that noticed its
    /// ceiling in the second tick and spent past it into the third. That reading is now impossible and
    /// the case is stronger for it: the cost stops at the ceiling rather than at the tick boundary,
    /// which is the whole of rule 14.6's "before spending" and is asserted below as eighteen
    /// exchanges rather than twenty-four. The refusal from the single-tick door is
    /// `Simulation::advance_tick`'s own, for rule 14.8's reason — a replay has no ceiling and this
    /// entry point is the replay host's, so a port reaching it with a ceiling reached is a port it was
    /// never meant to hold.
    #[test]
    fn a_lent_ports_cost_rises_across_ticks_and_reaches_a_ceiling() {
        let mut port = ScriptedPort::always(Action::Wait).billing(
            declared_prices(),
            Some(18),
            whole_cent_usage(),
        );
        let mut simulation = Simulation::new(llm_config(42, 10, false)).unwrap();

        simulation
            .advance_tick(Some(&mut port))
            .expect("the first tick is under the ceiling");
        // The discriminator: a port rebuilt each tick would report this same figure for every tick
        // and would never reach the ceiling at all.
        assert_eq!(port.account.cost(), WHOLE_CENT * 12, "twelve a tick");
        assert!(!port.account.ceiling_reached(), "not in the first tick");

        let refused = simulation
            .advance_tick(Some(&mut port))
            .expect_err("the second tick meets the ceiling");
        assert!(
            refused.contains("spend ceiling reached at tick 2"),
            "{refused}"
        );
        assert!(port.account.ceiling_reached(), "in a later tick");

        // Rule 14.6 as a count: the ceiling is eighteen cents, an exchange costs one, and eighteen
        // exchanges were issued. Nineteen would be the check made after the spending, and twenty-four
        // would be the check made at the tick boundary.
        assert_eq!(port.account.exchanges(), 18);
        assert_eq!(port.account.cost(), WHOLE_CENT * 18);
        assert_eq!(port.account.cost_cents(), 18);
    }

    /// Case **L18**, and rules 14.6, 14.7 and 15.5: a run that reaches its ceiling mid-run issues no
    /// exchange after it and leaves both streams complete to the tick reached.
    ///
    /// Eighteen cents at one cent an exchange, which [`whole_cent_usage`] makes exact, so the stop
    /// falls in the second tick of a run whose horizon is ten — **mid-run**, and not at a boundary
    /// the run would have stopped at anyway, which is what makes the absences below mean something.
    ///
    /// **What is absent is the case.** `SPEC-MOK-006` rule 8.1 gives a run one run record and rule
    /// 8.9's `reason` domain has no member for a ceiling stop, so a run that stopped short writes
    /// neither that record nor rule 9.3's summary line, rather than writing either of them about a
    /// horizon it did not reach — which is rule 15.5's prohibition read as an instruction. The
    /// `simulation_ended` event is absent for the same reason and for one more: the simulation did not
    /// end, and `is_finished` is asked below and agrees. Rule 15.5's "the record says so" is met by
    /// rule 15's own record, which is a separate output and item 11's; nothing here is a record that
    /// says a false thing.
    ///
    /// Rule 14.7's "complete and readable to the tick reached" is asserted as the shape of what did
    /// survive: a header first, the one completed tick's metrics record, and every line a whole
    /// record rather than a truncation.
    #[test]
    fn a_run_stops_at_its_ceiling_and_leaves_both_streams_complete_to_the_tick_reached() {
        let mut port = ScriptedPort::always(Action::Wait).billing(
            declared_prices(),
            Some(18),
            whole_cent_usage(),
        );
        let mut text = Vec::new();
        let mut records = Vec::new();
        let mut simulation = Simulation::new(llm_config(42, 10, false)).unwrap();

        let outcome = simulation
            .run_recording(&mut text, Some(&mut records), Some(&mut port))
            .expect("a ceiling stop is not an error");

        assert!(
            matches!(outcome, RunOutcome::Ceiling { tick_reached: 2 }),
            "{outcome:?}"
        );
        // The horizon was ten and the run reached two, so the stop is the ceiling's and not the tick
        // limit's, and the engine does not claim otherwise to a host that asks.
        assert!(!simulation.is_finished());
        assert_eq!(simulation.termination_reason(), None);

        // Rule 14.6 as a count, and L18's "issues no exchange after the ceiling": eighteen, out of the
        // hundred and twenty opportunities ten ticks hold. The port's own transcript agrees with the
        // port's own account, which is rule 11.3 and rule 14.1 reading the same exchanges.
        assert_eq!(port.account.exchanges(), 18);
        assert_eq!(port.account.cost_cents(), 18);
        assert_eq!(port.exchanges().len(), 18);

        let text = String::from_utf8(text).unwrap();
        let records = String::from_utf8(records).unwrap();
        assert!(
            !text.contains("summary reason="),
            "a summary of a horizon the run did not reach"
        );
        for stream in [&text, &records] {
            assert!(!stream.contains("simulation_ended"), "{stream}");
        }
        assert!(!records.contains("\"record\":\"run\""), "{records}");

        let lines: Vec<&str> = records.lines().collect();
        assert!(lines[0].contains("\"record\":\"header\""), "{}", lines[0]);
        assert_eq!(
            lines
                .iter()
                .filter(|line| line.contains("\"record\":\"metrics\""))
                .count(),
            1,
            "one metrics record, for the one tick that completed"
        );
        for line in &lines {
            assert!(line.starts_with('{') && line.ends_with('}'), "{line}");
        }
    }

    /// Case **L19** at the run level: a ceiling equal to the cost of two exchanges admits exactly two.
    ///
    /// The arithmetic of that claim is checked in
    /// [`the_ceiling_is_checked_before_the_exchange_and_admits_exactly_two`], where a loop asks and
    /// spends. What this adds is that **the engine's own seam is that loop**: the check is made per
    /// decision opportunity, so a two-exchange ceiling stops the run inside the first tick with ten
    /// opportunities of that tick still unspent. A check at the tick boundary would have issued all
    /// twelve and reported a cost of twelve cents against a ceiling of two.
    #[test]
    fn the_engine_stops_inside_the_first_tick_when_the_ceiling_is_two_exchanges() {
        let mut port = ScriptedPort::always(Action::Wait).billing(
            declared_prices(),
            Some(2),
            whole_cent_usage(),
        );
        let outcome = Simulation::new(llm_config(42, 10, false))
            .unwrap()
            .run_recording(&mut io::sink(), None, Some(&mut port))
            .expect("a ceiling stop is not an error");

        assert!(
            matches!(outcome, RunOutcome::Ceiling { tick_reached: 1 }),
            "{outcome:?}"
        );
        assert_eq!(port.account.exchanges(), 2);
        assert_eq!(port.account.cost(), WHOLE_CENT * 2);
        assert_eq!(port.exchanges().len(), 2);
    }

    /// Rule 14.8: **a replay has no ceiling.** [`ReplayPort`] takes [`Proposer::halted`]'s default
    /// and answers `false` for ever, whatever the run it is replaying spent.
    ///
    /// Stated as a pair of runs, because the interesting failure is a replay port that answered from
    /// a figure it read out of the transcript: the recording below declares prices and a ceiling and
    /// costs twelve cents, and the replay of the transcript it produced spends nothing, computes no
    /// ratio, meets no ceiling and completes. The port is asked before and after, so a `halted` that
    /// became true partway through the replay fails here as well.
    #[test]
    fn a_replay_never_halts_whatever_the_recorded_run_spent() {
        let recording = record_a_run_with(
            42,
            1,
            ScriptedPort::always(Action::Wait).billing(
                declared_prices(),
                Some(100),
                whole_cent_usage(),
            ),
        );
        assert_eq!(recording.port.account.cost_cents(), 12, "a tick of twelve");
        assert!(!recording.port.account.ceiling_reached());

        let transcript = recording.port.transcript();
        let mut port = ReplayPort::new(transcript.as_bytes());
        assert!(!port.halted(), "before the first exchange");

        let outcome = Simulation::new(llm_config(42, 1, true))
            .unwrap()
            .run_recording(&mut io::sink(), None, Some(&mut port))
            .expect("the replay completes");
        assert!(matches!(outcome, RunOutcome::Completed(_)), "{outcome:?}");
        assert!(!port.halted(), "after the last one");
    }

    /// Rules 15.2 and 15.3: the record carries every figure the rule names, and states its zeros.
    ///
    /// Field by field rather than as one expected line, so that a failure names the figure that
    /// moved. The line is asserted whole in the case below it, where the subject is the bytes.
    #[test]
    fn the_run_record_carries_every_figure_rule_15_names() {
        let config = llm_config(7, 500, true);
        let account = RunAccount::declared(declared_prices(), Some(104))
            .after_exchange(&synthetic_usage())
            .after_exchange(&ExchangeUsage::unreported())
            .counting_fallback();
        let record = RunRecord::new(
            &config,
            "a-model-identifier",
            "none",
            &account,
            250,
            RunEnd::Completed(TerminationReason::TickLimit),
        )
        .render();

        for figure in [
            // The four token totals, rule 14.1's, with the reasoning total's reported zero stated.
            "\"tokens\":{\"prompt\":6000,\"cached_prompt\":5400,\"output\":40,\"reasoning\":0}",
            // The ratio, unaffected by the fallback that reported nothing.
            "\"cache_ratio_basis_points\":9000",
            // The declared ceiling. `$1.04` is rule 9.8's own estimate of what a full run costs,
            // and 104 is that figure in the minor unit rule 14.2 states.
            "\"ceiling_cents\":104",
            // The fallback count and rule 15.4's mark.
            "\"fallbacks\":1",
            "\"unfit_to_publish\":true",
            // The tick reached, and the run's own inputs.
            "\"tick_reached\":250",
            "\"seed\":7",
            "\"ticks\":500",
            "\"density\":\"0.75\"",
            "\"trace_actions\":true",
            "\"model\":\"a-model-identifier\"",
            "\"reasoning\":\"none\"",
            // Rule 15.1's authority rather than rule 15.2's enumeration: the exchange count is what
            // rule 14.5's threshold of 200 is counted in.
            "\"exchanges\":2",
            "\"ended\":\"tick_limit\"",
        ] {
            assert!(record.contains(figure), "{figure} is missing from {record}");
        }

        // **The reported cost of this two-exchange run is `0` cents, and that is the truncation
        // stated rather than a figure gone missing.** One exchange of `synthetic_usage` costs 185,200
        // microcents — 0.1852 of a cent — and rule 14.2's unit cannot carry it. Both figures are
        // asserted, because a case that checked only the cent would pass against an account that
        // accumulated nothing at all, which is the failure `PerTokenPrices` exists to prevent. The
        // exact figure stays recoverable from the record: rule 14.1's four token totals are in it and
        // rule 14.3a's prices are inputs of the run.
        assert!(record.contains("\"cost_cents\":0"), "{record}");
        assert_eq!(account.cost(), SYNTHETIC_COST);

        // Rule 15.3 against rule 11.5: a run that spent nothing states four zeros and a zero cost,
        // while the two figures that can be *absent* are `null` rather than `0`. A record that wrote
        // `0` for an uncomputable ratio would report a run that cached nothing.
        let clean = RunAccount::declared(declared_prices(), None);
        let record = RunRecord::new(
            &config,
            "a-model-identifier",
            "none",
            &clean,
            0,
            RunEnd::Completed(TerminationReason::Extinction),
        )
        .render();
        assert!(
            record.contains(
                "\"tokens\":{\"prompt\":0,\"cached_prompt\":0,\"output\":0,\"reasoning\":0}"
            ),
            "{record}"
        );
        assert!(record.contains("\"cost_cents\":0"), "{record}");
        assert_eq!(clean.cost(), 0, "and zero in the finer unit too");
        assert!(record.contains("\"fallbacks\":0"), "{record}");
        assert!(record.contains("\"unfit_to_publish\":false"), "{record}");
        assert!(
            record.contains("\"cache_ratio_basis_points\":null"),
            "{record}"
        );
        assert!(record.contains("\"ceiling_cents\":null"), "{record}");
        assert!(record.contains("\"ended\":\"extinction\""), "{record}");
    }

    /// Rule 15.5, case **L18**'s reporting half and scenario **A4**: a ceiling stop says so and
    /// states the tick reached.
    ///
    /// The run that stops is `REQ-MOK-071`'s and lands with live mode; what is built here is the
    /// record such a run reports, and property **P6** over its bytes: no figure carries a decimal
    /// separator. The density does, and it is an echoed input rather than an accounting figure —
    /// `SPEC-MOK-006` rule 4.1 prohibits a floating-point *value*, and this is the same quoted
    /// two-decimal string the header record carries.
    #[test]
    fn a_ceiling_stop_says_so_and_carries_no_decimal_separator() {
        let config = llm_config(7, 500, false);
        let mut account = RunAccount::declared(declared_prices(), Some(18));
        while !account.ceiling_reached() {
            account = account.after_exchange(&whole_cent_usage());
        }
        let record = RunRecord::new(
            &config,
            "a-model-identifier",
            "none",
            &account,
            250,
            RunEnd::Ceiling,
        )
        .render();

        assert!(record.contains("\"ended\":\"ceiling\""), "{record}");
        assert!(record.contains("\"tick_reached\":250"), "{record}");
        // The reported cost is at or above the declared ceiling, which is the invariant
        // [`RunAccount::cost_cents`]'s truncation exists to keep: a run that stopped here can be told
        // from one that did not by its own record.
        assert!(record.contains("\"cost_cents\":18"), "{record}");
        assert!(record.contains("\"ceiling_cents\":18"), "{record}");

        // One line, unframed, on `exchange_record`'s precedent: where it goes is the host's.
        assert!(!record.contains('\n'), "{record}");

        // **P6**: the density is the only decimal separator in the line.
        let figures = record.replace("\"density\":\"0.75\"", "\"density\":\"\"");
        assert!(!figures.contains('.'), "{figures}");

        // The other shape a floating-point figure prints in. Scanned as digit-adjacency rather
        // than as a search for `e`, because half this line's field names contain one.
        let characters: Vec<char> = figures.chars().collect();
        assert!(
            !characters
                .windows(2)
                .any(|pair| pair[0].is_ascii_digit() && matches!(pair[1], 'e' | 'E')),
            "{figures}"
        );
    }

    // -----------------------------------------------------------------------------------
    // `SPEC-MOK-007` rule 10: the connector port, over streams assembled here.
    //
    // **No process is started anywhere in this section.** The port holds two streams and a sink,
    // so an exchange is a slice of bytes going in and a `Vec<u8>` coming out, and every case rule
    // 10.4 admits or refuses can be written down as a line of text. The host's half — the spawn,
    // the inheritance, the reaping — is `tests/connector.rs`, which needs a real child; this tier
    // is the reading, which does not.
    //
    // Rule 10.7 declares the connector's output untrusted in whole, and that is what most of the
    // cases below are: lines no honest connector would send, each of which must become rule 9.5's
    // counted fallback rather than a panic, a coerced figure or a plausible wrong action.
    // -----------------------------------------------------------------------------------

    /// A request composed from a real observation.
    ///
    /// Composed and not hand-built, because the request's own bytes are part of what is asserted:
    /// rule 3.6 is a claim about the order and the joining of the four blocks, and a request whose
    /// blocks this test wrote could not check it.
    fn connector_request() -> DecisionRequest {
        let simulation = Simulation::new(llm_config(42, 10, false)).unwrap();
        DecisionRequest::compose(&simulation.observation(0))
    }

    /// One exchange over in-memory streams: what was sent, what came back, and the transcript.
    ///
    /// The record is written here rather than inside the port, because [`Proposer`] separates the
    /// two — `propose` answers and `record` keeps — and [`PortDecisionSource::decide`] is what
    /// pairs them in a run. Doing the same pairing here is what makes the transcript below the
    /// transcript a run would have written.
    ///
    /// **Including rule 11.2's record for every abandoned attempt**, which is why the loop is here
    /// and not only in `decide`: for the single-attempt responses most cases below supply it runs
    /// zero times, and for a retried one a fixture that wrote the outcome alone would report a
    /// transcript no run produces.
    fn exchange_over(responses: &str) -> (String, Proposal, String) {
        let request = connector_request();
        let mut requests = Vec::new();
        let mut transcript = Vec::new();
        let proposal = {
            let mut port = ConnectorPort::new(
                responses.as_bytes(),
                &mut requests,
                &mut transcript,
                connector_prices(),
                None,
            );
            let proposal = port.propose(request.clone());
            for attempt in &proposal.earlier_attempts {
                port.record(&attempt_record(&request, attempt)).unwrap();
            }
            port.record(&exchange_record(&request, &proposal)).unwrap();
            proposal
        };
        (
            String::from_utf8(requests).unwrap(),
            proposal,
            String::from_utf8(transcript).unwrap(),
        )
    }

    /// [`declared_prices`]'s four figures in the shape the *operator* declares them, rule 14.3a's
    /// [`UnitPrices`], whose first field is the price of a prompt token the cache did not serve.
    ///
    /// The two are held equal by a case of their own, so the accounting figures asserted through this
    /// port and those asserted through [`ScriptedPort`] are the same prices.
    fn connector_prices() -> UnitPrices {
        UnitPrices {
            prompt: 125,
            cached: 13,
            output: 1_000,
            reasoning: 1_000,
        }
    }

    /// One exchange over in-memory streams, returning the proposal and the port's account.
    ///
    /// A sibling of [`exchange_over`] rather than a widening of it. Rule 14's accounting is what a
    /// few cases below are about and what none of the grammar cases are, and a fixture returning a
    /// figure every caller ignored would be a figure nothing holds.
    fn billed_exchange_over(responses: &str) -> (Proposal, accounting::RunAccount) {
        billed_exchange_under(responses, None)
    }

    /// [`billed_exchange_over`] with a ceiling in rule 14.2's cents.
    ///
    /// The ceiling is a parameter of the port's construction, rule 14.3, so a case about rule 14.6
    /// cannot reach it by any other route than building a second port.
    fn billed_exchange_under(
        responses: &str,
        ceiling: Option<u64>,
    ) -> (Proposal, accounting::RunAccount) {
        let mut requests = Vec::new();
        let mut transcript = Vec::new();
        let mut port = ConnectorPort::new(
            responses.as_bytes(),
            &mut requests,
            &mut transcript,
            connector_prices(),
            ceiling,
        );
        let proposal = port.propose(connector_request());
        (proposal, port.account)
    }

    /// A response that every grammar case below starts from and spoils in one way.
    ///
    /// Well formed in full: the protocol, an action with a legal verb, what answered, at what
    /// level, and the four counts. A case that spoils one field therefore differs from an admitted
    /// response in exactly that field.
    const WELL_FORMED: &str = "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"},\
                               \"model\":\"a-model-identifier\",\"reasoning\":\"none\",\
                               \"usage\":{\"prompt\":1000,\"cached_prompt\":900,\"output\":8,\
                               \"reasoning\":0}}";

    /// Rules 10.2, 10.3, 10.3a and 3.6: the request is one line of five fields, and the prompt is
    /// the four blocks in order.
    ///
    /// The field order is asserted by position rather than by membership, because rule 10.4b makes
    /// the names `docs/CONNECTOR_PROTOCOL.md` documents normative and a connector author reading
    /// that document writes a reader against this shape.
    ///
    /// **The absence of a sixth field is the security-relevant half.** Rule 10.3a keeps the model,
    /// the reasoning level and the credential off the request: the first two are the connector's
    /// choice to make and report back, and the third is a value this process never holds.
    #[test]
    fn the_request_is_one_line_of_the_five_documented_fields() {
        let request = connector_request();
        let (sent, _, _) = exchange_over(WELL_FORMED);

        // Rule 10.2's framing: one object, one terminator, and no raw newline inside — which is
        // the whole reason the four blocks are escaped rather than sent as they are worded.
        assert!(sent.ends_with('\n'), "{sent}");
        assert_eq!(sent.matches('\n').count(), 1, "{sent}");

        let line = sent.trim_end_matches('\n');
        assert!(line.starts_with("{\"protocol\":1,\"tick\":"), "{line}");
        let expected_head = format!(
            "{{\"protocol\":1,\"tick\":{},\"actor\":\"{}\",\"prompt\":\"",
            request.tick(),
            request.actor_id()
        );
        assert!(line.starts_with(&expected_head), "{line}");
        assert!(
            line.ends_with(&format!(",\"schema\":{}}}", response_schema())),
            "{line}"
        );

        // Rule 3.6: the prompt is `blocks()` concatenated, with no separator this module invented.
        // Asserted through the round trip, so it is the *bytes* that are checked rather than a
        // second composition agreeing with the first.
        let prompt = line
            .split_once("\"prompt\":\"")
            .and_then(|(_, rest)| rest.split_once("\",\"schema\":"))
            .map(|(prompt, _)| prompt)
            .unwrap();
        assert_eq!(
            unescape_transcript_text(prompt).as_deref(),
            Some(request.blocks().concat().as_str())
        );

        // Rule 10.3a: no sixth field. Checked as the field forms, because the prompt is prose and
        // may legitimately contain any word.
        let fields = format!("{}\"schema\"", line.split_once("\"prompt\"").unwrap().0);
        for absent in ["\"model\"", "\"reasoning\"", "\"key\"", "\"credential\""] {
            assert!(!fields.contains(absent), "{fields} names {absent}");
        }
    }

    /// Rule 10.4: an admitted response yields the action, the counts and the line.
    ///
    /// All three, because rule 11.3 and rule 14 need different halves of the same exchange and the
    /// port is the only place both are visible: the action reaches the run, the counts reach the
    /// accounting, and the line reaches the transcript.
    #[test]
    fn an_admitted_response_yields_the_action_the_counts_and_the_line() {
        let (_, proposal, transcript) = exchange_over(WELL_FORMED);

        assert_eq!(proposal.action, Some(Action::Sleep));
        assert_eq!(proposal.usage.prompt, Some(1000));
        assert_eq!(proposal.usage.cached_prompt, Some(900));
        assert_eq!(proposal.usage.output, Some(8));
        assert_eq!(proposal.usage.reasoning, Some(0));
        assert_eq!(proposal.response.as_deref(), Some(WELL_FORMED));

        // Rule 11.2's one line per exchange, terminated with one byte on every platform.
        assert_eq!(transcript.matches('\n').count(), 1, "{transcript}");
        assert!(!transcript.contains('\r'), "{transcript}");
        assert!(transcript.contains("\"fallback\":false"), "{transcript}");
    }

    /// Rule 10.2's one-for-one ordering, and the trailing carriage return a connector on Windows
    /// may write.
    ///
    /// A response line is terminated by the connector and this engine did not choose how. A reader
    /// that kept the `\r` would carry it into every parsed string and would fail the grammar check
    /// on a well-formed response, for a reason no operator could see.
    #[test]
    fn a_response_terminated_with_a_carriage_return_is_read() {
        let (_, proposal, _) = exchange_over(&format!("{WELL_FORMED}\r\n"));
        assert_eq!(proposal.action, Some(Action::Sleep));
    }

    /// Rules 10.4, 10.4c and 8.2: every case the grammar refuses, and each is a counted fallback
    /// carrying its own line.
    ///
    /// One test for all of them, because the assertion is identical and the cases are a list: no
    /// action, the line recorded as received, and the record marked. What differs between them is
    /// only which field was spoiled, and naming that in the loop is what a failure needs to say.
    ///
    /// The `usage` case is a reading of rule 10.4a rather than of 10.4c, and it is disclosed as
    /// one: 10.4a puts a `usage` on the response that carries an action, so an action arriving
    /// with no `usage` at all is not the documented response. Its *contents* are not required —
    /// rule 11.5 makes every individual count absent-able — which the case below the list checks.
    #[test]
    fn every_response_the_grammar_refuses_is_a_counted_fallback() {
        for (why, response) in [
            (
                "a protocol version this engine does not read",
                "{\"protocol\":2,\"action\":{\"verb\":\"sleep\"},\"model\":\"m\",\
                 \"reasoning\":\"none\",\"usage\":{}}",
            ),
            (
                "no protocol version at all",
                "{\"action\":{\"verb\":\"sleep\"},\"model\":\"m\",\"reasoning\":\"none\",\
                 \"usage\":{}}",
            ),
            (
                "both an action and an error, which rule 10.4 admits exactly one of",
                "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"},\"error\":{\"kind\":\"provider\",\
                 \"message\":\"m\"},\"model\":\"m\",\"reasoning\":\"none\",\"usage\":{}}",
            ),
            (
                "an error, which is rule 19.5a's immediate fallback and carries no action",
                "{\"protocol\":1,\"error\":{\"kind\":\"refused\",\"message\":\"no credential\"}}",
            ),
            (
                "no model, which rule 10.4c names",
                "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"},\"reasoning\":\"none\",\
                 \"usage\":{}}",
            ),
            (
                "an empty model, which is not a name",
                "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"},\"model\":\"\",\
                 \"reasoning\":\"none\",\"usage\":{}}",
            ),
            (
                "a model that is not a string",
                "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"},\"model\":7,\
                 \"reasoning\":\"none\",\"usage\":{}}",
            ),
            (
                "no reasoning level, which rule 10.4c names beside the model",
                "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"},\"model\":\"m\",\"usage\":{}}",
            ),
            (
                "a reasoning level that is the count and not the level",
                "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"},\"model\":\"m\",\"reasoning\":0,\
                 \"usage\":{}}",
            ),
            (
                "no usage, which rule 10.4a puts on every response carrying an action",
                "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"},\"model\":\"m\",\
                 \"reasoning\":\"none\"}",
            ),
            (
                "a verb outside rule 8.2's eleven",
                "{\"protocol\":1,\"action\":{\"verb\":\"negotiate\",\"parameter\":\"M02\"},\
                 \"model\":\"m\",\"reasoning\":\"none\",\"usage\":{}}",
            ),
            (
                "a verb given a parameter it does not take",
                "{\"protocol\":1,\"action\":{\"verb\":\"wait\",\"parameter\":\"M02\"},\
                 \"model\":\"m\",\"reasoning\":\"none\",\"usage\":{}}",
            ),
            (
                "a verb missing the parameter it does take",
                "{\"protocol\":1,\"action\":{\"verb\":\"eat\"},\"model\":\"m\",\
                 \"reasoning\":\"none\",\"usage\":{}}",
            ),
            (
                "a parameter that is present and is not a string",
                "{\"protocol\":1,\"action\":{\"verb\":\"avoid\",\"parameter\":2},\"model\":\"m\",\
                 \"reasoning\":\"none\",\"usage\":{}}",
            ),
            (
                "an action that is not an object",
                "{\"protocol\":1,\"action\":\"sleep\",\"model\":\"m\",\"reasoning\":\"none\",\
                 \"usage\":{}}",
            ),
            (
                "no action and no error, which answers nothing at all",
                "{\"protocol\":1,\"model\":\"m\",\"reasoning\":\"none\",\"usage\":{}}",
            ),
        ] {
            let (_, proposal, transcript) = exchange_over(response);
            assert!(proposal.action.is_none(), "admitted {why}: {response}");
            // Rule 11.3: as received, in full. The refused line is the one a reader needs most.
            assert_eq!(proposal.response.as_deref(), Some(response), "{why}");
            assert!(
                transcript.contains("\"fallback\":true"),
                "{why}: {transcript}"
            );
            // Rule 9.5's fallback is `wait`, and rule 9.7 forbids a substitute from elsewhere: the
            // refused `sleep` must not have become the recorded action.
            assert!(
                transcript.contains("\"action\":{\"verb\":\"wait\"}"),
                "{why}: {transcript}"
            );
        }
    }

    /// The two price shapes are the same four figures, and the conversion between them names the
    /// field rule 14.3a's ordering only implies.
    ///
    /// The one place a transposition could enter the arithmetic, held by a case: `UnitPrices::prompt`
    /// is documented "Uncached prompt tokens" and `PerTokenPrices::uncached_prompt` says so in its
    /// name, and a conversion that crossed the first two fields would bill a cached token at the
    /// uncached price and still pass every ratio assertion in this file.
    #[test]
    fn the_declared_prices_cross_into_the_arithmetics_order_unchanged() {
        assert_eq!(
            PerTokenPrices::declared(&connector_prices()),
            declared_prices()
        );

        // Rule 14.3a's own example, parsed rather than written: the four integers reach the
        // arithmetic as the four integers the operator typed, in that order and in that unit.
        let parsed = UnitPrices::parse("125:13:1000:0").expect("rule 14.3a's example");
        assert_eq!(
            PerTokenPrices::declared(&parsed),
            PerTokenPrices {
                uncached_prompt: 125,
                cached_prompt: 13,
                output: 1_000,
                reasoning: 0,
            }
        );
    }

    /// Rules 14.1 and 10.4c: **every** exchange is billed, and a grammar-refused one is billed *and*
    /// counted.
    ///
    /// This is the case the fallback's accounting was reshaped for. A response that carries usage and
    /// an action naming neither the model nor the reasoning level is refused by rule 10.4c and became
    /// a counted fallback — and it was *paid for*, because the provider answered. The two figures are
    /// asserted against the admitted twin: same response but for the missing `model`, so the cost is
    /// identical and the only difference is the count.
    ///
    /// An earlier shape booked a fallback as an exchange reporting nothing, which would report `0`
    /// here. Rule 10.8 does not cover that — it covers a connector that under-reports its usage, not
    /// a host that discards usage the connector did report.
    #[test]
    fn a_grammar_refused_response_is_billed_as_well_as_counted() {
        const BILLED: u64 = 100 * 125 + 900 * 13 + 8 * 1_000;

        let (admitted, account) = billed_exchange_over(WELL_FORMED);
        assert_eq!(admitted.action, Some(Action::Sleep));
        assert_eq!(account.exchanges(), 1);
        assert_eq!(account.fallbacks(), 0);
        assert_eq!(account.cost(), BILLED);

        // The same response with `model` removed, which rule 10.4c refuses.
        let refused_line = WELL_FORMED.replace("\"model\":\"a-model-identifier\",", "");
        let (refused, refused_account) = billed_exchange_over(&refused_line);
        assert_eq!(refused.action, None, "{refused_line}");
        assert_eq!(refused_account.exchanges(), 1);
        assert_eq!(refused_account.fallbacks(), 1);
        assert_eq!(
            refused_account.cost(),
            BILLED,
            "a refused response's exchange was billed at the same figure as the admitted one"
        );

        // Rule 14.4 is unaffected: the counts reached the totals, so the ratio is the response's own.
        assert_eq!(refused_account.cache_ratio_basis_points(), Some(9_000));
    }

    /// Rule 11.5 and rule 9.4 from the accounting's side: an exchange that reported nothing costs
    /// nothing, and the exchange still happened.
    ///
    /// The counterpart of the case above, and the reason the bill and the count are two statements
    /// rather than one. Rule 10.4's error response and a pipe that failed both carry no usage at all,
    /// and each is a fallback whose cost is zero — so a `0` here is the absence rule 11.5 describes
    /// and not an accounting that lost a figure.
    #[test]
    fn an_exchange_that_reported_nothing_costs_nothing_and_is_still_an_exchange() {
        for response in [
            "{\"protocol\":1,\"error\":{\"kind\":\"refused\",\"message\":\"no credential\"}}",
            "not a response at all",
            "",
        ] {
            let (proposal, account) = billed_exchange_over(response);
            assert_eq!(proposal.action, None, "{response}");
            assert_eq!(account.exchanges(), 1, "{response}");
            assert_eq!(account.fallbacks(), 1, "{response}");
            assert_eq!(account.cost(), 0, "{response}");
            // Rule 14.5: no prompt count was reported, so the denominator is zero and the ratio is
            // uncomputable rather than zero.
            assert_eq!(account.cache_ratio_basis_points(), None, "{response}");
        }
    }

    /// Rule 11.5: an unusable count is **absent**, not zero, and does not cost the exchange its
    /// action.
    ///
    /// The two halves are one test because they are one design decision. Rule 10.8 says the ceiling
    /// protects against an honest connector and not a dishonest one, so a figure this reader cannot
    /// use is not a reason to refuse an action that is otherwise well formed — but it must not be
    /// read as zero either, because zero is a cost and absence is a failure to evaluate, and rule
    /// 14.5 acts on the second.
    #[test]
    fn an_unusable_count_is_absent_rather_than_zero() {
        for figure in ["-1", "1.5", "1e9", "\"1000\"", "null", "true", "[]", "{}"] {
            let response = format!(
                "{{\"protocol\":1,\"action\":{{\"verb\":\"sleep\"}},\"model\":\"m\",\
                 \"reasoning\":\"none\",\"usage\":{{\"prompt\":{figure},\"cached_prompt\":900,\
                 \"output\":8,\"reasoning\":0}}}}"
            );
            let (_, proposal, transcript) = exchange_over(&response);
            assert_eq!(proposal.action, Some(Action::Sleep), "{figure}");
            assert_eq!(proposal.usage.prompt, None, "{figure} became a count");
            assert_eq!(proposal.usage.cached_prompt, Some(900), "{figure}");
            // And the record says absent rather than omitting the member, which rule 11.5's second
            // level of absence is for.
            assert!(transcript.contains("\"prompt\":null"), "{transcript}");
        }
    }

    /// Rule 19.5a's `malformed`, and rule 10.7's totality: a line that is not a response is a
    /// counted fallback and never a panic.
    ///
    /// The list is what a connector goes wrong in: a truncated line, a line that is not JSON at
    /// all, an empty line, a second object after the first, and nesting past this reader's bound.
    /// Every one of them arrives from a child process, so a reader that panicked on any of them
    /// would be a run ended by the connector's choice of bytes.
    #[test]
    fn a_line_that_is_not_a_response_is_a_counted_fallback() {
        let deep = format!("{}1{}", "[".repeat(64), "]".repeat(64));
        for line in [
            "",
            " ",
            "not json",
            "{",
            "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"}",
            "{\"protocol\":1} {\"protocol\":1}",
            "{\"protocol\":1}trailing",
            "{\"protocol\":01}",
            "{\"protocol\":1,\"model\":\"\\ud800\"}",
            "{\"protocol\":1,\"model\":\"a\tb\"}",
            &deep,
        ] {
            let (_, proposal, transcript) = exchange_over(line);
            assert!(proposal.action.is_none(), "admitted {line:?}");
            assert_eq!(proposal.usage, ReportedUsage::default(), "{line:?}");
            assert!(transcript.contains("\"fallback\":true"), "{line:?}");
        }
    }

    /// Rule 10.2's exactly one response per request: a connector that closed its output has
    /// stopped answering, which is an error and not an empty answer.
    ///
    /// Recorded as rule 11.3's "or the error", and a counted fallback rather than an abort under
    /// rule 9.8. Rule 19.7 is kept by what the message can contain: an `io::Error` from a stream
    /// names no credential, and this port resolved no path to name.
    #[test]
    fn a_closed_output_is_an_error_and_not_an_empty_response() {
        let (sent, proposal, transcript) = exchange_over("");

        // The request was still written: the failure is the answer's absence, not a refusal to ask.
        assert!(sent.ends_with('\n'), "{sent}");
        assert!(proposal.action.is_none());
        let recorded = proposal.response.unwrap();
        assert!(recorded.starts_with("connector: "), "{recorded}");
        assert!(transcript.contains("\"fallback\":true"), "{transcript}");
        for forbidden in ["sk-", "api", "key", "authorization"] {
            assert!(!recorded.to_lowercase().contains(forbidden), "{recorded}");
        }
    }

    /// Rule 11.2: one record per line, and the sink is flushed as each is written.
    ///
    /// The flush is asserted as bytes present in the sink *before* the port is dropped, which is
    /// the only way to tell a written record from a buffered one. A live run's records are evidence
    /// somebody paid for, and a buffer holding the last exchanges of a run that then died would
    /// lose exactly the records nobody can regenerate without spending again.
    #[test]
    fn every_record_reaches_the_transcript_before_the_port_is_dropped() {
        let mut requests = Vec::new();
        let mut transcript = Vec::new();
        // No response is needed: this is the recording half of [`Proposer`], and the two halves are
        // separate for exactly this reason — a record is written whether an exchange yielded
        // anything or not.
        let mut port = ConnectorPort::new(
            "".as_bytes(),
            &mut requests,
            &mut transcript,
            connector_prices(),
            None,
        );

        port.record("first").unwrap();
        port.record("second").unwrap();
        drop(port);

        assert_eq!(String::from_utf8(transcript).unwrap(), "first\nsecond\n");
    }

    /// Rule 11.3 and rule 12.6: a response is recorded so that reading the record back yields the
    /// bytes the connector sent.
    ///
    /// The round trip is what rule 11.4.1 asks for and what a replay of a live recording depends
    /// on. A response is the one string on a record that this library did not compose, so it is
    /// the one that can carry a quotation mark, a backslash or a newline of somebody else's
    /// choosing — and a record that escaped it wrongly would replay as a different exchange.
    #[test]
    fn a_response_survives_the_record_it_is_written_into() {
        let awkward = "{\"protocol\":1,\"model\":\"a \\\"quoted\\\" \\\\ name\"}";
        let (_, proposal, transcript) = exchange_over(awkward);

        assert_eq!(proposal.response.as_deref(), Some(awkward));
        assert_eq!(transcript.lines().count(), 1, "{transcript}");
        let recorded = transcript
            .split_once("\"response\":\"")
            .and_then(|(_, rest)| rest.split_once("\",\"usage\":"))
            .map(|(response, _)| response)
            .unwrap();
        assert_eq!(unescape_transcript_text(recorded).as_deref(), Some(awkward));
    }

    /// Rule 8.4 against rule 8.2: the schema offers a provider exactly the verbs this engine
    /// admits, and no others.
    ///
    /// [`RESPONSE_VERBS`] is a second statement of a set [`action_from_parts`] already fixes, and
    /// this is the test that makes the duplication safe rather than merely small. The failure it
    /// prevents costs money: a schema offering a twelfth verb would have a provider answer with it
    /// under a structured-output guarantee, and the answer would be refused by the grammar check —
    /// a paid exchange spent on a counted fallback, every time, for as long as the two disagreed.
    #[test]
    fn the_schema_offers_exactly_the_verbs_the_engine_admits() {
        // Forwards: every verb the schema offers is admitted under *some* parameter. Three forms,
        // because the eleven do not agree on what a parameter is — four take none, six take an
        // identifier and `move` takes a direction — and the claim being checked here is that the
        // verb is admitted at all rather than that any particular pairing is.
        let direction = Direction::ORDERED[0].to_string();
        for verb in RESPONSE_VERBS {
            assert!(
                [None, Some("M02"), Some(direction.as_str())]
                    .into_iter()
                    .any(|parameter| action_from_parts(verb, parameter).is_some()),
                "the schema offers {verb}, which the grammar refuses"
            );
        }

        // Backwards: every action the engine can carry has its verb in the schema. The eleven are
        // written out because the enumeration has no iterator, and the count is asserted so that a
        // twelfth variant fails here rather than being silently unlisted.
        let every_action = [
            Action::Wait,
            Action::Sleep,
            Action::Eat {
                food_id: "F01".to_string(),
            },
            Action::Move {
                direction: Direction::ORDERED[0],
            },
            Action::Attack {
                target: "M02".to_string(),
            },
            Action::Threaten {
                target: "M02".to_string(),
            },
            Action::Fight {
                target: "M02".to_string(),
            },
            Action::Retreat {
                target: "M02".to_string(),
            },
            Action::Surrender {
                target: "M02".to_string(),
            },
            Action::Approach {
                target: "M02".to_string(),
            },
            Action::Avoid {
                target: "M02".to_string(),
            },
        ];
        let verbs: std::collections::BTreeSet<&str> = every_action
            .iter()
            .map(|action| action_parts(action).0)
            .collect();
        assert_eq!(verbs.len(), every_action.len(), "two actions share a verb");
        assert_eq!(
            verbs,
            RESPONSE_VERBS
                .into_iter()
                .collect::<std::collections::BTreeSet<&str>>()
        );

        // And the schema states them as its enumeration, in the documented order.
        let schema = response_schema();
        assert!(
            schema.contains(&format!(
                "\"enum\":[{}]",
                RESPONSE_VERBS
                    .iter()
                    .map(|verb| format!("\"{verb}\""))
                    .collect::<Vec<_>>()
                    .join(",")
            )),
            "{schema}"
        );
        // `parameter` is offered and is not required, because four of the eleven take none.
        assert!(schema.contains("\"required\":[\"verb\"]"), "{schema}");
        assert!(
            schema.contains("\"parameter\":{\"type\":\"string\"}"),
            "{schema}"
        );
    }

    // -----------------------------------------------------------------------------------
    // `SPEC-MOK-007` rule 19.5: the bounded retry, over the same assembled streams.
    //
    // The bound, the vocabulary of rule 19.5a and rule 14.6's check before each attempt are all
    // decisions taken inside `ConnectorPort::propose`, so they are measured here, where a response
    // is a line of text and a retry is the next line. `VER-MOK-018` cases **R1** and **R2** are the
    // same properties through a real child process, in `tests/connector.rs`.
    //
    // No case below asserts a duration, which is case **R5**: rule 11.4 admits no timestamp in a
    // transcript, so an elapsed figure is not something this repository retains and not something a
    // case may pass on.
    // -----------------------------------------------------------------------------------

    /// Rule 19.5a's transient kind, the one every case below retries.
    const TRANSPORT_FAILURE: &str =
        "{\"protocol\":1,\"error\":{\"kind\":\"transport\",\"message\":\"the socket closed\"}}";

    /// A provider failure that reports what it cost, which is the case rule 14.6's check before each
    /// attempt exists for.
    ///
    /// Rule 10.4a puts a `usage` on a response carrying an action, and this one carries an error
    /// instead — but [`ConnectorPort::interpret`] reads the two halves independently, and a provider
    /// that answered and failed has billed for it. Its counts are [`whole_cent_usage`]'s, so each
    /// attempt costs exactly one cent and a ceiling can be declared in attempts.
    const BILLED_PROVIDER_FAILURE: &str = "{\"protocol\":1,\"error\":{\"kind\":\"provider\",\"message\":\"rate limited\"},\
         \"usage\":{\"prompt\":1000,\"cached_prompt\":1000,\"output\":987,\"reasoning\":0}}";

    /// A response repeated as its own lines, which is what a connector failing the same way looks
    /// like from this side of the pipe.
    fn repeated(response: &str, times: usize) -> String {
        vec![response; times].join("\n")
    }

    /// Rule 19.5 and case **R2**: a transient failure is attempted four times and no more, every
    /// attempt is its own record, and the opportunity counts **one** fallback.
    ///
    /// The count is the load-bearing assertion and case **P5** is why. Rule 15.4's figure is
    /// reconciled against the records marked as fallbacks, so a port that counted per attempt would
    /// report four against one marked record and the property would fail — which is the failure this
    /// case exists to make impossible. The bill goes the other way: rule 11.2 calls each attempt its
    /// own billed exchange, so the exchange count is four.
    #[test]
    fn a_transient_failure_is_attempted_four_times_and_counts_one_fallback() {
        let (_, proposal, transcript) = exchange_over(&repeated(TRANSPORT_FAILURE, 6));

        // Rule 19.5's bound is three retries, so three attempts were abandoned and the fourth is
        // the one the opportunity ended on. The two lines the fixture did not need were not read:
        // the loop stops at the bound and not at the end of the stream.
        assert_eq!(proposal.earlier_attempts.len(), RETRY_BOUND);
        assert!(proposal.action.is_none());
        assert_eq!(proposal.response.as_deref(), Some(TRANSPORT_FAILURE));

        // Rule 11.2: one record per attempt, in the order they were made, and the last is the
        // opportunity's own.
        let records: Vec<&str> = transcript.lines().collect();
        assert_eq!(records.len(), RETRY_BOUND + 1, "{transcript}");
        for record in &records[..RETRY_BOUND] {
            assert_eq!(transcript_flag(record, "fallback"), Some(false), "{record}");
        }
        assert_eq!(
            transcript_flag(records[RETRY_BOUND], "fallback"),
            Some(true),
            "{transcript}"
        );

        // Every record is the same tick and the same actor, because they are one opportunity. This
        // is also the shape no replay can read: see [`Proposal::earlier_attempts`].
        let request = connector_request();
        for record in &records {
            assert!(
                record.contains(&format!("\"tick\":{}", request.tick())),
                "{record}"
            );
            assert!(record.contains(request.actor_id()), "{record}");
        }

        // Rule 14.1 against rule 9.5: four billed exchanges, one counted decision. An error
        // response reports no usage, so the bill is zero and the *count* of exchanges is what moved.
        let (_, account) = billed_exchange_over(&repeated(TRANSPORT_FAILURE, 6));
        assert_eq!(account.exchanges(), (RETRY_BOUND + 1) as u64);
        assert_eq!(account.fallbacks(), 1);
        assert_eq!(account.cost(), 0);
    }

    /// Rule 19.5 and case **R1**: a transient failure that succeeds on a later attempt is not a
    /// fallback at all, and the attempts before it are still recorded.
    ///
    /// The pair with the case above, and the reason the count cannot be taken per exchange: this run
    /// spent three exchanges and reached a decision, so rule 15.4's mark must stay off it. A port
    /// that counted a fallback when an attempt failed would report two here and mark a clean run as
    /// unfit to publish.
    #[test]
    fn a_transient_failure_that_then_succeeds_counts_no_fallback() {
        let script = format!(
            "{}\n{}\n{WELL_FORMED}",
            TRANSPORT_FAILURE, TRANSPORT_FAILURE
        );
        let (sent, proposal, transcript) = exchange_over(&script);

        assert_eq!(proposal.action, Some(Action::Sleep));
        assert_eq!(proposal.earlier_attempts.len(), 2);
        assert_eq!(proposal.response.as_deref(), Some(WELL_FORMED));

        // Rule 10.2's one-for-one ordering holds per *attempt*: three requests were written for one
        // opportunity, which is what makes each attempt an exchange rather than a re-reading.
        assert_eq!(sent.lines().count(), 3, "{sent}");

        // Three records, none of them a fallback: the two abandoned attempts are marked `false`
        // because they were not the decision, and the decision was an action.
        let records: Vec<&str> = transcript.lines().collect();
        assert_eq!(records.len(), 3, "{transcript}");
        for record in &records {
            assert_eq!(transcript_flag(record, "fallback"), Some(false), "{record}");
        }
        // The two abandoned attempts carry rule 11.3's response, which is the failure a reader is
        // looking for, and rule 9.5's `wait` in the action field, because they obtained no action.
        for record in &records[..2] {
            assert!(
                record.contains("\"action\":{\"verb\":\"wait\"}"),
                "{record}"
            );
            assert!(record.contains("the socket closed"), "{record}");
        }
        assert!(
            records[2].contains("\"action\":{\"verb\":\"sleep\"}"),
            "{transcript}"
        );

        let (_, account) = billed_exchange_over(&script);
        assert_eq!(account.exchanges(), 3);
        assert_eq!(account.fallbacks(), 0);
        // Rule 14.1: only the third exchange reported anything, and the two before it contributed
        // nothing rather than being unbilled.
        assert_eq!(account.cost(), 100 * 125 + 900 * 13 + 8 * 1_000);
    }

    /// Rule 19.5a: everything but `transport` and `provider` is attempted **once**.
    ///
    /// The four cases are the rule's own reasoning as a list. `malformed` and `refused` would be
    /// answered identically by the same request, an unrecognised kind is treated as `malformed`
    /// because rule 10.7 distrusts the connector's error vocabulary too, and a response carrying an
    /// action is not an error however it is dressed. Each of them retried would cost three exchanges
    /// per opportunity for nothing, which is why the count of requests is what is asserted.
    #[test]
    fn only_the_two_transient_kinds_are_attempted_more_than_once() {
        for (why, response) in [
            (
                "rule 19.5a's `malformed`, reported as the kind",
                "{\"protocol\":1,\"error\":{\"kind\":\"malformed\",\"message\":\"m\"}}",
            ),
            (
                "rule 19.5a's `refused`",
                "{\"protocol\":1,\"error\":{\"kind\":\"refused\",\"message\":\"no credential\"}}",
            ),
            (
                "a kind outside the four, which rule 19.5a treats as `malformed`",
                "{\"protocol\":1,\"error\":{\"kind\":\"transport-ish\",\"message\":\"m\"}}",
            ),
            (
                "a line this reader cannot parse, which is `malformed` in fact",
                "not a response at all",
            ),
            (
                "a transient kind alongside an action, which rule 10.4 admits one of",
                "{\"protocol\":1,\"action\":{\"verb\":\"sleep\"},\
                 \"error\":{\"kind\":\"transport\",\"message\":\"m\"},\"model\":\"m\",\
                 \"reasoning\":\"none\",\"usage\":{}}",
            ),
        ] {
            let (sent, proposal, transcript) = exchange_over(&repeated(response, 6));
            assert!(proposal.earlier_attempts.is_empty(), "retried {why}");
            assert_eq!(sent.lines().count(), 1, "retried {why}");
            assert_eq!(transcript.lines().count(), 1, "retried {why}");
            assert_eq!(
                transcript_flag(transcript.trim_end(), "fallback"),
                Some(true),
                "{why}: {transcript}"
            );
        }
    }

    /// Rule 19.5's own sentence: "rule 14.6's check runs before each attempt, so the bound cannot
    /// breach a ceiling whatever it is set to".
    ///
    /// The ceiling is declared in attempts, one cent each, and the measurement is the count of
    /// exchanges the port issued. Without the check inside the loop a bound of three would spend up
    /// to three exchanges past a ceiling, which is the reading the rule forecloses — and it would do
    /// it invisibly, because the engine's own check happens once per opportunity and every attempt
    /// after the first is inside a single call to `propose`.
    ///
    /// The unbounded row is the control: the same response with no ceiling reaches the bound, so a
    /// low count in the rows above is the ceiling stopping the retrying rather than the response
    /// failing to be retried at all.
    #[test]
    fn the_ceiling_stops_the_retrying_before_the_bound() {
        for (ceiling, attempts) in [
            (Some(1), 1),
            (Some(2), 2),
            (Some(3), 3),
            (Some(4), RETRY_BOUND + 1),
            // A ceiling the four attempts cannot reach, and none at all: the bound is what stops it.
            (Some(99), RETRY_BOUND + 1),
            (None, RETRY_BOUND + 1),
        ] {
            let (proposal, account) =
                billed_exchange_under(&repeated(BILLED_PROVIDER_FAILURE, 8), ceiling);
            assert_eq!(
                account.exchanges(),
                attempts as u64,
                "under a ceiling of {ceiling:?} cents"
            );
            assert_eq!(proposal.earlier_attempts.len(), attempts - 1);
            // Whatever stopped it, the opportunity counts one fallback and costs a cent an attempt.
            assert_eq!(account.fallbacks(), 1, "under {ceiling:?}");
            assert_eq!(
                account.cost(),
                WHOLE_CENT * attempts as u64,
                "under {ceiling:?}"
            );
        }
    }

    /// A pipe that failed is not retried, and the reason is that there is nothing left to ask.
    ///
    /// Which transport failures are retried is `WO-MOK-026`'s envelope, and this is the line it
    /// draws: rule 19.5a's kinds are kinds a connector *reports*, and a connector whose output has
    /// closed has reported nothing. Three further attempts would write three more identical records
    /// for a process that is gone.
    ///
    /// The second half is the one that shows the retrying is not merely unreachable: the port's
    /// first exchange succeeded, so it was in working order when the pipe went.
    #[test]
    fn a_pipe_that_failed_is_not_retried() {
        // Closed before answering anything.
        let (proposal, account) = billed_exchange_over("");
        assert!(proposal.earlier_attempts.is_empty());
        assert_eq!(account.exchanges(), 1);
        assert!(
            proposal
                .response
                .unwrap_or_default()
                .starts_with("connector: "),
            "rule 11.3's \"or the error\""
        );

        // Closed after one answer, over a port that is reused as a run reuses it.
        let one_answer = format!("{WELL_FORMED}\n");
        let mut requests = Vec::new();
        let mut transcript = Vec::new();
        let mut port = ConnectorPort::new(
            one_answer.as_bytes(),
            &mut requests,
            &mut transcript,
            connector_prices(),
            None,
        );
        let first = port.propose(connector_request());
        assert_eq!(first.action, Some(Action::Sleep));
        assert!(first.earlier_attempts.is_empty());

        let second = port.propose(connector_request());
        assert!(second.action.is_none());
        assert!(
            second.earlier_attempts.is_empty(),
            "the dead pipe was retried"
        );
        assert_eq!(port.account.exchanges(), 2);
    }

    /// The gap this work order **discloses and does not repair**: a transcript holding a retried
    /// exchange cannot be replayed.
    ///
    /// Rule 11.2 gives every attempt its own record and rule 12.3 has a replay read one record per
    /// decision opportunity, checking the tick and the actor. The two do not reconcile. The first
    /// attempt's record matches the first opportunity and is consumed by it — an attempt record is
    /// marked `"fallback":false` under rule 11.3, because the attempt was not the decision, and it
    /// carries rule 9.5's `wait`, so it is indistinguishable from a legitimately recorded `wait` —
    /// and the *second* opportunity then meets a record for the first actor and fails rule 12.3's
    /// check.
    ///
    /// **The failure is loud, which is the whole reason this is a disclosure rather than a stop.**
    /// Rule 12.3 refuses and names the disagreement, so no replay silently invents a run. What it
    /// names is the wrong record, and repairing that means either a field on the exchange record
    /// distinguishing an attempt — which amends rule 11.3 — or rule 12.3 reading a group per
    /// opportunity. Both are changes of substance in an approved artifact, which this work order's
    /// stop-and-escalate condition 6 reserves to the owner.
    ///
    /// This case exists so the behaviour is pinned rather than discovered by a live run: a live
    /// transcript that retried is not a replay input, and case **L30**'s replay identity would have
    /// to be captured from a run that did not retry.
    #[test]
    fn a_transcript_holding_a_retried_exchange_cannot_be_replayed() {
        // One opportunity's worth of records, written the way `PortDecisionSource::decide` writes
        // them: two abandoned attempts and the decision.
        let script = format!(
            "{}\n{}\n{WELL_FORMED}",
            TRANSPORT_FAILURE, TRANSPORT_FAILURE
        );
        let (_, _, transcript) = exchange_over(&script);
        assert_eq!(transcript.lines().count(), 3, "{transcript}");

        // The first record is not distinguishable from a recorded `wait` by any field rule 11.3
        // fixes, which is the half a replay cannot see.
        let first = transcript.lines().next().unwrap();
        assert_eq!(transcript_flag(first, "fallback"), Some(false), "{first}");
        assert!(first.contains("\"action\":{\"verb\":\"wait\"}"), "{first}");
    }
}
