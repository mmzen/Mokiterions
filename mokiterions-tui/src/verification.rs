//! `VER-MOK-005`'s cross-cutting cases.
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

use std::collections::BTreeMap;

use mokiterions::simulation::{
    Action, AgentSnapshot, Coordinate, DecisionOutcome, DecisionSnapshot, Direction, EventType,
    FoodClass, Policy, ResourceSnapshot, Simulation, TerminationReason, Territory, WorldSnapshot,
};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier};

use crate::state::{Filter, Observer, Progression};
use crate::{authority, export, layout, options, render, spatial};

/// The declared verification seed set, fixed by `VER-MOK-005` so that observed runs are compared
/// against runs whose unobserved behavior is already recorded evidence.
const SEEDS: [u64; 5] = [0, 1, 42, 123, 777];

/// The declared viewport set. `33 x 21` is the one-below-floor case and is refused, not rendered.
const VIEWPORTS: [(u16, u16); 7] = [
    (160, 48),
    (160, 44),
    (140, 44),
    (120, 48),
    (100, 30),
    (34, 22),
    (33, 21),
];

/// The declared viewports above the floor, each with the canvas interior rule 5 derives for it.
const RENDERABLE: [(u16, u16, u16, u16); 6] = [
    (160, 48, 67, 32),
    (160, 44, 67, 32),
    (140, 44, 47, 32),
    (120, 48, 71, 36),
    (100, 30, 98, 24),
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

/// The engine binary's stream for one configuration: its event lines and its summary line.
///
/// This is the authoritative reference. Nothing in it comes from the observer.
fn unobserved(args: &[&str]) -> (Vec<String>, String) {
    let config = match options::parse(args.to_vec()) {
        Ok(options::Startup::Run(options)) => options.config,
        other => panic!("{args:?} did not yield a runnable configuration: {other:?}"),
    };
    let mut simulation = Simulation::new(config).expect("the configuration is valid");
    let mut bytes = Vec::new();
    simulation.run(&mut bytes).expect("writing to a vector");
    let text = String::from_utf8(bytes).expect("the stream is text");

    let mut lines: Vec<String> = text.lines().map(str::to_string).collect();
    let summary = lines.pop().expect("the stream ends with a summary");
    assert!(summary.starts_with("summary "), "{summary}");
    (lines, summary)
}

/// The observer's retained records as stream lines, which is what an export contains.
fn observed_lines(observer: &Observer) -> Vec<String> {
    observer
        .events()
        .iter()
        .map(|event| event.to_string())
        .collect()
}

/// The summary line the observer's final state implies, in the engine binary's own words.
///
/// Reconstructing it from the snapshot rather than reading it from the engine is deliberate: it
/// asserts that the observer's picture of the final state is the engine's, field by field.
fn summary_from(observer: &Observer) -> String {
    let snapshot = observer.snapshot();
    let survivors = snapshot.agents.len();
    let territory_a = snapshot
        .agents
        .iter()
        .filter(|agent| agent.territory == Territory::A)
        .count();
    let [a, b] = &snapshot.territories;
    format!(
        "summary reason={} ticks={} survivors={survivors} deaths={} territory_a={territory_a} \
         territory_b={} food_a_low={} food_a_medium={} food_a_high={} food_b_low={} \
         food_b_medium={} food_b_high={}",
        observer.termination_reason().expect("the run has ended"),
        snapshot.tick,
        snapshot.deaths,
        survivors - territory_a,
        a.low,
        a.medium,
        a.high,
        b.low,
        b.medium,
        b.high,
    )
}

/// Groups stream lines by the tick each belongs to.
fn by_tick(lines: &[String]) -> BTreeMap<u64, Vec<&String>> {
    let mut grouped: BTreeMap<u64, Vec<&String>> = BTreeMap::new();
    for line in lines {
        let tick = line
            .strip_prefix("tick=")
            .and_then(|rest| rest.split(' ').next())
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or_else(|| panic!("no tick in {line}"));
        grouped.entry(tick).or_default().push(line);
    }
    grouped
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

/// Everything the operator can reach that must not perturb the run, applied every tick.
///
/// The sequence deliberately mixes progression, selection, panning, zooming, filtering, overlays,
/// log paging and an unbound key, and it is applied while the run is advancing rather than between
/// runs, because `REQ-MOK-025`'s claim is about interaction during a run.
fn interact(observer: &mut Observer, round: u64) {
    const KEYS: [KeyCode; 12] = [
        KeyCode::Tab,
        KeyCode::Char('z'),
        KeyCode::Char('l'),
        KeyCode::Char('j'),
        KeyCode::Char('e'),
        KeyCode::Char('f'),
        KeyCode::BackTab,
        KeyCode::Char('L'),
        KeyCode::PageUp,
        KeyCode::Char('c'),
        KeyCode::Esc,
        // Unbound: it must change nothing at all.
        KeyCode::Char('W'),
    ];
    let step = usize::try_from(round % 12).expect("a small remainder");
    tap(observer, KEYS[step]);
    // Speed is walked too, since it is what a host would otherwise use to schedule ticks.
    if step % 3 == 0 {
        tap(observer, KeyCode::Char('+'));
    }
    if step % 5 == 0 {
        tap(observer, KeyCode::Char('-'));
    }
}

/// Drives an observed run to the engine's own end, interacting and drawing throughout.
///
/// The viewport rotates through the declared set, including below the floor, so a run is observed
/// at every layout tier and while drawing is suspended.
fn observed_run(args: &[&str]) -> Observer {
    let mut observer = observer_for(args);
    let mut round = 0u64;
    while !observer.is_finished() {
        let (width, height) = VIEWPORTS[usize::try_from(round % 7).expect("a small remainder")];
        frame(&mut observer, width, height);
        interact(&mut observer, round);
        observer.advance().expect("the engine advances");
        round += 1;
    }
    // A finished run is still drawn and still interacted with.
    frame(&mut observer, 160, 48);
    interact(&mut observer, round);
    observer
}

// ---- the primary property ------------------------------------------------------------------

/// `REQ-MOK-025`: on every declared seed, an observed run's authoritative stream and final state
/// are the engine binary's, byte for byte, under interaction throughout.
#[test]
fn observed_and_unobserved_runs_are_identical_on_every_declared_seed() {
    for seed in SEEDS {
        let seed = seed.to_string();
        let args = ["--seed", &seed, "--ticks", "60"];
        let (expected, summary) = unobserved(&args);
        let observer = observed_run(&args);
        let observed = observed_lines(&observer);

        assert_eq!(
            observed.len(),
            expected.len(),
            "seed {seed}: {} records observed, {} unobserved",
            observed.len(),
            expected.len()
        );
        for (index, (left, right)) in observed.iter().zip(expected.iter()).enumerate() {
            assert_eq!(left, right, "seed {seed} record {index}");
        }
        assert_eq!(summary_from(&observer), summary, "seed {seed} final state");
    }
}

/// `REQ-MOK-025`: per-tick entropy draw counts are identical observed and unobserved.
///
/// The specified observation surface exposes no draw counter, so the comparison is made on what
/// the stream shows of every entropy-consuming operation: each regenerated resource's placement,
/// each regeneration the engine skipped, and each applied action, tick by tick. The engine's
/// entropy is one sequential stream shared by placement and by the decision source, so a tick in
/// which the observer had drawn even once would shift every later value; per-tick record identity
/// across a whole run is therefore the observable form of draw-count identity.
#[test]
fn per_tick_records_match_so_the_observer_draws_no_entropy() {
    fn entropy_bearing(line: &str) -> bool {
        [
            "event=food_regenerated",
            "event=food_regeneration_skipped",
            "event=action_trace",
        ]
        .iter()
        .any(|marker| line.contains(marker))
    }

    for seed in SEEDS {
        let seed = seed.to_string();
        let args = ["--seed", &seed, "--ticks", "40"];
        let (expected, _) = unobserved(&args);
        let observer = observed_run(&args);
        let observed = observed_lines(&observer);

        let mine = by_tick(&observed);
        let theirs = by_tick(&expected);
        assert_eq!(
            mine.keys().collect::<Vec<_>>(),
            theirs.keys().collect::<Vec<_>>(),
            "seed {seed}: the runs cover different ticks"
        );
        for (tick, lines) in &mine {
            let left: Vec<&&String> = lines.iter().filter(|line| entropy_bearing(line)).collect();
            let right: Vec<&&String> = theirs[tick]
                .iter()
                .filter(|line| entropy_bearing(line))
                .collect();
            assert_eq!(
                left.len(),
                right.len(),
                "seed {seed} tick {tick}: {} entropy-bearing records observed, {} unobserved",
                left.len(),
                right.len()
            );
            assert_eq!(left, right, "seed {seed} tick {tick}");
        }
    }
}

/// `REQ-MOK-025`: held across many frames and many key presses, nothing advances.
#[test]
fn holding_consumes_nothing_however_long_it_is_held() {
    let mut observer = observer_for(&["--start-paused", "--seed", "42"]);
    let before = observer.snapshot().clone();
    let retained = observer.events().len();

    for round in 0..200u64 {
        frame(&mut observer, 160, 48);
        interact(&mut observer, round);
    }
    assert_eq!(observer.snapshot(), &before, "held state changed");
    assert_eq!(
        observer.events().len(),
        retained,
        "records accrued while held"
    );

    // Advancing once then produces exactly the engine's own first tick, unaffected by all of it.
    let (expected, _) = unobserved(&["--seed", "42"]);
    observer.advance().expect("the engine advances");
    assert_eq!(observer.snapshot().tick, 1);
    let observed = observed_lines(&observer);
    let grouped = by_tick(&expected);
    assert_eq!(observed.len(), grouped[&0].len() + grouped[&1].len());
    for (index, (left, right)) in observed.iter().zip(expected.iter()).enumerate() {
        assert_eq!(left, right, "record {index}");
    }
}

/// `REQ-MOK-025`: a run the operator ended yields a prefix of the unobserved run.
#[test]
fn an_operator_ended_run_is_a_prefix_of_the_unobserved_run() {
    let args = ["--seed", "123", "--ticks", "80"];
    let (expected, _) = unobserved(&args);
    let mut observer = observer_for(&args);
    for round in 0..25u64 {
        frame(&mut observer, 120, 48);
        interact(&mut observer, round);
        observer.advance().expect("the engine advances");
    }
    observer.mark_ended_early();

    let observed = observed_lines(&observer);
    assert!(
        observed.len() < expected.len(),
        "an early exit retained the whole run"
    );
    assert_eq!(observed, expected[..observed.len()], "not a prefix");
    assert!(observer.ended_early());
    assert!(
        !observer.is_finished(),
        "an operator's exit is not the engine's end"
    );
}

/// `REQ-MOK-025`: no catch-up. One advance is one tick, and a finished run refuses to advance.
#[test]
fn one_advance_is_one_tick_and_a_finished_run_refuses() {
    let mut observer = observer_for(&["--start-paused", "--ticks", "10"]);
    for expected in 1..=10u64 {
        observer.advance().expect("the engine advances");
        assert_eq!(observer.snapshot().tick, expected);
    }
    assert!(observer.is_finished());
    for _ in 0..5 {
        observer.advance().expect("a refusal is not an error");
        assert_eq!(observer.snapshot().tick, 10);
    }
    assert_eq!(
        observer.termination_reason(),
        Some(TerminationReason::TickLimit)
    );
}

/// `REQ-MOK-023`: a finished run stays inspectable and exportable, and refuses progression.
#[test]
fn a_finished_run_stays_inspectable_and_exportable() {
    let mut observer = observer_for(&["--ticks", "20"]);
    while !observer.is_finished() {
        observer.advance().expect("the engine advances");
    }
    let retained = observer.events().len();

    tap(&mut observer, KeyCode::Char(' '));
    tap(&mut observer, KeyCode::Char('.'));
    assert_eq!(observer.snapshot().tick, 20);
    assert_eq!(observer.events().len(), retained);

    let buffer = frame(&mut observer, 160, 48).expect("above the floor");
    assert!(
        flatten(&buffer).contains("tick_limit"),
        "the engine's reason is not presented"
    );

    let mut bytes = Vec::new();
    export::write_records(&mut bytes, observer.events()).expect("writing to a vector");
    let text = String::from_utf8(bytes).expect("the export is text");
    assert_eq!(
        text.lines().count(),
        retained + 1,
        "every record plus the trailer"
    );
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

/// `REQ-MOK-024`: layout is a function of dimensions alone.
///
/// The same dimensions are resolved at different ticks, speeds, selections, filters, overlays and
/// run states; every pane must land in the same place.
#[test]
fn layout_reads_nothing_but_the_dimensions() {
    let mut observer = observer_for(&["--seed", "1", "--start-paused"]);
    let reference: Vec<_> = VIEWPORTS
        .iter()
        .map(|(width, height)| layout::resolve(Rect::new(0, 0, *width, *height)))
        .collect();

    for round in 0..30u64 {
        observer.advance().expect("the engine advances");
        interact(&mut observer, round);
        for (index, (width, height)) in VIEWPORTS.iter().enumerate() {
            assert_eq!(
                layout::resolve(Rect::new(0, 0, *width, *height)),
                reference[index],
                "{width}x{height} at tick {}",
                observer.snapshot().tick
            );
        }
    }
}

/// `REQ-MOK-024`: the canvas interior at every declared viewport is the derived one, the header
/// and footer are present at every one of them including the floor itself, and the one-below-floor
/// case presents nothing.
#[test]
fn every_declared_viewport_has_its_derived_canvas_with_a_header_and_a_footer() {
    let mut observer = observer_for(&["--seed", "42", "--start-paused"]);
    observer.advance().expect("the engine advances");

    for (width, height, canvas_width, canvas_height) in RENDERABLE {
        let panes = layout::resolve(Rect::new(0, 0, width, height));
        assert_eq!(
            layout::canvas_cells(panes.view),
            (canvas_width, canvas_height),
            "{width}x{height}"
        );
        assert_eq!(panes.header.height, 3, "{width}x{height}");
        assert_eq!(panes.footer.height, 1, "{width}x{height}");

        let buffer = frame(&mut observer, width, height).expect("above the floor");
        let text = flatten(&buffer);
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), usize::from(height), "{width}x{height}");
        // The header states the run and both territories; the footer states the provenance.
        assert!(lines[0].contains("HELD"), "{width}x{height}: {}", lines[0]);
        assert!(lines[1].starts_with('A'), "{width}x{height}: {}", lines[1]);
        assert!(lines[2].starts_with('B'), "{width}x{height}: {}", lines[2]);
        let footer = lines[lines.len() - 1];
        assert!(
            footer.contains("42") && footer.contains("reference"),
            "{width}x{height}: {footer}"
        );
    }

    assert!(frame(&mut observer, 33, 21).is_none());
}

/// `VER-MOK-005`: orientation. Within a visible region, a smaller world row never renders below a
/// larger one, at any declared viewport, either zoom, or any camera position.
#[test]
fn a_smaller_world_row_never_renders_below_a_larger_one() {
    let mut observer = observer_for(&["--seed", "0", "--start-paused"]);
    observer.advance().expect("the engine advances");

    for (width, height, _, _) in RENDERABLE {
        for zoom_presses in 0..2 {
            for pans in [0usize, 5, 40, 200] {
                for _ in 0..zoom_presses {
                    tap(&mut observer, KeyCode::Char('z'));
                }
                for _ in 0..pans {
                    tap(&mut observer, KeyCode::Char('j'));
                }
                frame(&mut observer, width, height);

                let viewport = observer.viewport();
                let zoom = observer.zoom();
                let mut previous: Option<u16> = None;
                for world_y in viewport.origin_y..=viewport.last_y() {
                    let row = viewport
                        .cell_of(zoom, viewport.origin_x, world_y)
                        .expect("inside the region")
                        .1;
                    if let Some(previous) = previous {
                        assert!(
                            row >= previous,
                            "{width}x{height}: world row {world_y} rose above its predecessor"
                        );
                    }
                    previous = Some(row);
                }
                // Return the camera and the zoom for the next case.
                for _ in 0..200 {
                    tap(&mut observer, KeyCode::Char('k'));
                }
                for _ in 0..zoom_presses {
                    tap(&mut observer, KeyCode::Char('z'));
                }
            }
        }
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

/// `REQ-MOK-022`: two exports from runs sharing seed, configuration, source and stopping tick are
/// byte-identical, and each is the engine binary's stream with the trailer appended.
#[test]
fn exports_are_reproducible_and_are_the_engines_own_records() {
    fn rendered(observer: &Observer) -> String {
        let mut bytes = Vec::new();
        export::write_records(&mut bytes, observer.events()).expect("writing to a vector");
        String::from_utf8(bytes).expect("the export is text")
    }

    for seed in SEEDS {
        let seed = seed.to_string();
        let args = ["--seed", &seed, "--ticks", "30"];
        let (expected, _) = unobserved(&args);

        // One run interacted with heavily, one run left alone: the exports must agree.
        let busy = observed_run(&args);
        let mut quiet = observer_for(&args);
        while !quiet.is_finished() {
            quiet.advance().expect("the engine advances");
        }

        let first = rendered(&busy);
        assert_eq!(first, rendered(&quiet), "seed {seed}");
        assert_eq!(first, rendered(&busy), "seed {seed} is not stable");

        let lines: Vec<&str> = first.lines().collect();
        assert_eq!(&lines[..lines.len() - 1], &expected[..], "seed {seed}");
        assert_eq!(
            lines[lines.len() - 1],
            format!("# retained={} truncated=false", expected.len()),
            "seed {seed}"
        );
    }
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

/// `REQ-MOK-019`: an action the engine applied is the action presented, for every living
/// Mokiterion, at every tick of a short run.
#[test]
fn the_applied_action_presented_is_always_the_engines() {
    let mut observer = observer_for(&["--seed", "1", "--start-paused"]);
    for _ in 0..15 {
        observer.advance().expect("the engine advances");
        let buffer = frame(&mut observer, 160, 48).expect("above the floor");
        let text = flatten(&buffer);
        let applied: Vec<(String, Option<Action>)> = observer
            .snapshot()
            .agents
            .iter()
            .map(|agent| (agent.id.clone(), agent.applied_action.clone()))
            .collect();
        for (id, action) in applied {
            if let Some(action) = action {
                let rendered = action.to_string();
                assert!(
                    text.contains(&rendered),
                    "{id}'s applied action {rendered} is not presented"
                );
            }
        }
    }
}

/// Neither shipped decision source can have a proposal rejected.
///
/// `BaselineDecisionSource` selects from the observation's valid actions and
/// `ReferenceDecisionSource` guards every candidate with the observation's own `allows`, so both
/// propose only what the engine has already declared valid. A rejection is therefore unreachable
/// through a run of either policy, which is asserted here as the fact it is: `VER-MOK-005`'s
/// acceptance scenario 2 describes a state no shipped source produces, and the case that follows
/// reaches it the only way it can be reached.
#[test]
fn no_shipped_decision_source_has_a_proposal_rejected() {
    for policy in ["baseline", "reference"] {
        let mut observer = observer_for(&["--policy", policy, "--seed", "42", "--ticks", "400"]);
        while !observer.is_finished() {
            observer.advance().expect("the engine advances");
            for decision in &observer.snapshot().decisions {
                assert_eq!(
                    decision.outcome,
                    DecisionOutcome::Accepted,
                    "{policy} had {} rejected at tick {}",
                    decision.agent_id,
                    observer.snapshot().tick
                );
            }
        }
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

// ---- security ------------------------------------------------------------------------------

/// `REQ-MOK-027`: no frame carries an environment value, at any declared viewport, at any tick.
#[test]
fn no_frame_carries_an_environment_value() {
    let mut observer = observer_for(&["--seed", "777"]);
    for round in 0..40u64 {
        observer.advance().expect("the engine advances");
        interact(&mut observer, round);
        for (width, height, _, _) in RENDERABLE {
            let buffer = frame(&mut observer, width, height).expect("above the floor");
            let text = flatten(&buffer);
            for forbidden in [
                "C:\\",
                "/home/",
                "/Users/",
                "AppData",
                "PATH=",
                "TEMP",
                "token",
                "secret",
                "api_key",
                "ANTHROPIC",
            ] {
                assert!(
                    !text.contains(forbidden),
                    "{width}x{height} carries {forbidden}"
                );
            }
        }
    }
}

/// `REQ-MOK-025`: an injected export failure leaves the run running and no tick partly applied.
#[test]
fn an_injected_export_failure_leaves_the_tick_intact() {
    let directory = std::env::temp_dir().join("mokiterions-verification-absent");
    let _ = std::fs::remove_dir_all(&directory);
    let unwritable = directory.join("nested").join("events.log");
    let unwritable = unwritable.to_str().expect("a text path").to_string();

    let mut observer = observer_for(&["--start-paused", "--export", &unwritable, "--seed", "42"]);
    for _ in 0..5 {
        observer.advance().expect("the engine advances");
    }
    let before = observer.snapshot().clone();
    let records = observed_lines(&observer);

    tap(&mut observer, KeyCode::Char('x'));
    assert!(
        observer
            .notice()
            .is_some_and(|text| text.starts_with("export failed")),
        "the failure is not reported: {:?}",
        observer.notice()
    );
    assert_eq!(observer.snapshot(), &before);
    assert_eq!(observed_lines(&observer), records);
    assert!(!std::fs::exists(&unwritable).unwrap_or(false));

    // The run continues from exactly where it was.
    observer.advance().expect("the engine advances");
    assert_eq!(observer.snapshot().tick, 6);
}

// ---- the declared sets ---------------------------------------------------------------------

/// Guards the declared sets themselves, so a later edit cannot quietly shrink coverage.
#[test]
fn the_declared_sets_are_the_contracts() {
    assert_eq!(SEEDS, [0, 1, 42, 123, 777]);
    assert_eq!(VIEWPORTS.len(), 7);
    assert_eq!(RENDERABLE.len(), 6);
    assert!(VIEWPORTS.contains(&(33, 21)));
    assert!(layout::below_floor(33, 21));
    for (width, height, _, _) in RENDERABLE {
        assert!(VIEWPORTS.contains(&(width, height)));
        assert!(!layout::below_floor(width, height));
    }
    // Rule 11's mapping covers every event type the observer can retain.
    for event_type in EventType::ALL {
        assert!(
            authority::for_type(event_type, Some(Policy::Reference)).is_some(),
            "{event_type} has no authority"
        );
    }
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

/// `REQ-MOK-019` and `REQ-MOK-020`: a dead Mokiterion leaves the presentation, and the death is
/// corroborated by the counts on the tick it is applied.
///
/// The engine's snapshot carries living Mokiterions only, so this cannot be satisfied by filtering
/// in the observer. The case is here to assert the consequence at the buffer, which is that no
/// identifier and no glyph for the dead subject survives where the living are presented.
#[test]
fn a_death_removes_the_subject_from_the_presentation_and_is_corroborated() {
    let mut observer = observer_for(&["--policy", "baseline", "--ticks", "400", "--start-paused"]);
    let mut previous = observer.snapshot().clone();
    let dead = loop {
        assert!(!observer.is_finished(), "the run ended before any death");
        observer.advance().expect("the engine advances");
        let now = observer.snapshot().clone();
        if now.deaths > previous.deaths {
            let gone: Vec<String> = previous
                .agents
                .iter()
                .filter(|was| !now.agents.iter().any(|is| is.id == was.id))
                .map(|was| was.id.clone())
                .collect();
            // The counts corroborate each other: the entries that disappeared are exactly the
            // deaths the engine reported, and the living count fell by the same number.
            assert_eq!(gone.len(), now.deaths - previous.deaths);
            assert_eq!(now.living_count, previous.living_count - gone.len());
            assert_eq!(now.living_count, now.agents.len());
            break gone[0].clone();
        }
        previous = now;
    };

    let buffer = frame(&mut observer, 160, 48).expect("above the floor");
    let roster = layout::resolve(*buffer.area())
        .roster
        .expect("the reference viewport shows the roster");
    assert!(
        !region(&buffer, roster).contains(&dead),
        "{dead} is still in the roster:\n{}",
        region(&buffer, roster)
    );

    // Rule 2's glyphs are one per identifier, so the dead subject's glyph cannot be another's.
    let glyph = spatial::agent_glyph(&dead);
    for zoom in ["overview", "detail"] {
        let buffer = frame(&mut observer, 160, 48).expect("above the floor");
        let canvas = region(&buffer, canvas_of(160, 48));
        assert!(
            !canvas.contains(glyph),
            "{dead}'s glyph {glyph} is still drawn in {zoom} zoom:\n{canvas}"
        );
        tap(&mut observer, KeyCode::Char('z'));
    }
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
