# This chain was renumbered from `010` to `011`

| Field | Value |
|---|---|
| What moved | `WO-MOK-010` → `WO-MOK-011`, `VER-MOK-010` → `VER-MOK-011`, `VREC-MOK-010` → `VREC-MOK-011`, and this directory |
| What did not move | `INT-MOK-008`, `CAP-MOK-008`, `REQ-MOK-040`, `REQ-MOK-041`, and every measurement in this packet |
| Decided by | engineering owner, 2026-08-19 |
| Baseline commit | `524a6758d74b5240079959e9827ea40a7af22a30`, unchanged |
| Date | 2026-08-19 |

Nothing here is a change to the work. The renumber alters no requirement, no provision, no oracle, no
measured figure and no line of behaviour. It exists because two chains claimed one set of names.

---

## Why

`feature/phase-2-individuality` renumbered *its* chain from `007` to `010` at commit `7012ff8`, on the
recorded ground that `010` was "the lowest number free across every ref". That was true of every
**pushed** ref. This branch had been holding `WO-MOK-010`, `VER-MOK-010` and
`evidence/WO-MOK-010/` for two hours without pushing, so the scan could not see it, and the two
chains met as 18 merge conflicts — eleven of them add/add over this directory.

Occupancy re-measured across every remote ref before choosing `011`
(`git ls-tree -r --name-only <ref> -- docs/engineering`, every `refs/remotes/origin/*`):

| Identifier | Highest in use, any ref | Held by |
|---|---|---|
| `WO-MOK-nnn` | `010` | `009` on `master`, `010` on `feature/phase-2-individuality` |
| `VER-MOK-nnn` | `010` | `008` on `master`, `010` on `feature/phase-2-individuality` |
| `VREC-MOK-nnn` | `010` | `008` on `master`, `010` on `feature/phase-2-individuality` |

`011` is free on every ref for all three types. `master` reached `008` by renumbering a verification
record of its own for the same reason, at commit `2bcb9f3`.

---

## How the rewrite was done, and what that guarantees

Three artifact files and this 220-file directory were renamed with `git mv`. The 37 files whose
contents mention the chain were then rewritten **from their `HEAD` blob bytes**, replacing only the
three identifier tokens, so no other byte in any file could move. A first attempt used `sed -i`,
which normalised CRLF to LF in files that mix line endings — it removed 221 CR bytes from
`interface.txt` alone — and was discarded rather than committed.

`WO-MOK-010` and `WO-MOK-011` are the same length. So:

- every rewritten file keeps its **exact byte length** — checked for all 37, `git cat-file -s` on both
  sides of the move;
- no line and no column moved, so every line number, byte count and character position recorded in
  this packet still points where it did;
- 191 occurrences were rewritten outside the verification record — 106 `WO-`, 77 `VER-`, 8 `VREC-`.

Every retained capture is byte-identical across the move: the 180 `init/` streams, the 6 `full/`
streams, `frames-pre.txt`, `frames-post.txt`, `bar-rows.txt`, `bar-rows.diff` and both `test-run.txt`
contain no identifier at all. The digests in `baseline/pre-manifest.txt`, `post/post-manifest.txt`,
`additivity.txt` and `observer/frames.txt` are digests of those streams, and therefore stand
unchanged.

Where an identifier does appear in a retained `.txt`, it is in a header line or in prose — written
either by the script that emits the file or by hand. The emitting scripts (`baseline/compare.py`,
`analysis/census.py`, `analysis/amendments.py`, `analysis/interface.py`, `baseline/retain.py`,
`capture.sh`) are renamed in this same commit, so re-running them reproduces the renamed headers.

`amendment-approvals.md` was not edited by hand but **regenerated** from the renamed
`analysis/amendments.py`, its nine self-tests passing, and is byte-identical to the file in this
packet:

    sha256  f675831ed9be8533431718f77d1d240e897f13502a06123f7456a88637e60d9c   before the renumber
    sha256  5b3e8b19cfef265e252960b89c46a1e245ce4b18aa04d28e1defd097655a07f1   after, and reproduced
    RESULT: PASS — 9/9 controls, unchanged

---

## One recorded digest moves, and cannot be preserved

`analysis/mutation-control.txt` records `sha256(mokiterions-core/src/simulation.rs)` twice — once
before the perturbation and once after the revert, as the proof that the working tree was restored:

    cca972343a13d4b04cd643052baaee55a97127847bc67a719a1105f8b7f8ef30   as recorded, at 52b41c8

The renumber edits two comment lines in that file, so at this commit the same file digests to

    1aab8e51ef2814c585fadcc7272c2749f491d39a4bda43447931bc6366471ede   158626 bytes, unchanged length

The recorded value is not wrong: it describes the file at the commit the measurement was taken at.
A reviewer re-computing it against this tree will get the second value, and this is the note that
says why. What the control established — that one extra draw makes oracles 1 and 2 fail, and that the
revert put the stream back — does not depend on either digest.

The comment lines the renumber touched, all five of them, in four files:

| File | Line | What it is |
|---|---|---|
| `mokiterions-core/src/simulation.rs` | 412 | doc comment citing the work order that decided the name table |
| `mokiterions-core/src/simulation.rs` | 4008 | section marker |
| `mokiterions-core/tests/naming.rs` | 17 | doc comment citing the reused seed set |
| `mokiterions-tui/src/verification.rs` | 22 | module comment on the ninth negative control |
| `mokiterions-tui/tests/verification.rs` | 843 | section marker |

One further quotation is affected: `analysis/mutation-control.txt` lines 26–27 quote the comment the
perturbation inserted, which now reads `WO-MOK-011` and `evidence/WO-MOK-011/` although the mutation
as performed carried the old spelling. The perturbation was a working-tree edit, reverted
immediately and never committed, so no commit records either spelling. It is flagged here because the
quoted text is authored, not tool output.

---

## Gates re-run at this tree, not carried over

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | clean, exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | both crates re-linted after touching their roots; 0 warnings, exit 0 |
| `cargo test --workspace` | 205 passed, 0 failed, 0 ignored — the same census this packet records |
| `python scripts/validate_engineering_artifacts.py` | PASS, 83 artifacts, 0 errors, 0 warnings |

The artifact count is 83 rather than 84 because the verification record is not in this commit: its
binding named paths that this commit moves, so it is re-captured against this commit instead.

---

## When this note is superseded

This packet is owed a full re-capture when this branch merges `master`'s release-ci work, which moved
the test population and changed `mokiterions-tui/src/render.rs`. At that point oracle 3's census,
oracle 4's frames and `SPEC-MOK-004`'s counted rules are re-derived, `mutation-control.txt` is
re-measured against the merged tree, and the digest exception above ceases to exist.
