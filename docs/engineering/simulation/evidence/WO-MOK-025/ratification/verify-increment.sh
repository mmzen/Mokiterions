#!/usr/bin/env bash
# Measures that SPEC-MOK-006's ratified schema increment changes one integer in the header record
# of every stream and nothing else.
#
# The amendment row's claim is narrow and checkable: `schema` becomes 2, and no other byte of any
# stream moves. Asserting that from a two-line diff is weaker than measuring it, because the claim
# is about output rather than about source. So this re-runs the whole of REQ-MOK-068's matrix with a
# record sink and, for every one of the forty cells, checks three things against the base-commit
# manifests taken at cc5418553cb433715b7d6b15dea3886bff30ffaa:
#
#   1. The text stream's sha256 is UNCHANGED. `schema` appears in no text line, so any movement here
#      would mean the increment reached something it has no business reaching.
#   2. The record stream's sha256 has MOVED. If it had not, the increment did not take effect and
#      the check above would be passing for the wrong reason.
#   3. The record stream with `"schema":2` reverted to `"schema":1` on its first line only is
#      byte-identical to the base capture's record stream. This is the load-bearing one: it says the
#      whole of the difference is that one integer, on that one line, in that one record.
#
# Check 3 is why this script exists rather than a digest comparison. A differing digest names no
# byte; a reverted digest that matches names every byte at once.
#
# Peak disk is one cell, not the 183 MB the full sink capture occupies: each stream is hashed and
# deleted before the next runs. That is the only reason this is affordable to commit as a check.
#
# Usage, from the workspace root, after `cargo build --release`:
#
#     docs/engineering/simulation/evidence/WO-MOK-025/ratification/verify-increment.sh <scratch-dir>
#
# Written to standard output; redirect it. Exits 1 if any cell fails any of the three checks, so it
# is usable as a gate and not only as a report.
set -u
scratch="$1"
mkdir -p "$scratch"
binary=./target/release/Mokiterions
base=docs/engineering/simulation/evidence/WO-MOK-025/base

printf '# schema increment: confined to one integer in the header record\n'
printf '# base commit cc5418553cb433715b7d6b15dea3886bff30ffaa, base schema 1, this tree schema 2\n'
printf '# cell  text-stream  record-stream  reverted-vs-base\n'
printf '\n'

failures=0
for seed in 0 1 42 123 777; do
  for policy in baseline reference individual social; do
    for trace in off on; do
      name="seed${seed}-${policy}-trace${trace}"
      arguments=(--seed "$seed" --ticks 1000 --policy "$policy" --density 0.75)
      if [ "$trace" = on ]; then
        arguments+=(--trace-actions)
      fi
      arguments+=(--events-path "$scratch/$name.jsonl")
      "$binary" "${arguments[@]}" > "$scratch/$name.txt" 2>"$scratch/$name.err"

      # 1. The text stream, against the base no-sink manifest.
      text_now=$(sha256sum "$scratch/$name.txt" | cut -d' ' -f1)
      text_base=$(grep -E "^$name " "$base/nosink-manifest.txt" | awk '{print $2}')
      if [ "$text_now" = "$text_base" ]; then text_verdict=unchanged; else text_verdict=MOVED; fi

      # 2 and 3. The record stream, raw and with the header's schema reverted.
      records_now=$(sha256sum "$scratch/$name.jsonl" | cut -d' ' -f1)
      records_base=$(grep -E "^$name " "$base/sink-manifest.txt" | awk '{print $7}')
      reverted=$(sed '1s/"schema":2/"schema":1/' "$scratch/$name.jsonl" | sha256sum | cut -d' ' -f1)
      if [ "$records_now" != "$records_base" ]; then records_verdict=moved; else records_verdict=UNMOVED; fi
      if [ "$reverted" = "$records_base" ]; then reverted_verdict=identical; else reverted_verdict=DIFFERS; fi

      if [ "$text_verdict" != unchanged ] || [ "$records_verdict" != moved ] \
        || [ "$reverted_verdict" != identical ]; then
        failures=$((failures + 1))
      fi
      printf '%-26s %-11s %-14s %s\n' "$name" "$text_verdict" "$records_verdict" "$reverted_verdict"

      rm -f "$scratch/$name.jsonl" "$scratch/$name.txt" "$scratch/$name.err"
    done
  done
done

printf '\n'
printf 'cells: 40   failures: %s\n' "$failures"
printf '\n'
printf 'Expected reading: every cell unchanged / moved / identical. "unchanged" says the increment\n'
printf 'did not reach the text stream, "moved" says it reached the record stream, and "identical"\n'
printf 'says it reached nothing in the record stream except the one integer on the header line.\n'
[ "$failures" -eq 0 ] || exit 1
