"""WO-MOK-017: the enumeration showing that no decision source reads composition.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/reads.py \
        mokiterions-core/src/simulation.rs \
        > docs/engineering/simulation/evidence/WO-MOK-017/post/reads.md

Writes markdown to stdout and exits `0` when every check passes, non-zero otherwise.

WHY THIS CLAIM IS LOAD-BEARING
------------------------------
`REQ-MOK-060` bounds a territory's class composition. `post/composition.txt` measures the bound met and
attributes the improvement to the corrected non-waste condition -- to Mokiterions eating resources they
previously left standing. That attribution is only sound if composition is an *output* of the simulation.
If any decision source could read the per-class counts, three things would follow, and all three would be
worse than the drift this work order corrects:

  * The measured curve would be a control loop rather than a consequence, so nothing in
    `post/composition.txt` would isolate the condition's effect from the loop's.
  * The ceiling would be gameable: a source could satisfy `REQ-MOK-060` by steering toward the ratio
    rather than by feeding sensibly, and the requirement would have specified a number the engine
    optimizes for instead of a property of a working world.
  * `SPEC-MOK-001` rule 3's own contract would be broken. Rule 3 fixes what a source may see, and a
    source that reached past it would make every other "the source cannot know that" argument in this
    repository unreliable.

So the enumeration is made two ways. Structurally, from the type a source is handed -- which is the
argument that cannot be defeated by a future edit inside a source body. And empirically, over every
function each source can transitively reach, which is the argument that catches a helper doing the
reaching on a source's behalf.
"""

import re
import sys
from pathlib import Path

# LF regardless of platform: `.gitattributes` pins this evidence tree `-text`.
sys.stdout.reconfigure(encoding='utf-8', newline='\n')

TEST_BOUNDARY = '#[cfg(test)]'

SOURCES = ('Baseline', 'Reference', 'Individual', 'Social')
DECIDERS = tuple(f'{source}DecisionSource::decide' for source in SOURCES)

# Composition is the per-territory, per-class count of standing resources. `food_counts` is where
# it is computed, `foods` is the authoritative collection it is computed from, and the six
# `food_*` summary fields are where it is reported. Any of the three in a source's reach would
# sink the claim.
COMPOSITION = re.compile(r'\b(food_counts|foods|food_a|food_b|standing|counts\[)')

FUNCTION = re.compile(r'^(\s*)fn (\w+)')
IMPL = re.compile(r'^impl(?:<[^>]*>)?\s+(?:\w+(?:<[^>]*>)?\s+for\s+)?(\w+)')
CALL = re.compile(r'\b([a-z_][a-z0-9_]*)\s*\(')
FIELD = re.compile(r'\.([a-z_][a-z0-9_]*)\b(?!\s*\()')

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


def function_bodies(lines, boundary):
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
    return bodies


def call_closure(bodies, roots, barred=()):
    """Every module function reachable from `roots`, following calls by bare name.

    Bare-name matching over-approximates, which is the safe direction for a negative claim: the
    closure scans more than a source can truly reach, so "no reachable function touches
    composition" is stronger than the call graph strictly requires, never weaker. `barred` stops
    re-entry into the other three `decide` implementations, which share a bare name -- without it
    every source's closure swallows the others and each claim becomes the same claim.
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


def struct_fields(lines, name):
    """The declared field names of a struct, in declaration order, doc comments skipped."""
    start = next(number for number, text in enumerate(lines, start=1)
                 if text.startswith(f'struct {name} {{'))
    fields = []
    for text in lines[start:]:
        if text == '}':
            break
        stripped = text.strip()
        if stripped.startswith('///') or stripped.startswith('//') or not stripped:
            continue
        match = re.match(r'([a-z_][a-z0-9_]*):\s*(.+?),?$', stripped)
        if match:
            fields.append((match.group(1), match.group(2).rstrip(',')))
    return fields


def main():
    source_path = Path(sys.argv[1])
    lines = source_path.read_text(encoding='utf-8').splitlines()
    boundary = next(number for number, text in enumerate(lines, start=1) if text == TEST_BOUNDARY)
    bodies = function_bodies(lines, boundary)
    observation = struct_fields(lines, 'Observation')

    print('# WO-MOK-017: no decision source reads composition')
    print()
    print('| | |')
    print('|---|---|')
    print('| Work order | `WO-MOK-017` (the resource composition drift) |')
    print('| Retains | "the enumeration showing no source reads composition" |')
    print('| Candidate | `post/COMMIT.txt` |')
    print('| Reader | `analysis/reads.py`, over the candidate\'s '
          '`mokiterions-core/src/simulation.rs` |')
    print('| Date | 2026-08-21 |')
    print()
    print('`REQ-MOK-060` bounds a territory\'s class composition, and `post/composition.txt` attributes')
    print('the corrected ratio to Mokiterions eating resources they used to leave standing. That')
    print('attribution holds only if composition is something the simulation *produces*. A source that')
    print('could read the per-class counts would turn the measured curve into a control loop, would make')
    print('the ceiling satisfiable by steering rather than by feeding, and would break `SPEC-MOK-001`')
    print('rule 3\'s contract about what a source may see. So the claim is enumerated twice below: once')
    print('from the type a source is handed, and once over every function a source can reach.')

    # ------------------------------------------------------------------ 1
    print()
    print('## 1. Where composition exists in the engine, and who reads it')
    print()
    print('Composition is the per-territory, per-class count of standing resources. One function computes')
    print('it, and every caller of that function is listed -- not a sample of them.')
    print()
    if not check('Simulation::food_counts' in bodies, 'Simulation::food_counts is absent'):
        return 1
    start, end, _ = bodies['Simulation::food_counts']
    print(f'`Simulation::food_counts(territory) -> [usize; 3]`, lines {start}-{end}, is the only place the')
    print('counts are formed. Its callers ahead of the test module:')
    print()
    print('| caller | line | what it feeds | on a decision path |')
    print('|---|---|---|---|')
    callers = []
    for label, (body_start, body_end, body) in sorted(bodies.items(), key=lambda item: item[1][0]):
        for offset, text in enumerate(body):
            if 'food_counts(' in text and label != 'Simulation::food_counts':
                callers.append((label, body_start + offset))
    described = {
        'Simulation::territory_snapshot': "the TUI's `TerritorySnapshot`, an observer surface",
        'Simulation::summary': "rule 18's `RunSummary`, emitted after the run ends",
    }
    for label, line in callers:
        purpose = described.get(label, '**UNDESCRIBED -- classify this caller**')
        check(label in described, f'{label} calls food_counts and is not classified')
        print(f'| `{label}` | {line} | {purpose} | no |')
    print()
    distinct = sorted({label for label, _ in callers})
    print(f'That is {len(callers)} call sites in {len(distinct)} functions -- `Simulation::summary` calls it')
    print('once per territory -- and both functions are reporting surfaces. Rule 18\'s summary is written')
    print('once the run is over, so nothing can act on it. The TUI\'s snapshot is read by an observer that')
    print('never proposes an action -- `INT-MOK-010` separates the two, and the composition figures in')
    print('`post/composition.txt` are recovered from the summary line for exactly this reason.')

    # ------------------------------------------------------------------ 2
    print()
    print('## 2. The structural enumeration: what a source is handed')
    print()
    print('This is the argument that survives future edits, because it is about the type rather than about')
    print('what today\'s bodies happen to do.')
    print()
    trait_start = next(number for number, text in enumerate(lines, start=1)
                       if text.startswith('trait DecisionSource'))
    signature = next(text.strip() for text in lines[trait_start:]
                     if text.strip().startswith('fn decide'))
    print('```rust')
    print(signature)
    print('```')
    print()
    print(f'Declared at line {trait_start}. A source receives an `&Observation` and an')
    print('`&mut DecisionEntropy`, and nothing else. It is handed no `&Simulation`, so `self.foods` and')
    print('`food_counts` are not merely unused by the four sources -- they are **out of scope by type**.')
    check('&Simulation' not in signature and 'Simulation' not in signature,
          'DecisionSource::decide takes a Simulation reference')
    check(signature.count('&') == 3,
          f'DecisionSource::decide has an unexpected parameter list: {signature}')
    print('A source cannot read composition without a signature change, and a signature change is a')
    print('visible amendment to rule 3 rather than a quiet edit inside a source body.')
    print()
    print('So the whole of what a source may see is the observation. Every field of it, enumerated in')
    print('declaration order, with what it could tell a source about composition:')
    print()
    print('| field | type | carries composition |')
    print('|---|---|---|')
    verdicts = {
        'tick': 'no — a scalar clock',
        'agent_id': 'no — an identity',
        'position': 'no — one coordinate',
        'territory': 'no — which territory, not what is in it',
        'health': 'no — the observer\'s own attribute',
        'satiety': 'no — the observer\'s own attribute',
        'energy': 'no — the observer\'s own attribute',
        'fear': 'no — the observer\'s own attribute',
        'waste_tolerance': 'no — the observer\'s own trait',
        'suffered': 'no — attacks suffered, with attacker and damage',
        'co_located_food': 'no — ids underfoot, no class and no counts',
        'perceived_food': '**class-bearing, but not composition** — see below',
        'perceived_mokiterions': 'no — id, direction, distance',
        'valid_actions': 'no — rule 5\'s core proposals',
    }
    for field, kind in observation:
        verdict = verdicts.get(field, '**UNCLASSIFIED — a new field needs a verdict here**')
        check(field in verdicts, f'Observation field {field} is unclassified')
        print(f'| `{field}` | `{kind}` | {verdict} |')
    stale = sorted(set(verdicts) - {field for field, _ in observation})
    check(not stale, f'classified fields that no longer exist on Observation: {stale}')
    print()
    print(f'{len(observation)} fields, each classified. `perceived_food` is the only one that needs an')
    print('argument rather than a glance, and it gets one because it is the field a sceptical reader')
    print('should press on:')
    print()
    perceived = struct_fields(lines, 'PerceivedFood')
    print('```rust')
    print('struct PerceivedFood {')
    for field, kind in perceived:
        print(f'    {field}: {kind},')
    print('}')
    print('```')
    print()
    print('It carries `class`, so a source knows the calorie class of resources it can *see*. That is not')
    print('composition, on three counts, and the difference is the whole of this section:')
    print()
    print('* It is **radius-limited**. The list is built inside `PERCEPTION_RADIUS` of the observer, so it')
    print('  is a local window and not a territory. A territory is 128 by 64 cells.')
    print('* It is **a list of individuals, not counts**. Nothing sums it by class, and nothing compares')
    print('  the sums. Composition is the ratio between three totals; this is a sorted sequence of items.')
    print('* It **omits what has been eaten**. Composition is a statement about what remains standing')
    print('  across a territory, including everything out of perception. No source can see that set.')

    # ------------------------------------------------------------------ 3
    print()
    print('## 3. The empirical enumeration: every function each source can reach')
    print()
    print('The structural argument is about the four sources\' signature. This one closes the remaining')
    print('gap -- a helper reading composition on a source\'s behalf -- by asking the question of every')
    print('function each source can transitively reach, not just of the four `decide` bodies.')
    print()
    print('| source | `decide` at | functions reachable | any reads composition |')
    print('|---|---|---|---|')
    for source in SOURCES:
        label = f'{source}DecisionSource::decide'
        if not check(label in bodies, f'{label} is absent'):
            continue
        start, _, _ = bodies[label]
        closure = call_closure(bodies, [label], barred=DECIDERS)
        touching = sorted(reached for reached in closure
                          if any(COMPOSITION.search(text) for text in bodies[reached][2]))
        check(not touching, f'{label} can reach composition through {touching}')
        verdict = 'none' if not touching else '**' + ', '.join(touching) + '**'
        print(f'| `{source.lower()}` | {start} | {len(closure)} | {verdict} |')
    print()
    print('The patterns searched for are `food_counts`, `foods`, `food_a`, `food_b`, `standing` and')
    print('`counts[` -- the computing function, the authoritative collection it reads, the summary fields')
    print('it feeds and the snapshot field beside them. The call graph is followed by bare callee name,')
    print('which over-approximates: each closure contains functions the source may not truly reach, so a')
    print('clean result here is stronger than the graph strictly requires rather than weaker.')
    print()
    print('Which fields the sources *do* read, for contrast. This table is built from each `decide` body')
    print('alone, plus the helpers it hands the observation to, named separately. The over-approximated')
    print('closure is deliberately **not** used here: it reaches `Observation::is_consistent`, which')
    print('touches every field in a `debug_assert!`, so a closure-wide intersection returns all fourteen')
    print('for all four sources and says nothing. Over-approximation is the safe direction for the')
    print('negative claim above and the useless direction for this one.')
    print()
    print('| source | fields read in `decide` itself | functions called where the observation appears |')
    print('|---|---|---|')
    names = {field for field, _ in observation}
    for source in SOURCES:
        label = f'{source}DecisionSource::decide'
        if label not in bodies:
            continue
        body = bodies[label][2]
        read = sorted({match for text in body for match in FIELD.findall(text)} & names)
        # A callee is listed when the observation is on the same line as the call, which is what
        # "hands the observation to" means here. Names are resolved against the module's functions
        # by bare name, and `decide` is excluded so that the signature line does not list itself.
        defined = {label.split('::')[-1] for label in bodies}
        delegates = sorted({name for text in body for name in CALL.findall(text)
                            if name in defined and name != 'decide' and 'observation' in text})
        print(f'| `{source.lower()}` | {", ".join(f"`{field}`" for field in read) or "none"} | '
              f'{", ".join(f"`{name}`" for name in delegates) or "nothing"} |')
    print()
    print('The third column is literal: the functions called on a line where `observation` also appears.')
    print('`baseline`\'s `choose_index` is there because it is handed `observation.valid_actions.len()`,')
    print('not the observation, and `is_consistent` is a `debug_assert!` in all four.')
    print()
    print('`baseline` is the row that matters. It reads `valid_actions`, draws over its length, and calls')
    print('none of the `fits` family or the rule 19 helpers -- so the corrected condition is unreachable')
    print('from rule 4. That is why a change to the waste arithmetic leaves `--policy baseline` alone, and')
    print('why `post/byte-identity.txt` can hold in all thirty baseline cells. The other three reach the')
    print('condition through `best_fitting_*` or the `tolerant_*` helpers named beside them, which is the')
    print('path every divergence in `post/divergence.txt` is attributed to.')

    # ------------------------------------------------------------------ 4
    print()
    print('## 4. What this does not claim')
    print()
    print('* **Not** that the sources are indifferent to class. They are not: preference is ranked by')
    print('  calorie class, and `REQ-MOK-060` exists because that preference plus an over-strict waste')
    print('  condition left high-class resources standing. The sources are causally *upstream* of')
    print('  composition, which is exactly why the correction moves it. Being upstream of a quantity is')
    print('  not reading it.')
    print('* **Not** that nothing in the repository reads composition. Two things do, both listed in')
    print('  section 1, and the TUI\'s observer is one of them. `INT-MOK-010` separates observation from')
    print('  decision, and this file is evidence that the separation holds on the decision side.')
    print('* **Not** a guarantee about future sources. It is a measurement of the four that exist at this')
    print('  candidate. What generalizes is section 2: while `decide` takes only an observation, a new')
    print('  source inherits the same inability, and granting one composition means amending rule 3 in')
    print('  the open.')
    print()
    if failures:
        print(f'**RESULT: FAIL — {len(failures)} check(s) failed:**')
        print()
        for failure in failures:
            print(f'* {failure}')
    else:
        print('**RESULT: PASS — composition is computed in one function with two callers, both reporting')
        print('surfaces and neither on a decision path; a source is handed no reference that could reach')
        print('it; and none of the functions the four sources can transitively reach touches it.**')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
