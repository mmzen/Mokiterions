#!/usr/bin/env bash
# Reduces a capture directory produced by capture.sh to one line per cell, for VER-MOK-018 case L9.
#
# The captures themselves are not committed. The 40-cell nosink capture is 63 MB and the sink
# capture is 183 MB, which is not size a repository should carry for a comparison whose whole
# content is "equal or not equal". What is committed is this manifest: for every cell, the digest,
# the byte count and the line count of standard output, the digest of standard error, the exit code,
# and -- when the capture was taken with a sink -- the same three figures for the record stream. Two
# manifests taken at two commits compare with `diff`, and a single differing digest names the cell.
# This is the reduction WO-MOK-011 established and WO-MOK-019 reused.
#
# Usage, from the workspace root:
#
#     docs/engineering/simulation/evidence/WO-MOK-025/manifest.sh <capture-dir> <header-line>
#
# Written to standard output; redirect it. Cells are emitted in `sort` order so that two manifests
# are comparable line for line regardless of the order the shell enumerated them in.
set -u
capture="$1"
header="${2:-capture}"
printf '# %s\n' "$header"
printf '# cell  sha256(stdout)  bytes  lines  sha256(stderr)  exit  [sha256(records)  bytes  lines]\n'
printf '\n'
for text in $(ls "$capture"/*.txt | sort); do
  cell=$(basename "$text" .txt)
  out=$(sha256sum "$text" | cut -d' ' -f1)
  bytes=$(wc -c < "$text" | tr -d ' ')
  lines=$(wc -l < "$text" | tr -d ' ')
  err=$(sha256sum "$capture/$cell.err" | cut -d' ' -f1)
  exit_code=$(cat "$capture/$cell.exit")
  if [ -f "$capture/$cell.jsonl" ]; then
    rec=$(sha256sum "$capture/$cell.jsonl" | cut -d' ' -f1)
    rbytes=$(wc -c < "$capture/$cell.jsonl" | tr -d ' ')
    rlines=$(wc -l < "$capture/$cell.jsonl" | tr -d ' ')
    printf '%-30s %s %9s %7s %s %2s  %s %10s %7s\n' \
      "$cell" "$out" "$bytes" "$lines" "$err" "$exit_code" "$rec" "$rbytes" "$rlines"
  else
    printf '%-30s %s %9s %7s %s %2s\n' \
      "$cell" "$out" "$bytes" "$lines" "$err" "$exit_code"
  fi
done
