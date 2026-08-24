#!/usr/bin/env bash
# Captures REQ-MOK-068's matrix -- the four existing decision sources at every declared seed --
# into $1, for VER-MOK-018 case L9.
#
# The matrix is the five declared seeds of VER-MOK-002 (0, 1, 42, 123, 777) and all four existing
# sources (baseline, reference, individual, social) at the default density, with and without
# --trace-actions, at 1,000 ticks. That is the twenty configurations WO-MOK-025 names, observed two
# ways each: 40 cells per mode, 80 runs per full capture.
#
# Standard output, standard error and the exit code are captured separately for every cell, because
# L9 compares all three and an exit-code change is the one difference a stream comparison cannot
# see. This follows the capture.sh WO-MOK-011 established and WO-MOK-019 reused.
#
# Usage, from the workspace root:
#
#     docs/engineering/simulation/evidence/WO-MOK-025/capture.sh <target-dir> [sink]
#
# With `sink`, every run additionally receives `--events-path <target-dir>/<cell>.jsonl`, so one
# invocation produces the text stream and the record stream of the same run. Without it, no sink is
# configured and the run is the one SPEC-MOK-006 rule 1.1 requires to be indistinguishable from a
# run in a build without this capability. Both modes are captured at both commits, so the
# indistinguishability is re-established rather than assumed.
#
# The horizon of 1,000 ticks is a local decision under WO-MOK-025's Authorized decision envelope:
# REQ-MOK-068 and L9 fix the seeds and the density and leave the horizon open, and 1,000 is the
# horizon every prior capture matrix in this repository used. It is long enough that deaths occur
# under every source, which the 100-tick default does not reach on every seed.
#
# The release profile is used for speed only; SPEC-MOK-001 requires byte-identical output from any
# build of one commit, and the engine contains no debug-only path.
set -u
target="$1"
mode="${2:-nosink}"
mkdir -p "$target"
binary=./target/release/Mokiterions

for seed in 0 1 42 123 777; do
  for policy in baseline reference individual social; do
    for trace in off on; do
      name="seed${seed}-${policy}-trace${trace}"
      arguments=(--seed "$seed" --ticks 1000 --policy "$policy" --density 0.75)
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

echo "captured $(ls "$target" | grep -c '\.txt$') streams into $target (mode: $mode)"
