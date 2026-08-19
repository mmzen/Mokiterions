+++
id = "REQ-MOK-037"
type = "requirement"
title = "Carry provenance inside every published asset"
status = "approved"
owners = ["product owner"]
created = "2026-08-19"
updated = "2026-08-19"
statement = "Every published release asset SHALL state within itself the full authorized commit, the tag, the version, the identifiers of the authorizing records, the build target and the compiler version, SHALL have a checksum published beside it, and SHALL state no credential and no path from the machine that built it."
verification_method = "automated-test"

[relations]
derives_from = ["CAP-MOK-007"]
+++

# Requirement: Carry provenance inside every published asset

## Rationale

An archive separated from the release page it was downloaded from is, today, unidentifiable. The two binaries it
contains print a version at most; the observer's provenance footer reads a compile-time commit that nothing
populates. A recipient holding such an archive cannot establish which commit it was built from, which decisions it
contains, or whether the bytes are the bytes that were produced.

That recipient is the one stakeholder with no access to anything else. They cannot clone the repository, run the
harness, read the artifact graph, or ask the release owner. Whatever they can establish, they must establish from
the file in their hands. So provenance that lives only on a release page, or only in the artifact graph, is not
provenance for them — it is provenance for people who already had it.

The full commit matters specifically. An abbreviated hash is ambiguous in principle and, more practically, cannot
be compared character-for-character against a release record without someone deciding how many characters to
compare. The release record states forty characters; the archive must state the same forty.

The checksum matters for a different reason: it is the only claim in this requirement that a recipient can check
rather than read. Everything else in the provenance statement is an assertion. The checksum is a test.

The prohibitions matter because a provenance file is an unusual thing to write: it is the one file in the release
whose entire purpose is to describe the machine that produced it, which makes it exactly the file most likely to
leak a token from an environment variable or a working directory from a build path. `SPEC-MOK-003` already imposes
the same discipline on the observer's footer, and the reasoning transfers.

## Preconditions and trigger

**Trigger.** A release has been authorized under `REQ-MOK-035` and compliance has been re-established under
`REQ-MOK-036`, and an asset is being produced for a supported target.

**Preconditions.**

- The authorized facts — commit, version, tag, release record, release contract, released work orders, included
  verification records — are available as fixed values emitted by the authorization step.
- The compiler version in use is the declared one, per `REQ-MOK-039`.

## Required response

**Each asset SHALL contain a provenance statement** in plain text, readable without extracting the whole archive
and without any tool specific to this project, stating at least:

1. the full authorized commit, complete for the repository's declared object format, unabbreviated;
2. the tag the release was requested for;
3. the version the release record states;
4. the identifier of the release record;
5. the identifier of the release contract it satisfies;
6. the identifiers of every work order the record releases;
7. the identifiers of every verification record the release includes;
8. the build target the asset is for;
9. the compiler version that produced it;
10. an identifier for the build run that produced it, sufficient to locate that run's own record.

**Each asset SHALL have a checksum published beside it**, using a published algorithm and a format a standard
verification tool accepts without editing.

**The values SHALL come from the authorized facts.** The version comes from the release record, not from a package
manifest. The commit comes from the release record, not from the current checkout. The released work orders come
from the release record, not from a commit range or a changelog.

**The release's own notes SHALL state the same facts**, so that the release page and the archive agree and neither
is the sole holder of the information.

**The statement SHALL NOT contain** a credential, token, or secret in any form; an absolute path from the machine
that built the asset; or any value that varies between two builds of the same commit on the same target other than
the build-run identifier.

## Failure and boundary behavior

- **An asset that cannot state a required fact is not published.** An empty or placeholder value is a failure, not
  a degraded success: a provenance file with a blank commit line is worse than none, because it looks answered.
- **An abbreviated commit is a failure.** Not expanded, not accepted with a note. The requirement is character
  equality with the release record.
- **A missing checksum is a failure for the whole release**, not for the one asset. A release in which some assets
  are verifiable and others are not invites the recipient to assume the unverifiable ones are fine.
- **A checksum computed over a different file than the one published** is the failure mode this requirement most
  needs to exclude; the checksum is computed from the published bytes.
- **The compiler version is recorded as reported by the compiler**, not as declared in the pin. The pin states what
  should have been used; `REQ-MOK-039` establishes they agree; the provenance statement records what did.
- **The build-run identifier is the single permitted environment-derived value.** It is not a secret, it identifies
  a record rather than a machine, and without it the archive cannot be traced back to the run whose checks
  established `REQ-MOK-036`. Any other environment-derived value is prohibited.
- **Wall-clock time is not required and not prohibited by this requirement**, but a timestamp is not accepted as a
  substitute for any of the ten facts above, and it must not be the only thing distinguishing two archives.
- **The compiled binaries are not required to state the commit.** The observer's provenance footer cannot currently
  carry it at the narrowest declared viewport, and `WO-MOK-008` owns that defect. Provenance therefore travels
  beside the binaries, inside the same archive. If `WO-MOK-008` is completed, that becomes an additional place the
  commit appears, not a replacement for this one.
- **An asset is not published before its provenance statement and checksum exist.** A partial upload followed by a
  correction is not permitted, because the intermediate state is a published asset that violates this requirement.

## Constraints

- Plain text, no project-specific format, no schema a recipient must obtain separately. A recipient reading it in
  a text viewer is the design target.
- The statement is a single file at a predictable location inside the archive, so that a recipient can be told
  where to look in one sentence.
- One archive per supported target, named so that the version and the target are both readable from the file name
  without opening it.
- The archive contains the built binaries and the repository's licence, readme and simulation-rules documents, so
  that the terms and the behavior description travel with the binaries.
- Producing the statement adds no dependency to either package and requires no build script, and therefore does
  not affect the engine's empty dependency table.
- The provenance statement is not compiled into either binary, so producing it cannot invalidate the compilation
  the compliance checks covered.
- No secret is required to produce the statement or the checksum.

## Acceptance examples

**A published archive states its provenance.**
Given an authorized release with commit `C`, version `0.1.0`, tag `v0.1.0`, record `RLS-MOK-001`, contract
`REL-MOK-001`, work orders `WO-MOK-001`..`006` and verification record `VREC-MOK-007`,
when the archive for a supported target is produced,
then it contains a plain-text provenance statement naming all forty characters of `C`, `v0.1.0`, `0.1.0`,
`RLS-MOK-001`, `REL-MOK-001`, the six work orders, `VREC-MOK-007`, the target, the compiler version and the build
run.

**A recipient verifies the bytes.**
Given a published archive and the checksum published beside it,
when the recipient runs a standard checksum verification tool over the pair,
then verification succeeds without editing the checksum file.

**The commit matches the record character for character.**
Given the published archive and the release record `RLS-MOK-001`,
when the commit line of the provenance statement is compared to the record's commit,
then they are equal over all forty characters.

**The version comes from the record, not the manifest.**
Given a release record stating version `0.2.0` while the package manifests still declare `0.1.0`,
when the archive is produced,
then the archive name and the provenance statement state `0.2.0`.

**An abbreviated commit is refused.**
Given a production step that would write a shortened commit into the provenance statement,
when the asset is examined before publication,
then the release fails, naming the abbreviated commit.

**No credential and no local path appear.**
Given a build environment holding a credential in an environment variable and a working directory under a
user-specific absolute path,
when the provenance statement is examined,
then it contains neither the credential nor the absolute path.

**Two builds of the same commit differ only by the run identifier.**
Given two build runs of the same commit for the same target,
when their provenance statements are compared,
then every line is identical except the build-run identifier.

**The release notes agree with the archive.**
Given the published release and any one of its archives,
when the notes and the provenance statement are compared,
then both state the same commit, version, record, contract, released work orders and verification records.

## Open decisions

- **Whether the checksum should also be published in a single combined manifest** covering every asset, in addition
  to one file per asset. A combined manifest is more convenient and creates a second thing that can disagree.
- **Whether the provenance statement should be machine-readable.** Plain text serves the recipient; a structured
  format would serve tooling that does not yet exist. Adding one later is compatible; requiring one now is
  speculative.
- **Whether the compiled binaries should also state the commit once `WO-MOK-008` is settled.** That is
  `WO-MOK-008`'s question, and this requirement neither pre-empts nor blocks it.
- **Whether the archive should also carry the release record and contract themselves, rather than only their
  identifiers.** It would make the archive self-contained; it would also duplicate governed artifacts into a place
  where they cannot be superseded.
- **Whether signing should eventually replace or supplement the checksum.** Out of scope by `INT-MOK-007`, recorded
  here so the boundary is visible rather than forgotten.
