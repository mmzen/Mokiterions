use std::fmt;
use std::io::{self, Write};

const WORLD_SIZE: u8 = 128;
const TERRITORY_HEIGHT: u8 = 64;
const ATTRIBUTE_MAX: u8 = 100;
const FOOD_CAPACITY_PER_TERRITORY: usize = 12;
const REGENERATION_INTERVAL: u64 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub seed: u64,
    pub tick_limit: u64,
    pub trace_actions: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Territory {
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
struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    fn territory(self) -> Territory {
        if self.y < TERRITORY_HEIGHT {
            Territory::A
        } else {
            Territory::B
        }
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
enum Direction {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FoodClass {
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
enum Action {
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
    }
}

trait DecisionSource {
    fn decide(&mut self, observation: &Observation, seeded_choice: usize) -> Action;
}

#[derive(Default)]
struct BaselineDecisionSource;

impl DecisionSource for BaselineDecisionSource {
    fn decide(&mut self, observation: &Observation, seeded_choice: usize) -> Action {
        debug_assert!(observation.is_consistent());
        observation
            .valid_actions
            .get(seeded_choice)
            .cloned()
            .unwrap_or(Action::Wait)
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
enum TerminationReason {
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

pub struct Simulation {
    config: Config,
    tick: u64,
    agents: Vec<Mokiterion>,
    foods: Vec<Food>,
    entropy: SplitMix64,
    next_food_id: u32,
}

impl Simulation {
    pub fn new(config: Config) -> Result<Self, String> {
        if config.tick_limit == 0 {
            return Err("tick limit must be greater than zero".into());
        }

        let mut entropy = SplitMix64::new(config.seed);
        let mut foods: Vec<Food> = Vec::with_capacity(FOOD_CAPACITY_PER_TERRITORY * 2);
        let mut next_food_id = 1;

        for territory in Territory::ALL {
            for class in FoodClass::ALL {
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
        })
    }

    pub fn run<W: Write>(&mut self, output: &mut W) -> io::Result<RunSummary> {
        let mut baseline = BaselineDecisionSource;
        self.run_with_source(output, &mut baseline)
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

        self.emit_initialization(output)?;

        loop {
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
                let summary = self.summary(reason);
                write_event(
                    output,
                    self.tick,
                    "world",
                    "simulation_ended",
                    &format!("reason:{reason}"),
                )?;
                self.emit_summary(output, &summary)?;
                return Ok(summary);
            }
        }
    }

    fn emit_initialization<W: Write>(&self, output: &mut W) -> io::Result<()> {
        write_event(
            output,
            0,
            "world",
            "world_initialized",
            "width:128,height:128,territories:2",
        )?;
        for food in &self.foods {
            write_event(
                output,
                0,
                &food.id,
                "food_initialized",
                &format!(
                    "class:{},position:{},territory:{}",
                    food.class,
                    food.position,
                    food.position.territory()
                ),
            )?;
        }
        for agent in &self.agents {
            write_event(
                output,
                0,
                &agent.id,
                "agent_initialized",
                &format!(
                    "position:{},territory:{},health:{},satiety:{},energy:{}",
                    agent.position,
                    agent.position.territory(),
                    agent.health,
                    agent.satiety,
                    agent.energy
                ),
            )?;
        }
        Ok(())
    }

    fn run_tick<W: Write, D: DecisionSource>(
        &mut self,
        output: &mut W,
        decision_source: &mut D,
    ) -> io::Result<()> {
        for agent_index in 0..self.agents.len() {
            if !self.agents[agent_index].alive {
                continue;
            }

            let observation = self.observation(agent_index);
            let seeded_choice = self.entropy.choose_index(observation.valid_actions.len());
            let proposal = decision_source.decide(&observation, seeded_choice);
            let result = self.apply_action(output, agent_index, &proposal)?;

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
                    write_event(
                        output,
                        self.tick,
                        &self.agents[agent_index].id,
                        "territory_crossed",
                        &format!("from:{previous_territory},to:{current_territory}"),
                    )?;
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
                write_event(
                    output,
                    self.tick,
                    &agent.id,
                    "food_consumed",
                    &format!(
                        "food:{},class:{},satiety:{}->{},energy:{}->{}",
                        food.id,
                        food.class,
                        previous_satiety,
                        agent.satiety,
                        previous_energy,
                        agent.energy
                    ),
                )?;
                Ok(ActionResult {
                    accepted: true,
                    detail: format!("food:{};class:{}", food.id, food.class),
                })
            }
        }
    }

    fn emit_action_trace<W: Write>(
        &self,
        output: &mut W,
        agent_index: usize,
        action: &Action,
        result: &ActionResult,
    ) -> io::Result<()> {
        let agent = &self.agents[agent_index];
        let status = if result.accepted {
            "accepted"
        } else {
            "rejected"
        };
        write_event(
            output,
            self.tick,
            &agent.id,
            "action_trace",
            &format!(
                "proposal:{action},status:{status},detail:{},position:{},territory:{},health:{},satiety:{},energy:{}",
                result.detail,
                agent.position,
                agent.position.territory(),
                agent.health,
                agent.satiety,
                agent.energy
            ),
        )
    }

    fn apply_survival<W: Write>(&mut self, output: &mut W, agent_index: usize) -> io::Result<()> {
        let agent = &mut self.agents[agent_index];
        let previous_health = agent.health;
        let previous_satiety = agent.satiety;
        let previous_energy = agent.energy;

        agent.satiety = agent.satiety.saturating_sub(2);
        agent.energy = agent.energy.saturating_sub(1);
        if agent.satiety == 0 || agent.energy == 0 {
            agent.health = agent.health.saturating_sub(5);
        }

        write_event(
            output,
            self.tick,
            &agent.id,
            "survival_changed",
            &format!(
                "health:{previous_health}->{},satiety:{previous_satiety}->{},energy:{previous_energy}->{}",
                agent.health, agent.satiety, agent.energy
            ),
        )?;

        if agent.health == 0 {
            agent.alive = false;
            write_event(output, self.tick, &agent.id, "agent_died", "health:0")?;
        }
        Ok(())
    }

    fn regenerate_food<W: Write>(
        &mut self,
        output: &mut W,
        territory: Territory,
    ) -> io::Result<()> {
        let current_count = self
            .foods
            .iter()
            .filter(|food| food.position.territory() == territory)
            .count();

        if current_count == 0 || current_count >= FOOD_CAPACITY_PER_TERRITORY {
            let reason = if current_count == 0 {
                "depleted"
            } else {
                "capacity"
            };
            return write_event(
                output,
                self.tick,
                &territory.to_string(),
                "food_regeneration_skipped",
                &format!("reason:{reason},count:{current_count}"),
            );
        }

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
        write_event(
            output,
            self.tick,
            &territory.to_string(),
            "food_regenerated",
            &format!("food:{id},class:{class},position:{position}"),
        )
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

fn write_event<W: Write>(
    output: &mut W,
    tick: u64,
    subject: &str,
    event: &str,
    result: &str,
) -> io::Result<()> {
    writeln!(
        output,
        "tick={tick} subject={subject} event={event} result={result}"
    )
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn config(seed: u64, tick_limit: u64, trace_actions: bool) -> Config {
        Config {
            seed,
            tick_limit,
            trace_actions,
        }
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

        assert_eq!(simulation.agents.len(), 12);
        assert_eq!(simulation.foods.len(), 6);

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

        for territory in Territory::ALL {
            for class in FoodClass::ALL {
                assert_eq!(
                    simulation
                        .foods
                        .iter()
                        .filter(|food| {
                            food.position.territory() == territory && food.class == class
                        })
                        .count(),
                    1
                );
            }
        }
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
            2
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
        simulation.tick = 10;
        simulation
            .foods
            .retain(|food| food.position.territory() == Territory::B);
        for number in 0..FOOD_CAPACITY_PER_TERRITORY {
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

        assert_eq!(simulation.food_counts(Territory::A)[0], 12);
        assert!(
            String::from_utf8(output)
                .unwrap()
                .contains("reason:capacity")
        );
    }

    struct InvalidNorthDecisionSource;

    impl DecisionSource for InvalidNorthDecisionSource {
        fn decide(&mut self, _observation: &Observation, _seeded_choice: usize) -> Action {
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
}
