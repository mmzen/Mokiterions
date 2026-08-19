#!/usr/bin/env bash
# WO-MOK-007: capture the tooling runs `analysis/static-checks.py` reads.
#
# Usage, from the repository root:
#
#     bash docs/engineering/simulation/evidence/WO-MOK-007/analysis/capture-static.sh target/static
#     python docs/engineering/simulation/evidence/WO-MOK-007/analysis/static-checks.py target/static
#
# Each file holds the command line as a `### ` heading, that command's own combined output, and its
# exit status as a final `exit=<n>` line. Nothing is filtered: the analysis reads these files, so a
# capture that dropped a warning would hide it from the check rather than from a reader.
#
# The clippy run is preceded by a forced re-lint. This is the mistake the script exists to prevent
# from recurring: clippy reports a cached result without re-checking anything, and a clippy run that
# lints nothing prints no warning, so a capture taken after an earlier run passes vacuously.
# Touching every source file makes the run real, and `static-checks.py` fails the artifact if fewer
# than two crates were re-checked.
#
# The two `-pre` dependency graphs are not captured here. They come from a clean git worktree at the
# pre-change commit, which is created and removed by hand:
#
#     git worktree add target/pre 60fda9faffbd452752a34efa356f16cc6ad1d3ff
#     (cd target/pre && cargo tree -p Mokiterions)      > target/static/tree-pre.txt
#     (cd target/pre && cargo tree -p mokiterions-tui)  > target/static/tree-tui-pre.txt
#     git worktree remove target/pre

set -u

dir="${1:-target/static}"
mkdir -p "$dir"

capture() {
    local name="$1"
    shift
    {
        printf '### %s\n' "$*"
        "$@" 2>&1
        printf 'exit=%d\n' "$?"
    } >"$dir/$name"
    printf '%-16s %s\n' "$name" "$*"
}

{
    for tool in "cargo --version" "rustc --version" "cargo fmt --version" "cargo clippy --version"; do
        printf '### %s\n' "$tool"
        $tool 2>&1
    done
} >"$dir/versions.txt"
printf '%-16s %s\n' "versions.txt" "cargo, rustc, rustfmt and clippy versions"

capture fmt.txt cargo fmt --all -- --check

# Force the re-lint, then lint.
find mokiterions-core/src mokiterions-tui/src mokiterions-core/tests mokiterions-tui/tests \
    -name '*.rs' -exec touch {} +
capture clippy.txt cargo clippy --workspace --all-targets --all-features -- -D warnings

capture test.txt cargo test --workspace
capture tree.txt cargo tree -p Mokiterions
capture tree-tui.txt cargo tree -p mokiterions-tui
