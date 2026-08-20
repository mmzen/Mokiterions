#!/usr/bin/env bash
# Captures VER-MOK-012 oracle 1's declared matrix into $1.
#
# The matrix is the five declared seeds of VER-MOK-002, the three decision sources that exist before
# this change, the default density and the two swept densities, with and without --trace-actions, at
# 1,000 ticks: 90 runs, of which 30 are baseline. It is VER-MOK-011's matrix and this script is that
# work order's capture.sh unchanged but for these comments, so that the two captures are comparable
# without a reader having to diff the harness that took them.
#
# Exit codes are captured beside each stream, because VER-MOK-012 compares them too.
#
# Note what this script does NOT take: the 30 cells under --policy social. That source does not exist
# at the commit the pre-change capture is taken from, and a fourth loop value here would fail 30 runs
# with a configuration error. The social cells are a separate post-change capture, listed separately
# in VER-MOK-012's evidence retention.
#
# Run from the workspace root. The release profile is used for speed only; SPEC-MOK-001 requires
# byte-identical output from any build of one commit, and the engine contains no debug-only path.
set -u
target="$1"
mkdir -p "$target"
binary=./target/release/Mokiterions

for seed in 0 1 42 123 777; do
  for policy in baseline reference individual; do
    for density in 0.15 0.75 1.50; do
      for trace in off on; do
        name="seed${seed}-${policy}-d${density}-trace${trace}"
        if [ "$trace" = on ]; then
          "$binary" --seed "$seed" --ticks 1000 --policy "$policy" \
            --density "$density" --trace-actions > "$target/$name.txt"
        else
          "$binary" --seed "$seed" --ticks 1000 --policy "$policy" \
            --density "$density" > "$target/$name.txt"
        fi
        echo "$?" > "$target/$name.exit"
      done
    done
  done
done

echo "captured $(ls "$target" | grep -c '\.txt$') streams into $target"
