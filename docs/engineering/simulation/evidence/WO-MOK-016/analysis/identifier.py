"""VER-MOK-016 oracle 5: the per-identifier series, the correlations, and the turn-position bound.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-016/analysis/identifier.py \
        sweep <binary> <seed-count> <out.json>
    python docs/engineering/simulation/evidence/WO-MOK-016/analysis/identifier.py \
        tables <in.json>

`sweep` runs the release binary once per seed under `--policy social` at 1,000 ticks and the default
density, and writes, per seed, three twelve-element series: survival to the tick limit as 0 or 1,
attacks applied, and attacks suffered. It also prints the run's event census, which is what the totals
in `identifier.md` are quoted from. `tables` reads that file and emits every figure `identifier.md`
records.

Give the binary as an absolute path or with a `./` prefix. A bare relative path fails on Windows with
`WinError 2` even where the file plainly exists, and a sweep that cannot start is better than one that
silently measures a different binary.

**The three readers are the same patterns `tests/viability.rs` uses**, deliberately, so that the
in-process test and this out-of-process sweep are measuring one thing:

  survival   the absence of any `subject=<id> event=agent_died` record, not a summary count, because
             the count says how many lived and this needs to know which;
  applied    `subject=<id> event=attack_resolved`;
  suffered   `event=attack_resolved result=target:<id>,` -- the `event=` prefix is required. Without
             it the pattern also matches every other verb's `result=target:` field and over-counts by
             about a factor of three, which is caught by applied and suffered having to balance.

A `fight` resolves as a strike and reports `attack_resolved`, so the two strike series are strikes
dealt and taken however they were proposed. That the two totals balance is asserted here rather than
assumed.

The rank correlation is Spearman with midrank ties, transcribed from `viability.rs`'s
`rank_correlation`, including its convention that a series with no spread correlates at 0.0.
"""

import json
import subprocess
import sys

SEEDS = 1000
TICKS = 1000
DECLARED = [0, 1, 42, 123, 777]
DIAGNOSTIC = 200


def identifiers():
    return [f"M{index + 1:02}" for index in range(12)]


CENSUS = ["attack_resolved", "threat_resolved", "surrender_resolved", "agent_died"]


def one_run(binary, seed, census=None):
    """The three series for one seed, read from one run's event stream."""
    completed = subprocess.run(
        [binary, "--seed", str(seed), "--ticks", str(TICKS), "--policy", "social"],
        capture_output=True,
        text=True,
        check=True,
    )
    stream = completed.stdout
    if census is not None:
        for event in CENSUS:
            census[event] += stream.count(f"event={event}")
        census["struck to death"] += stream.count("target_died:yes")
    survivals, applied, suffered = [], [], []
    for identifier in identifiers():
        survivals.append(0 if f"subject={identifier} event=agent_died" in stream else 1)
        applied.append(stream.count(f"subject={identifier} event=attack_resolved"))
        suffered.append(stream.count(f"event=attack_resolved result=target:{identifier},"))
    if sum(applied) != sum(suffered):
        raise SystemExit(
            f"seed {seed}: {sum(applied)} strikes applied against {sum(suffered)} suffered; "
            "every strike has exactly one target, so a reader that disagrees is wrong"
        )
    return [survivals, applied, suffered]


def sweep(binary, count, out_path):
    rows = []
    census = dict.fromkeys(CENSUS + ["struck to death"], 0)
    for seed in range(count):
        rows.append(one_run(binary, seed, census))
        if (seed + 1) % 100 == 0:
            print(f"{seed + 1}/{count}", file=sys.stderr)
    with open(out_path, "w", encoding="utf-8") as handle:
        json.dump(rows, handle)
    print(f"{len(rows)} seeds written to {out_path}")
    print("event census over the whole sweep:")
    for event, total in census.items():
        print(f"  {event:<20} {total:>7}")


def rank_correlation(series):
    """Spearman with midrank ties, as `viability.rs` computes it."""
    count = len(series)
    order = sorted(range(count), key=lambda index: series[index])
    ranks = [0.0] * count
    index = 0
    while index < count:
        last = index
        while last + 1 < count and series[order[last + 1]] == series[order[index]]:
            last += 1
        midrank = (index + last) / 2 + 1
        for position in range(index, last + 1):
            ranks[order[position]] = midrank
        index = last + 1
    mean = (count + 1) / 2
    numerator = sum((ranks[i] - mean) * ((i + 1) - mean) for i in range(count))
    left = sum((rank - mean) ** 2 for rank in ranks) ** 0.5
    right = sum(((i + 1) - mean) ** 2 for i in range(count)) ** 0.5
    if left == 0 or right == 0:
        return 0.0
    return numerator / (left * right)


def totals(rows, seeds):
    """The three series summed over the given seeds."""
    out = [[0] * 12 for _ in range(3)]
    for seed in seeds:
        for which, series in enumerate(rows[seed]):
            for index, value in enumerate(series):
                out[which][index] += value
    return out


def by_turn_position(series):
    """Folded onto turn position within a Mokiterion's own territory: M01-M06 in A, M07-M12 in B."""
    pooled = [0] * 6
    for index, value in enumerate(series):
        pooled[index % 6] += value
    return pooled


NAMES = ["survivals", "attacks applied", "attacks suffered"]


def report(rows, label, seeds):
    print(f"\n=== {label}: {len(seeds)} seeds ===")
    series = totals(rows, seeds)
    print(f"{'series':<17} {'M01 … M12':<44} {'total':>6} {'vs id':>7} {'vs pos':>7}")
    for name, values in zip(NAMES, series):
        print(
            f"{name:<17} {str(values):<44} {sum(values):>6} "
            f"{rank_correlation(values):>+7.3f} "
            f"{rank_correlation(by_turn_position(values)):>+7.3f}"
        )
    print(f"{'':<17} pooled by turn position 1 … 6")
    for name, values in zip(NAMES, series):
        print(f"{name:<17} {by_turn_position(values)}")
    opportunities = len(seeds) * 2
    rates = [count / opportunities for count in by_turn_position(series[0])]
    print("survival rate      " + "  ".join(f"{position + 1}:{rate:.4f}" for position, rate in enumerate(rates)))
    print(
        f"ratio highest/lowest {max(rates) / min(rates):.4f}, "
        f"last/first {rates[5] / rates[0]:.4f}"
    )
    return max(rates) / min(rates), rates[5] / rates[0]


def groups(rows, size):
    """The bound's statistic and the direction diagnostic over disjoint groups of `size` seeds."""
    spreads, directions = [], []
    for group in range(len(rows) // size):
        seeds = range(group * size, (group + 1) * size)
        rates = [
            count / (size * 2) for count in by_turn_position(totals(rows, seeds)[0])
        ]
        spreads.append(max(rates) / min(rates))
        directions.append(rates[5] / rates[0])
    return spreads, directions


def tables(in_path):
    with open(in_path, encoding="utf-8") as handle:
        rows = json.load(handle)
    report(rows, "the five declared seeds", DECLARED)
    report(rows, f"the declared diagnostic set, 0-{DIAGNOSTIC - 1}", range(DIAGNOSTIC))
    report(rows, "the whole sweep", range(len(rows)))

    print("\n=== stability by group size ===")
    print(f"{'size':>5} {'groups':>7} {'spread low':>11} {'spread high':>12} "
          f"{'breaching 1.25':>15} {'last>first':>11}")
    for size in (50, 100, 200, 250, 500):
        if len(rows) % size:
            continue
        spreads, directions = groups(rows, size)
        print(
            f"{size:>5} {len(spreads):>7} {min(spreads):>11.3f} {max(spreads):>12.3f} "
            f"{sum(1 for value in spreads if value >= 1.25):>15} "
            f"{sum(1 for value in directions if value > 1):>7}/{len(directions)}"
        )


def main(argv):
    if len(argv) == 5 and argv[1] == "sweep":
        sweep(argv[2], int(argv[3]), argv[4])
    elif len(argv) == 3 and argv[1] == "tables":
        tables(argv[2])
    else:
        raise SystemExit(__doc__)


if __name__ == "__main__":
    main(sys.argv)
