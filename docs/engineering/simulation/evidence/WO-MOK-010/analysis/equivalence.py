"""WO-MOK-010 oracle 3: the enumerated situation set, its size, and the run that exhausts it.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-010/analysis/equivalence.py

Oracle 3 is arithmetic equivalence at the trait's lower bound: rule 19's tolerant test is written so
that at tolerance `0` it reduces to rule 5's exact-fit test, and the trait-aware source must therefore
propose exactly what the reference source proposes, and leave the shared stream exactly where the
reference source leaves it, for every observation at that tolerance. The oracle is discharged by
enumeration rather than by argument, and `at_tolerance_zero_the_trait_aware_source_proposes_what_the_\
reference_source_proposes` is the enumeration.

The claim that needs checking is not that the test passes -- `static-checks.txt` shows the whole suite
passing -- but that the set it enumerates is the set the evidence says it is. A test that asserts its
own case count can still be enumerating the wrong thing, and a reader told "2,808 situations" has been
given a number, not a set. So this script reads the enumeration out of the test source, factors it
into the five dimensions the test varies, multiplies them, and compares the product against both the
count the test asserts for itself and the figure this evidence packet reports. Then it runs the test.

What the enumeration does not cover is stated here too, because a reader should not have to infer the
boundary of a claim from the boundary of a loop.
"""

import io
import os
import re
import subprocess
import sys

sys.stdout.reconfigure(encoding='utf-8')

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.exists(os.path.join(ROOT, 'Cargo.toml')):
    ROOT = os.path.dirname(ROOT)
SOURCE = os.path.join(ROOT, 'mokiterions-core', 'src', 'simulation.rs')
TEST = ('simulation::tests::at_tolerance_zero_the_trait_aware_source_'
        'proposes_what_the_reference_source_proposes')
COMPANIONS = ['simulation::tests::the_tolerant_test_governs_seeking_as_well_as_eating',
              'simulation::tests::a_trait_difference_alone_decides_whether_a_clipped_resource_'
              'is_eaten']


def source():
    return io.open(SOURCE, encoding='utf-8').read()


def array(text, pattern):
    """The comma-separated entries of the first array matching `pattern`.

    Two forms occur: a `const NAME: [T; N] = [..];` declaration, and a `for name in [..] {` loop. The
    entries are read as written -- `REFERENCE_SLEEP_THRESHOLD - 1` stays as that text -- so the
    artifact quotes the source rather than a value that has been resolved away from it.
    """
    for form in (pattern + r'\s*=\s*\[(.*?)\]\s*;', pattern + r'\s*\[(.*?)\]\s*\{'):
        match = re.search(form, text, re.S)
        if match:
            body = re.sub(r'//[^\n]*', '', match.group(1))
            return [entry.strip() for entry in body.split(',') if entry.strip()]
    raise SystemExit(f'no array matched {pattern}')


def offsets(text):
    """The offset tuples in `enumerated_placements`, counted from the source rather than assumed."""
    region = text[text.index('fn enumerated_placements'):]
    region = region[:region.index('\n    }\n')]
    tuples = re.findall(r'\(\s*-?\d+\s*,\s*-?\d+\s*\)', region)
    distances = array(region, r'for distance in')
    return tuples, distances, region


def main():
    out = os.path.join(os.path.dirname(HERE), 'measurements', 'equivalence.txt')
    text = source()

    satieties = array(text, r'const SATIETIES: \[u8; \d+\]')
    # `FoodClass::ALL` is one of several `const ALL` in the file, so it is read from inside that
    # type's own implementation block rather than by name alone.
    classes = array(text[text.index('impl FoodClass {'):], r'const ALL: \[Self; \d+\]')
    tuples, distances, region = offsets(text)
    energies = array(text, r'for energy in')
    companions = array(text, r'for companion in')
    placements = 1 + len(tuples) * len(distances) + 1

    dimensions = [
        ('satiety', len(satieties), ', '.join(satieties)),
        ('resource class', len(classes), ', '.join(entry.split('::')[-1] for entry in classes)),
        ('resource placement', placements,
         f'underfoot, then the {len(tuples)} compass offsets at each of the {len(distances)} '
         f'distances {", ".join(distances)}, then no resource at all'),
        ('energy', len(energies), ', '.join(energies)),
        ('a second resource underfoot', len(companions), ', '.join(companions)),
    ]
    product = 1
    for _, size, _ in dimensions:
        product *= size

    asserted = re.search(r'assert_eq!\(\s*cases,\s*([\d_]+),', text)
    asserted = int(asserted.group(1).replace('_', '')) if asserted else None
    structural = re.search(r'assert_eq!\(\s*cases,\s*(SATIETIES\.len\(\).*?)\);', text, re.S)

    lines = [
        'VER-MOK-010 oracle 3 - the enumerated situation set, and the run that exhausts it',
        '',
        'What this checks, and why the number is not enough on its own: see the header of',
        'analysis/equivalence.py. Every figure below is read out of',
        'mokiterions-core/src/simulation.rs.',
        '',
        'The five dimensions the enumeration varies',
        '',
        f'  {"dimension":28}  {"size":>4}  values',
        f'  {"-" * 28}  ----  ------',
    ]
    for name, size, values in dimensions:
        lines.append(f'  {name:28}  {size:4}  {values}')
    lines += [
        f'  {"":28}  ----',
        f'  {"product":28}  {product:4}',
        '',
        f'  the count the test asserts for itself:              {asserted}',
        f'  the count this evidence packet reports:             2808',
        f'  all three agree:                                    '
        f'{product == asserted == 2808}',
        '',
        '  The test asserts its own count twice, once structurally and once as a literal, so a change',
        '  to any dimension fails it rather than silently shrinking the set:',
        '',
        f'    assert_eq!(cases, {structural.group(1) if structural else "?"});',
        f'    assert_eq!(cases, {asserted:_}, "the enumerated situation set changed size");',
        '',
        'What each dimension is chosen to catch',
        '',
        '  satiety      The 13 values are not a sweep, they are the boundaries. Each class of resource',
        '               has a satiety at which it exactly fills the attribute and one either side:',
        '               50/51 and 49 for the high class, 70/71 and 69 for the medium, 85/86 and 84 for',
        '               the low, plus 0 and 100 at the ends. A tolerant test that differed from the',
        '               exact-fit test would differ first at one of these, because that is where the',
        '               clipped part changes from zero to non-zero.',
        '  class        All three, because the admitted waste is a fraction of the restoration and so',
        '               depends on the class.',
        '  placement    Underfoot exercises rule 19 case 1, distance 1 and distance 16 exercise case 3',
        '               at the near end and exactly at the perception radius, all eight compass',
        '               directions exercise the direction preference, and no resource at all exercises',
        '               the fallback. The two distances are read from the source, and one of them is',
        '               `PERCEPTION_RADIUS` itself rather than a copy of its value.',
        '  energy       Either side of the sleep threshold, because rule 19 case 2 and rule 5 case 2',
        '               share one constant and the enumeration should notice if they stopped doing so.',
        '  companion    A second resource underfoot makes case 1 choose between two candidates rather',
        '               than accept or decline one, which is where a difference in the selection order',
        '               would appear.',
        '',
        'What it does not cover, stated rather than left to be inferred',
        '',
        '  * One acting Mokiterion, alone. The eleven others are placed far away, so nothing is',
        '    perceived but resources. Fear is not in the observation at all -- `interface-and-purity.txt`',
        '    checks that -- so company cannot enter a decision, and this dimension is deliberately',
        '    absent rather than overlooked.',
        '  * Tolerance `0` only. That is the whole point of the oracle: it is the lower bound, where the',
        '    two sources must coincide. Sensitivity above the bound is a different claim, established by',
        '    the two tests run below it and measured over whole runs in `measurements/divergence.txt`.',
        '  * One position in the world, far from every edge. Edge behaviour is rule 4\'s and is',
        '    unchanged by this work order.',
        '',
        'The runs',
        '',
    ]

    ok = product == asserted == 2808
    for name in [TEST] + COMPANIONS:
        finished = subprocess.run(
            ['cargo', 'test', '-p', 'Mokiterions', '--lib', name, '--', '--exact'],
            cwd=ROOT, capture_output=True, text=True, encoding='utf-8', errors='replace')
        output = finished.stdout + finished.stderr
        result = next((line.strip() for line in output.split('\n')
                       if line.strip().startswith('test result:')), '(no result line)')
        passed = finished.returncode == 0 and result.startswith('test result: ok.')
        ok = ok and passed
        lines += [f'  $ cargo test -p Mokiterions --lib {name.split("::")[-1]} -- --exact',
                  f'    {result}',
                  '']

    lines += [
        '  The first is the enumeration itself. The two below it are the companions that keep it from',
        '  being vacuous in the other direction: one shows the same tolerant test governing rule 19',
        '  case 3 as well as case 1, and one shows a single unit of tolerance changing the proposal --',
        '  at `T = 34` and not at `T = 33` for a medium-class resource, which is what pins the division',
        '  as truncating rather than rounding.',
        '',
        '  `negative-control/oracle-3.txt` shows this enumeration failing when the trait-aware source',
        '  is perturbed, on the proposal in one control and on the stream position in the other, so a',
        '  pass here is a result and not an absence of checking.',
        '',
        'RESULT: ' + ('PASS - the enumerated set is 2,808 situations by construction, by the test\'s own\n'
                      '        assertion and by this independent factorization, and it is exhausted'
                      if ok else 'FAIL'),
    ]

    io.open(out, 'w', encoding='utf-8', newline='\n').write(
        '\n'.join(line.rstrip() for line in lines) + '\n')
    print('\n'.join(lines))
    print(f'written to: {out}')
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
