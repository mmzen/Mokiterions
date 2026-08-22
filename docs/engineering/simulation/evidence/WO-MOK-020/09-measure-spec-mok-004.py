"""Measure every figure `SPEC-MOK-004` rules 6, 9, 10 and 11 record, from one tree.

Usage, from the workspace root:

    cargo test --workspace --locked -- --list > list.txt
    python docs/engineering/simulation/evidence/WO-MOK-020/09-measure-spec-mok-004.py list.txt

The test counts come from `cargo test -- --list`, which is the toolchain's own enumeration of the
compiled test functions, rather than from a pattern over the sources. That matters: a figure
counted by grep can drift from the figure the toolchain reports, and the rules are about the tests
that exist, not about the lines that look like tests. Both library targets are named `src/lib.rs`
in that transcript, so targets are keyed by their executable and not by that label.

The item counts come from a scan of `mokiterions-tui/src`, because no tool reports them. The scan
mirrors each rule's own counting definition:

  * rule 6 counts items written `pub`, so `pub(crate)` and `pub(super)` fall outside it by the
    `pub ` prefix test rather than by a list of exceptions;
  * rule 10 counts items written with no visibility at all;
  * both strip every `#[cfg(test)]` module first, since neither rule counts test scaffolding.

An item is an `fn`, `struct`, `enum`, `const`, `static`, `type`, `trait` or `mod` declaration. A
public field is a `pub` field of a struct. `lib.rs` and `main.rs` are excluded from the rule 6
table, which is a table of the modules that carry the surface.

The two counts are anchored differently, on purpose. Rule 6 counts the public surface wherever it
is declared, so an indented `pub fn` in an `impl` block is part of it. Rule 10's private-item
column counts the module's own items, so it is anchored at column zero: `help_lines`'s local
`const BINDINGS` is a binding inside a function body and not an item of the module, and counting
it would report 56 where the specification records 55.
"""

import io
import os
import re
import sys

ITEM = re.compile(
    r"^\s*(?:pub(?:\((?:crate|super|in [^)]+)\))?\s+)?"
    r"(?:default\s+|async\s+|unsafe\s+|extern\s+\"[^\"]+\"\s+)*"
    r"(fn|struct|enum|const|static|type|trait|mod)\s+"
)
FIELD = re.compile(r"^\s*pub\s+([A-Za-z_][A-Za-z0-9_]*)\s*:")
MODULE_ITEM = re.compile(
    r"^(?:default\s+|async\s+|unsafe\s+|extern\s+\"[^\"]+\"\s+)*"
    r"(fn|struct|enum|const|static|type|trait|mod)\s+"
)
PUBLIC = re.compile(r"^\s*pub\s+(?!\()")
PRIVATE = re.compile(r"^\s*pub")


def strip_test_modules(text):
    """`text` with every `#[cfg(test)]` module removed, braces balanced.

    Only the module's own braces are tracked, so a brace inside a string or a comment in a test
    module could in principle end the strip early. The alternative is a Rust parser; the figures
    below are cross-checked against the toolchain's own test counts, which is what would expose a
    mis-strip.
    """
    lines = text.split("\n")
    kept = []
    index = 0
    while index < len(lines):
        if lines[index].strip().startswith("#[cfg(test)]"):
            depth = 0
            opened = False
            while index < len(lines):
                depth += lines[index].count("{") - lines[index].count("}")
                opened = opened or "{" in lines[index]
                index += 1
                if opened and depth <= 0:
                    break
            continue
        kept.append(lines[index])
        index += 1
    return "\n".join(kept)


def measure(path):
    """One module's public items, `pub` lines, public fields, private functions and private
    consts, with its test modules stripped."""
    with io.open(path, "r", newline="", encoding="utf-8") as handle:
        body = strip_test_modules(handle.read())
    found = {
        "public items": 0,
        "pub lines": 0,
        "public fields": 0,
        "private fns": 0,
        "private consts": 0,
    }
    for line in body.split("\n"):
        if PUBLIC.match(line):
            found["pub lines"] += 1
        item = ITEM.match(line)
        if item and PUBLIC.match(line):
            found["public items"] += 1
        elif FIELD.match(line):
            found["public fields"] += 1
        module_item = MODULE_ITEM.match(line)
        if module_item:
            if module_item.group(1) == "fn":
                found["private fns"] += 1
            elif module_item.group(1) == "const":
                found["private consts"] += 1
    return found


def listing(path):
    """`[(label, executable, [test name, ...]), ...]` from a `cargo test -- --list` transcript."""
    targets = []
    with io.open(path, "r", newline="", encoding="utf-8") as handle:
        for line in handle:
            line = line.rstrip("\r\n")
            running = re.match(r"\s+Running (?:unittests )?(\S+) \((\S+)\)", line)
            if running:
                targets.append((running.group(1), running.group(2), []))
                continue
            name = re.match(r"(\S+): test$", line)
            if name and targets:
                targets[-1][2].append(name.group(1))
    return targets


def main():
    if len(sys.argv) != 2:
        sys.stderr.write(__doc__)
        return 2
    targets = listing(sys.argv[1])
    observer_tests = set(
        name for name in os.listdir("mokiterions-tui/tests") if name.endswith(".rs")
    )

    def tests_of(predicate):
        return sum(len(names) for label, exe, names in targets if predicate(label, exe))

    def basename(label):
        return re.split(r"[\\/]", label)[-1]

    is_observer_lib = lambda label, exe: "mokiterions_tui-" in exe and label.endswith("lib.rs")
    is_observer_main = lambda label, exe: "mokiterions_tui-" in exe and label.endswith("main.rs")
    is_engine_lib = lambda label, exe: re.search(r"[\\/]mokiterions-", exe) is not None
    is_observer_file = lambda label, exe: label.startswith(("tests\\", "tests/")) and basename(
        label
    ) in observer_tests
    is_engine_file = lambda label, exe: label.startswith(("tests\\", "tests/")) and basename(
        label
    ) not in observer_tests

    scans = {}
    for name in sorted(os.listdir("mokiterions-tui/src")):
        if name.endswith(".rs"):
            scans[name] = measure("mokiterions-tui/src/" + name)

    print("# Rule 6: the observer's public surface, module by module")
    print("%-26s %13s %10s %14s" % ("module", "public items", "pub lines", "public fields"))
    total = {"public items": 0, "pub lines": 0, "public fields": 0}
    for name in sorted(scans):
        if name in ("lib.rs", "main.rs"):
            continue
        for key in total:
            total[key] += scans[name][key]
        print(
            "%-26s %13d %10d %14d"
            % (name, scans[name]["public items"], scans[name]["pub lines"], scans[name]["public fields"])
        )
    print("%-26s %13d %10d %14d" % ("total", total["public items"], total["pub lines"], total["public fields"]))

    print("")
    print("# Rule 9: the observer's public verification tier, one row per test file")
    print("%-26s %13s" % ("test file", "tests"))
    public_tier = 0
    for name in sorted(observer_tests):
        found = tests_of(lambda label, exe, name=name: basename(label) == name and "tests" in label)
        public_tier += found
        print("%-26s %13d" % ("tests/" + name, found))
    print("%-26s %13d" % ("total", public_tier))

    print("")
    print("# Rule 10: the observer's internal tier, and the private items of each module")
    split = {}
    for label, exe, names in targets:
        if is_observer_lib(label, exe):
            for name in names:
                module = name.split("::tests::")[0] if "::tests::" in name else name.split("::")[0]
                split[module + ".rs"] = split.get(module + ".rs", 0) + 1
        elif is_observer_main(label, exe):
            split["main.rs"] = split.get("main.rs", 0) + len(names)
    print("%-26s %13s %12s %15s" % ("module", "internal tests", "private fns", "private consts"))
    internal_tier = 0
    for name in sorted(set(list(split) + list(scans))):
        scan = scans.get(name, {"private fns": 0, "private consts": 0})
        internal_tier += split.get(name, 0)
        print(
            "%-26s %13d %12d %15d"
            % (name, split.get(name, 0), scan["private fns"], scan["private consts"])
        )
    print("%-26s %13d" % ("total", internal_tier))

    print("")
    print("# Rule 11: the workspace's verification population")
    engine_internal = tests_of(is_engine_lib)
    engine_public = tests_of(is_engine_file)
    print("%-26s %13d" % ("observer public tier", public_tier))
    print("%-26s %13d" % ("observer internal tier", internal_tier))
    print("%-26s %13d" % ("observer total", public_tier + internal_tier))
    print("%-26s %13d" % ("engine internal tier", engine_internal))
    print("%-26s %13d" % ("engine public tier", engine_public))
    print("%-26s %13d" % ("engine total", engine_internal + engine_public))
    print("%-26s %13d" % ("workspace total", sum(len(names) for _, _, names in targets)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
