"""PROBE ONLY. Detail and robustness for the candidate corrected condition
    S + R <= 100, or else S + R - 100 <= R * R / 100
which is rule 19's tolerant test at a tolerance equal to the restoration itself."""

import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from importlib import import_module

sweep = import_module("probe-sweep")

os.environ["MOK_PROBE_SQ"] = "100"


def detail():
    print("=== the five declared seeds, default density, 1,000 ticks")
    print("  policy       seed  surv  floor  worst class share   A counts     B counts")
    minimum = {}
    worst_overall = 0.0
    for policy in sweep.POLICIES:
        for seed in sweep.SEEDS:
            surv, a, b, _ = sweep.cell(0, 0, True, policy, seed)
            sa, ca = sweep.share(a)
            sb, cb = sweep.share(b)
            top, cls = (sa, ca) if sa >= sb else (sb, cb)
            worst_overall = max(worst_overall, top)
            minimum[policy] = min(minimum.get(policy, 99), surv)
            flag = "" if surv >= sweep.FLOORS[policy] else "  FLOOR MISSED"
            print(f"  {policy:<11} {seed:>4}  {surv:>4}  {sweep.FLOORS[policy]:>5}  "
                  f"{top:5.1f}% {cls:<5}          "
                  f"{'/'.join(map(str, a)):<12} {'/'.join(map(str, b)):<12}{flag}")
    print(f"  worst class share over all 15 cells: {worst_overall:.1f}%")
    print(f"  minimum survivors per source: {minimum} against floors {sweep.FLOORS}")


def trait_still_live():
    print("\n=== does the trait still change a decision? (reference vs individual)")
    same = 0
    for seed in sweep.SEEDS:
        _, _, _, ref = sweep.cell(0, 0, True, "reference", seed)
        _, _, _, ind = sweep.cell(0, 0, True, "individual", seed)
        identical = ref == ind
        same += identical
        print(f"  seed {seed:>3}: summaries {'IDENTICAL - trait inert' if identical else 'differ - trait live'}")
    print(f"  identical summaries: {same}/5 (5 would mean waste_tolerance no longer "
          f"changes any outcome)")


def robustness(count=50):
    print(f"\n=== robustness: seeds 0..{count - 1}, not an obligation, checked for curve-fit")
    over = []
    short = []
    worst = 0.0
    for policy in sweep.POLICIES:
        for seed in range(count):
            surv, a, b, _ = sweep.cell(0, 0, True, policy, seed)
            sa, _ = sweep.share(a)
            sb, _ = sweep.share(b)
            worst = max(worst, sa, sb)
            if max(sa, sb) > 60.0:
                over.append((policy, seed, round(max(sa, sb), 1)))
            if surv < sweep.FLOORS[policy]:
                short.append((policy, seed, surv))
    print(f"  cells measured: {3 * count}")
    print(f"  worst class share: {worst:.1f}%")
    print(f"  cells above 60%: {len(over)}  {over[:8]}")
    print(f"  cells below the floor: {len(short)}  {short[:8]}")


if __name__ == "__main__":
    detail()
    trait_still_live()
    robustness()
