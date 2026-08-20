//! TEMPORARY ORACLE for `WO-MOK-013`. Not part of the product.
//!
//! Placed in the tree, run once, removed, and retained as evidence, on the precedent of
//! `WO-MOK-012`'s `assessment-oracle.rs`, `WO-MOK-010`'s `observer/frame-probe.rs` and
//! `WO-MOK-006`'s `frame-and-export-oracle.rs`. **It asserts nothing.** It renders into an
//! in-memory backend and writes down what the buffer holds, so that judging what the instrument
//! shows is a separate act from measuring it. `VER-MOK-013`'s *Evidence retention* states that a
//! capture is re-run rather than corrected.
//!
//! It produces two of that section's items:
//!
//!   * `gauge-resolution.txt` — for every gauge width the implementation can produce, the filled
//!     cell count at every value in `0..=100`. The before form of exactly this table is
//!     `evidence/WO-MOK-012/assessment-material/bar-quantization.txt`, which recorded a two-cell
//!     bar with three renderable states for 101 values.
//!   * `frames.txt` — a frame at every viewport of `SPEC-MOK-003` rule 5's derived table plus the
//!     floor, each showing the header line in full and every roster entry it draws.
//!
//! It lives inside the crate rather than beside it because the value table reaches
//! `Observer::replace_snapshot_for_test`, which is `#[cfg(test)]` and which no target outside the
//! crate can link. **No item was widened for it**, so `SPEC-MOK-004` rule 6's interface is
//! untouched by its presence, and rule 11's test totals were measured on the tree without it.
//!
//! Unlike the `WO-MOK-012` oracle, which recomputed the fill arithmetic in its own code, every
//! figure here is read out of the rendered buffer: the bar widths, the filled counts and the
//! gauge positions are counted from cells. Nothing below knows `bar_width` or `BAR_ROW_OVERHEAD`,
//! which is the same independence `VER-MOK-013` requires of its cases.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use mokiterions::simulation::Simulation;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::layout::Rect;

use crate::state::Observer;
use crate::{export, layout, options, render};

/// The filled and empty cell glyphs of a gauge, read as data rather than named as constants.
const FILLED: char = '\u{2588}';
const EMPTY: char = '\u{2591}';

/// The nine declared viewports of `VER-MOK-005` that render, which are the viewports of rule 5's
/// derived table. `34 x 22` is the floor and is included; `33 x 21` is refused and draws nothing,
/// so it has no frame to capture.
const VIEWPORTS: [(u16, u16); 9] = [
    (160, 48),
    (160, 44),
    (160, 40),
    (140, 44),
    (140, 43),
    (120, 48),
    (120, 30),
    (100, 30),
    (34, 22),
];

/// The horizon `VER-MOK-013`'s reference assessment names. The engine default is 100 ticks, so
/// `--ticks` has to be supplied for a 200-tick capture to reach tick 200 at all.
const HORIZON: u64 = 200;

fn observer_for(args: &[&str]) -> Observer {
    match options::parse(args.to_vec()) {
        Ok(options::Startup::Run(options)) => {
            Observer::new(options).expect("the configuration is valid")
        }
        other => panic!("{args:?} did not yield a runnable configuration: {other:?}"),
    }
}

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn frame(observer: &mut Observer, width: u16, height: u16) -> Buffer {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test backend");
    terminal
        .draw(|target| render::draw(target, observer))
        .expect("drawing into a buffer");
    terminal.backend().buffer().clone()
}

/// One region's rows as symbols, joined.
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

fn symbols(buffer: &Buffer) -> String {
    region(buffer, *buffer.area())
}

/// Every attribute of every Mokiterion held at one value, through the hook that exists so a test
/// can reach a value a run does not produce. This is the same route
/// `render::tests::hold_every_attribute_at` takes.
fn hold_every_attribute_at(observer: &mut Observer, value: u8) {
    let mut snapshot = observer.snapshot().clone();
    for agent in &mut snapshot.agents {
        agent.health = value;
        agent.satiety = value;
        agent.energy = value;
        agent.fear = value;
    }
    observer.replace_snapshot_for_test(snapshot);
}

/// Each maximal run of gauge cells in a line, as `(filled, total)`. A gauge is found by its cells
/// and not by a column figure, so a change to the label or the numeric width does not silently
/// move what is measured.
fn gauges(line: &str) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    let mut filled = 0usize;
    let mut total = 0usize;
    for character in line.chars() {
        if character == FILLED || character == EMPTY {
            total += 1;
            if character == FILLED {
                filled += 1;
            }
        } else if total > 0 {
            found.push((filled, total));
            filled = 0;
            total = 0;
        }
    }
    if total > 0 {
        found.push((filled, total));
    }
    found
}

/// The gauges of the first roster entry, in drawing order, at this viewport.
fn first_entry_gauges(buffer: &Buffer, roster: Rect) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    for line in region(buffer, roster).lines() {
        let row = gauges(line);
        if row.is_empty() {
            if found.is_empty() {
                continue;
            }
            break;
        }
        found.extend(row);
        if found.len() >= 4 {
            break;
        }
    }
    found
}

fn write(name: &str, body: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the workspace root")
        .join("docs")
        .join("engineering")
        .join("simulation")
        .join("evidence")
        .join("WO-MOK-013");
    fs::create_dir_all(&root).expect("the output directory");
    let path = root.join(name);
    fs::write(&path, body).expect("writing the capture");
    path
}

#[test]
fn capture_the_gauge_resolution() {
    // Every distinct roster pane width the layout can produce over the plane the monotonicity
    // obligation sweeps. The layout is resolved rather than rendered here, so the sweep is cheap;
    // the widths it finds are then rendered.
    let mut widths: Vec<u16> = Vec::new();
    let mut example: Vec<(u16, u16, u16)> = Vec::new();
    for width in layout::MIN_WIDTH..=200 {
        for height in layout::MIN_HEIGHT..=60 {
            if layout::below_floor(width, height) {
                continue;
            }
            let panes = layout::resolve(Rect::new(0, 0, width, height));
            if let Some(roster) = panes.roster
                && !widths.contains(&roster.width)
            {
                widths.push(roster.width);
                example.push((roster.width, width, height));
            }
        }
    }

    let mut out = String::new();
    out.push_str(
        "WO-MOK-013 gauge resolution — the after form of\n\
         evidence/WO-MOK-012/assessment-material/bar-quantization.txt\n\n\
         Every figure below is counted out of a rendered buffer. Nothing here knows the layout's\n\
         constants or the fill arithmetic: a gauge is located by its cells and its filled count is\n\
         the number of full blocks drawn in it.\n\n",
    );
    let _ = writeln!(
        out,
        "Distinct roster pane widths over 34..=200 x 22..=60: {}\n\
         (SPEC-MOK-003 rule 5 fixes the roster at 47 columns wherever it is present, so one width\n\
         is the whole set. The sweep is what establishes that rather than the rule being taken on\n\
         trust — a second width would appear here as a second table below.)\n",
        widths
            .iter()
            .map(|width| width.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    for (pane_width, at_width, at_height) in example {
        let mut observer = observer_for(&["--seed", "42", "--ticks", "300", "--start-paused"]);
        observer.advance().expect("the engine advances");

        let probe = frame(&mut observer, at_width, at_height);
        let roster = layout::resolve(*probe.area())
            .roster
            .expect("this viewport was selected because it presents the roster");
        let shape = first_entry_gauges(&probe, roster);
        let _ = writeln!(
            out,
            "=== roster pane {pane_width} columns, first produced at {at_width} x {at_height} ===\n\
             pane interior {} columns; {} gauges on the first entry, each {} cells wide\n",
            roster.width - 2,
            shape.len(),
            shape
                .iter()
                .map(|(_, total)| total.to_string())
                .collect::<Vec<_>>()
                .join(" / ")
        );

        // The filled count of each gauge at every value in 0..=100, held through the hook.
        let mut counts: Vec<Vec<usize>> = Vec::new();
        for value in 0..=100u8 {
            hold_every_attribute_at(&mut observer, value);
            let buffer = frame(&mut observer, at_width, at_height);
            let roster = layout::resolve(*buffer.area()).roster.expect("as above");
            counts.push(
                first_entry_gauges(&buffer, roster)
                    .into_iter()
                    .map(|(filled, _)| filled)
                    .collect(),
            );
        }

        let width = shape.first().map(|(_, total)| *total).unwrap_or(0);
        out.push_str("bar                 values          span\n");
        let mut runs: Vec<(usize, u8, u8)> = Vec::new();
        for (value, row) in counts.iter().enumerate() {
            let filled = row.first().copied().unwrap_or(0);
            let value = u8::try_from(value).expect("0..=100");
            match runs.last_mut() {
                Some(last) if last.0 == filled => last.2 = value,
                _ => runs.push((filled, value, value)),
            }
        }
        for (filled, first, last) in &runs {
            let _ = writeln!(
                out,
                "{}{}   {first:>3}..={last:<3}      {} values",
                FILLED.to_string().repeat(*filled),
                EMPTY.to_string().repeat(width - *filled),
                usize::from(last - first) + 1
            );
        }
        let _ = writeln!(
            out,
            "\n{} distinct renderable states for 101 values, against the 3 of the two-cell bar.",
            runs.len()
        );

        // REQ-MOK-047 as VER-MOK-013 states it: a ten-point step moves the fill, at every value.
        let mut failures = Vec::new();
        for value in 0..=90usize {
            let before = counts[value].first().copied().unwrap_or(0);
            let after = counts[value + 10].first().copied().unwrap_or(0);
            if after <= before {
                failures.push((value, before, after));
            }
        }
        let _ = writeln!(
            out,
            "Ten-point step over 0..=90: {}",
            if failures.is_empty() {
                "every step increases the filled count".to_string()
            } else {
                format!("{failures:?}")
            }
        );

        // The four gauges of one entry, so that the fourth is inspectable beside the other three.
        out.push_str(
            "\nAll four gauges of the first entry, filled cells at ten-point values:\n\
             value    h    s    e    f\n",
        );
        for value in (0..=100usize).step_by(10) {
            let row = &counts[value];
            let _ = writeln!(
                out,
                "{value:>5} {:>4} {:>4} {:>4} {:>4}",
                row.first().copied().unwrap_or(0),
                row.get(1).copied().unwrap_or(0),
                row.get(2).copied().unwrap_or(0),
                row.get(3).copied().unwrap_or(0)
            );
        }
        out.push('\n');
    }

    write("gauge-resolution.txt", &out);
}

#[test]
fn capture_the_frames() {
    let mut out = String::new();
    out.push_str(
        "WO-MOK-013 frame captures — every viewport of SPEC-MOK-003 rule 5's derived table plus\n\
         the floor, at seed 42, policy reference, --ticks 300, at tick 200, with the first living\n\
         Mokiterion selected through Tab rather than through a hook.\n\n\
         An oracle asserts nothing. What each frame shows is judged elsewhere: the automated cases\n\
         are in tests/render.rs and tests/layout.rs, and the two manual assessments are\n\
         VER-MOK-013's.\n\n",
    );

    for (width, height) in VIEWPORTS {
        let mut observer = observer_for(&[
            "--seed",
            "42",
            "--policy",
            "reference",
            "--ticks",
            "300",
            "--start-paused",
        ]);
        for _ in 0..HORIZON {
            observer.advance().expect("the engine advances");
        }
        observer
            .handle_key(press(KeyCode::Tab))
            .expect("no binding fails");

        let buffer = frame(&mut observer, width, height);
        let panes = layout::resolve(*buffer.area());
        let (canvas_columns, canvas_rows) = layout::canvas_cells(panes.view);
        let _ = writeln!(
            out,
            "================================================================================\n\
             === {width} x {height}\n\
             === roster {}   inspector {}   log {}\n\
             === canvas interior {} x {} cells; view pane {} x {} at {},{}\n\
             ================================================================================",
            panes
                .roster
                .map(|pane| format!("{} x {}", pane.width, pane.height))
                .unwrap_or_else(|| "absent".to_string()),
            panes
                .inspector
                .map(|pane| format!("{} x {}", pane.width, pane.height))
                .unwrap_or_else(|| "absent".to_string()),
            panes
                .log
                .map(|pane| format!("{} x {} rows", pane.width, pane.height))
                .unwrap_or_else(|| "absent".to_string()),
            canvas_columns,
            canvas_rows,
            panes.view.width,
            panes.view.height,
            panes.view.x,
            panes.view.y,
        );
        let _ = writeln!(out, "{}\n", symbols(&buffer));
    }

    write("frames.txt", &out);
}

/// The interaction script `tests/verification.rs` drives its observed runs with, verbatim, so that
/// the artifact retained here is comparable with the case that asserts the same property.
fn interact(observer: &mut Observer, round: u64) {
    const KEYS: [KeyCode; 12] = [
        KeyCode::Tab,
        KeyCode::Char('z'),
        KeyCode::Char('l'),
        KeyCode::Char('j'),
        KeyCode::Char('e'),
        KeyCode::Char('f'),
        KeyCode::BackTab,
        KeyCode::Char('L'),
        KeyCode::PageUp,
        KeyCode::Char('c'),
        KeyCode::Esc,
        KeyCode::Char('W'),
    ];
    let step = usize::try_from(round % 12).expect("a small remainder");
    observer
        .handle_key(press(KEYS[step]))
        .expect("no binding fails");
    if step % 3 == 0 {
        observer
            .handle_key(press(KeyCode::Char('+')))
            .expect("no binding fails");
    }
    if step % 5 == 0 {
        observer
            .handle_key(press(KeyCode::Char('-')))
            .expect("no binding fails");
    }
}

#[test]
fn capture_the_non_perturbation_comparison() {
    const ARGS: [&str; 8] = [
        "--seed",
        "42",
        "--ticks",
        "300",
        "--policy",
        "reference",
        "--density",
        "0.75",
    ];

    // The unobserved run: the engine's own `Simulation::run`, one function away from the engine
    // binary's `main`, writing the authoritative text stream into memory.
    let config = match options::parse(ARGS.to_vec()) {
        Ok(options::Startup::Run(options)) => options.config,
        other => panic!("{ARGS:?} did not yield a runnable configuration: {other:?}"),
    };
    let mut simulation = Simulation::new(config).expect("the configuration is valid");
    let mut unobserved = Vec::new();
    simulation
        .run(&mut unobserved)
        .expect("writing to a vector never fails");
    let unobserved = String::from_utf8(unobserved).expect("the stream is text");

    // The observed run: the same configuration, drawn at a rotating viewport and interacted with
    // on every round, driven to the engine's own end.
    let mut observer = observer_for(&ARGS);
    let mut round = 0u64;
    while !observer.is_finished() {
        let (width, height) = VIEWPORTS[usize::try_from(round).expect("fits") % VIEWPORTS.len()];
        frame(&mut observer, width, height);
        interact(&mut observer, round);
        observer.advance().expect("the engine advances");
        round += 1;
    }
    frame(&mut observer, 160, 48);
    interact(&mut observer, round);

    let mut exported = Vec::new();
    export::write_records(&mut exported, observer.events()).expect("writing to a vector");
    let exported = String::from_utf8(exported).expect("the export is text");

    // The comparison. The export carries a trailer that the engine stream does not, and the engine
    // stream ends with its summary line, so the record bodies are compared line for line and the
    // two tails are shown rather than diffed against each other.
    let engine_lines: Vec<&str> = unobserved.lines().collect();
    let (engine_records, engine_summary) = engine_lines.split_at(engine_lines.len() - 1);
    let export_lines: Vec<&str> = exported.lines().collect();
    let export_records: Vec<&str> = export_lines
        .iter()
        .copied()
        .filter(|line| line.starts_with("tick="))
        .collect();

    let identical = engine_records == export_records.as_slice();
    let first_difference = engine_records
        .iter()
        .zip(export_records.iter())
        .enumerate()
        .find(|(_, (left, right))| left != right)
        .map(|(index, (left, right))| format!("record {index}:\n  unobserved {left}\n  observed   {right}"));

    let mut out = String::new();
    let _ = writeln!(
        out,
        "WO-MOK-013 non-perturbation comparison\n\n\
         Configuration: {}\n\n\
         The observed run was drawn at a rotating viewport over the nine renderable declared\n\
         viewports and interacted with on every round, using the twelve-key script\n\
         tests/verification.rs drives its own observed runs with, plus the speed controls. The\n\
         unobserved run is the engine's `Simulation::run`, which is the engine binary's whole\n\
         behaviour, writing into memory.\n\n\
         unobserved records          {}\n\
         observed retained records   {}\n\
         records identical, in order {}\n\
         first difference            {}\n\n\
         engine summary line (unobserved):\n  {}\n\n\
         export trailer (observed):\n{}\n\n\
         The two streams are retained beside this file as `non-perturbation-unobserved.txt` and\n\
         `non-perturbation-observed-export.txt`, so the comparison is reproducible with `diff`\n\
         rather than only stated here.\n",
        ARGS.join(" "),
        engine_records.len(),
        export_records.len(),
        if identical { "yes" } else { "NO" },
        first_difference.unwrap_or_else(|| "none".to_string()),
        engine_summary.first().copied().unwrap_or(""),
        export_lines
            .iter()
            .filter(|line| !line.starts_with("tick="))
            .map(|line| format!("  {line}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    write("non-perturbation.txt", &out);
    write("non-perturbation-unobserved.txt", &unobserved);
    write("non-perturbation-observed-export.txt", &exported);
}
