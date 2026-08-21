"""PROBE ONLY. Compares candidate arithmetic shapes for the corrected waste
condition against the 60% ceiling and the floors of 8, 8 and 5."""

import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from importlib import import_module

sweep = import_module("probe-sweep")

CEILING = 60.0


def measure(label, sq=0, k=0, high=0, w=0):
    os.environ["MOK_PROBE_SQ"] = str(sq)
    breaches = 0
    worst = 0.0
    misses = []
    thresholds = {}
    for restored, name in ((15, "low"), (30, "med"), (50, "high")):
        allowance = k * restored // 100 + w + (high if restored == 50 else 0) \
            + (restored * restored // sq if sq else 0)
        thresholds[name] = min(100, 100 - restored + allowance)
    for policy in sweep.POLICIES:
        for seed in sweep.SEEDS:
            surv, a, b, _ = sweep.cell(k, 0, True, policy, seed, w=w, high=high)
            sa, _ = sweep.share(a)
            sb, _ = sweep.share(b)
            worst = max(worst, sa, sb)
            if max(sa, sb) > CEILING:
                breaches += 1
            if surv < sweep.FLOORS[policy]:
                misses.append((policy, seed, surv))
    verdict = "MEETS 60% AND ALL FLOORS" if breaches == 0 and not misses else "no"
    print(f"{label:<34} eat-up-to satiety {thresholds}  "
          f"over-60% {breaches}/15  worst {worst:5.1f}%  "
          f"floor misses {len(misses)}  -> {verdict}")
    os.environ["MOK_PROBE_SQ"] = "0"
    return breaches, worst, misses


if __name__ == "__main__":
    print(f"ceiling under test: {CEILING}%  floors: reference 8, individual 8, social 5\n")
    for denominator in (60, 80, 100, 120, 150):
        measure(f"single term  R*R/{denominator}", sq=denominator)
    print()
    measure("two term     25R/100 + 10 on high", k=25, high=10)
    measure("two term     20R/100 + 10 on high", k=20, high=10)
    measure("two term     25R/100 +  8 on high", k=25, high=8)
