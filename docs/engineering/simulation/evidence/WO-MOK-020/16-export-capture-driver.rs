//! The export capture driver: the observer's own export file, before and after.
//!
//! This file is **not** part of either commit. It is a measurement instrument, placed at
//! `mokiterions-tui/tests/capture.rs` in each tree for the duration of the capture and removed
//! afterwards. It is retained here so the digests in `17-export-unmoved.txt` are recomputable:
//!
//!     cp docs/engineering/simulation/evidence/WO-MOK-020/16-export-capture-driver.rs \
//!        mokiterions-tui/tests/capture.rs
//!     MOK_EXPORT_CAPTURE=<path> cargo test --locked -p mokiterions-tui --test capture \
//!        -- --nocapture
//!     rm mokiterions-tui/tests/capture.rs
//!
//! It reaches the observer only through items that were already public before this work order, so
//! the identical file compiles against the base commit and against the candidate.
//!
//! `VER-MOK-017` O18 asserts inside the suite that the export equals the engine's own records with
//! the retention footer and carries no phrase that belongs to the pane. This driver exists for the
//! other half of the same check: it writes the file an operator would get, so the bytes themselves
//! can be hashed at both trees rather than only asserted about at one.

use mokiterions_tui::export;
use mokiterions_tui::options::{self, Startup};
use mokiterions_tui::state::Observer;

/// Writes the observer's export for a fixed run to the path named by `MOK_EXPORT_CAPTURE`.
///
/// The run is observed to the engine's own end before the export is taken, so the exported buffer
/// is the whole run's records and not a prefix of them. The inspector is not drawn: the export is
/// written from the retained event buffer, and whether a pane was on screen has no bearing on it -
/// which is itself part of what O18 is about.
#[test]
fn the_observers_export_for_a_fixed_run() {
    let destination = std::env::var("MOK_EXPORT_CAPTURE")
        .expect("MOK_EXPORT_CAPTURE names the file to write");
    let mut observer = match options::parse(vec![
        "--policy",
        "social",
        "--seed",
        "42",
        "--ticks",
        "200",
        "--start-paused",
    ])
    .unwrap()
    {
        Startup::Run(options) => Observer::new(options).unwrap(),
        Startup::Help => panic!("expected a run"),
    };
    while !observer.is_finished() {
        observer.advance().unwrap();
    }
    assert_eq!(observer.snapshot().tick, 200);
    export::write_file(&destination, observer.events()).expect("the export is written");
    println!(
        "wrote {destination} from {} retained records at tick {}",
        observer.events().len(),
        observer.snapshot().tick
    );
}
