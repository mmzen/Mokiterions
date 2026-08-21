"""Does the merge conserve every test the other side had, and add exactly this work order's one?

`SPEC-MOK-004` rule 11 states the standard this reader is written against: "a work order that adds a
test corrects these figures here, and one that loses a test has a defect". A merge is the case the
rule's own paragraph anticipates -- figures re-measured on the merged tree rather than carried over --
so the question is not how many tests the merged tree runs but whether the set is the union, with
nothing dropped and nothing arriving unexplained.

The comparison is against `WO-MOK-019`'s own committed census rather than against a figure quoted from
a document. That file was written by the other chain, at its own commit, by its own command, and
`master` merged the tree it was taken on; using it means neither side's count is this reader's
assertion. Its format is `cargo test --workspace -- --list` output; this packet's format is
`analysis/test-census.py`'s `<target> :: <name> :: <outcome>`. Both are parsed to bare test names,
because the two readers qualify targets differently and a target-qualified comparison would measure
the two readers' conventions rather than the two suites.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/merge/census-conservation.py \
        docs/engineering/simulation/evidence/WO-MOK-019/merge/second/test-census.txt \
        docs/engineering/simulation/evidence/WO-MOK-017/merge/test-census.txt \
        docs/engineering/simulation/evidence/WO-MOK-017/post/test-census.txt
"""
import collections
import pathlib
import re
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

ADDED = 'simulation::tests::the_corrected_non_waste_condition_admits_the_specified_boundaries'

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


def rule(title):
    print()
    print(title)
    print('-' * len(title))


def listed_names(path):
    """Names out of `cargo test -- --list` output: lines reading `<name>: test`."""
    text = pathlib.Path(path).read_text(encoding='utf-8')
    return [match.group(1)
            for match in (re.match(r'^(\S+): test\s*$', line) for line in text.splitlines())
            if match]


def census_names(path):
    """Names out of this packet's census format, `<target> :: <name> :: <outcome>`."""
    text = pathlib.Path(path).read_text(encoding='utf-8')
    names, outcomes = [], []
    for line in text.splitlines():
        if ' :: ' not in line:
            continue
        fields = line.split(' :: ')
        names.append(fields[1].strip())
        outcomes.append(fields[2].strip() if len(fields) > 2 else '')
    return names, outcomes


def report(label, names):
    counts = collections.Counter(names)
    duplicates = sorted(name for name, count in counts.items() if count > 1)
    print(f'    {label:<44}{len(names):>5} names, {len(counts):>5} distinct')
    check(not duplicates, f'{label}: duplicate names {duplicates}')
    return counts


def main():
    if len(sys.argv) != 4:
        print(__doc__)
        return 2
    other_path, merge_path, candidate_path = sys.argv[1], sys.argv[2], sys.argv[3]

    print('Test conservation across the merge')
    print('=' * 34)
    print()
    print(f'other side  {other_path}')
    print(f'merge       {merge_path}')
    print(f'candidate   {candidate_path}')
    print()
    print("The other side's file is `WO-MOK-019`'s census at `e96648a`, the second merge of `master`")
    print('into that chain. `master` merged that tree as `3f47743`, which is this merge\'s second')
    print('parent, so it is the census of the tree this merge takes its other half from.')

    other = listed_names(other_path)
    merge, merge_outcomes = census_names(merge_path)
    candidate, candidate_outcomes = census_names(candidate_path)

    rule('1. The three sets')
    other_counts = report('WO-MOK-019 at e96648a  (--list)', other)
    merge_counts = report('this merge at ae2e44f  (census)', merge)
    candidate_counts = report('this candidate at 26ae6ba (census)', candidate)

    rule('2. The merge against the other side: nothing lost, one added')
    print('An arrival is explained only if it is this work order\'s own test. A departure is a defect')
    print('under rule 11 whatever explains it, so departures are listed in full rather than counted.')
    print()
    added = sorted((merge_counts - other_counts).elements())
    lost = sorted((other_counts - merge_counts).elements())
    print(f'    only on the merged tree   {len(added)}')
    for name in added:
        print(f'      + {name}')
    print(f'    only on the other side    {len(lost)}')
    for name in lost:
        print(f'      - {name}')
    check(not lost, f'{len(lost)} test(s) present on the other side and absent from the merge: {lost}')
    check(added == [ADDED], f'unexpected arrivals on the merged tree: {added}')

    rule('3. The merge against this branch\'s own candidate')
    print('The other direction of the same question: what the merge takes from `master` should be')
    print("`master`'s own tests and nothing else, and this branch's one addition should survive.")
    print()
    from_master = sorted((merge_counts - candidate_counts).elements())
    dropped = sorted((candidate_counts - merge_counts).elements())
    print(f'    arrived with master       {len(from_master)}')
    print(f'    lost from the candidate   {len(dropped)}')
    for name in dropped:
        print(f'      - {name}')
    check(not dropped,
          f'{len(dropped)} test(s) present at the candidate and absent from the merge: {dropped}')
    check(ADDED in merge_counts, f'this work order\'s own test is absent from the merged tree: {ADDED}')
    expected = len(other) + 1
    print()
    print(f'    {len(candidate)} at the candidate + {len(from_master)} arriving with master '
          f'= {len(merge)} on the merged tree')
    print(f'    {len(other)} on the other side + 1 added by this work order = {expected}')
    check(len(merge) == expected,
          f'the merged tree runs {len(merge)} tests where {expected} are accounted for')

    rule('4. Every case passed, and none is ignored')
    print('A census whose reader dropped a failing case would report a suite that passes by omission,')
    print('so the outcome column is counted rather than assumed. `analysis/test-census.py` records the')
    print('outcome it read; `test-run.txt` is the log it read it from.')
    print()
    for label, outcomes in (('this merge', merge_outcomes), ('this candidate', candidate_outcomes)):
        tally = collections.Counter(outcomes)
        print(f'    {label:<18}{dict(sorted(tally.items()))}')
        check(set(tally) == {'ok'}, f'{label}: outcomes other than ok: {dict(tally)}')

    rule('5. What rule 11 is owed, and it is not paid here')
    print('Rule 11\'s last approved figures are 301 for the workspace, as corrected for the second')
    print('merge of `master` into the `WO-MOK-019` chain, and 267 at the point this branch was cut.')
    print(f'The merged tree runs {len(merge)}. The correction is a paragraph in an approved')
    print('specification and belongs to the technical owner; `SPEC-MOK-004-rule-11.md` in this')
    print('directory drafts it and states what it would replace. This reader measures the figure and')
    print('does not write it.')

    print()
    if failures:
        print(f'RESULT: FAIL -- {len(failures)} check(s) failed:')
        for message in failures:
            print(f'  {message}')
        return 1
    print(f'RESULT: PASS -- {len(merge)} names on the merged tree, {len(lost)} lost, '
          f'{len(added)} added and accounted for')
    return 0


if __name__ == '__main__':
    sys.exit(main())
