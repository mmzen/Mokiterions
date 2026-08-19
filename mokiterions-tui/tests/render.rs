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

/// The frame's gauge rows, in the order the frame presents them.
fn gauge_rows(buffer: &Buffer) -> Vec<Vec<Gauge>> {
    rows(buffer)
        .iter()
        .map(|row| gauges_in(row))
        .filter(|gauges| !gauges.is_empty())
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
/// wide, so the two-line form and its two-cell bars are the same ones the four already asserted.
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
        let found = gauge_rows(&buffer);
        assert_eq!(
            !found.is_empty(),
            presents_gauges,
            "{width}x{height} presents {} gauge rows",
            found.len()
        );
        if !presents_gauges {
            continue;
        }
        presenting += 1;

        let agents = &observer.snapshot().agents;
        assert_eq!(
            found.len(),
            agents.len(),
            "{width}x{height} presents {} gauge rows for {} living Mokiterions",
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
            // bar, a space and three value columns, and consecutive gauges are two columns apart.
            // That is the arithmetic `BAR_ROW_OVERHEAD` encodes, asserted against the buffer.
            for (index, gauge) in gauges.iter().enumerate() {
                let expected = gauges[0].column + index * (bar + 8);
                assert_eq!(
                    gauge.column, expected,
                    "{width}x{height} places the {} gauge at column {} rather than {expected}",
                    gauge.label, gauge.column
                );
            }
        }

        // Every presented value is the snapshot's own, the fourth slot on the same footing as the
        // other three. Compared as multisets because a gauge row carries no identifier of its own.
        let slots: [(char, Vec<u8>); 4] = [
            ('h', agents.iter().map(|agent| agent.health).collect()),
            ('s', agents.iter().map(|agent| agent.satiety).collect()),
            ('e', agents.iter().map(|agent| agent.energy).collect()),
            ('f', agents.iter().map(|agent| agent.fear).collect()),
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
    let found = gauge_rows(&buffer);
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
    // `WO-MOK-007` stop condition 9's arithmetic, read off the frame rather than off the constant:
    // the pane's interior is 45 columns, `(45 - 35) / 4` is 2, and two cells is at least one, so
    // the narrowing the fourth gauge causes needs no escalation.
    assert_eq!(found[0][0].bar(), 2, "the roster's bar width moved");

    for _ in 0..30 {
        observer.advance().unwrap();
    }
    let buffer = frame_of(&mut observer, 160, 48);
    let found = gauge_rows(&buffer);
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
