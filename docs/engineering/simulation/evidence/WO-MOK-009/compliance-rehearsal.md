# Compliance rehearsal — `VER-MOK-008` C1 through C5

Captured 2026-08-19 in a linked worktree of the primary clone, on branch `feature/release-ci`, HEAD
`54c21abcfb9caa4474c9ca5f194289e055c86a23`, with this work order's changes present but uncommitted.

**Harness build.** Every harness command below was run with the wheel the workflow installs —
`pip install --disable-pip-version-check --no-deps --only-binary=:all: se-harness==0.4.0` into a
throwaway virtual environment — rather than with whatever harness happens to be on the machine. That
distinction is not pedantry: it is the subject of the C1 section, and it is why every other evidence
file in this directory that quotes a harness command should be read as quoting harness `0.4.0`.

| Scenario | Status here |
| --- | --- |
| C1 — version equality precedes every harness command | **observed**, on a real mismatch, and **rehearsed** in both directions |
| C2 — the graph is checked at the governance revision | **rehearsed** locally; all three halves pass |
| C3 — the declared checks pass at the authorized commit | **observed**; see `verification-output.md` and `determinism-rehearsal.md` |
| C4 — a failing check publishes nothing | **not performed.** Requires running the process, which requires a tag and a push |
| C5 — warnings do not refuse | **observed**, under the row as amended on 2026-08-19. The method as *originally* written could not be exercised; see below for the finding, the evidence, and the owner's disposition |

## C1 — the version predicate, and the day it fired on its own

`SPEC-MOK-005` rule 7.1: *"The installed harness version equals the version the harness configuration
declares. A mismatch in either direction refuses, naming both versions, before any harness command
runs."*

### The unplanned case, which is the better evidence

This rehearsal was written to fabricate a mismatch. It did not need to. Partway through this work the
machine's harness moved from `0.4.0` to `0.4.1` — it is an editable install of the harness project's
own working clone, and that clone advanced — while this repository's `.engineering-harness.toml` went
on declaring `0.4.0`. The predicate caught it with no fixture at all:

```text
### C1 in the field -- rule 7.1 firing on a real mismatch, unplanned
#
# The predicate, verbatim from the `harness` job step *The installed harness must be the
# version the repository declares*. Run twice against the same repository with two
# different interpreters: the machine-wide harness, and the pinned wheel the workflow
# installs. Only `installed` differs between the two runs.

--- interpreter: machine-wide, an editable install of the harness's own working clone
repository declares 0.4.0; runner has 0.4.1
REFUSED: this repository declares harness 0.4.0, but the runner has 0.4.1.
exit=1  (the step stops here; doctor, validate and preflight never run)

--- interpreter: the pinned wheel: pip install --no-deps --only-binary=:all: se-harness==0.4.0
repository declares 0.4.0; runner has 0.4.0
exit=0  (the step proceeds to doctor)
```

What rule 7.1 bought, made visible by running `doctor` anyway on the mismatched build:

```text
$ python -m se_harness doctor .   # harness 0.4.1 against a repository installed at 0.4.0
FAIL distribution:.engineering-harness.toml: differs from distribution template
FAIL distribution:.github/workflows/engineering-harness.yml: differs from distribution template
FAIL distribution:ENGINEERING_HARNESS.md: differs from distribution template
FAIL distribution:docs/engineering/WORKFLOW.md: differs from distribution template
FAIL distribution:docs/engineering/templates/RELEASE_RECORD.template.md: differs from distribution template
FAIL distribution:scripts/generate_harness_dashboard.py: differs from distribution template
FAIL distribution:scripts/harness_explorer/index.template.html: differs from distribution template
FAIL distribution:scripts/validate_engineering_artifacts.py: differs from distribution template
exit=1

$ <pinned 0.4.0> -m se_harness doctor .
PASS lines: 81   FAIL lines: 0
exit=0
```

Eight failures on the mismatched build, none on the pinned one, against the *same* repository at the
*same* commit. Every one of the eight is a `distribution:` check — the harness comparing a file against
the template its own release ships — so all eight say the same thing in eight voices: *this repository
was installed from a different harness than the one now reading it*. None of them is a tampering
finding; `managed:` and `seed:` checks pass in both runs.

That is the failure rule 7.1 exists to convert into one sentence. Without it, a release run would
refuse at rule 7.2 with an eight-line report that reads like managed-file damage, and the reader would
go looking for damage that is not there. The rule also decides the *order*: the version check comes
before `doctor`, `validate` and `preflight`, so the cheap and specific refusal wins the race against
the expensive and misleading one.

Two consequences worth recording, because they outlive this incident:

- **The workflow does not use the machine's harness.** It installs a pinned wheel from an index
  (`--only-binary=:all:`, `--no-deps`), so `installed` is a fixed published artifact and the whole
  compliance job is reproducible. A developer's box, where the harness may be an editable clone of a
  moving branch, is the environment where this can drift; CI is not.
- **Harness upgrades are a governed act, not a background one.** When `0.4.1` becomes the declared
  version, the eight `distribution:` differences are what must be reconciled, and they are all
  deliberate customizations of this repository. Rule 7.1 ensures the decision is taken rather than
  absorbed.

### The rehearsed cases, both directions plus the unreadable one

Four fabricated `.engineering-harness.toml` files in a scratch directory. The repository's own
configuration was never edited and no harness was installed or removed to make a check fire.

```text
### C1 — the harness version predicate, rehearsed in both directions
#
# The predicate, lifted verbatim from the `harness` job step *The installed harness must be
# the version the repository declares*. `installed` is read from the real installed package;
# `declared` is read from a fabricated `.engineering-harness.toml` in a scratch directory, so
# the repository's own configuration is never edited to make a check fire.
#
declared="$(python -c "...tomllib...[harness][tool_version]")"
installed="$(python -c "import se_harness; print(se_harness.__version__)")"
echo "repository declares $declared; runner has $installed"
if [[ "$declared" != "$installed" ]]; then
  echo "REFUSED: this repository declares harness $declared, but the runner has $installed." >&2
  exit 1
fi

$ python -c 'import se_harness; print(se_harness.__version__)'
0.4.0

--- declared: 0.4.0
$ cat case-0.4.0/.engineering-harness.toml
[harness]
tool_version = "0.4.0"
repository declares 0.4.0; runner has 0.4.0
exit=0  (equal in both directions; the step proceeds to doctor)

--- declared: 0.3.9
$ cat case-0.3.9/.engineering-harness.toml
[harness]
tool_version = "0.3.9"
repository declares 0.3.9; runner has 0.4.0
REFUSED: this repository declares harness 0.3.9, but the runner has 0.4.0.
exit=1

--- declared: 0.5.0
$ cat case-0.5.0/.engineering-harness.toml
[harness]
tool_version = "0.5.0"
repository declares 0.5.0; runner has 0.4.0
REFUSED: this repository declares harness 0.5.0, but the runner has 0.4.0.
exit=1

--- declared: <absent>
$ cat case-empty/.engineering-harness.toml
[harness]
KeyError: 'tool_version'
exit=1  (the declaration is unreadable; the step fails before any harness command)
```

A declaration below the installed version refuses; one above it refuses; a declaration that cannot be
read fails before any harness command, which is the fail-closed reading of rule 7.1 rather than a
generous one. The refusal names both values in every direction, which is what rule 7.1 asks for and
what this changeset had to be corrected to do — see `candidate-conformance.md`, rule 7.1.

## C2 — the graph is checked at the governance revision

Three claims: the revision is the default branch's tip, validation reports zero errors, and one review
preflight is logged per released work order, each naming its work order.

```text
### C2, half one -- the pinning predicate
$ git rev-parse HEAD; git rev-parse origin/master
54c21abcfb9caa4474c9ca5f194289e055c86a23
54c21abcfb9caa4474c9ca5f194289e055c86a23
$ git log -1 --format="governance revision %H (%ci) %s" HEAD
governance revision 54c21abcfb9caa4474c9ca5f194289e055c86a23 (2026-08-19 10:49:44 +0200) Merge pull request #16 from mmzen/governance/wo-mok-005-implemented

### C2, half two -- validation reports zero errors
$ python scripts/validate_engineering_artifacts.py --root .
Engineering artifact validation: PASS
Artifacts: 79 | Errors: 0 | Warnings: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
exit=0

### C2, half three -- the review-preflight loop, run verbatim from the step
#
# WORK_ORDERS holds the six work orders a first release record would name. Each report is
# elided to its verdict and work-order lines; the full report is ~50 lines each.

::group::preflight WO-MOK-001
Harness preflight: PASS
Work order: WO-MOK-001 (implemented)
::endgroup::
::group::preflight WO-MOK-002
Harness preflight: PASS
Work order: WO-MOK-002 (implemented)
::endgroup::
::group::preflight WO-MOK-003
Harness preflight: PASS
Work order: WO-MOK-003 (implemented)
::endgroup::
::group::preflight WO-MOK-004
Harness preflight: PASS
Work order: WO-MOK-004 (implemented)
::endgroup::
::group::preflight WO-MOK-005
Harness preflight: PASS
Work order: WO-MOK-005 (implemented)
::endgroup::
::group::preflight WO-MOK-006
Harness preflight: PASS
Work order: WO-MOK-006 (implemented)
::endgroup::
review preflight passes for WO-MOK-001 WO-MOK-002 WO-MOK-003 WO-MOK-004 WO-MOK-005 WO-MOK-006
exit=0
```

Four notes on what this rehearsal is and is not.

**`WO-MOK-001`'s PASS depends on an uncommitted edit, and would be a FAIL at `HEAD`.** The half-three
capture above was taken in this worktree, where `WO-MOK-001.md` carries an `[assurance]` table that the
committed file does not. Measured against a pristine clone whose copy of that file is byte-identical to
`HEAD`'s (LF-normalized SHA-256 `bd1a78b477cef73bba58dd39b24d3f48c06a77385a2427ad7b764f381e239c2c`):

```text
$ python -m se_harness preflight . --work-order WO-MOK-001 --phase review
Harness preflight: FAIL
Work order: WO-MOK-001 (implemented)
- [W023] docs/engineering/simulation/work-orders/WO-MOK-001.md: selected work order requires an
  accountable explicit assurance decision: assurance classification is missing
exit=1
```

So at the governance revision as committed, this loop reports five passes and one failure, and rule 7.4
refuses. That is a pre-existing governance gap in `WO-MOK-001` — the declaration it needs did not exist
when that work order was approved — surfaced by this rehearsal rather than caused by it. The edit that
makes it pass is outside this work order's declared change surface; `candidate-conformance.md`'s closing
section states the finding, the evidence and the recommended disposition, and it is the engineering
owner's to affirm.

**The work-order set is fabricated, and has to be.** On a run, `WORK_ORDERS` comes from the release
record, which lists the work orders being released. No release record exists, so the gate emits an
empty set and the loop would run zero times. The six `implemented` work orders are the ones a first
release record would plausibly name, so the loop is exercised against them. A reader should treat the
six names as a stand-in and the loop's behavior as the finding.

**The `::group::` markers are load-bearing.** They are what makes one collapsible section per work
order in the run log, which is how rule 7.4's *"a failure names the work order"* stays legible when six
reports of fifty lines each land in one step.

**The refusal path was also exercised, and it is what corrected the step.** Run against a harness that
disagrees with the declaration — the `0.4.1` build above — all six preflights fail, and the step's
original wording said only *"at least one released work order does not pass review preflight"*, naming
none of them. Rule 7.4 asks for the name. The step now collects the failures and names every one:

```text
REFUSED: review preflight does not pass for WO-MOK-001 WO-MOK-002 WO-MOK-003 WO-MOK-004 WO-MOK-005 WO-MOK-006.
exit=1
```

It also keeps going after the first failure rather than stopping, because a reader fixing this wants
the whole list. See `candidate-conformance.md`, rule 7.4.

## C3 — the declared checks pass at the authorized commit

Discharged elsewhere and not restated here: `verification-output.md` holds the formatter, linter, test
and dependency-tree output — the tree resolving to exactly one crate for the engine — and
`determinism-rehearsal.md` holds the two-runs-per-policy comparison with byte identity, identical final
state and identical exit code.

The one thing to say here is what C3's *"at the authorized commit"* rests on, since nothing in a local
capture can demonstrate it: the `verify` job checks out `needs.authorize.outputs.commit` and its second
step compares `git rev-parse HEAD` against that value, refusing on a mismatch. `static-checks.md` S11
records the three revision claims and the three steps that assert them.

## C4 — a failing check publishes nothing

**Not performed.** The scenario is explicitly *"rehearsed rather than performed on a real release"*,
but even its rehearsed form requires running the process: breaking a declared check on a branch,
triggering the workflow, and confirming afterwards that no release object and no asset exist. Running
the process requires an annotated tag on the remote and a push, both reserved to the release owner by
`ENGINEERING_HARNESS.md` and `docs/engineering/DECISION_RIGHTS.md`. This work order does not perform
reserved acts, so C4 is left outstanding for whoever runs the first release rehearsal.

What can be said without a run, from the definition rather than from behavior, is the structural half:

- `publish` declares `needs: [authorize, harness, verify, build]`. A failed step fails its job, and a
  failed job's dependents do not start.
- There is no `continue-on-error` anywhere and no `if:` on any job, so no job can run past a failure or
  around one. `static-checks.md` S1 and S11 record both searches.
- Packaging happens in `build`, after every check in `verify`, and the assets exist only as run
  artifacts until `publish` uploads them. A refusal in `verify` therefore costs no packaging, which is
  the performance claim `VER-MOK-008` attaches to C4.

That is a reading of the definition, not a measurement, and it is offered as the former.

## C5 — warnings do not refuse

The scenario's substance is right and holds. Its *method*, **as originally written**, named an instrument
that reports zero warnings, so the assertion it asked for could not be made. That was raised as a finding,
and the assurance owner amended the row on 2026-08-19; this section keeps the original analysis because it
is the evidence for the amendment, and records the outcome at the end.

```text
### C5 -- what each instrument reports today
#
# Harness build: the pinned wheel se-harness==0.4.0, the same one the workflow installs.

$ python scripts/validate_engineering_artifacts.py --root .   # the rule 7.3 validation step
Engineering artifact validation: PASS
Artifacts: 79 | Errors: 0 | Warnings: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
exit=0

$ <pinned 0.4.0> -m se_harness inspect .   # a different instrument, not the rule 7.3 step
Harness inspection
Finding severity: error 0 | warning 9 | info 6
- [WARNING] W-HEX-001 (derived): 6 observations [WO-MOK-001, WO-MOK-002, WO-MOK-003, WO-MOK-004, WO-MOK-005, WO-MOK-006]
- [WARNING] W-HEX-003 (derived): 3 observations [ADR-MOK-001, ARCH-MOK-001, ARCH-MOK-002, SPEC-MOK-003, SPEC-MOK-004]
- [INFO] I-REV-001 (derived): 6 observations [VREC-MOK-001, VREC-MOK-002, VREC-MOK-003, VREC-MOK-004, VREC-MOK-005, VREC-MOK-006]
exit=0
```

C5, before the amendment, said: *"The real repository currently reports zero errors and several warnings,
with four open amendment rows. A run against it must treat validation as passing. This is checked by
asserting the validation step's success against its own reported warning count being non-zero."*

Taking those clauses one at a time against the run above:

| Clause | Today |
| --- | --- |
| zero errors | **true** — both instruments report `error 0` |
| several warnings | **true of `inspect`** (9), **false of the validation step** (0) |
| four open amendment rows | **true** — `REQ-MOK-035` names them, and `SPEC-MOK-002` and `ARCH-MOK-001` carry them as **OUTSTANDING** rows |
| a run must treat validation as passing | **true and satisfied** — the step asserts exit 0 and reads no warning count |
| assert against *the validation step's own* reported warning count being non-zero | **not possible** — that count is `0` |

So rule 7.3's *"warnings do not refuse"* is implemented correctly and demonstrated: the validation step
succeeds, the workflow reads nothing but its exit code, and `inspect`'s nine warnings and six
informational findings do not enter the decision anywhere. What fails is the scenario's chosen
*witness*. The rule-7.3 validation step is
`python scripts/validate_engineering_artifacts.py --root .`, and it reports `Warnings: 0` across all
four planes, so an assertion that its warning count is non-zero would fail against a conformant
implementation. The nine warnings live in `se_harness inspect`, which rule 7 does not require the
process to run and which the process does not run.

**This is a gap in `VER-MOK-008`, not in the implementation,** and it is the assurance owner's to close
— the same standing the contract gives itself in M3: *"A missing row is a gap in this contract, not in
the implementation."* Three ways to close it, in increasing order of change:

1. Drop the non-zero clause. C5's substance is that a passing validation with warnings does not refuse;
   asserting the step's success while *not* asserting anything about its warnings already establishes
   that the count is unread, which is the property that matters.
2. Point the clause at `inspect` and keep the count. This requires the process to run `inspect` and
   attach it, which rule 7 does not currently ask for and which would be a `SPEC-MOK-005` change.
3. Leave it and let the row stand as unexercisable. Not recommended: an unexercisable row in a
   contract is indistinguishable, six months on, from one nobody got round to.

Option 1 is what this evidence supports. **The assurance owner chose option 1 on 2026-08-19**, and
`VER-MOK-008` C5 now reads:

> This is checked by asserting the validation step's success while asserting nothing whatever about its
> reported warning count. The absence of any predicate on the count is what establishes that the count is
> unread; a scenario requiring a non-zero count would be unsatisfiable against a conformant repository,
> whose warning count may legitimately be zero — as this one's is.

Under the amended row, **C5 is observed rather than unexercisable.** The witness is the run at the top of
this section: the validation step exits 0, and the workflow's rule 7.3 step reads its exit code and nothing
else. The two facts the amended row asks for are both in that transcript. The clause about *"several
warnings"* stays true of the repository as `inspect` sees it and is now correctly not something the
scenario tests.

One caveat on the counts. `inspect`'s warning total is a property of the harness build as well as of
the repository: the same commit reports `warning 9` under `0.4.0` and `warning 3` under `0.4.1`. Any
future contract row that asserts a *number* here will need to say which harness produced it. That is
another argument for option 1.
