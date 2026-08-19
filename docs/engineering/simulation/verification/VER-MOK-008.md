+++
id = "VER-MOK-008"
type = "verification"
title = "Release authorization, compliance, provenance and reserved-act verification"
status = "approved"
owners = ["assurance owner"]
created = "2026-08-19"
updated = "2026-08-19"

[relations]
verifies = ["REQ-MOK-035", "REQ-MOK-036", "REQ-MOK-037", "REQ-MOK-038", "REQ-MOK-039"]
+++

# Verification Contract: Release authorization, compliance, provenance and reserved-act verification

## Independence

This contract has an independence problem the other verification contracts in this repository do not, and it has to
be addressed rather than asserted away.

Every other contract verifies behavior a test can observe from outside the thing under test. This one verifies a
**refusal mechanism**, and the tests of a refusal mechanism are written by whoever wrote the mechanism, from the same
understanding of what should be refused. If that understanding is wrong or incomplete, the mechanism and its tests
are wrong in the same direction and both pass. A green suite would then mean only that the gate does what its author
thought it should.

Four measures constrain that, in decreasing order of strength.

**1. The refusal set is enumerated by this contract, not by the implementation.** The scenario list below is derived
from `SPEC-MOK-005` rule 4, which is derived from `REQ-MOK-035`. An implementation that refuses fewer conditions
than this list fails; one that refuses more is not thereby conforming, because an unlisted refusal is an unspecified
behavior. Coverage is therefore measured against the contract, not against the code's branches, and the assurance
owner reads the list rather than a coverage percentage.

**2. One oracle is independent of both the gate and its tests: this repository, today.** As of 2026-08-19 there is
no release contract, no release record, and no aggregate verification record. The correct answer for every tag is a
refusal, and it stays the correct answer until a release record exists. That is a negative oracle nobody
constructed, running against the real artifact graph rather than a fixture, and it is available immediately.

**3. Fixture repositories are built with real git objects, not simulated ones.** Each scenario constructs a
throwaway repository with a real initialization, real commits, a real annotated tag and real `+++`-delimited
artifacts, and invokes the gate as a separate process. Nothing is stubbed, so a scenario cannot pass because a
stub agreed with the gate about what git would have said.

**4. Two independent evidence kinds cover the acts the gate does not perform.** `REQ-MOK-038` is largely a claim
about absence — no tag created, no status transitioned, no branch pushed — and absence is not something a passing
test demonstrates well. It is covered twice: by a static read of the process definition, enumerated in the static
checks below, and by inspecting the repository state before and after a run.

**What remains genuinely unverified is stated in Residual uncertainty** rather than dissolved here. In particular,
the person who reads a produced archive's provenance must not be the person who wrote the packaging step, and this
contract cannot enforce that; it can only require it and record it as an assumption.

## Requirement-to-evidence matrix

| Requirement | Claim | Evidence | Kind |
|---|---|---|---|
| `REQ-MOK-035` | authorizes a correct graph | scenarios A1–A3 | automated |
| `REQ-MOK-035` | refuses each enumerated missing graph fact | scenarios R1–R22 | automated |
| `REQ-MOK-035` | reports the facts it authorized on | scenario A4 | automated |
| `REQ-MOK-035` | reads the graph at the governance revision | scenario A2 | automated |
| `REQ-MOK-035` | refuses a commit not on a release-bearing branch | scenarios R23–R24 | automated |
| `REQ-MOK-035` | refuses in this repository, today | scenario A5 | automated |
| `REQ-MOK-035` | writes nothing | scenario P4 | automated |
| `REQ-MOK-036` | harness version equality is established first | scenario C1 | manual, on a run |
| `REQ-MOK-036` | graph valid and preflights pass at the governance revision | scenario C2 | automated, on a run |
| `REQ-MOK-036` | declared checks pass at the authorized commit | scenario C3 | automated, on a run |
| `REQ-MOK-036` | a failing check publishes nothing | scenario C4 | manual, rehearsed |
| `REQ-MOK-036` | warnings do not refuse | scenario C5 | automated |
| `REQ-MOK-036` | no check writes to the tracked tree | scenario S6 | static |
| `REQ-MOK-037` | every required provenance line is present and correct | scenario V1 | manual, independent reader |
| `REQ-MOK-037` | the commit is complete and equals the record's | scenario V2 | manual, independent reader |
| `REQ-MOK-037` | a checksum verifies with a standard tool | scenario V3 | manual, independent reader |
| `REQ-MOK-037` | notes and archive agree | scenario V4 | manual, independent reader |
| `REQ-MOK-037` | no credential and no absolute local path appear | scenarios V5, S5 | manual and static |
| `REQ-MOK-037` | two builds of one commit differ only by run identity | scenario V6 | manual, two runs |
| `REQ-MOK-038` | the process performs no reserved act | scenarios S1–S4 | static |
| `REQ-MOK-038` | write access is confined to the attaching step | scenario S2 | static |
| `REQ-MOK-038` | a refused run leaves no trace | scenario P1 | automated, on a run |
| `REQ-MOK-038` | an authorized run leaves the release invisible | scenario P2 | manual, on a run |
| `REQ-MOK-038` | no status changes across a successful run | scenario P3 | automated, on a run |
| `REQ-MOK-038` | the human sequence is documented and disclaims authority | scenario M1 | manual |
| `REQ-MOK-039` | a clone selects the declared version | scenario T1 | automated |
| `REQ-MOK-039` | a matching version passes | scenario T2 | automated, on a run |
| `REQ-MOK-039` | a differing version refuses, in both directions | scenarios T3–T4 | automated |
| `REQ-MOK-039` | the declaration has exactly one home | scenario S7 | static |
| `REQ-MOK-039` | the recorded version equals the declared one | scenario V1 | manual, independent reader |
| `REQ-MOK-039` | components are available without separate installation | scenario T5 | automated |

Every requirement carries at least one automated row and at least one row whose evidence is not produced by the
implementation's own test suite.

## Acceptance scenarios

Scenarios prefixed **A** and **R** run against synthetic fixture repositories and are executable at any time.
Scenarios prefixed **C**, **P** and **V** require a release run. Scenarios prefixed **T** concern the toolchain.

### Authorizing scenarios

**A1 — a correct graph authorizes.** A fixture with a `released` release record naming the tagged commit, an active
contract gating the released work, releasable work orders, and a `verified` verification record naming the same
commit and covering exactly that work. The gate exits successfully.

**A2 — the record lives in a later commit than the one it names.** Construct A1's graph, then commit it, so the
tagged tree provably does not contain the release record — asserted by confirming the record's path is absent from
the tagged tree. The gate must still authorize. This is the normal order of events, and a gate that fails here
refuses every genuine release while appearing to work in every other scenario.

**A3 — a record that states no tag.** The record omits the tag field and states a version whose conventional tag
name is the requested tag. The gate authorizes.

**A4 — the authorized facts are reported.** For A1, the gate's output names the version, the release record, the
release contract, each released work order, each included verification record, and the full commit.

**A5 — this repository refuses, today.** Run the gate against the real repository for any tag. It refuses, naming
the absent release record. Re-run after each of the governance artifacts is added, and record the refusal that
remains at each step; the last refusal to disappear is the release record's transition to `released`.

### Refusal scenarios

Each constructs A1's graph and breaks exactly one fact. Each must exit failing, with a refusal naming that fact.

| # | Broken fact | Specification rule |
|---|---|---|
| R1 | the requested tag does not exist | 4.1 |
| R2 | the requested name is a branch, not a tag | 4.1 |
| R3 | the tag is lightweight rather than annotated | 3 |
| R4 | two artifacts share an identifier | 4.2 |
| R5 | the repository does not require full commits | 4.3 |
| R6 | no release record exists at all | 4.4 |
| R7 | the release record is `ready`, not `released` | 4.4 |
| R8 | two `released` records claim the same tag | 4.4 |
| R9 | the record's commit is abbreviated | 4.5 |
| R10 | the record's commit is an unrelated commit | 4.6 |
| R11 | the tag was force-moved after the record was written | 4.6 |
| R12 | the gating contract does not exist | 4.7 |
| R13 | the gating contract's declared type is not a release contract | 4.7 |
| R14 | the gating contract is `draft` | 4.7 |
| R15 | the contract does not gate a released work order | 4.8 |
| R16 | a released work order is `approved`, not releasable | 4.9 |
| R17 | an included verification record does not exist | 4.10 |
| R18 | an included verification record is `ready` | 4.10 |
| R19 | an included verification record names another commit | 4.10 |
| R20 | work is released but not verified | 4.11 |
| R21 | work is verified but not released | 4.11 |
| R22 | the record's version is not usable as a name | 4.12 |
| R23 | the authorized commit is reachable only from a feature branch | 5 |
| R24 | the authorized commit is reachable from no remote branch | 5 |

R23 and R24 are reachability scenarios and may be exercised against the process rather than the gate, since rule 5
places them outside the gate so the gate stays runnable in a clone with no remote. Whichever component carries them,
both must refuse.

Each refusal is asserted on two things: a failing exit status, and a message containing the named fact. Asserting
only the status would pass a gate that refuses everything for one reason.

### Compliance scenarios

**C1 — harness version equality precedes every harness command.** On a run, the log shows the installed version
compared to the declared version before `doctor`, `validate` or any preflight runs. Rehearse the failure by
requesting a run with a deliberately mismatched declared version and confirm it refuses naming both versions,
before any harness command.

**C2 — the graph is checked at the governance revision.** On a run, the logged governance revision equals the tip of
the default branch at the time the run started, the validation reports zero errors, and one review preflight is
logged per released work order, each naming its work order.

**C3 — the declared checks pass at the authorized commit.** On a run, formatting, lint, test, dependency-tree and
determinism steps all run at the authorized commit and pass. The dependency-tree step reports exactly one crate for
the engine package. The determinism step compares two runs per deterministic policy and reports byte identity,
identical final state and identical exit code.

**C4 — a failing check publishes nothing.** Rehearsed rather than performed on a real release: on a branch, break
one declared check at the candidate commit, run the process, and confirm it refuses and that no release object and
no asset exist afterwards.

**C5 — warnings do not refuse.** The real repository currently reports zero errors. Rule 7.3 requires the validation
step's exit code to be the only thing read, so a run against a repository that reports warnings must still treat
validation as passing. This is checked by asserting the validation step's success while asserting nothing whatever
about its reported warning count. The absence of any predicate on the count is what establishes that the count is
unread; a scenario requiring a non-zero count would be unsatisfiable against a conformant repository, whose warning
count may legitimately be zero — as this one's is.

### Provenance scenarios

These are performed by someone who did not write the packaging step. See Residual uncertainty.

**V1 — every required line is present and correct.** Download one archive per target. Each contains the provenance
statement at the specified path, and each of the ten required lines is present and non-empty. The version equals the
release record's version. The compiler line equals the version declared at the authorized commit. The target line
equals the target the archive is named for.

**V2 — the commit is complete and matches.** The commit line is compared character-by-character to the release
record's commit. Equal over the full length. A truncated value fails even if its prefix matches.

**V3 — a checksum verifies.** Each archive's published checksum verifies against the archive with a standard tool,
unedited.

**V4 — the notes agree with the archive.** The release notes and any archive's provenance statement state the same
commit, version, release record, release contract, released work orders and verification records.

**V5 — no credential and no local path.** The provenance statement contains no token-like value, no credential, and
no absolute path from the building machine. Checked by reading the file, and by searching it for the build
environment's home-directory and workspace path prefixes.

**V6 — two builds differ only by run identity.** Request a second run for the same tag. The two provenance
statements for the same target differ on the build-run line and nowhere else.

### Persistence and reserved-act scenarios

**P1 — a refused run leaves no trace.** Record the remote's tags, branches, and every artifact's status before a
refused run; compare after. Identical. No release object exists.

**P2 — an authorized run leaves the release invisible.** After a successful run, the release exists, holds every
archive and checksum, and is not publicly visible. The tag resolves to the same commit as before the run.

**P3 — no status changes across a successful run.** Every artifact's status before the run equals its status after.
Checked mechanically over the artifact root at the governance revision.

**P4 — the gate writes nothing.** Run the gate against a clean checkout of the real repository and confirm the
working tree, including untracked files, is unchanged afterwards, and that no tag or branch was created.

### Toolchain scenarios

**T1 — a clone selects the declared version.** In a fresh clone, query the active toolchain. It reports the declared
version and identifies the repository's declaration as the reason.

**T2 — a matching version passes.** On a run, each verifying and building step logs the compiler version and the
declared version and proceeds.

**T3 — a newer version refuses.** With a declared version below the available compiler, the step refuses, naming
both, before compiling.

**T4 — an older version refuses.** With a declared version above the available compiler, the step refuses on the
same comparison, demonstrating the check is equality and not a minimum.

**T5 — the declared components are present.** In a fresh clone with no separate component installation, the declared
formatting and lint checks both run.

## Property and invariant tests

- **Refusal totality.** For every refusal scenario, no asset exists and the repository is unchanged. The property
  holds across the whole refusal set, not per scenario.
- **Authorization is a conjunction.** For any single broken fact from the enumerated set, the gate refuses. Verified
  by construction: each refusal scenario breaks exactly one fact from a graph that otherwise authorizes, so no
  scenario can pass for an unrelated reason.
- **Commit equality is total, not prefix.** For a record commit that is a strict prefix of the tagged commit, the
  gate refuses. Together with R9 this establishes that no abbreviation is ever accepted.
- **Set equality between released and verified work is symmetric.** R20 and R21 exercise both directions, so the
  check is equality rather than containment in one direction.
- **Type is read, never inferred.** For each relation the gate follows, an artifact of the wrong declared type at
  the target refuses. R13 exercises this for the contract; the property extends to work orders and verification
  records.
- **Idempotence of the gate.** Running the gate twice on an unchanged repository produces the same outcome and the
  same reported facts.
- **The governance revision is fixed.** Within one run, every step that reads the graph reads the same commit hash.
  Verified by comparing the hash logged by the pinning step against the hash each later step checked out.
- **Determinism of the provenance statement.** For one commit and one target, the statement is a function of the
  authorized facts, the target and the compiler, plus the run identifier. V6 exercises it; the property is that no
  other input reaches the file.

## Static and architecture checks

**S1 — no tag, branch, commit or push.** Read the process definition. It contains no tag creation, deletion or
update; no branch creation; no commit; and no push. Enumerated by inspection of every command the process runs,
including every failure path and cleanup step.

**S2 — write access is confined.** Every step's granted access is read. Exactly one step holds write access, and it
is the step that attaches assets.

**S3 — no artifact is written.** No step writes, edits, deletes or commits a file under the artifact root, and no
step changes a status field.

**S4 — no version is changed.** No step modifies a package manifest or the lockfile. The lockfile is honoured and
not updated wherever dependencies are resolved.

**S5 — no secret is required or emitted.** The process references no secret beyond the platform's scoped credential,
and the provenance statement's construction reads no environment value other than the run identity.

**S6 — no check writes to the tracked tree.** The formatting check reports rather than reformats; dependency
resolution honours the lockfile rather than updating it.

**S7 — the declared compiler version has one home.** Searching the repository for the declared version finds it as a
value in exactly one tracked file, and every step reads it from that file rather than repeating it.

**S8 — the managed harness workflow is untouched.** Its hash matches the recorded one, and the process neither
invokes nor modifies it.

**S9 — the engine's dependency table is unchanged.** No rule of this work adds a dependency to either package, and
the engine's table remains empty.

**S10 — the human procedure lives outside the artifact root.** The documented sequence is not under the governed
artifact root, so it cannot be mistaken for an artifact carrying authority.

**S11 — the process definition parses.** The process definition is parsed by a general parser for its format and
declares the expected steps and their declared revisions: authorization at the default branch, compliance at the
pinned governance revision, checks and builds at the authorized commit.

## Security and privacy checks

- **No secret in the repository.** The repository contains no credential, confirmed by S5 and by the existing
  restricted-path constraint.
- **The provenance statement leaks nothing.** V5 and S5. This is the file whose purpose is to describe the build
  machine, so it is the file most likely to carry a token or a home directory.
- **A moved tag cannot redirect a release.** R11. The graph looks entirely correct in this scenario, which is why it
  is the most important security check here rather than the most obscure.
- **A branch cannot impersonate a tag.** R2. Tag resolution is confined to the tag namespace.
- **A feature branch cannot be released.** R23. Otherwise any commit anyone pushed could be released given a tag.
- **An artifact cannot lie about its type.** R13 and the type-is-read property.
- **The gate cannot alter what it inspects.** P4.
- **Only one step can write.** S2, so a defect in a check cannot produce a repository or publication change.
- **Assets are not published by automation.** P2 confirms invisibility, which bounds the consequence of every other
  failure to something a person must still confirm.

## Performance and resilience checks

- **A refusal costs no compilation.** For each refusal scenario run against the process, no build step executes.
  Checked by confirming the build steps did not start.
- **A compliance failure costs no packaging.** C4 confirms no asset exists.
- **A concurrent push cannot change the answer mid-run.** Push a commit to the default branch while a run is in
  flight; every step still reads the pinned governance revision. Verified by the fixed-revision property rather than
  by racing, since a race is not reliably reproducible.
- **The full history is available.** Ancestry and tag-object checks succeed, which they cannot on a shallow clone.
- **No performance target is asserted for the process.** Its cost is the workspace's own build and test cost, which
  this work does not change.

## Manual assessments

**M1 — the human sequence.** The assurance owner reads the documented procedure and confirms it states the ordered
acts, names an accountable role for each, states what must hold before each, and states that it is not authority and
does not override a governed artifact.

**M2 — the archive, read by someone who did not build it.** V1 through V6 are performed by a person other than the
implementer. This is the contract's principal independence measure for `REQ-MOK-037` and it is a procedural
requirement, not a technical one.

**M3 — the refusal set is complete.** The assurance owner compares the refusal scenario table against
`SPEC-MOK-005` rule 4 and rule 5 and confirms every enumerated fact has a scenario. A missing row is a gap in this
contract, not in the implementation.

**M4 — the release contract's judgements.** Before a release, the assurance owner confirms that the four open
amendment rows and the open `WO-MOK-008` defect have been judged in the release contract, since rule 7.3
deliberately does not refuse on them.

**M5 — the two revisions were not conflated.** The assurance owner reads the run's log and confirms which revision
each step ran at. A2 makes this mechanical for the gate; M5 covers the process as a whole, because the failure is
easy to introduce and invisible when both revisions coincide in a test.

## Evidence retention

- The fixture scenario suite is committed to the repository and runnable by hand, so any scenario can be re-executed
  at any commit.
- Each release run's log is retained by the platform and is the evidence for the C, P and T scenarios. The
  provenance statement's build-run line is what links an archive back to its run.
- The harness report is attached to each run.
- The A5 refusal ladder — which refusal remained after each governance artifact was added — is recorded as evidence,
  because it is the one measurement that demonstrates the gate discriminating between adjacent incomplete states
  of the real repository rather than of a fixture.
- V1 through V6 are recorded as a written observation naming the person who performed them, the archive checked, the
  checksum verified, and the commit compared.
- Static check results S1 through S11 are recorded as a written enumeration, since a static read leaves no artifact
  of its own.

## Residual uncertainty

- **The gate and its scenario suite share an author.** Measures 1 through 4 in the Independence section constrain
  this; they do not eliminate it. A fact that neither `SPEC-MOK-005` nor the implementer thought of is refused by
  neither and is not detected by this contract.
- **M2's independence cannot be enforced by this contract.** It is a procedural requirement on who performs the
  provenance reading. If the same person writes the packaging and reads the archive, the strongest evidence for
  `REQ-MOK-037` degrades to a self-check, and this contract's only recourse is that the recorded observation names
  the person.
- **The first release exercises every scenario for the first time.** No scenario here has ever run against a real
  release, because the repository has never released. Until one has, C, P and V evidence is rehearsed rather than
  observed, and the difference is real.
- **C4 and T3–T4 are rehearsed by deliberate breakage.** A deliberately broken check is not identical to a
  spontaneously failing one, and a rehearsal on a branch is not a rehearsal on the release path.
- **Absence is verified statically.** S1 through S4 establish that the process definition contains no reserved act.
  They do not establish that no tool the process invokes performs one on its behalf. P1 and P3 bound this
  observationally for the runs that occur, not universally.
- **The determinism comparison is two runs on one platform per deterministic policy.** It does not establish
  cross-platform determinism, and `INT-MOK-007` explicitly does not claim bit-identical builds across machines.
- **A checksum is not a signature.** V3 establishes that the bytes match the published checksum. It establishes
  nothing about who published either, and signing is out of scope by `INT-MOK-007`.
- **The binaries state no commit.** By `SPEC-MOK-005` rule 10.7, provenance is beside the binaries rather than
  inside them. A binary extracted from its archive and separated from the provenance statement is unidentifiable,
  and this contract does not close that gap. `WO-MOK-008` owns it.
- **The reachability check accepts any maintenance branch present on the remote.** An abandoned release line remains
  release-bearing, and no scenario here refuses that, because `REQ-MOK-035` leaves it deliberately permissive.
- **Nothing verifies that the compiler declared today is the compiler the six existing verification records were
  produced under.** The declaration was introduced after them, and the artifact graph carries no field recording a
  compiler per record. `REQ-MOK-039` names this as an open decision and this contract cannot resolve it.
