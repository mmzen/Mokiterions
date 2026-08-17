# WO-MOK-004 evidence: the usage text before and after

`VER-MOK-004` requires the full pre-change and post-change usage text and a line-by-line diff of the
two, "so that the whole of the change to observable output is legible in one place". This is that
record. The raw diff is `usage-text.diff.txt`; the two full texts are `baseline/usage.txt` and
`after/usage.txt`.

## Conformance to the authorized rendering

`WO-MOK-004` fixed the exact target text before the work began. The emitted help output was extracted
from the work order and compared with the program's actual output:

```
diff <authorized rendering from WO-MOK-004> after/usage.txt
```

The two are **byte-identical**. Nothing in the shipped text was decided after approval.

## Measurements

| Measure | Before | After |
| --- | ---: | ---: |
| Lines | 12 | 25 |
| Non-blank lines | 10 | 21 |
| Bytes | 700 | 1326 |
| Widest line | 81 | 81 |
| Widest line the options block introduces | — | 80 |
| Declared defaults stated | 1 of 5 | 5 of 5 |
| `Default: ` statements | 0 | 4 |
| `defaults to` statements in prose | 1 | 0 |
| Carriage-return bytes | 0 | 0 |

The widest line is 81 columns before and after: it is the first line of the synopsis, which this work
order did not touch. `SPEC-MOK-001` leaves width unspecified and `WO-MOK-004` constrains only the lines
the options block introduces, whose widest is 80. Per-line widths of the introduced block:

```
  8  Options:
 65    --seed <u64>                   Entropy stream seed. Default: 0.
 73    --ticks <u64>                  Ticks to run; must be greater than zero.
 46                                   Default: 100.
 69    --policy <baseline|reference>  Decision source. Default: reference.
 76    --density <percent>            Resource density per territory, at most two
 63                                   decimal places. Default: 0.75.
 80    --trace-actions                Emit one action trace per living-agent decision
 63                                   opportunity. Off unless given.
 75    --help                         Print this usage and exit without running.
  0
 49  Options may appear in any order and at most once.
```

The **zero carriage-return bytes** matter more than they look. The constant is built with `concat!`,
one string literal per output line with an explicit `\n`, precisely because this repository is worked on
under Windows with end-of-line conversion: a multi-line string literal would have taken its line endings
from however the file was checked out, and the usage text would then differ between checkouts of the
same commit. The measurement above confirms the emitted bytes carry `\n` only.

## Before, in full

```text
Usage: Mokiterions [--seed <u64>] [--ticks <u64>] [--policy <baseline|reference>]
                   [--density <percent>] [--trace-actions]
       Mokiterions --help

The reference policy is a deterministic development instrument, not autonomous
behavior. It seeks and consumes perceived food so that world viability can be
measured. The baseline policy selects uniformly among valid actions.

--density is the percentage of a territory's cells that hold a resource, with at
most two decimal places. It defaults to 0.75. It sets the initial endowment, the
territory capacity, and the replenishment target together. Only the densities
declared in the requirements carry a population viability floor.
```

An operator reading this learns each option's name and the shape of its value. `--seed` and `--ticks`
appear only as type placeholders. `--trace-actions` is named and never explained. One default is stated,
in prose, in a sentence nothing held equal to the code.

## After, in full

```text
Usage: Mokiterions [--seed <u64>] [--ticks <u64>] [--policy <baseline|reference>]
                   [--density <percent>] [--trace-actions]
       Mokiterions --help

Options:
  --seed <u64>                   Entropy stream seed. Default: 0.
  --ticks <u64>                  Ticks to run; must be greater than zero.
                                 Default: 100.
  --policy <baseline|reference>  Decision source. Default: reference.
  --density <percent>            Resource density per territory, at most two
                                 decimal places. Default: 0.75.
  --trace-actions                Emit one action trace per living-agent decision
                                 opportunity. Off unless given.
  --help                         Print this usage and exit without running.

Options may appear in any order and at most once.

The reference policy is a deterministic development instrument, not autonomous
behavior. It seeks and consumes perceived food so that world viability can be
measured. The baseline policy selects uniformly among valid actions.

--density is the percentage of a territory's cells that hold a resource. It sets
the initial endowment, the territory capacity, and the replenishment target
together. Only the densities declared in the requirements carry a population
viability floor.
```

## Line-by-line change

```diff
@@ -2,11 +2,24 @@
                    [--density <percent>] [--trace-actions]
        Mokiterions --help
 
+Options:
+  --seed <u64>                   Entropy stream seed. Default: 0.
+  --ticks <u64>                  Ticks to run; must be greater than zero.
+                                 Default: 100.
+  --policy <baseline|reference>  Decision source. Default: reference.
+  --density <percent>            Resource density per territory, at most two
+                                 decimal places. Default: 0.75.
+  --trace-actions                Emit one action trace per living-agent decision
+                                 opportunity. Off unless given.
+  --help                         Print this usage and exit without running.
+
+Options may appear in any order and at most once.
+
 The reference policy is a deterministic development instrument, not autonomous
 behavior. It seeks and consumes perceived food so that world viability can be
 measured. The baseline policy selects uniformly among valid actions.
 
---density is the percentage of a territory's cells that hold a resource, with at
-most two decimal places. It defaults to 0.75. It sets the initial endowment, the
-territory capacity, and the replenishment target together. Only the densities
-declared in the requirements carry a population viability floor.
+--density is the percentage of a territory's cells that hold a resource. It sets
+the initial endowment, the territory capacity, and the replenishment target
+together. Only the densities declared in the requirements carry a population
+viability floor.
```

Two changes, and nothing else:

1. **Thirteen lines added** — the options block and the statement about order and repetition. The
   synopsis above them and the decision-source paragraph below them are untouched, which the diff shows
   as context rather than as change.
2. **The density paragraph re-wrapped**, with two facts removed from it: `with at most two decimal
   places` and `It defaults to 0.75.` Both are now stated in the `--density` entry. This is
   `SPEC-MOK-001`'s *state each fact once* clause being obeyed rather than a stylistic edit: leaving
   the sentence in place would have created exactly the second copy this requirement exists to prevent,
   and it is the one place in the old text where that copy already existed.

The paragraph's remaining three claims — what a density is, what it binds together, and which densities
carry a viability floor — are what an options entry cannot carry, so they stay.
