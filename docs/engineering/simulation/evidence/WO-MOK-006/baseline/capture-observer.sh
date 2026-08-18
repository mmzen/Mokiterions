#!/usr/bin/env bash
# Pre-change / post-change observer argument-handling capture for WO-MOK-006.
# Only cases that exit before terminal acquisition are admissible here: --help and
# every rejected input. A valid invocation needs a terminal and is not captured.
# Usage: capture-observer.sh <output-directory>
set -u
OUT="$1"
BIN=./target/debug/mokiterions-tui
mkdir -p "$OUT"
: > "$OUT/arguments.txt"

run_case() {
  label="$1"; shift
  stdout=$("$BIN" "$@" 2>"$OUT/.stderr"); code=$?
  {
    printf '=== %s\n' "$label"
    printf 'argv:        %s\n' "$*"
    printf 'exit:        %s\n' "$code"
    printf 'stdout_sha:  %s\n' "$(printf '%s' "$stdout" | sha256sum | cut -d' ' -f1)"
    printf 'stdout_len:  %s\n' "$(printf '%s' "$stdout" | wc -c | tr -d ' ')"
    printf 'stderr:\n'
    sed 's/^/  | /' "$OUT/.stderr"
  } >> "$OUT/arguments.txt"
}

run_case "help"                  --help
run_case "ticks zero"            --ticks 0
run_case "ticks missing value"   --ticks
run_case "ticks non-numeric"     --ticks abc
run_case "seed non-numeric"      --seed abc
run_case "seed missing value"    --seed
run_case "policy invalid"        --policy random
run_case "policy missing value"  --policy
run_case "density zero cells"    --density 0.01
run_case "density literal zero"  --density 0
run_case "density precision"     --density 0.751
run_case "density negative"      --density -1
run_case "density over 100"      --density 101
run_case "density non-numeric"   --density abc
run_case "speed invalid"         --speed 3
run_case "speed zero"            --speed 0
run_case "speed missing value"   --speed
run_case "speed non-numeric"     --speed abc
run_case "export missing value"  --export
run_case "unknown option"        --unknown
run_case "duplicate seed"        --seed 1 --seed 2
run_case "duplicate speed"       --speed 4 --speed 8
run_case "duplicate start-paused" --start-paused --start-paused
run_case "duplicate export"      --export a.txt --export b.txt
rm -f "$OUT/.stderr"

"$BIN" --help > "$OUT/usage.txt" 2>&1
printf 'usage_sha:  %s\n' "$(sha256sum < "$OUT/usage.txt" | cut -d' ' -f1)" >> "$OUT/arguments.txt"
echo "cases: $(grep -c '^=== ' "$OUT/arguments.txt")"
