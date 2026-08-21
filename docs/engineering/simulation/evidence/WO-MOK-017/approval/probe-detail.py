"""PROBE ONLY. Per-cell detail for the two decision-relevant points, plus the
check that `baseline` does not move under any probe setting."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from importlib import import_module

sweep = import_module("probe-sweep")


def detail(label, k, high):
    print(f"\n=== {label}  (K={k}, high-class extra={high})")
    print("  policy       seed  surv  floor  A share        B share")
    per_policy_min = {}
    for policy in sweep.POLICIES:
        for seed in sweep.SEEDS:
            surv, a, b, _ = sweep.cell(k, 0, True, policy, seed, high=high)
            sa, ca = sweep.share(a)
            sb, cb = sweep.share(b)
            mark = "" if surv >= sweep.FLOORS[policy] else "  FLOOR MISSED"
            over = "" if max(sa, sb) <= 50.0 else "  over half"
            per_policy_min[policy] = min(per_policy_min.get(policy, 99), surv)
            print(f"  {policy:<11} {seed:>4}  {surv:>4}  {sweep.FLOORS[policy]:>5}  "
                  f"{sa:5.1f}% {ca:<4} {'/'.join(map(str, a)):<10} "
                  f"{sb:5.1f}% {cb:<4} {'/'.join(map(str, b)):<10}{mark}{over}")
    print(f"  minimum survivors per source: {per_policy_min}")


def baseline_invariance():
    print("\n=== baseline under every probe setting (must not move)")
    reference_lines = {}
    for seed in sweep.SEEDS:
        _, _, _, line = sweep.cell(0, 0, False, "baseline", seed)
        reference_lines[seed] = line
    moved = 0
    for k, high, shared in [(25, 10, True), (45, 10, True), (100, 50, True), (40, 0, False)]:
        for seed in sweep.SEEDS:
            _, _, _, line = sweep.cell(k, 0, shared, "baseline", seed, high=high)
            if line != reference_lines[seed]:
                moved += 1
                print(f"  MOVED: seed {seed} at K={k} H={high}")
    print(f"  baseline summaries compared: {4 * len(sweep.SEEDS)}, moved: {moved}")


if __name__ == "__main__":
    detail("best point holding all three floors", 25, 10)
    detail("closest point meeting the ceiling on all 15 cells", 45, 10)
    baseline_invariance()
