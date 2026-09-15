# B3, P1, P2 - the repair changed only what it declared, and its scope is its scope

`git diff main -- docs/engineering/simulation/work-orders/WO-MOK-027.md` on 2026-09-15:

```diff
diff --git a/docs/engineering/simulation/work-orders/WO-MOK-027.md b/docs/engineering/simulation/work-orders/WO-MOK-027.md
index e9dd538..df968d8 100644
--- a/docs/engineering/simulation/work-orders/WO-MOK-027.md
+++ b/docs/engineering/simulation/work-orders/WO-MOK-027.md
@@ -5,7 +5,7 @@ title = "Stage 5c: the authorized measurement — the model-backed source publis
 status = "approved"
 owners = ["engineering owner"]
 created = "2026-08-23"
-updated = "2026-08-29"
+updated = "2026-09-15"
 
 [assurance]
 commit_bound_verification = "required"
@@ -26,6 +26,14 @@ implements = ["REQ-MOK-075", "REQ-MOK-076"]
 specifications = ["SPEC-MOK-001", "SPEC-MOK-007"]
 verification = ["VER-MOK-018"]
 architecture = ["ARCH-MOK-001", "ADR-MOK-007"]
+[[lifecycle_events]]
+from = "draft"
+to = "approved"
+decided_at = "2026-09-15T18:46:34Z"
+decided_by = "engineering-owner"
+reason = "Owner approval of remaining execution under DECISION_RIGHTS.md#approved-execution (DR-015), recorded under WO-HUP-003 on 2026-09-15 by the repository owner acting as engineering owner, by selecting the presented option 'Repair it here'. This work order was approved on 2026-08-23 in the owner's words 'i approve the 3 work orders' (ADR-MOK-007 act 12) under a 0.4.0 root that wrote no lifecycle events, and gained its execution scope on 2026-08-28 under WO-HUP-002; the 0.18.0 evaluator reads the execution grant from a scoped approval event and refused the start checkpoint with WEX-ECP-022. The edge is draft to approved because that is the only edge into approved the evaluator admits; the date is the date of this decision, not of the original approval, which stands unchanged. Nothing else about this work order moves: its sequencing gate on WO-MOK-026's cache-ratio result and the separate owner authorization each live run needs are untouched by this event."
+scope_paths = ["docs/PHASE_5_MEASUREMENT.md", "docs/engineering/simulation/evidence/WO-MOK-027/", "docs/engineering/simulation/intent/INT-MOK-001.md", "docs/engineering/simulation/work-orders/WO-MOK-027.md", "scripts/"]
+
 +++
 
 # Work Order: Stage 5c — the authorized measurement
@@ -339,3 +347,23 @@ this work order's surface states positively that it changes neither. A scope wid
 defeat the boundary it exists to be.
 
 Nothing else in this artifact moves.
+
+**2026-09-15, one scoped approval event added, under `WO-HUP-003`, by the engineering owner.**
+
+The 0.18.0 evaluator adopted under `WO-HUP-003` reads a work order's execution grant from its approval event — `to = "approved"`,
+`decided_by = "engineering-owner"`, and `scope_paths` — and refused this work order's start checkpoint with
+`QGP-G3-SCOPE: WEX-ECP-022: WO-MOK-027.md has no recorded engineering-owner approval`, before and after the adoption
+transaction. The cause is the same one `WO-HUP-002` met on 2026-08-28 in a different field: this work order was approved on
+2026-08-23 under a 0.4.0 root that wrote no lifecycle events, so the approval exists as prose in `ADR-MOK-007` act 12 and as
+`status = "approved"`, and nowhere the adopted evaluator reads.
+
+The adopted `DECISION_RIGHTS.md#approved-execution` names the repair: *an older approval without that grant requires owner
+approval of remaining execution through the existing amendment process*, and *historical records are not rewritten*. So the
+2026-08-23 approval is not back-dated into an event it never had. One new `[[lifecycle_events]]` table is added, dated at the
+owner's decision of 2026-09-15, decided by `engineering-owner`, carrying the five paths of the `[execution_scope]` table exactly
+as `WO-HUP-002` set them. Its edge is `draft` to `approved` because the evaluator admits no other edge into `approved` — an
+`approved` to `approved` event is `E014: unsupported transition`, measured in the adoption's rehearsal. `updated` moves to
+2026-09-15. **No other field moves**: not `status`, not a relation, not the scope, not one word of the body. The two gates the
+*Lifecycle* section states — `WO-MOK-026` verified with its cache-ratio result passed, and a separate owner instruction for
+every live run — are not lifted by this event and could not be. `WO-HUP-003`'s evidence `b2-start-after.md` holds the
+start-checkpoint reading after this amendment.
```

**B3.** One `[[lifecycle_events]]` table, one `## Amendment record` entry, and `updated`. `status` stays `approved`; no relation, no assurance field, no scope path and no word of the body moves.

**P1 and P2.** The event's `scope_paths` are the five entries of the unchanged `[execution_scope]` table, in the same order, so the scope the owner approved on 2026-09-15 is exactly the scope `WO-HUP-002` derived from this work order's own surface on 2026-08-28, neither wider nor narrower. The evaluator checks the equality itself: `WEX-ECP-022: scope changed since owner approval` is what it reports when they differ, measured in the rehearsal with a deliberately partial list.

Among governed artifacts outside the `harness` domain, the diff of the whole change against `main` touches exactly this one.
