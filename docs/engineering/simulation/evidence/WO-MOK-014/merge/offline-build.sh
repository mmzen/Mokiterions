#!/usr/bin/env bash
# Re-derives WO-MOK-014-offline-build.txt at the merge commit.
#
# Run from the workspace root:
#     bash docs/engineering/simulation/evidence/WO-MOK-014/merge/offline-build.sh
#
# The retained WO-MOK-014-offline-build.txt was composed by hand from pasted command output. This
# script emits the same sections from live commands instead, so the re-derivation is reproducible
# from this checkout and the comparison against the retained file is a comparison of measurements
# and not of two transcriptions. Every command is the retained file's command, unchanged.
set -u

export PYTHONIOENCODING=utf-8
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../../../.." && pwd)"
out="$here/WO-MOK-014-offline-build.txt"
proxy="http://127.0.0.1:1"

cd "$root"

indent() { sed 's/^/    /'; }

{
  echo "VER-MOK-014 — the engine offline from the committed lockfile, with the registry unreachable,"
  echo "re-derived at the merge commit 9599c0a"
  echo "==========================================================================================="
  echo
  echo "Work order   WO-MOK-014 (crates policy, declared-set comparison)"
  echo "Branch       governance/adr-mok-006-third-party-crates"
  echo "Commit       $(git rev-parse HEAD)"
  echo "Toolchain    $(cargo --version); $(rustc --version)"
  echo
  echo "This is VER-MOK-014's matrix row \"Offline resolution, build and test of the engine\" and its"
  echo "acceptance scenario 5, re-asked at the merge commit. It is a matrix row and not one of the five"
  echo "numbered oracles -- oracle 4 is the independently written prohibition list, which is measured in"
  echo "WO-MOK-014-scan.txt beside this file. The packet's WO-MOK-014-offline-build.txt answers this row"
  echo "at candidate 65ac88b; that file was composed by hand from pasted output, so this one is emitted"
  echo "by offline-build.sh beside it, running the same commands. SPEC-MOK-005 rule 8.4c is the gate."
  echo
  echo "Making the registry unreachable rather than asserting it"
  echo "-------------------------------------------------------"
  echo
  echo "--offline tells cargo not to use the network. That is cargo's promise, not a measurement of"
  echo "it, so every command below also runs with an HTTP proxy pointed at a closed port:"
  echo
  echo "    CARGO_HTTP_PROXY=$proxy"
  echo
  echo "Control first -- a command that does need the registry, under that proxy, so that a"
  echo "succeeding build below cannot be a proxy that was never consulted:"
  echo
  echo "    \$ CARGO_HTTP_PROXY=$proxy cargo search ratatui --limit 1"
  CARGO_HTTP_PROXY="$proxy" cargo search ratatui --limit 1 2>&1 | indent
  echo "    exit ${PIPESTATUS[0]}"
  echo
  echo "The engine, built and tested offline"
  echo "-----------------------------------"
  echo
  echo "Cargo.lock sha256 before: $(sha256sum Cargo.lock | cut -d' ' -f1)"
  echo "git status --porcelain before: $(git status --porcelain | wc -l | tr -d ' ') entries -- the merge evidence"
  echo "packet is untracked and WO-MOK-014-merge.md is modified at the time of this run, so the assertion is"
  echo "that the count and the content are unchanged afterwards and not that they are zero"
  git status --porcelain > "$here/.status-before"
  echo
  echo "The engine's own source is touched first so that neither command reports a cached result."
  touch mokiterions-core/src/lib.rs
  echo
  echo "    \$ CARGO_HTTP_PROXY=$proxy cargo build -p Mokiterions --locked --offline"
  CARGO_HTTP_PROXY="$proxy" cargo build -p Mokiterions --locked --offline 2>&1 | indent
  echo "    exit ${PIPESTATUS[0]}"
  echo
  touch mokiterions-core/src/lib.rs
  echo "    \$ CARGO_HTTP_PROXY=$proxy cargo test -p Mokiterions --locked --offline"
  CARGO_HTTP_PROXY="$proxy" cargo test -p Mokiterions --locked --offline 2>&1 | indent
  echo "    exit ${PIPESTATUS[0]}"
  echo
  echo "Cargo.lock sha256 after:  $(sha256sum Cargo.lock | cut -d' ' -f1)"
  git status --porcelain > "$here/.status-after"
  echo
  echo "    \$ diff <(git status --porcelain before) <(git status --porcelain after)"
  if diff "$here/.status-before" "$here/.status-after" > /dev/null; then
    echo "    (no output: the working tree after these commands is the working tree before them)"
  else
    echo "    THE WORKING TREE CHANGED:"
    diff "$here/.status-before" "$here/.status-after" | indent | indent
  fi
  rm -f "$here/.status-before" "$here/.status-after"
  echo
  echo "What this establishes, and what it does not"
  echo "------------------------------------------"
  echo
  echo "It establishes that the engine package resolves, compiles and tests from the committed lockfile"
  echo "with the registry provably out of reach, at the merge commit, which is what SPEC-MOK-005 rule"
  echo "8.4c requires as a check. The merge changed no engine source, no manifest and no lockfile byte,"
  echo "so a reader could have inferred this; rule 8.4c asks for a measurement and this is the"
  echo "measurement."
  echo
  echo "It does not establish anything about the observer, and the observer is what the merge changed."
  echo "\`cargo build -p mokiterions-tui --offline\` needs the \`ratatui\` graph in the local cache, and a"
  echo "build on a machine with an empty cache would fetch it. The offline claim in this repository is"
  echo "about the engine, and rule 8.4c says so. The observer's own build and its 141 tests are in"
  echo "gates.txt beside this file, run --locked but not under this proxy."
  echo
  echo "It does not establish that the engine has no dependencies -- \`cargo tree\` does that, in"
  echo "WO-MOK-014-graphs.txt beside this file, one node per target. A package can depend on crates"
  echo "already in the cache and still build offline."
} > "$out" 2>&1

python - "$out" <<'PY'
import pathlib, sys
p = pathlib.Path(sys.argv[1])
b = p.read_bytes()
n = b.count(b"\r\n")
p.write_bytes(b.replace(b"\r\n", b"\n"))
print(f"normalized {n} CRLF line ending(s) in {p.name}")
PY
echo "wrote $out"
