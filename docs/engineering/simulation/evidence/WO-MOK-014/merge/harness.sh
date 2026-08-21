#!/usr/bin/env bash
# Captures the five harness gates twice: at the candidate commit with nothing of this act in the
# tree, and in this checkout with the whole act present.
#
# Run from the workspace root, LAST, after every other file of this packet and VREC-MOK-015 exist:
#     bash docs/engineering/simulation/evidence/WO-MOK-014/merge/harness.sh <clean-worktree-path>
#
# The clean worktree must be a detached checkout of the candidate commit whose LEAF DIRECTORY NAME
# equals this repository's, because build_snapshot puts the checkout directory's name into the
# hashed document. A worktree with a different name yields a different digest over an identical tree.
set -u

export PYTHONIOENCODING=utf-8
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../../../../.." && pwd)"
out="$here/harness.txt"
clean="${1:-C:/Users/mathi/AppData/Local/Temp/wo14vrec/Mokiterions-understand-yellow-8201}"
pinned="/c/Users/mathi/.harness040/Scripts/python.exe"
candidate="9599c0a91bb2b6e183bce3a5e82b570d547594f8"

cd "$root"

if [ "$(git -C "$clean" rev-parse HEAD 2>/dev/null)" != "$candidate" ]; then
  echo "the clean worktree is not at $candidate" >&2
  exit 2
fi
if [ "$(basename "$clean")" != "$(basename "$root")" ]; then
  echo "the clean worktree's leaf name differs from this repository's" >&2
  exit 2
fi

run() { # run <label> <dir> <command...>
  local label="$1" dir="$2"; shift 2
  local output status
  output="$(cd "$dir" && "$@" 2>&1)"; status=$?
  printf '  %s\n' "$label"
  printf '%s\n' "$output" | sed 's/^/    /'
  printf '    exit %s\n' "$status"
}

{
  echo "VER-MOK-014 — the harness five, at the candidate commit and with this act in the tree"
  echo "===================================================================================="
  echo
  echo "Work order   WO-MOK-014 (crates policy, declared-set comparison)"
  echo "Candidate    $candidate"
  echo "Branch       governance/adr-mok-006-third-party-crates"
  echo "Date         2026-08-20"
  echo "Python       $(python --version 2>&1); pinned venv $("$pinned" --version 2>&1)"
  echo
  echo "Two columns throughout:"
  echo
  echo "  A  a detached worktree checked out at the candidate commit, git status empty. Neither this"
  echo "     packet nor VREC-MOK-015 exists in it, because neither exists at that commit."
  echo "  B  this checkout, HEAD at the same commit, with the whole act present in the working tree:"
  echo "     the merge/ packet and VREC-MOK-015.md untracked, WO-MOK-014-merge.md modified."
  echo
  echo "A is what the candidate's own graph says. B is what the commit that carries this record will"
  echo "say once it exists. The difference between them IS this act, measured rather than asserted."
  echo
  echo "This file is the last thing written in this packet, and it is taken while HEAD is still the"
  echo "candidate. Column B's dashboard digest is therefore a figure of this commit's CONTENT measured"
  echo "at its parent, not the digest of this commit: build_snapshot writes git rev-parse HEAD into the"
  echo "hashed document in 25 places, so the digest of the commit that carries these files does not"
  echo "exist until that commit does. No record binds column B's digest. VREC-MOK-015 declares column"
  echo "A's, which is a figure of the candidate and is stable."
  echo
  echo "Validator"
  echo "---------"
  echo
  echo "    \$ python scripts/validate_engineering_artifacts.py --root ."
  echo
  run "A  candidate, clean" "$clean" python scripts/validate_engineering_artifacts.py --root .
  echo
  run "B  this checkout" "$root" python scripts/validate_engineering_artifacts.py --root .
  echo
  echo "Dashboard"
  echo "---------"
  echo
  echo "    \$ python scripts/generate_harness_dashboard.py --root ."
  echo
  run "A  candidate, clean" "$clean" python scripts/generate_harness_dashboard.py --root .
  echo
  run "B  this checkout" "$root" python scripts/generate_harness_dashboard.py --root .
  echo
  echo "    \$ sha256sum target/harness-dashboard/dashboard-data.json"
  echo "    A  $(cd "$clean" && sha256sum target/harness-dashboard/dashboard-data.json | cut -d' ' -f1)"
  echo "    B  $(sha256sum target/harness-dashboard/dashboard-data.json | cut -d' ' -f1)"
  echo
  echo "The printed Snapshot line and the file's own sha256 agree in both columns, which is what makes"
  echo "either figure checkable by a reader who has neither this shell nor this moment."
  echo
  echo "Inspector"
  echo "---------"
  echo
  echo "    \$ python scripts/inspect_engineering_artifacts.py --root ."
  echo
  run "A  candidate, clean" "$clean" python scripts/inspect_engineering_artifacts.py --root .
  echo
  run "B  this checkout" "$root" python scripts/inspect_engineering_artifacts.py --root .
  echo
  echo "Doctor, under the pinned 0.4.0 venv"
  echo "-----------------------------------"
  echo
  echo "    \$ <pinned 0.4.0 venv>/python -m se_harness doctor <tree>"
  echo
  run "A  candidate, clean" "$clean" "$pinned" -m se_harness doctor .
  echo
  run "B  this checkout" "$root" "$pinned" -m se_harness doctor .
  echo
  echo "The pinned 0.4.0 reading is the one .github/workflows/engineering-harness.yml installs by exact"
  echo "pin. The machine-wide 0.4.1 install reports exit 1 and eight distribution: FAILs against a"
  echo "repository whose declared runtime is 0.4.0, at this commit as at every other; ../WO-MOK-014-"
  echo "harness.txt records that reading and records that an earlier draft of it had the cause wrong."
  echo "A reader who measures doctor with 0.4.1 will not reproduce these lines and should not."
  echo
  echo "Preflight, review phase, under the pinned 0.4.0 venv"
  echo "---------------------------------------------------"
  echo
  echo "    \$ <pinned 0.4.0 venv>/python -m se_harness preflight <tree> --work-order WO-MOK-014 --phase review"
  echo
  run "A  candidate, clean" "$clean" "$pinned" -m se_harness preflight . --work-order WO-MOK-014 --phase review
  echo
  run "B  this checkout" "$root" "$pinned" -m se_harness preflight . --work-order WO-MOK-014 --phase review
  echo
  echo "Where each tree stands"
  echo "----------------------"
  echo
  echo "    \$ git rev-parse HEAD; git status --porcelain"
  echo "    A  $(git -C "$clean" rev-parse HEAD)"
  git -C "$clean" status --porcelain | sed 's/^/       /'
  echo "       ($(git -C "$clean" status --porcelain | wc -l | tr -d ' ') entries)"
  echo "    B  $(git rev-parse HEAD)"
  git status --porcelain | sed 's/^/       /'
  echo "       ($(git status --porcelain | wc -l | tr -d ' ') entries)"
  echo
  echo "Column B's three entries are this commit's content: the untracked packet, the untracked record,"
  echo "and the tracked correction to WO-MOK-014-merge.md. No tracked file differs from the candidate in"
  echo "anything VER-MOK-014 reads -- no source, no manifest, no lockfile, no workflow, no script and no"
  echo "specification -- which is what contract-identity.txt beside this file establishes file by file."
  echo
  echo "What the columns differ by, and what changes again at the next commit"
  echo "--------------------------------------------------------------------"
  echo
  echo "The whole of the difference is this act: 1 artifact, VREC-MOK-015; 2 relations, its"
  echo "verifies_work_order to WO-MOK-014 and its conforms_to to VER-MOK-014; 1 queue entry,"
  echo "decision_required / assurance-review; and 1 warning. Nothing else in 114 artifacts and 396"
  echo "relations moved, and the four formal planes are E0/W0 in both columns."
  echo
  echo "The new warning is W-REV-004: \"VREC-MOK-015 is ready but its work is fully covered by verified"
  echo "or released records; review possible supersession without inferring authority\", naming both"
  echo "records, with review-verification-supersession suggested to the assurance owner. The harness is"
  echo "right to ask: one work order now carries two records. VREC-MOK-015 states why supersession is"
  echo "not available here -- VREC-MOK-014 is verified, and the template permits superseding a ready"
  echo "record only -- and saying so does not close the finding, which is addressed to the assurance"
  echo "owner and says in its own words not to infer authority. It is reported here, not silenced."
  echo
  echo "I-REV-001 reads 14 observations in BOTH columns, and that is a coincidence of this moment rather"
  echo "than stability. It fires for a record whose declared commit differs from the observed revision."
  echo "VREC-MOK-015 declares 9599c0a and the observed revision IS 9599c0a while this file is taken, so"
  echo "it does not fire. As soon as the commit carrying this packet exists, the observed revision moves"
  echo "past 9599c0a and I-REV-001 reads 15. Neither column's warning or info count survives its own"
  echo "commit, and a reader measuring at the branch tip should expect that difference rather than read"
  echo "it as drift. The validator's Warnings: 0 and the dashboard's Warnings: 10 are two different"
  echo "populations, not a contradiction: the validator counts formal-plane warnings and the dashboard"
  echo "counts derived findings."
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
