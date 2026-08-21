#!/usr/bin/env bash
# Captures VER-MOK-016's long-horizon runs into $1.
#
# The contract's performance and resilience section asks for "a 10,000-tick run under the `social`
# source" that "completes without panic and without unbounded growth in retained state", with its
# composition and survivor figures "captured as evidence rather than bound as obligations". This
# script takes that run on all five declared seeds of VER-MOK-002 rather than on one, because two of
# the five do not reach tick 10,000 and a single-seed capture would have shown only one of the two
# termination reasons.
#
# It also takes a tick ladder on seed 0 -- 1,000, 2,000, 5,000, 10,000 and 20,000 ticks of the same
# world -- which is what makes the retained-state claim measurable rather than argued: the same seed
# run for twenty times as long is the only comparison in which every difference is the tick count.
# The 20,000-tick cell is expected to be byte-identical to the 10,000-tick one, since seed 0 goes
# extinct before either limit, and `analysis/horizon.py` checks that rather than assuming it.
#
# One trace-on cell, at the seed that survives longest, carries the `suffered` field: the record is
# the only item of transient state this change adds, and a trace-off stream never shows it.
#
# Everything else is capture-social.sh's shape unchanged -- the same cell naming with the tick count
# added, the same exit code beside each stream -- so `baseline/manifest.py` digests this capture with
# no argument of its own.
#
# Run from the workspace root.
set -u
target="$1"
mkdir -p "$target"
binary=./target/release/Mokiterions

for seed in 0 1 42 123 777; do
  name="seed${seed}-social-d0.75-t10000-traceoff"
  "$binary" --seed "$seed" --ticks 10000 --policy social --density 0.75 > "$target/$name.txt"
  echo "$?" > "$target/$name.exit"
done

name=seed1-social-d0.75-t10000-traceon
"$binary" --seed 1 --ticks 10000 --policy social --density 0.75 --trace-actions > "$target/$name.txt"
echo "$?" > "$target/$name.exit"

for ticks in 1000 2000 5000 20000; do
  name="seed0-social-d0.75-t${ticks}-traceoff"
  "$binary" --seed 0 --ticks "$ticks" --policy social --density 0.75 > "$target/$name.txt"
  echo "$?" > "$target/$name.exit"
done

echo "captured $(ls "$target" | grep -c '\.txt$') streams into $target"
