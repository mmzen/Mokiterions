"""VER-MOK-016 oracle 7: is every amendment this change needs present, and approved by the owner it needs?

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-016/analysis/amendments.py \
        > docs/engineering/simulation/evidence/WO-MOK-016/amendment-approvals.md

The oracle's words are that absence "fails this contract regardless of code state", and the contract
adds a second static row of its own: the `VREC-MOK-005` gate, whose state "is recorded here". So this
script measures two things a reader cannot get from the code at all -- who approved the text the code
implements, and what happened to the debt this work inherited.

The failure this script exists to catch is not a missing signature. It is a work order that amends a
specification in its body without recording it, or records an amendment its body never made. Both read
as success to a reader who checks one side, so every provision is looked for **twice, in disjoint
text**: once in the amendment row that must state it, and once in the specification's body with every
amendment row removed. A record whose own prose satisfied a body check would otherwise pass.

Eight things are measured:

  1. the `status` and `updated` field of every artifact in the chain, and the `specifies` relation of
     the three amended specifications, which is what made this chain's requirements approvable;
  2. each provision `WO-MOK-016`'s *Required amendments* section states -- twenty-one across five
     artifacts -- in the row and in the body, with the row located by its own subject rather than by
     date, so that a provision recorded in the wrong row fails;
  3. the three rows this chain wrote **after** its own approval act, each of which needed an act of its
     own: two corrections to `SPEC-MOK-001` -- one of them rule 26's branch order, six provisions of
     that rule alone -- and one addition to `SPEC-MOK-002` rule 5's growth table;
  4. the text this chain **retains rather than deletes**: four sentences that an amendment inverted or
     retracted, each of which must still be present *as a quotation of a superseded position* and must
     not survive anywhere as an assertion. A phrase search cannot show that a phrase is gone, and here
     it must not be gone; what must be gone is its standing, so the paragraph holding it is located and
     the introducer that makes it a quotation is asserted in the same paragraph;
  5. what was amended beyond the approved list, and what the approved list asked for and did not get:
     `REQ-MOK-014`'s conditional floor amendment, which the measurement did not trigger, and
     `SPEC-MOK-004` rule 11's test-count figures, which are owed and outstanding;
  6. the earlier layer: every amendment row that existed at the baseline commit, in order, byte for
     byte where it is unchanged and classified where it is not -- because eight of them **did** change,
     none of them by this work order, and a check that merely failed there would report a defect where
     the truth is another work order's ratification arriving through the merge. The two cells of a row
     are classified apart, because an edited provision and a moved approval state are different events
     and only one of them may legitimately happen after an approval;
  7. what must not have changed at all, compared against `master` rather than against the baseline, and
     compared both after folding CRLF to LF and on raw bytes, so the normalization cannot hide an edit;
  8. the `VREC-MOK-005` gate: the record's status now, the eleven provisions it names, the seven manual
     assessments it names, and what `WO-MOK-012` did to each on `master`.

`self_test` injects each failure mode and asserts that it is reported, because a check that finds
nothing reads exactly like a check that looks for nothing. It runs before any result is printed.

Two commits are read, and the distinction between them is the whole of section 6:

  * `39662d13` -- the baseline this work order's captures were taken against, on this branch, before
    the change and before `master` moved;
  * `d8e2079` -- `master`'s tip, merged into this branch at `259859df`. Anything this branch must not
    have changed is compared against **that**, not against the baseline, because `master` legitimately
    changed artifacts in the interval and the question oracle 7 asks is what *this* work order did.
"""

import io
import os
import re
import subprocess
import sys

BASE = '39662d13abd08e3410648d1c59ad38384f8ad2d2'
MASTER = 'd8e207941f99ee47ae6c7f3ffeb1769f560fd4dc'
MERGE = '259859dffe1f5f856e154263c48d8d1e04808903'
DATE = '2026-08-20'
WORK_ORDER = 'WO-MOK-016'
DOCS = 'docs/engineering/simulation'

ROW = re.compile(r'^\| \d{4}-\d\d-\d\d \|')
IDENTIFIER = re.compile(r'(REQ|VER|WO|VREC|INT|CAP|SPEC|ADR|RLS)-MOK-\d{3}')
STATUS_WORDS = re.compile(r'OUTSTANDING|[Rr]atified|outstanding|awaiting|Requires the|Approved')

CHAIN = [
    ('INT-MOK-010', 'intent', 'approved', 'this chain\'s intent'),
    ('CAP-MOK-010', 'capabilities', 'approved', 'the capability every amendment below cites'),
    ('REQ-MOK-051', 'requirements', 'approved', ''),
    ('REQ-MOK-052', 'requirements', 'approved', ''),
    ('REQ-MOK-053', 'requirements', 'approved', ''),
    ('REQ-MOK-054', 'requirements', 'approved', ''),
    ('REQ-MOK-055', 'requirements', 'approved', ''),
    ('REQ-MOK-056', 'requirements', 'approved', 'amended in this chain, twice'),
    ('REQ-MOK-057', 'requirements', 'approved', 'amended in this chain'),
    ('REQ-MOK-058', 'requirements', 'approved', 'amended in this chain'),
    ('REQ-MOK-059', 'requirements', 'approved', ''),
    ('REQ-MOK-060', 'requirements', 'approved', 'descoped 2026-08-21 to `WO-MOK-017`, unamended'),
    ('VER-MOK-016', 'verification', 'approved', 'amended in this chain, three rows'),
    ('WO-MOK-016', 'work-orders', 'implemented', 'this work order'),
    ('WO-MOK-017', 'work-orders', 'draft', 'carries `REQ-MOK-060`'),
    ('SPEC-MOK-001', 'specifications', 'approved', 'amended, three rows'),
    ('SPEC-MOK-002', 'specifications', 'approved', 'amended, two rows'),
    ('SPEC-MOK-003', 'specifications', 'approved', 'amended, one row and one reconciliation'),
    ('SPEC-MOK-004', 'specifications', 'approved', 'not amended; rule 11 figures owed, section 5'),
    ('SPEC-MOK-005', 'specifications', 'approved', 'not amended'),
    ('REQ-MOK-005', 'requirements', 'approved', 'amended, one row'),
    ('REQ-MOK-014', 'requirements', 'approved', 'not amended; the floor did not move, section 5'),
    ('REQ-MOK-034', 'requirements', 'approved', 'amended, one row'),
    ('ARCH-MOK-001', 'architecture', 'approved', 'untouched'),
    ('ARCH-MOK-002', 'architecture', 'approved', 'untouched'),
    ('VREC-MOK-005', 'verification-records', 'verified', 'the gate, section 8'),
]

# The `specifies` relation each amended specification gains. Without these entries `validate` raises
# E007 on every requirement in this chain, which is why the amendment act had to be single.
SPECIFIES = {
    'SPEC-MOK-001': ['REQ-MOK-051', 'REQ-MOK-052', 'REQ-MOK-053', 'REQ-MOK-054', 'REQ-MOK-055',
                     'REQ-MOK-056', 'REQ-MOK-057', 'REQ-MOK-058', 'REQ-MOK-059', 'REQ-MOK-060'],
    'SPEC-MOK-002': ['REQ-MOK-052', 'REQ-MOK-053', 'REQ-MOK-054', 'REQ-MOK-055', 'REQ-MOK-056',
                     'REQ-MOK-057'],
    'SPEC-MOK-003': ['REQ-MOK-052', 'REQ-MOK-053', 'REQ-MOK-055', 'REQ-MOK-056', 'REQ-MOK-057'],
}

# Every amendment row this chain writes, located by a phrase of its own subject rather than by date:
# eleven rows share the date 2026-08-20 in `SPEC-MOK-003` alone, and a provision recorded in the wrong
# row is a defect this check must report rather than absorb.
#
# `provisions` are (label, phrases that must be in this row's operative cell, phrases that must be in
# the artifact's body with every amendment row removed). The two lists are deliberately different
# strings: a row that merely quoted the body would not satisfy the row check, and the body is searched
# with the rows gone.
ROWS = [
    dict(
        artifact='SPEC-MOK-001', folder='specifications',
        key='Contact, conflict and society, under `CAP-MOK-010`',
        title='the `CAP-MOK-010` amendment: thirteen provisions, of which seven are appended rules',
        role='technical owner', act='single',
        count='Thirteen provisions amended, of which seven are appended rules',
        provisions=[
            ('1. *Scope* names the capability and drops the interaction exclusion',
             ['*Scope* names `CAP-MOK-010` and no longer excludes interaction between Mokiterions',
              'cooperation, memory of encounters and perceived relative strength stay excluded'],
             ['contact and conflict between Mokiterions',
              'What remains undefined here is a boundary of `CAP-MOK-010` rather than of this specification']),
            ('2. *Actors* names four decision sources',
             ['*Actors*, *Inputs* and *Help output* name a fourth decision source'],
             ['Four exist: the random baseline source',
              'the social source of rule 26, which is the only one that reads `fear`']),
            ('3. *Inputs* and *Help output*: `--policy` takes a fourth value, the default stays',
             ['with `reference` still the default'],
             ['[--policy <baseline|reference|individual|social>]',
              'Only `baseline`, `reference`, `individual`, and `social` are valid values',
              'The explanatory prose on the decision sources describes all four']),
            ('4. *State model / Mokiterion*: no attribute, one item of transient state',
             ['*State model / Mokiterion* gains **no attribute** and one item of transient state',
              'the record of attacks suffered since that Mokiterion'],
             ['one item of **transient state**',
              'It is not a bounded `0..=100` attribute and is not reported as one',
              'It is per-Mokiterion and never per-pair']),
            ('5. A *Contact* subsection fixes the relation and its radius',
             ['a new *Contact* subsection fixes the contact relation at Chebyshev distance `1`',
              'recomputed from positions and never stored'],
             ['### Contact',
              'The contact radius is `1` cell',
              'The distance is the same Chebyshev distance *Perception* defines',
              'Contact is recomputed from current positions and never stored',
              'two Mokiterions in contact still do not block each other']),
            ('6. *Data and interface contracts*: the eleven kinds, three event types, the trace shape',
             ['*Data and interface contracts* fixes the eleven-kind action contract, the three added '
              'event types with their field lists, and the `action_trace` line',
              'conditional `suffered` field'],
             ['The vocabulary gains three types and no more',
              'event=attack_resolved result=target:<mokiterion-id>,damage:<number>',
              'event=threat_resolved result=target:<mokiterion-id>,increase:<number>',
              'event=surrender_resolved result=recipient:<mokiterion-id>,transferred:<number>',
              'Each of these records carries the transitions the resolution caused, in both directions',
              'A targeted proposal reports its target in a `target` field of its own',
              'The `suffered` field is appended after `fear`, and it is present only when the '
              'suffered-attack record is non-empty',
              'The seven targeted forms all name the other Mokiterion in one field called `target`']),
            ('7. Rule 3 carries `fear` and the record; its valid-proposal list is untouched',
             ['Rule 3 carries `fear` and the suffered-attack record and its valid-proposal list is '
              '**untouched**',
              'so rule 6 becomes the complete statement of what may be proposed'],
             ['`fear` and the suffered-attack record are carried, and this replaces the refusal',
              'The list of currently valid proposals is unchanged, and no targeted action ever appears in it',
              '`REQ-MOK-054` now obli']),
            ("8. *Name*'s justification is replaced, and the name stays off the observation",
             ["*Name*'s justification for a name's absence from the observation is replaced",
              'the name stays off the observation'],
             ['This reason once ran through `fear` and no longer can',
              '`REQ-MOK-041` obliges that nothing reads a name',
              'The name stays off the observation, and what changed is the argument for it']),
            ("9. Rule 5's accumulation and rule 19's tolerance state the ceiling as an obligation",
             ["Rule 5's accumulation paragraph and rule 19's tolerance test now state "
              "`REQ-MOK-060`'s ceiling as an obligation where they stated none",
              'name the two places the correction may be made',
              'The measured 45 of 61 figure is **retained**'],
             ['Where the correction is made, and where it may not be',
              'The numeric form of the corrected condition is a later amendment to this specification',
              'decided on measurement under `WO-MOK-016`']),
            ('10. Rule 6 extends validation to targeted proposals',
             ["Rule 6 extends validation to targeted proposals against the target's authoritative state"],
             ['For a targeted proposal the engine validates against the target',
              'a rejected targeted proposal mutates **neither** Mokiterion',
              'This rule is the complete statement of what may be proposed',
              'the first unmet condition is the rejection reason']),
            ('11. Rule 7 fixes the trace-before-clearing order',
             ['rule 7 fixes the trace-before-clearing order'],
             ['Targeted actions are traced on the same terms as core ones',
              'it reports it before the record is cleared',
              "The record's clearing is positioned identically whether or not the flag is set"]),
            ("12. Rule 12's closing sentence is inverted and the composition stated",
             ['rule 12\'s closing "No rule reads `fear`" is **inverted** and the composition of its '
              'two writers stated'],
             ['**`fear` is read.**',
              '`fear` now has two writers within one tick, and their composition is stated',
              'in whichever order rule 2']),
            ('13. Rule 13 states that combat death uses the existing path, event and finality',
             ['rule 13 states that combat death uses its existing path, event and finality'],
             ["`health` may now reach zero through rule 22's damage",
              'Death stays one concept',
              'A dead Mokiterion is also no longer a valid target']),
            ('14. Rules 20 to 26 appended after rule 19, each with its position in tick order',
             ['**Rules 20 to 26 are appended after rule 19 and not inserted**',
              "on `WO-MOK-010`'s precedent and for its stated reason",
              'contact, targeted actions, combat resolution, threat, surrender, the suffered-attack '
              'window'],
             ['Rules 1 through 18 are stated in tick order. **Rules 19 through 26 are not**',
              '| 20 | Contact | No position',
              '| 22 | Combat resolution | Inside rule 21',
              '| 25 | The suffered-attack window | Twice',
              '20. **Contact.**',
              '21. **Targeted actions.**',
              '22. **Combat resolution.**',
              '23. **Threat.**',
              '24. **Surrender.**',
              '25. **The suffered-attack window.**',
              '26. **The `social` decision source.**',
              'The action contract is closed at eleven kinds',
              'Damage is `10 + (striker.energy + striker.health) / 10`',
              "The striker pays a flat `5` `energy`",
              "The target's `fear` rises by `30`, saturating at `ATTRIBUTE_MAX`",
              'forfeits `satiety / 2` of its own `satiety`',
              'A surrender below `satiety` `2` transfers `0` and still succeeds',
              'The latency this produces is asymmetric and is stated rather than corrected',
              'No selection among Mokiterions happens in this rule']),
        ],
    ),
    dict(
        artifact='SPEC-MOK-002', folder='specifications',
        key="Rule 5's enumeration amended and rule 6 re-checked, under `CAP-MOK-010`",
        title="the `CAP-MOK-010` amendment: rule 5's enumeration, rule 6 re-checked",
        role='technical owner', act='single',
        provisions=[
            ("1. Rule 5's enumeration gains the variants, and `EventType::ALL` moves from 12 to 15",
             ['`simulation::Policy` gains a fourth variant, `Social`',
              '`simulation::Action` gains seven target-carrying variants',
              "`EventType::ALL`'s length moves from `12` to `15`",
              'which is public-surface growth because that array is a `pub const`',
              'The observation\'s two new fields are not interface growth'],
             ['| `simulation::Policy` | one variant, `Social`',
              '`simulation::Action` | seven variants',
              'so `ALL` goes from twelve entries to fifteen',
              'What does not grow is part of the enumeration',
              "The observation's two new fields are not interface growth, and the distinction is "
              'load-bearing']),
            ('2. Rule 6 re-checked and recorded as not amended, cross-agent mutation being new',
             ['Rule 6 is re-checked and recorded as **not amended**',
              'a target is an identifier and not a reference',
              'no `pub(crate)` is widened'],
             ['Re-checked 2026-08-20 under `CAP-MOK-010` and not amended',
              'an action by one Mokiterion that mutates another',
              'stay prohibited and stay private']),
        ],
    ),
    dict(
        artifact='SPEC-MOK-003', folder='specifications',
        key='Three provisions amended under `CAP-MOK-010`',
        title='the `CAP-MOK-010` amendment: three provisions',
        role='technical owner', act='single',
        provisions=[
            ("1. Rule 11's authority table gains three rows, and `REQ-MOK-052` takes none",
             ["**Rule 11's** authority table gains three rows, one per added event type",
              '`attack_resolved` to `REQ-MOK-053`',
              '**`REQ-MOK-052` takes no row**'],
             ['| `attack_resolved` | `REQ-MOK-053` |',
              '| `threat_resolved` | `REQ-MOK-055` |',
              '| `surrender_resolved` | `REQ-MOK-056` |',
              'Amended 2026-08-20: three rows added, and `REQ-MOK-052` takes none']),
            ("2. Rule 4's roster and rule 10's inspector present a targeted action's subject",
             ["**Rule 4's** roster and **rule 10's** inspector present a targeted action's subject as "
              'well as its verb',
              'by identifier and never by name'],
             ['Amended 2026-08-20 under `REQ-MOK-052`: the applied action carries its subject',
              'as `attack M03`, `threaten M07`, `surrender M02` and so on for all seven verbs',
              'Amended 2026-08-20 under `REQ-MOK-052`: a target may be a Mokiterion']),
            ("3. Clause 5's refusal of inert values is satisfied a second way for `fear`",
             ["**Rule 4 clause 5's** refusal of inert values is unchanged and is now satisfied "
              'differently for `fear`, which has a reader'],
             ['the ground for filling the slot is now stronger than computation',
              '`fear` is not inert in any sense',
              'That reasoning is\n   retained here rather than deleted']),
        ],
    ),
    dict(
        artifact='REQ-MOK-005', folder='requirements',
        key='The four-verb enumeration is re-read as the **core** set',
        title='the core-set re-reading',
        role='product owner', act='single',
        provisions=[
            ('1. The four verbs are the core set, beside which `REQ-MOK-052` places seven',
             ['beside which `REQ-MOK-052` places seven targeted verbs',
              '**Nothing this requirement obliges changes**'],
             ['These four are the core set, and they are not the whole action contract',
              'Exactly one action is applied for the decision opportunity',
              'its title has always said *core*']),
        ],
    ),
    dict(
        artifact='REQ-MOK-034', folder='requirements',
        key='Narrowed the frozen-outcome constraint',
        title='the frozen-outcome narrowing',
        role='product owner', act='same act, earlier than its ordering required',
        provisions=[
            ('1. The frozen clause is narrowed to `baseline` alone',
             ['from "the reference or baseline source" to `baseline` alone',
              '**The floor of eight of twelve is not touched by this row**'],
             ['Meeting this floor must not be achieved by any change to the baseline source',
              'Narrowed 2026-08-20 under `REQ-MOK-060`',
              'and it named one source too many']),
        ],
    ),
]

# The three amendment rows taken **after** the single act above. Each stands on its own approval, on a
# ground its own status cell names, and each is stated in `WO-MOK-016`'s *Required amendments* section
# as a correction to that section rather than only to the specification. `status` is what that cell must
# carry: the ground, the role, and who decided the substance.
LATER = [
    dict(
        artifact='SPEC-MOK-001', folder='specifications',
        key="Rule 21's co-location fallback and rule 6's paragraph on it are corrected",
        title="rule 21 and rule 6: the co-located fallback is `avoid`'s **and** `retreat`'s",
        status=['Approved 2026-08-20 by the repository owner acting as technical owner',
                'on the discrepancy being put to them with the alternatives',
                'Both were declined',
                'The implementation agent found the discrepancy',
                'did not decide it'],
        provisions=[
            ('Both verbs are named where one was, and consequence 4 is corrected with them',
             ['name **both `avoid` and `retreat`**',
              'This is a correction of an incomplete enumeration rather than of an obligation'],
             ['`avoid` and `retreat` against a co-located target move north, and where north is '
              'invalid',
              '**Amended 2026-08-20**: this paragraph named `avoid` alone',
              'differing from `approach` only in sign']),
        ],
    ),
    dict(
        artifact='SPEC-MOK-001', folder='specifications',
        key="Rule 26's branch order and engagement threshold",
        title="rule 26: a branch hoisted, the engagement gate at `95`",
        status=['Approved 2026-08-20 by the repository owner acting as product owner and technical '
                'owner',
                'on the measured evidence in `evidence/WO-MOK-016/escalation.md`',
                'Seventeen variants were measured across three levers',
                'leaves rule 12 as Phase 2 approved it'],
        provisions=[
            ("Rule 19's case 3 becomes branch 3 and the gate moves from `30` to `95`",
             ['**Six provisions of that rule alone; no other rule is'],
             ['3. **Food perceived outranks company perceived.**',
              "propose `attack` while the actor's own `fear` is below `95`",
              "propose `approach` while the actor's own `fear` is below `95`",
              'Amended 2026-08-20, under `REQ-MOK-057`',
              'Three constants and no more',
              'What the engagement threshold of `95` means, and what it gives up']),
        ],
    ),
    dict(
        artifact='SPEC-MOK-002', folder='specifications',
        key="Rule 5's growth table gains a fourth row",
        title="rule 5: a field appended to a public variant that already existed",
        status=['Approved 2026-08-20 by the repository owner acting as technical owner, in a '
                '**separate act** from the amendment above',
                'the omission having been found after that act was taken',
                'it did not decide the substance',
                "It is stated in full in `WO-MOK-016`'s *Required amendments* section as provision 3"],
        provisions=[
            ('`EventDetail::ActionTrace` gains `suffered`, and the growth becomes 1 + 7 + 3 + 3 + 1',
             ['`suffered: Vec<(String, u8)>`, appended after `fear`',
              '`1 + 7 + 3 + 3 + 1`',
              '**no type is added and rule 6\'s ten private names are untouched**'],
             ['| `simulation::EventDetail`, the **existing** `ActionTrace` variant | one field, '
              '`suffered: Vec<(String, u8)>`, appended after `fear` | 1 |',
              'Four items on the lists above change shape',
              'the one form of growth an enumeration of added variants does not catch']),
        ],
    ),
]

# Text this chain retains rather than deletes. Each entry names the superseded sentence and the
# introducers that make an occurrence of it a quotation of a position rather than the position itself.
# **Every** paragraph holding the sentence must carry one of them: the count is not what matters, since
# a sentence may honestly be quoted twice, but an occurrence in a paragraph that introduces it as
# nothing is the retracted claim still standing.
RETAINED = [
    ('SPEC-MOK-001', 'specifications',
     "rule 3's refusal to carry `fear`",
     '`fear` is deliberately **not** carried',
     ['The withheld form read']),
    ('SPEC-MOK-001', 'specifications',
     "rule 12's closing sentence",
     '**No rule reads `fear`.**',
     ['For two phases this paragraph closed']),
    ('SPEC-MOK-001', 'specifications',
     "rule 5's refusal to state an obligation",
     'No obligation is stated on the result in either direction',
     ['is retracted rather than dropped']),
    ('SPEC-MOK-003', 'specifications',
     "clause 5's reason for reserving the fourth slot",
     'would be a claim the engine cannot support',
     ['retained here rather than deleted', 'against the earlier position that']),
]

# Artifacts this work order must not have changed, compared against `master`'s tip rather than against
# the baseline: `master` moved in the interval and the question is what this branch did.
UNCHANGED = [
    ('specifications/SPEC-MOK-004.md',
     'the test-census authority. Rule 11 obliges a figure correction this work order owes and has not '
     'made; section 5 measures it'),
    ('specifications/SPEC-MOK-005.md', 'the release specification; no amendment is required'),
    ('requirements/REQ-MOK-014.md',
     "the default source's survivor floor. Its amendment was conditional on a measurement that did not "
     'move it; section 5'),
    ('architecture/ARCH-MOK-001.md', 'no architecture amendment is required by this work order'),
    ('architecture/ARCH-MOK-002.md', 'the same'),
]

VRECS = ['VREC-MOK-001', 'VREC-MOK-002', 'VREC-MOK-003', 'VREC-MOK-004', 'VREC-MOK-005',
         'VREC-MOK-006', 'VREC-MOK-007', 'VREC-MOK-008', 'VREC-MOK-009', 'VREC-MOK-010',
         'VREC-MOK-011', 'VREC-MOK-013', 'VREC-MOK-014', 'VREC-MOK-015']

# The gate. `VREC-MOK-005` transitioned to `verified` with these two debts open, and `VER-MOK-016`
# requires their state to be recorded here rather than asserted anywhere.
GATE_ARTIFACTS = [
    ('specifications/SPEC-MOK-002.md', 4, 'four provisions, row dated 2026-08-18'),
    ('specifications/SPEC-MOK-003.md', 1, 'one provision, row dated 2026-08-18'),
    ('specifications/SPEC-MOK-004.md', 5, 'five provisions in one row, dated 2026-08-19'),
    ('architecture/ARCH-MOK-001.md', 1, 'one provision, row dated 2026-08-18'),
]


def read(path):
    """The file's text with line endings normalized.

    This clone has `core.autocrlf = true`, so a tracked governance file is CRLF in the working tree and
    LF in the blob `git show` prints. Every comparison here is therefore of content after newline
    normalization; `raw_differs_only_by_newlines` is what checks that the distinction is the whole
    difference, so the normalization cannot hide an edit.
    """
    return io.open(path, encoding='utf-8', newline='').read().replace('\r\n', '\n')


def at(ref, path):
    out = subprocess.run(['git', 'show', f'{ref}:{path}'], capture_output=True)
    if out.returncode:
        return None
    return out.stdout.decode('utf-8').replace('\r\n', '\n')


def raw_differs_only_by_newlines(ref, path):
    raw = io.open(path, 'rb').read()
    blob = subprocess.run(['git', 'show', f'{ref}:{path}'], capture_output=True,
                          check=True).stdout
    return raw != blob and raw.replace(b'\r\n', b'\n') == blob.replace(b'\r\n', b'\n')


def field(text, name):
    match = re.search(r'^%s = "([^"]*)"' % name, text, re.M)
    return match.group(1) if match else '<absent>'


def specifies(text):
    match = re.search(r'^specifies = \[(.*?)\]', text, re.M | re.S)
    return re.findall(r'"([^"]+)"', match.group(1)) if match else []


def amendment_rows(text):
    """Every row of the amendment record, in order, as written."""
    return [line for line in text.split('\n') if ROW.match(line)]


def cells(row):
    return [cell.strip() for cell in row.split('|')][1:-1]


def is_outstanding(row):
    """True when the row's own status cell -- the last -- declares it outstanding.

    A row that merely says which earlier rows are untouched mentions the word in its middle cell, and
    must not be counted as outstanding itself.
    """
    parts = cells(row)
    return len(parts) >= 2 and parts[-1].startswith('**OUTSTANDING')


def body_without_record(text):
    """The artifact with every amendment row removed, so the record cannot satisfy a body check."""
    return '\n'.join(line for line in text.split('\n') if not ROW.match(line))


def locate(text, key):
    """The one amendment row whose operative cell carries this subject phrase."""
    found = [row for row in amendment_rows(text) if key in cells(row)[1]]
    return found[0] if len(found) == 1 else None


def check_provisions(text, row, provisions):
    """For each provision: is it in this row's operative cell, and in the body with the rows gone?"""
    record = cells(row)[1] if row else ''
    body = body_without_record(text)
    results = []
    for label, in_record, in_body in provisions:
        results.append({
            'label': label,
            'record': (sum(p in record for p in in_record), len(in_record)),
            'body': (sum(p in body for p in in_body), len(in_body)),
            'missing_record': [p for p in in_record if p not in record],
            'missing_body': [p for p in in_body if p not in body],
        })
    return results


def check_retention(text, quoted, introducers):
    """The superseded sentence is present, and every paragraph holding it introduces it as superseded.

    Not a count. A sentence may honestly be quoted twice -- once by the amendment that retracted it and
    once by a later amendment that names the position it replaced. What may not exist is an occurrence
    in a paragraph that introduces it as nothing, because that is the retracted claim still asserted.
    """
    body = body_without_record(text)
    paragraphs = [p for p in re.split(r'\n\s*\n', body) if quoted in p]
    return {
        'present': bool(paragraphs),
        'occurrences': body.count(quoted),
        'paragraphs': len(paragraphs),
        'unquoted': sum(1 for p in paragraphs
                        if not any(intro in p for intro in introducers)),
    }


def mask(text):
    return IDENTIFIER.sub(lambda m: m.group(1) + '-MOK-XXX', text)


def strip_status_sentences(text):
    """The cell with every sentence that talks about an approval state removed.

    Eight rows of the earlier layer differ between the baseline and this candidate, and in every case
    the difference is a status: `master` ratified them under `WO-MOK-012` in the interval. Removing the
    sentences that discuss a status is what lets the check ask the question it means to ask -- did the
    *provision* change -- instead of failing on another work order's act.
    """
    return ' '.join(s for s in re.split(r'(?<=[.;])\s+', text) if not STATUS_WORDS.search(s))


def classify(base_row, head_rows):
    """How a baseline amendment row appears at the candidate.

    The two cells are classified separately, because they answer different questions. A changed
    operative cell would mean an approved provision was edited after the fact. A changed status cell
    means an approval state moved, which is exactly what a ratification is and is legitimate when the
    act is named. Conflating them would report a defect where the truth is a debt being paid.
    """
    if base_row in head_rows:
        return 'identical', None
    index = {}
    for row in head_rows:
        index.setdefault(mask(strip_status_sentences(cells(row)[1])), []).append(row)
    match = index.get(mask(strip_status_sentences(cells(base_row)[1])))
    if not match:
        return 'absent', None
    before, after = cells(base_row), cells(match[0])
    if mask(before[1]) != mask(after[1]):
        return 'operative cell differs', match[0]
    if before[1] != after[1]:
        if mask(before[-1]) == mask(after[-1]):
            return "renumbered, and nothing else", match[0]
        return "renumbered, and the status moved", match[0]
    return 'status only', match[0]


def self_test():
    """Inject each failure mode and assert it is reported. Returns a list of (control, ok)."""
    controls = []

    good = ('| 2026-08-20 | states the provision | Approved 2026-08-20. |\n'
            'the body carries the provision\n')
    row = locate(good, 'states the provision')
    result = check_provisions(good, row, [('c', ['states the provision'], ['body carries'])])[0]
    controls.append(('a provision stated in its row and carried by the body reads clean',
                     result['record'] == (1, 1) and result['body'] == (1, 1)))

    silent = '| 2026-08-20 | says nothing of it | Approved. |\nthe body carries the provision\n'
    row = locate(silent, 'says nothing of it')
    result = check_provisions(silent, row, [('c', ['states the provision'], ['body carries'])])[0]
    controls.append(('a provision its row does not state is reported', result['record'] == (0, 1)))

    absent = '| 2026-08-20 | states the provision | Approved. |\nan unrelated body\n'
    row = locate(absent, 'states the provision')
    result = check_provisions(absent, row, [('c', ['states the provision'], ['body carries'])])[0]
    controls.append(('a provision absent from the body is reported', result['body'] == (0, 1)))

    only_row = '| 2026-08-20 | states the provision and body carries it | Approved. |\n'
    row = locate(only_row, 'states the provision')
    result = check_provisions(only_row, row, [('c', ['states the provision'], ['body carries'])])[0]
    controls.append(("a row's own prose cannot satisfy a body check", result['body'] == (0, 1)))

    two = ('| 2026-08-20 | states the provision | Approved. |\n'
           '| 2026-08-20 | states the provision | Approved. |\n')
    controls.append(('a subject phrase matching two rows locates neither, rather than the first',
                     locate(two, 'states the provision') is None))

    wrong_row = ('| 2026-08-20 | the subject of this amendment | Approved. |\n'
                 '| 2026-08-20 | another amendment, which states the provision | Approved. |\n'
                 'the body carries the provision\n')
    row = locate(wrong_row, 'the subject of this amendment')
    result = check_provisions(wrong_row, row, [('c', ['states the provision'], ['body carries'])])[0]
    controls.append(('a provision recorded in the wrong row is reported, not credited',
                     result['record'] == (0, 1)))

    intro = ['The withheld form read']
    kept = 'The withheld form read "the old sentence", and it was correct while that held.\n'
    controls.append(('a superseded sentence quoted by its introducer reads clean',
                     check_retention(kept, 'the old sentence', intro)
                     == {'present': True, 'occurrences': 1, 'paragraphs': 1, 'unquoted': 0}))

    unquoted = 'the old sentence\n\nThe withheld form read something else entirely.\n'
    result = check_retention(unquoted, 'the old sentence', intro)
    controls.append(('a superseded sentence standing in a paragraph of its own is reported',
                     result['present'] and result['unquoted'] == 1))

    twice = ('The withheld form read "the old sentence".\n\n'
             'A rule elsewhere still says the old sentence.\n')
    result = check_retention(twice, 'the old sentence', intro)
    controls.append(('a second copy in a paragraph that introduces it as nothing is reported',
                     result['occurrences'] == 2 and result['unquoted'] == 1))

    both = ('The withheld form read "the old sentence".\n\n'
            'A later row moved against the earlier position that "the old sentence" held.\n')
    result = check_retention(both, 'the old sentence',
                             intro + ['against the earlier position that'])
    controls.append(('a sentence quoted twice, each time as superseded, is not reported',
                     result['paragraphs'] == 2 and result['unquoted'] == 0))

    gone = 'nothing here quotes anything\n'
    result = check_retention(gone, 'the old sentence', intro)
    controls.append(('a retained sentence that was deleted instead is reported',
                     result['present'] is False))

    rows = amendment_rows('| 2026-08-18 | a | x |\n| 2026-08-19 | b | y |\nprose\n')
    controls.append(('the row reader finds rows in order and ignores prose',
                     len(rows) == 2 and rows[0].startswith('| 2026-08-18')))

    own = '| 2026-08-19 | amended something | **OUTSTANDING.** Requires the technical owner. |'
    other = '| 2026-08-19 | the row above marked **OUTSTANDING** is untouched | Approved. |'
    controls.append(('a row outstanding in its own status cell is told from one that mentions the word',
                     is_outstanding(own) and not is_outstanding(other)))

    before = '| 2026-08-18 | a provision, under `CAP-MOK-009` | **OUTSTANDING.** Requires the owner. |'
    ratified = '| 2026-08-18 | a provision, under `CAP-MOK-009` | **Ratified** 2026-08-20. |'
    controls.append(('a row ratified in place, its provision untouched, is classified as a status '
                     'change and not as a provision change',
                     classify(before, [ratified])[0] == 'status only'))

    renumbered = '| 2026-08-18 | a provision, under `CAP-MOK-010` | **OUTSTANDING.** Requires the owner. |'
    controls.append(("a row carrying this chain's renumber and no other change is classified as that",
                     classify(before, [renumbered])[0] == 'renumbered, and nothing else'))

    both_moved = '| 2026-08-18 | a provision, under `CAP-MOK-010` | **Ratified** 2026-08-20. |'
    controls.append(('a row both renumbered and ratified is reported as both, not as one',
                     classify(before, [both_moved])[0] == 'renumbered, and the status moved'))

    edited = '| 2026-08-18 | a different provision entirely | **OUTSTANDING.** Requires the owner. |'
    controls.append(('a row whose operative cell was rewritten is reported as absent, not absorbed',
                     classify(before, [edited])[0] == 'absent'))

    reworded = ('| 2026-08-18 | a provision, reworded, under `CAP-MOK-009` '
                '| **OUTSTANDING.** Requires the owner. |')
    controls.append(('a row whose operative cell was reworded within the same status is reported',
                     classify(before, [reworded])[0] in ('operative cell differs', 'absent')))

    controls.append(('an unchanged row is reported as identical rather than as a change',
                     classify(before, [before])[0] == 'identical'))

    return controls


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    controls = self_test()
    if not all(ok for _, ok in controls):
        for label, ok in controls:
            print(('ok   ' if ok else 'FAIL ') + label, file=sys.stderr)
        return 2

    out = []
    add = out.append
    verdicts = {}

    add('# Amendment approvals: `VER-MOK-016` oracle 7, and the `VREC-MOK-005` gate')
    add('')
    add('| Field | Value |')
    add('|---|---|')
    add('| Contract | `VER-MOK-016`, oracle 7 — "the `SPEC-MOK-001`, `SPEC-MOK-002` and `SPEC-MOK-003` '
        'amendments and the `REQ-MOK-005`, `REQ-MOK-014` and `REQ-MOK-034` amendments `WO-MOK-016` '
        'names are approved. Absence fails this contract regardless of code state" |')
    add('| Second row | "The `VREC-MOK-005` gate — the row `VREC-MOK-010` records as outstanding is a '
        'precondition of this work, not an output of it, and its state is recorded here" |')
    add('| Reader | `analysis/amendments.py`, over the artifacts and the git history |')
    add('| Invocation | `python docs/engineering/simulation/evidence/WO-MOK-016/analysis/amendments.py '
        '> docs/engineering/simulation/evidence/WO-MOK-016/amendment-approvals.md` |')
    add(f'| Baseline | `{BASE[:8]}` — this branch before the change, and before `master` moved |')
    add(f'| Master | `{MASTER[:8]}` — `master`\'s tip, merged into this branch at `{MERGE[:8]}` |')
    add('| Date | 2026-08-21 |')
    add('')
    add('This file is generated. What it checks, and why each check is not a formality, is in the '
        "script's header; the short version is that every provision is looked for **twice, in "
        'disjoint text** — in the amendment row that must state it, and in the artifact\'s body with '
        'every amendment row removed — because a record that claimed an amendment the text does not '
        'carry would otherwise satisfy both halves with one sentence.')
    add('')
    add('It **signs nothing**. Where a decision is owed it is named as owed. Oracle 7 is a check on '
        "acts already taken, and this file's authority is that of a measurement.")
    add('')
    add('---')
    add('')

    # ---------------------------------------------------------------- 1
    add("## 1. The chain's governance state")
    add('')
    add('| Artifact | status | expected | updated | ok | note |')
    add('|---|---|---|---|---|---|')
    chain_ok = True
    for artifact, folder, expected, note in CHAIN:
        text = read(f'{DOCS}/{folder}/{artifact}.md')
        status, updated = field(text, 'status'), field(text, 'updated')
        good = status == expected
        chain_ok = chain_ok and good
        add('| `%s` | `%s` | `%s` | %s | %s | %s |' % (
            artifact, status, expected, updated, 'yes' if good else '**no**', note))
    verdicts['the chain carries the statuses it should'] = chain_ok
    add('')
    add('`WO-MOK-016` is `implemented` and not `complete`: a work order is closed by a verification '
        'record that binds a commit, and that record is written after the commit it names. '
        '`WO-MOK-017` is `draft` because `REQ-MOK-060` was carried into it on 2026-08-21 and a work '
        'order leaves `draft` only on the owner act its own scope requires.')
    add('')
    add('### The `specifies` relation, which is what made this chain approvable')
    add('')
    add('| Specification | requirements it must specify | present | gained since `master` |')
    add('|---|---|---|---|')
    specifies_ok = True
    for artifact, required in SPECIFIES.items():
        path = f'{DOCS}/specifications/{artifact}.md'
        now, before = specifies(read(path)), specifies(at(MASTER, path))
        missing = [r for r in required if r not in now]
        gained = [r for r in now if r not in before]
        specifies_ok = specifies_ok and not missing
        add('| `%s` | %d | %s | %d: %s |' % (
            artifact, len(required),
            '%d of %d' % (len(required) - len(missing), len(required)) if not missing
            else '**missing %s**' % ', '.join(missing),
            len(gained), ', '.join('`%s`' % r for r in gained) or 'none'))
    verdicts['every amended specification specifies the requirements it must'] = specifies_ok
    add('')
    add("Without those entries `validate` raises `E007` on every requirement in this chain and "
        '`preflight --phase start` raises `W016`, which is why the amendment act had to be single: the '
        'relation is what made ten requirements approvable at all, so it could not follow their '
        'approval.')
    add('')

    # ---------------------------------------------------------------- 2
    add('## 2. The amendment record against the approved list')
    add('')
    add('The %d subjects `WO-MOK-016`\'s *Required amendments* section names across five artifacts, '
        'each checked in the row that must state it and in the body with every row removed. **The row '
        'is located by its own subject, not by its date**: eleven rows in `SPEC-MOK-003` share '
        '2026-08-20, and a provision recorded in the wrong row is a defect this table reports rather '
        'than absorbs.' % sum(len(spec['provisions']) for spec in ROWS))
    add('')
    provisions_ok = True
    for spec in ROWS:
        path = f'{DOCS}/{spec["folder"]}/{spec["artifact"]}.md'
        text = read(path)
        row = locate(text, spec['key'])
        add('### `%s` — %s' % (spec['artifact'], spec['title']))
        add('')
        if row is None:
            provisions_ok = False
            add('- **the row is not located by its subject phrase.** Every check below fails with it.')
            add('')
            continue
        status = cells(row)[-1]
        approved = 'Approved %s by the repository owner acting as %s' % (DATE, spec['role'])
        wrote = 'wrote the text and did not decide the substance' in status
        add('- rows in this artifact dated %s: **%d**, of which this one is located by its subject'
            % (DATE, len([r for r in amendment_rows(text) if r.startswith('| ' + DATE + ' |')])))
        add('- approval recorded in the row: %s' % ('yes — "%s"' % approved if approved in status
                                                    else '**no**'))
        add('- records that the implementation agent wrote the text and decided no substance: %s'
            % ('yes' if wrote else '**no**'))
        if approved not in status or not wrote:
            provisions_ok = False
        if spec.get('count'):
            present = spec['count'] in cells(row)[1]
            provisions_ok = provisions_ok and present
            add('- the row states its own count: %s'
                % ('"%s"' % spec['count'] if present else '**absent**'))
            add('- subjects checked below: **%d**, which cuts some of that count\'s sentences finer '
                'and its appended-rules clause coarser; it is a decomposition of the same thirteen '
                'provisions and not a different number of them' % len(spec['provisions']))
        add('')
        add('| Provision | in the row | in the text |')
        add('|---|---|---|')
        for result in check_provisions(text, row, spec['provisions']):
            r, b = result['record'], result['body']
            good = r[0] == r[1] and b[0] == b[1]
            provisions_ok = provisions_ok and good
            add('| %s | %s | %s |' % (
                result['label'],
                'yes' if r[0] == r[1] else '**%d/%d phrases**' % r,
                '%d/%d phrases' % b if b[0] == b[1] else '**%d/%d phrases**' % b))
            if not good:
                for phrase in result['missing_record']:
                    add('| | **missing from the row**: `%s` | |' % phrase[:90])
                for phrase in result['missing_body']:
                    add('| | | **missing from the text**: `%s` |' % phrase[:90])
        add('')
    verdicts['every provision on the approved list is in its row and in the text'] = provisions_ok

    # ---------------------------------------------------------------- 3
    add('## 3. The three rows taken after the single act')
    add('')
    add('Two omissions were found **after** the single act above was taken, and one measurement '
        'refuted an ordering that act had approved. None could be folded into an act that had already '
        'happened, so each carries a row of its own, each stands on a ground its own approval cell '
        'names, and one says "in a **separate act**" in as many words. `WO-MOK-016` states all three '
        'in its *Required amendments* section, corrected in place rather than silently. The rule 26 '
        'row carries six provisions of that rule; the other two carry one each.')
    add('')
    later_ok = True
    for spec in LATER:
        path = f'{DOCS}/{spec["folder"]}/{spec["artifact"]}.md'
        text = read(path)
        row = locate(text, spec['key'])
        add('### `%s` — %s' % (spec['artifact'], spec['title']))
        add('')
        if row is None:
            later_ok = False
            add('- **the row is not located by its subject phrase.**')
            add('')
            continue
        status = cells(row)[-1]
        for phrase in spec['status']:
            present = phrase in status
            later_ok = later_ok and present
            add('- the approval cell %s: "%s"'
                % ('carries' if present else '**does not carry**', phrase))
        add('')
        add('| Provision | in the row | in the text |')
        add('|---|---|---|')
        for result in check_provisions(text, row, spec['provisions']):
            r, b = result['record'], result['body']
            good = r[0] == r[1] and b[0] == b[1]
            later_ok = later_ok and good
            add('| %s | %s | %s |' % (
                result['label'],
                'yes' if r[0] == r[1] else '**%d/%d phrases**' % r,
                '%d/%d phrases' % b if b[0] == b[1] else '**%d/%d phrases**' % b))
            if not good:
                for phrase in result['missing_record']:
                    add('| | **missing from the row**: `%s` | |' % phrase[:90])
                for phrase in result['missing_body']:
                    add('| | | **missing from the text**: `%s` |' % phrase[:90])
        add('')
    verdicts['each row taken after the single act stands on its own recorded approval'] = later_ok
    add('Two of the three are **understatements of a closed enumeration** — a public variant that '
        'already existed gaining a field, and one verb named where two share a path — which is the '
        'failure mode an enumeration written from the change rather than from the surface produces. '
        'The third is an approved ordering refuted by measurement rather than an enumeration defect, '
        'and `escalation.md` is the record it was decided against.')
    add('')

    # ---------------------------------------------------------------- 4
    add('## 4. What is retained rather than deleted')
    add('')
    add('Four sentences were inverted or retracted by the amendments above, and `SPEC-MOK-004` rule 11 '
        "requires the record to keep what it corrects. So each must still be present — and must be "
        'present **as a quotation of a superseded position**, not as a position. No phrase search can '
        'show that a phrase is gone; here nothing may be gone, and what must be gone is its standing. '
        'So each paragraph holding the sentence is located and the introducer that makes it a '
        'quotation is asserted **in that same paragraph**. The count is not the check — a sentence may '
        'honestly be quoted twice, once by the amendment that retracted it and once by a later one '
        'naming the position it replaced — but an occurrence in a paragraph that introduces it as '
        'nothing is the retracted claim still asserted, and that is what the last column reports.')
    add('')
    add('| Artifact | the superseded sentence | present | paragraphs holding it | holding it as a claim |')
    add('|---|---|---|---|---|')
    retained_ok = True
    for artifact, folder, label, quoted, introducers in RETAINED:
        text = read(f'{DOCS}/{folder}/{artifact}.md')
        result = check_retention(text, quoted, introducers)
        good = result['present'] and result['unquoted'] == 0
        retained_ok = retained_ok and good
        add('| `%s` | %s | %s | %d, %s | %s |' % (
            artifact, label,
            'yes' if result['present'] else '**no**',
            result['paragraphs'],
            'introducing it as superseded' if result['paragraphs'] == 1
            else 'each introducing it as superseded',
            'none' if result['unquoted'] == 0 else '**%d**' % result['unquoted']))
    verdicts['every superseded sentence is retained as a quotation and nowhere as a claim'] = retained_ok
    add('')
    add("`SPEC-MOK-003`'s is quoted twice and both are quotations: the 2026-08-19 amendment retracted "
        'it, and this chain\'s 2026-08-20 amendment names it again as "the earlier position" it is now '
        'satisfied a second way against. That is why this table counts paragraphs that hold the '
        'sentence as a claim rather than paragraphs that hold it at all.')
    add('')

    # ---------------------------------------------------------------- 5
    add('## 5. What was amended beyond the list, and what the list asked for and did not get')
    add('')
    add('### `REQ-MOK-057`, `REQ-MOK-058` and `VER-MOK-016`: amended inside their own chain')
    add('')
    add('These three are not on the *Required amendments* list, because on 2026-08-20 they were being '
        'written rather than amended. The measurement refuted them after they were approved, and each '
        'carries a row of its own.')
    add('')
    add('| Artifact | rows | the amendment | the act, as its own cell records it | ground named |')
    add('|---|---|---|---|---|')
    beyond_ok = True
    for artifact, folder, key in [
            ('REQ-MOK-057', 'requirements', 'The branch order and the engagement threshold'),
            ('REQ-MOK-058', 'requirements', 'The floor of five is ratified unchanged'),
            ('VER-MOK-016', 'verification', "Realigned to `REQ-MOK-057`'s first amendment")]:
        text = read(f'{DOCS}/{folder}/{artifact}.md')
        rows = amendment_rows(text)
        row = locate(text, key)
        status = cells(row)[-1] if row else ''
        act = re.match(r'(Approved|Ratified) %s by the repository owner acting as ([a-z and]+),'
                       % DATE, status)
        ground = 'escalation.md' in status
        good = row is not None and act is not None and ground
        beyond_ok = beyond_ok and good
        add('| `%s` | %d | %s | %s | %s |' % (
            artifact, len(rows), key,
            '**%s** %s, %s' % (act.group(1).lower(), DATE, act.group(2)) if act else '**not recorded**',
            '`evidence/WO-MOK-016/escalation.md`' if ground else '**none**'))
    verdicts['the three rows amended inside their own chain record their act and its ground'] = beyond_ok
    add('')
    add('`REQ-MOK-058`\'s cell says *ratified* rather than *approved* and the distinction is not '
        'cosmetic: its value did not move. The row records that the floor of five survived the first '
        'measured curve unchanged, against two lower alternatives measured and declined in the same '
        'act, which is a different act from amending a figure.')
    add('')
    add('`REQ-MOK-057` and `REQ-MOK-058` each open with an **Original approved content** row, so the '
        'ordering that was refuted and the bound that was ratified are both readable at their approved '
        'form rather than only in their corrected one. `VER-MOK-016` carries three rows: its original '
        "content, its realignment to `REQ-MOK-057`'s amendment, and oracle 5's restatement.")
    add('')
    add("### `SPEC-MOK-003`'s reconciliation row: recorded, and standing on no owner act")
    add('')
    spec3 = read(f'{DOCS}/specifications/SPEC-MOK-003.md')
    reconciliation = locate(spec3, 'This row records the reconciliation of the `CAP-MOK-010` rule 4 '
                                  'amendment above with the `WO-MOK-013` amendments')
    add('| | |')
    add('|---|---|')
    add('| located | %s |' % ('yes' if reconciliation else '**no**'))
    if reconciliation:
        status = cells(reconciliation)[-1]
        add('| recorded by | %s |' % ('the implementation agent, as a statement of fact about '
                                      'amendments it holds no authority over'
                                      if 'implementation agent' in status else '**unclear**'))
        add('| ratifies anything | %s |' % ('no — "Nothing is ratified here and no provision changes"'
                                            if 'Nothing is ratified here' in status else '**unclear**'))
        add('| precedent cited | %s |' % ('the two 2026-08-19 reconciliation rows above it'
                                          if 'reconciliation rows above' in status else '**none**'))
    add('')
    add('It states that two owner acts written against different trees met in a merge and both hold, '
        'and it changes no provision. **It stands on a precedent rather than an approval**, which is '
        'named as a residual in `completion-summary.md` item 17 rather than settled here: whether a '
        'statement of fact about two amendments needs an act of its own is the owner\'s to decide.')
    add('')
    add("### `REQ-MOK-014`: the amendment the list asked for and the measurement did not trigger")
    add('')
    req14 = read(f'{DOCS}/requirements/REQ-MOK-014.md')
    rows14 = amendment_rows(req14)
    recent14 = [r for r in rows14 if r.startswith('| 2026-08-2')]
    unchanged14 = req14 == at(MASTER, f'{DOCS}/requirements/REQ-MOK-014.md')
    add('| | |')
    add('|---|---|')
    add('| amendment rows in total | %d, the latest dated %s |'
        % (len(rows14), cells(rows14[-1])[0] if rows14 else 'n/a'))
    add('| rows dated 2026-08-20 or 2026-08-21 | **%d** |' % len(recent14))
    add('| identical to `master` | %s |' % ('yes' if unchanged14 else '**no**'))
    add('')
    add('`WO-MOK-016` made this amendment conditional in its own words: the floor "is expected to '
        'stand; whether it does is measured, and if it does not the amendment is the owner\'s '
        'decision". The measurement is in `post/byte-identity.txt` and `completion-summary.md` item 7: '
        'all sixty declared cells of `reference` and `individual` are byte-identical to the pre-change '
        'capture, and identical output is identical survivors. So the floor is **preserved rather than '
        're-established**, the condition never fires, and the absence of a row is the correct state '
        'rather than a missing act. Oracle 7 names three requirement amendments; two exist and the '
        'third was contingent on a measurement that did not move.')
    add('')
    add('### `SPEC-MOK-004`: an amendment this work order owes and has not made')
    add('')
    spec4_path = f'{DOCS}/specifications/SPEC-MOK-004.md'
    spec4 = read(spec4_path)
    rows4 = [r for r in amendment_rows(spec4) if WORK_ORDER in r]
    add('| | |')
    add('|---|---|')
    add('| rows naming `%s` | **%d** |' % (WORK_ORDER, len(rows4)))
    add('| identical to `master` | %s |'
        % ('yes' if spec4 == at(MASTER, spec4_path) else '**no**'))
    add('')
    add('Rule 11 states the obligation in its own text: a work order that adds a test corrects the '
        "recorded figures there. This work order adds tests — `post/test-census-reconciliation.md` "
        'counts them — and **has not corrected them**. It is recorded as owed rather than done, for '
        'the reason `completion-summary.md` item 17 finding 1 gives: correcting the figures moves the '
        'tree that this packet\'s census, test-run and updated-test captures were taken against, and '
        'rule 11 requires figures to be re-derived rather than edited. Whether that re-derivation '
        'happens under this work order or the next is an owner\'s call, and it is the one thing on '
        "oracle 7's surface that is neither present nor contingent.")
    add('')

    # ---------------------------------------------------------------- 6
    add('## 6. The earlier layer')
    add('')
    add('Every amendment row that existed at the baseline, compared against the candidate in order. '
        'Some of them changed, and **none of them changed here**. A check that simply required byte '
        'equality would report a defect where the truth is another work order paying a debt through '
        'the merge, so the two cells of each row are classified separately: identifiers are masked to '
        'see through this chain\'s own renumber, and the operative cell is compared apart from the '
        'approval cell, because an edited provision and a moved approval state are different events '
        'and only one of them is legitimate after the fact.')
    add('')
    add('| Artifact | rows at the baseline | rows now | identical | status moved | renumbered only | '
        'operative cell moved | absent |')
    add('|---|---:|---:|---:|---:|---:|---:|---:|')
    layer_ok = True
    layer_detail = []
    totals = {'base': 0, 'identical': 0, 'status': 0, 'renumber': 0, 'moved': 0, 'absent': 0}
    for artifact in ('SPEC-MOK-001', 'SPEC-MOK-002', 'SPEC-MOK-003', 'SPEC-MOK-004'):
        path = f'{DOCS}/specifications/{artifact}.md'
        base_rows = amendment_rows(at(BASE, path))
        head_rows = amendment_rows(read(path))
        counts = {'identical': 0, 'status': 0, 'renumber': 0, 'moved': 0, 'absent': 0}
        for row in base_rows:
            kind, match = classify(row, head_rows)
            if kind == 'identical':
                counts['identical'] += 1
                continue
            if kind == 'status only':
                counts['status'] += 1
            elif kind.startswith('renumbered'):
                counts['renumber'] += 1
            elif kind == 'absent':
                counts['absent'] += 1
            else:
                counts['moved'] += 1
            layer_detail.append((artifact, cells(row)[0], cells(row)[1][:66], kind, match))
        layer_ok = layer_ok and counts['moved'] == 0 and counts['absent'] == 0
        totals['base'] += len(base_rows)
        for key in counts:
            totals[key] += counts[key]
        add('| `%s` | %d | %d | %d | %d | %d | %s | %s |' % (
            artifact, len(base_rows), len(head_rows), counts['identical'], counts['status'],
            counts['renumber'],
            counts['moved'] if counts['moved'] == 0 else '**%d**' % counts['moved'],
            counts['absent'] if counts['absent'] == 0 else '**%d**' % counts['absent']))
    add('| **total** | **%d** | | **%d** | **%d** | **%d** | **%s** | **%s** |' % (
        totals['base'], totals['identical'], totals['status'], totals['renumber'],
        totals['moved'], totals['absent']))
    verdicts['no earlier provision moved'] = layer_ok
    add('')
    add('So of the %d rows the baseline held, %d are byte-identical and %d changed: %d whose approval '
        'cell moved with the operative cell untouched to the byte, and %d of this chain\'s own rows '
        'carrying nothing but the renumber recorded below. **No operative cell moved and no row is '
        'absent.**'
        % (totals['base'], totals['identical'], totals['base'] - totals['identical'],
           totals['status'], totals['renumber']))
    add('')
    add('| Artifact | row | subject | what changed | the act that changed it |')
    add('|---|---|---|---|---|')
    for artifact, date, subject, kind, match in layer_detail:
        if match is None:
            under = '**none found**'
        elif 'WO-MOK-012' in cells(match)[-1]:
            under = '`WO-MOK-012`, on `master`, through the merge'
        else:
            under = "this chain's own row, renumbered"
        add('| `%s` | %s | %s | %s | %s |' % (artifact, date, subject, kind, under))
    add('')
    miscounts = [match for _, _, _, kind, match in layer_detail
                 if kind == 'status only' and 'miscount is recorded' in cells(match)[-1]]
    add('The %d approval cells that moved all moved in the same direction and by the same act: '
        '`WO-MOK-012` ratified them on `master` on 2026-08-20, and each cell now names that act, the '
        'role it was taken in, and the interval it was outstanding for. %d of them also corrects its '
        'own earlier wording, having said that both of two rows remained outstanding when only the '
        'first ever was — a miscount its new text reports rather than drops. The %d renumbered rows '
        "are this chain's own amendment rows, and masking the identifiers shows both cells of each "
        'unchanged in every other character.'
        % (totals['status'], len(miscounts), totals['renumber']))
    add('')
    add('| | |')
    add('|---|---:|')
    prefix_ok = True
    for artifact in ('SPEC-MOK-001', 'SPEC-MOK-002', 'SPEC-MOK-003', 'SPEC-MOK-004'):
        path = f'{DOCS}/specifications/{artifact}.md'
        master_rows = amendment_rows(at(MASTER, path))
        head_rows = amendment_rows(read(path))
        good = head_rows[:len(master_rows)] == master_rows
        prefix_ok = prefix_ok and good
        add("| `%s`: `master`'s rows are a prefix of the candidate's, in order | %s |"
            % (artifact, 'yes' if good else '**no**'))
    verdicts["this chain's rows are appended after `master`'s, none interleaved"] = prefix_ok
    add('')
    add("So the rows this chain adds are **appended after everything `master` holds**. No row of "
        "`master`'s is edited here, reordered, summarised or folded into a later one; the %d of them "
        'that differ from the baseline differ in their approval cells, in the direction of being '
        'ratified, by an act this branch did not take and does not claim.' % totals['status'])
    add('')
    add('| Rows carrying **OUTSTANDING** as their own state | count |')
    add('|---|---:|')
    outstanding = {}
    for ref, label in [(BASE, 'at the baseline `%s`' % BASE[:8]),
                       (MASTER, "at `master`'s tip `%s`" % MASTER[:8]),
                       (None, 'at this candidate')]:
        total = 0
        for folder in sorted(f for f in os.listdir(DOCS)
                             if f != 'evidence' and os.path.isdir(f'{DOCS}/{f}')):
            directory = f'{DOCS}/{folder}'
            for name in sorted(os.listdir(directory)):
                if not name.endswith('.md'):
                    continue
                path = f'{directory}/{name}'
                text = read(path) if ref is None else at(ref, path)
                if text is None:
                    continue
                total += sum(is_outstanding(row) for row in amendment_rows(text))
        outstanding[label] = total
        add('| %s | %d |' % (label, total))
    add('')
    add('The four at the baseline are the gate of section 8. **None is outstanding now**, and the act '
        'that closed each is `WO-MOK-012`\'s, on `master`, recorded in the row it closed. The count '
        'covers every non-evidence artifact in `%s`, not only the four: a row left outstanding '
        'anywhere else would appear here.' % DOCS)
    add('')

    # ---------------------------------------------------------------- 7
    add('## 7. What must not have changed at all')
    add('')
    add('| Artifact | changed since `master` | line endings only | why it must not be |')
    add('|---|---|---|---|')
    frozen_ok = True
    for path, reason in UNCHANGED:
        full = f'{DOCS}/{path}'
        same = read(full) == at(MASTER, full)
        frozen_ok = frozen_ok and same
        add('| `%s` | %s | %s | %s |' % (
            os.path.basename(path)[:-3], 'no' if same else '**yes**',
            'yes' if raw_differs_only_by_newlines(MASTER, full) else 'n/a — raw bytes equal', reason))
    changed_vrecs = [v for v in VRECS
                     if read(f'{DOCS}/verification-records/{v}.md')
                     != at(MASTER, f'{DOCS}/verification-records/{v}.md')]
    frozen_ok = frozen_ok and not changed_vrecs
    add('| all %d verification records | %s | — | commit-bound records; none is re-opened, '
        '`VREC-MOK-005` and `VREC-MOK-010` included |'
        % (len(VRECS), 'no' if not changed_vrecs else '**%s**' % ', '.join(changed_vrecs)))
    verdicts['nothing that must not have changed changed'] = frozen_ok
    add('')
    add('The middle column is why this table compares content rather than raw bytes: this clone has '
        '`core.autocrlf = true`, so a tracked governance file is CRLF in the working tree and LF in '
        'the blob. The check is run both ways — equal after folding CRLF to LF, and raw bytes equal '
        'once both sides are folded — so the normalization cannot absorb an edit.')
    add('')
    add('`VREC-MOK-010` is in that list for a reason beyond commit-binding. It states that the '
        '`VREC-MOK-005` gate "is not satisfied" and that the transition of that record to `verified` '
        '"does not close the gate" — "a cost carried forward, not a debt paid". That was measured at '
        'its own commit and it is not edited here. What changed since is measured in the next section.')
    add('')

    # ---------------------------------------------------------------- 8
    add('## 8. The `VREC-MOK-005` gate, recorded')
    add('')
    vrec5_path = f'{DOCS}/verification-records/VREC-MOK-005.md'
    vrec5 = read(vrec5_path)
    add('| | |')
    add('|---|---|')
    add('| `VREC-MOK-005` status | `%s`, updated %s, binding `%s` |'
        % (field(vrec5, 'status'), field(vrec5, 'updated'), field(vrec5, 'commit')[:8]))
    add('| its own statement of what it accepted | "with all seven manual assessments outstanding and '
        'unauthored, and eleven provisions across four approved artifacts awaiting the technical '
        'owner" |')
    add('| that sentence at this candidate | unchanged — the record is not edited, and it was correct '
        'at its commit |')
    add('')
    add('### The eleven provisions')
    add('')
    add('| Artifact | provisions | **OUTSTANDING** at the baseline | now | the act |')
    add('|---|---:|---|---|---|')
    gate_provisions = 0
    gate_open_now = 0
    for path, count, note in GATE_ARTIFACTS:
        full = f'{DOCS}/{path}'
        base_rows = amendment_rows(at(BASE, full))
        head_rows = amendment_rows(read(full))
        then = sum(is_outstanding(row) for row in base_rows)
        now = sum(is_outstanding(row) for row in head_rows)
        gate_provisions += count
        gate_open_now += now
        ratified = [row for row in head_rows
                    if 'Ratified 2026-08-20' in cells(row)[-1] and 'WO-MOK-012' in cells(row)[-1]]
        add('| `%s` | %d | %d row%s | %s | %s |' % (
            os.path.basename(path)[:-3], count, then, '' if then == 1 else 's',
            '%d' % now if now == 0 else '**%d**' % now,
            'ratified 2026-08-20 by the technical owner under `WO-MOK-012`' if ratified
            else '**no ratification row found**'))
    add('| **total** | **%d** | 4 rows | %s | |' % (
        gate_provisions, '0' if gate_open_now == 0 else '**%d**' % gate_open_now))
    add('')
    add('`WO-MOK-012`\'s own `amendment-ratifications.md` opens on the same count — "Eleven provisions '
        'across four approved artifacts were amended in the tree with an Approval cell reading '
        '**OUTSTANDING**" — and records that all eleven were ratified **as written, without '
        'modification**, by the repository owner acting as technical owner, each by editing that '
        "row's Approval cell in place rather than appending a row. Section 6's classification is what "
        'checks that: the operative cell of every one of those rows is unchanged, so the act ratified '
        'text rather than replacing it.')
    add('')
    add('### The seven manual assessments')
    add('')
    assessment_path = f'{DOCS}/evidence/WO-MOK-012/manual-assessment.md'
    assessment = read(assessment_path)
    summary_rows = [line for line in assessment.split('\n')
                    if re.match(r'^\| \d \| ', line)]
    authored = [r for r in summary_rows if 'none' not in cells(r)[-1].lower()]
    outstanding_rows = [r for r in summary_rows if 'Outstanding' in cells(r)[2]]
    add('| | |')
    add('|---|---|')
    add('| record | `evidence/WO-MOK-012/manual-assessment.md`, "closing `VER-MOK-005`\'s seven" |')
    add('| assessments in its summary table | **%d** |' % len(summary_rows))
    add('| authored | **%d**, every one by the repository owner in the role named |' % len(authored))
    add('| outstanding | **%d** — assessment %s, "outstanding by decision" |'
        % (len(outstanding_rows),
           ', '.join(cells(r)[0] for r in outstanding_rows) or 'none'))
    gate_assessments_ok = len(summary_rows) == 7 and len(authored) == 6 and len(outstanding_rows) == 1
    verdicts["the gate's assessments are measured, six authored and one open by decision"] = \
        gate_assessments_ok
    add('')
    add('The seventh is the terminal-restoration inspection, and it is open **by the assurance '
        "owner's decision of 2026-08-20** rather than by omission: the contract asks for a live "
        'inspection "rather than only by an automated assertion", and the shipped binary has no '
        'operator-reachable panic path, so performing it would require inspecting a terminal restored '
        'by a program that is not the one released. The recorded decision is to leave it outstanding '
        'and visible, and that `VREC-MOK-005` must continue to disclose it.')
    add('')
    add('### What that means for this contract, and what it does not')
    add('')
    add('- **The amendment half of the gate is closed, and not by this work order.** Eleven of eleven '
        'provisions are ratified, each in the row it belongs to, by the technical owner on 2026-08-20 '
        'under `WO-MOK-012`. This branch inherited the closure through the merge at `%s` and claims '
        'no part of it.' % MERGE[:8])
    add('- **The assessment half is six of seven, with the seventh open by a recorded decision.** That '
        'is a different state from unauthored, and a different state from closed.')
    add('- **`VREC-MOK-010`\'s reading is superseded rather than wrong.** It measured the gate open at '
        'its own commit and said the status had moved while the substance had not. The substance has '
        'since moved, on `master`, and this file is where that is recorded for this chain.')
    add('- **This contract still does not close `VREC-MOK-005`.** `VER-MOK-016` says so in its own '
        'residual-uncertainty section, and nothing here transitions that record, performs its seventh '
        'assessment or re-opens its commit.')
    add('')

    # ---------------------------------------------------------------- controls
    add('## The controls on the checks themselves')
    add('')
    add('All %d held, so no line above is a check that looked for nothing:' % len(controls))
    add('')
    for label, ok in controls:
        add('- %s — %s' % ('ok' if ok else 'FAIL', label))
    add('')

    # ---------------------------------------------------------------- result
    add('## Result')
    add('')
    verdict = all(verdicts.values())
    add('| Check | Result |')
    add('|---|---|')
    for label, ok in verdicts.items():
        add('| %s | %s |' % (label, 'PASS' if ok else '**FAIL**'))
    add('')
    add('**RESULT: %s.**' % ('PASS' if verdict else 'FAIL'))
    add('')
    add('What that means, stated as the claims it rests on. Every artifact in the chain carries the '
        'status it should, and the three amended specifications specify the requirements that made '
        'this chain approvable at all. All %d subjects of the approved list are present in '
        'the row that must state them **and** in the artifact with every row removed, which are '
        'disjoint texts, and each row is found by its own subject rather than by a date eleven rows '
        'share. The three rows taken after the single act each stand on an approval their own cell '
        'records, with the role and the ground named. The four superseded sentences are retained and '
        'every paragraph holding one introduces it as superseded. No operative cell of any earlier row '
        "moved, this chain's rows are appended after everything `master` holds, and nothing that must "
        'not have changed changed.' % sum(len(spec['provisions']) for spec in ROWS))
    add('')
    add('What this file does **not** establish:')
    add('')
    add('- **That oracle 7 is satisfied.** Two things on its surface are not acts this file can '
        'find. `SPEC-MOK-004` rule 11\'s figures are owed and unmade, by a decision recorded in '
        "`completion-summary.md` item 17 rather than taken; and `SPEC-MOK-003`'s reconciliation row "
        'stands on a precedent rather than an approval. Both are named for the owner, and neither is '
        'silently absorbed here.')
    add('- **That the `VREC-MOK-005` gate is closed.** Its amendment half is, its assessment half is '
        'six of seven, and whether that state satisfies this contract\'s static row is the assurance '
        "owner's reading. `manual-assessment.md` in this packet is where `VER-MOK-016`'s own eleven "
        'assessments wait, and this file is one of the things assessment 11 reads.')
    print('\n'.join(out))
    return 0 if verdict else 1


if __name__ == '__main__':
    sys.exit(main())
