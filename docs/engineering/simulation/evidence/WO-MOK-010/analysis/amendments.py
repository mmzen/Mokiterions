"""WO-MOK-010 oracle 5: the governance state of every artifact this change amends.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-010/analysis/amendments.py

`VER-MOK-010` makes this a verification oracle rather than a formality: "an amendment nobody approved is not a
specification", and its absence fails the contract regardless of the state of the code. So this script reads the
artifacts and the git history and checks four things a reader would otherwise take on trust.

  1. **Status.** Every artifact in this work order's chain is `approved` and the work order is `in_progress`.
  2. **The amendment record covers what the owner approved, and the text carries it.** `WO-MOK-010` states the
     required amendments in full -- nine provisions and one appended rule in `SPEC-MOK-001`, two provisions and a
     re-check in `SPEC-MOK-002`, three provisions in `SPEC-MOK-003`. Each is looked for twice: in the amendment
     record's 2026-08-19 row, and in the specification's body. The two searches are over disjoint text -- the
     `## Amendment record` section is cut out of the body before the body is searched -- because otherwise a record
     that claims an amendment the text does not carry would satisfy both searches with the same sentence, which is
     exactly the failure this oracle exists to catch.
  3. **The earlier layer is untouched.** `WO-MOK-005` left six amendments **OUTSTANDING** across `SPEC-MOK-002`,
     `SPEC-MOK-003` and `ARCH-MOK-001`, and the repository owner overrode the gate that would have settled them
     before this work began. The mitigation recorded in `WO-MOK-010` is that the two layers stay separable by
     inspection, and that is a checkable claim: every amendment row dated before 2026-08-19 must be byte-identical to
     the one at the commit this work started from, and `VREC-MOK-005` and `ARCH-MOK-001` must not have been touched at
     all.
  4. **What this work order added beyond what was approved.** Three amendments were written during implementation that
     were not in the owner's stated list. They are named here with what each needs, rather than left for a reviewer to
     find by diffing.

Two provisions amend by deletion, and a search for a phrase cannot show that a phrase is gone. Those two are checked
negatively instead: the sentence that used to name `fear` and traits is located and its contents asserted. A negative
check that located nothing would be vacuous, so each is anchored on a phrase that must still be in the sentence, at
the far end of the list being inspected, and fails if the capture stopped short of it.

`self_test` exercises both checks on deliberately broken inputs before they are used, and the artifact reports how many
controls held. This is not ceremony: the first form of the `SPEC-MOK-003` deletion check terminated its sentence at the
em dash *preceding* the list, inspected forty-six characters of preamble, found no `fear` in them and passed. A check
that finds nothing reads exactly like a check that looks for nothing, so each one is made to fail on purpose first.

The date and the commit are inputs, not discoveries: the work began from 60fda9f and every row this work order adds is
dated 2026-08-19. A row appearing under this work order with a different date, or an earlier row that moved, is a
finding.
"""

import io
import os
import re
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8')

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.exists(os.path.join(ROOT, 'Cargo.toml')):
    ROOT = os.path.dirname(ROOT)
OUT = os.path.join(os.path.dirname(HERE), 'amendment-approvals.md')

BASE = '60fda9faffbd452752a34efa356f16cc6ad1d3ff'
TODAY = '2026-08-19'
DOCS = 'docs/engineering/simulation'

CHAIN = [
    (f'{DOCS}/intent/INT-MOK-006.md', 'approved'),
    (f'{DOCS}/capabilities/CAP-MOK-006.md', 'approved'),
    (f'{DOCS}/requirements/REQ-MOK-031.md', 'approved'),
    (f'{DOCS}/requirements/REQ-MOK-032.md', 'approved'),
    (f'{DOCS}/requirements/REQ-MOK-033.md', 'approved'),
    (f'{DOCS}/requirements/REQ-MOK-034.md', 'approved'),
    (f'{DOCS}/verification/VER-MOK-010.md', 'approved'),
    (f'{DOCS}/work-orders/WO-MOK-010.md', 'in_progress'),
]

# The provisions `WO-MOK-010` states in full. Each is a label, the phrase that identifies it in the amendment record,
# and the phrases that identify the amended text in the body -- an empty list where the provision is a statement about
# the amendment itself rather than a change to the text.
AMENDED = [
    {
        'path': f'{DOCS}/specifications/SPEC-MOK-001.md',
        'stated': 'nine provisions amended and one rule appended',
        'provisions': [
            ('1. *Scope* drops fear and traits from its exclusions',
             '*Scope* no longer excludes fear and individual traits',
             ['`CAP-MOK-002`, and `CAP-MOK-006`',
              'It defines one behavioral trait and the `fear` attribute, but no rule reads `fear`']),
            ('2. *State model / Mokiterion* gains fear and the trait',
             '*State model / Mokiterion* gains `fear`',
             ['`fear` starts at `0`, so no Mokiterion begins afraid']),
            ('3. A *Behavioral trait* subsection',
             'A new *Behavioral trait* subsection fixes `waste_tolerance`',
             ['### Behavioral trait',
              'The trait is `waste_tolerance`, an integer in']),
            ('4. *Time and entropy* records the one exception',
             '*Time and entropy* records that trait derivation is the one exception',
             ['Trait derivation is the one exception and lies outside it']),
            ('5. *Inputs* and *Help output* take the third value',
             '*Inputs* and *Help output* take the third `--policy` value',
             ['Only `baseline`, `reference`, and `individual` are valid values']),
            ('6. *Data and interface contracts* carries the trait and fear',
             '*Data and interface contracts* puts `waste_tolerance` on the observation',
             ['`waste_tolerance` is reported once, in `agent_initialized`']),
            ('7. Rule 1 places the derivation',
             'Rule 1 places the derivation',
             ["Each agent's `waste_tolerance` is derived as *Behavioral trait* specifies"]),
            ('8. Rule 3 carries the trait',
             'Rule 3 carries the trait',
             ["including the acting Mokiterion's `waste_tolerance`"]),
            ('9. Rule 7 traces the pre-update fear, rule 12 updates it',
             'Rule 12 becomes *Survival decay and fear*',
             ['**Survival decay and fear.**',
              'the value held **before** rule 12\'s update for this tick']),
            ('addition: rule 19, appended rather than renumbered',
             'Rule 19 is appended',
             ['19. **Trait-aware decision.**']),
            ('and rule 5 otherwise untouched',
             '**rule 5 is otherwise untouched',
             []),
        ],
    },
    {
        'path': f'{DOCS}/specifications/SPEC-MOK-002.md',
        'stated': "two entries amended in rule 5, and rule 6 re-checked rather than amended",
        'provisions': [
            ('1. `Policy` gains a third variant',
             '`simulation::Policy` gains a third variant',
             ['variants `Baseline`, `Reference` and `Individual`']),
            ('2. `AgentSnapshot` carries four `u8` attributes',
             '`simulation::AgentSnapshot` carries four `u8` attributes',
             ['four `u8` attributes and `Option<Action>`']),
            ('and rule 6 re-checked, not amended',
             'Rule 6 is **not** amended and was re-checked instead',
             []),
        ],
    },
    {
        'path': f'{DOCS}/specifications/SPEC-MOK-003.md',
        'stated': 'three provisions amended in rule 4',
        'provisions': [
            ('1. The two-line mockup shows four gauges',
             'The two-line mockup shows four gauges',
             ['f ████░░░░░░░░░░░░░░░░  20']),
            ("2. Rule 4's prose reads four attributes, both forms",
             "Rule 4's prose reads four attributes and four numeric values",
             ['carries health, satiety, energy and fear',
              'carrying identifier, territory and the four']),
            ("3. Item 5's reservation becomes a computed value",
             '**Item 5, the reservation, is replaced by the presentation of a computed value',
             ['The line-two bar row carries four gauges, the fourth being',
              'bar_width(interior) = min(20, (interior - 35) / 4)']),
            ("and the reservation's own reasoning retained, not deleted",
             'an inert `fear 0` would be a claim the engine cannot support',
             ['That reasoning is']),
        ],
    },
]

# Amendments by deletion. A phrase search cannot show that a phrase is gone, so the sentence that carried it is located
# and its contents asserted. Each entry is the artifact, the sentence's opening, its terminator, a phrase that must
# still be inside it, and the words that must no longer be.
#
# The anchor is not decoration. The first form of this check terminated `SPEC-MOK-003`'s sentence at the em dash that
# *precedes* the list, so it inspected forty-six characters of preamble and found no `fear` in them -- a pass that
# established nothing. An anchor at the far end of the list fails that truncation instead of tolerating it.
DELETIONS = [
    ('SPEC-MOK-001', "*Scope*'s exclusion sentence no longer names fear or traits",
     'It does not define OpenAI integration', '.', 'user interface', ['fear', 'trait']),
    ('SPEC-MOK-003', "rule 10 item 7's list of what the engine does not compute no longer names them",
     'Fields for values the engine does not compute', ' are absent', 'per-agent entropy', ['fear', 'trait']),
]

# Written during implementation, beyond the owner's stated list. Each is named with what it needs.
BEYOND = [
    {
        'artifact': 'SPEC-MOK-001',
        'what': 'The trait range narrowed from `0..=100` to `0..=40`, and with it rule 19\'s upper-bound note and the '
                'two acceptance examples that cited unreachable tolerances.',
        'marker': 'Narrowed the `waste_tolerance` range from `0..=100` to `0..=40`',
        'body': ['an integer in `0..=40`',
                 'The range is `0..=40`, narrowed on measured evidence'],
        'state': '**Approved.** The repository owner, acting as technical owner, chose narrowing over amending '
                 '`REQ-MOK-034`\'s survivor floor on 2026-08-19, when `WO-MOK-010` stop condition 6 fired. The '
                 'work order records the decision and `escalation.md` the measurement it was taken on. The first '
                 'form of *Behavioral trait* named this amendment as the one to make on exactly this evidence, so '
                 'it is a foreseen correction rather than an unplanned one.',
    },
    {
        'artifact': 'SPEC-MOK-001',
        'what': 'A correction to this work order\'s own first amendment: the *Help output* sentence it added required '
                'the explanatory prose to state which decision source is the default, contradicting the same '
                'section\'s approved *stated once* paragraph. The default clause is withdrawn; the three-source '
                'description stays.',
        'marker': "Corrected the *Help output* sentence this work order's first amendment added",
        'body': ['It states no default and no value constraint',
                 'the prose is where an earlier copy of them lived'],
        'state': '**Recorded, not approved — OUTSTANDING.** The contradiction was between two provisions of one '
                 'section, one approved 2026-08-17 and one 2026-08-19, and the inherited test '
                 '`cli::each_declared_default_is_stated_once` — bound by a `verified` `VREC-MOK-004` — asserts the '
                 'side the implementation is already on. Satisfying the withdrawn clause would have meant relaxing '
                 'that assertion, which `WO-MOK-010` forbids, so the specification is corrected instead. **It is a '
                 'correction to text the technical owner approved on 2026-08-19 and needs that owner\'s '
                 'ratification.**',
    },
    {
        'artifact': 'SPEC-MOK-003',
        'what': 'Three further provisions outside rule 4: the `AgentSnapshot` field list gains `fear`; rule 10 item 7 '
                'loses `fear` and traits from its list of values the engine does not compute; rule 11\'s '
                '`decision_source_selected` row gains `REQ-MOK-033` for `individual`.',
        'marker': 'Three further provisions were found during implementation',
        'body': ['energy, fear, applied_action',
                 'Amended 2026-08-19: this list named `fear` and traits',
                 '`REQ-MOK-033` when `individual`'],
        'state': '**Recorded, not separately approved.** These were found while implementing and are not in the list '
                 '`WO-MOK-010` states, so the owner has not approved them as such. Each is forced by the change '
                 'rather than chosen with it — a field list omitting `fear` would contradict `SPEC-MOK-002` rule 5, '
                 'an item claiming the engine computes neither `fear` nor traits would be false, and an exhaustive '
                 'mapping missing a row is a gap the compiler reaches before an operator does — and each is written '
                 'into the 2026-08-19 amendment row rather than made quietly. **They require the technical owner\'s '
                 'ratification, and this artifact is where that obligation is recorded.**',
    },
]

UNTOUCHED = [
    (f'{DOCS}/verification-records/VREC-MOK-005.md',
     'the record whose gate was overridden: still `ready`, its six amendments still OUTSTANDING, its seven manual '
     'assessments still unrecorded'),
    (f'{DOCS}/architecture/ARCH-MOK-001.md',
     'no architecture amendment is required by this work order, and none was made'),
]

ROW = re.compile(r'^\| (\d{4}-\d{2}-\d{2}) \|')
HEADING = re.compile(r'^## ')


def at(commit, path):
    finished = subprocess.run(['git', 'show', f'{commit}:{path}'], cwd=ROOT,
                              capture_output=True, text=True, encoding='utf-8', errors='replace')
    if finished.returncode != 0:
        raise SystemExit(f'{path} is not at {commit}: {finished.stderr.strip()}')
    return finished.stdout


def now(path):
    return io.open(os.path.join(ROOT, path), encoding='utf-8').read()


def field(text, name):
    match = re.search(rf'^{name} = "([^"]*)"', text, re.M)
    return match.group(1) if match else '(absent)'


def split(text):
    """(body, record): the document with its `## Amendment record` section cut out, and that section alone.

    The two are disjoint, which is the point: a body check must not be satisfiable by the record's own prose.
    """
    lines = text.split('\n')
    body, record, inside = [], [], False
    for line in lines:
        if HEADING.match(line):
            inside = line.strip() == '## Amendment record'
        (record if inside else body).append(line)
    if not record:
        raise SystemExit('no `## Amendment record` section found; the split is unsafe')
    return '\n'.join(body), '\n'.join(record)


def rows(text):
    """The amendment-record rows, by date. A row is one table line beginning with a date."""
    collected = {}
    for line in text.split('\n'):
        match = ROW.match(line.strip())
        if match:
            collected.setdefault(match.group(1), []).append(line.strip())
    return collected


def sentence(text, opening, terminator):
    """The sentence beginning with `opening`, up to `terminator`, with line wrapping flattened."""
    flat = ' '.join(text.split())
    start = flat.find(opening)
    if start < 0:
        return None
    end = flat.find(terminator, start + len(opening))
    return flat[start:end if end >= 0 else len(flat)]


def provision(name, body, row_text, label, marker, markers):
    """(cell, problems) for one provision: is it in the record, and is the amended text in the body."""
    in_record = marker in row_text
    missing = [phrase for phrase in markers if phrase not in body]
    problems = []
    if not in_record:
        problems.append(f'{name}: the record does not state "{label}"')
    if missing:
        problems.append(f'{name}: the body lacks {missing} for "{label}"')
    text = (f'{len(markers) - len(missing)}/{len(markers)} phrases' if markers
            else 'n/a — a statement about the amendment')
    return (f'| {label} | {"yes" if in_record else "**NO**"} | {text}'
            + (' |' if not missing else ' **NO** |')), problems


def deletion(name, body, label, opening, terminator, anchor, forbidden):
    """(cell, problems) for one amendment by deletion, checked negatively and anchored against truncation."""
    found = sentence(body, opening, terminator)
    if found is None or anchor not in found:
        return (f'| `{name}` | {label} | **NO** | not looked for |'),\
            [f'{name}: the sentence beginning "{opening}" was not captured through "{anchor}", '
             f'so its check would be vacuous']
    present = [word for word in forbidden if word in found.lower()]
    return (f'| `{name}` | {label} | yes, {len(found)} characters through `{anchor}` | '
            f'{"yes" if not present else "**NO**, still names " + str(present)} |'),\
        ([] if not present else [f'{name}: "{opening}..." still names {present}'])


def self_test():
    """Controls on the two checks above, run before they are used, so that a passing artifact is not a vacuous one.

    A checker that finds nothing is indistinguishable from a checker that looks for nothing, and this one was wrong in
    that direction once already: the first form of `SPEC-MOK-003`'s deletion check terminated its sentence before the
    list it was meant to inspect and passed by looking at preamble. So each control is the failure it guards against,
    injected deliberately, and asserted to be reported.
    """
    body, record = split(now(f'{DOCS}/specifications/SPEC-MOK-002.md'))
    row = '\n'.join(rows(record).get(TODAY, []))
    real = '`simulation::Policy` gains a third variant'
    controls = [
        ('a provision the record states and the body carries is reported clean',
         not provision('X', body, row, 'L', real, ['variants `Baseline`, `Reference` and `Individual`'])[1]),
        ('a provision the record does not state is reported',
         len(provision('X', body, row, 'L', 'a provision nobody wrote', [])[1]) == 1),
        ('a provision whose amended text is absent from the body is reported',
         len(provision('X', body, row, 'L', real, ['a phrase no specification contains'])[1]) == 1),
        ('the body and the record are disjoint, so the record\'s own prose cannot satisfy a body check',
         len(provision('X', body, row, 'L', real, ['gains a third variant'])[1]) == 1),
        ('a deletion check whose sentence is not found is reported rather than passed',
         len(deletion('X', body, 'L', 'a sentence no specification contains', '.', 'a', [])[1]) == 1),
        ('a deletion check truncated before its anchor is reported rather than passed',
         len(deletion('X', body, 'L', 'Rule 5', '.', 'an anchor beyond the terminator', [])[1]) == 1),
        ('a forbidden word still present in the sentence is reported',
         len(deletion('X', body, 'L', 'Rule 5', 'Every field', 'snapshot', ['snapshot'])[1]) == 1),
    ]
    return controls


def main():
    problems = []
    lines = [
        '# WO-MOK-010 amendment approvals — oracle 5',
        '',
        '`VER-MOK-010`\'s fifth oracle is the governance state of the artifacts this change amends: "an amendment',
        'nobody approved is not a specification", and its absence fails the contract regardless of the state of the',
        'code. This file is generated by `analysis/amendments.py`, which reads the artifacts and the git history.',
        'What it checks, and why each check is not a formality, is in that script\'s header.',
        '',
        f'Everything below is measured against the commit this work started from, `{BASE[:7]}`, and every row this',
        f'work order adds is dated `{TODAY}`.',
        '',
        '## 1. The chain\'s governance state',
        '',
        '| Artifact | status | expected | updated | ok |',
        '|---|---|---|---|---|',
    ]
    for path, expected in CHAIN:
        text = now(path)
        status = field(text, 'status')
        ok = status == expected
        if not ok:
            problems.append(f'{os.path.basename(path)} is `{status}`, expected `{expected}`')
        lines.append(f'| `{os.path.basename(path)[:-3]}` | `{status}` | `{expected}` | '
                     f'{field(text, "updated")} | {"yes" if ok else "**NO**"} |')
    for spec in AMENDED:
        text = now(spec['path'])
        name = os.path.basename(spec['path'])[:-3]
        moved = field(text, 'updated') == TODAY
        if not moved:
            problems.append(f'{name} was amended but its `updated` date is not {TODAY}')
        lines.append(f'| `{name}` | `{field(text, "status")}` | `approved`, amended | '
                     f'{field(text, "updated")} | {"yes" if moved else "**NO**"} |')

    lines += [
        '',
        'The work order is `in_progress` and not `complete`: a work order is closed by a verification record that',
        'binds a commit, and that record is written after the commit it names. `WO-MOK-006` closed the same way.',
        '',
        '## 2. The amendment record against the approved list',
        '',
        f'Each provision `WO-MOK-010` states in full is looked for twice — in the amendment record\'s {TODAY} row,',
        'and in the specification\'s body — and the two searches are over disjoint text, because a record that',
        'claimed an amendment the text does not carry would otherwise satisfy both with the same sentence.',
        '',
    ]
    for spec in AMENDED:
        body, record = split(now(spec['path']))
        name = os.path.basename(spec['path'])[:-3]
        dated = rows(record).get(TODAY, [])
        row_text = '\n'.join(dated)
        approved = f'Approved {TODAY}' in row_text
        names_wo = 'WO-MOK-010' in row_text
        attributed = 'did not decide the substance' in row_text
        for held, complaint in ((approved, f'no approval recorded in its {TODAY} row'),
                                (names_wo, f'its {TODAY} row does not name this work order'),
                                (attributed, f'its {TODAY} row does not record who wrote the text')):
            if not held:
                problems.append(f'{name}: {complaint}')
        lines += [f'### `{name}` — {spec["stated"]}', '',
                  f'- rows dated {TODAY}: **{len(dated)}**',
                  f'- approval recorded in the row: {"yes" if approved else "**NO**"}',
                  f'- names this work order: {"yes" if names_wo else "**NO**"}',
                  '- records that the implementation agent wrote the text and did not decide the substance: '
                  f'{"yes" if attributed else "**NO**"}',
                  '',
                  '| Provision | in the record | in the text |',
                  '|---|---|---|']
        for label, marker, markers in spec['provisions']:
            cell, found = provision(name, body, row_text, label, marker, markers)
            lines.append(cell)
            problems += found
        lines.append('')

    lines += [
        'Two of those provisions amend by deletion, and no phrase search can show that a phrase is gone. The sentence',
        'that carried it is located instead, and its contents asserted. Each sentence is anchored on a phrase that must',
        'still be in it, at the far end of the list being inspected, so a capture that stopped short of the list fails',
        'rather than reporting an absence it never looked for.',
        '',
        '| Artifact | sentence | located, and the anchor in it | no longer names |',
        '|---|---|---|---|',
    ]
    for artifact, label, opening, terminator, anchor, forbidden in DELETIONS:
        spec = next(entry for entry in AMENDED if entry['path'].endswith(f'{artifact}.md'))
        body, _ = split(now(spec['path']))
        cell, found = deletion(artifact, body, label, opening, terminator, anchor, forbidden)
        lines.append(cell)
        problems += found

    controls = self_test()
    lines += [
        '',
        'Both of those checks, and the provision check above them, were exercised on deliberately broken inputs before',
        'being used, because a check that finds nothing reads the same as a check that looks for nothing — and this one',
        'was wrong in that direction once already. `self_test` in the script injects each failure and asserts that it is',
        f'reported. {sum(1 for _, held in controls if held)} of {len(controls)} controls held:',
        '',
    ]
    for label, held in controls:
        if not held:
            problems.append(f'control failed: {label}')
        lines.append(f'- {"ok" if held else "**FAILED**"} — {label}')

    lines += [
        '',
        '## 3. What was amended beyond the approved list',
        '',
        'Three amendments were written during implementation and are not in the list `WO-MOK-010` states. None is left',
        'to be found in a diff: each is written into the specification\'s own amendment record, and each is named here',
        'with what it still needs. One is approved, because the owner took it as a decision under a stop condition.',
        'The other two are not, and say so.',
        '',
    ]
    for extra in BEYOND:
        spec = next(entry for entry in AMENDED if entry['path'].endswith(f'{extra["artifact"]}.md'))
        body, record = split(now(spec['path']))
        in_record = extra['marker'] in record
        missing = [phrase for phrase in extra['body'] if phrase not in body]
        if not in_record:
            problems.append(f'{extra["artifact"]}: the record does not state the amendment beyond the list')
        if missing:
            problems.append(f'{extra["artifact"]}: the body lacks {missing} for the amendment beyond the list')
        lines += [f'**`{extra["artifact"]}`.** {extra["what"]}', '',
                  f'- in the amendment record: {"yes" if in_record else "**NO**"}',
                  f'- in the specification\'s body: {len(extra["body"]) - len(missing)}/{len(extra["body"])} phrases'
                  + ('' if not missing else f', **missing {missing}**'),
                  f'- {extra["state"]}', '']

    lines += [
        '## 4. The earlier layer, left where it was',
        '',
        '`WO-MOK-005` left six amendments **OUTSTANDING** across `SPEC-MOK-002`, `SPEC-MOK-003` and `ARCH-MOK-001`,',
        'and the repository owner overrode the gate that would have settled them before this work began. The',
        'mitigation `WO-MOK-010` records is that the two layers remain separable by inspection. That is a checkable',
        'claim, and this is the check: every amendment row dated before this work order\'s date, compared byte for',
        f'byte against `{BASE[:7]}`.',
        '',
        '| Artifact | rows before this date | identical to the base commit | OUTSTANDING markers, then and now |',
        '|---|---|---|---|',
    ]
    for spec in AMENDED:
        name = os.path.basename(spec['path'])[:-3]
        before = rows(split(at(BASE, spec['path']))[1])
        after = rows(split(now(spec['path']))[1])
        earlier_before = {date: value for date, value in before.items() if date < TODAY}
        earlier_after = {date: value for date, value in after.items() if date < TODAY}
        identical = earlier_before == earlier_after
        if not identical:
            problems.append(f'{name}: an amendment row dated before {TODAY} moved')
        marks_before = sum(value.count('OUTSTANDING')
                           for values in earlier_before.values() for value in values)
        marks_after = sum(value.count('OUTSTANDING')
                          for values in earlier_after.values() for value in values)
        if marks_before != marks_after:
            problems.append(f'{name}: an OUTSTANDING marking changed')
        lines.append(f'| `{name}` | {sum(len(value) for value in earlier_after.values())} | '
                     f'{"yes" if identical else "**NO**"} | {marks_before}, then {marks_after} |')

    lines += ['', '| Artifact | changed since the base commit | why it must not be |', '|---|---|---|']
    for path, why in UNTOUCHED:
        name = os.path.basename(path)[:-3]
        changed = at(BASE, path) != now(path)
        if changed:
            problems.append(f'{name} changed, and this work order must not touch it')
        lines.append(f'| `{name}` | {"**YES**" if changed else "no"} | {why} |')

    vrec = now(f'{DOCS}/verification-records/VREC-MOK-005.md')
    lines += [
        '',
        f'`VREC-MOK-005` is `{field(vrec, "status")}`, as it was. This work order does not approve its six',
        'amendments, does not verify it, does not perform its seven manual assessments and does not transition',
        '`WO-MOK-005`. **The override is a cost carried forward, not a debt paid.** So the honest statement of this',
        'oracle is that its second condition — that the amendments already outstanding under `VREC-MOK-005` be',
        'resolved before this change is verified — is **not met**, by the repository owner\'s explicit decision of',
        f'{TODAY}, recorded in `WO-MOK-010` under *The gate was overridden*. What this artifact establishes is the',
        'first condition: the amendments this change itself requires are present, approved, carried by the text, and',
        'separable from the earlier layer.',
        '',
        '## Result',
        '',
    ]

    if problems:
        lines += ['Findings:', ''] + [f'- {problem}' for problem in problems] + ['']
    lines += ['**RESULT: ' + (
        'PASS** — every artifact in the chain is approved, every provision the owner approved is in both the record '
        'and the text, the two provisions that amend by deletion are shown to have deleted, the three amendments beyond '
        'the approved list are named with what each needs, and the earlier layer is byte-identical to the commit this '
        f'work started from. All {len(controls)} controls on the checks themselves held, so no result above is a check '
        'that looked for nothing. Oracle 5\'s second condition is unmet by the owner\'s recorded override, which is '
        'stated above rather than counted as a pass.'
        if not problems else f'FAIL** — {len(problems)} finding(s) above.')]

    io.open(OUT, 'w', encoding='utf-8', newline='\n').write(
        '\n'.join(line.rstrip() for line in lines) + '\n')
    print('\n'.join(lines))
    print(f'\nwritten to: {OUT}')
    return 0 if not problems else 1


if __name__ == '__main__':
    sys.exit(main())
