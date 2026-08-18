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
