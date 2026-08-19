# Scenario suite output

Captured 2026-08-19 on `feature/release-ci`, at working-tree state
`54c21abcfb9caa4474c9ca5f194289e055c86a23` plus this work order's uncommitted changes.
Toolchain: Python 3.14.6, git as installed. `scenario-map.md` maps every test below to the
`VER-MOK-008` scenario it discharges.

| Suite | Tests | Result | Duration |
| --- | --- | --- | --- |
| `scripts/test_check_release_authorization.py` | 48 | OK | 27.5 s |
| `scripts/test_check_release_reachability.py` | 22 | OK | 15.4 s |
| **total** | **70** | **OK** | — |

Both suites are committed to the repository and runnable by hand at any commit, which is what
`VER-MOK-008`'s Evidence retention section asks of them. Neither needs a network: the reachability
fixtures build `refs/remotes/origin/*` with `git update-ref` rather than by talking to a server, and
one test deliberately points `origin` at a path that does not exist to prove no fetch is attempted.

Fixtures are real — `git init`, real commits, real annotated tags, real `+++`-delimited TOML. Nothing
is stubbed, so no scenario can pass because a stub agreed with the gate about what git would have
said.

## `scripts/test_check_release_authorization.py`

```text
$ python scripts/test_check_release_authorization.py
test_a5_a_ready_verification_record_still_refuses (__main__.A5RealRepositoryTest.test_a5_a_ready_verification_record_still_refuses) ... ok
test_a5_the_real_graph_holds_no_release_record (__main__.A5RealRepositoryTest.test_a5_the_real_graph_holds_no_release_record) ... ok
test_a5_the_real_repository_refuses_every_tag_today (__main__.A5RealRepositoryTest.test_a5_the_real_repository_refuses_every_tag_today)
No tag exists, so the first fact the gate cannot establish is the tag itself. ... ok
test_a5_the_refusal_ladder (__main__.A5RealRepositoryTest.test_a5_the_refusal_ladder)
Which refusal remains after each governance artifact is added. ... ok
test_a5_the_six_existing_records_cannot_release_an_aggregate (__main__.A5RealRepositoryTest.test_a5_the_six_existing_records_cannot_release_an_aggregate)
Each existing record binds its own work order's commit, and they differ. ... ok
test_p4_the_gate_leaves_a_real_checkout_unchanged (__main__.A5RealRepositoryTest.test_p4_the_gate_leaves_a_real_checkout_unchanged) ... ok
test_a1_a_correct_graph_authorizes (__main__.GateTest.test_a1_a_correct_graph_authorizes) ... ok
test_a2_the_record_lives_in_a_later_commit_than_the_one_it_names (__main__.GateTest.test_a2_the_record_lives_in_a_later_commit_than_the_one_it_names)
The real order of events, and the one the workflow's checkout must match. ... ok
test_a3_a_record_stating_a_different_tag_is_not_a_match (__main__.GateTest.test_a3_a_record_stating_a_different_tag_is_not_a_match) ... ok
test_a3_a_record_that_states_no_tag (__main__.GateTest.test_a3_a_record_that_states_no_tag) ... ok
test_a4_the_authorized_facts_are_emitted_for_the_workflow (__main__.GateTest.test_a4_the_authorized_facts_are_emitted_for_the_workflow) ... ok
test_a4_the_authorized_facts_are_reported (__main__.GateTest.test_a4_the_authorized_facts_are_reported) ... ok
test_ignores_files_under_an_excluded_directory (__main__.GateTest.test_ignores_files_under_an_excluded_directory)
Retained evidence and templates are not artifacts, exactly as the validator sees it. ... ok
test_ignores_prose_that_carries_no_front_matter (__main__.GateTest.test_ignores_prose_that_carries_no_front_matter) ... ok
test_p4_the_gate_writes_nothing_when_it_authorizes (__main__.GateTest.test_p4_the_gate_writes_nothing_when_it_authorizes) ... ok
test_p4_the_gate_writes_nothing_when_it_refuses (__main__.GateTest.test_p4_the_gate_writes_nothing_when_it_refuses) ... ok
test_property_a_refusal_is_idempotent (__main__.GateTest.test_property_a_refusal_is_idempotent) ... ok
test_property_commit_equality_is_total_not_prefix (__main__.GateTest.test_property_commit_equality_is_total_not_prefix)
A record commit that is a strict prefix of the tagged commit is still refused. ... ok
test_property_the_gate_is_idempotent (__main__.GateTest.test_property_the_gate_is_idempotent) ... ok
test_property_type_is_read_for_a_verification_record (__main__.GateTest.test_property_type_is_read_for_a_verification_record) ... ok
test_property_type_is_read_for_a_work_order (__main__.GateTest.test_property_type_is_read_for_a_work_order) ... ok
test_r10_the_records_commit_is_an_unrelated_commit (__main__.GateTest.test_r10_the_records_commit_is_an_unrelated_commit) ... ok
test_r11_the_tag_was_force_moved_after_the_record_was_written (__main__.GateTest.test_r11_the_tag_was_force_moved_after_the_record_was_written) ... ok
test_r12_the_gating_contract_does_not_exist (__main__.GateTest.test_r12_the_gating_contract_does_not_exist) ... ok
test_r13_the_gating_contracts_declared_type_is_not_a_release_contract (__main__.GateTest.test_r13_the_gating_contracts_declared_type_is_not_a_release_contract) ... ok
test_r14_the_gating_contract_is_draft (__main__.GateTest.test_r14_the_gating_contract_is_draft) ... ok
test_r15_the_contract_does_not_gate_a_released_work_order (__main__.GateTest.test_r15_the_contract_does_not_gate_a_released_work_order) ... ok
test_r16_a_released_work_order_is_approved_not_releasable (__main__.GateTest.test_r16_a_released_work_order_is_approved_not_releasable) ... ok
test_r17_an_included_verification_record_does_not_exist (__main__.GateTest.test_r17_an_included_verification_record_does_not_exist) ... ok
test_r18_an_included_verification_record_is_ready (__main__.GateTest.test_r18_an_included_verification_record_is_ready) ... ok
test_r19_an_included_verification_record_names_another_commit (__main__.GateTest.test_r19_an_included_verification_record_names_another_commit) ... ok
test_r1_the_requested_tag_does_not_exist (__main__.GateTest.test_r1_the_requested_tag_does_not_exist) ... ok
test_r20_work_is_released_but_not_verified (__main__.GateTest.test_r20_work_is_released_but_not_verified) ... ok
test_r21_work_is_verified_but_not_released (__main__.GateTest.test_r21_work_is_verified_but_not_released) ... ok
test_r22_the_records_version_is_not_usable_as_a_name (__main__.GateTest.test_r22_the_records_version_is_not_usable_as_a_name) ... ok
test_r2_the_requested_name_is_a_branch_not_a_tag (__main__.GateTest.test_r2_the_requested_name_is_a_branch_not_a_tag)
Tag resolution is confined to `refs/tags/`, so a branch cannot impersonate a tag. ... ok
test_r3_the_tag_is_lightweight_rather_than_annotated (__main__.GateTest.test_r3_the_tag_is_lightweight_rather_than_annotated) ... ok
test_r4_two_artifacts_share_an_identifier (__main__.GateTest.test_r4_two_artifacts_share_an_identifier) ... ok
test_r5_the_repository_does_not_require_full_commits (__main__.GateTest.test_r5_the_repository_does_not_require_full_commits)
The gate compares complete hashes, so it will not run where prefixes are allowed. ... ok
test_r6_no_release_record_exists_at_all (__main__.GateTest.test_r6_no_release_record_exists_at_all) ... ok
test_r7_the_release_record_is_ready_not_released (__main__.GateTest.test_r7_the_release_record_is_ready_not_released) ... ok
test_r8_two_released_records_claim_the_same_tag (__main__.GateTest.test_r8_two_released_records_claim_the_same_tag) ... ok
test_r9_the_records_commit_is_abbreviated (__main__.GateTest.test_r9_the_records_commit_is_abbreviated) ... ok
test_reads_the_declared_artifact_root_rather_than_assuming_one (__main__.GateTest.test_reads_the_declared_artifact_root_rather_than_assuming_one) ... ok
test_refuses_front_matter_that_does_not_parse (__main__.GateTest.test_refuses_front_matter_that_does_not_parse)
A damaged release record must be named, not silently treated as absent. ... ok
test_refuses_front_matter_that_is_never_closed (__main__.GateTest.test_refuses_front_matter_that_is_never_closed) ... ok
test_refuses_when_the_configuration_is_unreadable (__main__.GateTest.test_refuses_when_the_configuration_is_unreadable) ... ok
test_refuses_when_the_declared_artifact_root_is_missing (__main__.GateTest.test_refuses_when_the_declared_artifact_root_is_missing) ... ok

----------------------------------------------------------------------
Ran 48 tests in 27.474s

OK
exit=0
```

## `scripts/test_check_release_reachability.py`

```text
$ python scripts/test_check_release_reachability.py
test_accepts_a_commit_a_maintenance_branch_merely_contains (__main__.ReachabilityTest.test_accepts_a_commit_a_maintenance_branch_merely_contains) ... ok
test_accepts_a_commit_on_a_maintenance_branch (__main__.ReachabilityTest.test_accepts_a_commit_on_a_maintenance_branch) ... ok
test_accepts_a_commit_on_the_default_branch (__main__.ReachabilityTest.test_accepts_a_commit_on_the_default_branch) ... ok
test_accepts_a_maintenance_branch_nobody_named_in_advance (__main__.ReachabilityTest.test_accepts_a_maintenance_branch_nobody_named_in_advance)
The branch list is enumerated from the remote, so `release/9.9` needs no edit here. ... ok
test_emits_the_containing_branch_for_the_workflow (__main__.ReachabilityTest.test_emits_the_containing_branch_for_the_workflow) ... ok
test_honours_an_explicitly_declared_default_branch (__main__.ReachabilityTest.test_honours_an_explicitly_declared_default_branch)
The workflow passes the platform's declared default branch rather than guessing. ... ok
test_names_every_branch_it_examined (__main__.ReachabilityTest.test_names_every_branch_it_examined) ... ok
test_p4_the_check_writes_nothing (__main__.ReachabilityTest.test_p4_the_check_writes_nothing) ... ok
test_prefers_the_default_branch_when_both_contain_the_commit (__main__.ReachabilityTest.test_prefers_the_default_branch_when_both_contain_the_commit) ... ok
test_property_the_check_fetches_nothing (__main__.ReachabilityTest.test_property_the_check_fetches_nothing)
A missing reference is a refusal, not a reason to go and get it. ... ok
test_property_the_check_is_idempotent (__main__.ReachabilityTest.test_property_the_check_is_idempotent) ... ok
test_r23_a_local_release_branch_does_not_count (__main__.ReachabilityTest.test_r23_a_local_release_branch_does_not_count)
Local branches are not consulted: a recipient cannot find a commit that never left. ... ok
test_r23_the_authorized_commit_is_reachable_only_from_a_feature_branch (__main__.ReachabilityTest.test_r23_the_authorized_commit_is_reachable_only_from_a_feature_branch)
Otherwise any commit anyone pushed could be released, given a tag. ... ok
test_r24_the_authorized_commit_is_reachable_from_no_remote_branch (__main__.ReachabilityTest.test_r24_the_authorized_commit_is_reachable_from_no_remote_branch)
A commit that exists only locally, with the remote still at the base commit. ... ok
test_reads_the_default_branch_from_the_remotes_own_head (__main__.ReachabilityTest.test_reads_the_default_branch_from_the_remotes_own_head) ... ok
test_refuses_a_commit_that_is_not_present (__main__.ReachabilityTest.test_refuses_a_commit_that_is_not_present) ... ok
test_refuses_a_remote_that_has_no_references (__main__.ReachabilityTest.test_refuses_a_remote_that_has_no_references) ... ok
test_refuses_a_revision_name_rather_than_a_hash (__main__.ReachabilityTest.test_refuses_a_revision_name_rather_than_a_hash) ... ok
test_refuses_an_abbreviated_commit (__main__.ReachabilityTest.test_refuses_an_abbreviated_commit) ... ok
test_refuses_an_uppercase_commit (__main__.ReachabilityTest.test_refuses_an_uppercase_commit) ... ok
test_refuses_when_the_clone_holds_no_release_bearing_reference (__main__.ReachabilityTest.test_refuses_when_the_clone_holds_no_release_bearing_reference) ... ok
test_refuses_when_the_default_branch_cannot_be_determined (__main__.ReachabilityTest.test_refuses_when_the_default_branch_cannot_be_determined) ... ok

----------------------------------------------------------------------
Ran 22 tests in 15.406s

OK
exit=0
```

## What each refusal test asserts

Every refusal scenario asserts **two** things, not one: a failing exit status, *and* a message naming
the fact that could not be established. Asserting only the status would pass a gate that refuses
everything for one reason — which is precisely the fail-open-in-reverse defect that the pre-existing
candidate's malformed-front-matter handling would have produced (see `candidate-conformance.md`).

Every scenario additionally asserts that the run left the repository untouched. That is
`VER-MOK-008`'s refusal-totality property, applied across the whole set rather than written once:
`assert_authorized` and `assert_refused` both snapshot `git status --porcelain --untracked-files=all`,
`git tag --list` and `git branch --list` before and after, and compare. A gate that refused correctly
but created a tag on the way would fail all 48 tests, not one.
