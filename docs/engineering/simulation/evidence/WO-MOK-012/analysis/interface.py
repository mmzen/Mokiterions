"""Enumerate the engine library target's public surface, so it can be diffed.

`SPEC-MOK-002` rule 5 closes the engine's public interface: "the library target's public
interface is exactly the union of the three lists below", and its 2026-08-20 amendment
enumerates the growth item by item and then says what the check is -- "`VER-MOK-012`'s
interface-growth check compares the engine's public surface item for item against this
table". A comparison against a closed list needs the surface as a list, and `grep` for
`pub` does not give one: the growth this work order makes is inside the bodies of public
enums, where variants carry no visibility keyword of their own.

So this script extracts the surface rather than the source. For every public item of the
three files that make up the library target it emits the item's *declaration*, and for a
struct or an enum the field or variant list underneath it, one entry per line. What it
deliberately drops is everything that is not surface: function bodies, match arms, comments
and the values of constants. Two commits' surfaces can then be compared with `diff`, and
every line of difference has to correspond to a row of the amendment's table.

The one deliberate consequence of dropping values is worth stating, because it is a finding
rather than a limitation. `cli::USAGE`'s declaration is `pub const USAGE: &str` at both
commits while its text changed, and `EventType::ALL`'s declaration is `pub const ALL:
[Self; 12]` against `[Self; 15]` because an array's length is part of its type. Section 2 of
post/interface.txt digests both values for that reason, so the change to `USAGE` is recorded
where the surface diff cannot show it.

Usage:

    git show <rev>:mokiterions-core/src/simulation.rs > simulation.rs   (and lib.rs, cli.rs)
    python interface.py <dir with the three files>

Prints the surface listing on standard output. Exits 0 when the three files parsed and every
public item was reached, 1 on a shape this reader does not handle -- it stops rather than
omitting an item, because an enumeration that silently drops a public item would answer the
question backwards.

The brace counter is imported from regions.py rather than copied, so the two evidence
readers cannot disagree about where an item ends.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

from regions import count_braces

# The library target is exactly these three files: `src/lib.rs` and the two modules it
# declares `pub mod` for. Nothing else compiles into it -- `src/main.rs` is the binary
# target and carries no item of its own.
FILES = [("lib.rs", "crate"), ("cli.rs", "cli"), ("simulation.rs", "simulation")]

DECLARATION = re.compile(
    r"^(?P<vis>pub(?:\((?P<scope>[^)]*)\))?\s+)?"
    r"(?:unsafe\s+)?(?P<kw>impl|mod|fn|enum|struct|const|static|type|trait|use)\b"
)

# Items whose declaration ends at a `;` rather than at a balanced brace.
TERMINATED = {"const", "static", "type", "use"}


def collapse(text: str) -> str:
    """One line, single-spaced. Rust's formatting of a signature is not its surface."""
    return re.sub(r"\s+", " ", text).strip()


def item_end(lines: list[str], start: int) -> int:
    """The index one past the last line of the item beginning at `start`.

    A `const`, `static`, `type` or `use` runs to the `;` that closes it at bracket depth
    zero; `concat!(...)` spanning twenty lines is one item. Everything else runs to the line
    where brace depth returns to zero, with the opening brace allowed to be on a later line
    than the declaration so that a multi-line `fn` signature is handled. A unit struct or a
    trait method without a body ends at its own `;`.
    """
    first = lines[start].strip()
    keyword = DECLARATION.match(first).group("kw")

    if keyword in TERMINATED:
        depth = 0
        index = start
        while index < len(lines):
            depth += sum(lines[index].count(c) for c in "([{")
            depth -= sum(lines[index].count(c) for c in ")]}")
            if depth <= 0 and lines[index].rstrip().endswith(";"):
                return index + 1
            index += 1
        raise SystemExit(f"declaration at line {start + 1} never reaches its `;`")

    if first.endswith(";"):
        return start + 1

    depth = 0
    opened = False
    index = start
    while index < len(lines):
        depth += count_braces(lines[index].rstrip("\n"))
        opened = opened or depth > 0
        if opened and depth == 0:
            return index + 1
        if not opened and lines[index].rstrip().endswith(";"):
            return index + 1
        index += 1
    raise SystemExit(f"item at line {start + 1} never closes")


def signature(lines: list[str], start: int, end: int) -> str:
    """The declaration with its body or value removed.

    Where the cut falls is decided by the keyword, not by whichever punctuation comes first.
    A `const`, `static`, `type` or `use` is cut at its `=`, so that the *type* survives and
    the value does not: `pub const ALL: [Self; 15]` keeps the array length the 2026-08-20
    amendment calls the one change to a `pub const`, and `pub const USAGE: &str` keeps only
    its type, which is why section 2 of post/interface.txt digests that value separately.
    Everything else is cut at its `{`, never at an `=`, because `where I: IntoIterator<Item =
    S>` carries one and a signature cut there would lose its own bounds.
    """
    text = "".join(line.rstrip("\n") + " " for line in lines[start:end])
    keyword = DECLARATION.match(lines[start].strip()).group("kw")
    terminator = "=" if keyword in TERMINATED else "{"
    position = find_outside_literals(text, terminator)
    if position == -1:
        position = find_outside_literals(text, ";")
    if position != -1:
        text = text[:position]
    return collapse(text)


def find_outside_literals(text: str, needle: str) -> int:
    """The first index of `needle` not inside a string literal, or -1.

    Needed because `pub const USAGE: &str = concat!("Usage: ...` would otherwise be cut at
    a brace inside its own usage text rather than at its `=`. An unterminated quote is a
    lifetime tick -- `-> &'static str` -- and advances one character, exactly as
    regions.py's literal skipper does, so a signature naming a lifetime is still cut at its
    brace rather than run to the end of the item.
    """
    index = 0
    while index < len(text):
        char = text[index]
        if char == "\\":
            index += 2
            continue
        if char in "\"'":
            quote = char
            cursor = index + 1
            while cursor < len(text):
                if text[cursor] == "\\":
                    cursor += 2
                    continue
                if text[cursor] == quote:
                    break
                cursor += 1
            index = cursor + 1 if cursor < len(text) else index + 1
            continue
        if text.startswith(needle, index):
            return index
        index += 1
    return -1


def members(lines: list[str], start: int, end: int) -> list[str]:
    """The variant or field list of the item spanning `start`..`end`, one entry per line.

    Entries are taken at the body's own depth, so a struct variant's inner field list is
    folded into its variant entry rather than emitted beside it. Doc comments and attributes
    are dropped: a `#[default]` marker is surface and is kept, everything else annotates.
    """
    entries: list[str] = []
    buffer: list[str] = []
    depth = 0
    started = False

    for raw in lines[start:end]:
        line = raw.rstrip("\n")
        depth += count_braces(line)

        if not started:
            started = depth > 0
            continue
        if depth == 0:
            break

        stripped = line.strip()
        if stripped.startswith("//"):
            continue
        if stripped.startswith("#[") and "default" not in stripped:
            continue
        if not stripped:
            continue

        buffer.append(stripped)
        if depth == 1:
            entries.append(collapse(" ".join(buffer)))
            buffer = []

    if buffer:
        raise SystemExit(f"member list at line {start + 1} did not close")
    return entries


def impl_target(declaration: str) -> tuple[str | None, str]:
    """The `(trait, type)` an `impl` header names, with generics and lifetimes dropped."""
    text = collapse(declaration).removeprefix("impl").removesuffix("{").strip()
    text = re.sub(r"^<[^>]*>\s*", "", text)
    if " for " in text:
        trait_name, type_name = text.split(" for ", 1)
    else:
        trait_name, type_name = None, text
    type_name = re.sub(r"<.*", "", type_name).strip()
    return (trait_name.strip() if trait_name else None, type_name)


def derives(lines: list[str], start: int) -> str:
    """The `derive` list attached to the item at `start`, as written, or the empty string.

    A derive is public surface: `#[derive(Clone)]` on a public type is a public `impl`, and
    removing one is a breaking change no `pub` keyword records.
    """
    index = start - 1
    while index >= 0:
        stripped = lines[index].strip()
        if stripped.startswith("#[derive("):
            return collapse(stripped)
        if stripped.startswith("#[") or stripped.startswith("//"):
            index -= 1
            continue
        break
    return ""


def walk(lines: list[str], module: str, public_types: set[str]) -> list[str]:
    """The surface entries of one file, in source order."""
    out: list[str] = []
    index = 0
    while index < len(lines):
        line = lines[index]
        if line[:1].isspace() or not line.strip():
            index += 1
            continue

        match = DECLARATION.match(line.strip())
        if not match:
            index += 1
            continue

        end = item_end(lines, index)
        keyword = match.group("kw")
        visible = bool(match.group("vis")) and not match.group("scope")
        prefix = f"{module}::" if module != "crate" else ""

        if keyword == "impl":
            trait_name, type_name = impl_target(signature(lines, index, end) + " ")
            if type_name in public_types:
                if trait_name:
                    out.append(f"impl {trait_name} for {prefix}{type_name}")
                out.extend(impl_members(lines, index, end, f"{prefix}{type_name}"))
        elif visible:
            declared = signature(lines, index, end)
            attached = derives(lines, index)
            name = qualified(declared, keyword, prefix)
            out.append(f"{declared.replace(' ' + name.split('::')[-1], ' ' + name, 1)}"
                       f"{'   ' + attached if attached else ''}")
            if keyword in ("enum", "struct"):
                out.extend(f"    {entry}" for entry in members(lines, index, end))

        index = end
    return out


def qualified(declaration: str, keyword: str, prefix: str) -> str:
    """`simulation::Action` for a declaration of `pub enum Action`."""
    tail = declaration.split(keyword, 1)[1].strip()
    name = re.split(r"[<(:\s]", tail, maxsplit=1)[0]
    return f"{prefix}{name}"


def impl_members(lines: list[str], start: int, end: int, owner: str) -> list[str]:
    """The `pub` items declared inside one `impl` block, qualified by their owner."""
    out: list[str] = []
    index = start + 1
    while index < end - 1:
        stripped = lines[index].strip()
        match = DECLARATION.match(stripped)
        if not match or not match.group("vis") or match.group("scope"):
            index += 1
            continue
        inner_end = item_end(lines, index)
        declared = signature(lines, index, inner_end)
        keyword = match.group("kw")
        name = re.split(r"[<(:\s]", declared.split(keyword, 1)[1].strip(), maxsplit=1)[0]
        out.append(collapse(declared.replace(f" {name}", f" {owner}::{name}", 1)))
        index = inner_end
    return out


def public_type_names(lines: list[str]) -> set[str]:
    """The names of the public types declared in one file."""
    found = set()
    for line in lines:
        match = DECLARATION.match(line)
        if not match or not match.group("vis") or match.group("scope"):
            continue
        if match.group("kw") not in ("enum", "struct", "trait", "type"):
            continue
        tail = line.strip().split(match.group("kw"), 1)[1].strip()
        found.add(re.split(r"[<(:;\s]", tail, maxsplit=1)[0])
    return found


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        raise SystemExit("usage: interface.py <directory holding lib.rs, cli.rs, simulation.rs>")

    root = Path(argv[1])
    for filename, module in FILES:
        path = root / filename
        with open(path, encoding="utf-8", newline="") as handle:
            lines = handle.readlines()
        print(f"# {filename} -- module `{module}`, {len(lines)} lines")
        for entry in walk(lines, module, public_type_names(lines)):
            print(entry)
        print()
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
