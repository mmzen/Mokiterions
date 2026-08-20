"""VER-MOK-012 oracle 4: whole runs under `social`, replayed from the released event stream.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-012/analysis/runs.py <capture-dir>

`<capture-dir>` is one produced by `../capture-social.sh`: 30 `<cell>.txt` streams and 30 `<cell>.exit`
codes. Everything this script prints is derived from those bytes and from nothing else. It takes about
a minute over the 30 cells and writes `runs.md`'s and `branches.md`'s figures to stdout.

WHY A REPLAY RATHER THAN A COUNT
--------------------------------

`VER-MOK-012` retains "the branch distribution under `social` per seed -- how often each of
`REQ-MOK-048`'s five branches fired", and the amendment of 2026-08-20 made those five six. Five of the
six are readable off a trace line directly: the answer branch by a non-empty `suffered` field, branch 2
by `eat` or `sleep`, branch 4 by `attack` or `threaten`, branch 5 by `approach` or `avoid`. **Branch 3
and branch 6 both render as `move:<direction>` and cannot be told apart by counting verbs** -- and they
are exactly the pair that matters for entropy, because branch 6 is the only branch that draws.

So this script reconstructs the part of rule 3's observation the two branches turn on, and it does so
from the stream, which carries every input needed:

  positions      `agent_initialized` gives the start, and every `action_trace` gives the position after
                 that agent's action. Nothing else moves a Mokiterion, so replaying the traces in
                 stream order gives every agent's exact position at every decision moment -- including
                 the moves earlier-acting Mokiterions already made this tick, which is what rule 13's
                 ascending pass means and what the acting Mokiterion actually saw.
  food           `food_initialized` and `food_regenerated` add, `food_consumed` removes, and rule 15
                 has no other mutation. Replayed in stream order, the map is exact at every moment.
  satiety        the trace's own field. Rule 12's decay comes after the trace, and no move changes
                 satiety, so on a `move` line the traced satiety *is* the satiety the source read.
  tolerance      `agent_initialized`, once per Mokiterion, per `SPEC-MOK-001`'s *Behavioral trait*.
  fear           the trace's field, which rule 7 fixes as the pre-update value: the value the source
                 read at this opportunity.
  aliveness      `agent_died`, applied in stream order, so a Mokiterion struck down earlier in the tick
                 is not perceived by a later actor. One case needs care and the script states it where
                 it is handled: a target killed by the very action being traced dies *before* that
                 action's trace line, because rule 22 resolves damage inside the striker's turn and the
                 trace is emitted after the action applies. Such a target was alive and in contact when
                 the decision was taken, and is counted so. A death by rule 13's decay is the opposite
                 case -- it follows its own holder's trace line and is genuinely gone before the next
                 decision -- and the two are told apart by which action's `attack_resolved` reported the
                 kill, not by their `agent_died` records, which are identical by that rule's design.

The reconstruction is then *checked against the engine* rather than trusted, and the checks are what
make the classification evidence instead of an assertion. They are printed under CHECKS and every one
of them must read zero:

  1. Branch 3 predicted, direction disagrees. Where the reconstruction says a tolerated resource is
     perceived with an admissible step, rule 26 branch 3 must propose exactly that step. Rule 5 case 3's
     axis rule -- horizontal while the direction has an easterly or westerly component, otherwise
     vertical, and the other axis when the preferred one is invalid -- is reimplemented here and
     compared against the engine's own choice on every such decision.
  2. Branch 6 classified with company perceived. Branch 6 sits behind branches 4 and 5, so a search
     step is only reachable when no living Mokiterion is within the perception radius. This is the
     check that a branch-3 decision cannot hide in the branch-6 count: if the food predicate were too
     strict, the misclassified decisions would be the ones with company in sight.
  3. Branch 4 or 5 with a branch-3 candidate. Branch 3 outranks both after the amendment, so a social
     verb proves the reconstruction found no tolerated resource. This is the check in the opposite
     direction: a food predicate that were too lax would fire here.
  4. Branch 4 without contact, branch 5 with contact, and the two thresholds. Rule 20's contact
     relation and rule 26's three constants, re-derived from reconstructed positions and the traced
     `fear` and compared with the verb the engine chose.

Checks 1 and 3 bound the food predicate from both sides and check 2 bounds the branch-3/branch-6 split
where it is most load-bearing, so a reconstruction that passed all of them while misclassifying would
have to be wrong in a way that is invisible to the rule it is reconstructing.

WHAT AN ENCOUNTER IS HERE
-------------------------

`VER-MOK-012` asks for encounters per seed and strikes per encounter, and neither is defined by the
specification, so this script fixes a definition and states it: an **encounter** is a maximal run of
consecutive ticks during which one unordered pair of living Mokiterions is in contact -- rule 20's
Chebyshev distance of `1` -- where a pair counts as in contact in a tick if it was in contact at any
moment within that tick. The "any moment" is deliberate: positions move mid-tick, and a pair that was
in contact when the strike landed and apart by the end of the tick is one encounter with one strike in
it, not zero encounters with one strike. Every resolution is attributed to the episode holding its tick
and its pair, and the script asserts that none is left over.
"""

import io
import os
import re
import sys

# The other scripts in this packet reconfigure the encoding alone. This one also fixes the line
# ending, because its stdout is what `post/runs.txt` holds: a reader who redirects it on Windows
# reproduces the retained bytes rather than the same text with 129 extra carriage returns.
sys.stdout.reconfigure(encoding='utf-8', newline='\n')

WORLD_SIZE = 128
PERCEPTION_RADIUS = 16
CONTACT_RADIUS = 1
ATTRIBUTE_MAX = 100
SURRENDER_FEAR_THRESHOLD = 60
RETREAT_FEAR_THRESHOLD = 30
ENGAGEMENT_FEAR_THRESHOLD = 95
STRIKE_BASE_DAMAGE = 10
STRIKE_ENERGY_COST = 5
THREAT_FEAR_INCREASE = 30

# SPEC-MOK-001's food table: satiety restoration and the calorie rank the tie-breaks read.
RESTORATION = {'low': 15, 'medium': 30, 'high': 50}
RANK = {'low': 0, 'medium': 1, 'high': 2}

NORTH, EAST, SOUTH, WEST = 'north', 'east', 'south', 'west'

VERBS = ['wait', 'sleep', 'eat', 'move', 'attack', 'threaten', 'fight', 'retreat', 'surrender',
         'approach', 'avoid']
TARGETED = ['attack', 'threaten', 'fight', 'retreat', 'surrender', 'approach', 'avoid']

LINE = re.compile(r'^tick=(\d+) subject=(\S+) event=(\S+) result=(.*)$')
# The trace's trailing fields are fixed in order and the action detail before them may hold commas
# and colons of its own, so the tail is matched from the right rather than split from the left.
TRACE_TAIL = re.compile(
    r'^proposal:(?P<verb>[a-z]+)(?::(?P<argument>[A-Za-z0-9_]+))?'
    r'(?:,target:(?P<target>M\d\d))?'
    r',status:(?P<status>accepted|rejected)'
    r',detail:(?P<detail>.*)'
    r',position:(?P<x>\d+):(?P<y>\d+),territory:(?P<territory>[AB])'
    r',health:(?P<health>\d+),satiety:(?P<satiety>\d+),energy:(?P<energy>\d+),fear:(?P<fear>\d+)'
    r'(?:,suffered:(?P<suffered>\S+))?$'
)


def fields(text):
    """`a:1,b:2` as a dict, keeping only the first colon as the separator."""
    out = {}
    for item in text.split(','):
        key, _, value = item.partition(':')
        out[key] = value
    return out


def moved(x, y, direction):
    """`Coordinate::moved`: the cell one step away, or None where the step leaves the world."""
    if direction == NORTH:
        return (x, y - 1) if y > 0 else None
    if direction == EAST:
        return (x + 1, y) if x < WORLD_SIZE - 1 else None
    if direction == SOUTH:
        return (x, y + 1) if y < WORLD_SIZE - 1 else None
    return (x - 1, y) if x > 0 else None


def chebyshev(left, right):
    return max(abs(left[0] - right[0]), abs(left[1] - right[1]))


def fits_within_tolerance(satiety, food_class, tolerance):
    """Rule 19's tolerant test verbatim, with the division truncating toward zero."""
    restored = RESTORATION[food_class]
    resulting = satiety + restored
    if resulting <= ATTRIBUTE_MAX:
        return True
    return resulting - ATTRIBUTE_MAX <= tolerance * restored // 100


def best_tolerated_distant_food(position, satiety, tolerance, food):
    """Rule 19 case 3's target: nearest perceived and tolerated at distance >= 1, then highest
    calorie class, then lowest identifier."""
    best = None
    best_key = None
    for identifier, (food_class, cell) in food.items():
        distance = chebyshev(position, cell)
        if distance < 1 or distance > PERCEPTION_RADIUS:
            continue
        if not fits_within_tolerance(satiety, food_class, tolerance):
            continue
        key = (distance, -RANK[food_class], identifier)
        if best_key is None or key < best_key:
            best, best_key = (identifier, food_class, cell), key
    return best


def axis_step(position, target):
    """Rule 5 case 3's axis rule, which rule 19 case 3 and rule 21's three moves all reuse."""
    x, y = position
    offset_x = target[0] - x
    offset_y = target[1] - y
    horizontal = EAST if offset_x > 0 else WEST if offset_x < 0 else None
    vertical = NORTH if offset_y < 0 else SOUTH if offset_y > 0 else None
    preferred = horizontal or vertical
    alternate = vertical or horizontal
    for candidate in (preferred, alternate):
        if candidate is not None and moved(x, y, candidate) is not None:
            return candidate
    return None


class Replay:
    """One cell, replayed. Everything below is read off the stream in order."""

    def __init__(self, text):
        self.events = {}
        self.summary = None
        self.exit_reason = None
        self.ticks = 0
        self.tolerance = {}
        self.position = {}
        self.alive = set()
        self.food = {}
        self.traced = False

        self.proposed = dict.fromkeys(VERBS, 0)
        self.applied = dict.fromkeys(VERBS, 0)
        self.rejections = {}            # (verb, ground) -> count
        self.branches = dict.fromkeys(range(1, 7), 0)
        self.answers = dict.fromkeys(['fight', 'retreat', 'surrender'], 0)
        self.answer_target_dead = 0
        self.suffered_entries = []      # how many attacks stood in the record at each answer
        self.checks = dict.fromkeys(
            ['branch 3 direction disagrees',
             'branch 6 with company perceived',
             'branch 4 or 5 with a branch 3 candidate',
             'branch 4 without contact',
             'branch 5 in contact',
             'branch 1 verb against the two thresholds',
             'branch 4 or 5 verb against the engagement threshold',
             'a wait was proposed',
             'move direction absent',
             'rule 22 damage disagrees',
             'rule 22 transition disagrees',
             'rule 23 increase disagrees',
             'rule 24 forfeit arithmetic disagrees'], 0)

        self.strikes = []               # (tick, striker, target, damage, died)
        self.threats = []               # (tick, subject, target, increase, fear_before, fear_after)
        self.surrenders = []            # (tick, subject, recipient, transferred, discarded, before)
        self.deaths = []                # (tick, identifier, cause)
        self.consumed = dict.fromkeys(RESTORATION, 0)
        self.contact_ticks = {}         # pair -> set of ticks
        self.current_tick = 0
        # Targets the action now being traced has just killed. Rule 22 resolves damage inside the
        # striker's turn and `emit_action_trace` runs after the action applies, so the stream order is
        # `attack_resolved`, the target's `agent_died`, and only then the striker's `action_trace`. By
        # the time the trace line arrives the target is out of `alive`, though it was alive and in
        # contact when the decision was taken. Held here and added back for that one decision, then
        # cleared. It cannot be conflated with a death by rule 13's decay, which follows its own
        # holder's trace line and so is genuinely gone before the next decision.
        self.killed_by_this_action = set()

        for line in text.split('\n'):
            match = LINE.match(line)
            if match:
                self.event(int(match.group(1)), match.group(2), match.group(3), match.group(4))
            elif line.startswith('summary '):
                # Rule 18's line is `k=v` pairs, not the `k:v` of a result field.
                self.summary = dict(item.split('=', 1) for item in line.split()[1:])

    # -- the replay ------------------------------------------------------------------------

    def event(self, tick, subject, kind, result):
        self.events[kind] = self.events.get(kind, 0) + 1
        if tick != self.current_tick:
            self.current_tick = tick
            self.mark_contacts()
        self.ticks = max(self.ticks, tick)
        row = None if kind == 'action_trace' else fields(result)

        if kind == 'agent_initialized':
            # `position:x:y` holds a colon of its own, so it is read from the raw text and not
            # from the field split.
            x, y = re.search(r'position:(\d+):(\d+)', result).groups()
            self.position[subject] = (int(x), int(y))
            self.tolerance[subject] = int(row['waste_tolerance'])
            self.alive.add(subject)
        elif kind in ('food_initialized', 'food_regenerated'):
            x, y = re.search(r'position:(\d+):(\d+)', result).groups()
            self.food[row['food'] if 'food' in row else subject] = (row['class'], (int(x), int(y)))
        elif kind == 'food_consumed':
            self.food.pop(row['food'], None)
            self.consumed[row['class']] += 1
        elif kind == 'attack_resolved':
            health_before, health_after = (int(part) for part in
                                           row['target_health'].split('->'))
            energy_before, energy_after = (int(part) for part in
                                           row['striker_energy'].split('->'))
            damage = int(row['damage'])
            died = row['target_died'] == 'yes'
            self.strikes.append((tick, subject, row['target'], damage, died,
                                 energy_before, health_before, health_after, energy_after))
            # Rule 22's transitions, from the one line that carries both parties'. The damage
            # itself needs the striker's health and so is checked at its trace line below.
            if (health_after != max(0, health_before - damage)
                    or energy_after != max(0, energy_before - STRIKE_ENERGY_COST)
                    or died != (health_after == 0)):
                self.checks['rule 22 transition disagrees'] += 1
            if died:
                self.killed_by_this_action.add(row['target'])
        elif kind == 'threat_resolved':
            before, after = (int(part) for part in row['target_fear'].split('->'))
            increase = int(row['increase'])
            self.threats.append((tick, subject, row['target'], increase, before, after))
            # Rule 23: a flat THREAT_FEAR_INCREASE, saturating at ATTRIBUTE_MAX.
            if (increase != min(THREAT_FEAR_INCREASE, ATTRIBUTE_MAX - before)
                    or after != before + increase):
                self.checks['rule 23 increase disagrees'] += 1
        elif kind == 'surrender_resolved':
            before, after = (int(part) for part in row['subject_satiety'].split('->'))
            recipient_before, recipient_after = (int(part) for part in
                                                 row['recipient_satiety'].split('->'))
            transferred, discarded = int(row['transferred']), int(row['discarded'])
            self.surrenders.append((tick, subject, row['recipient'], transferred, discarded,
                                    before, recipient_before, recipient_after))
            # Rule 24: forfeit half, truncating; give what the recipient has room for; discard
            # the rest. Every term is on this one line, so the arithmetic closes without
            # reference to anything else in the stream.
            forfeit = before // 2
            expected = min(forfeit, ATTRIBUTE_MAX - recipient_before)
            if (after != before - forfeit
                    or transferred != expected
                    or discarded != forfeit - transferred
                    or recipient_after != recipient_before + transferred):
                self.checks['rule 24 forfeit arithmetic disagrees'] += 1
        elif kind == 'agent_died':
            struck = any(strike[0] == tick and strike[2] == subject and strike[4]
                         for strike in self.strikes)
            self.deaths.append((tick, subject, 'combat' if struck else 'attrition'))
            self.alive.discard(subject)
            self.mark_contacts()
        elif kind == 'action_trace':
            self.traced = True
            self.trace(subject, result)
            self.mark_contacts()
        elif kind == 'simulation_ended':
            self.exit_reason = row['reason']

    def mark_contacts(self):
        """Every pair in contact at this moment, recorded against the current tick."""
        living = sorted(self.alive)
        for index, left in enumerate(living):
            for right in living[index + 1:]:
                if chebyshev(self.position[left], self.position[right]) <= CONTACT_RADIUS:
                    self.contact_ticks.setdefault((left, right), set()).add(self.current_tick)

    def trace(self, subject, result):
        match = TRACE_TAIL.match(result)
        if not match:
            raise SystemExit(f'unparsed trace line: result={result}')
        verb = match.group('verb')
        accepted = match.group('status') == 'accepted'
        satiety = int(match.group('satiety'))
        fear = int(match.group('fear'))
        suffered = match.group('suffered')
        position = self.position[subject]          # before this action; the decision-time position
        self.position[subject] = (int(match.group('x')), int(match.group('y')))

        self.proposed[verb] += 1
        if accepted:
            self.applied[verb] += 1
        else:
            ground = match.group('detail')
            self.rejections[(verb, ground)] = self.rejections.get((verb, ground), 0) + 1

        if verb == 'wait':
            self.checks['a wait was proposed'] += 1

        # Rule 22's damage is `10 + (striker.energy + striker.health) / 10` from the striker's own
        # condition at the moment of resolution. `attack_resolved` carries the energy before the
        # cost; the health is on this line, and it is the health at resolution because a strike
        # does not change the striker's health and rule 12's decay follows the trace.
        if verb in ('attack', 'fight') and accepted:
            tick, striker, _, damage, _, energy_before = self.strikes[-1][:6]
            health = int(match.group('health'))
            if tick != self.current_tick or striker != subject:
                raise SystemExit(f'no strike to match the {verb} of {subject} '
                                 f'at tick {self.current_tick}')
            if damage != STRIKE_BASE_DAMAGE + (energy_before + health) // 10:
                self.checks['rule 22 damage disagrees'] += 1

        decided_among = self.alive | self.killed_by_this_action
        self.killed_by_this_action = set()
        others = [(other, self.position[other]) for other in decided_among if other != subject]
        contact = [other for other, cell in others
                   if chebyshev(position, cell) <= CONTACT_RADIUS]
        perceived = [other for other, cell in others
                     if chebyshev(position, cell) <= PERCEPTION_RADIUS]
        candidate = best_tolerated_distant_food(position, satiety, self.tolerance[subject],
                                                self.food)
        seek = axis_step(position, candidate[2]) if candidate else None

        if suffered:
            self.branches[1] += 1
            self.answers[verb] = self.answers.get(verb, 0) + 1
            self.suffered_entries.append(len(suffered.split(';')))
            expected = ('surrender' if fear >= SURRENDER_FEAR_THRESHOLD
                        else 'retreat' if fear >= RETREAT_FEAR_THRESHOLD else 'fight')
            if verb != expected:
                self.checks['branch 1 verb against the two thresholds'] += 1
            if match.group('target') not in decided_among:
                self.answer_target_dead += 1
        elif verb in ('eat', 'sleep'):
            self.branches[2] += 1
        elif verb == 'move':
            if match.group('argument') is None:
                self.checks['move direction absent'] += 1
            if seek is not None:
                self.branches[3] += 1
                if seek != match.group('argument'):
                    self.checks['branch 3 direction disagrees'] += 1
            else:
                self.branches[6] += 1
                if perceived:
                    self.checks['branch 6 with company perceived'] += 1
        elif verb in ('attack', 'threaten', 'approach', 'avoid'):
            social = 4 if verb in ('attack', 'threaten') else 5
            self.branches[social] += 1
            if seek is not None:
                self.checks['branch 4 or 5 with a branch 3 candidate'] += 1
            if social == 4 and not contact:
                self.checks['branch 4 without contact'] += 1
            if social == 5 and contact:
                self.checks['branch 5 in contact'] += 1
            engaged = verb in ('attack', 'approach')
            if engaged != (fear < ENGAGEMENT_FEAR_THRESHOLD):
                self.checks['branch 4 or 5 verb against the engagement threshold'] += 1

    # -- what the tables read --------------------------------------------------------------

    def encounters(self):
        """Maximal runs of consecutive contact ticks, per pair, with what resolved inside them."""
        episodes = []
        for pair, ticks in self.contact_ticks.items():
            run = []
            for tick in sorted(ticks):
                if run and tick == run[-1] + 1:
                    run.append(tick)
                else:
                    if run:
                        episodes.append((pair, run[0], run[-1]))
                    run = [tick]
            if run:
                episodes.append((pair, run[0], run[-1]))
        return episodes

    def attribute(self, episodes):
        """Every resolution into the episode holding its tick and its pair. Nothing left over."""
        index = {}
        for number, (pair, first, last) in enumerate(episodes):
            for tick in range(first, last + 1):
                index[(pair, tick)] = number
        counts = [{'strikes': 0, 'threats': 0, 'surrenders': 0, 'deaths': 0}
                  for _ in episodes]
        orphans = 0
        for kind, rows in (('strikes', self.strikes), ('threats', self.threats),
                           ('surrenders', self.surrenders)):
            for row in rows:
                tick, one, two = row[0], row[1], row[2]
                pair = (one, two) if one < two else (two, one)
                number = index.get((pair, tick))
                if number is None:
                    orphans += 1
                    continue
                counts[number][kind] += 1
                if kind == 'strikes' and row[4]:
                    counts[number]['deaths'] += 1
        return counts, orphans


def cells(directory):
    return sorted(name[:-4] for name in os.listdir(directory) if name.endswith('.txt'))


def read(path):
    return io.open(path, encoding='utf-8', newline='').read()


def survivors(replay):
    return int(replay.summary['survivors']) if replay.summary else -1


def main(argv):
    if len(argv) != 2:
        raise SystemExit(__doc__)
    directory = argv[1]
    names = cells(directory)
    replays = {}
    for name in names:
        replays[name] = Replay(read(os.path.join(directory, name + '.txt')))

    print('=== 1. every cell: outcome, deaths by cause, exit code ===')
    print(f'{"cell":34} {"exit":>4} {"end":>11} {"surv":>4} {"dead":>4} {"combat":>6} '
          f'{"attrit":>6} {"terr A":>6} {"terr B":>6} {"strikes":>7} {"threats":>7} {"surr":>4}')
    for name in names:
        replay = replays[name]
        code = read(os.path.join(directory, name + '.exit')).strip()
        combat = sum(1 for death in replay.deaths if death[2] == 'combat')
        attrition = sum(1 for death in replay.deaths if death[2] == 'attrition')
        summary = replay.summary or {}
        print(f'{name:34} {code:>4} {replay.exit_reason:>11} {survivors(replay):>4} '
              f'{len(replay.deaths):>4} {combat:>6} {attrition:>6} '
              f'{summary.get("territory_a", "?"):>6} {summary.get("territory_b", "?"):>6} '
              f'{len(replay.strikes):>7} {len(replay.threats):>7} {len(replay.surrenders):>4}')

    traced = [name for name in names if replays[name].traced]

    print('\n=== 2. traced cells: every verb proposed and applied ===')
    print(f'{"cell":34} ' + ' '.join(f'{verb[:9]:>10}' for verb in VERBS))
    for name in traced:
        replay = replays[name]
        print(f'{name:34} ' + ' '.join(
            f'{replay.proposed[verb]:>4}/{replay.applied[verb]:<5}' for verb in VERBS))
    print('  each cell reads proposed/applied')

    print('\n=== 3. traced cells: rejections by verb and ground ===')
    any_rejection = False
    for name in traced:
        rows = sorted(replays[name].rejections.items())
        if rows:
            any_rejection = True
            print(f'{name:34} ' + '  '.join(f'{verb}:{ground}={count}'
                                            for (verb, ground), count in rows))
    if not any_rejection:
        print('  none in any traced cell')

    print('\n=== 4. traced cells: the six branches of rule 26 ===')
    print(f'{"cell":34} {"b1":>6} {"b2":>6} {"b3":>6} {"b4":>6} {"b5":>6} {"b6":>6} '
          f'{"total":>7} {"fight":>6} {"retr":>6} {"surr":>6} {"dead tgt":>8}')
    for name in traced:
        replay = replays[name]
        total = sum(replay.branches.values())
        print(f'{name:34} ' + ' '.join(f'{replay.branches[number]:>6}' for number in range(1, 7))
              + f' {total:>7} {replay.answers["fight"]:>6} {replay.answers["retreat"]:>6} '
              f'{replay.answers["surrender"]:>6} {replay.answer_target_dead:>8}')

    print('\n=== 5. traced cells: encounters, and what resolved inside them ===')
    print(f'{"cell":34} {"encs":>5} {"ticks":>6} {"mean":>6} {"longest":>7} {"strikes":>7} '
          f'{"str/enc":>7} {"threats":>7} {"surr":>5} {"deaths":>6} {"silent":>6} {"orphan":>6}')
    for name in traced:
        replay = replays[name]
        episodes = replay.encounters()
        counts, orphans = replay.attribute(episodes)
        length = sum(last - first + 1 for _, first, last in episodes)
        strikes = sum(count['strikes'] for count in counts)
        silent = sum(1 for count in counts if not any(count.values()))
        longest = max((last - first + 1 for _, first, last in episodes), default=0)
        print(f'{name:34} {len(episodes):>5} {length:>6} '
              f'{(length / len(episodes) if episodes else 0):>6.2f} {longest:>7} {strikes:>7} '
              f'{(strikes / len(episodes) if episodes else 0):>7.2f} '
              f'{sum(count["threats"] for count in counts):>7} '
              f'{sum(count["surrenders"] for count in counts):>5} '
              f'{sum(count["deaths"] for count in counts):>6} {silent:>6} {orphans:>6}')
    print('  silent: encounters in which nothing resolved. orphan: resolutions in no encounter')

    histogram = {}
    for name in traced:
        replay = replays[name]
        episodes = replay.encounters()
        counts, _ = replay.attribute(episodes)
        for count in counts:
            histogram[count['strikes']] = histogram.get(count['strikes'], 0) + 1
    total_encounters = sum(histogram.values())
    print(f'  strikes per encounter, over all {total_encounters} encounters:')
    for strikes in sorted(histogram):
        share = 100 * histogram[strikes] / total_encounters
        print(f'    {strikes:>2} strike(s): {histogram[strikes]:>4} encounters  {share:>5.1f}%')

    print('\n=== 6. rule 23 and rule 24 at the boundaries ===')
    print(f'{"cell":34} {"threats":>7} {"saturated":>9} {"surr":>5} {"discarded":>9} '
          f'{"all lost":>8} {"below 2":>7} {"transferred":>11}')
    for name in traced:
        replay = replays[name]
        saturated = sum(1 for threat in replay.threats if threat[3] == 0)
        discarded = sum(1 for row in replay.surrenders if row[4] > 0)
        all_lost = sum(1 for row in replay.surrenders if row[3] == 0 and row[4] > 0)
        free = sum(1 for row in replay.surrenders if row[5] < 2)
        print(f'{name:34} {len(replay.threats):>7} {saturated:>9} {len(replay.surrenders):>5} '
              f'{discarded:>9} {all_lost:>8} {free:>7} '
              f'{sum(row[3] for row in replay.surrenders):>11}')
    print('  saturated: increase 0 at fear 100. all lost: transferred 0 with a discard.')
    print('  below 2: a surrender at satiety 0 or 1, which rule 24 halves to nothing')

    forfeits = [(row[5] // 2, row[3], row[4], row[6]) for name in traced
                for row in replays[name].surrenders]
    print(f'  every one of the {len(forfeits)} surrenders, as rule 24 divides it:')
    print(f'    {"":>8} {"forfeit":>9} {"transferred":>11} {"discarded":>9} '
          f'{"recipient satiety":>17}')
    for label, pick in (('least', min), ('greatest', max), ('total', sum)):
        cells_of_row = [pick(row[index] for row in forfeits) for index in (0, 1, 2, 3)]
        if label == 'total':
            cells_of_row[3] = ''
        print(f'    {label:>8} {cells_of_row[0]:>9} {cells_of_row[1]:>11} '
              f'{cells_of_row[2]:>9} {cells_of_row[3]:>17}')
    full = sum(1 for row in forfeits if row[3] == ATTRIBUTE_MAX)
    print(f'  surrenders whose recipient was already at satiety {ATTRIBUTE_MAX}, so the whole '
          f'forfeit was discarded: {full}')

    print('\n=== 7. CHECKS: the reconstruction against the engine. Every row must read 0 ===')
    print('  the branch checks need trace lines and are silent on the fifteen untraced cells;')
    print('  the four resolution checks read events and run on all thirty.')
    names_of_checks = list(next(iter(replays.values())).checks)
    print(f'{"check":48} {"total":>7}  cells with a nonzero count')
    failures = 0
    for check in names_of_checks:
        row = {name: replays[name].checks[check] for name in names}
        failures += sum(row.values())
        offenders = [f'{name}={count}' for name, count in row.items() if count]
        print(f'{check:48} {sum(row.values()):>7}  '
              f'{", ".join(offenders) if offenders else "none"}')
    decisions = sum(sum(replays[name].branches.values()) for name in traced)
    print(f'\n{decisions} decisions classified across {len(traced)} traced cells, '
          f'{failures} check failures')
    print('branch 3 vs branch 6, over those cells: '
          f'{sum(replays[name].branches[3] for name in traced)} seek steps, '
          f'{sum(replays[name].branches[6] for name in traced)} search steps')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main(sys.argv))
