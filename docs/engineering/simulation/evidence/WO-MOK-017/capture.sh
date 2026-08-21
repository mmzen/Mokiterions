#!/usr/bin/env bash
# Captures VER-MOK-016 oracle 1's declared matrix into $1, at either of this work order's commits.
#
# The matrix is the five declared seeds of VER-MOK-002, the four decision sources, the default density
# and the two swept densities, with and without --trace-actions, at 1,000 ticks: 120 runs, of which 30
# are baseline.
#
# This is WO-MOK-016's capture.sh with one change: `social` is a fourth loop value here rather than a
# separate script. That work order needed two scripts because `social` did not exist at its pre-change
# commit and thirty runs would have failed with a configuration error. It exists at both of this work
# order's commits, so one script takes the whole matrix and the two captures are directly comparable.
# The cell naming, the loop order and the exit code beside each stream are unchanged, so the 120 cells
# sort into WO-MOK-016's 90 plus its 30 social cells and one manifest reader digests any of them.
#
# The obligated subsets, stated so a reader does not have to derive them:
#   * 30 baseline cells      -- byte-identical across the change. REQ-MOK-060's load-bearing constraint.
#   * 60 reference and individual cells -- expected to diverge, each divergence characterized.
#   * 30 social cells        -- expected to diverge, since rule 26 delegates eating and seeking to
#                               rule 19, whose test this change amends.
#
# Exit codes are captured beside each stream, because VER-MOK-016 compares them too.
#
# Run from the workspace root. The release profile is used for speed only; SPEC-MOK-001 requires
# byte-identical output from any build of one commit, and the engine contains no debug-only path.
set -u
target="$1"
mkdir -p "$target"
binary=./target/release/Mokiterions

for seed in 0 1 42 123 777; do
  for policy in baseline reference individual social; do
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
