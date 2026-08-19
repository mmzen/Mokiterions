"""WO-MOK-010: the tooling runs, assembled from their own captured output.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-010/analysis/static-checks.py <capture-dir>

The capture directory holds one file per command, each of them the command line, that command's own
combined output, and its exit status as a final `exit=<n>` line:

    versions.txt      cargo, rustc, rustfmt and clippy versions
    fmt.txt           cargo fmt --all -- --check
    clippy.txt        cargo clippy --workspace --all-targets --all-features -- -D warnings
    test.txt          cargo test --workspace
    tree.txt          cargo tree -p Mokiterions                 (the candidate tree)
    tree-tui.txt      cargo tree -p mokiterions-tui             (the candidate tree)
    tree-pre.txt      cargo tree -p Mokiterions                 (60fda9f, a clean git worktree)
    tree-tui-pre.txt  cargo tree -p mokiterions-tui             (60fda9f, a clean git worktree)

This artifact does not restate the outputs -- it reads them and checks the claims a reader would
otherwise have to take on trust:

  - every command exited zero;
  - `--check` printed no diff, so no file is unformatted;
  - clippy actually re-linted both crates in this run rather than reporting a cached result, and
    emitted no warning; `-D warnings` means a single warning would have failed it;
  - every test runner reported `ok`, and the ignored and filtered counts are zero, so the pass count
    is the whole suite and not a subset. `SPEC-MOK-004` forbids an ignored test, and this is where
    that is checked; `test-census.txt` reconciles the names;
  - the dependency graph is unchanged from the pre-change commit, and the engine's is empty.

The engine having no dependencies at all is the load-bearing one of those. `SPEC-MOK-001` puts
determinism, integer arithmetic and the entropy stream inside the engine, and nothing outside it can
move them if there is nothing outside it.
"""

import io
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')

RESULT = re.compile(r'^test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; '
                    r'(\d+) measured; (\d+) filtered out')
WARNING = re.compile(r'^(warning|error)(\[|:|\s)')
ROOT = re.compile(r'\((?:[A-Za-z]:)?[\\/].*?\)')


def read(directory, name):
    path = os.path.join(directory, name)
    text = io.open(path, encoding='utf-8').read()
    lines = text.split('\n')
    command = lines[0][4:].strip() if lines[0].startswith('### ') else '(unlabelled)'
    status = next((int(line[5:]) for line in reversed(lines) if line.startswith('exit=')), None)
    body = [line for line in lines[1:] if not line.startswith('exit=')]
    return command, status, body


def graph(body):
    """A dependency tree with the workspace-local paths removed, so two checkouts can be compared."""
    return [ROOT.sub('(local path)', line).rstrip()
            for line in body if line.strip()]


def main():
    directory = sys.argv[1]
    out = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                       'static-checks.txt')
    failures = []

    lines = ['WO-MOK-010 - the tooling runs', '',
             'Method, and what is checked rather than restated: see the header of',
             'analysis/static-checks.py. Every figure below is read from the command\'s own captured',
             f'output under {directory}/.', '']

    _, status, body = read(directory, 'versions.txt')
    lines += ['The toolchain that produced every result in this file', '',
              '  ' + '\n  '.join(line for line in body
                                 if line.strip() and not line.startswith('### ')),
              '',
              '  A different toolchain can format differently and lint differently, so the version is',
              '  part of the result. It is not part of the simulation\'s determinism: `SPEC-MOK-001`',
              '  fixes the arithmetic, and `baseline/` shows the frozen sources reproducing their',
              '  pre-change output byte for byte under this toolchain.',
              '']

    command, status, body = read(directory, 'fmt.txt')
    diff = [line for line in body if line.startswith(('Diff in ', '+', '-'))]
    if status != 0 or diff:
        failures.append('cargo fmt --check reported unformatted files')
    lines += ['1. Formatting', '', f'  $ {command}',
              f'    exit={status}, and {len(diff)} diff lines printed.',
              '',
              '  `--check` prints a diff for every file it would rewrite and exits non-zero. It printed',
              '  nothing. The engine source was reformatted by `cargo fmt` during this work order, and',
              '  the reformatting happened after the binary that produced the first post-change capture',
              '  had been built -- as did two later edits to the same file. `baseline/rebuild-check.txt`',
              '  records what was done about that: the tree was rebuilt from the committed source and the',
              '  matrix captured again, and all 83 shared cells came out byte-identical, so none of the',
              '  three is left to be argued about.',
              '']

    command, status, body = read(directory, 'clippy.txt')
    checked = [line.strip() for line in body if line.strip().startswith('Checking ')]
    warnings = [line for line in body if WARNING.match(line.strip())]
    if status != 0 or warnings or len(checked) < 2:
        failures.append('clippy failed, warned, or did not re-lint both crates')
    lines += ['2. Lints', '', f'  $ {command}', f'    exit={status}, '
              f'{len(warnings)} warning or error lines, {len(checked)} crates re-linted in this run:',
              ''] + [f'      {line}' for line in checked] + [
              '',
              '  The sources were touched before this run so that the result is a fresh lint of the',
              '  candidate tree and not a cached one -- a cached clippy run prints no warning because it',
              '  ran nothing, which is the failure mode this guards against. `--all-targets` includes',
              '  the test code, `--all-features` includes every feature, and `-D warnings` turns any',
              '  single warning into a non-zero exit. `SPEC-MOK-004` forbids an `allow` attribute added',
              '  to keep this quiet, and none was: the diff of this work order adds no `allow`.',
              '']

    command, status, body = read(directory, 'test.txt')
    results = [RESULT.match(line.strip()) for line in body]
    results = [match for match in results if match]
    passed = sum(int(match.group(2)) for match in results)
    failed = sum(int(match.group(3)) for match in results)
    ignored = sum(int(match.group(4)) for match in results)
    filtered = sum(int(match.group(6)) for match in results)
    not_ok = [match.group(0) for match in results if match.group(1) != 'ok']
    if status != 0 or failed or ignored or filtered or not_ok or not results:
        failures.append('the test run failed, ignored a test, or filtered one out')
    lines += ['3. Tests', '', f'  $ {command}', f'    exit={status}',
              '',
              f'  runners reporting a result: {len(results)}',
              f'  runners reporting ok:       {len(results) - len(not_ok)}',
              f'  passed:                     {passed}',
              f'  failed:                     {failed}',
              f'  ignored:                    {ignored}',
              f'  filtered out:               {filtered}',
              '',
              '  Zero ignored and zero filtered out is the part worth stating: a suite can be made to',
              '  pass by not running, and these two counts are what would show it. The whole suite ran.',
              f'  `test-census.txt` reconciles those {passed} names against the pre-change commit\'s,',
              '  name by name and tier by tier.',
              '']

    body_engine = graph(read(directory, 'tree.txt')[2])
    body_engine_pre = graph(read(directory, 'tree-pre.txt')[2])
    body_tui = graph(read(directory, 'tree-tui.txt')[2])
    body_tui_pre = graph(read(directory, 'tree-tui-pre.txt')[2])
    if body_engine != body_engine_pre or body_tui != body_tui_pre:
        failures.append('the dependency graph changed')
    if len(body_engine) != 1:
        failures.append('the engine crate has acquired a dependency')
    lines += ['4. Dependencies', '', f'  $ cargo tree -p Mokiterions',
              ''] + [f'    {line}' for line in body_engine] + [
              '',
              '  One line: the engine crate depends on nothing. `SPEC-MOK-001` puts determinism, integer',
              '  arithmetic and the single entropy stream inside this crate, and no external crate can',
              '  move any of them because there is no external crate. WO-MOK-010 added no dependency,',
              '  and it did not need one -- SplitMix64 is thirty lines of shifts and multiplications.',
              '',
              f'  $ cargo tree -p mokiterions-tui   ->   {len(body_tui)} lines, unchanged',
              '',
              '  The observer draws with ratatui and that is where its tree comes from. Both trees are',
              '  identical to the pre-change commit\'s, compared line by line with the checkout path',
              '  normalised away, so nothing was added, removed or bumped:',
              '',
              f'    engine tree lines:   {len(body_engine_pre)} before, {len(body_engine)} after, '
              f'identical = {body_engine == body_engine_pre}',
              f'    observer tree lines: {len(body_tui_pre)} before, {len(body_tui)} after, '
              f'identical = {body_tui == body_tui_pre}',
              '']

    lines += ['RESULT: ' + ('PASS - four commands, all clean, and no dependency moved'
                            if not failures else 'FAIL')]
    lines += [f'  - {failure}' for failure in failures]

    io.open(out, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
    print('\n'.join(lines))
    print(f'written to: {out}')
    return 0 if not failures else 1


if __name__ == '__main__':
    sys.exit(main())
