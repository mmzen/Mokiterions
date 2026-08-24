#!/usr/bin/env bash
# Reduces the per-boundary entropy capture to one line per configuration, for VER-MOK-018 case L9.
#
# The instrument `simulation::tests::the_four_existing_sources_entropy_state_at_every_tick_boundary`
# prints one line per tick boundary of each of REQ-MOK-068's twenty configurations, which is about
# 17,000 lines. That is the wrong size to commit for a comparison whose content is "equal or not
# equal", so what is retained is this reduction: for every configuration, the number of boundaries
# and the sha256 of its own lines. Two manifests taken at two commits compare with `diff`, and a
# differing digest names the configuration; the instrument is then re-run to name the boundary.
#
# The digest is over the boundary lines of one configuration only, so a configuration that moved
# cannot hide behind one that did not.
#
# Usage, from the workspace root:
#
#     cargo test -p Mokiterions --lib --release -q -- --exact --ignored --nocapture \
#         simulation::tests::the_four_existing_sources_entropy_state_at_every_tick_boundary \
#         > <capture-file>
#     docs/engineering/simulation/evidence/WO-MOK-025/entropy-manifest.sh <capture-file> <header>
#
# Written to standard output; redirect it.
set -u
capture="$1"
header="${2:-capture}"
printf '# %s\n' "$header"
printf '# configuration  boundaries  sha256(that configuration'"'"'s boundary lines)  final state\n'
printf '\n'
for seed in 0 1 42 123 777; do
  for policy in baseline reference individual social; do
    lines=$(grep "^seed=$seed density=0.75 policy=$policy " "$capture" || true)
    count=$(printf '%s\n' "$lines" | grep -c 'boundary=')
    digest=$(printf '%s\n' "$lines" | sha256sum | cut -d' ' -f1)
    final=$(printf '%s\n' "$lines" | tail -1 | sed 's/.*state=//')
    printf 'seed%-4s %-11s %6s  %s  %s\n' "$seed" "$policy" "$count" "$digest" "$final"
  done
done
