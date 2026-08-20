"""VER-MOK-012 oracle 3: parse every retained capture with a parser this repository does not own.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-012/analysis/validate.py \
        <capture-dir> <output-file>

The parser is Python's standard-library `json`. That is the whole point of this oracle: `ARCH-MOK-001`
keeps the engine's dependency table empty, so the engine cannot link a parser and cannot check its own
output's validity with anything but code by the same author as the writer. A hand-written emitter
checked only by a hand-written reader is one mistake, made twice, agreeing with itself.

Six properties are checked over every line of every record stream in the capture directory. Each is a
row of `VER-MOK-012`'s requirement-to-evidence matrix, and each is checked here by a tool with no
knowledge of the engine's intent:

1. **Every line parses as a JSON object.** Not as a value -- as an object, with `record` its first key.
2. **Integer closure.** Every numeric value is an `int` and not a `float`. `json` decides that, not this
   script: it produces `float` for anything with a fractional part or an exponent, so a single `1.0` or
   `1e3` anywhere in 90 streams fails here. No field is named as a mean, average, ratio or rate.
3. **Alphabet closure over full runs.** Every key and every string value lies on `SPEC-MOK-006` rule
   3.3's closed alphabet. The Rust suite asserts this over the domains; this asserts it over the runs,
   and `VER-MOK-012` requires both -- "asserted over full runs as well as over the enumeration".
4. **Field-set closure.** The union of every key in every stream is compared against the field set
   `SPEC-MOK-006` names. This is how `REQ-MOK-044`'s "no classification anywhere" is checked against
   the whole field set rather than against a list of forbidden names: a field naming an outcome fails
   because it is not in the set, whatever it is called.
5. **Absence encoding.** Every `null` is one of the three absences the specification permits -- a `min`
   or `max` over an empty living population, or a survivor's `died_at`. No other field is ever null.
6. **No duplicate key.** A duplicate would make the stream's meaning depend on the reader's last-wins
   or first-wins choice, and `json`'s default silently takes the last. The hook below refuses instead.

The exit status is non-zero on any failure, and every failure is reported with its file, line number
and the offending value, so a failure names the record rather than the run.
"""

import io
import json
import os
import sys

ALPHABET = set(
    'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
    'abcdefghijklmnopqrstuvwxyz'
    '0123456789'
    '_.-+:;>'
)

# Every field `SPEC-MOK-006` names, transcribed from the specification rather than gathered from the
# streams. Gathering it from the streams would make this check pass for any field the engine emits,
# which is the failure it exists to catch.
NAMED_FIELDS = {
    # Framing and the header record, rules 2.2 and 5.
    'record', 'schema', 'engine', 'config', 'seed', 'ticks', 'policy', 'density', 'trace_actions',
    # Event records, rule 6, and the result shapes of rules 6.3 to 6.7.
    'tick', 'subject', 'event', 'result',
    'width', 'height', 'territories', 'class', 'position', 'x', 'y', 'territory', 'name',
    'health', 'satiety', 'energy', 'fear', 'waste_tolerance', 'source',
    'from', 'to', 'food', 'count', 'reason', 'proposal', 'action', 'direction', 'status', 'detail',
    # Metrics records, rule 7.
    'living', 'deaths', 'population', 'A', 'B', 'sum', 'min', 'max',
    'standing', 'low', 'medium', 'high', 'capacity', 'depleted',
    # The run record, rule 8.
    'survivors', 'crossings', 'consumed', 'regenerated', 'regeneration_skipped', 'final',
    'agents', 'id', 'died_at',
}

# Rule 4.2: no derived statistic. Checked as a name test in addition to the field-set closure above,
# because a reader of this script should not have to derive the prohibition from a set.
#
# Compared against the field name's `_`-separated tokens and against the whole name, not as a
# substring: `regenerated` contains `rate` and `regeneration_skipped` contains it twice, and both are
# fields `SPEC-MOK-006` names. A run-together name like `deathrate` would slip past a token test, and
# is caught by the field-set closure above instead -- it is not a field the specification names.
FORBIDDEN_NAMES = frozenset(
    ('mean', 'average', 'avg', 'ratio', 'rate', 'rates', 'percent', 'percentage', 'median')
)


def names_a_statistic(field):
    tokens = set(field.lower().split('_'))
    tokens.add(field.lower())
    return sorted(tokens & FORBIDDEN_NAMES)

# Rule 4.4: the only three absences in the stream.
PERMITTED_NULLS = {'min', 'max', 'died_at'}


class DuplicateKey(ValueError):
    pass


def no_duplicate_keys(pairs):
    keys = [key for key, _ in pairs]
    if len(keys) != len(set(keys)):
        duplicated = sorted({key for key in keys if keys.count(key) > 1})
        raise DuplicateKey(f'duplicate key(s) {duplicated}')
    return dict(pairs)


def walk(value, key, findings, where):
    """Every property that is a statement about one value, checked as the tree is walked."""
    if isinstance(value, bool):
        # Checked before `int`, because `bool` is a subclass of `int` in Python and `trace_actions`
        # and `depleted` are the two legitimate booleans in the stream.
        return
    if isinstance(value, float):
        findings.append(f'{where}: {key} is a floating-point value {value!r}')
        return
    if isinstance(value, int):
        return
    if value is None:
        if key not in PERMITTED_NULLS:
            findings.append(f'{where}: {key} is null, which is not one of the three permitted absences')
        return
    if isinstance(value, str):
        off = sorted({character for character in value if character not in ALPHABET})
        if off:
            findings.append(f'{where}: the value of {key} carries {off!r}, off rule 3.3\'s alphabet')
        return
    if isinstance(value, dict):
        for nested_key, nested in value.items():
            off = sorted({character for character in nested_key if character not in ALPHABET})
            if off:
                findings.append(f'{where}: the key {nested_key!r} carries {off!r}, off rule 3.3\'s alphabet')
            if nested_key not in NAMED_FIELDS:
                findings.append(f'{where}: {nested_key!r} is not a field SPEC-MOK-006 names')
            statistic = names_a_statistic(nested_key)
            if statistic:
                findings.append(
                    f'{where}: the field {nested_key!r} names a derived statistic {statistic}'
                )
            walk(nested, nested_key, findings, where)
        return
    if isinstance(value, list):
        for item in value:
            walk(item, key, findings, where)
        return
    findings.append(f'{where}: {key} has an unexpected type {type(value).__name__}')


def check(path, findings):
    """One record stream. Returns the number of lines parsed."""
    name = os.path.basename(path)
    parsed = 0
    with io.open(path, 'r', encoding='utf-8', newline='') as handle:
        raw = handle.read()

    if not raw.endswith('\n'):
        findings.append(f'{name}: the stream does not end with a newline')
    if '\r' in raw:
        findings.append(f'{name}: the stream carries a carriage return')

    for number, line in enumerate(raw.split('\n'), start=1):
        if not line:
            if number != raw.count('\n') + 1:
                findings.append(f'{name}:{number}: blank line')
            continue
        where = f'{name}:{number}'
        try:
            record = json.loads(line, object_pairs_hook=no_duplicate_keys)
        except DuplicateKey as error:
            findings.append(f'{where}: {error}')
            continue
        except ValueError as error:
            findings.append(f'{where}: does not parse as JSON: {error}')
            continue
        parsed += 1
        if not isinstance(record, dict):
            findings.append(f'{where}: parses as {type(record).__name__}, not an object')
            continue
        if next(iter(record)) != 'record':
            findings.append(f'{where}: the first key is {next(iter(record))!r}, not "record"')
        walk(record, '<record>', findings, where)
    return parsed


def main():
    capture_dir, output_file = sys.argv[1:3]
    streams = sorted(name for name in os.listdir(capture_dir) if name.endswith('.jsonl'))
    if not streams:
        print(f'no record stream in {capture_dir}', file=sys.stderr)
        return 1

    findings = []

    # The transcribed field set is itself checked against rule 4.2, so that a statistic admitted into
    # the specification would fail here rather than pass by being on the allow-list.
    for field in sorted(NAMED_FIELDS):
        statistic = names_a_statistic(field)
        if statistic:
            findings.append(f'SPEC-MOK-006 names the field {field!r}, a derived statistic {statistic}')

    total = 0
    per_stream = []
    for name in streams:
        parsed = check(os.path.join(capture_dir, name), findings)
        total += parsed
        per_stream.append((name, parsed))

    lines = [
        '# VER-MOK-012 oracle 3: every retained capture parsed by a parser outside this repository',
        '#',
        f'# capture directory: {capture_dir}',
        f'# parser: Python {sys.version.split()[0]} standard-library json module',
        '# command: python docs/engineering/simulation/evidence/WO-MOK-012/analysis/validate.py '
        '<capture-dir> <output-file>',
        '#',
        '# Checked per line: parses as an object with "record" first; every number an int and not a',
        '# float; every key and string value on SPEC-MOK-006 rule 3.3\'s alphabet; every key a field',
        '# the specification names; every null one of the three permitted absences; no duplicate key.',
        '',
    ]
    for name, parsed in per_stream:
        lines.append(f'{name:<41} {parsed:>7} records parsed')
    lines.append('')
    lines.append(f'# {len(streams)} streams, {total} records, {len(findings)} findings')
    lines.append(f'# result: {"FAIL" if findings else "PASS"}')
    if findings:
        lines.append('')
        lines.append('# findings')
        lines.extend(findings[:500])
        if len(findings) > 500:
            lines.append(f'# ... and {len(findings) - 500} more')

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(lines) + '\n')

    print(f'{len(streams)} streams, {total} records parsed, {len(findings)} findings')
    return 1 if findings else 0


if __name__ == '__main__':
    sys.exit(main())
