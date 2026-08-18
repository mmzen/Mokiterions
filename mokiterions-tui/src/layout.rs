//! Layout, tiers and the viewport floor, as `SPEC-MOK-003` rule 5 fixes them.
//!
//! Layout is a pure function of viewport width and height. It reads no tick, no run state,
//! no entropy and no clock, so the same dimensions always produce the same layout. The
//! arithmetic is performed here rather than delegated to a constraint solver so that the
//! derived canvas figures rule 5 declares are checkable directly.

use ratatui::layout::Rect;

/// The floor `SPEC-MOK-003` rule 5 fixes: a 32 x 16 canvas, its border, and four rows of
/// header and footer.
pub const MIN_WIDTH: u16 = 34;
pub const MIN_HEIGHT: u16 = 22;

const HEADER_HEIGHT: u16 = 3;
const FOOTER_HEIGHT: u16 = 1;
const ROSTER_WIDTH: u16 = 47;
const INSPECTOR_WIDTH: u16 = 44;
const FULL_LOG_HEIGHT: u16 = 10;
const COMPACT_LOG_HEIGHT: u16 = 6;

/// The pane width at or above which a roster entry occupies two lines (rule 4).
pub const ROSTER_TWO_LINE_WIDTH: u16 = 47;

/// The layout tier. The first matching condition applies, in the order of rule 5's table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    A,
    B,
    C,
    D,
}

impl Tier {
    pub fn label(self) -> &'static str {
        match self {
            Self::A => "A full",
            Self::B => "B compact-log",
            Self::C => "C narrow",
            Self::D => "D minimal",
        }
    }
}

/// A pane that a tier may exclude from the body. Every excluded pane is reachable as a
/// full-body overlay by its bound key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Roster,
    Inspector,
    Log,
}

impl Pane {
    pub fn label(self) -> &'static str {
        match self {
            Self::Roster => "roster",
            Self::Inspector => "inspector",
            Self::Log => "log",
        }
    }
}

/// The resolved regions of one frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Panes {
    pub tier: Tier,
    pub header: Rect,
    pub roster: Option<Rect>,
    pub view: Rect,
    pub inspector: Option<Rect>,
    pub log: Option<Rect>,
    pub footer: Rect,
    /// Everything between the header and the footer: the region a full-body overlay covers.
    pub overlay: Rect,
}

impl Panes {
    /// The panes this tier excludes, in a stable order, for the header's announcement.
    pub fn overlay_only(&self) -> Vec<Pane> {
        let mut panes = Vec::new();
        if self.roster.is_none() {
            panes.push(Pane::Roster);
        }
        if self.log.is_none() {
            panes.push(Pane::Log);
        }
        if self.inspector.is_none() {
            panes.push(Pane::Inspector);
        }
        panes
    }
}

/// Whether the viewport is below the floor. At start-up this is a refusal with exit `2`; at
/// run time it suspends drawing and leaves the run alone.
pub fn below_floor(width: u16, height: u16) -> bool {
    width < MIN_WIDTH || height < MIN_HEIGHT
}

/// Rule 5's tier table. The first matching row applies.
pub fn tier_for(width: u16, height: u16) -> Tier {
    if width >= 140 && height >= 48 {
        Tier::A
    } else if width >= 140 && (44..48).contains(&height) {
        Tier::B
    } else if (100..140).contains(&width) && height >= 38 {
        Tier::C
    } else {
        Tier::D
    }
}

/// Resolves every region for a viewport above the floor.
pub fn resolve(area: Rect) -> Panes {
    let tier = tier_for(area.width, area.height);
    let log_height = match tier {
        Tier::A => FULL_LOG_HEIGHT,
        Tier::B | Tier::C => COMPACT_LOG_HEIGHT,
        Tier::D => 0,
    };

    let header = Rect {
        height: HEADER_HEIGHT.min(area.height),
        ..area
    };
    let footer = Rect {
        y: area.y + area.height - FOOTER_HEIGHT,
        height: FOOTER_HEIGHT,
        ..area
    };
    // The body carries rule 5's `Min` constraint, so it absorbs whatever the fixed rows
    // leave. Every tier condition guarantees that remainder meets the tier's minimum.
    let body_height = area
        .height
        .saturating_sub(HEADER_HEIGHT + FOOTER_HEIGHT + log_height);
    let body = Rect {
        y: area.y + HEADER_HEIGHT,
        height: body_height,
        ..area
    };
    let log = (log_height > 0).then(|| Rect {
        y: body.y + body.height,
        height: log_height,
        ..area
    });
    let overlay = Rect {
        y: body.y,
        height: body.height + log_height,
        ..area
    };

    let (roster, view, inspector) = match tier {
        Tier::A | Tier::B => {
            let view_width = body.width.saturating_sub(ROSTER_WIDTH + INSPECTOR_WIDTH);
            (
                Some(Rect {
                    width: ROSTER_WIDTH,
                    ..body
                }),
                Rect {
                    x: body.x + ROSTER_WIDTH,
                    width: view_width,
                    ..body
                },
                Some(Rect {
                    x: body.x + ROSTER_WIDTH + view_width,
                    width: INSPECTOR_WIDTH,
                    ..body
                }),
            )
        }
        Tier::C => {
            let view_width = body.width.saturating_sub(ROSTER_WIDTH);
            (
                Some(Rect {
                    width: ROSTER_WIDTH,
                    ..body
                }),
                Rect {
                    x: body.x + ROSTER_WIDTH,
                    width: view_width,
                    ..body
                },
                None,
            )
        }
        Tier::D => (None, body, None),
    };

    Panes {
        tier,
        header,
        roster,
        view,
        inspector,
        log,
        footer,
        overlay,
    }
}

/// The canvas of a bordered pane: its interior, which is the pane less the two cells its
/// border occupies in each axis.
pub fn canvas_cells(pane: Rect) -> (u16, u16) {
    (pane.width.saturating_sub(2), pane.height.saturating_sub(2))
}
