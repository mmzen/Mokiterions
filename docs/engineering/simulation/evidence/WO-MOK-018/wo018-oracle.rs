//! TEMPORARY ORACLE for `WO-MOK-018`. Not part of the product.
//!
//! Placed in the tree, run once, removed, and retained as evidence, on the precedent of
//! `WO-MOK-013`'s `wo013-oracle.rs`, `WO-MOK-012`'s `assessment-oracle.rs`, `WO-MOK-010`'s
//! `observer/frame-probe.rs` and `WO-MOK-006`'s `frame-and-export-oracle.rs`. **It asserts
//! nothing.** It renders into an in-memory backend and writes down what the buffer holds, so that
//! judging what the inspector shows at death is a separate act from measuring it.
//!
//! It produces two of `WO-MOK-018`'s *Evidence to record* items:
//!
//!   * `inspector.txt` — the inspector pane for a dead subject at the reference viewport and at
//!     the smallest viewport presenting the pane, in both the reported-`fear` case and the
//!     no-`survival_changed`-record case, together with the interior width and the width the
//!     one-line form of the same four values would need. That last figure is the defect this work
//!     order's frame case found, measured rather than recounted.
//!   * `filter-vocabulary.txt` — `EventType::ALL` enumerated with its `as_str()` rendering and
//!     partitioned into `SPEC-MOK-001`'s fourteen stable core types and the one optional type,
//!     which is what `SPEC-MOK-003` rule 9 item 2's corrected figure is measured against.
//!
//! **It is a child module of `state` rather than of the crate**, which no earlier oracle needed to
//! be. The no-record case is constructible only by calling `Observer::ingest`, and that method
//! carries no visibility modifier, so it is private to the `state` module and reachable from that
//! module's descendants and nowhere else. `ARCH-MOK-002` forbids the two alternatives by name —
//! widening an item to reach it from a test, and a fifth `#[cfg(test)]` hook on the observer's
//! state type. **No item was widened for this file and no hook was added**, so `SPEC-MOK-004`
//! rule 6's interface is untouched by its presence, and rules 9 to 11's test totals were measured
//! on the tree without it.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use mokiterions::simulation::{Event, EventDetail, EventType};
use ratatui::Terminal;
use ratatui::backend::TestBackend;

use super::Observer;
use crate::layout;
use crate::options::{self, Startup};
use crate::render;

/// The reference viewport of `SPEC-MOK-003` rule 5.
const REFERENCE: (u16, u16) = (160, 48);

/// The smallest viewport presenting the inspector: the pane arrives at 140 columns and the floor
/// is 22 rows, so no smaller viewport in either axis carries it.
const SMALLEST_WITH_INSPECTOR: (u16, u16) = (140, 22);

fn write(name: &str, body: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the workspace root")
        .join("docs")
        .join("engineering")
        .join("simulation")
        .join("evidence")
        .join("WO-MOK-018");
    fs::create_dir_all(&root).expect("the output directory");
    let path = root.join(name);
    fs::write(&path, body).expect("writing the capture");
    path
}

fn observer(args: &[&str]) -> Observer {
    match options::parse(args.to_vec()).expect("the argument forms are the declared ones") {
        Startup::Run(options) => Observer::new(options).expect("an observer over a fresh run"),
        Startup::Help => panic!("expected a run"),
    }
}

/// The inspector pane's rows at one viewport, with the pane's own rectangle.
fn inspector(observer: &mut Observer, (width, height): (u16, u16)) -> (String, u16, u16) {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test backend");
    terminal
        .draw(|target| render::draw(target, observer))
        .expect("drawing into a buffer");
    let buffer = terminal.backend().buffer().clone();
    let area = layout::resolve(*buffer.area())
        .inspector
        .expect("this viewport presents the inspector");
    let rows = (area.y..area.y + area.height)
        .map(|y| {
            (area.x..area.x + area.width)
                .map(|x| buffer.cell((x, y)).expect("inside the area").symbol())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    (rows, area.width, area.height)
}

fn record(out: &mut String, banner: &str, observer: &mut Observer, viewport: (u16, u16)) {
    let (rows, width, height) = inspector(observer, viewport);
    writeln!(out, "--- {banner}").unwrap();
    writeln!(
        out,
        "    viewport {}x{}   inspector pane {}x{}   interior width {}",
        viewport.0,
        viewport.1,
        width,
        height,
        width.saturating_sub(2)
    )
    .unwrap();
    writeln!(out).unwrap();
    for row in rows.lines() {
        writeln!(out, "    {row}").unwrap();
    }
    writeln!(out).unwrap();
}

#[test]
fn capture_the_inspector_at_death() {
    let mut out = String::new();
    out.push_str(
        "WO-MOK-018 — the inspector pane for a dead subject\n\
         \n\
         Captured by a temporary oracle, which asserts nothing. Every row below is read out of a\n\
         rendered buffer cell by cell, including the pane's own border, so a value clipped off the\n\
         pane is visible here as an absent character rather than as a shorter string.\n\
         \n\
         `SPEC-MOK-003` rule 10 item 6 as amended 2026-08-21 fixes four presented values for a\n\
         dead subject and pairs them across two lines. Rule 10 item 7 makes an unreported value\n\
         absent rather than zero-filled. The two cases below are those two provisions.\n\n",
    );

    // Case A: a subject the engine reported survival for before it died, which is every death a
    // run produces. Nothing is constructed here — the run is the ordinary one.
    let mut run = observer(&["--policy", "baseline", "--ticks", "400", "--start-paused"]);
    while !run.is_finished() && run.deaths().is_empty() {
        run.advance().expect("the run advances");
    }
    let death = run
        .deaths()
        .first()
        .expect("the baseline policy starves its population well inside 400 ticks")
        .clone();
    run.select_for_test(&death.id);

    writeln!(
        out,
        "=== CASE A — the engine reported this subject's survival before it died\n\n    \
         subject {}   died on tick {}   health {}   satiety {:?}   energy {:?}   fear {:?}\n",
        death.id, death.tick, death.health, death.satiety, death.energy, death.fear
    )
    .unwrap();

    // The counterfactual the work order's frame case found: the same four values on one line.
    // This is the form the implementation carried before the pairing, and its width is why the
    // pairing exists. Measured, not recounted.
    let one_line = format!(
        "final health {}  satiety {}  energy {}  fear {}",
        death.health,
        death.satiety.unwrap_or(0),
        death.energy.unwrap_or(0),
        death.fear.unwrap_or(0)
    );
    writeln!(
        out,
        "    the one-line form of the same four values: {} columns\n    {:?}\n",
        one_line.chars().count(),
        one_line
    )
    .unwrap();

    record(&mut out, "CASE A at the reference viewport", &mut run, REFERENCE);
    record(
        &mut out,
        "CASE A at the smallest viewport presenting the inspector",
        &mut run,
        SMALLEST_WITH_INSPECTOR,
    );

    // Case B: a death for a subject no `survival_changed` record was ever seen for. No run reaches
    // this state, because the engine reports survival before it applies a death, so it is ingested
    // directly. `M99` is not a member of any run's population.
    let mut constructed = observer(&["--start-paused"]);
    constructed.ingest(vec![Event {
        tick: 7,
        subject: "M99".to_string(),
        detail: EventDetail::AgentDied { health: 0 },
    }]);
    let absent = constructed
        .death_of("M99")
        .expect("the ingested death is retained")
        .clone();
    constructed.select_for_test("M99");

    writeln!(
        out,
        "=== CASE B — the engine never reported this subject's survival\n\n    \
         subject {}   died on tick {}   health {}   satiety {:?}   energy {:?}   fear {:?}\n",
        absent.id, absent.tick, absent.health, absent.satiety, absent.energy, absent.fear
    )
    .unwrap();

    record(
        &mut out,
        "CASE B at the reference viewport",
        &mut constructed,
        REFERENCE,
    );
    record(
        &mut out,
        "CASE B at the smallest viewport presenting the inspector",
        &mut constructed,
        SMALLEST_WITH_INSPECTOR,
    );

    let path = write("inspector.txt", &out);
    println!("wrote {}", path.display());
}

#[test]
fn capture_the_filter_vocabulary() {
    // `SPEC-MOK-001`'s *Data and interface contracts* names fourteen stable core types and one
    // optional type. The partition here reads that document's own division — the optional type is
    // the per-action trace — and nothing about the observer's filter.
    let mut out = String::new();
    out.push_str(
        "WO-MOK-018 — the event-type vocabulary\n\
         \n\
         `EventType::ALL` enumerated in its declared order, with the `event=` string each renders\n\
         to. `SPEC-MOK-003` rule 9 item 2's figure counts the stable core types and excludes the\n\
         optional per-action trace, which `SPEC-MOK-001` lists separately.\n\n",
    );
    writeln!(out, "EventType::ALL.len() = {}\n", EventType::ALL.len()).unwrap();
    let mut core = 0;
    let mut optional = 0;
    for (index, kind) in EventType::ALL.iter().enumerate() {
        let name = kind.as_str();
        let class = if name == "action_trace" {
            optional += 1;
            "optional"
        } else {
            core += 1;
            "core"
        };
        writeln!(out, "  {:>2}  {:<28} {}", index + 1, name, class).unwrap();
    }
    writeln!(out, "\ncore = {core}   optional = {optional}   total = {}", core + optional).unwrap();
    let path = write("filter-vocabulary.txt", &out);
    println!("wrote {}", path.display());
}
