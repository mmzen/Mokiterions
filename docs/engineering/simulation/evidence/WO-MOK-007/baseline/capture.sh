#!/usr/bin/env bash
# WO-MOK-007 / VER-MOK-007 oracle 1: capture the declared matrix from one engine binary.
#
# The matrix is the one VER-MOK-007 oracle 1 declares: the seeds 0, 1, 42, 123 and 777; the frozen
# decision sources `baseline` and `reference`; the default density 0.75% and the swept density 1.50%
# that VER-MOK-002 declares; --ticks 1000; with and without --trace-actions. That is 40 cells.
#
# The post-change run of this script also captures the same grid under the new `individual` source,
# twice per cell, which is the determinism evidence REQ-MOK-009 requires of the third source. The
# pre-change binary has no such source, so those cells are skipped when it is absent.
#
# This script only writes raw output. Hashing, the projection and the comparison are done by
# compare.py, so that the projection exists in exactly one place and is reviewable on its own.
#
# Usage: capture.sh <engine-binary> <output-directory>
set -u

BIN="$1"
OUT="$2"
SEEDS="0 1 42 123 777"
DENSITIES="0.75 1.50"
TICKS=1000

mkdir -p "$OUT/raw"
: > "$OUT/exit-codes.txt"

capture() {
  cell="$1"; shift
  "$BIN" "$@" > "$OUT/raw/$cell.log" 2> "$OUT/raw/$cell.err"
  printf '%s  exit=%s  stderr_bytes=%s  argv=%s\n' \
    "$cell" "$?" "$(wc -c < "$OUT/raw/$cell.err" | tr -d ' ')" "$*" >> "$OUT/exit-codes.txt"
}

# --- the declared matrix, under the two frozen sources ---
for seed in $SEEDS; do
  for policy in baseline reference; do
    for density in $DENSITIES; do
      capture "seed${seed}_${policy}_d${density}_traceoff" \
        --seed "$seed" --ticks "$TICKS" --policy "$policy" --density "$density"
      capture "seed${seed}_${policy}_d${density}_traceon" \
        --seed "$seed" --ticks "$TICKS" --policy "$policy" --density "$density" --trace-actions
    done
  done
done

# --- the same grid under the new source, captured twice per cell for determinism ---
if "$BIN" --policy individual --ticks 1 > /dev/null 2>&1; then
  for seed in $SEEDS; do
    for density in $DENSITIES; do
      for pass in a b; do
        capture "seed${seed}_individual_d${density}_traceoff_$pass" \
          --seed "$seed" --ticks "$TICKS" --policy individual --density "$density"
        capture "seed${seed}_individual_d${density}_traceon_$pass" \
          --seed "$seed" --ticks "$TICKS" --policy individual --density "$density" --trace-actions
      done
    done
  done
else
  echo "no individual source in this binary: the new-source cells are skipped" >> "$OUT/exit-codes.txt"
fi

# --- short readable runs, retained in full rather than as a digest ---
capture "short_seed42_reference_traceon" --seed 42 --ticks 20 --policy reference --trace-actions
if "$BIN" --policy individual --ticks 1 > /dev/null 2>&1; then
  capture "short_seed42_individual_traceon" --seed 42 --ticks 20 --policy individual --trace-actions
fi

# --- the no-argument default, which is a cell of its own under SPEC-MOK-001's Inputs section ---
capture "no_arguments"

rm -f "$OUT"/raw/*.err
echo "cells: $(ls "$OUT/raw" | wc -l | tr -d ' ')"
