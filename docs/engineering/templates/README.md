# Artifact Templates

Copy a template into the relevant product/domain directory, allocate a unique ID, complete the metadata and body, then run the validator.

Templates are intentionally excluded from active-artifact validation.

Use `VERIFICATION_RECORD.template.md` and `RELEASE_RECORD.template.md` only for commit-bound assurance and release instances. Prefer `harnessctl capture-verification` and `harnessctl prepare-release`, which derive the candidate commit safely and always produce `ready` records.
