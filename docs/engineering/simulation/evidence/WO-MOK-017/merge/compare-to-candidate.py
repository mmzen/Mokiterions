"""Does the merge of `master` produce the candidate's output, cell for cell?

The whole of `WO-MOK-017`'s evidence is read from one 120-cell capture of the candidate commit
`26ae6ba`. The merge commit is a different tree -- `master` rewrote about two thousand lines of
`mokiterions-core/src/simulation.rs` for the structured record stream -- so every figure in the packet
is a figure about a tree that is no longer the branch tip. This reader is the licence to carry them
forward, and it is a measurement rather than an argument: if the merged tree emits the same 120
streams byte for byte, then every quantity read out of those streams is the same quantity, and no
reader that consumes them needs to be re-run to know its answer.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-017/merge/compare-to-candidate.py \
        docs/engineering/simulation/evidence/WO-MOK-017/post/post-manifest.txt \
        docs/engineering/simulation/evidence/WO-MOK-017/merge/merge-manifest.txt

Both manifests are `analysis/manifest.py`'s output on a capture taken from its own commit's archived
tree, so the comparison is a statement about two commits and not about two working directories.
"""
import pathlib
import sys

sys.stdout.reconfigure(encoding='utf-8', newline='\n')

SOURCES = ('baseline', 'reference', 'individual', 'social')

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


def rule(title):
    print()
    print(title)
    print('-' * len(title))


def read(path):
    """The manifest's description line and its rows, keyed by cell name."""
    lines = pathlib.Path(path).read_text(encoding='utf-8').splitlines()
    description = lines[0].lstrip('# ').strip()
    rows = {}
    for line in lines:
        if line.startswith('#') or not line.strip() or line.startswith(' '):
            continue
        fields = line.split()
        if len(fields) != 5:
            # `manifest.py` closes with its own count lines -- `cells: 120` and
            # `exit codes observed: [...]`. They are the reader's summary of the rows above and
            # are re-derived here from the rows themselves rather than parsed.
            continue
        cell, sha, size, count, code = fields
        rows[cell] = (sha, int(size), int(count), int(code))
    return description, rows


def source_of(cell):
    for source in SOURCES:
        if f'-{source}-' in cell:
            return source
    return 'UNKNOWN'


def main():
    if len(sys.argv) != 3:
        print(__doc__)
        return 2
    candidate_path, merge_path = sys.argv[1], sys.argv[2]
    candidate_description, candidate = read(candidate_path)
    merge_description, merge = read(merge_path)

    print('The merged tree measured against the candidate, cell for cell')
    print('=' * 60)
    print()
    print(f'candidate manifest  {candidate_path}')
    print(f'                    {candidate_description}')
    print(f'merge manifest      {merge_path}')
    print(f'                    {merge_description}')

    rule('1. The two captures cover the same matrix')
    print('A comparison over an unequal cell set would report agreement on the intersection and say')
    print('nothing about the difference, so the sets are compared before the contents are.')
    print()
    print(f'    candidate cells   {len(candidate)}')
    print(f'    merge cells       {len(merge)}')
    only_candidate = sorted(set(candidate) - set(merge))
    only_merge = sorted(set(merge) - set(candidate))
    print(f'    only in candidate {len(only_candidate)}')
    print(f'    only in merge     {len(only_merge)}')
    check(len(candidate) == 120, f'the candidate manifest has {len(candidate)} cells, not 120')
    check(len(merge) == 120, f'the merge manifest has {len(merge)} cells, not 120')
    check(not only_candidate, f'cells absent from the merge capture: {only_candidate}')
    check(not only_merge, f'cells absent from the candidate capture: {only_merge}')

    rule('2. Every cell, on all four columns')
    print('Each row is the raw SHA-256 of the stream, its byte count, its line count and the exit code')
    print('of the run that produced it. The digest alone would settle the question; the other three are')
    print('kept because a digest mismatch says only that something moved, and the columns say what.')
    print()
    differing = []
    for cell in sorted(candidate):
        if cell not in merge:
            continue
        if candidate[cell] != merge[cell]:
            differing.append(cell)
    per_source = {source: [0, 0] for source in SOURCES}
    for cell in sorted(candidate):
        if cell not in merge:
            continue
        entry = per_source[source_of(cell)]
        entry[0] += 1
        if candidate[cell] == merge[cell]:
            entry[1] += 1
    print(f'    {"source":<12}{"cells":>7}{"identical":>11}{"differing":>11}')
    for source in SOURCES:
        total, same = per_source[source]
        print(f'    {source:<12}{total:>7}{same:>11}{total - same:>11}')
    total = sum(entry[0] for entry in per_source.values())
    same = sum(entry[1] for entry in per_source.values())
    print(f'    {"all":<12}{total:>7}{same:>11}{total - same:>11}')
    check(not differing, f'{len(differing)} cell(s) differ between the candidate and the merge')
    if differing:
        print()
        print('    the differing rows, candidate above merge:')
        for cell in differing:
            print(f'      {cell}')
            print(f'        candidate  {candidate[cell]}')
            print(f'        merge      {merge[cell]}')

    rule('3. What this licenses')
    print('Every capture-derived figure in this packet is read from these 120 streams, so each of them')
    print('is a figure at the merge commit as well as at the candidate:')
    print()
    print('  * `post/composition.txt`  -- `REQ-MOK-060` at three fifths, worst 54.1% of 61 on 30 of 30')
    print('    territory evaluations, and the one-half comparison beside it.')
    print('  * `post/survivors.txt`    -- `REQ-MOK-014` 8, `REQ-MOK-034` 8 and `REQ-MOK-058` 5, at')
    print('    margins 0, 0 and 2, with combat deaths per seed unchanged.')
    print('  * `post/byte-identity.txt` and `post/divergence.txt` -- the 30 identical `baseline` cells')
    print('    and the 90 obligated divergences with 0 unattributed.')
    print('  * `post/entropy.txt`      -- no entropy consumed by the correction.')
    print('  * `post/health-falls.txt`, `post/dead-neighbours.txt` -- the two observer-side test')
    print('    amendments measured from the streams.')
    print('  * `{pre,post}/{init/,full/,summary.txt,census.txt,eaten.txt}` -- the retained subset, which')
    print('    is a projection of these streams and moves only if they move.')
    print()
    print('None of those readers is re-run here, and re-running them would measure this same equality a')
    print('second time rather than measure anything new.')

    rule('4. What this does not license')
    print('  * **The two source-side readers.** `analysis/world-rules.py` and `analysis/reads.py` read')
    print('    `mokiterions-core/src/simulation.rs` rather than a capture, and `master` changed that')
    print('    file. Both are re-run on the merged source and both print `RESULT: FAIL`;')
    print('    `world-rules-merged.txt` and `reads-merged.md` are those runs in full, and `gates.txt`')
    print('    says what each failure is and why neither is a world rule moving.')
    print('  * **`post/docs-figures.txt` in part.** Its capture-derived figures carry forward with the')
    print('    streams above; the seven out-of-matrix runs in its section 10 are not re-taken here, and')
    print('    neither is the pass over `SIMULATION_RULES.md` and `docs/ROADMAP.md`, both of which this')
    print('    merge edits in their evidence-directory list and their Phase 3 blocks. What moved there')
    print('    is prose and a table of work-order identifiers, not a figure.')
    print('  * **The record stream `master` added.** `capture.sh` never passes `--events-path`, so these')
    print('    120 cells exercise the default path and say nothing about the sink. That is deliberate:')
    print('    it is what makes the equality above the right measurement, and `SPEC-MOK-006`\'s own')
    print('    obligation -- that configuring a sink leaves standard output byte-identical -- is')
    print('    corroborated by this table from the other side. `VREC-MOK-019` is where the stream itself')
    print('    is verified.')
    print('  * **Anything about `master`\'s own tree.** This compares the merge against this branch\'s')
    print('    candidate. It does not measure what `master` emitted before the merge, and it is not')
    print('    evidence about `WO-MOK-018` or `WO-MOK-019`.')

    print()
    if failures:
        print(f'RESULT: FAIL -- {len(failures)} check(s) failed:')
        for message in failures:
            print(f'  {message}')
        return 1
    print(f'RESULT: PASS -- {total} of {total} cells identical on all four columns, '
          f'{len(differing)} differing')
    return 0


if __name__ == '__main__':
    sys.exit(main())
