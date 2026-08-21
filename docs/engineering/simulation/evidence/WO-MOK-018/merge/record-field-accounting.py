"""VER-MOK-012, oracle 1 — account for the record stream's `suffered` field across a merge.

The merge of master's combat resolution into Phase 4a's record stream adds one field to every
`action_trace` record: `result.suffered`, a list of the strikes the subject absorbed inside rule
26's window. Every record the candidate commit wrote therefore grows, and a digest comparison
between the candidate's record streams and the merge's reports "differs" without saying why.

This instrument says why, three ways, and each mode is a separate claim:

    kinds     <capture-dir> ...
              Which event kinds the record stream exhibits, in how many cells, and how many
              records each. Twelve kinds under the three older policies and fifteen under the
              fourth is `EventType::ALL` moving from 12 to 15, measured in the emitted stream
              rather than read off the enum.

    field     <capture-dir> ...
              Every `action_trace` record, whether it carries `result.suffered`, whether the list
              is empty, how many entries the non-empty ones hold, what keys each entry has, and
              the serialized byte cost of the field. `,"suffered":[]` is fourteen bytes.

    compare   <left-manifest> <right-manifest> <capture-dir>
              The candidate's record-stream digests against the merge's, cell by cell. Where the
              digests differ the byte difference must equal fourteen times that cell's
              `action_trace` count, and the line counts must be equal, because the field is
              inserted into records that already exist and adds none. A cell that differs by any
              other amount, or by any line, is a FAIL and is named.

No projection, no normalization, no tolerance: every comparison is over bytes, counts and exact
key sets.

Manifest columns, as `capture.sh` and `capture-social.sh` write them:

    cell  sha256(stdout) bytes lines  sha256(stderr) bytes  exit  sha256(sink) bytes lines

Usage, from any directory:

    python record-field-accounting.py kinds   <dir> [<dir> ...]
    python record-field-accounting.py field   <dir> [<dir> ...]
    python record-field-accounting.py compare <left.txt> <right.txt> <dir>
"""

import glob
import io
import json
import os
import sys

EMPTY = ',"suffered":[]'
COMBAT = ('attack_resolved', 'threat_resolved', 'surrender_resolved')


def streams(directory):
    """Every .jsonl in the directory, as (cell name, parsed records)."""
    for path in sorted(glob.glob(os.path.join(directory, '*.jsonl'))):
        records = []
        for line in io.open(path, encoding='utf-8'):
            if line.strip():
                records.append(json.loads(line))
        yield os.path.basename(path)[:-6], records


def suffered_of(record):
    """The record's `result.suffered`, or None where the key is absent."""
    result = record.get('result')
    if isinstance(result, dict) and 'suffered' in result:
        return result['suffered']
    return None


# --------------------------------------------------------------------------- kinds

def mode_kinds(directories):
    for directory in directories:
        cells = {}
        types = set()
        for cell, records in streams(directory):
            kinds = {}
            for record in records:
                types.add(record.get('record'))
                kind = record.get('event')
                if kind is not None:
                    kinds[kind] = kinds.get(kind, 0) + 1
            cells[cell] = kinds
        every = set()
        for kinds in cells.values():
            every |= set(kinds)
        print('=== %s ===' % directory)
        print('cells %d, distinct event kinds %d, record types %s'
              % (len(cells), len(every),
                 ', '.join(sorted(str(t) for t in types))))
        print()
        print('  %-28s %6s %9s' % ('event kind', 'cells', 'records'))
        for kind in sorted(every):
            n = sum(1 for k in cells.values() if kind in k)
            total = sum(k.get(kind, 0) for k in cells.values())
            print('  %-28s %6d %9d' % (kind, n, total))
        print()
        for kind in COMBAT:
            n = sum(1 for k in cells.values() if kind in k)
            print('  %-22s in %d of %d cells' % (kind, n, len(cells)))
        print()


# --------------------------------------------------------------------------- field

def mode_field(directories):
    for directory in directories:
        cells = 0
        records = 0
        trace = 0
        with_key = 0
        empty = 0
        nonempty = 0
        entries = 0
        empty_bytes = 0
        nonempty_bytes = 0
        lengths = {}
        shapes = {}
        other = {}
        for cell, stream in streams(directory):
            cells += 1
            for record in stream:
                records += 1
                value = suffered_of(record)
                kind = record.get('event')
                if kind != 'action_trace':
                    if value is not None:
                        other[kind] = other.get(kind, 0) + 1
                    continue
                trace += 1
                if value is None:
                    continue
                with_key += 1
                if value == []:
                    empty += 1
                    empty_bytes += len(EMPTY)
                    continue
                nonempty += 1
                entries += len(value)
                lengths[len(value)] = lengths.get(len(value), 0) + 1
                for entry in value:
                    key = tuple(sorted(entry)) if isinstance(entry, dict) \
                        else type(entry).__name__
                    shapes[key] = shapes.get(key, 0) + 1
                nonempty_bytes += len(
                    ',"suffered":' + json.dumps(value, separators=(',', ':')))
        print('=== %s ===' % directory)
        print('  cells                          %d' % cells)
        print('  records, all kinds             %d' % records)
        print('  action_trace records           %d' % trace)
        print('  ... carrying result.suffered   %d' % with_key)
        print('  ... suffered == []             %d' % empty)
        print('  ... suffered non-empty         %d' % nonempty)
        print('  entries across non-empty lists %d' % entries)
        print('  list lengths seen              %s' % (lengths or 'none'))
        print('  entry key sets seen            %s' % (shapes or 'none'))
        print('  bytes, `%s`  %d = %d x %d'
              % (EMPTY, empty_bytes, len(EMPTY), empty))
        print('  bytes, non-empty occurrences   %d' % nonempty_bytes)
        print('  suffered on any other kind     %s' % (other or 'none'))
        print()


# --------------------------------------------------------------------------- compare

def manifest(path):
    cells = {}
    for line in io.open(path, encoding='utf-8'):
        line = line.strip()
        if not line or line.startswith('#'):
            continue
        f = line.split()
        if len(f) < 10:
            continue
        cells[f[0]] = dict(sha=f[7], bytes=int(f[8]), lines=int(f[9]))
    return cells


def trace_count(directory, cell):
    path = os.path.join(directory, cell + '.jsonl')
    n = 0
    nonempty = 0
    for line in io.open(path, encoding='utf-8'):
        if not line.strip():
            continue
        record = json.loads(line)
        if record.get('event') != 'action_trace':
            continue
        n += 1
        if suffered_of(record):
            nonempty += 1
    return n, nonempty


def mode_compare(left_path, right_path, directory):
    left = manifest(left_path)
    right = manifest(right_path)
    print("# oracle 1, comparison H: the candidate commit's record streams against the merge's")
    print('# left:  %s' % left_path)
    print('# right: %s' % right_path)
    print('#')
    print('# Compared columns: sha256(sink), sink bytes, sink lines.')
    print('# Where the digests differ the byte difference is accounted for as occurrences of')
    print('# `%s` (%d bytes) on action_trace records, and the line counts must be'
          % (EMPTY, len(EMPTY)))
    print('# equal. No projection, no normalization, no tolerance.')
    print()
    shared = sorted(set(left) & set(right))
    same = differ = accounted = 0
    delta_total = trace_total = 0
    faults = []
    for cell in shared:
        l, r = left[cell], right[cell]
        n, nonempty = trace_count(directory, cell)
        trace_total += n
        if l['sha'] == r['sha']:
            same += 1
            state = 'identical'
            if l['bytes'] != r['bytes'] or l['lines'] != r['lines']:
                faults.append('%s: digest equal, counts differ' % cell)
            if n:
                faults.append('%s: identical yet carries %d action_trace' % (cell, n))
        else:
            differ += 1
            delta = r['bytes'] - l['bytes']
            delta_total += delta
            expect = len(EMPTY) * n
            if l['lines'] != r['lines']:
                faults.append('%s: line counts differ, %d -> %d'
                              % (cell, l['lines'], r['lines']))
            if delta == expect and nonempty == 0:
                accounted += 1
                state = 'differs, +%d bytes = %d x %d' % (delta, len(EMPTY), n)
            else:
                faults.append('%s: +%d bytes, expected %d for %d records, %d non-empty'
                              % (cell, delta, expect, n, nonempty))
                state = 'differs, +%d bytes, expected %d for %d records' % (
                    delta, expect, n)
        print('%-34s %s' % (cell, state))
    print()
    print('# %d cells compared: %d record streams byte-identical, %d differing'
          % (len(shared), same, differ))
    print('# %d of the %d differing accounted for exactly by the field insertion'
          % (accounted, differ))
    print('# action_trace records across the capture: %d' % trace_total)
    if len(EMPTY) and delta_total % len(EMPTY) == 0:
        print('# total byte difference: %d = %d x %d'
              % (delta_total, len(EMPTY), delta_total // len(EMPTY)))
    else:
        print('# total byte difference: %d, not a whole multiple of %d'
              % (delta_total, len(EMPTY)))
    print('# result: %s' % ('PASS' if not faults else 'FAIL'))
    for fault in faults:
        print('# fault: %s' % fault)


if __name__ == '__main__':
    mode = sys.argv[1] if len(sys.argv) > 1 else ''
    if mode == 'kinds':
        mode_kinds(sys.argv[2:])
    elif mode == 'field':
        mode_field(sys.argv[2:])
    elif mode == 'compare':
        mode_compare(sys.argv[2], sys.argv[3], sys.argv[4])
    else:
        sys.stderr.write(__doc__)
        sys.exit(2)
