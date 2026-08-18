# WO-MOK-003 evidence: compile-time cost

`SPEC-MOK-002`'s *Performance and capacity* section states that runtime behavior is unchanged and that
compile-time cost grows: the binary target links the library target, and each public-tier file becomes
an additional test target, so `cargo test` builds more artifacts and `target/` grows. `VER-MOK-003`
requires the growth to be measured and recorded.

**No obligation is attached to these numbers.** Neither `VER-MOK-001` nor `VER-MOK-002` imposes a
wall-clock target, and this work order does not introduce one. The measurement exists so that the
predicted cost is a known quantity rather than a shrug, and so a later change can be compared against
something.

## Method

Both trees were measured on the same machine, in the same session, with the same toolchain
(rustc 1.97.1, cargo 1.97.1), debug profile, no `--release`.

- **"Before"** is commit `77010d02319051a20f8e45282f9c813ce4199956` extracted with
  `git archive | tar -x` into a scratch directory. The working tree was never reverted to obtain it,
  so there was no path by which the pre-change baseline capture could be disturbed.
- **"After"** is the finished tree on branch `feature/library-target-and-test-placement`.
- Each measurement used its own `CARGO_TARGET_DIR`, outside both trees, so neither measurement
  benefited from the other's cache or from the repository's existing `target/`.
- **Cold** means the target directory was deleted immediately before the command.
- **Incremental** means the target directory was warm from a prior `cargo test --no-run` and
  `src/simulation.rs` was then touched — the ordinary edit-test loop, and the case where the extra
  targets cost the most, because every test target depends on that file.

Timings are a single run each. They are wall-clock milliseconds on a developer machine and should be
read as magnitudes, not benchmarks.

## Measurements

| Measurement | Before | After | Change |
| --- | ---: | ---: | --- |
| Cold `cargo build` | 1,165 ms | 1,755 ms | +590 ms, ×1.5 |
| Cold `cargo test --no-run` | 1,432 ms | 2,270 ms | +838 ms, ×1.6 |
| Incremental `cargo test --no-run` after touching `src/simulation.rs` | 554 ms | 1,006 ms | +452 ms, ×1.8 |
| Test executables built by `cargo test` | 1 | 7 | +6 |
| Executables in `target/debug/deps` after `cargo test --no-run` | 1 | 8 | +7 |
| `target/` size after cold `cargo test --no-run` | 26 MB | 88 MB | +62 MB, ×3.4 |

## What the numbers mean

**The test-executable count is the whole story.** Before, `cargo test` built one test binary: the unit
tests compiled into the `Mokiterions` bin target. After, it builds seven — unit tests for the library
target, unit tests for the binary target (which contains none, and so runs zero tests), and one per
public-tier file. Each of the five integration-test targets is a separate crate that compiles and
links the library from scratch, which is where both the time and the disk go.

The eighth executable in `deps` is the plain `Mokiterions` binary. Cargo links it during `cargo test`
now that integration-test targets exist, where before `cargo test --no-run` produced only the test
binary. It is not a new artifact of the build, only one produced at a different point.

**Disk grows faster than time**, ×3.4 against ×1.6, because five statically-linked test executables
each carry their own copy of the library's code and of the standard-library machinery they use.
62 MB of debug artifacts in a gitignored directory is not a cost worth engineering against.

**The incremental figure is the one a developer feels**, and it roughly doubled: half a second becomes
one second when `src/simulation.rs` changes. This is the honest downside of the split, and it is
proportionate to what was bought — 15 tests that now exercise the published interface the way an
external consumer would, and a public surface that is checked by the compiler rather than by review.

**One new step appears in `cargo test`.** Having a library target means cargo runs a doc-test pass. The
library has doc comments but no code blocks in them, so it reports `running 0 tests`. It costs
essentially nothing and is visible in `test-run.txt` as the eighth section.

## What did not change

- **Runtime.** `baseline-comparison.md` establishes byte-identical output across all 43 captured cells,
  and `resilience-and-viability.md` records 10,000-tick runs terminating at the same tick as before.
  Per-tick work, memory use, and output volume are untouched.
- **Dependencies.** `[dependencies]` is still empty, there is no `[dev-dependencies]` table, there is
  no build script, and `Cargo.lock` is unchanged. None of the added compile time is third-party code;
  all of it is this crate compiled more times.
- **The one-invocation property.** `cargo test` still runs everything, with no feature, no environment
  variable, no `#[ignore]`, and no particular working directory, as `SPEC-MOK-002` rule 10 requires.
