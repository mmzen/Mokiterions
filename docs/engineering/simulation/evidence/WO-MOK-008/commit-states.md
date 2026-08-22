# The three states of `MOKITERIONS_COMMIT`

`SPEC-MOK-003` rule 8 clause 2 fixes the candidate commit as a compile-time read, and as amended on
2026-08-22 it adds that **supplied means non-empty**. That makes three states of the variable into two
outcomes, and the middle state is the one the superseded renderer got wrong: `option_env!` reports a
variable set to the empty string as `Some("")`, which was drawn as a bare `#` carrying no value at all.

## What the recompile question turned out to be

`WO-MOK-008` requires the three states measured "with a forced recompile between values", on the reasoning
that nothing declares the variable to cargo — there is no build script, which the work order puts out of
scope, so there is no `rerun-if-env-changed`.

**That reasoning was wrong, and the correction is a measurement rather than an argument.** `rustc` records
every variable an `env!` or `option_env!` expansion reads in the crate's dependency information, and cargo
invalidates on it. Each state below was therefore run twice — once after `cargo clean -p mokiterions-tui`
and once with no clean at all — and the two agree in all three cases. **No forced recompile is needed to
reach any state, and no build script would help.** The forced recompile the work order asks for was run
regardless; the unclean run is retained beside it because a measurement that agrees with a clean one is
worth more than an assumption in either direction.

## Result

| Variable | `option_env!` reads | Field at `160 × 48` | Row at `34 × 22` |
|---|---|---|---|
| unset | `None` | absent | `s0 t100 d0.75% reference @0 e136` (32) |
| set to `""` | `Some("")` | **absent** — clause 2 as amended | `s0 t100 d0.75% reference @0 e136` (32) |
| set to a 40-character SHA-1 | `Some("0123…4567")` | `commit 0123456789abcdef0123456789abcdef01234567` | `s0 t100 d0.75% reference @0 e136` (32) |

**The floor row is identical in all three states.** That is clause 4's first row working: the candidate
commit is shed before every field rule 8 requires, so a stamp of any length costs the row nothing at the
narrowest declared viewport. It is also the answer to the work order's completion-report item 5, and
`completion-report.md` states it there as a measurement.

## Full run

```
The three states of MOKITERIONS_COMMIT, each measured at a rendered frame.

SPEC-MOK-003 rule 8 clause 2 fixes a compile-time read, and WO-MOK-008 puts a build script out
of scope, so nothing in this repository declares the variable to cargo. Whether cargo notices a
change to it is therefore a property of the toolchain rather than of anything here, and it is
measured below rather than assumed: each state is run twice, once after `cargo clean -p
mokiterions-tui` and once with no clean at all.

MOKITERIONS_COMMIT unset
  after cargo clean -p mokiterions-tui: option_env! = None
  with no clean at all               : option_env! = None
  the two agree, so the change was picked up without a forced recompile
  160 x 48  |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols
  34 x 22   |s0 t100 d0.75% reference @0 e136|  32 cols

MOKITERIONS_COMMIT set to the empty string
  after cargo clean -p mokiterions-tui: option_env! = Some("")
  with no clean at all               : option_env! = Some("")
  the two agree, so the change was picked up without a forced recompile
  160 x 48  |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136|  70 cols
  34 x 22   |s0 t100 d0.75% reference @0 e136|  32 cols

MOKITERIONS_COMMIT set to a 40-character value (0123456…)
  after cargo clean -p mokiterions-tui: option_env! = Some("0123456789abcdef0123456789abcdef01234567")
  with no clean at all               : option_env! = Some("0123456789abcdef0123456789abcdef01234567")
  the two agree, so the change was picked up without a forced recompile
  160 x 48  |seed 0  ticks 100  density 0.75%  source reference  tick 0  events 136  commit 0123456789abcdef0123456789abcdef01234567|  119 cols
  34 x 22   |s0 t100 d0.75% reference @0 e136|  32 cols

What this measures

  rustc records every variable an env!/option_env! expansion reads in the crate's dependency
  information, and cargo invalidates on it, so the three states above are reached by changing
  the variable alone. The forced recompile WO-MOK-008 requires was run regardless, and the
  unclean run is retained beside it because a measurement that agrees with a clean one is worth
  more than an assumption in either direction. No state needs a build script to be reached.
```
