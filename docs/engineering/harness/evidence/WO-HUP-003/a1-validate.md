# A1 - the graph validates under the adopted evaluator

Evaluator: se-harness 0.18.0 from the exact public wheel, outside the checkout, `python -I -m se_harness`. Captured 2026-09-15 on the working tree of branch `harness/wo-hup-003-adopt-se-harness-0.18.0` after all three steps and the three hand edits, before the implementation commit.

**212 artifacts, 0 errors, 0 warnings, 0 advisories.** `REQ-HUP-001`'s measure is the error count, and it is 0. `VER-HUP-002` B5 asks for the full totals as printed, and they are printed below.

Under the 0.8.0 evaluator the same tree read 0 errors and 155 warnings, all `W-AUT-*` authoring advisories; 0.18.0 reports those as advisories and, over this tree, reports none. That is a change in what the evaluator says, not in any artifact.

    harnessctl validate .

```text
Engineering artifact validation: PASS
Artifacts: 212 | Errors: 0 | Warnings: 0 | Advisories: 0
Planes: structure E0/W0 | governance E0/W0 | policy E0/W0 | maintenance E0/W0
```
