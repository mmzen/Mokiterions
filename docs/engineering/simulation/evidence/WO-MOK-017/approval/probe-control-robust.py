"""PROBE ONLY. Is the corrected world worse on unbound seeds than today's, or
comparably variable? The floors and the ceiling bind only the five declared seeds."""

import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from importlib import import_module

sweep = import_module("probe-sweep")


def survey(label, sq, count=50):
    os.environ["MOK_PROBE_SQ"] = str(sq)
    over60 = 0
    over50 = 0
    short = 0
    worst = 0.0
    lowest = {}
    for policy in sweep.POLICIES:
        for seed in range(count):
            surv, a, b, _ = sweep.cell(0, 0, True, policy, seed)
            sa, _ = sweep.share(a)
            sb, _ = sweep.share(b)
            top = max(sa, sb)
            worst = max(worst, top)
            over60 += top > 60.0
            over50 += top > 50.0
            lowest[policy] = min(lowest.get(policy, 99), surv)
            short += surv < sweep.FLOORS[policy]
    print(f"{label:<22} over-60% {over60:>3}/{3 * count}   over-50% {over50:>3}/{3 * count}   "
          f"worst {worst:5.1f}%   below floor {short:>3}/{3 * count}   lowest {lowest}")


if __name__ == "__main__":
    print("seeds 0..49 at the default density, 1,000 ticks; obligations bind only "
          "seeds 0, 1, 42, 123, 777\n")
    survey("today (uncorrected)", 0)
    survey("corrected R*R/100", 100)
