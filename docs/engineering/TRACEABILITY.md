# Traceability

The normative chain is:

```text
Intent -> Capability -> Requirement -> Specification
                             |-> Architecture / ADR
                             |-> Verification
                                      |
                              Work order(s)
                                      |
                     Verification record + commit
                                      |
                  Release record + same commit
                                      |
                              Operating contract
```

Only declared relations in formal TOML metadata establish authority. Source comments, filenames, commits, tickets, and conversational references may aid discovery but do not satisfy formal coverage.

`verification_record` binds one or more release-bearing work orders, their declared verification contracts, and retained evidence to one clean final candidate commit. `release_record` binds a release contract to the same commit and an exact released-work set equal to included verification coverage. Single-work-order records remain aggregates of cardinality one.

Governance-only work may authorize review, verification transition, release transition, tagging, or publication in later commits, but it is not automatically release payload. The dashboard's observed checkout revision is derived context, not release authority.

A governance-only work order records completed execution as `implemented`. It does not become `verified` merely because it authorizes the transition of a different VREC; doing so would create recursive governance work. When configured provenance is required, a work order may claim `verified` or `released` only when a verified or released VREC explicitly includes it. VREC and RLS records, rather than work-order status alone, are the authoritative commit and release bindings.

A stale `ready` verification record may be retained as `superseded` only through a separate accountable governance decision. Its `superseded_by` relation names one distinct verified or released VREC that covers every original work order. Supersession preserves the old candidate and evidence facts, is terminal, and never contributes verification or release readiness. Dashboard overlap findings are non-authoritative prompts, not lifecycle decisions.
