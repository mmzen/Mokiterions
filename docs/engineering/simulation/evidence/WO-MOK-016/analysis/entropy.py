"""VER-MOK-016 oracle 2: the shared entropy stream's position, re-derived outside the crate.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-016/analysis/entropy.py

Writes its report to stdout and exits `0` when every check passes, non-zero otherwise.

WHAT THIS READER IS FOR
-----------------------
`VER-MOK-016` oracle 2 has two halves. The first reuses `VER-MOK-007`'s recorded
post-initialization stream expectations unchanged; those live in the suite as
`INITIALIZATION_DRAWS` in `mokiterions-core/src/simulation.rs`, as a *count* of values the
shared stream has produced. The second requires that the stream's state be equal immediately
before and immediately after each resolution this change adds.

The retention item asks for "the shared stream's recorded state either side of every resolution
kind, and the reused `VER-MOK-007` initialization expectations". A count is not a state, and the
engine has no command that prints one, so this reader computes the state from first principles
and independently of the engine:

  * `SplitMix64::new(seed)` sets `state = seed`, and `next_u64` advances the state by exactly one
    fixed increment `GAMMA` before mixing. So the absolute state after `k` values is
    `seed + k * GAMMA (mod 2^64)`, and a count is convertible to a state without inverting the
    mixing function.
  * `choose_index` costs exactly one value here, which is derived rather than assumed: its
    rejection threshold is `bound.wrapping_neg() % bound`, and both bounds in play -- `128` for
    `x` and `64` for the territory-local `y` -- divide `2^64`, so the threshold is `0` and no
    value is ever rejected for modulo bias. This is asserted below, because it is the step that
    makes "two values per coordinate attempt" exact rather than typical.

The placement of `Simulation::new` is then replayed in full, which yields the draw count, the
occupancy-rejection count and the final state for any `(seed, density)` pair -- including pairs
the suite does not record.

WHAT VALIDATES THE REPLAY
-------------------------
A reimplementation that agreed with nothing would only measure itself. Two things check it:

  * the 90 retained initialization excerpts in `baseline/init/`, which carry every food's class,
    position and territory and every Mokiterion's position, from the engine's own event stream.
    Every reconstructed position is compared against them, cell for cell.
  * `INITIALIZATION_DRAWS`, transcribed below, which the replay must reproduce at all fifteen
    declared `(seed, density)` pairs without being told the answer.

The excerpts also let the reader check a claim the suite states but no capture asserts on its
own: that the decision source and the trace setting move no placement draw. The six files that
share a `(seed, density)` pair must agree exactly, and they are compared.
"""

import re
import sys
from collections import defaultdict
from pathlib import Path

# LF regardless of platform, so a plain redirect reproduces the retained file byte for byte:
# `.gitattributes` pins this evidence tree `-text`, so a CRLF here would be committed as one.
sys.stdout.reconfigure(encoding="utf-8", newline="\n")

HERE = Path(__file__).resolve().parent
INIT = HERE.parent / "baseline" / "init"

GAMMA = 0x9E37_79B9_7F4A_7C15
MIX_1 = 0xBF58_476D_1CE4_E5B9
MIX_2 = 0x94D0_49BB_1331_11EB
MASK = (1 << 64) - 1

WORLD_SIZE = 128
TERRITORY_HEIGHT = 64
CELLS_PER_TERRITORY = WORLD_SIZE * TERRITORY_HEIGHT

CLASSES = ("low", "medium", "high")
SEEDS = (0, 1, 42, 123, 777)
DENSITIES = (("0.15", 15), ("0.75", 75), ("1.50", 150))

# `mokiterions-core/src/simulation.rs`, `INITIALIZATION_DRAWS`, transcribed. The reader must
# reproduce these without reading them: they are compared, never used as input.
RECORDED_DRAWS = {
    "0.15": (72, 72, 72, 72, 72),
    "0.75": (270, 268, 268, 268, 268),
    "1.50": (516, 516, 516, 514, 516),
}

# The constructed encounter every resolution test builds: `encounter(9, ..)` calls
# `Simulation::new(social_config(9, 1_000, false))`, whose density is `Density::DEFAULT`.
ENCOUNTER_SEED = 9
DEFAULT_HUNDREDTHS = 75

# `no_resolution_and_no_targeted_move_touches_the_shared_stream`, transcribed from
# `mokiterions-core/src/simulation.rs`: the seven targeted verbs of `targeted_proposals`, over
# the four geometries the test enumerates.
VERBS = ("attack", "threaten", "fight", "retreat", "surrender", "approach", "avoid")
GEOMETRIES = (
    ("in contact, record present", "40:40 / 41:40", "all seven reachable"),
    ("perceived, not in contact", "40:40 / 48:40", "the moves apply; the three contact verbs are rejected"),
    ("co-located", "40:40 / 40:40", "the move fallbacks and `target_co_located`"),
    ("in contact, no record", "40:40 / 41:40", "`fight`, `retreat` and `surrender` are rejected"),
)

SOURCE = Path("mokiterions-core/src/simulation.rs")
if not SOURCE.exists():
    SOURCE = HERE.parents[4] / "mokiterions-core" / "src" / "simulation.rs"
lines = SOURCE.read_text(encoding="utf-8").splitlines()
boundary = next(index for index, text in enumerate(lines, start=1) if text == "#[cfg(test)]")

CALL = re.compile(r"\.(next_u64|choose_index)\(")
STREAM = re.compile(r"\bentropy\b|\bnext_u64\b|\bchoose_index\b|\bSplitMix64\b|\bDecisionEntropy\b")
FUNCTION = re.compile(r"^(\s*)fn (\w+)")
IMPL = re.compile(r"^impl(?:<[^>]*>)?\s+(?:\w+(?:<[^>]*>)?\s+for\s+)?(\w+)")
TRAIT = re.compile(r"^trait\s+(\w+)")


def extent(start):
    """`start` is 1-indexed. Returns the function's 1-indexed last line, and whether it has a
    body: a trait declaration ends its signature with `;` and owns no lines beyond it."""
    for number in range(start, boundary):
        stripped = lines[number - 1].rstrip()
        if stripped.endswith(";"):
            return number, False
        if stripped.endswith("{"):
            break
    indent = len(lines[start - 1]) - len(lines[start - 1].lstrip())
    closer = " " * indent + "}"
    for number in range(start + 1, boundary):
        if lines[number - 1] == closer:
            return number, True
    return boundary - 1, True


functions = []
context = None
for number, text in enumerate(lines[: boundary - 1], start=1):
    impl = IMPL.match(text) or TRAIT.match(text)
    if impl:
        context = impl.group(1)
    elif text == "}":
        context = None
    match = FUNCTION.match(text)
    if match:
        end, has_body = extent(number)
        label = f"{context}::{match.group(2)}" if context and match.group(1) else match.group(2)
        functions.append((number, label if has_body else f"{label}  (declaration)", end))

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


class SplitMix64:
    """`mokiterions-core/src/simulation.rs`'s generator, reimplemented."""

    def __init__(self, seed):
        self.state = seed & MASK
        self.draws = 0

    def next_u64(self):
        self.state = (self.state + GAMMA) & MASK
        value = self.state
        value = ((value ^ (value >> 30)) * MIX_1) & MASK
        value = ((value ^ (value >> 27)) * MIX_2) & MASK
        self.draws += 1
        return value ^ (value >> 31)

    def choose_index(self, bound):
        threshold = ((-bound) & MASK) % bound
        while True:
            value = self.next_u64()
            if value >= threshold:
                return value % bound


def resources_per_territory(hundredths):
    return hundredths * CELLS_PER_TERRITORY // 10_000


def initialize(seed, hundredths):
    """`Simulation::new`'s placement, replayed. Returns the reconstruction and the stream."""
    per_territory = resources_per_territory(hundredths)
    entropy = SplitMix64(seed)
    attempts = 0
    occupancy_rejections = 0

    foods = []
    for territory in ("A", "B"):
        for index in range(per_territory):
            occupied = {food[1] for food in foods if food[2] == territory}
            while True:
                attempts += 1
                x = entropy.choose_index(WORLD_SIZE)
                local_y = entropy.choose_index(TERRITORY_HEIGHT)
                y = local_y if territory == "A" else local_y + TERRITORY_HEIGHT
                if (x, y) not in occupied:
                    break
                occupancy_rejections += 1
            foods.append((CLASSES[index % 3], (x, y), territory))

    agents = []
    taken = []
    for number in range(1, 13):
        territory = "A" if number <= 6 else "B"
        while True:
            attempts += 1
            x = entropy.choose_index(WORLD_SIZE)
            local_y = entropy.choose_index(TERRITORY_HEIGHT)
            y = local_y if territory == "A" else local_y + TERRITORY_HEIGHT
            if (x, y) not in taken:
                break
            occupancy_rejections += 1
        taken.append((x, y))
        agents.append(((x, y), territory))

    return {
        "foods": foods,
        "agents": agents,
        "draws": entropy.draws,
        "state": entropy.state,
        "attempts": attempts,
        "occupancy_rejections": occupancy_rejections,
        "per_territory": per_territory,
    }


FOOD_LINE = re.compile(
    r"subject=(F\d+) event=food_initialized result=class:(\w+),position:(\d+):(\d+),territory:(\w)"
)
AGENT_LINE = re.compile(
    r"subject=(M\d+) event=agent_initialized result=name:\w+,position:(\d+):(\d+),territory:(\w)"
)


def read_excerpt(path):
    foods, agents = [], []
    for line in path.read_text(encoding="utf-8").splitlines():
        food = FOOD_LINE.search(line)
        if food:
            foods.append((food.group(2), (int(food.group(3)), int(food.group(4))), food.group(5)))
            continue
        agent = AGENT_LINE.search(line)
        if agent:
            agents.append(((int(agent.group(2)), int(agent.group(3))), agent.group(4)))
    return foods, agents


def rule(title):
    print()
    print(title)
    print("-" * len(title))


TITLE = "VER-MOK-016 oracle 2: where the shared entropy stream stands"
print(TITLE)
print("=" * len(TITLE))
print()
print("Work order   WO-MOK-016 (Phase 3.1, encounters)")
print('Retains      "the shared stream\'s recorded state either side of every resolution kind, and')
print('             the reused VER-MOK-007 initialization expectations"')
print("Authority    VER-MOK-016 oracle 2; the no-entropy constraints of REQ-MOK-052, REQ-MOK-053,")
print("             REQ-MOK-054, REQ-MOK-055 and REQ-MOK-056; REQ-MOK-057's at-most-one-draw")
print("             constraint; REQ-MOK-031's and REQ-MOK-040's untouched-stream constraints")
print("Candidate    6c1496befb1a34f08cac6124ea2133460fe7a8e5   (feature/phase-3-definition)")
print("Baseline     39662d13abd08e3410648d1c59ad38384f8ad2d2   (baseline/COMMIT.txt)")
print("Reader       analysis/entropy.py")
print("Date         2026-08-21")
print()
print("VER-MOK-016 oracle 2 asks for a state, and INITIALIZATION_DRAWS holds a count. The engine")
print("has no command that prints either, so this file computes the state and then checks the")
print("computation against things the engine did print. Sections 1 to 3 derive the count and the")
print("state and reproduce the suite's fifteen recorded counts; sections 4 and 5 check the")
print("reconstruction against 90 retained initialization excerpts; section 6 gives the state at")
print("which every resolution in the suite is asserted to leave the stream unmoved; section 7")
print("enumerates every draw site the shipped engine has; section 8 says what none of it settles.")
print()
print("Constants, transcribed from mokiterions-core/src/simulation.rs:")
print()
print(f"  GAMMA               0x{GAMMA:016X}  ({GAMMA})")
print(f"  MIX_1               0x{MIX_1:016X}")
print(f"  MIX_2               0x{MIX_2:016X}")
print(f"  WORLD_SIZE          {WORLD_SIZE}")
print(f"  TERRITORY_HEIGHT    {TERRITORY_HEIGHT}")
print(f"  CELLS_PER_TERRITORY {CELLS_PER_TERRITORY}")

rule("1. One value per index, derived rather than assumed")
print()
print("`choose_index` rejects a value below `bound.wrapping_neg() % bound`. Both bounds in play")
print("divide 2^64, so the threshold is zero and nothing is ever rejected for modulo bias:")
print()
for bound in (WORLD_SIZE, TERRITORY_HEIGHT):
    threshold = ((-bound) & MASK) % bound
    print(f"    bound {bound:>3}   wrapping_neg() % bound = {threshold}")
    check(threshold == 0, f"choose_index threshold for bound {bound} is {threshold}, not 0")
print()
print("So a coordinate attempt costs exactly two values, and the draw count below is exact.")
print()
print("Control -- a bound that does not divide 2^64 would reject, which is what makes the check")
print("above load-bearing rather than decorative:")
print()
for bound in (100, 61, 3):
    print(f"    bound {bound:>3}   wrapping_neg() % bound = {((-bound) & MASK) % bound}")

rule("2. The generator reproduces the crate's own recorded values")
print()
print("`splitmix64_sequence_is_stable`, in `mokiterions-core/src/simulation.rs`, asserts the first")
print("three values at seed `0`. This reader is not told them; it computes them:")
print()
probe = SplitMix64(0)
expected_first = (0xE220_A839_7B1D_CDAF, 0x6E78_9E6A_A1B9_65F4, 0x06C4_5D18_8009_454F)
for index, expected in enumerate(expected_first, start=1):
    produced = probe.next_u64()
    ok = produced == expected
    check(ok, f"value {index} at seed 0 is 0x{produced:016X}, expected 0x{expected:016X}")
    print(f"    value {index}   0x{produced:016X}   {'matches' if ok else 'DIFFERS FROM'}  0x{expected:016X}")

rule("3. Post-initialization draw count and absolute state, all fifteen declared pairs")
print()
print("`draws` is replayed, not read. `recorded` is `INITIALIZATION_DRAWS` transcribed from the")
print("suite. `attempts` is coordinate attempts; `rejected` is attempts refused as occupied.")
print("`state` is the shared stream's absolute position, `seed + draws * GAMMA (mod 2^64)`.")
print()
print(f"    {'density':>7} {'seed':>4} {'entities':>8} {'attempts':>8} {'rejected':>8} "
      f"{'draws':>6} {'recorded':>8}  state")
states = {}
for label, hundredths in DENSITIES:
    for position, seed in enumerate(SEEDS):
        result = initialize(seed, hundredths)
        entities = result["per_territory"] * 2 + 12
        recorded = RECORDED_DRAWS[label][position]
        derived = (seed + result["draws"] * GAMMA) & MASK
        check(
            result["draws"] == recorded,
            f"draw count at seed {seed} density {label} is {result['draws']}, "
            f"INITIALIZATION_DRAWS records {recorded}",
        )
        check(
            derived == result["state"],
            f"state arithmetic disagrees with the replayed state at seed {seed} density {label}",
        )
        check(
            result["draws"] == 2 * result["attempts"],
            f"draws is not twice attempts at seed {seed} density {label}",
        )
        states[(label, seed)] = result["state"]
        flag = "" if result["draws"] == recorded else "   <-- DIFFERS"
        print(f"    {label:>7} {seed:>4} {entities:>8} {result['attempts']:>8} "
              f"{result['occupancy_rejections']:>8} {result['draws']:>6} {recorded:>8}  "
              f"0x{result['state']:016X}{flag}")
print()
print("The count decomposes exactly, which is what makes a wrong figure visible rather than")
print("merely different: `2 x (2 x resources_per_territory + 12)` is the no-rejection floor, and")
print("every excess is two values per occupied cell refused.")
print()
print(f"    {'density':>7} {'per territory':>13} {'floor':>6}  seeds above the floor")
for label, hundredths in DENSITIES:
    per = resources_per_territory(hundredths)
    floor = 2 * (2 * per + 12)
    above = [
        f"{seed} (+{RECORDED_DRAWS[label][index] - floor})"
        for index, seed in enumerate(SEEDS)
        if RECORDED_DRAWS[label][index] != floor
    ]
    print(f"    {label:>7} {per:>13} {floor:>6}  {', '.join(above) if above else 'none'}")

rule("4. The replay against the engine's own event stream, all 90 excerpts")
print()
print("Every food's class, position and territory and every Mokiterion's position, compared")
print("cell for cell against `baseline/init/`.")
print()
excerpts = sorted(INIT.glob("*.txt"))
check(len(excerpts) == 90, f"expected 90 initialization excerpts, found {len(excerpts)}")
by_pair = defaultdict(list)
compared_cells = 0
mismatched_files = []
for path in excerpts:
    match = re.match(r"seed(\d+)-(\w+)-d([\d.]+)-trace(on|off)\.txt", path.name)
    check(match is not None, f"unparsed excerpt name: {path.name}")
    if not match:
        continue
    seed = int(match.group(1))
    label = match.group(3)
    hundredths = dict((a, b) for a, b in DENSITIES)[label]
    foods, agents = read_excerpt(path)
    by_pair[(label, seed)].append((path.name, foods, agents))
    replay = initialize(seed, hundredths)
    if foods != replay["foods"] or agents != replay["agents"]:
        mismatched_files.append(path.name)
    compared_cells += len(foods) + len(agents)
print(f"    excerpts read             {len(excerpts)}")
print(f"    cells compared            {compared_cells}")
print(f"    files matching the replay {len(excerpts) - len(mismatched_files)}")
print(f"    files differing           {len(mismatched_files)}")
check(not mismatched_files, f"replay differs from the engine in: {', '.join(mismatched_files[:5])}")
if mismatched_files:
    for name in mismatched_files[:5]:
        print(f"      DIFFERS: {name}")

rule("5. The source and the trace setting move no placement draw")
print()
print("The six excerpts sharing a `(seed, density)` pair -- three sources by two trace settings --")
print("must be identical in every placed cell. This is asserted nowhere else in the packet.")
print()
groups_checked = 0
disagreeing = 0
for (label, seed), entries in sorted(by_pair.items()):
    check(len(entries) == 6, f"pair d{label} seed {seed} has {len(entries)} excerpts, expected 6")
    reference = entries[0]
    for name, foods, agents in entries[1:]:
        if foods != reference[1] or agents != reference[2]:
            disagreeing += 1
            check(False, f"{name} differs from {reference[0]} in a placed cell")
    groups_checked += 1
print(f"    pairs checked            {groups_checked}")
print(f"    excerpts per pair        6")
print(f"    pairs whose six disagree {disagreeing}")

rule("6. The state either side of every resolution, at the constructed encounter")
print()
print("`encounter(9, ..)` calls `Simulation::new(social_config(9, 1_000, false))` and then sets")
print("`tick`, clears `foods` and moves positions. None of those draws, so the stream sits where")
print("initialization left it when the resolution is applied.")
print()
encounter = initialize(ENCOUNTER_SEED, DEFAULT_HUNDREDTHS)
before = encounter["state"]
print(f"    seed                     {ENCOUNTER_SEED}")
print(f"    density                  0.75 (Density::DEFAULT), {encounter['per_territory']} per territory")
print(f"    coordinate attempts      {encounter['attempts']}")
print(f"    refused as occupied      {encounter['occupancy_rejections']}")
print(f"    draws                    {encounter['draws']}")
print(f"    state BEFORE             0x{before:016X}")
print(f"    state AFTER (asserted)   0x{before:016X}")
print()
print("The suite builds encounters at more seeds than nine, and each has its own state. Every")
print("`encounter(..)` call in the file is found and its seed reported, so a seed added later")
print("appears here rather than being covered by silence:")
print()
ENCOUNTER_CALL = re.compile(r"\bencounter\((\d+),")
encounter_seeds = defaultdict(int)
for text in lines:
    for seed in ENCOUNTER_CALL.findall(text):
        encounter_seeds[int(seed)] += 1
print(f"    {'seed':>4} {'sites':>5} {'attempts':>8} {'rejected':>8} {'draws':>6}  "
      f"state either side of every resolution")
for seed in sorted(encounter_seeds):
    result = initialize(seed, DEFAULT_HUNDREDTHS)
    check(
        result["draws"] == 2 * result["attempts"],
        f"encounter seed {seed}: draws is not twice attempts",
    )
    print(f"    {seed:>4} {encounter_seeds[seed]:>5} {result['attempts']:>8} "
          f"{result['occupancy_rejections']:>8} {result['draws']:>6}  0x{result['state']:016X}")
print(f"    {'all':>4} {sum(encounter_seeds.values()):>5}  call sites, over "
      f"{len(encounter_seeds)} distinct seeds")
check(bool(encounter_seeds), "no `encounter(..)` call site found, which cannot be")
print()
print("`encounter` is one helper -- `Simulation::new(social_config(seed, 1_000, false))`, then a")
print("tick number, a cleared food vector and twelve position assignments -- so every row above is")
print("the state at which that seed's resolutions are asserted to leave the stream unmoved.")
print()
print("The table checks itself. Because the state is `seed + draws * GAMMA`, two seeds with the")
print("same draw count must have states differing by exactly their seed difference, and two with")
print("different counts must not. Both hold in every pair:")
print()
groups = defaultdict(list)
for seed in sorted(encounter_seeds):
    result = initialize(seed, DEFAULT_HUNDREDTHS)
    groups[result["draws"]].append((seed, result["state"]))
for draws in sorted(groups):
    members = groups[draws]
    anchor_seed, anchor_state = members[0]
    print(f"    {draws} draws: seeds {', '.join(str(seed) for seed, _ in members)}")
    for seed, state in members[1:]:
        check(
            (state - anchor_state) & MASK == (seed - anchor_seed) & MASK,
            f"encounter seeds {anchor_seed} and {seed} share a draw count but not their offset",
        )
    print(f"      offsets from seed {anchor_seed}: "
          f"{', '.join(f'+{(state - anchor_state) & MASK}' for _, state in members[1:]) or 'none'}")
crossings = 0
for left in sorted(groups):
    for right in sorted(groups):
        if left >= right:
            continue
        for seed_a, state_a in groups[left]:
            for seed_b, state_b in groups[right]:
                crossings += 1
                check(
                    (state_b - state_a) & MASK != (seed_b - seed_a) & MASK,
                    f"seeds {seed_a} and {seed_b} differ in draw count but not in state offset",
                )
print(f"    and across the four groups, {crossings} cross pairs, none of which has the seed")
print("    difference as its state offset -- so an unnoticed change in draw count could not")
print("    produce this table.")
print()
print("`no_resolution_and_no_targeted_move_touches_the_shared_stream` asserts")
print("`simulation.entropy == before` after each application, over these 28 cases:")
print()
print(f"    {'geometry':<28} {'positions':<16} verbs")
for name, positions, _ in GEOMETRIES:
    print(f"    {name:<28} {positions:<16} {', '.join(VERBS)}")
print()
print(f"    cases: {len(GEOMETRIES)} geometries x {len(VERBS)} verbs = {len(GEOMETRIES) * len(VERBS)}")
print()
print("The four resolutions this change adds -- attack, fight, threaten, surrender -- and both")
print("targeted moves -- approach, avoid -- are each covered at every geometry, with retreat the")
print("seventh verb. Equality is what the engine asserts; the table above is what it holds at.")
print()
print("Control -- a single restored draw would leave the count equal and the state moved, so the")
print("state is the quantity that catches it and the count is not:")
print()
print(f"    one further value        0x{(before + GAMMA) & MASK:016X}")
print(f"    difference               GAMMA")
check(((before + GAMMA) & MASK) != before, "GAMMA is zero, which cannot be")

rule("7. Every draw site in the shipped engine, enumerated")
print()
print("The `no entropy draw` constraints of `REQ-MOK-052`, `REQ-MOK-053`, `REQ-MOK-054`,")
print("`REQ-MOK-055` and `REQ-MOK-056` are claims about code that does not exist, and a test can")
print("only sample them. This section enumerates instead: every call to `next_u64` or")
print("`choose_index` in the shipped half of `mokiterions-core/src/simulation.rs`, checked against")
print("the set below. A draw added anywhere in shipped code fails this check by appearing.")
print()
print(f"    file                     {SOURCE.as_posix()}")
print(f"    lines                    {len(lines)}")
print(f"    `#[cfg(test)]` at        {boundary} -- shipped code is lines 1 to {boundary - 1}")
print()

EXPECTED_CALLS = (
    ("generator.choose_index(usize::from(WASTE_TOLERANCE_MAX) + 1) as u8",
     "the trait's own side generator -- not the shared stream"),
    ("self.stream.choose_index(upper_bound)",
     "`DecisionEntropy::choose_index`, the only path a source has to the stream"),
    ("let choice = entropy.choose_index(observation.valid_actions.len());",
     "`baseline`'s uniform selection, one per opportunity"),
    ("let choice = entropy.choose_index(moves.len());",
     "`reference`'s rule 19 case 4"),
    ("let choice = entropy.choose_index(moves.len());",
     "`tolerant_search_choice` -- rule 19 case 4, reached by `individual` and by `social` branch 6"),
    ("let value = self.next_u64();",
     "inside `SplitMix64::choose_index` itself"),
    ("let class = FoodClass::ALL[self.entropy.choose_index(FoodClass::ALL.len())];",
     "rule 16's regeneration class"),
    ("let x = entropy.choose_index(WORLD_SIZE as usize) as u8;",
     "`choose_free_coordinate`, used by initialization and by regeneration"),
    ("let local_y = entropy.choose_index(TERRITORY_HEIGHT as usize) as u8;",
     "the same"),
)
found_calls = [
    (number, text.strip())
    for number, text in enumerate(lines[: boundary - 1], start=1)
    if CALL.search(text)
]
print(f"    {'line':>6}  site")
for number, text in found_calls:
    print(f"    {number:>6}  {text}")
print()
check(
    [text for _, text in found_calls] == [text for text, _ in EXPECTED_CALLS],
    f"the shipped draw sites are not the {len(EXPECTED_CALLS)} expected: found {len(found_calls)}",
)
print(f"    sites found {len(found_calls)}, expected {len(EXPECTED_CALLS)}: "
      f"{'the same set, in the same order' if len(found_calls) == len(EXPECTED_CALLS) else 'DIFFERENT'}")
print()
print("    what each one is:")
for (_, purpose), (number, _) in zip(EXPECTED_CALLS, found_calls):
    print(f"      {number:>6}  {purpose}")
print()
print("Two of the nine are not draws against the shared stream at all. The first is the trait's")
print("side generator, which is `REQ-MOK-031`'s structural claim rather than an asserted one: the")
print("shipped file constructs a `SplitMix64` in exactly two places, and they are different")
print("objects.")
print()
constructions = [
    (number, text.strip())
    for number, text in enumerate(lines[: boundary - 1], start=1)
    if "SplitMix64::new(" in text
]
for number, text in constructions:
    print(f"    {number:>6}  {text}")
check(len(constructions) == 2, f"expected 2 shipped `SplitMix64::new` sites, found {len(constructions)}")
print()
print("The sixth is `choose_index`'s own loop, which is where the other draws land. That leaves")
print("**seven** call sites against the shared stream, and they are not seven independent draws:")
print("`DecisionEntropy::choose_index` at 882 is the funnel the three decision-source sites reach")
print("the stream through, so the distinct origins are **six** -- `baseline`'s selection,")
print("`reference`'s case 4, `tolerant_search_choice`, the regeneration class and the two")
print("coordinate indices -- of which three can only draw by passing through the counted wrapper.")
print("That is the bounded capability `DecisionEntropy`'s doc comment describes, and it is checkable")
print("rather than merely described: no `decide` in the file mentions `SplitMix64` anywhere in its")
print("body, so a source cannot reach past the wrapper's `draws` counter without a signature change")
print("that this enumeration would show.")
print()
deciders = [
    (start, name, end)
    for start, name, end in functions
    if name.endswith("::decide") and "declaration" not in name
]
for start, name, end in deciders:
    body = lines[start - 1 : end]
    leaks = [
        offset
        for offset, text in enumerate(body, start=start)
        if "SplitMix64" in text and not text.strip().startswith("//")
    ]
    check(not leaks, f"{name} mentions SplitMix64 at {leaks}")
    print(f"    {start:>5}-{end:<6}  {name} -- {'no `SplitMix64`' if not leaks else f'LEAKS at {leaks}'}")
check(len(deciders) == 4, f"expected 4 `decide` implementations, found {len(deciders)}")

print()
print("**No resolution path can draw, because none of them is given the stream.** Every function")
print("in the shipped half is taken in turn, its extent read from `rustfmt`'s indentation, and its")
print("body scanned for a draw call and for any mention of the stream outside a comment. Comments")
print("are excluded deliberately: `resolve_strike`'s own doc comment at line 2612 asserts that")
print("\"no entropy is drawn by any part of this\", and a claim in a comment is the thing being")
print("checked, not the check.")
print()
touching, clean = [], []
for start, name, end in functions:
    body = [
        (offset, text)
        for offset, text in enumerate(lines[start - 1 : end], start=start)
        if not text.strip().startswith("//")
    ]
    draws = [offset for offset, text in body if CALL.search(text)]
    mentions = [offset for offset, text in body if STREAM.search(text)]
    (touching if mentions else clean).append((start, name, end, draws, mentions))

print(f"    functions in the shipped half            {len(functions)}")
print(f"    mentioning the stream outside a comment  {len(touching)}")
print(f"    not mentioning it at all                 {len(clean)}")
print()
print(f"    {'lines':>12}  {'draws':>5}  function")
for start, name, end, draws, _ in touching:
    print(f"    {start:>5}-{end:<6}  {len(draws):>5}  {name}")
print()
RESOLUTION = re.compile(r"^(?:\w+::)?(apply|resolve|validate)_")
resolution_side = [entry for entry in functions if RESOLUTION.match(entry[1])]
compromised = [
    (start, name)
    for start, name, end, draws, mentions in touching
    if RESOLUTION.match(name)
]
print(f"    {'lines':>12}  resolution, validation and application functions, and what they hold")
for start, name, end in resolution_side:
    entry = next(item for item in touching + clean if item[0] == start)
    verdict = "no draw, no stream" if not entry[4] else f"MENTIONS THE STREAM at {entry[4]}"
    print(f"    {start:>5}-{end:<6}  {name} -- {verdict}")
print()
check(
    len(resolution_side) >= 9,
    f"only {len(resolution_side)} resolution-side functions found, expected at least 9",
)
check(
    not compromised,
    f"{len(compromised)} resolution-side function(s) mention the stream: "
    f"{', '.join(name for _, name in compromised)}",
)
print(f"    {len(resolution_side)} functions, {len(compromised)} of them able to reach the stream.")
print()
print("So `validate_targeted`, `apply_targeted_action`, `apply_targeted_move`, `resolve_strike`,")
print("`resolve_threat`, `resolve_surrender`, `apply_action`, `apply_move` and `apply_survival`")
print("do not merely refrain from drawing: none of them is passed the stream. The field is bound")
print("in the shipped `impl Simulation` at three places only -- the decision call at line 2163,")
print("regeneration's class at 2905 and regeneration's placement at 2912 -- and a resolution is")
print("none of the three. That is `REQ-MOK-053`'s *No entropy draw is taken by validation, damage")
print("computation, cost application or death handling* discharged structurally rather than by")
print("sampling, and the same enumeration discharges the identical constraint in `REQ-MOK-052`,")
print("`REQ-MOK-054`, `REQ-MOK-055` and `REQ-MOK-056`.")

print()
print("**The `social` source draws in one branch, and the enumeration says which.** `entropy`")
print("reaches the body of `SocialDecisionSource::decide` exactly once:")
print()
decide_start = next(
    number
    for number, text in enumerate(lines[: boundary - 1], start=1)
    if number > 1100 and "fn decide" in text
)
decide_end = next(
    number
    for number, text in enumerate(lines[: boundary - 1], start=1)
    if number > decide_start and text == "    }"
)
body_uses = [
    (number, text.strip())
    for number, text in enumerate(lines[decide_start - 1 : decide_end], start=decide_start)
    if "entropy" in text and "fn decide" not in text
]
returns = [
    number
    for number, text in enumerate(lines[decide_start - 1 : decide_end], start=decide_start)
    if "return " in text
]
print(f"    body                     lines {decide_start} to {decide_end}")
print(f"    `return` statements      {len(returns)} -- branches 1 to 5, at lines "
      f"{', '.join(str(number) for number in returns)}")
for number, text in body_uses:
    print(f"    entropy at line {number:>6}  {text}")
check(len(body_uses) == 1, f"`entropy` appears {len(body_uses)} times in the social body, expected 1")
check(
    bool(body_uses) and all(number < body_uses[0][0] for number in returns),
    "a `return` in the social body follows the draw, so a branch above 6 could draw",
)
print()
print("Every `return` precedes it, so branches 1 to 5 cannot reach the stream by construction and")
print("branch 6 draws exactly once through `tolerant_search_choice`. `post/branches.md` §2 counts")
print("branch 6 at **37,975** of 118,201 decisions across the fifteen traced cells; with this")
print("enumeration that count *is* the source's entire draw consumption, and the other 80,226")
print("decisions are derived. This is the claim `branches.md` defers to this file, and what makes")
print("it a measurement rather than a restatement is the enumeration above: the count is the draw")
print("total only because no other branch has a draw site to reach.")

rule("8. What this reader does not establish")
print()
print("It does not re-measure the engine. The draw count is reproduced from the placement")
print("arithmetic and checked against the engine's placed cells, which is a strong check on the")
print("reconstruction and no check at all on whether some later code path draws. That claim is")
print("carried by the in-crate assertion named in section 6 and by `shared_stream_draws`.")
print()
print("It does not exhibit the state at a mid-run resolution. The encounter tests construct their")
print("world and resolve immediately, which is why their state is computable here; a resolution")
print("reached by running the world sits at a position no released stream reports, because the")
print("engine has no command that prints the stream. Bounding that would need an engine-side")
print("diagnostic this work order did not add, and it is recorded as a residual rather than")
print("implied to be covered.")

print()
print("=" * 74)
if failures:
    print(f"RESULT: FAIL -- {len(failures)} check(s) failed")
    for message in failures:
        print(f"  - {message}")
    sys.exit(1)
print("RESULT: PASS -- every check above holds")
