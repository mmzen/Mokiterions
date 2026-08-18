//! Public tier: spatial.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Every one of them was in
//! `src/spatial.rs`'s `#[cfg(test)] mod tests` block and reached the code through items that were
//! already public, so the move changes the path and nothing else: the assertions are verbatim and
//! no item was widened to bring them out. `SPEC-MOK-004` rule 12 is the obligation and the
//! per-test comparison under `WO-MOK-006` is the evidence.

use mokiterions::simulation::FoodClass;
use mokiterions_tui::spatial::*;

#[test]
fn the_whole_world_needs_both_axes_and_never_width_alone() {
    // The reference viewport's canvas.
    let whole = Viewport::resolve(Zoom::Overview, (67, 32), (0, 0));
    assert!(whole.is_whole_world());
    assert_eq!((whole.width, whole.height), (128, 128));

    // 98 cells address 196 world columns, more than exist; 24 cells address 96 rows.
    let region = Viewport::resolve(Zoom::Overview, (98, 24), (0, 0));
    assert!(!region.is_whole_world());
    assert_eq!((region.width, region.height), (128, 96));
    assert_eq!((region.last_x(), region.last_y()), (127, 95));

    // The floor canvas addresses a 64 x 64 region.
    let floor = Viewport::resolve(Zoom::Overview, (32, 16), (0, 0));
    assert_eq!((floor.width, floor.height), (64, 64));

    // 47 cells address 94 world columns, so columns 0 to 93.
    let narrow = Viewport::resolve(Zoom::Overview, (47, 32), (0, 0));
    assert_eq!((narrow.width, narrow.height), (94, 128));
    assert_eq!(narrow.last_x(), 93);
    assert!(!narrow.is_whole_world());

    // The exact thresholds.
    assert!(Viewport::resolve(Zoom::Overview, (64, 32), (0, 0)).is_whole_world());
    assert!(!Viewport::resolve(Zoom::Overview, (63, 32), (0, 0)).is_whole_world());
    assert!(!Viewport::resolve(Zoom::Overview, (64, 31), (0, 0)).is_whole_world());
}

#[test]
fn territory_a_is_above_territory_b() {
    let viewport = Viewport::resolve(Zoom::Overview, (67, 32), (0, 0));

    // World row 0 is the topmost dot row, which is the largest canvas `y`.
    let (_, top) = viewport.dot_of(0, 0).unwrap();
    let (_, bottom) = viewport.dot_of(0, 127).unwrap();
    let (_, boundary_a) = viewport.dot_of(0, 63).unwrap();
    let (_, boundary_b) = viewport.dot_of(0, 64).unwrap();

    assert_eq!(top, 127.0);
    assert_eq!(bottom, 0.0);
    assert!(top > bottom, "world y must increase downward on screen");
    assert!(boundary_a > boundary_b, "territory A must sit above B");

    // The counterexample: plotting world `y` directly would invert the world.
    assert_ne!(viewport.dot_of(0, 0).unwrap().1, 0.0);
}

#[test]
fn the_overview_dot_grid_is_one_dot_per_world_cell() {
    let viewport = Viewport::resolve(Zoom::Overview, (67, 32), (0, 0));
    let (x_bounds, y_bounds) = viewport.bounds();
    assert_eq!(x_bounds, [0.0, 133.0]);
    assert_eq!(y_bounds, [0.0, 127.0]);

    // Distinct world cells never collide on a dot.
    let mut seen = std::collections::HashSet::new();
    for y in 0..128u16 {
        for x in 0..128u16 {
            let dot = viewport.dot_of(x, y).unwrap();
            assert!(seen.insert((dot.0 as u32, dot.1 as u32)), "{x}:{y}");
        }
    }
    assert_eq!(seen.len(), 128 * 128);
}

#[test]
fn a_character_cell_covers_two_by_four_world_cells_in_overview_and_one_in_detail() {
    let overview = Viewport::resolve(Zoom::Overview, (67, 32), (0, 0));
    assert_eq!(overview.cell_of(Zoom::Overview, 0, 0), Some((0, 0)));
    assert_eq!(overview.cell_of(Zoom::Overview, 1, 3), Some((0, 0)));
    assert_eq!(overview.cell_of(Zoom::Overview, 2, 4), Some((1, 1)));
    assert_eq!(overview.cell_of(Zoom::Overview, 127, 127), Some((63, 31)));

    let detail = Viewport::resolve(Zoom::Detail, (40, 20), (10, 20));
    assert_eq!(detail.cell_of(Zoom::Detail, 10, 20), Some((0, 0)));
    assert_eq!(detail.cell_of(Zoom::Detail, 49, 39), Some((39, 19)));
    assert_eq!(detail.cell_of(Zoom::Detail, 50, 39), None);
    assert_eq!(detail.cell_of(Zoom::Detail, 9, 20), None);
}

#[test]
fn the_camera_is_clamped_so_the_region_never_leaves_the_world() {
    let clamped = Viewport::resolve(Zoom::Detail, (40, 20), (200, 200));
    assert_eq!((clamped.origin_x, clamped.origin_y), (88, 108));
    assert_eq!((clamped.last_x(), clamped.last_y()), (127, 127));
    assert_eq!(clamped.camera_limit(), (88, 108));

    // A region as large as the world pins the camera at the origin.
    let whole = Viewport::resolve(Zoom::Overview, (67, 32), (100, 100));
    assert_eq!((whole.origin_x, whole.origin_y), (0, 0));
    assert_eq!(whole.camera_limit(), (0, 0));
}

#[test]
fn the_territory_rule_is_present_exactly_when_the_boundary_is_visible() {
    assert!(Viewport::resolve(Zoom::Overview, (67, 32), (0, 0)).shows_territory_boundary());
    assert!(Viewport::resolve(Zoom::Detail, (40, 20), (0, 60)).shows_territory_boundary());
    assert!(!Viewport::resolve(Zoom::Detail, (40, 20), (0, 64)).shows_territory_boundary());
    assert!(!Viewport::resolve(Zoom::Detail, (40, 20), (0, 0)).shows_territory_boundary());
    assert!(Viewport::resolve(Zoom::Detail, (40, 44), (0, 40)).shows_territory_boundary());
}

#[test]
fn glyphs_are_the_assigned_ones() {
    let expected = [
        ("M01", '1'),
        ("M02", '2'),
        ("M09", '9'),
        ("M10", 'A'),
        ("M11", 'B'),
        ("M12", 'C'),
    ];
    for (id, glyph) in expected {
        assert_eq!(agent_glyph(id), glyph, "{id}");
    }
    assert_eq!(agent_glyph("unnamed"), 'D');
    assert_eq!(agent_glyph(""), '?');

    assert_eq!(resource_glyph(FoodClass::Low), '○');
    assert_eq!(resource_glyph(FoodClass::Medium), '◎');
    assert_eq!(resource_glyph(FoodClass::High), '●');
}
