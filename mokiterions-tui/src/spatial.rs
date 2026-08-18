//! The mapping from world cells to canvas cells and braille dots, as `SPEC-MOK-003` rule 2
//! fixes it.
//!
//! Nothing here reads engine state. It is arithmetic over a viewport size and a camera, so
//! the orientation obligation and the fidelity thresholds are checkable without a terminal
//! and without a simulation.

use mokiterions::simulation::FoodClass;

/// The world's extent in each axis. `SPEC-MOK-001` fixes it; it is consumed here, never
/// restated as authority. `crate::state` asserts it against the engine's own
/// `world_initialized` event, so a divergence fails a test rather than mis-drawing a frame.
pub const WORLD_SIZE: u16 = 128;

/// The last world row of territory A. The territory rule is drawn between this row and the
/// next.
pub const TERRITORY_A_LAST_ROW: u16 = 63;

/// One braille character cell carries a 2-wide by 4-tall grid of addressable dots.
pub const DOTS_PER_CELL_X: u16 = 2;
const DOTS_PER_CELL_Y: u16 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zoom {
    Overview,
    Detail,
}

impl Zoom {
    pub fn label(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Detail => "detail",
        }
    }

    pub fn toggled(self) -> Self {
        match self {
            Self::Overview => Self::Detail,
            Self::Detail => Self::Overview,
        }
    }
}

/// The world region a canvas presents, resolved from the zoom, the canvas size and the
/// camera. The camera is clamped here, so the visible region never leaves the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    pub origin_x: u16,
    pub origin_y: u16,
    pub width: u16,
    pub height: u16,
    /// The dots the canvas addresses in each axis. In detail zoom this is the canvas size.
    /// It can exceed the world, in which case the surplus is empty and the region is whole.
    pub span_x: u16,
    pub span_y: u16,
}

impl Viewport {
    pub fn resolve(zoom: Zoom, canvas: (u16, u16), camera: (u16, u16)) -> Self {
        let (span_x, span_y) = match zoom {
            Zoom::Overview => (
                canvas.0.saturating_mul(DOTS_PER_CELL_X),
                canvas.1.saturating_mul(DOTS_PER_CELL_Y),
            ),
            Zoom::Detail => canvas,
        };
        let width = span_x.clamp(1, WORLD_SIZE);
        let height = span_y.clamp(1, WORLD_SIZE);
        Self {
            origin_x: camera.0.min(WORLD_SIZE - width),
            origin_y: camera.1.min(WORLD_SIZE - height),
            width,
            height,
            span_x: span_x.max(1),
            span_y: span_y.max(1),
        }
    }

    /// Whether the whole world is presented, which requires both axes and not width alone.
    pub fn is_whole_world(self) -> bool {
        self.width >= WORLD_SIZE && self.height >= WORLD_SIZE
    }

    pub fn last_x(self) -> u16 {
        self.origin_x + self.width - 1
    }

    pub fn last_y(self) -> u16 {
        self.origin_y + self.height - 1
    }

    pub fn contains(self, x: u16, y: u16) -> bool {
        (self.origin_x..=self.last_x()).contains(&x) && (self.origin_y..=self.last_y()).contains(&y)
    }

    /// The furthest camera position that keeps the region inside the world.
    pub fn camera_limit(self) -> (u16, u16) {
        (WORLD_SIZE - self.width, WORLD_SIZE - self.height)
    }

    /// The character cell holding a world cell, as an offset inside the canvas.
    ///
    /// In overview zoom one cell covers a 2 x 4 block of world cells, which is why an
    /// overview Mokiterion glyph locates its subject to within that block.
    pub fn cell_of(self, zoom: Zoom, x: u16, y: u16) -> Option<(u16, u16)> {
        if !self.contains(x, y) {
            return None;
        }
        let (dx, dy) = (x - self.origin_x, y - self.origin_y);
        Some(match zoom {
            Zoom::Overview => (dx / DOTS_PER_CELL_X, dy / DOTS_PER_CELL_Y),
            Zoom::Detail => (dx, dy),
        })
    }

    /// The canvas point of a world cell in overview zoom.
    ///
    /// The canvas coordinate system is bottom-up, so world `y` maps to `span_y − 1 − dy`.
    /// With `x_bounds` `[0, span_x − 1]` and `y_bounds` `[0, span_y − 1]` this is an exact
    /// one-dot-per-world-cell grid whose first world row is the topmost dot row, which is
    /// rule 2.1's required orientation.
    pub fn dot_of(self, x: u16, y: u16) -> Option<(f64, f64)> {
        if !self.contains(x, y) {
            return None;
        }
        Some((
            f64::from(x - self.origin_x),
            f64::from(self.span_y - 1 - (y - self.origin_y)),
        ))
    }

    /// The canvas bounds for overview zoom.
    pub fn bounds(self) -> ([f64; 2], [f64; 2]) {
        (
            [0.0, f64::from(self.span_x - 1)],
            [0.0, f64::from(self.span_y - 1)],
        )
    }

    /// Whether the territory rule lies in the visible region.
    pub fn shows_territory_boundary(self) -> bool {
        self.origin_y <= TERRITORY_A_LAST_ROW && TERRITORY_A_LAST_ROW < self.origin_y + self.height
    }
}

/// The glyph for a Mokiterion, derived mechanically from the engine's identifier.
///
/// `SPEC-MOK-003` rule 2 assigns `1`–`9` to `M01`–`M09` and `A`, `B`, `C` to `M10`–`M12`.
/// The identifier's numeric suffix written as an uppercase base-13 digit is exactly that
/// assignment. An identifier without a numeric suffix falls back to its last character
/// uppercased, which is the layer table's general rule.
pub fn agent_glyph(id: &str) -> char {
    let digits: String = id
        .chars()
        .filter(|character| character.is_ascii_digit())
        .collect();
    if let Some(glyph) = digits
        .parse::<u32>()
        .ok()
        .and_then(|index| char::from_digit(index, 13))
    {
        return glyph.to_ascii_uppercase();
    }
    id.chars()
        .next_back()
        .map(|character| character.to_ascii_uppercase())
        .unwrap_or('?')
}

/// The detail-zoom glyph for a resource class.
pub fn resource_glyph(class: FoodClass) -> char {
    match class {
        FoodClass::Low => '\u{25cb}',    // ○
        FoodClass::Medium => '\u{25ce}', // ◎
        FoodClass::High => '\u{25cf}',   // ●
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
