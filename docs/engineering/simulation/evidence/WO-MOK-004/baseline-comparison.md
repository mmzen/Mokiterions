# WO-MOK-004 evidence: baseline comparison

`VER-MOK-004` states the expected change surface in advance and requires that a difference outside it
fails the contract. This is the comparison against that statement.

## Commit binding

| Fact | Value |
| --- | --- |
| Pre-change baseline captured at | `08dc8d47aa569c77fb7c9b091e328eb8f40c5285` (tip of `master`) |
| Source confirmed untouched at capture | `git status --short -- src tests Cargo.toml Cargo.lock` reported 0 lines |
| Baseline regenerated after capture | **No.** Captured once, before the first edit, and never re-run against a modified tree. |
| Capture script | `capture.sh`, byte-identical to the script used under `WO-MOK-003` (verified with `diff`), run unmodified for both sides |
| Toolchain | rustc 1.97.1, cargo 1.97.1, clippy 0.1.97, rustfmt 1.9.0-stable |

The baseline is the third of the three oracles `VER-MOK-004` names. It is the only one that can detect a
change this work order was not supposed to make, because the other two oracles look only at the help
text and would say nothing about a perturbed simulation.

## Result against the stated surface

| Baseline artifact | Cells | Required result | Observed |
| --- | ---: | --- | --- |
| Matrix manifest and summary lines | 43 | Byte-identical: digest, line count, exit code, full summary text | `diff baseline/manifest.txt after/manifest.txt` empty; `diff baseline/summaries.txt after/summaries.txt` empty; `diff -r baseline/full after/full` empty |
| `--help` case | 1 | Exit `0` and empty standard error unchanged; standard output is the new usage text | Exit `0`; standard output digest `6154a2ef…` → `f13bfeee…`, length 699 → 1325 bytes |
| Invalid-configuration cases | 14 | Exit `2` unchanged, standard output still empty, `configuration error: …` line byte-identical, appended usage block replaced | All 14: exit `2`, `stdout_len: 0`, diagnostic line identical; only the appended block differs |
| Valid single-tick case | 1 | Byte-identical in every field | Identical |

### Method for the diagnostic cases

`exit-codes.diff.txt` is the raw `diff` of the 16 named cases: 342 lines. Every changed line in it was
classified mechanically rather than by reading:

```
grep -E '^[<>]' exit-codes.diff.txt | grep -vE '^[<>]   \| '
< stdout_len:  699
< stdout_sha:  6154a2efe7f1773b850cf2751e7309883150be417ee5ea40d76029852b091249
> stdout_len:  1325
> stdout_sha:  f13bfeeed47885da2496aa0c582f42baa50153851b503b6e30400418a8451150
```

Standard-error text is captured with each line prefixed `  | `, so removing those lines leaves every
change that is *not* inside a usage block. Four lines remain, and all four are the `--help` case's
standard output. Nothing outside the stated surface changed.

The exit code and standard-output digest of each of the 16 cases were then extracted and compared
independently of the line-prefix argument above. One case differs and it is `help`, in standard output
only:

```
< === help    exit=0 len=699   sha=6154a2ef…
> === help    exit=0 len=1325  sha=f13bfeee…
```

The first standard-error line of each of the 14 invalid cases was extracted and compared on its own.
All 14 are identical, including the operator's rejected value where the message echoes it:

| Case | `configuration error:` line, unchanged |
| --- | --- |
| ticks zero | `--ticks must be greater than zero` |
| density zero cells | `invalid --density value: 0.01; resolves to zero resources per territory; the smallest usable density is 0.02` |
| density literal zero | `invalid --density value: 0; resolves to zero resources per territory; the smallest usable density is 0.02` |
| density precision | `invalid --density value: 0.751; at most two decimal places are accepted` |
| density negative | `invalid --density value: -1; expected digits and at most one decimal point` |
| density over 100 | `invalid --density value: 101; must not exceed 100` |
| density non-numeric | `invalid --density value: abc; expected digits and at most one decimal point` |
| policy invalid | `invalid --policy value: random; expected baseline or reference` |
| policy missing value | `missing value for --policy` |
| seed non-numeric | `invalid --seed value: abc` |
| unknown option | `unknown option: --unknown` |
| duplicate seed | `--seed may appear at most once` |
| duplicate trace | `--trace-actions may appear at most once` |
| ticks missing value | `missing value for --ticks` |

All 14 append the usage text, which is what makes this the larger half of the change surface: the help
path is one invocation an operator chooses, and the diagnostic path is every invocation an operator gets
wrong.

## Determinism

Two post-change runs of `--seed 42 --ticks 1000 --trace-actions` produce the identical digest
`ee2d6e7f…`. Equality with the pre-change bytes is established by the manifest comparison above rather
than by this pair, because the manifest cell is built by `printf '%s'` and so omits the trailing
newline a direct pipe keeps; the two constructions are not comparable to each other and neither is
wrong.

## What this comparison does not cover

- **Undeclared seeds and densities.** The matrix is the five declared seeds and the two declared
  densities. A help-text change has no mechanism for seed-dependent divergence, so this is the shape of
  the claim rather than a gap in executing it.
- **The release profile.** Everything here is the debug profile, as under `WO-MOK-003`.
- **Terminal rendering.** Line widths are measured; behaviour in a narrow window, under wrapping, or
  through a pager is not.
