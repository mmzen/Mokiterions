"""WO-MOK-010 oracle 5: the governance state of every artifact this change amends.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-010/analysis/amendments.py

`VER-MOK-010` makes this a verification oracle rather than a formality: "an amendment nobody approved is not a
specification", and its absence fails the contract regardless of the state of the code. So this script reads the
artifacts and the git history and checks five things a reader would otherwise take on trust.

  1. **Status.** Every artifact in this work order's chain is `approved` and the work order is `in_progress`.
  2. **The amendment record covers what the owner approved, and the text carries it.** `WO-MOK-010` states the
     required amendments in full -- nine provisions and one appended rule in `SPEC-MOK-001`, two provisions and a
     re-check in `SPEC-MOK-002`, three provisions in `SPEC-MOK-003`. Each is looked for twice: in the amendment
     record's 2026-08-19 row, and in the specification's body. The two searches are over disjoint text -- the
     `## Amendment record` section is cut out of the body before the body is searched -- because otherwise a record
     that claims an amendment the text does not carry would satisfy both searches with the same sentence, which is
     exactly the failure this oracle exists to catch. The rows read are the 2026-08-19 rows *naming this work order*:
     the merge brought rows of the same date, approved by the same owner on the same day, into three of these records,
     and a search across the whole date would let one of `master`'s rows supply this work order's approval.
  3. **The earlier layer is untouched.** `WO-MOK-005` left six amendments **OUTSTANDING** across `SPEC-MOK-002`,
     `SPEC-MOK-003` and `ARCH-MOK-001`, and the repository owner overrode the gate that would have settled them
     before this work began. The mitigation recorded in `WO-MOK-010` is that the two layers stay separable by
     inspection, and that is a checkable claim: every amendment row dated before 2026-08-19 must be byte-identical to
     the one at the commit this work started from, and `VREC-MOK-005` and `ARCH-MOK-001` must not have been touched by
     this branch at all.
  4. **`master`'s own rows survived the merge.** `master` advanced by ten commits while this branch sat unmerged, and
     four of its amendment rows are dated 2026-08-19 -- the same date this work order's rows carry. Check 3 cannot see
     them: they did not exist at the commit this work started from, and their date puts them outside the "before this
     date" window. So they are checked against `master`'s tip instead, and every row present there must be present here
     byte for byte. A merge that silently dropped one of them would otherwise read as a clean result.
  5. **What this work order added beyond what was approved.** Seven amendments were written during implementation that
     were not in the owner's stated list, four of them after the merge. They are named here with what each needs, rather
     than left for a reviewer to find by diffing.

Two provisions amend by deletion, and a search for a phrase cannot show that a phrase is gone. Those two are checked
negatively instead: the sentence that used to name `fear` and traits is located and its contents asserted. A negative
check that located nothing would be vacuous, so each is anchored on a phrase that must still be in the sentence, at
the far end of the list being inspected, and fails if the capture stopped short of it.

`self_test` exercises both checks on deliberately broken inputs before they are used, and the artifact reports how many
controls held. This is not ceremony: the first form of the `SPEC-MOK-003` deletion check terminated its sentence at the
em dash *preceding* the list, inspected forty-six characters of preamble, found no `fear` in them and passed. A check
that finds nothing reads exactly like a check that looks for nothing, so each one is made to fail on purpose first.

The date and the two commits are inputs, not discoveries: the work began from 60fda9f, it is merged with `master` at
7a2b502, and every row this work order adds is dated 2026-08-19. A row appearing under this work order with a different
date, an earlier row that moved, or a row of `master`'s that the merge did not carry through, is a finding.

Which commit each check uses is a decision the merge forced, and it is stated rather than left implicit. The
row-immobility check of item 3 stays on 60fda9f, because what it establishes is that *this branch* moved no earlier row.
The untouched-artifact check of item 3 moved to 7a2b502, because `VREC-MOK-005` is no longer the file this work started
from: `master` re-captured it and transitioned it to `verified`, in commits 3696fae and a53712c, and comparing against
60fda9f reports that change as this work order's when it is not. Against `master`'s tip the same check answers the
question it was written to answer -- did this branch touch it -- and the change `master` made is disclosed in the table
rather than dropped.
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
MASTER = '7a2b502b908be03ad8e2de7c23ee3eaaf4ece048'
TODAY = '2026-08-19'
DOCS = 'docs/engineering/simulation'


def spec_path(name):
    """The path of a specification by identifier, so an artifact can be reached without being in `AMENDED`.

    `SPEC-MOK-004` is amended by this work order but is not in `AMENDED`, because `WO-MOK-010` states no provision of
    it: the amendment is a consequence the implementation found, and it is in `BEYOND` instead.
    """
    return f'{DOCS}/specifications/{name}.md'


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
#
# The last four were written after `master` was merged in, and three of them exist only because two owners' approvals of
# the same date met in one tree. They are here rather than in `AMENDED` because `WO-MOK-010` states none of them: a work
# order approved before the merge could not have.
BEYOND = [
    {
        'artifact': 'SPEC-MOK-001',
        'what': 'The trait range narrowed from `0..=100` to `0..=40`, and with it rule 19\'s upper-bound note and the '
                'two acceptance examples that cited unreachable tolerances.',
        'marker': 'Narrowed the `waste_tolerance` range from `0..=100` to `0..=40`',
        'outstanding': False,
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
        'outstanding': True,
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
        'outstanding': True,
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
    {
        'artifact': 'SPEC-MOK-003',
        'what': 'A row that changes no provision, recording that rule 5 as `WO-MOK-005` amended it and rule 4 as this '
                'work order amended it were approved on the same date by the same owner against different trees, and '
                'that where they meet — rule 4\'s collapse threshold of 47 columns against rule 5\'s roster pane of '
                '47 columns — the merged text is consistent. The consequence is a re-derivation of oracle 4, not a '
                'change of text.',
        'marker': '**No rule changed. This row records the reconciliation of the rule 5 and rule 4 rows above',
        'outstanding': False,
        'body': ['| roster | `W ≥ 100` |',
                 'bar_width(interior) = min(20, (interior - 35) / 4)'],
        'state': '**Recorded, and it ratifies nothing.** The row states a fact about two amendments it holds no '
                 'authority over, and adds, removes and rewords no provision — which is checkable, and is what the two '
                 'phrases above check: both amendments are present in the body, in the words their own rows use. It '
                 'needs no ratification of its own, and it supplies none for the two rows it reconciles.',
    },
    {
        'artifact': 'SPEC-MOK-003',
        'what': 'Rule 4 clause 7 amended in two provisions, so that the four gauges of clause 5 coexist with '
                '`master`\'s bands: the bands apply to health, satiety and energy and not to `fear`, and `fear` renders '
                'as a numeric value with no colour at all. Clause 7 as `master` approved it said "the roster\'s three '
                'bars" when clause 5 as this work order approved it draws four.',
        'marker': '**Rule 4 clause 7 amended in two provisions',
        'outstanding': True,
        'body': ['7. **Survival bands.**',
                 'Each of the three survival bars — health, satiety and energy —',
                 'numeric value with no colour at all'],
        'state': '**Decided by the owner; the wording is the agent\'s and is OUTSTANDING.** The repository owner, '
                 'acting as technical owner, was shown the collision and chose bands on health, satiety and energy '
                 'only, with `fear` unbanded, on 2026-08-19. The substance is the owner\'s. The text that records it '
                 'was written by the implementation agent and **requires that owner\'s ratification**; the decision it '
                 'records does not.',
    },
    {
        'artifact': 'SPEC-MOK-004',
        'what': 'Recorded test-count figures corrected in rules 9, 10 and 11 for this work order and for `master`\'s '
                '`WO-MOK-007`, neither of which corrected them: the public tier reaches 85, `render.rs` 17 internal '
                'tests and 47 private items, and the workspace 200 tests. Rule 11 instructs a work order that adds a '
                'test to correct these figures, so the correction is the rule\'s own requirement rather than a '
                'discretionary edit.',
        'marker': '**Recorded test-count figures corrected for `WO-MOK-010` and for `master`\'s `WO-MOK-007`',
        'outstanding': True,
        'body': ['| **Total** | | **85** |',
                 'the module declares 47 private items — 30 functions and 17 constants',
                 "the workspace's is **200**"],
        'state': '**Recorded, not approved — OUTSTANDING.** This artifact is not in `WO-MOK-010`\'s amendment list at '
                 'all: the obligation was found by measuring the merged tree against rule 11. Half of it is not this '
                 'branch\'s to answer for — `WO-MOK-007` reached `master` with seven tests added and rules 9, 10 and 11 '
                 'left as they were — and neither half can be stated without the other, because only the merged tree '
                 'runs both sets. **It requires the technical owner\'s ratification.**',
    },
    {
        'artifact': 'SPEC-MOK-004',
        'what': 'Rule 11\'s pointer to this work order\'s `test-census.txt` brought up to the recapture: the census '
                'now reads 179 before and 200 after against `master`\'s tip, where the sentence still described the '
                'earlier capture at `4f32a9f` reaching 190 and the recapture as something still to be taken.',
        'marker': "**Rule 11's pointer to `WO-MOK-010`'s census corrected",
        'outstanding': False,
        'body': ["was re-taken on 2026-08-19 against `master`'s tip and reads **179 before, 200 after**",
                 'capture, taken at `4f32a9f` against the branch point, reached 190'],
        'state': '**Recorded, and it ratifies nothing.** No provision is added, removed or reworded and no figure '
                 'moves: the 122, 78 and 200 are the row above\'s. The superseded 190 is kept in the text rather than '
                 'deleted, because a capture is re-run rather than corrected and a reader should be able to see which '
                 'tree each figure was taken on. The ratification this points at is the row above\'s, which is '
                 '**OUTSTANDING**.',
    },
]

# Artifacts this work order must not touch. Each is compared against `master`'s tip rather than the commit this work
# started from, because the question is whether *this branch* touched it, and one of the two moved on `master` in the
# meantime. The move is disclosed in the third column rather than hidden by the choice of base.
UNTOUCHED = [
    (f'{DOCS}/verification-records/VREC-MOK-005.md',
     'the record whose gate was overridden: this work order does not approve its amendments, does not perform its '
     'seven manual assessments and does not transition `WO-MOK-005`',
     '`master` re-captured it in 3696fae and transitioned it from `ready` to `verified` in a53712c, rebinding it from '
     'commit `9d9641fe` to `f3613701`. The transition accepted the automated evidence with all seven manual '
     'assessments outstanding and eleven provisions across four artifacts awaiting the technical owner, so the gate '
     '`WO-MOK-010` names is not met by it: the status moved and the substance did not.'),
    (f'{DOCS}/architecture/ARCH-MOK-001.md',
     'no architecture amendment is required by this work order, and none was made',
     'nothing. It is byte-identical at both, so this check is unaffected by the choice of base.'),
]

# Specifications amended beyond the stated list, so not in `AMENDED`, but still subject to the checks of section 4.
BEYOND_PATHS = [spec_path('SPEC-MOK-004')]

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


def own_rows(record):
    """The rows of `record` dated `TODAY` that name this work order.

    The merge brought rows of the same date, approved by the same owner on the same day, into three of these records.
    A search across every row of the date would let one of `master`'s rows supply the approval or the attribution for an
    amendment *this* work order made, which is the one thing section 2 exists to establish. So the rows are narrowed to
    those naming `WO-MOK-010` before anything is looked for in them, and the controls check that the narrowing is not a
    formality: `master`'s rows of this date do carry an approval marking, and none of them survives the narrowing.
    """
    return [row for row in rows(record).get(TODAY, []) if 'WO-MOK-010' in row]


def preserved(before, after):
    """The rows in `before` that no row of `after` reproduces byte for byte, as (date, row) pairs.

    A merge is the one operation that can lose a row with no edit having been made to this branch, and a lost row reads
    exactly like a row that was never written. So this is a containment check rather than a count: rows may be added
    beside `master`'s, and none of `master`'s may be altered or dropped.
    """
    lost = []
    for date, values in before.items():
        for value in values:
            if value not in after.get(date, []):
                lost.append((date, value))
    return lost


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

    # Controls on the row-preservation check of section 4, which is the one check whose subject is a merge rather than
    # an edit. A merge can lose a row without anything on this branch having touched it.
    earlier = rows(split(at(MASTER, f'{DOCS}/specifications/SPEC-MOK-003.md'))[1])
    total = sum(len(values) for values in earlier.values())
    dropped = {date: values for date, values in list(earlier.items())[1:]}
    altered = {date: [value.replace('|', '¦', 1) for value in values] for date, values in earlier.items()}
    # Controls on the row narrowing of section 2. The narrowing is only worth anything if the rows it drops could have
    # satisfied the searches, so that is checked on the record where they exist rather than assumed.
    spec3 = split(now(f'{DOCS}/specifications/SPEC-MOK-003.md'))[1]
    foreign = [row for row in rows(spec3).get(TODAY, []) if 'WO-MOK-010' not in row]
    controls += [
        (f'a row of {TODAY} that the narrowing drops carries an approval marking too, so narrowing the search to this '
         'work order\'s rows is not a formality',
         any(f'Approved {TODAY}' in row for row in foreign)),
        ('no row lacking this work order\'s identifier survives the narrowing',
         not [row for row in own_rows(spec3) if row in foreign]),
        ('a record compared against itself reports no row lost', not preserved(earlier, earlier)),
        ('a row the later record does not carry is reported lost', len(preserved(earlier, dropped)) >= 1),
        ('a row altered by one character is reported lost rather than matched loosely',
         len(preserved(earlier, altered)) == total),
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
        'Everything below is measured against one of two commits, and each section says which. Amendment rows this',
        f'branch must not have moved are compared against `{BASE[:7]}`, the commit this work started from. Artifacts it',
        f'must not have touched at all are compared against `{MASTER[:7]}`, `master`\'s tip, which this branch is merged',
        'with — because one of those artifacts moved on `master` in the meantime, and against the earlier commit the',
        'check would report `master`\'s act as this work order\'s. What `master` changed is disclosed in §4 rather than',
        f'hidden by the choice of base. Every row this work order adds is dated `{TODAY}`.',
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
    for path in [entry['path'] for entry in AMENDED] + BEYOND_PATHS:
        text = now(path)
        name = os.path.basename(path)[:-3]
        moved = field(text, 'updated') == TODAY
        if not moved:
            problems.append(f'{name} was amended but its `updated` date is not {TODAY}')
        stated = '`approved`, amended' if path in [entry['path'] for entry in AMENDED] \
            else '`approved`, amended beyond the list'
        lines.append(f'| `{name}` | `{field(text, "status")}` | {stated} | '
                     f'{field(text, "updated")} | {"yes" if moved else "**NO**"} |')

    lines += [
        '',
        'The work order is `in_progress` and not `complete`: a work order is closed by a verification record that',
        'binds a commit, and that record is written after the commit it names. `WO-MOK-006` closed the same way.',
        '',
        '`SPEC-MOK-004` is in that table although `WO-MOK-010` states no provision of it. Rule 11 of it instructs a',
        'work order that adds a test to correct the recorded counts there, and this one adds twenty-one; the correction',
        'is in §3 with the rest of what was written beyond the stated list.',
        '',
        '## 2. The amendment record against the approved list',
        '',
        f'Each provision `WO-MOK-010` states in full is looked for twice — in the amendment record\'s {TODAY} row,',
        'and in the specification\'s body — and the two searches are over disjoint text, because a record that',
        'claimed an amendment the text does not carry would otherwise satisfy both with the same sentence.',
        '',
        f'The rows read here are the {TODAY} rows **that name `WO-MOK-010`**, not every row of that date. Three of',
        'these records now carry rows the same owner approved on the same day through `master`, and a search across the',
        'whole date would let one of those vouch for an amendment this work order made. Two controls below check that',
        'the narrowing bites rather than reading as diligence: a row it drops does carry an approval marking, and no',
        'row it drops is among the rows searched. In `SPEC-MOK-003` it drops three of five — two of `master`\'s and one',
        'of this work order\'s own, a beyond-the-list row that names no work order and is checked in §3 by its own',
        'text instead.',
        '',
    ]
    for spec in AMENDED:
        body, record = split(now(spec['path']))
        name = os.path.basename(spec['path'])[:-3]
        all_dated = rows(record).get(TODAY, [])
        dated = own_rows(record)
        row_text = '\n'.join(dated)
        approved = f'Approved {TODAY}' in row_text
        names_wo = bool(dated)
        attributed = 'did not decide the substance' in row_text
        for held, complaint in ((approved, f'no approval recorded in its own {TODAY} rows'),
                                (names_wo, f'no {TODAY} row names this work order'),
                                (attributed, f'its own {TODAY} rows do not record who wrote the text')):
            if not held:
                problems.append(f'{name}: {complaint}')
        lines += [f'### `{name}` — {spec["stated"]}', '',
                  f'- rows dated {TODAY}: **{len(all_dated)}**, of which naming this work order: **{len(dated)}** — '
                  'the searches below read those alone',
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
        body, _ = split(now(spec_path(artifact)))
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
        f'{len(BEYOND)} amendments were written during implementation and are not in the list `WO-MOK-010` states. None',
        'is left to be found in a diff: each is written into the specification\'s own amendment record, and each is',
        'named here with what it still needs.',
        '',
        'Three were written before the merge. One of those is **approved**, because the owner took it as a decision',
        'under a stop condition; the other two are not, and say so. The remaining four were written after `master` was',
        'merged in, and three of them exist only because two owners\' approvals of the same date met in one tree: an',
        'approved work order drafted before the merge could not have listed them. **Two of the four require the',
        'technical owner\'s ratification and two record facts that change no provision** — which is a claim about the',
        'text, so each of those two is checked against the body rather than asserted.',
        '',
        'Whether a row is outstanding is read off the specification, not declared here: a row this file calls unratified',
        'must carry the **OUTSTANDING** marking where a reader of the specification meets it, and a run in which one did',
        'not would fail. Only that direction is checked, because two of the rows that need no ratification of their own',
        'quote the marking of a row that does, and a check that forbade the word would report those as defects.',
        '',
        f'**{sum(1 for extra in BEYOND if extra["outstanding"])} of the {len(BEYOND)} require the technical owner\'s',
        'ratification and are marked OUTSTANDING in the specification\'s own record. The other',
        f'{sum(1 for extra in BEYOND if not extra["outstanding"])} are one approved decision and two rows that change',
        'no provision.**',
        '',
    ]
    for extra in BEYOND:
        body, record = split(now(spec_path(extra['artifact'])))
        in_record = extra['marker'] in record
        missing = [phrase for phrase in extra['body'] if phrase not in body]
        if not in_record:
            problems.append(f'{extra["artifact"]}: the record does not state the amendment beyond the list')
        if missing:
            problems.append(f'{extra["artifact"]}: the body lacks {missing} for the amendment beyond the list')
        # The OUTSTANDING flag is checked against the specification's own row rather than taken from this script: a
        # row this file calls unratified must say so where a reader of the specification will meet it. Only that
        # direction is checked. A row that changes no provision may still mention the marking of another row, and one
        # of them does.
        row = next((line for lines_ in rows(record).values() for line in lines_ if extra['marker'] in line), '')
        marked = 'OUTSTANDING' in row
        if extra['outstanding'] and not marked:
            problems.append(f'{extra["artifact"]}: the row for "{extra["marker"][:40]}..." is named here as '
                            f'awaiting ratification but is not marked OUTSTANDING in the specification')
        lines += [f'**`{extra["artifact"]}`.** {extra["what"]}', '',
                  f'- in the amendment record: {"yes" if in_record else "**NO**"}',
                  f'- **OUTSTANDING** appears in its row: {"yes" if marked else "no"}'
                  + ('' if extra['outstanding']
                     else ", but not as this row's own state — it names the marking of another row"
                     if marked else ', and this row needs none'),
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

    lines += [
        '',
        '`master` advanced by ten commits while this branch sat unmerged, and four of its own amendment rows are dated',
        f'{TODAY} — the same date this work order\'s rows carry. The table above cannot see them: they did not exist at',
        f'`{BASE[:7]}`, and their date puts them outside its window. So they are checked separately, against',
        f'`{MASTER[:7]}`. Every row present there must be present here byte for byte, because a merge is the one act',
        'that can lose a row with nothing on this branch having edited it, and a lost row reads exactly like a row that',
        'was never written.',
        '',
        f'| Artifact | rows at `{MASTER[:7]}` | all preserved here byte for byte | rows added by this branch |',
        '|---|---|---|---|',
    ]
    for path in [entry['path'] for entry in AMENDED] + BEYOND_PATHS:
        name = os.path.basename(path)[:-3]
        before = rows(split(at(MASTER, path))[1])
        after = rows(split(now(path))[1])
        lost = preserved(before, after)
        for date, _ in lost:
            problems.append(f'{name}: a {date} amendment row present at {MASTER[:7]} is not preserved here')
        count_before = sum(len(value) for value in before.values())
        count_after = sum(len(value) for value in after.values())
        lines.append(f'| `{name}` | {count_before} | '
                     f'{"yes" if not lost else f"**NO**, {len(lost)} lost"} | {count_after - count_before} |')

    lines += [
        '',
        f'Two artifacts must not have been touched at all. They are compared against `{MASTER[:7]}` and not against',
        f'`{BASE[:7]}`, because the question is whether *this branch* touched them, and one of the two moved on',
        '`master` in the meantime. What `master` did to it is in the last column rather than absent from the check.',
        '',
        f'| Artifact | changed by this branch | why it must not be | what `master` did between `{BASE[:7]}` and'
        f' `{MASTER[:7]}` |',
        '|---|---|---|---|',
    ]
    for path, why, master_note in UNTOUCHED:
        name = os.path.basename(path)[:-3]
        changed = at(MASTER, path) != now(path)
        moved_on_master = at(BASE, path) != at(MASTER, path)
        if changed:
            problems.append(f'{name} changed, and this work order must not touch it')
        lines.append(f'| `{name}` | {"**YES**" if changed else "no"} | {why} | '
                     f'{"**changed.** " if moved_on_master else ""}{master_note} |')

    vrec = now(f'{DOCS}/verification-records/VREC-MOK-005.md')
    quoted = 'with all seven manual assessments outstanding and unauthored, and eleven'
    if quoted not in ' '.join(vrec.split()):
        problems.append('VREC-MOK-005 no longer carries the sentence this artifact quotes about the scope of its own '
                        'transition, so the paragraph below states it from memory rather than from the record')
    lines += [
        '',
        f'`VREC-MOK-005` is `{field(vrec, "status")}`, which is not what it was when this work began, and the change is',
        f'`master`\'s: it was `ready` at `{BASE[:7]}` and the assurance owner transitioned it on {TODAY}. That does not',
        'close the gate, and the record says so in its own words — it accepts the automated evidence at its candidate',
        f'commit "{quoted} provisions across four approved artifacts awaiting the technical owner". The status moved and',
        'the substance did not. This work order approves none of those provisions, performs none of those seven',
        'assessments, verifies nothing and does not transition `WO-MOK-005`. **The override is a cost carried forward,',
        'not a debt paid.** So the honest statement of this',
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
        'and the text, the two provisions that amend by deletion are shown to have deleted, the '
        f'{len(BEYOND)} amendments beyond the approved list are named with what each needs and '
        f'{sum(1 for extra in BEYOND if extra["outstanding"])} of them are marked OUTSTANDING where a reader of the '
        f'specification will meet them, the earlier layer is byte-identical to `{BASE[:7]}`, and every amendment row '
        f'`master` carried at `{MASTER[:7]}` survived the merge byte for byte. All {len(controls)} controls on the '
        'checks themselves held, so no result above is a check that looked for nothing. Oracle 5\'s second condition '
        'is unmet by the owner\'s recorded override, which is stated above rather than counted as a pass, and '
        '`master`\'s transition of `VREC-MOK-005` to `verified` does not meet it either: that record\'s own text says '
        'the substance stayed where it was.'
        if not problems else f'FAIL** — {len(problems)} finding(s) above.')]

    io.open(OUT, 'w', encoding='utf-8', newline='\n').write(
        '\n'.join(line.rstrip() for line in lines) + '\n')
    print('\n'.join(lines))
    print(f'\nwritten to: {OUT}')
    return 0 if not problems else 1


if __name__ == '__main__':
    sys.exit(main())
