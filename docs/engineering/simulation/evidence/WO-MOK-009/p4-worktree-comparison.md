# P4 — the gate writes nothing

`VER-MOK-008` scenario **P4**, captured 2026-08-19. Rule 12.1 through 12.4 of `SPEC-MOK-005` reserve
tagging, branching, committing and status transitions to accountable humans. P4 is the observational
half of that: not "the definition contains no such command" — that is S1 through S4 — but "running it
changed nothing."

Three independent measurements, at three levels.

## 1. Against this checkout, before and after

The snapshot is everything the gate is forbidden to change: the working tree including untracked files,
the tag list, the branch list, every reference with its object name, and `HEAD`.

```text
$ git rev-parse HEAD
54c21abcfb9caa4474c9ca5f194289e055c86a23
$ git rev-parse origin/master
54c21abcfb9caa4474c9ca5f194289e055c86a23

$ python scripts/check_release_authorization.py --root . --tag v0.1.0
REFUSED: v0.1.0 is not a tag in this repository. The release owner creates the tag; this workflow never does.

This gate does not grant authority. It reports that the accountable release decision recorded under the artifact root does not authorize publishing this tag.
exit=1

$ python scripts/check_release_authorization.py --root . --tag v9.9.9
REFUSED: v9.9.9 is not a tag in this repository. The release owner creates the tag; this workflow never does.

This gate does not grant authority. It reports that the accountable release decision recorded under the artifact root does not authorize publishing this tag.
exit=1

$ python scripts/check_release_reachability.py --root . --commit $(git rev-parse origin/master) --default-branch master --remote origin
REACHABLE commit 54c21abcfb9caa4474c9ca5f194289e055c86a23
  contained by         refs/remotes/origin/master
exit=0

$ python scripts/check_release_reachability.py --root . --commit $(git rev-parse HEAD) --default-branch master --remote origin   # this feature branch's tip
REACHABLE commit 54c21abcfb9caa4474c9ca5f194289e055c86a23
  contained by         refs/remotes/origin/master
exit=0

$ diff /tmp/mok-ci/p4-before.txt /tmp/mok-ci/p4-after.txt
(no difference: the checkout is byte-for-byte unchanged)
```

`diff` of the before and after snapshots is empty. Note what the two gate runs did *not* do: no tag
named `v0.1.0` or `v9.9.9` came into existence, and the refusal message says so in words —
*"The release owner creates the tag; this workflow never does."*

The reachability check is the interesting half here, because it is the only component that consults
`refs/remotes/*`. It reports `REACHABLE` against `refs/remotes/origin/master` and creates nothing. In
this worktree `HEAD` equals `origin/master` (`54c21abc…`) because none of this work is committed yet,
so the two invocations name the same commit; the feature-branch and unpushed-commit cases are covered
by fixtures instead — `test_r23_the_authorized_commit_is_reachable_only_from_a_feature_branch` and
`test_r24_the_authorized_commit_is_reachable_from_no_remote_branch`.

## 2. Against a throwaway clone of the real repository

`test_p4_the_gate_leaves_a_real_checkout_unchanged` clones the repository, runs the gate twice — once
for a tag the graph could plausibly authorize, once for a tag nobody has ever mentioned — and compares
the same four facts. It passes; see `a5-refusal-ladder.md` for the run.

## 3. Folded into every fixture scenario

```text
$ python scripts/test_check_release_authorization.py GateTest.test_a2_the_record_lives_in_a_later_commit_than_the_one_it_names GateTest.test_p4_the_gate_writes_nothing_when_it_authorizes GateTest.test_p4_the_gate_writes_nothing_when_it_refuses
test_a2_the_record_lives_in_a_later_commit_than_the_one_it_names (__main__.GateTest.test_a2_the_record_lives_in_a_later_commit_than_the_one_it_names)
The real order of events, and the one the workflow's checkout must match. ... ok
test_p4_the_gate_writes_nothing_when_it_authorizes (__main__.GateTest.test_p4_the_gate_writes_nothing_when_it_authorizes) ... ok
test_p4_the_gate_writes_nothing_when_it_refuses (__main__.GateTest.test_p4_the_gate_writes_nothing_when_it_refuses) ... ok

----------------------------------------------------------------------
Ran 3 tests in 1.899s

OK
exit=0
```

The two dedicated P4 tests cover the authorizing and refusing paths explicitly, one with an untracked
file present so that a gate which "cleaned up" would be caught. Beyond them, the property is asserted
on **every** scenario: `assert_authorized` and `assert_refused` each snapshot before and after and
compare, which is `VER-MOK-008`'s refusal-totality property applied across the whole set rather than
once. A gate that refused correctly but created a tag on the way would fail all 48 tests.

## The one thing either program writes

`$GITHUB_OUTPUT`, when the platform sets it, and nothing else. Both programs write it through the same
narrow `emit()` function: append `key=value` lines to the path in that variable, or do nothing if it is
unset. That is how the authorized facts reach later jobs without any step re-deriving them.
`test_a4_the_authorized_facts_are_emitted_for_the_workflow` and
`test_emits_the_containing_branch_for_the_workflow` assert the contents by pointing the variable at a
temporary file.

Standard output is a report for a human reading a log. Nothing downstream parses it, so its wording can
change without breaking a job.
