"""VER-MOK-012's long-horizon oracle, second half: the retained state no stream shows.

Usage, from the repository root, on Windows:

    python docs/engineering/simulation/evidence/WO-MOK-012/analysis/peak.py
    python docs/engineering/simulation/evidence/WO-MOK-012/analysis/peak.py --control

`analysis/horizon.py` reconstructs the three retained collections a stream reveals -- the
Mokiterions, the food and the suffered-attack record. Two things it cannot see: the previous tick's
decision snapshots, which no event reports, and the collected-event vector the text host leaves
absent. Both are retained state, so both belong to the contract's "without unbounded growth in
retained state", and the only instrument that reaches them is the process's own memory.

This script runs the same world at five tick limits and reports each run's **peak working set**,
read from `GetProcessMemoryInfo` on the child's handle after it exits. That figure is a ceiling
over the whole run rather than a sample, which is what a growth claim needs: a sampler could miss
a peak between samples, and a final-state reading would miss anything freed before exit.

**The criterion is a ratio, not a limit.** Twenty times the ticks is the only difference between
the first cell and the last, so if any retained structure grew with ticks the peak would grow with
it. The check is that the largest peak is within 5% of the smallest across a 20-fold range. The
same cell is also run three times, so a reader can see the measurement's own noise band beside the
figure being read; the band is reported rather than subtracted.

**The instrument is controlled.** `--control` runs a child that allocates a known amount instead of
the engine, at two sizes, so the reported peak is shown to move when memory really is retained.
Without that, a flat column measures nothing: it could be a broken reading.

Windows only. The figure has no portable equivalent, and this is where the capture was taken.
"""

import ctypes
import ctypes.wintypes as w
import os
import subprocess
import sys
import time

sys.stdout.reconfigure(encoding="utf-8")

BINARY = os.path.join("target", "release", "Mokiterions")
LADDER = (1000, 2000, 5000, 10000, 20000)
SEED = 0
DENSITY = "0.75"
REPEATS = 3

# A 20-fold tick range against a 5% memory band. Stated here rather than buried in the comparison.
TOLERANCE = 1.05


class PROCESS_MEMORY_COUNTERS(ctypes.Structure):
    """`psapi.h`. Only `PeakWorkingSetSize` is read; the rest fixes the layout."""

    _fields_ = [
        ("cb", w.DWORD),
        ("PageFaultCount", w.DWORD),
        ("PeakWorkingSetSize", ctypes.c_size_t),
        ("WorkingSetSize", ctypes.c_size_t),
        ("QuotaPeakPagedPoolUsage", ctypes.c_size_t),
        ("QuotaPagedPoolUsage", ctypes.c_size_t),
        ("QuotaPeakNonPagedPoolUsage", ctypes.c_size_t),
        ("QuotaNonPagedPoolUsage", ctypes.c_size_t),
        ("PagefileUsage", ctypes.c_size_t),
        ("PeakPagefileUsage", ctypes.c_size_t),
    ]


def measure(command: list[str]) -> tuple:
    """Run `command` with its output discarded; return `(exit code, seconds, peak KiB)`.

    The stream goes to the null device because the figure of interest is what the child retains,
    not what it prints, and a 4 MB write per cell would put the cost of the file system into the
    seconds column. The seconds are reported for scale only; nothing here is a timing bound.
    """
    with open(os.devnull, "wb") as sink:
        started = time.perf_counter()
        child = subprocess.Popen(command, stdout=sink, stderr=subprocess.DEVNULL)
        code = child.wait()
        elapsed = time.perf_counter() - started
        counters = PROCESS_MEMORY_COUNTERS()
        counters.cb = ctypes.sizeof(counters)
        ok = ctypes.windll.psapi.GetProcessMemoryInfo(
            int(child._handle), ctypes.byref(counters), counters.cb
        )
    if not ok:
        raise SystemExit("GetProcessMemoryInfo failed on the child handle")
    return code, elapsed, counters.PeakWorkingSetSize // 1024


def run(ticks: int) -> tuple:
    return measure(
        [
            BINARY,
            "--seed",
            str(SEED),
            "--ticks",
            str(ticks),
            "--policy",
            "social",
            "--density",
            DENSITY,
        ]
    )


def allocating(mib: int) -> tuple:
    """A child that retains a known amount, to show the instrument reads growth."""
    return measure(
        [
            sys.executable,
            "-c",
            f"held = bytearray({mib} * 1024 * 1024); held[::4096] = b'x' * len(held[::4096])",
        ]
    )


def report() -> int:
    if not os.path.exists(BINARY) and not os.path.exists(BINARY + ".exe"):
        raise SystemExit(f"{BINARY} not built; run cargo build --release --locked")

    print(f"1. The tick ladder, seed {SEED}, --policy social --density {DENSITY}")
    print(f"   {'ticks':>7} {'exit':>4} {'seconds':>8} {'peak KiB':>9}")
    peaks = {}
    for ticks in LADDER:
        code, elapsed, peak = run(ticks)
        peaks[ticks] = peak
        print(f"   {ticks:>7} {code:>4} {elapsed:>8.2f} {peak:>9}")
        if code != 0:
            print(f"   RESULT: FAIL -- {ticks} ticks exited {code}")
            return 1

    print()
    print(f"2. The same cell {REPEATS} times, for the measurement's own noise band")
    print(f"   {'run':>7} {'exit':>4} {'seconds':>8} {'peak KiB':>9}")
    repeats = []
    for index in range(REPEATS):
        code, elapsed, peak = run(LADDER[-2])
        repeats.append(peak)
        print(f"   {index + 1:>7} {code:>4} {elapsed:>8.2f} {peak:>9}")

    low, high = min(peaks.values()), max(peaks.values())
    ratio = high / low
    band = max(repeats) - min(repeats)
    print()
    print("3. The comparison")
    print(f"   ticks spanned                     {LADDER[0]} to {LADDER[-1]}, a factor of {LADDER[-1] // LADDER[0]}")
    print(f"   peak working set spanned          {low} to {high} KiB, a factor of {ratio:.4f}")
    print(f"   tolerance                         {TOLERANCE}")
    print(f"   noise band, same cell {REPEATS} times    {band} KiB")
    print(f"   ladder spread                     {high - low} KiB")

    print()
    if ratio >= TOLERANCE:
        print(f"RESULT: FAIL -- the peak grew by a factor of {ratio:.4f} over the ladder")
        return 1
    print(
        f"RESULT: PASS -- {LADDER[-1] // LADDER[0]}x the ticks moves the peak working set by "
        f"{high - low} KiB, a factor of {ratio:.4f} against a tolerance of {TOLERANCE}; "
        f"the same cell {REPEATS} times moves it by {band} KiB"
    )
    return 0


def control() -> int:
    print("CONTROL: a child that retains a known amount, in place of the engine")
    print(f"   {'MiB held':>9} {'exit':>4} {'peak KiB':>9}")
    peaks = {}
    for mib in (16, 64):
        code, _, peak = allocating(mib)
        peaks[mib] = peak
        print(f"   {mib:>9} {code:>4} {peak:>9}")
    _, _, engine = run(LADDER[-2])
    print(f"   {'engine':>9} {0:>4} {engine:>9}")
    print()
    grew = peaks[64] - peaks[16]
    print(f"   48 MiB more held moves the peak by {grew} KiB")
    if grew < 40 * 1024:
        print("RESULT: FAIL -- the instrument did not read the growth it was given")
        return 1
    print("RESULT: PASS -- the instrument reads retained memory, so a flat column is a finding")
    return 0


def main(argv: list[str]) -> int:
    if "--control" in argv:
        return control()
    return report()


if __name__ == "__main__":
    sys.exit(main(sys.argv))
