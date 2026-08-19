# Toolchain evidence — `REQ-MOK-039`, `VER-MOK-008` T1 through T5

Captured 2026-08-19.

`rust-toolchain.toml` is new in this changeset. It is repository-owned: not a harness artifact and not
an approved architectural decision. It records the toolchain the retained evidence under
`docs/engineering/simulation/evidence/` was actually produced with, so a later build can reproduce it.

| Scenario | Status here |
| --- | --- |
| T1 — a clone selects the declared version | **observed** in this worktree; see the caveat about clones |
| T2 — a matching version passes | **observed** |
| T3 — a declaration below the available compiler refuses | **rehearsed** with a fabricated declaration |
| T4 — a declaration above the available compiler refuses | **rehearsed** with a fabricated declaration |
| T5 — the declared components are present | **observed** — `rustfmt` and `clippy` both ran, exit 0 |

## T1 and T2 — the declaration selects the compiler

The home-directory prefix is replaced by `<home>` throughout, and the checkout path by
`<checkout>`. No other evidence file in this repository carries an absolute local path and this one
should not be the first; the paths are not load-bearing here -- what matters is *which file* rustup
names as the reason, and that name survives the redaction.

```text
$ rustup show active-toolchain   # inside the repository, where the pin applies
1.97.1-x86_64-pc-windows-msvc (overridden by '<checkout>\rust-toolchain.toml')

$ rustup show 2>&1 | head -20
Default host: x86_64-pc-windows-msvc
rustup home:  <home>\.rustup

installed toolchains
--------------------
stable-x86_64-pc-windows-msvc (default)
1.97.1-x86_64-pc-windows-msvc (active)

active toolchain
----------------
name: 1.97.1-x86_64-pc-windows-msvc
active because: overridden by '<checkout>\rust-toolchain.toml'
installed targets:
  x86_64-pc-windows-msvc

$ rustc --version
rustc 1.97.1 (8bab26f4f 2026-07-14)
```

`rustup` reports the active toolchain as `1.97.1-x86_64-pc-windows-msvc`, *"active because: overridden
by …\rust-toolchain.toml"*, while the machine's default remains `stable`. That is the mechanism the
declaration relies on, and the reason `docs/engineering/REPOSITORY_CONTEXT.md` tells a reader not to
install a toolchain by hand and not to override the pin with `+stable`.

T5 is discharged by the same observation plus `verification-output.md`: `profile = "minimal"` with
`components = ["rustfmt", "clippy"]` is the whole declaration, and both `cargo fmt` and `cargo clippy`
ran to exit 0 without a separate component installation.

**Caveat, stated rather than glossed:** `rust-toolchain.toml` is uncommitted in this changeset, so a
clone of `master` does not yet carry it. T1's "fresh clone" form becomes reproducible once the file is
committed. What is observed today is the same mechanism in the worktree where the file lives, with the
machine default deliberately different from the pin, so the override is visible rather than
coincidental.

## T3 and T4 — both directions of the mismatch refusal

The predicate is run verbatim against fabricated declaration files rather than by editing the
repository's own pin, so no real declaration was changed and no toolchain was installed to make a
check pass. The predicate under test, from `.github/workflows/release.yml` — the `verify` job's
*Toolchain, from the repository's pin* step and the `build` job's *Toolchain and target* step:

```bash
pinned="$(sed -n 's/^channel *= *"\(.*\)"/\1/p' rust-toolchain.toml)"
if [[ -z "$pinned" ]]; then
  echo "REFUSED: rust-toolchain.toml declares no channel." >&2
  exit 1
fi
if [[ "$(rustc --version)" != "rustc $pinned "* ]]; then
  echo "REFUSED: rust-toolchain.toml pins $pinned, but rustc is $(rustc --version)." >&2
  exit 1
fi
```

```text
$ rustup show active-toolchain
stable-x86_64-pc-windows-msvc (default)

$ rustc --version --verbose
rustc 1.97.1 (8bab26f4f 2026-07-14)
binary: rustc
commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
commit-date: 2026-07-14
host: x86_64-pc-windows-msvc
release: 1.97.1
LLVM version: 22.1.6

$ cargo --version
cargo 1.97.1 (c980f4866 2026-06-30)

$ cat rust-toolchain.toml
# Pins the toolchain that builds and verifies this repository.
#
# Repository-owned. Not a harness artifact and not an approved architectural decision: it
# records the toolchain that the retained evidence under
# `docs/engineering/simulation/evidence/` was actually produced with, so a later build can
# reproduce it. `docs/engineering/REPOSITORY_CONTEXT.md` states the setup command; this file
# is what makes that command land on one version.
#
# Why an exact version rather than `stable`:
#
#   - `VREC-MOK-001` through `VREC-MOK-006` bind verification to a commit. A commit does not
#     pin a compiler, so "verified at this commit" means "verified with whatever stable was
#     installed that day" unless something records the version. This file records it.
#   - `cargo clippy --workspace --all-targets --all-features -- -D warnings` is a required
#     gate in every work order. A new stable adds lints, so on a floating channel that gate
#     can fail on an unchanged commit — the release candidate would stop verifying without
#     anyone changing it.
#   - The engine package's dependency table is required to stay empty and the observer has
#     exactly one dependency, so a toolchain bump is the only thing that can change what is
#     compiled without a source change.
#
# Both packages declare `edition = "2024"` and the workspace declares `resolver = "3"`, so
# the floor is well above what any currently supported toolchain provides; this pin is about
# reproducibility, not about a minimum.
#
# Raising it is an ordinary repository change, not a governed one, but it invalidates the
# "same toolchain as the evidence" claim above: re-run the required verification for the
# affected work orders and say in the commit message which version was left behind.

[toolchain]
channel = "1.97.1"

# `minimal` plus the two components the quality gates require. The release workflow's build
# matrix compiles for the host only, so no `targets` list is needed here; a cross build would
# add its target on the command line.
profile = "minimal"
components = ["rustfmt", "clippy"]

--- rehearsal against pin-real.toml (channel: 1.97.1) ---
ACCEPTED: pinned 1.97.1 matches rustc 1.97.1 (8bab26f4f 2026-07-14)
exit=0
--- rehearsal against pin-below.toml (channel: 1.96.0) ---
REFUSED: rust-toolchain.toml pins 1.96.0, but rustc is rustc 1.97.1 (8bab26f4f 2026-07-14).
exit=1
--- rehearsal against pin-above.toml (channel: 1.98.0) ---
REFUSED: rust-toolchain.toml pins 1.98.0, but rustc is rustc 1.97.1 (8bab26f4f 2026-07-14).
exit=1
--- rehearsal against pin-empty.toml (channel: ) ---
REFUSED: rust-toolchain.toml declares no channel.
exit=1
```

Four declarations, four outcomes: the real pin accepts; a declaration *below* the available compiler
refuses; a declaration *above* it refuses; a declaration with no channel at all refuses before any
comparison happens. The empty case is the one a bad edit produces, and without it the comparison would
silently succeed against an empty string.

The trailing space in the pattern `"rustc $pinned "*` is load-bearing: without it, a declaration of
`1.9` would match `rustc 1.97.1`. That is the same total-versus-prefix mistake the gate refuses for
commit hashes, in a different place.

## The residual `REQ-MOK-039` names, narrowed but not closed

`VER-MOK-008` records: *"Nothing verifies that the compiler declared today is the compiler the six
existing verification records were produced under."* That stays true, and this declaration cannot
close it, because the artifact graph carries no per-record compiler field. What can be said precisely,
from the repository as it stands:

| Record | Compiler recorded | Matches the declaration |
| --- | --- | --- |
| `VREC-MOK-003` | `rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable`, in the record itself | yes |
| `VREC-MOK-004` | the same, in the record itself | yes |
| `VREC-MOK-002` | `rustc 1.97.1` in `evidence/WO-MOK-002/static-checks.txt` | yes |
| `VREC-MOK-005` | `rustc 1.97.1` in `evidence/WO-MOK-005/static-checks.txt` | yes |
| `VREC-MOK-001` | nothing — no compiler version anywhere under `evidence/WO-MOK-001/` | unknown |
| `VREC-MOK-006` | nothing — no compiler version anywhere under `evidence/WO-MOK-006/` | unknown |

Four of six are corroborated — two by the record, two by retained evidence — and two are silent. The
declaration is consistent with every record that says anything at all. Whether the two silent ones
matter enough to re-verify is the decision `REQ-MOK-039` leaves open, and it belongs to the assurance
owner rather than to this work order.

Searches behind the table:

```text
grep -rniE "toolchain|rustc|cargo 1|edition" docs/engineering/simulation/verification-records/VREC-MOK-00*.md
grep -rniE "rustc [0-9]|cargo [0-9]|toolchain" docs/engineering/simulation/evidence/WO-MOK-001/
grep -rniE "rustc [0-9]|cargo [0-9]|toolchain" docs/engineering/simulation/evidence/WO-MOK-006/
```

The first names only `VREC-MOK-003` and `VREC-MOK-004` (`VREC-MOK-005` matches on the unrelated
sentence about `rustc_version` and `semver` being crates neither binary links). The second and third
return nothing at all.
