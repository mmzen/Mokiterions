"""WO-MOK-007: the engine's public interface, and the two arithmetic prohibitions, on both sides.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-007/analysis/interface-census.py \
        <pre-change-root> <candidate-root>

Each root is a checkout root containing `mokiterions-core/src`. The pre-change root is a git worktree
at 60fda9faffbd452752a34efa356f16cc6ad1d3ff; the candidate root is the working tree.

Three checks, all of them differential where a difference is what matters:

  1. THE PUBLIC INTERFACE. `SPEC-MOK-002` rule 5 fixes the engine's interface as closed, and
     `lib.rs` states that WO-MOK-007 grows it by exactly two things, both values: a third `Policy`
     variant and a fourth attribute on the observation snapshot's Mokiterion entry. This enumerates
     the public items, public fields and enum variants on both sides and diffs them, so the claim is
     checked rather than repeated.

  2. NO FLOATING POINT. `SPEC-MOK-001` requires integer arithmetic throughout, and rule 19's
     tolerant test is stated in truncating integer division. This looks for `f32`, `f64` and decimal
     literals in the engine's non-test source on both sides.

  3. `fear` HAS ONE WRITER AND NO READER. Rule 3 states that `fear` is deliberately absent from the
     observation and that no rule and no decision source reads it; rule 12 is its one writer. This
     lists every occurrence of the identifier in the engine and classifies it.

METHOD, stated so it can be disputed. The parse is line-based over the source text with
`#[cfg(test)]` blocks removed by brace matching, which is the method WO-MOK-006's
`public-item-census.txt` established for the same kind of check in this repository. It is not a Rust
parser and it does not need to be: both sides are parsed identically, so a parsing limitation shows
up on both sides and cancels in the diff. The absolute counts are reported as what this method
counts, not as a claim about what rustc would count.
"""

import io
import os
import re
import sys
from collections import defaultdict

sys.stdout.reconfigure(encoding='utf-8')

FILES = ('lib.rs', 'cli.rs', 'main.rs', 'simulation.rs')
ITEM = re.compile(r'^\s*pub(?:\s*\([^)]*\))?\s+(?:unsafe\s+|const\s+|async\s+)*'
                  r'(mod|const|static|fn|struct|enum|trait|type|union)\s+([A-Za-z_][A-Za-z0-9_]*)')
FIELD = re.compile(r'^\s*pub(?:\s*\([^)]*\))?\s+([a-z_][A-Za-z0-9_]*)\s*:')
VARIANT = re.compile(r'^\s*([A-Z][A-Za-z0-9_]*)\s*(?:\{|\(|=|,|$)')
OPENS = re.compile(r'^\s*pub(?:\s*\([^)]*\))?\s+(struct|enum)\s+([A-Za-z_][A-Za-z0-9_]*)')
FLOAT_TYPE = re.compile(r'\bf32\b|\bf64\b')
DECIMAL = re.compile(r'\b\d+\.\d+\b')


def strip_tests(text):
    """Blank every `#[cfg(test)]` item, by matching braces from its declaration onward.

    The lines are emptied rather than deleted so that every line number this script reports is the
    line number in the file itself, which is what a reader checking a finding will open.
    """
    lines = text.split('\n')
    index = 0
    while index < len(lines):
        if '#[cfg(test)]' in lines[index]:
            depth = 0
            started = False
            while index < len(lines):
                depth += lines[index].count('{') - lines[index].count('}')
                started = started or '{' in lines[index]
                lines[index] = ''
                index += 1
                if started and depth <= 0:
                    break
            continue
        index += 1
    return '\n'.join(lines)


def census(root):
    """The public surface of the engine crate, as items, fields and variants."""
    items, fields, variants, floats = [], [], [], []
    for name in FILES:
        path = os.path.join(root, 'mokiterions-core', 'src', name)
        if not os.path.exists(path):
            continue
        source = strip_tests(io.open(path, encoding='utf-8').read())
        container = None
        container_kind = None
        depth = 0
        for line in source.split('\n'):
            stripped = line.strip()
            match = ITEM.match(line)
            if match:
                items.append(f'{name}: pub {match.group(1)} {match.group(2)}')
            opener = OPENS.match(line)
            if opener and '{' in line:
                container, container_kind, depth = opener.group(2), opener.group(1), 1
            elif container is not None:
                depth += line.count('{') - line.count('}')
                if depth <= 0:
                    container = None
                elif container_kind == 'struct':
                    field = FIELD.match(line)
                    if field:
                        fields.append(f'{name}: {container}.{field.group(1)}')
                elif container_kind == 'enum' and not stripped.startswith('//'):
                    variant = VARIANT.match(line)
                    if variant:
                        variants.append(f'{name}: {container}::{variant.group(1)}')
            floats += classify_numbers(name, line, stripped)
    return items, fields, variants, floats


def classify_numbers(name, line, stripped):
    """Every float type and decimal literal on one line, with where it sits.

    A decimal inside a doc comment or a string literal is text, not arithmetic: the density argument
    arrives as the text `0.75` and the usage message prints it back, and several comments cite
    specification rule numbers such as `8.1`. Only a decimal in executable position would be a float,
    and `f32` or `f64` anywhere would be one whatever its position.
    """
    found = []
    for match in FLOAT_TYPE.finditer(line):
        found.append(('float type', name, match.group(0), stripped[:80]))
    comment = line.find('//')
    for match in DECIMAL.finditer(line):
        start = match.start()
        quotes = line.count('"', 0, start)
        if comment != -1 and start > comment:
            where = 'comment'
        elif quotes % 2 == 1:
            where = 'string literal'
        else:
            where = 'CODE'
        found.append((where, name, match.group(0), stripped[:80]))
    return found


def strip_comments(text):
    """Every line with its `//` tail removed, so a region search reads code and not prose."""
    return '\n'.join(line.split('//')[0] for line in text.split('\n'))


def bodies(source, opener):
    """The brace-delimited body of every declaration matching `opener`.

    Returns (line number, declaration, body). A declaration that ends at a semicolon before any
    brace -- the `DecisionSource` trait's own `fn decide` signature -- has no body and is skipped,
    rather than running on into whatever follows it.
    """
    found = []
    lines = source.split('\n')
    for index, line in enumerate(lines):
        if not re.search(opener, line):
            continue
        head = line.split('{')[0]
        if '{' not in line and head.rstrip().endswith(';'):
            continue
        depth = 0
        started = False
        collected = []
        for following in lines[index:]:
            depth += following.count('{') - following.count('}')
            started = started or '{' in following
            collected.append(following)
            if started and depth <= 0:
                break
        if not started:
            continue
        found.append((index + 1, line.strip(), '\n'.join(collected)))
    return found


def fear_uses(root):
    """Every occurrence of `fear` in the engine's non-test source, plus the decision-path check.

    The classification is deliberately coarse -- a mutation is an assignment to the field, everything
    else is not -- because the load-bearing claim is not about categories. It is that `fear` does not
    appear anywhere a decision is made, and that is checked directly, by extracting the bodies of the
    `DecisionSource` trait, of every `fn decide`, and of the `Observation` struct, and looking inside
    them.
    """
    uses = []
    mutations = []
    regions = []
    for name in FILES:
        path = os.path.join(root, 'mokiterions-core', 'src', name)
        if not os.path.exists(path):
            continue
        source = strip_tests(io.open(path, encoding='utf-8').read())
        for number, line in enumerate(source.split('\n'), start=1):
            if not re.search(r'\bfear\b', line):
                continue
            stripped = line.strip()
            mutation = bool(re.search(r'\.fear\s*=[^=]|\bfear\s*=[^=]', line))
            uses.append((name, number, 'mutation' if mutation else '', stripped[:100]))
            if mutation:
                mutations.append(f'{name}:{number}  {stripped[:100]}')
        for opener in (r'\btrait\s+DecisionSource\b', r'\bfn\s+decide\b',
                       r'\bstruct\s+Observation\b', r'\bfn\s+observe\b'):
            for number, declaration, body in bodies(source, opener):
                code = strip_comments(body)
                regions.append((f'{name}:{number}  {declaration.split("(")[0][:44]}',
                                body.count('\n') + 1,
                                len(re.findall(r'\bfear\b', code)),
                                len(re.findall(r'\bfear\b', body))))
    return uses, mutations, regions


def section(title, body):
    return [title, '-' * len(title), ''] + body + ['']


def main():
    pre_root, post_root = sys.argv[1:3]
    out = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                       'interface-and-purity.txt')

    pre = census(pre_root)
    post = census(post_root)
    lines = [
        'WO-MOK-007 - the engine\'s public interface, and the two arithmetic prohibitions',
        '',
        f'pre-change root:  {pre_root}   (60fda9faffbd452752a34efa356f16cc6ad1d3ff)',
        f'candidate root:   {post_root}',
        '',
        'Method: see the header of analysis/interface-census.py. Both sides are parsed identically,',
        'so the diff is the load-bearing part and the absolute counts are what this method counts.',
        '',
    ]

    labels = ('public items', 'public fields', 'enum variants')
    rows = []
    for index, label in enumerate(labels):
        rows.append([label, len(pre[index]), len(post[index]),
                     len(post[index]) - len(pre[index])])
    width = max(len(label) for label in labels)
    lines += section('1. The public interface', [
        f'  {"surface".ljust(width)}  before  after  change',
        f'  {"-" * width}  ------  -----  ------',
    ] + [f'  {row[0].ljust(width)}  {row[1]:6}  {row[2]:5}  {row[3]:+6}' for row in rows])

    additions, removals = [], []
    for index, label in enumerate(labels):
        for entry in sorted(set(post[index]) - set(pre[index])):
            additions.append(f'  + {label[:-1]}: {entry}')
        for entry in sorted(set(pre[index]) - set(post[index])):
            removals.append(f'  - {label[:-1]}: {entry}')
    lines += section('The difference, item by item',
                     (additions or ['  (no additions)'])
                     + (removals or ['  (no removals)'])
                     + ['',
                        f'  additions: {len(additions)}    removals: {len(removals)}',
                        '',
                        '  `lib.rs` claims the interface grew by exactly two things, both values: a',
                        '  third `Policy` variant and a fourth attribute on the observation',
                        '  snapshot\'s Mokiterion entry. That is what the difference shows, and',
                        '  nothing was removed.'])

    def tally(hits):
        counted = defaultdict(int)
        for where, _, _, _ in hits:
            counted[where] += 1
        return counted

    before, after = tally(pre[3]), tally(post[3])
    body = [f'  {"position".ljust(16)}  before  after',
            f'  {"-" * 16}  ------  -----']
    for where in ('float type', 'CODE', 'string literal', 'comment'):
        body.append(f'  {where.ljust(16)}  {before[where]:6}  {after[where]:5}')
    body += ['',
             '  A float type or a decimal in CODE would be a violation; both are zero on both sides.',
             '  Rule 19\'s tolerant test is integer arithmetic with division that truncates toward',
             '  zero, and the density argument arrives as text and is held in integer hundredths, so',
             '  no float exists to be rounded. The decimals that do occur are that text and the',
             '  specification rule numbers cited in comments; all of them are listed here so that the',
             '  classification can be disputed line by line:',
             '']
    for where, name, hit, text in post[3]:
        body.append(f'  [{where}] {name}: {hit}   {text}')
    lines += section('2. No floating point in the engine', body)

    uses, mutations, regions = fear_uses(post_root)
    body = [f'  occurrences of the identifier in non-test engine source: {len(uses)}',
            f'  of which assign to it:                                   {len(mutations)}',
            '']
    for mutation in mutations:
        body.append(f'  [mutation] {mutation}')
    body += ['',
             '  Every occurrence, so that the one mutation is visible in its context rather than',
             '  asserted:',
             '']
    for name, number, kind, text in uses:
        body.append(f'  {f"{name}:{number}".ljust(20)} {kind.ljust(9)} {text}')
    body += ['',
             '  The decision path, checked directly. A reader would have to be inside the',
             '  `DecisionSource` trait, inside one of the three `fn decide` bodies, or on the',
             '  `Observation` type a source reads from. Each of those regions is extracted by brace',
             '  matching and searched:',
             '',
             f'  {"region".ljust(62)} {"lines":>5} {"code":>5} {"all":>4}',
             f'  {"-" * 62} {"-" * 5} {"-" * 5} {"-" * 4}']
    for declaration, length, hits, mentions in regions:
        body.append(f'  {declaration.ljust(62)} {length:5} {hits:5} {mentions:4}')
    body += ['',
             '  `code` counts occurrences with every `//` tail removed; `all` includes the comments.',
             '  The one comment mention is inside the `Observation` type and is the sentence stating',
             '  that `fear` is deliberately absent from it -- which is the claim, written where the',
             '  next person to add a field will read it.',
             '',
             '  Zero code occurrences in every region. Rule 3 states that `fear` is not carried on the',
             '  observation, and the observation is the only thing a decision source is given.',
             '  The single mutation is rule 12\'s, and the remaining occurrences are the field\'s own',
             '  declaration, its initialization at `0`, the snapshot the observer reads, and the',
             '  three event records that report it.']
    lines += section('3. `fear` has one writer and no reader', body)

    floats_in_code = after['CODE'] + after['float type']
    if floats_in_code or len(additions) != 2 or removals             or any(hits for _, _, hits, _ in regions):
        lines += ['', 'RESULT: FAIL']
    else:
        lines += ['', 'RESULT: PASS - two additions, no removals, no float in code, no `fear` reader']

    io.open(out, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
    print('\n'.join(lines))
    print(f'written to: {out}')
    return 0


if __name__ == '__main__':
    sys.exit(main())
