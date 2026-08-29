# Handoff checkpoint evidence

Retained for the `in_progress` to `implemented` transition of `WO-MOK-032`.

artifact: WO-MOK-032
checkpoint: handoff
formal_snapshot_sha256: c817d322fabc15678d56def4c6d1b4898cf6a96a3c7b406a0acb0f232ae6de53

Assurance is `required`, decided by the engineering owner, so this work order does not stop at
`implemented`: a verification record covers it and carries the one disclosed deviation.

## The declared change set

```text
  docs/engineering/simulation/evidence/WO-MOK-032/handoff-check.md
  docs/engineering/simulation/evidence/WO-MOK-032/mutation-check.md
  docs/engineering/simulation/evidence/WO-MOK-032/test-run.md
  docs/engineering/simulation/work-orders/WO-MOK-032.md
  mokiterions-core/tests/cli.rs
  mokiterions-core/tests/connector.rs
```

Two test files and this work order's own governance. **No `src/` file in either package**, no
manifest, no workflow, and no other governance artifact. The formal snapshot is taken over the
repository's artifacts and not over retained evidence, so naming this file in its own change set does
not move the digest above.

## The deviation this hands off

`VER-MOK-018` case `L20` asks that "an empty or malformed credential is treated as absent". The empty
half is asserted by `an_empty_credential_is_treated_as_absent`. The malformed half is **not covered**,
by the engineering owner's decision of 2026-08-29 taken with the cost measured: a non-Unicode value
has no portable constructor, so reaching it needs the first `cfg(windows)` or `cfg(unix)` in either
package plus an `OsStr`-taking helper signature, in order to enter the same single catch-all arm in
test-support code that the empty case already enters. Rule 13.4 forbids either host to read the
credential, so no engine code distinguishes the two.

A verification record covering this work order must therefore record `L20` as satisfied **in part**,
with the malformed-credential clause disclosed rather than claimed.
