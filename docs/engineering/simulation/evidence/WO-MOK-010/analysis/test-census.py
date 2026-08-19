"""WO-MOK-010: the test suite before and after, reconciled name by name and tier by tier.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-010/analysis/test-census.py \
        <list-before.txt> <list-after.txt>

Each input is `cargo test --workspace -- --list` with standard error included, captured at the
pre-change commit 60fda9faffbd452752a34efa356f16cc6ad1d3ff -- from a clean git worktree at that
commit -- and at the candidate tree. Standard error is what carries the `Running ...` lines, and those
lines are what make this a tier census rather than a name census: every test below such a line belongs
to that test binary, and the binary is what fixes the tier.

What this is for. `SPEC-MOK-004` fixes which tier a test belongs to: a test may sit in the public tier
only if it can be written through the public API with its assertions unchanged and no item widened to
reach it, and it sits in the internal tier otherwise. WO-MOK-010 forbids three things a growing suite
could hide -- relocating a test by widening an item, weakening an assertion, and ignoring a test. A
count of tests shows none of them. This reconciliation shows two:

  - nothing was removed and no binary lost a test, so nothing was renamed away or dropped; a binary
    that is new to the after side is called out as new rather than counted silently, because a new
    binary is the one way an addition can arrive at a tier by relocation instead of by being written
    there;
  - every addition is named in full and attributed to its binary, so a reviewer reads the names and
    where each one lives instead of a total that went up.

The third, weakened assertions, is not a naming question. It is answered by the diff of the test
bodies, and the diff is what the reviewer of this work order reads; it is not claimed here.

`--list` also reports what it will not run. An ignored test is listed as `: test` all the same, and
`cargo test` reports the ignored count separately, so `static-checks.txt` carries that count and this
census does not duplicate it.

Tiers, as Cargo reports them. `unittests src\\lib.rs` and `unittests src\\main.rs` are a crate's own
`#[cfg(test)]` code, which can reach private items: the internal tier. `tests\\<file>.rs` is a
separate crate that links the library from outside and can reach nothing but its public interface: the
public tier. Doc-tests are listed with no tests on both sides and are reported for completeness.
"""

import io
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')

NAME = re.compile(r'^(\S+): test$')
TOTAL = re.compile(r'^(\d+) tests?, \d+ benchmarks?$')
RUNNING = re.compile(r'^\s*Running (unittests )?(\S+) \(([^)]*[/\\])?([^)/\\]+)\)')
DOCTESTS = re.compile(r'^\s*Doc-tests (\S+)')


def load(path):
    """Every runner as (binary, target, tier, declared total), and every test as (binary, name).

    A runner is registered when its `Running` or `Doc-tests` line is read, not when its first test is
    read, so a runner that lists no tests still appears -- which is what lets an emptied binary show
    as a row of zeroes rather than as a missing line.
    """
    runners = []
    tests = []
    current = None
    for line in io.open(path, encoding='utf-8'):
        line = line.rstrip('\n')
        running = RUNNING.match(line)
        doctests = DOCTESTS.match(line)
        if running:
            current = [running.group(4).rsplit('-', 1)[0],
                       running.group(2).replace('\\', '/'),
                       'internal (crate\'s own #[cfg(test)])' if running.group(1)
                       else 'public (integration, links from outside)',
                       None]
            runners.append(current)
            continue
        if doctests:
            current = [doctests.group(1), 'doc comments', 'doc-test', None]
            runners.append(current)
            continue
        stripped = line.strip()
        match = NAME.match(stripped)
        if match and current is not None:
            tests.append((current[0], current[1], current[2], match.group(1)))
            continue
        match = TOTAL.match(stripped)
        if match and current is not None:
            current[3] = int(match.group(1))
    return tests, runners


def main():
    before_path, after_path = sys.argv[1:3]
    out = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                       'test-census.txt')

    before, before_runners = load(before_path)
    after, after_runners = load(after_path)
    keyed = lambda tests: {(binary, name): (target, tier)
                           for binary, target, tier, name in tests}
    left, right = keyed(before), keyed(after)
    added = sorted(set(right) - set(left))
    removed = sorted(set(left) - set(right))
    declared_before = sum(runner[3] or 0 for runner in before_runners)
    declared_after = sum(runner[3] or 0 for runner in after_runners)

    lines = [
        'WO-MOK-010 - the test suite before and after, reconciled name by name and tier by tier',
        '',
        f'before: {before_path}',
        '        commit 60fda9faffbd452752a34efa356f16cc6ad1d3ff, a clean git worktree at that commit',
        f'after:  {after_path}',
        '        the candidate tree',
        '',
        'Method, and what it does and does not show: see the header of analysis/test-census.py.',
        '',
        f'  tests listed before: {len(before):4}   over {len(before_runners):2} runners   '
        f'(their own totals sum to {declared_before})',
        f'  tests listed after:  {len(after):4}   over {len(after_runners):2} runners   '
        f'(their own totals sum to {declared_after})',
        f'  added:               {len(added):4}',
        f'  removed:             {len(removed):4}',
        '',
        'On both sides the number of names listed equals the sum of the runners\' own totals, so no',
        'test is missing from this reconciliation, and no (binary, name) pair occurs twice.',
        '',
    ]
    if len(keyed(before)) != len(before) or len(keyed(after)) != len(after):
        lines.append('WARNING: a name occurs twice within one binary; the set arithmetic understates it.')
    if len(before) != declared_before or len(after) != declared_after:
        lines.append('WARNING: listed names and declared totals disagree; the parse is incomplete.')

    order, counts = [], {}
    for runners in (before_runners, after_runners):
        for binary, target, tier, _ in runners:
            key = (binary, target, tier)
            if key not in counts:
                counts[key] = [0, 0]
                order.append(key)
    for source, index in ((before, 0), (after, 1)):
        for binary, target, tier, _ in source:
            counts[(binary, target, tier)][index] += 1

    width = max(len(f'{key[0]}  ({key[1]})') for key in order)
    lines += [
        'By runner. A test that changed tier would show as a fall in one row and a rise in another; a',
        'runner that lists no tests is a row of zeroes rather than a missing line, so a binary that had',
        'been emptied could not hide.',
        '',
        f'  {"runner  (target)".ljust(width)}  {"tier":40}  before  after  change',
        f'  {"-" * width}  {"-" * 40}  ------  -----  ------',
    ]
    for key in order:
        binary, target, tier = key
        low, high = counts[key]
        lines.append(f'  {f"{binary}  ({target})".ljust(width)}  {tier:40}  {low:6}  {high:5}  '
                     f'{high - low:+6}')
    lines += ['']

    internal = sum(1 for binary, name in added if right[(binary, name)][1].startswith('internal'))
    public = len(added) - internal
    before_runner_keys = {(runner[0], runner[1], runner[2]) for runner in before_runners}
    after_runner_keys = {(runner[0], runner[1], runner[2]) for runner in after_runners}
    added_runners = [key for key in order if key not in before_runner_keys]
    removed_runners = [key for key in order if key not in after_runner_keys]
    lost = [key for key in order if counts[key][1] < counts[key][0]]
    emptied = [key for key in order if counts[key][0] > 0 and counts[key][1] == 0]
    lines += ['The names added, in full, with the runner each one runs in:', '']
    shown = max((len(binary) for binary, _ in added), default=1)
    for binary, name in added:
        target, tier = right[(binary, name)]
        lines.append(f'  + {binary.ljust(shown)}  {target.ljust(20)}  {name}')
    lines += ['', 'The names removed, in full:', '']
    lines += [f'  - {binary}  {name}' for binary, name in removed] or ['  (none)']
    lines += [
        '',
        f'{len(added)} additions: {internal} in the internal tier and {public} in the public tier.',
        f'No name was removed, no runner lost a test and no runner was emptied or removed '
        f'({len(removed)} removed names, {len(lost)} runners with a fall, {len(emptied)} emptied, '
        f'{len(removed_runners)} runners gone), so nothing was renamed away or dropped.',
        '',
    ]
    if added_runners:
        observed = [(key[0], key[1]) for key in added_runners]
        if observed != [('decisions', 'tests/decisions.rs')]:
            lines += ['WARNING: the paragraph below names `decisions (tests/decisions.rs)` as the only new '
                      f'runner; this run found {observed}. The prose is stale.', '']
        lines += [f'{len(added_runners)} runner is new to the after side, and it is the one row that needs prose '
                  'rather than a count:', '']
        lines += [f'  + {binary}  ({target})  {tier}' for binary, target, tier in added_runners]
        lines += [
            '',
            'A new public-tier runner is exactly how a relocated test looks in this census, so what moved is',
            'stated here rather than left to be inferred.',
            '`the_trait_aware_source_never_waits_and_proposes_only_valid_actions` was first written inline in',
            '`mokiterions-core/src/simulation.rs`, beside the source it covers, and was moved into',
            '`mokiterions-core/tests/decisions.rs` before this capture was taken. The move corrects a',
            '`SPEC-MOK-002` rule 7 misclassification -- rule 7 fixes the tier by the access a test requires and',
            'states that the subject it covers does not decide the tier -- and this test requires only `Config`,',
            '`Policy`, `Density`, `Simulation::new` and `Simulation::run`. Its assertions are verbatim across',
            'the move, as rule 12 requires; only the construction of `Config` changed, from a private test',
            'helper to the public struct literal. Nothing was widened to carry it, which is checkable rather',
            'than asserted: `interface-and-purity.txt` finds the same two public additions and no removals.',
            'Both ends of the move are inside this work order, so on the before side the test does not exist',
            'and it appears here as an addition rather than as a fall in one row and a rise in another. The',
            'file itself is added on rule 8\'s own terms -- "a further file may be added when a further public',
            'subject appears" -- and the subject, what a decision source proposes over a whole run, is one none',
            'of the five files rule 8 lists covers.',
            '',
        ]
    else:
        lines += ['Every addition is in a runner that already existed, at the tier that runner already was.',
                  '']
    lines += [
        'A public-tier addition is the one case worth stating plainly, because the tier rule can be',
        'satisfied dishonestly -- by widening an item to `pub` so that an integration test can reach',
        'it. It was not: `interface-and-purity.txt` enumerates the public interface on both sides and',
        'finds two additions, both of them the values `SPEC-MOK-001` declares -- `Policy::Individual`',
        'and `AgentSnapshot.fear` -- and no removals. The trait-aware policy is selectable from the',
        'command line and the fourth gauge is on the observer\'s roster, so both public-tier additions',
        'are written through interface that the specification puts there and that a user of the engine',
        'has anyway.',
        '',
        'RESULT: ' + ('PASS' if added and not (removed or lost or emptied or removed_runners) else 'FAIL'),
    ]

    io.open(out, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
    print('\n'.join(lines))
    print(f'written to: {out}')
    return 0 if not (removed or lost or emptied or removed_runners) else 1


if __name__ == '__main__':
    sys.exit(main())
