# Rule-by-rule conformance — what the candidate satisfied, and what changed

`WO-MOK-009`'s Lifecycle section requires this: *"Approval does not adopt the candidate. It authorizes
implementing `SPEC-MOK-005`. Where the candidate conforms, it may be used; where it does not, it changes.
The specification is the authority in every case, and the completion report states, rule by rule, what
the candidate already satisfied and what had to change."*

Captured 2026-08-19 in a linked worktree of the primary clone, branch `feature/release-ci`, HEAD
`54c21abcfb9caa4474c9ca5f194289e055c86a23`.

## What "the candidate" means here, and why this is not a diff

The candidate was never committed in its pre-conformance state. At capture, `feature/release-ci` held no
commit beyond `origin/master` (`git log origin/master..HEAD` was empty) and the whole changeset — candidate
and corrections alike — was uncommitted working-tree state. So this file cannot be, and is not, a mechanical
diff between a candidate revision and a conformant one. **It is a reconstruction from the change record kept
while the work was done**, and a reader should weigh it as that: a statement of what was changed and why,
not a machine-checkable comparison.

**The commit the owner directed afterwards does not change this**, and it is worth saying so where a reader
will look for it. `17be4bad444a4199da53e72ae8be491ba5f46ee1` adds all 34 files at once, so `git show`
renders it as an addition rather than a before-and-after; no revision in this repository holds the candidate
in its earlier form. `commit-binding.md` records the commit and this consequence.

That is a real weakness in this evidence and it is worth being plain about. The mitigation is that every
*current* claim below is checkable against the files as they stand — `static-checks.md` re-derives the
static ones from the frozen definition, `suite-output.md` re-derives the scenario ones — and only the
"before" column rests on the record rather than on an artifact.

## Summary

`SPEC-MOK-005` states 61 countable rules: six with no sub-rules (1, 2, 3, 5, 6, 14) and 55 sub-rules
(4.1–4.12, 7.1–7.5, 8.1–8.5, 9.1–9.6, 10.1–10.7, 11.1–11.5, 12.1–12.6, 13.1–13.9). Against that:

| Kind | Count |
| --- | --- |
| Rules the candidate satisfied unchanged | 50 |
| Rules that required a change | 10, by way of 12 distinct changes (4.2 accounts for three) |
| Rules satisfied by amending the rule rather than the implementation | 1 — 12.5, amended 2026-08-19 |
| Of the above, rules satisfied in the definition but not on a produced archive | 7 — 10.1–10.6 and 11.3 |
| Divergences deliberately left in place | 2 |
| Changes outside this work order's declared change surface | 1 |

The fourth row overlaps the first two rather than adding to them: 10.5 and 10.7 are among the changed
rules, and the rest are unchanged. It is listed because "satisfied" means something weaker for those seven
than for the other 54 — the definition says the right thing, and no one has yet read a produced archive.
`scenario-map.md` records V1 through V6 as not performed for exactly this reason.

The twelve changes, ordered by how badly the unchanged form would have failed:

| # | Rule | The candidate did | Severity |
| --- | --- | --- | --- |
| 1 | 1 | read the artifact graph from the **tagged tree** | **would have refused every genuine release** |
| 2 | 4.2 | skip artifacts whose front matter does not parse | **fail-open** |
| 3 | 4.3 | proceed leniently when `require_full_commit = false` | **fail-open** |
| 4 | 2 | skip `publish` entirely on the explicit trigger | conformance gap |
| 5 | 4.2 | walk directories the validator excludes | conformance gap |
| 6 | 4.2 | hard-code the artifact root | conformance gap |
| 7 | 3 | check the tag object only in the workflow, not in the gate | testability gap |
| 8 | 7.1 | refuse without naming both versions | refusal-message gap |
| 9 | 7.4 | refuse without naming the work order | refusal-message gap |
| 10 | 8.5 | make one comparison where the rule asks for three | under-check |
| 11 | 10.5 | assume `sha256sum` exists on every runner | portability defect |
| 12 | 10.7 | not set the commit stamp, without asserting it is unset | unchecked assumption |

Numbers 2 and 3 are the two that matter most after number 1, because both fail **open**: a damaged
release record produced the message *"no release record exists at all"*, and a repository that permits
abbreviated commits got a lenient gate rather than a refusal. A fail-open defect in an authorization gate
is worse than a noisy one, and neither was visible from a passing suite.

## Rule 1 — two revisions, and which is read for what

**Changed, and this is the headline change.** The candidate's `authorize` job checked out the requested
tag and ran the gate there. The tagged tree cannot contain the release record that authorizes it — the
record states the tagged commit's hash, so it is written afterwards — so the gate found no release record
and refused. It would have refused **every** genuine release, with a message that reads like a governance
failure rather than a defect. This is the exact failure rule 1 exists to prevent, and `WO-MOK-009` records
it as the worked example of why scenario A2 is mandatory.

Now: `authorize` checks out `${{ github.event.repository.default_branch }}`, verifies `git rev-parse HEAD`
equals `git rev-parse "origin/$default"` before recording anything, emits the governance commit, and
`harness` checks out that fixed value and re-asserts it. `verify` and `build` check out the authorized
commit and re-assert that. `static-checks.md` S11 records the three declared refs and the three run-time
assertions; `a2-transcript.md` records the gate authorizing a fixture in which the record's path is
provably absent from the tagged tree.

The candidate's own suite passed while this defect was present, because its fixture wrote artifacts into
the working tree at the tagged commit, so the two revisions were the same object. That is why A2 asserts
the record's *absence from the tagged tree* rather than assuming it.

## Rule 2 — trigger and requested tag

**Changed.** The candidate carried `if: github.event_name == 'push'` on `publish`, so an explicit
`workflow_dispatch` run performed every check and then published nothing. Rule 2 says the explicit form
*"re-runs a release; it does not preview one"*, so the skip was a conformance gap rather than a safety
measure.

Now `publish` carries no `if:` at all — no job does — and the attaching step takes one of two mutually
exclusive paths: `gh release create --verify-tag --draft` when no release exists for the tag, and
`gh release edit --notes-file` plus `gh release upload --clobber` when one does. Exactly one path runs,
and attachment is a single call on either path, which is what rule 12.3 requires. `static-checks.md` S1
records the five `gh` calls and which of them mutate.

Satisfied unchanged: the tag name is resolved once, at workflow level, into `RELEASE_TAG`
(`${{ inputs.tag || github.ref_name }}`) and used unchanged thereafter; the `push` trigger is filtered to
`tags: v*`; an explicit request naming a tag not on the remote reaches rule 4.1 and refuses there.

## Rule 3 — the tag must be annotated

**Changed, for testability rather than for behavior.** The candidate checked the tag object's type in the
workflow only. The behavior was right; the check was unreachable from a fixture, so `VER-MOK-008` R3 could
not be exercised without a runner. The check moved into the gate — `git cat-file -t refs/tags/<tag>`, at
`scripts/check_release_authorization.py:218` — and R3 is now
`test_r3_the_tag_is_lightweight_rather_than_annotated`.

This is the pattern `WO-MOK-009`'s decision envelope permits: *"how the gate is implemented"* is the
implementer's, and moving a check into the component a fixture can drive is an implementation choice, not
a change to which facts authorize a release.

## Rule 4 — the authorization gate

| Rule | Status | Note |
| --- | --- | --- |
| 4.1 the tag resolves | **satisfied** | `git rev-parse --verify --quiet refs/tags/<tag>^{commit}` (`check_release_authorization.py:205`), confined to the tag namespace, so R2's branch cannot impersonate a tag |
| 4.2 the graph is unambiguous | **changed, three times** | see below |
| 4.3 the repository requires full commits | **changed** | see below |
| 4.4 exactly one release record claims the tag | **satisfied** | including the conventional-tag fallback, zero → "no release record", two → ambiguous |
| 4.5 the record's commit is complete | **satisfied** | lowercase hex, length from the declared object format, never expanded |
| 4.6 the record's commit equals the tagged commit | **satisfied** | full-length character comparison; R11's moved tag refuses |
| 4.7 the gating contract is present, typed and active | **satisfied** | the target's *declared* type is read, never the relation's name |
| 4.8 the contract gates all released work | **satisfied** | ungated work is named in the refusal |
| 4.9 released work exists and is releasable | **satisfied** | `implemented`, `verified`, `released` |
| 4.10 included verification is present, eligible, commit-bound | **satisfied** | and it is the rung that refuses this repository today |
| 4.11 released and verified work agree exactly | **satisfied** | both directions, both naming the offending identifiers |
| 4.12 the version is usable | **satisfied** | the regex, unchanged |

**4.2, change one — malformed front matter was skipped.** The candidate treated an artifact whose `+++`
block does not parse as an artifact that is not there. A damaged release record therefore produced *"no
release record exists at all"*, which is a **fail-open** result dressed as a refusal: the refusal names
the wrong fact, and a reader would go add a release record that already exists. `SPEC-MOK-005`'s Error and
recovery behavior is explicit — *"an unreadable artifact, malformed front matter … each is a refusal,
never a pass"* — so unparseable front matter is now a refusal naming the file.
`test_refuses_front_matter_that_does_not_parse` and `test_refuses_front_matter_that_is_never_closed` cover
both forms. `suite-output.md` records why every refusal test asserts the message and not only the status:
this is the defect that behavior would hide.

**4.2, change two — excluded directories were walked.** The candidate walked every directory under the
artifact root. Rule 4.2 says *"Directories the validator excludes are excluded here too, so a template is
not mistaken for an artifact"*, and the validator excludes `templates`, `evidence`, `.git`, `.idea`,
`target` and `node_modules`. Without the exclusion, `docs/engineering/templates/RELEASE_RECORD.template.md`
is a release record, and the duplicate-identifier check in 4.2 fires against retained evidence.
`test_ignores_files_under_an_excluded_directory` covers it.

**4.2, change three — the artifact root was hard-coded.** The candidate assumed `docs/engineering`. The
Inputs table names the harness configuration as the source of the artifact root, so the gate now reads
`[harness].artifact_root` and refuses when it is absent, unreadable, or points at a directory that does
not exist. Three tests cover those: `test_reads_the_declared_artifact_root_rather_than_assuming_one`,
`test_refuses_when_the_declared_artifact_root_is_missing`,
`test_refuses_when_the_configuration_is_unreadable`.

**4.3 — a lenient path where a refusal belongs.** The candidate read
`[revision_provenance].require_full_commit` and, when it was `false`, proceeded with a relaxed comparison.
Rule 4.3 says the opposite: *"The gate's central check is equality of complete hashes and it does not run
where abbreviations are permitted."* A repository that permits abbreviations gets a refusal, not a
tolerant gate. This is the second **fail-open** defect and it is subtler than the first, because the
configuration it trusts is the one that would make its central check meaningless.
`test_r5_the_repository_does_not_require_full_commits` covers it.

**The closing paragraph — reporting.** Satisfied unchanged: on success the gate writes the authorized
facts to `$GITHUB_OUTPUT` when one is provided and a human-readable summary always. Both are tested
separately (`test_a4_the_authorized_facts_are_reported`,
`test_a4_the_authorized_facts_are_emitted_for_the_workflow`), because the machine channel is what every
later step reads and a summary that is right while the channel is wrong would pass a single test.

## Rule 5 — reachability

**Satisfied**, and unchanged in substance. The check is a separate program
(`scripts/check_release_reachability.py`) precisely for the reason rule 5 gives — *"the gate must remain
runnable in a clone with no remote"* — and it enumerates maintenance branches from
`refs/remotes/<remote>/release/*` rather than assuming a set. R23 and R24 both refuse; so does a *local*
`release/0.1` that the remote does not carry, which rule 5's *"present on the remote"* requires and which
`scenario-map.md` lists among the cases beyond the enumeration.

## Rule 6 — order of establishment

**Satisfied**, with one reading recorded rather than assumed. `authorize` has no `needs`, so it runs
first; `build` needs `[authorize, harness, verify]`, so no compilation happens until authorization,
reachability, harness compliance and the declared checks have all passed; `publish` needs all four.

The reading: `verify` runs **parallel to** `harness` rather than after it. Rule 6 says *"Compliance
re-establishment runs before any asset is produced"* — before asset production, not before the rule 8
checks — and `build` is where assets are produced and it needs both. So the parallelism conforms. It is
recorded here because a reader checking rule 6 against the job graph will see `verify` and `harness` start
together and should not have to re-derive why that is admitted.

## Rule 7 — harness compliance at the governance revision

| Rule | Status |
| --- | --- |
| 7.1 version equality first, naming both versions | **changed** |
| 7.2 the repository check passes | **satisfied** |
| 7.3 validation reports zero errors; warnings do not refuse | **satisfied** |
| 7.4 review preflight per released work order, failure names it | **changed** |
| 7.5 the harness report is attached | **satisfied** |

**7.1 — the refusal did not name both versions.** The candidate compared the declared and installed
versions and refused, but the message named neither. Rule 7.1 requires *"naming both versions"*. The step
now prints `repository declares $declared; runner has $installed` on every run and, on a mismatch,
`REFUSED: this repository declares harness $declared, but the runner has $installed.` A declaration that
cannot be read fails before any harness command, which is the fail-closed reading.

This rule earned its keep during the work rather than in a rehearsal. The machine's harness moved from
`0.4.0` to `0.4.1` mid-session while the repository declared `0.4.0`, and the predicate caught it. Run
`doctor` on the mismatched build instead and you get eight `FAIL distribution:` lines that read like
managed-file damage and are not. `compliance-rehearsal.md` C1 records both runs.

**7.4 — the refusal named no work order.** The candidate's loop set a flag and, at the end, reported *"at
least one released work order does not pass review preflight"*. Rule 7.4 says *"A failure names the work
order."* The loop now collects failures and names every one:
`REFUSED: review preflight does not pass for WO-MOK-001 WO-MOK-002 …`. It also keeps going after the first
failure rather than stopping, because a reader fixing this wants the whole list rather than the first
name. `compliance-rehearsal.md` C2 records both the passing and the refusing form.

Satisfied unchanged: 7.2 is `python -m se_harness doctor .`; 7.3 is
`python scripts/validate_engineering_artifacts.py --root .`, whose exit code is the only thing read, so
warnings cannot refuse; 7.5 uploads `target/harness-dashboard` with `if: always()`. Harness commands are
invoked as a Python module throughout — the candidate's header comment said `harnessctl doctor`, which is
not on this repository's executable search path, and was corrected to `python -m se_harness doctor`.

## Rule 8 — repository checks at the authorized commit

| Rule | Status | Command |
| --- | --- | --- |
| 8.1 formatting | **satisfied** | `cargo fmt --all -- --check` (line 289) |
| 8.2 lint, all targets, all features, warnings as errors, from the lockfile | **satisfied, stricter than the declaration** | line 292 |
| 8.3 tests across the workspace from the lockfile | **satisfied** | line 295 |
| 8.4 the engine's tree resolves to exactly one crate | **satisfied** | line 303, with a line-count assertion |
| 8.5 two runs per deterministic policy, three comparisons | **changed** | lines 331–350 |

**8.5 — one comparison where the rule asks for three.** Rule 8.5 asks for *"byte-identical output, an
identical final state and an identical exit code"*. The candidate compared the byte streams only, leaving
the exit code to `set -e` and the final state implicit. The step now captures each run's status into a
file and compares those too, and prints the trailing `summary` line — which *is* the final state, and is
inside the byte comparison — so a reader sees all three rather than inferring two.

**The 8.2 divergence, deliberately left in place.** `docs/engineering/REPOSITORY_CONTEXT.md` declares the
lint command *without* `--locked`, while rule 8.2 requires resolution from the committed lockfile. The
workflow adds the flag. `--locked` is strictly stricter — it makes cargo fail rather than update
`Cargo.lock`, so the declared command's outcome is unchanged whenever the lockfile is already current —
and adding it to the *declaration* would be an edit to `REPOSITORY_CONTEXT.md`'s commands section beyond
what `WO-MOK-009` names, so it was left alone and recorded. `static-checks.md` S4 holds the same note.
Closing it is a one-line change to the declaration and belongs to whoever next opens that file.

Satisfied unchanged: *"No check may modify a tracked file."* `fmt --check` reports rather than reformats;
`--locked` prevents a lockfile rewrite; the determinism transcripts are written to `$RUNNER_TEMP`, outside
the checkout, and the step ends with a logged `git status --porcelain --untracked-files=all` so a
regression shows in the transcript. `determinism-rehearsal.md` measured `git status` identical before and
after.

## Rule 9 — the compiler declaration

| Rule | Status | Note |
| --- | --- | --- |
| 9.1 one exact version in one tracked file, with components | **satisfied** | `rust-toolchain.toml:31`, `channel = "1.97.1"`, with `rustfmt` and `clippy` |
| 9.2 a clone honours it on first invocation | **satisfied** | `toolchain-evidence.md` T1, with the caveat that the pristine clone did not yet carry the file and it was placed by hand |
| 9.3 every verifying and building step compares, refusing either way, before compiling | **satisfied** | lines 275 and 398, each before its first `cargo` call |
| 9.4 not duplicated into the process definition | **satisfied** | `static-checks.md` S7: exactly one occurrence as a value, and both steps `sed` it out of the file |
| 9.5 exact, not a minimum | **satisfied** | `toolchain-evidence.md` T3 and T4 refuse in both directions |
| 9.6 changing it is authorized work | **satisfied by construction** | the file is tracked and this work order is what introduces it |

The declaration is new in this changeset, so there is no candidate behavior to compare against except
`rust-toolchain.toml`'s absence. `REPOSITORY_CONTEXT.md`'s setup line was updated to say the pin selects
the toolchain and that no toolchain should be installed by hand or overridden with `+stable`, which is
inside `WO-MOK-009`'s named change surface for that file.

## Rule 10 — asset composition

| Rule | Status | Note |
| --- | --- | --- |
| 10.1 both binaries, release configuration, per target, from the lockfile | **satisfied in the definition** | `cargo build --release --locked -p Mokiterions -p mokiterions-tui` (line 436) |
| 10.2 licence, readme, simulation-rules document | **satisfied in the definition** | staged alongside the binaries |
| 10.3 a provenance statement at a predictable path | **satisfied in the definition** | `dist/$stage/PROVENANCE.txt` |
| 10.4 the archive name states version and target | **satisfied in the definition** | |
| 10.5 a checksum beside each archive, standard tool, unedited | **changed** | see below |
| 10.6 no compilation result carried from rule 8 to rule 10 | **satisfied by construction** | separate jobs, separate runners, no cache action anywhere in the definition |
| 10.7 no commit stamp compiled in | **changed** | see below |

Rules 10.1 through 10.5 are where *satisfaction* means something weaker than elsewhere in this file,
because what they constrain is a produced archive and no archive has been produced. What is established is
that the definition says the right thing. 10.6 and 10.7 are different: 10.6 holds by construction — the
jobs are separate, the runners are separate, and no cache action appears anywhere in the definition — and
10.7 is a guard whose presence is a static fact. `scenario-map.md` records V1 through V6 as not performed
for this reason, and M2 requires that they be performed by someone who did not write the packaging step.

**10.5 — `sha256sum` is not guaranteed on a macOS runner.** The candidate called `sha256sum`
unconditionally. macOS images carry `shasum` instead, so one of the three targets would have failed at the
checksum step. The step now selects: `command -v sha256sum` and use it, else `shasum -a 256` (lines
483–488). Both emit the identical `<hash>  <name>` format and each accepts the other's output unedited,
which is what rule 10.5's *"a format a standard verification tool accepts unedited"* requires.

**10.7 — not setting the stamp is not the same as knowing it is unset.** The candidate did not set
`MOKITERIONS_COMMIT`, which is correct, and asserted nothing. But `mokiterions-tui/src/render.rs:34`
reads it via `option_env!`, and a repository or organization variable — or a runner image — can put it in
the environment without the workflow mentioning it. Nothing downstream would notice: it is not a rebuild
input for either package, since neither has a build script, and no test compiles with it set. The damage
would appear only in a shipped binary's footer. The `Build` step now refuses when the variable is present
(line 432), which turns rule 10.7 into a check rather than an intention. `static-checks.md` S5 records it.

## Rule 11 — provenance statement

| Rule | Status |
| --- | --- |
| 11.1 every value from the authorized facts, matrix, compiler or run identity | **satisfied** — `static-checks.md` S5 reads the construction line by line |
| 11.2 the commit is complete | **satisfied** — `$AUTHORIZED_COMMIT` is the gate's full hash, and `build` step 2 refuses a value that is not a complete lowercase hash |
| 11.3 no credential, no absolute path, no varying value but the run identifier | **satisfied in the definition** — the ten lines and nothing else; V5 remains unperformed |
| 11.4 the notes state the same facts, not a commit range | **satisfied** — the notes step reads only the authorized facts; line 538 records that it is *"not generated from `git log`"* |
| 11.5 no dependency, no build script | **satisfied by construction** — the statement is written by `echo` in a shell block |

All ten required lines are present and each is sourced as the rule 11 table specifies. Satisfied
unchanged from the candidate; the only correction in this area was the `$RUNNER_OS` read, which stays in
the *step* (to choose the `.exe` suffix) and does not reach the statement — a distinction
`static-checks.md` S5 states explicitly so that a future reader does not "fix" it in the wrong direction.

## Rule 12 — publication boundary

| Rule | Status | Note |
| --- | --- | --- |
| 12.1 attach to a tag that already exists; fail rather than create | **satisfied** | `--verify-tag` makes `gh` abort when the tag is absent |
| 12.2 created not publicly visible, and nothing changes that default | **satisfied** | `--draft` on the create path; the re-run path names no visibility flag in either direction, so the owner's last setting stands |
| 12.3 attachment happens once, for all assets | **satisfied** | one call per path, carrying every asset of every target |
| 12.4 making it visible is the owner's act | **satisfied** | nothing in the definition publishes |
| 12.5 write access confined to the smallest job holding the attaching step | **satisfied**, against the rule as amended 2026-08-19 | see below |
| 12.6 a deployment environment is referenced | **satisfied as a reference** | see below |

**12.5 was unsatisfiable as originally written, and was amended rather than approximated.** The original
wording — *"the attaching step is the only step granted write access"* — cannot be expressed on GitHub
Actions, which scopes `permissions` **per job, not per step**. There is no step-level grant to write.

`WO-MOK-009` is explicit about what to do with that: *"If a rule of `SPEC-MOK-005` cannot be satisfied as
written, that is a specification question for the technical owner, not an implementation choice."* It was
raised as one, and **the technical owner amended 12.5 on 2026-08-19**. The rule now confines write access to
the smallest job containing the attaching step and requires the credential to reach that step alone, with a
sentence recording that a per-step grant is what a platform offering one should use instead.

The definition meets the amended rule: one job holds `contents: write`, and within it
`GH_TOKEN: ${{ github.token }}` (line 556) is on the attaching step alone — the download and notes steps
receive no token. Splitting `publish` into two jobs would not narrow it further, since the attaching job
would still need `contents: write`, so this is the floor of what the platform can express rather than a
concession above it. The reasoning stays in a comment at the grant itself (lines 511–514) so that a future
reader does not read the job-level scope as carelessness; `static-checks.md` S2 holds the measurement.

This is the one rule in this file whose "satisfied" is the product of changing the rule rather than the
implementation. Recorded that way on purpose.

**12.6 is a reference, and a reference is all a file can be.** `publish` declares
`environment: release` (line 520). Nothing in the repository configures that environment; the protection
rule 12.6 exists to enable — a required reviewer before upload — is a repository *setting*. The rule asks
only that it *"be required by configuration alone, without editing the process"*, and that is satisfied.
It is recorded here because "rule 12.6 satisfied" could otherwise be read as "an approval gate exists",
and none does until someone configures it.

## Rule 13 — acts the process never performs

| Rule | Status | Evidence |
| --- | --- | --- |
| 13.1 no tag created, moved, deleted or force-updated | **satisfied** | `static-checks.md` S1: no `git tag`, and `--verify-tag` aborts rather than creating |
| 13.2 no artifact transitioned, written, edited, deleted or committed | **satisfied** | S3: every mention of the artifact root is a comment or a read-only argument |
| 13.3 no commit or push | **satisfied** | S1, Appendix B: 16 git invocations across the process — 9 in the workflow's run blocks, 2 in the gate, 5 in the reachability check — all read-only. The only mutating verbs in the file are inside the trailing comment recording what the release owner runs by hand |
| 13.4 no branch created, including the maintenance branch | **satisfied** | the two `git switch`/`git push` lines in the definition are the trailing comment recording what the *release owner* runs by hand (lines 614–615) |
| 13.5 no version changed in a manifest, lockfile or tracked file | **satisfied** | S4: `--locked` on every resolving command |
| 13.6 no release made publicly visible | **satisfied** | 12.2 |
| 13.7 no secret held, required or emitted | **satisfied** | S5: `grep -n 'secrets\.'` returns nothing; one use of `${{ github.token }}` |
| 13.8 the managed harness workflow is neither invoked nor modified | **satisfied** | S8: it appears twice, both in comments; `doctor` confirms the recorded hash; the compliance job re-runs the checks rather than calling it, because it declares no `workflow_call` |
| 13.9 no release deleted that was previously attached | **satisfied** | S1: no `gh release delete` anywhere |

Satisfied unchanged by the candidate in all nine, and the failure-path clause too: there is no
`continue-on-error`, no `always()` cleanup that mutates anything, and no trap. The one `if: always()` is
on `actions/upload-artifact@v4` for the harness dashboard, which writes to the run's artifact store.

The standing limitation `VER-MOK-008` already states applies unchanged: *"S1 through S4 establish that the
process definition contains no reserved act. They do not establish that no tool the process invokes
performs one on its behalf."*

## Rule 14 — the documented human sequence

**Satisfied.** `docs/RELEASE_RUNBOOK.md` is outside the governed artifact root — `[harness].artifact_root`
is `docs/engineering`, and `docs/engineering/RELEASE_RUNBOOK.md` does not exist — so it cannot be mistaken
for an artifact carrying authority, is not walked by the validator, and has no status field. It covers all
seven acts rule 14 lists, across nine phases A through I, and states for each what must already hold, who
performs it and what it produces. It names roles rather than people — a "Who decides what" section maps
each act to a role. Its opening blockquote, before any procedure, says it is operator documentation and not
authority and that a governed artifact prevails where they disagree. `static-checks.md` S10 records the root
comparison.

**One ordering difference, stated rather than glossed.** Rule 14 lists the maintenance branch *after* the
release record; the runbook cuts it in Phase C, before the verification record, the contract and the record.
That is deliberate and the file gives the reason: `REPOSITORY_CONTEXT.md` cuts `release/<major>.<minor>`
*"when a release enters stabilization"* — before publication — so by the time the release record exists the
moment has passed. Phase C also states the consequence rule 14's ordering hides: if stabilization adds
commits, the candidate commit becomes the tip of `release/0.1` and Phase B must be re-run against it. Rule
14 requires that the procedure be ordered and that it cover the seven acts; it is read here as not fixing
the order against the repository's own stabilization policy. **The technical owner accepted that reading on
2026-08-19**, so Phase C stays where it is and rule 14's enumeration stays as written. The alternative had
been to move one section or reorder the rule's list; neither was taken.

Confirming its adequacy is `VER-MOK-008` M1, which is the assurance owner's read, not this file's claim.

## Error and recovery behavior

| Clause | Status |
| --- | --- |
| every check fails closed | **satisfied after the two 4.2/4.3 fixes**; before them, two paths failed open |
| a refusal names the fact | **satisfied after the 7.1 and 7.4 fixes**; every refusal in the gate and the process now names its fact, and every refusal test asserts the message as well as the status |
| a refusal is total | **satisfied** — `assert_refused` snapshots `git status --porcelain --untracked-files=all`, `git tag --list` and `git branch --list` before and after, across all 24 refusal scenarios |
| recovery is forward only | **satisfied by construction** — nothing in the process edits a record |
| a moved tag is not recoverable by re-running | **satisfied** — rule 4.6, R11 |
| a partial failure during production leaves nothing published | **satisfied** — attachment is a single terminal step |

## Data and interface contracts

| Clause | Status |
| --- | --- |
| `+++`-delimited TOML, BOM tolerated, closing delimiter is the first `+++` after the first | **satisfied**, and now tested — `test_refuses_front_matter_that_is_never_closed` |
| the gate takes a repository root and a tag and is runnable by hand | **satisfied** — every A and R scenario invokes it as a separate process with exactly those arguments |
| the authorized facts are the only channel | **satisfied** — no later step re-reads the graph for the version, commit or identifiers |
| identifier lists are stable and readable | **satisfied** — space-separated identifiers, quoted directly into the provenance statement |
| the harness is invoked as a Python module | **satisfied after the header-comment correction** |
| no interface of either Rust package is read, extended or depended on | **satisfied** — no Rust source changed; `static-checks.md` S9 |

## Two divergences deliberately left in place

1. **`REPOSITORY_CONTEXT.md`'s lint command lacks `--locked`.** Recorded under rule 8 above. The workflow
   is stricter than the declaration, which is the safe direction; closing it means editing the
   declaration, which is beyond what `WO-MOK-009` names for that file.
2. **Rule 12.6's environment is unconfigured.** Recorded under rule 12 above. Configuring it is a
   repository setting and a release-owner decision, not a file this work order writes.

## One change outside this work order's declared change surface

`WO-MOK-009`'s Expected change surface says: *"no artifact other than this work order's own status and its
evidence."* One tracked artifact outside that surface was modified: `WO-MOK-001.md` gained an
`[assurance]` table, a `decided_by` field, an `updated` bump, and a Lifecycle note recording the addition.
Nothing else about it changed — not its status, not its scope, not its relations, not its evidence.

**Why it was touched.** Rule 7.4 requires that the harness's review-phase preflight pass for every work
order the release record releases, and the six `implemented` work orders are the ones a first release
record would name. `WO-MOK-001` does not pass. Measured against a pristine clone whose copy of the file is
byte-identical to `HEAD`'s:

```text
$ <pinned 0.4.0> -m se_harness preflight . --work-order WO-MOK-001 --phase review
Harness preflight: FAIL
Work order: WO-MOK-001 (implemented)
- [W023] docs/engineering/simulation/work-orders/WO-MOK-001.md: selected work order requires an
  accountable explicit assurance decision: assurance classification is missing
exit=1
```

The declaration `docs/engineering/WORKFLOW.md` requires did not exist when `WO-MOK-001` was approved on
2026-08-11; `WO-MOK-002`, approved 2026-08-17, is the first to carry one. Completed legacy work may omit
it, but renewed preflight selection may not. So this is a **pre-existing governance gap in `WO-MOK-001`
that `WO-MOK-009`'s rule 7.4 rehearsal surfaced**, and it blocks a first release until it is closed.

**What was and was not done.** Preparing the record is permitted — `ENGINEERING_HARNESS.md`: *"Harness
commands may prepare records, but never exercise accountable decision rights"* — and no status was
transitioned. The file is outside this work order's declared surface, and the `decided_by` field names the
engineering owner, so the classification was theirs to affirm or replace, including the rationale text.

**Disposition: kept and affirmed.** The engineering owner affirmed the classification and its rationale on
2026-08-19 and chose to let the scope deviation stand as a stated finding here rather than widen
`WO-MOK-009`'s change surface or open a separate governance work order. `WO-MOK-001`'s Lifecycle section
records the affirmation and the disposition at the artifact itself, which is where a reader of that file
will look; this section is the finding as `WO-MOK-009`'s completion report carries it.

The two paths not taken, for the record: widening `WO-MOK-009`'s Expected change surface by one line, or
reverting the edit and closing the gap under a governance work order of its own. The first is tidier on
paper but amends an approved-scope statement after the fact; the second is the most correct and makes a
first release wait on another approval cycle. Reverting without either would leave rule 7.4 refusing on
`WO-MOK-001` with no path forward that does not involve this same edit.

Also recorded so it is not discovered later: `compliance-rehearsal.md` C2's *"all six PASS"* result
depends on this edit. Without it, five pass and `WO-MOK-001` fails.
