//! Public tier: render.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Each was in `src/render.rs`'s
//! `#[cfg(test)] mod tests` block and reaches the code through items that were already public, so
//! the move changes the path and nothing else: the assertions are verbatim and no item was widened
//! to bring them out. The tests rule 10 keeps inline are the ones that name a private item of the
//! module or one of the `#[cfg(test)]` hooks, and they stay in `src/render.rs`.

use std::collections::BTreeMap;

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier};

// `use super::*` used to supply these. A test tier outside the crate gets none of the module's
// private imports, so every name a moved test uses is named here through the public interface.
use mokiterions_tui::layout::{self, Pane};
use mokiterions_tui::options::{self, Startup};
use mokiterions_tui::render::*;
use mokiterions_tui::spatial::{self, Zoom};
use mokiterions_tui::state::{Observer, Overlay};

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
///
/// The `ovl` abbreviation this asserted is not a form the announcement takes after rule 5's
/// 2026-08-20 amendment: the shortened rungs drop the word `overlays` and the pane names before they
/// drop anything an operator acts on, so at the floor the announcement is the key, the axis and the
/// threshold and nothing else. The case is unchanged — what the header names at each of these three
/// viewports — and each viewport keeps an assertion at least as strong as the one it had.
#[test]
fn the_header_names_the_panes_that_are_only_overlays() {
    let mut observer = start(&[]);
    let narrow = rows(&frame_of(&mut observer, 34, 22))[0].clone();
    // All three panes at the floor, each with its key, its axis and the extent that returns it.
    assert!(narrow.contains("r W100"), "{narrow}");
    assert!(narrow.contains("L H38"), "{narrow}");
    assert!(narrow.contains("i W140"), "{narrow}");

    // 120 columns is above the roster's threshold and below the inspector's, so exactly one pane
    // is announced.
    let no_inspector = rows(&frame_of(&mut observer, 120, 48))[0].clone();
    assert!(no_inspector.contains("inspector i"), "{no_inspector}");
    assert!(!no_inspector.contains("roster r"), "{no_inspector}");
    assert!(!no_inspector.contains("log L"), "{no_inspector}");

    // The reference viewport excludes nothing, so it announces nothing.
    let full = rows(&frame_of(&mut observer, 160, 48))[0].clone();
    assert!(!full.contains("overlays"), "{full}");
    assert!(!full.contains("W140") && !full.contains("H38"), "{full}");
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

/// One roster entry's bar rows, split into the pieces clause 7 bands and the pieces it leaves alone.
///
/// Rule 4 as amended 2026-08-20 spreads the four gauges over two lines, two to a line, so an entry
/// has two bar rows and this reads both of them as one subject. What clause 7 governs is unchanged:
/// a band belongs to a gauge, and the indent, the separators and the fourth gauge carry none.
struct BarRow {
    /// Both rows' text inside the pane's borders, one line each.
    text: String,
    /// Each banded gauge's foreground, in health, satiety, energy order.
    ///
    /// Clause 5 as amended puts a fourth gauge after them and clause 7 as amended gives it no
    /// band, so it is not read as a band here; the four-gauge form is asserted by the `Gauge`
    /// tier below, and this row reads only what clause 7 governs.
    gauges: Vec<Color>,
    /// Each banded gauge's numeric value as rendered, in the same order.
    values: Vec<String>,
    /// The foreground of every cell clause 7 leaves unstyled: the indent, the separators, and
    /// the fourth gauge, which clause 7 as amended leaves unstyled in full.
    unstyled: Vec<Color>,
    /// Whether every cell the rows' characters occupy carries reversed video.
    ///
    /// Not necessarily every cell inside the borders: a cell no character occupies carries no
    /// reversal, which has always been true of the identity line above these rows. Two gauges to a
    /// line fill the reference roster's 45-column interior exactly — `5 + 2 * (13 + 6) + 2 = 45` —
    /// where four on one line left it two columns short, so the distinction does not arise at that
    /// viewport and still can at others.
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

/// Rule 4's name field: the entry's first six columns, the identifier beginning after them.
const NAME_COLUMNS: usize = 6;

/// The row on which `id`'s entry opens, found by the identity line rule 4 puts first.
///
/// Rule 4 as amended for `REQ-MOK-041` puts the name in that line's first six columns and the
/// identifier immediately after them, so the row is found by where the identifier sits rather than
/// by what the line opens with. A bar row cannot match: its columns six to eight are the health
/// gauge's label and the opening cells of its bar.
fn entry_row(buffer: &Buffer, id: &str) -> u16 {
    (buffer.area.top()..buffer.area.bottom())
        .find(|&y| {
            roster_cells(buffer, y).is_some_and(|cells| {
                cells
                    .iter()
                    .map(|cell| cell.symbol())
                    .collect::<String>()
                    .get(NAME_COLUMNS..NAME_COLUMNS + id.len())
                    == Some(id)
            })
        })
        .unwrap_or_else(|| panic!("{id} has no roster entry"))
}

/// Reads both bar rows of the entry whose identity line is at `y`, out of the roster pane.
///
/// Rule 4 as amended fixes the form — a five-column indent, then two gauges of a label character, a
/// space, the bar, a space and a three-column value, with two spaces between them, on each of two
/// lines — so the pieces are located from that form. The bar width is whatever the layout produced
/// at this viewport, counted off the row rather than assumed. A gauge whose cells do not share one
/// foreground fails here rather than downstream, since a gauge reading as two states at once is what
/// clause 7 forbids when it applies the band to the gauge as a whole.
///
/// The two rows are read as one subject because clause 7's obligations run across both: `health` and
/// `satiety` are on the first, `energy` and `fear` on the second, which is the order clause 5 as
/// amended fixes. Only the three banded gauges are returned as bands; `fear` is read as part of what
/// clause 7 leaves unstyled, exactly as it was when the four shared a line.
fn bar_rows(buffer: &Buffer, y: u16) -> BarRow {
    let mut text = Vec::new();
    let mut gauges = Vec::new();
    let mut values = Vec::new();
    let mut unstyled = Vec::new();
    let mut reversed = true;

    for (offset, labels) in [['h', 's'], ['e', 'f']].into_iter().enumerate() {
        let row = y + 1 + u16::try_from(offset).unwrap();
        let cells = roster_cells(buffer, row).expect("the bar row lies inside the roster pane");
        let indent = cells
            .iter()
            .position(|cell| cell.symbol() == labels[0].to_string())
            .unwrap_or_else(|| panic!("rule 4's bar row opens with the {} gauge", labels[0]));
        assert_eq!(indent, 5, "rule 4 fixes a five-column indent");
        // Two gauges to a line since clause 5 was amended, so two bars' worth of cells on the row.
        let bar = cells
            .iter()
            .filter(|cell| matches!(cell.symbol(), "█" | "░"))
            .count()
            / 2;
        let width = "h ".len() + bar + " ".len() + "100".len();
        // Where the row's characters end: two gauges and the separator between them. Cells past
        // this carry no character, and therefore no band and no reversal.
        let occupied = indent + 2 * width + 2;
        for (past, cell) in cells[occupied..].iter().enumerate() {
            assert_eq!(
                cell.symbol(),
                " ",
                "the row's two gauges end at column {}, but column {} carries a character",
                occupied,
                occupied + past
            );
        }

        let mut banded = vec![false; cells.len()];
        for (index, label) in labels.into_iter().enumerate() {
            let start = indent + index * (width + 2);
            let gauge = &cells[start..start + width];
            assert_eq!(
                gauge[0].symbol(),
                label.to_string(),
                "rule 4 fixes the gauge order as health and satiety, then energy and fear"
            );
            for (column, cell) in gauge.iter().enumerate() {
                assert_eq!(
                    cell.fg,
                    gauge[0].fg,
                    "the {label} gauge reads as more than one state at column {}",
                    start + column
                );
                // `fear` takes no band, so its cells are read as unstyled rather than as a gauge.
                banded[start + column] = label != 'f';
            }
            if label == 'f' {
                continue;
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

        text.push(cells.iter().map(|cell| cell.symbol()).collect::<String>());
        unstyled.extend(
            cells
                .iter()
                .zip(&banded)
                .filter(|(_, banded)| !**banded)
                .map(|(cell, _)| cell.fg),
        );
        reversed &= cells[..occupied]
            .iter()
            .all(|cell| cell.modifier.contains(Modifier::REVERSED));
    }

    BarRow {
        text: text.join("\n"),
        gauges,
        values,
        unstyled,
        reversed,
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
        let row = bar_rows(&buffer, entry_row(&buffer, &agent.id));
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
                "clause 7 leaves the indent, the separators and the fourth gauge unstyled: {}",
                row.text
            );
        }
    }

    let agent = snapshot
        .agents
        .iter()
        .find(|agent| agent.id == spread)
        .unwrap();
    let row = bar_rows(&buffer, entry_row(&buffer, &spread));
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
    let before = bar_rows(&buffer, entry_row(&buffer, &spread));
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
    let after = bar_rows(&buffer, entry_row(&buffer, &spread));
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

// ---- rule 4's four-gauge row -------------------------------------------------------------

/// One gauge parsed out of a rendered roster row: its label, the column the label occupies, the
/// filled and shaded halves of its bar, and the value it presents.
#[derive(Clone)]
struct Gauge {
    label: char,
    column: usize,
    filled: usize,
    shaded: usize,
    value: u8,
}

impl Gauge {
    fn bar(&self) -> usize {
        self.filled + self.shaded
    }
}

/// The gauge starting at `index`, and the column just past it, or `None` if none starts there.
///
/// A gauge is a label, one space, a run of filled cells, a run of shaded cells, one space, and a
/// three-column right-aligned value. Recognised positionally off the buffer's own cells so that a
/// slot rendered as a blank, as a dash, or with a bar of zero cells is not recognised at all —
/// which is what makes the absence visible as a gap in the parsed order rather than as a passing
/// substring match on the neighbouring gauges. Rule 4's one-line form, which presents the same
/// four values without bars, yields nothing here for the same reason.
fn gauge_at(cells: &[char], index: usize) -> Option<(Gauge, usize)> {
    const FILLED: char = '\u{2588}';
    const SHADED: char = '\u{2591}';

    let label = *cells.get(index)?;
    if !matches!(label, 'h' | 's' | 'e' | 'f') || *cells.get(index + 1)? != ' ' {
        return None;
    }
    let mut cursor = index + 2;
    let mut filled = 0;
    while cells.get(cursor) == Some(&FILLED) {
        filled += 1;
        cursor += 1;
    }
    let mut shaded = 0;
    while cells.get(cursor) == Some(&SHADED) {
        shaded += 1;
        cursor += 1;
    }
    if filled + shaded == 0 || *cells.get(cursor)? != ' ' {
        return None;
    }
    let value: u8 = cells
        .get(cursor + 1..cursor + 4)?
        .iter()
        .collect::<String>()
        .trim()
        .parse()
        .ok()?;
    Some((
        Gauge {
            label,
            column: index,
            filled,
            shaded,
            value,
        },
        cursor + 4,
    ))
}

/// Every gauge on one rendered row, in the order the row presents them.
fn gauges_in(row: &str) -> Vec<Gauge> {
    let cells: Vec<char> = row.chars().collect();
    let mut gauges = Vec::new();
    let mut index = 0;
    while index < cells.len() {
        match gauge_at(&cells, index) {
            Some((gauge, next)) => {
                gauges.push(gauge);
                index = next;
            }
            None => index += 1,
        }
    }
    gauges
}

/// The count of entries the roster title reports as hidden, `0` where it reports none.
///
/// Rule 4.3 puts the count in the pane's title, in a long and a short form. The roster is 47 columns
/// wherever rule 5 admits it at all, which admits the long form, so the long form is what is read
/// here. The header's own rows are skipped: rule 5's announcement names the roster too, and only the
/// pane's title reports what the pane could not draw.
fn hidden_reported(buffer: &Buffer) -> usize {
    let title = rows(buffer)
        .into_iter()
        .skip(3)
        .find(|row| row.contains("roster"))
        .expect("the roster pane carries a title");
    let Some(rest) = title.split("hidden ").nth(1) else {
        return 0;
    };
    rest.chars()
        .take_while(|symbol| symbol.is_ascii_digit())
        .collect::<String>()
        .parse()
        .expect("the title states the hidden count as a number")
}

/// The frame's gauge rows, in the order the frame presents them, each with the row it occupies.
fn gauge_rows(buffer: &Buffer) -> Vec<(usize, Vec<Gauge>)> {
    rows(buffer)
        .iter()
        .enumerate()
        .map(|(y, row)| (y, gauges_in(row)))
        .filter(|(_, gauges)| !gauges.is_empty())
        .collect()
}

/// The frame's roster entries, each as the four gauges it presents, in the frame's own order.
///
/// Rule 4 as amended on 2026-08-20 puts the four gauges of one Mokiterion on two adjacent rows —
/// health and satiety above energy and fear — so an entry's gauges are the gauges of a consecutive
/// pair of gauge rows. The pairing asserts the adjacency rather than assuming it: two bar rows a
/// row apart would be two halves of one entry read as neighbours, and an odd number of bar rows
/// would be an entry rendered half-height, so both fail here.
fn gauge_entries(buffer: &Buffer) -> Vec<Vec<Gauge>> {
    let bar_rows = gauge_rows(buffer);
    assert_eq!(
        bar_rows.len() % 2,
        0,
        "{} bar rows do not pair into entries of two",
        bar_rows.len()
    );
    bar_rows
        .chunks(2)
        .map(|pair| {
            let [(first, upper), (second, lower)] = pair else {
                unreachable!("chunks of two over an even length")
            };
            assert_eq!(
                *second,
                first + 1,
                "an entry's two bar rows are at {first} and {second} rather than adjacent"
            );
            upper.iter().chain(lower).cloned().collect()
        })
        .collect()
}

/// Rule 4 as amended on 2026-08-19, and `REQ-MOK-032`'s reporting clause read at the frame: the
/// roster presents four gauges, not three and a gap, at every declared viewport that presents the
/// roster at all.
///
/// Which viewports those are is recorded rather than derived, so that a layout change dropping the
/// roster from a viewport that used to carry it fails here. The list is rule 5's own derived table
/// as amended on 2026-08-19, which admits the roster on width alone at `W >= 100`: eight of its nine
/// declared viewports carry it, and only `34x22` — above the floor but below the roster's width —
/// does not. `33x21` is below the floor and presents no frame at all.
///
/// The list was four viewports when this test was written, against rule 5's withdrawn tier table,
/// under which `100x30` fell to `otherwise` and excluded the roster. That is the merge with
/// `WO-MOK-005`, not a change in rule 4: every viewport presenting the pane presents it 47 columns
/// wide, so the multi-line form and its bars are the same ones the four already asserted.
///
/// Rule 4 as amended on 2026-08-20 spreads the four gauges over two rows of a three-row entry, so
/// the unit compared with the living count is the entry rather than the row, and the columns of the
/// four gauges run two to a row instead of four. Both the count and the placement are asserted in
/// the amended form; neither obligation is dropped. `REQ-MOK-047`'s width is asserted separately as
/// a property of the fill, since a figure asserted here would be the rendering and not the property.
#[test]
fn the_roster_presents_four_gauges_at_every_declared_viewport_that_presents_it() {
    let declared = [
        (160u16, 48u16, true),
        (160, 44, true),
        (160, 40, true),
        (140, 44, true),
        (140, 43, true),
        (120, 48, true),
        (120, 30, true),
        (100, 30, true),
        (34, 22, false),
        (33, 21, false),
    ];

    let mut presenting = 0;
    for (width, height, presents_gauges) in declared {
        let mut observer = start(&[]);
        // Thirty ticks, so that the values asserted below are ones the engine had to compute and
        // not the ones every Mokiterion starts the run holding.
        for _ in 0..30 {
            observer.advance().unwrap();
        }
        let buffer = frame_of(&mut observer, width, height);
        let found = gauge_entries(&buffer);
        assert_eq!(
            !found.is_empty(),
            presents_gauges,
            "{width}x{height} presents {} entries",
            found.len()
        );
        if !presents_gauges {
            continue;
        }
        presenting += 1;

        let agents = &observer.snapshot().agents;
        // The entry is three lines as of rule 4's 2026-08-20 amendment, so a roster interior
        // shorter than 36 rows holds fewer than twelve entries and reports the remainder in its
        // title. What this asserted — one entry per living Mokiterion at every viewport — was true
        // of the two-line entry and is not of the three-line one; the obligation behind it is that
        // no entry is lost silently, and that is asserted here in the form the amendment leaves it:
        // drawn plus reported hidden is the living count. The reference viewport, where nothing is
        // hidden and all twelve are drawn, is `REQ-MOK-020`'s own case and is asserted as such in
        // `every_living_mokiterion_has_an_entry_at_the_reference_viewport`.
        let hidden = hidden_reported(&buffer);
        assert_eq!(
            found.len() + hidden,
            agents.len(),
            "{width}x{height} draws {} entries and reports {hidden} hidden of {} living",
            found.len(),
            agents.len()
        );

        for gauges in &found {
            let labels: Vec<char> = gauges.iter().map(|gauge| gauge.label).collect();
            assert_eq!(
                labels,
                vec!['h', 's', 'e', 'f'],
                "{width}x{height} presents the gauges as {labels:?} rather than health, satiety, \
                 energy, fear"
            );

            // The fourth gauge is on the same terms as the first three: the same bar width, and a
            // bar of at least one cell. A zero-width bar in the fourth slot is `VREC-MOK-005`
            // finding 3, and a narrower one there would be the reserved slot half-honoured.
            let bar = gauges[0].bar();
            assert!(
                bar >= 1,
                "{width}x{height} renders a bar of zero cells, which rule 4.4 forbids"
            );
            for gauge in gauges {
                assert_eq!(
                    gauge.bar(),
                    bar,
                    "{width}x{height} renders the {} gauge {} cells wide against {bar} for health",
                    gauge.label,
                    gauge.bar()
                );
            }

            // Cell positions rather than order alone: each gauge occupies a label, a space, the
            // bar, a space and three value columns, and consecutive gauges on a line are two
            // columns apart. That is the row overhead's arithmetic, asserted against the buffer.
            // Two gauges to a line as of rule 4's 2026-08-20 amendment, so the third restarts at
            // the first's column on the line below — which `gauge_entries` has already asserted is
            // the next row and not some further one.
            for (index, gauge) in gauges.iter().enumerate() {
                let expected = gauges[0].column + (index % 2) * (bar + 8);
                assert_eq!(
                    gauge.column, expected,
                    "{width}x{height} places the {} gauge at column {} rather than {expected}",
                    gauge.label, gauge.column
                );
            }
        }

        // Every presented value is the snapshot's own, the fourth slot on the same footing as the
        // other three. Compared as multisets because a gauge row carries no identifier of its own.
        //
        // Against the drawn window rather than the whole population: nothing is selected here, so
        // rule 4's window opens at the head of the ascending-identifier order and the entries drawn
        // are the first `found.len()` of it. Where the pane holds every entry the two are the same
        // set, which is what this compared before the entry became three lines.
        let drawn = &agents[..found.len()];
        let slots: [(char, Vec<u8>); 4] = [
            ('h', drawn.iter().map(|agent| agent.health).collect()),
            ('s', drawn.iter().map(|agent| agent.satiety).collect()),
            ('e', drawn.iter().map(|agent| agent.energy).collect()),
            ('f', drawn.iter().map(|agent| agent.fear).collect()),
        ];
        for (slot, (label, mut expected)) in slots.into_iter().enumerate() {
            let mut presented: Vec<u8> = found.iter().map(|row| row[slot].value).collect();
            presented.sort_unstable();
            expected.sort_unstable();
            assert_eq!(
                presented, expected,
                "{width}x{height} presents the {label} gauges as {presented:?} against the \
                 snapshot's {expected:?}"
            );
        }
    }

    assert_eq!(
        presenting, 8,
        "eight declared viewports are expected to present the roster's gauge rows"
    );
}

/// `VREC-MOK-005` finding 3, closed: the slot rule 4.5 reserved carries a gauge on rule 4's own
/// terms, at zero as well as away from it.
///
/// The finding was that the fourth slot presented nothing, because no engine attribute stood behind
/// it. Asserting a non-zero value alone would not close it, since a slot could present a value with
/// a bar that is decorative or absent. So the bar is checked against rule 4.4 in both directions:
/// empty at zero, and proportional to the value the snapshot holds once fear has risen.
#[test]
fn the_fourth_gauge_is_a_proportional_bar_at_zero_and_away_from_it() {
    let mut observer = start(&[]);

    // Rule 4.4 at the initial tick, where every Mokiterion's fear is zero: a presented `0` over an
    // empty bar. This is what distinguishes a gauge reading zero from a gauge that is not there.
    let buffer = frame_of(&mut observer, 160, 48);
    let found = gauge_entries(&buffer);
    assert_eq!(
        found.len(),
        12,
        "the reference viewport presents twelve entries"
    );
    for gauges in &found {
        // Checked before the slot is read, so that a row presenting three gauges reports the
        // missing fourth rather than panicking on the index.
        assert_eq!(gauges.len(), 4, "a row presents no fourth gauge");
        let fear = &gauges[3];
        assert_eq!(fear.label, 'f');
        assert_eq!(fear.value, 0);
        assert_eq!(fear.filled, 0);
        assert_eq!(
            fear.shaded,
            fear.bar(),
            "a gauge reading zero renders an empty bar, not an absent one"
        );
    }
    // The same arithmetic read off the frame rather than off the constant, in the form rule 4's
    // 2026-08-20 amendment leaves it: the pane's interior is 45 columns, two gauges to a line make
    // the overhead 19, and `(45 - 19) / 2` is 13. `WO-MOK-010` stop condition 9 was raised against
    // the two cells the four-to-a-line form produced, which is the defect `REQ-MOK-047` closes.
    assert_eq!(found[0][0].bar(), 13, "the roster's bar width moved");

    for _ in 0..30 {
        observer.advance().unwrap();
    }
    let buffer = frame_of(&mut observer, 160, 48);
    let found = gauge_entries(&buffer);
    let agents = &observer.snapshot().agents;
    assert_eq!(found.len(), agents.len());
    assert!(
        agents.iter().any(|agent| agent.fear > 0),
        "no Mokiterion perceived another in thirty ticks, so the fourth gauge is unexercised"
    );

    // Roster order is ascending identifier, which is the snapshot's own order, so the rows pair
    // with the agents positionally.
    for (gauges, agent) in found.iter().zip(agents) {
        assert_eq!(gauges.len(), 4, "{} carries a fifth gauge", agent.id);
        let fear = &gauges[3];
        assert_eq!(
            fear.value, agent.fear,
            "{} presents fear {} against the snapshot's {}",
            agent.id, fear.value, agent.fear
        );
        assert_eq!(
            fear.filled,
            usize::from(agent.fear) * fear.bar() / 100,
            "{}'s fear bar is not proportional to the {} it presents",
            agent.id,
            agent.fear
        );
    }
}

// ---- a gauge that resolves, a control that is findable, and a notice that names the remedy ----
//
// `VER-MOK-013`'s public-tier cases. Each asserts a property rather than a rendering, which is what
// that contract's *Independence* section requires of every case in it: the two-cell gauge passed a
// suite of eighty-seven cases because those cases asserted what they had been told to assert.

/// Rule 5's derived table above the floor, plus the floor itself: every declared viewport the
/// observer draws a frame at. `33 x 21` is below the floor and presents nothing, so it is not here.
const ABOVE_THE_FLOOR: [(u16, u16); 9] = [
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

/// The character `SPEC-MOK-003` rule 7 binds to the key-binding overlay.
///
/// Rule 7 is the source, not the implementation, so this is written down here rather than read from
/// the observer. That the character written down is the one actually bound is not assumed: the cold
/// start case below presses it and asserts the key-binding overlay opens.
const OVERLAY_KEY: char = '?';

/// The character rule 7 binds to the overlay that reaches each pane's content.
fn specified_key(pane: Pane) -> char {
    match pane {
        Pane::Roster => 'r',
        Pane::Log => 'L',
        Pane::Inspector => 'i',
    }
}

/// The axis that constrains `pane`, as an initial, and the extent at which the pane returns.
///
/// Measured from the layout rather than written down. `VER-MOK-013`'s "stated value is the layout's
/// own" row rules out a case that fixes a literal `140` in its expectation, and the two sweeps below
/// ask the layout exactly the question the announcement answers: the smallest extent on each axis at
/// which the pane is part of the body, with the other axis at the largest declared value. The axis
/// that constrains the pane is the one whose answer is above the floor; a pane constrained on neither
/// would never be announced and so has no threshold to state.
fn measured_threshold(pane: Pane) -> (char, u16) {
    let in_the_body = |width: u16, height: u16| {
        !layout::resolve(Rect::new(0, 0, width, height))
            .overlay_only()
            .contains(&pane)
    };
    let by_width = (layout::MIN_WIDTH..=200).find(|&width| in_the_body(width, 60));
    let by_height = (layout::MIN_HEIGHT..=60).find(|&height| in_the_body(200, height));
    match (by_width, by_height) {
        (Some(width), _) if width > layout::MIN_WIDTH => ('W', width),
        (_, Some(height)) if height > layout::MIN_HEIGHT => ('H', height),
        other => panic!("{} is excluded by no extent: {other:?}", pane.label()),
    }
}

/// Whether `row` announces `pane` in one of the forms rule 5's ladder admits.
///
/// Every rung carries the key, the axis and the value; they differ in how much of the surrounding
/// wording they keep, and rule 5 as amended fixes that order of loss. Both the spelled axis and its
/// initial are accepted, since which one a viewport carries is the width's business — and neither is
/// accepted without the value beside it, which is the part `REQ-MOK-049` exists for.
fn announces(row: &str, pane: Pane) -> bool {
    let (axis, value) = measured_threshold(pane);
    let key = specified_key(pane);
    let word = match axis {
        'W' => "width",
        _ => "height",
    };
    row.contains(&format!("{key} at {word} {value}"))
        || row.contains(&format!("{key} {axis}{value}"))
}

/// The runs of emphasised cells on the header's first row, in the order the row presents them.
///
/// Rule 5 as amended emphasises the run state, the permanent hint and the announcement, and leaves
/// the optional segments and the notice unemphasised. The head and the hint are adjacent, so they
/// read as one run; the announcement is right-aligned behind an unemphasised pad, so it reads as a
/// second. A third would mean something else on the line has taken emphasis, and then "the last run
/// is the announcement" would no longer be true, so it fails here rather than passing quietly.
///
/// Read off the cells' modifiers because what rule 5 requires is a style, and asserted as a
/// modifier rather than a colour because rule 2.5's reading has to keep it.
fn emphasised_runs(buffer: &Buffer) -> Vec<String> {
    let mut runs: Vec<String> = Vec::new();
    let mut current = String::new();
    for x in buffer.area.left()..buffer.area.right() {
        let cell = buffer.cell((x, buffer.area.top())).unwrap();
        if cell.modifier.contains(Modifier::BOLD) {
            current.push_str(cell.symbol());
        } else if !current.trim().is_empty() {
            runs.push(current.trim().to_string());
            current = String::new();
        } else {
            current = String::new();
        }
    }
    if !current.trim().is_empty() {
        runs.push(current.trim().to_string());
    }
    assert!(
        (1..=2).contains(&runs.len()),
        "the header's first row carries {} emphasised runs rather than the head with the hint and \
         at most the announcement: {runs:?}",
        runs.len()
    );
    runs
}

/// The permanent hint as the frame carries it, with the run state removed.
///
/// The two share one emphasised run because rule 5 puts them adjacent, and they are not the same
/// kind of thing: the run state is the observer's and two frames of one viewport may legitimately
/// differ in it, while the hint is a function of the viewport alone. The label is read from the
/// observer rather than written down, so this states no run-state wording of its own.
fn hint_on_screen(buffer: &Buffer, observer: &Observer) -> String {
    let head = emphasised_runs(buffer)[0].clone();
    let label = observer.progression().label();
    head.strip_prefix(label)
        .unwrap_or_else(|| {
            panic!("the header's first emphasised run does not open with the run state: {head}")
        })
        .trim()
        .to_string()
}

/// The announcement as the frame carries it, or `None` where the viewport excludes no pane.
fn announcement_on_screen(buffer: &Buffer) -> Option<String> {
    let runs = emphasised_runs(buffer);
    (runs.len() == 2).then(|| runs[1].clone())
}

/// The modifiers of the cells `text` occupies on the header's first row.
fn modifiers_of(buffer: &Buffer, text: &str) -> Vec<Modifier> {
    let row = rows(buffer)[0].clone();
    let at = row
        .find(text)
        .unwrap_or_else(|| panic!("the header's first row does not carry {text}: {row}"));
    let start = u16::try_from(row[..at].chars().count()).unwrap();
    (start..start + u16::try_from(text.chars().count()).unwrap())
        .map(|x| buffer.cell((x, buffer.area.top())).unwrap().modifier)
        .collect()
}

/// `REQ-MOK-020` at the reference viewport, which is what satisfying `REQ-MOK-047` threatened.
///
/// The entry grew from two lines to three so that a gauge could carry a ten-point change, and
/// `12 x 3 = 36` fits the reference roster's interior exactly — with the log held at six rows and
/// not otherwise. Nothing about a fit that exact should be left implicit, so it is asserted at the
/// frame: every living Mokiterion has an entry of its own, and the pane reports nothing hidden. A
/// pass with eleven entries is what `VER-MOK-013` calls a failure of the contract.
#[test]
fn every_living_mokiterion_has_an_entry_at_the_reference_viewport() {
    let mut observer = start(&[]);
    let population = observer.snapshot().agents.len();
    assert_eq!(population, 12, "the initial population moved");

    // Before the run and after it has moved, since the fit is a property of the pane and not of the
    // tick, and a death would otherwise be the only thing keeping this passing.
    for ticks in [0u64, 30] {
        while observer.snapshot().tick < ticks {
            observer.advance().unwrap();
            assert!(!observer.is_finished(), "the run ended before tick {ticks}");
        }
        let buffer = frame_of(&mut observer, 160, 48);
        let living: Vec<String> = observer
            .snapshot()
            .agents
            .iter()
            .map(|agent| agent.id.clone())
            .collect();
        for id in &living {
            // Panics naming the identifier where the entry is absent, which is the report wanted.
            let row = entry_row(&buffer, id);
            let bars = gauges_in(&rows(&buffer)[usize::from(row) + 1]);
            assert_eq!(bars.len(), 2, "{id}'s entry carries {} gauges", bars.len());
        }
        assert_eq!(
            gauge_entries(&buffer).len(),
            living.len(),
            "at tick {ticks}"
        );
        assert_eq!(
            hidden_reported(&buffer),
            0,
            "the reference viewport hides an entry at tick {ticks}"
        );
    }
}

/// `VER-MOK-013` acceptance scenario 1: the declining Mokiterion.
///
/// This is the scenario the two-cell gauge fails. At `ff3a155` a fall from 99 to 50 left the filled
/// count at 1, so an operator watching the bar saw nothing happen. The subject is the Mokiterion
/// that declines furthest over the run rather than a named one, so the case survives a change in how
/// fast the world moves; the seed is declared, so which one that is remains fixed.
///
/// Two properties are asserted, and the second is the one that matters. The fill at the end is
/// strictly below the fill at the start — the scenario's own condition. And across every sample, the
/// fill is monotone in the value: no pair of frames presents a lower health over a fuller bar. A
/// gauge can satisfy the first with a single step somewhere in the middle of the range; only the
/// second says the bar tracks the decline.
///
/// **The run selects `social` from 2026-08-21, where it took the default source before.** The
/// scenario `VER-MOK-013` states is "200 ticks at a declared seed at the reference viewport" and
/// names no source, so this is a change of scenario parameter and not of the scenario. The reason it
/// was needed is `REQ-MOK-060`: the corrected waste condition feeds the population well enough that
/// at 200 ticks **no declared seed under `reference` or `individual` produces any health fall at
/// all** — seed 42 fell 35 points before the correction and falls 0 after it, and the deepest fall
/// over all five declared seeds under either source is now 0. Health falls only once satiety reaches
/// zero, so a better-fed world has no declining health to draw, and the ≥30 guard below fired.
///
/// `social` is the durable choice rather than a longer run, because there the decline is combat
/// damage and not starvation: seed 42's deepest fall is 78 points at **both** commits, so this test
/// is no longer coupled to the nutrition model it has twice been broken by. The guard stays at
/// thirty and the assertions are untouched.
#[test]
fn a_declining_mokiterion_shows_a_declining_bar() {
    /// The scenario's own run length, declared to the run rather than assumed of it: the default
    /// configuration stops at a hundred, and `advance` on a finished run does nothing rather than
    /// refusing, so a run asked for two hundred ticks has to be told so.
    const TICKS: u64 = 200;
    const LENGTH: usize = 200;

    // The subject is found without drawing, so the pass that draws reads one gauge per frame.
    let mut observer = start(&["--seed", "42", "--ticks", "200", "--policy", "social"]);
    let mut series: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for _ in 0..LENGTH {
        for agent in &observer.snapshot().agents {
            series
                .entry(agent.id.clone())
                .or_default()
                .push(agent.health);
        }
        observer.advance().unwrap();
    }
    assert_eq!(
        observer.snapshot().tick,
        TICKS,
        "the run stopped short of the scenario's length"
    );
    let length = LENGTH;
    let (subject, fall) = series
        .iter()
        .filter(|(_, health)| health.len() == length)
        .map(|(id, health)| {
            (
                id.clone(),
                i32::from(health[0]) - i32::from(*health.last().unwrap()),
            )
        })
        .max_by_key(|(_, fall)| *fall)
        .expect("the run has a subject that lives the whole way");
    assert!(
        fall >= 30,
        "no Mokiterion alive at tick {TICKS} on seed 42 fell thirty points of health; the deepest \
         fall was {fall}, so this scenario is unexercised"
    );

    // The same run again, drawing this time. The two passes must select the same source or the
    // subject found by the first is not the subject drawn by the second.
    let mut observer = start(&["--seed", "42", "--ticks", "200", "--policy", "social"]);
    let mut samples: Vec<(u8, usize)> = Vec::new();
    for _ in 0..LENGTH {
        let buffer = frame_of(&mut observer, 160, 48);
        let row = entry_row(&buffer, &subject);
        let gauges = gauges_in(&rows(&buffer)[usize::from(row) + 1]);
        let health = gauges
            .first()
            .expect("rule 4 opens the entry's first bar row with the health gauge");
        assert_eq!(health.label, 'h');
        samples.push((health.value, health.filled));
        observer.advance().unwrap();
    }

    let (first_value, first_fill) = samples[0];
    let (last_value, last_fill) = *samples.last().unwrap();
    println!(
        "{subject} fell from {first_value} to {last_value} over {TICKS} ticks: {first_fill} filled \
         cells to {last_fill}"
    );
    assert!(
        last_fill < first_fill,
        "{subject}'s health fell {fall} points and its bar went from {first_fill} filled cells to \
         {last_fill}"
    );
    for (value, fill) in &samples {
        for (other_value, other_fill) in &samples {
            if value <= other_value {
                assert!(
                    fill <= other_fill,
                    "{subject} presents {value} over {fill} filled cells and {other_value} over \
                     {other_fill}, so the bar does not track the decline"
                );
            }
        }
    }
}

/// `REQ-MOK-048` and `VER-MOK-013` acceptance scenario 2: the cold start.
///
/// The defect was that the key opening the key-binding overlay appeared nowhere on screen, so an
/// operator had to already know it. What closes that is the character being on screen in the frame
/// the observer draws before any input reaches it, at every viewport it draws a frame at — which is
/// why no key is pressed above and the tick is asserted to be zero.
///
/// The character asserted is the one rule 7 names. That the character rule 7 names is the one
/// actually bound is asserted at the end, by pressing it: a hint naming a key that opens nothing
/// would satisfy the requirement's letter and lose its purpose.
#[test]
fn the_key_binding_hint_is_on_screen_in_the_first_frame_at_every_viewport() {
    for (width, height) in ABOVE_THE_FLOOR {
        let mut observer = start(&[]);
        let buffer = frame_of(&mut observer, width, height);
        let header = rows(&buffer)[0].clone();
        assert!(
            header.contains(OVERLAY_KEY),
            "{width}x{height} draws no {OVERLAY_KEY} in the header: {header}"
        );
        // Emphasised, so it reads as an affordance rather than as one of the optional segments, and
        // in the run of the head rather than in the announcement's.
        assert!(
            emphasised_runs(&buffer)[0].contains(OVERLAY_KEY),
            "{width}x{height} draws the hint without emphasis: {header}"
        );
        assert_eq!(observer.snapshot().tick, 0, "the frame is not the first");
        assert_eq!(
            observer.overlay(),
            Overlay::None,
            "no overlay is open before any key is delivered"
        );
    }

    let mut observer = start(&[]);
    let _ = frame_of(&mut observer, 160, 48);
    observer
        .handle_key(press(ratatui::crossterm::event::KeyCode::Char(OVERLAY_KEY)))
        .unwrap();
    assert_eq!(
        observer.overlay(),
        Overlay::Help,
        "{OVERLAY_KEY} does not open the key-binding overlay, so the hint names the wrong key"
    );
}

/// `REQ-MOK-048`'s "it is not timed" row: the hint is an affordance, not a startup message.
///
/// Both run states, because a held run draws the same header and an operator who holds the run is
/// exactly the one looking for the controls.
#[test]
fn the_hint_is_present_after_two_hundred_ticks_in_both_run_states() {
    for args in [
        vec!["--ticks", "200"],
        vec!["--ticks", "200", "--start-paused"],
    ] {
        let mut observer = start(&args);
        let state = observer.progression();
        for _ in 0..200 {
            // Drawn every tick, so the assertion below is not the one frame in two hundred that
            // happens to carry it.
            let buffer = frame_of(&mut observer, 160, 48);
            let header = rows(&buffer)[0].clone();
            assert!(
                header.contains(OVERLAY_KEY),
                "{state:?} loses the hint at tick {}: {header}",
                observer.snapshot().tick
            );
            observer.advance().unwrap();
        }
        assert_eq!(observer.snapshot().tick, 200);
        assert_eq!(
            observer.progression(),
            state,
            "drawing changed the run state"
        );
    }
}

/// `REQ-MOK-048`'s "it displaces no obligation" row: the hint is reserved beside the announcement
/// and rule 8's footer, not instead of either.
///
/// The narrowest viewports are where a reservation is decided, and at the floor all three of them
/// are due at once on 34 columns. What this asserts is that none of the three is the one that gave
/// way.
#[test]
fn the_hint_displaces_neither_the_announcement_nor_the_footer() {
    for (width, height) in ABOVE_THE_FLOOR {
        let mut observer = start(&[]);
        let buffer = frame_of(&mut observer, width, height);
        let header = rows(&buffer)[0].clone();
        assert!(header.contains(OVERLAY_KEY), "{width}x{height}: {header}");

        for pane in layout::resolve(Rect::new(0, 0, width, height)).overlay_only() {
            assert!(
                announces(&header, pane),
                "{width}x{height} excludes the {} and does not announce it: {header}",
                pane.label()
            );
        }

        // Rule 8's provenance line, in whichever of its two forms the width carries.
        let footer = rows(&buffer).last().unwrap().clone();
        assert!(
            footer.contains("seed") || footer.contains("s0"),
            "{width}x{height}: {footer}"
        );
        assert!(
            footer.contains("tick") || footer.contains("@"),
            "{width}x{height}: {footer}"
        );
    }
}

/// `REQ-MOK-049`: for every pane the viewport excludes, the announcement names the axis that
/// excludes it, the extent at which it returns, and the key that reaches it meanwhile.
///
/// The value is measured from the layout, never written down, which is the condition
/// `VER-MOK-013` sets for this row: a case fixing a literal `140` would pass a layout that had
/// stopped agreeing with it. The panes the viewport does present are asserted to be absent from the
/// announcement, so an implementation naming all three everywhere does not pass.
///
/// The floor is `VER-MOK-013`'s "every excluded pane is announced" row: all three at once, on 34
/// columns, is the case the ladder's shortest rung exists for.
#[test]
fn the_announcement_states_the_axis_and_the_value_the_layout_decides_presence_from() {
    let mut announcing = 0;
    for (width, height) in ABOVE_THE_FLOOR {
        let mut observer = start(&[]);
        let buffer = frame_of(&mut observer, width, height);
        let header = rows(&buffer)[0].clone();
        let excluded = layout::resolve(Rect::new(0, 0, width, height)).overlay_only();

        for pane in [Pane::Roster, Pane::Log, Pane::Inspector] {
            let due = excluded.contains(&pane);
            assert_eq!(
                announces(&header, pane),
                due,
                "{width}x{height} {} the {}: {header}",
                if due { "excludes" } else { "presents" },
                pane.label()
            );
        }

        assert_eq!(
            announcement_on_screen(&buffer).is_some(),
            !excluded.is_empty(),
            "{width}x{height} excludes {} panes: {header}",
            excluded.len()
        );
        if !excluded.is_empty() {
            announcing += 1;
        }
    }
    assert_eq!(
        announcing, 4,
        "four declared viewports above the floor are expected to exclude a pane"
    );
}

/// `REQ-MOK-049`'s emphasis row, read off the cells rather than off the wording.
///
/// The defect was a notice in unstyled text among the optional segments, indistinguishable from
/// them. So the comparison is against those segments on the same line: the announcement carries a
/// modifier they do not. `filter` is the segment used because rule 5 makes it the last to be
/// dropped, so it is present wherever any optional segment is.
#[test]
fn the_announcement_is_emphasised_and_the_optional_segments_are_not() {
    let mut observer = start(&[]);
    let buffer = frame_of(&mut observer, 120, 48);
    let announcement = announcement_on_screen(&buffer).expect("120 columns excludes the inspector");
    assert!(announces(&announcement, Pane::Inspector), "{announcement}");

    for modifier in modifiers_of(&buffer, "filter") {
        assert!(
            !modifier.contains(Modifier::BOLD),
            "an optional segment carries the announcement's emphasis"
        );
    }
    for modifier in modifiers_of(&buffer, &announcement) {
        assert!(
            modifier.contains(Modifier::BOLD),
            "the announcement is not emphasised: {announcement}"
        );
        // Rule 4.6 spends reversed video on the roster's selection, so the emphasis here is not it.
        assert!(
            !modifier.contains(Modifier::REVERSED),
            "the announcement takes rule 4.6's reversed video"
        );
    }
}

/// `VER-MOK-013`'s layout-purity property, over the announcement and the hint.
///
/// Both are functions of the viewport and the excluded-pane set alone. A tick, a run state, an
/// advanced entropy state and a selection all change the header's optional detail and must change
/// neither of them — which is asserted by comparing the emphasised runs and not the whole line,
/// since a selection legitimately adds `sel` to the segments between them.
#[test]
fn the_announcement_and_the_hint_read_nothing_but_the_viewport() {
    let mut fresh = start(&["--seed", "42"]);
    let expected: Vec<(String, Option<String>)> = ABOVE_THE_FLOOR
        .iter()
        .map(|(width, height)| {
            let buffer = frame_of(&mut fresh, *width, *height);
            (
                hint_on_screen(&buffer, &fresh),
                announcement_on_screen(&buffer),
            )
        })
        .collect();

    let mut moved = start(&["--seed", "42", "--ticks", "200", "--start-paused"]);
    for _ in 0..137 {
        moved.advance().unwrap();
    }
    assert_eq!(moved.snapshot().tick, 137, "the run stopped short");
    moved
        .handle_key(press(ratatui::crossterm::event::KeyCode::Tab))
        .unwrap();
    assert!(moved.selection().is_some(), "the walk selected nothing");

    for ((width, height), (hint, announcement)) in ABOVE_THE_FLOOR.iter().zip(&expected) {
        let buffer = frame_of(&mut moved, *width, *height);
        assert_eq!(
            &hint_on_screen(&buffer, &moved),
            hint,
            "{width}x{height} reads the hint off something other than the viewport"
        );
        assert_eq!(
            &announcement_on_screen(&buffer),
            announcement,
            "{width}x{height} reads the announcement off something other than the viewport"
        );
    }
}

/// `VER-MOK-013` acceptance scenario 5: crossing a threshold in both directions.
///
/// One observer, so the crossing is a resize of a running instrument and not two runs compared. The
/// announcement appears with the exclusion, disappears with it, and states the same extent both
/// times — the last of which is what a value computed per frame from the current width would fail.
#[test]
fn the_announcement_appears_and_disappears_with_the_pane_it_names() {
    let (_, threshold) = measured_threshold(Pane::Inspector);
    let mut observer = start(&[]);

    let narrow = rows(&frame_of(&mut observer, threshold - 1, 48))[0].clone();
    assert!(announces(&narrow, Pane::Inspector), "{narrow}");

    let wide = rows(&frame_of(&mut observer, threshold, 48))[0].clone();
    assert!(!announces(&wide, Pane::Inspector), "{wide}");
    assert!(
        layout::resolve(Rect::new(0, 0, threshold, 48))
            .inspector
            .is_some(),
        "the pane did not return at the extent the announcement stated"
    );

    let narrow_again = rows(&frame_of(&mut observer, threshold - 1, 48))[0].clone();
    assert_eq!(
        narrow_again, narrow,
        "the announcement does not read the same across a crossing in both directions"
    );
}

/// `VER-MOK-013`'s no-entry-lost-silently invariant, over every roster the layout can produce.
///
/// This is what makes an entry falling off the pane an announced fact. It is load-bearing below the
/// reference viewport, where the three-line entry holds fewer than twelve and `WO-MOK-013`'s
/// decision 1 does not claim otherwise.
///
/// The plane is swept at the layout and one frame is drawn per distinct roster rectangle. Two
/// viewports resolving the same roster pane hand the same rectangle to the same drawing code and the
/// same population, so the second frame would assert what the first did; sweeping the layout is what
/// makes sure no rectangle the plane can produce is missed.
#[test]
fn no_entry_is_lost_silently_at_any_viewport_presenting_the_roster() {
    let mut observer = start(&[]);
    for _ in 0..30 {
        observer.advance().unwrap();
    }
    let living = observer.snapshot().agents.len();

    let mut geometries: BTreeMap<(u16, u16, u16, u16), (u16, u16)> = BTreeMap::new();
    for width in layout::MIN_WIDTH..=200 {
        for height in layout::MIN_HEIGHT..=60 {
            if let Some(roster) = layout::resolve(Rect::new(0, 0, width, height)).roster {
                geometries
                    .entry((roster.x, roster.y, roster.width, roster.height))
                    .or_insert((width, height));
            }
        }
    }

    let mut hiding = 0;
    for (width, height) in geometries.values() {
        let buffer = frame_of(&mut observer, *width, *height);
        let drawn = gauge_entries(&buffer).len();
        let hidden = hidden_reported(&buffer);
        assert_eq!(
            drawn + hidden,
            living,
            "{width}x{height} draws {drawn} entries and reports {hidden} hidden of {living} living"
        );
        if hidden > 0 {
            hiding += 1;
        }
    }
    println!(
        "{} distinct roster panes over the plane, {hiding} of them hiding an entry",
        geometries.len()
    );
    assert!(
        hiding > 0,
        "no roster the plane can produce hides an entry, so the reporting clause is unexercised"
    );
}
