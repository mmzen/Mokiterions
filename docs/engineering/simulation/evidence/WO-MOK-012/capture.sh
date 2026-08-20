#!/usr/bin/env bash
# Captures VER-MOK-012 oracle 1's declared matrix into $1.
#
# The matrix is the five declared seeds of VER-MOK-002, all three decision sources, the default
# density and both swept densities, with and without --trace-actions, at 1,000 ticks: 90 runs.
# VER-MOK-012 declares 60 cells -- the default density and the swept `1.50%`. This script captures
# the 90-cell matrix `WO-MOK-011`'s capture.sh established, which is a superset: adding `0.15%`
# costs a third more runtime and gives the comparison a scarcer world to disagree in.
#
# Standard output, standard error and the exit code are captured separately for every cell, because
# oracle 1 compares all three and an exit-code change is the one difference a stream comparison
# cannot see.
#
# Usage, from the workspace root:
#
#     docs/engineering/simulation/evidence/WO-MOK-012/capture.sh <target-dir> [sink]
#
# With `sink`, every run additionally receives `--events-path <target-dir>/<cell>.jsonl`, so one
# invocation produces the text stream and the record stream of the same run. Without it, no sink is
# configured and the run is the one `SPEC-MOK-006` rule 1.1 requires to be indistinguishable from a
# run in a build without this capability.
#
# The release profile is used for speed only; SPEC-MOK-001 requires byte-identical output from any
# build of one commit, and the engine contains no debug-only path.
set -u
target="$1"
mode="${2:-nosink}"
mkdir -p "$target"
binary=./target/release/Mokiterions

for seed in 0 1 42 123 777; do
  for policy in baseline reference individual; do
    for density in 0.15 0.75 1.50; do
      for trace in off on; do
        name="seed${seed}-${policy}-d${density}-trace${trace}"
        arguments=(--seed "$seed" --ticks 1000 --policy "$policy" --density "$density")
        if [ "$trace" = on ]; then
          arguments+=(--trace-actions)
        fi
        if [ "$mode" = sink ]; then
          arguments+=(--events-path "$target/$name.jsonl")
        fi
        "$binary" "${arguments[@]}" > "$target/$name.txt" 2> "$target/$name.err"
        echo "$?" > "$target/$name.exit"
      done
    done
  done
done

echo "captured $(ls "$target" | grep -c '\.txt$') streams into $target (mode: $mode)"
