"""Measure the four request blocks of a transcript, and the enumeration alternative.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-025/analysis/request-layout.py \
        mokiterions-core/tests/transcript-seed0-ticks20-hunting.jsonl

## Why this exists

`WO-MOK-025`'s completion report item 6 asks for each block's size, the cacheable share, and which
of `SPEC-MOK-007`'s two candidate enumeration renderings was chosen with its measured cost. Every
figure it asks for is derivable from the committed transcript, because `SPEC-MOK-007` rule 11.3
carries the request as sent: blocks A and B in a prefix record, blocks C and D in each exchange
record. So this reads the transcript rather than the engine, and a reader who distrusts it can
re-derive every figure from a file in the repository with no build.

It measures **bytes and characters, never tokens**. There is no tokenizer here and there cannot be
one: `SPEC-MOK-006` rule 12.4 forbids adding a crate, and a hand-written tokenizer would be a second
thing to verify for a figure rule 14.4 refuses to accept anyway. The token estimates in
`candidate/request-layout.txt` are that file's, derived from these byte figures and labelled as
estimates.

## The nested alternative

`SPEC-MOK-007`'s *Explicitly unspecified decisions* leaves the action grammar's shape open — "whether
block D enumerates verb-target pairs as one flat list or as a verb list with per-verb target lists is
unspecified, and the trade-off is left to measurement". The engine built the flat list. `nested`
below re-renders each measured permitted set as the other shape so that the cost side of that
trade-off is a measurement rather than a guess. It is a **cost model and not a proposal**: it is
never sent anywhere, and it says nothing about which shape is answered better.
"""

import collections
import json
import pathlib
import re
import statistics
import sys
import unicodedata

BACKSLASH = chr(92)


def escape(text: str) -> str:
    """`escape_transcript_text`'s rule, implemented independently of the engine.

    `mokiterions-core/src/simulation.rs` escapes the backslash, the quotation mark, the newline, the
    carriage return, the tab, and every remaining control character as `\\uXXXX`. Re-deriving that
    here rather than importing it is the point: comparing this output against the bytes the
    transcript holds checks the engine's escaper against a second reading of the same rule.
    """
    out = []
    for character in text:
        code = ord(character)
        if character == BACKSLASH:
            out.append(BACKSLASH + BACKSLASH)
        elif character == '"':
            out.append(BACKSLASH + '"')
        elif character == "\n":
            out.append(BACKSLASH + "n")
        elif character == "\r":
            out.append(BACKSLASH + "r")
        elif character == "\t":
            out.append(BACKSLASH + "t")
        elif code < 0x20 or code == 0x7F or 0x80 <= code <= 0x9F:
            out.append(BACKSLASH + "u%04x" % code)
        else:
            out.append(character)
    return "".join(out)


def actions(permitted: str) -> list[str]:
    """One permitted set's actions, from the flat rendering the engine writes."""
    lines = permitted.split("\n")
    assert lines[0] == "ACTIONS YOU MAY TAKE", lines[0]
    return [line.strip() for line in lines[1:] if line.strip()]


def nested(permitted: str) -> str:
    """The same set as a verb list with per-verb target lists."""
    groups: dict[str, list[str | None]] = collections.OrderedDict()
    for action in actions(permitted):
        parts = action.split(" ", 1)
        groups.setdefault(parts[0], []).append(parts[1] if len(parts) > 1 else None)
    out = ["ACTIONS YOU MAY TAKE"]
    for verb, targets in groups.items():
        if targets == [None]:
            out.append("  " + verb)
        else:
            out.append("  " + verb + ": " + " ".join(target for target in targets if target))
    return "\n".join(out) + "\n"


def size(name: str, values: list[str]) -> None:
    counts = [len(value.encode("utf-8")) for value in values]
    print(
        f"  {name:22} n={len(values):3}  distinct={len(set(values)):3}  "
        f"bytes {min(counts)}..{max(counts)}  mean {statistics.mean(counts):.1f}  "
        f"total {sum(counts)}"
    )


def field(observation: str, key: str) -> str | None:
    found = re.search("^" + key + r": (.+)$", observation, re.M)
    return found.group(1) if found else None


def co_located(observation: str) -> list[str]:
    found = re.search(r"resources in your cell:\n((?:  .+\n?)+)", observation)
    return [line.strip() for line in found.group(1).strip().split("\n")] if found else []


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        print(__doc__)
        return 2
    path = pathlib.Path(argv[1])
    raw = path.read_bytes()
    lines = [line for line in raw.split(b"\n") if line]

    prefixes, exchanges = [], []
    for line in lines:
        record = json.loads(line)
        (prefixes if "blocks" in record else exchanges).append((record, line))

    # The argument as given rather than the parsed path, so the line does not carry this platform's
    # directory separator into a retained capture.
    print(f"transcript {argv[1]}")
    print(f"  {len(raw)} bytes, {len(lines)} record(s): {len(prefixes)} prefix, {len(exchanges)} exchange")

    block_a = [record["blocks"][0] for record, _ in prefixes]
    block_b = [record["blocks"][1] for record, _ in prefixes]
    block_c = [record["observation"] for record, _ in exchanges]
    block_d = [record["permitted"] for record, _ in exchanges]

    print()
    print("--- the four blocks, as the transcript carries them")
    size("A shared rules", block_a)
    size("B actor", block_b)
    size("C observation", block_c)
    size("D permitted set", block_d)
    print(
        f"  A characters {len(block_a[0])}, bytes {len(block_a[0].encode('utf-8'))}; "
        f"A+B bytes per Mokiterion "
        f"{sorted({len((a + b).encode('utf-8')) for a, b in zip(block_a, block_b)})}"
    )

    print()
    print("--- the request, per exchange")
    actor_block = {record["actor"]: record["blocks"][1] for record, _ in prefixes}
    requests, shares = [], []
    for record, _ in exchanges:
        prefix = len(block_a[0].encode("utf-8")) + len(actor_block[record["actor"]].encode("utf-8"))
        variable = len(record["observation"].encode("utf-8")) + len(record["permitted"].encode("utf-8"))
        requests.append(prefix + variable)
        shares.append(100 * prefix / (prefix + variable))
    print(f"  bytes           min {min(requests)}  max {max(requests)}  mean {statistics.mean(requests):.1f}")
    print(
        f"  prefix share    min {min(shares):.2f}%  max {max(shares):.2f}%  "
        f"mean {statistics.mean(shares):.2f}%   (A+B over A+B+C+D, in bytes)"
    )

    def cell(label: str, index: int) -> None:
        record, _ = exchanges[index]
        prefix_b = len(actor_block[record["actor"]].encode("utf-8"))
        c = len(record["observation"].encode("utf-8"))
        d = len(record["permitted"].encode("utf-8"))
        total = len(block_a[0].encode("utf-8")) + prefix_b + c + d
        print(
            f"  {label:14} tick {record['tick']:2} actor {record['actor']}  "
            f"A {len(block_a[0].encode('utf-8'))}  B {prefix_b}  C {c}  D {d}  "
            f"request {total}  prefix share {100 * (total - c - d) / total:.2f}%"
        )

    by_c = sorted(range(len(block_c)), key=lambda i: len(block_c[i].encode("utf-8")))
    by_d = sorted(range(len(block_d)), key=lambda i: len(block_d[i].encode("utf-8")))
    print()
    print("--- five named exchanges")
    cell("median C", by_c[len(by_c) // 2])
    cell("smallest C", by_c[0])
    cell("largest C", by_c[-1])
    cell("smallest D", by_d[0])
    cell("largest D", by_d[-1])

    print()
    print("--- block D: the flat rendering built, against the nested alternative")
    flat_bytes = [len(value.encode("utf-8")) for value in block_d]
    nested_bytes = [len(nested(value).encode("utf-8")) for value in block_d]
    print(
        f"  flat    total {sum(flat_bytes)}  mean {statistics.mean(flat_bytes):.1f}  "
        f"min {min(flat_bytes)}  max {max(flat_bytes)}"
    )
    print(
        f"  nested  total {sum(nested_bytes)}  mean {statistics.mean(nested_bytes):.1f}  "
        f"min {min(nested_bytes)}  max {max(nested_bytes)}"
    )
    saved = sum(flat_bytes) - sum(nested_bytes)
    print(
        f"  nested saves {saved} bytes = {100 * saved / sum(flat_bytes):.1f}% of block D "
        f"= {100 * (statistics.mean(flat_bytes) - statistics.mean(nested_bytes)) / statistics.mean(requests):.2f}% "
        f"of a mean request"
    )

    print()
    print("--- block D: what was enumerated")
    counts = [len(actions(value)) for value in block_d]
    print(
        f"  actions per set  min {min(counts)}  max {max(counts)}  "
        f"mean {statistics.mean(counts):.2f}  total {sum(counts)}"
    )
    print(f"  sizes            {dict(sorted(collections.Counter(counts).items()))}")
    verbs = collections.Counter(action.split(" ")[0] for value in block_d for action in actions(value))
    for verb, count in sorted(verbs.items(), key=lambda pair: -pair[1]):
        print(f"    {verb:10} {count}")

    print()
    print("--- block D against the observation beside it")
    no_sleep = [record for record, _ in exchanges if "sleep" not in actions(record["permitted"])]
    print(
        f"  sleep absent in {len(no_sleep)} of {len(exchanges)}; their energy values "
        f"{sorted({field(record['observation'], 'energy') for record in no_sleep})}"
    )
    with_sleep = [
        int(field(record["observation"], "energy"))
        for record, _ in exchanges
        if "sleep" in actions(record["permitted"])
    ]
    print(f"  sleep present where energy is {min(with_sleep)}..{max(with_sleep)}")
    eat = [
        record
        for record, _ in exchanges
        if any(action.startswith("eat ") for action in actions(record["permitted"]))
    ]
    print(f"  eat enumerated in {len(eat)} of {len(exchanges)}:")
    for record in eat:
        print(
            f"    tick {record['tick']:2} {record['actor']}  cell {co_located(record['observation'])}"
            f"  ->  {[a for a in actions(record['permitted']) if a.startswith('eat')]}"
        )
    missing_eat = [
        record
        for record, _ in exchanges
        if co_located(record["observation"]) != ["none"]
        and not any(action.startswith("eat ") for action in actions(record["permitted"]))
    ]
    print(f"  co-located food with no eat enumerated: {len(missing_eat)}")
    moves = collections.Counter(
        sum(1 for action in actions(record["permitted"]) if action.startswith("move "))
        for record, _ in exchanges
    )
    print(f"  move actions per set: {dict(sorted(moves.items()))}")
    perceived = 0
    for record, _ in exchanges:
        found = re.search(r"mokiterions perceived:\n((?:  .+\n?)+)", record["observation"])
        entries = [line.strip() for line in found.group(1).strip().split("\n")] if found else []
        perceived += 0 if entries == ["none"] else len(entries)
    print(
        f"  perceived Mokiterions summed over every observation: {perceived}"
        f"   (against approach {verbs['approach']} and avoid {verbs['avoid']})"
    )
    first, second = [], []
    for record, _ in exchanges:
        found = re.search(r"^position: (\d+):(\d+) ", record["observation"], re.M)
        first.append(int(found.group(1)))
        second.append(int(found.group(2)))
    print(
        f"  observed position components: {min(first)}..{max(first)} and {min(second)}..{max(second)}"
        f"  (neither reaches 0, so no boundary case is in this transcript)"
    )

    print()
    print("--- the escaping that carries the blocks, checked against a second reading of the rule")
    checked = mismatched = 0
    for record, line in prefixes + exchanges:
        values = record["blocks"] if "blocks" in record else [record["observation"], record["permitted"]]
        for value in values:
            if ('"' + escape(value) + '"').encode("utf-8") not in line:
                mismatched += 1
            checked += 1
    print(f"  block values re-escaped and compared: {checked}, mismatches {mismatched}")
    text = raw.decode("utf-8")
    for name, sequence in (
        ("newline", BACKSLASH + "n"),
        ("quotation mark", BACKSLASH + '"'),
        ("backslash", BACKSLASH + BACKSLASH),
        ("carriage return", BACKSLASH + "r"),
        ("tab", BACKSLASH + "t"),
        ("control", BACKSLASH + "u"),
    ):
        print(f"    escaped {name:16} {text.count(sequence)}")
    print("  non-ASCII census, because the escaping leaves such characters alone:")
    for name, values in (("A", block_a), ("B", block_b), ("C", block_c), ("D", block_d)):
        non_ascii = sum(1 for value in values for character in value if ord(character) > 127)
        print(f"    block {name} over {len(values):3} value(s): {non_ascii}")
    # The code point and its Unicode name rather than the character: this output is captured on a
    # console whose encoding is not this script's to choose, and a name is readable on all of them.
    print("  where block A's are, by character offset:")
    for index, character in enumerate(block_a[0]):
        if ord(character) > 127:
            print(
                f"    character offset {index}  U+{ord(character):04X}  "
                f"{unicodedata.name(character)}"
            )
    print(
        f"  U+2014 occurrences in the whole transcript: "
        f"{raw.count(chr(0x2014).encode('utf-8'))}"
    )
    # `SPEC-MOK-007` rule 11.4.1 names the characters that put block A outside the record stream's
    # closed alphabet `A-Z a-z 0-9 _ . - + : ; >`. Each is counted rather than accepted, and the
    # `in alphabet` column says whether its presence supports the rule's conclusion at all.
    print("  rule 11.4.1's named characters in block A, counted:")
    for name, character, in_alphabet in (
        ("space", " ", False),
        ("comma", ",", False),
        ("opening parenthesis", "(", False),
        ("closing parenthesis", ")", False),
        ("full stop", ".", True),
        ("em dash", chr(0x2014), False),
        ("newline", "\n", False),
    ):
        print(
            f"    {name:20} {block_a[0].count(character):5}   "
            f"in the closed alphabet: {'yes' if in_alphabet else 'no'}"
        )

    print()
    print("--- reconciliation with SPEC-MOK-007 rules 11.7 and 11.7.1")
    prefix_json = sum(len(line) for _, line in prefixes)
    exchange_json = sum(len(line) for _, line in exchanges)
    print(f"  prefix records    {prefix_json} + {len(prefixes)} newline(s) = {prefix_json + len(prefixes)}")
    print(f"  exchange records  {exchange_json} + {len(exchanges)} newline(s) = {exchange_json + len(exchanges)}")
    print(
        f"  total             {prefix_json + exchange_json} + {len(lines)} newline(s) = "
        f"{prefix_json + exchange_json + len(lines)}"
    )
    mean_exchange = (exchange_json + len(exchanges)) / len(exchanges)
    print(f"  mean exchange record {mean_exchange:.1f}")
    inline = mean_exchange + statistics.mean(
        len((a + b).encode("utf-8")) for a, b in zip(block_a, block_b)
    )
    print(f"  a record carrying A and B inline would mean {inline:.1f}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
