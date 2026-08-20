#!/usr/bin/env bash
# Demonstrates that check_declared_dependencies.py refuses, by making it refuse ten ways.
#
# Run from the workspace root:
#     bash docs/engineering/simulation/evidence/WO-MOK-014/merge/WO-MOK-014-injection.sh
#
# Why this exists. A check written by the same agent that wrote the thing being checked, against a
# tree that already conforms, produces a green line either way; a check that cannot fail is
# indistinguishable from no check at all. WO-MOK-014's assurance rationale says so, and
# VER-MOK-014 scenarios 2, 3, 4 and 10 are the scenarios. The unit suite exercises the readers and
# the comparisons directly; this script exercises the whole program end to end, on a copy of this
# workspace with one declaration edited at a time.
#
# Nothing here touches the working tree. Each case copies the manifests, the lockfile, the two
# sources and the two specifications into a scratch root outside the repository, edits the copy,
# and runs the program with --root pointed at it. The scratch root is removed at the end. It is
# outside the repository on purpose: an injected mismatch inside it would otherwise be a real
# mismatch in a real checkout, and git status would carry it.
#
# --no-engine-build is passed throughout: 8.4c compiles the engine, which is measured once for
# real in WO-MOK-014-check-run.txt and would add minutes per case here for no new information.
set -u

export PYTHONIOENCODING=utf-8
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../../../.." && pwd)"
scratch="$HOME/wo13-injection"
# $HOME and not mktemp -d under /tmp: on this host Git Bash maps /tmp into AppData\Local\Temp
# while a native Python process reads it as C:\tmp, so the shell would write where the program
# cannot read.

report="$here/WO-MOK-014-injection.txt"

prepare() {
  rm -rf "$scratch"
  mkdir -p "$scratch/docs/engineering/simulation/specifications"
  cp "$root/Cargo.toml" "$root/Cargo.lock" "$root/rust-toolchain.toml" "$scratch/"
  cp -r "$root/mokiterions-core" "$root/mokiterions-tui" "$scratch/"
  rm -rf "$scratch/mokiterions-core/target" "$scratch/mokiterions-tui/target"
  cp "$root/docs/engineering/simulation/specifications/SPEC-MOK-002.md" \
     "$root/docs/engineering/simulation/specifications/SPEC-MOK-003.md" \
     "$scratch/docs/engineering/simulation/specifications/"
}

# case <number> <what was injected> <why it must refuse>
case_run() {
  echo
  echo "=============================================================================="
  echo "Case $1: $2"
  echo "Expected: $3"
  echo "------------------------------------------------------------------------------"
  python "$root/scripts/check_declared_dependencies.py" --root "$scratch" --no-engine-build 2>&1 \
    | grep -v -e '^  ' -e '^targets:' -e '^cargo metadata' -e '^\[workspace.dependencies\]'
  echo "exit ${PIPESTATUS[0]}"
}

# The anchor must match exactly once. An anchor that matches twice edits whichever occurrence
# comes first, which is how an earlier draft of case 2 rewrote a prose paragraph instead of the
# declaration and then reported that the program had failed to notice. A silent multi-match is the
# one failure mode an injection harness cannot afford, since its output looks like a passing check.
python_edit() {
  python - "$@" <<'PY'
import pathlib, sys
path = pathlib.Path(sys.argv[1])
old, new = sys.argv[2], sys.argv[3]
text = path.read_text(encoding="utf-8")
hits = text.count(old)
if hits != 1:
    sys.exit(f"REFUSED: the injection anchor matches {hits} times in {path.name}: {old[:70]!r}")
path.write_text(text.replace(old, new, 1), encoding="utf-8")
PY
}

{
  echo "# WO-MOK-014: the checking program refusing, ten ways"
  echo "# $(cargo --version)"
  echo "#"
  echo "# One declaration is edited per case, on a copy of this workspace. The notes each run"
  echo "# prints are filtered out here; what is left is the refusals and the exit code."

  observer="$scratch/docs/engineering/simulation/specifications/SPEC-MOK-003.md"
  engine="$scratch/docs/engineering/simulation/specifications/SPEC-MOK-002.md"

  prepare
  python_edit "$observer" '| `ratatui` | `0.30.2` |' '| `ratatui` | `0.30.1` |'
  case_run 1 "the declared ratatui version moved to 0.30.1" \
    "8.4a, a version mismatch in both directions"

  prepare
  python_edit "$observer" \
    '| `ratatui` | `0.30.2` | `default-features = false`, plus `crossterm`, `layout-cache`, `underline-color`.' \
    '| `ratatui` | `0.30.2` | `default-features = false`, plus `crossterm`, `underline-color`.'
  case_run 2 "layout-cache removed from the declared features" \
    "8.4b, a resolved feature that is neither declared nor implied, and a manifest feature the declaration does not state"

  prepare
  python_edit "$observer" '| `x86_64-pc-windows-msvc` | **57** | 7 |' \
    '| `x86_64-pc-windows-msvc` | **58** | 7 |'
  case_run 3 "the Windows external-crate figure moved to 58" \
    "8.4c, a declared count that the graph does not have"

  prepare
  python_edit "$observer" '| `winapi` | `0.3.9` | Windows |' ''
  case_run 4 "the winapi build-script row deleted" \
    "decision 13, a build-script crate the table does not list, and a count of 7 that is now 6 declared"

  prepare
  python_edit "$observer" '| `mio` | `1.2.2` |' '| `mio-not-this-one` | `1.2.2` |'
  case_run 5 "the mio disclosure row renamed, so nothing discloses mio" \
    "8.4d twice: an undisclosed prohibited-class name on two targets, and a disclosure no target reaches"

  prepare
  python_edit "$observer" \
    '| `ratatui` | `0.30.2` | `default-features = false`' \
    '| `serde` | `1.0.229` | `default-features = false` | no | injected |
| `ratatui` | `0.30.2` | `default-features = false`'
  case_run 6 "serde 1.0.229 declared, which the manifest does not name" \
    "8.4a, a declared entry absent from the manifest"

  prepare
  python_edit "$engine" \
    '| Crate | Version | Features | Build script | Admitted by |
|---|---|---|---|---|

**The table is empty.**' \
    '| Crate | Version | Features | Build script | Admitted by |
|---|---|---|---|---|
| `ratatui` | `0.30.2` | `default-features = false` | no | injected |

**The table is empty.**'
  case_run 7 "ratatui declared in the engine's rule 13 table" \
    "8.4a, a declared entry absent from the engine's manifest, and 8.4d, a user-interface crate declared by the engine"

  prepare
  python_edit "$scratch/mokiterions-tui/Cargo.toml" \
    'ratatui = { version = "0.30.2"' 'thiserror = { version = "2.0.20" }
ratatui = { version = "0.30.2"'
  # The lockfile is updated in the same edit, offline, because that is what adding a dependency
  # actually looks like: cargo writes the lock entry as part of the first build. Without it the
  # run stops on `--locked` before any comparison happens, which is case 9.
  (cd "$scratch" && cargo metadata --offline --format-version 1 > /dev/null 2>&1) \
    || echo "NOTE: the offline lockfile update failed, so case 8 measures nothing"
  case_run 8 "thiserror added to the observer's manifest and to the lockfile, declared nowhere" \
    "8.4a, a manifest dependency that is not a declared entry"

  prepare
  python_edit "$scratch/mokiterions-tui/Cargo.toml" \
    'ratatui = { version = "0.30.2"' 'thiserror = { version = "2.0.20" }
ratatui = { version = "0.30.2"'
  case_run 9 "the same dependency added to the manifest and NOT to the lockfile" \
    "a stale-lockfile refusal, before rule 8.4 compares anything -- cargo will not resolve under --locked, and the message has to say that rather than send a reader to fetch"

  # Case 10 is the one prohibition ADR-MOK-006 decision 4 kept absolutely: no user-interface
  # crate in the engine, ever, declared or not. It is made the realistic way -- manifest, rule 13
  # table and lockfile all consistent -- because that is the change an implementation agent could
  # plausibly make and believe legitimate. Its output is long: rule 13's strict closure refuses
  # once per external crate the engine's graph has acquired. The 8.4d refusals are printed in full
  # and the closure refusals are counted, with the count stated rather than the tail dropped.
  prepare
  python_edit "$scratch/mokiterions-core/Cargo.toml" '[dependencies]' \
    '[dependencies]
ratatui = { version = "0.30.2", default-features = false, features = ["crossterm", "layout-cache", "underline-color"] }'
  python_edit "$engine" \
    '| Crate | Version | Features | Build script | Admitted by |
|---|---|---|---|---|

**The table is empty.**' \
    '| Crate | Version | Features | Build script | Admitted by |
|---|---|---|---|---|
| `ratatui` | `0.30.2` | `default-features = false`, plus `crossterm`, `layout-cache`, `underline-color`. `std` is implied. | no | injected |

**The table is empty.**'
  (cd "$scratch" && cargo metadata --offline --format-version 1 > /dev/null 2>&1) \
    || echo "NOTE: the offline lockfile update failed, so case 10 measures nothing"
  echo
  echo "=============================================================================="
  echo "Case 10: ratatui added to the ENGINE's manifest, its rule 13 table and the lockfile"
  echo "Expected: 8.4d, the surviving REQ-MOK-026 prohibition, which no declaration can satisfy"
  echo "------------------------------------------------------------------------------"
  python "$root/scripts/check_declared_dependencies.py" --root "$scratch" --no-engine-build \
    > "$scratch/case10.txt" 2>&1
  status=$?
  grep "8.4d Mokiterions" "$scratch/case10.txt"
  echo "... and $(grep -c "rule 13 Mokiterions" "$scratch/case10.txt") refusal(s) from rule 13's"
  echo "strict closure, one per target, each naming every external crate the engine's graph"
  echo "acquired -- 57 to 63 of them depending on the target, so they are counted and not quoted."
  echo "total refusal lines in the run: $(grep -c "^REFUSED" "$scratch/case10.txt")"
  echo "exit $status"

  rm -rf "$scratch"
  echo
  echo "=============================================================================="
  echo "The scratch root is removed. This checkout was not edited: the working tree after this"
  echo "script is the working tree before it, which is what the program's own git status"
  echo "comparison asserts on the real root in WO-MOK-014-check-run.txt."
} > "$report" 2>&1

echo "wrote $report"
grep -c "^exit 0" "$report" | sed 's/^/cases that passed when they should have refused: /'
