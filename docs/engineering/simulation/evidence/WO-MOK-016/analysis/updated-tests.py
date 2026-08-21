"""Name every test this change updated, with its assertion count before and after.

`WO-MOK-016`'s *Completion report format* item 13 asks for the census reconciliation "with
every updated test named and its assertion count before and after". The census answers half
of that: `post/test-census-reconciliation.md` reconciles the two name lists exactly, and a
name list cannot show that a test which kept its name changed what it asserts. This script
answers the other half, and it answers a stop condition with it -- condition 8, that no
existing test's assertions may be relaxed, widened, removed, renamed away or `#[ignore]`d.
A test whose assertion count *fell* is the visible form of that condition being reached, so
the count is compared in both directions and a fall is a failure here rather than a note.

What is parsed, and what "assertion count" counts:

  * A test is a `#[test]`-attributed `fn`, taken with its brace-balanced body. Attributes
    between `#[test]` and the signature -- `#[should_panic]`, `#[ignore]` -- are carried, so
    an `#[ignore]` added to an existing test appears here as a changed body and is also
    counted on its own.
  * An assertion is an invocation of `assert!`, `assert_eq!`, `assert_ne!`, their three
    `debug_assert` forms, `panic!` or `unreachable!`, counted after comments are stripped.
    `unwrap()`, `expect()` and `?` are *not* counted: each aborts the test, so each is an
    assertion in effect, but they appear in setup as often as in checking and counting them
    would make the figure a measure of style. The figure here is deliberately the narrow one,
    and the body's non-comment line count is printed beside it so that a test which moved
    work between the two is visible.
  * Comment stripping tracks string literals, so a `//` inside a string is not a comment and
    an `assert!` inside one is not an assertion.
  * Files are read with `Path.read_text`, so `\r\n` becomes `\n` before anything is compared.
    That is deliberate rather than incidental: this repository's working tree carries the
    Rust sources with CRLF endings and an extraction on another platform will not, and a
    figure for item 13 that changed with the reader's operating system would be worthless.
    `post/updated-tests.txt`'s control E measures it -- the same comparison against an
    LF-converted candidate produces byte-identical output.

The comparison is between two extracted trees rather than between two revisions of one file,
because 20 files carry tests and every one of them moves. Extract each side with
`git archive`, which is what `post/capture-state.txt` uses for the captures' provenance:

    mkdir base cand
    git archive 39662d13 | tar -x -C base
    git archive 13906153 | tar -x -C cand
    python updated-tests.py base cand [baseline|master|renumber]

`13906153` is this branch's amended candidate, whose non-documentation tree is byte-identical
to `59d61b9`'s -- `git diff --name-only 59d61b9 13906153` names no file outside `docs/`, so
either revision extracts the same suite.

The third argument names which pair of trees is being read, because what a run reconciles
against differs between them:

  * `baseline` -- `39662d13` against `13906153`, the comparison item 13 asks for.
  * `master` -- `master`'s tip `d8e2079` against the merge `259859d`. This one says something
    the baseline pair cannot: `WO-MOK-013` is present on both sides, so anything of master's
    own cancels, and the same nine names with no tenth means no test of master's was absorbed
    into this work order's list.
  * `renumber` -- `13906153` against `d72465c`, the commit that moved this chain out of four
    occupied identifier ranges. Seven bodies change, no count does, and no name does. It is
    the reason the two candidate trees above disagree on which identifier two of the authority
    tests assert while agreeing on every figure in this file.

Exits 0 when every check passes and no retained test lost an assertion, 1 otherwise. The
totals are checked against `post/test-census-reconciliation.md`'s own figures, which are read
from a cargo test log rather than from source, so the two disagreeing would mean this parser
is wrong about what the suite is. `post/updated-tests.txt` retains all three runs and five
controls -- three of the controls must fail, and do.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8", newline="\n")

SOURCE_ROOTS = [
    "mokiterions-core/src",
    "mokiterions-core/tests",
    "mokiterions-tui/src",
    "mokiterions-tui/tests",
]

ASSERTION = re.compile(
    r"\b(?:debug_)?assert(?:_eq|_ne)?!|\bpanic!|\bunreachable!"
)

# post/test-census-reconciliation.md section 2's table, target by target, as it stands: the
# baseline at 39662d13 against the 249-name census at 7c4aef39. This parser reads source and
# that table was read from a cargo test log, so the two are independent instruments and are
# compared here rather than assumed to agree. The candidate column is expected to differ by
# section 8's five-line step -- one viability test became two -- and by nothing else.
CENSUS_SECTION_2 = {
    "unittests": (93, 121),
    "tests/cli.rs": (13, 15),
    "tests/decisions.rs": (1, 3),
    "tests/viability.rs": (2, 4),
    "tests/process.rs": (6, 7),
    "tests/termination.rs": (4, 5),
    "tests/verification.rs": (19, 20),
    "tests/authority.rs": (4, 4),
    "tests/density.rs": (2, 2),
    "tests/export.rs": (7, 7),
    "tests/layout.rs": (10, 10),
    "tests/naming.rs": (3, 3),
    "tests/options.rs": (8, 8),
    "tests/render.rs": (12, 12),
    "tests/spatial.rs": (7, 7),
    "tests/state.rs": (21, 21),
}

# The one target where this candidate is expected to exceed that table, and by how much.
CENSUS_STEP = {"tests/viability.rs": 1}

# The nine retained tests this work order updates, as the baseline run below measures them.
# For that run this constant is a tripwire and nothing more: it repeats what the run just
# read. It earns its place on the `master` run, where the before side already carries
# `WO-MOK-013` -- there, anything of master's own is on both sides and cancels, so the updated
# set must come out as these same nine names. A tenth, or one of these nine missing, would
# mean the merge resolution absorbed a test of master's into this work order's list.
EXPECTED_UPDATED = [
    "mokiterions-core/src/simulation.rs :: a_name_is_the_same_value_at_both_ends_of_a_run",
    "mokiterions-core/src/simulation.rs :: naming_draws_nothing_and_reads_neither_the_seed_nor_the_configuration",
    "mokiterions-core/src/simulation.rs :: the_trait_is_fixed_for_the_run_and_independent_of_every_configuration",
    "mokiterions-core/tests/cli.rs :: the_entries_state_the_constraints_that_decide_validity",
    "mokiterions-core/tests/naming.rs :: every_run_reports_the_specified_twelve_names_in_identifier_order",
    "mokiterions-tui/tests/authority.rs :: every_event_type_the_observer_can_present_has_an_entry",
    "mokiterions-tui/tests/authority.rs :: the_decision_source_maps_by_the_source_the_record_names",
    "mokiterions-tui/tests/authority.rs :: the_mapping_is_the_specified_one",
    "mokiterions-tui/tests/options.rs :: the_usage_text_advertises_every_policy_the_engine_accepts",
]

# The seven bodies the renumber commit `d72465c` rewrites. It moved this chain out of four
# occupied identifier ranges and touched nothing else, so every one of these is an identifier
# substitution: `REQ-MOK-048` to `REQ-MOK-057`, `049` to `058`, `044/046/047` to `053/055/056`,
# `043` to `052`, `VER-MOK-012` to `VER-MOK-016` and `WO-MOK-012` to `WO-MOK-016`. Four of the
# seven are comments and doc comments, three carry the substitution inside an assertion or a
# `println!`. No count moves, which makes this pair the clearest illustration of what section 4
# discloses: a changed body at an unchanged count is where a weakening could hide, and here it
# does not.
RENUMBER_UPDATED = [
    "mokiterions-core/src/simulation.rs :: a_threat_composes_with_rule_12_in_turn_order_and_outlasts_its_tick",
    "mokiterions-core/tests/decisions.rs :: every_targeted_verb_applies_somewhere_in_the_declared_matrix",
    "mokiterions-core/tests/viability.rs :: survival_by_turn_position_stays_inside_the_stated_bound",
    "mokiterions-core/tests/viability.rs :: the_social_source_keeps_the_world_habitable_and_combat_lethal",
    "mokiterions-tui/tests/authority.rs :: the_decision_source_maps_by_the_source_the_record_names",
    "mokiterions-tui/tests/authority.rs :: the_mapping_is_the_specified_one",
    "mokiterions-tui/tests/options.rs :: the_usage_text_advertises_every_policy_the_engine_accepts",
]

# Which pair of trees is being read, and what the census says the totals are for that pair.
# The per-target table is only meaningful for the baseline pair; §9.2 reconciles the master
# pair by total and by added-name set, not target by target.
PAIRS = {
    "baseline": {
        "totals": (212, 250),
        "targets": CENSUS_SECTION_2,
        "step": CENSUS_STEP,
        "updated": EXPECTED_UPDATED,
        "source": "post/test-census-reconciliation.md §§1-2 and §8",
    },
    "master": {
        "totals": (226, 264),
        "targets": None,
        "step": {},
        "updated": EXPECTED_UPDATED,
        "source": "post/test-census-reconciliation.md §9.2",
    },
    "renumber": {
        "totals": (250, 250),
        "targets": None,
        "step": {},
        "updated": RENUMBER_UPDATED,
        "source": "no census row -- 13906153 against the renumber commit d72465c",
    },
}

failures: list[str] = []


def check(condition: bool, message: str) -> bool:
    if not condition:
        failures.append(message)
    return condition


def sample(names: list[str], keep: int = 3) -> str:
    """Name the first few and count the rest, so a failing check stays one readable line."""
    if len(names) <= keep:
        return ", ".join(names)
    return ", ".join(names[:keep]) + f", and {len(names) - keep} more"


def strip_comments(text: str) -> str:
    """Remove `//` line comments and `/* */` blocks, leaving string literals intact."""
    out: list[str] = []
    i, n = 0, len(text)
    in_string = False
    while i < n:
        ch = text[i]
        if in_string:
            out.append(ch)
            if ch == "\\" and i + 1 < n:
                out.append(text[i + 1])
                i += 2
                continue
            if ch == '"':
                in_string = False
            i += 1
            continue
        if ch == '"':
            in_string = True
            out.append(ch)
            i += 1
            continue
        if ch == "/" and i + 1 < n and text[i + 1] == "/":
            while i < n and text[i] != "\n":
                i += 1
            continue
        if ch == "/" and i + 1 < n and text[i + 1] == "*":
            i += 2
            while i + 1 < n and not (text[i] == "*" and text[i + 1] == "/"):
                i += 1
            i += 2
            continue
        out.append(ch)
        i += 1
    return "".join(out)


def parse_tests(text: str, path: str) -> dict[str, str]:
    """Every `#[test]` function in one file, keyed by name, valued by its whole body."""
    lines = text.splitlines(keepends=True)
    tests: dict[str, str] = {}
    index = 0
    while index < len(lines):
        if lines[index].strip() != "#[test]":
            index += 1
            continue
        cursor = index + 1
        while cursor < len(lines) and lines[cursor].lstrip().startswith("#["):
            cursor += 1
        signature = cursor
        while signature < len(lines) and " fn " not in lines[signature] and not lines[
            signature
        ].lstrip().startswith("fn "):
            signature += 1
            if signature - cursor > 4:
                raise SystemExit(f"{path}: no signature after the #[test] at line {index + 1}")
        match = re.search(r"\bfn\s+([A-Za-z0-9_]+)", lines[signature])
        if match is None:
            raise SystemExit(f"{path}: unreadable signature at line {signature + 1}")
        name = match.group(1)
        depth, end = 0, signature
        opened = False
        while end < len(lines):
            for ch in strip_comments(lines[end]):
                if ch == "{":
                    depth += 1
                    opened = True
                elif ch == "}":
                    depth -= 1
            if opened and depth == 0:
                break
            end += 1
        if not opened or depth != 0:
            raise SystemExit(f"{path}: unbalanced body for {name} at line {signature + 1}")
        if name in tests:
            raise SystemExit(f"{path}: two tests named {name}")
        tests[name] = "".join(lines[index : end + 1])
        index = end + 1
    return tests


def target_of(path: str) -> str:
    """The cargo test target a file's tests run in, as the census names it."""
    if "/tests/" in path:
        return "tests/" + path.rsplit("/", 1)[1]
    return "unittests"


def collect(root: Path) -> dict[str, str]:
    """Every test in one extracted tree, keyed by `<path> :: <name>`."""
    found: dict[str, str] = {}
    for source_root in SOURCE_ROOTS:
        directory = root / source_root
        if not directory.is_dir():
            raise SystemExit(f"{directory}: not a directory -- is this a git archive extraction?")
        for path in sorted(directory.rglob("*.rs")):
            relative = path.relative_to(root).as_posix()
            text = path.read_text(encoding="utf-8")
            for name, body in parse_tests(text, relative).items():
                found[f"{relative} :: {name}"] = body
    return found


def assertions(body: str) -> int:
    return len(ASSERTION.findall(strip_comments(body)))


def body_lines(body: str) -> int:
    stripped = strip_comments(body)
    return sum(1 for line in stripped.splitlines() if line.strip())


def rule(title: str) -> None:
    print()
    print(title)
    print("-" * len(title))


def main(argv: list[str]) -> int:
    if len(argv) not in (3, 4):
        raise SystemExit(
            "usage: updated-tests.py <before tree> <after tree> [baseline|master]"
        )
    pair_name = argv[3] if len(argv) == 4 else "baseline"
    if pair_name not in PAIRS:
        raise SystemExit(f"unknown pair {pair_name!r}: expected one of {', '.join(PAIRS)}")
    pair = PAIRS[pair_name]
    base_root, cand_root = Path(argv[1]), Path(argv[2])
    base, cand = collect(base_root), collect(cand_root)

    print("Every test updated between two trees, with its assertion count before and after")
    print("=" * 79)
    print()
    print(f"before tree     {base_root}")
    print(f"after tree      {cand_root}")
    print(f"pair            {pair_name} -- reconciled by {pair['source']}")
    print(f"source roots    {', '.join(SOURCE_ROOTS)}")
    print(f"tests found     {len(base)} before, {len(cand)} after")

    rule("1. The two sides by target, against the census taken from the cargo test log")

    targets = sorted({target_of(key.split(" :: ")[0]) for key in list(base) + list(cand)})
    table, step_by_target = pair["targets"], pair["step"]
    heading = "   census §2" if table else ""
    print(f"{'target':24} {'before':>9} {'after':>10} {'':>5}{heading}")
    for target in targets:
        before = sum(1 for key in base if target_of(key.split(" :: ")[0]) == target)
        after = sum(1 for key in cand if target_of(key.split(" :: ")[0]) == target)
        delta = "" if before == after else f"{after - before:+d}"
        note = ""
        if table is not None:
            expected = table.get(target)
            if expected is None:
                note = "   no row in §2's table"
                check(False, f"{target} has no row in the census's §2 table")
            else:
                step = step_by_target.get(target, 0)
                note = f"   {expected[0]:>3} -> {expected[1]:<3}"
                if step:
                    note += f" and §8's +{step}"
                check(
                    before == expected[0],
                    f"{target}: {before} before against §2's {expected[0]}",
                )
                check(
                    after == expected[1] + step,
                    f"{target}: {after} after against §2's {expected[1]} plus {step}",
                )
        print(f"{target:24} {before:>9} {after:>10} {delta:>5}{note}")
    print(f"{'total':24} {len(base):>9} {len(cand):>10}")
    if table is not None:
        for target in sorted(set(table) - set(targets)):
            check(False, f"{target} has a row in the census's §2 table and no test in either tree")
    print()
    expected_before, expected_after = pair["totals"]
    print(f"{pair['source']} reads {expected_before} names before and {expected_after} after,")
    print("from the cargo test logs. This parser reads source. They agree or one of the two is")
    print("wrong about what the suite is.")
    if table is not None:
        print()
        print("The right-hand column is that file's §2 table, whose after side is the 249-name")
        print("census at 7c4aef39. Its §8 records the step from there to 250 as one viability test")
        print("becoming two, so tests/viability.rs is expected to read one higher here and every")
        print("other target is expected to match exactly.")
    else:
        print()
        print("§9.2 reconciles this pair by total and by added-name set rather than target by")
        print("target, so no per-target expectation is applied and the columns above stand as")
        print("measurement only.")
    check(len(base) == expected_before, f"before total {len(base)} is not the census's {expected_before}")
    check(len(cand) == expected_after, f"after total {len(cand)} is not the census's {expected_after}")

    retained = sorted(set(base) & set(cand))
    added = sorted(set(cand) - set(base))
    removed = sorted(set(base) - set(cand))
    check(
        len(retained) + len(removed) == len(base),
        "retained plus removed does not account for the baseline",
    )
    check(
        len(retained) + len(added) == len(cand),
        "retained plus added does not account for the candidate",
    )

    rule("2. Name reconciliation, by file rather than by target")

    print(f"present on both sides        {len(retained)}")
    print(f"added on the after side      {len(added)}")
    print(f"absent on the after side     {len(removed)}")
    for key in removed:
        print(f"  gone     {key}")
    print()
    print("A rename appears here as one removal and one addition, which is what the census's")
    print("§§3, 6 and 7 reconcile by body rather than by name.")

    rule("3. The updated tests: same name, changed body")

    updated = [key for key in retained if base[key] != cand[key]]
    unchanged = len(retained) - len(updated)
    print(f"{'test':64} {'assertions':>18} {'body lines':>14}")
    print(f"{'':64} {'before  after':>18} {'before after':>14}")
    fell = []
    for key in updated:
        before, after = assertions(base[key]), assertions(cand[key])
        lines_before, lines_after = body_lines(base[key]), body_lines(cand[key])
        mark = ""
        if after < before:
            mark = "  <- FEWER ASSERTIONS"
            fell.append((key, before, after))
        elif after == before:
            mark = "  same count"
        print(
            f"{key:64} {before:>7}{after:>8} {lines_before:>8}{lines_after:>7}{mark}"
        )
    print()
    print(f"retained and byte-identical  {unchanged}")
    print(f"retained and updated         {len(updated)}")
    total_before = sum(assertions(base[key]) for key in updated)
    total_after = sum(assertions(cand[key]) for key in updated)
    print(f"assertions across the updated tests: {total_before} before, {total_after} after")
    print()
    recorded = pair["updated"]
    unexpected = sorted(set(updated) - set(recorded))
    missing = sorted(set(recorded) - set(updated))
    check(
        not unexpected,
        f"tests updated that this pair's recorded {len(recorded)} do not name:"
        f" {sample(unexpected)}",
    )
    check(
        not missing,
        f"of this pair's recorded {len(recorded)}, read here as unchanged: {sample(missing)}",
    )
    print(f"The {len(recorded)} names this pair expects are fixed in this reader. On the baseline")
    print("pair that is a tripwire: it repeats what the run just read. On the master pair it is a")
    print("claim -- WO-MOK-013 is on both sides there and cancels, so the same nine and no tenth")
    print("means no test of master's was absorbed into this work order's list. On the renumber")
    print("pair it is the whole point: seven bodies move and not one count does.")

    rule("4. Stop condition 8, measured rather than asserted")

    print("Condition 8 is reached if any existing test's assertions are relaxed, widened,")
    print("removed, renamed away or #[ignore]d. Three of the five are measurable here.")
    print()
    check(
        not fell,
        f"retained tests asserting fewer things than before: {len(fell)}"
        f" -- {sample([f[0] for f in fell])}",
    )
    print(f"retained tests asserting fewer things than before   {len(fell)}")
    ignored_base = sum(1 for body in base.values() if "#[ignore]" in body)
    ignored_cand = sum(1 for body in cand.values() if "#[ignore]" in body)
    check(ignored_cand == 0, f"tests carrying #[ignore] on the after side: {ignored_cand}")
    print(f"#[ignore] before / after                            {ignored_base} / {ignored_cand}")
    print()
    print("Each absent name is either a rename or a removal, and the difference is the whole of")
    print("condition 8's third clause. A rename is resolved here by body rather than by judgement:")
    print("the absent test's body is compared against every test added to the same file, with each")
    print("side's own function name blanked out, and a match means the case survived under a new")
    print("identifier with nothing else altered. An absent name with no match is a removal.")
    print()
    unmatched = []
    for key in removed:
        path, name = key.split(" :: ")
        stem = base[key].replace(name, "FN")
        renamed_to = None
        for candidate_key in added:
            candidate_path, candidate_name = candidate_key.split(" :: ")
            if candidate_path == path and cand[candidate_key].replace(candidate_name, "FN") == stem:
                renamed_to = candidate_key
                break
        if renamed_to is None:
            unmatched.append(key)
            print(f"  REMOVAL  {key}")
            print(
                f"           {assertions(base[key])} assertions, and no test added to {path}"
                f" carries its body"
            )
            continue
        print(f"  rename   {key}")
        print(f"        -> {renamed_to.split(' :: ')[1]}")
        print(
            f"           assertions: {assertions(base[key])} before,"
            f" {assertions(cand[renamed_to])} after; body identical with the name blanked"
        )
    check(
        not unmatched,
        f"tests removed rather than renamed: {len(unmatched)} -- {sample(unmatched)}",
    )
    print()
    print("What this cannot see: an assertion whose *content* was weakened while the count held.")
    print("The rows above with 'same count' are where that could hide, and each is read by hand")
    print("in post/updated-tests.md. A count is a tripwire, not a proof.")

    rule("5. Where the added tests sit")

    by_target: dict[str, int] = {}
    for key in added:
        by_target[target_of(key.split(" :: ")[0])] = (
            by_target.get(target_of(key.split(" :: ")[0]), 0) + 1
        )
    for target in sorted(by_target):
        print(f"{target:24} {by_target[target]:>4}")
    print(f"{'total':24} {len(added):>4}")
    added_assertions = sum(assertions(cand[key]) for key in added)
    print()
    print(f"assertions in the added tests  {added_assertions}")

    rule("Checks")

    if failures:
        for message in failures:
            print(f"FAIL  {message}")
        print()
        print(f"RESULT: FAIL ({len(failures)} of the checks fired)")
        return 1
    print("Every check passed.")
    print()
    print("RESULT: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
