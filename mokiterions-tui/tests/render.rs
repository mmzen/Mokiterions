//! Public tier: render.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Each was in `src/render.rs`'s
//! `#[cfg(test)] mod tests` block and reaches the code through items that were already public, so
//! the move changes the path and nothing else: the assertions are verbatim and no item was widened
//! to bring them out. The tests rule 10 keeps inline are the ones that name a private item of the
//! module or one of the `#[cfg(test)]` hooks, and they stay in `src/render.rs`.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;

// `use super::*` used to supply these. A test tier outside the crate gets none of the module's
// private imports, so every name a moved test uses is named here through the public interface.
use mokiterions_tui::options::{self, Startup};
use mokiterions_tui::render::*;
use mokiterions_tui::spatial::{self, Zoom};
use mokiterions_tui::state::Observer;

fn start(args: &[&str]) -> Observer {
    match options::parse(args.to_vec()).unwrap() {
        Startup::Run(options) => Observer::new(options).unwrap(),
        Startup::Help => panic!("expected a run"),
    }
}

fn frame_of(observer: &mut Observer, width: u16, height: u16) -> Buffer {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal.draw(|frame| draw(frame, observer)).unwrap();
    terminal.backend().buffer().clone()
}

fn rows(buffer: &Buffer) -> Vec<String> {
    (buffer.area.top()..buffer.area.bottom())
        .map(|y| {
            (buffer.area.left()..buffer.area.right())
                .map(|x| buffer.cell((x, y)).unwrap().symbol().to_string())
                .collect()
        })
        .collect()
}

fn text_of(buffer: &Buffer) -> String {
    rows(buffer).join("\n")
}

/// Rule 5's derived table, checked through the rendered title rather than the arithmetic.
#[test]
fn every_declared_viewport_renders_and_annotates_what_it_presents() {
    let cases = [
        (160u16, 48u16, "whole world"),
        (160, 44, "whole world"),
        (140, 44, "region"),
        (120, 48, "whole world"),
        (100, 30, "region"),
        (34, 22, "region"),
    ];
    for (width, height, extent) in cases {
        let mut observer = start(&[]);
        let buffer = frame_of(&mut observer, width, height);
        let text = text_of(&buffer);
        assert!(text.contains(extent), "{width}x{height} lacks {extent}");
        assert!(
            text.contains("RUNNING"),
            "{width}x{height} lacks the run state"
        );
        assert!(
            rows(&buffer).last().unwrap().contains(&"tick".to_string())
                || rows(&buffer).last().unwrap().contains(&"@".to_string()),
            "{width}x{height} lacks the provenance footer"
        );
    }
}

/// The region annotation must state the range, since absence from the view is not death.
#[test]
fn a_region_states_the_world_range_it_presents() {
    let mut observer = start(&[]);
    let text = text_of(&frame_of(&mut observer, 140, 44));
    assert!(text.contains("x0-93 y0-127"), "{text}");

    let text = text_of(&frame_of(&mut observer, 100, 30));
    assert!(text.contains("x0-127 y0-95"), "{text}");
}

#[test]
fn below_the_floor_nothing_is_presented() {
    let mut observer = start(&[]);
    let buffer = frame_of(&mut observer, 33, 21);
    assert!(
        text_of(&buffer).trim().is_empty(),
        "a viewport below the floor must present nothing"
    );
    // The run is untouched: drawing is suspended, not terminated.
    assert_eq!(observer.snapshot().tick, 0);
    assert!(!observer.is_finished());
}

/// Rule 2.1 through the buffer: a world cell lands where the mapping says it does, and
/// smaller world `y` is a smaller screen row.
#[test]
fn detail_zoom_places_every_visible_entity_at_its_mapped_cell() {
    let mut observer = start(&[]);
    observer.record_geometry((32, 16), 8);
    observer
        .handle_key(press(ratatui::crossterm::event::KeyCode::Char('z')))
        .unwrap();
    assert_eq!(observer.zoom(), Zoom::Detail);

    let buffer = frame_of(&mut observer, 34, 22);
    let viewport = observer.viewport();
    let occupied: Vec<(u16, u16)> = observer
        .snapshot()
        .agents
        .iter()
        .filter_map(|agent| {
            viewport.cell_of(
                Zoom::Detail,
                agent.position.x.into(),
                agent.position.y.into(),
            )
        })
        .collect();

    let mut checked = 0;
    for resource in &observer.snapshot().resources {
        let Some((x, y)) = viewport.cell_of(
            Zoom::Detail,
            resource.position.x.into(),
            resource.position.y.into(),
        ) else {
            continue;
        };
        if occupied.contains(&(x, y)) {
            continue; // A Mokiterion glyph takes precedence over a resource glyph.
        }
        let symbol = buffer.cell((1 + x, 4 + y)).unwrap().symbol();
        assert_eq!(
            symbol,
            spatial::resource_glyph(resource.class).to_string(),
            "{} at {}",
            resource.id,
            resource.position
        );
        checked += 1;
    }
    assert!(checked > 0, "the region must contain a visible resource");
}

fn press(code: ratatui::crossterm::event::KeyCode) -> ratatui::crossterm::event::KeyEvent {
    ratatui::crossterm::event::KeyEvent::new(code, ratatui::crossterm::event::KeyModifiers::NONE)
}

/// Rule 5's announcement survives the narrowest viewport, where every pane is an overlay.
#[test]
fn the_header_names_the_panes_that_are_only_overlays() {
    let mut observer = start(&[]);
    let narrow = rows(&frame_of(&mut observer, 34, 22))[0].clone();
    assert!(narrow.contains("ovl"), "{narrow}");
    assert!(narrow.contains('r') && narrow.contains('L') && narrow.contains('i'));

    let tier_c = rows(&frame_of(&mut observer, 120, 48))[0].clone();
    assert!(tier_c.contains("inspector i"), "{tier_c}");
    assert!(!tier_c.contains("roster r"), "{tier_c}");

    // Tier A excludes nothing, so it announces nothing.
    let full = rows(&frame_of(&mut observer, 160, 48))[0].clone();
    assert!(!full.contains("overlays"), "{full}");
}

/// Rule 8, read off the footer row itself.
#[test]
fn the_footer_carries_the_provenance_and_nothing_environment_specific() {
    let mut observer = start(&["--seed", "42", "--ticks", "500", "--policy", "baseline"]);
    let buffer = frame_of(&mut observer, 160, 48);
    let footer = rows(&buffer)[47].clone();

    assert!(footer.contains("seed 42"), "{footer}");
    assert!(footer.contains("ticks 500"), "{footer}");
    assert!(footer.contains("density 0.75%"), "{footer}");
    assert!(footer.contains("source baseline"), "{footer}");
    assert!(footer.contains("tick 0"), "{footer}");
    assert!(footer.contains("events "), "{footer}");
    assert!(!footer.contains("truncated"), "{footer}");

    for forbidden in ["C:\\", "/home/", "AppData", "PATH", "token"] {
        assert!(!footer.contains(forbidden), "{footer}");
    }
    // A defaulted density presents exactly as an explicit one (rule 8.1).
    let mut explicit = start(&["--seed", "42", "--ticks", "500", "--density", "0.75"]);
    let explicit = rows(&frame_of(&mut explicit, 160, 48))[47].clone();
    assert!(explicit.contains("density 0.75%"), "{explicit}");
}

/// A failed export is reported in the header, and the run keeps running (rule 9.6).
#[test]
fn a_reported_failure_reaches_the_header() {
    let mut observer = start(&[]);
    observer.set_notice("export failed: no such directory");
    let header = rows(&frame_of(&mut observer, 160, 48))[0].clone();
    assert!(header.contains("export failed"), "{header}");
    assert!(header.contains("RUNNING"), "{header}");
}

/// Rule 12.2, read at the frame boundary: drawing draws no entropy and changes no state.
#[test]
fn drawing_never_advances_the_simulation() {
    let mut observer = start(&["--start-paused"]);
    observer.advance().unwrap();
    let events: Vec<String> = observer.events().iter().map(ToString::to_string).collect();

    for _ in 0..20 {
        let _ = frame_of(&mut observer, 160, 48);
    }

    assert_eq!(observer.snapshot().tick, 1);
    let after: Vec<String> = observer.events().iter().map(ToString::to_string).collect();
    assert_eq!(after, events);
}
