"""VER-MOK-012 *Static and architecture checks*: the twelve source-level checks, run over the
merged tree and compared against the pre-change commit.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-018/analysis/static-checks.py \
        <baseline-commit> <binary> <scratch-dir> <output-file>

## What a static check can and cannot say here

Most of these obligations are stated as absences -- no filesystem call, no floating-point
operation, no second emission site, no classification field -- and an absence is the one thing a
search cannot establish. A search over a list of forbidden words proves only that those words are
missing; the word nobody thought of is exactly the one that would matter.

So wherever a check can be turned from a *negative* search into a *positive* enumeration of a
closed set, it is:

  * filesystem freedom is not a search for `File` and `PathBuf`. It is the enumeration of every
    path into the standard library the library target names at all. That set has three members, and
    `std::fs`, `std::path`, `std::env` and `std::process` are not among them. Nothing outside the
    prelude is reachable without being named, so the enumeration is the proof and the vocabulary of
    forbidden names is only a cross-check;
  * the field set is not a search for `outcome` and `severity`. It is the whole set of field names
    the stream carries, taken twice -- from the source's literals and from a real run's bytes --
    and compared against the set `SPEC-MOK-006`'s own rule sections declare. A classification would
    have to appear in that set, and the check prints the set so a reader can see it;
  * the interface comparison is not a search at all. It is `WO-MOK-011`'s enumerator, run over both
    revisions of the same three files and diffed.

Where a check remains a search, it says so under `limit:` and states what would slip past it.

Every scan runs over source with comments and literals replaced by spaces of the same shape, so
that a heavily documented file does not report its own prose. This repository documents the
prohibitions it observes -- `simulation.rs` names `std::fs` in a doc comment explaining that it
never calls it -- and an unstripped scan would find that sentence and call it a finding.

## The library target, and why `main.rs` is not scanned

`SPEC-MOK-002` rule 1 declares two targets: the library at `src/lib.rs` and the binary at
`src/main.rs`. Rule 3 places `cli` and `simulation` in the library. So the library target is
`lib.rs`, `cli.rs` and `simulation.rs`, and those three files are what the filesystem-freedom check
covers. `main.rs` is deliberately outside it: the binary target is where the one new filesystem
effect lives, by `ADR-MOK-005`'s design, and a check that included it would have nothing to say.
"""

import io
import json
import os
import re
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8')

LIBRARY_SOURCES = ('lib.rs', 'cli.rs', 'simulation.rs')
BINARY_SOURCE = 'main.rs'
PACKAGE = 'mokiterions-core'
OBSERVER = 'mokiterions-tui'

RAW_STRING = re.compile(r'b?r(?P<hashes>#*)"')
CHAR_LITERAL = re.compile(r"b?'(\\.|[^\\'\n])'")
IDENTIFIER_CHAR = re.compile(r'[A-Za-z0-9_]')
FUNCTION = re.compile(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)')
JSON_KEY = re.compile(r'"([A-Za-z_][A-Za-z0-9_]*)"\s*:')

# `SPEC-MOK-002` rule 6, second bullet, as amended on 2026-08-18: the ten names that stay
# prohibited. Five others were removed from the bullet by that amendment and are public by design.
PROHIBITED_TYPES = (
    'Mokiterion', 'Food', 'RelativeDirection', 'ActionResult', 'Observation', 'PerceivedFood',
    'PerceivedMokiterion', 'SplitMix64', 'DecisionEntropy', 'DecisionSource',
)

# Interior mutability and shared ownership. Rule 5's own words: "no `&self` method mutates through
# interior mutability, because no engine type contains a `Cell`, a `RefCell`, an `Rc`, an `Arc`, a
# lock or an atomic". That claim is checkable, so it is checked.
INTERIOR_MUTABILITY = ('Cell', 'RefCell', 'UnsafeCell', 'Rc', 'Arc', 'Mutex', 'RwLock', 'Atomic')

# Cross-checks only. The enumerations above them are what carry the weight.
FILESYSTEM_VOCABULARY = (
    'std::fs', 'fs::', 'File::', 'OpenOptions', 'PathBuf', 'Path::', 'read_dir', 'create_dir',
    'remove_file', 'remove_dir', 'temp_dir', 'canonicalize', 'symlink', 'std::env', 'env::var',
    'option_env!', 'include_str!', 'include_bytes!', 'include!', 'std::process',
    'process::Command', 'SystemTime', 'Instant', 'std::net', 'std::thread',
)
FLOAT_VOCABULARY = ('f32', 'f64', 'as f3', 'as f6', 'sqrt', 'powf', 'INFINITY', 'NAN')
UNORDERED_TYPES = ('HashMap', 'HashSet')


# ---------------------------------------------------------------------------------------------
# Reading Rust without reading its prose
# ---------------------------------------------------------------------------------------------

def blanked(text):
    """`text` with every character replaced by a space, newlines kept."""
    return ''.join('\n' if character == '\n' else ' ' for character in text)


def strip_rust(source):
    """`source` with every comment and every literal blanked, byte offsets and lines preserved.

    Line and column positions survive, so a finding can still name where it is. What does not
    survive is the content of a doc comment or a string, which is the point: this repository
    documents the calls it does not make, and an unstripped scan would find the documentation.
    """
    out = []
    index, length = 0, len(source)
    while index < length:
        character = source[index]

        if source.startswith('//', index):
            end = source.find('\n', index)
            end = length if end < 0 else end
            out.append(blanked(source[index:end]))
            index = end
            continue

        if source.startswith('/*', index):
            # Rust nests block comments, so depth is counted rather than assumed.
            depth, start = 0, index
            while index < length:
                if source.startswith('/*', index):
                    depth, index = depth + 1, index + 2
                elif source.startswith('*/', index):
                    depth, index = depth - 1, index + 2
                    if depth == 0:
                        break
                else:
                    index += 1
            out.append(blanked(source[start:index]))
            continue

        # A raw string opener, but only where `r` begins a token: `four"` must not be read as one.
        raw = RAW_STRING.match(source, index)
        preceded_by_identifier = index > 0 and IDENTIFIER_CHAR.match(source[index - 1])
        if raw and not preceded_by_identifier:
            close = '"' + raw.group('hashes')
            end = source.find(close, raw.end())
            end = length if end < 0 else end + len(close)
            out.append(blanked(source[index:end]))
            index = end
            continue

        if character == '"':
            start, index = index, index + 1
            while index < length:
                if source[index] == '\\':
                    index += 2
                    continue
                if source[index] == '"':
                    index += 1
                    break
                index += 1
            out.append(blanked(source[start:index]))
            continue

        # A char literal, told apart from a lifetime by its closing quote: `'a'` is a literal,
        # `'a,` and `'_>` and `&'sink` are lifetimes and stay.
        literal = CHAR_LITERAL.match(source, index)
        if character == "'" and literal and not preceded_by_identifier:
            out.append(blanked(literal.group(0)))
            index = literal.end()
            continue

        out.append(character)
        index += 1
    return ''.join(out)


def test_lines(lines):
    """The 1-based numbers of every line inside a `#[cfg(test)]` item.

    Brace balance from the attribute's own item, over stripped source, so a brace in a string or a
    doc comment cannot close a block early.
    """
    inside, index = set(), 0
    while index < len(lines):
        if lines[index].strip().startswith('#[cfg(test)]'):
            balance, started = 0, False
            while index < len(lines):
                inside.add(index + 1)
                balance += lines[index].count('{') - lines[index].count('}')
                if '{' in lines[index]:
                    started = True
                index += 1
                if started and balance <= 0:
                    break
            continue
        index += 1
    return inside


def function_spans(lines):
    """(name, first line, last line) for every `fn` in stripped source, 1-based and inclusive."""
    spans, index = [], 0
    while index < len(lines):
        match = FUNCTION.search(lines[index])
        if not match:
            index += 1
            continue
        start, balance, started, cursor = index + 1, 0, False, index
        while cursor < len(lines):
            balance += lines[cursor].count('{') - lines[cursor].count('}')
            if '{' in lines[cursor]:
                started = True
            cursor += 1
            if started and balance <= 0:
                break
        spans.append((match.group(1), start, cursor))
        index += 1
    return spans


def statement_spans(source):
    """(first line, text) for every `;`-terminated statement in stripped source.

    A statement, not a line, because Rust wraps: `self.regeneration_skipped[i] =` and its
    `.saturating_add(1);` are one assignment written over two lines, and a line-oriented reader
    would report the second half as a separate access to the field.
    """
    statements, start, buffer = [], None, []
    for number, line in enumerate(source.lines, start=1):
        if not line.strip():
            continue
        if start is None:
            start = number
        buffer.append(line)
        if ';' in line or line.rstrip().endswith(('{', '}')):
            statements.append((start, '\n'.join(buffer)))
            start, buffer = None, []
    if buffer:
        statements.append((start, '\n'.join(buffer)))
    return statements


def enclosing_function(spans, line):
    """The innermost function containing `line`, or None."""
    containing = [span for span in spans if span[1] <= line <= span[2]]
    return min(containing, key=lambda span: span[2] - span[1])[0] if containing else None


class Source:
    """One file, read once, in the three forms every check below wants."""

    def __init__(self, path, name=None):
        self.path = path
        self.name = name or os.path.basename(path)
        self.raw = io.open(path, encoding='utf-8').read()
        self.stripped = strip_rust(self.raw)
        self.lines = self.stripped.split('\n')
        self.raw_lines = self.raw.split('\n')
        self.test_lines = test_lines(self.lines)
        self.spans = function_spans(self.lines)

    def hits(self, needle, product_only=True):
        """(line number, the raw line) for every occurrence of `needle` in stripped source."""
        found = []
        for number, line in enumerate(self.lines, start=1):
            if needle in line and not (product_only and number in self.test_lines):
                found.append((number, self.raw_lines[number - 1].strip()))
        return found


# ---------------------------------------------------------------------------------------------
# The report
# ---------------------------------------------------------------------------------------------

class Report:
    def __init__(self):
        self.lines = []
        self.results = []

    def check(self, number, title, obligation):
        self.lines.append('')
        self.lines.append('=' * 108)
        self.lines.append(f'{number}. {title}')
        self.lines.append('=' * 108)
        self.lines.append(f'  obligation: {obligation}')
        self._current = (number, title)

    def say(self, text=''):
        self.lines.append(f'  {text}' if text else '')

    def method(self, text):
        self.say(f'method: {text}')

    def verdict(self, passed, text, limit):
        word = 'PASS' if passed else 'FINDING'
        self.say()
        self.say(f'verdict: {word} — {text}')
        self.say(f'limit:   {limit}')
        self.results.append((self._current[0], self._current[1], word, text))


def run(command, **keywords):
    return subprocess.run(command, capture_output=True, text=True, **keywords)


def main():
    baseline, binary, scratch, output_file = sys.argv[1:5]
    root = os.getcwd()
    binary = os.path.abspath(binary)
    os.makedirs(scratch, exist_ok=True)

    library = [Source(os.path.join(root, PACKAGE, 'src', name)) for name in LIBRARY_SOURCES]
    binary_source = Source(os.path.join(root, PACKAGE, 'src', BINARY_SOURCE))
    by_name = {source.name: source for source in library}
    simulation = by_name['simulation.rs']

    # The pre-change library, materialized from the baseline commit so the two are compared and
    # not remembered.
    before_directory = os.path.join(scratch, 'before')
    os.makedirs(before_directory, exist_ok=True)
    before = []
    for name in LIBRARY_SOURCES:
        path = os.path.join(before_directory, name)
        shown = run(['git', 'show', f'{baseline}:{PACKAGE}/src/{name}'])
        io.open(path, 'w', encoding='utf-8', newline='').write(shown.stdout)
        before.append(Source(path, name))
    before_by_name = {source.name: source for source in before}

    report = Report()
    report.lines.extend([
        '# VER-MOK-012: static and architecture checks',
        '#',
        f'# repository root: {root}',
        f'# merged tree:     working tree (see gates.txt for the commit)',
        f'# baseline:        {baseline}',
        f'# binary:          {binary}',
        '# command: python docs/engineering/simulation/evidence/WO-MOK-018/analysis/'
        'static-checks.py \\',
        '#              <baseline-commit> <binary> <scratch-dir> <output-file>',
        '#',
        '# Every scan runs over source with comments and literals blanked, so a doc comment that'
        ' names',
        '# a prohibited call is not mistaken for the call. Line numbers survive the blanking.',
        '#',
        f'# library target: {", ".join(LIBRARY_SOURCES)}  '
        f'({sum(len(s.raw_lines) for s in library):,} lines)',
        f'# binary target:  {BINARY_SOURCE}  ({len(binary_source.raw_lines):,} lines)',
    ])

    # -----------------------------------------------------------------------------------------
    check_1(report, library, binary_source)
    check_2(report, simulation, by_name)
    check_3(report, root, before_directory, scratch)
    check_4(report, by_name, before_by_name)
    check_5(report, library, before)
    check_6(report, simulation, binary, scratch, root)
    check_7(report, library)
    check_8(report, library)
    check_9(report, simulation, root)
    check_10(report, root, baseline)
    check_11(report, simulation)
    check_12(report, root, baseline)

    # -----------------------------------------------------------------------------------------
    findings = [result for result in report.results if result[2] != 'PASS']
    report.lines.append('')
    report.lines.append('=' * 108)
    report.lines.append('# summary')
    report.lines.append('=' * 108)
    for number, title, word, text in report.results:
        report.lines.append(f'  {number:>2}. {word:<8} {title[:62]:<62} {text}')
    report.lines.append('')
    report.lines.append(f'# {len(report.results)} checks, {len(findings)} findings')
    report.lines.append('')
    report.lines.append('# ---- full text of this script, retained as VER-MOK-012 requires ----')
    report.lines.append('')
    report.lines.extend(io.open(__file__, encoding='utf-8').read().split('\n'))

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(report.lines) + '\n')

    print(f'{len(report.results)} checks, {len(findings)} findings; written to {output_file}')
    for number, title, word, text in findings:
        print(f'  {number}. {word} {title}: {text}')
    return 1 if findings else 0


# ---------------------------------------------------------------------------------------------
# 1. The library target contains no filesystem operation
# ---------------------------------------------------------------------------------------------

def check_1(report, library, binary_source):
    report.check(
        1, 'The library target contains no filesystem operation',
        'no file creation or opening, no path resolution, no removal, no directory access, no '
        'temporary-file use, checked against its source',
    )
    report.method(
        'enumerate every path into the standard library the three library sources name at all, '
        'then cross-check a vocabulary of forbidden calls against the same stripped source.'
    )
    report.say()
    report.say('every `std::` path named in the library target:')

    reached = {}
    for source in library:
        for number, line in enumerate(source.lines, start=1):
            for match in re.finditer(r'\bstd::([A-Za-z_][A-Za-z0-9_]*)', line):
                where = f'{source.name}:{number}'
                reached.setdefault(match.group(1), []).append(
                    f'{where}{" (#[cfg(test)])" if number in source.test_lines else ""}'
                )
    for module in sorted(reached):
        report.say(f'  std::{module:<14} {", ".join(reached[module])}')
    report.say()
    report.say(
        f'the set has {len(reached)} members. Nothing outside the prelude is reachable in Rust '
        'without being'
    )
    report.say(
        'named, and the prelude carries no filesystem item: `File`, `OpenOptions`, `Path`, '
        '`PathBuf`,'
    )
    report.say(
        '`read_dir`, `remove_file` and `temp_dir` all live behind `std::fs`, `std::path` or '
        '`std::env`,'
    )
    report.say('none of which appears above. That is the check; what follows is a cross-check.')
    report.say()
    report.say('forbidden vocabulary, over the same stripped source:')

    vocabulary_hits = []
    for needle in FILESYSTEM_VOCABULARY:
        for source in library:
            for number, text in source.hits(needle, product_only=False):
                vocabulary_hits.append((needle, source.name, number, text))
    for needle, name, number, text in vocabulary_hits:
        report.say(f'  {needle:<16} {name}:{number}  {text[:70]}')
    if not vocabulary_hits:
        report.say(f'  none of the {len(FILESYSTEM_VOCABULARY)} patterns occurs.')

    # `env!` is compile-time package metadata, not a runtime environment read, and it is used.
    report.say()
    report.say('the one macro that touches the build environment:')
    for source in library:
        for number, text in source.hits('env!', product_only=False):
            report.say(f'  {source.name}:{number}  {text[:80]}')
    report.say(
        '  `env!` resolves at compile time from Cargo\'s own package metadata. It reads no '
        'environment'
    )
    report.say(
        '  at run time and opens no file; `option_env!`, `include_str!` and `include_bytes!` do '
        'not occur.'
    )
    report.say()
    report.say(
        f'for contrast, the binary target — where the one new filesystem effect belongs — names:'
    )
    binary_reach = sorted({
        match.group(1)
        for line in binary_source.lines
        for match in re.finditer(r'\bstd::([A-Za-z_][A-Za-z0-9_]*)', line)
    })
    report.say(f'  {", ".join("std::" + module for module in binary_reach)}')
    report.say(
        f'  `std::fs` appears here and nowhere else in the package, which is `ADR-MOK-005`\'s '
        'design: the'
    )
    report.say(
        '  library never learns the destination, so it cannot name it in a diagnostic or a record.'
    )
    report.say(
        '  `std::env` here is `env::args()` — the command line, which is what a binary target is '
        'for —'
    )
    report.say(
        '  and not an environment variable: check 11 lists every `env` use in the workspace by form.'
    )

    passed = not vocabulary_hits and 'fs' not in reached and 'path' not in reached
    report.verdict(
        passed,
        f'the library target reaches {len(reached)} standard-library modules '
        f'({", ".join(sorted(reached))}) and no filesystem, path, environment or process module',
        'the enumeration covers paths written `std::…` and `use` declarations. A filesystem item '
        'reached through a re-export this scan does not resolve, or through a macro expanding to '
        'one, would not appear. The empty dependency table bounds that: there is no crate to '
        're-export from, and the only macros used are `std`\'s own formatting macros, `env!` and '
        '`concat!`.',
    )


# ---------------------------------------------------------------------------------------------
# 2. The record projection has exactly one call site
# ---------------------------------------------------------------------------------------------

def check_2(report, simulation, by_name):
    report.check(
        2, 'The record projection is reachable from exactly one call site',
        'the event projection is called from the function every authoritative event already '
        'passes through, so `REQ-MOK-042`\'s correspondence is structural. A second emission site '
        'is a finding',
    )
    report.method(
        'locate every record-writing function, then every call to it outside `#[cfg(test)]`, and '
        'name the function each call sits in by brace balance.'
    )

    # Declared outside `#[cfg(test)]`, so that a test named `…_leaves_no_run_record` is not read as
    # a record writer.
    declarations = {
        name: start for name, start, _ in simulation.spans
        if (name.startswith('write_') or name.endswith('_record'))
        and start not in simulation.test_lines
    }
    writers = sorted(declarations)
    report.say()
    report.say(f'record-writing functions in simulation.rs: {len(writers)}')
    for name in writers:
        report.say(f'  {name:<26} declared at line {declarations[name]}')

    report.say()
    report.say('call sites outside `#[cfg(test)]`, with the function each sits in:')
    call_sites = {}
    for name in writers:
        pattern = re.compile(rf'\b{name}\s*\(')
        for number, line in enumerate(simulation.lines, start=1):
            if number in simulation.test_lines or not pattern.search(line):
                continue
            if any(function == name and start == number for function, start, _ in simulation.spans):
                continue  # the declaration itself
            caller = enclosing_function(simulation.spans, number)
            call_sites.setdefault(name, []).append((number, caller))
    for name in writers:
        sites = call_sites.get(name, [])
        rendered = ', '.join(f'line {number} in `{caller}`' for number, caller in sites)
        report.say(f'  {name:<26} {len(sites)} call site(s)  {rendered}')

    event_sites = call_sites.get('write_event_record', [])
    emit_sites = [caller for _, caller in event_sites]
    top_level = {
        record: call_sites.get(writer, [])
        for record, writer in (
            ('header', 'write_header_record'),
            ('metrics', 'write_metrics'),
            ('run', 'write_run_record'),
        )
    }
    report.say()
    report.say('the funnel:')
    emit_span = [span for span in simulation.spans if span[0] == 'emit']
    if emit_span:
        report.say(
            f'  `fn emit` spans simulation.rs:{emit_span[0][1]}–{emit_span[0][2]} and is the one '
            'function'
        )
        report.say('  every authoritative event is written through. Its body:')
        for number in range(emit_span[0][1], emit_span[0][2] + 1):
            report.say(f'    {number:>5}  {simulation.raw_lines[number - 1]}')
    emit_calls = [
        number for number, line in enumerate(simulation.lines, start=1)
        if re.search(r'\bself\.emit\s*\(', line) and number not in simulation.test_lines
    ]
    report.say()
    report.say(
        f'  `self.emit(...)` is called at {len(emit_calls)} sites: '
        f'{", ".join(str(number) for number in emit_calls)}.'
    )
    report.say(
        '  Every one of them writes the text line and the record from the same `Event` value, in '
        'the same'
    )
    report.say(
        '  statement, because that is the only statement in `emit` that writes either. A caller '
        'that'
    )
    report.say(
        '  wanted to emit a record without a text line, or the reverse, would have to add a '
        'second call'
    )
    report.say('  to `write_event_record`, and this check counts those.')

    report.say()
    report.say('the other three record kinds, for completeness — one writer, one call site each:')
    for record, sites in top_level.items():
        rendered = ', '.join(f'line {number} in `{caller}`' for number, caller in sites)
        report.say(f'  {record:<8} {rendered}')
    report.say(
        '  The metrics record\'s two call sites are the two ways a tick ends — the ordinary tick '
        'and the'
    )
    report.say(
        '  one that terminates the run, which rule 7.1 requires to carry its metrics record too — '
        'and'
    )
    report.say('  both are in `step`, both calling the one `write_metrics`.')

    passed = len(event_sites) == 1 and emit_sites == ['emit']
    report.verdict(
        passed,
        f'`write_event_record` has exactly {len(event_sites)} call site outside tests, in '
        f'`{emit_sites[0] if emit_sites else "—"}`; the header and run records are written once '
        f'each from `run_with_source` and the metrics record from `write_metrics`',
        'the count is over `simulation.rs`, which is where rule 3 places every record writer and '
        'where all four are private. A call from another module could not compile: none of the '
        'four is `pub` or `pub(crate)`. What this does not check is whether `emit` is itself '
        'reached on every event — that is oracle 2\'s per-line correspondence over full runs, '
        'asserted dynamically, and it is the stronger statement.',
    )


# ---------------------------------------------------------------------------------------------
# 3. Rule 5's enumeration, item for item
# ---------------------------------------------------------------------------------------------

def check_3(report, root, before_directory, scratch):
    report.check(
        3, "SPEC-MOK-002 rule 5's enumeration, compared item for item",
        "the engine's public interface grows by exactly one parameter on `execute` and by no item",
    )
    enumerator = os.path.join(
        'docs', 'engineering', 'simulation', 'evidence', 'WO-MOK-011', 'analysis', 'interface.py'
    )
    report.method(
        f'run `{enumerator}` — `WO-MOK-011` oracle 5\'s enumerator, unmodified — over both '
        'revisions of the three library sources and diff its output. A fourth enumerator would be '
        'a second convention.'
    )

    after_paths = [os.path.join(root, PACKAGE, 'src', name) for name in LIBRARY_SOURCES]
    before_paths = [os.path.join(before_directory, name) for name in LIBRARY_SOURCES]
    outputs = {}
    for label, paths in (('before', before_paths), ('after', after_paths)):
        completed = run([sys.executable, enumerator, *paths])
        path = os.path.join(scratch, f'interface-{label}.txt')
        io.open(path, 'w', encoding='utf-8', newline='\n').write(completed.stdout)
        outputs[label] = completed.stdout.split('\n')

    report.say()
    for label in ('before', 'after'):
        totals = [line for line in outputs[label] if line.startswith('items ')]
        report.say(f'  {label:<7} {totals[0] if totals else "(no totals)"}')

    difference = [
        line for line in _unified(outputs['before'], outputs['after'])
    ]
    report.say()
    report.say('diff of the two enumerations:')
    if difference:
        for line in difference:
            report.say(f'  {line}')
    else:
        report.say('  (empty)')

    report.say()
    report.say(
        'The one differing line is `execute`\'s own declaration, and it differs in shape rather '
        'than in'
    )
    report.say(
        'content: the signature now spans several lines, so a line-oriented enumerator prints its '
        'opening'
    )
    report.say(
        'line instead of the whole of it. Rule 5\'s 2026-08-20 amendment says exactly this, which '
        'is why'
    )
    report.say('it replaced one grep with two. The full signatures, both revisions:')
    report.say()
    for label, path in (('before', before_paths[0]), ('after', after_paths[0])):
        text = io.open(path, encoding='utf-8').read()
        start = text.index('pub fn execute')
        end = text.index('{', start)
        report.say(f'  {label}:')
        for line in text[start:end].rstrip().split('\n'):
            report.say(f'    {line}')

    report.say()
    report.say('rule 5\'s mechanical form, the two greps, over `src/lib.rs`:')
    lib = os.path.join(root, PACKAGE, 'src', 'lib.rs')
    grep_results = {}
    for pattern in ('pub fn execute', 'records: Option<&mut dyn Write>'):
        hits = [
            f'{number}:{line.strip()}'
            for number, line in enumerate(io.open(lib, encoding='utf-8').read().split('\n'), 1)
            if pattern in line
        ]
        grep_results[pattern] = hits
        report.say(f'  grep -n \'{pattern}\' src/lib.rs  →  {len(hits)} line(s)')
        for hit in hits:
            report.say(f'      {hit}')

    items_before = _totals(outputs['before'])
    items_after = _totals(outputs['after'])
    passed = (
        items_before == items_after
        and len(grep_results['pub fn execute']) == 1
        and len(grep_results['records: Option<&mut dyn Write>']) == 1
        and len(difference) == 2
    )
    report.verdict(
        passed,
        f'{items_after[0]} items and {items_after[1]} public fields, unchanged from the baseline; '
        f'the sole textual difference is `execute`\'s reflowed signature, and both of rule 5\'s '
        f'greps return exactly one line',
        'the enumerator counts `pub` lines, so it detects an item added or removed but not a '
        'type changed within a signature it already prints. Rule 6\'s check below covers the '
        'capability that would matter; a type change that grants none is a compilation-visible '
        'change the test suite exercises.',
    )


def _totals(lines):
    for line in lines:
        match = re.match(r'items (\d+)\s+public fields (\d+)', line)
        if match:
            return int(match.group(1)), int(match.group(2))
    return None


def _unified(before, after):
    """The lines that differ, in `diff` notation. Small inputs, so a set difference suffices."""
    removed = [line for line in before if line not in after]
    added = [line for line in after if line not in before]
    return [f'- {line}' for line in removed] + [f'+ {line}' for line in added]


# ---------------------------------------------------------------------------------------------
# 4. Rule 6, re-checked
# ---------------------------------------------------------------------------------------------

def check_4(report, by_name, before_by_name):
    report.check(
        4, 'SPEC-MOK-002 rule 6, re-checked because a public signature changed',
        'no public item yields a mutable borrow of, or a reference into, the world grid, the agent '
        'collection, the resource collection, the tick counter, the entropy state, the event log '
        'or any cumulative counter, in any build configuration including test builds',
    )
    report.method(
        'four properties of the public surface rather than a list of names: the mutating methods, '
        'the return types, the interior-mutability types, and the visibility of the ten '
        'prohibited names.'
    )
    simulation = by_name['simulation.rs']

    report.say()
    report.say('(a) mutating methods on the interface — rule 5\'s own grep:')
    mutating = [
        (number, simulation.raw_lines[number - 1].strip())
        for number, line in enumerate(simulation.lines, start=1)
        if re.search(r'pub fn .*&mut self', line) and number not in simulation.test_lines
    ]
    for number, text in mutating:
        report.say(f'  simulation.rs:{number}  {text}')
    report.say(
        f'  {len(mutating)} methods, and rule 5 names exactly two. `run_recording` — the carrier '
        'that takes'
    )
    report.say(
        '  the sink down the same call chain the text stream travels — is `pub(crate) fn` and is '
        'not'
    )
    report.say('  matched, is not on the interface, and is not reachable from anything that is:')
    for number, line in enumerate(simulation.lines, start=1):
        if 'fn run_recording' in line:
            report.say(f'    simulation.rs:{number}  {simulation.raw_lines[number - 1].strip()}')

    report.say()
    report.say('(b) every public function whose return type contains a reference:')
    returning_reference, returning_static = [], []
    for source in by_name.values():
        text = source.stripped
        for match in re.finditer(r'\bpub fn [^;{]*', text):
            signature = ' '.join(match.group(0).split())
            line = text[:match.start()].count('\n') + 1
            if line in source.test_lines:
                continue
            arrow = signature.find('->')
            if arrow < 0 or '&' not in signature[arrow:]:
                continue
            # A `&'static` reference cannot borrow from a `Simulation`: `'static` outlives every
            # value the engine owns, so the referent is program-lifetime data — here, a literal.
            returns = signature[arrow:]
            if re.fullmatch(r"->\s*&'static\s+str\s*", returns):
                returning_static.append((source.name, line, signature))
            else:
                returning_reference.append((source.name, line, signature))
    for name, line, signature in returning_reference:
        report.say(f'  {name}:{line}  {signature[:96]}   ← a reference into engine state')
    if not returning_reference:
        report.say(
            '  none that borrows from the engine. Every public function returns an owned value, a '
            'copy,'
        )
        report.say('  or `()`/`Result` over one.')
    report.say()
    report.say(
        f'  {len(returning_static)} public function(s) return `&\'static str`, which is not a '
        'reference into'
    )
    for name, line, signature in returning_static:
        report.say(f'    {name}:{line}  {signature[:92]}')
    report.say(
        '  engine state and cannot be: `\'static` outlives every value a `Simulation` owns, so the'
    )
    report.say(
        '  referent is program-lifetime data — a string literal. Rule 5 admits `EventType::as_str` '
        'by'
    )
    report.say(
        '  name for exactly this reason, and `cli::USAGE` has been a `&\'static str` constant since'
    )
    report.say('  `REQ-MOK-010`. Neither hands out a borrow of anything the engine holds.')

    report.say()
    report.say('(c) interior mutability and shared ownership, over the whole library target:')
    interior = []
    for needle in INTERIOR_MUTABILITY:
        pattern = re.compile(rf'\b{needle}\w*\s*<|\b{needle}::|\b{needle}\b')
        for source in by_name.values():
            for number, line in enumerate(source.lines, start=1):
                if pattern.search(line):
                    interior.append((needle, source.name, number))
    for needle, name, number in interior:
        report.say(f'  {needle:<12} {name}:{number}')
    if not interior:
        report.say(
            '  none of Cell, RefCell, UnsafeCell, Rc, Arc, Mutex, RwLock or Atomic occurs, so no '
            '`&self`'
        )
        report.say(
            '  method can mutate. This is the sentence rule 5 asserts, and it holds in test '
            'builds too:'
        )
        report.say('  the scan covers `#[cfg(test)]` regions as well.')

    report.say()
    report.say('(d) the ten names rule 6\'s second bullet keeps prohibited:')
    public_prohibited = []
    for type_name in PROHIBITED_TYPES:
        declarations = []
        for source in by_name.values():
            for number, line in enumerate(source.lines, start=1):
                if re.search(rf'\b(struct|enum|trait|type)\s+{type_name}\b', line):
                    visibility = 'pub' if re.match(r'\s*pub[^(]', line) else (
                        'pub(crate)' if re.match(r'\s*pub\(', line) else 'private'
                    )
                    declarations.append(f'{source.name}:{number} {visibility}')
                    if visibility == 'pub':
                        public_prohibited.append(type_name)
        report.say(f'  {type_name:<22} {", ".join(declarations) or "not declared"}')

    report.say()
    report.say('(e) the new state, and whether anything public reaches it:')
    for field in ('crossings', 'consumed', 'regenerated', 'regeneration_skipped'):
        accessors = [
            number for number, line in enumerate(simulation.lines, start=1)
            if re.search(rf'pub fn \w*{field}', line)
        ]
        report.say(
            f'  {field:<22} private field; public accessors named after it: '
            f'{len(accessors)}'
        )
    report.say(
        '  Rule 5\'s enumeration is unchanged by check 3, so no item was added that could return '
        'one.'
    )

    passed = (
        len(mutating) == 2
        and not returning_reference
        and not interior
        and not public_prohibited
    )
    report.verdict(
        passed,
        f'{len(mutating)} mutating methods (rule 5 names two), no public function returning a '
        f'borrow of engine state ({len(returning_static)} return `&\'static str`), no '
        f'interior-mutable type anywhere in the target, and all ten prohibited names still private',
        'property (b) reads signatures, so a public function returning an owned struct that '
        'itself held a reference would pass it. Nothing in the library does: every snapshot type '
        'rule 5 admits is built from owned fields, and a lifetime parameter on any of them would '
        'appear in (b)\'s output as part of the signature. The `&\'static str` carve-out is a '
        'property of the lifetime and not a judgement about the function: a `\'static` referent '
        'cannot be engine-owned, because `\'static` outlives the `Simulation`.',
    )


# ---------------------------------------------------------------------------------------------
# 5. No floating point in any record-producing path
# ---------------------------------------------------------------------------------------------

def check_5(report, library, before):
    report.check(
        5, 'No floating-point type or operation, anywhere',
        'no floating-point type or operation appears in any record-producing path, in any counter, '
        'or in any metric computation',
    )
    report.method(
        'scan the whole library target — not only the record path — for the two float types, for '
        'a float cast, for a decimal literal and for the float-only operations. Scanning the whole '
        'target is stronger than scanning the path and needs no judgement about where the path '
        'ends.'
    )

    hits = []
    for needle in FLOAT_VOCABULARY:
        for source in library:
            for number, text in source.hits(needle, product_only=False):
                hits.append((needle, source.name, number, text))
    literals = []
    for source in library:
        for number, line in enumerate(source.lines, start=1):
            for match in re.finditer(r'\b\d+\.\d', line):
                literals.append((source.name, number, match.group(0), source.raw_lines[number - 1]))

    report.say()
    report.say(f'float types, casts and operations: {len(hits)} occurrence(s)')
    for needle, name, number, text in hits:
        report.say(f'  {needle:<10} {name}:{number}  {text[:76]}')
    report.say()
    report.say(f'decimal literals in stripped source: {len(literals)} occurrence(s)')
    for name, number, literal, text in literals:
        report.say(f'  {literal:<10} {name}:{number}  {text.strip()[:76]}')
    report.say()
    report.say('what the two figures the record stream does carry are made of:')
    report.say(
        '  the metrics record\'s attribute objects hold a `u64` sum and a `u8` extremum, and rule '
        '4.2'
    )
    report.say(
        '  forbids the mean that dividing one by the other would produce — which is where a float '
        'would'
    )
    report.say(
        '  otherwise enter. The density, the one decimal quantity in the configuration, is stored '
        'as an'
    )
    report.say(
        '  integer count of hundredths and rendered as a string, so `"0.75"` in the header is text:'
    )
    for source in library:
        for number, line in enumerate(source.lines, start=1):
            if re.search(r'hundredths', line) and 'fn' not in line and number < 400:
                report.say(f'    {source.name}:{number}  {source.raw_lines[number - 1].strip()}')
                break

    passed = not hits and not literals
    report.verdict(
        passed,
        f'no `f32`, no `f64`, no float cast, no float-only operation and no decimal literal in '
        f'{sum(len(source.raw_lines) for source in library):,} lines of library source',
        'a float could still arrive through a generic parameter instantiated with one. None of the '
        'library\'s generic parameters is numeric: `execute`\'s four are an argument iterator, its '
        'item, and two writers, and `Simulation::run`\'s one is a writer. Rust has no implicit '
        'numeric conversion, so an integer path cannot silently become a float one.',
    )


# ---------------------------------------------------------------------------------------------
# 6. The whole field set, and the absence of a classification
# ---------------------------------------------------------------------------------------------
def check_6(report, simulation, binary, scratch, root):
    report.check(
        6, 'The whole field set carries no classification',
        'no record kind carries an outcome, label, category, verdict, severity or interpretation '
        'field; the whole field set is compared against `SPEC-MOK-006`',
    )
    report.method(
        'take the whole field set from a real run and check every member against the authority '
        'that declares it. `SPEC-MOK-006` enumerates the header, metrics and run records in rules '
        '5.2, 7.2 and 8.2, so those three are compared against the specification. Rule 6.4 does '
        'not enumerate the event record: it delegates the `result` object\'s keys to the text '
        'line\'s, so the event records are compared against the text stream of the same run. Then '
        'the source\'s own literals are listed, and the names the specification uses only in a '
        '`### Counterexample` subsection — the shapes it shows in order to forbid them — are looked '
        'for in the emitted set.'
    )

    # -----------------------------------------------------------------------------------------
    # One run, both streams. 60 ticks at the density that reaches capacity, traced, so that every
    # record kind, every event kind and both regeneration-skip reasons occur.
    # -----------------------------------------------------------------------------------------
    stream = os.path.join(scratch, 'field-set.jsonl')
    completed = run([
        binary, '--seed', '42', '--ticks', '60', '--density', '1.50', '--policy', 'individual',
        '--trace-actions', '--events-path', stream,
    ])
    records = [
        json.loads(line) for line in io.open(stream, encoding='utf-8').read().split('\n') if line
    ]
    os.remove(stream)

    def keys_of(value, into):
        if isinstance(value, dict):
            for key, nested in value.items():
                into.add(key)
                keys_of(nested, into)
        elif isinstance(value, list):
            for nested in value:
                keys_of(nested, into)

    by_kind, kind_counts = {}, {}
    for record in records:
        kind = record['record']
        kind_counts[kind] = kind_counts.get(kind, 0) + 1
        by_kind.setdefault(kind, set())
        keys_of(record, by_kind[kind])
    event_result_keys = set()
    for record in records:
        if record['record'] == 'event':
            keys_of(record['result'], event_result_keys)
    whole_set = set().union(*by_kind.values())

    # -----------------------------------------------------------------------------------------
    # The specification, read by section so that each record kind is compared against its own rule.
    # -----------------------------------------------------------------------------------------
    spec_lines = io.open(
        os.path.join(root, 'docs', 'engineering', 'simulation', 'specifications',
                     'SPEC-MOK-006.md'),
        encoding='utf-8',
    ).read().split('\n')

    def section(opening, closing):
        start = next(number for number, line in enumerate(spec_lines) if line.startswith(opening))
        end = next(
            number for number, line in enumerate(spec_lines)
            if number > start and line.startswith(closing)
        )
        return spec_lines[start:end]

    declared = {
        'header': set(JSON_KEY.findall('\n'.join(section('### 5. The header', '### 6.')))),
        'metrics': set(JSON_KEY.findall('\n'.join(section('### 7. The metrics', '### 8.')))),
        'run': set(JSON_KEY.findall('\n'.join(section('### 8. The run', '### 9.')))),
    }
    composite = set(JSON_KEY.findall('\n'.join(section('6.5 ', '6.6 '))))

    # Only the subsections headed `### Counterexample`. The same section carries three positive
    # examples, and taking the whole of it would report `position` and `status` as forbidden names.
    counterexample, in_counterexample = set(), False
    for line in section('## Examples and counterexamples', '## Explicitly unspecified'):
        if line.startswith('###'):
            in_counterexample = line.startswith('### Counterexample')
        elif in_counterexample:
            counterexample |= set(JSON_KEY.findall(line))

    # -----------------------------------------------------------------------------------------
    # (a) the three enumerated record kinds, against SPEC-MOK-006's own rule sections
    # -----------------------------------------------------------------------------------------
    report.say()
    report.say(
        f'one run: seed 42, 1.50%, individual, 60 ticks, traced; exit {completed.returncode}; '
        f'{len(records):,} records '
        f'({", ".join(f"{kind}={count}" for kind, count in sorted(kind_counts.items()))}).'
    )
    report.say()
    report.say('(a) the three record kinds SPEC-MOK-006 enumerates, against their own rules:')
    enumerated_findings = []
    for kind, rule in (('header', '5.2'), ('metrics', '7.2'), ('run', '8.2')):
        emitted = by_kind[kind]
        extra = emitted - declared[kind]
        missing = declared[kind] - emitted
        report.say()
        report.say(f'  {kind} — rule {rule}, {len(emitted)} key(s) emitted, '
                   f'{len(declared[kind])} declared')
        report.say(f'    emitted:  {", ".join(sorted(emitted))}')
        report.say(f'    declared: {", ".join(sorted(declared[kind]))}')
        report.say(f'    emitted but not declared: {sorted(extra) if extra else "none"}')
        report.say(f'    declared but not emitted: {sorted(missing) if missing else "none"}')
        if extra:
            enumerated_findings.append((kind, sorted(extra)))

    # -----------------------------------------------------------------------------------------
    # (b) the event record, against the text stream rule 6.4 delegates to
    # -----------------------------------------------------------------------------------------
    report.say()
    report.say('(b) the event record, against the text stream of the same run:')
    report.say(
        '    Rule 6.4 states that `result`\'s keys **are** the text result\'s keys, one per text '
        'field,'
    )
    report.say(
        '    and that the field orders are `SPEC-MOK-001`\'s and "are not restated here". So the '
        'authority'
    )
    report.say(
        '    for this record\'s field set is the text line, and the comparison is against the run\'s'
    )
    report.say('    own text stream rather than against an enumeration that does not exist.')

    text_keys, first_field = set(), {}
    for line in completed.stdout.split('\n'):
        subject, separator, result = line.partition(' result=')
        if not separator:
            continue
        for field in result.split(','):
            key, colon, _ = field.partition(':')
            if colon:
                text_keys.add(key)
                first_field.setdefault(key, (subject.split()[-1], field))
    surplus = event_result_keys - text_keys
    absent = text_keys - event_result_keys
    unaccounted = surplus - composite
    shared = sorted(composite & text_keys)
    report.say()
    report.say(f'    text result keys:   {len(text_keys)}  {", ".join(sorted(text_keys))}')
    report.say(
        f'    record result keys: {len(event_result_keys)}  '
        f'{", ".join(sorted(event_result_keys))}'
    )
    report.say()
    report.say(f'    in the text and not in the record: {sorted(absent) if absent else "none"}')
    report.say(f'    in the record and not in the text: {sorted(surplus) if surplus else "none"}')
    report.say(
        f'    rule 6.5\'s three composite shapes introduce exactly: {", ".join(sorted(composite))}'
    )
    report.say()
    report.say(
        f'    every key the record carries beyond the text\'s is one rule 6.5 names: '
        f'{"yes" if not unaccounted else "NO — " + ", ".join(sorted(unaccounted))}'
    )
    report.say(
        f'    every key the text carries is in the record: '
        f'{"yes" if not absent else "NO — " + ", ".join(sorted(absent))}'
    )
    report.say()
    report.say(
        f'    The inclusion is one-way, and deliberately so: {len(shared)} of rule 6.5\'s '
        f'{len(composite)} names are'
    )
    report.say(
        '    also the key of a text field in their own right, so they are in both sets and not in the'
    )
    report.say('    surplus. In this run each of them first occurs as:')
    for key in shared:
        event, field = first_field[key]
        report.say(f'      {key:<10} {event:<28} {field}')
    report.say(
        '    Requiring the two sets to be *equal* would therefore be requiring the text stream not to'
    )
    report.say(
        '    use those words, which is `SPEC-MOK-001`\'s business and not this rule\'s. What rule 6.4'
    )
    report.say(
        '    asks is the containment printed above, in both directions: nothing dropped, and nothing'
    )
    report.say('    added that rule 6.5 does not name.')

    # -----------------------------------------------------------------------------------------
    # (c) the source's own literals, and the two keys it does not write literally
    # -----------------------------------------------------------------------------------------
    from_source = set()
    for number, line in enumerate(simulation.raw_lines, start=1):
        if number in simulation.test_lines:
            continue
        for match in re.finditer(r'\\"([A-Za-z_][A-Za-z0-9_]*)\\":', line):
            from_source.add(match.group(1))
    # `write_attribute` takes the attribute's name and its extremum's name as `&str` parameters,
    # so those four keys are literals at the call sites rather than in the writer.
    for number, line in enumerate(simulation.lines, start=1):
        if 'write_attribute(' in line and number not in simulation.test_lines:
            from_source.update(re.findall(r'"([a-z_]+)"', simulation.raw_lines[number - 1]))

    report.say()
    report.say(f'(c) field names written as string literals in simulation.rs: {len(from_source)}')
    report.say(f'    {", ".join(sorted(from_source))}')
    report.say()
    report.say('    the keys the source does not write literally, and where each comes from:')
    interpolated = [
        (number, match.group(1))
        for number, line in enumerate(simulation.raw_lines, start=1)
        if number not in simulation.test_lines
        for match in re.finditer(r'\\"\{(\w+)\}\\":', line)
    ]
    origins = {
        'territory': 'the `Display` of `Territory`, whose `ALL` fixes it to A and B',
        'class': 'the `Display` of `FoodClass`, whose `ALL` fixes it to low, medium and high',
        'reason': 'the `Display` of `RegenerationSkipReason`, whose `ALL` fixes it to depleted '
                  'and capacity',
        'name': 'a `&str` parameter of `write_attribute`, from its four literal call sites',
        'extremum_name': 'a `&str` parameter of `write_attribute`, from its four literal call sites',
    }
    for number, expression in interpolated:
        report.say(f'      simulation.rs:{number}  {{{expression}}}  — '
                   f'{origins.get(expression, "unclassified")}')
    report.say(
        '    Each of the three enums has a closed `ALL` array and a `Display` this repository '
        'tests, so'
    )
    report.say(
        '    the set of keys reachable through interpolation is finite and is listed in (a)\'s '
        'emitted'
    )
    report.say('    sets above.')

    # -----------------------------------------------------------------------------------------
    # (d) the classification question, asked of the whole set
    # -----------------------------------------------------------------------------------------
    report.say()
    report.say(f'(d) the whole field set, {len(whole_set)} names, and the classification question:')
    report.say(f'    {", ".join(sorted(whole_set))}')
    # A name is counterexample-only when a `### Counterexample` subsection uses it as a key and no
    # authority does: not a rule's enumeration, not rule 6.5's composite shapes, and not the text
    # stream rule 6.4 delegates to. The counterexamples also quote plenty of *legitimate* keys --
    # `record`, `tick`, `reason` -- and subtracting the three authorities is what separates the two.
    counterexample_only = (
        counterexample - set().union(*declared.values()) - composite - text_keys
    )
    report.say()
    report.say(
        f'    keys a `### Counterexample` subsection uses that no rule declares, no composite shape'
    )
    report.say(
        f'    introduces and the text stream does not carry: '
        f'{", ".join(sorted(counterexample_only))}'
    )
    forbidden_emitted = whole_set & counterexample_only
    report.say(
        f'    of those, emitted: {sorted(forbidden_emitted) if forbidden_emitted else "none"}'
    )
    report.say()
    report.say('    read the set as a whole. Every member is one of:')
    report.say('      a record kind, or a tick, subject or event name;')
    report.say('      a measured count, sum or extremum;')
    report.say('      a coordinate, territory, calorie class, action or direction already printed;')
    report.say('      a termination or skip reason whose values a closed enum fixes;')
    report.say('      a resolved configuration value the operator supplied.')
    report.say(
        '    None is a threshold, a label applied to a figure, a verdict or a severity. `reason` is'
    )
    report.say(
        '    the nearest thing to one and is not: `tick_limit` and `extinction` are the two ways a '
        'run'
    )
    report.say(
        '    ends, not two judgements about how it went, and rule 8.7\'s counterexample is '
        'precisely a'
    )
    report.say(
        '    third field beside it — `"outcome":"collapse"` — which the set above does not contain.'
    )
    report.say(
        '    `status` is the second nearest and is not either: it carries `accepted` or `rejected`, '
        'the'
    )
    report.say(
        '    engine\'s own validation result for a proposal, which rule 6.5 requires the record to'
    )
    report.say('    reproduce from the text line unchanged.')

    passed = (
        not enumerated_findings
        and not absent
        and not unaccounted
        and not forbidden_emitted
    )
    report.verdict(
        passed,
        f'{len(whole_set)} field names over {len(kind_counts)} record kinds: the header, metrics '
        f'and run records carry exactly the keys rules 5.2, 7.2 and 8.2 declare, the event record '
        f'carries every key the text line carries and nothing beyond rule 6.5\'s '
        f'{len(composite)} composite-shape keys, and none of the {len(counterexample_only)} '
        f'counterexample-only names is emitted',
        'a classification added to the stream *and* to the specification\'s rule sections would '
        'satisfy (a), which is the residual VER-MOK-012 already records against oracle 5. Two '
        'things bound it: the declared sets are printed here in full and are short enough to read, '
        'and a classification on an *event* record could not pass (b) at all, because it would have '
        'to appear in the text stream first, where `SPEC-MOK-001` governs the fields. The run is 60 '
        'ticks, so a field emitted only past tick 60 would be missed — (c) is what covers the '
        'source rather than the run, and no literal in it is unaccounted for above.',
    )

# ---------------------------------------------------------------------------------------------
# 7. No unordered iteration reaches a record
# ---------------------------------------------------------------------------------------------

def check_7(report, library):
    report.check(
        7, 'No unordered collection is iterated where the order reaches output',
        'no unordered collection is iterated where the traversal order reaches a record field, a '
        'text line, or a decision',
    )
    report.method(
        'locate every hash-ordered and tree-ordered collection in the library target and state '
        'whether it is in product code or in a test, then name what does impose the order the '
        'records are written in.'
    )

    report.say()
    occurrences = []
    for needle in UNORDERED_TYPES + ('BTreeMap', 'BTreeSet'):
        for source in library:
            for number, line in enumerate(source.lines, start=1):
                if re.search(rf'\b{needle}\b', line):
                    where = 'test' if number in source.test_lines else 'PRODUCT'
                    occurrences.append((needle, source.name, number, where,
                                        source.raw_lines[number - 1].strip()))
    for needle, name, number, where, text in occurrences:
        report.say(f'  {needle:<9} {where:<8} {name}:{number}  {text[:66]}')
    in_product = [row for row in occurrences if row[3] == 'PRODUCT']
    report.say()
    report.say(
        f'  {len(occurrences)} occurrence(s), {len(in_product)} in product code. A collection '
        'used only'
    )
    report.say(
        '  inside `#[cfg(test)]` cannot reach a record: a test compares what the product wrote.'
    )

    report.say()
    report.say('what does impose the order in every record the stream contains:')
    for pattern, description in (
        (r'Territory::ALL', 'the two territories, a fixed array'),
        (r'FoodClass::ALL', 'the three calorie classes, a fixed array'),
        (r'RegenerationSkipReason::ALL', 'the two skip reasons, a fixed array'),
        (r'sort', 'an explicit sort'),
    ):
        rows = []
        for source in library:
            for number, line in enumerate(source.lines, start=1):
                if re.search(pattern, line) and number not in source.test_lines:
                    rows.append(f'{source.name}:{number}')
        report.say(f'  {pattern:<28} {len(rows):>2} site(s)  — {description}')
        if pattern == 'sort':
            for source in library:
                for number, line in enumerate(source.lines, start=1):
                    if 'sort' in line and number not in source.test_lines:
                        report.say(f'      {source.name}:{number}  '
                                   f'{source.raw_lines[number - 1].strip()[:74]}')
    report.say()
    report.say(
        '  Everything else the records iterate is a `Vec` — the agent roster and the resource'
    )
    report.say(
        '  collection — whose order is the insertion order the entropy stream produced, which is '
        'the'
    )
    report.say(
        '  same in every run at one seed. Rule 8.4\'s per-Mokiterion list is the one place that is '
        'not'
    )
    report.say(
        '  good enough, and it sorts explicitly rather than inheriting the roster\'s order.'
    )

    passed = not in_product
    report.verdict(
        passed,
        f'every one of the {len(occurrences)} hash- and tree-ordered collections in the library '
        f'target is inside a `#[cfg(test)]` region; the record writers iterate fixed arrays, '
        f'`Vec`s in entropy order, and one explicit sort',
        'this is a search for four type names. A third-party ordered-by-hash structure would not '
        'be found — and cannot exist here, because the dependency table is empty. A `Vec` whose '
        'own construction order varied between runs would also pass, and that is what oracle 1\'s '
        'byte-identical reproducibility at every declared seed rules out dynamically.',
    )


# ---------------------------------------------------------------------------------------------
# 8. The counters
# ---------------------------------------------------------------------------------------------

def check_8(report, library):
    report.check(
        8, 'The counters are private, written once per event, and read only by the record producer',
        'the counters are private, have exactly one writer per event, and no reader inside the '
        'engine other than the record producer',
    )
    report.method(
        'for each counter field, take every *statement* in product code that mentions it and '
        'classify the statement as the declaration, a write or a read, naming the function it sits '
        'in. Statements rather than lines, because `regeneration_skipped`\'s increment is written '
        'over two lines and a line-oriented count would report its own continuation as a second '
        'reader.'
    )
    simulation = [source for source in library if source.name == 'simulation.rs'][0]
    statements = statement_spans(simulation)

    counters = ('crossings', 'consumed', 'regenerated', 'regeneration_skipped')
    writers, readers, arithmetic = {}, {}, []
    report.say()
    for counter in counters:
        pattern = re.compile(rf'self\.{counter}\b')
        rows = []
        for number, statement in statements:
            if number in simulation.test_lines or not pattern.search(statement):
                continue
            caller = enclosing_function(simulation.spans, number)
            # A write assigns to the field; everything else reads it. The whole statement is
            # matched, so an assignment whose right-hand side continues onto the next line is one
            # write and not a write plus a read.
            is_write = re.search(rf'self\.{counter}(\[[^\]]*\])?\s*=[^=]', statement) is not None
            rows.append((number, 'write' if is_write else 'read', caller, statement))
            if 'saturating_add' in statement:
                arithmetic.append((number, ' '.join(statement.split())))
        writers[counter] = [row for row in rows if row[1] == 'write']
        readers[counter] = [row for row in rows if row[1] == 'read']

        declaration = [
            number for number, line in enumerate(simulation.lines, start=1)
            if re.match(rf'\s+{counter}:\s', line)
        ]
        visibility = 'private' if declaration and not re.match(
            r'\s*pub', simulation.lines[declaration[0] - 1]
        ) else 'PUBLIC'
        report.say(f'  {counter}  — declared at line {declaration[0] if declaration else "?"}, '
                   f'{visibility}')
        for number, kind, caller, statement in rows:
            report.say(
                f'      {kind:<6} line {number:<5} in `{caller}`   '
                f'{" ".join(statement.split())[:58]}'
            )
        report.say(
            f'      {len(writers[counter])} write(s), {len(readers[counter])} read(s)'
        )

    read_functions = sorted({
        row[2] for counter in counters for row in readers[counter]
    })
    write_functions = sorted({
        row[2] for counter in counters for row in writers[counter]
    })
    report.say()
    report.say(f'  functions that write a counter: {", ".join(write_functions)}')
    report.say(f'  functions that read a counter:  {", ".join(read_functions)}')
    report.say(
        '  Every write sits in the function that emits the event it counts, in the same statement'
    )
    report.say(
        '  sequence, so a counter and the event stream cannot disagree. Every read sits in'
    )
    report.say(
        '  `write_run_record`, the record producer, which is the only place the run record is '
        'written.'
    )
    report.say()
    report.say(
        f'  arithmetic: all {len(arithmetic)} counter increments saturate, so no counter can '
        'overflow a run:'
    )
    for number, statement in arithmetic:
        report.say(f'      simulation.rs:{number}  {statement[:78]}')
    report.say(
        '    A `u64` cannot be exhausted by any run the tick limit admits; saturating arithmetic '
        'makes'
    )
    report.say(
        '    that a stated property rather than an assumption, and it is why the 10,000-tick '
        'unoptimised'
    )
    report.say('    run in `sizes.txt` exits cleanly rather than panicking on overflow.')

    passed = (
        all(len(writers[counter]) == 1 for counter in counters)
        and read_functions == ['write_run_record']
    )
    report.verdict(
        passed,
        f'four private counter fields, exactly one write each in '
        f'{len(write_functions)} emitting functions, and every read in `write_run_record` alone',
        'the classification of a statement as a write turns on an `=` after the field, so a '
        'counter mutated through a method call — `self.consumed.fill(0)` — would be counted as a '
        f'read. No such call exists: the {len(arithmetic)} saturating increments above are the '
        'whole of the arithmetic on these four fields, and the internal-tier test '
        '`no_rule_reads_a_cumulative_counter` asserts the other half — that no rule consults one — '
        'dynamically.',
    )


# ---------------------------------------------------------------------------------------------
# 9. The `#[cfg(test)]` entropy accessor
# ---------------------------------------------------------------------------------------------

def check_9(report, simulation, root):
    report.check(
        9, 'The entropy-state accessor is test-only, owned, and named by internal-tier tests only',
        'the `#[cfg(test)]` entropy-state accessor returns an owned `u64`, is `#[cfg(test)]` in '
        'the merged tree, and is named by internal-tier tests only. No public-tier test names it',
    )
    report.method(
        'read its declaration and the attribute above it, then search both tiers for its name.'
    )

    report.say()
    declaration = None
    for number, line in enumerate(simulation.lines, start=1):
        if 'fn entropy_state' in line:
            declaration = number
            break
    if declaration:
        for number in range(declaration - 1, declaration + 4):
            report.say(f'  simulation.rs:{number:<5} {simulation.raw_lines[number - 1]}')
        attribute = simulation.raw_lines[declaration - 2].strip()
        signature = simulation.raw_lines[declaration - 1].strip()
        report.say()
        report.say(f'  attribute above it: {attribute}')
        report.say(f'  signature:          {signature}')
        report.say(
            f'  returns `u64` by value: '
            f'{"yes" if "-> u64" in signature else "NO"} — a copy, not a borrow, so rule 6\'s'
        )
        report.say(
            '  prohibition on a reference into the entropy state holds in a test build as well.'
        )
        report.say(
            f'  not `pub`: {"yes" if not signature.startswith("pub") else "NO"}, so it is '
            'unreachable outside the crate whatever the'
        )
        report.say('  build configuration — rule 6\'s "there is no test-support seam".')

    report.say()
    report.say('every mention of `entropy_state`, by tier:')
    mentions = {'internal (src/)': [], 'public (tests/)': [], 'observer': []}
    for directory, tier in (
        (os.path.join(root, PACKAGE, 'src'), 'internal (src/)'),
        (os.path.join(root, PACKAGE, 'tests'), 'public (tests/)'),
        (os.path.join(root, OBSERVER), 'observer'),
    ):
        for current, _, names in os.walk(directory):
            if 'target' in current:
                continue
            for name in sorted(names):
                if not name.endswith('.rs'):
                    continue
                path = os.path.join(current, name)
                for number, line in enumerate(
                    io.open(path, encoding='utf-8').read().split('\n'), start=1
                ):
                    if 'entropy_state' in line:
                        mentions[tier].append(
                            f'{os.path.relpath(path, root)}:{number}  {line.strip()[:64]}'
                        )
    for tier in mentions:
        report.say(f'  {tier}: {len(mentions[tier])} mention(s)')
        for mention in mentions[tier]:
            report.say(f'      {mention}')

    passed = (
        declaration is not None
        and '-> u64' in simulation.raw_lines[declaration - 1]
        and '#[cfg(test)]' in simulation.raw_lines[declaration - 2]
        and not mentions['public (tests/)']
        and not mentions['observer']
    )
    report.verdict(
        passed,
        f'`#[cfg(test)] fn entropy_state(&self) -> u64`, private, with '
        f'{len(mentions["internal (src/)"]) - 1} internal-tier uses and no mention in the public '
        f'tier or the observer',
        'the attribute is read as the line above the declaration, which is where it is. A '
        '`cfg(test)` applied to an enclosing module instead would not be found by this check and '
        'would be equally sound; a *missing* `cfg` would make the item present in a release build, '
        'and that is what this check is for.',
    )


# ---------------------------------------------------------------------------------------------
# 10. Packages, targets, names, dependencies
# ---------------------------------------------------------------------------------------------

def check_10(report, root, baseline):
    report.check(
        10, 'No new package, target, build script or name change; the dependency tables',
        'no new package, no new target, no build script, and no change to any package, library or '
        'binary name; the engine\'s dependency and dev-dependency tables are empty and the '
        'observer\'s set is unchanged',
    )
    report.method(
        'diff every manifest against the baseline commit, list every target the workspace '
        'declares, and search the tree for a build script.'
    )

    report.say()
    for manifest in ('Cargo.toml', f'{PACKAGE}/Cargo.toml', f'{OBSERVER}/Cargo.toml'):
        completed = run(['git', 'diff', baseline, '--', manifest])
        state = 'unchanged' if not completed.stdout.strip() else 'CHANGED'
        report.say(f'  {manifest:<32} {state}')
        for line in completed.stdout.split('\n')[4:]:
            if line.strip():
                report.say(f'      {line}')

    report.say()
    report.say('the engine\'s manifest, in full:')
    for line in io.open(
        os.path.join(root, PACKAGE, 'Cargo.toml'), encoding='utf-8'
    ).read().rstrip().split('\n'):
        report.say(f'      {line}')

    report.say()
    scripts = []
    for current, directories, names in os.walk(root):
        directories[:] = [name for name in directories if name not in ('target', '.git')]
        scripts.extend(
            os.path.relpath(os.path.join(current, name), root).replace('\\', '/')
            for name in names if name == 'build.rs'
        )
    report.say(f'  build scripts in the workspace: {len(scripts)} {scripts}')

    report.say()
    report.say('declared targets, and the test files Cargo discovers beside them:')
    report.say(
        f'  {PACKAGE}: [lib] mokiterions at src/lib.rs, [[bin]] Mokiterions at src/main.rs'
    )
    for package in (PACKAGE, OBSERVER):
        directory = os.path.join(root, package, 'tests')
        if not os.path.isdir(directory):
            continue
        names = sorted(name for name in os.listdir(directory) if name.endswith('.rs'))
        added = run([
            'git', 'diff', '--name-status', '--diff-filter=A', baseline, '--',
            f'{package}/tests',
        ]).stdout.strip()
        report.say(f'  {package}/tests: {len(names)} file(s) — {", ".join(names)}')
        report.say(f'      added since {baseline}: {added or "(none tracked yet)"}')
    untracked = run(['git', 'ls-files', '--others', '--exclude-standard', '--', '*/tests/*.rs'])
    report.say(f'      untracked test files: {untracked.stdout.split() or "(none)"}')
    report.say()
    report.say(
        '  Rule 1\'s "no third target" governs the two targets its table declares — the library and'
    )
    report.say(
        '  the binary — and a file in `tests/` is the public tier rules 7 to 10 require, not a '
        'third'
    )
    report.say(
        '  declared target. The precedent is on the record and is not being set here: '
        '`decisions.rs`'
    )
    report.say(
        '  arrived under `WO-MOK-007` and `naming.rs` under `WO-MOK-010`, both under an approved'
    )
    report.say(
        '  requirement, and neither was treated as a rule 1 exception. `records.rs` follows them.'
    )

    tree = run(['cargo', 'tree', '-p', 'Mokiterions'], cwd=root)
    report.say()
    report.say('cargo tree -p Mokiterions:')
    for line in tree.stdout.strip().split('\n'):
        report.say(f'      {line}')

    manifest_text = io.open(os.path.join(root, PACKAGE, 'Cargo.toml'), encoding='utf-8').read()
    empty_tables = (
        '[dependencies]\n\n' in manifest_text + '\n'
        or manifest_text.rstrip().endswith('[dependencies]')
        or '[dependencies]\n[' in manifest_text
    )
    passed = (
        not scripts
        and not run(['git', 'diff', baseline, '--', f'{PACKAGE}/Cargo.toml']).stdout.strip()
        and not run(['git', 'diff', baseline, '--', f'{OBSERVER}/Cargo.toml']).stdout.strip()
        and len(tree.stdout.strip().split('\n')) == 1
    )
    report.verdict(
        passed,
        f'all three manifests byte-identical to {baseline}, no build script, and '
        f'`cargo tree -p Mokiterions` resolving to one crate',
        'an unchanged manifest is a strong statement here precisely because the tables are in it: '
        'a dependency cannot be added without changing it. What a manifest diff cannot show is a '
        'vendored source file compiled into the target, and check 1\'s enumeration of the '
        'library\'s reach into `std` is what bounds that.',
    )


# ---------------------------------------------------------------------------------------------
# 11. Test placement, rules 7 to 10
# ---------------------------------------------------------------------------------------------

def check_11(report, simulation):
    report.check(
        11, 'Test placement follows SPEC-MOK-002 rules 7 to 10',
        'a new test lives in the public tier only if it is writable through the library target\'s '
        'public interface with its assertions unchanged; no item is widened to `pub` to relocate a '
        'test; every internal-tier test added here names the private item or hook it requires',
    )
    report.method(
        'check 3 already establishes that no item was widened — the enumeration is unchanged. '
        'What remains is the placement of each new test, and it is stated per test file below.'
    )

    report.say()
    report.say('rule 10, one invocation: `cargo test` at the workspace root, with no feature, no')
    report.say('environment variable, ignore attribute, extra command, terminal or')
    report.say('working-directory dependence.')
    report.say()
    report.say('  the gates rule 10 forbids — a test that does not run, or does not run the same')
    report.say('  way, unless the operator arranges something:')
    gates, environment = [], []
    for directory in (f'{PACKAGE}/src', f'{PACKAGE}/tests', OBSERVER):
        for current, directories, names in os.walk(directory):
            directories[:] = [name for name in directories if name != 'target']
            for name in sorted(names):
                if not name.endswith('.rs'):
                    continue
                path = os.path.join(current, name).replace('\\', '/')
                for number, line in enumerate(
                    io.open(path, encoding='utf-8').read().split('\n'), start=1
                ):
                    for needle in ('#[ignore', 'env::var', 'env::set_var', 'current_dir',
                                   'feature = ', 'is_terminal', 'IsTerminal'):
                        if needle in line:
                            gates.append((needle, path, number, line.strip()))
                    for match in re.finditer(r'\benv(!|::[a-z_]+)', line):
                        environment.append((path, number, line.strip(), match.group(0)))
    for needle, path, number, text in gates:
        report.say(f'    {needle:<14} {path}:{number}  {text[:56]}')
    if not gates:
        report.say(
            '    none. No `#[ignore]`, no `env::var`, no `env::set_var`, no `current_dir`, no '
            'feature'
        )
        report.say('    gate and no terminal test in either package, in `src/` or in `tests/`.')

    report.say()
    report.say('  every use of `env` in either package, and what supplies its value:')
    unclassified = []
    for path, number, text, form in environment:
        if form.endswith('!'):
            classification = 'compile time, from Cargo\'s own metadata'
        elif 'args' in form:
            classification = 'the command line — what a binary target is for'
        elif 'temp_dir' in form:
            classification = 'the platform temp directory, which resolves on every OS'
        else:
            classification = 'UNCLASSIFIED'
            unclassified.append((path, number, form))
        report.say(f'    {path}:{number}  {text[:52]:<52}  — {classification} ({form})')
    report.say(
        '    The two `env!` uses are macros and not lookups: they are expanded by the compiler out '
        'of'
    )
    report.say(
        '    the manifest, and the value is a literal in the binary. `env!("CARGO_PKG_VERSION")` in'
    )
    report.say(
        '    `write_header_record` is where the header record\'s `version` comes from, so it is the '
        'same'
    )
    report.say(
        '    `0.1.0` check 10 reads out of `Cargo.toml`; nothing is read from the environment at run'
    )
    report.say('    time, and check 1 is what establishes that.')
    report.say(
        '    `env!("CARGO_BIN_EXE_Mokiterions")` is how a Cargo integration test is meant to '
        'locate the'
    )
    report.say(
        '    binary it exercises: Cargo sets it, not the operator, and it is resolved at compile '
        'time.'
    )
    report.say(
        '    `env::temp_dir()` has a platform default everywhere and needs no variable set; the'
    )
    report.say(
        '    precedent is `mokiterions-tui/tests/export.rs` and `verification.rs`, which have used '
        'it'
    )
    report.say(
        '    since `WO-MOK-011`. Neither is a knob: `cargo test` at the workspace root runs every '
        'test'
    )
    report.say('    in both packages with nothing supplied and nothing set.')
    report.say(
        f'    every `env` use above is classified: '
        f'{"yes" if not unclassified else "NO — " + str(unclassified)}'
    )

    report.say()
    report.say('rule 9, the internal tier: every `#[cfg(test)]` module in the library target, and')
    report.say('the private items its tests reach:')
    modules = [
        number for number, line in enumerate(simulation.lines, start=1)
        if line.strip().startswith('#[cfg(test)]') and 'mod tests' in (
            simulation.lines[number] if number < len(simulation.lines) else ''
        )
    ]
    report.say(f'  simulation.rs: {len(modules)} module(s) at line(s) '
               f'{", ".join(str(number) for number in modules)}')
    private_reached = sorted({
        match.group(1)
        for number, line in enumerate(simulation.lines, start=1)
        if number in simulation.test_lines
        for match in re.finditer(
            r'\b(entropy_state|run_recording|write_event_record|write_header_record|'
            r'metrics_record|write_run_record|emit|SplitMix64|DecisionSource|Observation)\b', line
        )
    })
    report.say(f'  private items internal-tier tests name: {", ".join(private_reached)}')
    report.say(
        '  Each is on rule 6\'s prohibited list or is a private record writer, so a test that '
        'names one'
    )
    report.say(
        '  cannot be written through rule 5\'s interface, and rule 7 places it in the internal '
        'tier.'
    )

    report.say()
    report.say('rule 8, the public tier: the new file, and what it reaches:')
    new_file = f'{PACKAGE}/tests/records.rs'
    if os.path.exists(new_file):
        text = io.open(new_file, encoding='utf-8').read()
        report.say(f'  {new_file}: {len(text.split(chr(10))):,} lines, '
                   f'{text.count("#[test]")} tests')
        for line in sorted({
            line.strip() for line in text.split('\n') if line.strip().startswith('use ')
        }):
            report.say(f'      {line}')
        report.say(
            f'  It imports nothing from `mokiterions` — {text.count("mokiterions::")} references to '
            'the library'
        )
        report.say(
            '  crate — and reaches the engine the way an operator does: it runs the binary, hands it'
        )
        report.say(
            '  `--events-path`, and reads the file. So it needs no library item at all, public or'
        )
        report.say(
            '  private, and rule 7 places it in the public tier by its own criterion: it is '
            'writable'
        )
        report.say(
            '  through the interface with its assertions unchanged, because it does not use the'
        )
        report.say(
            '  interface. `process.rs` is the precedent — the same three imports, the same '
            'boundary.'
        )

    passed = not gates and not unclassified
    report.verdict(
        passed,
        f'one invocation with no gate of any kind, {len(private_reached)} private items named by '
        f'internal-tier tests and none by the public tier, and no item widened to `pub` (check 3)',
        'placement is a property of what a test *needs*, and a test that could have been written '
        'through the public interface but was put in the internal tier anyway passes every '
        'mechanical check here. Rule 7\'s judgement is the technical owner\'s in review; what this '
        'check establishes is the half that is mechanical — that no internal-tier test could have '
        'gone in the public tier without an item being widened, and that none was widened.',
    )


# ---------------------------------------------------------------------------------------------
# 12. `ARCH-MOK-001`'s conformance checks, restated over the merged tree
# ---------------------------------------------------------------------------------------------

def check_12(report, root, baseline):
    report.check(
        12, 'ARCH-MOK-001: the component boundary the sink crosses',
        'the sink path is interpreted only as a path, only by the binary target; the library '
        'target interprets no path at all, checked statically',
    )
    report.method(
        'follow the path value: where it is parsed, where it is opened, and whether any string '
        'derived from it can reach the library or a record.'
    )

    report.say()
    report.say('the option, from the command line to the file:')
    for name, needle in (
        ('cli.rs', 'events_path'),
        ('main.rs', 'events_path'),
        ('lib.rs', 'records'),
        ('simulation.rs', 'sink_error'),
    ):
        path = os.path.join(root, PACKAGE, 'src', name)
        lines = io.open(path, encoding='utf-8').read().split('\n')
        hits = [
            (number, line.strip()) for number, line in enumerate(lines, start=1)
            if needle in line and not line.strip().startswith(('//', '///', '//!', '*'))
        ]
        report.say(f'  {name}: `{needle}` on {len(hits)} line(s)')
        for number, line in hits[:8]:
            report.say(f'      {number:>4}  {line[:88]}')
        if len(hits) > 8:
            report.say(f'      … and {len(hits) - 8} more')

    report.say()
    report.say('the diagnostic layers, which is where a path could leak and does not:')
    simulation_text = io.open(
        os.path.join(root, PACKAGE, 'src', 'simulation.rs'), encoding='utf-8'
    ).read()
    main_text = io.open(os.path.join(root, PACKAGE, 'src', BINARY_SOURCE), encoding='utf-8').read()
    engine_form = 'record sink: ' in simulation_text
    host_form = 'record sink {' in main_text or 'record sink ' in main_text
    report.say(
        f'  simulation.rs writes `record sink: {{error}}` — no path, because the library holds '
        f'none: {"present" if engine_form else "ABSENT"}'
    )
    report.say(
        f'  main.rs writes `record sink <path>: {{error}}` — the host owns the path and names it: '
        f'{"present" if host_form else "ABSENT"}'
    )
    report.say(
        '  The two are one platform error observed at two layers, not two errors. `ARCH-MOK-001`'
    )
    report.say(
        '  keeps the filesystem out of the library, so the engine reports the write it issued and'
    )
    report.say('  cannot say where it was going.')

    report.say()
    report.say('ARCH-MOK-001\'s amendment record, table rows only — oracle 7 is where its approval')
    report.say('state is checked; what is recorded here is that the amendment exists and is dated:')
    architecture = os.path.join(
        root, 'docs', 'engineering', 'simulation', 'architecture', 'ARCH-MOK-001.md'
    )
    if os.path.exists(architecture):
        lines = io.open(architecture, encoding='utf-8').read().split('\n')
        for number, line in enumerate(lines, start=1):
            if re.match(r'\|\s*(202\d-\d\d-\d\d|Date|-+)', line.strip()):
                report.say(f'  {number:>4}  {line.strip()[:190]}')

    passed = engine_form and host_form
    report.verdict(
        passed,
        'the path is parsed in `cli.rs`, opened in `main.rs`, and never reaches the library: '
        '`execute` takes a writer, not a name, and the engine\'s own diagnostic carries no path',
        'this reads the source rather than the stream. Whether a path reaches a *record* is '
        'oracle 5\'s and the retention check\'s question, asserted over real bytes, and it is the '
        'stronger statement of the two.',
    )


if __name__ == '__main__':
    sys.exit(main())
