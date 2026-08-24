"""`VER-MOK-018`'s architecture, usage and security checks that no test and no other file runs.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-025/analysis/architecture-checks.py \
        <base-commit> <output-file>

## Why this exists

`WO-MOK-025`'s *Required verification* names twelve static checks and five security checks. Most of
them have an owner already: **S4** and **S4a** are `candidate/public-surface.txt`, **S5a**, **S6**
and **S6a** are tests that run on every `cargo test`, and **L3** and **L11**'s first clause are
`analysis/static-checks.py`. A label-by-label search of this packet found **seven with no runner at
all** — `S1`, `S2a`, `S3`, `S3a`, `S5`, `S6b`, `S7` — and three security checks whose measurable
half nothing captures: `C1`, `C3` and `C5`. This file is those ten, so that item 7 of the completion
report can state a result for each rather than leaving a row blank.

Each check prints its obligation quoted from `VER-MOK-018`, the method, the hits with line numbers,
and a verdict with a stated limit. The format is `analysis/static-checks.py`'s, because a reader
comparing the two files should not have to learn a second one.

## Nothing here is a new convention

`Source`, `Report` and the comment-and-literal blanking are `WO-MOK-019`'s, **imported and called**.
The reason is the reason its sibling gives: a copy can drift, and the day it drifts the reader who
compares two capture files is comparing two different instruments. `MODE_VOCABULARY` is imported
from `analysis/static-checks.py` for the same reason — check 6 below re-runs that file's check 2
over the observer, and re-running it means the same word list.

## What a scan over blanked source buys, and what it costs

Every scan runs over source whose comments **and string literals** are blanked, so a doc comment
that quotes a prohibited word is not counted as the word. That matters more here than elsewhere:
`mokiterions-core/src/cli.rs` quotes the withdrawn sentence check 5 looks for, in a doc comment
explaining that it was withdrawn, and the observer's usage text names the connector check 6 looks
for, in the sentence that tells an operator this host has none. Both are reported as raw hits with
their reason, because a reader who greps the tree and finds them deserves the explanation rather
than a contradiction.

The cost is that a literal cannot be scanned this way, so check 2's search for a compiled-in
connector path reads the **raw** literals and says so.
"""

import importlib.util
import io
import os
import re
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8')

PACKAGE = 'mokiterions-core'
OBSERVER = 'mokiterions-tui'
LIBRARY_SOURCES = ('lib.rs', 'cli.rs', 'simulation.rs')

EVIDENCE = os.path.join('docs', 'engineering', 'simulation', 'evidence')
WO_019 = os.path.join(EVIDENCE, 'WO-MOK-019', 'analysis', 'static-checks.py')
SIBLING = os.path.join(EVIDENCE, 'WO-MOK-025', 'analysis', 'static-checks.py')
DEPENDENCIES = os.path.join(EVIDENCE, 'WO-MOK-025', 'candidate', 'declared-dependencies.txt')

# The manifests and the lockfile. S1's "unchanged from the base commit" is a byte comparison over
# exactly these files: a declared set lives in a manifest, and what a declaration resolves to is
# fixed by the lockfile, so if none of them moved then no resolved graph moved either.
DECLARATION_FILES = (
    'Cargo.toml',
    'Cargo.lock',
    'mokiterions-core/Cargo.toml',
    'mokiterions-tui/Cargo.toml',
    'rust-toolchain.toml',
)

# `SPEC-MOK-002` rule 1 as amended: exactly two packages, no third member.
DECLARED_MEMBERS = ('mokiterions-core', 'mokiterions-tui')

# The four capability classes S3 names, as the identifiers each would have to be spelled in. Word
# boundaries throughout, because `execute` contains `exec` and `transcript_path` contains `path`,
# and a substring reader would report both as capabilities.
CAPABILITIES = {
    'filesystem': (
        r'\bstd::fs\b', r'\bfs::', r'\bFile\b', r'\bOpenOptions\b', r'\bPathBuf\b', r'\bPath\b',
        r'\bread_to_string\b', r'\bcreate_dir\b', r'\bcreate_dir_all\b', r'\bremove_file\b',
        r'\bremove_dir_all\b', r'\bcanonicalize\b', r'\bread_dir\b', r'\bDirEntry\b',
        r'\bhard_link\b', r'\bsymlink', r'\bcurrent_dir\b', r'\btemp_dir\b',
    ),
    'socket': (
        r'\bstd::net\b', r'\bTcpStream\b', r'\bTcpListener\b', r'\bUdpSocket\b', r'\bSocketAddr\b',
        r'\bIpAddr\b', r'\bToSocketAddrs\b', r'\bconnect\b',
    ),
    'process': (
        r'\bstd::process\b', r'\bCommand\b', r'\bChild\b', r'\bStdio\b', r'\bspawn\b',
        r'\bexecvp?\b', r'\bfork\b',
    ),
    'environment': (
        r'\bstd::env\b', r'\benv::', r'\bvar_os\b', r'\bvars_os\b',
    ),
}

# Two identifiers in the process class are not processes, and both have to be dismissed by a rule
# the report prints rather than by a filter nobody sees.
#
#   `mokiterions::cli::Command` is this crate's own two-variant enum for what the arguments asked
#   for. It collides by name with `std::process::Command`.
#   `std::process::ExitCode` is the value `main` returns. It names the `process` module and starts
#   no process; a program that could not name it could not report a status.
#
# Anything else in the class is a finding.
DISMISSALS = (
    (r'\benum Command\b|\bcli::Command\b|\bcli::\{[^}]*\bCommand\b|\bCommand::Help\b'
     r'|\bCommand::Run\b|\bResult<Command\b|\bCommand,',
     "cli::Command, this crate's own enum"),
    (r'\bExitCode\b',
     "std::process::ExitCode, the value `main` returns; it starts nothing"),
)

# `env::args` is the argument vector, which every command-line program reads and which
# `SPEC-MOK-002` rule 13.1 has always admitted. `env::var` is the environment, which is where a
# credential would come from. S3a speaks of "the environment pass-through", and the two must be
# told apart or the observer's argument parsing reads as a violation.
ARGUMENT_FORMS = ('env::args',)
ENVIRONMENT_READS = (r'\benv::var\b', r'\benv::var_os\b', r'\benv::vars\b', r'\benv::vars_os\b')

# The sentence `SPEC-MOK-007` rule 18.3 withdraws, and the fifth policy value rule 18.1 adds.
WITHDRAWN_SENTENCE = 'None of the four learns anything'
FIFTH_POLICY_VALUE = 'llm'
POLICY_OPTION_LINE = '--policy <baseline|reference|individual|social|llm>'

# What a spend ceiling would have to be named. The plain word `cost` is deliberately **not** here:
# the observer's layout arithmetic calls a row's height a cost — `used += cost;` in `render.rs` —
# and a scan for it would report a pane-height sum as a spend ceiling. Naming the exclusion is part
# of the check.
CEILING_VOCABULARY = (
    r'\bceiling\b', r'\bspend\b', r'\bbudget\b', r'\bRunAccount\b', r'\bExchangeUsage\b',
    r'\bcost_of\b', r'\bdeclared_prices\b', r'\bprice\b', r'\bprices\b', r'\bmicro_cost\b',
    r'\bcost_ceiling\b', r'\btokens_in\b', r'\btokens_out\b',
)

# What a credential would have to be named, and what reading one would have to call.
CREDENTIAL_VOCABULARY = (
    r'\bcredential\b', r'(?i)\bapi_key\b', r'(?i)\bapikey\b', r'(?i)\bbearer\b',
    r'(?i)\bauthorization\b', r'(?i)\bsecret\b', r'\btoken\b', r'(?i)\bpassword\b',
    r'(?i)\bkeychain\b', r'\bDPAPI\b', r'\bdotenv\b', r'\bconfig_dir\b', r'\bhome_dir\b',
)

# What the connector would have to be named or reached by.
CONNECTOR_VOCABULARY = (r'(?i)\bconnector\b', r'\bCommand::new\b', r'\bspawn\b', r'\bStdio\b')

# What an outbound call would have to be spelled in, beyond the socket types above.
OUTBOUND_VOCABULARY = (
    r'\bhttps?://', r'(?i)\bhttp\b', r'\breqwest\b', r'\bhyper\b', r'\bureq\b', r'\bcurl\b',
    r'\bUrl\b', r'\bUri\b', r'\bpost\b', r'\bendpoint\b',
)

# Shapes a compiled-in connector path would take. Read over raw string literals, because the
# blanking that makes every other scan honest removes exactly what this one is looking for.
PATH_SHAPES = (
    r'\.exe\b', r'\.bat\b', r'\.cmd\b', r'\.sh\b', r'\.py\b', r'\.ps1\b',
    r'/usr/', r'/bin/', r'/opt/', r'[A-Za-z]:\\\\', r'\.\./', r'~/',
)

STRING_LITERAL = re.compile(r'"(?:[^"\\]|\\.)*"')
COMMENT_LINE = re.compile(r'^\s*(//|/\*|\*)')


def load(path, name):
    """A module beside this one, imported so its code runs rather than being reimplemented."""
    specification = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


def run(command, **keywords):
    return subprocess.run(command, capture_output=True, text=True, **keywords)


def scan(sources, patterns, product_only=True):
    """(file, line number, raw line, pattern) for every match in blanked source."""
    found = []
    for source in sources:
        for number, line in enumerate(source.lines, start=1):
            if product_only and number in source.test_lines:
                continue
            for pattern in patterns:
                if re.search(pattern, line):
                    found.append((source.name, number, source.raw_lines[number - 1].strip(),
                                  pattern))
    return found


def dismissal(line):
    """Why a process-class hit is not a process, or None if it is one."""
    for pattern, reason in DISMISSALS:
        if re.search(pattern, line):
            return reason
    return None


def raw_scan(sources, needle):
    """(file, line number, raw line) for every occurrence in raw source, comments included."""
    found = []
    for source in sources:
        for number, line in enumerate(source.raw_lines, start=1):
            if needle in line:
                found.append((source.name, number, line.strip()))
    return found


def literals(source):
    """(line number, the literal with its quotes) for every string literal outside a comment."""
    found = []
    for number, line in enumerate(source.raw_lines, start=1):
        if COMMENT_LINE.match(line):
            continue
        for match in STRING_LITERAL.finditer(line):
            found.append((number, match.group(0)))
    return found


def unescape(text):
    """The four escapes these usage texts use. Not a general Rust literal reader, and not needed
    as one: `cargo test` holds both texts byte-for-byte, so this only has to be faithful enough to
    search."""
    return (
        text.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace('\\\\', '\\')
    )


def usage_text(source, name='USAGE'):
    """The assembled text of a `const <name>: &str = concat!(...)` declaration."""
    marker = f'const {name}: &str = concat!('
    start = source.raw.index(marker)
    end = source.raw.index('\n);', start)
    region = source.raw[start:end]
    return ''.join(unescape(match.group(0)[1:-1]) for match in STRING_LITERAL.finditer(region))


def concat_literals(source, name):
    """Every string literal of a `concat!` declaration, unassembled, for a copy search."""
    marker = f'const {name}: &str = concat!('
    start = source.raw.index(marker)
    end = source.raw.index('\n);', start)
    region = source.raw[start:end]
    return [match.group(0)[1:-1] for match in STRING_LITERAL.finditer(region)]


def report_hits(report, hits, label='hits'):
    if not hits:
        report.say(f'{label}: none')
        return
    report.say(f'{label}: {len(hits)}')
    for hit in hits:
        if len(hit) == 4:
            name, number, line, pattern = hit
            report.say(f'  {name}:{number}  [{pattern}]  {line[:96]}')
        else:
            name, number, line = hit
            report.say(f'  {name}:{number}  {line[:96]}')


# ---------------------------------------------------------------------------------------------
# 1. S1 — the declared sets did not move, and the resolved comparison is captured beside this
# ---------------------------------------------------------------------------------------------

def check_1(report, root, base):
    report.check(1, 'S1 — the declared dependency sets, unchanged from the base commit', (
        'The engine\'s and the observer\'s resolved dependency graphs equal the declared sets, '
        'unchanged from the base commit - the engine\'s table empty, the observer\'s one entry '
        'with every other crate reached transitively through `ratatui`. ARCH-MOK-001\'s by-name '
        'scan for a network, asynchronous-runtime, database, model-provider or user-interface '
        'crate is re-run and continues to find none.'
    ))
    report.method(
        'two halves. The "unchanged from the base commit" half is a byte comparison over the '
        'manifests and the lockfile, here. The resolved-graph half needs cargo and three targets, '
        'so it is the repository\'s own gate, captured beside this file.'
    )
    report.say()
    report.say('the declaration, base against candidate:')
    moved = []
    for path in DECLARATION_FILES:
        difference = run(['git', 'diff', '--stat', base, 'HEAD', '--', path], cwd=root).stdout
        state = 'unchanged' if not difference.strip() else difference.strip()
        report.say(f'  {path:32} {state}')
        if difference.strip():
            moved.append(path)

    report.say()
    report.say('the resolved graphs, from scripts/check_declared_dependencies.py:')
    quoted, present = [], os.path.exists(os.path.join(root, DEPENDENCIES))
    if present:
        text = io.open(os.path.join(root, DEPENDENCIES), encoding='utf-8').read().split('\n')
        for line in text:
            if line.startswith('#'):
                continue
            if any(word in line for word in ('declared', 'raw hit', 'disclosed', '8.4a')):
                quoted.append(line.rstrip())
        for line in quoted:
            report.say(f'  {line}')
    else:
        report.say(f'  {DEPENDENCIES} is not present; that half is not read here')

    passed = not moved and present and any('8.4a-8.4d pass' in line for line in quoted)
    report.verdict(
        passed,
        f'no manifest and no lockfile entry moved between {base} and the candidate'
        + (', and the captured gate reports every declared set equal to its resolved graph'
           if present else ', and the resolved half is not read here'),
        'a byte comparison establishes that the declaration did not move, not that it was right. '
        'The by-name scan\'s two accepted hits, mio and signal-hook-mio, are the observer\'s and '
        'predate this work order; VER-MOK-014 manual assessment 6 is where they were judged.',
    )


# ---------------------------------------------------------------------------------------------
# 2. S2a — no third member, no third package, no connector, no compiled-in path
# ---------------------------------------------------------------------------------------------

def check_2(report, root, product):
    report.check(2, 'S2a — nothing makes a connector reachable without an operator naming it', (
        'No third workspace member, no third package directory, no connector source outside the '
        'canned one, and no connector path compiled into either package as a default. A default '
        'path would make a live run reachable without the operator naming anything, which is '
        'REQ-MOK-072\'s gate defeated by a constant.'
    ))
    report.method(
        'the workspace manifest\'s member list; every tracked Cargo.toml; every [[bin]] target; '
        'and a search of every raw string literal of both packages\' product source for the '
        'shapes a path takes. Literals are read raw, because the blanking every other scan here '
        'depends on removes exactly what this one looks for.'
    )

    manifest = io.open(os.path.join(root, 'Cargo.toml'), encoding='utf-8').read()
    start = manifest.index('members')
    members = re.findall(r'"([^"]+)"', manifest[start:manifest.index(']', start)])
    report.say()
    report.say(f'[workspace] members: {members}')

    manifests = sorted(run(['git', 'ls-files', '*Cargo.toml'], cwd=root).stdout.split())
    report.say(f'tracked Cargo.toml files: {len(manifests)}')
    for path in manifests:
        report.say(f'  {path}')

    binaries = []
    for member in members:
        text = io.open(os.path.join(root, member, 'Cargo.toml'), encoding='utf-8').read()
        for block in re.findall(r'\[\[bin\]\]([^\[]*)', text):
            name = re.search(r'name\s*=\s*"([^"]+)"', block)
            path = re.search(r'path\s*=\s*"([^"]+)"', block)
            binaries.append((member, name.group(1) if name else '?',
                             path.group(1) if path else '?'))
    report.say(f'[[bin]] targets: {len(binaries)}')
    for member, name, path in binaries:
        report.say(f'  {member}: {name} at {path}')

    report.say()
    report.say('raw string literals searched for a path shape:')
    counted, shaped = 0, []
    for source in product:
        for number, literal in literals(source):
            counted += 1
            for shape in PATH_SHAPES:
                if re.search(shape, literal):
                    shaped.append((source.name, number, literal[:88], shape))
    report.say(f'  {counted} literal(s) across {len(product)} file(s)')
    report_hits(report, shaped, 'path-shaped literals')

    report.say()
    report.say('the connector, named in blanked product source:')
    connector = scan(product, (r'(?i)\bconnector\b',))
    report_hits(report, connector, 'connector hits')
    raw_connector = raw_scan(product, 'connector')
    report.say(f'the same word in raw source, comments and literals included: {len(raw_connector)}')
    for name, number, line in raw_connector[:6]:
        report.say(f'  {name}:{number}  {line[:96]}')
    report.say(
        '  these are the usage text and the comments that explain it. The sentence an operator '
        'reads is "the model is reached through a separate connector program you supply, never '
        'by this program itself", which is the absence this check measures, written down.'
    )

    problems = []
    if sorted(members) != sorted(DECLARED_MEMBERS):
        problems.append(f'members are {members}')
    if len(manifests) != 1 + len(DECLARED_MEMBERS):
        problems.append(f'{len(manifests)} manifests')
    if len(binaries) != 2:
        problems.append(f'{len(binaries)} binary targets')
    if shaped:
        problems.append(f'{len(shaped)} path-shaped literal(s)')
    if connector:
        problems.append(f'{len(connector)} connector identifier(s)')

    report.verdict(
        not problems,
        'two members, three manifests, two binary targets each in its own package, no '
        f'path-shaped literal among {counted} and no connector identifier in either package'
        if not problems else '; '.join(problems),
        'a bare program name in a literal - "connector", say - is not distinguishable from prose '
        'by shape, so this check does not rest on the literal search alone. It rests on check 4: '
        'there is no process-spawning site in either package, so a default path would have '
        'nothing to be passed to.',
    )
    return counted


# ---------------------------------------------------------------------------------------------
# 3. S3 — the library target's four capability classes
# ---------------------------------------------------------------------------------------------

def check_3(report, library):
    report.check(3, 'S3 — the library target performs none of the four capabilities', (
        'The library target performs no filesystem operation, opens no socket, spawns no process '
        'and reads no environment variable, extending ARCH-MOK-001\'s 2026-08-20 prohibition to '
        'the three new capability classes. Both of this source\'s streams are covered: the '
        'transcript it writes in live mode and the transcript it reads in replay both arrive as '
        'already-open handles, per SPEC-MOK-007 rules 11.1 and 12.1.1.'
    ))
    report.method(
        'the four classes as identifier patterns with word boundaries, over blanked product '
        'source of lib.rs, cli.rs and simulation.rs. Word boundaries because `execute` contains '
        '`exec` and `transcript_path` contains `path`.'
    )
    total, dismissed = 0, []
    for capability, patterns in CAPABILITIES.items():
        hits = scan(library, patterns)
        report.say()
        report.say(f'{capability}:')
        for name, number, line, pattern in hits:
            reason = dismissal(line)
            report.say(
                f'  {name}:{number}  [{pattern}]  {line[:88]}'
                + (f'   <- {reason}' if reason else '')
            )
            if reason:
                dismissed.append((name, number))
            else:
                total += 1
        if not hits:
            report.say('  none')
    report.say()
    report.say(
        f'{len(dismissed)} hit(s) dismissed, each by a rule printed beside it. The two rules are '
        'the only ones this file has, they are declared in `DISMISSALS` at the top of it, and '
        'nothing else was dismissed.'
    )
    report.verdict(
        total == 0,
        f'{total} capability site(s) in the library target'
        if total else 'no filesystem operation, no socket, no spawn and no environment read in '
                      'lib.rs, cli.rs or simulation.rs',
        'this reads the library\'s own source. A capability reached through a dependency would '
        'not be found here - and cannot be, because SPEC-MOK-002 rule 13 declares the engine\'s '
        'external set empty and check 1 measured that it is still empty. The two transcript '
        'streams arriving as open handles is visible in the signatures, not in this scan: '
        '`ReplayPort::new` takes a `BufRead` and the record sink takes a `Write`.',
    )


# ---------------------------------------------------------------------------------------------
# 4. S3a — the spawn and the environment, located
# ---------------------------------------------------------------------------------------------

def check_4(report, tiers, library, engine_binary, observer):
    report.check(4, 'S3a — no process spawn in either package, and where the environment is read', (
        'The process spawn and the environment pass-through appear in the engine\'s binary target '
        'and nowhere else in either package. The observer\'s source contains no process spawn at '
        'all, which is REQ-MOK-077\'s prohibition checked statically rather than assumed from its '
        'absence today.'
    ))
    report.method(
        'the process class over every source file of both packages, product code and test code '
        'separately; then the environment, with `env::args` told apart from `env::var`. The '
        'distinction is not cosmetic: both binaries read the argument vector, and a check that '
        'counted that as "the environment" would report the observer as violating this.'
    )
    spawns = {}
    for label, sources in tiers:
        product = scan(sources, CAPABILITIES['process'])
        everything = scan(sources, CAPABILITIES['process'], product_only=False)
        real = [hit for hit in product if not dismissal(hit[2])]
        all_real = [hit for hit in everything if not dismissal(hit[2])]
        spawns[label] = (real, all_real)
        report.say()
        report.say(
            f'{label}: process-class hits, product {len(product)}, all tiers {len(everything)}; '
            f'after the two dismissal rules, {len(real)} and {len(all_real)}'
        )
        for name, number, line, pattern in everything:
            reason = dismissal(line)
            report.say(
                f'  {name}:{number}  [{pattern}]  {line[:80]}' + (f'   <- {reason}' if reason else '')
            )
        if not everything:
            report.say('  none')

    report.say()
    report.say('the environment, over every source file of both packages:')
    argument_sites, variable_sites = [], []
    for label, sources in tiers:
        for name, number, line, pattern in scan(sources, (r'\benv::',), product_only=False):
            where = 'argument vector' if any(f in line for f in ARGUMENT_FORMS) else 'other'
            (argument_sites if where == 'argument vector' else variable_sites).append(
                (label, name, number, line, where)
            )
    for label, name, number, line, where in argument_sites + variable_sites:
        report.say(f'  {label:15} {name}:{number}  {where:16} {line[:70]}')
    reads = scan(library + [engine_binary] + observer, ENVIRONMENT_READS, product_only=False)
    report.say(f'environment-variable reads (env::var, env::var_os, env::vars): {len(reads)}')
    for name, number, line, pattern in reads:
        report.say(f'  {name}:{number}  {line[:88]}')

    spawned = sum(len(pair[1]) for pair in spawns.values())
    report.verdict(
        spawned == 0 and not reads,
        'no process-spawning site in either package at any tier, and no environment variable is '
        f'read anywhere: {len(argument_sites)} `env::args` site(s), which is the argument vector, '
        f'and {len(variable_sites)} other `env::` site(s)'
        if spawned == 0 and not reads
        else f'{spawned} process site(s) and {len(reads)} environment read(s)',
        'S3a is written for the stage that has a connector: it expects the spawn and the '
        'credential pass-through to exist and to be confined to the engine\'s binary. Neither '
        'exists yet, so what is checked here is the absence, and the absence is the stronger '
        'reading. The one `env::` site that is neither `args` nor `var` is `env::temp_dir` in the '
        'observer\'s own test module, which chooses a directory for an export test.',
    )


# ---------------------------------------------------------------------------------------------
# 5. S5 — the usage text agrees with the program
# ---------------------------------------------------------------------------------------------

def check_5(report, cli_source, options_source, product):
    report.check(5, 'S5 — the fifth policy value is in both texts and the withdrawn sentence is gone', (
        'The usage text\'s fifth policy value is present in both hosts\' texts, and the sentence '
        '"None of the four learns anything or calls a model; all four are deterministic" no '
        'longer appears in either. A usage text that contradicts the program is the first defect '
        'a reader meets, so it is checked rather than reviewed.'
    ))
    report.method(
        'both `USAGE` constants assembled from their `concat!` literals and searched; then the '
        'withdrawn sentence over raw source of both packages, comments included, so that a hit '
        'in a comment is reported rather than blanked away.'
    )
    texts = (
        ('mokiterions-core/src/cli.rs', usage_text(cli_source)),
        ('mokiterions-tui/src/options.rs', usage_text(options_source)),
    )
    report.say()
    missing = []
    for path, text in texts:
        option = text.count(POLICY_OPTION_LINE)
        entry = len(re.findall(r'^\s+' + FIFTH_POLICY_VALUE + r'\s{2,}\S', text, re.M))
        withdrawn = text.count('None of the four learns anything')
        report.say(
            f'  {path:34} {len(text):5} characters   '
            f'"{POLICY_OPTION_LINE}" x{option}   "{FIFTH_POLICY_VALUE}" entry x{entry}   '
            f'withdrawn sentence x{withdrawn}'
        )
        if option < 1 or entry != 1 or withdrawn:
            missing.append(path)

    report.say()
    report.say('the sentence each text carries in its place, quoted from the assembled constants:')
    for path, text in texts:
        found = re.search(r'The first four are[^\n]*\n[^\n]*\n[^\n]*', text)
        for line in (found.group(0).split('\n') if found else ['(not found)']):
            report.say(f'    {line.strip()}')
        report.say(f'      ^ {path}')

    report.say()
    raw = raw_scan(product, WITHDRAWN_SENTENCE)
    report.say(f'the withdrawn sentence in raw source of both packages: {len(raw)} hit(s)')
    for name, number, line in raw:
        report.say(f'  {name}:{number}  {line[:96]}')
    if raw:
        report.say(
            '  a doc comment, not a usage text: cli.rs states which two sentences rule 18.3 '
            'required to change and quotes the one it withdrew. A reader who greps for the '
            'sentence finds this and should find it.'
        )
    blanked = scan(product, (re.escape(WITHDRAWN_SENTENCE),))
    report.say(f'the same sentence in blanked product source: {len(blanked)} hit(s)')

    report.verdict(
        not missing and not blanked,
        'both texts carry the five-value option line and exactly one `llm` entry, and neither '
        'carries the withdrawn sentence outside a comment that explains its withdrawal'
        if not missing and not blanked else f'{missing}, blanked hits {len(blanked)}',
        'this checks that the text says the right thing, not that the program does what the text '
        'says. S5a is the byte-identity of the shared descriptions between the two hosts and is '
        'held by mokiterions-tui/tests/options.rs; case L27 is whether the shared rules block '
        'reads as an instruction, and it is a manual assessment.',
    )


# ---------------------------------------------------------------------------------------------
# 6. S6b — no mode branch in the library, and no live path in the observer
# ---------------------------------------------------------------------------------------------

def check_6(report, library, observer, mode_vocabulary):
    report.check(6, 'S6b — no live-versus-replay branch, and the observer has no live path', (
        'Neither host contains a live-versus-replay branch inside the library target, and the '
        'observer contains no live path at all: no ceiling parsing that reaches a run, no '
        'connector spawn, no credential read. REQ-MOK-077\'s prohibition is checked as an '
        'absence, and an absence nobody looks for is indistinguishable from an oversight.'
    ))
    report.method(
        'the mode vocabulary imported from analysis/static-checks.py, run over the library and '
        'over the observer; then the ceiling, credential and connector vocabularies over the '
        'observer. The mode scan duplicates that file\'s check 2 on purpose: S6b\'s subject is '
        'neither host, so the observer needs the same word list, and re-running the check is '
        'cheaper than splitting it across two files.'
    )
    report.say()
    library_mode = scan(library, [r'\b' + re.escape(word) + r'\b' for word in mode_vocabulary])
    observer_mode = scan(observer, [r'\b' + re.escape(word) + r'\b' for word in mode_vocabulary])
    report.say(f'mode vocabulary ({len(mode_vocabulary)} words):')
    report_hits(report, library_mode, '  engine library')
    report_hits(report, observer_mode, '  observer')

    report.say()
    ceiling = scan(observer, CEILING_VOCABULARY)
    credential = scan(observer, CREDENTIAL_VOCABULARY)
    connector = scan(observer, CONNECTOR_VOCABULARY)
    report_hits(report, ceiling, 'observer, ceiling vocabulary')
    report_hits(report, credential, 'observer, credential vocabulary')
    report_hits(report, connector, 'observer, connector vocabulary')

    report.say()
    report.say(
        'the word `cost` is excluded from the ceiling vocabulary, and the exclusion is a finding '
        'about the scan rather than about the code: the observer\'s layout arithmetic calls a '
        'row\'s height a cost.'
    )
    for name, number, line, pattern in scan(observer, (r'\bcost\b',))[:4]:
        report.say(f'  {name}:{number}  {line[:88]}')

    total = len(library_mode) + len(observer_mode) + len(ceiling) + len(credential) + len(connector)
    report.verdict(
        total == 0,
        'no mode value or live-versus-replay branch in either host, and no ceiling, credential '
        'or connector identifier anywhere in the observer'
        if total == 0 else f'{total} hit(s) across the five vocabularies',
        'a word list finds a live path that is spelled. The positive statement - that the '
        'observer reaches a decision only through a replay port - is a test: '
        '`the_observer_replays_this_source_to_the_horizon_with_every_pane` and '
        '`the_replay_source_with_no_port_is_refused_on_the_first_tick`, both in '
        'mokiterions-tui/tests/replay.rs. This check is the absence beside them.',
    )


# ---------------------------------------------------------------------------------------------
# 7. S7 — the shared rules block exists once
# ---------------------------------------------------------------------------------------------

def check_7(report, root, simulation, product):
    report.check(7, 'S7 — the shared rules block exists in exactly one place in the source', (
        'The shared rules block exists in exactly one place in the source, so that case L27\'s '
        'assessment has one object and a drift between two copies is impossible.'
    ))
    report.method(
        'the declaration counted across both packages; then every literal of the `concat!` that '
        'builds it, each searched over every tracked Rust file. A copy that shared no literal '
        'with the original would not be a copy of it.'
    )
    declarations = raw_scan(product, 'const SHARED_RULES')
    report.say()
    report.say(f'declarations of `const SHARED_RULES`: {len(declarations)}')
    for name, number, line in declarations:
        report.say(f'  {name}:{number}  {line[:96]}')

    pieces = concat_literals(simulation, 'SHARED_RULES')
    assembled = ''.join(unescape(piece) for piece in pieces)
    report.say(
        f'the `concat!` holds {len(pieces)} literal(s) and assembles to {len(assembled)} '
        f'characters, {len(assembled.encode("utf-8"))} bytes'
    )

    rust_files = sorted(run(['git', 'ls-files', '*.rs'], cwd=root).stdout.split())
    report.say(f'tracked Rust files searched: {len(rust_files)}')
    substantial = [piece for piece in pieces if len(piece.strip()) >= 24]
    report.say(f'literals long enough to identify a copy (>= 24 characters): {len(substantial)}')
    elsewhere = {}
    for path in rust_files:
        text = io.open(os.path.join(root, path), encoding='utf-8').read()
        count = sum(text.count(piece) for piece in substantial)
        if count:
            elsewhere[path] = count
    for path, count in sorted(elsewhere.items()):
        report.say(f'  {path}: {count} literal occurrence(s)')

    report.say()
    report.say('the block by name, everywhere it is used:')
    for name, number, line in raw_scan(product, 'SHARED_RULES'):
        report.say(f'  {name}:{number}  {line[:88]}')

    passed = len(declarations) == 1 and list(elsewhere) == [
        p for p in rust_files if p.endswith('mokiterions-core/src/simulation.rs')
    ]
    report.verdict(
        passed,
        'one declaration, and every literal of it occurs in that file alone'
        if passed else f'{len(declarations)} declaration(s), literals also in {list(elsewhere)}',
        'the rules are also written in prose in SIMULATION_RULES.md and in SPEC-MOK-001 rule 21. '
        'That is the authority the block is derived from, not a second copy, and this check does '
        'not compare the two - whether the block says what the rules say is case L27, a manual '
        'assessment. What is established here is that there is one object for that assessment to '
        'be about.',
    )


# ---------------------------------------------------------------------------------------------
# 8. C1 and C3 — no credential, and nothing opened in search of one
# ---------------------------------------------------------------------------------------------

def check_8(report, tiers, library, engine_binary, observer, literal_count):
    report.check(8, 'C1 and C3 — no credential exists to appear anywhere, and nothing hunts for one', (
        'C1: no credential appears in any transcript, any record stream, any run record, any '
        'authorization record, either output stream, or any error message. Checked by pattern '
        'over retained evidence and by a test that sets a synthetic credential value and asserts '
        'it appears in no produced byte. C3: the credential is read from the process environment '
        'and from nowhere else: no file, keychain or configuration directory is opened in the '
        'search for one.'
    ))
    report.method(
        'the credential vocabulary over blanked product source of both packages; every '
        'filesystem-opening site enumerated with what it opens; and the environment-read count '
        'from check 4. The pattern half over retained evidence is scripts/check_transcript_'
        'reading.py, whose L17 reading examined 1,828 text values of the committed transcript.'
    )
    everything = library + [engine_binary] + observer
    report.say()
    hits = scan(everything, CREDENTIAL_VOCABULARY)
    report_hits(report, hits, 'credential vocabulary in product source')
    all_tiers = scan(everything, CREDENTIAL_VOCABULARY, product_only=False)
    report.say(f'the same vocabulary at every tier, tests included: {len(all_tiers)} hit(s)')
    for name, number, line, pattern in all_tiers:
        report.say(f'  {name}:{number}  [{pattern}]  {line[:82]}')
    report.say(
        '  a count of zero at every tier does not mean the words are absent from the file. They '
        'are in string literals, and the blanking removes literals, so a raw search is the only '
        'way to see them. Where they are:'
    )
    for word in ('authorization', 'bearer', 'api_key', 'credential'):
        for name, number, line in raw_scan(everything, f'"{word}"'):
            report.say(f'    {name}:{number}  {line[:86]}')
    report.say(
        '  that is the vocabulary list a test holds in order to assert its absence from a '
        'record: `a_record_states_no_response_no_usage_and_no_credential`. The list being in a '
        'test and nowhere else is the reading this check wants.'
    )

    report.say()
    report.say('every filesystem-opening site in either package\'s product code, and what it does:')
    openers = (r'\bFile::open\b', r'\bFile::create\b', r'\bOpenOptions\b', r'\bread_to_string\b',
               r'\bcreate_dir', r'\bremove_file\b', r'\bremove_dir')
    purposes = (
        ('File::open', 'reads a path the operator wrote on the command line'),
        ('OpenOptions', 'writes a path the operator wrote on the command line'),
        ('File::create', 'writes a path the operator wrote on the command line'),
        ('remove_file', 'removes a destination this process created, per SPEC-MOK-007 rule 13.4'),
        ('remove_dir', 'removes a directory this process created'),
    )
    opens, by_tier = [], []
    for label, sources in tiers:
        found = scan(sources, openers)
        by_tier.append((label, len(found)))
        opens.extend(found)
        report.say(f'  {label}: {len(found)} site(s)')
        for name, number, line, pattern in found:
            purpose = next((text for needle, text in purposes if needle in line), 'unclassified')
            report.say(f'    {name}:{number}  {line[:62]}')
            report.say(f'        {purpose}')
    report.say(
        f'  {len(opens)} site(s): ' + ', '.join(f'{count} in the {label}' for label, count in by_tier)
        + '. The engine\'s library target opens nothing, which is check 3. The '
        f'{sum(count for label, count in by_tier if "library" in label)} site(s) outside a binary '
        'are in the observer\'s export module, which writes and removes the file an operator asked '
        'it to export to; it predates this work order and no path of it reaches a decision. '
        'Each takes a variable, never a literal: check 2 searched the '
        f'{literal_count:,} string literals of both packages\' product source and found no '
        'path-shaped one. So no site here can be a '
        'configuration directory, a keychain or a dotfile - there is no constant for one to be '
        'named by, and nothing in either package searches for a path it was not given.'
    )
    reads = scan(everything, ENVIRONMENT_READS, product_only=False)
    report.say(f'environment-variable reads: {len(reads)}')

    report.verdict(
        False,
        'the measurable half passes and C1\'s second half cannot be run at this stage: no code '
        'path reads a credential, so there is no value to set synthetically and no byte for it '
        'to appear in. The connector, which is where SPEC-MOK-007 rules 10.5 and 13.4 place the '
        'credential, is out of this work order\'s scope.',
        'this is an escalation and not a pass. The obligation stands and its subject does not '
        'exist yet; the work order that builds the connector inherits it. What is established '
        'here is stronger than a negative test would be for the code that exists: there is no '
        'environment read at all, so no credential can enter this program by any route.',
    )


# ---------------------------------------------------------------------------------------------
# 9. C5 — nothing can leave, so nothing does
# ---------------------------------------------------------------------------------------------

def check_9(report, library, engine_binary, observer):
    report.check(9, 'C5 — nothing leaves the repository, because no outbound path exists', (
        'What leaves the repository in a live run is the request text only. No source, no path, '
        'no repository content and no identity beyond a Mokiterion identifier appears in any '
        'request.'
    ))
    report.method(
        'the socket class and the outbound vocabulary over both packages at every tier; then a '
        'pointer to where the positive half of the sentence is measured.'
    )
    everything = library + [engine_binary] + observer
    report.say()
    sockets = scan(everything, CAPABILITIES['socket'], product_only=False)
    report_hits(report, sockets, 'socket types and connect, every tier')
    outbound = scan(everything, OUTBOUND_VOCABULARY, product_only=False)
    report_hits(report, outbound, 'outbound vocabulary, every tier')

    report.say()
    report.say('what a request carries, measured elsewhere and named here:')
    for line in (
        'block A  the shared rules, one constant, no identity and nothing that varies  '
        '-> block_a_gives_no_strategy_and_names_nothing_that_varies',
        'block B  the deciding Mokiterion\'s own identifier and nothing else  -> case L15a',
        'block C  the deciding Mokiterion\'s own observation  -> cases L12 and L13, and '
        'scripts/check_transcript_reading.py',
        'block D  verb-target pairs over the core proposal list  -> case L4',
        'no path, no float and no timestamp across 233 records and 1,828 text values  '
        '-> scripts/check_transcript_reading.py, case L17',
    ):
        report.say(f'  {line}')

    passed = not sockets and not outbound
    report.verdict(
        passed,
        'no socket type, no connect, no URL and no HTTP identifier anywhere in either package, '
        'at any tier - so there is no route by which anything leaves at all'
        if passed else f'{len(sockets)} socket hit(s), {len(outbound)} outbound hit(s)',
        'the positive half - that what leaves in a live run is the request text only - needs a '
        'live run, which this work order excludes. What is measurable now is the content of the '
        'request as the transcript carries it, which is the four cases named above, and the '
        'absence of any transport to carry it. An operator\'s connector is outside this '
        'repository and nothing here can see what it sends.',
    )


def main():
    base, output_file = sys.argv[1:3]
    root = os.getcwd()
    wo = load(WO_019, 'wo_019_static_checks')
    sibling = load(SIBLING, 'wo_025_static_checks')

    library = [wo.Source(os.path.join(root, PACKAGE, 'src', name)) for name in LIBRARY_SOURCES]
    by_name = {source.name: source for source in library}
    engine_binary = wo.Source(os.path.join(root, PACKAGE, 'src', 'main.rs'))
    observer_names = sorted(
        name for name in os.listdir(os.path.join(root, OBSERVER, 'src')) if name.endswith('.rs')
    )
    observer = [wo.Source(os.path.join(root, OBSERVER, 'src', name)) for name in observer_names]
    product = library + [engine_binary] + observer
    observer_binary = next(source for source in observer if source.name == 'main.rs')
    observer_library = [source for source in observer if source.name != 'main.rs']
    # Four tiers rather than two packages, because a capability in a library module and the same
    # capability in a binary are different facts: ARCH-MOK-001's prohibition is on a library.
    tiers = (
        ('engine library', library),
        ('engine binary', [engine_binary]),
        ('observer library', observer_library),
        ('observer binary', [observer_binary]),
    )

    report = wo.Report()
    report.lines.extend([
        '# VER-MOK-018: the architecture, usage and security checks with no other runner',
        '#',
        f'# repository root:  {root}',
        f'# candidate:        {run(["git", "rev-parse", "HEAD"]).stdout.strip()}',
        f'# base commit:      {base}',
        f'# imported:         {WO_019}  (Source, Report, strip_rust)',
        f'# imported:         {SIBLING}  (MODE_VOCABULARY)',
        '#',
        '# command: python docs/engineering/simulation/evidence/WO-MOK-025/analysis/'
        'architecture-checks.py \\',
        '#              <base-commit> <output-file>',
        '#',
        '# checks: 1 S1, 2 S2a, 3 S3, 4 S3a, 5 S5, 6 S6b, 7 S7, 8 C1 and C3, 9 C5.',
        '#',
        '# S2, S4, S4a, S5a, S6 and S6a are not here and are not gaps. S2 is the connector\'s',
        '# dependency surface, which does not exist. S4 and S4a are candidate/public-surface.txt.',
        '# S5a, S6 and S6a are tests that run on every cargo test, named in the completion',
        '# report\'s item 7 table. L3 and L11 are analysis/static-checks.py beside this file.',
        '#',
        '# Every scan runs over source with comments and literals blanked by the imported',
        '# module\'s `strip_rust`, except check 2\'s literal search, which says so. Line numbers',
        '# survive the blanking. `product_only` excludes lines inside a `#[cfg(test)]` module,',
        '# and a check that wants the test tier too asks for it and prints both figures.',
        '#',
        f'# engine library: {", ".join(LIBRARY_SOURCES)}  '
        f'({sum(len(source.raw_lines) for source in library):,} lines)',
        f'# engine binary:  main.rs  ({len(engine_binary.raw_lines):,} lines)',
        f'# observer:       {", ".join(observer_names)}  '
        f'({sum(len(source.raw_lines) for source in observer):,} lines)',
    ])

    check_1(report, root, base)
    literal_count = check_2(report, root, product)
    check_3(report, library)
    check_4(report, tiers, library, engine_binary, observer)
    check_5(report, by_name['cli.rs'], next(s for s in observer if s.name == 'options.rs'), product)
    check_6(report, library, observer, sibling.MODE_VOCABULARY)
    check_7(report, root, by_name['simulation.rs'], product)
    check_8(report, tiers, library, engine_binary, observer, literal_count)
    check_9(report, library, engine_binary, observer)

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
    report.lines.append('# ---- full text of this script, retained as the WO-MOK-019 packet does ----')
    report.lines.append('')
    report.lines.extend(io.open(__file__, encoding='utf-8').read().split('\n'))

    with io.open(output_file, 'w', encoding='utf-8', newline='\n') as handle:
        handle.write('\n'.join(report.lines) + '\n')

    print(f'{len(report.results)} checks, {len(findings)} findings; written to {output_file}')
    for number, title, word, text in findings:
        print(f'  {number}. {word} {title}: {text}')
    return 1 if findings else 0


if __name__ == '__main__':
    raise SystemExit(main())
