#!/usr/bin/env bash
# Captures VER-MOK-016's new `social` cells into $1.
#
# The thirty cells capture.sh cannot take: the five declared seeds of VER-MOK-002 x --policy social x
# the default density and the two swept densities x with and without --trace-actions, at 1,000 ticks.
# They are a separate script rather than a fourth loop value in capture.sh because that script runs at
# both commits and this source exists at only one of them; a fourth value there would fail thirty runs
# with a configuration error at the baseline.
#
# Everything else is capture.sh's shape unchanged -- the same loop order, the same cell naming, the
# same exit code beside each stream -- so that the thirty cells sort into the ninety and one manifest
# reader digests either capture.
#
# Run from the workspace root.
set -u
target="$1"
mkdir -p "$target"
binary=./target/release/Mokiterions

for seed in 0 1 42 123 777; do
  for density in 0.15 0.75 1.50; do
    for trace in off on; do
      name="seed${seed}-social-d${density}-trace${trace}"
      if [ "$trace" = on ]; then
        "$binary" --seed "$seed" --ticks 1000 --policy social \
          --density "$density" --trace-actions > "$target/$name.txt"
      else
        "$binary" --seed "$seed" --ticks 1000 --policy social \
          --density "$density" > "$target/$name.txt"
      fi
      echo "$?" > "$target/$name.exit"
    done
  done
done

echo "captured $(ls "$target" | grep -c '\.txt$') streams into $target"
