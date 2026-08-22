//! `VER-MOK-005`'s cross-cutting cases: the public tier.
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
//! `SPEC-MOK-004` rule 9 places these cases here and rule 10 keeps eight of them in
//! `src/verification.rs`, because those eight reach a `#[cfg(test)]` hook that no test outside
//! the crate can link. The two files carry the same documentation, the same declared sets and
//! the same helpers, and every assertion is verbatim what it was.

use std::collections::BTreeMap;

use mokiterions::simulation::{
    Action, DecisionOutcome, EventType, Policy, Simulation, TerminationReason, Territory,
};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::style::Modifier;

// `crate::` used to reach these. A test tier outside the crate names them through the public
// interface instead; nothing else about the cases changes.
use mokiterions_tui::state::Observer;
use mokiterions_tui::{authority, export, layout, options, render, spatial};

/// The declared verification seed set, fixed by `VER-MOK-005` so that observed runs are compared
/// against runs whose unobserved behavior is already recorded evidence.
const SEEDS: [u64; 5] = [0, 1, 42, 123, 777];

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
///
/// The reference row reads a 36-row canvas as of rule 5's 2026-08-20 amendment, which holds the log
/// at six rows everywhere and returns to the body the four the withdrawn ten-row growth took. It is
/// the only row the amendment moves: every other viewport here either failed one of the two
/// conditions that growth required or has no log at all.
const RENDERABLE: [(u16, u16, u16, u16); 9] = [
    (160, 48, 67, 36),
    (160, 44, 67, 32),
    (160, 40, 67, 28),
    (140, 44, 47, 32),
    (140, 43, 47, 31),
    (120, 48, 71, 36),
    (120, 30, 71, 24),
    (100, 30, 51, 24),
    (34, 22, 32, 16),
];

/// The declared footer seed set, fixed by `VER-MOK-005` so that rule 8's order of loss is measured
/// across the whole accepted `u64` range rather than on small seeds only.
///
/// It carries both ends of the range and one seed of every decimal magnitude the range admits,
/// because the row's width is decided by the seed's digit count and by nothing else about its value.
/// The defect `WO-MOK-008` closes was reachable from 13 digits upward and no seed in `SEEDS` has
/// more than three.
const FOOTER_SEEDS: [u64; 22] = [
    0,
    1,
    9,
    99,
    999,
    9_999,
    99_999,
    999_999,
    9_999_999,
    99_999_999,
    999_999_999,
    9_999_999_999,
    99_999_999_999,
    999_999_999_999,
    9_999_999_999_999,
    99_999_999_999_999,
    999_999_999_999_999,
    9_999_999_999_999_999,
    99_999_999_999_999_999,
    999_999_999_999_999_999,
    9_999_999_999_999_999_999,
    u64::MAX,
];

fn observer_for(args: &[&str]) -> Observer {
    match options::parse(args.to_vec()) {
        Ok(options::Startup::Run(options)) => {
            Observer::new(options).expect("the configuration is valid")
        }
        other => panic!("{args:?} did not yield a runnable configuration: {other:?}"),
    }
}

/// The name the engine reported for each identifier, read from the retained records.
///
/// `REQ-MOK-041` forbids the observer from holding a name table or deriving a name from an
/// identifier, so the expectation a frame is checked against is read from the engine's own
/// `agent_initialized` record — the same source the observer reads, reached here through the
/// already-public `events()` accessor. No name is written as a literal anywhere in this package's
/// presentation path or in this file.
fn reported_names(observer: &Observer) -> BTreeMap<String, String> {
    observer
        .events()
        .iter()
        .filter(|event| event.event_type() == EventType::AgentInitialized)
        .map(|event| {
            let details = event.detail.to_string();
            let name = details
                .strip_prefix("name:")
                .expect("the name is the first detail")
                .split_once(',')
                .expect("a field is followed by the next")
                .0
                .to_string();
            (event.subject.clone(), name)
        })
        .collect()
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

/// The bottommost row of a frame, which rule 5 fixes as the provenance footer.
fn footer_of(buffer: &Buffer) -> String {
    flatten(buffer)
        .lines()
        .last()
        .expect("a frame has rows")
        .trim_end()
        .to_string()
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
/// at every combination of panes rule 5 admits and while drawing is suspended.
fn observed_run(args: &[&str]) -> Observer {
    let mut observer = observer_for(args);
    let mut round = 0u64;
    while !observer.is_finished() {
        let index = usize::try_from(round).expect("a tick count fits a usize") % VIEWPORTS.len();
        let (width, height) = VIEWPORTS[index];
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

        // The reported symptom was a pane missing from the screen, so what is drawn is checked to
        // agree with what was resolved. This does not check the thresholds themselves — it
        // compares the frame against the layout, and `tests/layout.rs` compares the layout against
        // rule 5 — but it is what fails if a pane is resolved and then not drawn. The body's first
        // row carries the roster's and the inspector's titles and the log's title is on its own
        // first row; the header is excluded, because it names a pane precisely when it is absent.
        let body_top = lines[3];
        assert_eq!(
            body_top.contains("roster"),
            panes.roster.is_some(),
            "{width}x{height}: {body_top}"
        );
        assert_eq!(
            body_top.contains("inspector"),
            panes.inspector.is_some(),
            "{width}x{height}: {body_top}"
        );
        if let Some(log) = panes.log {
            let log_top = lines[usize::from(log.y)];
            assert!(log_top.contains("log"), "{width}x{height}: {log_top}");
        }
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

// ---- records, filters and the export -------------------------------------------------------

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

/// No decision source that proposes only from the observation's valid-action list can have a
/// proposal rejected.
///
/// `BaselineDecisionSource` selects from the observation's valid actions and
/// `ReferenceDecisionSource` guards every candidate with the observation's own `allows`, so both
/// propose only what the engine has already declared valid. A rejection is therefore unreachable
/// through a run of either policy, which is asserted here as the fact it is: `VER-MOK-005`'s
/// acceptance scenario 2 describes a state no shipped source produces, and the case that follows
/// reaches it the only way it can be reached.
///
/// `WO-MOK-010` added a third shipped source and extended the sweep below to it rather than
/// leaving the name of this case broader than what it checked. `IndividualDecisionSource` screens
/// its candidates through the same `allows`, so the claim holds for the same reason; `VER-MOK-010`
/// requires it as *validation is not relaxed*. No assertion here was changed to admit it.
///
/// `WO-MOK-016` added a fourth shipped source that **is** rejected, and the name of this case moved
/// rather than the sweep, for the same reason it moved under `WO-MOK-010`: the claim now states the
/// property the three share instead of a count of what ships. `SocialDecisionSource` proposes
/// targeted actions, which `SPEC-MOK-001` rule 3 keeps off the valid-action list on purpose, so
/// `allows` cannot screen them and rule 6 is the only gate. Rule 26's own text fixes that its
/// branch 1 proposes an answer "whether or not that answer can succeed", and rule 6's fifth
/// condition rejects a targeted move with no valid direction — both are specified behavior, and
/// `the_social_source_is_rejected_only_as_the_specification_admits` below asserts which grounds are
/// reachable rather than that none is. **No assertion in this case was relaxed, widened or removed**:
/// the three policies below and their per-decision equality are verbatim.
#[test]
fn no_source_confined_to_the_valid_action_list_has_a_proposal_rejected() {
    for policy in ["baseline", "reference", "individual"] {
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

/// `SPEC-MOK-003` rule 10 as amended: a rejection under the `social` source is presented as the
/// engine's own ground, and the grounds reachable are the ones `SPEC-MOK-001` rule 6 names.
///
/// This is the counterpart of the case above rather than a relaxation of it. A rejection here is an
/// expected outcome of the authority boundary, so what is asserted is that the observer presents the
/// engine's word for it and invents nothing — the reason it presents is one of rule 6's, never a
/// phrase of the observer's own, and never a fault or a warning.
#[test]
fn the_social_source_is_rejected_only_as_the_specification_admits() {
    // Rule 6's five conditions, in the order that rule fixes them, plus rule 8's own reason
    // reached by a targeted verb. Nothing outside this set is a ground the engine can state.
    const GROUNDS: [&str; 9] = [
        "agent_dead",
        "target_unknown",
        "target_dead",
        "target_is_actor",
        "target_not_perceived",
        "target_not_in_contact",
        "target_not_in_record",
        "target_co_located",
        "out_of_bounds",
    ];

    let mut rejections = 0usize;
    for seed in ["0", "42", "123"] {
        let mut observer = observer_for(&["--policy", "social", "--seed", seed, "--ticks", "400"]);
        while !observer.is_finished() {
            observer.advance().expect("the engine advances");
            for decision in &observer.snapshot().decisions {
                if let DecisionOutcome::Rejected { ground } = &decision.outcome {
                    rejections += 1;
                    assert!(
                        GROUNDS.contains(&ground.as_str()),
                        "seed {seed} tick {} rejected {} on the unnamed ground {ground}",
                        observer.snapshot().tick,
                        decision.agent_id
                    );
                }
            }
        }
    }

    // The measured figure across these three seeds is one rejection, at seed 0 tick 11: an `avoid`
    // whose only away-axis left the world. It is asserted as a bound rather than an equality, since
    // rule 26 makes rejection rare but not impossible and pinning the count would make this case a
    // second capture rather than a claim about grounds.
    assert!(rejections <= 8, "{rejections} rejections is not rare");
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

// ---- rule 8's order of loss, at rendered frames ---------------------------------------------

/// `REQ-MOK-024` and `REQ-MOK-027` through `SPEC-MOK-003` rule 8 clause 7: at every declared
/// viewport, for every declared footer seed, the row fits its pane, presents the entropy seed whole,
/// and presents no digit run that is not a value of the run.
///
/// The digit-run assertion is the one that fails on the defect this closes. At `34 x 22` with a
/// 20-digit seed the superseded renderer drew a row 51 characters long into a 34-column pane, and
/// the pane kept the first 34 — so an operator read a tick limit of `18446744073` where the run's
/// was `18446744073709551615`. That is a value no field of the run holds, which is what this
/// detects; a merely shorter row would not have been a defect at all.
#[test]
fn the_footer_never_clips_a_value_at_any_declared_viewport() {
    // The row's width is decided by the digit counts of the entropy seed and the configured tick
    // limit *together*, so the declared seeds are crossed with both ends of the tick limit's range.
    //
    // Sweeping the seed alone does not reach the defect, and a sweep that does not reach it is not
    // a regression case however wide it looks: at the floor a 20-digit seed beside the default
    // limit of 100 came to 34 characters exactly under the superseded renderer, so nothing was cut.
    // A 20-digit seed beside a 20-digit limit came to 51, and the pane kept 34.
    const MAX: &str = "18446744073709551615";
    for seed in FOOTER_SEEDS {
        let seed_text = seed.to_string();
        for ticks in [None, Some(MAX)] {
            let mut arguments = vec!["--seed", &seed_text];
            if let Some(ticks) = ticks {
                arguments.extend(["--ticks", ticks]);
            }
            arguments.push("--start-paused");
            for (width, height, _, _) in RENDERABLE {
                let mut observer = observer_for(&arguments);
                let buffer = frame(&mut observer, width, height).expect("above the floor, a frame");
                let row = footer_of(&buffer);
                let case = format!(
                    "seed {seed}, ticks {} at {width}x{height}",
                    ticks.unwrap_or("100")
                );

                assert!(
                    row.chars().count() <= usize::from(width),
                    "{case}: the row overflows its pane: {row:?}"
                );
                assert!(
                    row.contains(&seed_text),
                    "{case}: the seed is gone from {row:?}"
                );

                // Every value this run carries, in the only radix rule 8 clause 8 admits. `0` and
                // `75` are the default density's two digit runs, and `0` is also the current tick,
                // the run being paused at its start.
                let admissible = [
                    seed_text.clone(),
                    ticks.unwrap_or("100").to_string(),
                    "0".to_string(),
                    "75".to_string(),
                    observer.events().len().to_string(),
                ];
                for run in row
                    .split(|character: char| !character.is_ascii_digit())
                    .filter(|run| !run.is_empty())
                {
                    assert!(
                        admissible.contains(&run.to_string()),
                        "{case}: {row:?} presents {run:?}, which is no value of this run"
                    );
                }
            }
        }
    }
}

/// Rule 8 clause 4's order of loss, read off rendered frames at the floor, character for character.
///
/// These four rows are the measurement the technical owner was shown when the order was fixed on
/// 2026-08-22. The second is the case the order decides: a 20-digit seed leaves room for exactly one
/// of the density and the active decision source, and clause 4 keeps the density. The fourth is
/// clause 6's case, where the seed is all that fits and the row says so rather than cutting the tick
/// limit.
#[test]
fn the_footer_sheds_the_specified_fields_at_the_floor() {
    const MAX: &str = "18446744073709551615";
    let cases: [(&[&str], &str); 4] = [
        // Nothing shed: every field rule 8 requires fits at the floor.
        (&["--seed", "0"], "s0 t100 d0.75% reference @0 e{e}"),
        // Shed the event count, the current tick, then the active source. Density survives.
        (&["--seed", MAX], "s18446744073709551615 t100 d0.75%"),
        // Shed the event count and the current tick. Both density and the source survive.
        (
            &["--seed", "0", "--ticks", MAX],
            "s0 t18446744073709551615 d0.75% r",
        ),
        // Clause 6: 21 columns of seed beside 22 of tick limit is 43 of 34, so the tick limit goes
        // too and the seed is presented in the widest labelling the remaining width holds.
        (
            &["--seed", MAX, "--ticks", MAX],
            "seed 18446744073709551615",
        ),
    ];
    for (args, expected) in cases {
        let mut arguments = args.to_vec();
        arguments.push("--start-paused");
        let mut observer = observer_for(&arguments);
        let retained = observer.events().len().to_string();
        let buffer = frame(&mut observer, 34, 22).expect("the floor draws");
        assert_eq!(
            footer_of(&buffer),
            expected.replace("{e}", &retained),
            "{args:?} at the floor"
        );
    }
}

// ---- the declared sets ---------------------------------------------------------------------

/// Guards the declared sets themselves, so a later edit cannot quietly shrink coverage.
#[test]
fn the_declared_sets_are_the_contracts() {
    assert_eq!(SEEDS, [0, 1, 42, 123, 777]);
    assert_eq!(VIEWPORTS.len(), 10);
    assert_eq!(RENDERABLE.len(), 9);
    assert!(VIEWPORTS.contains(&(33, 21)));
    // Rule 8's order of loss is decided by a seed's digit count, so the declared footer seeds
    // cover every magnitude the accepted range admits, and both of its ends.
    assert_eq!(FOOTER_SEEDS.len(), 22);
    assert_eq!(FOOTER_SEEDS[0], 0);
    assert_eq!(FOOTER_SEEDS[FOOTER_SEEDS.len() - 1], u64::MAX);
    for digits in 1..=20usize {
        assert!(
            FOOTER_SEEDS
                .iter()
                .any(|seed| seed.to_string().len() == digits),
            "no declared footer seed has {digits} digits"
        );
    }
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

    // Rule 2's glyphs are one per name and the twelve names' initials are distinct, so the dead
    // subject's glyph cannot be another's. The expected name is the engine's reported one.
    let names = reported_names(&observer);
    let name = names
        .get(&dead)
        .expect("the engine reported every name before tick 1");
    let glyph = spatial::agent_glyph(name);
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

// ---- WO-MOK-011: the name in the presentation -----------------------------------------------

/// `REQ-MOK-041`: wherever a pane identifies a Mokiterion, the name it presents is the one the
/// engine reported for *that* Mokiterion, and the identifier is still there beside it.
///
/// The pairing is the point. A frame that presented one subject's name against another's values
/// would be a defect of the same kind as `SPEC-MOK-003` rule 10.3's proposal-outcome mismatch, and a
/// per-pane check that only counted names would not catch it, so every identifier drawn in the
/// roster is required to be preceded by its own name in rule 4's six-column field.
#[test]
fn every_pane_identifying_a_mokiterion_presents_its_own_reported_name() {
    for seed in SEEDS {
        let mut observer = observer_for(&["--seed", &seed.to_string(), "--start-paused"]);
        observer.advance().expect("the engine advances");
        let names = reported_names(&observer);
        assert_eq!(
            names.len(),
            12,
            "seed {seed} reported {} names",
            names.len()
        );

        for (width, height) in VIEWPORTS
            .into_iter()
            .filter(|(width, height)| !layout::below_floor(*width, *height))
        {
            let buffer = frame(&mut observer, width, height).expect("above the floor");
            let panes = layout::resolve(*buffer.area());

            let Some(roster) = panes.roster else {
                continue;
            };
            let text = region(&buffer, roster);
            let mut presented = 0;
            for (id, name) in &names {
                if !text.contains(id.as_str()) {
                    continue;
                }
                presented += 1;
                assert!(
                    text.contains(&format!("{name:<6}{id}")),
                    "{width}x{height} presents {id} without its own name {name}:\n{text}"
                );
            }
            assert!(
                presented > 0,
                "{width}x{height} presents a roster with no entry in it:\n{text}"
            );
        }

        // Rule 10: the inspector identifies the selected subject by name and identifier. The
        // selection is made through the key binding, so nothing here reaches past the interface.
        tap(&mut observer, KeyCode::Tab);
        let selected = observer
            .selection()
            .expect("Tab selects the first living Mokiterion")
            .to_string();
        let name = names
            .get(&selected)
            .expect("the selected subject was named");
        let buffer = frame(&mut observer, 160, 48).expect("above the floor");
        let inspector = layout::resolve(*buffer.area())
            .inspector
            .expect("the reference viewport shows the inspector");
        let text = region(&buffer, inspector);
        assert!(
            text.contains(&format!("{name}  {selected}")),
            "the inspector does not identify {selected} as {name}:\n{text}"
        );
    }
}

/// `REQ-MOK-041` and `SPEC-MOK-003` rule 2 as amended: every Mokiterion glyph drawn is its own
/// subject's initial, in both zoom levels, and the withdrawn digit assignment is gone from the
/// canvas entirely.
///
/// The absence of a digit is what makes this able to fail against the previous behavior rather than
/// merely agree with the new one: `M01`-`M09` drew `1`-`9`, and no other canvas layer draws a digit.
#[test]
fn every_glyph_drawn_is_its_own_subjects_initial_in_both_zooms() {
    let mut observer = observer_for(&["--seed", "42", "--start-paused"]);
    observer.advance().expect("the engine advances");
    let names = reported_names(&observer);
    let initials: std::collections::BTreeSet<char> = names
        .values()
        .map(|name| spatial::agent_glyph(name))
        .collect();

    for zoom in ["overview", "detail"] {
        let buffer = frame(&mut observer, 160, 48).expect("above the floor");
        let canvas = region(&buffer, canvas_of(160, 48));

        // The cell each visible Mokiterion maps onto has to carry that Mokiterion's own initial.
        // A cell with more than one occupant is rule 2.4's, which draws the lowest identifier, so
        // only uniquely occupied cells are asserted; a shared cell's glyph is
        // `a_death_removes_the_subject_from_the_presentation_and_is_corroborated`'s subject and
        // `every_distinction_survives_the_loss_of_colour`'s.
        let area = canvas_of(160, 48);
        let viewport = spatial::Viewport::resolve(
            observer.zoom(),
            (area.width, area.height),
            observer.camera(),
        );
        let mut occupants: BTreeMap<(u16, u16), Vec<String>> = BTreeMap::new();
        for agent in &observer.snapshot().agents {
            if let Some(cell) = viewport.cell_of(
                observer.zoom(),
                agent.position.x.into(),
                agent.position.y.into(),
            ) && cell.0 < area.width
                && cell.1 < area.height
            {
                occupants.entry(cell).or_default().push(agent.id.clone());
            }
        }
        assert!(
            !occupants.is_empty(),
            "{zoom} zoom maps no Mokiterion into the canvas at all"
        );
        let mut asserted = 0;
        for ((x, y), ids) in &occupants {
            if ids.len() > 1 {
                continue;
            }
            let name = names.get(&ids[0]).expect("every living subject was named");
            let drawn = buffer
                .cell((area.x + x, area.y + y))
                .expect("inside the canvas")
                .symbol();
            assert_eq!(
                drawn,
                spatial::agent_glyph(name).to_string(),
                "{} is drawn as {drawn} rather than as {name}'s initial in {zoom} zoom",
                ids[0]
            );
            asserted += 1;
        }
        assert!(
            asserted > 0,
            "{zoom} zoom drew nothing but shared cells, so no pairing was checked"
        );

        // No drawn cell carries the withdrawn identifier-derived assignment.
        assert!(
            !canvas.chars().any(|character| character.is_ascii_digit()),
            "{zoom} zoom still draws a digit glyph:\n{canvas}"
        );

        // Every uppercase letter drawn is one of the reported names' initials, so no glyph belongs
        // to nothing and none is a placeholder.
        for character in canvas
            .chars()
            .filter(|character| character.is_ascii_uppercase())
        {
            assert!(
                initials.contains(&character),
                "{zoom} zoom draws {character}, which is no reported name's initial:\n{canvas}"
            );
        }

        tap(&mut observer, KeyCode::Char('z'));
    }
}

/// An observer holding a selection on a Mokiterion that has just died, with that subject's
/// identifier and the name the engine reported for it.
///
/// The subject has to be selected before it dies, because a dead subject cannot be selected — it is
/// out of the roster. So the run is taken twice: once to find which Mokiterion dies first and when,
/// and once to select that subject and hold the selection through its death. Both runs are the same
/// seed and configuration, so the second reaches the same death on the same tick.
///
/// Two rule 10.6 cases need this state, and it is constructed here once rather than twice so that
/// they cannot drift onto different deaths.
///
/// **The policy is `reference` and not `baseline`, under `WO-MOK-020`.** Rule 10 as amended there
/// clears the selection when no living Mokiterion remains, and `baseline` starves the whole
/// population on one tick: its first death *is* extinction, so the state this fixture is for — a
/// retained selection on a dead subject — stopped existing under that policy at the moment the
/// amendment landed. Under `reference` the first death leaves eleven living, so the selection is
/// retained and every assertion below holds as written. The coverage this costs is stated in
/// `WO-MOK-020`'s completion report rather than absorbed here: rule 10.6's retention is now
/// exercised on a mid-run death and no longer on a terminal one, which is the case the amendment
/// removed rather than a case that went untested.
fn observer_holding_a_dead_selection() -> (Observer, String, String) {
    let arguments = ["--policy", "reference", "--ticks", "400", "--start-paused"];

    let mut scout = observer_for(&arguments);
    let (victim, tick) = loop {
        assert!(!scout.is_finished(), "the run ended before any death");
        let living: Vec<String> = scout
            .snapshot()
            .agents
            .iter()
            .map(|agent| agent.id.clone())
            .collect();
        scout.advance().expect("the engine advances");
        if let Some(gone) = living
            .into_iter()
            .find(|was| !scout.snapshot().agents.iter().any(|is| is.id == *was))
        {
            break (gone, scout.snapshot().tick);
        }
    };

    let mut observer = observer_for(&arguments);
    let names = reported_names(&observer);
    let name = names.get(&victim).expect("the victim was named").clone();
    for _ in 0..12 {
        if observer.selection() == Some(victim.as_str()) {
            break;
        }
        tap(&mut observer, KeyCode::Tab);
    }
    assert_eq!(
        observer.selection(),
        Some(victim.as_str()),
        "the victim could not be selected while living"
    );

    while observer.snapshot().tick < tick {
        observer.advance().expect("the engine advances");
    }
    assert!(
        !observer
            .snapshot()
            .agents
            .iter()
            .any(|agent| agent.id == victim),
        "{victim} is still living at tick {tick}"
    );
    assert_eq!(
        observer.selection(),
        Some(victim.as_str()),
        "rule 10.6's selection did not survive the death"
    );

    (observer, victim, name)
}

/// The inspector's region on the reference viewport, as text.
fn inspector_text(observer: &mut Observer) -> String {
    let buffer = frame(observer, 160, 48).expect("above the floor");
    let inspector = layout::resolve(*buffer.area())
        .inspector
        .expect("the reference viewport shows the inspector");
    region(&buffer, inspector)
}

/// `REQ-MOK-041` with `SPEC-MOK-003` rule 10.6: the inspector identifies a dead subject by name and
/// identifier, the same way it identified it living.
#[test]
fn the_inspector_identifies_a_dead_subject_by_name_and_identifier() {
    let (mut observer, victim, name) = observer_holding_a_dead_selection();

    let text = inspector_text(&mut observer);
    assert!(
        text.contains(&format!("{name}  {victim}")),
        "the inspector does not identify the dead {victim} as {name}:\n{text}"
    );
    assert!(text.contains("died on tick"), "{text}");
}

/// `SPEC-MOK-003` rule 10.6 as amended by `WO-MOK-018`: the fourth attribute is on the death line.
///
/// Rule 4 presents fear for every living Mokiterion on the roster's second bar line, and a dead
/// subject is not on the roster. Before this case the value became unreachable at the moment it
/// stopped changing, which rule 10.7's own justification had assumed could not happen.
///
/// The presented number is compared against the death record the state derived, and that record is
/// compared against the engine's own report by `tests/state.rs`, so no tier restates a constant.
#[test]
fn the_inspector_presents_a_dead_subject_s_final_fear() {
    let (mut observer, victim, _) = observer_holding_a_dead_selection();
    let death = observer.death_of(&victim).expect("the death was retained");
    let fear = death
        .fear
        .expect("a death a run reaches follows a survival record for the same subject");
    let expected = format!("fear {fear}");

    let text = inspector_text(&mut observer);
    assert!(
        text.contains(&expected),
        "the dead {victim}'s final fear is not presented as `{expected}`:\n{text}"
    );

    // Rule 4's banded gauges belong to the roster's living rows. The death line states the four
    // values plainly, so finding this one there is not finding a gauge that leaked into the pane.
    assert!(text.contains("final health"), "{text}");
}

// ---- WO-MOK-013: the notice that names the remedy, without colour ---------------------------

/// Every cell of one region, as symbol and modifier, with colour discarded.
///
/// The same projection `src/verification.rs` uses for rule 2.5's cases, restated here rather than
/// shared: that one is a `#[cfg(test)]` helper inside the crate, and an integration test links the
/// crate's public interface, which no such helper is part of. Restating a helper across the two tiers
/// is what `SPEC-MOK-004` rule 9's split already requires of this file.
///
/// Returning a value that colour is not in at all is the point of it. A test that read `fg` and
/// asserted something about it could still pass on a frame whose meaning lived in colour.
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

/// The character `SPEC-MOK-003` rule 7 binds to the key-binding overlay, and the one it binds to each
/// pane's own overlay. Rule 7 is the source, so these are stated rather than read from the observer.
const OVERLAY_KEY: char = '?';

fn specified_key(pane: layout::Pane) -> char {
    match pane {
        layout::Pane::Roster => 'r',
        layout::Pane::Log => 'L',
        layout::Pane::Inspector => 'i',
    }
}

/// The axis that constrains `pane`, as an initial, and the extent at which the pane returns,
/// measured from the layout rather than written down — the condition `VER-MOK-013` sets for every
/// case that reads the announcement's stated value.
fn measured_threshold(pane: layout::Pane) -> (char, u16) {
    let in_the_body = |width: u16, height: u16| {
        !layout::resolve(Rect::new(0, 0, width, height))
            .overlay_only()
            .contains(&pane)
    };
    let by_width = (layout::MIN_WIDTH..=200).find(|&width| in_the_body(width, 60));
    let by_height = (layout::MIN_HEIGHT..=60).find(|&height| in_the_body(200, height));
    match (by_width, by_height) {
        (Some(width), _) if width > layout::MIN_WIDTH => ('W', width),
        (_, Some(height)) if height > layout::MIN_HEIGHT => ('H', height),
        other => panic!("{} is excluded by no extent: {other:?}", pane.label()),
    }
}

/// `REQ-MOK-049`'s "legible without colour" row, and the projection clause of `VER-MOK-013`'s
/// acceptance scenarios 3 and 4.
///
/// The defect this closes was a notice an operator could not act on. A notice whose meaning lived in
/// its colour would be the same defect for one operator in twelve, so the axis, the extent and the
/// key are all asserted against a projection colour has been discarded from — at `120 x 48`, where
/// one pane is excluded, and at the floor, where all three are.
///
/// The emphasis is asserted in the same projection for the same reason. Rule 5 requires the
/// announcement to be distinguishable from the optional segments beside it, and a modifier is what
/// survives here where a colour would not.
#[test]
fn the_announcement_and_the_hint_survive_the_loss_of_colour() {
    for (width, height) in [(120u16, 48u16), (34, 22)] {
        let mut observer = observer_for(&["--seed", "42"]);
        let buffer = frame(&mut observer, width, height).expect("above the floor");
        let cells = monochrome(&buffer, Rect::new(0, 0, width, 1));
        let plain = symbols(&cells);

        assert!(
            plain.contains(OVERLAY_KEY),
            "{width}x{height} loses the hint with colour discarded: {plain}"
        );

        let excluded = layout::resolve(Rect::new(0, 0, width, height)).overlay_only();
        assert!(
            !excluded.is_empty(),
            "{width}x{height} excludes no pane, so this case is unexercised"
        );
        for pane in excluded {
            let (axis, value) = measured_threshold(pane);
            let key = specified_key(pane);
            let word = match axis {
                'W' => "width",
                _ => "height",
            };
            assert!(
                plain.contains(&format!("{key} at {word} {value}"))
                    || plain.contains(&format!("{key} {axis}{value}")),
                "{width}x{height} states neither the axis nor the extent of the {} with colour \
                 discarded: {plain}",
                pane.label()
            );
        }

        // Rule 5's emphasis, in the only part of the projection that carries style at all.
        assert!(
            cells[0]
                .iter()
                .any(|(_, modifier)| modifier.contains(Modifier::BOLD)),
            "{width}x{height} carries no emphasis in the header's first row"
        );
    }
}

// ---- `VER-MOK-017`: the cumulative activity profile, cross-cutting -------------------------
//
// `WO-MOK-020` widens what the observer accumulates from the event stream, so `REQ-MOK-025`'s
// non-perturbation obligation is in scope by construction and is measured here rather than
// argued. The two cases below are the contract's O17 and O18; the five after them are its static
// checks O19.1 to O19.4 and O20.1, which read the tree rather than a frame because each is a
// claim about what the source does not contain.
//
// `VER-MOK-017`'s independence clause 1 forbids an oracle that reads the accumulator, so every
// figure these cases compare against is the engine binary's own, reached through `unobserved`.

/// The pane-only vocabulary of the activity profile: strings the inspector presents and that the
/// authoritative stream must therefore never carry.
///
/// The list is written here as literals rather than imported from the presentation path, so that a
/// label the implementation renames cannot silently make this case vacuous.
const PANE_ONLY: [&str; 8] = [
    "cumulative activity",
    "population activity",
    "no tick has completed",
    "nothing selected",
    "crossings",
    "decisions",
    "by a strike",
    "unattributed",
];

/// The repository root, from this package's own manifest directory.
///
/// `SPEC-MOK-004` rule 1 makes the tree authoritative, so the static checks locate their subjects
/// from the workspace layout rather than from a path written into a test.
fn repository_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the package sits under the workspace root")
        .to_path_buf()
}

fn source(relative: &str) -> String {
    let path = repository_root().join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// Every file under a directory whose name ends in one of `suffixes`, as `(path, text)` pairs.
fn tree(relative: &str, suffixes: &[&str]) -> Vec<(String, String)> {
    fn walk(at: &std::path::Path, suffixes: &[&str], into: &mut Vec<(String, String)>) {
        let entries = std::fs::read_dir(at).unwrap_or_else(|error| panic!("{at:?}: {error}"));
        let mut sorted: Vec<std::path::PathBuf> = entries
            .map(|entry| entry.expect("a readable entry").path())
            .collect();
        sorted.sort();
        for path in sorted {
            if path.is_dir() {
                walk(&path, suffixes, into);
            } else if suffixes
                .iter()
                .any(|suffix| path.to_string_lossy().ends_with(suffix))
            {
                let text = std::fs::read_to_string(&path)
                    .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
                into.push((path.display().to_string(), text));
            }
        }
    }

    let mut found = Vec::new();
    walk(&repository_root().join(relative), suffixes, &mut found);
    assert!(!found.is_empty(), "{relative} yielded no {suffixes:?} file");
    found
}

/// Drives an observed run to the engine's own end with the inspector drawn in **both** pane states
/// on every tick, at the reference viewport where every figure is presented.
///
/// This is deliberately not `observed_run`: that helper rotates the viewport, so on most ticks the
/// inspector is absent and the totals are never computed for presentation. `REQ-MOK-062`'s claim is
/// that summing the population on every tick perturbs nothing, and a run that presents the sums on
/// one tick in ten cannot measure it.
fn profiled_run(args: &[&str]) -> Observer {
    let mut observer = observer_for(args);
    let mut round = 0u64;
    while !observer.is_finished() {
        // Selected on even ticks, cleared on odd: the per-Mokiterion block and the population
        // block are each drawn on every other tick, and both are drawn within every pair.
        tap(
            &mut observer,
            if round.is_multiple_of(2) {
                KeyCode::Tab
            } else {
                KeyCode::Esc
            },
        );
        frame(&mut observer, 160, 48);
        observer.advance().expect("the engine advances");
        round += 1;
    }
    // A finished run is still drawn, in the state extinction or the tick limit left it in.
    frame(&mut observer, 160, 48);
    observer
}

/// `VER-MOK-017` O17: presenting the totals perturbs no run.
///
/// On every declared seed the observed run's records and final state are the engine binary's, and
/// the per-tick entropy-bearing record counts are identical, while the inspector draws the
/// per-Mokiterion totals and the population sums on every tick.
#[test]
fn presenting_the_totals_perturbs_no_run_on_any_declared_seed() {
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
        // `social` is the policy under which every one of the eleven kinds can occur, so the
        // accumulation being measured is the one that runs over the widest set of records.
        let args = ["--seed", &seed, "--ticks", "60", "--policy", "social"];
        let (expected, summary) = unobserved(&args);
        let observer = profiled_run(&args);
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

        let mine = by_tick(&observed);
        let theirs = by_tick(&expected);
        assert_eq!(
            mine.keys().collect::<Vec<_>>(),
            theirs.keys().collect::<Vec<_>>(),
            "seed {seed}: the runs cover different ticks"
        );
        for (tick, lines) in &mine {
            let drawn = lines.iter().filter(|line| entropy_bearing(line)).count();
            let reference = theirs[tick]
                .iter()
                .filter(|line| entropy_bearing(line))
                .count();
            assert_eq!(
                drawn, reference,
                "seed {seed} tick {tick}: {drawn} entropy-bearing records observed, {reference} \
                 unobserved"
            );
        }

        // The identity above is only evidence if the totals were in fact accumulated and drawn.
        assert!(
            observer.snapshot().tick > 0,
            "seed {seed}: no tick completed, so nothing was accumulated"
        );
    }
}

/// `VER-MOK-017` O18: no total reaches the export, and the export is still the engine's records.
///
/// Rule 9.4 keeps the export the retained event stream in its text format. The comparison is
/// against `unobserved`, so the assertion is that a run whose inspector presented every total on
/// every tick exports the engine binary's bytes and nothing else.
#[test]
fn no_total_reaches_the_export_and_it_stays_the_engines_records() {
    for seed in SEEDS {
        let seed = seed.to_string();
        let args = ["--seed", &seed, "--ticks", "40", "--policy", "social"];
        let (expected, _) = unobserved(&args);
        let observer = profiled_run(&args);

        let mut bytes = Vec::new();
        export::write_records(&mut bytes, observer.events()).expect("writing to a vector");
        let text = String::from_utf8(bytes).expect("the export is text");

        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(&lines[..lines.len() - 1], &expected[..], "seed {seed}");
        assert_eq!(
            lines[lines.len() - 1],
            format!("# retained={} truncated=false", expected.len()),
            "seed {seed}"
        );

        for phrase in PANE_ONLY {
            assert!(
                !text.contains(phrase),
                "seed {seed}: the export carries the pane-only phrase {phrase:?}"
            );
        }
    }
}

/// `VER-MOK-017` O19.1: the engine's declared dependency set does not name the observer.
#[test]
fn the_engine_declares_no_dependency_on_the_observer() {
    let manifest = source("mokiterions-core/Cargo.toml");
    assert!(
        !manifest.contains("mokiterions-tui"),
        "the engine's manifest names the observer package:\n{manifest}"
    );
    // The `[dependencies]` table carries no entry at all, which is the stronger statement and the
    // one `ADR-MOK-006` decision 1 records. Reading to the next section header keeps the check
    // honest if a later table is added.
    let dependencies = manifest
        .split_once("[dependencies]")
        .expect("the manifest declares a dependencies table")
        .1;
    let body = match dependencies.find("\n[") {
        Some(end) => &dependencies[..end],
        None => dependencies,
    };
    let entries: Vec<&str> = body
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();
    assert!(
        entries.is_empty(),
        "the engine's dependencies table is no longer empty: {entries:?}"
    );
}

/// `VER-MOK-017` O19.2: no name of the retained profile appears anywhere in the engine.
///
/// `WO-MOK-020` puts the accumulation in the observer and forbids per-Mokiterion counters in the
/// engine, so the engine's own tree is the place that claim is falsifiable.
///
/// The counter *names* the observer shares with the engine are not in the list, and the omission is
/// measured rather than assumed: the engine already keeps one private world-wide `crossings` total
/// for its structured run record, which predates this work order and is neither per-Mokiterion nor
/// public. Asserting that word's absence would fail on state this work order does not touch and
/// would say nothing about what it adds. What is asserted is the absence of the retained profile
/// itself — its type, its kind enumeration and its three accessors.
#[test]
fn no_profile_type_or_accessor_appears_in_the_engine() {
    const ABSENT: [&str; 5] = [
        "ActionKind",
        "Profile",
        "profile_of",
        "population_profile",
        "initialized_count",
    ];

    for (path, text) in tree("mokiterions-core", &[".rs", ".toml"]) {
        for name in ABSENT {
            assert!(
                !text.contains(name),
                "{path} names {name}, which belongs to the observer's retained profile"
            );
        }
    }
}

/// `VER-MOK-017` O19.3: the engine's mutating surface is unchanged.
///
/// `SPEC-MOK-003`'s *Data and interface contracts* clause 2 fixes it at two functions, and
/// `WO-MOK-020` constraint 5 restates the exact command. The parse here is the command's meaning
/// rather than its text, so the check runs on any platform.
#[test]
fn the_engine_still_exposes_exactly_two_mutating_entry_points() {
    let engine = source("mokiterions-core/src/simulation.rs");
    let mutating: Vec<&str> = engine
        .lines()
        .filter(|line| line.contains("pub fn ") && line.contains("&mut self"))
        .map(|line| {
            let after = line.split_once("pub fn ").expect("checked by the filter").1;
            after
                .split(['(', '<'])
                .next()
                .expect("a name precedes the parameter list")
                .trim()
        })
        .collect();
    assert_eq!(
        mutating,
        vec!["run", "advance_tick"],
        "the engine's mutating entry points moved"
    );
}

/// `VER-MOK-017` O19.4: no total is placed in an `Observation` or reached by a `DecisionSource`.
///
/// `REQ-MOK-062`'s safety argument is a boundary claim, and the boundary is that the observer's
/// package names neither of the engine's decision-side types. The one match that does exist is
/// `DecisionSourceSelected`, an event-type variant the authority overlay reads by name; the check
/// admits it explicitly rather than by a looser pattern that would admit anything.
#[test]
fn no_total_can_reach_an_observation_or_a_decision_source() {
    for (path, text) in tree("mokiterions-tui/src", &[".rs"]) {
        assert!(
            !text.contains("Observation"),
            "{path} names Observation, which carries values into a decision"
        );
        for (number, line) in text.lines().enumerate() {
            for (index, _) in line.match_indices("DecisionSource") {
                let rest = &line[index..];
                assert!(
                    rest.starts_with("DecisionSourceSelected"),
                    "{path}:{} reaches a DecisionSource: {line}",
                    number + 1
                );
            }
        }
    }
}

/// `VER-MOK-017` O20.1: no total is a float, and no presented figure is a ratio.
///
/// The scope is the observer code this work order touches. `spatial.rs` already computes canvas
/// coordinates in `f64` and is untouched here, so a package-wide check would fail on state that
/// predates this work order and would say nothing about it; the check therefore names the two
/// modules the change surface names and asserts the exclusion there.
#[test]
fn no_total_is_a_float_and_no_presented_figure_is_a_ratio() {
    for module in [
        "mokiterions-tui/src/state.rs",
        "mokiterions-tui/src/render.rs",
    ] {
        let text = source(module);
        for name in ["f32", "f64"] {
            assert!(
                !text.contains(name),
                "{module} uses {name}; every total is an integer"
            );
        }
    }

    // And the rendered pane, in both states, carries no percentage, no decimal separator between
    // digits and no ratio between digits.
    let mut observer = profiled_run(&["--seed", "123", "--ticks", "40", "--policy", "social"]);
    for selected in [true, false] {
        tap(
            &mut observer,
            if selected { KeyCode::Tab } else { KeyCode::Esc },
        );
        let buffer = frame(&mut observer, 160, 48).expect("the reference viewport renders");
        let inspector = layout::resolve(*buffer.area())
            .inspector
            .expect("the reference viewport shows the inspector");
        let text = region(&buffer, inspector);
        assert!(
            !text.contains('%'),
            "the inspector presents a percentage:\n{text}"
        );
        let cells: Vec<char> = text.chars().collect();
        for window in cells.windows(3) {
            let [left, middle, right] = [window[0], window[1], window[2]];
            assert!(
                !(left.is_ascii_digit()
                    && right.is_ascii_digit()
                    && (middle == '.' || middle == '/')),
                "the inspector presents {left}{middle}{right}, which is not an integer:\n{text}"
            );
        }
    }
}
