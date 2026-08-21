//! TEMPORARY ORACLE for the `WO-MOK-005` remediation assessments. Not part of the product.
//!
//! Placed here, run once, removed, retained as evidence. It asserts nothing. It reaches only
//! public items, so it links the library target from outside the crate and no item was widened
//! for it.
//!
//! It writes down what `SPEC-MOK-003` rule 5's **Announcement** currently renders at viewports
//! where panes are excluded, which is what the repository owner's third assessment finding is
//! about: rule 5 obliges the observer to state that a pane is excluded, and `render.rs:187`
//! implements it, so the question is whether what it states is what an operator needs.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use mokiterions_tui::state::Observer;
use mokiterions_tui::{layout, options, render};

/// Viewports either side of each pane threshold: roster `W >= 100`, inspector `W >= 140`,
/// log `H >= 38` at six rows and ten when `W >= 140` and `H >= 48`. The floor is 34 x 22.
const VIEWPORTS: [(u16, u16, &str); 9] = [
    (160, 48, "every pane, ten-row log"),
    (160, 44, "every pane, six-row log"),
    (139, 48, "one column below the inspector threshold"),
    (120, 48, "inspector excluded"),
    (120, 37, "inspector and log excluded"),
    (100, 30, "roster present, inspector and log excluded"),
    (99, 30, "one column below the roster threshold"),
    (34, 22, "the floor"),
    (33, 21, "one below the floor: nothing is presented"),
];

fn observer_at_tick(tick: u64) -> Observer {
    let args = vec![
        "--seed", "42", "--policy", "reference", "--ticks", "300", "--start-paused",
    ];
    let mut observer = match options::parse(args) {
        Ok(options::Startup::Run(options)) => {
            Observer::new(options).expect("the configuration is valid")
        }
        other => panic!("not a runnable configuration: {other:?}"),
    };
    for _ in 0..tick {
        observer.advance().expect("the engine advances");
    }
    observer
}

fn region(buffer: &Buffer, area: Rect) -> String {
    (area.y..area.y + area.height)
        .map(|y| {
            (area.x..area.x + area.width)
                .map(|x| buffer.cell((x, y)).expect("inside the area").symbol())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[test]
fn what_the_announcement_says_when_panes_are_excluded() {
    let mut out = String::from(
        "SPEC-MOK-003 rule 5, Announcement (line 405):\n  \
         \"Whenever any pane is excluded, any roster entry is not visible, or the view presents\n  \
         a region, the observer states it: the header lists the panes currently available only as\n  \
         overlays, the roster title states how many entries are hidden, and the view title states\n  \
         the visible world range.\"\n\n\
         render.rs:187 announcement_text implements it and is evaluated on every frame, so the\n\
         statement is permanent wherever it appears at all. What follows is the header row and the\n\
         pane titles as actually drawn.\n\n\
         Note what the announcement does and does not say: it names the excluded panes and the key\n\
         that opens each as an overlay. It does not say the viewport is too small, and it does not\n\
         say that enlarging the terminal would restore the pane.\n\n",
    );

    for (width, height, note) in VIEWPORTS {
        let mut observer = observer_at_tick(200);
        let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test backend");
        terminal
            .draw(|target| render::draw(target, &mut observer))
            .expect("drawing into a buffer");
        let buffer = terminal.backend().buffer().clone();

        let _ = writeln!(
            out,
            "================ {width} x {height} — {note} ================"
        );
        if layout::below_floor(width, height) {
            let drawn = region(&buffer, *buffer.area())
                .chars()
                .filter(|glyph| !glyph.is_whitespace())
                .count();
            let _ = writeln!(
                out,
                "below the floor: {drawn} non-blank cells drawn (rule 5 presents nothing)\n"
            );
            continue;
        }

        let panes = layout::resolve(*buffer.area());
        let _ = writeln!(
            out,
            "panes present:  roster {}  inspector {}  log {}",
            panes.roster.is_some(),
            panes.inspector.is_some(),
            panes.log.is_some()
        );
        // The header is the first three rows: run state plus announcement, then two territories.
        let header = Rect {
            x: 0,
            y: 0,
            width,
            height: 1,
        };
        let _ = writeln!(out, "header row:\n[{}]", region(&buffer, header));
        for (label, pane) in [
            ("roster title", panes.roster),
            ("view title", Some(panes.view)),
            ("log title", panes.log),
        ] {
            if let Some(area) = pane {
                let title = Rect { height: 1, ..area };
                let _ = writeln!(out, "{label}:\n[{}]", region(&buffer, title));
            }
        }
        out.push('\n');
    }
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the workspace root")
        .join("assessment-material");
    fs::create_dir_all(&root).expect("the output directory");
    fs::write(root.join("announcement-at-reduced-viewports.txt"), out).expect("writing the capture");
}
