# Determinism rehearsal — `SPEC-MOK-005` rule 8.5

Captured 2026-08-19. Rehearsal, not observation: this is the `verify` job's determinism step run
verbatim on a developer machine, because the repository has never released and no run of the workflow
has ever occurred. `VER-MOK-008` records that distinction as a residual uncertainty and it is real.

Rule 8.5 asks for **three** comparisons per declared decision policy — the byte stream, the final
state, and the exit code. An earlier draft of the step made one. All three are now compared:

- the byte stream, by `cmp` over the two transcripts;
- the exit code, captured per run into its own file and `cmp`d, rather than left to `set -e` to
  notice;
- the final state, which *is* the trailing `summary reason=… ticks=… survivors=…` line, so it is
  inside the byte comparison — and is additionally printed, so a reader of the log sees the state
  rather than being told it matched.

`--policy <baseline|reference>` is the whole declared policy set (`mokiterions-core/src/cli.rs`), and
both are deterministic given a seed, so both are exercised. The `individual` policy exists only on the
unmerged `feature/phase-2-individuality` branch and is deliberately not referenced here.

## Transcript

```text
Rehearsal of the verify job's determinism step, run verbatim outside CI.

transcripts written to $out = /tmp/tmp.mqao7Qz0E8 (outside the checkout)
baseline: identical over 3029 lines, exit 0, final state:
summary reason=extinction ticks=119 survivors=0 deaths=12 territory_a=0 territory_b=0 food_a_low=21 food_a_medium=20 food_a_high=20 food_b_low=21 food_b_medium=20 food_b_high=20
  sha256 a54bf414b51bc6581f17b4193828439920266c4262f0f5d1fca1d02c9c425f6d
reference: identical over 7592 lines, exit 0, final state:
summary reason=tick_limit ticks=300 survivors=12 deaths=0 territory_a=4 territory_b=8 food_a_low=18 food_a_medium=11 food_a_high=31 food_b_low=6 food_b_medium=10 food_b_high=31
  sha256 6126765886b32dd5aa950ed51fe7e3776ac844c9d6e1c146f5885eadc8325147

cross-policy: the two policies must NOT be identical, or the flag would mean nothing
  differ, as declared:
    1c1
    < summary reason=extinction ticks=119 survivors=0 deaths=12 territory_a=0 territory_b=0 food_a_low=21 food_a_medium=20 food_a_high=20 food_b_low=21 food_b_medium=20 food_b_high=20
    ---
    > summary reason=tick_limit ticks=300 survivors=12 deaths=0 territory_a=4 territory_b=8 food_a_low=18 food_a_medium=11 food_a_high=31 food_b_low=6 food_b_medium=10 food_b_high=31

working tree unchanged by the step:
  yes -- git status is identical before and after
```

## What it establishes, and what it does not

| Claim | Status |
| --- | --- |
| One seed, one binary, one platform → identical bytes, per policy | established, twice per policy |
| Identical exit status per policy | established |
| The final state is stated in the log, not merely compared | established |
| The policy flag changes the outcome | established — `baseline` goes extinct at tick 119, `reference` survives all 300 |
| Cross-platform or cross-build determinism | **not established** |

The step proves determinism *within* one build of one binary. Bit-identical output across machines
would need a committed golden record, which is a specification question and not something CI may
decide; `INT-MOK-007` explicitly does not claim it. The comment in the step says so, so that a later
reader does not mistake the check for more than it is.

The cross-policy comparison is worth keeping for a different reason: if both policies produced the
same transcript, the two `cmp` calls would still pass and the check would be measuring nothing. They
differ at the level of the outcome itself — one population dies out, the other does not.

## The step writes outside the checkout

The transcripts are written to `$RUNNER_TEMP` (falling back to `mktemp -d` outside CI), not into the
working tree. An earlier draft wrote `first-baseline.txt` and three siblings into the checkout root.
Nothing tracked would have changed, but the `verify` job would have left four files behind, and
`VER-MOK-008` S6 asks whether checks write to the tree. They now do not, and the step ends with a
`git status --porcelain --untracked-files=all` — logged, and explicitly *not* a check — so a future
regression appears in the transcript instead of being inferred. The rehearsal above confirms `git
status` is identical before and after.
