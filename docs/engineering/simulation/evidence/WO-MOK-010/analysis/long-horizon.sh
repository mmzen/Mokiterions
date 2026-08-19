#!/usr/bin/env bash
# WO-MOK-010: the tick-10,000 runs, which carry no obligation in either direction.
#
# VER-MOK-002 records the reference source reaching extinction at tick 9,154, and WO-MOK-010 asks for
# the same horizon under the new source so that the two can be compared. No target exists at this
# horizon and none is claimed; a difference is information, not a failure.
#
# The runs are untraced. Everything the long-horizon table reports -- the termination reason, the
# elapsed ticks, survivors, deaths and consumption -- comes from records the engine emits with or
# without --trace-actions, and a traced 10,000-tick run is thirty times the size for nothing. The
# properties that do need the trace are measured on the 1,000-tick runs, in full, in fear.txt and
# divergence.txt.
#
# Usage: long-horizon.sh <engine-binary> <output-directory>
set -u

BIN="$1"
OUT="$2"
SEEDS="0 1 42 123 777"
TICKS=10000

mkdir -p "$OUT/raw"

for seed in $SEEDS; do
  for policy in reference individual; do
    cell="long_seed${seed}_${policy}"
    "$BIN" --seed "$seed" --ticks "$TICKS" --policy "$policy" > "$OUT/raw/$cell.log" 2>&1
    printf '%s  exit=%s\n' "$cell" "$?" >> "$OUT/exit-codes.txt"
  done
done

echo "long-horizon cells: $(ls "$OUT"/raw/long_* | wc -l | tr -d ' ')"
