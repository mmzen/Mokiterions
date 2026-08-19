"""VER-MOK-011 oracle 3: turn a `cargo test` log into a target-qualified census of test names.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-011/analysis/census.py <cargo-test-log> <out.txt>

Each retained line is `<target> :: <test name> :: <result>`, sorted. The target qualifier matters:
two targets may hold a test of the same name, and a reconciliation that collapsed them would let a
case move between tiers unnoticed -- which `SPEC-MOK-004` governs and this work must not do
silently.

The census is taken from the log of one `cargo test` invocation at the workspace root, because that
is the gate `SPEC-MOK-004` states. It is not assembled from per-package runs.
"""

import io
import re
import sys

RUNNING = re.compile(r'^\s+(?:Running|Doc-tests)\s+(\S+)')
TEST = re.compile(r'^test (\S+) \.\.\. (\w+)')
RESULT = re.compile(r'^test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored')


def main():
    log_path, out_path = sys.argv[1:3]
    target = '<unknown>'
    rows, passed, failed, ignored = [], 0, 0, 0
    for line in io.open(log_path, encoding='utf-8', errors='replace'):
        line = line.rstrip('\n')
        running = RUNNING.match(line)
        if running:
            target = running.group(1).replace('\\', '/')
            continue
        test = TEST.match(line)
        if test:
            rows.append(f'{target} :: {test.group(1)} :: {test.group(2)}')
            continue
        result = RESULT.match(line)
        if result:
            passed += int(result.group(2))
            failed += int(result.group(3))
            ignored += int(result.group(4))
    rows.sort()
    header = [
        f'# census of {log_path}',
        f'# {len(rows)} test names; reported totals: {passed} passed, {failed} failed, {ignored} ignored',
        '',
    ]
    io.open(out_path, 'w', encoding='utf-8', newline='\n').write('\n'.join(header + rows) + '\n')
    print('\n'.join(header[:2]))
    return 0 if failed == 0 else 1


if __name__ == '__main__':
    sys.exit(main())
