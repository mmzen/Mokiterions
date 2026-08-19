//! Public tier: render.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Each was in `src/render.rs`'s
//! `#[cfg(test)] mod tests` block and reaches the code through items that were already public, so
//! the move changes the path and nothing else: the assertions are verbatim and no item was widened
//! to bring them out. The tests rule 10 keeps inline are the ones that name a private item of the
//! module or one of the `#[cfg(test)]` hooks, and they stay in `src/render.rs`.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::{Buffer, Cell};
use ratatui::style::{Color, Modifier};

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
        (160, 40, "region"),
        (140, 44, "region"),
        (140, 43, "region"),
        (120, 48, "whole world"),
        (120, 30, "region"),
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
///
/// A region can be short in either axis or in both, so one case of each is asserted: `140 x 44`
/// addresses every world row and not every column, `120 x 30` every column and not every row, and
/// `140 x 43` neither.
#[test]
fn a_region_states_the_world_range_it_presents() {
    let mut observer = start(&[]);
    let text = text_of(&frame_of(&mut observer, 140, 44));
    assert!(text.contains("x0-93 y0-127"), "{text}");

    let text = text_of(&frame_of(&mut observer, 120, 30));
    assert!(text.contains("x0-127 y0-95"), "{text}");

    let text = text_of(&frame_of(&mut observer, 140, 43));
    assert!(text.contains("x0-93 y0-123"), "{text}");

    let text = text_of(&frame_of(&mut observer, 100, 30));
    assert!(text.contains("x0-101 y0-95"), "{text}");
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

    // 120 columns is above the roster's threshold and below the inspector's, so exactly one pane
    // is announced.
    let no_inspector = rows(&frame_of(&mut observer, 120, 48))[0].clone();
    assert!(no_inspector.contains("inspector i"), "{no_inspector}");
    assert!(!no_inspector.contains("roster r"), "{no_inspector}");

    // The reference viewport excludes nothing, so it announces nothing.
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

/// Rule 4 clause 7's three bands, stated here rather than imported.
///
/// `VER-MOK-007` requires it: a case that read the implementation's constants would pass whatever
/// those constants said. The cases below name a band by its index in this table and never name a
/// colour, because the palette is the implementation's to choose. What they assert is that two
/// gauges agree in colour exactly when they agree in band.
fn specified_band(value: u8) -> usize {
    match value {
        80..=100 => 0,
        40..=79 => 1,
        0..=39 => 2,
        outside => panic!("{outside} is outside the attribute domain rule 4 presents"),
    }
}

/// One roster entry's bar row, split into the pieces clause 7 bands and the pieces it leaves alone.
struct BarRow {
    /// The row's text inside the pane's borders.
    text: String,
    /// Each gauge's foreground, in health, satiety, energy order.
    gauges: Vec<Color>,
    /// Each gauge's numeric value as rendered, in the same order.
    values: Vec<String>,
    /// The foreground of every cell clause 7 leaves unstyled: the indent and the two separators.
    unstyled: Vec<Color>,
    /// Whether every cell inside the borders carries reversed video.
    reversed: bool,
}

/// The roster pane's cells on row `y`, or `None` where the row is outside the pane's body.
///
/// The roster is the leftmost pane, so its two borders are the first two vertical rules on the row.
/// Reading between them keeps the view pane's glyphs out of the bar-width count below.
fn roster_cells(buffer: &Buffer, y: u16) -> Option<Vec<&Cell>> {
    let row: Vec<&Cell> = (buffer.area.left()..buffer.area.right())
        .map(|x| buffer.cell((x, y)).unwrap())
        .collect();
    let borders: Vec<usize> = row
        .iter()
        .enumerate()
        .filter(|(_, cell)| cell.symbol() == "│")
        .map(|(x, _)| x)
        .collect();
    match borders.as_slice() {
        [left, right, ..] => Some(row[left + 1..*right].to_vec()),
        _ => None,
    }
}

/// The row on which `id`'s entry opens, found by the identity line rule 4 puts first.
fn entry_row(buffer: &Buffer, id: &str) -> u16 {
    (buffer.area.top()..buffer.area.bottom())
        .find(|&y| {
            roster_cells(buffer, y).is_some_and(|cells| {
                cells
                    .iter()
                    .map(|cell| cell.symbol())
                    .collect::<String>()
                    .starts_with(id)
            })
        })
        .unwrap_or_else(|| panic!("{id} has no roster entry"))
}

/// Reads the bar row at `y` out of the roster pane.
///
/// Rule 4 fixes the form — a five-column indent, then three gauges of a label character, a space,
/// the bar, a space and a three-column value, with two spaces between gauges — so the pieces are
/// located from that form. The bar width is whatever the layout produced at this viewport, counted
/// off the row rather than assumed. A gauge whose cells do not share one foreground fails here
/// rather than downstream, since a gauge reading as two states at once is what clause 7 forbids
/// when it applies the band to the gauge as a whole.
fn bar_row(buffer: &Buffer, y: u16) -> BarRow {
    let cells = roster_cells(buffer, y).expect("the bar row lies inside the roster pane");
    let indent = cells
        .iter()
        .position(|cell| cell.symbol() == "h")
        .expect("rule 4's bar row opens with the health gauge");
    assert_eq!(indent, 5, "rule 4 fixes a five-column indent");
    let bar = cells
        .iter()
        .filter(|cell| matches!(cell.symbol(), "█" | "░"))
        .count()
        / 3;
    let width = "h ".len() + bar + " ".len() + "100".len();

    let mut gauges = Vec::new();
    let mut values = Vec::new();
    let mut banded = vec![false; cells.len()];
    for (index, label) in ['h', 's', 'e'].into_iter().enumerate() {
        let start = indent + index * (width + 2);
        let gauge = &cells[start..start + width];
        assert_eq!(
            gauge[0].symbol(),
            label.to_string(),
            "rule 4 fixes the gauge order as health, satiety, energy"
        );
        for (offset, cell) in gauge.iter().enumerate() {
            assert_eq!(
                cell.fg,
                gauge[0].fg,
                "the {label} gauge reads as more than one state at column {}",
                start + offset
            );
            banded[start + offset] = true;
        }
        gauges.push(gauge[0].fg);
        values.push(
            gauge[width - 3..]
                .iter()
                .map(|cell| cell.symbol())
                .collect::<String>()
                .trim()
                .to_string(),
        );
    }

    BarRow {
        text: cells.iter().map(|cell| cell.symbol()).collect(),
        gauges,
        values,
        unstyled: cells
            .iter()
            .zip(&banded)
            .filter(|(_, banded)| !**banded)
            .map(|(cell, _)| cell.fg)
            .collect(),
        reversed: cells
            .iter()
            .all(|cell| cell.modifier.contains(Modifier::REVERSED)),
    }
}

/// A run advanced until one Mokiterion's three attributes fall in three different bands.
///
/// Every Mokiterion starts at full satiety and energy, so a spread takes ticks to appear. The run
/// is seeded, so where this search stops is fixed; it is searched for rather than written down so
/// that the case survives a change in how fast the world moves.
fn a_roster_spanning_three_bands() -> (Observer, String) {
    let mut observer = start(&[]);
    loop {
        let spread = observer
            .snapshot()
            .agents
            .iter()
            .find(|agent| {
                let bands = [
                    specified_band(agent.health),
                    specified_band(agent.satiety),
                    specified_band(agent.energy),
                ];
                bands[0] != bands[1] && bands[1] != bands[2] && bands[0] != bands[2]
            })
            .map(|agent| agent.id.clone());
        if let Some(id) = spread {
            return (observer, id);
        }
        assert!(
            observer.snapshot().tick < 500,
            "no entry spanned three bands within 500 ticks"
        );
        observer.advance().unwrap();
    }
}

/// `SPEC-MOK-003` rule 4 clause 7, read out of a drawn frame rather than out of a line.
///
/// The internal tier asserts the band function and the spans it builds. This tier asserts the one
/// thing that tier cannot: that a band survives the draw into the terminal's own cells, on a roster
/// produced by a run rather than by a fixture. One style shared by all three gauges is the obvious
/// defect and would pass any single-gauge case, so the entry looked for is one whose three
/// attributes fall in three different bands, and every gauge in the pane is then checked for
/// agreement between colour and band.
#[test]
fn the_survival_bands_reach_the_frame_and_three_differ_in_one_entry() {
    let (mut observer, spread) = a_roster_spanning_three_bands();
    let buffer = frame_of(&mut observer, 160, 48);
    let snapshot = observer.snapshot();

    // Colour agrees with band across the whole pane, and disagrees wherever the band does.
    let mut seen: Vec<(usize, Color)> = Vec::new();
    for agent in &snapshot.agents {
        let row = bar_row(&buffer, entry_row(&buffer, &agent.id) + 1);
        assert_eq!(
            row.values,
            [agent.health, agent.satiety, agent.energy].map(|value| value.to_string()),
            "{} presents the snapshot's own values: {}",
            agent.id,
            row.text
        );
        for (value, fg) in [agent.health, agent.satiety, agent.energy]
            .into_iter()
            .zip(&row.gauges)
        {
            let band = specified_band(value);
            for (other_band, other_fg) in &seen {
                if *other_band == band {
                    assert_eq!(fg, other_fg, "two gauges at band {band} differ in colour");
                } else {
                    assert_ne!(
                        fg, other_fg,
                        "bands {band} and {other_band} share one colour"
                    );
                }
            }
            seen.push((band, *fg));
        }
        for fg in row.unstyled {
            assert_eq!(
                fg,
                Color::Reset,
                "clause 7 leaves the indent and the separators unstyled: {}",
                row.text
            );
        }
    }

    let agent = snapshot
        .agents
        .iter()
        .find(|agent| agent.id == spread)
        .unwrap();
    let row = bar_row(&buffer, entry_row(&buffer, &spread) + 1);
    println!(
        "three bands in one entry at tick {}: {} h{} s{} e{} -> {:?}",
        snapshot.tick, agent.id, agent.health, agent.satiety, agent.energy, row.gauges
    );
    assert_ne!(row.gauges[0], row.gauges[1], "{}", row.text);
    assert_ne!(row.gauges[1], row.gauges[2], "{}", row.text);
    assert_ne!(row.gauges[0], row.gauges[2], "{}", row.text);
}

/// Rule 4 clause 6's reversed video and clause 7's band on one entry, neither replacing the other.
///
/// The bands are captured before the entry is selected and compared after, so what is asserted is
/// not that some colour is present but that selection changed no band at all. Selection is reached
/// through the bound key rather than through a hook, which is also how an operator reaches it.
#[test]
fn a_selected_entry_keeps_its_bands_under_reversed_video() {
    let (mut observer, spread) = a_roster_spanning_three_bands();
    let buffer = frame_of(&mut observer, 160, 48);
    let before = bar_row(&buffer, entry_row(&buffer, &spread) + 1);
    assert!(!before.reversed, "an unselected entry is not reversed");

    // Tab walks the roster in order, from no selection through to this entry.
    let living = observer.snapshot().agents.len();
    for _ in 0..=living {
        if observer.selection() == Some(spread.as_str()) {
            break;
        }
        observer
            .handle_key(press(ratatui::crossterm::event::KeyCode::Tab))
            .unwrap();
    }
    assert_eq!(observer.selection(), Some(spread.as_str()));

    let buffer = frame_of(&mut observer, 160, 48);
    let after = bar_row(&buffer, entry_row(&buffer, &spread) + 1);
    assert!(
        after.reversed,
        "clause 6 marks the selected entry by reversing it: {}",
        after.text
    );
    assert_eq!(
        after.gauges, before.gauges,
        "clause 7's bands survive clause 6's reversal: {}",
        after.text
    );
    assert_eq!(
        after.unstyled, before.unstyled,
        "selection colours nothing clause 7 left unstyled"
    );
    assert_eq!(after.text, before.text, "selection moves no character");
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
