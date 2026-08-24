#!/usr/bin/env bash
#
# The candidate's record streams differ from the earlier captures in one byte, and this measures that
# claim rather than asserting it.
#
# `SPEC-MOK-006`'s ratified increment moved the header's `schema` field from 1 to 2, and this stage's
# increment moves it to 3. Nothing else in a record stream is meant to move, because `WO-MOK-025`'s
# *Out of scope* forbids any change to the four existing sources' behaviour and `REQ-MOK-068` requires
# their output byte for byte. The manifests show forty record digests that all differ and forty byte
# counts and line counts that are all identical, which is consistent with a one-byte fixed-width
# change but does not establish it.
#
# This establishes it. For each of the forty candidate streams it rewrites the first line's
# `"schema":3` to the digit an earlier capture wrote, digests the result, and compares that digest to
# the digest that capture's manifest recorded for the same cell. A match means the two streams are
# byte-identical apart from that digit; a mismatch means something else moved and names the cell.
#
# Only line 1 is rewritten, and each stream was checked to carry exactly one `"schema"` field.
#
# Usage: verify-schema-digit.sh <candidate-sink-directory> <digit> <manifest>
#   digit    1 for the base-commit capture, 2 for the post-ratification capture
#   manifest that capture's sink manifest, whose record digest column is the authority
#
# Exit status is 0 when all forty cells match and 1 otherwise, so this can be read as a check.

set -u

directory="$1"
digit="$2"
manifest="$3"

failures=0
for path in "$directory"/*.jsonl; do
    cell=$(basename "$path" .jsonl)
    rewritten=$(sed "1s/\"schema\":3/\"schema\":$digit/" "$path" | sha256sum | cut -d' ' -f1)
    recorded=$(awk -v cell="$cell" '$1 == cell { print $7 }' "$manifest")
    if [ -z "$recorded" ]; then
        printf '%-30s NO SUCH CELL IN %s\n' "$cell" "$manifest"
        failures=$((failures + 1))
    elif [ "$rewritten" = "$recorded" ]; then
        printf '%-30s identical apart from the schema digit  %s\n' "$cell" "$recorded"
    else
        printf '%-30s DIFFERS BEYOND THE SCHEMA DIGIT  recorded %s  rewritten %s\n' \
            "$cell" "$recorded" "$rewritten"
        failures=$((failures + 1))
    fi
done

printf '\n%d cells compared against schema %s, %d failures\n' \
    "$(ls "$directory"/*.jsonl | wc -l)" "$digit" "$failures"
[ "$failures" -eq 0 ]
