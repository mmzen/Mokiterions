//! Layout, pane thresholds and the viewport floor, as `SPEC-MOK-003` rule 5 fixes them.
//!
//! Layout is a pure function of viewport width and height. It reads no tick, no run state,
//! no entropy and no clock, so the same dimensions always produce the same layout. The
//! arithmetic is performed here rather than delegated to a constraint solver so that the
//! derived canvas figures rule 5 declares are checkable directly.
//!
//! Each optional pane is decided by one threshold on the one axis that constrains it, so the
//! combination a viewport gets is whatever those thresholds independently decide. There is no
//! ordered table of named configurations and therefore no viewport that matches none of them,
//! which is what rule 5's monotonicity obligation requires: enlarging a terminal never removes
//! a pane.

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

/// Rule 5's pane thresholds. The roster is a vertical list in a fixed-width column and the
/// inspector needs width for the roster and a usable view beside it, so both read the width;
/// the log is a fixed-height band of rows, so it reads the height.
const ROSTER_MIN_WIDTH: u16 = 100;
const INSPECTOR_MIN_WIDTH: u16 = 140;
const LOG_MIN_HEIGHT: u16 = 38;
const FULL_LOG_MIN_WIDTH: u16 = 140;
const FULL_LOG_MIN_HEIGHT: u16 = 48;

/// The pane width at or above which a roster entry occupies two lines (rule 4).
pub const ROSTER_TWO_LINE_WIDTH: u16 = 47;

/// A pane the current viewport may exclude from the body. Every excluded pane is reachable as
/// a full-body overlay by its bound key.
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
    /// The panes this viewport excludes, in a stable order, for the header's announcement.
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

/// The rows the log occupies, `0` when the height excludes it.
fn log_rows(width: u16, height: u16) -> u16 {
    if height < LOG_MIN_HEIGHT {
        0
    } else if width >= FULL_LOG_MIN_WIDTH && height >= FULL_LOG_MIN_HEIGHT {
        FULL_LOG_HEIGHT
    } else {
        COMPACT_LOG_HEIGHT
    }
}

/// Resolves every region for a viewport above the floor.
pub fn resolve(area: Rect) -> Panes {
    let log_height = log_rows(area.width, area.height);

    let header = Rect {
        height: HEADER_HEIGHT.min(area.height),
        ..area
    };
    let footer = Rect {
        y: area.y + area.height - FOOTER_HEIGHT,
        height: FOOTER_HEIGHT,
        ..area
    };
    // The body absorbs whatever the fixed rows leave. The floor guarantees the remainder is at
    // least three rows: the log costs six and appears only from 38 rows.
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

    // Widths first, so the columns are seen to tile the body exactly. A pane the width excludes
    // costs nothing, which is why the view's width is the subtraction of both.
    let roster_width = if area.width >= ROSTER_MIN_WIDTH {
        ROSTER_WIDTH
    } else {
        0
    };
    let inspector_width = if area.width >= INSPECTOR_MIN_WIDTH {
        INSPECTOR_WIDTH
    } else {
        0
    };
    // The thresholds themselves keep this positive: the roster's 47 columns arrive at 100, and
    // the inspector's 44 more arrive at 140, so the view keeps at least 49 columns wherever both
    // are present and the whole width wherever neither is.
    let view_width = body.width.saturating_sub(roster_width + inspector_width);

    let roster = (roster_width > 0).then_some(Rect {
        width: roster_width,
        ..body
    });
    let view = Rect {
        x: body.x + roster_width,
        width: view_width,
        ..body
    };
    let inspector = (inspector_width > 0).then_some(Rect {
        x: view.x + view_width,
        width: inspector_width,
        ..body
    });

    Panes {
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
