# The approval and classification decisions of 2026-08-20 on `WO-MOK-012`

`WO-MOK-012` moved from `draft` to `approved` on 2026-08-20, and its `commit_bound_verification` classification was
confirmed `not_required`, both by the repository owner acting as **engineering owner** — the role `DECISION_RIGHTS.md`
names for a bounded work order, and the role the classification's own `decided_by` field names.

This note takes no decision. It records two, and states what they did not cover.

It follows the form of `evidence/WO-MOK-011/assurance-decision.md` and `evidence/WO-MOK-010/assurance-decision.md`.

## The instruction

Verbatim and complete, in the turn it was given:

> i approve WO-MOK-012, i confirm commit_bound_verification = "not_required"

## What it named, and what it did not

**Two acts on one artifact.** The approval, and the classification confirmation — stated separately, which is why they
are recorded as two decisions rather than one. The second is not read as following from the first: the work order's
`rationale` field carried an explicit demand that the engineering owner confirm the classification at approval, and the
owner answered it in its own clause rather than leaving it to be inferred.

**It named no other act.** Specifically, it did not authorize:

| Not authorized | Consequence |
|---|---|
| A transition to `in_progress` or `implemented` | The work order stands at `approved`. Its substance is complete and committed, so the `implemented` gate is met on the facts, but the transition is a separate act and was not taken |
| A push, pull request, tag or release | Nothing left the clone. The work is committed on `assessment/wo-mok-005-remediation` and stops there |
| Anything touching `VREC-MOK-005` | It stays `verified` at commit `f361370`, unedited. Whether the chain needs a re-captured record is still undecided |
| Any identifier claim or work for the second chain | Decisions 13 to 15 settle its design and nothing more. It re-sweeps its own identifiers |
| Manual assessment 7 | Still outstanding by the assurance owner's decision of the same date. Approving the work order that records it as outstanding is not closing it |

The precedent for reading an instruction this narrowly is in this repository and is explicit.
`evidence/WO-MOK-011/assurance-decision.md` records an instruction that named `VREC-MOK-011` and three acts, and notes
that because it did not name `WO-MOK-011` the work order was not moved — drawing the contrast with the `VREC-MOK-010`
instruction of 2026-08-19, which carried the parenthetical *"(implying transitioning WO-MOK-10)"* where this one carried
no equivalent. **Nothing is approved by implication**, and the reason is structural rather than cautious: one person
holds product, technical, assurance, engineering and release authority here, so an act in one role is not an act in
another and only what was said was decided.

## What decision 16 approved

**The scope in `WO-MOK-012`'s *In scope* section, and nothing wider.** That scope is nine items of governance recording:
the eleven ratifications, the six authored assessments, the recorded disposition of the seventh, the three `VER-MOK-005`
restatements, the corrected cross-references, the three adverse observations, the three procedure defects, the evidence
directory, and the roadmap entry.

**It is not an approval of the three adverse observations' remedies.** Decisions 13 to 15 are recorded here as the
settled design of a later chain, and the work order's *Out of scope* section places every fix outside it. Approving a
work order that records a design is not approving the change that implements it, which will need its own chain,
requirements, specification amendments and approval.

## What decision 17 confirmed

**That no commit-bound verification record is required for this work.** `WORKFLOW.md` allows `not_required` "only for
work whose sole purpose is to record or transport an already authorized governance decision", and requires that mixed or
uncertain scope be classified `required` or escalated rather than inferred by automation.

The measured basis put to the owner, re-confirmed at the commit this note accompanies:

- `git diff --stat origin/master` over `mokiterions-core/`, `mokiterions-tui/`, `Cargo.toml`, `Cargo.lock` and
  `rust-toolchain.toml` is **empty**. No executable behavior, no test, no manifest, no lockfile, no toolchain pin.
- No managed harness file and no CI definition changes.
- The workspace's **212 tests** across 21 binaries, `cargo fmt --all -- --check`, and
  `cargo clippy --workspace --all-targets --all-features -- -D warnings` are unmoved and clean.
- `scripts/validate_engineering_artifacts.py` reports **PASS, 103 artifacts, 0 errors, 0 warnings** across all four
  planes.

The one change reaching an approved assurance artifact — the `VER-MOK-005` amendment — is itself one of the recorded
decisions of the 2026-08-20 review, taken by the owner as assurance owner. That is what keeps the scope from being
mixed, and it was stated to the owner as the load-bearing part of the argument.

**The consequence, stated plainly.** Being governance-only, this work order **stops at `implemented`** and takes no
verification record, as `WORKFLOW.md` requires: "Governance-only work that authorizes verification, release, tagging,
review, or publication stops at `implemented` unless a distinct later VREC selects it." It does not become `verified` by
recording assessments that `VER-MOK-005` requires.

## What neither decision retired

- **Manual assessment 7 of `VER-MOK-005`.** Outstanding by deliberate decision. `VREC-MOK-005` continues to disclose it
  and any release record covering this chain inherits the disclosure. It is not closed by the approval of the work order
  that records it as open.
- **The three adverse observations.** Recorded, measured, designed for — and unfixed. The observer at `ff3a155` still
  advertises the `?` key nowhere, still draws two-column survival gauges, and still announces excluded panes without
  emphasis and without naming the enlargement remedy.
- **`VREC-MOK-005`'s staleness.** Ten of its eleven disclosed provisions and six of its seven disclosed assessments no
  longer describe the tree. The record is unedited and correct at its commit; a reader taking it as current will be
  wrong about them.
- **The sequencing deviation.** The governance text was committed at `aebb26f` while this work order read `draft`, and
  approval followed in the next turn. `WO-MOK-012`'s *Lifecycle* section records this rather than smoothing it over.
  Every decision the work order records was the owner's own act taken before any file was edited, so no decision was
  taken without authority — what ran ahead of its authorization was the work order's own approval. **The approval of
  2026-08-20 does not retroactively make the order of acts what `WORKFLOW.md` describes**, and this note does not read
  it as doing so.

## What the agent did

Wrote `approved` into the `status` field, updated the `rationale` field to record the confirmation, added decisions 16
and 17 to the decision record, and authored this note. It decided neither act and transitioned nothing else.
