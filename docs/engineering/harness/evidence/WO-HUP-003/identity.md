# The evaluator's runtime identity

Captured 2026-09-15 at `654e3f6`. Every figure in this evidence directory was produced by this installation.

    harnessctl identity --role released-evaluator --expected-version 0.18.0 --expected-root <plugin data root>/verity-plane/evaluator --checkout-root . --require-isolated-python --evaluator-wheel-sha256 a683dbdf485d42aa20ea8502122c171a4c61c7d60f85db5b5f264bd336371c54 --json

**`passed`: True**; diagnostics: none.

| Field | Value |
|---|---|
| `schema` | `se-harness-runtime-identity-v4` |
| `role` | `released-evaluator` |
| `harness_version` | `0.18.0` |
| `isolated_python` | `True` |
| `python_version` | `3.14.6` |
| `python_binary_position` | `within-expected-root` |
| `pythonpath_present` | `False` |
| `user_site_enabled` | `False` |
| `evaluator_archive_name` | `se_harness-0.18.0-py3-none-any.whl` |
| `evaluator_archive_sha256` | `a683dbdf485d42aa20ea8502122c171a4c61c7d60f85db5b5f264bd336371c54` |
| `evaluator_wheel_sha256` | `a683dbdf485d42aa20ea8502122c171a4c61c7d60f85db5b5f264bd336371c54` |
| `evaluator_payload_manifest` | `se-harness-installed-payload-v1` |
| `evaluator_payload_sha256` | `cf28e03f21a0e474af8c415499c69193ab2a20e08a95c0ad3514758a84cff54d` |

The archive digest equals the SHA-256 of `se_harness-0.18.0-py3-none-any.whl` as published on the public index, downloaded independently on 2026-09-15 and hashed. The installation was made from that wheel file rather than from the index, so the lock the transaction writes can bind the digest; an index install records no archive digest at all. Machine paths are omitted here; the JSON result carries them.
