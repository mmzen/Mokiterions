"""VER-MOK-016 row 248: turn one `cargo test` log into a target-qualified census of test names.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-016/analysis/test-census.py \
        <test-run.txt> <out-path>

Reads the log of a single `cargo test --locked --workspace` invocation and writes one line per test
case, `<target> :: <name> :: <outcome>`, sorted. The target is `tests/<file>.rs` for an integration
target and `unittests` for a `src/lib.rs` one; both crates' unit tests therefore share the `unittests`
qualifier, which is unambiguous because their module paths differ.

This reproduces the format of `baseline/test-census.txt`, which was written by hand at the baseline
commit. The format is reproduced rather than reinvented because row 248 requires the two sides
"reconciled name by name", and a reconciliation of two lists written by two different readers measures
the readers as much as the suite.

**The outcome is recorded, not filtered.** A census whose reader dropped failing cases would report a
suite that passes by omission, and the whole purpose of row 248 -- "no case removed, renamed away or
`#[ignore]`d" -- is to make a case that stopped passing impossible to lose quietly. Three cases fail at
this candidate and they appear here as `FAILED`.
"""

import io
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')

RUNNING = re.compile(r'^\s+Running (?:unittests \S+|tests[\\/](\S+))')
CASE = re.compile(r'^test (\S+) \.\.\. (ok|FAILED|ignored)')
TOTALS = re.compile(r'^test result: \S+\. (\d+) passed; (\d+) failed; (\d+) ignored')


def main():
    log_path, out_path = sys.argv[1:3]
    target = None
    cases = []
    passed = failed = ignored = 0

    for line in io.open(log_path, encoding='utf-8', newline='').read().splitlines():
        running = RUNNING.match(line)
        if running:
            # `Running unittests src\lib.rs (target\debug\deps\...)` -> `unittests`
            # `Running tests\authority.rs (target\debug\deps\...)`   -> `tests/authority.rs`
            path = running.group(1)
            target = 'unittests' if path is None else 'tests/' + path
            continue
        case = CASE.match(line)
        if case:
            name, outcome = case.groups()
            cases.append((target, name, outcome))
            continue
        totals = TOTALS.match(line)
        if totals:
            a, b, c = (int(value) for value in totals.groups())
            passed, failed, ignored = passed + a, failed + b, ignored + c

    cases.sort()
    lines = [
        f'# census of {log_path.replace(chr(92), "/")}',
        f'# {len(cases)} test names; reported totals: {passed} passed, {failed} failed, '
        f'{ignored} ignored',
        '',
    ]
    lines += [f'{target} :: {name} :: {outcome}' for target, name, outcome in cases]
    lines.append('')
    io.open(out_path, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines))

    print(f'{len(cases)} names; {passed} passed, {failed} failed, {ignored} ignored')
    # The counted names must account for every reported case, or the reader has missed a target.
    return 0 if len(cases) == passed + failed + ignored else 1


if __name__ == '__main__':
    sys.exit(main())
