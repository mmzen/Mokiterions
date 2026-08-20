//! TEMPORARY ORACLE for the `WO-MOK-005` remediation assessments. Not part of the product.
//!
//! Placed here, run once, removed, and retained as evidence, on the precedent of `WO-MOK-010`'s
//! `observer/frame-probe.rs` and `WO-MOK-006`'s `frame-and-export-oracle.rs`.
//!
//! It asserts nothing. It renders into an in-memory backend at the reference viewport
//! `VER-MOK-005` names and writes down what the buffer holds, so that a person assessing the
//! live terminal has the current screen to compare against. The material `VER-MOK-005` points
//! at — `evidence/WO-MOK-005/frames.txt` — was captured at `f361370` and predates the colour
//! bands of `WO-MOK-007`, the fourth gauge of `WO-MOK-010` and the names of `WO-MOK-011`.
//!
//! It lives inside the crate rather than beside it because two of the four captures reach
//! `Observer::replace_decisions_for_test` and `Observer::select_for_test`, both `#[cfg(test)]`,
//! which no target outside the crate can link. No item was widened for it.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use mokiterions::simulation::{Action, DecisionOutcome, DecisionSnapshot, Direction};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::style::Modifier;

use crate::state::Observer;
use crate::{layout, options, render};

/// The reference viewport `VER-MOK-005` names for the legibility assessments.
const WIDTH: u16 = 160;
const HEIGHT: u16 = 48;

/// The horizon assessment 1 requires. The recorded procedure omitted `--ticks`, so it inherited
/// the engine default of 100 and could not reach this tick at all.
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

/// The whole screen as symbols.
fn symbols(buffer: &Buffer) -> String {
    region(buffer, *buffer.area())
}

/// The same screen with colour discarded, one glyph per cell for whatever modifier the cell
/// carries. This is the projection `verification::every_distinction_survives_the_loss_of_colour`
/// reads: `(symbol, modifier)` and nothing else. A person assessing colour independence needs to
/// know *where* to look on the live terminal, which is what this locates.
fn modifiers(buffer: &Buffer) -> String {
    let area = *buffer.area();
    (area.y..area.y + area.height)
        .map(|y| {
            (area.x..area.x + area.width)
                .map(|x| {
                    let cell = buffer.cell((x, y)).expect("inside the area");
                    let modifier = cell.style().add_modifier;
                    let blank = cell.symbol().trim().is_empty();
                    match (
                        modifier.contains(Modifier::REVERSED),
                        modifier.contains(Modifier::UNDERLINED),
                        modifier.contains(Modifier::BOLD),
                        blank,
                    ) {
                        (true, true, _, _) => '#',
                        (true, false, _, _) => 'R',
                        (false, true, _, _) => 'U',
                        (false, false, true, _) => 'B',
                        (false, false, false, true) => ' ',
                        (false, false, false, false) => '.',
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Every distinct foreground colour on the screen, with a count and one example cell, so the
/// bands of `SPEC-MOK-003` rule 4 clause 7 can be checked against what the live terminal draws.
fn palette(buffer: &Buffer) -> String {
    let area = *buffer.area();
    let mut seen: Vec<(String, usize, (u16, u16), String)> = Vec::new();
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            let cell = buffer.cell((x, y)).expect("inside the area");
            if cell.symbol().trim().is_empty() {
                continue;
            }
            let key = format!("{:?}", cell.style().fg);
            match seen.iter_mut().find(|entry| entry.0 == key) {
                Some(entry) => entry.1 += 1,
                None => seen.push((key, 1, (x, y), cell.symbol().to_string())),
            }
        }
    }
    seen.sort_by(|left, right| right.1.cmp(&left.1));
    let mut out = String::from("foreground colour        cells   first at   glyph\n");
    for (colour, count, (x, y), glyph) in seen {
        let _ = writeln!(out, "{colour:<24} {count:>5}   {x:>3},{y:<3}   {glyph}");
    }
    out
}

fn write(name: &str, body: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the workspace root")
        .join("assessment-material");
    fs::create_dir_all(&root).expect("the output directory");
    let path = root.join(name);
    fs::write(&path, body).expect("writing the capture");
    path
}

#[test]
fn capture_the_current_instrument() {
    // Assessment 1's corrected configuration: the declared seed and policy, with the tick limit
    // the horizon actually needs. `--start-paused` so the capture is taken at a known tick.
    let mut observer = observer_for(&[
        "--seed", "42", "--policy", "reference", "--ticks", "300", "--start-paused",
    ]);
    for _ in 0..HORIZON {
        observer.advance().expect("the engine advances");
    }
    // Select through the real binding rather than the hook, so the frame is one an operator can
    // reach: `Tab` selects the next living Mokiterion in roster order.
    observer
        .handle_key(press(KeyCode::Tab))
        .expect("no binding fails");

    let buffer = frame(&mut observer, WIDTH, HEIGHT);
    let panes = layout::resolve(*buffer.area());

    let mut header = String::new();
    let _ = writeln!(
        header,
        "Captured at 160 x 48, seed 42, policy reference, --ticks 300, at tick {HORIZON}.\n\
         Selection made with Tab. Below the floor is never reached here: 160 x 48 is above it.\n"
    );

    write(
        "frame-160x48-tick200.txt",
        &format!("{header}{}", symbols(&buffer)),
    );
    write(
        "modifiers-160x48-tick200.txt",
        &format!(
            "{header}Colour discarded. One glyph per cell:\n  \
             R = REVERSED   U = UNDERLINED   # = both   B = BOLD   . = styled by colour only   \
             (space) = blank cell\n\n{}",
            modifiers(&buffer)
        ),
    );
    write(
        "palette-160x48-tick200.txt",
        &format!("{header}{}", palette(&buffer)),
    );

    let mut panes_out = String::from(header.clone());
    if let Some(roster) = panes.roster {
        let _ = writeln!(
            panes_out,
            "=== roster pane, {} x {} at {},{} ===\n{}\n",
            roster.width,
            roster.height,
            roster.x,
            roster.y,
            region(&buffer, roster)
        );
    } else {
        panes_out.push_str("=== roster pane: ABSENT at this viewport ===\n\n");
    }
    if let Some(inspector) = panes.inspector {
        let _ = writeln!(
            panes_out,
            "=== inspector pane, {} x {} at {},{} ===\n{}\n",
            inspector.width,
            inspector.height,
            inspector.x,
            inspector.y,
            region(&buffer, inspector)
        );
    } else {
        panes_out.push_str("=== inspector pane: ABSENT at this viewport ===\n\n");
    }
    write("panes-160x48-tick200.txt", &panes_out);
}

#[test]
fn capture_a_rejection() {
    // Restated assessment 4. No shipped decision source can have a proposal rejected — asserted
    // over 400 ticks of both policies — so this state is unreachable by running the observer and
    // is reached through the hook that exists for exactly that reason. The injected decision is
    // self-contradictory on purpose: a westward move from an interior cell, which any validation
    // rule read alone would accept, carrying the outcome `rejected`.
    let mut observer = observer_for(&["--seed", "42", "--policy", "reference", "--start-paused"]);
    observer.advance().expect("the engine advances");
    let subject = observer.snapshot().agents[0].id.clone();
    let ground = "the target cell lies outside the world".to_string();

    observer.replace_decisions_for_test(vec![DecisionSnapshot {
        agent_id: subject.clone(),
        proposed: Action::Move {
            direction: Direction::West,
        },
        outcome: DecisionOutcome::Rejected { ground },
        applied: None,
    }]);
    observer.select_for_test(&subject);

    let buffer = frame(&mut observer, WIDTH, HEIGHT);
    let panes = layout::resolve(*buffer.area());
    let mut out = format!(
        "Captured at 160 x 48, seed 42, policy reference, at tick 1, subject {subject}.\n\
         The rejection is injected through Observer::replace_decisions_for_test, which is\n\
         #[cfg(test)] and compiled out of the shipped binary. It cannot be reached by running\n\
         the observer.\n\n"
    );
    if let Some(inspector) = panes.inspector {
        let _ = writeln!(
            out,
            "=== inspector pane, where the outcome is presented ===\n{}\n",
            region(&buffer, inspector)
        );
    }
    let _ = writeln!(out, "=== whole screen ===\n{}", symbols(&buffer));
    let _ = writeln!(out, "\n=== colour on this frame ===\n{}", palette(&buffer));
    write("rejection-160x48.txt", &out);
}

/// The overlays and the second zoom, which assessment 1's three questions need and the recorded
/// procedure's material does not contain.
#[test]
fn capture_the_overlays_and_the_detail_zoom() {
    let mut out = String::new();
    for (label, keys) in [
        ("authority overlay (t)", vec![KeyCode::Char('t')]),
        ("key-binding overlay (?)", vec![KeyCode::Char('?')]),
        ("detail zoom (z)", vec![KeyCode::Char('z')]),
        (
            "detail zoom, following the selection (z then f)",
            vec![KeyCode::Char('z'), KeyCode::Char('f')],
        ),
    ] {
        let mut observer = observer_for(&[
            "--seed", "42", "--policy", "reference", "--ticks", "300", "--start-paused",
        ]);
        for _ in 0..HORIZON {
            observer.advance().expect("the engine advances");
        }
        observer
            .handle_key(press(KeyCode::Tab))
            .expect("no binding fails");
        for key in keys {
            observer.handle_key(press(key)).expect("no binding fails");
        }
        let buffer = frame(&mut observer, WIDTH, HEIGHT);
        let _ = writeln!(
            out,
            "================ {label} ================\n{}\n",
            symbols(&buffer)
        );
    }
    write("overlays-and-zoom-160x48.txt", &out);
}

/// Restated assessment 3 needs a frame that actually carries `UNDERLINED`, and the recommended run
/// does not. `render.rs:410` uses it for exactly one thing: a rendered cell holding more than one
/// Mokiterion. At overview zoom one character cell covers eight world cells, so co-occupancy is
/// reachable, but it is seed- and tick-dependent — and at tick 200 on seed 42 there is none.
///
/// This searches for the first frame that draws an underlined cell and writes it down, so the
/// assessor has a run in which the modifier is on screen to be judged.
#[test]
fn find_a_frame_that_underlines() {
    let mut report = String::from(
        "UNDERLINED is drawn by render.rs:410 on a rendered cell holding more than one\n\
         Mokiterion, and by nothing else. It is the only marker of co-occupancy for a\n\
         Mokiterion that is not the selected one: the inspector's \"sharing N in this rendered\n\
         cell\" line reports it textually for the selection alone.\n\n\
         Searched seeds 0..=40, ticks 1..=200, at 160 x 48 in overview zoom.\n\n",
    );
    let mut found: Option<(u64, u64, Buffer)> = None;
    'search: for seed in 0..=40u64 {
        let seed = seed.to_string();
        let mut observer = observer_for(&[
            "--seed",
            &seed,
            "--policy",
            "reference",
            "--ticks",
            "300",
            "--start-paused",
        ]);
        for tick in 1..=200u64 {
            if observer.advance().is_err() || observer.is_finished() {
                break;
            }
            let buffer = frame(&mut observer, WIDTH, HEIGHT);
            let underlined = buffer
                .content()
                .iter()
                .filter(|cell| cell.style().add_modifier.contains(Modifier::UNDERLINED))
                .count();
            if underlined > 0 {
                found = Some((seed.parse().expect("a seed"), tick, buffer));
                break 'search;
            }
        }
    }

    match found {
        Some((seed, tick, buffer)) => {
            let _ = writeln!(
                report,
                "FOUND: seed {seed}, tick {tick}.\n\n\
                 Reproduce with:\n  \
                 mokiterions-tui --seed {seed} --policy reference --ticks 300 --start-paused\n  \
                 then press . exactly {tick} times (or release with Space and hold at tick {tick}).\n\n\
                 === whole screen ===\n{}\n\n=== colour discarded ===\n{}",
                symbols(&buffer),
                modifiers(&buffer)
            );
        }
        None => report.push_str(
            "NOT FOUND. No frame in the searched range draws an underlined cell, so the\n\
             co-occupancy modifier cannot be assessed on a live terminal within it. Restated\n\
             assessment 3 would need either a wider search, a constructed snapshot, or the\n\
             finding that the modifier is unreachable in practice.\n",
        ),
    }
    write("underline-search.txt", &report);
}

/// The sharpest form of restated assessment 3. `find_a_frame_that_underlines` locates a frame in
/// which co-occupancy is drawn; this one selects a Mokiterion *in* the co-occupied cell, so the
/// same character carries `REVERSED` and `UNDERLINED` together. The question a person has to answer
/// is whether that cell is distinguishable from a cell carrying reversal alone — which is what
/// decides whether co-occupancy survives on a terminal that renders underline weakly.
#[test]
fn capture_reversed_and_underlined_on_one_cell() {
    const SEED: &str = "0";
    const TICK: u64 = 18;

    let mut report = String::new();
    let mut hit = None;
    let probe = observer_for(&[
        "--seed", SEED, "--policy", "reference", "--ticks", "300", "--start-paused",
    ]);
    let mut probe = probe;
    for _ in 0..TICK {
        probe.advance().expect("the engine advances");
    }
    let ids: Vec<String> = probe
        .snapshot()
        .agents
        .iter()
        .map(|agent| agent.id.clone())
        .collect();

    for id in &ids {
        let mut observer = observer_for(&[
            "--seed", SEED, "--policy", "reference", "--ticks", "300", "--start-paused",
        ]);
        for _ in 0..TICK {
            observer.advance().expect("the engine advances");
        }
        observer.select_for_test(id);
        let buffer = frame(&mut observer, WIDTH, HEIGHT);
        let both = buffer
            .content()
            .iter()
            .filter(|cell| {
                let modifier = cell.style().add_modifier;
                modifier.contains(Modifier::UNDERLINED) && modifier.contains(Modifier::REVERSED)
            })
            .count();
        if both > 0 {
            hit = Some((id.clone(), buffer));
            break;
        }
    }

    match hit {
        Some((id, buffer)) => {
            let _ = writeln!(
                report,
                "Seed {SEED}, tick {TICK}, selection {id}.\n\n\
                 This frame draws one character carrying REVERSED and UNDERLINED at once: the\n\
                 selected Mokiterion shares its rendered cell with at least one other. Compare it\n\
                 against the other Mokiterion glyphs, which carry BOLD and a territory colour only.\n\n\
                 Reproduce with:\n  \
                 mokiterions-tui --seed {SEED} --policy reference --ticks 300 --start-paused\n  \
                 then press . exactly {TICK} times, then Tab until the inspector names {id}.\n  \
                 The inspector's \"sharing\" line states how many share the cell.\n\n\
                 === whole screen ===\n{}\n\n=== colour discarded ===\n{}\n\n=== colour ===\n{}",
                symbols(&buffer),
                modifiers(&buffer),
                palette(&buffer)
            );
        }
        None => report.push_str(
            "No selection at this frame puts REVERSED and UNDERLINED on one cell: the underlined\n\
             cell's occupants are not reachable as a selection here. The two modifiers must then be\n\
             assessed separately.\n",
        ),
    }
    write("reversed-and-underlined-160x48.txt", &report);
}

/// Assessment 2 is about legibility, and the bar is the part of the roster whose legibility is
/// arithmetic rather than opinion. `gauge_text` fills `value * width / 100` cells, so the number of
/// distinct bars a gauge can draw is `width + 1`, whatever the attribute's range.
#[test]
fn tabulate_the_bar_quantization() {
    let mut out = String::from(
        "render.rs:572  filled = value * width / 100, integer division.\n\
         layout ROSTER_WIDTH is 47, so the pane interior is 45 and\n\
         render.rs:546  bar_width(45) = min(20, (45 - 35) / 4) = 2.\n\n\
         A two-cell bar draws three distinct states for 101 attribute values:\n\n\
         bar    values          span\n",
    );
    let width = 2usize;
    let mut runs: Vec<(usize, u8, u8)> = Vec::new();
    for value in 0..=100u8 {
        let filled = (usize::from(value) * width / 100).min(width);
        match runs.last_mut() {
            Some(last) if last.0 == filled => last.2 = value,
            _ => runs.push((filled, value, value)),
        }
    }
    for (filled, first, last) in runs {
        let _ = writeln!(
            out,
            "{}{}   {first:>3}..={last:<3}      {} values",
            "\u{2588}".repeat(filled),
            "\u{2591}".repeat(width - filled),
            usize::from(last - first) + 1
        );
    }
    out.push_str(
        "\nSo at the reference roster the proportional fill distinguishes 0..=49 from 50..=99\n\
         from 100, and nothing finer. Two attributes 49 apart draw the same bar. The level is\n\
         carried by the three-digit numeric value, which is why SPEC-MOK-003 rule 2.5 still\n\
         holds without colour — but the fill is not doing the work the rule's wording implies\n\
         it shares.\n",
    );
    write("bar-quantization.txt", &out);
}
