# N1 - the declaration is closed

The exemption set resolved by the adopted evaluator from committed artifact content alone.

```text
exemptions: {'RLS-MOK-001': 'WO-HUP-001'}
defects:    ()
undeclared: ()
```

## The two instants

| Fact | Value |
|---|---|
| `RLS-MOK-001` `released_at` | `2026-08-19T17:53:05Z` |
| `WO-HUP-001` draft-to-approved `decided_at` | `2026-08-28T20:30:00Z` |

The approval is later than the release, which is what `SPEC-HUP-001` rule 6 requires.
`RLS-MOK-001` resolves to `WO-HUP-001`; no defect and nothing undeclared.
