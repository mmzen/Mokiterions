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

#[cfg(test)]
mod tests {
    use super::*;

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
}
