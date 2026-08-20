//! Public tier: layout.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Every one of them was in
//! `src/layout.rs`'s `#[cfg(test)] mod tests` block and reached the code through items that were
//! already public, so the move changes the path and nothing else: the assertions are verbatim and
//! no item was widened to bring them out. `SPEC-MOK-004` rule 12 is the obligation and the
//! per-test comparison under `WO-MOK-006` is the evidence.
//!
//! `SPEC-MOK-003` rule 5's 2026-08-19 amendment replaced the ordered tier table with one
//! threshold per pane on the axis that constrains it. The cases that named a tier are restated
//! against those thresholds, and the sweep below is new: the defect the amendment corrects sat
//! between the declared viewports, so a set of named sizes could not have caught it.

use mokiterions_tui::layout::*;
use ratatui::layout::Rect;

fn viewport(width: u16, height: u16) -> Rect {
    Rect::new(0, 0, width, height)
}

fn present(width: u16, height: u16) -> (bool, bool, bool) {
    let panes = resolve(viewport(width, height));
    (
        panes.roster.is_some(),
        panes.inspector.is_some(),
        panes.log.is_some(),
    )
}

#[test]
fn the_floor_is_the_specified_one() {
    assert!(!below_floor(34, 22));
    assert!(below_floor(33, 22));
    assert!(below_floor(34, 21));
    assert!(below_floor(0, 0));
    assert!(!below_floor(200, 60));
}

/// Rule 5's thresholds, at their boundaries and one cell either side of each. Each pane reads one
/// axis, so each threshold is asserted to be indifferent to the other axis.
#[test]
fn each_pane_appears_at_its_threshold_on_the_axis_that_constrains_it() {
    for height in [22u16, 30, 37, 38, 43, 44, 48, 60] {
        assert!(!present(99, height).0, "roster at 99x{height}");
        assert!(present(100, height).0, "roster at 100x{height}");
        assert!(!present(139, height).1, "inspector at 139x{height}");
        assert!(present(140, height).1, "inspector at 140x{height}");
    }
    for width in [34u16, 99, 100, 139, 140, 200] {
        assert!(!present(width, 37).2, "log at {width}x37");
        assert!(present(width, 38).2, "log at {width}x38");
    }
}

/// The log is six rows wherever rule 5 admits it, as amended 2026-08-20, and absent below `H = 38`.
///
/// This is `the_log_is_ten_rows_only_where_both_thresholds_are_met` renamed and corrected. The
/// ten-row height it asserted no longer occurs at any viewport, but the case it belongs to does not
/// disappear with it: a constant is a pass condition, and it is the one `VER-MOK-005`'s log-height
/// row states as amended. The two viewports that carried ten are kept as the cases that now read
/// six, so the withdrawn growth is asserted absent rather than left untested.
#[test]
fn the_log_is_six_rows_wherever_it_is_present() {
    let rows = |width, height| {
        resolve(viewport(width, height))
            .log
            .map_or(0, |log| log.height)
    };

    assert_eq!(rows(140, 48), 6);
    assert_eq!(rows(200, 60), 6);
    assert_eq!(rows(139, 48), 6);
    assert_eq!(rows(140, 47), 6);
    assert_eq!(rows(140, 38), 6);
    assert_eq!(rows(34, 60), 6);
    assert_eq!(rows(200, 37), 0);

    // And over the plane, so no width reaches a second height: the log reads one axis, and after
    // this amendment it reads one threshold on it.
    for width in 34..=200u16 {
        for height in 22..=60u16 {
            let expected = if height >= 38 { 6 } else { 0 };
            assert_eq!(rows(width, height), expected, "log at {width}x{height}");
        }
    }
}

/// Rule 5's derived-consequences table, which is an obligation because it is checkable.
///
/// `160 x 40`, `140 x 43` and `120 x 30` are the shapes at which the superseded tier table
/// matched no row and excluded the roster, the inspector and the log at once.
#[test]
fn the_declared_viewports_yield_the_declared_canvases() {
    // width, height, roster, inspector, log rows, canvas width, canvas height
    //
    // The reference row reads a six-row log and a `67 x 36` canvas as amended 2026-08-20. It is the
    // only row the amendment moves: every other declared viewport already failed one of the two
    // conditions the withdrawn ten-row growth required, or has no log at all.
    let cases = [
        (160u16, 48u16, true, true, 6u16, 67u16, 36u16),
        (160, 44, true, true, 6, 67, 32),
        (160, 40, true, true, 6, 67, 28),
        (140, 44, true, true, 6, 47, 32),
        (140, 43, true, true, 6, 47, 31),
        (120, 48, true, false, 6, 71, 36),
        (120, 30, true, false, 0, 71, 24),
        (100, 30, true, false, 0, 51, 24),
        (34, 22, false, false, 0, 32, 16),
    ];

    for (width, height, roster, inspector, log_rows, canvas_width, canvas_height) in cases {
        let panes = resolve(viewport(width, height));
        assert_eq!(panes.roster.is_some(), roster, "{width}x{height} roster");
        assert_eq!(
            panes.inspector.is_some(),
            inspector,
            "{width}x{height} inspector"
        );
        assert_eq!(
            panes.log.map_or(0, |log| log.height),
            log_rows,
            "{width}x{height} log"
        );
        assert_eq!(
            canvas_cells(panes.view),
            (canvas_width, canvas_height),
            "{width}x{height} canvas"
        );
    }
}

#[test]
fn every_region_stays_inside_the_viewport_and_the_body_rows_are_contiguous() {
    for width in [34u16, 60, 99, 100, 120, 139, 140, 157, 160, 200] {
        for height in [22u16, 30, 37, 38, 40, 43, 44, 47, 48, 60] {
            let area = viewport(width, height);
            let panes = resolve(area);

            assert_eq!(panes.header.y, 0);
            assert_eq!(panes.header.height, 3);
            assert_eq!(panes.footer.y, height - 1);
            assert_eq!(panes.footer.height, 1);

            let body = panes.view;
            assert_eq!(body.y, 3, "{width}x{height}");
            let log_height = panes.log.map_or(0, |log| log.height);
            assert_eq!(body.height + log_height + 4, height, "{width}x{height}");
            if let Some(log) = panes.log {
                assert_eq!(log.y, body.y + body.height);
                assert_eq!(log.y + log.height, panes.footer.y);
            } else {
                assert_eq!(body.y + body.height, panes.footer.y);
            }

            assert_eq!(panes.overlay.y, panes.header.height);
            assert_eq!(panes.overlay.y + panes.overlay.height, panes.footer.y);

            // The body columns tile the width exactly, left to right, with no overlap.
            let mut cursor = 0;
            for pane in [panes.roster, Some(panes.view), panes.inspector]
                .into_iter()
                .flatten()
            {
                assert_eq!(pane.x, cursor, "{width}x{height}");
                cursor += pane.width;
            }
            assert_eq!(cursor, width, "{width}x{height}");
            assert!(panes.view.width >= 3, "{width}x{height}");
            assert!(panes.view.height >= 3, "{width}x{height}");
        }
    }
}

/// Rule 5's monotonicity obligation and `VER-MOK-005`'s case for it: no pane present at one
/// viewport is absent at a wider or a taller one.
///
/// Adjacent steps are asserted, which gives the general `W' >= W` and `H' >= H` form by
/// induction over the two axes. This sweeps the plane rather than the declared viewports because
/// the defect it exists to exclude was invisible to every named size: the superseded tier table
/// dropped three panes at `W >= 140` with `38 <= H < 44`, and no declared viewport was in that
/// region.
#[test]
fn enlarging_the_viewport_never_removes_a_pane() {
    let mut pairs = 0usize;
    for width in 34..=200u16 {
        for height in 22..=60u16 {
            let (roster, inspector, log) = present(width, height);
            for (next_width, next_height) in [(width + 1, height), (width, height + 1)] {
                let (next_roster, next_inspector, next_log) = present(next_width, next_height);
                for (pane, before, after) in [
                    ("roster", roster, next_roster),
                    ("inspector", inspector, next_inspector),
                    ("log", log, next_log),
                ] {
                    assert!(
                        after || !before,
                        "the {pane} left the body between {width}x{height} and \
                         {next_width}x{next_height}"
                    );
                }
                pairs += 1;
            }
        }
    }
    // 167 widths by 39 heights, each compared with its wider and its taller neighbour.
    assert_eq!(pairs, 13_026);
}

#[test]
fn excluded_panes_are_the_ones_the_viewport_omits() {
    assert!(resolve(viewport(160, 48)).overlay_only().is_empty());
    assert!(resolve(viewport(160, 44)).overlay_only().is_empty());
    assert!(resolve(viewport(160, 40)).overlay_only().is_empty());
    assert_eq!(
        resolve(viewport(120, 48)).overlay_only(),
        vec![Pane::Inspector]
    );
    assert_eq!(
        resolve(viewport(120, 30)).overlay_only(),
        vec![Pane::Log, Pane::Inspector]
    );
    assert_eq!(
        resolve(viewport(34, 22)).overlay_only(),
        vec![Pane::Roster, Pane::Log, Pane::Inspector]
    );
}

/// Rule 4's entry height as amended on 2026-08-20: an identity line and two bar lines.
///
/// Stated here rather than imported. The implementation's own figure would make this case pass
/// whatever that figure said, and what `VER-MOK-013` asks is whether the pane the layout resolves
/// holds the population at the height the specification fixes.
const SPECIFIED_ENTRY_ROWS: u16 = 3;

/// `SPEC-MOK-001`'s population: the number of Mokiterions a run creates, and therefore the number of
/// entries `REQ-MOK-020` requires the roster present at the reference viewport.
const SPECIFIED_POPULATION: u16 = 12;

/// `VER-MOK-013`'s "the reference interior is what the fit assumes" row.
///
/// `REQ-MOK-047` was met by growing the entry to three lines, and three lines times twelve entries is
/// thirty-six rows — exactly the roster interior the reference viewport has once rule 5 holds the log
/// at six. The fit is exact in both directions, so it is asserted in both: the interior is thirty-six
/// rows, and thirty-six rows hold the whole population and no more. A later change to the log's
/// height, to the entry's height or to the population fails here, rather than at an entry silently
/// vanishing from a pane.
#[test]
fn the_reference_roster_interior_holds_the_whole_population() {
    let panes = resolve(viewport(160, 48));
    let roster = panes
        .roster
        .expect("the reference viewport presents the roster");
    assert_eq!(
        panes.log.map_or(0, |log| log.height),
        6,
        "the interior below follows from a six-row log"
    );

    // A bordered pane spends its first and last row on the border, so the interior is two fewer.
    let interior = roster.height - 2;
    assert_eq!(interior, 36, "the reference roster interior moved");
    assert!(
        interior / SPECIFIED_ENTRY_ROWS >= SPECIFIED_POPULATION,
        "an interior of {interior} rows holds {} entries of {SPECIFIED_ENTRY_ROWS} rows against a \
         population of {SPECIFIED_POPULATION}",
        interior / SPECIFIED_ENTRY_ROWS
    );
    assert_eq!(
        interior % SPECIFIED_ENTRY_ROWS,
        0,
        "the interior does not divide into whole entries, so the pane ends mid-entry"
    );
    // No row to spare, which is the whole reason rule 5's log height is load-bearing here: one more
    // row of log and the twelfth entry is the one that goes.
    assert_eq!(interior / SPECIFIED_ENTRY_ROWS, SPECIFIED_POPULATION);
}

/// Rule 5 states the 1:1-with-inspector threshold as `47 + 44 + 66 = 157`.
#[test]
fn the_one_to_one_threshold_with_the_inspector_shown_is_157_columns() {
    for width in 140..157u16 {
        let panes = resolve(viewport(width, 48));
        assert!(panes.inspector.is_some(), "{width}");
        assert!(canvas_cells(panes.view).0 < 64, "{width}");
    }
    for width in 157..170u16 {
        let panes = resolve(viewport(width, 48));
        assert!(panes.inspector.is_some(), "{width}");
        assert!(canvas_cells(panes.view).0 >= 64, "{width}");
    }
}

/// And with the roster but no inspector as `47 + 66 = 113`.
#[test]
fn the_one_to_one_threshold_with_the_roster_alone_is_113_columns() {
    for width in 100..113u16 {
        let panes = resolve(viewport(width, 48));
        assert!(
            panes.roster.is_some() && panes.inspector.is_none(),
            "{width}"
        );
        assert!(canvas_cells(panes.view).0 < 64, "{width}");
    }
    for width in 113..140u16 {
        let panes = resolve(viewport(width, 48));
        assert!(
            panes.roster.is_some() && panes.inspector.is_none(),
            "{width}"
        );
        assert!(canvas_cells(panes.view).0 >= 64, "{width}");
    }
}

/// Rule 5 states the vertical 1:1 threshold as `H >= 44`: a canvas of 32 rows needs a body of 34,
/// and the header, the footer and a six-row log take ten more. It holds at every width, which is
/// why height alone decides it.
#[test]
fn the_vertical_one_to_one_threshold_is_44_rows() {
    for width in [34u16, 99, 100, 139, 140, 160, 200] {
        for height in 22..44u16 {
            let panes = resolve(viewport(width, height));
            assert!(canvas_cells(panes.view).1 < 32, "{width}x{height}");
        }
        for height in 44..=60u16 {
            let panes = resolve(viewport(width, height));
            assert!(canvas_cells(panes.view).1 >= 32, "{width}x{height}");
        }
    }
}
