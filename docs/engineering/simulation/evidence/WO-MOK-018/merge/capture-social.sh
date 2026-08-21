#!/usr/bin/env bash
# Captures the fourth policy's thirty cells into $1, with and without a record sink.
#
# Neither parent of this merge could take this capture. `WO-MOK-016` captured the `social` cells on
# `master` at a tree with no `--events-path`, so it has thirty text streams and no record stream;
# `WO-MOK-018` captured the record stream on this branch at a tree with no `social` policy, so its
# ninety cells reach every record shape except the three the fourth policy resolves. The merge is the
# first tree in which the two exist together, and this is the capture that covers the intersection.
#
# The matrix is `WO-MOK-016`'s `capture-social.sh` unchanged in shape -- the five declared seeds of
# `VER-MOK-002` x `--policy social` x the default density and the two swept densities x with and
# without `--trace-actions`, at 1,000 ticks -- with two additions that script had no use for:
# standard error is captured beside standard output, and `sink` adds `--events-path`. Both are
# `WO-MOK-018/capture.sh`'s form, so `analysis/digest.py` reads either capture with no argument of its
# own and the cells sort into the ninety.
#
# Usage, from the workspace root:
#
#     docs/engineering/simulation/evidence/WO-MOK-018/merge/capture-social.sh <target-dir> [sink]
#
# The release profile is used for speed only; SPEC-MOK-001 requires byte-identical output from any
# build of one commit, and the engine contains no debug-only path.
set -u
target="$1"
mode="${2:-nosink}"
mkdir -p "$target"
binary=./target/release/Mokiterions

for seed in 0 1 42 123 777; do
  for density in 0.15 0.75 1.50; do
    for trace in off on; do
      name="seed${seed}-social-d${density}-trace${trace}"
      arguments=(--seed "$seed" --ticks 1000 --policy social --density "$density")
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
