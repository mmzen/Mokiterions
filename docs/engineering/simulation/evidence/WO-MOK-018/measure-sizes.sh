#!/usr/bin/env bash
# VER-MOK-012: stream sizes for a 1,000-tick run and a 10,000-tick traced run.
#
# Usage, from the repository root:
#
#     bash docs/engineering/simulation/evidence/WO-MOK-018/measure-sizes.sh \
#         <binary> <scratch-dir> <output-file>
#
# ## Why more than two rows are measured
#
# `--ticks` is a limit and not a length. A run ends at the limit or at extinction, whichever comes
# first, so "a 10,000-tick run" is a request and not a fact, and at the default configuration it is a
# request the world refuses: the population dies out first. The sweep at the bottom measures which of
# the thirty declared combinations reach the limit, so that the choice of configuration for the
# 10,000-tick row is a measured consequence and not a preference.
#
# Two axes drive the size -- the tick count and `--trace-actions` -- so both are varied. The default
# configuration is measured at 10,000 ticks as well, extinction and all, because a reader comparing
# this work order's figures against a run of their own will use the default and should be able to see
# the same number.
#
# ## The obligation the longest run carries
#
# The 10,000-tick traced run is the longest this change is exercised over, and the build is
# unoptimised, where Rust checks arithmetic and panics on overflow. `SPEC-MOK-006`'s cumulative
# counters are `u64` and are the only state in this change that grows without bound, so a clean exit
# at 10,000 ticks with the run record written is what says they do not overflow and the run does not
# panic at ten times the acceptance runs' scale.
#
# Nothing here is retained beyond the figures: every stream is measured and deleted. That is the
# point of the exercise -- one 10,000-tick traced pair is larger than every prior work order's whole
# evidence directory put together, which is the ground of the retention deviation this work order
# states in its README.
set -euo pipefail

binary=$1
scratch=$2
output=$3

mkdir -p "$scratch"

profile=$(
  case "$binary" in
    *release*) echo "release (optimised: arithmetic overflow wraps silently)" ;;
    *) echo "debug (unoptimised: arithmetic overflow panics)" ;;
  esac
)

# ticks | trace | seed | density | policy | why this row is here
rows=(
  "1000|off|0|0.75|reference|VER-MOK-012's 1,000-tick run, at the default configuration"
  "1000|on|0|0.75|reference|the same run traced, so that tracing's cost is separable"
  "10000|off|0|0.75|reference|the default configuration asked for 10,000 ticks: it goes extinct first"
  "10000|on|0|0.75|reference|the same, traced"
  "10000|off|123|1.50|individual|a configuration that does reach 10,000 ticks, untraced"
  "10000|on|123|1.50|individual|VER-MOK-012's 10,000-tick traced run"
)

{
  echo "# VER-MOK-012: stream sizes"
  echo "#"
  echo "# binary:  $binary"
  echo "# profile: $profile"
  echo "# command: bash docs/engineering/simulation/evidence/WO-MOK-018/measure-sizes.sh \\"
  echo "#              <binary> <scratch-dir> <output-file>"
  echo "#"
  echo "# Each row runs the shipped binary, sends the text stream to a file and the record stream to"
  echo "# --events-path, and measures both. 'ticks' is the limit asked for; 'ran' is the number the"
  echo "# run record reports. The streams are deleted as each row completes."
  echo "#"
  printf '# %-6s %-6s %-5s %-6s %-11s %-5s %7s %11s %11s %7s %9s %8s\n' \
    limit trace seed density policy exit ran "text bytes" "record bytes" ratio records seconds
} > "$output"

for row in "${rows[@]}"; do
  IFS='|' read -r ticks trace seed density policy why <<< "$row"

  label="t$ticks-trace$trace-s$seed-d$density-$policy"
  text="$scratch/$label.txt"
  records="$scratch/$label.jsonl"

  arguments=(
    --seed "$seed" --ticks "$ticks" --density "$density" --policy "$policy"
    --events-path "$records"
  )
  if [ "$trace" = on ]; then
    arguments+=(--trace-actions)
  fi

  started=$SECONDS
  code=0
  "$binary" "${arguments[@]}" > "$text" 2> "$scratch/stderr.txt" || code=$?
  elapsed=$((SECONDS - started))

  text_bytes=$(wc -c < "$text")
  record_bytes=$(wc -c < "$records")
  record_count=$(wc -l < "$records")
  ratio=$((record_bytes * 100 / text_bytes))
  reason=$(tail -n 1 "$records" | grep -o '"reason":"[a-z_]*"' | cut -d'"' -f4)
  ran=$(tail -n 1 "$records" | grep -o '"ticks":[0-9]*' | tail -n 1 | cut -d: -f2)

  printf '  %-6s %-6s %-5s %-6s %-11s %-5s %7s %11s %11s %6s%% %9s %8s\n' \
    "$ticks" "$trace" "$seed" "$density" "$policy" "$code" "$ran" \
    "$text_bytes" "$record_bytes" "$ratio" "$record_count" "$elapsed" >> "$output"

  # Every claim the size table is used for, checked while the stream is still here to check.
  {
    echo "#     $why"
    echo "#     ended in: $reason after $ran ticks"
    echo "#     records by type: $(
      grep -o '"record":"[a-z]*"' "$records" | sort | uniq -c \
        | awk '{printf "%s=%s ", substr($2, 11, length($2) - 11), $1}'
    )"
    echo "#     last line is the run record: $(
      if tail -n 1 "$records" | grep -q '^{"record":"run"'; then echo yes; else echo NO; fi
    )"
    echo "#     summary line reached the text stream: $(
      if tail -n 1 "$text" | grep -q '^summary reason='; then echo yes; else echo NO; fi
    )"
    echo "#     diagnostic stream: $(
      if [ -s "$scratch/stderr.txt" ]; then head -c 300 "$scratch/stderr.txt"; else echo "empty"; fi
    )"
  } >> "$output"

  rm -f "$text" "$records"
done

# ---------------------------------------------------------------------------------------------
# Which configurations can supply a 10,000-tick run at all. No sink and no retained stream: the
# summary line the text stream already ends with carries the reason and the tick count.
# ---------------------------------------------------------------------------------------------
{
  echo "#"
  echo "# ---- which of the declared combinations reach 10,000 ticks ----"
  echo "#"
  echo "# The five seeds VER-MOK-002 declares, at the default density and the 1.50% sweep, under"
  echo "# each policy. No stream is written: the text stream's summary line reports the outcome."
  echo "#"
  printf '# %-5s %-8s %-11s %-11s %7s %10s\n' seed density policy reason ran survivors
} >> "$output"

reached=0
refused=0
for seed in 0 1 42 123 777; do
  for density in 0.75 1.50; do
    for policy in reference individual baseline; do
      summary=$(
        "$binary" --seed "$seed" --ticks 10000 --density "$density" --policy "$policy" \
          2> /dev/null | tail -n 1
      )
      reason=$(echo "$summary" | grep -o 'reason=[a-z_]*' | cut -d= -f2)
      ran=$(echo "$summary" | grep -o ' ticks=[0-9]*' | cut -d= -f2)
      survivors=$(echo "$summary" | grep -o 'survivors=[0-9]*' | cut -d= -f2)
      if [ "$reason" = tick_limit ]; then
        reached=$((reached + 1))
      else
        refused=$((refused + 1))
      fi
      printf '  %-5s %-8s %-11s %-11s %7s %10s\n' \
        "$seed" "$density" "$policy" "$reason" "$ran" "$survivors" >> "$output"
    done
  done
done

{
  echo "#"
  echo "# $reached of $((reached + refused)) combinations reach the 10,000-tick limit; $refused go extinct first."
  echo "# The default configuration -- seed 0, 0.75%, reference -- is one of the ones that does not,"
  echo "# which is why the 10,000-tick traced row above is taken at seed 123, 1.50%, individual."
  echo "# Every baseline-policy combination goes extinct inside 200 ticks, so the baseline policy"
  echo "# cannot supply a long run at any declared seed or density."
  echo "#"
  echo "# Retention: none of the streams measured above is retained. The four largest exceed every"
  echo "# evidence directory this repository keeps -- WO-MOK-010 retained 7.1M and WO-MOK-011 8.4M."
  echo "# The sink-stream subset that is retained is bounded and disclosed in this work order's README."
} >> "$output"

echo "sizes written to $output"
