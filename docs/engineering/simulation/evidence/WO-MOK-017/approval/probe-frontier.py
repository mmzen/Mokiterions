"""PROBE ONLY. The best composition the permitted surface buys with no floor missed."""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from importlib import import_module

scan = import_module("probe-scan")

points = [(k, 0, h) for k in (0, 5, 10, 15, 20, 25, 30, 35, 40, 45)
          for h in (0, 5, 10, 15)]
rows = scan.scan(points, "floor-respecting frontier")
clean = [r for r in rows if r[5] == 0]
clean.sort(key=lambda r: (r[3], r[4]))
print("\nBest points with NO floor miss (breaching cells, worst share):")
for row in clean[:10]:
    print(f"  K={row[0]:>3} H={row[2]:>3}: breach {row[3]:>2}/15, worst {row[4]:.1f}%")
if not clean:
    print("  none")
