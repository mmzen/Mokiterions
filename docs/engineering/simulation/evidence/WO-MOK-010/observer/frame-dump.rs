//! TEMPORARY evidence harness for `WO-MOK-010`, oracle 4. Not part of the suite.
//!
//! Prints the roster, map and inspector buffers as text at each declared viewport, with the cell
//! positions of the name, the identifier and the glyph, and the inspector for a dead selection.
//! Written to compile against the pre-change revision as well, so that one harness produces both
//! sides of the bar-row comparison: nothing here assumes a record carries a name.
//!
//! Run with `cargo test --test zz_frame_dump -- --nocapture`. Deleted after the capture.

use std::collections::BTreeMap;

use mokiterions::simulation::EventType;
use mokiterions_tui::state::Observer;
use mokiterions_tui::{layout, options, render, spatial};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::layout::Rect;

const RENDERABLE: [(u16, u16); 9] = [
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

fn observer_for(args: &[&str]) -> Observer {
    match options::parse(args.to_vec()) {
        Ok(options::Startup::Run(options)) => {
            Observer::new(options).expect("the configuration is valid")
        }
        other => panic!("{args:?} did not yield a runnable configuration: {other:?}"),
    }
}

/// The name the engine reported per identifier, or an empty map on a revision that reports none.
fn reported_names(observer: &Observer) -> BTreeMap<String, String> {
    observer
        .events()
        .iter()
        .filter(|event| event.event_type() == EventType::AgentInitialized)
        .filter_map(|event| {
            let details = event.detail.to_string();
            let name = details.strip_prefix("name:")?.split_once(',')?.0.to_string();
            Some((event.subject.clone(), name))
        })
        .collect()
}

fn frame(observer: &mut Observer, width: u16, height: u16) -> Buffer {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test backend");
    terminal
        .draw(|target| render::draw(target, observer))
        .expect("drawing into a buffer");
    terminal.backend().buffer().clone()
}

fn rows(buffer: &Buffer, area: Rect) -> Vec<String> {
    (area.y..area.y + area.height)
        .map(|y| {
            (area.x..area.x + area.width)
                .map(|x| buffer.cell((x, y)).expect("inside the area").symbol())
                .collect::<String>()
        })
        .collect()
}

fn print_region(label: &str, buffer: &Buffer, area: Rect) {
    println!(
        "  {label}: x={} y={} w={} h={}",
        area.x, area.y, area.width, area.height
    );
    for (index, row) in rows(buffer, area).into_iter().enumerate() {
        println!("    y={:<3} |{row}|", area.y + index as u16);
    }
}

fn canvas_of(width: u16, height: u16) -> Rect {
    let view = layout::resolve(Rect::new(0, 0, width, height)).view;
    Rect::new(view.x + 1, view.y + 1, view.width - 2, view.height - 2)
}

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

/// Where an identifier sits on a roster row, and what the six columns before it hold.
fn entry_positions(buffer: &Buffer, area: Rect, names: &BTreeMap<String, String>) {
    for (index, row) in rows(buffer, area).into_iter().enumerate() {
        let y = area.y + index as u16;
        let characters: Vec<char> = row.chars().collect();
        for start in 0..characters.len().saturating_sub(2) {
            if characters[start] != 'M'
                || !characters[start + 1].is_ascii_digit()
                || !characters[start + 2].is_ascii_digit()
            {
                continue;
            }
            let id: String = characters[start..start + 3].iter().collect();
            if !names.is_empty() && !names.contains_key(&id) {
                continue;
            }
            let id_x = area.x + start as u16;
            let reported = names
                .get(&id)
                .map(String::as_str)
                .unwrap_or("<no name on this revision>");
            if start >= 6 {
                let field: String = characters[start - 6..start].iter().collect();
                println!(
                    "    {id}: identifier at (x={id_x}, y={y}); the six columns before it, x={}..{}, hold |{field}|; the engine reported {reported:?}",
                    id_x - 6,
                    id_x - 1
                );
            } else {
                println!(
                    "    {id}: identifier at (x={id_x}, y={y}); no six columns precede it on this revision; the engine reported {reported:?}"
                );
            }
            break;
        }
    }
}

#[test]
fn dump() {
    println!("### seed 42, --start-paused, one advance, reference policy, density 0.75");
    let mut observer = observer_for(&["--seed", "42", "--start-paused"]);
    observer.advance().expect("the engine advances");
    let names = reported_names(&observer);
    println!("### reported names: {names:?}");

    for (width, height) in RENDERABLE {
        println!("\n=== VIEWPORT {width}x{height} ===");
        let buffer = frame(&mut observer, width, height);
        let panes = layout::resolve(*buffer.area());
        println!(
            "  panes: header {:?} view {:?} roster {:?} inspector {:?} log {:?}",
            panes.header, panes.view, panes.roster, panes.inspector, panes.log
        );

        match panes.roster {
            Some(roster) => {
                print_region("roster pane (border included)", &buffer, roster);
                println!("  name and identifier cell positions:");
                entry_positions(&buffer, roster, &names);
            }
            None => println!("  roster pane: excluded at this viewport, reachable as an overlay"),
        }

        let canvas = canvas_of(width, height);
        println!("  map canvas glyph cells (letters and digits only; food is ○◎● and the boundary ─):");
        for (index, row) in rows(&buffer, canvas).into_iter().enumerate() {
            for (column, symbol) in row.chars().enumerate() {
                if symbol.is_ascii_alphanumeric() {
                    println!(
                        "    (x={}, y={}) glyph {symbol:?}",
                        canvas.x + column as u16,
                        canvas.y + index as u16
                    );
                }
            }
        }
        let viewport = spatial::Viewport::resolve(
            observer.zoom(),
            (canvas.width, canvas.height),
            observer.camera(),
        );
        println!("  where each living subject maps, by the observer's own mapping:");
        for agent in &observer.snapshot().agents {
            if let Some(cell) = viewport.cell_of(
                observer.zoom(),
                agent.position.x.into(),
                agent.position.y.into(),
            ) {
                if cell.0 < canvas.width && cell.1 < canvas.height {
                    println!(
                        "    {} at world {}:{} -> canvas cell (x={}, y={}), absolute (x={}, y={})",
                        agent.id,
                        agent.position.x,
                        agent.position.y,
                        cell.0,
                        cell.1,
                        canvas.x + cell.0,
                        canvas.y + cell.1
                    );
                }
            }
        }

        match panes.inspector {
            Some(inspector) => print_region("inspector pane", &buffer, inspector),
            None => println!("  inspector pane: excluded at this viewport"),
        }
    }

    println!("\n=== DETAIL ZOOM, 160x48, first living subject selected ===");
    tap(&mut observer, KeyCode::Tab);
    tap(&mut observer, KeyCode::Char('z'));
    let buffer = frame(&mut observer, 160, 48);
    let canvas = canvas_of(160, 48);
    println!("  zoom is now {:?}", observer.zoom());
    print_region("map canvas", &buffer, canvas);
    let viewport = spatial::Viewport::resolve(
        observer.zoom(),
        (canvas.width, canvas.height),
        observer.camera(),
    );
    for agent in &observer.snapshot().agents {
        if let Some(cell) = viewport.cell_of(
            observer.zoom(),
            agent.position.x.into(),
            agent.position.y.into(),
        ) {
            if cell.0 < canvas.width && cell.1 < canvas.height {
                println!(
                    "    {} at world {}:{} -> absolute (x={}, y={})",
                    agent.id,
                    agent.position.x,
                    agent.position.y,
                    canvas.x + cell.0,
                    canvas.y + cell.1
                );
            }
        }
    }
    if let Some(inspector) = layout::resolve(*buffer.area()).inspector {
        print_region("inspector pane, living selection", &buffer, inspector);
    }

    println!("\n=== A DEAD SELECTION, 160x48 ===");
    let arguments = ["--policy", "baseline", "--ticks", "400", "--start-paused"];
    let mut scout = observer_for(&arguments);
    let (victim, tick) = loop {
        assert!(!scout.is_finished(), "the run ended before any death");
        let living: Vec<String> = scout
            .snapshot()
            .agents
            .iter()
            .map(|agent| agent.id.clone())
            .collect();
        scout.advance().expect("the engine advances");
        if let Some(gone) = living
            .into_iter()
            .find(|was| !scout.snapshot().agents.iter().any(|is| is.id == *was))
        {
            break (gone, scout.snapshot().tick);
        }
    };
    let mut observer = observer_for(&arguments);
    let dead_names = reported_names(&observer);
    println!(
        "  the first death under the baseline policy is {victim} at tick {tick}; the engine reported its name as {:?}",
        dead_names
            .get(&victim)
            .map(String::as_str)
            .unwrap_or("<no name on this revision>")
    );
    for _ in 0..12 {
        if observer.selection() == Some(victim.as_str()) {
            break;
        }
        tap(&mut observer, KeyCode::Tab);
    }
    while observer.snapshot().tick < tick {
        observer.advance().expect("the engine advances");
    }
    println!("  selection after the death: {:?}", observer.selection());
    let buffer = frame(&mut observer, 160, 48);
    if let Some(inspector) = layout::resolve(*buffer.area()).inspector {
        print_region("inspector pane, dead selection", &buffer, inspector);
    }
    if let Some(roster) = layout::resolve(*buffer.area()).roster {
        print_region("roster pane, after the death", &buffer, roster);
    }
}
