# WO-MOK-003 evidence: governance amendments and their approval

`SPEC-MOK-002` could not be conformed to under `ARCH-MOK-001` and `ADR-MOK-001` as they stood: both
stated the program as "one binary crate", and a Rust integration test cannot link a binary target.
`ADR-MOK-002` tabulated the amendments that removes that obstruction, and `WO-MOK-003` made all of them
**approval preconditions** — governance acts that had to be complete and approved before any line of
code changed.

`VER-MOK-003` requires confirmation that they were approved before the change, with dates. This is that
confirmation.

## Authority

`DECISION_RIGHTS.md` assigns specification and architecture to the technical owner and prohibits an
implementation agent from self-approving an architecture decision assessment or its supporting ADR. The
repository owner holds every accountable role in this repository — product, technical, engineering, and
assurance — and cleared the gate explicitly on **2026-08-17**, directing that the amendments be applied
exactly as `ADR-MOK-002` tabulated them.

The implementation agent that drafted the packet approved nothing. It surfaced the gate, named who owned
it, and stopped. That sequence is the point of the precondition and it is recorded in `WO-MOK-003`'s
*Approval preconditions* section as well as here.

## The four preconditions

| # | Precondition | Artifact | Date | Status |
| ---: | --- | --- | --- | --- |
| 1 | `ARCH-MOK-001` amended exactly as tabulated, including the relation additions | `ARCH-MOK-001` | 2026-08-17 | Satisfied |
| 2 | `ADR-MOK-001` narrowed on its structural point | `ADR-MOK-001` | 2026-08-17 | Satisfied |
| 3 | `SPEC-MOK-001`'s delegation of test organization narrowed | `SPEC-MOK-001` | 2026-08-17 | Satisfied |
| 4 | The seven packet artifacts moved to `approved` | see below | 2026-08-17 | Satisfied |

### 1. `ARCH-MOK-001` — ten amendments, all applied

`ADR-MOK-002`'s *Required amendments* table for `ARCH-MOK-001` has ten rows. All ten are applied:

| Location | What changed |
| --- | --- |
| *Components*, closing paragraph | "one binary crate" → "one Cargo package, built as a library target and a thin binary target" |
| *Components* | new paragraph naming the thin binary target and the rule-5-enumerated library interface |
| *Prohibited patterns* | "Separate crates or services…" → "Separate Cargo packages, workspaces, or services… The library and binary targets of the single package are not separate crates in this sense." |
| *Prohibited patterns* | new entry prohibiting public items that expose mutable or owned authoritative state, and any feature flag, `cfg` attribute, or test-support seam that exposes it |
| *Quality attributes*, Simplicity | "one binary crate and the minimum modules needed…" → "one Cargo package with one library target and one thin binary target, and the minimum modules needed…" |
| *Quality attributes*, Testability | added "and the program's public contract can be tested from outside the implementation source files." |
| *Conformance checks* | "Confirm the program builds as one Rust binary crate." → "…one Cargo package with exactly one library target and one binary target, with an empty dependency table." |
| *Conformance checks* | new check on the library target's public interface matching `SPEC-MOK-002` rule 5 exactly, and no public item yielding mutable or owned authoritative state |
| `[relations] addresses` | `REQ-MOK-016` added |
| `[relations] conforms_to` | `SPEC-MOK-002` added |

Plus the two the table required of the record itself: `[decision_assessment] rationale` records that the
decision is covered by `ADR-MOK-001` and `ADR-MOK-002` together, and *Related ADRs* gained
`ADR-MOK-002`. `decision_assessment.outcome` stays `adr_required` and its trigger list is unchanged, as
`ADR-MOK-002` specified.

The architecture's amendment record carries the dated row:

> 2026-08-17 — One binary crate narrowed to one Cargo package built as a library target and a thin
> binary target; prohibited-pattern, quality-attribute, and conformance-check wording updated;
> `REQ-MOK-016` added to `addresses` and `SPEC-MOK-002` to `conforms_to`. Decided by `ADR-MOK-002`;
> engine authority unchanged. — *repository owner, on the technical owner's behalf*

The `conforms_to` addition was not cosmetic. The validator enforces that an addressed requirement be
specified by a conforming specification, so adding `REQ-MOK-016` to `addresses` without adding
`SPEC-MOK-002` to `conforms_to` would have failed validation.

### 2. `ADR-MOK-001` — amended in place, not superseded

`ADR-MOK-002` left the choice between in-place amendment and supersession to the technical owner, and
recommended in place. The owner chose in place. `ADR-MOK-001`'s decision bullet now reads "one Cargo
package, built as a library target and a thin binary target", carries a dated note pointing at
`ADR-MOK-002`, and its *Status* records:

> Accepted. Amended in place 2026-08-17 on one structural point… The substantive decision — option 3,
> engine authority, and the immutable observation and typed proposal boundary — is unchanged, so this ADR
> is not superseded.

That reasoning is the operative one: `ADR-MOK-001`'s own migration clause reserves supersession for
replacing engine authority, and engine authority is untouched. What narrowed is the structural
expression of the decision, and the narrowing makes the prohibition on exposing mutable state apply to a
surface that is now actually reachable from outside the crate — a strengthening, not a relaxation.

### 3. `SPEC-MOK-001` — one row, no behavior change

The *Explicitly unspecified decisions* entry on test organization now reads that test helper functions
and the internal organization of a test module within its owning source file remain delegated, while
crate target layout, the public interface, and which tier a test belongs to are governed by
`SPEC-MOK-002`. The specification's amendment record carries the dated row:

> 2026-08-17 — Narrowed the *Explicitly unspecified decisions* entry on test organization… No specified
> behavior changed. — *Approved 2026-08-17 by the repository owner acting as technical owner, as required
> by `ADR-MOK-002` and as an approval precondition of `WO-MOK-003`.*

No behavioral rule of `SPEC-MOK-001` was touched, which matters because rule 11 of `SPEC-MOK-002` binds
this work order to preserving `SPEC-MOK-001`'s behavior byte-for-byte. Amending the behavior contract in
the same act would have destroyed the standard the work was measured against.

### 4. The packet artifacts

Seven artifacts moved from `draft` to `approved` on 2026-08-17: `INT-MOK-003`, `CAP-MOK-003`,
`REQ-MOK-016`, `REQ-MOK-017`, `SPEC-MOK-002`, `ADR-MOK-002`, and `VER-MOK-003`. `WO-MOK-003` itself moved
to `in_progress`, with its *Lifecycle* paragraph recording approval and the transition, attributed to the
repository owner acting as technical, engineering, and assurance owner.

## Ordering

The order matters as much as the fact, so it is stated plainly:

1. The packet was drafted and the gate was surfaced. No code was written.
2. The repository owner cleared the gate and directed the amendments.
3. The amendments were applied to `ARCH-MOK-001`, `ADR-MOK-001`, and `SPEC-MOK-001`; the seven artifacts
   were approved; `WO-MOK-003` moved to `in_progress`.
4. `python scripts/validate_engineering_artifacts.py` reported **PASS — 36 artifacts, 0 errors, 0
   warnings** across all four planes.
5. The pre-change baseline was captured from the unmodified tree at commit
   `77010d02319051a20f8e45282f9c813ce4199956`.
6. Only then did `Cargo.toml`, `src/`, and `tests/` change.

Step 5 sitting after step 3 and before step 6 is deliberate. The amendments are documentation; they
cannot alter program output, so capturing the baseline after them costs nothing and keeps the capture as
close as possible to the code change it is the oracle for.

## Conformance of the result to the amended architecture

The amendments added two conformance checks. Both are discharged:

| Amended check | Evidence | Result |
| --- | --- | --- |
| One Cargo package, exactly one library target and one binary target, empty dependency table | `Cargo.toml`; `completion-summary.md`; `compile-time.md` | one `[lib]`, one `[[bin]]`, no workspace, no build script, `[dependencies]` empty, no `[dev-dependencies]`, `Cargo.lock` unchanged |
| The library target's public interface matches `SPEC-MOK-002` rule 5 exactly, and no public item yields mutable or owned authoritative state | `public-surface-inventory.md`, `boundary-review.md` | 13 public items, all inside rule 5's union; two authorized accessors deliberately omitted; no public signature returns a reference, a mutable borrow, an iterator, or a collection |
