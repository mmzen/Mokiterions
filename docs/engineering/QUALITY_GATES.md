# Quality Gates

| Gate | Required evidence |
|---|---|
| G0 Intent | Approved intent, capability, and requirements |
| G1 Definition | Active specification and independent verification cover each active requirement |
| G2 Architecture | Applicable architecture and ADR constraints are approved |
| G3 Work authorization | One bounded work order references the complete chain and phase-appropriate preflight passes |
| G4 Verification | Repository checks pass, evidence is retained, and a verified `VREC-*` binds the clean candidate commit |
| G5 Release and operation | A released `RLS-*` binds the same commit and accountable owners accept release and operating contracts |

Missing or externally held evidence is `not assessable`, not implicitly satisfied. Do not replace these gates with an aggregate health score.
