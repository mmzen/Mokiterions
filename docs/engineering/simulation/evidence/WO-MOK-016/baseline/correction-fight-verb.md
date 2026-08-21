# A corrected census column: `verb:fight`, not `verb:observe`

| Field | Value |
|---|---|
| What was wrong | `retain.py`'s named list of the seven targeted verbs held `observe`, which `SPEC-MOK-001` rule 21 does not define, and omitted `fight`, which it does |
| Where it showed | `baseline/census.txt`, all 90 rows: a `verb:observe=0` column that could never be anything but zero, and no `fight` column at all |
| Found | 2026-08-20, while reading rule 21's verb table to write `branches.md` |
| Fixed | `retain.py`'s `NEW_VERBS`, and `census.txt` regenerated from a reproduction of the pre-change capture |
| Verdict | **the claim of absence now covers all seven verbs**, and the 90 rows are otherwise unchanged |

## 1. Why a wrong name matters here

`census.txt` exists to make a **zero** visible. `retain.py` says so: the verbs are "counted by name
rather than discovered, so that a zero is reported as a zero instead of vanishing from the census", and
the load-bearing sentence in its docstring is that on this side of the change "no `target:` field, no
`suffered:` field and none of the seven targeted verbs occurs anywhere in the 110 MB". A claim of
absence is only as good as the list it enumerates. With `observe` in place of `fight` the file asserted
a zero for a verb the engine has never had — a column that is zero by construction and cannot fail —
and asserted nothing whatever about `fight`, which is `SPEC-MOK-001` rule 21's fifth targeted verb and
the one `REQ-MOK-057` branch 1 proposes at `fear` below `30`.

Nothing in the measurement was wrong: `fight` genuinely does not occur in the pre-change capture,
because `Action::Fight` does not exist at `39662d1`. What was wrong is that the file did not check it.
That distinction is the whole reason the correction is recorded rather than quietly applied: the figures
stand, the check did not.

The seven verbs, from rule 21's own table, are `approach`, `avoid`, `threaten`, `attack`, `fight`,
`retreat` and `surrender`. `mokiterions-core/tests/decisions.rs`'s `TARGETED_VERBS` has always carried
that list correctly, which is why the "every verb applies" oracle was unaffected; the defect was in this
retention script alone.

## 2. The regeneration, and what it also proves

`census.txt` is derived from a 110 MB capture that is not retained, so correcting it means reproducing
that capture. The reproduction is the one `capture-state.txt` §2 states, re-run on 2026-08-20:

    $ git archive 39662d13abd08e3410648d1c59ad38384f8ad2d2 | tar -x -C <scratch>/recap-tree
    $ cd <scratch>/recap-tree && cargo build --release --offline -p Mokiterions
        Finished `release` profile [optimized] target(s) in 2.97s
    $ bash <repo>/docs/engineering/simulation/evidence/WO-MOK-016/capture.sh <scratch>/recap
    captured 90 streams into <scratch>/recap
    $ python baseline/manifest.py <scratch>/recap <scratch>/recap-manifest.txt "<label>"
    reproduction …: 90 cells, exit codes ['0']

Compared against the committed `pre-manifest.txt` on all four columns at once — raw SHA-256, byte count,
line count and exit code:

| | |
|---|---:|
| rows in `pre-manifest.txt` | 95 |
| reproduced identically | **95** |
| mismatched | **0** |

Then `retain.py` was re-run over the reproduction and its whole output diffed against what this
directory holds:

| Retained artifact | Result |
|---|---|
| `init/` — 90 initialization blocks | **byte-identical**, all 90 |
| `summary.txt` — rule 18's final line per cell | **byte-identical** |
| `full/seed42-baseline-d0.75-traceon.txt` — 414 KB whole cell | **byte-identical** |
| `census.txt` | 90 rows differ, **by exactly one column each** |

The 90 census rows were then compared token by token as sets, which is the check that matters, because
a corrected list could have moved a count as well as a name:

    90 cells: removed ['verb:observe=0'] added ['verb:fight=0']

One token out, one token in, in every row, and **no other token in the file changed** — not an event
count, not a core verb count, not `field:target=0`, not `field:suffered=0`. So this correction is a
correction of the enumeration and of nothing else, and the 2026-08-20 reproduction is a second
independent confirmation of `capture-state.txt`'s result: the pre-change capture is still exactly the
output of `39662d1`, now measured at two dates, from two archived trees, with the same 90 digests.

## 3. What this does not change

- The comparison oracle 1 rests on is `post/byte-identity.txt`'s 30 `baseline` cells, which is taken
  against `pre-manifest.txt`. That file is untouched here and its digests are re-confirmed above.
- The census's value on this side of the change is unchanged in substance: every targeted verb and both
  new fields are absent from all 90 cells. Only now all seven are named.
- The post-change side of this claim is `post/social-manifest.txt` and the `social` census, where these
  verbs are expected to be **non**-zero; `runs.md` measures them, and it is the same seven-name list.
