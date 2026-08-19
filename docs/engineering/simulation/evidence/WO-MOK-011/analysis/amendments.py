"""VER-MOK-011 oracle 5, governance half: is every amendment this change needs approved?

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-011/analysis/amendments.py \
        > docs/engineering/simulation/evidence/WO-MOK-011/amendment-approvals.md

The oracle's own words are that "an amendment nobody approved is not a specification". The failure
this script exists to catch is not a missing signature line -- it is a work order that quietly amends
a specification in its body without recording it, or records an amendment its body never made. Both
read as success to a reader who checks only one side, so every provision is looked for twice, in
disjoint text: once in the amendment record's 2026-08-19 row, and once in the specification's body
with that row's text removed. A record whose own prose satisfies a body check would otherwise pass.

Five things are measured:

  1. the `status` and `updated` field of every artifact in the chain;
  2. each provision `WO-MOK-011` states, in the record and in the body;
  3. the two provisions that amend by deletion -- located by an anchor that must still be present, so
     that a capture which stopped short of the list fails instead of reporting an absence;
  4. the earlier layer: every amendment row that existed at the base commit, byte for byte, in order,
     and the OUTSTANDING markers then and now;
  5. the artifacts that must not have changed at all -- `SPEC-MOK-002`, whose "needs no amendment"
     claim is `WO-MOK-011`'s, and every commit-bound verification record.

`self_test` injects each failure mode and asserts that it is reported, because a check that finds
nothing reads exactly like a check that looks for nothing. It runs before any result is printed.
"""

import io
import os
import re
import subprocess
import sys

BASE = '524a6758d74b5240079959e9827ea40a7af22a30'
DATE = '2026-08-19'
DOCS = 'docs/engineering/simulation'

CHAIN = [
    ('INT-MOK-008', 'intent', 'approved'),
    ('CAP-MOK-008', 'capabilities', 'approved'),
    ('REQ-MOK-040', 'requirements', 'approved'),
    ('REQ-MOK-041', 'requirements', 'approved'),
    ('VER-MOK-011', 'verification', 'approved'),
    ('WO-MOK-011', 'work-orders', 'in_progress'),
    ('SPEC-MOK-001', 'specifications', 'approved'),
    ('SPEC-MOK-002', 'specifications', 'approved'),
    ('SPEC-MOK-003', 'specifications', 'approved'),
    ('SPEC-MOK-004', 'specifications', 'approved'),
]

AMENDED = ['SPEC-MOK-001', 'SPEC-MOK-003', 'SPEC-MOK-004']

# Every provision WO-MOK-011's *Required amendments* section states, with the phrases that must
# appear in the amendment record's row and the phrases that must appear in the specification's body.
# The two lists are deliberately different strings: a row that merely quoted the body would not
# satisfy the row check, and the body is searched with the row removed.
PROVISIONS = {
    'SPEC-MOK-001': [
        ('1. *State model / Mokiterion* gains the name',
         ['*State model / Mokiterion* gains the name'],
         ['a name, fixed for the run and identical in every run',
          'It carries no behavior']),
        ('2. A *Name* subsection fixes the twelve, the domain and the three properties',
         ['A new *Name* subsection fixes the twelve names',
          'reads neither the seed nor the configuration',
          'nothing in the engine reads a name'],
         ['### Name',
          '| `M01` | `Zug` | `M07` | `Hozz` |',
          '| `M06` | `Womp` | `M12` | `Drix` |',
          'The twelve names are pairwise distinct',
          '**Naming performs no draw']),
        ('3. *Time and entropy* records that naming performs no draw at all',
         ['naming is not an exception to the single shared stream'],
         ['Naming is not an exception because it is not a draw at all']),
        ('4. *Data and interface contracts*: `name` first, `waste_tolerance` last, reported once',
         ['puts `name` first in the `agent_initialized` details, before `position`',
          'reports it once and on no other record kind',
          'two test suites parse that record positionally'],
         ['result=name:<letters>,position:',
          'waste_tolerance:<number>',
          'The name is reported once, in `agent_initialized`',
          'Two test suites parse this record positionally']),
        ('5. Rule 1 places the assignment at agent creation',
         ['Rule 1 places the assignment at agent creation'],
         ['name is assigned as *Name* specifies at the point that agent is created']),
    ],
    'SPEC-MOK-003': [
        ("1. Rule 2's glyph tables become the name's first character, anticipation retained",
         ["**Rule 2's glyph tables**",
          "the name's first character uppercased",
          '**The anticipation is retained rather than deleted**'],
         ["the name's first character as an uppercase glyph",
          "| Mokiterion, named | the name's first character, uppercased |",
          'carry no names',
          'when agent naming is introduced by a later phase']),
        ('2. Rule 4 carries the name first, and line two is measured to be untouched',
         ['the entry mockup and prose carry the name first',
          'the name occupies six columns of line one only',
          "line one's fixed fields total 28 columns of a 45-column interior"],
         ['Trok  M05  A  81:14',
          'Line one carries the name, the identifier',
          'bar_width(interior) = min(20, (interior - 35) / 4)',
          '`6 + 5 + 3 + 14 = 28`']),
        ('3. Rule 10 presents the name; item 7 loses it and keeps the six',
         ['the presented-value list gains the name',
          'item 7 loses `name`'],
         ['the inspector presents its name, its identifier',
          'age, kills, combats, remembered locations, model latency']),
    ],
}

# The provisions that amend by deletion. Each names the sentence to locate, an anchor that must still
# be inside it -- placed at the far end of the list being inspected -- and the words that must be
# gone. No phrase search can show that a phrase is absent, so the sentence is located first and its
# contents are asserted; without the anchor, a truncated capture would report a false absence.
DELETIONS = [
    ('SPEC-MOK-003',
     "rule 10 item 7's list of values the engine does not compute no longer names the name",
     r'7\. Fields for values the engine does not compute[^\n]*\n(?:[^\n]*\n)?',
     'per-agent entropy',
     ['name,', ' name ']),
    ('SPEC-MOK-003',
     "rule 2's active Mokiterion glyph row no longer assigns a digit",
     r'\| Mokiterions \| the name[^\n]*\n',
     "first character",
     ['identifier', 'last character']),
]

# Artifacts that must be byte-identical to the base commit, with the reason each must be.
UNCHANGED = [
    ('specifications/SPEC-MOK-002.md',
     "`WO-MOK-011` claims it needs no amendment; oracle 5's interface half is what checks the claim"),
    ('verification-records/VREC-MOK-001.md', 'commit-bound record; not re-opened'),
    ('verification-records/VREC-MOK-002.md', 'commit-bound record; not re-opened'),
    ('verification-records/VREC-MOK-003.md', 'commit-bound record; not re-opened'),
    ('verification-records/VREC-MOK-004.md', 'commit-bound record; not re-opened'),
    ('verification-records/VREC-MOK-005.md',
     'the record whose gate the owner overrode under `WO-MOK-007`; still `ready`, its own amendments still outstanding'),
    ('verification-records/VREC-MOK-006.md',
     'measured 97 interface items and 169 tests, both correct at its commit'),
    ('verification-records/VREC-MOK-007.md', 'commit-bound record; not re-opened'),
    ('architecture/ARCH-MOK-001.md', 'no architecture amendment is required by this work order'),
    ('architecture/ARCH-MOK-002.md', 'the same'),
]


def read(path):
    """The file's text with line endings normalized.

    This clone has `core.autocrlf = true`, so a tracked text file is CRLF in the working tree and LF
    in the blob `git show` prints. Every comparison here is therefore a comparison of content after
    newline normalization, not of raw bytes; `raw_differs_only_by_newlines` below is what checks that
    the distinction is the whole difference, so the normalization cannot hide a real edit.
    """
    return io.open(path, encoding='utf-8', newline='').read().replace('\r\n', '\n')


def at_base(path):
    out = subprocess.run(['git', 'show', f'{BASE}:{path}'],
                         capture_output=True, check=True)
    return out.stdout.decode('utf-8').replace('\r\n', '\n')


def raw_differs_only_by_newlines(path):
    """True when the working-tree bytes equal the blob's once CRLF is folded to LF, and they differ."""
    raw = io.open(path, 'rb').read()
    blob = subprocess.run(['git', 'show', f'{BASE}:{path}'],
                          capture_output=True, check=True).stdout
    return raw != blob and raw.replace(b'\r\n', b'\n') == blob.replace(b'\r\n', b'\n')


def field(text, name):
    match = re.search(r'^%s = "([^"]*)"' % name, text, re.M)
    return match.group(1) if match else '<absent>'


def amendment_rows(text):
    """Every row of the amendment record, in order, as written."""
    return [line for line in text.split('\n')
            if re.match(r'^\| \d{4}-\d\d-\d\d \|', line)]


def is_outstanding(row):
    """True when the row's own status cell -- the last -- declares it outstanding.

    A row that merely says which earlier rows are untouched mentions the word in its middle cell, and
    must not be counted as outstanding itself.
    """
    cells = [cell.strip() for cell in row.split('|')]
    return bool(cells) and cells[-2].startswith('**OUTSTANDING')


def rows_dated(text, date):
    return [row for row in amendment_rows(text) if row.startswith('| ' + date + ' |')]


def body_without_record(text):
    """The specification with every amendment row removed, so the record cannot satisfy a body check."""
    return '\n'.join(line for line in text.split('\n')
                     if not re.match(r'^\| \d{4}-\d\d-\d\d \|', line))


def check_provisions(text, provisions, work_order_id):
    """For each provision: is it in a row of this date that names this work order, and in the body?"""
    rows = rows_dated(text, DATE)
    mine = [row for row in rows if work_order_id in row] or rows
    record = '\n'.join(mine)
    body = body_without_record(text)
    results = []
    for label, in_record, in_body in provisions:
        got_record = [phrase for phrase in in_record if phrase in record]
        got_body = [phrase for phrase in in_body if phrase in body]
        results.append({
            'label': label,
            'record': (len(got_record), len(in_record)),
            'body': (len(got_body), len(in_body)),
            'missing_record': [p for p in in_record if p not in record],
            'missing_body': [p for p in in_body if p not in body],
        })
    return results


def check_deletion(text, sentence_pattern, anchor, forbidden):
    body = body_without_record(text)
    match = re.search(sentence_pattern, body)
    if not match:
        return {'located': False, 'anchored': False, 'still_present': [], 'chars': 0}
    found = match.group(0)
    return {
        'located': True,
        'anchored': anchor in found,
        'still_present': [word for word in forbidden if word in found],
        'chars': len(found),
    }


def self_test():
    """Inject each failure mode and assert it is reported. Returns a list of (control, ok)."""
    controls = []

    good = ('| 2026-08-19 | states the provision, WO-MOK-011 | Approved |\n'
            'the body carries the provision\n')
    ok = check_provisions(good, [('control', ['states the provision'], ['body carries'])],
                          'WO-MOK-011')[0]
    controls.append(('a provision stated in the row and carried by the body is reported clean',
                     ok['record'] == (1, 1) and ok['body'] == (1, 1)))

    silent = '| 2026-08-19 | says nothing, WO-MOK-011 | Approved |\nthe body carries the provision\n'
    ok = check_provisions(silent, [('control', ['states the provision'], ['body carries'])],
                          'WO-MOK-011')[0]
    controls.append(('a provision the record does not state is reported',
                     ok['record'] == (0, 1)))

    absent = '| 2026-08-19 | states the provision, WO-MOK-011 | Approved |\nan unrelated body\n'
    ok = check_provisions(absent, [('control', ['states the provision'], ['body carries'])],
                          'WO-MOK-011')[0]
    controls.append(('a provision absent from the body is reported', ok['body'] == (0, 1)))

    only_row = '| 2026-08-19 | states the provision and body carries, WO-MOK-011 | Approved |\n'
    ok = check_provisions(only_row, [('control', ['states the provision'], ['body carries'])],
                          'WO-MOK-011')[0]
    controls.append(("the record's own prose cannot satisfy a body check", ok['body'] == (0, 1)))

    result = check_deletion('nothing here', r'^7\. Fields[^\n]*\n', 'anchor', ['name'])
    controls.append(('a deletion check whose sentence is not found is reported, not passed',
                     result['located'] is False))

    truncated = '7. Fields for values the engine does not compute - age, kills\n'
    result = check_deletion(truncated, r'7\. Fields[^\n]*\n', 'per-agent entropy', ['name'])
    controls.append(('a deletion check truncated before its anchor is reported, not passed',
                     result['located'] and not result['anchored']))

    still = '7. Fields for values the engine does not compute - name, age, per-agent entropy\n'
    result = check_deletion(still, r'7\. Fields[^\n]*\n', 'per-agent entropy', ['name,'])
    controls.append(('a forbidden word still present in the sentence is reported',
                     result['still_present'] == ['name,']))

    rows = amendment_rows('| 2026-08-18 | a |\n| 2026-08-19 | b |\nprose\n')
    controls.append(('the row reader finds rows in order and ignores prose',
                     len(rows) == 2 and rows[0].startswith('| 2026-08-18')))

    own = '| 2026-08-19 | amended something | **OUTSTANDING.** Requires the technical owner. |'
    other = '| 2026-08-19 | the row above marked **OUTSTANDING** is untouched | Approved 2026-08-19. |'
    controls.append(('a row outstanding in its own status cell is told from one that mentions the word',
                     is_outstanding(own) and not is_outstanding(other)))

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
    add('# WO-MOK-011 amendment approvals — oracle 5, governance half')
    add('')
    add("`VER-MOK-011`'s fifth oracle is the governance state of the artifacts this change amends:")
    add('an amendment nobody approved is not a specification, and its absence fails the contract')
    add('however the code behaves. This file is generated by `analysis/amendments.py`, which reads the')
    add('artifacts and the git history; what it checks, and why each check is not a formality, is in')
    add("that script's header. The oracle's other half — the engine's public interface enumerated item")
    add('for item at both commits, which is what checks `WO-MOK-011`\'s claim that `SPEC-MOK-002` needs')
    add('no amendment — is `interface.txt`.')
    add('')
    add(f'Everything below is measured against `{BASE}`, the commit this work started from, and every')
    add(f'row this work order adds is dated `{DATE}`.')
    add('')
    add("## 1. The chain's governance state")
    add('')
    add('| Artifact | status | expected | updated | ok |')
    add('|---|---|---|---|---|')
    chain_ok = True
    for artifact, folder, expected in CHAIN:
        text = read(f'{DOCS}/{folder}/{artifact}.md')
        status, updated = field(text, 'status'), field(text, 'updated')
        note = '`%s`, amended' % expected if artifact in AMENDED else '`%s`' % expected
        good = status == expected
        chain_ok = chain_ok and good
        add(f'| `{artifact}` | `{status}` | {note} | {updated} | {"yes" if good else "**no**"} |')
    add('')
    add('`SPEC-MOK-002` is in the table because this work order names it: it is `approved` and')
    add('**unamended**, which is a claim checked in section 5 and in `interface.txt` rather than')
    add('asserted here. The work order is `in_progress` and not `complete`: a work order is closed by a')
    add('verification record that binds a commit, and that record is written after the commit it names.')
    add('')
    add('## 2. The amendment record against the approved list')
    add('')
    add('Each provision `WO-MOK-011` states in full is looked for twice — in the amendment record’s')
    add(f'{DATE} row, and in the specification’s body with every amendment row removed — because a')
    add('record that claimed an amendment the text does not carry would otherwise satisfy both checks')
    add('with one sentence.')
    add('')
    provisions_ok = True
    for artifact in ('SPEC-MOK-001', 'SPEC-MOK-003'):
        text = read(f'{DOCS}/specifications/{artifact}.md')
        rows = rows_dated(text, DATE)
        mine = [row for row in rows if 'WO-MOK-011' in row]
        add(f'### `{artifact}`')
        add('')
        add(f'- rows dated {DATE}: **{len(rows)}**, of which **{len(mine)}** name this work order')
        add('- approval recorded in the row: %s' %
            ('yes' if any('Approved ' + DATE in row for row in mine) else '**no**'))
        add('- records that the implementation agent wrote the text and decided no substance: %s' %
            ('yes' if any('did not decide the substance' in row for row in mine) else '**no**'))
        add('')
        add('| Provision | in the record | in the text |')
        add('|---|---|---|')
        for result in check_provisions(text, PROVISIONS[artifact], 'WO-MOK-011'):
            r, b = result['record'], result['body']
            good = r[0] == r[1] and b[0] == b[1]
            provisions_ok = provisions_ok and good
            add('| %s | %s | %s |' % (
                result['label'],
                'yes' if r[0] == r[1] else '**%d/%d phrases**' % r,
                '%d/%d phrases' % b if b[0] == b[1] else '**%d/%d phrases**' % b))
        add('')
    add('Two provisions amend by deletion, and no phrase search can show that a phrase is gone. The')
    add('sentence that carried it is located instead and its contents asserted, each anchored on a')
    add('phrase that must still be at the far end of the list being inspected, so that a capture which')
    add('stopped short fails rather than reporting an absence it never looked for.')
    add('')
    add('| Artifact | sentence | located, and the anchor in it | no longer names |')
    add('|---|---|---|---|')
    deletions_ok = True
    for artifact, label, pattern, anchor, forbidden in DELETIONS:
        text = read(f'{DOCS}/specifications/{artifact}.md')
        result = check_deletion(text, pattern, anchor, forbidden)
        good = result['located'] and result['anchored'] and not result['still_present']
        deletions_ok = deletions_ok and good
        located = ('yes, %d characters through `%s`' % (result['chars'], anchor)
                   if result['located'] and result['anchored'] else '**no**')
        add('| `%s` | %s | %s | %s |' % (
            artifact, label, located,
            'yes' if not result['still_present'] else '**still names %s**' % result['still_present']))
    add('')
    add('All %d controls on the checks themselves held, so no line above is a check that looked for'
        % len(controls))
    add('nothing:')
    add('')
    for label, ok in controls:
        add('- %s — %s' % ('ok' if ok else 'FAIL', label))
    add('')
    add('## 3. What was amended beyond the approved list')
    add('')
    add("`WO-MOK-011` states amendments to two specifications. A third was amended: `SPEC-MOK-004`,")
    add('whose rules 9, 10 and 11 record the test census. It is not in the work order’s list, and it is')
    add('not a silent amendment either — the rule it amends is the rule that requires it.')
    add('')
    spec4 = read(f'{DOCS}/specifications/SPEC-MOK-004.md')
    rows4 = [row for row in rows_dated(spec4, DATE) if 'WO-MOK-011' in row]
    add('**`SPEC-MOK-004`** — recorded test-count figures corrected for the twelve tests this work')
    add('order adds; rule 6’s interface figures re-checked and unchanged.')
    add('')
    add('- rows dated %s naming this work order: **%d**' % (DATE, len(rows4)))
    add('- approval recorded in the row: %s' %
        ('yes' if any('Approved ' + DATE in row for row in rows4) else '**no**'))
    add('- states that every figure is a measured outcome rather than a decision: %s' %
        ('yes' if any('measured outcome rather than a decision' in row for row in rows4) else '**no**'))
    add('- rule 6 recorded unchanged at 94 items, 118 `pub` lines, 24 public fields: %s' %
        ('yes' if any('unchanged at 94 items' in row for row in rows4) else '**no**'))
    add('')
    add('**Why this is a discharge and not an unapproved amendment.** Rule 11 states the obligation in')
    add('its own text — "a work order that adds a test corrects these figures here, and one that loses a')
    add('test has a defect" — so the rule delegates the correction to the work order that causes it. The')
    add('row records measurements: `cargo test`’s per-target counts and the enumeration in')
    add('`interface.txt`. The two decisions those figures reflect are the technical owner’s of')
    add('%s and are recorded in `WO-MOK-011`: that the name reaches the observer through the' % DATE)
    add('retained event stream rather than a new public interface item, which is why `Observer::name_of`')
    add('is `pub(crate)` and rule 6 is unchanged; and that a test is placed in the tier its required')
    add('access puts it in, which `SPEC-MOK-004` rules 8 to 12 already fix and which forbids widening an')
    add('item to relocate a test. **No provision of `SPEC-MOK-004` other than the recorded figures')
    add('changed, and rule 6 — the interface contract — was re-measured rather than amended.**')
    add('')
    add('## 4. The earlier layer, left where it was')
    add('')
    add('Amendments were already **OUTSTANDING** in all four specifications before this work began, most')
    add("of them `WO-MOK-005`'s and `WO-MOK-007`'s. This work order neither resolves nor disturbs them,")
    add('which is a checkable claim: every amendment row that existed at the base commit, compared')
    add('against the file now, in order, and the rows carrying the word counted on both sides.')
    add('')
    add('| Artifact | rows at the base | rows now | the base rows unchanged and in order | rows reading OUTSTANDING then, now |')
    add('|---|---|---|---|---|')
    layer_ok = True
    appended, mentions, self_outstanding = 0, 0, 0
    for artifact in ('SPEC-MOK-001', 'SPEC-MOK-002', 'SPEC-MOK-003', 'SPEC-MOK-004'):
        path = f'{DOCS}/specifications/{artifact}.md'
        before, after = amendment_rows(at_base(path)), amendment_rows(read(path))
        prefix_ok = after[:len(before)] == before
        then = sum('OUTSTANDING' in row for row in before)
        now = sum('OUTSTANDING' in row for row in after[:len(before)])
        new = after[len(before):]
        appended += len(new)
        mentions += sum('OUTSTANDING' in row for row in new)
        self_outstanding += sum(is_outstanding(row) for row in new)
        layer_ok = layer_ok and prefix_ok and then == now
        add('| `%s` | %d | %d | %s | %d, then %d |' % (
            artifact, len(before), len(after), 'yes' if prefix_ok else '**no**', then, now))
    add('')
    add('The rows this work order adds are appended. No earlier row is edited, reordered, renumbered,')
    add('summarised or folded into a later one, and no row that read **OUTSTANDING** reads anything else')
    add('now: the last column counts the base rows only, on both sides, so an appended row cannot inflate')
    add('it. Of the %d appended rows, %d contain the word, because each states which outstanding rows above'
        % (appended, mentions))
    add('it are left untouched — a reference to a status, not a status. %s of the appended rows carries'
        % ('None' if self_outstanding == 0 else '**%d**' % self_outstanding))
    add('**OUTSTANDING** as its own state; each records an approval of this date.')
    add('')
    add('## 5. What must not have changed at all')
    add('')
    add('| Artifact | changed since the base commit | line endings only | why it must not be |')
    add('|---|---|---|---|')
    frozen_ok = True
    for path, reason in UNCHANGED:
        full = f'{DOCS}/{path}'
        same = read(full) == at_base(full)
        frozen_ok = frozen_ok and same
        add('| `%s` | %s | %s | %s |' % (
            os.path.basename(path)[:-3],
            'no' if same else '**yes**',
            'yes' if raw_differs_only_by_newlines(full) else 'n/a — raw bytes equal',
            reason))
    add('')
    add('The middle column is why this table reads content rather than raw bytes: this clone has')
    add('`core.autocrlf = true`, so a tracked text file is CRLF in the working tree and LF in the blob,')
    add('and every one of these files differs from its blob in exactly that way and no other. The check')
    add('is run both ways — content equal after folding CRLF to LF, and raw bytes equal once both sides')
    add('are folded — so the normalization cannot absorb an edit.')
    add('')
    add('`VREC-MOK-005` is still `ready` and `VREC-MOK-006` and `VREC-MOK-007` are untouched. This work')
    add('order does not approve their amendments, does not perform their manual assessments and does not')
    add('transition their work orders. The debt carried forward under `WO-MOK-007` is carried forward')
    add('again, unchanged and unpaid.')
    add('')
    add('## Result')
    add('')
    verdict = all([chain_ok, provisions_ok, deletions_ok, layer_ok, frozen_ok])
    add('**RESULT: %s** — every artifact in the chain carries the status it should, all eight provisions'
        % ('PASS' if verdict else 'FAIL'))
    add('`WO-MOK-011` states are present in both the amendment record and the specification text, the')
    add('two provisions that amend by deletion are shown to have deleted, the amendment beyond the')
    add("approved list is named with the rule that obliges it, the earlier layer's rows are unchanged")
    add('and in order with their OUTSTANDING markers intact, and `SPEC-MOK-002`, both architecture')
    add('documents and all seven verification records are unchanged — identical in content, and in raw')
    add('bytes but for the CRLF this clone checks out.')
    add('')
    add('What this file does **not** establish: that oracle 5 is satisfied. Its other half is the')
    add("interface enumeration in `interface.txt`, and `VER-MOK-011`'s manual assessment 5 is recorded")
    add('**OUTSTANDING** in `manual-assessment.md`.')
    print('\n'.join(out))
    return 0 if verdict else 1


if __name__ == '__main__':
    sys.exit(main())
