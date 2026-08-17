# Quality Gates

| Gate | Required evidence |
|---|---|
| G0 Intent | Approved intent, capability, and requirements |
| G1 Definition | Active specification and independent verification cover each active requirement |
| G2 Architecture | Applicable architecture identifies its architecturally significant requirement drivers and conforming specifications; each has a valid decision assessment and each `adr_required` architecture has active deciding ADR coverage |
| G3 Work authorization | One bounded work order references the complete chain and phase-appropriate preflight passes |
| G4 Verification | Repository checks pass, evidence is retained, and a verified `VREC-*` binds the clean candidate commit |
| G5 Release and operation | A released `RLS-*` binds the same commit and accountable owners accept release and operating contracts |

Missing or externally held evidence is `not assessable`, not implicitly satisfied. Do not replace these gates with an aggregate health score.

## Validation assessment planes

Validator findings identify one explanatory plane: `structure` for formal graph shape, `governance` for non-waivable assurance invariants, `policy` for rules activated by explicit repository configuration, and `maintenance` for non-blocking compatibility or placement advice. The plane does not change error versus warning severity, lifecycle authority, or process exit behavior, and it is never an aggregate health score.
