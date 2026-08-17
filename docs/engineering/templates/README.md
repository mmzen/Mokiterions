# Artifact Templates

Prefer `harnessctl scaffold-domain . --domain <domain>` and `harnessctl create-artifact . --domain <domain> --type <type> --id <ID>`. The commands validate the destination and create only an incomplete `draft`; complete its accountable metadata and body, then run the validator.

Canonical formal-artifact locations below `docs/engineering/<domain>/` are:

| Type | Directory |
| --- | --- |
| `intent` | `intent/` |
| `capability` | `capabilities/` |
| `requirement` | `requirements/` |
| `specification` | `specifications/` |
| `architecture` | `architecture/` |
| `adr` | `architecture/adr/` |
| `verification` | `verification/` |
| `work_order` | `work-orders/` |
| `verification_record` | `verification-records/` |
| `release_contract` | `release/` |
| `release_record` | `releases/` |
| `operating_contract` | `operations/` |

Use `evidence/` for retained work-order evidence and `acceptance/` for Gherkin scenarios. Stable metadata and typed relations remain authoritative; legacy flat artifacts remain valid and are never moved by upgrade.

Templates are intentionally excluded from active-artifact validation.

Use `VERIFICATION_RECORD.template.md` and `RELEASE_RECORD.template.md` only for commit-bound assurance and release instances. Prefer `harnessctl capture-verification` and `harnessctl prepare-release`, which derive the candidate commit safely, choose a domain-aware destination, and always produce `ready` records.
