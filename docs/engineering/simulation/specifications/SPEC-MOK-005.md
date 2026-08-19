+++
id = "SPEC-MOK-005"
type = "specification"
title = "Release authorization, compliance re-establishment, asset provenance and reserved acts"
status = "approved"
owners = ["technical owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
specifies = ["REQ-MOK-035", "REQ-MOK-036", "REQ-MOK-037", "REQ-MOK-038", "REQ-MOK-039"]
+++

# Specification: Release authorization, compliance re-establishment, asset provenance and reserved acts

## Amendment record

| Date | Change | Approval |
|---|---|---|
| 2026-08-19 | Original content for `REQ-MOK-035` through `REQ-MOK-039`. | Technical owner, 2026-08-19. |
| 2026-08-19 | Rule 12.5 restated in platform-aware terms: write access is confined to the smallest job containing the attaching step, with the credential reaching that step alone. The original wording ("the attaching step is the only step granted write access") was unsatisfiable as written, because GitHub Actions scopes `permissions` per job and offers no step-level grant. Raised by `WO-MOK-009`'s implementation. | Technical owner, 2026-08-19. |

## Scope

This is the contract for **turning a tag into a published release, and for refusing to**. It covers the two
revisions a release involves, the authorization decision, the compliance re-establishment, the composition of a
published asset, the provenance an asset states, the compiler declaration, and the acts the process is forbidden to
perform.

It does **not** cover simulation or observer behavior. No rule here changes what either binary computes, prints, or
returns, and no rule here changes any public interface of either package. `SPEC-MOK-002`, `SPEC-MOK-003` and
`SPEC-MOK-004` remain the sole authorities for those subjects, and this specification's only relationship to them is
that it re-runs the checks they are verified by.

It does **not** cover the human decisions a release requires. Deciding contents, version, sufficiency of evidence,
and whether to publish are the release owner's and assurance owner's acts. This specification governs the mechanism
around them and rule 14 governs where their sequence is written down.

Three artifacts realize this specification: an **authorization gate**, a **release process** with the gate as its
first step, and a **compiler declaration**. Rule 1 defines the vocabulary the rest depends on, and getting rule 1
wrong is the single most likely way to produce a process that refuses every genuine release.

## Actors and external systems

- **The release owner** creates the annotated tag, pushes it, and later makes the release visible. They are outside
  the process on both sides of it.
- **The assurance owner** transitions the aggregate verification record before any of this runs.
- **A recipient** consumes the published assets and never interacts with the process.
- **The hosting platform** provides the trigger, the compute, a scoped credential, the release object, and the
  ability to hold a release invisible. It is the only external system.
- **The harness** is invoked as a Python module in the repository. It is not assumed present on the executable
  search path.
- **The toolchain manager** honours the compiler declaration.
- **The managed harness workflow** is neither invoked nor edited. It is integrity checked, and it declares no
  reusable-workflow trigger, so it cannot be called even if calling it were desirable.

## Inputs

| Input | Source | Form |
|---|---|---|
| tag name | trigger reference, or an explicit request parameter | a string naming an existing tag |
| repository history | the remote, fetched in full | complete, so that ancestry and tag objects can be resolved |
| default branch name | the trigger's event payload | a branch name |
| artifact graph | the working tree at the governance revision | `+++`-delimited TOML front matter under the artifact root |
| harness configuration | the repository | declares the artifact root, the harness version, and whether full commits are required |
| compiler declaration | a tracked file in the repository | one exact version and the required components |
| committed lockfile | the repository at the authorized commit | resolved dependency versions |
| platform credential | the platform | scoped, never stored in the repository |

No input is read from a package manifest, a branch name, a commit message, or a commit range. Every fact about what
is being released comes from the release record.

## Outputs

| Output | Where | Visibility |
|---|---|---|
| authorized facts | the process's own step outputs | internal to the run |
| refusal statements | the process's diagnostic output | readable by whoever requested the release |
| harness report artifact | attached to the run | internal to the run |
| one archive per supported target | attached to the release | published with the release |
| one checksum file per archive | attached to the release | published with the release |
| a provenance statement inside each archive | inside the archive | published with the release |
| release notes | the release object | published with the release |
| a release that is not publicly visible | the platform | invisible until a person acts |

There is no output that modifies the repository. There is no output on a refusal other than the refusal itself.

## State model

The process has four states and no others.

1. **Unauthorized** — the initial state. Nothing has been established. The only transitions out are to
   *Authorized* or *Refused*.
2. **Authorized** — a `released` release record has been established as naming the tagged commit, and the
   authorized facts are fixed. No fact is re-derived after this point.
3. **Refused** — terminal. No asset exists, nothing is attached, the repository is unchanged. Reachable from
   *Unauthorized*, *Authorized* and *Produced*.
4. **Produced** — assets and checksums exist for every supported target, and are attached to the release as
   invisible. Terminal as far as the process is concerned; the release owner's act moves it to visible, and that
   transition is outside this specification.

There is no state in which some assets are attached and others are not. There is no state in which the release is
visible and the process is still running.

## Behavioral rules

### 1. Two revisions, and which is read for what

A release involves exactly two revisions, and they are not interchangeable.

- The **authorized commit** is the commit being released. It is reachable from the integration branch or from a
  maintenance branch, and the release record names it.
- The **governance revision** is the commit holding the release record. It is strictly later than the authorized
  commit.

The second follows from the first. A release record states the authorized commit's hash, so it can only be written
after that commit exists; the harness says so in its own source — *"the release candidate commit may precede the
governance commit retaining this record"*, and *"the record is intentionally created after the candidate commit it
names, avoiding self-referential commit metadata"*. `VREC-MOK-001` already demonstrates the pattern: it binds
`ecd03a89c0c8680d8ce82d7767b787a49aa815fb` from a later commit.

Therefore:

- **The artifact graph is read at the governance revision.** The tagged tree does not contain the release record
  that authorizes it. A process that reads the graph from the tag refuses every genuine release, and does so with a
  message that looks like a governance failure rather than a defect.
- **Code is checked and built at the authorized commit.**
- **The governance revision is established once**, at the start of the run, as the tip of the default branch, and
  every later step that reads the graph reads that fixed commit. A push landing mid-run cannot change the answer.
- Establishing the governance revision requires the default branch name from the event payload. An absent name is a
  refusal, not a fallback to a hard-coded branch name.
- The checkout used to establish the governance revision is verified to be at the tip of the default branch before
  the commit is recorded.

### 2. Trigger and requested tag

The process runs on a pushed tag matching the released-version naming convention, or on an explicit request naming
an existing tag. In both cases the tag name is resolved to a single value at the start of the run and used
unchanged thereafter.

An explicit request naming a tag that is not on the remote refuses under rule 4.1. The explicit form re-runs a
release; it does not preview one.

### 3. The tag must be annotated

The requested tag's object type is established before anything else about it. An annotated tag records who created
it and when; a lightweight tag records neither. A lightweight tag is refused.

### 4. The authorization gate

The gate is a program in the repository, runnable by hand with the same arguments the process passes it. It reads
the working tree and the git history and writes nothing. It refuses unless every one of the following holds, and
each refusal names the fact it could not establish.

**4.1 The tag resolves.** The requested name resolves through the tag reference namespace to a commit. Resolution
is confined to tags, so a branch of the same name cannot satisfy it.

**4.2 The graph is unambiguous.** Every artifact under the artifact root has front matter that parses and an
identifier that appears exactly once. A duplicate identifier is refused before any other graph fact is read.
Directories the validator excludes are excluded here too, so a template is not mistaken for an artifact.

**4.3 The repository requires full commits.** The harness configuration declares that full commits are required. The
gate's central check is equality of complete hashes and it does not run where abbreviations are permitted.

**4.4 Exactly one release record claims the tag.** Among artifacts whose declared type is a release record and whose
status is `released`, exactly one states the requested tag. If none states a tag, exactly one whose conventional
tag name — the letter `v` followed by its version — equals the requested tag is accepted. Zero is refused as "no
release record"; two or more is refused as ambiguous.

**4.5 The record's commit is complete.** The record's commit consists only of lowercase hexadecimal characters and
has the length the record's declared object format requires — 40 for `sha1`, 64 for `sha256`. An abbreviated commit
is refused, never expanded.

**4.6 The record's commit equals the tagged commit.** Character-for-character over the full length. This is the
check that catches a moved tag, and it is the reason 4.5 exists.

**4.7 The gating contract is present, correctly typed and active.** The artifact the record satisfies exists, its
own declared type is a release contract, and its status is one of `approved`, `in_progress`, `implemented`,
`verified`, `released`. A relation's name is never taken as evidence about its target's type; the target's declared
type is.

**4.8 The contract gates all released work.** Every work order the record releases appears in the contract's gated
set. Work the contract does not gate is refused, named.

**4.9 Released work exists and is releasable.** Every work order the record releases exists, is declared a work
order, and holds a status of `implemented`, `verified` or `released`.

**4.10 Included verification is present, eligible and commit-bound.** Every verification record the release record
includes exists, is declared a verification record, holds a status of `verified` or `released`, and states a commit
equal to the record's commit under 4.5's completeness rule. A verification record bound to any other commit is
refused, naming that a release requires verification captured at the final candidate commit.

**4.11 Released and verified work agree exactly.** The set of work orders the record releases equals the union of
the work orders the included verification records cover. Released-but-not-verified and verified-but-not-released are
both refused, and both name the offending identifiers.

**4.12 The version is usable.** The record's version matches `^[A-Za-z0-9][A-Za-z0-9._+-]{0,63}$`, so that it can be
used in a release name and an archive file name without interpretation.

On success the gate writes the authorized facts — commit, version, release record, release contract, released work
orders, included verification records — to its output channel and reports that it authorized the release, naming
each fact. On any failure it writes a refusal naming the missing fact and terminates with a failing status.

### 5. Reachability

The authorized commit is an ancestor of the default branch or of some maintenance branch present on the remote.
Maintenance branches are enumerated from the remote's own references rather than assumed, because a release that
entered stabilization is released from its maintenance branch and its tip is not an ancestor of the default branch.

A commit reachable only from a feature or bugfix branch is refused. A commit reachable from no remote branch is
refused.

This check is separate from rule 4 because it is a property of the history rather than of the graph, and because the
gate must remain runnable in a clone with no remote.

### 6. Order of establishment

Authorization and reachability run before any compilation, so a refusal costs no build time. Compliance
re-establishment runs before any asset is produced. Asset production runs before anything is attached. Attachment
happens once, for all assets, at the end.

### 7. Harness compliance at the governance revision

At the governance revision, in this order:

**7.1** The installed harness version equals the version the harness configuration declares. A mismatch in either
direction refuses, naming both versions, before any harness command runs.

**7.2** The harness's repository check passes, which includes the integrity of every managed file.

**7.3** The repository's artifact validation reports zero errors across every governed plane. Warnings do not
refuse; the repository carries warnings and open amendment rows by design, and a judgement about one belongs in the
release contract.

**7.4** For every work order the release record releases, the harness's review-phase preflight passes. A failure
names the work order.

**7.5** The harness's report is attached to the run, so the compliance claim is inspectable after the fact.

Harness commands are invoked as a Python module.

### 8. Repository checks at the authorized commit

At the authorized commit, the checks `docs/engineering/REPOSITORY_CONTEXT.md` declares, invoked with the commands it
declares:

**8.1** The formatting check across the workspace.

**8.2** The lint check across the workspace, all targets, all features, warnings as errors, resolving dependencies
from the committed lockfile without updating it.

**8.3** The test command across the workspace, resolving from the committed lockfile without updating it. No tier is
skipped, feature-gated, ignored, or dependent on an interactive terminal.

**8.4** The engine package's dependency tree resolves to exactly one crate. This discharges the named architectural
claim that the engine's dependency table stays empty, with no exception, including a dependency shared with the
observer.

**8.5** For each decision policy the engine declares deterministic, two runs with identical arguments produce
byte-identical output, an identical final state and an identical exit code. A policy whose output legitimately
varies is not compared.

No check may modify a tracked file. A check that would reformat rather than report, or update the lockfile rather
than honour it, is prohibited: the released commit must be the commit that was checked.

### 9. The compiler declaration

**9.1** The repository declares one exact compiler version in one tracked file, together with the components the
declared checks need. A channel, a range, or a floating alias is not a declaration.

**9.2** A clone honours the declaration on the first invocation of the build tool, with no separate instruction.

**9.3** Every step that verifies or builds a release reads the declared version from that file and compares it to
the version the compiler reports. A difference in either direction refuses, naming both. The comparison happens
before any compilation in that step.

**9.4** The declared version is not duplicated into the process definition. One file holds it; every step reads it
from there.

**9.5** The comparison is against the exact version, not a minimum. The language edition both packages declare and
the resolver version the workspace declares establish a floor far below it; the declaration is about identity.

**9.6** Changing the declared version is authorized work, because it changes what future evidence means and
invalidates the claim that a release was built with the compiler its evidence was produced under.

### 10. Asset composition

One archive per supported target. The supported targets are Linux on x86-64, macOS on arm64, and Windows on
x86-64.

**10.1** Each archive contains both binaries — the engine binary and the observer binary — built in release
configuration for that target, resolving dependencies from the committed lockfile without updating it.

**10.2** Each archive contains the repository's licence, its readme, and its simulation-rules document, so the terms
and the behavior description travel with the binaries.

**10.3** Each archive contains a provenance statement at a predictable path, per rule 11.

**10.4** Each archive's name states the version and the target.

**10.5** A checksum file sits beside each archive, computed over the published bytes, in a format a standard
verification tool accepts unedited.

**10.6** No compilation result is carried between the checking steps of rule 8 and the building steps of this rule.
A shared cache would let an artifact built before a check reach an archive.

**10.7** The binaries are built without a commit stamp compiled in. The observer's provenance footer cannot carry
one at the narrowest declared viewport — the `short` tier occupies 33 of the 34 available columns, and a stamp costs
at least three — and `WO-MOK-008` owns that defect. The compile-time stamp is also not a rebuild input for either
package, since neither has a build script, so setting it could silently reuse objects compiled without it.
Provenance therefore travels beside the binaries.

### 11. Provenance statement

A plain-text file, readable without extracting the whole archive and without any project-specific tool, stating:

| Line | Value | Source |
|---|---|---|
| version | the version | the release record, via the authorized facts |
| release record | its identifier | the authorized facts |
| release contract | its identifier | the authorized facts |
| released work | every released work order's identifier | the authorized facts |
| verification records | every included record's identifier | the authorized facts |
| tag | the requested tag | the run's fixed tag value |
| commit | the authorized commit, complete and unabbreviated | the authorized facts |
| target | the build target | the build matrix |
| toolchain | the compiler version as the compiler reports it | the compiler |
| built by | an identifier locating this run's own record | the platform |

**11.1** Every value comes from the authorized facts, the build matrix, the compiler, or the platform's run
identity. None is read from a package manifest or derived from history.

**11.2** The commit is complete. An abbreviated commit in this file is a failure of the release.

**11.3** The statement contains no credential, token, or secret; no absolute path from the building machine; and no
value that varies between two builds of the same commit for the same target other than the build-run identifier.
The build-run identifier is the single permitted environment-derived value: it names a record, not a machine, and
without it an archive cannot be traced to the run whose checks established rule 7 and rule 8.

**11.4** The release notes state the same version, commit, record, contract, released work orders and verification
records, built from the authorized facts and not from a commit range or a changelog, so the release page and the
archive agree.

**11.5** Producing the statement adds no dependency to either package and requires no build script.

### 12. Publication boundary

**12.1** Assets are attached to the tag that already exists. The attaching call is invoked in a form that fails when
the tag is absent rather than creating it.

**12.2** The release is created not publicly visible, and no configuration of the process changes that default.

**12.3** Attachment happens once, for all assets of all targets. There is no state in which some are attached.

**12.4** Making the release visible is the release owner's act and is not performed by the process.

**12.5** Write access is confined to the smallest job that contains the attaching step, and within that job the
credential is passed to the attaching step alone. Every other step in that job, and every other job, runs read-only.
The narrower form — a grant scoped to the step itself — is not available on GitHub Actions, where `permissions` is a
job-level key; an implementation on a platform that does scope write access per step confines it there instead.

**12.6** The process references a deployment environment for the attaching step, so that an additional human
approval before upload can be required by configuration alone, without editing the process.

### 13. Acts the process never performs

The process does not, in any outcome including every failure path:

**13.1** create, move, delete, or force-update any tag, locally or on the remote;

**13.2** transition the status of any artifact, or write, edit, delete or commit any artifact;

**13.3** commit or push to any branch;

**13.4** create any branch, including the maintenance branch — the branching model cuts
`release/<major>.<minor>` from the integration branch when a release enters stabilization, which is before the tag
exists, so by the time the process runs the moment has passed and creating it would be both the wrong actor and the
wrong commit;

**13.5** change a version in a package manifest, a lockfile, or any other tracked file;

**13.6** make a release publicly visible;

**13.7** hold, require, or emit a secret beyond the platform's own scoped credential;

**13.8** invoke or modify the managed harness workflow;

**13.9** delete a release it previously attached.

Diagnostic output is not a repository write, and refusals are expected to be verbose.

### 14. The documented human sequence

An ordered procedure lives in the repository, outside the governed artifact root, stating for each human act who
performs it, what must already hold, and what it produces. It covers at minimum: deciding contents and version;
capturing the aggregate verification record and transitioning it; approving the release contract; preparing the
release record and transitioning it; cutting the maintenance branch; creating and pushing the annotated tag; and
making the release visible.

It states that it is operator documentation and not authority, and that where it disagrees with a governed artifact
the artifact prevails. It names roles, not people. It lives outside the artifact root precisely so that it cannot be
mistaken for an artifact carrying authority.

## Error and recovery behavior

- **Every check fails closed.** A check that cannot establish its fact refuses. An unreadable artifact, malformed
  front matter, a failing git invocation, an absent configuration value, an unavailable toolchain — each is a
  refusal, never a pass.
- **A refusal names the fact.** "Authorization failed" is not a conforming message. The named fact is what the
  release owner acts on.
- **A refusal is total.** No asset, no attachment, no repository change.
- **Recovery is forward only.** A refused run is corrected by changing the graph or the code and re-running. If the
  authorized commit itself must change, the version changes too, because a published tag is never moved and a
  release record is superseded rather than edited.
- **A moved tag is refused by rule 4.6 and is not recoverable by re-running.** This is intended: the graph and the
  history disagree, and only a person can decide which is wrong.
- **A partial failure during production leaves nothing published**, because attachment is a single terminal step.

## Data and interface contracts

- **Artifact front matter** is `+++`-delimited TOML at the head of a Markdown file. The gate reads it with a TOML
  parser, tolerating a byte-order mark, and locates the closing delimiter as the first `+++` line after the first.
- **The gate's command interface** takes a repository root and a tag name, and is runnable by hand. It writes the
  authorized facts to a machine-readable output channel when one is provided, and a human-readable summary always.
- **The authorized facts are the only channel** between authorization and every later step. Later steps do not
  re-read the graph to learn the version, the commit, or the identifiers.
- **Identifier lists** passed between steps are stable and readable, so that a provenance statement can quote them
  directly.
- **The harness is invoked as a Python module**, with the repository root as its target.
- **No interface of either Rust package is read, extended, or depended on.** This specification adds no Rust code.

## Security and privacy properties

- No secret is stored in the repository, per `docs/engineering/REPOSITORY_CONTEXT.md`. The process requires none
  beyond the platform's scoped credential.
- Write access is held by exactly one step, and only for attaching assets.
- The provenance statement is the file most likely to leak an environment value, and rule 11.3 forbids credentials,
  absolute local paths, and every environment-derived value except the build-run identifier. The same discipline
  `SPEC-MOK-003` imposes on the observer's footer applies here for the same reason.
- The process trusts no input about its own type or status: an artifact's declared type is read from the artifact,
  never inferred from the relation that pointed at it.
- A moved tag is treated as a refusal rather than a recoverable condition, so a tag cannot be used to redirect a
  release to an unauthorized commit.
- The gate performs no write of any kind, so running it cannot alter the repository it inspects.
- Dependency resolution honours the committed lockfile, so a release is not built from dependency versions the
  repository never recorded.

## Performance and capacity

- Authorization and reachability precede all compilation, so a refusal costs no build time.
- Compliance checks precede asset production, so a compliance failure costs no packaging.
- Builds run one per target and may proceed concurrently. No build shares compilation results with the checking
  steps.
- The full history is fetched, because tag objects and ancestry cannot be resolved from a shallow clone.
- No performance target is set for the process itself. Its cost is bounded by the workspace's own build and test
  cost, which no rule here changes.

## Observability

- The gate reports every fact it authorized on, so the run's own log states what the release is.
- The governance revision is logged with its hash, date and subject, so the revision the graph was read at is
  recoverable from the run.
- Each refusal is a single line naming the fact, distinguishable from tool output.
- The harness report is attached to the run.
- The provenance statement is printed as it is produced, so each archive's contents are visible in the run's log as
  well as in the archive.
- The compiler version comparison logs both versions when it refuses.

## Compatibility and migration

- The repository has never released. There is no prior release, no tag, no release contract and no release record,
  so nothing is migrated and no compatibility with an earlier process is owed.
- The first release is the first exercise of every rule here. Rule 4's refusal for "no release record" is the
  repository's current correct behavior and remains so until a release record exists.
- Adding a supported target is a change to rule 10 and requires an amendment.
- The six existing verification records are not edited or replaced. An aggregate release adds one record captured at
  the candidate commit; the six remain the record of their own work.
- The managed harness workflow is unaffected, and this specification introduces no managed-file change.
- Nothing here changes either package's public interface, so `SPEC-MOK-002` rule 5 and `SPEC-MOK-004` rule 6 remain
  satisfied by construction.

## Examples and counterexamples

**Conforming: the ordinary first release.** `RLS-MOK-001` is `released`, states version `0.1.0`, tag `v0.1.0` and
commit `C`; it satisfies `REL-MOK-001`, which is `approved` and gates `WO-MOK-001`..`006` and `VER-MOK-001`..`006`;
it releases those six work orders and includes `VREC-MOK-007`, which is `verified`, names `C`, and covers exactly
those six. `C` is an ancestor of the default branch. The annotated tag `v0.1.0` resolves to `C`. The release record
was committed after `C`, so the tree at `v0.1.0` does not contain it. The process authorizes, re-establishes
compliance at the governance revision and at `C`, builds three archives, and attaches them invisibly.

**Conforming: released from a maintenance branch.** Stabilization added two commits to `release/0.1`, so the
candidate is that branch's tip `S`. A verification record is captured at `S`, the release record names `S`, and the
tag resolves to `S`. Rule 5 matches `origin/release/0.1` and the release proceeds. `S` is not an ancestor of the
default branch, and that is not a defect.

**Counterexample: reading the graph from the tag.** A process that checks out the tag and runs the gate there finds
no release record, refuses with "no release record exists", and would refuse every genuine release. This violates
rule 1 and is the specific defect rule 1 exists to prevent.

**Counterexample: a moved tag.** The graph is untouched and correct; the tag was force-updated to a later commit
`D`. Rule 4.6 refuses. Re-running does not help, and it must not.

**Counterexample: creating the missing tag.** The requested tag does not exist. Creating it from the release
record's commit would make the run succeed and would violate rules 12.1 and 13.1. The conforming response is a
refusal.

**Counterexample: flipping the record on success.** Setting the release record to `released` after a successful
publish, or the work orders to `released`, violates rule 13.2 and inverts where authority sits.

**Counterexample: cutting the maintenance branch in the process.** Convenient, and wrong twice: the wrong actor,
per rule 13.4, and the wrong commit, because the branching model cuts it at stabilization rather than at
publication.

**Counterexample: reading the version from a manifest.** The manifests declare `0.1.0`; a release record declaring
`0.2.0` is the authority. Reading the manifest violates rule 11.1 and would name the archive after a version nobody
authorized.

**Counterexample: an abbreviated commit anywhere.** In the record, refused by rule 4.5. In the provenance
statement, a failure by rule 11.2. Expanding a short hash to make either pass is prohibited, because it resolves an
ambiguity belonging to whoever wrote it.

**Counterexample: refusing on validation warnings.** Rule 7.3 counts errors. The repository carries warnings and
four open amendment rows and is valid; refusing on warnings would refuse a release the repository considers sound
and would move a judgement out of the release contract.

**Counterexample: sharing a build cache between checking and building.** Rule 10.6 forbids it, because an object
compiled before a check could reach an archive the check is claimed to cover.

**Counterexample: a floating compiler channel.** Declaring `stable` satisfies the toolchain manager and violates
rule 9.1, because it resolves differently over time — which is exactly the instability the declaration removes.

## Explicitly unspecified decisions

- **Which release-notes wording is used**, beyond the facts rule 11.4 requires. The prose is the implementer's.
- **How the gate is implemented.** Language, structure and internal organization are unspecified; its command
  interface, its refusals and its outputs are not.
- **How targets are built** — which runner image, which build invocation flags beyond honouring the lockfile and the
  release configuration.
- **The archive container format.** Rule 10.4 constrains the name, not the format.
- **The checksum algorithm**, beyond being published and accepted unedited by a standard tool.
- **Whether an additional human approval before upload is required.** Rule 12.6 requires only that it be
  configurable.
- **Whether an architecture and a deciding record should govern the deployment and operating model.** A release and
  publication pipeline is an architecturally significant trigger, and no active architecture addresses any
  requirement this specification specifies. Whether one should exist is the technical owner's decision.
- **What the post-release observation window observes, and what rollback means.** Both belong to the release
  contract, not here.
- **Whether the first release proceeds with `WO-MOK-008` open.** Rule 10.7 records why the binaries carry no stamp;
  whether that is acceptable for a given release is recorded in that release's contract.
- **How many supported targets a release must have to be publishable.** Rule 10 lists three and rule 9.3 refuses a
  target whose compiler differs; whether a partial release is ever acceptable is a release-contract question.
