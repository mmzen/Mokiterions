"""Build rules 22, 23 and 24's resolution tables, and check every released resolution against them.

`VER-MOK-016` retains "the constructed-state resolution tables for damage, energy cost,
threat increase and forfeit, with every boundary case". A table of a function is only
evidence if the function it tabulates is the specification's and not the code's, so the four
functions here are transcribed from `SPEC-MOK-001` rules 22, 23 and 24 and from nothing else:

    damage(e, h)        = 10 + (e + h) // 10                     rule 22
    cost(e)             = min(5, e)                              rule 22, saturating at zero
    increase(f)         = min(f + 30, 100) - f                   rule 23, saturating at the bound
    forfeit(s)          = s // 2                                 rule 24
    transferred(s, r)   = min(forfeit(s), 100 - r)               rule 24, excess destroyed
    discarded(s, r)     = forfeit(s) - transferred(s, r)         rule 24

`simulation.rs` is never imported, parsed or consulted. That is the point: an oracle that
read the implementation would agree with it by construction.

The tables are exhaustive rather than sampled. Each of the four is a function of one or two
attributes bounded in `0..=100`, so its whole domain is enumerable -- 10,201 pairs for
damage, 101 values for the threat increase, 10,201 for the forfeit -- and the tables below
are those domains collapsed onto their distinct outputs, with the interval of inputs that
produces each. A boundary case is then not something to remember to include: it is the first
or last row of an interval, and the enumeration produces all of them.

The second half is a check rather than a table. Every `attack_resolved`, `threat_resolved`
and `surrender_resolved` line of the released 30-cell capture is parsed and recomputed from
the functions above, and the run exits non-zero if any line disagrees. What each event line
carries decides how complete that check can be, and the two cases are different:

  * A threat and a surrender are fully determined by what their own event line reports, so
    those two are recomputed exactly.
  * `attack_resolved` carries the striker's `energy` but not its `health`, so the damage
    cannot be recomputed from the line alone. It is checked two ways instead: the target's
    health transition must be the saturating subtraction of the damage reported, and the
    damage must imply a *non-empty* striker-health interval under the formula. A base term
    or divisor even slightly wrong makes that interval empty for some lines, which is why
    this is a check and not a restatement.

Controls, because a checker that reports zero disagreements is worth reading only if it can
report one. `--control <name>` perturbs one term of the reader's own arithmetic and leaves
everything else alone; the expected result of every control is a non-zero exit with the
disagreeing lines named. The controls are the reader's, not the engine's: no build is
mutated and no capture is retaken.

    damage-base       10 -> 11        threat-constant   30 -> 25
    damage-divisor    10 -> 9         forfeit-divisor    2 ->  3
    strike-cost        5 ->  7        saturation       100 -> 99

Usage:

    python resolutions.py <capture directory>            tables, then the check
    python resolutions.py <capture directory> --control damage-base

Exits 0 when every parsed resolution agrees with the tables, 1 on any disagreement, on a
line whose shape this reader does not recognize, or on a capture holding no resolution at
all -- an empty check must not read as a passing one.
"""

from __future__ import annotations

import re
import sys
from collections import Counter
from pathlib import Path

ATTRIBUTE_MAX = 100

# Rule 22's two constants, rule 23's one and rule 24's proportion, as the rules state them.
STRIKE_BASE_DAMAGE = 10
STRIKE_CONDITION_DIVISOR = 10
STRIKE_ENERGY_COST = 5
THREAT_FEAR_INCREASE = 30
FORFEIT_DIVISOR = 2

# Each control names one global and the single value it is given. Nothing else moves.
CONTROLS = {
    "damage-base": ("STRIKE_BASE_DAMAGE", 11),
    "damage-divisor": ("STRIKE_CONDITION_DIVISOR", 9),
    "strike-cost": ("STRIKE_ENERGY_COST", 7),
    "threat-constant": ("THREAT_FEAR_INCREASE", 25),
    "forfeit-divisor": ("FORFEIT_DIVISOR", 3),
    "saturation": ("ATTRIBUTE_MAX", 99),
}

ATTACK = re.compile(
    r"^tick=(?P<tick>\d+) subject=(?P<subject>\S+) event=attack_resolved "
    r"result=target:(?P<target>[^,]+),damage:(?P<damage>\d+),"
    r"target_health:(?P<hp_before>\d+)->(?P<hp_after>\d+),"
    r"striker_energy:(?P<en_before>\d+)->(?P<en_after>\d+),"
    r"target_died:(?P<died>yes|no)$"
)
THREAT = re.compile(
    r"^tick=(?P<tick>\d+) subject=(?P<subject>\S+) event=threat_resolved "
    r"result=target:(?P<target>[^,]+),increase:(?P<increase>\d+),"
    r"target_fear:(?P<before>\d+)->(?P<after>\d+)$"
)
SURRENDER = re.compile(
    r"^tick=(?P<tick>\d+) subject=(?P<subject>\S+) event=surrender_resolved "
    r"result=recipient:(?P<recipient>[^,]+),transferred:(?P<transferred>\d+),"
    r"discarded:(?P<discarded>\d+),"
    r"subject_satiety:(?P<subject_before>\d+)->(?P<subject_after>\d+),"
    r"recipient_satiety:(?P<recipient_before>\d+)->(?P<recipient_after>\d+)$"
)


def damage(energy: int, health: int) -> int:
    """Rule 22: `10 + (striker.energy + striker.health) / 10`, truncating toward zero."""
    return STRIKE_BASE_DAMAGE + (energy + health) // STRIKE_CONDITION_DIVISOR


def cost(energy: int) -> int:
    """Rule 22: a flat `5`, saturating at zero. The amount actually paid."""
    return min(STRIKE_ENERGY_COST, energy)


def increase(fear: int) -> int:
    """Rule 23: `30`, saturating at the attribute bound. The amount actually applied."""
    return min(fear + THREAT_FEAR_INCREASE, ATTRIBUTE_MAX) - fear


def forfeit(satiety: int) -> int:
    """Rule 24: `satiety / 2` of its own `satiety`, truncating toward zero."""
    return satiety // FORFEIT_DIVISOR


def transferred(satiety: int, recipient: int) -> int:
    """Rule 24: what fits in the recipient, the excess being destroyed."""
    return min(forfeit(satiety), ATTRIBUTE_MAX - recipient)


def discarded(satiety: int, recipient: int) -> int:
    """Rule 24: the excess, which `surrender_resolved` reports so non-conservation is visible."""
    return forfeit(satiety) - transferred(satiety, recipient)


def implied_health(energy: int, dealt: int) -> tuple[int, int] | None:
    """The striker `health` interval a reported damage implies, or `None` if empty.

    `attack_resolved` carries the striker's `energy` and not its `health`, so this is the
    strongest statement the released line supports about the damage function: inverting
    `dealt = 10 + (energy + health) // 10` gives `energy + health` in a ten-wide interval,
    and the interval's intersection with `0..=100` must be non-empty for the line to be
    consistent with rule 22 at all. Empty means no striker in a legal state could have dealt
    that damage from that energy.
    """
    quotient = dealt - STRIKE_BASE_DAMAGE
    if quotient < 0:
        return None
    low = quotient * STRIKE_CONDITION_DIVISOR - energy
    high = low + STRIKE_CONDITION_DIVISOR - 1
    low, high = max(low, 0), min(high, ATTRIBUTE_MAX)
    return (low, high) if low <= high else None


def damage_table() -> list[tuple[int, int, int, int]]:
    """`(dealt, condition_low, condition_high, pairs)` over the whole `0..=100` square."""
    pairs: Counter[int] = Counter()
    span: dict[int, list[int]] = {}
    for energy in range(ATTRIBUTE_MAX + 1):
        for health in range(ATTRIBUTE_MAX + 1):
            dealt = damage(energy, health)
            pairs[dealt] += 1
            condition = energy + health
            bounds = span.setdefault(dealt, [condition, condition])
            bounds[0] = min(bounds[0], condition)
            bounds[1] = max(bounds[1], condition)
    return [(dealt, span[dealt][0], span[dealt][1], pairs[dealt]) for dealt in sorted(pairs)]


def cost_table() -> list[tuple[int, int, int, int]]:
    """`(energy_low, energy_high, paid, remaining_at_low)`, collapsed onto distinct outputs."""
    rows: list[tuple[int, int, int, int]] = []
    start = 0
    for energy in range(ATTRIBUTE_MAX + 1):
        paid = cost(energy)
        if energy == ATTRIBUTE_MAX or cost(energy + 1) != paid:
            rows.append((start, energy, paid, start - paid))
            start = energy + 1
    return rows


def increase_table() -> list[tuple[int, int, int]]:
    """`(fear_low, fear_high, applied)`, collapsed onto distinct outputs."""
    rows: list[tuple[int, int, int]] = []
    start = 0
    for fear in range(ATTRIBUTE_MAX + 1):
        applied = increase(fear)
        if fear == ATTRIBUTE_MAX or increase(fear + 1) != applied:
            rows.append((start, fear, applied))
            start = fear + 1
    return rows


def forfeit_table() -> list[tuple[int, int, int]]:
    """`(satiety_low, satiety_high, given_up)`, collapsed onto distinct outputs."""
    rows: list[tuple[int, int, int]] = []
    start = 0
    for satiety in range(ATTRIBUTE_MAX + 1):
        given = forfeit(satiety)
        if satiety == ATTRIBUTE_MAX or forfeit(satiety + 1) != given:
            rows.append((start, satiety, given))
            start = satiety + 1
    return rows


def discard_boundary() -> list[tuple[int, int, int, int]]:
    """The recipient headroom at which a forfeit starts being destroyed, per forfeit size.

    One row per distinct forfeit: the recipient `satiety` at which the transfer is still
    whole, the first at which it is not, and the largest discard the forfeit admits. This is
    the second dimension of rule 24 and the one whole runs do not reach.
    """
    rows = []
    for given in sorted({forfeit(s) for s in range(ATTRIBUTE_MAX + 1)}):
        satiety = given * FORFEIT_DIVISOR
        whole = ATTRIBUTE_MAX - given
        rows.append((given, satiety, whole, given))
    return rows


# The constructed-state rows the retained unit tests carry, transcribed from the test source
# with the line each is on. These are what the *engine* produced: the suite asserts the engine
# against each literal below and passes at 250 of 250, so a row here that disagreed with the
# paper function would be a test asserting the wrong value. That is the third column
# `VER-MOK-016`'s retention clause asks for, and checking it is the reason they are transcribed
# rather than described. Two rows are written as expressions in the constant they are about
# rather than as values, which is noted where it happens: such a row cannot fail when that
# constant changes, and the `contradicted by` column measures the consequence.
DAMAGE_ROWS = [
    # (energy, health, damage), simulation.rs:5413-5421
    (0, 1, 10),
    (0, 9, 10),
    (0, 10, 11),
    (5, 5, 11),
    (50, 50, 20),
    (0, 100, 20),
    (100, 1, 20),
    (99, 100, 29),
    (100, 100, 30),
]
THREAT_ROWS = [
    # (fear before, fear after), simulation.rs:5572-5576
    (0, 30),  # source form `(0u8, THREAT_FEAR_INCREASE)`: the expectation *is* the constant
    (69, 99),
    (70, 100),  # source form `ATTRIBUTE_MAX`, and 70 + 30 lands on it exactly
    (85, 100),  # source form `ATTRIBUTE_MAX`
    (100, 100),  # source form `(ATTRIBUTE_MAX, ATTRIBUTE_MAX)`
]
SURRENDER_ROWS = [
    # (subject satiety, recipient satiety, transferred, discarded), simulation.rs:5642-5655
    (80, 30, 40, 0),
    (20, 50, 10, 0),
    (81, 30, 40, 0),
    (100, 50, 50, 0),
    (100, 90, 10, 40),
    (100, 100, 0, 50),
    (3, 50, 1, 0),
    (2, 50, 1, 0),
    (1, 50, 0, 0),
    (0, 50, 0, 0),
]


def contradicted_by(agrees) -> str:
    """The controls under which this row stops matching what the engine produced.

    A row is evidence about a term only if some other value of that term would make the row
    false, and that is not decided by how the row is written. So it is measured rather than
    annotated: each control is applied alone, the row recomputed, the control listed if the
    row then disagrees. A row listing nothing holds under all six perturbations at once.
    """
    hit = []
    for name, (target, value) in CONTROLS.items():
        restore = globals()[target]
        globals()[target] = value
        try:
            if not agrees():
                hit.append(name)
        finally:
            globals()[target] = restore
    return " ".join(hit) if hit else "--"


def retained_rows() -> tuple[list[str], list[str]]:
    """The transcribed test rows against the paper functions. Returns `(lines, failures)`."""
    lines: list[str] = []
    failures: list[str] = []

    lines.append("   rule 22, strike_damage_is_the_strikers_own_condition_and_the_cost_is_flat")
    lines.append("     energy  health   paper   engine   contradicted by")
    for energy, health, expected in DAMAGE_ROWS:
        paper = damage(energy, health)
        held = contradicted_by(lambda: damage(energy, health) == expected)
        lines.append(f"     {energy:>6}  {health:>6}  {paper:>6}  {expected:>7}   {held}")
        if paper != expected:
            failures.append(f"damage({energy},{health}) paper {paper} against test {expected}")

    lines.append("   rule 23, a_threat_moves_the_targets_fear_and_nothing_else")
    lines.append("     fear    applied  paper   engine   contradicted by")
    for before, after in THREAT_ROWS:
        paper = before + increase(before)
        held = contradicted_by(lambda: before + increase(before) == after)
        lines.append(f"     {before:>6}  {after - before:>7}  {paper:>6}  {after:>7}   {held}")
        if paper != after:
            failures.append(f"increase({before}) paper {paper} against test {after}")

    lines.append("   rule 24, a_surrender_forfeits_half_its_own_satiety_and_discards_what_does_not_fit")
    lines.append("     satiety  recipient   paper t/d   engine t/d   contradicted by")
    for satiety, recipient, moved, lost in SURRENDER_ROWS:
        paper = (transferred(satiety, recipient), discarded(satiety, recipient))
        held = contradicted_by(
            lambda: (transferred(satiety, recipient), discarded(satiety, recipient))
            == (moved, lost)
        )
        lines.append(
            f"     {satiety:>7}  {recipient:>9}   {paper[0]:>4}/{paper[1]:<4}   "
            f"{moved:>4}/{lost:<4}    {held}"
        )
        if paper != (moved, lost):
            failures.append(
                f"surrender({satiety},{recipient}) paper {paper[0]}/{paper[1]}"
                f" against test {moved}/{lost}"
            )

    return lines, failures


def check(paths: list[Path]) -> tuple[int, list[str], dict[str, Counter]]:
    """Recompute every resolution in every stream. Returns `(checked, failures, coverage)`."""
    checked = 0
    failures: list[str] = []
    coverage = {
        "damage": Counter(),
        "cost": Counter(),
        "increase": Counter(),
        "forfeit": Counter(),
        "discarded": Counter(),
        "kinds": Counter(),
    }

    for path in paths:
        with open(path, encoding="utf-8", newline="") as handle:
            for number, raw in enumerate(handle, start=1):
                line = raw.rstrip("\r\n")
                if "event=attack_resolved" in line:
                    match = ATTACK.match(line)
                    if not match:
                        failures.append(f"{path.name}:{number}: unrecognized attack line")
                        continue
                    checked += 1
                    coverage["kinds"]["attack_resolved"] += 1
                    dealt = int(match["damage"])
                    hp_before, hp_after = int(match["hp_before"]), int(match["hp_after"])
                    en_before, en_after = int(match["en_before"]), int(match["en_after"])
                    died = match["died"] == "yes"
                    coverage["damage"][dealt] += 1
                    coverage["cost"][en_before - en_after] += 1

                    if hp_after != max(hp_before - dealt, 0):
                        failures.append(
                            f"{path.name}:{number}: health {hp_before}->{hp_after} is not"
                            f" saturating subtraction of damage {dealt}"
                        )
                    if en_after != en_before - cost(en_before):
                        failures.append(
                            f"{path.name}:{number}: energy {en_before}->{en_after} is not a"
                            f" flat {STRIKE_ENERGY_COST} saturating at zero"
                        )
                    if died != (hp_after == 0):
                        failures.append(
                            f"{path.name}:{number}: target_died={match['died']} against"
                            f" health {hp_after}"
                        )
                    if implied_health(en_before, dealt) is None:
                        failures.append(
                            f"{path.name}:{number}: damage {dealt} from energy {en_before}"
                            f" implies no legal striker health"
                        )
                elif "event=threat_resolved" in line:
                    match = THREAT.match(line)
                    if not match:
                        failures.append(f"{path.name}:{number}: unrecognized threat line")
                        continue
                    checked += 1
                    coverage["kinds"]["threat_resolved"] += 1
                    before, after = int(match["before"]), int(match["after"])
                    applied = int(match["increase"])
                    coverage["increase"][before] += 1
                    if after != before + increase(before):
                        failures.append(
                            f"{path.name}:{number}: fear {before}->{after} is not"
                            f" {THREAT_FEAR_INCREASE} saturating at {ATTRIBUTE_MAX}"
                        )
                    if applied != after - before:
                        failures.append(
                            f"{path.name}:{number}: increase {applied} against transition"
                            f" {before}->{after}"
                        )
                elif "event=surrender_resolved" in line:
                    match = SURRENDER.match(line)
                    if not match:
                        failures.append(f"{path.name}:{number}: unrecognized surrender line")
                        continue
                    checked += 1
                    coverage["kinds"]["surrender_resolved"] += 1
                    s_before = int(match["subject_before"])
                    s_after = int(match["subject_after"])
                    r_before = int(match["recipient_before"])
                    r_after = int(match["recipient_after"])
                    moved = int(match["transferred"])
                    lost = int(match["discarded"])
                    coverage["forfeit"][s_before] += 1
                    coverage["discarded"][lost] += 1

                    if s_after != s_before - forfeit(s_before):
                        failures.append(
                            f"{path.name}:{number}: subject satiety {s_before}->{s_after} is"
                            f" not a loss of satiety/{FORFEIT_DIVISOR}"
                        )
                    if moved != transferred(s_before, r_before):
                        failures.append(
                            f"{path.name}:{number}: transferred {moved} against"
                            f" {transferred(s_before, r_before)}"
                        )
                    if lost != discarded(s_before, r_before):
                        failures.append(
                            f"{path.name}:{number}: discarded {lost} against"
                            f" {discarded(s_before, r_before)}"
                        )
                    if r_after != r_before + moved:
                        failures.append(
                            f"{path.name}:{number}: recipient {r_before}->{r_after} against"
                            f" transferred {moved}"
                        )

    return checked, failures, coverage


def disagreeing_resolutions(failures: list[str]) -> int:
    """How many distinct stream lines are behind a failure list.

    One resolution can fail several checks at once -- a wrong forfeit moves the subject's
    transition, the transfer and the discard together -- so a count of failing checks is not a
    count of failing resolutions, and reporting either as the other would misstate both the
    passing runs and the controls. Every stream failure begins `<stream>:<line>: `.
    """
    return len({failure.split(": ", 1)[0] for failure in failures})


def report(paths: list[Path]) -> int:
    print("1. Rule 22's damage, over its whole domain")
    print("   damage = 10 + (energy + health) / 10, both terms in 0..=100")
    print()
    print("   damage   energy+health   (energy,health) pairs")
    total = 0
    for dealt, low, high, pairs in damage_table():
        total += pairs
        print(f"   {dealt:>6}   {low:>4}..={high:<4}      {pairs:>5}")
    print(f"   {'':>6}   {'':>4}   {'':>4}      {total:>5}  = 101 x 101")
    print()

    print("2. Rule 22's energy cost, over its whole domain")
    print("   energy before   paid   energy after")
    for low, high, paid, _ in cost_table():
        span = f"{low}..={high}" if low != high else f"{low}"
        after = f"{low - paid}..={high - paid}" if low != high else f"{low - paid}"
        print(f"   {span:<15} {paid:>4}   {after}")
    print()

    print("3. Rule 23's increase, over its whole domain")
    print("   fear before   applied   fear after")
    for low, high, applied in increase_table():
        span = f"{low}..={high}" if low != high else f"{low}"
        after = f"{low + applied}..={high + applied}" if low != high else f"{low + applied}"
        print(f"   {span:<13} {applied:>7}   {after}")
    print()

    print("4. Rule 24's forfeit, over its whole domain")
    print("   satiety before   forfeited   satiety after")
    for low, high, given in forfeit_table():
        span = f"{low}..={high}" if low != high else f"{low}"
        after = f"{low - given}..={high - given}" if low != high else f"{low - given}"
        print(f"   {span:<16} {given:>9}   {after}")
    print()

    print("5. Rule 24's second dimension: where a forfeit starts being destroyed")
    print("   forfeit   from satiety   whole while recipient <=   largest discard")
    for given, satiety, whole, largest in discard_boundary():
        print(f"   {given:>7}   {satiety:>12}   {whole:>24}   {largest:>15}")
    print()

    print("6. The retained constructed-state rows: paper value against the value the engine produced")
    retained, retained_failures = retained_rows()
    for line in retained:
        print(line)
    print()
    for failure in retained_failures:
        print(f"   FAIL {failure}")
    if retained_failures:
        print()

    checked, failures, coverage = check(paths)
    print(f"7. The released capture checked against tables 1 to 5: {len(paths)} streams")
    for kind, count in sorted(coverage["kinds"].items()):
        print(f"   {kind:<20} {count:>5}")
    print(f"   {'checked':<20} {checked:>5}")
    print(f"   {'in error':<20} {disagreeing_resolutions(failures):>5}")
    print(f"   {'failing checks':<20} {len(failures):>5}")
    print()
    for failure in failures:
        print(f"   FAIL {failure}")
    if failures:
        print()

    print("8. Which table rows the runs reach")
    print("   damage dealt, by value:")
    for value in sorted(coverage["damage"]):
        print(f"     {value:>3} x{coverage['damage'][value]}")
    print("   energy actually paid, by value:")
    for value in sorted(coverage["cost"]):
        print(f"     {value:>3} x{coverage['cost'][value]}")
    print("   threat target fear before, distinct values:", len(coverage["increase"]))
    print(
        "     saturating rows reached (fear before > "
        f"{ATTRIBUTE_MAX - THREAT_FEAR_INCREASE}):",
        sum(n for f, n in coverage["increase"].items() if f > ATTRIBUTE_MAX - THREAT_FEAR_INCREASE),
    )
    print("   surrender subject satiety before, distinct values:", len(coverage["forfeit"]))
    print("     below 2, transferring nothing:", sum(n for s, n in coverage["forfeit"].items() if s < 2))
    print("   discarded, by value:")
    for value in sorted(coverage["discarded"]):
        print(f"     {value:>3} x{coverage['discarded'][value]}")
    print()

    if not checked:
        print("RESULT: FAIL -- no resolution was found, so nothing was checked")
        return 1
    if retained_failures or failures:
        print(
            f"RESULT: FAIL -- {len(retained_failures)} of"
            f" {len(DAMAGE_ROWS) + len(THREAT_ROWS) + len(SURRENDER_ROWS)} retained rows and"
            f" {disagreeing_resolutions(failures)} of {checked} released resolutions disagree,"
            f" on {len(failures)} failing checks"
        )
        return 1
    print(
        f"RESULT: PASS -- {len(DAMAGE_ROWS) + len(THREAT_ROWS) + len(SURRENDER_ROWS)} of"
        f" {len(DAMAGE_ROWS) + len(THREAT_ROWS) + len(SURRENDER_ROWS)} retained rows and"
        f" {checked} of {checked} released resolutions agree with the tables"
    )
    return 0


def main(argv: list[str]) -> int:
    if len(argv) < 2:
        raise SystemExit("usage: resolutions.py <capture directory> [--control <name>]")

    if "--control" in argv:
        name = argv[argv.index("--control") + 1]
        if name not in CONTROLS:
            raise SystemExit(f"unknown control {name}; one of {', '.join(sorted(CONTROLS))}")
        target, value = CONTROLS[name]
        globals()[target] = value
        print(f"# CONTROL {name}: {target} = {value}")
        print("# A non-zero exit naming disagreeing lines is this control's expected result.")
        print()

    root = Path(argv[1])
    paths = sorted(path for path in root.glob("*.txt"))
    if not paths:
        raise SystemExit(f"no streams in {root}")
    return report(paths)


if __name__ == "__main__":
    sys.exit(main(sys.argv))
