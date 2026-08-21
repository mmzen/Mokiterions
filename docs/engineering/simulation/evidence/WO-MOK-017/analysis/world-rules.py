"""WO-MOK-017: the world rules the correction is required not to touch, compared line for line.

Usage, from the repository root:

    git show <pre-commit>:mokiterions-core/src/simulation.rs > <somewhere>/pre-simulation.rs
    python docs/engineering/simulation/evidence/WO-MOK-017/analysis/world-rules.py \
        <somewhere>/pre-simulation.rs mokiterions-core/src/simulation.rs \
        > docs/.../post/world-rules-unchanged.txt

Writes its report to stdout and exits `0` when every check passes, non-zero otherwise.

WHAT THIS FILE IS FOR
---------------------
`WO-MOK-017` retains "the line-for-line comparison showing rule 4, rule 9's eat effect, the food table
and rules 14 to 16 unchanged". Those four are named together because each is a way the correction could
have escaped its own scope, and each escape would break something different:

  * **Rule 4**, the baseline candidate list. `INT-MOK-010` promises `--policy baseline` is byte-identical
    across any change that does not alter the world, and rule 4 applies no waste condition at all. If the
    correction reached the candidate list, `post/byte-identity.txt`'s thirty passes would be luck.
  * **Rule 9's eat effect**, what consuming a resource actually does. `REQ-MOK-060` changes *whether* a
    resource is eaten, never what eating restores. If the effect moved, every satiety figure in
    `post/composition.txt` and `post/survivors.txt` would be measuring two changes at once.
  * **The food table**, the three restorations. It is the input to the corrected allowance
    (`R * R / 100`), so a change here would move the boundaries the new test asserts and make the
    specification's `87`, `79` and `75` agree with the engine for the wrong reason.
  * **Rules 14 to 16**, regeneration. The composition curve is the joint product of what regenerates and
    what is eaten. `REQ-MOK-060` is an argument that consumption alone explains the drift, and that
    argument requires regeneration held fixed.

Two things are shown rather than one. First that the four regions are identical, character for character,
which is a claim about them. Second that **every** changed line in the engine falls inside the three
functions of the non-waste condition, which is the stronger claim: it holds for the whole module, so it
covers rules this work order did not think to name.

WHY THE PRE-CHANGE SOURCE COMES IN AS A FILE
--------------------------------------------
This reader shells out to nothing and reads no git history. The pre-change text is extracted by the
caller and passed in, so the comparison is reproducible from two files a reader can hold, and so the
reader cannot quietly compare the working tree against itself.
"""

import difflib
import hashlib
import re
import sys
from pathlib import Path

# LF regardless of platform: `.gitattributes` pins this evidence tree `-text`, so a CRLF here
# would be committed as one.
sys.stdout.reconfigure(encoding='utf-8', newline='\n')

TEST_BOUNDARY = '#[cfg(test)]'

# The three functions of the non-waste condition -- the whole of what this work order is permitted
# to change in the engine.
# The three functions of the non-waste condition, each with whether it existed before this change.
CONDITION = (('fits_within', False), ('fits', True), ('fits_within_tolerance', True))

failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
    return condition


def engine_lines(path):
    """The module's lines ahead of `#[cfg(test)]`.

    The test module is excluded deliberately and its exclusion is reported: this work order does
    change tests, and three amendments plus one addition there are accounted for by name in
    `post/test-census-reconciliation.md`. Mixing them into this comparison would bury the claim
    that matters -- that the *engine* changed in one place -- under expected test churn.
    """
    lines = path.read_text(encoding='utf-8').splitlines()
    boundary = next(number for number, text in enumerate(lines, start=1) if text == TEST_BOUNDARY)
    return lines[:boundary - 1], boundary


def extent(lines, opener, terminator=None, with_docs=False):
    """The inclusive line range of the region a line matching `opener` begins.

    Without `terminator` the region is the brace-delimited block the opener starts. Rust is
    brace-delimited and the module is `rustfmt`-formatted, so the closing brace sits at the opener's
    own indentation; that is a formatting assumption, and a safe one here because `post/gates.txt`
    records `cargo fmt --check` passing on this candidate. With `terminator` the region runs from the
    opener to the line before the first later line matching it -- needed for rule 3's candidate list,
    which is a run of statements rather than a block and so has no brace of its own to close.

    `with_docs` extends the start upward over the item's contiguous `///` lines and attributes. A doc
    comment belongs to the item it documents, and the containment test in section 1 is wrong without
    it: the first draft of this reader placed the permitted extent at the `fn` line and reported five
    of seven changed blocks as escapes, when all five were the rewritten documentation of the three
    functions the work order is permitted to change.
    """
    pattern = re.compile(opener)
    for index in range(len(lines)):
        if not pattern.search(lines[index]):
            continue
        start = index + 1
        if with_docs:
            while start > 1:
                above = lines[start - 2].strip()
                if above.startswith('///') or above.startswith('#['):
                    start -= 1
                else:
                    break
        if terminator:
            stop = re.compile(terminator)
            for later in range(index + 1, len(lines)):
                if stop.search(lines[later]):
                    return start, later
            return start, len(lines)
        indent = len(lines[index]) - len(lines[index].lstrip())
        closer = ' ' * indent + '}'
        for later in range(index + 1, len(lines)):
            if lines[later] == closer:
                return start, later + 1
        return start, len(lines)
    return None


def digest(lines):
    return hashlib.sha256('\n'.join(lines).encode('utf-8')).hexdigest()


def rule(title):
    print()
    print(title)
    print('-' * len(title))


def show(lines, start):
    for offset, text in enumerate(lines):
        print(f'    {start + offset:>5}  {text}')


def main():
    pre_path, post_path = (Path(argument) for argument in sys.argv[1:3])
    pre, pre_boundary = engine_lines(pre_path)
    post, post_boundary = engine_lines(post_path)

    title = 'WO-MOK-017: the world rules the correction does not touch, compared line for line'
    print(title)
    print('=' * len(title))
    print()
    print('Work order   WO-MOK-017 (the resource composition drift)')
    print('Retains      "the line-for-line comparison showing rule 4, rule 9\'s eat effect, the food')
    print('             table and rules 14 to 16 unchanged"')
    print('Pre-change   pre/COMMIT.txt      Candidate  post/COMMIT.txt')
    print('Reader       analysis/world-rules.py')
    print('Date         2026-08-21')
    print()
    print(f'    pre-change engine lines, ahead of `{TEST_BOUNDARY}` at line {pre_boundary}'
          f'{"":<3}{len(pre):>6}')
    print(f'    candidate engine lines, ahead of it at line {post_boundary}{"":<15}{len(post):>6}')
    print(f'    pre-change engine SHA-256   {digest(pre)}')
    print(f'    candidate  engine SHA-256   {digest(post)}')
    print()
    print('The two digests differ, which they must -- this work order changes the engine. What follows is')
    print('where, and where not. The test module is excluded from every comparison here and accounted for')
    print('separately in `post/test-census-reconciliation.md`, so that expected test churn cannot bury')
    print('the claim that the engine moved in exactly one place.')

    # ------------------------------------------------------------------ 1
    rule('1. Every changed engine line, and the one place they all sit')
    print('This is the general claim, and it is made first because the four named rules are corollaries')
    print('of it. The two engine texts are aligned by `difflib.SequenceMatcher` and every non-equal')
    print('block is listed. Each is then tested for containment in the non-waste condition family.')
    print()
    permitted = {}
    for name, existed in CONDITION:
        spans = []
        for label, lines, expected in (('pre-change', pre, existed), ('candidate', post, True)):
            span = extent(lines, rf'^\s*fn {name}\(', with_docs=True)
            check((span is not None) == expected,
                  f'fn {name} is {"absent from" if expected else "already in"} '
                  f'the {label} engine')
            spans.append(span)
        permitted[name] = tuple(spans)
    print(f'    {"the only functions this work order may change":<44}'
          f'{"pre-change":>13}{"candidate":>13}')
    for name, (span_pre, span_post) in permitted.items():
        print(f'    Observation::{name:<31}'
              f'{(f"{span_pre[0]}-{span_pre[1]}" if span_pre else "new here"):>13}'
              f'{(f"{span_post[0]}-{span_post[1]}" if span_post else "GONE"):>13}')
    print()
    print('    `fits_within` is new at the candidate, so it has no pre-change extent, and that absence is')
    print('    asserted rather than tolerated: `REQ-MOK-060` introduced it, so finding it in the')
    print('    pre-change text would mean the two sources are not the two commits they claim to be.')
    print('    Each extent includes the function\'s own documentation, because a doc comment belongs to')
    print('    the item it documents and all three were rewritten with the arithmetic they describe.')
    print('    Containment is tested against the union of the three rather than against one at a time: a')
    print('    single aligned block can straddle two of them, and it does here, where `fits_within` was')
    print('    inserted immediately above `fits`. It is also tested on both sides, so a world rule')
    print('    deleted inside a `replace` block cannot pass by having an innocent candidate range.')
    print()
    pre_union = {number for span_pre, _ in permitted.values() if span_pre
                 for number in range(span_pre[0], span_pre[1] + 1)}
    post_union = {number for _, span_post in permitted.values() if span_post
                  for number in range(span_post[0], span_post[1] + 1)}
    blocks = [opcode for opcode in difflib.SequenceMatcher(None, pre, post).get_opcodes()
              if opcode[0] != 'equal']
    print(f'    {"kind":<9}{"pre-change lines":>18}{"candidate lines":>17}   inside')
    outside = []
    for kind, pre_start, pre_end, post_start, post_end in blocks:
        pre_range = set(range(pre_start + 1, pre_end + 1))
        post_range = set(range(post_start + 1, post_end + 1))
        # Blank lines are excluded from the containment test. The blank separating `fits_within`
        # from `fits` belongs to neither function's extent, so a block spanning both would read as
        # an escape over one empty line. A blank line is not a world rule.
        contained = ({number for number in pre_range if pre[number - 1].strip()} <= pre_union
                     and {number for number in post_range if post[number - 1].strip()} <= post_union)
        holders = sorted(
            (name for name, (span_pre, span_post) in permitted.items()
             if (span_pre and pre_range & set(range(span_pre[0], span_pre[1] + 1)))
             or (span_post and post_range & set(range(span_post[0], span_post[1] + 1)))),
            key=lambda name: permitted[name][1][0])
        if not contained:
            outside.append((kind, pre_start + 1, pre_end, post_start + 1, post_end))
        print(f'    {kind:<9}{(f"{pre_start + 1}-{pre_end}" if pre_range else "-"):>18}'
              f'{(f"{post_start + 1}-{post_end}" if post_range else "-"):>17}   '
              f'{", ".join(holders) if contained else "*** OUTSIDE ***"}')
    check(not outside, f'{len(outside)} changed engine block(s) outside the condition family')
    print()
    print(f'    changed blocks in the engine                                {len(blocks)}')
    print(f'    changed blocks outside the non-waste condition              {len(outside)}')
    if outside:
        print('    THE CHANGE ESCAPED ITS SCOPE. Nothing below can be relied on until this is resolved.')
    else:
        print('    **Zero.** Every line the engine gained, lost or altered lies inside `fits_within`,')
        print('    `fits` or `fits_within_tolerance`. So the four rules named in the retention item are')
        print('    unchanged, and so is every other rule in the module -- including any this work order')
        print('    failed to think of. The named four are still shown individually below, because a')
        print('    containment argument is only as trustworthy as its alignment and a reader is entitled')
        print('    to see the code itself.')

    # ------------------------------------------------------------------ 2-5
    regions = [
        ('2. Rule 4: the baseline candidate list, and the source that draws from it',
         [('Simulation::observation, where rule 3 assembles the candidate list',
           r'^\s*let mut valid_actions = vec!\[Action::Wait\];', r'^\s*Observation \{'),
          ('BaselineDecisionSource::decide, rule 4 itself',
           r'^impl DecisionSource for BaselineDecisionSource', None)],
         ['Rule 4 applies no waste condition, so `fits` is not reachable from it. That is what makes',
          '`INT-MOK-010`\'s byte-identity promise for `--policy baseline` survivable across a change to',
          'the waste arithmetic, and `post/byte-identity.txt` measures the promise kept in all thirty',
          'baseline cells. This section is the reason it was kept rather than the evidence that it was.']),
        ('3. Rule 9: what consuming a resource does',
         [('Simulation::apply_action, the eat arm',
           r'^\s*Action::Eat \{ food_id \} => \{', None)],
         ['`REQ-MOK-060` changes whether a resource is eaten and never what eating restores. The clip is',
          'still here and still unconditional -- `saturating_add` then `min(ATTRIBUTE_MAX)`, on satiety',
          'and on energy alike, printed below inside the identical region. That is the whole relationship',
          'between this rule and the correction: the corrected condition decides that a resource whose',
          'restoration will be clipped is worth taking anyway, and this code goes on clipping it. The',
          'waste `REQ-MOK-060` tolerates is real waste, not waste the effect was quietly changed to',
          'avoid. Nothing here needed to change for the correction to work, and nothing did.']),
        ('4. The food table',
         [('FoodClass::restoration', r'^\s*fn restoration\(self\) -> \(u8, u8\)', None)],
         ['This table is the input to the corrected allowance. `R * R / 100` is `2`, `9` and `25` for the',
          'three classes because `R` is `15`, `30` and `50` here, so the specification\'s boundaries of',
          '`87`, `79` and `75` are derived from these six numbers. Had the table moved, the engine and',
          'the specification could agree while both were wrong.']),
        ('5. Rules 14 to 16: regeneration',
         [('Simulation::regenerate_food', r'^\s*fn regenerate_food<W: Write>\(', None)],
         ['The composition curve in `post/composition.txt` is the joint product of what regenerates and',
          'what is eaten, and `REQ-MOK-060` is the claim that consumption alone explains the drift. That',
          'claim is only testable with regeneration held fixed -- including rule 16\'s uniform class',
          'selection, which is what would otherwise be suspected of favouring high class.']),
    ]
    for heading, parts, prose in regions:
        rule(heading)
        for line in prose:
            print(line)
        for label, opener, terminator in parts:
            span_post = extent(post, opener, terminator)
            span_pre = extent(pre, opener, terminator)
            if not check(span_post and span_pre, f'{label}: region not found in both texts'):
                print()
                print(f'    {label}: NOT FOUND')
                continue
            pre_text = pre[span_pre[0] - 1:span_pre[1]]
            post_text = post[span_post[0] - 1:span_post[1]]
            identical = pre_text == post_text
            check(identical, f'{label} differs between the two commits')
            print()
            print(f'    {label}')
            print(f'      pre-change lines {span_pre[0]}-{span_pre[1]}, '
                  f'candidate lines {span_post[0]}-{span_post[1]}, '
                  f'{len(post_text)} lines')
            print(f'      SHA-256 pre-change  {digest(pre_text)}')
            print(f'      SHA-256 candidate   {digest(post_text)}')
            print(f'      line for line       {"IDENTICAL" if identical else "*** DIFFERS ***"}')
            print()
            if identical:
                show(post_text, span_post[0])
            else:
                for line in difflib.unified_diff(pre_text, post_text, 'pre-change', 'candidate',
                                                 lineterm='', n=2):
                    print(f'    {line}')

    # ------------------------------------------------------------------ 6
    rule('6. What this comparison does not settle')
    print('  * That the engine changed in one place is not that the change is right. The condition\'s own')
    print('    correctness is `post/divergence.txt` and the suite\'s new boundary test, not this file.')
    print('  * The regions above shift by a line count between the two texts, because the condition')
    print('    family gained lines ahead of them. Shifted is not changed, and the digests are computed')
    print('    on the text rather than on the line numbers, so a pure move reads as identical here -- as')
    print('    it should.')
    print('  * The test module is excluded throughout. Its changes are real and are reconciled name by')
    print('    name in `post/test-census-reconciliation.md`, with the two consequential TUI fixes in')
    print('    `post/updated-tests.md`.')
    print('  * Two crates outside this module changed and are not compared here: `mokiterions-tui`\'s')
    print('    state and render tests, both of which are test-side and both accounted for in')
    print('    `post/updated-tests.md`. No `mokiterions-tui` source rule is a world rule.')

    print()
    if failures:
        print(f'RESULT: FAIL -- {len(failures)} check(s) failed:')
        for failure in failures:
            print(f'  {failure}')
    else:
        print('RESULT: PASS -- every changed line in the engine lies inside the three functions of the')
        print('non-waste condition, and rule 4, rule 9\'s eat effect, the food table and rules 14 to 16')
        print('are identical between the two commits, character for character.')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
