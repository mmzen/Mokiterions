//! Baseline capture for `WO-MOK-006`. This file is an oracle, not part of the product: it is
//! written into a scratch worktree at the predecessor commit and into the candidate tree, run
//! once on each side, and deleted. Its two copies differ in exactly one thing — the path the
//! observer's modules are reached by — which is the change being verified.

use std::fs;
use std::path::PathBuf;

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use PREFIX::options::{self, Startup};
use PREFIX::state::Observer;
use PREFIX::{export, render};

/// `VER-MOK-005`'s declared seed set.
const SEEDS: [u64; 5] = [0, 1, 42, 123, 777];

/// `VER-MOK-005`'s declared viewport set, including the one below the floor.
const VIEWPORTS: [(u16, u16); 7] = [
    (160, 48),
    (160, 44),
    (140, 44),
    (120, 48),
    (100, 30),
    (34, 22),
    (33, 21),
];

fn out() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("capture-out");
    fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

fn observer_for(args: &[&str]) -> Observer {
    match options::parse(args.to_vec()) {
        Ok(Startup::Run(options)) => Observer::new(options).expect("the configuration is valid"),
        other => panic!("{args:?}: {other:?}"),
    }
}

/// One frame as text: the cells a terminal would have received, styling omitted.
fn dump(observer: &mut Observer, width: u16, height: u16) -> String {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test backend");
    terminal
        .draw(|frame| render::draw(frame, observer))
        .expect("drawing into a buffer");
    let buffer = terminal.backend().buffer().clone();
    let area = *buffer.area();
    (area.y..area.y + area.height)
        .map(|y| {
            (area.x..area.x + area.width)
                .map(|x| buffer.cell((x, y)).expect("inside the area").symbol())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// One key press, through the public dispatch. `WO-MOK-005`'s retained dumps carry `sel M01`, so
/// the harness selects the same subject the way an operator would rather than through a hook — the
/// hooks are `#[cfg(test)]` and by design unreachable from outside the crate.
fn tap(observer: &mut Observer, code: KeyCode) {
    observer
        .handle_key(KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        })
        .expect("no binding fails");
}

#[test]
fn capture_the_declared_matrix() {
    let dir = out();

    // Frames: every declared seed at every declared viewport, drawn in the declared order from
    // one observer advanced to tick 200 with no interaction — `VER-MOK-005`'s own conditions.
    for seed in SEEDS {
        let seed_text = seed.to_string();
        let mut observer = observer_for(&["--seed", &seed_text, "--ticks", "400"]);
        for _ in 0..200 {
            observer.advance().expect("the engine advances");
        }
        tap(&mut observer, KeyCode::Tab);
        for (width, height) in VIEWPORTS {
            let text = dump(&mut observer, width, height);
            let name = format!("frame-seed{seed}-{width}x{height}.txt");
            fs::write(dir.join(name), text).expect("writing a frame");
        }
    }

    // Exports: one per declared seed at the tick limit `WO-MOK-005` retained, so the bytes are
    // comparable against evidence that is already verified.
    for seed in SEEDS {
        let seed_text = seed.to_string();
        let mut observer = observer_for(&["--seed", &seed_text, "--ticks", "20"]);
        while !observer.is_finished() {
            observer.advance().expect("the engine advances");
        }
        let mut bytes = Vec::new();
        export::write_records(&mut bytes, observer.events()).expect("writing to a vector");
        let name = format!("events-seed{seed}-ticks20.log");
        fs::write(dir.join(name), bytes).expect("writing an export");
    }
}
