# Q6: revisability, by digest

Retained rows before the edit: `e4fb3348aed830e61f55d9a81095b77744bd2efc1f8186afe6c7313ff5ff6e36`

Retained rows during the edit: `e4fb3348aed830e61f55d9a81095b77744bd2efc1f8186afe6c7313ff5ff6e36`

Retained rows after reverting: `e4fb3348aed830e61f55d9a81095b77744bd2efc1f8186afe6c7313ff5ff6e36`

The threshold edit:

```diff
--- scripts/classify_simulation_runs.py (before)
+++ scripts/classify_simulation_runs.py (after: collapse at a quarter of the roster instead of a half)
-return row["deaths"] * 2 >= row["roster"]
+return row["deaths"] * 4 >= row["roster"]
```

Classes before: `['coexistence', 'coexistence', 'coexistence', 'coexistence', 'coexistence', 'coexistence', 'coexistence', 'coexistence', 'coexistence', 'coexistence', 'coexistence', 'coexistence']`

Classes after: `['collapse', 'collapse', 'collapse', 'collapse', 'collapse', 'collapse', 'collapse', 'collapse', 'collapse', 'collapse', 'collapse', 'collapse']`

The distribution before the edit and the distribution after reverting it are byte-identical.
