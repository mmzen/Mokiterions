"""Oracle 5's governance half, re-derived with the merge's base rather than the branch's.

Usage, from the repository root:

    python docs/engineering/simulation/evidence/WO-MOK-011/merge/amendments-vs-master.py \
        > docs/engineering/simulation/evidence/WO-MOK-011/merge/amendment-approvals.md

`analysis/amendments.py` measures every "unchanged since the base" property against
`524a6758d74b5240079959e9827ea40a7af22a30`, the commit this branch started from. That is the right
base for the packet and the wrong one for the merge: `master` at `2157f77` has itself amended
`SPEC-MOK-001`, `003` and `004`, added `VREC-MOK-007` and `RLS-MOK-001` and rewritten
`SPEC-MOK-002`'s provenance since that commit, so run unchanged against the merged tree the script
reports **FAIL** on five rows that describe `master`'s work and not this work order's. The failure
is real as stated and misleading as read: what the merge has to show is not that those files still
match a commit `master` has moved past, but that this work order still appends and never edits ---
now measured against the tree it is merging into.

This wrapper changes exactly one thing, the base revision, and re-runs the script whole: the same
self-tests, the same five checks, the same report text. Nothing is relaxed and no check is skipped.
Both reports are retained --- `analysis/amendment-approvals.md` at the branch base, this one at the
merge base --- because the pair is what shows the difference between them is `master`'s and not this
work order's.
"""

import os
import runpy
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SCRIPT = os.path.join(os.path.dirname(HERE), 'analysis', 'amendments.py')
MASTER = '2157f77'

module = runpy.run_path(SCRIPT, run_name='amendments')
module['BASE'] = MASTER

# `main` closed over the module's globals when it was defined, so assigning into the dictionary
# `run_path` returns is what rebinds BASE for it. Verified by the header line of the report, which
# prints BASE, and by the section 4 and 5 tables, which read it.
main = module['main']
main.__globals__['BASE'] = MASTER
sys.exit(main())
