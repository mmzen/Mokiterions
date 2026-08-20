# WO-MOK-013 — the review gate, and the declared requirement set it corrected

**Work order** `WO-MOK-013` · **Branch** `governance/adr-mok-006-third-party-crates` · **Parent commit**
`f40b711b0985316494c6d3ac24f9b0434b0f0ad7` · **Pull request** #33 (draft) · **Date** 2026-08-20

This file records one governed act and the finding that forced it: `WO-MOK-013`'s `implements` relation narrowing from
three requirement identifiers to one. The hash of the commit that carries the act is not in this file — `WORKFLOW.md`
states that a record cannot contain the hash of its own commit, and this file travels in that commit.

## What the gate found

Opening pull request #33 ran the managed harness's `review` phase against this work order for the first time. It
failed, with one diagnostic:

    Harness preflight: FAIL
    Phase: review
    Work order: WO-MOK-013 (implemented)

    Diagnostics:
    - [W016] docs/engineering/simulation/work-orders/WO-MOK-013.md: verification coverage is missing REQ-MOK-026,
      REQ-MOK-036

    exit 1

| Where | Result |
|---|---|
| `Engineering Harness`, run `32371118119`, `pull_request`, job `candidate`, step *Review preflight* | **failure** in 7s, exit 1 |
| the same run, job `governor` | success |
| `Dependency declarations`, run `32371118176`, job `Declared sets` — this workflow's **first `pull_request` run** | success in 22s |
| `Engineering Harness`, four `push` runs `32367607331`, `32368286317`, `32370885081`, `32371014079` | success |

**Why four green pushes did not catch it.** Both the work-order selection step and the review preflight are guarded by
`if: github.event_name == 'pull_request'` in `.github/workflows/engineering-harness.yml`, and the selection step reads
the `Harness-Work-Order:` trailer from the stored event payload. No `push` run of this branch could reach the review
phase, whatever the graph contained.

## The rule, and why the finding is true

The diagnostic is not a harness quirk: it is this repository's own traceability rule, checked by managed code.

- `docs/engineering/TRACEABILITY.md` — *"Every active requirement selected for implementation needs selected active
  verification coverage at G1"*, and *"every implemented requirement needs selected specification and verification
  coverage"*.
- The check, in the declared candidate runtime `se-harness==0.4.0`, is `se_harness/preflight.py:506`: for each
  requirement in the work order's `implements`, the union of `verifies` over the work order's declared `verification`
  contracts must cover it. The same union is checked for `specifies` over the declared specifications, which passed.

The facts it read:

| | |
|---|---|
| `WO-MOK-013.implements` before the correction | `REQ-MOK-026`, `REQ-MOK-036`, `REQ-MOK-047` |
| `WO-MOK-013.verification` | `VER-MOK-013` |
| `VER-MOK-013.verifies` | `REQ-MOK-047` — and nothing else |
| what verifies `REQ-MOK-026` | `VER-MOK-005`, whose `verifies` is `REQ-MOK-019` through `REQ-MOK-027` |
| what verifies `REQ-MOK-036` | `VER-MOK-008`, whose `verifies` is `REQ-MOK-035` through `REQ-MOK-039` |

`VER-MOK-005` and `VER-MOK-008` are the two contracts the repository owner decided on 2026-08-20 to leave for a
separate change, so the gate was reporting the deferral from the one angle that had not yet been checked.

**Reproduced offline, under the pinned runtime, before anything was edited:**

    $ <pinned 0.4.0 venv>/python -m se_harness preflight . --work-order WO-MOK-013 --phase review
    Harness preflight: FAIL
    ...
    - [W016] docs/engineering/simulation/work-orders/WO-MOK-013.md: verification coverage is missing REQ-MOK-026,
      REQ-MOK-036
    exit 1

The message is byte-identical to CI's. `0.4.0` is the version the workflow installs by exact pin
(`se-harness==0.4.0`); the machine-wide `0.4.1` install is not used for any figure in this packet, for the reason
`WO-MOK-013-harness.txt` records.

## Three routes, measured before any was put to the owner

Each was applied to a scratch edit of the front matter, measured, and reverted. The repository was clean at
`f40b711b0985316494c6d3ac24f9b0434b0f0ad7` before and after each.

| Route | Preflight `--phase review` | Validator | Dashboard | Inspector |
|---|---|---|---|---|
| unchanged | **FAIL**, `W016`, exit 1 | PASS — 107 artifacts, 0/0 | 107 artifacts, **359** relations, 7 warnings | 20 findings, 0 error / 7 warning / 13 info |
| **A** — declare `VER-MOK-005` and `VER-MOK-008` in `verification` | **PASS**, exit 0 | PASS — 107 artifacts, 0/0 | 107 artifacts, **361** relations, 7 warnings | identical |
| **B** — narrow `implements` to `REQ-MOK-047` | **PASS**, exit 0 | PASS — 107 artifacts, 0/0 | 107 artifacts, **357** relations, 7 warnings | identical |

Neither route adds an error, a warning or a finding, and no snapshot digest is quoted for any of the three scratch
trees: a digest of a tree no commit holds is the defect `VREC-MOK-013` documents.

**A fourth route was named and excluded: leaving the gate red.** Reassessing `VER-MOK-005` and `VER-MOK-008` — the
deferred change — rewrites those contracts' text and does not add them to this work order's declared set, so it would
not clear `W016`. Waiting for it would have left a red required check on this pull request indefinitely.

## Why route A was refused, and a correction to the agent's own framing

Route A was put to the owner first, with the claim that it would leave `VREC-MOK-013`'s claims untouched. **That claim
was wrong, and it was corrected before any file was edited.**

`validate_engineering_artifacts.py` makes a record's *missing* contracts an error only when the record binds more than
one work order, which is what the first framing rested on. But `docs/engineering/templates/VERIFICATION_RECORD.template.md`
states the governing rule: *"The verification-contract set must equal the union declared by those work orders."*
`TRACEABILITY.md` says the same of a `verification_record`. The validator's narrower check is a gap in an automated
check, not a permission, and **all eleven records this repository already holds satisfy the equality** — checked one by
one, including `VREC-MOK-009`, which binds eight work orders and conforms to exactly the eight contracts they declare.

So route A, followed properly, would have required `VREC-MOK-013` to declare `conforms_to = ["VER-MOK-005",
"VER-MOK-008", "VER-MOK-013"]`. That is not extra assessment work against evidence this packet holds:

- **`VER-MOK-008` is the release contract.** Its scenarios are authorizing, refusal, compliance, provenance,
  persistence and toolchain checks on an actual release run — a published archive read by an independent reader, a
  checksum verified with a standard tool, two builds of one commit differing only by run identity. No release is
  authorized, and none happened.
- **`VER-MOK-005` requires manual legibility and colour-independence assessments in a real terminal with a real font**,
  and states at `VER-MOK-005:318` that this gap exists precisely because it cannot be automated.

A record claiming conformance to either would assert evidence that does not exist. Route A was therefore withdrawn.

## The act

`implements` moved from `["REQ-MOK-026", "REQ-MOK-036", "REQ-MOK-047"]` to `["REQ-MOK-047"]` on 2026-08-20 by the
repository owner acting as accountable **engineering owner**, answering the corrected framing with **"B — narrow
implements"** and, for the record bound to the tree this changes, **"Re-point to the new candidate"**. Editing an
approved and implemented work order's declared scope is that role's act; the implementation agent measured the routes
and wrote the text, and decided neither.

**The reading it rests on.** What this change did to `REQ-MOK-026` and `REQ-MOK-036` was strike a clause from each
under `ADR-MOK-006`'s product-owner authority. It did not implement their obligations. The obligation it implements is
`REQ-MOK-047`'s, and both amendments stand exactly as approved, each in its own artifact's amendment row and each still
among the twelve amendment preconditions `WO-MOK-013` lists.

| File | Edit |
|---|---|
| `work-orders/WO-MOK-013.md` front matter | `implements` narrowed to `["REQ-MOK-047"]` |
| `work-orders/WO-MOK-013.md` *Lifecycle* | The `architecture` relation's basis re-derived, and a new subsection *The declared requirement set, corrected* |
| `evidence/WO-MOK-013/WO-MOK-013-review-gate.md` | This file |

No specification, requirement, architecture document, ADR or verification contract changes. No Rust source, manifest,
lockfile, workflow or script changes.

**The `architecture` relation survives the narrowing, and it was checked rather than assumed.** `ARCH-MOK-001` and
`ARCH-MOK-002` each `addresses` `REQ-MOK-047` directly, and each shares a conforming specification with the work
order's declared set — `ARCH-MOK-001` through `SPEC-MOK-002`, `ARCH-MOK-002` through `SPEC-MOK-003` and `SPEC-MOK-004`.
The expectation going in was that dropping `REQ-MOK-026` would break `ARCH-MOK-002`'s selection and raise `W021`; it
does not, and the measured preflight PASS above is the check on that.

## The cost, stated rather than mitigated

`TRACEABILITY.md` holds that *"Only declared relations in formal TOML metadata establish authority"* and that source
comments, filenames, commits and conversational references *"may aid discovery but do not satisfy formal coverage"*.

**After this correction, no declared relation runs from either amended requirement to the work order that rewrote its
text.** This artifact model has no `amends` relation to carry one — the relation table in
`scripts/validate_engineering_artifacts.py` admits no such pair — so what remains is the phrase *"Written under
`WO-MOK-013`"* in each requirement's amendment row, which by that same rule is discovery and not coverage.

**The reviewer's reading set shrinks by four artifacts.** Preflight's reading manifest is derived from the declared
chain, so it went from 23 entries to 19: `REQ-MOK-026`, `REQ-MOK-036`, and — reached only through those two
requirements' `derives_from` edges — `INT-MOK-007` and `CAP-MOK-007`. A reviewer sent to this work order is no longer
sent to the two requirements whose text it edits. That is a real loss and it is not compensated here; it is the price
of the route, and the alternative was a record asserting evidence that does not exist.

**What is not lost.** `W-HEX-003` still names `ADR-MOK-002`, `ADR-MOK-004`, `ARCH-MOK-001`, `ARCH-MOK-002`,
`REQ-MOK-026`, `REQ-MOK-036`, `SPEC-MOK-003`, `VER-MOK-005` and two more — five observations, unchanged across all
three routes above, measured and not assumed. The deferred debt on `VER-MOK-005` and `VER-MOK-008` stays visible in the
graph exactly where it was.

## The harness and the gates at this tree

Measured from the repository root after every edit above, including this file:

    $ cargo fmt --all -- --check                                          exit 0
    $ cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
                                                                          exit 0, no allow added
    $ cargo test --workspace --locked                                     exit 0 — 21 result lines,
                                                                          212 passed, 0 failed, 0 ignored, 0 filtered
    $ python -m unittest discover -s scripts -p "test_*.py"               126 tests, OK
    $ python scripts/check_declared_dependencies.py --root .              exit 0
      Every declared set matches its resolved graph. 8.4a-8.4d pass.
      (both disclosed transitive matches still printed: mio 1.2.2, signal-hook-mio 0.2.5)

    $ python scripts/validate_engineering_artifacts.py --root .
    PASS | Artifacts: 107 | Errors: 0 | Warnings: 0 | exit 0
    Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0

    $ python scripts/generate_harness_dashboard.py --root .
    PASS | Artifacts: 107 | Relations: 357 | Errors: 0 | Warnings: 7 | exit 0

    $ python scripts/inspect_engineering_artifacts.py --root .
    Graph: 107 artifacts | 357 relations | 20 findings
    Finding severity: error 0 | warning 7 | info 13 | exit 0
    Decision required (1): VREC-MOK-013 [ready] assurance-review
    Assurance pending (0): none

    $ <pinned 0.4.0 venv>/python -m se_harness doctor .
    exit 0 | 81 PASS | 0 FAIL | 0 WARN

    $ <pinned 0.4.0 venv>/python -m se_harness preflight . --work-order WO-MOK-013 --phase review
    Harness preflight: PASS | Phase: review | Work order: WO-MOK-013 (implemented) | exit 0

`pytest` is not installed in this environment, so the Python suite runs through `unittest discover`, as
`WO-MOK-013-gates.txt` records. `Cargo.lock` is byte-identical to `origin/master`, and `git status --porcelain` is
empty after every check.

**Two relations fewer, and that is the whole graph movement.** 359 to 357 is `implements` losing two targets. Artifact
count, warning count, finding count and every severity are unchanged. **No snapshot digest is quoted for this tree**:
the digest for the commit this file travels in is measured after the commit exists and is recorded in the re-pointed
`VREC-MOK-013`.

## Statements this correction makes stale

Enumerated rather than rewritten, on the `VREC-MOK-005:433` precedent that a later change lists what it puts out of
date.

| Where | What it says | Standing now |
|---|---|---|
| `WO-MOK-013-harness.txt:114` | The preflight reading manifest, listing `REQ-MOK-026` and `REQ-MOK-036` among the artifacts the chain reaches | Still true of `84b21b9`, the tree it measured. The manifest at this tree holds 19 entries, and the four that left are named above |
| `WO-MOK-013-transition.md:87` and its snapshot table | *"twenty paths where there were nineteen"* | Still true of `d33ecb8`. This file is the twenty-first, and it is named `WO-MOK-013-*` like the other twenty |
| `WO-MOK-013-completion-summary.md:54` | *"Evidence — 19 files"* | Stale by two, and already recorded as stale by one in `WO-MOK-013-transition.md` |
| `VREC-MOK-013`'s `commit`, `artifact_snapshot_sha256`, `verified_at`, `evidence_paths` and its gate figures | Bound to `d33ecb8` | Superseded in the commit that follows this one, on the owner's instruction to re-point the record. The figures for `d33ecb8` remain recorded there as figures of that commit |
| Pull request #33's body | *"twenty retained evidence files"*, and the record bound to `d33ecb8` | Stale in those two readings. The body is not a governed artifact and is not edited here; the discrepancy is reported to the owner rather than corrected without instruction |

## What this correction does not do

- **It does not withdraw, weaken or reword either amendment.** `REQ-MOK-026` and `REQ-MOK-036` keep the text the
  product owner approved on 2026-08-20, and their amendment rows are untouched.
- **It does not clear `W-HEX-003`**, and it does not touch `VER-MOK-005` or `VER-MOK-008`, which stay owed on the
  owner's decision to leave them for a separate change.
- **It is not a verification, an assurance decision, a transition of `VREC-MOK-013`, a tag or a release.** It moves one
  relation in one work order and adds this file.
- **It does not take the pull request out of draft.** That remains the owner's act.
- **It changes no measured code figure**, and the gate re-run above is the check on that claim rather than the
  assertion of it.

Every command behind every figure in this file is offline, reads no credential, secret, token or environment value, and
none appears in this file or in any other retained evidence for this work order.
