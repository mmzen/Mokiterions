# Static and architecture checks — S1 through S11

`VER-MOK-008`'s Evidence retention section requires these as "a written enumeration, since a static
read leaves no artifact of its own." Captured 2026-08-19 against
`.github/workflows/release.yml` (616 lines, 5 jobs, 32 steps — 23 named `run:` steps and 9
marketplace actions), `scripts/check_release_authorization.py`,
`scripts/check_release_reachability.py`, the two scenario suites, and `rust-toolchain.toml`.

Every command read is named. The raw search output is appended at the end rather than paraphrased.

| Check | Finding |
| --- | --- |
| S1 — no tag, branch, commit or push | **holds.** 9 git invocations in the process definition's own run blocks, all read-only, plus 3 inside comments; one outward mutation per run, on one of two mutually exclusive paths |
| S2 — write access is confined | **holds** against rule 12.5 as amended 2026-08-19; the amendment and its reason are below |
| S3 — no artifact is written | **holds.** Every mention of the artifact root is a comment or a read-only argument |
| S4 — no version is changed | **holds.** No manifest or lockfile is written; `--locked` on every resolving command |
| S5 — no secret is required or emitted | **holds.** No `secrets.*` reference at all; one use of `${{ github.token }}` |
| S6 — no check writes to the tracked tree | **holds.** `fmt --check`; `--locked`; determinism transcripts written outside the checkout |
| S7 — the declared compiler version has one home | **holds.** One value, at `rust-toolchain.toml:31`; every step reads the file |
| S8 — the managed harness workflow is untouched | **holds.** `doctor` PASS at harness `0.4.0`, 81 lines, no `FAIL`; neither invoked nor modified |
| S9 — the engine's dependency table is unchanged | **holds.** `[dependencies]` empty; `cargo tree` resolves one crate |
| S10 — the human procedure lives outside the artifact root | **holds.** `docs/RELEASE_RUNBOOK.md`, root is `docs/engineering` |
| S11 — the process definition parses | **holds.** Parsed by PyYAML; jobs, refs and permissions as declared |

## S1 — no tag, branch, commit or push

Every command the process runs, enumerated. The process is the workflow plus the two programs it
invokes, so the programs' git usage is part of the enumeration rather than treated as opaque.

**In the workflow's own `run:` blocks** (5 distinct invocations, one repeated per job):

| Where | Command | Kind |
| --- | --- | --- |
| `authorize` step 3 | `git rev-parse HEAD`, `git rev-parse "origin/$default"` | read |
| `authorize` step 3 | `git log -1 --format="governance revision %H (%ci) %s" HEAD` | read |
| `authorize` step 5 | `git for-each-ref "refs/tags/$RELEASE_TAG" --format=…` | read |
| `harness` step 2 | `git rev-parse HEAD` | read |
| `verify` step 2 | `git rev-parse HEAD` | read |
| `verify` step 8 | `git status --porcelain --untracked-files=all` | read |
| `build` step 2 | `git rev-parse HEAD` | read |

**In `scripts/check_release_authorization.py`** (2):

- `git rev-parse --verify --quiet refs/tags/<tag>^{commit}` — resolves the tag, in the tag namespace
  only, so a branch of the same name cannot impersonate it (`:205`)
- `git cat-file -t refs/tags/<tag>` — the object type, refusing a lightweight tag (`:218`)

**In `scripts/check_release_reachability.py`** (5):

- `git cat-file -e <commit>^{commit}` (`:74`)
- `git symbolic-ref --quiet refs/remotes/<remote>/HEAD` (`:86`)
- `git rev-parse --verify --quiet refs/remotes/<remote>/<default>` (`:102`)
- `git for-each-ref --format=%(refname) refs/remotes/<remote>/release/*` (`:107`)
- `git merge-base --is-ancestor <commit> <reference>` (`:126`)

No `git tag`, `git push`, `git commit`, `git branch`, `git switch`, `git checkout`, `git update-ref` or
`git reset` appears in any executed command. The search for those verbs returns exactly two lines, both
inside the trailing comment block that records the commands the *release owner* runs by hand:

```text
614:#     git switch --create release/0.1 <authorized commit>
615:#     git push --set-upstream origin release/0.1
```

Those are documentation of a reserved act, not an invocation of one. `actions/checkout@v4` performs a
clone into the runner's own workspace, which creates nothing on the remote.

**Failure and cleanup paths.** There is no `continue-on-error`, no `always()` cleanup that mutates
anything, and no trap. The one `if: always()` is on `actions/upload-artifact@v4` for the harness
dashboard, which writes to the run's artifact store. Every refusal is `echo "REFUSED: …" >&2; exit 1`
and nothing after it runs, because a failed step fails its job and every downstream job declares
`needs:`.

**The outward mutations,** all in the `publish` job's third step, and all through `gh`:

| Line | Call | Kind |
| --- | --- | --- |
| 580 | `gh release view "$RELEASE_TAG" --json id` | read — does a release already exist? |
| 583 | `gh release edit "$RELEASE_TAG" --notes-file notes.md` | mutation, **re-run path only** |
| 586 | `gh release upload "$RELEASE_TAG" --clobber incoming/*` | mutation, **re-run path only** |
| 591 | `gh release create "$RELEASE_TAG" --verify-tag --draft … incoming/*` | mutation, **first-run path only** |
| 602 | `gh release view "$RELEASE_TAG" --json isDraft,url` | read — reported, not asserted |

The two paths are the branches of one `if`, so exactly one of them runs. Attachment is a single call on
either path, carrying every asset of every target, which is what rule 12.3 requires. On the first-run
path `--verify-tag` makes `gh` abort rather than create a tag when the tag is absent, and `--draft`
leaves publication to a person. On the re-run path the release already exists, so no tag can be
created; `--clobber` replaces assets and neither call names `--draft` in either direction, so the
release's visibility is whatever the owner last set it to. Nothing deletes a release (rule 13.9).

This second path is a change from the pre-existing candidate, which skipped `publish` entirely on
`workflow_dispatch`. `SPEC-MOK-005` rule 2 says the explicit form *"re-runs a release; it does not
preview one"*, so the skip was a conformance gap rather than a safety measure. See
`candidate-conformance.md`.

## S2 — write access is confined

| Scope | Grant |
| --- | --- |
| workflow default | `contents: read` |
| `authorize`, `harness`, `verify`, `build` | inherited — `contents: read` |
| `publish` | `contents: write` |

Exactly one job holds write access, and within it exactly one step receives a credential:
`GH_TOKEN: ${{ github.token }}` on *Attach assets to the existing tag* (line 556). The download and
notes steps receive no token.

**Why the grant is job-level, and why the rule now says so.** Rule 12.5 originally asked that write access
be confined to the *step* that attaches assets. GitHub Actions scopes `permissions` **per job, not per
step**, so a step-level grant does not exist to be written, and the rule was unsatisfiable as stated. That
was raised as a specification question rather than treated as an implementation choice, and **the technical
owner amended 12.5 on 2026-08-19**: write access is confined to the smallest job containing the attaching
step, and within that job the credential is passed to the attaching step alone.

That is exactly what the table above measures, so S2 now holds against the rule as written rather than up to
a limitation. The platform reasoning stays in a comment at the grant itself (lines 511–514) so that a future
reader does not read the job-level scope as carelessness. Splitting `publish` into two jobs would not
narrow it further — the attaching job would still need `contents: write` — so the amended rule is at the
floor of what this platform can express, not a concession above it.

`publish` additionally declares `environment: release` (line 520), which rule 12.6 requires. Nothing in
the repository configures that environment; the reference exists so that a required reviewer can be
added in repository settings without editing the process. That is the point of the rule, and it is also
the limit of what this changeset can establish: the protection is a setting, not a file.

## S3 — no artifact is written

`docs/engineering` appears five times in the definition: all five in comments (lines 10, 18, 241, 568,
608), none as a write target. No `run:` block redirects into the artifact root, edits a `+++` block, or
touches a `status` field. The harness steps invoke `doctor`, `validate_engineering_artifacts.py`,
`preflight` and `generate_harness_dashboard.py`; the first three are read-only by their own authority
statements, and the dashboard writes to `target/harness-dashboard`, which is outside the artifact root
and ignored by git. That directory is what `actions/upload-artifact@v4` then collects.

Both gate programs write to `$GITHUB_OUTPUT` and nothing else. See `p4-worktree-comparison.md` for the
observational half of this claim.

## S4 — no version is changed

No `run:` block names `Cargo.toml` or `Cargo.lock` as a write target; the only mention of either is a
comment identifying where the engine's dependency table lives (line 301). Every command that resolves
dependencies carries `--locked`:

```text
$ grep -n -- '--locked' .github/workflows/release.yml
292:        run: cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
295:        run: cargo test --workspace --locked
303:          tree="$(cargo tree -p Mokiterions --locked)"
331:              cargo run --quiet --release --locked -p Mokiterions -- \
436:          cargo build --release --locked --target "${{ matrix.target }}" \
```

`cargo fmt --all -- --check` (line 289) resolves nothing. `--locked` makes cargo fail rather than
update `Cargo.lock`, which is the fail-closed form of "the lockfile is honoured and not updated".

One divergence worth recording: `docs/engineering/REPOSITORY_CONTEXT.md` declares the lint command
*without* `--locked`. The workflow adds it, because rule 8.2 requires dependency resolution from the
committed lockfile. `--locked` is strictly stricter, so the declared command's outcome is unchanged
whenever the lockfile is already current; adding the flag to the declaration in
`REPOSITORY_CONTEXT.md` was left alone as outside this work order's scope.

## S5 — no secret is required or emitted

`grep -n 'secrets\.'` over the definition returns **nothing**. The only credential is
`${{ github.token }}`, the platform's own scoped token, used once.

The provenance statement's construction (lines 462–474) reads: the authorized facts passed as `env:`
from the `authorize` job (`RECORD`, `CONTRACT`, `WORK_ORDERS`, `VERIFICATION_RECORDS`, `RELEASE_TAG`,
`AUTHORIZED_COMMIT`, `VERSION`), the matrix target, `rustc --version`, and the run identity
(`github.server_url`, `github.repository`, `github.run_id`). No home directory, no runner hostname, no
timestamp, no token.

Precise about one thing: the same *step* also reads `$RUNNER_OS`, to choose the `.exe` suffix when
copying binaries. That value does not reach `PROVENANCE.txt`. S5 speaks of what the statement's
construction reads *into the statement*, and the target — which is in the statement — already names the
platform.

**One environment value the build asserts is absent.** `mokiterions-tui/src/render.rs:34` reads
`option_env!("MOKITERIONS_COMMIT")` and renders it in the interface footer when it is set, and rule 10.7
forbids a commit stamp in a released binary. Not setting it is not the same as knowing it is unset: a
repository or organization variable, or a runner image, can put it in the environment without this file
mentioning it, and nothing downstream would notice — no rebuild input changes and no test compiles with
it set, so the damage would appear only in a shipped binary. The `Build` step therefore refuses when the
variable is present (line 432, refusal at line 433), which makes rule 10.7 a check rather than an
intention. The two comments above it (lines 408 and 425) record the reasoning. Appendix A's second
search surfaces all four lines.

## S6 — no check writes to the tracked tree

`cargo fmt --all -- --check` reports and exits non-zero; it does not reformat. `--locked` prevents a
lockfile rewrite. The determinism step's four transcripts are written to `$RUNNER_TEMP`, outside the
checkout, and the step ends with a logged `git status --porcelain --untracked-files=all` (line 350) so
a regression is visible in the transcript. See `determinism-rehearsal.md`, which measured `git status`
identical before and after.

## S7 — the declared compiler version has one home

`grep -rn '1\.97\.1' . --exclude-dir=target --exclude-dir=.git --exclude-dir=WO-MOK-009` returns 20
lines. Exactly **one** is the version as a *value*:

```text
./rust-toolchain.toml:31:channel = "1.97.1"
```

The other nineteen are observations, in three classes: retained evidence recording the compiler a past
measurement was taken with (`WO-MOK-002`, `WO-MOK-003`, `WO-MOK-004`, `WO-MOK-005`); verification
records stating the toolchain their work was verified under (`VREC-MOK-003`, `VREC-MOK-004`); and the
new chain's own prose (`INT-MOK-007:198`, `CAP-MOK-007:128`, and `REQ-MOK-039`'s three scenario rows,
which quote versions as scenario inputs). None is read by any step.

This work order's own evidence directory is excluded from the search. It holds further occurrences of
the same three classes — 17 in `toolchain-evidence.md`, 2 in `verification-output.md`, and a great many
in this file, which quotes the search output itself. Including the directory would make the search grow
with every file written here without changing the finding, and it would make this file's own count a
count of this appendix. Appendix C reports the two countable files.

Both steps that need the version read it from the file:

```text
$ grep -n 'pinned=' .github/workflows/release.yml
275:          pinned="$(sed -n 's/^channel *= *"\(.*\)"/\1/p' rust-toolchain.toml)"
398:          pinned="$(sed -n 's/^channel *= *"\(.*\)"/\1/p' rust-toolchain.toml)"
```

## S8 — the managed harness workflow is untouched

```text
PASS distribution:.github/workflows/engineering-harness.yml: matches distribution
PASS managed:.github/workflows/engineering-harness.yml: unchanged
```

That is `python -m se_harness doctor .` at harness `0.4.0` — the version
`.engineering-harness.toml` declares and the version the workflow installs as a pinned wheel — and it
is the authoritative measurement. The whole run is 81 `PASS` lines and no `FAIL`.

The version qualification is not decoration. The `distribution:` line above compares this repository's
copy against the template the *installed harness release* ships, so it is a statement about a pair.
Under harness `0.4.1` the same file at the same commit reports
`FAIL distribution:.github/workflows/engineering-harness.yml: differs from distribution template`,
along with seven others, while the `managed:` line still passes. Nothing has been tampered with in
either reading; the two lines answer different questions. `compliance-rehearsal.md` C1 records both
runs and the rule 7.1 check that refuses before `doctor` can produce the confusing one.

A hand check is recorded here because it is instructive rather than because it is needed. Hashing the
file's raw bytes on this Windows worktree yields
`0eb859f630f1fc4a53f908d0fa69a8183b7d60eca46d0408a4f6f0a8d0b66732`, which does **not** equal the
`.engineering-harness.lock` entry `0b850207fb04c054f3c936bcbf2e2a18c1d9f79de14232b25fa328a0159c5ed4`.
The lock says why in its own header: `hash_mode` is `utf8-text-lf-v1`, so the recorded digest is over
LF-normalized UTF-8 text, and this worktree checks the file out with CRLF endings. Normalizing first
reproduces the recorded digest exactly. A naive byte comparison in a future check would therefore
report a false mismatch on Windows, which is worth knowing before someone writes one — and the mode
field is the answer, not a workaround.

The release workflow neither invokes nor modifies it: `engineering-harness.yml` appears twice in the
definition, both in comments (lines 4 and 153), and rule 13.8's reason is recorded there — the managed
workflow declares no `workflow_call` trigger, so the compliance job re-runs the checks itself rather
than calling it. The two other matches for the string are `.engineering-harness.toml`, the
configuration file, which is read for `tool_version`.

The lock also settles what this work order is allowed to write. Six of the seven files it creates are
absent from the lock entirely — the two gate programs, both test files, `rust-toolchain.toml`,
`docs/RELEASE_RUNBOOK.md` and `.github/workflows/release.yml` — so none of them is a managed file this
work could be damaging. The seventh, `docs/engineering/REPOSITORY_CONTEXT.md`, is present as
`{"mode": "seed", "state": "present"}`: recorded, but with no `sha256`, which is the harness's way of
saying the file was seeded and is the repository's to edit thereafter. That is the basis for
`WO-MOK-009`'s scope line permitting its setup and commands sections to change.

## S9 — the engine's dependency table is unchanged

`mokiterions-core/Cargo.toml` declares `[dependencies]` with nothing under it.
`mokiterions-tui/Cargo.toml` declares two, `Mokiterions` by path and `ratatui`. `cargo tree -p
Mokiterions --locked` resolves to a single crate. No rule of this work adds a dependency to either
package, and `git status --porcelain --untracked-files=all` filtered to `*Cargo.toml`, `*Cargo.lock`
and `*.rs` prints nothing.

The workflow re-checks this at the authorized commit rather than trusting it: the *engine's dependency
table must stay empty* step counts `cargo tree` output lines and refuses if the count is not exactly 1.

## S10 — the human procedure lives outside the artifact root

`[harness].artifact_root` is `docs/engineering`. The procedure is `docs/RELEASE_RUNBOOK.md`;
`docs/engineering/RELEASE_RUNBOOK.md` does not exist. So the runbook cannot be mistaken for an artifact
carrying authority, is not walked by the validator, and has no status field. Its own header says as
much in the first four lines.

## S11 — the process definition parses

Parsed with PyYAML (a general parser for the format, not a GitHub-specific one). Declared steps and
revisions:

| job | checkout ref | permissions | steps |
| --- | --- | --- | --- |
| `authorize` | `${{ github.event.repository.default_branch }}` | inherited | 6 |
| `harness` | `${{ needs.authorize.outputs.governance_commit }}` | inherited | 9 |
| `verify` | `${{ needs.authorize.outputs.commit }}` | inherited | 8 |
| `build` | `${{ needs.authorize.outputs.commit }}` | inherited | 6 |
| `publish` | none — consumes artifacts | `contents: write` | 3 |

Which is what S11 requires: authorization at the default branch, compliance at the pinned governance
revision, checks and builds at the authorized commit.

No job carries an `if:` condition. That is a change from the candidate, where `publish` carried
`if: github.event_name == 'push'`; see S1 and `candidate-conformance.md`.

Each of the three revision claims is additionally asserted *at run time*, not merely declared —
`harness` step 2, `verify` step 2 and `build` step 2 compare `git rev-parse HEAD` against the value
they were handed and refuse on a mismatch. `build` step 2 also refuses an authorized commit that is not
a complete lowercase hash. That turns `VER-MOK-008`'s *"the governance revision is fixed"* property and
manual assessment M5 from something read out of a log into something the run enforces.

## Appendix A — S1 through S6 searches

```text
### S1 — forbidden mutating verbs in the process definition
$ grep -nE 'git (tag|push|commit|branch|switch|checkout|update-ref|reset)|gh release (create|edit|upload|delete)|gh api' .github/workflows/release.yml
565:          #   first run -- `gh release create`. `--verify-tag` makes gh abort if the tag does
571:          #   re-run -- `gh release upload --clobber`, which replaces the assets of the
583:            gh release edit "$RELEASE_TAG" \
586:            gh release upload "$RELEASE_TAG" \
591:            gh release create "$RELEASE_TAG" \
614:#     git switch --create release/0.1 <authorized commit>
615:#     git push --set-upstream origin release/0.1

### S1 — the steps that exist to be read
$ grep -cE '^\s+- name:' .github/workflows/release.yml   # named run: steps
23
$ grep -nE '^\s+- uses:' .github/workflows/release.yml   # marketplace actions
83:      - uses: actions/checkout@v4
91:      - uses: actions/setup-python@v5
161:      - uses: actions/checkout@v4
180:      - uses: actions/setup-python@v5
233:      - uses: actions/upload-artifact@v4
249:      - uses: actions/checkout@v4
370:      - uses: actions/checkout@v4
492:      - uses: actions/upload-artifact@v4
522:      - uses: actions/download-artifact@v4

### S2 — write access
$ grep -n -A3 "permissions:" .github/workflows/release.yml
53:permissions:
54-  contents: read
55-
56-concurrency:
--
515:    permissions:
516-      contents: write
517-    # Configure this environment in repository settings with a required reviewer if the
518-    # release owner wants a second human gate before any asset is uploaded. Referenced

$ grep -n 'GH_TOKEN\|GITHUB_TOKEN\|secrets\.' .github/workflows/release.yml
556:          GH_TOKEN: ${{ github.token }}

### S3 — writes under the artifact root
$ grep -n "docs/engineering" .github/workflows/release.yml
10:# `docs/engineering/DECISION_RIGHTS.md` reserve these acts to accountable humans:
18:# Everything it does do is downstream of a decision already recorded in `docs/engineering`.
241:  #    Commands come from `docs/engineering/REPOSITORY_CONTEXT.md`; the last two discharge
568:          #     `docs/engineering/TRACEABILITY.md` treats publication as an authorized act,
608:# `docs/engineering/REPOSITORY_CONTEXT.md` has `release/<major>.<minor>` cut from the

### S4/S6 — manifests, lockfile, formatter mode
$ grep -n 'Cargo.toml\|Cargo.lock\|cargo update\|cargo fmt\|--locked\|--offline' .github/workflows/release.yml
289:        run: cargo fmt --all -- --check
292:        run: cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
295:        run: cargo test --workspace --locked
301:          # including one shared with `mokiterions-tui`. `mokiterions-core/Cargo.toml`
303:          tree="$(cargo tree -p Mokiterions --locked)"
331:              cargo run --quiet --release --locked -p Mokiterions -- \
436:          cargo build --release --locked --target "${{ matrix.target }}" \

### S5 — environment values read by the provenance statement
$ grep -n 'github\.\|runner\.\|env\.' .github/workflows/release.yml | grep -v '^[0-9]*: *#'
57:  group: release-${{ inputs.tag || github.ref_name }}
61:  RELEASE_TAG: ${{ inputs.tag || github.ref_name }}
88:          ref: ${{ github.event.repository.default_branch }}
99:          default="${{ github.event.repository.default_branch }}"
144:            --default-branch "${{ github.event.repository.default_branch }}" \
473:            echo "built by             ${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}"
556:          GH_TOKEN: ${{ github.token }}

$ grep -n 'RUNNER_OS\|RUNNER_TEMP\|MOKITERIONS_COMMIT' .github/workflows/release.yml
327:          out="${RUNNER_TEMP:-$(mktemp -d)}"
408:      # `MOKITERIONS_COMMIT` is deliberately NOT set, even though `render.rs` reads it via
425:      # variable, or a runner image, can put `MOKITERIONS_COMMIT` in the environment without
432:          if [[ -n "${MOKITERIONS_COMMIT:-}" ]]; then
433:            echo "REFUSED: MOKITERIONS_COMMIT is set in this environment; rule 10.7 forbids a commit stamp in a released binary." >&2
452:          if [[ "$RUNNER_OS" == "Windows" ]]; then suffix=".exe"; fi
```

## Appendix B — git invocations

```text
### git subcommands invoked by the two programs
$ grep -n 'git(' scripts/check_release_authorization.py scripts/check_release_reachability.py
scripts/check_release_authorization.py:191:def git(root: Path, *arguments: str) -> subprocess.CompletedProcess[str]:
scripts/check_release_authorization.py:205:    completed = git(root, "rev-parse", "--verify", "--quiet", f"refs/tags/{tag}^{{commit}}")
scripts/check_release_authorization.py:218:    completed = git(root, "cat-file", "-t", f"refs/tags/{tag}")
scripts/check_release_reachability.py:45:def git(root: Path, *arguments: str) -> subprocess.CompletedProcess[str]:
scripts/check_release_reachability.py:74:    completed = git(root, "cat-file", "-e", f"{commit}^{{commit}}")
scripts/check_release_reachability.py:86:    completed = git(root, "symbolic-ref", "--quiet", f"refs/remotes/{remote}/HEAD")
scripts/check_release_reachability.py:102:    if git(root, "rev-parse", "--verify", "--quiet", default_reference).returncode == 0:
scripts/check_release_reachability.py:105:    completed = git(
scripts/check_release_reachability.py:126:    return git(root, "merge-base", "--is-ancestor", commit, reference).returncode == 0

### git subcommands appearing in the workflow's own run blocks
$ grep -n 'git ' .github/workflows/release.yml
106:          if [[ "$(git rev-parse HEAD)" != "$(git rev-parse "origin/$default")" ]]; then
110:          echo "commit=$(git rev-parse HEAD)" >> "$GITHUB_OUTPUT"
111:          git log -1 --format="governance revision %H (%ci) %s" HEAD
126:          git for-each-ref "refs/tags/$RELEASE_TAG" \
173:          head="$(git rev-parse HEAD)"
260:          head="$(git rev-parse HEAD)"
350:          git status --porcelain --untracked-files=all
382:          head="$(git rev-parse HEAD)"
538:          # Not generated from `git log`: the release notes state what the release record
614:#     git switch --create release/0.1 <authorized commit>
615:#     git push --set-upstream origin release/0.1
```

## Appendix C — S7, S9 and S10 searches

```text
### S7 — where the declared compiler version appears
# This work order's own evidence directory is excluded. It quotes the version many times --
# every table and transcript below does -- and including it would make the search grow with
# each file written without changing the finding. Its count is reported separately.
$ grep -rn --binary-files=without-match '1\.97\.1' . --exclude-dir=target --exclude-dir=.git --exclude-dir=WO-MOK-009
./docs/engineering/simulation/capabilities/CAP-MOK-007.md:128:  `rustc 1.97.1 (8bab26f4f 2026-07-14)`.
./docs/engineering/simulation/evidence/WO-MOK-002/completion-summary.md:63:Toolchain: cargo 1.97.1, rustc 1.97.1. Full output in `static-checks.txt` and `test-run.txt`.
./docs/engineering/simulation/evidence/WO-MOK-002/static-checks.txt:8:cargo 1.97.1 (c980f4866 2026-06-30)
./docs/engineering/simulation/evidence/WO-MOK-002/static-checks.txt:9:rustc 1.97.1 (8bab26f4f 2026-07-14)
./docs/engineering/simulation/evidence/WO-MOK-003/compile-time.md:16:(rustc 1.97.1, cargo 1.97.1), debug profile, no `--release`.
./docs/engineering/simulation/evidence/WO-MOK-003/completion-summary.md:127:Toolchain: rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable.
./docs/engineering/simulation/evidence/WO-MOK-003/README.md:17:| Toolchain | rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable |
./docs/engineering/simulation/evidence/WO-MOK-003/static-checks.txt:2:rustc 1.97.1 (8bab26f4f 2026-07-14)
./docs/engineering/simulation/evidence/WO-MOK-003/static-checks.txt:3:cargo 1.97.1 (c980f4866 2026-06-30)
./docs/engineering/simulation/evidence/WO-MOK-004/baseline-comparison.md:14:| Toolchain | rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable |
./docs/engineering/simulation/evidence/WO-MOK-004/README.md:19:| Toolchain | rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable |
./docs/engineering/simulation/evidence/WO-MOK-005/static-checks.txt:19:cargo 1.97.1 (c980f4866 2026-06-30)
./docs/engineering/simulation/evidence/WO-MOK-005/static-checks.txt:21:rustc 1.97.1 (8bab26f4f 2026-07-14)
./docs/engineering/simulation/intent/INT-MOK-007.md:198:  is `rustc 1.97.1 (8bab26f4f 2026-07-14)`; both packages declare `edition = "2024"` and the workspace declares
./docs/engineering/simulation/requirements/REQ-MOK-039.md:116:Given the declared version `1.97.1` and a build environment providing `rustc 1.97.1`,
./docs/engineering/simulation/requirements/REQ-MOK-039.md:121:Given the declared version `1.97.1` and a build environment providing `rustc 1.98.0`,
./docs/engineering/simulation/requirements/REQ-MOK-039.md:126:Given the declared version `1.97.1` and an environment providing `rustc 1.90.0`,
./docs/engineering/simulation/verification-records/VREC-MOK-003.md:46:Toolchain: rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable, edition 2024.
./docs/engineering/simulation/verification-records/VREC-MOK-004.md:51:Toolchain: rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable, edition 2024.
./rust-toolchain.toml:31:channel = "1.97.1"

$ grep -rc '1\.97\.1' <this work order's other evidence files>
# static-checks.md is omitted from this count: it quotes the search results above verbatim,
# so counting its matches would be counting this appendix.
docs/engineering/simulation/evidence/WO-MOK-009/toolchain-evidence.md:17
docs/engineering/simulation/evidence/WO-MOK-009/verification-output.md:2

$ grep -n 'rust-toolchain\|pinned=' .github/workflows/release.yml   # every step reads the file
270:          # `rust-toolchain.toml` selects the version and declares rustfmt and clippy, so
275:          pinned="$(sed -n 's/^channel *= *"\(.*\)"/\1/p' rust-toolchain.toml)"
277:            echo "REFUSED: rust-toolchain.toml declares no channel." >&2
284:            echo "REFUSED: rust-toolchain.toml pins $pinned, but rustc is $(rustc --version)." >&2
398:          pinned="$(sed -n 's/^channel *= *"\(.*\)"/\1/p' rust-toolchain.toml)"
404:            echo "REFUSED: rust-toolchain.toml pins $pinned, but rustc is $(rustc --version)." >&2

### S9 — the engine's dependency table
$ sed -n '/\[dependencies\]/,/^\[/p' mokiterions-core/Cargo.toml
[dependencies]

$ sed -n '/\[dependencies\]/,$p' mokiterions-tui/Cargo.toml
[dependencies]
Mokiterions = { path = "../mokiterions-core" }
ratatui = { version = "0.30.2", default-features = false, features = [
  "crossterm",
  "layout-cache",
  "underline-color",
] }

$ git status --porcelain -- '*Cargo.toml' '*Cargo.lock' '*.rs'
(nothing printed above means no manifest, lockfile or source file is in the changeset)

### S10 — the human procedure is outside the artifact root
$ python -c "print(tomllib.loads(...)[harness][artifact_root])"
docs/engineering
$ ls docs/RELEASE_RUNBOOK.md docs/engineering/RELEASE_RUNBOOK.md
ls: cannot access 'docs/engineering/RELEASE_RUNBOOK.md': No such file or directory
docs/RELEASE_RUNBOOK.md
```

## Appendix D — S8 and S11, parsed

```text
### S11 — the process definition, parsed by PyYAML
triggers: ['push', 'workflow_dispatch']
workflow permissions: {'contents': 'read'}
workflow env keys: ['CARGO_TERM_COLOR', 'RELEASE_TAG', 'SE_HARNESS_VERSION']
concurrency: {'group': 'release-${{ inputs.tag || github.ref_name }}', 'cancel-in-progress': False}

job        checkout ref                                              permissions            steps
authorize  ${{ github.event.repository.default_branch }}             inherited              6
harness    ${{ needs.authorize.outputs.governance_commit }}          inherited              9
verify     ${{ needs.authorize.outputs.commit }}                     inherited              8
build      ${{ needs.authorize.outputs.commit }}                     inherited              6
publish    (no checkout)                                             {'contents': 'write'}  3

[authorize] needs=None if=None environment=None
    1. actions/checkout@v4
    2. actions/setup-python@v5
    3. Pin the governance revision every later job will read
    4. Refuse to publish without a released release record for this tag
    5. Record who created the tag
    6. The authorized commit must be reachable from a release-bearing branch
[harness] needs=authorize if=None environment=None
    1. actions/checkout@v4
    2. This job must be standing on the pinned governance revision
    3. actions/setup-python@v5
    4. Install the declared candidate runtime
    5. The installed harness must be the version the repository declares
    6. Managed-file integrity and formal graph validity
    7. Review preflight for every released work order
    8. Harness Explorer for the released commit
    9. actions/upload-artifact@v4
[verify] needs=authorize if=None environment=None
    1. actions/checkout@v4
    2. This job must be standing on the authorized commit
    3. Toolchain, from the repository's pin
    4. Format
    5. Lint
    6. Test
    7. The engine's dependency table must stay empty
    8. A seed determines the run, byte for byte
[build] needs=['authorize', 'harness', 'verify'] if=None environment=None
    1. actions/checkout@v4
    2. The assets must be built from the authorized commit, stated in full
    3. Toolchain and target
    4. Build
    5. Package
    6. actions/upload-artifact@v4
[publish] needs=['authorize', 'harness', 'verify', 'build'] if=None environment=release
    1. actions/download-artifact@v4
    2. Notes, stated from the authorized facts only
    3. Attach assets to the existing tag

### S8 — the managed harness workflow
lock hash_algorithm: sha256
lock hash_mode:      utf8-text-lf-v1   <- hashes are over LF-normalized UTF-8 text
recorded entry: {"mode": "managed", "sha256": "0b850207fb04c054f3c936bcbf2e2a18c1d9f79de14232b25fa328a0159c5ed4"}
measured, raw bytes as checked out here: 0eb859f630f1fc4a53f908d0fa69a8183b7d60eca46d0408a4f6f0a8d0b66732
measured, CRLF normalized to LF:         0b850207fb04c054f3c936bcbf2e2a18c1d9f79de14232b25fa328a0159c5ed4
LF-normalized measurement equals the recorded digest: True

### S8 — everything this work order writes, against the lock
scripts/check_release_authorization.py        not in the lock
scripts/check_release_reachability.py         not in the lock
scripts/test_check_release_authorization.py   not in the lock
scripts/test_check_release_reachability.py    not in the lock
rust-toolchain.toml                           not in the lock
docs/RELEASE_RUNBOOK.md                       not in the lock
.github/workflows/release.yml                 not in the lock
docs/engineering/REPOSITORY_CONTEXT.md        {"mode": "seed", "state": "present"}
```
