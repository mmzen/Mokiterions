# Evidence: `WO-MOK-012`, the optional structured record stream

This packet is the evidence `VER-MOK-012` requires for `WO-MOK-012`. It is written as the work proceeds
rather than assembled afterwards, so the reading order below is also the order the artifacts were produced.

## What is here now

| Path | What it is |
|---|---|
| `capture.sh` | The capture script for the whole matrix. Takes `<target-dir> [sink]`; in `sink` mode it additionally passes `--events-path`. |
| `analysis/digest.py` | Per-cell SHA-256, byte count and line count of standard output, standard error, exit code and — in sink mode — the record stream. |
| `analysis/retain.py` | Copies the stated subset of whole streams into the packet. |
| `baseline/COMMIT.txt` | The commit the pre-change capture was taken at. |
| `baseline/capture-state.txt` | The working tree's state at that commit, so that "before any code change" is checkable rather than asserted. |
| `baseline/pre-manifest.txt` | The digest manifest of all 90 pre-change cells. |
| `baseline/full/` | Three whole pre-change standard-output streams. |

Everything else `VER-MOK-012`'s *Evidence retention* list names is produced later in the work order and is
not here yet. This file is updated as each part lands; a section that is missing means the work is not done,
not that it was skipped.

## The capture matrix

Ninety cells: seeds `0 1 42 123 777`, policies `baseline reference individual`, densities `0.15 0.75 1.50`,
tracing off and on, `--ticks 1000` throughout.

`VER-MOK-012` declares sixty cells — the two densities `0.75` and `1.50` — and this capture is a **superset**,
adding `0.15`. That is the matrix `WO-MOK-011`'s capture used, and reusing it is what lets the re-run required
by *Evidence retention* compare against `WO-MOK-002`'s and `WO-MOK-011`'s retained captures cell for cell. A
superset satisfies the declared oracles and adds thirty more chances to fail them.

## Retention: what is kept whole, what is kept as a digest, and why

**This packet does not retain the captures whole, and `VER-MOK-012`'s *Evidence retention* list, read
literally, asks it to.** The deviation is stated here rather than left for a reader to infer from what is
absent.

The pre-change capture alone is 110 MB of standard output across its 90 cells. The list requires three
captures of that matrix — pre-change, post-change with no sink, post-change with a sink — plus, separately,
"one full sink stream per declared seed at the default density under each policy, with tracing off and on",
which is thirty more streams. Retaining all of that whole would put roughly a third of a gigabyte of
generated text into the repository, most of it three byte-identical copies of the same thing, since the whole
point of oracles 1 and 4 is that the three captures agree.

What is retained instead, following the form `WO-MOK-006`'s, `WO-MOK-007`'s and `WO-MOK-011`'s evidence
packets established for this oracle:

- **A digest manifest of every cell of every capture.** SHA-256, byte count and line count for each cell's
  standard output, standard error and exit code, and for its record stream where it has one. The comparison
  a reviewer must be able to make — "these two captures are byte-identical, and this third one differs
  exactly here" — is made on the manifests, and a digest comparison is stronger than an eyeball comparison of
  two 1.2 MB files, not weaker.
- **Three whole cells per capture**, at seed 42 and the default density, one per decision source, tracing off:
  `seed42-baseline-d0.75-traceoff`, `seed42-reference-d0.75-traceoff`, `seed42-individual-d0.75-traceoff`.
  These are the same three cells `WO-MOK-011` retained whole, so the two packets compare directly, and a
  reviewer who wants to read a complete 1,000-tick stream rather than trust a hash has three of them, and
  will have their record streams beside them.

Everything not retained is reproducible: `capture.sh` at the commit in `baseline/COMMIT.txt` reproduces the
pre-change capture, and at the candidate commit the two post-change captures. The manifests are what detect a
reproduction that failed.

**What this costs.** A reviewer cannot inspect an arbitrary cell's text without re-running the capture. That
is the cost, it is real, and it is accepted here because the alternative buys nothing a digest does not
already establish. Digests are taken over the bytes exactly as written — nothing is decoded, normalized or
newline-translated, because `SPEC-MOK-006` rule 11.1 admits no whitespace exemption and a normalizing
comparison would verify a weaker property than the one required.

## No secret is retained

No capture command carries a credential, and no retained artifact contains one. **No retained record stream
carries the path it was written to**, which `VER-MOK-012` names as the property that makes this evidence class
safe to retain at all: `SPEC-MOK-006` rule 5.5 keeps the sink's path out of the header record in every form,
and rule 3.2's closed value alphabet means no operator-supplied text can reach any field.
