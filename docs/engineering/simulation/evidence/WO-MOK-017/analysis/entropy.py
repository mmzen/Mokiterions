"""WO-MOK-017: where the shared entropy stream stands, either side of the correction.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/entropy.py \
        <pre-capture-dir> <post-capture-dir> > docs/.../post/entropy.txt

Writes its report to stdout and exits `0` when every check passes, non-zero otherwise.

THE RETENTION ITEM, AND WHICH HALF OF IT IS DERIVABLE HERE
---------------------------------------------------------
`WO-MOK-017` retains "the shared stream's recorded state either side of every resolution kind, and the
total draw count per run per source at matched seeds". Those two halves are not equally writable at this
candidate, and the difference is stated up front rather than left for a reader to notice.

**Derivable, and measured below.** The stream's state at the end of initialization, absolutely rather
than as a count, at all fifteen declared `(seed, density)` pairs and at both commits. `SplitMix64`
advances its state by one fixed increment per value, so `state = seed + k*GAMMA (mod 2^64)` and a draw
count converts to a state without inverting the mixing function. The placement of `Simulation::new` is
replayed here from first principles, which yields `k`, and the replay is validated against the 120
retained initialization excerpts and against `INITIALIZATION_DRAWS` transcribed from the suite.

**Derivable, and measured below.** That the corrected condition consumes no entropy. This is the check
that matters most for this work order rather than for its predecessor: if `fits_within` drew from the
shared stream, every divergence in `post/divergence.txt` would be confounded between "the condition
changed an answer" and "the condition moved the stream", and route R2 there would be unfalsifiable.

**Derivable, and measured below.** The count of every resolution kind per cell at both commits, and the
byte-identity of the whole initialization prefix in all 120 cells.

**NOT derivable at this candidate: the total draw count per run.** The engine exposes no counter outside
its own test module -- `--help` lists no such option and the rule 18 summary line carries no such field
-- so a per-run total can be had in only two ways, and this work order permits neither:

  * Instrumenting the engine to print one. `WO-MOK-017`'s *Out of scope* forbids "any new attribute,
    event, interface item or instrumentation", and a draw counter on the released surface is exactly
    that.
  * Replaying a thousand ticks outside the crate. Initialization is replayable because placement is a
    closed loop over two coordinate draws; a full run is not, because the number of draws a regeneration
    spends depends on how many occupancy rejections it hits, and a rejection is invisible in the stream
    -- only the accepted position is printed. Deriving it would mean reimplementing every draw-consuming
    path in the engine, which is the second implementation this project declines to write. It would
    agree with whichever of the two was written second.

So the total is recorded here as a residual gap with its reason, and what stands in its place is stated
exactly: the initialization total measured at both commits, the resolution counts either side, and the
static demonstration that the correction adds no draw. The gap is narrower than `WO-MOK-016`'s own
unwritable clause was, and it is the owner's to close or accept -- not this reader's to paper over with
a proxy that looks like a total.

WHAT IS REUSED, AND WHY THAT IS SOUND
-------------------------------------
The `SplitMix64` reimplementation, the `choose_index` rejection argument and the `initialize` placement
replay are reused from `WO-MOK-016`'s `analysis/entropy.py` rather than rewritten. That is sound here
because placement is on the far side of this change: `REQ-MOK-060` corrects a condition consulted when a
Mokiterion evaluates a perceived resource, and `Simulation::new` has run to completion before the first
Mokiterion perceives anything. The reuse is also *checked* rather than assumed -- the replay is validated
against this work order's own 120 retained excerpts, at the candidate, so a placement change would break
it here even though the code came from the previous work order.
"""

import io
import os
import re
import sys
from collections import Counter
from pathlib import Path

# LF regardless of platform, so a plain redirect reproduces the retained file byte for byte:
# `.gitattributes` pins this evidence tree `-text`, so a CRLF here would be committed as one.
sys.stdout.reconfigure(encoding='utf-8', newline='\n')

GAMMA = 0x9E37_79B9_7F4A_7C15
MIX_1 = 0xBF58_476D_1CE4_E5B9
MIX_2 = 0x94D0_49BB_1331_11EB
MASK = (1 << 64) - 1

WORLD_SIZE = 128
TERRITORY_HEIGHT = 64
CELLS_PER_TERRITORY = WORLD_SIZE * TERRITORY_HEIGHT

CLASSES = ('low', 'medium', 'high')
SEEDS = (0, 1, 42, 123, 777)
DENSITIES = (('0.15', 15), ('0.75', 75), ('1.50', 150))
SOURCES = ('baseline', 'reference', 'individual', 'social')
RESOLUTIONS = ('attack_resolved', 'threat_resolved', 'surrender_resolved')

# `mokiterions-core/src/simulation.rs`, `INITIALIZATION_DRAWS`, transcribed. The replay must
# reproduce these without reading them: they are compared, never used as input.
RECORDED_DRAWS = {
    '0.15': (72, 72, 72, 72, 72),
    '0.75': (270, 268, 268, 268, 268),
    '1.50': (516, 516, 516, 514, 516),
}

# The functions the correction touches, and the functions that consult them. None may reach the
# shared stream. `fits_within` is new at this candidate; the rest are the callers it is reached
# through, so a draw introduced anywhere on the path would be caught.
CORRECTION_PATH = (
    'Observation::fits_within',
    'Observation::fits',
    'Observation::fits_within_tolerance',
    'Observation::best_fitting_co_located_food',
    'Observation::best_fitting_distant_food',
    'Observation::best_tolerated_co_located_food',
    'Observation::best_tolerated_distant_food',
)

DECIDERS = tuple(f'{source}DecisionSource::decide'
                 for source in ('Baseline', 'Reference', 'Individual', 'Social'))

STREAM_CALL = re.compile(r'\b(next_u64|choose_index|entropy|SplitMix64|DecisionEntropy)\b')
FUNCTION = re.compile(r'^(\s*)fn (\w+)')
IMPL = re.compile(r'^impl(?:<[^>]*>)?\s+(?:\w+(?:<[^>]*>)?\s+for\s+)?(\w+)')

FOOD_LINE = re.compile(
    r'subject=(F\d+) event=food_initialized result=class:(\w+),position:(\d+):(\d+),territory:(\w)')
AGENT_LINE = re.compile(
    r'subject=(M\d+) event=agent_initialized result=name:\w+,position:(\d+):(\d+),territory:(\w)')
RECORD = re.compile(r'^tick=(\d+) subject=(\S+) event=(\w+) ')

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


class SplitMix64:
    """`mokiterions-core/src/simulation.rs`'s generator, reimplemented.

    Reused verbatim from `WO-MOK-016/analysis/entropy.py`.
    """

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
    """`Simulation::new`'s placement, replayed. Reused from `WO-MOK-016/analysis/entropy.py`."""
    per_territory = resources_per_territory(hundredths)
    entropy = SplitMix64(seed)
    attempts = 0
    occupancy_rejections = 0

    foods = []
    for territory in ('A', 'B'):
        for index in range(per_territory):
            occupied = {food[1] for food in foods if food[2] == territory}
            while True:
                attempts += 1
                x = entropy.choose_index(WORLD_SIZE)
                local_y = entropy.choose_index(TERRITORY_HEIGHT)
                y = local_y if territory == 'A' else local_y + TERRITORY_HEIGHT
                if (x, y) not in occupied:
                    break
                occupancy_rejections += 1
            foods.append((CLASSES[index % 3], (x, y), territory))

    agents = []
    taken = []
    for number in range(1, 13):
        territory = 'A' if number <= 6 else 'B'
        while True:
            attempts += 1
            x = entropy.choose_index(WORLD_SIZE)
            local_y = entropy.choose_index(TERRITORY_HEIGHT)
            y = local_y if territory == 'A' else local_y + TERRITORY_HEIGHT
            if (x, y) not in taken:
                break
            occupancy_rejections += 1
        taken.append((x, y))
        agents.append(((x, y), territory))

    return {
        'foods': foods, 'agents': agents, 'draws': entropy.draws, 'state': entropy.state,
        'attempts': attempts, 'occupancy_rejections': occupancy_rejections,
        'per_territory': per_territory,
    }


def read_excerpt(text):
    foods, agents = [], []
    for line in text.splitlines():
        food = FOOD_LINE.search(line)
        if food:
            foods.append((food.group(2), (int(food.group(3)), int(food.group(4))), food.group(5)))
            continue
        agent = AGENT_LINE.search(line)
        if agent:
            agents.append(((int(agent.group(2)), int(agent.group(3))), agent.group(4)))
    return foods, agents


def initialization_prefix(text):
    """Every record at tick 0, which is the whole of what `Simulation::new` emits."""
    kept = []
    for line in text.split('\n'):
        match = RECORD.match(line)
        if not match:
            continue
        if match.group(1) != '0':
            break
        kept.append(line)
    return kept


def function_bodies(path):
    """Each non-test function in the crate's simulation module, with its body text.

    The scan stops at `#[cfg(test)]`: the test module legitimately drives the stream, and including
    it would make the check below pass or fail on test code rather than on the engine.
    """
    lines = path.read_text(encoding='utf-8').splitlines()
    boundary = next(index for index, text in enumerate(lines, start=1) if text == '#[cfg(test)]')
    bodies = {}
    context = None
    for number, text in enumerate(lines[:boundary - 1], start=1):
        impl = IMPL.match(text)
        if impl:
            context = impl.group(1)
        elif text == '}':
            context = None
        match = FUNCTION.match(text)
        if not match:
            continue
        indent = len(text) - len(text.lstrip())
        closer = ' ' * indent + '}'
        end = boundary - 1
        for later in range(number + 1, boundary):
            if lines[later - 1] == closer:
                end = later
                break
        label = f'{context}::{match.group(2)}' if context and match.group(1) else match.group(2)
        bodies[label] = (number, end, lines[number - 1:end])
    return bodies, boundary


CALL = re.compile(r'\b([a-z_][a-z0-9_]*)\s*\(')


def call_closure(bodies, roots, barred=()):
    """Every module function reachable from `roots`, following calls by bare name.

    Bare-name matching over-approximates: `foo(` resolves to every `Type::foo` in the module, so
    the closure can contain a function the root cannot actually reach. That is the safe direction
    for the two negative claims made with it -- a closure that is too large scans code it did not
    have to, and can only turn a true "nothing here does X" into a false alarm, never the reverse.
    The regex admits `snake_case` callees only, so enum variants and type constructors are skipped.

    `barred` names labels that must not be entered as callees. It exists for exactly one case: the
    four `DecisionSource::decide` implementations share a bare name, so without it a closure rooted
    at any one source silently swallows the other three and every claim made per source becomes the
    same claim. A source never calls another source's `decide` -- the engine's dispatcher does -- so
    barring re-entry removes an artefact of the name matching rather than a real edge.
    """
    barred = set(barred) - set(roots)
    by_name = {}
    for label in bodies:
        if label not in barred:
            by_name.setdefault(label.split('::')[-1], []).append(label)
    seen = set()
    queue = list(roots)
    while queue:
        label = queue.pop()
        if label in seen or label not in bodies:
            continue
        seen.add(label)
        for text in bodies[label][2]:
            for name in CALL.findall(text):
                queue.extend(candidate for candidate in by_name.get(name, ())
                             if candidate not in seen)
    return seen


def rule(title):
    print()
    print(title)
    print('-' * len(title))


def main():
    pre_dir, post_dir = sys.argv[1:3]
    here = Path(__file__).resolve().parent
    source = Path('mokiterions-core/src/simulation.rs')
    if not source.exists():
        source = here.parents[4] / 'mokiterions-core' / 'src' / 'simulation.rs'
    names = sorted(entry[:-4] for entry in os.listdir(post_dir) if entry.endswith('.txt'))

    title = 'WO-MOK-017: where the shared entropy stream stands, either side of the correction'
    print(title)
    print('=' * len(title))
    print()
    print('Work order   WO-MOK-017 (the resource composition drift)')
    print('Retains      "the shared stream\'s recorded state either side of every resolution kind, and')
    print('             the total draw count per run per source at matched seeds"')
    print('Pre-change   pre/COMMIT.txt      Candidate  post/COMMIT.txt')
    print('Reader       analysis/entropy.py, reusing WO-MOK-016/analysis/entropy.py\'s replay')
    print('Date         2026-08-21')
    print()
    print('The second half of that retention item is not derivable at this candidate. Section 5 states')
    print('why, in full, and records it as a residual gap rather than substituting a proxy for it. The')
    print('first half is measured here, and so is the claim this work order actually depends on: that the')
    print('corrected condition consumes no entropy.')

    # ------------------------------------------------------------------ 1
    rule('1. The correction consumes no entropy')
    print('If `fits_within` drew from the shared stream, every divergence in `post/divergence.txt` would')
    print('be confounded -- "the condition changed an answer" and "the condition moved the stream" would')
    print('be indistinguishable, and route R2 there would be unfalsifiable rather than merely permissive.')
    print('So the corrected function and every function that reaches it are read for stream contact.')
    print()
    bodies, boundary = function_bodies(source)
    print(f'    {"function":<48}{"lines":>13}  reaches the stream')
    for label in CORRECTION_PATH:
        if not check(label in bodies, f'{label} is absent from the engine module'):
            print(f'    {label:<48}{"ABSENT":>13}  -')
            continue
        start, end, body = bodies[label]
        hits = [text.strip() for text in body if STREAM_CALL.search(text)]
        check(not hits, f'{label} reaches the shared stream: {hits}')
        print(f'    {label:<48}{f"{start}-{end}":>13}  {"NO" if not hits else "YES -- " + hits[0]}')
    print()
    print('Read at face value that is only seven bodies, and a draw could sit one call deeper. So the')
    print('same question is asked of everything those seven can reach, transitively:')
    print()
    reachable = call_closure(bodies, [label for label in CORRECTION_PATH if label in bodies])
    drawing = sorted(label for label in reachable
                     if any(STREAM_CALL.search(text) for text in bodies[label][2]))
    check(not drawing, f'the correction path transitively reaches the stream: {drawing}')
    print(f'    functions reachable from the seven, including themselves    {len(reachable)}')
    print(f'    of those, functions that reach the shared stream            {len(drawing)}')
    for label in drawing:
        print(f'      {label} lines {bodies[label][0]}-{bodies[label][1]}')
    if not drawing:
        print('    **Nothing the corrected condition can reach touches the shared stream.** So a')
        print('    divergence recorded in `post/divergence.txt` cannot be a stream displacement created')
        print('    inside the condition; it is a changed answer, or a consequence of one.')
    print()
    print('    The scope of that negative claim, stated rather than left to be assumed: the scan covers')
    print(f'    the engine module ahead of `#[cfg(test)]` at line {boundary}, matching `next_u64`,')
    print('    `choose_index`, `entropy`, `SplitMix64` and `DecisionEntropy` as whole words, and the')
    print('    call graph is followed by bare callee name -- which over-approximates, so the closure')
    print('    scans more than the path can truly reach rather than less. What it cannot see is a draw')
    print('    reached through a trait object or a closure stored elsewhere, which is why the suite\'s')
    print('    `no_resolution_and_no_targeted_move_touches_the_shared_stream` remains the primary')
    print('    guarantee and this is a check on the correction specifically.')

    # ------------------------------------------------------------------ 2
    rule('2. The stream\'s state at the end of initialization, at both commits')
    print('`SplitMix64::new(seed)` sets `state = seed`, and `next_u64` advances the state by exactly one')
    print('fixed increment before mixing. So the absolute state after `k` values is')
    print('`seed + k*GAMMA (mod 2^64)`, and a draw count converts to a state without inverting the mixing')
    print('function. `choose_index` costs exactly one value for both bounds in play -- 128 for `x` and 64')
    print('for the territory-local `y` -- because each divides 2^64, so the rejection threshold')
    print('`bound.wrapping_neg() % bound` is zero and no value is ever rejected for modulo bias. That is')
    print('asserted rather than assumed, because it is what makes "two values per coordinate attempt"')
    print('exact.')
    print()
    for bound in (WORLD_SIZE, TERRITORY_HEIGHT):
        threshold = ((-bound) & MASK) % bound
        check(threshold == 0, f'choose_index({bound}) can reject for modulo bias')
        print(f'    choose_index({bound:>3}) rejection threshold                {threshold}')
    print()
    print('    dens   seed   resources    draws  recorded  occupancy      the state the stream stands at')
    print('                per territory                    rejections     when initialization returns')
    replays = {}
    for density, hundredths in DENSITIES:
        for index, seed in enumerate(SEEDS):
            replayed = initialize(seed, hundredths)
            replays[(density, seed)] = replayed
            recorded = RECORDED_DRAWS[density][index]
            check(replayed['draws'] == recorded,
                  f'replayed draws {replayed["draws"]} != recorded {recorded} '
                  f'at seed {seed} density {density}')
            state = (seed + replayed['draws'] * GAMMA) & MASK
            check(state == replayed['state'],
                  f'the closed form disagrees with the replayed state at seed {seed}')
            print(f'    {density:>4} {seed:>6} {replayed["per_territory"]:>11} '
                  f'{replayed["draws"]:>8} {recorded:>9} {replayed["occupancy_rejections"]:>11}'
                  f'      0x{state:016X}')
    print()
    print('    The `draws` column is replayed and the `recorded` column is `INITIALIZATION_DRAWS`')
    print('    transcribed from the suite. They are compared, never substituted: the replay is not told')
    print('    the answer, and it reproduces all fifteen. The state column is the closed form')
    print('    `seed + k*GAMMA mod 2^64`, cross-checked against the state the replay itself carries.')
    print()
    print('    This table is a property of `Simulation::new`, which the correction does not touch, so it')
    print('    is the same at both commits. Section 3 measures that rather than asserting it.')

    # ------------------------------------------------------------------ 3
    rule('3. The replay checked against the captures, and the two commits checked against each other')
    print('A reimplementation that agreed with nothing would only measure itself. Every reconstructed')
    print('position is compared against the engine\'s own initialization records, cell for cell, at both')
    print('commits -- so a placement change would break this reader here even though the replay came from')
    print('the previous work order.')
    print()
    mismatched = []
    prefix_differs = []
    prefix_lines = 0
    for name in names:
        seed = int(re.search(r'^seed(\d+)', name).group(1))
        density = re.search(r'-d([0-9.]+)-', name).group(1)
        pre_text = io.open(os.path.join(pre_dir, name + '.txt'),
                           encoding='utf-8', newline='').read()
        post_text = io.open(os.path.join(post_dir, name + '.txt'),
                            encoding='utf-8', newline='').read()
        pre_prefix = initialization_prefix(pre_text)
        post_prefix = initialization_prefix(post_text)
        prefix_lines += len(post_prefix)
        if pre_prefix != post_prefix:
            prefix_differs.append(name)
        replayed = replays[(density, seed)]
        for label, text in (('pre-change', pre_text), ('candidate', post_text)):
            foods, agents = read_excerpt(text)
            expected_foods = [(item[0], item[1], item[2]) for item in replayed['foods']]
            if foods != expected_foods or agents != replayed['agents']:
                mismatched.append((name, label))
    check(not mismatched, f'{len(mismatched)} capture/replay placement mismatches')
    check(not prefix_differs, f'{len(prefix_differs)} cells whose initialization prefix moved')
    print(f'    cells whose placement was replayed and compared            {len(names)}')
    print(f'    comparisons made, counting both commits                    {len(names) * 2}')
    print(f'    placements the replay got wrong                            {len(mismatched)}')
    print(f'    tick-0 records compared across the two commits             {prefix_lines:,}')
    print(f'    cells whose initialization prefix differs                  {len(prefix_differs)}')
    print()
    if mismatched:
        print('    THE REPLAY DISAGREES WITH THE CAPTURE. Nothing in section 2 can be relied on:')
        for name, label in mismatched[:10]:
            print(f'      {name} ({label})')
    elif prefix_differs:
        print('    THE INITIALIZATION PREFIX MOVED in the cells below, which the correction cannot do:')
        for name in prefix_differs:
            print(f'      {name}')
    else:
        print('    **Every food class, food position, food territory and Mokiterion position in all')
        print(f'    {len(names)} cells matches the replay at both commits, and every cell\'s tick-0 records')
        print('    are identical across the two commits.** So the stream enters tick 1 at the same state')
        print('    in all 120 runs, and the correction demonstrably begins to act after initialization')
        print('    rather than within it.')
    print()
    print('    The excerpts also settle something the suite states but no single capture asserts: that')
    print('    the decision source and the trace setting move no placement draw. All eight cells sharing')
    print('    a `(seed, density)` pair replay from one reconstruction, and all eight match it.')

    # ------------------------------------------------------------------ 4
    rule('4. Every resolution kind, counted either side of the correction')
    print('`attack_resolved`, `threat_resolved` and `surrender_resolved` are the resolution kinds, and')
    print('rule 26 emits them whether or not actions are traced. Two thirds of the matrix cannot contain')
    print('one at all, and that is worth establishing before the table rather than leaving a reader to')
    print('read three pages of zeros as a bug: a resolution is the settlement of a contested action, and')
    print('only the social source ever proposes one. So the resolution half of this retention item is')
    print('informative under `social` and vacuous everywhere else. The zeros below are proved rather than')
    print('merely observed:')
    print()
    # `Action::Attack { .. }` is a pattern and `Action::Attack { target }` is a proposal, and the
    # difference matters: `Simulation`'s dispatchers match on all three kinds and propose none. So
    # the scan is scoped to the four decision sources, which is where a proposal can originate --
    # a dispatcher can only ever see a contested action some source handed it.
    contest = re.compile(r'Action::(Attack|Threaten|Surrender)\s*\{(?!\s*\.\.\s*\})')
    print('    Each source is read together with everything it can reach, because `individual` decides')
    print('    in two delegating lines and a five-line body proves nothing on its own.')
    print()
    print(f'    {"decision source":<34}{"lines":>11}{"reaches":>9}  proposes a contested action')
    for source in ('Baseline', 'Reference', 'Individual', 'Social'):
        label = f'{source}DecisionSource::decide'
        if not check(label in bodies, f'{label} is absent from the engine module'):
            continue
        start, end, _ = bodies[label]
        closure = call_closure(bodies, [label], barred=DECIDERS)
        hits = sorted({contest.search(text).group(1)
                       for reached in closure for text in bodies[reached][2]
                       if contest.search(text)})
        expected = source == 'Social'
        check(bool(hits) == expected,
              f'{label} proposes {hits or "nothing"}, which contradicts rule 26')
        print(f'    {label:<34}{f"{start}-{end}":>11}{len(closure):>9}  '
              f'{", ".join(hits) if hits else "none"}')
    print()
    print('    So `baseline`, `reference` and `individual` emit no resolution by construction, and their')
    print('    rows below are a consequence of the engine\'s shape rather than of these five seeds.')
    print()
    print('Under `social` the counts are expected to move, because the correction changes where')
    print('Mokiterions walk and so changes who meets whom. Under `baseline` they are expected not to,')
    print('and that is measured rather than inherited: a `baseline` resolution count that moved would')
    print('contradict `post/byte-identity.txt`, and a reader is entitled to see the two files agree.')
    print()
    counts = {}
    for name in names:
        for label, directory in (('pre', pre_dir), ('post', post_dir)):
            found = Counter()
            with io.open(os.path.join(directory, name + '.txt'),
                         encoding='utf-8', newline='') as handle:
                for line in handle:
                    match = RECORD.match(line)
                    if match and match.group(3) in RESOLUTIONS:
                        found[match.group(3)] += 1
            counts[(name, label)] = found
    print('    Totals over the ten cells of each source and density -- five seeds, traced and untraced.')
    print()
    print('    source      dens   attack_resolved   threat_resolved   surrender_resolved')
    print('                        pre -> candidate  pre -> candidate  pre -> candidate')
    for source in SOURCES:
        for density, _ in DENSITIES:
            cells = [name for name in names
                     if f'-{source}-d{density}-' in name]
            row = []
            for kind in RESOLUTIONS:
                before = sum(counts[(name, 'pre')][kind] for name in cells)
                after = sum(counts[(name, 'post')][kind] for name in cells)
                row.append(f'{before:>6} ->{after:>6}')
            print(f'    {source:<11}{density:>5}  ' + '  '.join(row))
    print()
    print('    Per cell under `social`, untraced, which is where the item has content. The traced half')
    print('    of each pair is identical and is checked below rather than reprinted.')
    print()
    print('    dens   seed   attack_resolved   threat_resolved   surrender_resolved')
    print('                   pre -> candidate  pre -> candidate  pre -> candidate')
    for density, _ in DENSITIES:
        for seed in SEEDS:
            match = [name for name in names
                     if f'-social-d{density}-' in name and name.startswith(f'seed{seed}-')
                     and '-traceoff' in name]
            if not check(len(match) == 1, f'no untraced social cell at seed {seed} density {density}'):
                continue
            name = match[0]
            row = [f'{counts[(name, "pre")][kind]:>6} ->{counts[(name, "post")][kind]:>6}'
                   for kind in RESOLUTIONS]
            print(f'    {density:>4} {seed:>6}  ' + '  '.join(row))
    print()
    baseline_moved = [name for name in names if '-baseline-' in name
                      and counts[(name, 'pre')] != counts[(name, 'post')]]
    check(not baseline_moved, f'{len(baseline_moved)} baseline cells changed a resolution count')
    print(f'    baseline cells whose resolution counts moved               {len(baseline_moved)}')
    if baseline_moved:
        for name in baseline_moved:
            print(f'      {name}')
    else:
        print('    Zero, which is what `post/byte-identity.txt` requires -- though on `baseline` this is')
        print('    a weak check twice over, since the construction proof above already forbids a')
        print('    resolution there. It is printed because a non-zero value would mean one of the two')
        print('    files is wrong, and finding that out from a cheap check is worth four lines.')
    print()
    traced_disagreement = []
    pairs = 0
    telling = 0
    for name in names:
        if '-traceon' not in name:
            continue
        sibling = name.replace('-traceon', '-traceoff')
        if sibling not in names:
            continue
        pairs += 1
        if any(counts[(name, label)] for label in ('pre', 'post')):
            telling += 1
        for label in ('pre', 'post'):
            if counts[(name, label)] != counts[(sibling, label)]:
                traced_disagreement.append((name, label))
    print(f'    traced/untraced pairs in the matrix                         {pairs}')
    print(f'    of those, pairs carrying a resolution at either commit      {telling}')
    print(f'    pairs whose two halves disagree on any resolution count,')
    print(f'    at either commit                                           {len(traced_disagreement)}')
    if traced_disagreement:
        print('    They are expected to agree only where the two halves have not diverged behaviorally.')
        print('    Tracing is instrumentation, but a traced run and an untraced run of the same')
        print('    `(seed, source, density)` are the same run, so a disagreement here would mean tracing')
        print('    itself moved a draw. Listed in full:')
        for name, label in traced_disagreement:
            print(f'      {name} ({label}): {dict(counts[(name, label)])} against '
                  f'{dict(counts[(name.replace("-traceon", "-traceoff"), label)])}')
        check(False, f'{len(traced_disagreement)} pairs where tracing moved a resolution count')
    else:
        print('    Zero, at both commits. The count that carries the claim is the middle one: on every')
        print('    pair that has a resolution to disagree about, the traced and untraced halves agree')
        print('    exactly. So tracing moves no draw, and the accounting above is a property of the run')
        print('    rather than of the instrumentation. The remaining pairs agree vacuously and are not')
        print('    evidence of anything.')

    # ------------------------------------------------------------------ 5
    rule('5. The residual gap: the total draw count per run')
    print('The second half of the retention item asks for "the total draw count per run per source at')
    print('matched seeds". It is not derivable at this candidate, and this section is the record of that')
    print('rather than a proxy dressed as an answer.')
    print()
    print('    what would be needed                      why it is unavailable here')
    print('    a counter on the released surface         WO-MOK-017 Out of scope: "any new attribute,')
    print('                                             event, interface item or instrumentation".')
    print('                                             `--help` lists no such option and the rule 18')
    print('                                             summary carries no such field.')
    print('    a full replay outside the crate          a regeneration spends an unknown number of')
    print('                                             draws, because occupancy rejections are')
    print('                                             invisible -- only the accepted position is')
    print('                                             printed. Recovering them means replaying every')
    print('                                             draw-consuming path, which is a second')
    print('                                             implementation of the engine.')
    print('    a golden-value test in the suite         no requirement states a draw total, so such a')
    print('                                             test would pin an arbitrary measured number and')
    print('                                             fail on any later legitimate behavior change.')
    print('                                             SPEC-MOK-002 rule 12 is the neighboring')
    print('                                             principle: a test\'s assertion is its meaning.')
    print()
    print('What stands in its place, all of it measured above rather than argued:')
    print('  * the initialization draw total and absolute stream state, at all fifteen declared pairs and')
    print('    identical at both commits (section 2), validated against the suite\'s own recorded counts;')
    print('  * the initialization prefix byte-identical in all 120 cells, so every run enters tick 1 at')
    print('    the same stream state at both commits (section 3);')
    print('  * every resolution kind counted per cell at both commits, with `baseline` unmoved and')
    print('    tracing shown to move no draw (section 4);')
    print('  * the correction itself shown to consume no entropy (section 1), which is the premise that')
    print('    makes `post/divergence.txt`\'s attribution mean anything.')
    print()
    print('What remains genuinely unmeasured is the absolute number of values each thousand-tick run')
    print('draws. Closing it needs a decision the technical owner has not been asked for -- whether a')
    print('draw counter belongs on the released surface, or whether a work order may carry a replay of')
    print('the whole engine as evidence. `WO-MOK-016` left a comparable clause unwritable and recorded it;')
    print('this is the same discipline applied to a narrower gap, and it is raised in')
    print('`manual-assessment.md` so that accepting it is an act rather than an omission.')

    # ------------------------------------------------------------------ result
    print()
    if failures:
        print(f'RESULT: FAIL -- {len(failures)} check(s) failed:')
        for failure in failures:
            print(f'  {failure}')
    else:
        print('RESULT: PASS on every derivable check -- the correction consumes no entropy, the '
              'initialization')
        print('replay reproduces all fifteen recorded draw counts and all 120 captured placements at both')
        print('commits, every initialization prefix is unchanged, and no `baseline` resolution count moved.')
        print('The per-run draw total is recorded as a residual gap in section 5, with its reason.')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
