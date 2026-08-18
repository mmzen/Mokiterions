//! Public tier: layout.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Every one of them was in
//! `src/layout.rs`'s `#[cfg(test)] mod tests` block and reached the code through items that were
//! already public, so the move changes the path and nothing else: the assertions are verbatim and
//! no item was widened to bring them out. `SPEC-MOK-004` rule 12 is the obligation and the
//! per-test comparison under `WO-MOK-006` is the evidence.

use mokiterions_tui::layout::*;
use ratatui::layout::Rect;

fn viewport(width: u16, height: u16) -> Rect {
    Rect::new(0, 0, width, height)
}

#[test]
fn the_floor_is_the_specified_one() {
    assert!(!below_floor(34, 22));
    assert!(below_floor(33, 22));
    assert!(below_floor(34, 21));
    assert!(below_floor(0, 0));
    assert!(!below_floor(200, 60));
}

#[test]
fn tiers_match_the_specified_table_including_its_boundaries() {
    assert_eq!(tier_for(140, 48), Tier::A);
    assert_eq!(tier_for(160, 48), Tier::A);
    assert_eq!(tier_for(140, 47), Tier::B);
    assert_eq!(tier_for(140, 44), Tier::B);
    assert_eq!(tier_for(160, 44), Tier::B);
    assert_eq!(tier_for(139, 48), Tier::C);
    assert_eq!(tier_for(100, 38), Tier::C);
    assert_eq!(tier_for(120, 48), Tier::C);
    assert_eq!(tier_for(99, 48), Tier::D);
    assert_eq!(tier_for(100, 37), Tier::D);
    assert_eq!(tier_for(140, 43), Tier::D);
    assert_eq!(tier_for(34, 22), Tier::D);
}

/// Rule 5's derived-consequences table, which is an obligation because it is checkable.
#[test]
fn the_declared_viewports_yield_the_declared_canvases() {
    let cases = [
        (160, 48, Tier::A, 67, 32),
        (160, 44, Tier::B, 67, 32),
        (140, 44, Tier::B, 47, 32),
        (120, 48, Tier::C, 71, 36),
        (100, 30, Tier::D, 98, 24),
        (34, 22, Tier::D, 32, 16),
    ];

    for (width, height, tier, canvas_width, canvas_height) in cases {
        let panes = resolve(viewport(width, height));
        assert_eq!(panes.tier, tier, "{width}x{height} tier");
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
        for height in [22u16, 30, 37, 38, 43, 44, 47, 48, 60] {
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

#[test]
fn tier_minimums_hold_wherever_the_tier_declares_one() {
    // Tiers A and B declare `Min(34)` for the body, so the body must never be smaller.
    for (width, height) in [(140, 48), (160, 48), (140, 44), (160, 47), (200, 100)] {
        let panes = resolve(viewport(width, height));
        assert!(
            panes.view.height >= 34,
            "{width}x{height} body {}",
            panes.view.height
        );
    }
}

#[test]
fn excluded_panes_are_the_ones_the_tier_omits() {
    assert!(resolve(viewport(160, 48)).overlay_only().is_empty());
    assert!(resolve(viewport(160, 44)).overlay_only().is_empty());
    assert_eq!(
        resolve(viewport(120, 48)).overlay_only(),
        vec![Pane::Inspector]
    );
    assert_eq!(
        resolve(viewport(34, 22)).overlay_only(),
        vec![Pane::Roster, Pane::Log, Pane::Inspector]
    );
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
