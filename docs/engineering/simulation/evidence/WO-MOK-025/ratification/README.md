# The ratified `schema` increment, measured

`SPEC-MOK-006`'s 2026-08-21 amendment row stood **OUTSTANDING** from 2026-08-21 to 2026-08-23. The
repository owner ratified it on 2026-08-23, acting as accountable technical owner, deciding separately each
of the three things the row had left open. This directory holds the measurement of the one decision that
changes a byte the engine writes.

The row obliges the product change and the ratification to land in **one commit**, on `SPEC-MOK-005`'s
2026-08-20 precedent, "so that no commit carries one without the other". This directory is committed in that
same commit.

## The three decisions

1. **The `schema` increment is ratified.** `schema` becomes `2`. The product change is `RECORD_SCHEMA_VERSION`
   and the one asserted literal in the header unit test — exactly what the row predicted, verified by
   grepping the whole workspace for the value rather than trusting the prediction. The owner declined
   folding this increment together with the `llm` source's coming additions into a single move to `2`, so
   `WO-MOK-025` writes `3`.
2. **The nesting of a targeted verb's `target` inside `proposal` is accepted as emitted.** No product change.
3. **The unconditional presence of `suffered` is accepted.** No product change.

Decisions 2 and 3 change nothing measurable, because they ratify what the engine already does. Only
decision 1 is measured here.

## What was measured

The row's claim about decision 1 is narrow: `schema` becomes `2` and no other byte of any stream moves.
That is a claim about output, so it is measured against output rather than read off a two-line diff.

`verify-increment.sh` re-runs all forty cells of `REQ-MOK-068`'s matrix with a record sink and checks three
things per cell against the base-commit manifests taken at `cc5418553cb433715b7d6b15dea3886bff30ffaa`:

1. **The text stream's sha256 is unchanged.** `schema` appears in no text line, so movement here would mean
   the increment reached something it has no business reaching.
2. **The record stream's sha256 has moved.** If it had not, the increment did not take effect, and check 1
   would be passing for the wrong reason.
3. **The record stream with `"schema":2` reverted to `"schema":1` on its first line only is byte-identical
   to the base capture's record stream.** This is the load-bearing check. A differing digest names no byte;
   a reverted digest that matches names every byte at once.

`increment-confinement.txt` is the result: **forty cells, zero failures**, every one reading
`unchanged / moved / identical`. The script exits 1 on any failure, so it is a gate and not only a report.

Peak disk is one cell rather than the 183 MB a full sink capture occupies, because each stream is hashed and
deleted before the next runs. That is the only reason a check of this shape is affordable to commit.

## The schema-2 baseline

`sink-manifest.txt` is the full record-stream manifest at this tree — the base-commit tree plus the ratified
increment and nothing else. It exists because **`base/sink-manifest.txt` is now stale for record streams**:
its digests were taken while `schema` was `1`, and this ratification moves all forty of them.

The relationship between the two, measured:

| Comparison | Result |
| --- | --- |
| Text-stream digests, `base/nosink-manifest.txt` against this one | **40 identical, 0 differing** |
| Record-stream digests, `base/sink-manifest.txt` against this one | **0 identical, 40 moved** |

Both readings are what the confinement check predicts, arrived at independently of it.

**`base/sink-manifest.txt` is not corrected, and that is deliberate.** It is a base-commit capture; the base
commit emitted `schema` `1`, and rewriting it to say otherwise would make it describe a tree that never
existed. `WO-MOK-025`'s later `REQ-MOK-068` comparison of record streams is made against
`ratification/sink-manifest.txt`. The text-stream comparison is still made against
`base/nosink-manifest.txt`, which this ratification does not touch.

`base/full/seed42-baseline-traceon.jsonl` is likewise left at `schema` `1`. It is a retained base-commit
stream and its header is correct for the commit it came from.

## What the entropy check says about this

`the_four_existing_sources_draw_what_the_base_commit_drew` passes unchanged after the increment. That is not
a formality: it says the header write consumes no entropy draw, so the twenty configurations still reach
every tick boundary in the same state. Had the increment perturbed a draw, this test would have failed
before any capture was taken.

## What this does not establish

- **That `WO-MOK-025`'s value of `3` is correct.** It follows from this ratification and from rule 10.2, and
  it will be written and measured under that work order, not here.
- **That the record stream is otherwise conformant.** This measures one integer. `VER-MOK-012` oracle 3 is
  where the field set is checked, and it was run under `WO-MOK-019`.
- **Anything about the `llm` source.** No connector exists, no live run is authorized, and none was made.

## Line endings

Every file here is `LF` and contains no `CR`, matching the rest of this packet. `.gitattributes` exempts this
tree with `-text`, so the bytes written are the bytes hashed.
