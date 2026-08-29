#!/usr/bin/env bash
# WO-MOK-026 evidence items 7, 9, 10 and 13: the gate matrix, the retry path, the ceiling stop and
# the pass-through, driven against the release binaries at this candidate.
#
# No provider, no network and no real credential: the connector is the canned fixture and the
# credential value is this script's own invention, the same value the test suite uses.
set -u

ENGINE=target/release/Mokiterions.exe
CONNECTOR=target/release/canned-connector.exe
OUT=target/wo026-cases
CRED='sk-canned-0000-authenticates-nothing'
CREDVAR=MOKITERIONS_TEST_CREDENTIAL
SCRIPTVAR=MOKITERIONS_CANNED_SCRIPT
PRICES='125:13:1000:0'

rm -rf "$OUT"
mkdir -p "$OUT"

# run <case> <script-lines-file-or-"-"> <credential:yes|no> <extra args...>
live() {
  case_name=$1; shift
  scriptfile=$1; shift
  cred=$1; shift
  d="$OUT/$case_name"
  mkdir -p "$d"
  args=(--policy llm --live --seed 42 --ticks 1 --spend-ceiling "${CEILING:-2}" --prices "$PRICES"
        --connector-path "$CONNECTOR" --transcript-output "$d/transcript.jsonl")
  if [ "$cred" = yes ]; then export "$CREDVAR=$CRED"; else unset "$CREDVAR"; fi
  if [ "$scriptfile" = "-" ]; then unset "$SCRIPTVAR"; else export "$SCRIPTVAR=$scriptfile"; fi
  "$ENGINE" "${args[@]}" "$@" > "$d/stdout.txt" 2> "$d/stderr.txt"
  echo $? > "$d/exit.txt"
  unset "$CREDVAR"; unset "$SCRIPTVAR"
  printf '%-22s exit=%s stdout=%s stderr=%s\n' "$case_name" "$(cat "$d/exit.txt")" \
    "$(wc -c < "$d/stdout.txt")" "$(wc -c < "$d/stderr.txt")"
}

# ---- gate rows 3 and 4, and the recording rows 1 and 2 replay ----
printf 'ok wait\n' > "$OUT/script-ok"
printf 'credential %s\n' "$CREDVAR" > "$OUT/script-credential"
printf 'ok wait prompt=1000 cached=1000 output=987 reasoning=0\n' > "$OUT/script-whole-cent"
printf 'error transport the socket closed\nerror transport the socket closed\nok wait\n' > "$OUT/script-retry-two"
printf 'error transport the socket closed\n' > "$OUT/script-retry-always"

live row4-selected-credentialled "$OUT/script-ok"         yes --events-path "$OUT/row4-selected-credentialled/records.jsonl"
live row3-selected-uncredentialled "$OUT/script-credential" no

# Rows 1 and 2: no live selection. The connector path names a program that cannot exist, so a host
# that reached the platform with it would fail loudly; the decisions come from row 4's transcript.
unstartable="$OUT/no-such-connector"
for pair in row1-unselected-uncredentialled:no row2-unselected-credentialled:yes; do
  name=${pair%%:*}; cred=${pair##*:}
  d="$OUT/$name"; mkdir -p "$d"
  if [ "$cred" = yes ]; then export "$CREDVAR=$CRED"; else unset "$CREDVAR"; fi
  "$ENGINE" --policy llm --seed 42 --ticks 1 \
    --transcript-path "$OUT/row4-selected-credentialled/transcript.jsonl" \
    --connector-path "$unstartable" > "$d/stdout.txt" 2> "$d/stderr.txt"
  echo $? > "$d/exit.txt"
  unset "$CREDVAR"
  printf '%-22s exit=%s stdout=%s stderr=%s\n' "$name" "$(cat "$d/exit.txt")" \
    "$(wc -c < "$d/stdout.txt")" "$(wc -c < "$d/stderr.txt")"
done

# Rows 1 and 2 must be indistinguishable from each other, and row 1's output identical to row 4's.
for name in row1-unselected-uncredentialled row2-unselected-credentialled; do
  if diff -q "$OUT/$name/stdout.txt" "$OUT/row4-selected-credentialled/stdout.txt" > /dev/null; then
    echo "$name: standard output IDENTICAL to the recording's"
  else
    echo "$name: standard output DIFFERS from the recording's"
  fi
done
if diff -q "$OUT/row1-unselected-uncredentialled/stdout.txt" \
           "$OUT/row2-unselected-credentialled/stdout.txt" > /dev/null; then
  echo "rows 1 and 2: indistinguishable"
else
  echo "rows 1 and 2: DIFFER"
fi

# ---- the ceiling stop: A4, L18, L19 ----
CEILING=0.02 live ceiling "$OUT/script-whole-cent" yes --events-path "$OUT/ceiling/records.jsonl"
unset CEILING

# ---- the retry path: R1 and R2 ----
live retry-bounded "$OUT/script-retry-two"    yes
live retry-exhausted "$OUT/script-retry-always" yes --trace-actions

# The disclosure: a transcript holding a retried exchange is refused by its own replay.
d="$OUT/retried-replay"; mkdir -p "$d"
"$ENGINE" --policy llm --seed 42 --ticks 1 \
  --transcript-path "$OUT/retry-bounded/transcript.jsonl" > "$d/stdout.txt" 2> "$d/stderr.txt"
echo $? > "$d/exit.txt"
printf '%-22s exit=%s\n' retried-replay "$(cat "$d/exit.txt")"

# ---- the pass-through and the leak search: S3a and C1 ----
d="$OUT/passthrough"; mkdir -p "$d"
export "$CREDVAR=$CRED"; export "$SCRIPTVAR=$OUT/script-credential"
"$ENGINE" --policy llm --live --seed 42 --ticks 1 --spend-ceiling 2 --prices "$PRICES" \
  --connector-path "$CONNECTOR" --transcript-output "$d/transcript.jsonl" \
  --events-path "$d/records.jsonl" > "$d/stdout.txt" 2> "$d/stderr.txt"
echo $? > "$d/exit.txt"
unset "$CREDVAR"; unset "$SCRIPTVAR"
printf '%-22s exit=%s\n' passthrough "$(cat "$d/exit.txt")"

echo
echo "leak search over every produced byte of the pass-through run:"
for f in "$d"/transcript.jsonl "$d"/records.jsonl "$d"/stdout.txt "$d"/stderr.txt; do
  whole=$(grep -c -F "$CRED" "$f" 2> /dev/null || true)
  frag=$(grep -c -F "${CRED:0:12}" "$f" 2> /dev/null || true)
  printf '  %-46s whole=%s fragment=%s\n' "$(basename "$f")" "${whole:-0}" "${frag:-0}"
done
echo "  variable name present in the transcript: $(grep -c -F "$CREDVAR" "$d/transcript.jsonl" || true) lines"
