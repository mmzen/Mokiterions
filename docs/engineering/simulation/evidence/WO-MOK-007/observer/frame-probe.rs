//! Oracle 4 for `WO-MOK-007`: the drawn roster, cell by cell.
//!
//! This file is an oracle, not part of the product. It is placed here, run once, removed, and
//! retained as evidence at
//! `docs/engineering/simulation/evidence/WO-MOK-007/observer/frame-probe.rs`, on the precedent of
//! `WO-MOK-006`'s `frame-and-export-oracle.rs`.
//!
//! It asserts nothing. It renders into an in-memory backend and writes down what the buffer holds,
//! together with the attribute values the same frame was drawn from, so that
//! `analysis/frames.py` can derive rule 4's expected geometry independently and compare. Keeping
//! the arithmetic out of the probe is the point: a probe that computed
//! `min(20, (interior - 35) / 4)` for itself would be checking the product against a copy of the
//! product.
//!
//! Everything it reaches is already public: `options::parse`, `Observer::new`, `Observer::advance`,
//! `Observer::snapshot`, `layout::resolve` and `render::draw`. No item was widened for it.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;

use mokiterions_tui::options::{self, Startup};
use mokiterions_tui::state::Observer;
use mokiterions_tui::{layout, render};

/// `VER-MOK-005`'s declared viewport set, in its declared order, including the three -- the last two
/// and 100x30, which reaches tier D on its height -- where no roster is drawn at all.
const VIEWPORTS: [(u16, u16); 7] = [
    (160, 48),
    (160, 44),
    (140, 44),
    (120, 48),
    (100, 30),
    (34, 22),
    (33, 21),
];

/// The ticks each capture is taken at. Tick 0 is the frame where every `fear` is still `0`, which is
/// rule 4.4's empty bar; the later two let the value vary.
const TICKS: [u64; 3] = [0, 200, 1000];

fn out() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("probe-out");
    fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

fn observer_for(args: &[&str]) -> Observer {
    match options::parse(args.to_vec()) {
        Ok(Startup::Run(options)) => Observer::new(options).expect("the configuration is valid"),
        other => panic!("{args:?}: {other:?}"),
    }
}

fn advanced(seed: u64, ticks: u64) -> Observer {
    let seed = seed.to_string();
    let mut observer =
        observer_for(&["--seed", &seed, "--ticks", "1000", "--policy", "individual"]);
    for _ in 0..ticks {
        observer.advance().expect("the engine advances");
    }
    observer
}

/// One frame's roster: the pane the layout puts it in, the cells inside that pane, and the
/// attribute values the frame was drawn from.
fn probe(observer: &mut Observer, width: u16, height: u16) -> String {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test backend");
    terminal
        .draw(|frame| render::draw(frame, observer))
        .expect("drawing into a buffer");
    let buffer = terminal.backend().buffer().clone();
    let panes = layout::resolve(Rect::new(0, 0, width, height));

    let mut text = String::new();
    writeln!(text, "-- viewport {width}x{height}").expect("a string");
    writeln!(text, "tier {}", panes.tier.label()).expect("a string");
    for agent in &observer.snapshot().agents {
        writeln!(
            text,
            "agent {} health={} satiety={} energy={} fear={}",
            agent.id, agent.health, agent.satiety, agent.energy, agent.fear
        )
        .expect("a string");
    }
    match panes.roster {
        None => writeln!(text, "roster none").expect("a string"),
        Some(rect) => {
            writeln!(
                text,
                "roster x={} y={} width={} height={}",
                rect.x, rect.y, rect.width, rect.height
            )
            .expect("a string");
            for y in rect.y..rect.y.saturating_add(rect.height) {
                let row: String = (rect.x..rect.x.saturating_add(rect.width))
                    .map(|x| buffer.cell((x, y)).expect("inside the area").symbol())
                    .collect();
                // The pipes fix the pane's first and last column, so a column index read out of
                // this row is the column the terminal would have received.
                writeln!(text, "row {y:>3} |{row}|").expect("a string");
            }
        }
    }
    text
}

#[test]
fn capture_the_roster_frames() {
    let dir = out();

    // The declared viewports, at three ticks, under the policy this work order adds.
    for ticks in TICKS {
        let mut observer = advanced(42, ticks);
        let mut text = format!("seed 42 policy individual tick {ticks}\n");
        for (width, height) in VIEWPORTS {
            text.push_str(&probe(&mut observer, width, height));
        }
        fs::write(dir.join(format!("declared-seed42-tick{ticks}.txt")), text)
            .expect("writing a capture");
    }

    // Every width from the floor to the widest declared viewport, at one height. The declared set
    // has no viewport whose roster pane falls below rule 4's 47-column threshold, so the collapsed
    // one-line entry form and the bar-width arithmetic below the cap are only reachable by sweeping.
    // The sweep is supplementary to the declared set, not a substitute for it.
    let mut observer = advanced(42, 200);
    let mut text = String::from("seed 42 policy individual tick 200\n");
    for width in layout::MIN_WIDTH..=160 {
        text.push_str(&probe(&mut observer, width, 48));
    }
    fs::write(dir.join("sweep-height48-seed42-tick200.txt"), text).expect("writing a capture");
}
