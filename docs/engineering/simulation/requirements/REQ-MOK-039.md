+++
id = "REQ-MOK-039"
type = "requirement"
title = "Build and verify a release with one compiler version the repository declares"
status = "draft"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "The repository SHALL declare one compiler version, every step that verifies or builds a release SHALL establish that the compiler in use is that version and refuse otherwise, and that version SHALL be recorded with each published asset."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-007"]
+++

# Requirement: Build and verify a release with one compiler version the repository declares

## Rationale

A verification record binds a commit. It does not bind a compiler, and a commit is not enough to determine what
happens when the commit is compiled.

The gap is concrete in this repository. The declared lint gate is `-D warnings`, so a compiler release that adds a
lint fails an unchanged commit. Whether a warning exists at all is a property of the compiler version, not of the
source. So a floating compiler version makes `REQ-MOK-036` unstable in a specific way: the same commit that a
verification record calls verified can fail its own re-check, weeks later, with nothing in the repository having
changed. The failure is genuine — the lint is real — but the repository would have no way to say whether the
evidence it holds was produced under the same conditions.

The gap also runs the other way. If a release is built with a compiler that differs from the one the evidence was
produced with, then the assurance owner's judgement — made by reading evidence from one compiler — is being applied
to a binary from another. That may be harmless. It is not established to be harmless, and nothing in the artifact
graph records which compiler produced which evidence.

Declaring the version fixes both, and it introduces its own obligation: **raising the pin invalidates the claim
that the release was built with the compiler the evidence was produced with.** That is why the version must be
declared in a tracked file whose change is a reviewable event, rather than inferred from whatever the build
environment happens to provide.

The recording clause exists because the declaration says what should have been used, and only the asset can say
what was.

## Preconditions and trigger

**Trigger.** Any step that compiles, lints, formats, tests, or builds the repository as part of a release, and any
local invocation of the toolchain in a clone of the repository.

**Preconditions.**

- A toolchain manager capable of honouring a repository-declared version is installed. Where none is, the
  declaration cannot be honoured and the requirement's refusal clause applies.

## Required response

1. **The repository declares exactly one compiler version**, in a tracked file, as an exact version rather than a
   channel, a range, or a floating alias.
2. **The declaration also names the components the declared checks need**, so that formatting and lint checks are
   available at that same version rather than resolved from elsewhere.
3. **A clone honours the declaration automatically.** The first invocation of the build tool in the repository
   installs and selects the declared version without a separate instruction.
4. **Every step that verifies or builds a release establishes the version in use.** It compares the version the
   compiler reports against the version the repository declares, and refuses when they differ, naming both.
5. **The comparison reads the declaration from the repository**, not from a value duplicated into the process
   definition, so the declaration has exactly one place to be changed.
6. **The compiler version is recorded with each published asset**, as reported by the compiler, per `REQ-MOK-037`.
7. **Raising or lowering the declared version is authorized work**, not a release step. It changes what future
   evidence means and it invalidates the claim that a release was built with the compiler its evidence was produced
   under, so it requires a work order like any other change with that consequence.

## Failure and boundary behavior

- **A compiler version that differs from the declaration refuses the step.** In either direction. A newer compiler
  can fail a lint the evidence never faced; an older one can lack a language feature the source relies on. Both are
  the same failure: the build is not the build the repository declared.
- **The comparison is against the exact version**, not a minimum. This is not a compatibility floor. Both packages
  declare a language edition and the workspace declares a resolver version, and those establish a floor that sits
  far below the declared version; the declaration is about identity, not capability.
- **A build environment that overrides the declaration is a failure.** An explicit toolchain override on the
  command line, or an environment that pre-selects a different version, is caught by the comparison rather than
  trusted.
- **An unreadable or absent declaration is a failure.** A step that cannot determine which version is declared has
  established nothing and must not proceed.
- **A declaration naming a channel rather than an exact version is a failure of this requirement**, even though the
  toolchain manager would accept it. A channel resolves differently over time, which is the condition the
  requirement exists to remove.
- **A component the declared checks need but the declaration omits** is a failure at the point the check cannot
  run, and the response is to name the component in the declaration rather than to install it separately.
- **The declared version being unavailable for a supported target** refuses that target's build rather than
  substituting a nearby version.
- **A local developer is subject to the same declaration**, so evidence produced locally is produced under the same
  compiler as evidence produced by the release process. A developer who overrides the pin produces evidence the
  release process will not reproduce.
- **The declaration does not constrain a consumer building from source.** It constrains this repository's evidence
  and this repository's releases. A downstream build with another compiler is out of scope and makes no claim this
  requirement governs.

## Constraints

- One declaration, in one tracked file, read by every step that needs it.
- The declared version is the version the repository's evidence was produced with as of the declaration's most
  recent change. It is a record, not an aspiration.
- The declaration adds no dependency to either package and does not affect the engine's empty dependency table.
- The repository's setup documentation states that the declaration selects the toolchain and that overriding it by
  hand is not permitted, so a new contributor does not install a version and then wonder why it is not in use.
- The comparison is a string comparison against the version the compiler reports, made before any compilation in
  the step, so a mismatch costs no build time.

## Acceptance examples

**A clone selects the declared version.**
Given a clone of the repository and a toolchain manager that honours a repository-declared version,
when the active toolchain is queried,
then it reports the declared version and identifies the repository's declaration as the reason.

**A verifying step establishes the version.**
Given the declared version `1.97.1` and a build environment providing `rustc 1.97.1`,
when the verifying step checks the toolchain,
then the comparison succeeds and the step proceeds.

**A drifting environment is refused.**
Given the declared version `1.97.1` and a build environment providing `rustc 1.98.0`,
when the verifying step checks the toolchain,
then it refuses, naming both the declared version and the version in use, and no compilation occurs.

**An older compiler is refused too.**
Given the declared version `1.97.1` and an environment providing `rustc 1.90.0`,
when the verifying step checks the toolchain,
then it refuses on the same comparison, without treating the declaration as a minimum.

**The building step is subject to the same check.**
Given an authorized release,
when each target's build begins,
then that build establishes the compiler version against the declaration before compiling, independently of the
verifying step.

**The recorded version matches the declared one.**
Given a published asset for any target,
when its provenance statement's compiler line is compared to the repository's declaration at the authorized commit,
then they name the same version.

**The declaration has one home.**
Given the process definition and the repository,
when the declared version is searched for,
then it appears as a value in exactly one tracked file, and every step reads it from there.

**A formatting or lint component is present at the declared version.**
Given a clone with no separate component installation,
when the declared formatting and lint checks are run,
then both are available at the declared version.

## Open decisions

- **Which role owns raising the declared version, and on what trigger.** A pin that is never raised eventually
  fails to build on a supported platform; one raised casually silently re-bases every evidence claim. The
  requirement establishes that raising it is authorized work; who initiates it, and how often, is unresolved.
- **Whether raising the pin should require re-capturing verification evidence.** If the pin is the condition under
  which evidence was produced, changing it arguably invalidates the evidence. Arguably it does not, because the
  commit is unchanged. This is the assurance owner's decision and it is not settled here.
- **Whether the declared version should be recorded in the artifact graph as well as in the declaration file**, so
  that a verification record states the compiler it was produced under. It would close the gap this requirement
  names at its root; it would also require the harness's record format to carry a field it does not currently have.
- **Whether a supported target that cannot provide the declared version should reduce the release's target set** or
  refuse the release. Currently the target's build refuses, which refuses the release; whether a partial release is
  ever acceptable is a release-contract question.
