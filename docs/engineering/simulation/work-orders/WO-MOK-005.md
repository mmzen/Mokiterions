+++
id = "WO-MOK-005"
type = "work_order"
title = "Split the workspace and implement the terminal observer"
status = "in_progress"
owners = ["engineering owner"]
created = "2026-08-17"
updated = "2026-08-17"

[assurance]
commit_bound_verification = "required"
rationale = "This work restructures the repository into two packages, takes the project's first external dependency at a measured surface of 57 crates, and promotes a read-only observation surface to a maintained public interface. It also introduces the instrument that later phases will use to assess behavior, so a defect in it would misinform every subsequent product judgement. The claim that observation cannot change a simulation outcome is a determinism claim of the same standing as REQ-MOK-009 and must be bound to a commit rather than asserted."
decided_by = "engineering owner"

[relations]
implements = [
  "REQ-MOK-019",
  "REQ-MOK-020",
  "REQ-MOK-021",
  "REQ-MOK-022",
  "REQ-MOK-023",
  "REQ-MOK-024",
  "REQ-MOK-025",
  "REQ-MOK-026",
  "REQ-MOK-027",
]
specifications = ["SPEC-MOK-003"]
architecture = ["ARCH-MOK-002", "ADR-MOK-003"]
verification = ["VER-MOK-005"]
+++

# Work Order: Split the workspace and implement the terminal observer

## Lifecycle

This work order remains a proposal while its status is `draft`. Transition to `approved` authorizes only the scope
below. Transition to `in_progress` records that implementation has begun. Transition to `implemented` requires the
completed change and retained evidence. Verification and release require separate commit-bound records.

Commit-bound verification is classified `required` above.

**This work order cannot be approved before its governing artifacts are.** It depends on `INT-MOK-004`, `CAP-MOK-004`,
and `REQ-MOK-019` through `REQ-MOK-027` being approved by the product owner; on `SPEC-MOK-003`, `ARCH-MOK-002`,
`ADR-MOK-003`, and the 2026-08-17 amendment to `ARCH-MOK-001` being approved by the technical owner; and on
`VER-MOK-005` being approved by the assurance owner. Every one of them was `draft` at the time this work order was
written. Preflight must report this exact work order as eligible before implementation begins.

**Approval record.** On 2026-08-17 the repository owner, acting in all four accountable roles, approved the complete
governing chain — `INT-MOK-004`, `CAP-MOK-004`, `REQ-MOK-019` through `REQ-MOK-027`, `SPEC-MOK-003`, `ARCH-MOK-002`,
`ADR-MOK-003`, the `ARCH-MOK-001` amendment, and `VER-MOK-005` — and authorized this work order, which was
transitioned `draft` → `approved` → `in_progress` on the same date. The implementation agent recorded the
transitions; it did not make the decision.

Two of those approvals are load-bearing rather than procedural. `REQ-MOK-026` is the approved requirement that
`ARCH-MOK-001`'s prohibition on separate crates has always required as its unlock, and without it a second package is
prohibited outright. The `ARCH-MOK-001` amendment is what scopes its prohibition on user-interface frameworks to the
engine package, and without it the dependency this work order adds is prohibited outright. Neither can be assumed and
neither may be self-approved by the implementation agent.

`ARCH-MOK-001` is deliberately **not** selected in `architecture` above, and its exclusion is not an oversight. The
applicability rule is that architecture is selected when active architecture directly `addresses` a requirement the
work order implements. `ARCH-MOK-001` addresses `REQ-MOK-004`, `REQ-MOK-008`, `REQ-MOK-009`, and `REQ-MOK-010`, none
of which are in this scope. Its amendment is a prerequisite of this work order's approval rather than work performed
under it, and its amended conformance checks appear below as constraints on this work. Nominal coverage is therefore
omitted rather than fabricated. The technical owner confirms this omission at approval.

## Objective

Restructure the repository into two packages and implement a terminal observer over a read-only engine surface,
exactly as specified by `SPEC-MOK-003`, governed by `ARCH-MOK-002` and `ADR-MOK-003`, and covered by `VER-MOK-005`,
without changing any simulation rule or any verified engine behavior.

## In scope

- Convert the repository to a Cargo workspace with exactly two packages: the engine package `mokiterions-core` at the
  root with its sources in their existing location, and the observer package `mokiterions-tui` as a member.
- Rename the engine package from `Mokiterions` to `mokiterions-core` and name its binary `mokiterions`.
- Add a library target to the engine package and expose the read-only observation surface of `SPEC-MOK-003`:
  `snapshot`, `advance_tick`, `is_finished`, `configuration`, and the snapshot types, all owning their data.
- Refactor the existing engine binary to drive the simulation through that same surface, so the command-line host and
  the observer are peer hosts of one interface rather than two paths through the engine.
- Add `ratatui` version `0.30.2` with `default-features = false` and features `crossterm`, `layout-cache`,
  `underline-color` as the observer package's only external dependency.
- Implement the observer: start-up input handling including `--speed`, `--start-paused` and `--export`; the spatial
  view in both zooms with the specified mapping, orientation, glyphs and shared-cell handling; the territory resource
  headline; the roster; the inspector; the event log with filtering and export; the provenance footer; the authority
  mapping; the tier-based layout with its floor, announcements and resize behavior; the key bindings; and
  unconditional terminal restoration on every exit path.
- Implement progression under rule 1: single-tick advance only, held at completed-tick boundaries, at most one tick
  per scheduling opportunity, and refusal to advance a finished run.
- Add automated tests covering every case, invariant and check in `VER-MOK-005`, including buffer assertions at every
  declared viewport and observed-versus-unobserved comparison on every declared seed.
- Record and retain the evidence `VER-MOK-005` lists under this work-order ID.

## Out of scope

- Any change to a simulation rule, constant, event type, event field, field order, exit code, text-stream line format,
  trace line, or summary. This work is additive to `SPEC-MOK-001` and changes nothing it fixes.
- Relocating the engine's sources. Their position is deliberately preserved so the `REQ-MOK-010` text stream is not
  disturbed.
- Any third package, service, network boundary, or separate release artifact.
- Fear, traits, names, combat, memory, model-provider integration, credentials, prompts, or per-agent entropy — every
  attribute the target design anticipates and the engine does not compute. `SPEC-MOK-003` reserves space for one of
  them and requires that space to render empty.
- Serialization, persistence, asynchronous runtimes, threads sharing simulation state, databases, and networking.
- Mouse input, configuration files, environment-variable configuration, and a standard-input protocol.
- Graphical or web interfaces.
- Reading the engine's text output back in. The observer obtains state from the surface, never by parsing.
- Structured or machine-readable simulation output, aggregate multi-run analysis, and outcome classification.
- Changes to approved behavior or artifact lifecycle outside this work order.

## Authorized decision envelope

The implementation agent may choose:

- private Rust type, function, module and file names in both packages, and how rendering is decomposed;
- how snapshots are built internally, provided the specified content, ordering and ownership hold;
- the concrete widget used for each pane, provided the specified content, constraints and announcements hold;
- exact diagnostic and pane-title wording, and the exact palette, provided every distinction remains available
  without colour;
- internal error types, and how terminal restoration is guaranteed on the panic path;
- test organization, fixtures and helpers, and whether layout tests construct a terminal or call layout directly;
- whether the reserved fourth roster bar is reserved by layout arithmetic or by a placeholder that renders nothing;
- comments and non-authoritative developer documentation.

The implementation agent may **not** choose: the dependency, its version, or its feature set; the package layout,
package names, binary names, or dependency direction; the coordinate mapping or orientation; the fidelity thresholds,
the tier table, or the floor; the glyph assignments; the key bindings; the event buffer capacity; the export format,
export path resolution, or filter semantics; the authority mapping; the snapshot contract or the number of mutating
operations on it; any figure fixed by `SPEC-MOK-001`; or any artifact lifecycle status.

Adding any external dependency beyond the one named above requires escalation and an updated architectural decision.
This includes a dependency that would appear only in the observer package, and it includes any dependency added to
make a test easier to write.

## Constraints

- Follow `INT-MOK-004`, `CAP-MOK-004`, `SPEC-MOK-003`, `ARCH-MOK-002`, `ADR-MOK-003`, and `ARCH-MOK-001` as amended.
- Preserve everything verified under `VREC-MOK-001` and `VREC-MOK-002`. Every existing test must pass unmodified. A
  test that must change to accommodate this work is a stop condition, not a task, because this change is specified as
  additive.
- Keep the engine package's external dependency set **empty**, with no exception, including a dependency shared with
  the observer package.
- Keep every user-interface dependency in the observer package's manifest and in no other.
- Add no dependency edge from the engine package to the observer package.
- Keep the engine package buildable and testable with no terminal present and with the observer package excluded.
- Use no network, model, credential, persistence, serialization, async-runtime, or database dependency in either
  package.
- Let the observer hold no mutable handle to world, agent, resource, or event-log state, and expose no operator
  control that mutates the world.
- Keep wall-clock time confined to deciding when to draw and when to advance. It must reach no engine input and no
  displayed authoritative value.
- Let the observer consume no simulation entropy.
- Re-derive no engine verdict in the observer. The displayed outcome is the engine's outcome or it is a defect.
- Present no value the engine does not compute, including an inert placeholder that reads as a computed zero.
- Read no repository file and invoke no version-control command at run time.
- Never open `--export` at start-up; validate it as a string only.
- Preserve unrelated user changes and do not modify generated output as source.

## Expected change surface

- `Cargo.toml` at the root: a workspace table, the package rename, and a library target alongside the existing binary.
- `Cargo.lock`: from an empty dependency set to a workspace including the observer's 57-crate surface.
- The engine's existing sources: a library entry point and the public observation surface, plus the refactor of the
  existing binary to drive the simulation through that surface. No simulation rule changes.
- A new `mokiterions-tui/` directory with its manifest, sources and tests.
- Automated tests in both packages.
- Work-order-keyed evidence under `docs/engineering/simulation/evidence/WO-MOK-005/`.
- `docs/mokiterions/ROADMAP.md`, to record the observer phase.
- No other product domain and no harness-managed policy file.

## Required verification

- Complete every case, invariant, check and manual assessment in `VER-MOK-005`.
- Run `cargo fmt --all -- --check`.
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Run `cargo test` at the workspace root, and `cargo test -p mokiterions-core` alone.
- Run `cargo build` at the workspace root, and `cargo build -p mokiterions-core` alone.
- Run `cargo tree -p mokiterions-core` and retain the output as the empty-dependency-set proof.
- Retain the observer package's resolved dependency graph, its crate count, and its enabled feature set.
- For each declared seed `0`, `1`, `42`, `123`, `777`: run the engine binary to completion, run the same
  configuration through the observer under the scripted interaction sequence, export, and compare byte for byte.
- Compare per-tick entropy draw counts observed and unobserved for each declared seed.
- Assert the character buffer at each declared viewport `160 × 48`, `160 × 44`, `140 × 44`, `120 × 48`, `100 × 30`,
  `34 × 22`, and capture the refusal output at `33 × 21`.
- Run one `10,000`-tick observed run at speed `64`.
- Demonstrate terminal restoration on normal exit, on error exit, and on panic.
- Perform and record the manual assessments, including the legibility and colour-independence assessments.

## Evidence to record

Retain, under `docs/engineering/simulation/evidence/WO-MOK-005/`, every item listed in `VER-MOK-005`'s evidence
retention section. The `cargo tree -p mokiterions-core` output and the per-seed observed-versus-unobserved comparison
are the two records that carry the load: the first is the only proof that the engine's dependency set survived this
change, and the second is the only proof that observation did not perturb a run. No screenshot or recording is
admissible for any rendering obligation; retain buffer dumps accompanying their assertions instead.

## Stop and escalate conditions

Stop before implementation if preflight does not report this exact work order as eligible, or if any governing
artifact remains `draft` or unapproved — in particular `REQ-MOK-026` and the `ARCH-MOK-001` amendment, without which
a second package and a user-interface dependency are both prohibited.

During implementation, stop and escalate if:

- an observed run and an unobserved run differ in any authoritative event or in final state on any declared seed. The
  non-perturbation property is the point of the design, so a difference means the design or the implementation is
  wrong and must be corrected rather than tolerated or documented;
- per-tick entropy draw counts differ observed and unobserved;
- the specified layout arithmetic does not close at any declared viewport, or a canvas interior does not match the
  figure `SPEC-MOK-003` derives. The specification is the authority, so a mismatch requires an amended specification
  and re-approval, never a quietly adjusted constraint or a relaxed assertion;
- the whole world cannot be presented at one dot per world cell at a viewport where `SPEC-MOK-003` says it can;
- the observation surface cannot be exposed without leaking a reference into engine state, an interior-mutable value,
  or a second mutating operation;
- the engine package cannot be kept free of external dependencies, or the observer's dependency cannot be confined to
  its own package;
- the resolved observer dependency graph does not match the specified version and feature set, or exceeds the
  measured 57 crates;
- any dependency beyond the one specified appears necessary, for any reason including test ergonomics;
- rendering cannot be asserted from an in-memory buffer, since verification then has no admissible evidence and the
  whole approach to covering presentation fails;
- an existing `VER-MOK-001` or `VER-MOK-002` test fails, or would need to be modified;
- terminal restoration cannot be guaranteed on the panic path;
- a value the target design shows cannot be rendered because the engine does not compute it. Such a value is out of
  scope above and must be left absent; inventing a placeholder for it is forbidden, and if it is judged necessary
  that is a product decision;
- specified behavior is contradictory or needs a product or technical decision;
- required verification fails and cannot be corrected within authorized scope;
- requested changes expand into an excluded feature or another artifact domain.

## Completion report format

Report:

1. implemented requirements and affected components;
2. any authorized local decisions, including how rendering was decomposed and how panic-path restoration is
   guaranteed;
3. verification commands and results;
4. the `cargo tree -p mokiterions-core` output, and the observer's crate count and enabled feature set as resolved;
5. the per-seed observed-versus-unobserved comparison result and the interaction sequence performed;
6. the per-viewport canvas interiors as measured, against the figures `SPEC-MOK-003` derives;
7. retained evidence paths;
8. residual limitations and explicitly deferred features, including every attribute of the target design left absent;
9. final worktree and candidate-commit status.
