//! Frame composition.
//!
//! Every function here reads presentation state and an engine snapshot and produces cells. None
//! of them advances the simulation, reads a clock, reads an environment variable at run time, or
//! reads a repository file, so a frame is a pure function of the state it is given and the
//! viewport it is given (`SPEC-MOK-003` rules 5 and 12).
//!
//! Colour is redundant everywhere: rule 2.5's identity distinctions are carried by glyph,
//! underline and position, and every textual indication rule 3 and rule 10 require is a word.

use std::collections::BTreeMap;

use mokiterions::simulation::{
    Action, AgentSnapshot, DecisionOutcome, FoodClass, Territory, TerritorySnapshot,
};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::Marker;
use ratatui::text::{Line, Span};
use ratatui::widgets::canvas::{Canvas, Points};
use ratatui::widgets::{Block, Clear, Paragraph, Wrap};

use crate::authority;
use crate::layout::{self, Pane, Panes};
use crate::spatial::{self, Viewport, Zoom};
use crate::state::{Observer, Overlay, Progression};

/// The candidate commit, supplied to the build as a compile-time value.
///
/// `SPEC-MOK-003` rule 8.2 authorizes exactly this: the field is present when the build supplied
/// it and absent otherwise. The observer never reads a repository file, never invokes a
/// version-control command, and never guesses.
pub const COMMIT: Option<&str> = option_env!("MOKITERIONS_COMMIT");

/// How an absent value renders (rule 4.4). It is distinguishable from `0`, which renders as `0`.
const ABSENT: &str = "\u{2014}"; // —

/// The territory rule of rule 2.2 in detail zoom.
const BOUNDARY_GLYPH: char = '\u{2500}'; // ─

const TERRITORY_A_COLOUR: Color = Color::Cyan;
const TERRITORY_B_COLOUR: Color = Color::Magenta;
const RESOURCE_COLOUR: Color = Color::Gray;
const LOW_COLOUR: Color = Color::Blue;
const MEDIUM_COLOUR: Color = Color::Yellow;
const HIGH_COLOUR: Color = Color::Green;
const BOUNDARY_COLOUR: Color = Color::DarkGray;

/// Rule 4.7's three survival bands. `Indexed(208)` is xterm's dark orange rather than
/// `Color::Yellow`, which `MEDIUM_COLOUR` already spends on a medium-class resource: two unrelated
/// meanings sharing one colour on one screen is avoidable here.
const BAND_HIGH_COLOUR: Color = Color::Green;
const BAND_MIDDLE_COLOUR: Color = Color::Indexed(208);
const BAND_LOW_COLOUR: Color = Color::Red;

/// The two boundaries rule 4.7 fixes: green at `80..=100`, orange at `40..=79`, red at `0..=39`.
const BAND_HIGH_FLOOR: u8 = 80;
const BAND_MIDDLE_FLOOR: u8 = 40;

/// Columns a roster bar row spends on labels, values and separators, so the bars themselves get
/// `(interior − this) / 4`. Derived from rule 4's form as amended on 2026-08-19: five leading
/// columns, then four groups of `label`, space, bar, space and a three-column value, separated
/// by two columns — `5 + 4 × 6 + 3 × 2`. It was `27` for three groups until `WO-MOK-010` filled
/// rule 4.5's reserved slot with `fear`.
const BAR_ROW_OVERHEAD: usize = 35;

/// The bar length rule 4's mockup uses, and the maximum this implementation draws.
const FULL_BAR: usize = 20;

/// Composes one frame.
///
/// Below the floor nothing is presented and the run is left alone (rule 5's resize behavior).
pub fn draw(frame: &mut Frame, observer: &mut Observer) {
    let area = frame.area();
    if layout::below_floor(area.width, area.height) {
        frame.render_widget(Clear, area);
        return;
    }

    let panes = layout::resolve(area);
    let canvas = layout::canvas_cells(panes.view);
    // Following depends on the canvas size, so it is applied here rather than on key press.
    observer.apply_follow(canvas);
    // Paging the log is relative to the overlay, which is where rule 9.1 makes older records
    // reachable.
    observer.record_geometry(canvas, usize::from(panes.overlay.height.saturating_sub(2)));

    draw_header(frame, panes.header, observer, &panes);
    draw_view(frame, panes.view, observer);
    if let Some(pane) = panes.roster {
        draw_roster(frame, pane, observer);
    }
    if let Some(pane) = panes.inspector {
        draw_inspector(frame, pane, observer);
    }
    if let Some(pane) = panes.log {
        draw_log(frame, pane, observer);
    }
    draw_footer(frame, panes.footer, observer);
    draw_overlay(frame, panes.overlay, observer);
}

// ---- header ------------------------------------------------------------------------------

fn draw_header(frame: &mut Frame, area: Rect, observer: &Observer, panes: &Panes) {
    let snapshot = observer.snapshot();
    let lines = vec![
        status_line(observer, panes, area.width),
        territory_line(&snapshot.territories[0], area.width),
        territory_line(&snapshot.territories[1], area.width),
    ];
    frame.render_widget(Paragraph::new(lines), area);
}

/// Run state, the announcement of rule 5, and any reported failure.
///
/// The announcement is reserved before the optional segments, because it is an obligation and
/// they are not: a narrow viewport drops speed or filter before it drops the statement that a
/// pane is only reachable as an overlay.
fn status_line(observer: &Observer, panes: &Panes, width: u16) -> Line<'static> {
    let width = usize::from(width);
    let announcement = announcement_text(panes, width);
    let reserved = announcement.as_ref().map_or(0, |text| count(text) + 2);

    let progression = observer.progression();
    let head = Span::styled(
        progression.label(),
        Style::new()
            .fg(match progression {
                Progression::Running => Color::Green,
                Progression::Held => Color::Yellow,
            })
            .add_modifier(Modifier::BOLD),
    );
    let mut used = count(progression.label());
    let mut left = vec![head];

    // A failure is reported in the header (rule 9.6 and the error table), ahead of the run
    // state's optional detail, and clipped rather than allowed to displace the announcement.
    if let Some(notice) = observer.notice() {
        let room = width.saturating_sub(used + reserved + 2);
        if room >= 6 {
            let text = clip(notice, room);
            used += count(&text) + 2;
            left.push(Span::styled(
                format!("  {text}"),
                Style::new().fg(Color::Yellow),
            ));
        }
    }

    let mut optional = Vec::new();
    if observer.is_finished() {
        let reason = observer
            .termination_reason()
            .map_or_else(|| "run over".to_string(), |reason| reason.to_string());
        optional.push(format!("finished {reason}"));
    }
    optional.push(format!("x{}", observer.speed()));
    optional.push(observer.zoom().label().to_string());
    if let Some(selection) = observer.selection() {
        optional.push(format!("sel {selection}"));
    }
    if observer.follow() {
        optional.push("follow".to_string());
    }
    optional.push(format!("filter {}", observer.filter().label()));

    for segment in optional {
        let cost = count(&segment) + 2;
        if used + cost + reserved <= width {
            used += cost;
            left.push(Span::raw(format!("  {segment}")));
        }
    }

    let right = announcement.map(Span::raw).into_iter().collect();
    justified(left, right, width)
}

/// The panes this viewport offers only as overlays, named with the keys that open them.
///
/// This is the whole account of a missing pane. Rule 5 excludes a pane on one threshold in one
/// axis, so naming the panes is more use to an operator than naming the configuration would be:
/// it says which key restores what, and it does not require knowing a table to read.
fn announcement_text(panes: &Panes, width: usize) -> Option<String> {
    let excluded = panes.overlay_only();
    if excluded.is_empty() {
        return None;
    }
    let key = |pane: Pane| match pane {
        Pane::Roster => "r",
        Pane::Log => "L",
        Pane::Inspector => "i",
    };
    let long: Vec<String> = excluded
        .iter()
        .map(|pane| format!("{} {}", pane.label(), key(*pane)))
        .collect();
    let long = format!("overlays: {}", long.join("  "));
    let short: Vec<&str> = excluded.iter().map(|pane| key(*pane)).collect();
    Some(fit(long, format!("ovl {}", short.join(" ")), width))
}

/// Rule 3's headline for one territory.
fn territory_line(territory: &TerritorySnapshot, width: u16) -> Line<'static> {
    // Rule 3.1 and 3.2: both states are irreversible consequences of `SPEC-MOK-001` rule 15, so
    // both are words, not counts, and both are stated as well as coloured (rule 3.3).
    let state = if territory.permanently_depleted {
        Some(("permanently depleted", "depleted", Color::Red))
    } else if territory.standing == 1 {
        Some(("one from sterile", "last one", Color::Yellow))
    } else {
        None
    };

    let long = format!(
        "{}  standing {}/{}  low {}  medium {}  high {}",
        territory.id,
        territory.standing,
        territory.capacity,
        territory.low,
        territory.medium,
        territory.high
    );
    let short = format!(
        "{} {}/{} {}/{}/{}",
        territory.id,
        territory.standing,
        territory.capacity,
        territory.low,
        territory.medium,
        territory.high
    );

    let width = usize::from(width);
    let long_fits = count(&long) + state.map_or(0, |(word, _, _)| count(word) + 2) <= width;
    let (body, note) = if long_fits {
        (long, state.map(|(word, _, colour)| (word, colour)))
    } else {
        (short, state.map(|(_, word, colour)| (word, colour)))
    };

    let mut spans = vec![Span::styled(body, territory_style(territory.id))];
    if let Some((word, colour)) = note {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(
            word,
            Style::new().fg(colour).add_modifier(Modifier::BOLD),
        ));
    }
    Line::from(spans)
}

// ---- spatial view ------------------------------------------------------------------------

fn draw_view(frame: &mut Frame, area: Rect, observer: &Observer) {
    let viewport = observer.viewport();
    let zoom = observer.zoom();
    let block = Block::bordered().title_top(view_title(viewport, zoom, area.width));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    if inner.width == 0 || inner.height == 0 {
        return;
    }

    match zoom {
        Zoom::Overview => draw_overview(frame, inner, observer, viewport),
        Zoom::Detail => draw_detail(frame, inner, observer, viewport),
    }
    // Mokiterions are the top layer in both zooms (rule 2's layer table and detail precedence).
    draw_mokiterions(frame, inner, observer, viewport, zoom);
}

/// Rule 2.3's region indication. The world range is always stated, which is a superset of the
/// obligation, and the whole-world case is named so a region is never read as the world.
fn view_title(viewport: Viewport, zoom: Zoom, pane_width: u16) -> String {
    let extent = if viewport.is_whole_world() {
        "whole world"
    } else {
        "region"
    };
    let range = format!(
        "x{}-{} y{}-{}",
        viewport.origin_x,
        viewport.last_x(),
        viewport.origin_y,
        viewport.last_y()
    );
    fit(
        format!(" view  {}  {range}  {extent} ", zoom.label()),
        format!(" {range} {extent} "),
        usize::from(pane_width.saturating_sub(2)),
    )
}

fn draw_overview(frame: &mut Frame, inner: Rect, observer: &Observer, viewport: Viewport) {
    let mut resources = Vec::new();
    for resource in &observer.snapshot().resources {
        if let Some(dot) = viewport.dot_of(resource.position.x.into(), resource.position.y.into()) {
            resources.push(dot);
        }
    }

    let mut boundary = Vec::new();
    if viewport.shows_territory_boundary() {
        for x in viewport.origin_x..=viewport.last_x() {
            if let Some(dot) = viewport.dot_of(x, spatial::TERRITORY_A_LAST_ROW) {
                boundary.push(dot);
            }
        }
    }

    // The bounds are the dot grid the canvas already addresses, so one world cell is one dot
    // with no scaling and no rounding (rule 2's overview zoom).
    let (x_bounds, y_bounds) = viewport.bounds();
    let canvas = Canvas::default()
        .marker(Marker::Braille)
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .paint(|context| {
            // Rule 2's layer order: resources, then the territory rule over them. Overview
            // encodes no resource class: one dot cannot carry three states.
            context.draw(&Points {
                coords: &resources,
                color: RESOURCE_COLOUR,
            });
            context.layer();
            context.draw(&Points {
                coords: &boundary,
                color: BOUNDARY_COLOUR,
            });
        });
    frame.render_widget(canvas, inner);
}

fn draw_detail(frame: &mut Frame, inner: Rect, observer: &Observer, viewport: Viewport) {
    let resources = &observer.snapshot().resources;
    let buffer = frame.buffer_mut();

    if viewport.shows_territory_boundary() {
        let row = spatial::TERRITORY_A_LAST_ROW - viewport.origin_y;
        if row < inner.height {
            for column in 0..inner.width {
                if let Some(cell) = buffer.cell_mut((inner.x + column, inner.y + row)) {
                    cell.set_char(BOUNDARY_GLYPH);
                    cell.set_style(Style::new().fg(BOUNDARY_COLOUR));
                }
            }
        }
    }

    for resource in resources {
        let Some((x, y)) = viewport.cell_of(
            Zoom::Detail,
            resource.position.x.into(),
            resource.position.y.into(),
        ) else {
            continue;
        };
        if x >= inner.width || y >= inner.height {
            continue;
        }
        if let Some(cell) = buffer.cell_mut((inner.x + x, inner.y + y)) {
            cell.set_char(spatial::resource_glyph(resource.class));
            cell.set_style(Style::new().fg(class_colour(resource.class)));
        }
    }
}

/// Rule 2.4: the lowest identifier in a shared cell is drawn, and the cell is underlined.
fn draw_mokiterions(
    frame: &mut Frame,
    inner: Rect,
    observer: &Observer,
    viewport: Viewport,
    zoom: Zoom,
) {
    let selection = observer.selection();
    // The engine reports living Mokiterions in ascending identifier order, so the first
    // occupant recorded for a cell is the one rule 2.4 draws.
    let mut occupants: BTreeMap<(u16, u16), (usize, &AgentSnapshot)> = BTreeMap::new();
    for agent in &observer.snapshot().agents {
        let Some(cell) = viewport.cell_of(zoom, agent.position.x.into(), agent.position.y.into())
        else {
            continue;
        };
        if cell.0 >= inner.width || cell.1 >= inner.height {
            continue;
        }
        occupants
            .entry(cell)
            .and_modify(|(count, _)| *count += 1)
            .or_insert((1, agent));
    }

    let buffer = frame.buffer_mut();
    for ((x, y), (occupancy, agent)) in occupants {
        let Some(cell) = buffer.cell_mut((inner.x + x, inner.y + y)) else {
            continue;
        };
        // Rule 2 as amended on 2026-08-19: the glyph is the engine's reported name's initial.
        cell.set_char(spatial::agent_glyph(
            observer.name_of(&agent.id).unwrap_or_default(),
        ));
        let mut style = Style::new()
            .fg(territory_colour(agent.territory))
            .add_modifier(Modifier::BOLD);
        if occupancy > 1 {
            style = style.add_modifier(Modifier::UNDERLINED);
        }
        // Only the drawn Mokiterion can be marked as selected; marking a cell whose selected
        // occupant is not the one drawn would misattribute the glyph.
        if selection == Some(agent.id.as_str()) {
            style = style.add_modifier(Modifier::REVERSED);
        }
        cell.set_style(style);
    }
}

// ---- roster ------------------------------------------------------------------------------

fn draw_roster(frame: &mut Frame, area: Rect, observer: &Observer) {
    let snapshot = observer.snapshot();
    let interior_width = area.width.saturating_sub(2);
    let interior_height = usize::from(area.height.saturating_sub(2));
    let two_line = area.width >= layout::ROSTER_TWO_LINE_WIDTH;
    let rows_per_entry = if two_line { 2 } else { 1 };
    let capacity = interior_height / rows_per_entry;

    let agents = &snapshot.agents;
    let selected = observer
        .selection()
        .and_then(|id| agents.iter().position(|agent| agent.id == id));
    // The window keeps the selection visible without reordering: rule 4 fixes ascending
    // identifier order, which is acting order.
    let start = match selected {
        Some(index) if index >= capacity => index + 1 - capacity,
        _ => 0,
    };
    let end = (start + capacity).min(agents.len());
    let hidden = agents.len() - (end - start);

    let bar = bar_width(interior_width);
    let mut lines = Vec::new();
    for agent in &agents[start..end] {
        // Rule 4.6: the selected entry is highlighted by reversed video, not by colour alone.
        let style = if observer.selection() == Some(agent.id.as_str()) {
            Style::new().add_modifier(Modifier::REVERSED)
        } else {
            Style::new()
        };
        // Rule 4.7: the entry's style is the line's, which every span patches rather than
        // replaces, so reversed video covers the whole entry and each gauge keeps its band
        // inside it. Selection stays marked by reversal and never by colour.
        for line in entry_lines(
            agent,
            observer.name_of(&agent.id).unwrap_or_default(),
            bar,
            two_line,
        ) {
            lines.push(line.style(style));
        }
    }

    // Rule 4.2, 4.3 and rule 5's announcement.
    let mut long = format!(
        " roster  living {}  deaths {} ",
        snapshot.living_count, snapshot.deaths
    );
    let mut short = format!(" roster {}/{} ", snapshot.living_count, snapshot.deaths);
    if hidden > 0 {
        long = format!(
            " roster  living {}  deaths {}  hidden {hidden} ",
            snapshot.living_count, snapshot.deaths
        );
        short = format!(
            " roster {}/{} h{hidden} ",
            snapshot.living_count, snapshot.deaths
        );
    }
    let title = fit(long, short, usize::from(interior_width));
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title_top(title)),
        area,
    );
}

/// An action as rule 4's action field and rule 10's decision record present it: the verb, and the
/// subject where the action has one.
///
/// The engine's `Display` renders a targeted verb bare, because `SPEC-MOK-001`'s `action_trace`
/// line carries the target in a field of its own. A roster field rendering `attack` alone would
/// leave an operator unable to read which Mokiterion struck against which was struck, which is the
/// pair rule 4's acting order exists to make comparable, so the subject is appended here. It is the
/// identifier and never the name: this field is a join key into the log pane and the export, and
/// rule 10 states that a target is presented as the engine reports it while the *name* belongs to
/// the pane's own selection.
///
/// The four core verbs are matched explicitly rather than by a wildcard, so a twelfth kind of action
/// would fail to compile here instead of silently rendering without its object.
fn action_text(action: &Action) -> String {
    match action {
        Action::Wait | Action::Sleep | Action::Eat { .. } | Action::Move { .. } => {
            action.to_string()
        }
        Action::Attack { target }
        | Action::Threaten { target }
        | Action::Fight { target }
        | Action::Retreat { target }
        | Action::Surrender { target }
        | Action::Approach { target }
        | Action::Avoid { target } => format!("{action} {target}"),
    }
}

/// Rule 4's entry form. Two lines at 47 columns or more, one line below.
///
/// `name` is what the engine reported for this Mokiterion, or empty where it reported none, in
/// which case the field is blank rather than filled with the identifier (`REQ-MOK-041`). It takes
/// six columns of line one only, so the bar row's 35-column overhead and `bar_width` are untouched.
fn entry_lines(
    agent: &AgentSnapshot,
    name: &str,
    bar: usize,
    two_line: bool,
) -> Vec<Line<'static>> {
    let applied = agent
        .applied_action
        .as_ref()
        .map_or_else(|| ABSENT.to_string(), action_text);
    // The engine's `Display` implementations write their text directly, so a column width has
    // to be applied to the rendered string rather than to the value.
    let territory = agent.territory.to_string();
    let position = agent.position.to_string();
    if !two_line {
        // Rule 4.7: the collapsed form has no bars and takes no band. It exists to keep the
        // numbers legible where the bar cells will not fit, and the numbers carry the level.
        // Rule 4.5 as amended makes that four numbers rather than three.
        return vec![Line::from(format!(
            "{name:<6}{:<5}{territory:<3}h{:>3} s{:>3} e{:>3} f{:>3}",
            agent.id, agent.health, agent.satiety, agent.energy, agent.fear
        ))];
    }
    vec![
        Line::from(format!(
            "{name:<6}{:<5}{territory:<3}{position:<14}{applied}",
            agent.id
        )),
        // Rule 4.5 reserved the fourth slot for Phase 2's `fear`, to render empty with no label,
        // no dash and no zero, because an inert `fear 0` would be a claim the engine cannot
        // support. `WO-MOK-010` made the value an engine attribute, so the slot is filled and the
        // gauge is no longer that claim.
        //
        // Rule 4.7: each gauge is its own span so that values can hold their own bands. The
        // five-column indent and the two-column separators are unstyled, which is what keeps a
        // band the property of one gauge rather than of the row. The fourth gauge is unbanded --
        // the three bands are a survival scale on which high is good, and `fear` inverts it.
        Line::from(vec![
            Span::raw("     "),
            gauge('h', agent.health, bar),
            Span::raw("  "),
            gauge('s', agent.satiety, bar),
            Span::raw("  "),
            gauge('e', agent.energy, bar),
            Span::raw("  "),
            unbanded_gauge('f', agent.fear, bar),
        ]),
    ]
}

/// The bar length that fits four bars in an interior of this width, capped at rule 4's twenty.
fn bar_width(interior_width: u16) -> usize {
    (usize::from(interior_width).saturating_sub(BAR_ROW_OVERHEAD) / 4).min(FULL_BAR)
}

/// One survival gauge: a proportional bar and its numeric value, in rule 4.7's band for that
/// value. `0` renders as `0` with an empty bar (rule 4.4) and takes the low band.
///
/// The band covers the whole gauge — label, bar cells and value — so one gauge reads as one state.
/// It changes no character: the text is what it was before rule 4.7 existed.
fn gauge(label: char, value: u8, width: usize) -> Span<'static> {
    Span::styled(
        gauge_text(label, value, width),
        Style::new().fg(band(value)),
    )
}

/// Rule 4.7's fourth gauge, which takes no band. `fear` is not on the scale the three bands read:
/// high health is the best state health has and high `fear` the worst state `fear` has, so banding
/// it green at `100` would say the opposite of what the same colour says on the other three. A
/// second scale running the other way was declined for putting two colour meanings on one row.
fn unbanded_gauge(label: char, value: u8, width: usize) -> Span<'static> {
    Span::raw(gauge_text(label, value, width))
}

/// The text of one gauge, which is the same for all four whether a band applies to it or not.
fn gauge_text(label: char, value: u8, width: usize) -> String {
    let filled = (usize::from(value) * width / 100).min(width);
    format!(
        "{label} {}{} {value:>3}",
        "\u{2588}".repeat(filled),
        "\u{2591}".repeat(width - filled)
    )
}

/// Rule 4.7's band for one survival value.
///
/// A band is a second presentation of the number the bar already shows. It reads one `u8` the
/// engine computed and retains nothing, so `REQ-MOK-020`'s constraint against any quantity the
/// engine does not produce holds literally. The boundaries are the specification's, not this
/// implementation's.
fn band(value: u8) -> Color {
    if value >= BAND_HIGH_FLOOR {
        BAND_HIGH_COLOUR
    } else if value >= BAND_MIDDLE_FLOOR {
        BAND_MIDDLE_COLOUR
    } else {
        BAND_LOW_COLOUR
    }
}

// ---- inspector ---------------------------------------------------------------------------

fn draw_inspector(frame: &mut Frame, area: Rect, observer: &Observer) {
    let title = fit(
        " inspector ".to_string(),
        " insp ".to_string(),
        usize::from(area.width.saturating_sub(2)),
    );
    frame.render_widget(
        Paragraph::new(inspector_lines(observer))
            .block(Block::bordered().title_top(title))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn inspector_lines(observer: &Observer) -> Vec<Line<'static>> {
    // Rule 10.5: nothing selected is stated, never defaulted to an arbitrary Mokiterion.
    let Some(selection) = observer.selection() else {
        return vec![
            Line::from("nothing selected"),
            Line::from("Tab selects a Mokiterion in roster order"),
        ];
    };

    // Rule 10 as amended on 2026-08-19: the name precedes the identifier, and this line is above
    // the living-or-dead branch below, so rule 10.6's retained selection is identified the same
    // way after death as before it. Where the engine reported no name the identifier stands alone
    // rather than being presented as one.
    let heading = match observer.name_of(selection) {
        Some(name) => format!("{name}  {selection}"),
        None => selection.to_string(),
    };
    let mut lines = vec![Line::from(Span::styled(
        heading,
        Style::new().add_modifier(Modifier::BOLD),
    ))];

    let Some(agent) = observer.selected_agent() else {
        // Rule 10.6: the selection survives the death and the pane presents it.
        match observer.death_of(selection) {
            Some(death) => {
                lines.push(Line::from(Span::styled(
                    format!("died on tick {}", death.tick),
                    Style::new().fg(Color::Red),
                )));
                let mut final_values = format!("final health {}", death.health);
                // Rule 10.7: a value the engine did not compute is absent, not zero-filled.
                if let Some(satiety) = death.satiety {
                    final_values.push_str(&format!("  satiety {satiety}"));
                }
                if let Some(energy) = death.energy {
                    final_values.push_str(&format!("  energy {energy}"));
                }
                lines.push(Line::from(final_values));
            }
            None => lines.push(Line::from(
                "no longer living, and no death record was retained",
            )),
        }
        return lines;
    };

    lines.push(Line::from(format!(
        "position  {}  territory {}",
        agent.position, agent.territory
    )));
    lines.push(Line::from(format!(
        "sharing   {} in this rendered cell",
        observer.shared_cell_count()
    )));
    lines.push(Line::from(format!(
        "health {}  satiety {}  energy {}",
        agent.health, agent.satiety, agent.energy
    )));
    lines.push(Line::from(""));

    let snapshot = observer.snapshot();
    // Rule 10.3: proposal and outcome come from one record of one tick, so they cannot be
    // presented from different ticks.
    match snapshot
        .decisions
        .iter()
        .find(|decision| decision.agent_id == selection)
    {
        // Rule 10.4.
        None => lines.push(Line::from("no proposal has yet been made")),
        Some(decision) => {
            lines.push(Line::from(format!("decision, tick {}", snapshot.tick)));
            lines.push(Line::from(format!(
                "proposed  {}",
                action_text(&decision.proposed)
            )));
            match &decision.outcome {
                DecisionOutcome::Accepted => lines.push(Line::from(Span::styled(
                    "outcome   + accepted",
                    Style::new().fg(Color::Green),
                ))),
                DecisionOutcome::Rejected { ground } => {
                    // Rule 10.1 and 10.2: a word and a symbol, and never a fault or a warning.
                    lines.push(Line::from(Span::styled(
                        "outcome   x rejected",
                        Style::new().fg(Color::Cyan),
                    )));
                    lines.push(Line::from(format!("ground    {ground}")));
                    lines.push(Line::from(
                        "the engine validates every proposal; a rejection is the authority boundary working",
                    ));
                }
            }
            lines.push(Line::from(format!(
                "applied   {}",
                decision
                    .applied
                    .as_ref()
                    .map_or_else(|| ABSENT.to_string(), action_text)
            )));
            lines.push(Line::from(format!(
                "authority {}",
                authority::DECISION_AUTHORITY
            )));
        }
    }
    lines
}

// ---- event log ---------------------------------------------------------------------------

fn draw_log(frame: &mut Frame, area: Rect, observer: &Observer) {
    let interior_width = usize::from(area.width.saturating_sub(2));
    let rows = usize::from(area.height.saturating_sub(2));
    let events = observer.events();
    let presented = observer.presented();

    let lines = if presented.is_empty() {
        // Rule 9.3.
        vec![Line::from("the filter matched no retained event")]
    } else {
        // Rule 9.1: the newest records are visible without operator action, and the cursor
        // moves the window towards older ones.
        let end = presented.len() - observer.log_cursor().min(presented.len() - 1);
        let start = end.saturating_sub(rows);
        presented[start..end]
            .iter()
            .enumerate()
            .map(|(offset, event)| {
                let style = if start + offset + 1 == end {
                    Style::new().add_modifier(Modifier::REVERSED)
                } else {
                    Style::new()
                };
                Line::from(Span::styled(event.to_string(), style))
            })
            .collect()
    };

    let truncation = if events.truncated() {
        "  truncated"
    } else {
        ""
    };
    let title = fit(
        format!(" log  retained {}{truncation} ", events.len()),
        format!(" log {}{truncation} ", events.len()),
        interior_width,
    );
    frame.render_widget(
        Paragraph::new(lines).block(Block::bordered().title_top(title)),
        area,
    );
}

// ---- provenance footer -------------------------------------------------------------------

fn draw_footer(frame: &mut Frame, area: Rect, observer: &Observer) {
    let config = observer.config();
    let snapshot = observer.snapshot();
    let events = observer.events();
    let truncation = if events.truncated() {
        "  truncated"
    } else {
        ""
    };
    let commit = COMMIT.map_or_else(String::new, |commit| format!("  commit {commit}"));
    let short_commit = COMMIT.map_or_else(String::new, |commit| format!(" #{commit}"));

    // Rule 8: seed, configured tick limit, density as supplied, active decision source, current
    // tick, retained-event count with a truncation marker. Rule 8.1: values come from the
    // engine's configuration, so a defaulted and an explicit value present identically. Rule
    // 8.3: no wall-clock time, no absolute path, no environment variable, no credential.
    let long = format!(
        "seed {}  ticks {}  density {}%  source {}  tick {}  events {}{truncation}{commit}",
        config.seed,
        config.tick_limit,
        config.density,
        config.policy,
        snapshot.tick,
        events.len()
    );
    let medium = format!(
        "seed {} ticks {} density {}% source {} tick {} events {}{truncation}{commit}",
        config.seed,
        config.tick_limit,
        config.density,
        config.policy,
        snapshot.tick,
        events.len()
    );
    let short = format!(
        "s{} t{} d{}% {} @{} e{}{truncation}{short_commit}",
        config.seed,
        config.tick_limit,
        config.density,
        config.policy,
        snapshot.tick,
        events.len()
    );
    let tiny = format!(
        "s{} t{} d{}% {} @{} e{}{short_commit}",
        config.seed,
        config.tick_limit,
        config.density,
        config.policy.to_string().chars().next().unwrap_or('?'),
        snapshot.tick,
        events.len()
    );

    let width = usize::from(area.width);
    let text = [long, medium, short, tiny]
        .into_iter()
        .find(|candidate| count(candidate) <= width)
        .unwrap_or_else(|| {
            format!(
                "s{} t{} @{} e{}",
                config.seed,
                config.tick_limit,
                snapshot.tick,
                events.len()
            )
        });
    frame.render_widget(Paragraph::new(Line::from(text)), area);
}

// ---- overlays ----------------------------------------------------------------------------

fn draw_overlay(frame: &mut Frame, area: Rect, observer: &Observer) {
    let interior_width = usize::from(area.width.saturating_sub(2));
    match observer.overlay() {
        Overlay::None => {}
        // Every pane the viewport excludes is reachable here, over the whole body (rule 5).
        Overlay::Roster => {
            frame.render_widget(Clear, area);
            draw_roster(frame, area, observer);
        }
        Overlay::Log => {
            frame.render_widget(Clear, area);
            draw_log(frame, area, observer);
        }
        Overlay::Inspector => {
            frame.render_widget(Clear, area);
            draw_inspector(frame, area, observer);
        }
        Overlay::Help => {
            frame.render_widget(Clear, area);
            let title = fit(
                " key bindings ".to_string(),
                " keys ".to_string(),
                interior_width,
            );
            frame.render_widget(
                Paragraph::new(help_lines()).block(Block::bordered().title_top(title)),
                area,
            );
        }
        Overlay::Authority => {
            frame.render_widget(Clear, area);
            let title = fit(
                " authority mapping ".to_string(),
                " authority ".to_string(),
                interior_width,
            );
            frame.render_widget(
                Paragraph::new(authority_lines(observer)).block(Block::bordered().title_top(title)),
                area,
            );
        }
    }
}

/// Rule 7's table, which is the complete set of operator influence over the run.
fn help_lines() -> Vec<Line<'static>> {
    const BINDINGS: [(&str, &str); 17] = [
        ("Space", "hold or release progression"),
        (".", "advance exactly one tick; only while held"),
        ("+ / -", "next faster / slower speed step"),
        ("Tab / Shift-Tab", "select next / previous Mokiterion"),
        ("Esc", "close an overlay, else clear the selection"),
        ("f", "toggle follow"),
        ("z", "toggle overview and detail zoom"),
        ("h j k l / arrows", "pan one world cell"),
        ("PageUp / PageDown", "pan one visible region vertically"),
        (
            "j k PageUp PageDown",
            "scroll the log while its overlay is open",
        ),
        ("e", "cycle the event-type filter"),
        ("u", "filter the log to the selected Mokiterion"),
        ("c", "clear the filter"),
        ("x", "export every retained event"),
        ("t", "authority of the highlighted event type"),
        ("r / L / i", "roster / log / inspector overlay"),
        ("q", "quit"),
    ];
    let mut lines: Vec<Line<'static>> = BINDINGS
        .iter()
        .map(|(key, control)| {
            Line::from(vec![
                Span::styled(
                    format!("{key:<21}"),
                    Style::new().add_modifier(Modifier::BOLD),
                ),
                Span::raw(*control),
            ])
        })
        .collect();
    lines.push(Line::from(vec![
        Span::styled("?", Style::new().add_modifier(Modifier::BOLD)),
        Span::raw("                    this overlay"),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from("No binding mutates world state."));
    lines
}

/// Rule 11's mapping, for the highlighted event type and in full.
fn authority_lines(observer: &Observer) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    match observer.highlighted() {
        Some(event) => {
            lines.push(Line::from(Span::styled(
                format!("highlighted    {}", event.event_type()),
                Style::new().add_modifier(Modifier::BOLD),
            )));
            match authority::for_event(event) {
                Some(identifier) => {
                    lines.push(Line::from(format!("authorized by  {identifier}")));
                }
                // Rule 11.2: a missing entry is stated, never replaced by a plausible one.
                None => lines.push(Line::from(Span::styled(
                    "the mapping is missing for this event",
                    Style::new().fg(Color::Red),
                ))),
            }
        }
        None => lines.push(Line::from("no event is highlighted")),
    }
    lines.push(Line::from(""));
    for (event_type, identifier) in authority::table(observer.config().policy) {
        lines.push(Line::from(format!("{event_type:<26} {identifier}")));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "{:<26} {}",
        "decision proposal/outcome",
        authority::DECISION_AUTHORITY
    )));
    lines.push(Line::from(format!(
        "{:<26} {}",
        "perceived entities",
        authority::PERCEPTION_AUTHORITY
    )));
    lines.push(Line::from(""));
    // Rule 11.1.
    lines.push(Line::from(
        "Identifiers only. Requirement text lives in the artifact that holds it.",
    ));
    lines
}

// ---- shared helpers ----------------------------------------------------------------------

fn territory_colour(territory: Territory) -> Color {
    match territory {
        Territory::A => TERRITORY_A_COLOUR,
        Territory::B => TERRITORY_B_COLOUR,
    }
}

fn territory_style(territory: Territory) -> Style {
    Style::new().fg(territory_colour(territory))
}

fn class_colour(class: FoodClass) -> Color {
    match class {
        FoodClass::Low => LOW_COLOUR,
        FoodClass::Medium => MEDIUM_COLOUR,
        FoodClass::High => HIGH_COLOUR,
    }
}

/// Columns a string occupies. Every glyph this observer draws is single-width.
fn count(text: &str) -> usize {
    text.chars().count()
}

/// The long form when it fits, the short form otherwise. Choosing a form is always preferred to
/// letting a pane truncate, because truncation is what silently drops an announcement.
fn fit(long: String, short: String, width: usize) -> String {
    if count(&long) <= width { long } else { short }
}

fn clip(text: &str, width: usize) -> String {
    if count(text) <= width {
        return text.to_string();
    }
    text.chars()
        .take(width.saturating_sub(1))
        .chain(std::iter::once('\u{2026}'))
        .collect()
}

/// One line with the left group at the left and the right group at the right edge.
fn justified(left: Vec<Span<'static>>, right: Vec<Span<'static>>, width: usize) -> Line<'static> {
    if right.is_empty() {
        return Line::from(left);
    }
    let used: usize = left.iter().chain(right.iter()).map(Span::width).sum();
    let mut spans = left;
    spans.push(Span::raw(" ".repeat(width.saturating_sub(used).max(1))));
    spans.extend(right);
    Line::from(spans)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::{self, Startup};
    use mokiterions::simulation::{Action, Coordinate};
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;

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

    fn press(code: ratatui::crossterm::event::KeyCode) -> ratatui::crossterm::event::KeyEvent {
        ratatui::crossterm::event::KeyEvent::new(
            code,
            ratatui::crossterm::event::KeyModifiers::NONE,
        )
    }

    /// Rule 2.2: the boundary sits between world rows 63 and 64, so territory A is above B.
    #[test]
    fn the_territory_rule_marks_the_row_between_the_territories() {
        let mut observer = start(&[]);
        observer.record_geometry((32, 16), 8);
        observer
            .handle_key(press(ratatui::crossterm::event::KeyCode::Char('z')))
            .unwrap();
        for _ in 0..60 {
            observer
                .handle_key(press(ratatui::crossterm::event::KeyCode::Char('j')))
                .unwrap();
        }
        assert_eq!(observer.camera(), (0, 60));

        let buffer = frame_of(&mut observer, 34, 22);
        // The header is three rows and the pane border one, so canvas row 0 is screen row 4.
        let boundary_row = 4usize + (63 - 60);
        let rendered = rows(&buffer);
        assert!(
            rendered[boundary_row].contains(BOUNDARY_GLYPH),
            "row {boundary_row} carries no territory rule: {}",
            rendered[boundary_row]
        );
        // The rows above it carry no rule, so the boundary is a single line in the right place.
        for (above, row) in rendered.iter().enumerate().take(boundary_row).skip(4) {
            assert!(
                !row.contains(BOUNDARY_GLYPH),
                "row {above} carries a second rule"
            );
        }
    }

    /// Rule 4's mockup line two, byte for byte at the width it was written for.
    #[test]
    fn the_bar_row_reproduces_the_specified_form() {
        let agent = AgentSnapshot {
            id: "M05".to_string(),
            position: Coordinate { x: 81, y: 14 },
            territory: Territory::A,
            health: 100,
            satiety: 81,
            energy: 72,
            fear: 20,
            applied_action: Some(Action::Eat {
                food_id: "F0058".to_string(),
            }),
        };
        // Rule 4.7 made an entry line a sequence of styled spans. `Line`'s `Display` writes span
        // content and nothing else, so the text asserted here is the text that reaches a cell.
        let lines: Vec<String> = entry_lines(&agent, "Blip", FULL_BAR, true)
            .iter()
            .map(ToString::to_string)
            .collect();

        assert_eq!(
            lines[1],
            "     h ████████████████████ 100  s ████████████████░░░░  81  e ██████████████░░░░░░  72  f ████░░░░░░░░░░░░░░░░  20"
        );
        // Line one carries the name, identifier, territory, position and the applied action, in
        // the mockup's columns. The action uses the engine's own rendering, `eat:F0058`. The bar
        // row asserted above is the mockup's unchanged, which is the measurement that rule 4 as
        // amended on 2026-08-19 for `REQ-MOK-041` leaves line two and its arithmetic alone.
        //
        // The name here is a fabricated four-character stand-in, not the name rule 4's mockup
        // prints beside `M05`: `REQ-MOK-041` lets no engine name be written down in this package,
        // and what the columns hold is a length, not a particular name.
        assert!(
            lines[0].starts_with("Blip  M05  A  81:14         "),
            "{}",
            lines[0]
        );
        assert!(lines[0].ends_with("eat:F0058"));

        // The fourth slot rule 4.5 reserved now carries `fear`, and the row ends there: no fifth
        // group and no trailing padding follow it.
        assert_eq!(lines[1].trim_end(), lines[1]);
        assert_eq!(count(&lines[1]), 4 * FULL_BAR + BAR_ROW_OVERHEAD);

        // Rule 4.7 on the mockup's own values: 100 and 81 are high, 72 is middle. Two gauges
        // sharing a band and one differing is the shape a band read from the row rather than from
        // the value would get wrong, and it would get it wrong while still reading all-green.
        //
        // The fourth gauge takes no band, so its span carries no foreground -- the same `None` a
        // separator carries. Its content is asserted alongside the vector, because the vector
        // alone cannot tell an unbanded gauge from the space in front of it.
        let spans = entry_lines(&agent, "Blip", FULL_BAR, true).remove(1).spans;
        let bands: Vec<Option<Color>> = spans.iter().map(|span| span.style.fg).collect();
        assert_eq!(
            bands,
            vec![
                None,
                Some(Color::Green),
                None,
                Some(Color::Green),
                None,
                Some(Color::Indexed(208)),
                None,
                None
            ]
        );
        assert!(spans[7].content.starts_with('f'), "{}", spans[7].content);
    }

    #[test]
    fn a_bar_row_shrinks_to_its_pane_and_never_overflows_it() {
        // The 47-column roster pane has a 45-column interior. Four groups narrow the bars to two
        // cells where three groups gave six — rule 4 as amended on 2026-08-19 accepts that.
        assert_eq!(bar_width(45), 2);
        // `45 − 35` is not a multiple of four, so two interior columns stay unused. The row is
        // shorter than the interior rather than wider than it, which is the property that matters.
        assert_eq!(4 * bar_width(45) + BAR_ROW_OVERHEAD, 43);
        // A full-width overlay reaches the mockup's twenty.
        assert_eq!(bar_width(158), FULL_BAR);
        // A pane too narrow for any bar asks for none rather than for a negative width.
        assert_eq!(bar_width(20), 0);

        for interior in 1..200u16 {
            let bar = bar_width(interior);
            if usize::from(interior) < BAR_ROW_OVERHEAD {
                // Nothing is left to divide, so the row asks for no bar at all.
                assert_eq!(bar, 0, "interior {interior} asked for a bar it cannot hold");
            } else {
                assert!(
                    4 * bar + BAR_ROW_OVERHEAD <= usize::from(interior),
                    "interior {interior} overflows with a bar of {bar}"
                );
            }
        }
    }

    /// Rule 4.7's band table, stated here as the specification states it rather than read from the
    /// implementation's constants, so a test cannot agree with a wrong boundary.
    fn specified_band(value: u8) -> Color {
        match value {
            80..=100 => Color::Green,
            40..=79 => Color::Indexed(208),
            0..=39 => Color::Red,
            _ => panic!("{value} is above the attribute maximum"),
        }
    }

    /// Rule 4.7's bands, over the whole domain and at both boundaries by literal value.
    #[test]
    fn the_survival_bands_are_the_three_the_rule_fixes() {
        // Each boundary by its own literal, both sides, so an off-by-one cannot hide in a range.
        assert_eq!(band(39), Color::Red);
        assert_eq!(band(40), Color::Indexed(208));
        assert_eq!(band(79), Color::Indexed(208));
        assert_eq!(band(80), Color::Green);
        // The ends, and zero, which the rule places in the low band.
        assert_eq!(band(0), Color::Red);
        assert_eq!(band(100), Color::Green);

        // Total and disjoint: every value in the domain is in exactly the band the rule names.
        for value in 0..=100u8 {
            assert_eq!(band(value), specified_band(value), "band of {value}");
        }

        // Monotone: the band never improves as the value falls. This is the property a trend
        // encoding would have failed, since satiety and energy decay every tick by construction.
        let rank = |colour: Color| match colour {
            Color::Red => 0,
            Color::Indexed(208) => 1,
            Color::Green => 2,
            other => panic!("{other:?} is not a band"),
        };
        for value in 1..=100u8 {
            assert!(
                rank(band(value)) >= rank(band(value - 1)),
                "band improves falling from {value} to {}",
                value - 1
            );
        }
    }

    /// Rule 4.7 changes no character. This is the property that keeps rule 4's mockup true and
    /// keeps every assertion made about the roster before bands existed meaningful.
    #[test]
    fn banding_changes_no_character_of_an_entry() {
        // The unbanded form, written out rather than captured from the current implementation, so
        // that a regression in `gauge` cannot ratify itself.
        let unbanded = |label: char, value: u8, width: usize| {
            let filled = (usize::from(value) * width / 100).min(width);
            format!(
                "{label} {}{} {value:>3}",
                "\u{2588}".repeat(filled),
                "\u{2591}".repeat(width - filled)
            )
        };

        let mut cases = 0;
        for width in 0..=FULL_BAR {
            for value in 0..=100u8 {
                assert_eq!(
                    gauge('h', value, width).content,
                    unbanded('h', value, width),
                    "gauge text at value {value} width {width}"
                );
                // Rule 4.7's fourth gauge takes no band, and the same property has to hold of
                // its absence: an unbanded gauge is a banded one with the colour removed and
                // not a different form. `fear` is held to rule 4's mockup exactly as the three
                // survival gauges are.
                assert_eq!(
                    unbanded_gauge('f', value, width).content,
                    unbanded('f', value, width),
                    "unbanded gauge text at value {value} width {width}"
                );
                cases += 1;
            }
        }
        // Every value at every width the layout can ask for: 21 widths × 101 values.
        assert_eq!(cases, 21 * 101);

        // And at the level of a whole entry, including the indent and the separators.
        let agent = AgentSnapshot {
            id: "M07".to_string(),
            position: Coordinate { x: 3, y: 40 },
            territory: Territory::B,
            health: 12,
            satiety: 55,
            energy: 88,
            // Rule 4.5 as amended fills the reserved fourth slot with `fear`, and rule 4.7
            // leaves that gauge unbanded. Its characters are still the rule's characters.
            fear: 33,
            applied_action: Some(Action::Sleep),
        };
        assert_eq!(
            entry_lines(&agent, "Ort", FULL_BAR, true)[1].to_string(),
            format!(
                "     {}  {}  {}  {}",
                unbanded('h', 12, FULL_BAR),
                unbanded('s', 55, FULL_BAR),
                unbanded('e', 88, FULL_BAR),
                unbanded('f', 33, FULL_BAR)
            )
        );
    }

    /// Rule 4.7: a band is the property of one gauge, not of the row.
    #[test]
    fn each_gauge_carries_its_own_band_and_nothing_else_carries_one() {
        let agent = AgentSnapshot {
            id: "M07".to_string(),
            position: Coordinate { x: 3, y: 40 },
            territory: Territory::B,
            // One value in each band, so a single shared style cannot pass.
            health: 12,
            satiety: 55,
            energy: 88,
            // In the low band, had it one, and distinct from all three so that the tail of the
            // row identifies it. Rule 4.7 as amended puts `fear` outside the scale, so a shared
            // style borrowed from the row would show here as a red gauge.
            fear: 7,
            applied_action: Some(Action::Sleep),
        };
        let row = entry_lines(&agent, "Ort", FULL_BAR, true).remove(1);

        // Indent, gauge, separator, gauge, separator, gauge, separator, gauge.
        assert_eq!(row.spans.len(), 8);
        assert_eq!(row.spans[0].style.fg, None, "the indent carries a band");
        assert_eq!(row.spans[2].style.fg, None, "a separator carries a band");
        assert_eq!(row.spans[4].style.fg, None, "a separator carries a band");
        assert_eq!(row.spans[6].style.fg, None, "a separator carries a band");
        assert_eq!(row.spans[1].style.fg, Some(Color::Red));
        assert_eq!(row.spans[3].style.fg, Some(Color::Indexed(208)));
        assert_eq!(row.spans[5].style.fg, Some(Color::Green));

        // Three distinct bands, and no modifier anywhere: rule 4.6 owns reversed video and rule
        // 4.7 adds no emphasis of its own.
        for span in &row.spans {
            assert_eq!(span.style.add_modifier, Modifier::empty(), "{span:?}");
        }

        // Rule 4.5's reserved fourth-bar slot is filled, and rule 4.7 as amended gives what
        // fills it no band. This is the one span that carries characters and no colour, so the
        // vector alone cannot tell it from a separator: its content is asserted with its style.
        assert_eq!(
            row.spans[7].style.fg, None,
            "the fourth gauge carries a band"
        );
        assert!(
            row.spans[7].content.starts_with('f'),
            "{}",
            row.spans[7].content
        );
        assert!(row.to_string().ends_with("  7"));
    }

    /// Rule 4.7 reads the current value and nothing else. No previous tick is retained, so
    /// `REQ-MOK-020`'s constraint against a quantity the engine does not produce still holds.
    #[test]
    fn a_band_reads_only_the_value_it_is_given() {
        let of = |value: u8| {
            let agent = AgentSnapshot {
                id: "M01".to_string(),
                position: Coordinate { x: 0, y: 0 },
                territory: Territory::A,
                health: value,
                satiety: value,
                energy: value,
                fear: value,
                applied_action: None,
            };
            entry_lines(&agent, "Ort", FULL_BAR, true)
                .remove(1)
                .spans
                .iter()
                .map(|span| span.style.fg)
                .collect::<Vec<_>>()
        };

        // Interleaved so that any hidden previous-value state would show as a difference between
        // the first and second reading of the same value.
        let first = of(85);
        for other in [0u8, 100, 39, 40, 79, 80, 12] {
            let _ = of(other);
        }
        assert_eq!(of(85), first, "a band depended on what was rendered before");
        assert_eq!(band(85), Color::Green);
    }

    /// Rule 4.7: the collapsed one-line form has no bars and takes no band.
    #[test]
    fn the_collapsed_form_takes_no_band() {
        let agent = AgentSnapshot {
            id: "M01".to_string(),
            position: Coordinate { x: 0, y: 0 },
            territory: Territory::A,
            health: 12,
            satiety: 55,
            energy: 88,
            // Rule 4.5 as amended makes the collapsed form four numeric values rather than
            // three, which is the correction this work order made to clause 7's own wording.
            fear: 7,
            applied_action: None,
        };
        let lines = entry_lines(&agent, "Ort", 0, false);
        assert_eq!(lines.len(), 1);
        // Rule 4 as amended for `REQ-MOK-041` gives line one a six-column name field, which
        // the collapsed form carries too. What clause 7 asserts here is unchanged: the form
        // has no bars and no span in it takes a band, the name field included.
        assert_eq!(lines[0].to_string(), "Ort   M01  A  h 12 s 55 e 88 f  7");
        for span in &lines[0].spans {
            assert_eq!(span.style.fg, None, "the collapsed form carries a band");
        }
    }

    #[test]
    fn a_zero_value_is_a_zero_and_an_absent_value_is_a_dash() {
        let mut agent = AgentSnapshot {
            id: "M01".to_string(),
            position: Coordinate { x: 0, y: 0 },
            territory: Territory::A,
            health: 0,
            satiety: 0,
            energy: 0,
            fear: 0,
            applied_action: None,
        };
        let lines: Vec<String> = entry_lines(&agent, "Ort", 4, true)
            .iter()
            .map(ToString::to_string)
            .collect();
        assert!(lines[0].ends_with(ABSENT), "{}", lines[0]);
        assert!(lines[1].contains("h ░░░░   0"), "{}", lines[1]);
        // Rule 4.4 governs the fourth gauge on the same terms: a computed zero is a zero and an
        // empty bar, which is also every Mokiterion's initial `fear`.
        assert!(lines[1].contains("f ░░░░   0"), "{}", lines[1]);
        // Rule 4.7 puts zero in the low band. It stays a `0` with an empty bar, so what
        // distinguishes it from an absent value is still the character and not the colour. The
        // fourth gauge is outside that scale entirely: `fear 0` is the best state `fear` has
        // rather than the worst, and it takes no band at all.
        let spans = entry_lines(&agent, "Ort", 4, true).remove(1).spans;
        assert_eq!(spans[1].style.fg, Some(Color::Red));
        assert_eq!(spans[7].style.fg, None);
        assert!(spans[7].content.starts_with('f'), "{}", spans[7].content);

        agent.applied_action = Some(Action::Wait);
        assert!(
            entry_lines(&agent, "Ort", 4, true)[0]
                .to_string()
                .ends_with("wait")
        );

        // The one-line form keeps the numbers and drops the bars, and carries the name.
        let compact = entry_lines(&agent, "Ort", 0, false);
        assert_eq!(compact.len(), 1);
        assert_eq!(compact[0].to_string(), "Ort   M01  A  h  0 s  0 e  0 f  0");
    }

    /// `REQ-MOK-041`: the name is presented in addition to the identifier and before it, in both
    /// entry forms, and it costs the fields beside it nothing.
    #[test]
    fn an_entry_carries_the_name_before_the_identifier_and_takes_six_columns() {
        let agent = AgentSnapshot {
            id: "M12".to_string(),
            position: Coordinate { x: 127, y: 127 },
            territory: Territory::B,
            health: 100,
            satiety: 100,
            energy: 100,
            fear: 0,
            applied_action: Some(Action::Sleep),
        };

        // The longest name the engine can report is five characters, so the six-column field
        // holds it with its separating space and the identifier still starts at column six. The
        // four fixtures are fabricated, one per admissible length: what the field has to hold is
        // a length, and an engine name written down here would be the table `REQ-MOK-041` forbids.
        for name in ["O", "Ort", "Blip", "Weeee"] {
            let two_line = entry_lines(&agent, name, 2, true);
            let one_line = entry_lines(&agent, name, 0, false);
            for line in [&two_line[0], &one_line[0]] {
                let text = line.to_string();
                assert!(text.starts_with(name), "{text}");
                assert_eq!(&text[6..9], "M12", "{text}");
            }
            // Line two is the same row whatever the name is: the name is on line one only. The
            // `Line`s are compared rather than their text, so rule 4.7's bands are held to it too.
            assert_eq!(two_line[1], entry_lines(&agent, "Ort", 2, true)[1]);
        }

        // Where the engine reported no name the field is blank. The identifier is not moved into
        // it, which `REQ-MOK-041` forbids as a derivation, and it is not filled with a
        // placeholder, which `SPEC-MOK-003` rule 10.7 forbids as an uncomputed value.
        let unnamed = entry_lines(&agent, "", 2, true);
        let first = unnamed[0].to_string();
        assert!(first.starts_with("      M12"), "{first}");
        assert_eq!(unnamed[1], entry_lines(&agent, "Ort", 2, true)[1]);
    }

    /// Rule 3.1 and 3.2, which are states rather than counts.
    #[test]
    fn a_depleted_territory_is_stated_in_words_at_every_width() {
        let territory = |standing: usize, depleted: bool| TerritorySnapshot {
            id: Territory::B,
            standing,
            low: standing,
            medium: 0,
            high: 0,
            capacity: 61,
            permanently_depleted: depleted,
        };

        let wide = territory_line(&territory(0, true), 160).to_string();
        assert!(wide.contains("permanently depleted"), "{wide}");
        assert!(wide.contains("standing 0/61"), "{wide}");

        let narrow = territory_line(&territory(0, true), 34).to_string();
        assert!(narrow.contains("depleted"), "{narrow}");
        assert!(count(&narrow) <= 34, "{narrow}");

        let last = territory_line(&territory(1, false), 160).to_string();
        assert!(last.contains("one from sterile"), "{last}");
        let last = territory_line(&territory(1, false), 34).to_string();
        assert!(last.contains("last one"), "{last}");
        assert!(count(&last) <= 34, "{last}");

        // An ordinary territory carries no state word at all.
        let ordinary = territory_line(&territory(40, false), 160).to_string();
        assert!(!ordinary.contains("depleted") && !ordinary.contains("sterile"));
    }

    #[test]
    fn the_footer_survives_the_narrowest_viewport() {
        let mut observer = start(&[]);
        let footer = rows(&frame_of(&mut observer, 34, 22))[21].clone();
        let footer = footer.trim_end().to_string();
        assert!(count(&footer) <= 34, "{footer}");
        for field in ["s0", "t100", "d0.75%", "@0", "e"] {
            assert!(footer.contains(field), "{footer} lacks {field}");
        }
    }

    /// Rule 10.4 and 10.5.
    #[test]
    fn the_inspector_states_absence_rather_than_inventing_a_subject() {
        let mut observer = start(&["--start-paused"]);
        let text = text_of(&frame_of(&mut observer, 160, 48));
        assert!(text.contains("nothing selected"), "{text}");

        observer.select_for_test("M03");
        let text = text_of(&frame_of(&mut observer, 160, 48));
        assert!(text.contains("no proposal has yet been made"), "{text}");
        assert!(text.contains("M03"));

        observer.advance().unwrap();
        let text = text_of(&frame_of(&mut observer, 160, 48));
        assert!(text.contains("decision, tick 1"), "{text}");
        assert!(text.contains("proposed"), "{text}");
        assert!(
            text.contains("+ accepted") || text.contains("x rejected"),
            "{text}"
        );
        assert!(text.contains(authority::DECISION_AUTHORITY), "{text}");
    }

    /// Rule 9.1 and 9.3.
    #[test]
    fn the_log_shows_the_newest_records_and_reports_an_empty_filter() {
        let mut observer = start(&["--start-paused"]);
        observer.advance().unwrap();
        let newest = observer.presented().last().unwrap().to_string();
        let text = text_of(&frame_of(&mut observer, 160, 48));
        assert!(text.contains(&newest[..40]), "{text}");

        // A subject that emits nothing leaves the pane stating exactly that.
        observer.select_for_test("M03");
        observer
            .handle_key(press(ratatui::crossterm::event::KeyCode::Char('e')))
            .unwrap();
        while !observer.presented().is_empty() {
            observer
                .handle_key(press(ratatui::crossterm::event::KeyCode::Char('e')))
                .unwrap();
            if matches!(observer.filter(), crate::state::Filter::None) {
                break;
            }
        }
        if observer.presented().is_empty() {
            let text = text_of(&frame_of(&mut observer, 160, 48));
            assert!(
                text.contains("the filter matched no retained event"),
                "{text}"
            );
        }
    }

    #[test]
    fn an_overlay_covers_the_body_and_leaves_the_header_and_the_footer() {
        let mut observer = start(&[]);
        observer.set_overlay_for_test(Overlay::Help);
        let buffer = frame_of(&mut observer, 160, 48);
        let rendered = rows(&buffer);

        assert!(rendered[0].contains("RUNNING"), "{}", rendered[0]);
        assert!(rendered[47].contains("seed 0"), "{}", rendered[47]);
        let body = rendered[3..47].join("\n");
        assert!(body.contains("key bindings"), "{body}");
        assert!(body.contains("hold or release progression"), "{body}");
        assert!(
            !body.contains("roster  living"),
            "the overlay must cover the body"
        );
    }

    /// Rule 7's table is the complete set of operator influence, so the overlay lists all of it.
    #[test]
    fn the_help_overlay_lists_every_bound_key() {
        let text = help_lines()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n");
        for key in [
            "Space",
            ".",
            "+ / -",
            "Tab / Shift-Tab",
            "Esc",
            "f",
            "z",
            "h j k l / arrows",
            "PageUp / PageDown",
            "e",
            "u",
            "c",
            "x",
            "t",
            "r / L / i",
            "?",
            "q",
        ] {
            assert!(text.contains(key), "{key} is not listed");
        }
        assert!(text.contains("No binding mutates world state."));
    }

    /// Rule 11: the overlay presents the highlighted type's authority and the whole mapping.
    #[test]
    fn the_authority_overlay_names_identifiers_for_every_event_type() {
        let mut observer = start(&[]);
        observer.set_overlay_for_test(Overlay::Authority);
        let text = text_of(&frame_of(&mut observer, 160, 48));

        assert!(text.contains("highlighted"), "{text}");
        assert!(text.contains("decision_source_selected"), "{text}");
        assert!(text.contains("REQ-MOK-015"), "{text}");
        assert!(text.contains(authority::PERCEPTION_AUTHORITY), "{text}");
        assert!(
            text.contains("Requirement text lives in the artifact"),
            "{text}"
        );
    }

    /// Rule 5's resize behavior: presentation state survives, and so does the run.
    #[test]
    fn a_resize_changes_the_layout_and_nothing_else() {
        let mut observer = start(&["--start-paused"]);
        observer.advance().unwrap();
        observer.select_for_test("M07");
        observer
            .handle_key(press(ratatui::crossterm::event::KeyCode::Char('z')))
            .unwrap();

        let before = (
            observer.snapshot().tick,
            observer.selection().map(str::to_string),
            observer.zoom(),
            observer.events().len(),
        );
        for (width, height) in [(160u16, 48u16), (33, 21), (100, 30), (34, 22), (160, 48)] {
            let _ = frame_of(&mut observer, width, height);
        }
        assert_eq!(
            (
                observer.snapshot().tick,
                observer.selection().map(str::to_string),
                observer.zoom(),
                observer.events().len(),
            ),
            before
        );
    }
}
