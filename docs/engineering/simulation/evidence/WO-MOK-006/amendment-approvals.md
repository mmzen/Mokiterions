# Amendment records: what was approved, what was written, and who did which

`WO-MOK-006` selects four artifacts for amendment and states the rule this file exists to satisfy:

> Writing the amended text is implementation; approving it is not, and each amendment record must say
> which of the two happened.

`DECISION_RIGHTS.md` line 3 is why: "AI agents may draft, challenge, decompose, implement, and verify
within an approved work order. They do not inherit accountability for product intent, architecture
risk acceptance, security exceptions, release authorization, or production operation." An amendment to
an approved specification is architecture risk acceptance. So each of the four rows below records the
same division: **the owner approved the requirement that the artifact be amended and the substance the
amendment must carry; the implementation agent wrote the words.**

## The authorization, and the reading it rests on

On 2026-08-18 the repository owner, acting in all four accountable roles, approved the complete
governing chain — `INT-MOK-005`, `CAP-MOK-005`, `REQ-MOK-028`, `REQ-MOK-029`, `REQ-MOK-030`,
`SPEC-MOK-004`, `ADR-MOK-004`, `VER-MOK-006` — and authorized `WO-MOK-006`. The owner's instruction on
recording was explicit: *"You approve, I record it."* Nine lifecycle transitions were recorded by the
implementation agent on that instruction; `WO-MOK-006`'s *Approval record* section states that the
agent recorded them and did not make the decision, and each amendment row below repeats it.

The four amendments were not approved by a separate act. They are approved **by way of
`ADR-MOK-004`**, whose *Required amendments* section states each of the four in full — which artifact,
which sections, and what each must say — and which the owner approved with the rest of the chain.

**This is a reading, and it is disclosed rather than assumed.** It is the reading `ADR-MOK-004` itself
sets up: the ADR enumerates the amendments as its own consequences, and `WO-MOK-006` makes them
approval preconditions, which is the same construction `WO-MOK-003` and `WO-MOK-005` used for their
chains. But an alternative reading exists, in which approving an ADR that *requires* amendments is not
the same act as approving the amended text. Under that reading the four rows below would remain
outstanding until the owner approves them individually. The distinction is not academic: two of the
four artifacts are bound by a `verified` verification record.

The completion report carries this as a disclosure for the assurance owner to weigh. Nothing in this
packet depends on the reading being the right one — the measurements stand either way, and if the
owner takes the narrower reading, the effect is that four Approval columns change and no evidence does.

## What was not done, and could not be

**No record bound to a commit was edited.** `VREC-MOK-001` through `VREC-MOK-005` and the retained
evidence under `WO-MOK-001` through `WO-MOK-005` name paths that no longer exist, and they stay exactly
as they are. Each was accurate about the tree it verified. `requirement-to-test-mapping.md` in this
packet is the superseding mapping `ADR-MOK-004` prescribed for that purpose.

**No outstanding amendment was quietly resolved.** `SPEC-MOK-002` carries a 2026-08-18 row marked
**OUTSTANDING** from `WO-MOK-005`, and `SPEC-MOK-003` carries two. All three are untouched and still
say OUTSTANDING. A new row was added below them in each case; adding a row does not discharge the ones
above it, and this work order has no authority to.

**`ARCH-MOK-001` was not amended.** `WO-MOK-006` states that this is "an obligation to confirm rather
than an assumption". Confirmed: `git diff` shows `ARCH-MOK-001` unchanged, and `conformance.md` walks
its prohibited patterns, conformance checks and quality attributes one at a time against the candidate.
It names no source path, which is why the move does not reach it.

---

## 1. `SPEC-MOK-002` — the engine package's structural contract

**Bound by:** `VREC-MOK-003`, status `verified`.

**What the owner approved** (`ADR-MOK-004`, *Required amendments*, `SPEC-MOK-002`): that every path
clause be re-based — *Inputs*, rule 1's target table, rule 3, rule 4, rule 5's `grep` check, rule 8's
file table and rule 9's locations — and that rule 1's note claiming the engine package is unchanged
"in source location" be corrected.

**What the agent wrote:** one *Paths* clause in *Scope* stating the re-basing once for the whole
document, plus a correction at rule 1, plus a new amendment-record row. Stating it once was a choice
about form, made so that no rule's substance is restated and none is re-opened: a rule that is not
edited cannot be accidentally changed. The clause also records the fact that keeps the change small —
the paths inside `mokiterions-core/Cargo.toml` did not move, because a manifest's paths were always
relative to the manifest.

**What did not change:** no file renamed, no rule's requirement altered, no target, target name, target
kind or package name changed, and the dependency table still empty. `file-comparison.txt` and
`manifests.txt` are the measurements.

**Approval column as written:** approved 2026-08-18 by the repository owner as technical owner, by way
of `ADR-MOK-004`; text written by the implementation agent under `WO-MOK-006`; `VREC-MOK-003` not
edited, with the row stating why the paths it names differ afterwards.

## 2. `SPEC-MOK-003` — the observer package's contract

**Bound by:** `VREC-MOK-005`, status `verified`.

**What the owner approved** (`ADR-MOK-004`, *Required amendments*, `SPEC-MOK-003`): the *Component
layout* tree and clauses 3 and 4; *Data and interface contracts* rule 2, "whose reasoning appeals to
that clause"; and both relevant entries of *Explicitly unspecified decisions*.

**What the agent wrote:** four amendments, each with a dated note at the provision it changes.

- The tree restated to one directory per package, matching `SPEC-MOK-004` rule 1, which is named as
  the authoritative tree so the two documents cannot drift apart. Clause 2 gained the concrete form
  `Mokiterions = { path = "../mokiterions-core" }`.
- Clause 3 — "The engine's sources are not relocated" — replaced by the reason it existed for. This is
  the substantive reversal in the whole amendment set, and the note says so: what the clause protected
  is the `REQ-MOK-010` text stream, not the directory, and moving files unchanged preserves it.
  `VER-MOK-006` makes byte-identical output the evidence instead of the file location that used to
  stand in for it. `comparison/engine-matrix.txt` is that evidence: seven captures, 0 differing lines.
- Rule 2's reasoning corrected. It argued that narrowing `Simulation::run` away "would mean relocating
  the engine's sources, which the component layout below forbids". The layout no longer forbids it,
  and the move does not narrow `run` away, because moving a directory changes no module's target
  membership. What would be needed is a target split. **The clause's conclusion is unchanged** — `run`
  stays on the interface, because it is the `REQ-MOK-010` whole-run entry point the engine's binary
  calls — and only its reasoning and its `grep` path changed.
- *Explicitly unspecified decisions*: the grant of "test organization" withdrawn to `REQ-MOK-029` and
  `SPEC-MOK-004` rules 8 to 10, leaving fixtures and helpers with the implementation; and "the package
  layout", withheld but previously fixed nowhere, pointed at `SPEC-MOK-004` rules 1 to 4. The note
  states plainly that the withdrawn grant was taken in good faith and is why all 109 observer tests
  were in one tier.

**What did not change:** no rule about the observer's behavior, presentation, key bindings, export,
snapshot contract or non-perturbation, and no figure. `comparison/observer-matrix.txt` is the
measurement.

**Approval column as written:** approved 2026-08-18 by the repository owner as technical owner, by way
of `ADR-MOK-004`; text written by the implementation agent under `WO-MOK-006`; `VREC-MOK-005` not
edited; the two OUTSTANDING rows above untouched.

## 3. `ARCH-MOK-002` — the observer architecture

**Bound by:** `VREC-MOK-005`, status `verified`. It is an approved architecture, and this is the
artifact whose amendment `ADR-MOK-004` was written to decide.

**What the owner approved** (`ADR-MOK-004`, *Required amendments*, `ARCH-MOK-002`): seven items —
component 4's "the new binary", the *Testability without a terminal* attribute, an addition to
*Required patterns*, additions to *Prohibited patterns*, additions to *Conformance checks*, the
`addresses` and `conforms_to` relations, and `decision_assessment.rationale`.

**What the agent wrote:** all seven, each carrying its own dated marker so a reader can tell amended
text from original text without a diff.

- Component 4 now names the binary target and states what it keeps; component 5 states that the
  presentation layer *is* the library target, that its interface is closed by provenance, and — the
  sentence that matters for the trust model — "It is not a trust boundary — component 2 is — and it
  holds no authority."
- *Testability without a terminal* extended, with the reason stated rather than asserted: in-memory
  assertion alone left every observer test inside the binary, "where it could reach any private item
  and so could be repaired against a changed contract without the change being visible".
- One required pattern added for the library target, one for one directory per package.
- **Three** prohibited patterns added where the ADR's summary says "add that no item is widened and no
  `#[cfg(test)]` attribute removed … and that the observer's binary target does not declare the
  presentation modules a second time". Written as three separate prohibitions — widening, ungating,
  and any test-support seam — because the ADR's *Decision drivers* and its Option 2 rejection state
  the seam prohibition independently, and a seam is the failure mode a reader is most likely to
  reinvent. The binary-target prohibition is stated at component 4 rather than in the list.
- Four conformance checks added under a dated heading.
- `addresses` grew by `REQ-MOK-028`; `conforms_to` by `SPEC-MOK-004`; `decision_assessment.rationale`
  gained a dated sentence recording both changes, naming `ADR-MOK-004` as the deciding ADR and its
  three rejected alternatives as material, and stating that the triggers already declared are the same
  triggers. `outcome` stays `adr_required`, as the ADR requires.
- An *Amendment record* table was added, which the ADR does not require. It is the form
  `SPEC-MOK-002` and `SPEC-MOK-003` already use, and without it this architecture would be the one
  amended artifact whose history is only recoverable from git.

**An observation, not a defect.** `addresses` grew by `REQ-MOK-028` alone, which is exactly what
`ADR-MOK-004` states. `REQ-MOK-029` and `REQ-MOK-030` are traced through `conforms_to = SPEC-MOK-004`,
which `specifies` all three, and both are visibly addressed in the architecture's own text — the
one-directory-per-package required pattern cites `REQ-MOK-030` by name. `harnessctl validate` reports
0 errors and 0 warnings across 67 artifacts, so the harness is satisfied. Whether the two relations
should also be listed is the technical owner's call; the agent wrote the amendment as approved rather
than as it might have been improved. Carried to the completion report.

**Approval column as written:** approved 2026-08-18 by the repository owner as technical owner, by way
of `ADR-MOK-004`; text written by the implementation agent under `WO-MOK-006`; `VREC-MOK-005` binds the
2026-08-17 content and is not edited.

## 4. `docs/engineering/REPOSITORY_CONTEXT.md` — repository-owned guidance

**Bound by:** nothing. `WO-MOK-006` states the distinction: "`REPOSITORY_CONTEXT.md` is
repository-owned guidance, not a governed artifact, so it is brought into line rather than amended."
It carries no `status`, no owner and no lifecycle, and no verification record binds it. Bringing it
into line with approved artifacts is implementation.

**What the owner approved** (`ADR-MOK-004`): its *Commands* and *Architecture* sections, and the
sentence stating the two-tier test convention repository-wide on engine-scoped authority.

**What the agent wrote:**

- *Commands*: the two package-selection forms for both build and test, and the fact that the root is a
  virtual manifest so a bare invocation reaches both members. The engine's `cargo run --bin
  Mokiterions` form was already correct here, which is worth noting because the same form in
  `SIMULATION_RULES.md` was not.
- *Architecture*: `mokiterions-core/` named as the engine's directory, with the sentence that prevents
  the predictable misreading — "The directory name `mokiterions-core` is a directory name only: the
  package, its library target and its binary are still `Mokiterions`, `mokiterions` and
  `Mokiterions`." The observer's two targets described, including why its binary is deliberately not
  thin. The observer's public interface described as closed differently from the engine's and
  explicitly as not a trust boundary.
- *Repository constraints*, test placement: **this is the sentence `WO-MOK-005` disclosed as its
  fifteenth finding.** It stated the two-tier convention repository-wide while citing only
  `SPEC-MOK-002`, which is engine-scoped, at a time when all 109 observer tests were in one tier — so
  it was a repository-wide claim on authority that did not cover the observer, and it disagreed with
  the code. It is now stated for both packages, with each package's own authority cited, an engine
  bullet, an observer bullet, the single-invocation clause, the no-widening and no-ungating
  prohibition for both packages, and the verbatim-assertions rule. `WO-MOK-006` says this work order
  "is what makes it true rather than what weakens it", and that is what the text now does.

**Approval column:** none required. Recorded here as implementation, performed by the agent under
`WO-MOK-006`, in the substance `ADR-MOK-004` states.

---

## The nine lifecycle transitions recorded

Each was the owner's decision, recorded by the implementation agent on the owner's explicit
instruction. Each is a one-line change to a `status` field, except where noted.

| Artifact | Transition | Role that approved |
|---|---|---|
| `INT-MOK-005` | `draft` → `approved` | product owner |
| `CAP-MOK-005` | `draft` → `approved` | product owner |
| `REQ-MOK-028` | `draft` → `approved` | product owner |
| `REQ-MOK-029` | `draft` → `approved` | product owner |
| `REQ-MOK-030` | `draft` → `approved` | product owner |
| `SPEC-MOK-004` | `draft` → `approved` | technical owner |
| `ADR-MOK-004` | `draft` → `approved` | technical owner |
| `VER-MOK-006` | `draft` → `approved` | assurance owner |
| `WO-MOK-006` | `draft` → `approved` → `in_progress` | engineering owner |

`WO-MOK-006` also gained an *Approval record* section stating the authorization, the recording, and the
reading the four amendments rest on. `ADR-MOK-004`'s own *Status* prose read "Proposed." after the
transition; it now reads "Accepted by the technical owner on 2026-08-18", matching the frontmatter and
the form `ADR-MOK-001` and `ADR-MOK-003` use. Leaving a frontmatter status and a prose status
disagreeing would have made the record ambiguous about the very thing it exists to state.

`harnessctl validate` reports **PASS, 67 artifacts, 0 errors, 0 warnings** on the amended set, and
`harnessctl preflight --work-order WO-MOK-006` reports the work order eligible with no blocking
finding.

## What an accountable reader should check

Not the words — the division. For each of the four artifacts: read `ADR-MOK-004`'s *Required
amendments* entry, then read the amended provision, and ask whether the second says more than the
first authorized. Three places are where a reader should look hardest, because each is where the agent
exercised judgement about form:

1. `SPEC-MOK-002`'s single *Paths* clause, rather than an edit at each of the seven path sites.
2. `ARCH-MOK-002`'s **three** prohibited patterns where the ADR's summary sentence names two subjects.
3. `ARCH-MOK-002`'s new *Amendment record* table, which the ADR does not ask for.

None of the three changes what any rule requires. All three are recorded here so that the judgement is
visible rather than buried in a diff, which is the same reason `ADR-MOK-004` required a comparison
mechanism for the code.
