//! The per-tick cost driver: wall clock for the declared long run, before and after.
//!
//! This file is **not** part of either commit. It is a measurement instrument, placed at
//! `mokiterions-tui/tests/timing.rs` in each tree for the duration of the measurement and removed
//! afterwards. It is retained here so the figures in `15-per-tick-cost.txt` are recomputable:
//!
//!     cp docs/engineering/simulation/evidence/WO-MOK-020/15-per-tick-cost-driver.rs \
//!        mokiterions-tui/tests/timing.rs
//!     cargo test --release --locked -p mokiterions-tui --test timing -- --nocapture
//!     rm mokiterions-tui/tests/timing.rs
//!
//! It reaches the observer only through items that were already public before this work order, so
//! the identical file compiles against the base commit and against the candidate. That is the
//! point: the difference between the two runs is the accumulation and nothing else, because the
//! driver, the arguments, the tick count and the platform are the same on both sides.
//!
//! Drawing is deliberately not timed. `VER-MOK-017`'s per-tick cost check is about the
//! accumulation, which happens as the observer ingests a tick's records, and a frame is drawn on
//! the terminal's schedule rather than the engine's.

use mokiterions_tui::options::{self, Startup};
use mokiterions_tui::state::Observer;
use std::time::Instant;

/// The declared long run, seven times, reporting every measurement rather than a mean alone.
///
/// Seven repetitions with the extremes reported is what lets a reader judge whether a difference
/// between two trees is larger than the measurement's own spread. One figure from each tree would
/// not support that judgement, and stating a mean without a spread would hide it.
#[test]
fn wall_clock_for_the_declared_long_run() {
    const TICKS: u64 = 6600;
    const REPETITIONS: usize = 7;
    let args = vec![
        "--policy",
        "individual",
        "--ticks",
        "6600",
        "--start-paused",
    ];

    let mut measurements = Vec::new();
    for repetition in 0..REPETITIONS {
        let mut observer = match options::parse(args.clone()).unwrap() {
            Startup::Run(options) => Observer::new(options).unwrap(),
            Startup::Help => panic!("expected a run"),
        };
        let started = Instant::now();
        while !observer.is_finished() {
            observer.advance().unwrap();
        }
        let elapsed = started.elapsed().as_secs_f64();
        assert_eq!(
            observer.snapshot().tick,
            TICKS,
            "the run ended early, so the repetitions are not comparable"
        );
        println!(
            "repetition {}: {:.4} s, {:.3} us per tick",
            repetition + 1,
            elapsed,
            elapsed * 1_000_000.0 / TICKS as f64
        );
        measurements.push(elapsed);
    }

    measurements.sort_by(f64::total_cmp);
    let mean = measurements.iter().sum::<f64>() / REPETITIONS as f64;
    let least = measurements[0];
    let greatest = measurements[REPETITIONS - 1];
    let median = measurements[REPETITIONS / 2];
    println!(
        "ticks {TICKS}, repetitions {REPETITIONS}\nmean {mean:.4} s\nmedian {median:.4} s\nleast \
         {least:.4} s\ngreatest {greatest:.4} s\nspread {:.4} s, {:.1}% of the mean\nmean per tick \
         {:.3} us",
        greatest - least,
        (greatest - least) * 100.0 / mean,
        mean * 1_000_000.0 / TICKS as f64
    );
}
