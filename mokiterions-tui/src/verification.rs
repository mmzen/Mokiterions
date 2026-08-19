//! `VER-MOK-005`'s cross-cutting cases that reach a `#[cfg(test)]` hook.
//!
//! The per-module tests verify one obligation at a time. The cases here verify the properties
//! that span the whole observer, and above all the contract's primary property: an observed run
//! and an unobserved run of the same seed and configuration produce the same authoritative
//! stream, byte for byte, however hard the operator works the instrument.
//!
//! **The unobserved run is the engine binary's run.** The binary's whole behavior is
//! `Simulation::run`, which is called here directly, one function away from the binary's `main`.
//! The engine's own tests cover the step from `main` to that call, so comparing against it here
//! compares against the binary. The evidence retained for `WO-MOK-005` closes the loop by running
//! the real binary and comparing its bytes against an export.
//!
//! Non-perturbation is established by comparison and never by reading the observer for mutating
//! calls, which is why every case below drives two runs and diffs them rather than asserting
//! something about the observer's structure.
//!
//! `SPEC-MOK-004` rule 10 keeps these eight cases inside the crate: each reaches a
//! `#[cfg(test)]` hook, which does not exist in the build an integration test links, so no
//! test outside the crate could name one. The other sixteen are in `tests/verification.rs`.

use mokiterions::simulation::{
    Action, AgentSnapshot, Coordinate, DecisionOutcome, DecisionSnapshot, Direction, EventType,
    FoodClass, ResourceSnapshot, TerminationReason, Territory, WorldSnapshot,
};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier};

use crate::state::{Filter, Observer, Progression};
use crate::{export, layout, options, render, spatial};

/// The declared viewport set. `33 x 21` is the one-below-floor case and is refused, not rendered.
///
/// `160 x 40`, `140 x 43` and `120 x 30` joined the set with `VER-MOK-005`'s 2026-08-19 amendment:
/// they are the shapes at which the superseded tier table excluded the roster, the inspector and
/// the log at once, and no viewport in the previous set reached that region.
const VIEWPORTS: [(u16, u16); 10] = [
    (160, 48),
    (160, 44),
    (160, 40),
    (140, 44),
    (140, 43),
    (120, 48),
    (120, 30),
    (100, 30),
    (34, 22),
    (33, 21),
];

/// The declared viewports above the floor, each with the canvas interior rule 5 derives for it.
const RENDERABLE: [(u16, u16, u16, u16); 9] = [
    (160, 48, 67, 32),
    (160, 44, 67, 32),
    (160, 40, 67, 28),
    (140, 44, 47, 32),
    (140, 43, 47, 31),
    (120, 48, 71, 36),
    (120, 30, 71, 24),
    (100, 30, 51, 24),
    (34, 22, 32, 16),
];

fn observer_for(args: &[&str]) -> Observer {
    match options::parse(args.to_vec()) {
        Ok(options::Startup::Run(options)) => {
            Observer::new(options).expect("the configuration is valid")
        }
        other => panic!("{args:?} did not yield a runnable configuration: {other:?}"),
    }
}

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn tap(observer: &mut Observer, code: KeyCode) {
    observer.handle_key(press(code)).expect("no binding fails");
}

/// The observer's retained records as stream lines, which is what an export contains.
fn observed_lines(observer: &Observer) -> Vec<String> {
    observer
        .events()
        .iter()
        .map(|event| event.to_string())
        .collect()
}

/// A frame drawn at `(width, height)`, or `None` below the floor, where nothing is presented.
fn frame(observer: &mut Observer, width: u16, height: u16) -> Option<Buffer> {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test backend");
    terminal
        .draw(|target| render::draw(target, observer))
        .expect("drawing into a buffer");
    (!layout::below_floor(width, height)).then(|| terminal.backend().buffer().clone())
}

fn flatten(buffer: &Buffer) -> String {
    region(buffer, *buffer.area())
}

/// One region's rows, joined. Reading a pane rather than the screen is what makes a claim about a
/// pane's content a claim about that pane, including where the pane wraps a line.
fn region(buffer: &Buffer, area: Rect) -> String {
    (area.y..area.y + area.height)
        .map(|y| {
            (area.x..area.x + area.width)
                .map(|x| buffer.cell((x, y)).expect("inside the area").symbol())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

// ---- rendering and layout purity -----------------------------------------------------------

/// `VER-MOK-005`: drawing the same state at the same viewport twice produces identical buffers,
/// and drawing changes no authoritative state.
#[test]
fn drawing_is_pure() {
    let mut observer = observer_for(&["--seed", "777", "--start-paused"]);
    for _ in 0..7 {
        observer.advance().expect("the engine advances");
    }
    observer.select_for_test("M04");

    for (width, height, _, _) in RENDERABLE {
        let before = observer.snapshot().clone();
        let records = observed_lines(&observer);
        let first = frame(&mut observer, width, height).expect("above the floor");
        let second = frame(&mut observer, width, height).expect("above the floor");
        assert_eq!(flatten(&first), flatten(&second), "{width}x{height}");
        assert_eq!(
            observer.snapshot(),
            &before,
            "{width}x{height} changed state"
        );
        assert_eq!(observed_lines(&observer), records, "{width}x{height}");
    }
}

/// `REQ-MOK-024`: presentation state survives every resize between declared viewports, including
/// below the floor and back, and a resize does not pause the run.
#[test]
fn presentation_state_survives_every_resize() {
    let mut observer = observer_for(&["--seed", "42", "--start-paused"]);
    for _ in 0..4 {
        observer.advance().expect("the engine advances");
    }
    observer.select_for_test("M07");
    tap(&mut observer, KeyCode::Char('e'));
    tap(&mut observer, KeyCode::Char('z'));

    let selection = observer.selection().map(str::to_string);
    let filter = observer.filter().label();
    let zoom = observer.zoom();
    let progression = observer.progression();
    let speed = observer.speed();
    let retained = observer.events().len();
    let tick = observer.snapshot().tick;

    for (width, height) in VIEWPORTS {
        frame(&mut observer, width, height);
        for (next_width, next_height) in VIEWPORTS {
            frame(&mut observer, next_width, next_height);
            assert_eq!(observer.selection().map(str::to_string), selection);
            assert_eq!(observer.filter().label(), filter);
            assert_eq!(observer.zoom(), zoom);
            assert_eq!(observer.progression(), progression);
            assert_eq!(observer.speed(), speed);
            assert_eq!(observer.events().len(), retained);
            assert_eq!(observer.snapshot().tick, tick);
        }
    }
    assert_eq!(observer.progression(), Progression::Held);
}

// ---- records, filters and the export -------------------------------------------------------

/// `REQ-MOK-022`: a filter restricts presentation only. The retained buffer, its order, its count
/// and the export are unchanged by applying and clearing any filter.
#[test]
fn a_filter_changes_what_is_presented_and_nothing_else() {
    let mut observer = observer_for(&["--seed", "42", "--ticks", "30"]);
    while !observer.is_finished() {
        observer.advance().expect("the engine advances");
    }
    let records = observed_lines(&observer);
    let mut before = Vec::new();
    export::write_records(&mut before, observer.events()).expect("writing to a vector");
    observer.select_for_test("M03");

    let mut restricted_something = false;
    for _ in 0..=EventType::ALL.len() {
        tap(&mut observer, KeyCode::Char('e'));
        restricted_something |= observer.presented().len() < records.len();

        let mut after = Vec::new();
        export::write_records(&mut after, observer.events()).expect("writing to a vector");
        assert_eq!(after, before, "the export followed the filter");
        assert_eq!(
            observed_lines(&observer),
            records,
            "the buffer was reordered or trimmed"
        );
    }
    assert!(restricted_something, "no type filter restricted anything");

    tap(&mut observer, KeyCode::Char('u'));
    assert!(matches!(observer.filter(), Filter::Subject(id) if id == "M03"));
    assert!(
        observer
            .presented()
            .iter()
            .all(|event| event.subject == "M03")
    );
    let mut after = Vec::new();
    export::write_records(&mut after, observer.events()).expect("writing to a vector");
    assert_eq!(after, before, "the export followed the subject filter");

    tap(&mut observer, KeyCode::Char('c'));
    assert!(matches!(observer.filter(), Filter::None));
    assert_eq!(observer.presented().len(), records.len());
}

// ---- faithfulness --------------------------------------------------------------------------

/// `REQ-MOK-019`, `REQ-MOK-020` and `REQ-MOK-021`: every presented value is the snapshot's.
#[test]
fn every_presented_value_is_the_snapshots() {
    let mut observer = observer_for(&["--seed", "42", "--start-paused"]);
    for _ in 0..9 {
        observer.advance().expect("the engine advances");
    }
    observer.select_for_test("M02");
    let buffer = frame(&mut observer, 160, 48).expect("above the floor");
    let text = flatten(&buffer);

    let snapshot = observer.snapshot().clone();
    for territory in &snapshot.territories {
        assert!(
            text.contains(&format!("{}/{}", territory.standing, territory.capacity)),
            "territory {}'s counts are not presented",
            territory.id
        );
    }
    for agent in &snapshot.agents {
        assert!(text.contains(&agent.id), "{} is not presented", agent.id);
    }

    let selected = observer.selected_agent().expect("M02 is living").clone();
    assert!(text.contains(&selected.position.to_string()));
    assert!(text.contains(&format!("health {}", selected.health)));
    assert!(text.contains(&format!("satiety {}", selected.satiety)));
    assert!(text.contains(&format!("energy {}", selected.energy)));

    let decision = snapshot
        .decisions
        .iter()
        .find(|decision| decision.agent_id == "M02")
        .expect("a decision for a living Mokiterion");
    assert!(text.contains(&decision.proposed.to_string()));
    match &decision.applied {
        Some(action) => assert!(text.contains(&action.to_string())),
        // Rule 10.3 makes a rejected proposal's applied action an absence, never an invention.
        None => assert!(text.contains("rejected")),
    }
}

/// `REQ-MOK-021`: the observer presents the snapshot's verdict and never re-derives one, and a
/// rejection is presented as an authority outcome rather than as a fault.
///
/// The injected decision is deliberately self-contradictory: a westward move from an interior
/// position, which any validation rule read independently would accept, carrying the outcome
/// `rejected`. The observer must present the outcome it was given.
#[test]
fn the_presented_verdict_is_the_snapshots_and_a_rejection_is_not_a_fault() {
    let mut observer = observer_for(&["--seed", "42", "--start-paused"]);
    observer.advance().expect("the engine advances");
    let subject = observer.snapshot().agents[0].id.clone();
    let ground = "the target cell lies outside the world".to_string();

    observer.replace_decisions_for_test(vec![DecisionSnapshot {
        agent_id: subject.clone(),
        proposed: Action::Move {
            direction: Direction::West,
        },
        outcome: DecisionOutcome::Rejected {
            ground: ground.clone(),
        },
        applied: None,
    }]);
    observer.select_for_test(&subject);

    let buffer = frame(&mut observer, 160, 48).expect("above the floor");
    let text = flatten(&buffer);
    assert!(text.contains("rejected"), "the outcome word is missing");
    assert!(text.contains("move:west"), "the proposal is not presented");
    // The inspector wraps, so the ground is matched across the wrap, inside the pane's own
    // columns: the rest of the screen lies between the two rows and is not part of the sentence.
    let pane = layout::resolve(*buffer.area())
        .inspector
        .expect("the reference viewport shows the inspector");
    let squash = |text: &str| -> String {
        text.chars()
            .filter(|glyph| !glyph.is_whitespace() && *glyph != '\u{2502}')
            .collect()
    };
    assert!(
        squash(&region(&buffer, pane)).contains(&squash(&ground)),
        "the engine's ground is not presented:\n{}",
        region(&buffer, pane)
    );
    // Rule 10.2: a rejection is the authority boundary working, so it carries no fault wording
    // and no fault styling.
    for forbidden in ["ERROR", "WARNING", "FAIL", "panic", "invalid", "illegal"] {
        assert!(
            !text.contains(forbidden),
            "a rejection reads as {forbidden}: {text}"
        );
    }
    let red = (0..buffer.area().width)
        .flat_map(|x| (0..buffer.area().height).map(move |y| (x, y)))
        .filter_map(|(x, y)| buffer.cell((x, y)))
        .filter(|cell| cell.style().fg == Some(Color::Red))
        .count();
    assert_eq!(red, 0, "a rejection is presented in fault colouring");
}

// ---- what is not drawn ---------------------------------------------------------------------

/// The canvas interior of the view pane at one viewport: the cells rule 2 maps the world onto.
fn canvas_of(width: u16, height: u16) -> Rect {
    let view = layout::resolve(Rect::new(0, 0, width, height)).view;
    Rect::new(view.x + 1, view.y + 1, view.width - 2, view.height - 2)
}

/// Every cell of one region, as symbol and modifier, with colour discarded.
///
/// Returning a projection that has no colour in it at all is deliberate. A test that read `fg` and
/// asserted something about it could still pass on a frame whose meaning lived in colour; a test
/// reading this cannot, because the colour is not in the value under assertion.
fn monochrome(buffer: &Buffer, area: Rect) -> Vec<Vec<(String, Modifier)>> {
    (area.y..area.y + area.height)
        .map(|y| {
            (area.x..area.x + area.width)
                .map(|x| {
                    let cell = buffer.cell((x, y)).expect("inside the area");
                    (cell.symbol().to_string(), cell.modifier)
                })
                .collect()
        })
        .collect()
}

/// The symbols of a monochrome projection, joined into rows.
fn symbols(cells: &[Vec<(String, Modifier)>]) -> String {
    cells
        .iter()
        .map(|row| {
            row.iter()
                .map(|(symbol, _)| symbol.as_str())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// `REQ-MOK-019`: a world with no living Mokiterions, one with no standing resources, and one with
/// neither each draw a frame at every renderable viewport without panicking.
///
/// Extinction is reached through a run. A world stripped of every standing resource is not
/// reachable through one — rule 15 makes regeneration conditional on a remaining resource, and
/// neither shipped source consumes fast enough to empty a territory before the population dies —
/// so it is reached through the test hook, which is the only way it can be reached.
#[test]
fn a_degenerate_world_still_draws_a_frame() {
    let mut extinct = observer_for(&["--policy", "baseline", "--ticks", "400"]);
    while !extinct.is_finished() {
        extinct.advance().expect("the engine advances");
    }
    assert_eq!(
        extinct.termination_reason(),
        Some(TerminationReason::Extinction)
    );
    assert!(extinct.snapshot().agents.is_empty());
    assert!(
        !extinct.snapshot().resources.is_empty(),
        "extinction leaves the resources standing, which is what makes this the first case"
    );

    let mut populated = observer_for(&["--seed", "42", "--start-paused"]);
    for _ in 0..5 {
        populated.advance().expect("the engine advances");
    }
    assert_eq!(populated.snapshot().agents.len(), 12);
    let base = populated.snapshot().clone();

    let barren = |keep_agents: bool| {
        let mut snapshot = base.clone();
        if !keep_agents {
            snapshot.agents.clear();
            snapshot.living_count = 0;
        }
        snapshot.resources.clear();
        for territory in &mut snapshot.territories {
            territory.standing = 0;
            territory.low = 0;
            territory.medium = 0;
            territory.high = 0;
            territory.permanently_depleted = true;
        }
        snapshot
    };

    let mut no_resources = observer_for(&["--seed", "42", "--start-paused"]);
    no_resources.replace_snapshot_for_test(barren(true));
    let mut neither = observer_for(&["--seed", "42", "--start-paused"]);
    neither.replace_snapshot_for_test(barren(false));

    for (case, observer, depleted) in [
        ("no living Mokiterions", &mut extinct, false),
        ("no standing resources", &mut no_resources, true),
        ("neither", &mut neither, true),
    ] {
        for (width, height, _, _) in RENDERABLE {
            let buffer = frame(observer, width, height).expect("above the floor");
            assert!(
                !flatten(&buffer).trim().is_empty(),
                "{case} at {width}x{height} drew nothing"
            );
            // A degenerate world is still a reported world: the header and the footer stay.
            let panes = layout::resolve(*buffer.area());
            assert!(
                !region(&buffer, panes.header).trim().is_empty(),
                "{case} at {width}x{height} lost the header"
            );
            assert!(
                !region(&buffer, panes.footer).trim().is_empty(),
                "{case} at {width}x{height} lost the footer"
            );
        }
        if depleted {
            // Rule 3.2: a standing count of zero is stated as a state, never as a bare zero.
            let buffer = frame(observer, 160, 48).expect("above the floor");
            assert!(flatten(&buffer).contains("permanently depleted"), "{case}");
        }
    }
}

/// A snapshot carrying one resource of each class and two Mokiterions in one overview cell.
///
/// Both facts are needed to assert what overview zoom does and does not encode, and neither is
/// reliably present at a chosen tick of a real run: which classes fall inside a detail-zoom window
/// and whether two Mokiterions share a `2 x 4` block are properties of the seed, not obligations.
fn contrived(base: &WorldSnapshot) -> WorldSnapshot {
    let mut snapshot = base.clone();
    let at = |id: &str, x: u8, y: u8, territory: Territory| AgentSnapshot {
        id: id.to_string(),
        position: Coordinate { x, y },
        territory,
        health: 60,
        satiety: 50,
        energy: 40,
        applied_action: Some(Action::Wait),
    };
    // (0, 0) and (1, 0) fall in one 2 x 4 overview cell, so rule 2.4's shared cell is reached.
    snapshot.agents = vec![
        at("M01", 0, 0, Territory::A),
        at("M02", 1, 0, Territory::A),
        at("M03", 20, 100, Territory::B),
    ];
    snapshot.living_count = 3;

    let food = |id: &str, x: u8, class: FoodClass| ResourceSnapshot {
        id: id.to_string(),
        position: Coordinate { x, y: 4 },
        territory: Territory::A,
        class,
    };
    snapshot.resources = vec![
        food("F0001", 1, FoodClass::Low),
        food("F0002", 3, FoodClass::Medium),
        food("F0003", 5, FoodClass::High),
    ];

    let a = &mut snapshot.territories[0];
    a.standing = 3;
    a.low = 1;
    a.medium = 1;
    a.high = 1;
    a.permanently_depleted = false;
    let b = &mut snapshot.territories[1];
    b.standing = 0;
    b.low = 0;
    b.medium = 0;
    b.high = 0;
    b.permanently_depleted = true;
    snapshot
}

/// `REQ-MOK-019`: overview zoom encodes no per-resource class, and detail zoom does.
///
/// One braille dot has one state, so a class it appeared to carry would be an invention. The class
/// stays available from rule 3's per-territory counts, which is asserted in the same frame.
#[test]
fn overview_encodes_no_resource_class_and_detail_zoom_does() {
    let mut observer = observer_for(&["--seed", "42", "--start-paused"]);
    observer.advance().expect("the engine advances");
    let contrived = contrived(observer.snapshot());
    observer.replace_snapshot_for_test(contrived);

    const CLASSES: [char; 3] = ['\u{25CB}', '\u{25CE}', '\u{25CF}'];

    let buffer = frame(&mut observer, 160, 48).expect("above the floor");
    let canvas = region(&buffer, canvas_of(160, 48));
    for glyph in CLASSES {
        assert!(
            !canvas.contains(glyph),
            "overview zoom draws the class glyph {glyph}:\n{canvas}"
        );
    }
    // The class is still available, from the counts rather than from the canvas.
    let text = flatten(&buffer);
    assert!(text.contains("low 1"), "{text}");
    assert!(text.contains("medium 1"), "{text}");
    assert!(text.contains("high 1"), "{text}");

    tap(&mut observer, KeyCode::Char('z'));
    let buffer = frame(&mut observer, 160, 48).expect("above the floor");
    let canvas = region(&buffer, canvas_of(160, 48));
    for glyph in CLASSES {
        assert!(
            canvas.contains(glyph),
            "detail zoom omits the class glyph {glyph}:\n{canvas}"
        );
    }
}

/// `REQ-MOK-019`: every distinction is present in glyph, position or underline with all styling
/// removed.
///
/// The observer uses colour in four places — a Mokiterion's territory, a resource's class, a
/// resource dot against the territory rule, and a territory's depleted state. Each is asserted here
/// to have a carrier that is not colour, read from a projection of the frame that colour has been
/// discarded from.
#[test]
fn every_distinction_survives_the_loss_of_colour() {
    let mut observer = observer_for(&["--seed", "42", "--start-paused"]);
    observer.advance().expect("the engine advances");
    let contrived = contrived(observer.snapshot());
    observer.replace_snapshot_for_test(contrived);
    observer.select_for_test("M01");

    let buffer = frame(&mut observer, 160, 48).expect("above the floor");
    let whole = monochrome(&buffer, *buffer.area());
    let plain = symbols(&whole);

    // A Mokiterion's territory: the letter in its roster entry, not the entry's colour.
    assert!(plain.contains("M01  A"), "{plain}");
    assert!(plain.contains("M03  B"), "{plain}");

    // A territory's state: the word, not the note's colour.
    assert!(plain.contains("permanently depleted"), "{plain}");

    // Rule 2.4's shared cell: the underline, not a colour. M01 and M02 share one overview cell.
    let canvas = canvas_of(160, 48);
    let cells = monochrome(&buffer, canvas);
    let underlined: Vec<&(String, Modifier)> = cells
        .iter()
        .flatten()
        .filter(|(_, modifier)| modifier.contains(Modifier::UNDERLINED))
        .collect();
    assert_eq!(underlined.len(), 1, "the shared cell is not marked once");
    assert_eq!(underlined[0].0, spatial::agent_glyph("M01").to_string());

    // Rule 4.6's selection: reversed video, not a colour.
    let roster = layout::resolve(*buffer.area())
        .roster
        .expect("the reference viewport shows the roster");
    let reversed = monochrome(&buffer, roster)
        .iter()
        .flatten()
        .filter(|(_, modifier)| modifier.contains(Modifier::REVERSED))
        .count();
    assert!(reversed > 0, "the selected entry is not reversed");

    // The territory rule against a resource dot: position, not colour. The rule runs unbroken
    // across the world's whole width and a resource occupies one cell, so the two are
    // distinguishable with colour discarded. Which row it is, is
    // `render::tests::the_territory_rule_marks_the_row_between_the_territories`.
    let world_columns = spatial::WORLD_SIZE / spatial::DOTS_PER_CELL_X;
    let ruled: Vec<usize> = cells
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            row.iter().take_while(|(symbol, _)| symbol != " ").count() >= usize::from(world_columns)
        })
        .map(|(index, _)| index)
        .collect();
    assert_eq!(
        ruled.len(),
        1,
        "the territory rule is not exactly one unbroken row:\n{}",
        symbols(&cells)
    );

    // Rule 2's class glyphs in detail zoom: the glyph itself.
    tap(&mut observer, KeyCode::Char('z'));
    let buffer = frame(&mut observer, 160, 48).expect("above the floor");
    let detail = symbols(&monochrome(&buffer, canvas_of(160, 48)));
    for glyph in ['\u{25CB}', '\u{25CE}', '\u{25CF}'] {
        assert!(detail.contains(glyph), "{glyph} is not drawn:\n{detail}");
    }
}
