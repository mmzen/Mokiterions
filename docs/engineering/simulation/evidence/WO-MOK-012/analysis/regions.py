"""Extract named regions of `mokiterions-core/src/simulation.rs` and digest them.

`VER-MOK-012`'s `REQ-MOK-051` row asks for a comparison "line for line against the
pre-change commit" showing that rule 4, rule 9's eat effect, the food table and rules 14
to 16 are unchanged. A `git diff` cannot answer that on its own: the file gains roughly
2,900 lines between the two commits, so every region in it moves, and a diff of the whole
file reports the movement rather than the question. What is wanted is each named region
lifted out by name at both commits and compared as text.

That is what this script does. A region is named by an anchor -- the exact source line that
opens it -- and is extracted either as a brace-balanced item (`item`), or as the span from
one anchor up to but not including a second (`span`). Neither form uses a line number, so
the same region definitions apply to both commits even though every line has moved.

The comparison is on the extracted text, and it is deliberately not normalized: no comment
stripping, no whitespace folding, no reformatting. "Unchanged" here means the bytes are the
same bytes, which is the strongest reading of the obligation and the only one that needs no
argument about what the normalization discarded.

Usage:

    git show <rev>:mokiterions-core/src/simulation.rs > a.rs
    git show <rev2>:mokiterions-core/src/simulation.rs > b.rs
    python regions.py a.rs b.rs

Exits 0 when every region marked `expect_same` is identical across the two files and every
region marked `expect_differs` differs, 1 otherwise. The expectations are part of the
evidence: a region that changed when it was required not to, and a region that did not
change when the work order says it did, are both findings, and a script that only printed
digests would leave the reader to notice.
"""

from __future__ import annotations

import hashlib
import sys

# name, kind, anchor, closing anchor (span only), expectation
#
# Expectations are `same` for the four regions VER-MOK-012 requires to be unchanged, and
# for the two waste conditions -- those two because REQ-MOK-051 is unimplemented under the
# product owner's approved deferral of 2026-08-20, so the correction that would have
# touched them has not been written. `differs` marks the regions this work order does
# change, and they are listed so that the run proves the extractor can tell the two apart:
# a script reporting "all same" is worth nothing if it would report that either way.
REGIONS = [
    # Rule 4, the baseline decision source.
    ("rule 4: BaselineDecisionSource", "item", "struct BaselineDecisionSource;", None, "same"),
    (
        "rule 4: its DecisionSource impl",
        "item",
        "impl DecisionSource for BaselineDecisionSource {",
        None,
        "same",
    ),
    # The food table: the class enum, its two tables, its display, and the record.
    ("food table: FoodClass", "item", "pub enum FoodClass {", None, "same"),
    ("food table: impl FoodClass", "item", "impl FoodClass {", None, "same"),
    ("food table: Display for FoodClass", "item", "impl fmt::Display for FoodClass {", None, "same"),
    ("food table: the Food record", "item", "struct Food {", None, "same"),
    # Rule 9's eat effect: the whole `Action::Eat` arm of `apply_action`, which is where the
    # resource is removed and the class values are restored under the attribute maximum.
    ("rule 9 eat effect: the Action::Eat arm", "item", "            Action::Eat { food_id } => {", None, "same"),
    # Rules 14 to 16, regeneration: the two constants, the skip reason, the opportunity
    # itself, and the per-class count it reads.
    (
        "rules 14 to 16: constants",
        "span",
        "const REGENERATION_INTERVAL: u64 = 10;",
        "const SATIETY_DECAY: u8 = 1;",
        "same",
    ),
    ("rules 14 to 16: RegenerationSkipReason", "item", "pub enum RegenerationSkipReason {", None, "same"),
    (
        "rules 14 to 16: Display for RegenerationSkipReason",
        "item",
        "impl fmt::Display for RegenerationSkipReason {",
        None,
        "same",
    ),
    # Rule 14 is a timing rule, so the call site is part of the region and not only the
    # function it calls: "after all scheduled agents are processed, on ticks divisible by 10".
    (
        "rules 14 to 16: the regeneration call site",
        "item",
        "        if self.tick.is_multiple_of(REGENERATION_INTERVAL) {",
        None,
        "same",
    ),
    ("rules 14 to 16: regenerate_food", "item", "    fn regenerate_food<W: Write>(", None, "same"),
    ("rules 14 to 16: food_counts", "item", "    fn food_counts(&self, territory: Territory) -> [usize; 3] {", None, "same"),
    # Rule 5's waste condition and the two selectors that apply it.
    ("rule 5 waste condition: fits", "item", "    fn fits(&self, food: &PerceivedFood) -> bool {", None, "same"),
    (
        "rule 5 waste condition: best_fitting_co_located_food",
        "item",
        "    fn best_fitting_co_located_food(&self) -> Option<&PerceivedFood> {",
        None,
        "same",
    ),
    (
        "rule 5 waste condition: best_fitting_distant_food",
        "item",
        "    fn best_fitting_distant_food(&self) -> Option<&PerceivedFood> {",
        None,
        "same",
    ),
    # Rule 19's waste condition, the tolerance it reads, and its two selectors.
    ("rule 19 waste condition: WASTE_TOLERANCE_MAX", "item", "const WASTE_TOLERANCE_MAX: u8 = 40;", None, "same"),
    (
        "rule 19 waste condition: derive_waste_tolerance",
        "item",
        "fn derive_waste_tolerance(seed: u64, number: u8) -> u8 {",
        None,
        "same",
    ),
    (
        "rule 19 waste condition: fits_within_tolerance",
        "item",
        "    fn fits_within_tolerance(&self, food: &PerceivedFood) -> bool {",
        None,
        "same",
    ),
    (
        "rule 19 waste condition: best_tolerated_co_located_food",
        "item",
        "    fn best_tolerated_co_located_food(&self) -> Option<&PerceivedFood> {",
        None,
        "same",
    ),
    (
        "rule 19 waste condition: best_tolerated_distant_food",
        "item",
        "    fn best_tolerated_distant_food(&self) -> Option<&PerceivedFood> {",
        None,
        "same",
    ),
    # Controls: regions this work order does change, so that a run showing "same"
    # everywhere else is a measurement and not a broken extractor.
    ("control: Action", "item", "pub enum Action {", None, "differs"),
    ("control: EventType", "item", "pub enum EventType {", None, "differs"),
    ("control: IndividualDecisionSource impl", "item", "impl DecisionSource for IndividualDecisionSource {", None, "differs"),
]


def find_anchor(lines: list[str], anchor: str) -> int:
    """The index of the one line equal to `anchor` after stripping the trailing newline.

    Exact equality, and exactly one match required. An anchor that matches twice or not at
    all is a defect in this script's region table rather than a finding about the source,
    and it stops the run instead of silently extracting the wrong span.
    """
    hits = [i for i, line in enumerate(lines) if line.rstrip("\n") == anchor]
    if len(hits) != 1:
        raise SystemExit(f"anchor {anchor!r} matched {len(hits)} lines, expected exactly 1")
    return hits[0]


def extract_item(lines: list[str], anchor: str) -> tuple[int, int]:
    """The half-open line range of the brace-balanced item opening at `anchor`.

    An anchor ending in `;` is a whole item on one line -- a `const` declaration, a unit
    struct -- and is the anchor line alone. Otherwise the item has a body, and the range runs
    from the anchor to the line where the brace depth returns to zero. The opening brace need
    not be on the anchor line: a multi-line `fn` signature is anchored on its `fn` line and
    the search for the first brace runs forward from there, so `fn regenerate_food<W: Write>(`
    yields the whole function rather than its first line.

    Brace counting ignores braces inside string literals and character literals, which
    `format!` lines in this file are full of. It does not attempt to parse Rust; it does not
    need to, because every region here is a top-level or one-level-nested item whose body is
    ordinary code.
    """
    start = find_anchor(lines, anchor)
    if lines[start].rstrip().endswith(";"):
        return start, start + 1

    depth = 0
    opened = False
    index = start
    while index < len(lines):
        depth += count_braces(lines[index])
        opened = opened or depth > 0
        if opened and depth == 0:
            return start, index + 1
        index += 1
    raise SystemExit(f"item at {anchor!r} never closes")


def count_braces(line: str) -> int:
    """Net brace depth of one line, skipping string and character literals."""
    depth = 0
    index = 0
    while index < len(line):
        char = line[index]
        if char == "\\":
            index += 2
            continue
        if char in "\"'":
            index = skip_literal(line, index)
            continue
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
        index += 1
    return depth


def skip_literal(line: str, index: int) -> int:
    """The index just past the literal opening at `index`, or past `index` if unterminated.

    An unterminated literal means a lifetime tick (`&'a str`) or a line-ending raw string,
    neither of which carries a brace this counter would miss, so advancing one character is
    the right recovery.
    """
    quote = line[index]
    cursor = index + 1
    while cursor < len(line):
        if line[cursor] == "\\":
            cursor += 2
            continue
        if line[cursor] == quote:
            return cursor + 1
        cursor += 1
    return index + 1


def extract_span(lines: list[str], anchor: str, closing: str) -> tuple[int, int]:
    """The half-open line range from `anchor` up to but not including `closing`."""
    start = find_anchor(lines, anchor)
    end = find_anchor(lines, closing)
    if end <= start:
        raise SystemExit(f"span {anchor!r} .. {closing!r} runs backwards")
    return start, end


def region_text(lines: list[str], kind: str, anchor: str, closing: str | None) -> tuple[str, int, int]:
    start, end = extract_item(lines, anchor) if kind == "item" else extract_span(lines, anchor, closing)
    return "".join(lines[start:end]), start + 1, end


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        raise SystemExit("usage: regions.py <old simulation.rs> <new simulation.rs>")

    with open(argv[1], encoding="utf-8", newline="") as handle:
        old = handle.readlines()
    with open(argv[2], encoding="utf-8", newline="") as handle:
        new = handle.readlines()

    print(f"# old {argv[1]}: {len(old)} lines")
    print(f"# new {argv[2]}: {len(new)} lines")
    print()
    print(f"{'region':52} {'old lines':>12} {'new lines':>12} {'bytes':>7} {'sha256':>10}  verdict")

    failures = 0
    for name, kind, anchor, closing, expectation in REGIONS:
        old_text, old_start, old_end = region_text(old, kind, anchor, closing)
        new_text, new_start, new_end = region_text(new, kind, anchor, closing)
        same = old_text == new_text
        digest = hashlib.sha256(new_text.encode("utf-8")).hexdigest()[:10]
        verdict = "same" if same else "differs"
        ok = verdict == expectation
        failures += not ok
        print(
            f"{name:52} {f'{old_start}-{old_end}':>12} {f'{new_start}-{new_end}':>12} "
            f"{len(new_text.encode('utf-8')):>7} {digest:>10}  {verdict}"
            f"{'' if ok else f'  <- EXPECTED {expectation.upper()}'}"
        )

    print()
    same_count = sum(1 for region in REGIONS if region[4] == "same")
    print(f"{same_count} regions required unchanged, {len(REGIONS) - same_count} controls required changed")
    print(f"regions not meeting their expectation: {failures}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
