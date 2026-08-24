#!/usr/bin/env python3
"""Read a retained transcript back and check what `VER-MOK-018` asks of a request.

Repository-owned, absent from `.engineering-harness.lock`, decides nothing, writes
nothing, reaches no network and reads no credential of its own.

This carries `VER-MOK-018` cases **L4**, **L5**, **L6**, **L12**, **L13**, **L14** and
**L15a**, every one of which states its condition *"over a retained transcript"*. The
engine's own tiers already check the composition of a request as the engine builds it —
`block_d_offers_exactly_what_the_engine_would_accept_from_the_observation` puts every
targeted verb to the engine's own validator, and `block_a_is_one_string_for_the_run_…`
measures the shared block across a run's requests. Those checks are worth having and
they are not these cases: they read the engine's own objects through the engine's own
private helpers, at the moment the engine composes them. What these cases ask is whether
the artifact a host actually retained — the file a reviewer can open, and the file a
replay reads — carries what the specification says it carries.

So this reads the committed transcript as data and recomputes, from the observation text
alone, what block D should have enumerated. The recomputation is written from
`SPEC-MOK-001` rule 6 and rule 21 and `SPEC-MOK-007` rule 7, in this file, in another
language, sharing nothing with the engine — which is what `L4` means by *"an
independently written enumerator"*. A helper shared with the code under test would agree
with it by construction, and a reviewer could not tell that from agreement.

The seven checks:

  **L4 — the enumeration is complete.** For every exchange, the enumerated set equals the
  set this file computes from that request's observation: `wait`; `sleep` unless energy is
  already 100; `eat` per co-located resource; `move` per cardinal direction whose
  destination is inside the grid; and each of rule 21's seven targeted verbs against each
  perceived Mokiterion whose precondition that verb satisfies. Compared as sets, because
  the case says *"equals the set"* and order is rule 7.7's separate obligation, checked
  in-crate by `the_same_configuration_composes_the_same_requests`.

  **L5 — the enumeration is not the core list.** At least one request enumerates a
  targeted action, and no request that *could* have enumerated one — one whose observation
  carries a perceived Mokiterion — has a set equal to the core-proposal list.
  See `LIMITS` below: the case's literal wording is wider than that, and this file
  reports the figure the wider reading turns on rather than deciding it.

  **L6 — no unsatisfiable offer.** No enumerated targeted entry names a target whose
  precondition that verb does not satisfy at that opportunity. `L4`'s equality implies
  this, and it is checked and reported separately because the two fail differently: an
  enumerator that offered too much and one that offered too little are the same `L4`
  failure and are not the same defect.

  **L12 — one Mokiterion per request.** Every line of the observation matches one of the
  forms below, all of which carry the acting Mokiterion's own attributes or another
  Mokiterion's identifier, direction and distance. Nothing else has anywhere to appear.

  **L13 — no aggregate and no derived value.** No count, mean, maximum or ranking over the
  population, checked as a vocabulary scan and as the absence of any figure on a list
  heading; and nothing taken from another exchange, checked as `L12`'s closed grammar
  rather than as a substring search — see `LIMITS`.

  **L14 — self-contained requests.** For each Mokiterion that acted more than once, the
  first and last exchanges carry the same prefix and the same prefix digest, differ in the
  observation and in the enumerated set, and neither observation contains the other. No
  record carries a conversation or session identifier.

  **L15a — the layout holds.** The shared block is byte-identical across every prefix
  record; each Mokiterion's prefix is byte-identical across every one of its exchanges,
  checked through the digest each exchange carries and against the prefix record it names;
  and the observation and the enumerated set follow both, checked as the order of the
  fields in the retained record.

LIMITS — what this program does not decide:

  * **`L5`'s literal wording.** The case reads *"no request's enumerated set equals the
    observation's core-proposal list"*. A request whose observation carries no perceived
    Mokiterion has no targeted verb to enumerate, so its block D *is* the core-proposal
    list — by coincidence rather than by derivation, and `SPEC-MOK-007` rule 7.2's concern
    is the derivation. Read literally the clause fails on any transcript containing such a
    request, which is most of them. This program enforces the reading above and prints the
    literal count, so the figure the question turns on is in the evidence either way. Which
    reading `VER-MOK-018` intends is not an implementation agent's to decide;
    `WO-MOK-025` stop condition 7 is the route for it.
  * **whether the transcript is the run it claims to be.** That is
    `mokiterions-core/tests/replay.rs`, which records the same configuration in-process and
    compares byte for byte, and it is why this program can treat the file as a measurement.
  * **anything about outcomes.** No survivor figure, death figure or comparison between
    sources is computed here, per `VER-MOK-018` case `L26` and `WO-MOK-025` stop
    condition 11.
"""

from __future__ import annotations

import argparse
import io
import json
import re
import sys
from pathlib import Path

# The world, from `SPEC-MOK-001`'s *World* section and the shared rules block's own words:
# "a grid 128 cells wide and 128 cells tall", "an x coordinate from 0 to 127 and a y coordinate
# from 0 to 127", and "y rises toward the south". Written here rather than read from the request,
# because a request that stated the wrong grid would then check out against itself.
GRID = 128

# North is `y - 1` because y rises toward the south. A move is permitted where the destination is
# inside the grid and nowhere else, which is the whole of rule 8's condition as the shared block
# states it: "Not permitted where the destination would be outside the grid." Occupancy is
# deliberately absent — `SPEC-MOK-007` rule 7.5 keeps a rejection block D could not have known
# about out of this and in the ordinary rejected-proposal path.
CARDINALS = {"north": (0, -1), "east": (1, 0), "south": (0, 1), "west": (-1, 0)}

# `SPEC-MOK-001` rule 21's seven verbs. The value is the precondition, evaluated against one
# perceived entry and the actor's suffered-attack record.
#
#   contact  the target is at a distance of 1 or less, which is the *Contact* section's Chebyshev
#            radius and includes distance 0
#   struck   the target is named in the actor's suffered-attack record
#
# `approach` is the one verb with a negative condition of its own: rule 21's paragraph rejects it
# against a co-located target, because there is no cell to move toward, and rule 7.4 therefore
# keeps it out of block D at distance 0.
TARGETED = {
    "attack": lambda distance, struck: distance <= 1,
    "threaten": lambda distance, struck: distance <= 1,
    "fight": lambda distance, struck: distance <= 1 and struck,
    "retreat": lambda distance, struck: struck,
    "surrender": lambda distance, struck: struck,
    "approach": lambda distance, struck: distance >= 1,
    "avoid": lambda distance, struck: True,
}

# The observation's fields in `SPEC-MOK-007` rule 6.1's order, each with the pattern its own line
# must match. This is `L12`'s and `L13`'s instrument: a closed grammar over every line of block C,
# so that a value belonging to another Mokiterion, an aggregate, or a byte sequence carried over
# from another exchange has no line it could occupy.
HEAD = "WHAT YOU SEE"
SCALARS = ("tick", "health", "satiety", "energy", "fear")
LISTS = (
    "attacks suffered since your previous action",
    "resources in your cell",
    "resources perceived",
    "mokiterions perceived",
)
POSITION = re.compile(r"^position: (\d+):(\d+) in territory [AB]$")
SCALAR = re.compile(r"^(%s): (\d+)$" % "|".join(SCALARS))
ENTRIES = {
    "attacks suffered since your previous action": re.compile(r"^  (M\d\d) for (\d+)$"),
    "resources in your cell": re.compile(r"^  (F\d+)$"),
    "resources perceived": re.compile(
        r"^  (F\d+) class (low|medium|high) direction ([a-z_]+) distance (\d+)$"
    ),
    "mokiterions perceived": re.compile(r"^  (M\d\d) direction ([a-z_]+) distance (\d+)$"),
}

# `L13`'s vocabulary half. A request that named the population would arrive at one of these words:
# the natural way to write an aggregate is to write what it aggregates.
AGGREGATES = (
    "population",
    "living",
    "count",
    "total",
    "average",
    "mean",
    "median",
    "maximum",
    "minimum",
    "highest",
    "lowest",
    "rank",
    "strongest",
    "weakest",
    "nearest",
    "all mokiterions",
    "other mokiterions",
)

# `L14`'s second half. A provider-side identifier is how a stateful conversation arrives, and rule
# 11.4's closed alphabet is not enough on its own to keep one out: an opaque identifier is spelled
# in the same characters an identifier of ours is.
SESSION_KEYS = (
    "session",
    "conversation",
    "thread",
    "message_id",
    "response_id",
    "request_id",
    "chat_id",
    "run_id",
    "trace_id",
    "correlation",
)

# The fields of an exchange record, in the order the layout puts them. `L15a`'s last clause — "the
# observation block and the enumerated set appear after both" — is a statement about where they are,
# and in a retained record that is the order of its fields.
EXCHANGE_ORDER = (
    "transcript",
    "version",
    "tick",
    "actor",
    "prefix",
    "prefix_digest",
    "observation",
    "permitted",
    "response",
    "usage",
    "action",
    "fallback",
)


class Failure(Exception):
    """A check's condition did not hold. Carries the text the report prints."""


def read_transcript(path: Path) -> tuple[list[dict], list[dict]]:
    """The transcript's prefix records and exchange records, in file order.

    Read as bytes and refused on a carriage return, for the reason `.gitattributes` marks this path
    `-text`: a CRLF checkout would leave one at the end of every record, and every length this
    program reports would then be one byte long.
    """
    raw = path.read_bytes()
    if b"\r" in raw:
        raise Failure(
            f"{path}: the transcript holds a carriage return; check the `-text` entry in "
            ".gitattributes"
        )
    if not raw.endswith(b"\n"):
        raise Failure(f"{path}: rule 11.2 frames one record per line, and the last has no newline")

    prefixes: list[dict] = []
    exchanges: list[dict] = []
    for number, line in enumerate(raw.decode("utf-8").splitlines(), start=1):
        try:
            record = json.loads(line)
        except ValueError as error:
            raise Failure(f"{path}:{number}: not a record: {error}") from error
        kind = record.get("transcript")
        if kind == "prefix":
            prefixes.append(record)
        elif kind == "exchange":
            exchanges.append(record)
        else:
            raise Failure(f"{path}:{number}: neither a prefix nor an exchange: {kind!r}")
    if not prefixes or not exchanges:
        raise Failure(
            f"{path}: {len(prefixes)} prefix and {len(exchanges)} exchange record(s); a transcript "
            "this program can check has at least one of each"
        )
    return prefixes, exchanges


def parse_observation(text: str) -> dict:
    """Block C read into its fields, refusing any line the grammar does not admit.

    This is `L12`'s check and half of `L13`'s, and it is also what every other check reads, so a
    request carrying a line no rule authorises is refused before it is used as evidence of anything.
    """
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or lines[0] != HEAD:
        raise Failure(f"block C is not headed {HEAD!r}: {lines[:1]}")

    parsed: dict = {"lists": {name: [] for name in LISTS}}
    seen: list[str] = []
    current: str | None = None
    for line in lines[1:]:
        if line.startswith("  "):
            if current is None:
                raise Failure(f"an entry line under no heading: {line!r}")
            if line == "  none":
                if parsed["lists"][current]:
                    raise Failure(f"{current!r} states both entries and none")
                parsed["lists"][current] = "none"
                continue
            if parsed["lists"][current] == "none":
                raise Failure(f"{current!r} states none and then an entry: {line!r}")
            match = ENTRIES[current].match(line)
            if match is None:
                raise Failure(f"an entry of {current!r} carries something else: {line!r}")
            parsed["lists"][current].append(match.groups())
            continue

        current = None
        position = POSITION.match(line)
        scalar = SCALAR.match(line)
        if position is not None:
            parsed["position"] = (int(position.group(1)), int(position.group(2)))
            seen.append("position")
        elif scalar is not None:
            parsed[scalar.group(1)] = int(scalar.group(2))
            seen.append(scalar.group(1))
        elif line.endswith(":") and line[:-1] in LISTS:
            current = line[:-1]
            seen.append(current)
        else:
            raise Failure(f"block C carries a line no rule authorises: {line!r}")

    expected = ["tick", "position", "health", "satiety", "energy", "fear", *LISTS]
    if seen != expected:
        raise Failure(f"block C's fields are {seen}, and rule 6.1 fixes {expected}")
    for name in LISTS:
        if parsed["lists"][name] == "none":
            parsed["lists"][name] = []
        elif not parsed["lists"][name]:
            raise Failure(f"{name!r} states neither an entry nor none, and rule 6.5 requires one")
    return parsed


def enumerate_independently(observation: dict) -> set[str]:
    """What block D must hold, computed from the observation and from the specifications alone.

    `SPEC-MOK-007` rule 7.3's four sources, in order: the core verbs, `eat` per co-located
    resource, `move` per valid cardinal direction, and rule 21's seven verbs per perceived
    Mokiterion whose precondition each satisfies.
    """
    forms = {"wait"}

    # "Not permitted while energy is already 100" — the shared block's own words for rule 10's
    # condition, and the one core verb that is conditional.
    if observation["energy"] < 100:
        forms.add("sleep")

    for (food_id,) in observation["lists"]["resources in your cell"]:
        forms.add(f"eat {food_id}")

    x, y = observation["position"]
    for name, (dx, dy) in CARDINALS.items():
        if 0 <= x + dx < GRID and 0 <= y + dy < GRID:
            forms.add(f"move {name}")

    struck = {
        attacker for attacker, _ in observation["lists"]["attacks suffered since your previous action"]
    }
    for actor_id, _direction, distance in observation["lists"]["mokiterions perceived"]:
        for verb, permitted in TARGETED.items():
            if permitted(int(distance), actor_id in struck):
                forms.add(f"{verb} {actor_id}")
    return forms


def parse_permitted(text: str) -> list[str]:
    """Block D read into its forms, refusing anything that is not one."""
    lines = text.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    if not lines or lines[0] != "ACTIONS YOU MAY TAKE":
        raise Failure(f"block D is not headed 'ACTIONS YOU MAY TAKE': {lines[:1]}")
    forms = []
    for line in lines[1:]:
        if not line.startswith("  ") or line[2:].strip() != line[2:]:
            raise Failure(f"block D carries a line that is not an entry: {line!r}")
        forms.append(line[2:])
    if not forms:
        raise Failure("block D enumerates nothing, and rule 7.6 requires an answer to be possible")
    return forms


def core_only(observation: dict) -> set[str]:
    """The observation's list of currently valid core proposals: rule 7.2's list, recomputed.

    `SPEC-MOK-001` rule 4 fixes its members — `wait`, `sleep` below full energy, `eat` per
    co-located resource, and the valid moves — and this is `L5`'s comparison set.
    """
    return {form for form in enumerate_independently(observation) if " " not in form or
            form.split(" ", 1)[0] in ("eat", "move")}


def check(path: Path, out) -> list[tuple[str, bool, str]]:
    """Every case, over one transcript. Returns one row per case: label, held, detail."""
    prefixes, exchanges = read_transcript(path)
    results: list[tuple[str, bool, str]] = []

    def record(label: str, held: bool, detail: str) -> None:
        results.append((label, held, detail))
        print(f"  {'PASS' if held else 'FAIL'}  {label}  {detail}", file=out)

    print(f"transcript: {path}", file=out)
    print(
        f"records: {len(prefixes) + len(exchanges)} "
        f"({len(prefixes)} prefix, {len(exchanges)} exchange)",
        file=out,
    )
    print(f"requests examined: {len(exchanges)}", file=out)
    print(file=out)

    # ---- L15a: the layout, over the retained records -----------------------------------------
    shared = {prefix["blocks"][0] for prefix in prefixes}
    by_actor = {prefix["actor"]: prefix for prefix in prefixes}
    layout: list[str] = []
    if len(shared) != 1:
        layout.append(f"{len(shared)} distinct shared blocks across {len(prefixes)} prefix records")
    for prefix in prefixes:
        if len(prefix["blocks"]) != 2:
            layout.append(f"{prefix['actor']} carries {len(prefix['blocks'])} blocks, not 2")
    for exchange in exchanges:
        named = by_actor.get(exchange["prefix"])
        if named is None:
            layout.append(f"tick {exchange['tick']} {exchange['actor']}: no such prefix record")
        elif named["digest"] != exchange["prefix_digest"]:
            layout.append(
                f"tick {exchange['tick']} {exchange['actor']}: prefix digest "
                f"{exchange['prefix_digest']} is not the prefix record's {named['digest']}"
            )
        elif exchange["prefix"] != exchange["actor"]:
            layout.append(
                f"tick {exchange['tick']} {exchange['actor']}: reads {exchange['prefix']}'s prefix"
            )
        if list(exchange) != list(EXCHANGE_ORDER):
            layout.append(f"tick {exchange['tick']} {exchange['actor']}: fields out of order")
    shared_bytes = len(next(iter(shared)).encode("utf-8"))
    actor_bytes = {
        prefix["actor"]: len(prefix["blocks"][1].encode("utf-8")) for prefix in prefixes
    }
    widths = sorted(set(actor_bytes.values()))
    record(
        "L15a the layout holds",
        not layout,
        f"shared block {shared_bytes} bytes, identical across all {len(prefixes)} prefix records; "
        f"actor blocks {min(widths)} to {max(widths)} bytes ({len(actor_bytes)} Mokiterions); "
        f"every exchange names its own prefix and its digest"
        + ("" if not layout else "; " + "; ".join(layout[:4])),
    )

    # ---- L4, L5, L6, L12, L13: one pass over every request ------------------------------------
    complete: list[str] = []
    unsatisfiable: list[str] = []
    grammar: list[str] = []
    aggregate: list[str] = []
    with_targeted = 0
    equal_to_core = 0
    equal_to_core_with_perceived = 0
    perceived_present = 0
    responses = {
        exchange["response"] for exchange in exchanges if exchange.get("response") is not None
    }

    for exchange in exchanges:
        where = f"tick {exchange['tick']} {exchange['actor']}"

        # `L13` reads the request's text and needs nothing the grammar produces, so it runs first.
        # Were it to run after the parse it would be skipped for exactly the requests most likely to
        # carry a derived value, and would then report holding over requests it never examined.
        lowered = f"{exchange['observation']}\n{exchange['permitted']}".lower()
        for word in AGGREGATES:
            if word in lowered:
                aggregate.append(f"{where}: the request says {word!r}")
        for heading in LISTS:
            for line in exchange["observation"].split("\n"):
                if line.startswith(heading) and line != f"{heading}:":
                    aggregate.append(f"{where}: the heading carries a figure: {line!r}")
        for response in responses:
            if response and response in exchange["observation"]:
                aggregate.append(f"{where}: the request carries a response from the transcript")

        try:
            observation = parse_observation(exchange["observation"])
        except Failure as error:
            grammar.append(f"{where}: {error}")
            continue

        offered = parse_permitted(exchange["permitted"])
        offered_set = set(offered)
        if len(offered) != len(offered_set):
            complete.append(f"{where}: block D enumerates a form twice")
        expected = enumerate_independently(observation)
        if offered_set != expected:
            missing = sorted(expected - offered_set)
            extra = sorted(offered_set - expected)
            complete.append(f"{where}: withheld {missing}, offered unauthorised {extra}")

        struck = {
            attacker
            for attacker, _ in observation["lists"]["attacks suffered since your previous action"]
        }
        distances = {
            actor_id: int(distance)
            for actor_id, _direction, distance in observation["lists"]["mokiterions perceived"]
        }
        targeted_here = False
        for form in offered:
            verb, _, target = form.partition(" ")
            if verb not in TARGETED:
                continue
            targeted_here = True
            if target not in distances:
                unsatisfiable.append(f"{where}: {form} names a Mokiterion the request never carried")
            elif not TARGETED[verb](distances[target], target in struck):
                unsatisfiable.append(f"{where}: {form}'s precondition is not met")
        if targeted_here:
            with_targeted += 1

        if observation["lists"]["mokiterions perceived"]:
            perceived_present += 1
        if offered_set == core_only(observation):
            equal_to_core += 1
            if observation["lists"]["mokiterions perceived"]:
                equal_to_core_with_perceived += 1

    record(
        "L4 the enumeration is complete",
        not complete,
        f"all {len(exchanges)} requests agree with the independent enumerator"
        if not complete
        else "; ".join(complete[:4]),
    )
    record(
        "L6 no unsatisfiable offer",
        not unsatisfiable,
        f"no offer among {len(exchanges)} requests names a target whose precondition is unmet"
        if not unsatisfiable
        else "; ".join(unsatisfiable[:4]),
    )
    record(
        "L5 the enumeration is not the core list",
        with_targeted > 0 and equal_to_core_with_perceived == 0,
        f"{with_targeted} of {len(exchanges)} requests enumerate a targeted action; "
        f"of the {perceived_present} whose observation carries a perceived Mokiterion, "
        f"{equal_to_core_with_perceived} have a set equal to the core-proposal list. "
        f"Under the case's literal wording the figure is {equal_to_core} of {len(exchanges)}, "
        "every one of them a request with nothing to target; see this program's LIMITS",
    )
    record(
        "L12 one Mokiterion per request",
        not grammar,
        f"every line of all {len(exchanges)} observations is one of rule 6.1's forms, carrying the "
        "acting Mokiterion's own attributes or another's identifier, direction and distance"
        if not grammar
        else "; ".join(grammar[:4]),
    )
    record(
        "L13 no aggregate and no derived value",
        not aggregate,
        f"no aggregate vocabulary, no figure on a list heading and no response text in any of "
        f"{len(exchanges)} requests; nothing from another exchange has a line to occupy, which is "
        "how the closed grammar above discharges the clause"
        if not aggregate
        else "; ".join(aggregate[:4]),
    )

    # ---- L14: the first and last request of each Mokiterion -----------------------------------
    contained: list[str] = []
    session: list[str] = []
    compared = 0
    serialised = "\n".join(json.dumps(record_, sort_keys=True) for record_ in prefixes + exchanges)
    for key in SESSION_KEYS:
        if key in serialised.lower():
            session.append(f"a record carries {key!r}")

    per_actor: dict[str, list[dict]] = {}
    for exchange in exchanges:
        per_actor.setdefault(exchange["actor"], []).append(exchange)
    for actor, theirs in sorted(per_actor.items()):
        if len(theirs) < 2:
            continue
        compared += 1
        first, last = theirs[0], theirs[-1]
        if (first["prefix"], first["prefix_digest"]) != (last["prefix"], last["prefix_digest"]):
            contained.append(f"{actor}: the prefix moved between tick {first['tick']} and {last['tick']}")
        if first["observation"] == last["observation"]:
            contained.append(f"{actor}: the observation did not change across the run")
        if first["observation"] in last["observation"] or last["observation"] in first["observation"]:
            contained.append(f"{actor}: one request contains the other's observation")
        if first["permitted"] in last["permitted"] and first["permitted"] != last["permitted"]:
            contained.append(f"{actor}: one request contains the other's enumerated set")
    record(
        "L14 self-contained requests",
        not contained and not session,
        f"{compared} Mokiterions acted more than once; each keeps its prefix and its digest, "
        "differs in the observation, and holds no part of its own other request; no record carries "
        "a conversation or session identifier"
        if not contained and not session
        else "; ".join((contained + session)[:4]),
    )
    return results


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "transcript",
        nargs="+",
        type=Path,
        help="a retained transcript, one record per line",
    )
    arguments = parser.parse_args(argv)

    out = io.StringIO()
    failures = 0
    for path in arguments.transcript:
        try:
            results = check(path, out)
        except Failure as error:
            print(f"transcript: {path}", file=out)
            print(f"  FAIL  the transcript could not be read  {error}", file=out)
            failures += 1
            continue
        failures += sum(1 for _label, held, _detail in results if not held)
        print(file=out)

    text = out.getvalue()
    sys.stdout.write(text)
    if failures:
        print(f"FAIL: {failures} check(s) did not hold")
        return 1
    print(
        "PASS: VER-MOK-018 cases L4, L5, L6, L12, L13, L14 and L15a hold over "
        f"{len(arguments.transcript)} retained transcript(s)"
    )
    print(
        "NOT CHECKED HERE: whether the transcript is the run it claims to be, which is\n"
        "  mokiterions-core/tests/replay.rs; and L5's literal wording, on which this program\n"
        "  reports the figure and decides nothing."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
