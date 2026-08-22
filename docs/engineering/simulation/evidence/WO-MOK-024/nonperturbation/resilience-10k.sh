#!/usr/bin/env bash
set -u
BIN=./target/debug/Mokiterions
for policy in baseline reference individual social; do
  for density in 0.75 1.50; do
    out=$($BIN --seed 123 --ticks 10000 --policy "$policy" --density "$density" 2>/dev/null)
    code=$?
    printf 'policy=%s density=%s seed=123 ticks=10000 exit=%s\n' "$policy" "$density" "$code"
    printf '  %s\n' "$(printf '%s' "$out" | tail -1)"
  done
done
