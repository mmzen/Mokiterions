"""Measure the committed transcript's per-tick record layout.

L30's replay half is "successive transcript records consumed rather than the first record three
times". What makes that measurable from outside the crate is the transcript's own shape: if the
cursor restarted each tick, the record offered at the first opportunity of tick 2 would be tick 1's
first record, and rule 12.3's tick-and-actor check would refuse it. So the figures that matter are
the per-tick record counts and the first actor of each of the first few ticks.
"""

import collections
import json
import pathlib

# Relative to the repository root, so the instrument names no absolute path and no capture it
# produces carries one. Run it from the root.
TRANSCRIPT = pathlib.Path("mokiterions-core/tests/transcript-seed0-ticks20-hunting.jsonl")

per_tick = collections.OrderedDict()
prefix = 0
for line in TRANSCRIPT.read_text(encoding="utf-8").splitlines():
    record = json.loads(line)
    kind = record.get("transcript")
    if kind != "exchange":
        prefix += 1
        continue
    tick = record["tick"]
    per_tick.setdefault(tick, []).append(record["actor"])

total = sum(len(a) for a in per_tick.values())
print(f"prefix records: {prefix}")
print(f"exchange records: {total} over {len(per_tick)} tick(s)")
print()
print("tick  records  first actor  last actor")
for tick, actors in per_tick.items():
    print(f"{tick:>4}  {len(actors):>7}  {actors[0]:<11}  {actors[-1]}")

print()
ticks = list(per_tick)
first = ticks[0]
print("If the cursor restarted at every tick, the record offered at the first opportunity of")
for tick in ticks[1:4]:
    offered = per_tick[first][0]
    reached = per_tick[tick][0]
    actor = "same actor" if offered == reached else f"actor {offered} vs {reached}"
    print(
        f"  tick {tick} would be the record for tick {first} actor {offered}, against the "
        f"opportunity tick {tick} actor {reached}"
    )
    print(f"      {actor}; tick {first} vs {tick}: the TICK is what differs")
print()
print("So the actor half of rule 12.3's check would not catch a restarted cursor on this")
print("transcript - every tick's first opportunity is M01 - and the tick half is what does.")
print()
print("Cumulative records consumed by the end of each of the first three ticks, lent versus rebuilt:")
running = 0
for tick in ticks[:3]:
    running += len(per_tick[tick])
    print(f"  tick {tick}: lent {running}, rebuilt {len(per_tick[first])}")
