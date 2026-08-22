#!/usr/bin/env bash
# Pre-change / post-change output capture for WO-MOK-003.
# Byte-exact oracle: SHA-256 per matrix cell, plus full text for the short readable runs.
# Usage: capture.sh <output-directory>
set -u

OUT="$1"
BIN=./target/debug/Mokiterions
SEEDS="0 1 42 123 777"
POLICIES="baseline reference"
DENSITIES="0.75 1.50"
TICKS=1000

mkdir -p "$OUT/full"
: > "$OUT/manifest.txt"
: > "$OUT/summaries.txt"
: > "$OUT/exit-codes.txt"

# --- Matrix: 5 seeds x 2 policies x 2 densities x 2 trace settings, 1000 ticks ---
for seed in $SEEDS; do
  for policy in $POLICIES; do
    for density in $DENSITIES; do
      for trace in off on; do
        args="--seed $seed --ticks $TICKS --policy $policy --density $density"
        [ "$trace" = on ] && args="$args --trace-actions"
        cell="seed${seed}_${policy}_d${density}_trace${trace}"
        out=$($BIN $args 2>/dev/null)
        code=$?
        hash=$(printf '%s' "$out" | sha256sum | cut -d' ' -f1)
        lines=$(printf '%s' "$out" | wc -l | tr -d ' ')
        printf '%s  %s  lines=%s  exit=%s\n' "$hash" "$cell" "$lines" "$code" >> "$OUT/manifest.txt"
        printf '%s  %s\n' "$cell" "$(printf '%s' "$out" | tail -1)" >> "$OUT/summaries.txt"
      done
    done
  done
done

# --- Short readable runs, retained in full ---
for policy in $POLICIES; do
  cell="short_seed42_${policy}_trace_on"
  $BIN --seed 42 --ticks 20 --policy "$policy" --trace-actions > "$OUT/full/$cell.txt" 2>/dev/null
  hash=$(sha256sum < "$OUT/full/$cell.txt" | cut -d' ' -f1)
  printf '%s  %s  lines=%s  exit=0\n' "$hash" "$cell" "$(wc -l < "$OUT/full/$cell.txt" | tr -d ' ')" >> "$OUT/manifest.txt"
done

# --- Default invocation with no arguments ---
out=$($BIN 2>/dev/null); code=$?
printf '%s  %s  lines=%s  exit=%s\n' \
  "$(printf '%s' "$out" | sha256sum | cut -d' ' -f1)" "no_arguments" \
  "$(printf '%s' "$out" | wc -l | tr -d ' ')" "$code" >> "$OUT/manifest.txt"

# --- Exit codes and stderr for the invalid-input and help cases ---
run_case() {
  label="$1"; shift
  stdout=$($BIN "$@" 2>"$OUT/.stderr"); code=$?
  {
    printf '=== %s\n' "$label"
    printf 'argv:        %s\n' "$*"
    printf 'exit:        %s\n' "$code"
    printf 'stdout_sha:  %s\n' "$(printf '%s' "$stdout" | sha256sum | cut -d' ' -f1)"
    printf 'stdout_len:  %s\n' "$(printf '%s' "$stdout" | wc -c | tr -d ' ')"
    printf 'stderr:\n'
    sed 's/^/  | /' "$OUT/.stderr"
  } >> "$OUT/exit-codes.txt"
}

run_case "help"                 --help
run_case "ticks zero"           --ticks 0
run_case "density zero cells"   --density 0.01
run_case "density literal zero" --density 0
run_case "density precision"    --density 0.751
run_case "density negative"     --density -1
run_case "density over 100"     --density 101
run_case "density non-numeric"  --density abc
run_case "policy invalid"       --policy random
run_case "policy missing value" --policy
run_case "seed non-numeric"     --seed abc
run_case "unknown option"       --unknown
run_case "duplicate seed"       --seed 1 --seed 2
run_case "duplicate trace"      --trace-actions --trace-actions
run_case "ticks missing value"  --ticks
run_case "single tick"          --ticks 1 --seed 42
rm -f "$OUT/.stderr"

echo "cells: $(wc -l < "$OUT/manifest.txt" | tr -d ' ')"
