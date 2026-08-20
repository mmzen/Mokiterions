"""VER-MOK-012 oracle 7: are the amendments this change requires present and approved?

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-012/analysis/amendments.py \
        > docs/engineering/simulation/evidence/WO-MOK-012/amendment-approvals.md

`VER-MOK-012`'s oracle 7 is stated as "**Required amendments present and approved** ... `ADR-MOK-005`
accepted, and the `ARCH-MOK-001`, `SPEC-MOK-001` and `SPEC-MOK-002` amendments it requires approved,
before this change is verified. Absence fails this contract regardless of code state."

The failure this script exists to catch is not a missing signature line. It is a work order that
quietly amends a specification in its body without recording it, or records an amendment its body
never made. Both read as success to a reader who checks only one side, so every provision is looked
for twice, in disjoint text: once in the amendment record's 2026-08-20 row, and once in the document's
body with every amendment row removed. A record whose own prose satisfied a body check would
otherwise pass.

`WO-MOK-011/analysis/amendments.py` is this script's ancestor and the checks are the same kind. It is
not reused unmodified because its provision tables, chain and base commit are that work order's; the
helpers below are its helpers, which is why the two files' `read`, `at_base`, `amendment_rows`,
`is_outstanding` and `check_provisions` are line-for-line the same design.

Seven things are measured:

  1. the `status` and `updated` field of every artifact in the chain;
  2. each provision `ADR-MOK-005`'s *Required amendments* section states, in the record and in the body;
  3. the one provision that amends by deletion -- located by an anchor that must still be present, so
     that a capture which stopped short of the sentence fails instead of reporting a false absence;
  4. the provision counts each amendment row claims, against the bullets `ADR-MOK-005` actually lists,
     because a row claiming twelve while the ADR lists thirteen is a discrepancy in one of the two;
  5. `SPEC-MOK-004`, amended beyond the approved list, with the rule that obliges it and the plain
     record that no owner approved its inclusion;
  6. the earlier layer: every amendment row that existed at the base commit, in order, and the
     OUTSTANDING markers then and now;
  7. the artifacts that must not have changed at all.

`self_test` injects each failure mode and asserts that it is reported, because a check that finds
nothing reads exactly like a check that looks for nothing. It runs before any result is printed.
"""

import io
import os
import re
import subprocess
import sys

BASE = 'de33d7440c323a98ac88db3fabaf87bea48ebf4e'
DATE = '2026-08-20'
DOCS = 'docs/engineering/simulation'

# Every artifact WO-MOK-012 names, with the status it must carry. A work order in flight is
# `in_progress` and not `complete`: a work order is closed by a verification record that binds a
# commit, and that record is written after the commit it names.
CHAIN = [
    ('INT-MOK-009', 'intent', 'approved'),
    ('CAP-MOK-009', 'capabilities', 'approved'),
    ('REQ-MOK-042', 'requirements', 'approved'),
    ('REQ-MOK-043', 'requirements', 'approved'),
    ('REQ-MOK-044', 'requirements', 'approved'),
    ('REQ-MOK-045', 'requirements', 'approved'),
    ('REQ-MOK-046', 'requirements', 'approved'),
    ('ADR-MOK-005', 'architecture/adr', 'approved'),
    ('SPEC-MOK-006', 'specifications', 'approved'),
    ('VER-MOK-012', 'verification', 'approved'),
    ('WO-MOK-012', 'work-orders', 'in_progress'),
    ('ARCH-MOK-001', 'architecture', 'approved'),
    ('SPEC-MOK-001', 'specifications', 'approved'),
    ('SPEC-MOK-002', 'specifications', 'approved'),
    ('SPEC-MOK-004', 'specifications', 'approved'),
]

# The three the owner approved, and the path each lives at.
AMENDED = {
    'ARCH-MOK-001': 'architecture/ARCH-MOK-001.md',
    'SPEC-MOK-001': 'specifications/SPEC-MOK-001.md',
    'SPEC-MOK-002': 'specifications/SPEC-MOK-002.md',
}

# Every provision, with the phrases that must appear in the amendment record's row and the phrases
# that must appear in the document's body. The two lists are deliberately different strings: a row
# that merely quoted the body would not satisfy the row check, and the body is searched with every
# row removed.
PROVISIONS = {
    'ARCH-MOK-001': [
        ('1. *Components* item 1 gains the entry point’s sink duties',
         ["item 1 gains the entry point's sink duties",
          'removing a file it created on failure'],
         ['it additionally resolves the optional record sink',
          'removes a file it created when the run fails',
          "Every one of those duties is the entry point's alone"]),
        ('2. *Components* item 2 gains the counters and record production',
         ['Item 2 gains the engine',
          'cumulative measurement counters'],
         ["it additionally owns the run's cumulative measurement counters",
          'Records are produced by the owner of the facts they state']),
        ('3. The observation-surface paragraph gains the host-supplied sink',
         ['fifth responsibility of the engine package and not a fourth component',
          'library target performs no filesystem operation'],
         ['**host-supplied record sink**',
          '**The library target performs no filesystem operation.**',
          'it never sees a path — the sink reaches it already open']),
        ('4. *Dependency direction* distinguishes a destination from persistence',
         ['distinguishes an output destination from persistence of state',
          'nothing is read back'],
         ['**output destination and not persistence of state**',
          'no state survives the process in a form the engine consumes',
          'as stateless on its next start as a run that does not']),
        ('5. *Data and control flow* gains the second, optional branch',
         ['ordered event -> record projection -> host-supplied sink',
          'text branch unchanged and unconditional'],
         ['ordered event -> record projection -> host-supplied sink',
          'the second branch is **optional**',
          '**mutates no simulation state and draws no entropy**']),
        ('6. *Prohibited patterns* gains three',
         ['no filesystem operation in the library target',
          'no entropy draw from a record-writing path',
          'free-text field in the stream'],
         ['**any filesystem operation in the engine package',
          '**any draw against the entropy stream from a record-writing path.**',
          'whose value is operator-supplied, environment-derived or free text**']),
        ('7. *Determinism* extends to the record stream',
         ['configuring a sink moves neither the text bytes nor the draw sequence'],
         ['byte-identical structured records',
          "configuring a sink changes neither the text stream's bytes nor the entropy draw sequence"]),
        ('8. *Debuggability* extends to structured recording',
         ['extends from action tracing to structured recording'],
         ['optional structured recording likewise exposes a run',
          'neither is a mode the program runs differently in']),
        ('9. *Conformance checks* gains four',
         ['**Conformance checks** gains four',
          'checked exhaustively'],
         ['no `std::fs`, no `File`, no `OpenOptions`, no `remove_file`',
          "**text stream's bytes are identical with and without a sink**",
          '**per-tick entropy draw sequence is identical with and without a sink**',
          "member of `SPEC-MOK-006` rule 3.2's enumeration**"]),
        ('10. *Related architecture and ADRs* gains `ADR-MOK-005`',
         ['**Related architecture and ADRs** gains `ADR-MOK-005`'],
         ['`ADR-MOK-005` decides the record sink',
          '**It supersedes nothing.**']),
        ('11. `addresses` gains two requirements; `conforms_to` gains `SPEC-MOK-006`',
         ['`addresses` gains `REQ-MOK-042` and `REQ-MOK-045`',
          '`conforms_to` gains `SPEC-MOK-006`'],
         ['"REQ-MOK-042"', '"REQ-MOK-045"', '"SPEC-MOK-006"']),
        ('12. `decision_assessment.rationale` records the decisions and stays `adr_required`',
         ['records the three decisions `ADR-MOK-005` makes',
          'stays `adr_required`'],
         ['**public-interface-or-protocol**',
          '**material-alternatives** triggers',
          'covered by `ADR-MOK-001` through `ADR-MOK-005` together']),
    ],
    'SPEC-MOK-001': [
        ('1. *Scope* stops excluding structured output and names `SPEC-MOK-006`',
         ['*Scope* stops excluding structured output',
          'while keeping persistence excluded'],
         ['Structured output was on the excluded list above and is removed from it',
          'projection of the output this specification fixes',
          '**Persistence stays on the excluded list.**']),
        ('2. *Actors* adds the filesystem as a destination, never a source',
         ['*Actors* adds the filesystem as a destination'],
         ['the filesystem is a destination for the optional record stream',
          'no filesystem location is a source of engine input']),
        ('3. *Inputs* takes `--events-path` in the synopsis and one bullet',
         ['`--events-path <path>` in the synopsis and one bullet',
          'classified as a runtime failure'],
         ['[--events-path <path>]',
          'names the destination of the structured record stream',
          'may appear at most once']),
        ('4. *Help output* gains the option’s entry, in order',
         ["*Help output* gains the option's entry between `--trace-actions` and `--help`"],
         ['`--events-path`, and `--help`, in that order',
          "`--events-path`'s row was added 2026-08-20",
          'It sits between `--trace-actions` and `--help`']),
        ('5. *Outputs* adds the stream and leaves the text stream unaffected',
         ['*Outputs* adds the stream',
          'the exit-code list is unchanged'],
         ['only when `--events-path` is given',
          'are stated nowhere else',
          'the 2026-08-20 amendment adds none']),
        ('6. *Error and recovery behavior* adds the sink failures and the removal',
         ['stops the run before any tick',
          'a file the process created is removed'],
         ['before the first entropy draw and before any text observation record',
          'no partial stream is left behind to read as a complete run',
          'is not removed, because removing an operator']),
        ('7. *Security and privacy properties* records the one path, and what no record carries',
         ['the one input interpreted as a path',
          'no record carries a path, a clock, a host, a user'],
         ['the one operator-supplied value that is interpreted as a filesystem path',
          'never as c']),
        ('8. *Performance and capacity* records the stream as write-only',
         ['write-only, linear in the run and flat in memory'],
         ['never reads back']),
        ('9. *Observability* adds a byte-identical record stream',
         ['adds byte-identical records for identical trace and sink configuration'],
         ['byte-identical record stream']),
        ('10. *Compatibility and migration* names the schema version',
         ["names the stream's own schema version",
          'no existing behavior, default or exit code changes'],
         ['carries its own schema version, governed by `SPEC-MOK-006` rule 10',
          '**No existing behavior, default or exit code changes.**']),
        ('11. *Explicitly unspecified decisions* records the stream as governed',
         ['governed rather than delegated'],
         ['are **not** unspecified',
          'whether it created the destination file']),
    ],
    'SPEC-MOK-002': [
        ('1. Rule 4 carries the literal signature with the one new parameter',
         ['`execute` gains exactly one parameter',
          'and nothing else'],
         ['records: Option<&mut dyn Write>',
          'a sink the caller owns']),
        ("2. Rule 5's `execute` row reworded, the enumeration otherwise untouched",
         ['the `execute` row reworded from "two writers" to "the caller\'s writers"',
          'a parameter is not an item'],
         ["Maps arguments and the caller's writers to an exit code"]),
        ("3. Rule 5's mechanical check restated as two greps",
         ['restated as two greps',
          'run_recording'],
         ['the mechanical form is two greps rather than one',
          "grep -n 'records: Option<&mut dyn Write>'",
          'is crate-private, is not on the interface']),
        ('4. Rule 6 **not** amended, and the omission recorded at the rule',
         ['**Rule 6**: **not** amended, and the omission recorded at the rule'],
         ['**Not amended on 2026-08-20',
          'stays on the second bullet, private in every build configuration']),
        ('5. *Scope* and *Compatibility and migration* name `SPEC-MOK-006`',
         ['`SPEC-MOK-006` named as the authority on the stream and this specification as the authority on the seam',
          'four `execute` call sites listed'],
         ['is the authority on what a record contains',
          '`mokiterions-core/src/main.rs`, `mokiterions-core/tests/process.rs`',
          'the observer supplies no sink']),
    ],
}

# The one provision that amends by deletion. No phrase search can show that a phrase is gone, so the
# sentence is located first and its contents are asserted, anchored on a phrase that must still be at
# the far end of the list being inspected -- without the anchor a truncated capture would report a
# false absence.
DELETIONS = [
    ('SPEC-MOK-001',
     "*Scope*'s excluded list no longer names structured output",
     r'It does not define OpenAI integration[^\n]*\n',
     'user interface',
     ['structured output', 'structured record']),
]

# Amended beyond the approved list, with the rule that obliges it.
BEYOND = [
    ('SPEC-MOK-004', 'specifications/SPEC-MOK-004.md',
     'rule 11 records the workspace test census and delegates its own correction',
     ['Recorded test-count figures corrected for `WO-MOK-012`',
      'a work order that adds a test corrects these figures here',
      'from 212 to **246**',
      'from 85 to **119**',
      "the observer's stays at **127**",
      '**0 removals**',
      '**Rules 9 and 10 are unchanged**',
      '**Rule 6 is unchanged at 94 items, 118 `pub` lines and 24 public fields**',
      'was not among the three amendments',
      '**OUTSTANDING**'],
     ['As corrected for `WO-MOK-012`',
      "the observer's total is **127**",
      "the engine's is **119**",
      "the workspace's is **246**",
      'The thirty-four arrivals',
      "The engine's split is 68 internal and 51 public",
      "Rule 6's interface is unchanged at **94** items"]),
]

# Places where a figure or an attribution in one artifact does not agree with another. Each is
# pre-declared here with the assertions that establish it, so that a disagreement is reported as the
# disagreement it is rather than either failing the oracle or passing unmentioned. An undeclared
# disagreement is a finding; a declared one whose assertions do not hold is also a finding, because
# otherwise this list would be a way to silence a check rather than a way to record a fact.
DISCREPANCIES = [
    {
        'id': "`ADR-MOK-005` attributes the signature to rule 5, and the signature is rule 4's",
        'reconciles': 'SPEC-MOK-002',
        'assertions': [
            ('`ADR-MOK-005` names rule 5 for the signature', 'architecture/adr/ADR-MOK-005.md',
             "Rule 5's enumeration: `execute`'s signature gains one optional sink parameter"),
            ('`SPEC-MOK-002` rule 5 places the signature in rule 4',
             'specifications/SPEC-MOK-002.md',
             "`execute`'s signature is enumerated by rule 4 and by nothing else"),
            ('the amendment row records rule 4 as amended', 'specifications/SPEC-MOK-002.md',
             '**Rule 4**: `execute` gains exactly one parameter'),
            ('the amendment row records rule 5 as amended separately',
             'specifications/SPEC-MOK-002.md',
             '**Rule 5**: the `execute` row reworded'),
        ],
        'reading': [
            'The substance is present and is more than the ADR named. Because the signature literal is',
            "rule 4's and the enumeration row is rule 5's, one bullet had to become two amendments, and the",
            'row states them as two — which is why it claims five where the ADR lists four. The ADR was',
            'written by the implementation agent and this is an imprecision in its location-naming, not a',
            'provision anybody approved and nobody made. Recorded in `completion-summary.md`.',
        ],
    },
    {
        'id': '`ADR-MOK-005` counts the `SPEC-MOK-001` list as nine and lists eleven provisions',
        'reconciles': None,
        'assertions': [
            ('the prose states the arithmetic', 'architecture/adr/ADR-MOK-005.md',
             'has grown from the five seams named in `docs/PHASE_4_PROPOSAL.md` to nine'),
            ('it names the four it added', 'architecture/adr/ADR-MOK-005.md',
             'The additional four are'),
        ],
        'reading': [
            'Five seams plus four additions is nine, and the list below that sentence carries eleven',
            'provisions. The two the arithmetic does not account for are *Security and privacy properties*',
            "and *Performance and capacity*, and both are in the ADR's list, in `SPEC-MOK-001`'s amendment",
            'row and in its body — provisions 7 and 8 of section 2 above. So the defect is in the sentence',
            'that counts them, not in the work. Recorded in `completion-summary.md`.',
        ],
    },
]

# Artifacts that must be identical to the base commit, with the reason each must be.
UNCHANGED = [
    ('specifications/SPEC-MOK-003.md',
     'the observer behavior contract; this work order adds a stream the observer does not read'),
    ('specifications/SPEC-MOK-005.md', 'not in this chain; not opened'),
    ('architecture/ARCH-MOK-002.md',
     "the observer architecture; outside `ARCH-MOK-001`'s boundary and outside this change"),
    ('architecture/adr/ADR-MOK-001.md',
     'the trust boundary `ADR-MOK-005` supersedes nothing of'),
    ('architecture/adr/ADR-MOK-002.md', 'the enumerated-interface decision; not re-opened'),
    ('architecture/adr/ADR-MOK-003.md', 'the two-package split; not re-opened'),
    ('architecture/adr/ADR-MOK-004.md', 'not in this chain; not opened'),
] + [('verification-records/VREC-MOK-%03d.md' % n, 'commit-bound record; not re-opened')
     for n in range(1, 12)]


def read(path):
    """The file's text with line endings normalized.

    This clone has `core.autocrlf = true`, so a tracked text file is CRLF in the working tree and LF
    in the blob `git show` prints. Every comparison here is therefore a comparison of content after
    newline normalization, not of raw bytes; `raw_differs_only_by_newlines` is what checks that the
    distinction is the whole difference, so the normalization cannot hide a real edit.
    """
    return io.open(path, encoding='utf-8', newline='').read().replace('\r\n', '\n')


def blob(path):
    """The file's bytes at the base commit.

    `git show` takes a path with forward slashes on every platform, and `os.path.join` produces
    backslashes here, so the separator is normalized rather than left to the caller.
    """
    out = subprocess.run(['git', 'show', '%s:%s' % (BASE, path.replace(os.sep, '/'))],
                         capture_output=True, check=True)
    return out.stdout


def at_base(path):
    return blob(path).decode('utf-8').replace('\r\n', '\n')


def raw_differs_only_by_newlines(path):
    """True when the working-tree bytes equal the blob's once CRLF is folded to LF, and they differ."""
    raw = io.open(path, 'rb').read()
    committed = blob(path)
    return raw != committed and raw.replace(b'\r\n', b'\n') == committed.replace(b'\r\n', b'\n')


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
    return len(cells) >= 2 and cells[-2].startswith('**OUTSTANDING')


def rows_dated(text, date):
    return [row for row in amendment_rows(text) if row.startswith('| ' + date + ' |')]


def body_without_record(text):
    """The document with every amendment row removed, so the record cannot satisfy a body check."""
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
        results.append({
            'label': label,
            'record': (len([p for p in in_record if p in record]), len(in_record)),
            'body': (len([p for p in in_body if p in body]), len(in_body)),
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


def adr_bullets(adr_text, heading):
    """The top-level bullets ADR-MOK-005's *Required amendments* lists under one document's heading.

    Continuation lines of a wrapped bullet start with spaces, so only lines beginning `- ` count. The
    last bullet of each list is the instruction to add an amendment-record row, which is the record of
    the amendment rather than a provision of it; both figures are reported so that the distinction is
    visible rather than folded in.
    """
    section = adr_text.split('## Required amendments', 1)[-1]
    part = section.split('### `%s`' % heading, 1)
    if len(part) == 1:
        return 0, 0
    rest = part[1].split('\n### ', 1)[0].split('\n## ', 1)[0]
    bullets = [line for line in rest.split('\n') if line.startswith('- ')]
    record_rows = [b for b in bullets if 'Amendment record' in b or 'amendment-record row' in b]
    return len(bullets), len(bullets) - len(record_rows)


WORDS = {'twelve': 12, 'eleven': 11, 'five': 5, 'nine': 9, 'thirteen': 13}


def claimed_provisions(row):
    """The provision count an amendment row claims in words, or None."""
    match = re.search(r'\b(%s) provisions\b' % '|'.join(WORDS), row, re.I)
    return WORDS[match.group(1).lower()] if match else None


def self_test():
    """Inject each failure mode and assert it is reported. Returns a list of (control, ok)."""
    controls = []

    good = ('| 2026-08-20 | states the provision, WO-MOK-012 | Approved |\n'
            'the body carries the provision\n')
    ok = check_provisions(good, [('control', ['states the provision'], ['body carries'])],
                          'WO-MOK-012')[0]
    controls.append(('a provision stated in the row and carried by the body is reported clean',
                     ok['record'] == (1, 1) and ok['body'] == (1, 1)))

    silent = '| 2026-08-20 | says nothing, WO-MOK-012 | Approved |\nthe body carries the provision\n'
    ok = check_provisions(silent, [('control', ['states the provision'], ['body carries'])],
                          'WO-MOK-012')[0]
    controls.append(('a provision the record does not state is reported', ok['record'] == (0, 1)))

    absent = '| 2026-08-20 | states the provision, WO-MOK-012 | Approved |\nan unrelated body\n'
    ok = check_provisions(absent, [('control', ['states the provision'], ['body carries'])],
                          'WO-MOK-012')[0]
    controls.append(('a provision absent from the body is reported', ok['body'] == (0, 1)))

    only_row = '| 2026-08-20 | states the provision and body carries, WO-MOK-012 | Approved |\n'
    ok = check_provisions(only_row, [('control', ['states the provision'], ['body carries'])],
                          'WO-MOK-012')[0]
    controls.append(("the record's own prose cannot satisfy a body check", ok['body'] == (0, 1)))

    result = check_deletion('nothing here', r'^It does not define[^\n]*\n', 'anchor', ['x'])
    controls.append(('a deletion check whose sentence is not found is reported, not passed',
                     result['located'] is False))

    truncated = 'It does not define OpenAI integration, combat\n'
    result = check_deletion(truncated, r'It does not define[^\n]*\n', 'user interface', ['x'])
    controls.append(('a deletion check truncated before its anchor is reported, not passed',
                     result['located'] and not result['anchored']))

    still = 'It does not define OpenAI integration, structured output, or a user interface\n'
    result = check_deletion(still, r'It does not define[^\n]*\n', 'user interface',
                            ['structured output'])
    controls.append(('a forbidden phrase still present in the sentence is reported',
                     result['still_present'] == ['structured output']))

    rows = amendment_rows('| 2026-08-19 | a |\n| 2026-08-20 | b |\nprose\n')
    controls.append(('the row reader finds rows in order and ignores prose',
                     len(rows) == 2 and rows[0].startswith('| 2026-08-19')))

    own = '| 2026-08-18 | amended something | **OUTSTANDING.** Requires the technical owner. |'
    other = '| 2026-08-20 | the row above marked **OUTSTANDING** is untouched | Approved 2026-08-20. |'
    controls.append(('a row outstanding in its own status cell is told from one that mentions the word',
                     is_outstanding(own) and not is_outstanding(other)))

    sample = ('## Required amendments\n### `X`\n- one\n  wrapped continuation\n- two\n'
              '- *Amendment record*: one new row, dated on approval.\n### `Y`\n- three\n')
    controls.append(('the ADR bullet counter ignores wrapped continuations and separates the record row',
                     adr_bullets(sample, 'X') == (3, 2) and adr_bullets(sample, 'Y') == (1, 1)))

    controls.append(('a row claiming a count in words is read, and one claiming none returns None',
                     claimed_provisions('| d | Twelve provisions. |') == 12
                     and claimed_provisions('| d | no claim |') is None))

    # The declared-discrepancy list is the one place a disagreement does not fail the oracle, so its
    # width is pinned here: exactly one artifact may reconcile a count mismatch this way, and every
    # declaration must carry assertions. A declaration without assertions would be an excuse.
    controls.append(('exactly one artifact has a declared count discrepancy, so the others cannot use it',
                     sorted(d['reconciles'] for d in DISCREPANCIES if d['reconciles'])
                     == ['SPEC-MOK-002']))
    controls.append(('every declared discrepancy carries at least two assertions and a reading',
                     all(len(d['assertions']) >= 2 and d['reading'] for d in DISCREPANCIES)))

    return controls


def main():
    sys.stdout.reconfigure(encoding='utf-8', newline='\n')
    controls = self_test()
    if not all(ok for _, ok in controls):
        for label, ok in controls:
            print(('ok   ' if ok else 'FAIL ') + label, file=sys.stderr)
        return 2

    adr = read(os.path.join(DOCS, 'architecture/adr/ADR-MOK-005.md'))
    findings = []
    out = []
    add = out.append

    add('# WO-MOK-012 amendment approvals — oracle 7')
    add('')
    add("`VER-MOK-012`'s seventh oracle is the governance state of the artifacts this change amends:")
    add('*"`ADR-MOK-005` accepted, and the `ARCH-MOK-001`, `SPEC-MOK-001` and `SPEC-MOK-002`')
    add('amendments it requires approved, before this change is verified. Absence fails this contract')
    add('regardless of code state."* This file is generated by `analysis/amendments.py`, which reads')
    add('the artifacts and the git history; what it checks, and why each check is not a formality, is')
    add("in that script's header. The engine's public interface, which is the other half of the claim")
    add('that the interface grows by one parameter and no item, is `interface.txt`.')
    add('')
    add('Everything below is measured against `%s`, the commit' % BASE)
    add('this work started from, and every row this work order adds is dated `%s`.' % DATE)
    add('')

    add("## 1. The chain's governance state")
    add('')
    add('| Artifact | status at the base | status now | expected | updated | ok |')
    add('|---|---|---|---|---|---|')
    for name, folder, expected in CHAIN:
        path = os.path.join(DOCS, folder, name + '.md')
        text = read(path)
        status, updated = field(text, 'status'), field(text, 'updated')
        good = status == expected
        note = expected
        if name in AMENDED:
            note = expected + ', amended'
        elif name == 'SPEC-MOK-004':
            note = expected + ', amended beyond the list'
        add('| `%s` | `%s` | `%s` | `%s` | %s | %s |'
            % (name, field(at_base(path), 'status'), status, note, updated,
               'yes' if good else '**NO**'))
        if not good:
            findings.append('%s carries status `%s`, expected `%s`' % (name, status, expected))
    add('')
    add('The second column is measured rather than asserted, and it is what separates the packet this')
    add('work order implements from the documents it amends: everything drafted for this chain was')
    add('`draft` at the base commit and was approved afterwards, in one act on 2026-08-20, while the')
    add('four documents that already existed were already `approved` and are amended in place. The work')
    add('order is `in_progress` and not `complete` because a work order is closed by a verification')
    add('record that binds a commit, and that record is written after the commit it names.')
    add('')

    add('## 2. Each provision, in the record and in the body')
    add('')
    add("Every provision `ADR-MOK-005`'s *Required amendments* section states is looked for twice — in")
    add("the amendment record's %s row, and in the document's body with every amendment row" % DATE)
    add('removed — because a record that claimed an amendment the text does not carry would otherwise')
    add('satisfy both checks with one sentence.')
    add('')
    for name, path in AMENDED.items():
        text = read(os.path.join(DOCS, path))
        rows = rows_dated(text, DATE)
        mine = [row for row in rows if 'WO-MOK-012' in row]
        add('### `%s`' % name)
        add('')
        add('- rows dated %s: **%d**, of which **%d** name this work order'
            % (DATE, len(rows), len(mine)))
        add('- approval recorded in the row: %s'
            % ('yes' if any('Approved ' + DATE in r for r in mine) else '**NO**'))
        add('- records that the implementation agent wrote the text and decided no substance: %s'
            % ('yes' if any('did not decide the substance' in r for r in mine) else '**NO**'))
        add('')
        add('| Provision | in the record | in the text |')
        add('|---|---|---|')
        for result in check_provisions(text, PROVISIONS[name], 'WO-MOK-012'):
            r_got, r_all = result['record']
            b_got, b_all = result['body']
            add('| %s | %s | %s |'
                % (result['label'],
                   'yes' if r_got == r_all else '**%d/%d**' % (r_got, r_all),
                   '%d/%d phrases' % (b_got, b_all) if b_got == b_all
                   else '**%d/%d phrases**' % (b_got, b_all)))
            for phrase in result['missing_record']:
                findings.append('%s: the row does not state %r' % (name, phrase))
            for phrase in result['missing_body']:
                findings.append('%s: the body does not carry %r' % (name, phrase))
        add('')

    add('### The one provision that amends by deletion')
    add('')
    add('No phrase search can show that a phrase is gone. The sentence that carried it is located')
    add('instead and its contents asserted, anchored on a phrase that must still be at the far end of')
    add('the list being inspected, so that a capture which stopped short fails rather than reporting an')
    add('absence it never looked for.')
    add('')
    add('| Artifact | sentence | located, and the anchor in it | no longer names |')
    add('|---|---|---|---|')
    for name, label, pattern, anchor, forbidden in DELETIONS:
        text = read(os.path.join(DOCS, AMENDED[name]))
        result = check_deletion(text, pattern, anchor, forbidden)
        located = ('yes, %d characters through `%s`' % (result['chars'], anchor)
                   if result['located'] and result['anchored'] else '**NO**')
        gone = 'yes' if result['located'] and not result['still_present'] else '**NO**'
        add('| `%s` | %s | %s | %s |' % (name, label, located, gone))
        if not (result['located'] and result['anchored']):
            findings.append('%s: the deletion check could not locate or anchor %r' % (name, label))
        for word in result['still_present']:
            findings.append('%s: %r still appears in the sentence it was removed from' % (name, word))
    add('')

    add('All %d controls on the checks themselves held, so no line above is a check that looked for'
        % len(controls))
    add('nothing:')
    add('')
    for label, _ in controls:
        add('- ok — ' + label)
    add('')

    add('## 3. The provision counts each row claims, against what the ADR lists')
    add('')
    add("A row that claims twelve provisions while the ADR lists thirteen bullets is a discrepancy in")
    add('one of the two, and it is worth measuring because the count is the only figure a reader can')
    add("check quickly. The ADR's last bullet for each document is the instruction to add an")
    add('amendment-record row, which is the record of the amendment rather than a provision of it, so')
    add('both figures are reported.')
    add('')
    add('| Artifact | bullets in `ADR-MOK-005` | of those, provisions | the row claims | this file checks | agrees |')
    add('|---|---|---|---|---|---|')
    declared = {d['reconciles'] for d in DISCREPANCIES if d['reconciles']}
    for name, path in AMENDED.items():
        bullets, provisions = adr_bullets(adr, name)
        text = read(os.path.join(DOCS, path))
        mine = [r for r in rows_dated(text, DATE) if 'WO-MOK-012' in r]
        claim = claimed_provisions('\n'.join(mine))
        tabled = len(PROVISIONS[name])
        agrees = claim == provisions == tabled
        add('| `%s` | %d | %d | %s | %d | %s |'
            % (name, bullets, provisions, claim if claim is not None else '—', tabled,
               'yes' if agrees else ('disclosed below' if name in declared else '**NO**')))
        if not agrees and name not in declared:
            findings.append('%s: the row claims %s provisions, the ADR lists %d, this file checks %d,'
                            ' and no discrepancy is declared for it'
                            % (name, claim, provisions, tabled))
    add('')
    add('`ARCH-MOK-001` and `SPEC-MOK-001` agree on all three counts. `SPEC-MOK-002` does not, and the')
    add('disagreement is declared rather than absorbed.')
    add('')

    add('### Where the artifacts disagree with each other')
    add('')
    add('Each disagreement below is pre-declared in the script with the assertions that establish it. An')
    add('undeclared one is a finding, and a declared one whose assertions do not hold is also a finding —')
    add('otherwise this section would be a way to silence a check rather than a way to record a fact.')
    add('')
    for item in DISCREPANCIES:
        add('**%s.**' % item['id'])
        add('')
        for label, path, phrase in item['assertions']:
            text = read(os.path.join(DOCS, path))
            present = phrase in text
            add('- %s — %s' % (label, 'confirmed' if present else '**NOT CONFIRMED**'))
            if not present:
                findings.append('a declared discrepancy does not hold: %r is not in %s'
                                % (phrase, path))
        add('')
        for line in item['reading']:
            add(line)
        add('')

    return out, findings, controls, adr


def report():
    result = main()
    if isinstance(result, int):
        return result
    out, findings, controls, adr = result
    add = out.append

    add('## 4. Amended beyond the approved list')
    add('')
    add('`ADR-MOK-005` names three documents to amend and `WO-MOK-012` makes all three approval')
    add('preconditions. A fourth was amended, and it is neither silent nor approved.')
    add('')
    for name, path, obliged_by, in_record, in_body in BEYOND:
        text = read(os.path.join(DOCS, path))
        rows = [r for r in rows_dated(text, DATE) if 'WO-MOK-012' in r]
        record = '\n'.join(rows)
        body = body_without_record(text)
        add('**`%s`** — %s.' % (name, obliged_by))
        add('')
        add('- rows dated %s naming this work order: **%d**' % (DATE, len(rows)))
        missing_record = [p for p in in_record if p not in record]
        missing_body = [p for p in in_body if p not in body]
        add('- the row states all %d of its required phrases: %s'
            % (len(in_record), 'yes' if not missing_record else '**NO**'))
        add('- the body carries all %d of its required figures: %s'
            % (len(in_body), 'yes' if not missing_body else '**NO**'))
        add('- the row records the amendment as **not approved** rather than claiming approval: %s'
            % ('yes' if 'is not claimed as approved' in record else '**NO**'))
        add('')
        for phrase in missing_record:
            findings.append('%s: the row does not state %r' % (name, phrase))
        for phrase in missing_body:
            findings.append('%s: the body does not carry %r' % (name, phrase))
    add('**Why this is disclosed rather than resolved.** Rule 11 states the obligation in its own text')
    add('— "a work order that adds a test corrects these figures here, and one that loses a test has a')
    add('defect" — so the rule delegates the correction to the work order that causes it, and every')
    add("figure in the row is a measured outcome rather than a decision: `cargo test`'s per-target")
    add('counts in `gates.txt`, the name-by-name reconciliation in')
    add('`analysis/census-reconciliation.txt`, and the enumeration in `interface.txt`. What the rule')
    add('cannot delegate is the judgement that `SPEC-MOK-004` belongs in this chain at all. The owner')
    add('approved three amendments on 2026-08-20 and `SPEC-MOK-004` was not among them, so its row')
    add("records the amendment as **OUTSTANDING** for the owner's confirmation rather than claiming an")
    add('approval nobody gave. `WO-MOK-011` faced the same situation on the same rule and recorded it')
    add('the same way, which is the precedent this follows rather than a new reading.')
    add('')

    add('## 5. The earlier layer, left where it was')
    add('')
    add('Amendments were already **OUTSTANDING** in three of these four documents before this work')
    add('began. This work order neither resolves nor disturbs any of them, which is a checkable claim:')
    add('every amendment row that existed at the base commit, compared against the file now, in order,')
    add('and the rows whose own status cell declares them outstanding counted on both sides.')
    add('')
    add('| Artifact | rows at the base | rows now | the base rows unchanged and in order | rows reading OUTSTANDING then, now |')
    add('|---|---|---|---|---|')
    layered = dict(AMENDED)
    layered['SPEC-MOK-004'] = 'specifications/SPEC-MOK-004.md'
    carried = []
    for name in ['ARCH-MOK-001', 'SPEC-MOK-001', 'SPEC-MOK-002', 'SPEC-MOK-004']:
        path = layered[name]
        before = amendment_rows(at_base(os.path.join(DOCS, path)))
        now = amendment_rows(read(os.path.join(DOCS, path)))
        prefix_ok = now[:len(before)] == before
        out_before = [r for r in before if is_outstanding(r)]
        out_now = [r for r in now[:len(before)] if is_outstanding(r)]
        add('| `%s` | %d | %d | %s | %d, then %d |'
            % (name, len(before), len(now), 'yes' if prefix_ok else '**NO**',
               len(out_before), len(out_now)))
        if not prefix_ok:
            findings.append('%s: a row that existed at the base commit was edited or reordered' % name)
        if len(out_before) != len(out_now):
            findings.append('%s: an OUTSTANDING row changed state' % name)
        for row in out_now:
            cells = [cell.strip() for cell in row.split('|')]
            carried.append((name, cells[1], cells[2], cells[-2]))
    add('')
    add('The %s rows still outstanding, each quoted from its own subject cell so that this file names'
        % {1: 'one', 2: 'two', 3: 'three', 4: 'four'}.get(len(carried), len(carried)))
    add('what it carries forward rather than counting it:')
    add('')
    for name, date, subject, status in carried:
        first = re.split(r'(?<=\.)\s', subject.strip())[0].strip()
        if len(first) > 150:
            first = first[:147].rstrip() + '...'
        # Only the status cell is read for the attribution. A subject cell may name a work order for
        # an unrelated reason -- `SPEC-MOK-004`'s row names `WO-MOK-006` as the scope of a byte-identity
        # check, not as its own author -- so a fallback to the subject would invent an attribution.
        owed = re.findall(r'WO-MOK-\d+', status)
        add('- `%s`, %s — %s%s'
            % (name, date, first,
               ' It belongs to `%s`.' % owed[0] if owed
               else ' Its status cell names no work order.'))
    add('')
    add('Each awaits the technical owner, which the repository owner also is, and none of them is this')
    add("work order's to pay. They are named here because carrying an outstanding amendment forward is a")
    add('fact about the governance state this change is verified against, and a count alone would let')
    add('the reader believe they had been looked at.')
    add('')
    add('The rows this work order adds are appended. No earlier row is edited, reordered, renumbered,')
    add('summarised or folded into a later one, and no row that read **OUTSTANDING** reads anything')
    add('else now: the last column counts the base rows only, on both sides, so an appended row cannot')
    add('inflate it. Each appended row states which outstanding rows above it are left untouched — a')
    add("reference to a status, not a status — and `is_outstanding` reads the row's own status cell")
    add('rather than searching the row for the word, which is the control that tells the two apart.')
    add('')

    add('## 6. What must not have changed at all')
    add('')
    add('| Artifact | changed since the base commit | line endings only | why it must not be |')
    add('|---|---|---|---|')
    for path, why in UNCHANGED:
        full = os.path.join(DOCS, path)
        same = read(full) == at_base(full)
        newline_only = raw_differs_only_by_newlines(full)
        add('| `%s` | %s | %s | %s |'
            % (os.path.basename(path).replace('.md', ''),
               'no' if same else '**YES**',
               'yes' if newline_only else 'no, identical bytes',
               why))
        if not same:
            findings.append('%s changed since the base commit and must not have' % path)
    add('')
    add('The middle column is why this table reads content rather than raw bytes: this clone has')
    add('`core.autocrlf = true`, so a tracked text file is CRLF in the working tree and LF in the blob,')
    add('and every one of these files differs from its blob in exactly that way and no other. The check')
    add('is run both ways — content equal after folding CRLF to LF, and raw bytes equal once both sides')
    add('are folded — so the normalization cannot absorb an edit.')
    add('')
    add('No verification record is re-opened. `VREC-MOK-001` through `VREC-MOK-011` stand as they are,')
    add('and the three amendments listed as outstanding in section 5 are carried forward again,')
    add('unchanged and unpaid.')
    add('')

    add('## Result')
    add('')
    if findings:
        add('**RESULT: FAIL** — %d finding%s:' % (len(findings), '' if len(findings) == 1 else 's'))
        add('')
        for finding in findings:
            add('- ' + finding)
    else:
        add('**RESULT: PASS** — `ADR-MOK-005` is `approved`, every artifact in the chain carries the')
        add('status it should, all twenty-eight provisions are present in both the amendment record and')
        add('the document text, the provision that amends by deletion is shown to have deleted, the')
        add('amendment beyond the approved list is named with the rule that obliges it and recorded as')
        add("unapproved rather than claimed, the earlier layer's rows are unchanged and in order with")
        add('their three OUTSTANDING markers intact and named, and the eighteen documents that must not')
        add('have moved are identical in content — and in raw bytes but for the CRLF this clone checks')
        add('out.')
        add('')
        add('Twenty-eight, not twenty-seven: `ADR-MOK-005` lists twenty-seven provisions across the three')
        add("documents, and the twenty-eighth is the rule 4 amendment its rule 5 bullet requires without")
        add('naming rule 4. That is one of the two places where the artifacts disagree with each other,')
        add('and both are recorded in section 3 rather than reconciled. Neither is a provision the owner')
        add('approved and the work did not make; both are imprecisions in the text of the ADR, which the')
        add('implementation agent wrote.')
    add('')
    add('What this file does **not** establish: that this change is verified. Oracle 7 is one of seven,')
    add("and `VER-MOK-012`'s eight manual assessments are recorded **OUTSTANDING** in")
    add('`manual-assessment.md`. An amendment being approved is not a judgement that the amended')
    add("contract is satisfied; the technical owner's confirmation of the amended component")
    add('boundaries, prohibited patterns, dependency prohibition and conformance checks is manual')
    add('assessment territory and is not recorded here.')

    sys.stdout.write('\n'.join(out) + '\n')
    return 1 if findings else 0


if __name__ == '__main__':
    sys.exit(report())
