"""PROBE ONLY - not repository evidence.

Scans the permitted correction surface for a point that satisfies REQ-MOK-060's
ceiling on all 15 obligated cells while holding REQ-MOK-014's, REQ-MOK-034's and
REQ-MOK-058's survivor floors of 8, 8 and 5.

Reports the Pareto frontier of (breaching cells, worst class share, floor deficit).
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from importlib import import_module

sweep = import_module("probe-sweep")

FLOORS = sweep.FLOORS


def evaluate(k, w, high):
    breaches = 0
    worst = 0.0
    deficit = 0
    misses = 0
    for policy in sweep.POLICIES:
        for seed in sweep.SEEDS:
            surv, a, b, _ = sweep.cell(k, 0, True, policy, seed, w=w, high=high)
            sa, _ = sweep.share(a)
            sb, _ = sweep.share(b)
            worst = max(worst, sa, sb)
            if sa > 50.0 or sb > 50.0:
                breaches += 1
            short = FLOORS[policy] - surv
            if short > 0:
                misses += 1
                deficit = max(deficit, short)
    return breaches, worst, misses, deficit


def scan(points, title):
    print(f"\n=== {title}")
    print("   K   W   H | breach/15  worst%  floor-misses  worst-deficit")
    results = []
    for k, w, high in points:
        breaches, worst, misses, deficit = evaluate(k, w, high)
        flag = "  <-- meets ceiling AND floors" if breaches == 0 and misses == 0 else ""
        print(f" {k:>3} {w:>3} {high:>3} | {breaches:>6}    {worst:>6.1f}  "
              f"{misses:>8}       {deficit:>6}{flag}")
        results.append((k, w, high, breaches, worst, misses, deficit))
    return results


if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "high-fine":
        scan([(0, 0, h) for h in range(16, 32, 2)], "high-class-only allowance, fine")
    elif mode == "combined":
        scan([(k, 0, h) for k in (10, 20, 30) for h in (10, 20, 25, 30)],
             "proportional K combined with a high-class allowance")
    elif mode == "prop-fine":
        scan([(k, 0, 0) for k in range(40, 90, 5)], "proportional K, floor-respecting end")
    elif mode == "flat-fine":
        scan([(0, w, 0) for w in range(20, 32, 2)], "flat allowance W, fine")
