# Post-release observations for Mokiterions 0.1.0

`v0.1.0` was published at `2026-08-19T20:01:16Z`. `REL-MOK-001` opens a seven-day observation window
at publication, so the window closes on **2026-08-26**. This file is where that window's observations
are retained.

**This is not evidence for `VREC-MOK-009`, and it is deliberately not in that record's directory.**
`VREC-MOK-009`'s evidence set is the nine files in `../release-0.1.0/`, fixed by its `evidence_paths`
frontmatter and frozen by the assurance owner's decision. Everything here was observed *after* that
decision and after publication. Adding it beside those nine would make a frozen evidence directory
look like it had grown.

It is also not an artifact. It approves nothing, transitions nothing, and grants nothing.
`REL-MOK-001` says the window closing "is not itself a decision and grants nothing"; this file
inherits that. Where something observed here needs acting on, the act is a work order, not a line in
this file.

## The five things the contract asks to observe

`REL-MOK-001`'s "Post-release observation window" numbers them in order of what would matter most.
Each is answered below under its own number.

| # | What the contract asks for | Answer |
| --- | --- | --- |
| 1 | The workflow's own first real run | All seven jobs succeeded; no step behaved differently against a real tag |
| 2 | Asset integrity | Three archives verified from the published bytes; two of three targets not started |
| 3 | `PROVENANCE.txt` against `RLS-MOK-001` | Matches on every field, all forty commit characters |
| 4 | The gate, re-run after the fact | `AUTHORIZED`, exit `0` — against a real published tag |
| 5 | Issues reported against the release | None |

One thing not on that list was observed and is recorded as a **known gap** in the last section: the
`release` environment now exists and protects nothing.

## 1. The workflow's first real run

Run `32294966039`, the first execution of `.github/workflows/release.yml` in this repository's
history. Triggered by the `v0.1.0` tag push (`event: push`, head `755db72…`), started
`2026-08-19T19:47:27Z`, finished `2026-08-19T19:49:37Z` — two minutes ten seconds end to end.

| Job | Result |
| --- | --- |
| Authorization | success |
| Repository checks | success |
| Harness compliance | success |
| Build `x86_64-unknown-linux-gnu` | success |
| Build `aarch64-apple-darwin` | success |
| Build `x86_64-pc-windows-msvc` | success |
| Publish draft assets | success |

Seven jobs, seven successes, no retry and no re-run. **The observation the contract asks for is
whether any step that had only ever run against fixtures behaved differently against a real tag. None
did.** Every figure the run reported matches what was measured locally at the candidate:

| Check | CI reported | Locally measured |
| --- | --- | --- |
| Authorization gate | `AUTHORIZED` | `AUTHORIZED` |
| Reachability | `REACHABLE` | `REACHABLE` |
| Harness version | repository declares `0.4.0`; runner has `0.4.0` | `0.4.0` |
| `cargo test --workspace --locked` | **179 passed, 0 failed** across 19 test binaries | 179 passed, 0 failed |
| `doctor` | 81 PASS, 0 WARN, 0 FAIL | 81 PASS, 0 WARN, 0 FAIL |
| `validate` | PASS — 95 artifacts, 0 errors, 0 warnings, four planes E0/W0 | same |
| Review preflight | passes for `WO-MOK-001`..`007`, `WO-MOK-009` | same eight |
| Tag authorship | `tagged by mmzen <mathieu.meadele@gmail.com> on 2026-08-19T21:45:26+02:00` | — |

The run also retained the dashboard for the governance revision as workflow artifact
`harness-dashboard-0.1.0` (3 files, 52,573 bytes, artifact ID `9380873687`). GitHub expires workflow
artifacts, so that is a convenience copy and not durable evidence.

### The governance revision the run read was not the one that was expected

This is the run's one genuine surprise, and it is a vindication of the design rather than a defect.

PR #17 (`feature/phase-2-individuality`, `WO-MOK-010`) merged to `master` **thirty seconds after the
tag push** — the tag was created at `19:45:26Z` and the merge landed at `19:45:55Z`, while the
workflow was still queuing. So the run pinned:

```
governance revision 5eed5a97235ab73d9964ff3ccde059b0ef74ddb3 (2026-08-19 21:45:55 +0200) Merge pull request #17 from mmzen/feature/phase-2-individuality
```

The compliance job therefore validated a graph containing `WO-MOK-010` and `VREC-MOK-010` — work that
is **not** in `v0.1.0` — and passed, which is correct: the compliance job's subject is the governance
revision's formal graph, not the candidate's tree. `docs/RELEASE_RUNBOOK.md` states the separation
directly, and this run is the first evidence that it holds under a real concurrent merge:

- The **candidate** is `755db72…`. It is what was built, what the tag points at, and it does not
  contain `WO-MOK-010`'s sources.
- The **governance revision** is `5eed5a9…`. It is what the graph was read from, and it does.

Both checks were re-run afterwards against the moved `master` and still pass (§4), so the release is
unaffected by the merge. Had the workflow read `master` per job instead of pinning it once, two jobs
could have read different graphs; the pin is why that could not happen here.

## 2. Asset integrity

Every figure below is from the **published** assets, downloaded from
`https://github.com/mmzen/Mokiterions/releases/download/v0.1.0/` with no authentication — the
recipient's view, not the maintainer's.

| Asset | Bytes | `sha256sum -c` |
| --- | --- | --- |
| `Mokiterions-0.1.0-x86_64-unknown-linux-gnu.tar.gz` | 707,362 | **OK** |
| `Mokiterions-0.1.0-aarch64-apple-darwin.tar.gz` | 652,216 | **OK** |
| `Mokiterions-0.1.0-x86_64-pc-windows-msvc.tar.gz` | 392,642 | **OK** |

Each archive holds one directory with six files: both binaries, `LICENSE`, `README.md`,
`SIMULATION_RULES.md`, `PROVENANCE.txt`. `REL-MOK-001`'s release unit is satisfied on all three.

The two Unix archives record the executable bit — `-rwxr-xr-x` on `Mokiterions` and
`mokiterions-tui`, `-rw-r--r--` on the four documents. That was worth checking rather than assuming:
an archive whose binaries arrive without `+x` is unusable without a `chmod` nobody was told about.

**The Windows binaries were run, from the published archive:**

- `Mokiterions.exe --help` and `mokiterions-tui.exe --help` both exit `0` and print the usage the
  specifications describe.
- `--seed 42 --ticks 50` run twice is **byte-identical**; `--seed 43 --ticks 50` differs. Exit `0`
  each time. Determinism from the seed holds in a released binary.
- The output contains no token of seven or more hexadecimal characters. **`SPEC-MOK-005` rule 10.7
  holds where it matters:** `MOKITERIONS_COMMIT` was left unset in the build, so no commit stamp
  reached a shipped binary.

**What was not exercised, stated plainly.** The Linux and macOS binaries were **not started**. No
host of either kind was used. `REL-MOK-001` already singles out `aarch64-apple-darwin` as
cross-built with no test run on that architecture; that limitation stands, and it now applies to the
Linux archive's binaries too as far as this window is concerned.

For the darwin target one thing short of running it was established: both binaries carry the
Mach-O 64-bit header `cf fa ed fe` with cputype `0x0100000c`, which is `CPU_TYPE_ARM64`
(`Mokiterions` 543,152 bytes, `mokiterions-tui` 1,063,056 bytes). **That establishes the
architecture, not that they run.** A binary can be the right architecture and still fail to launch.

## 3. `PROVENANCE.txt` against `RLS-MOK-001`

From the published Linux archive, verbatim:

```
Mokiterions 0.1.0

release record       RLS-MOK-001
release contract     REL-MOK-001
released work        WO-MOK-001 WO-MOK-002 WO-MOK-003 WO-MOK-004 WO-MOK-005 WO-MOK-006 WO-MOK-007 WO-MOK-009
verification records VREC-MOK-009
tag                  v0.1.0
commit               755db7297aa993f00d42f9c9794584b5d061f03d
target               x86_64-unknown-linux-gnu
toolchain            rustc 1.97.1 (8bab26f4f 2026-07-14)
built by             https://github.com/mmzen/Mokiterions/actions/runs/32294966039
```

Every field agrees with `RLS-MOK-001`: the version, the record, the contract it satisfies, the eight
work orders it releases, the verification record it includes, the tag, and the commit — **all forty
characters, unabbreviated**, which is the equality `REQ-MOK-037` asks for rather than a prefix match.
The toolchain is what the compiler reported, and `rust-toolchain.toml`'s pin is what it was asked for.

The three `PROVENANCE.txt` files differ from one another on **one line only** — `target`. That is the
whole of the difference between them.

Two notes that are observations rather than findings:

- **`REQ-MOK-037`'s "two builds differ only by the run identifier" was not tested**, because there was
  only one run. What was tested is the neighbouring property across targets.
- **The archives are not byte-reproducible**, and not only for the reason the contract gives. Each
  `tar` entry carries the runner's mtime (`2026-08-19 21:48`/`21:49` in the runner's local time), so
  re-running the same commit produces different archive bytes even with identical binaries.
  `REL-MOK-001` does not claim reproducible builds, so this is consistent with what was approved —
  but it means the `.sha256` files identify *these* archives, not *this commit's* archives.

## 4. The gate, re-run after the fact

Both checks were re-run after publication, against the real remote, with `master` already moved to
`5eed5a9…`:

```console
$ python scripts/check_release_authorization.py --root . --tag v0.1.0
AUTHORIZED release 0.1.0 of RLS-MOK-001
  candidate commit      755db7297aa993f00d42f9c9794584b5d061f03d
  release contract      REL-MOK-001
  released work         WO-MOK-001 WO-MOK-002 WO-MOK-003 WO-MOK-004 WO-MOK-005 WO-MOK-006 WO-MOK-007 WO-MOK-009
  included verification VREC-MOK-009
$ echo $?
0
$ python scripts/check_release_reachability.py --root . --commit 755db7297aa993f00d42f9c9794584b5d061f03d \
    --default-branch master --remote origin
REACHABLE commit 755db7297aa993f00d42f9c9794584b5d061f03d
  contained by         refs/remotes/origin/master
$ echo $?
0
```

The remote tag still resolves as it did: `refs/tags/v0.1.0` → annotated object
`b9a1eabc87c30126b29eb936051883a6a15f629b` → commit `755db7297aa993f00d42f9c9794584b5d061f03d`. The
gate authorizes the tag it refused before the tag existed, which is condition 1 finally being met
rather than anything changing about conditions 2 to 4.

## 5. Issues reported against the release

**None.** The repository has no issues, open or closed, as of this file being written.

## Known gap: the `release` environment protects nothing

`SPEC-MOK-005` rule 12.6 names a `release` environment and `.github/workflows/release.yml` declares
`environment: release` on the publish job. Before this release that environment did not exist in
repository settings. It exists now, and the run created it:

| Field | Value |
| --- | --- |
| Name | `release` |
| Created | `2026-08-19T19:49:25Z` — during run `32294966039`, by the publish job referencing it |
| `protection_rules` | `[]` |

An empty protection-rule list means the environment gates nothing. A required reviewer on it would
have been a second human gate **before any asset was uploaded**; there was none on this release, and
there will be none on the next one either unless it is configured. `docs/RELEASE_RUNBOOK.md` Phase I
says how, and it is one repository setting with no code change.

**What it did not do is weaken this release.** The workflow uploads a *draft* and publishes nothing;
publication was a separate human act after the checks in §2 and §3 were performed. The environment
would have gated the upload, not the publication, and the upload produced nothing reachable by a
recipient. The gap is that the second gate the specification contemplates is declared but inert — not
that this release went out unreviewed.

**Decided: recorded, not fixed.** The release owner decided on 2026-08-19 to record this gap and to
change no repository settings. This file is that record; no setting was touched, and the environment
is exactly as the run left it. The decision is the owner's to revisit before the next release.

## What this window did not cover

- **Neither Unix binary was started** (§2). The strongest statement available about the darwin
  archive is its Mach-O architecture.
- **Reproducibility across two runs of the same commit** was not tested, and the archives are known
  not to be byte-reproducible in any case (§3).
- **`WO-MOK-008` is still `draft`** and is not in this release. The observer's provenance footer
  therefore still cannot carry the commit at the narrowest declared viewport, which is why
  `PROVENANCE.txt` travels beside the binaries rather than inside them. `REQ-MOK-037` records this as
  intended, not deferred.
- **The window is calendar time, not attention.** Nothing observes this release continuously; the
  five items above were performed once, on the day of publication.
