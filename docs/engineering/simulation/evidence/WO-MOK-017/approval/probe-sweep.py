"""PROBE ONLY - not repository evidence.

Sweeps the two permitted mechanisms of REQ-MOK-060 over the declared verification
seed set at the default density, and reports, per cell, the survivor count and the
largest class share of each territory's standing resources at tick 1,000.

Not committed, not retained: WO-MOK-017 is draft, so this measures options for the
owner's approval decision and nothing else.
"""

import os
import re
import subprocess
import sys

BIN = os.path.join("target", "release", "Mokiterions.exe")
SEEDS = [0, 1, 42, 777, 123]
POLICIES = ["reference", "individual", "social"]
FLOORS = {"reference": 8, "individual": 8, "social": 5}


def cell(k, floor, shared, policy, seed, density="0.75", ticks="1000", w=0, high=0):
    env = dict(os.environ)
    env["MOK_PROBE_HIGH"] = str(high)
    env["MOK_PROBE_W"] = str(w)
    env["MOK_PROBE_K"] = str(k)
    env["MOK_PROBE_FLOOR"] = str(floor)
    env["MOK_PROBE_SHARED"] = "1" if shared else "0"
    out = subprocess.run(
        [BIN, "--seed", str(seed), "--ticks", ticks, "--policy", policy,
         "--density", density],
        capture_output=True, text=True, env=env, check=True,
    ).stdout
    line = out.strip().splitlines()[-1]
    assert line.startswith("summary "), line
    f = dict(re.findall(r"(\w+)=([\w:.\-]+)", line))
    a = [int(f["food_a_low"]), int(f["food_a_medium"]), int(f["food_a_high"])]
    b = [int(f["food_b_low"]), int(f["food_b_medium"]), int(f["food_b_high"])]
    return int(f["survivors"]), a, b, line


def share(counts):
    total = sum(counts)
    if total == 0:
        return 0.0, "-"
    top = max(counts)
    return 100.0 * top / total, ["low", "med", "high"][counts.index(top)]


def variant(label, k, floor, shared, verbose=True, w=0, high=0):
    """One configuration, measured over every policy and seed."""
    worst_share = 0.0
    worst_cell = None
    breaches = 0
    floor_misses = []
    rows = []
    for policy in POLICIES:
        for seed in SEEDS:
            surv, a, b, _ = cell(k, floor, shared, policy, seed, w=w, high=high)
            sa, ca = share(a)
            sb, cb = share(b)
            top = max(sa, sb)
            if top > worst_share:
                worst_share, worst_cell = top, (policy, seed, ca if sa >= sb else cb)
            if sa > 50.0 or sb > 50.0:
                breaches += 1
            if surv < FLOORS[policy]:
                floor_misses.append((policy, seed, surv))
            rows.append((policy, seed, surv, sa, ca, sb, cb, a, b))
            if verbose:
                print(f"  {policy:<11} seed {seed:>3}  surv {surv:>2}  "
                      f"A {sa:5.1f}% {ca:<4} {'/'.join(map(str, a)):<10} "
                      f"B {sb:5.1f}% {cb:<4} {'/'.join(map(str, b))}")
    print(f"{label}: breaching cells {breaches}/15, worst share "
          f"{worst_share:.1f}% at {worst_cell}, floor misses {floor_misses or 'none'}")
    return breaches, worst_share, floor_misses, rows


if __name__ == "__main__":
    what = sys.argv[1] if len(sys.argv) > 1 else "sweep"
    if what == "control":
        variant("control (K=0, no floor, unshared) = master's behavior", 0, 0, False)
    elif what == "sweep":
        for k in [10, 20, 30, 40, 50, 60, 70, 80]:
            variant(f"shared K={k}", k, 0, True, verbose=False)
    elif what == "rule5only":
        for k in [40, 60, 80]:
            variant(f"rule 5 only (rule 19 literal clause 1), K={k}", k, 0, False,
                    verbose=False)
    elif what == "flooronly":
        for fl in [10, 20, 30, 40]:
            variant(f"rule 19 floor only, T floor={fl}", 0, fl, False, verbose=False)
    elif what == "fine":
        for k in [45, 55, 65, 75, 85, 90, 95, 100]:
            variant(f"shared K={k}", k, 0, True, verbose=False)
    elif what == "flat":
        for w in [5, 10, 15, 20, 30, 40, 50]:
            variant(f"shared flat W={w}", 0, 0, True, verbose=False, w=w)
    elif what == "high":
        for h in [5, 10, 15, 20, 25, 30, 35, 40, 50]:
            variant(f"shared high-class-only allowance H={h}", 0, 0, True,
                    verbose=False, high=h)
    elif what == "detail":
        variant(f"shared K={sys.argv[2]}", int(sys.argv[2]), 0, True)
