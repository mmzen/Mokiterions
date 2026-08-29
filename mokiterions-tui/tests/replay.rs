//! Public tier: the observer as replay host, `SPEC-MOK-007` rules 20.3, 20.4 and 20.4.1.
//!
//! What is asserted here is the *lending*, not the replaying. The port is the engine's own
//! `ReplayPort` in a real run, and what it does with a transcript is asserted where a transcript can
//! be produced — the engine's internal tier. What this host is responsible for is building the port
//! once, keeping it for the whole run, handing it over one tick at a time, and reporting whatever it
//! says. Each of those is a way this program can be wrong about a port that is itself correct, and
//! rule 20.4.1 names the one that still compiles and still runs: a port rebuilt each tick resets the
//! transcript cursor, the accumulated cost and the fallback count every tick.
//!
//! The scripted port below stands in for the reader-backed one, because `Proposer` is the engine's one
//! interface for a decision from outside itself and this host cannot tell the two apart — which is the
//! property rule 1.1 exists for. It needs no transcript, no file and no crate: `ARCH-MOK-001` keeps
//! this package's dependency table empty, dev-dependencies included, so the shared counter is an `Rc`
//! and a `Cell` from the standard library.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::Path;
use std::rc::Rc;

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::layout::Rect;

use mokiterions::simulation::{Action, DecisionRequest, Direction, Proposal, Proposer};
use mokiterions_tui::options::{self, Options, Startup};
use mokiterions_tui::state::{Observer, Progression};
use mokiterions_tui::{layout, render};

/// What a lent port did, readable after the port has been given away.
///
/// The observer takes the port by value and never gives it back, so a test that wants to know what
/// happened to it has to share the record rather than borrow the port. That is the same shape the
/// engine's own internal tier uses, for the same reason.
#[derive(Debug, Default)]
struct Ledger {
    /// One entry per decision opportunity, in the order the engine reached them.
    proposed: Vec<(u64, String)>,
    /// One entry per record the engine authored, which is one per opportunity.
    recorded: usize,
}

/// A port that answers every opportunity, notes what it was asked, and can be told to fail.
struct ScriptedPort {
    ledger: Rc<RefCell<Ledger>>,
    /// The opportunity, counting from one, at which `record` reports a failure. `None` never fails.
    ///
    /// `record` rather than `propose`, because that is the seam rules 12.3 and 12.4 report through: a
    /// proposal's absence is rule 9.5's ordinary fallback and says nothing, so a replay port detects a
    /// mismatch while proposing and reports it when handed the record.
    failing_at: Option<usize>,
    /// Whether the port proposes movement rather than `Wait`.
    ///
    /// The lending cases below want the least eventful run they can get, because what they count is
    /// opportunities and a Mokiterion that dies stops producing them. Case L31 wants the opposite: it
    /// compares what the panes did against `social`, and a run in which nothing moves fills no event
    /// log and crosses no territory, so it would compare two empty panes and pass on nothing.
    moving: bool,
}

impl ScriptedPort {
    fn new(ledger: &Rc<RefCell<Ledger>>) -> Self {
        Self {
            ledger: Rc::clone(ledger),
            failing_at: None,
            moving: false,
        }
    }

    fn failing_at(ledger: &Rc<RefCell<Ledger>>, opportunity: usize) -> Self {
        Self {
            ledger: Rc::clone(ledger),
            failing_at: Some(opportunity),
            moving: false,
        }
    }

    /// A port whose proposals move, so that the run it drives has something for the panes to show.
    fn moving(ledger: &Rc<RefCell<Ledger>>) -> Self {
        Self {
            ledger: Rc::clone(ledger),
            failing_at: None,
            moving: true,
        }
    }
}

impl Proposer for ScriptedPort {
    fn propose(&mut self, request: DecisionRequest) -> Proposal {
        let mut ledger = self.ledger.borrow_mut();
        ledger
            .proposed
            .push((request.tick(), request.actor_id().to_string()));
        // Rule 1.1a's evidence fields stay empty throughout: this port answers from a counter, so
        // there is no response to carry and rule 11.5's four counts are absent.
        let proposing = |action| Proposal {
            action: Some(action),
            ..Proposal::nothing()
        };
        if !self.moving {
            return proposing(Action::Wait);
        }
        // The four cardinals in turn. A step off the grid is not a valid move, and rule 9.5's
        // fallback covers it, so the port needs no knowledge of where the Mokiterion stands.
        const CARDINALS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        proposing(Action::Move {
            direction: CARDINALS[ledger.proposed.len() % 4],
        })
    }

    fn record(&mut self, _record: &str) -> io::Result<()> {
        let mut ledger = self.ledger.borrow_mut();
        ledger.recorded += 1;
        if self.failing_at == Some(ledger.recorded) {
            // The port's own words, in the shape a replay port's mismatch takes: rule 19.4 requires
            // the opportunity to be named, and rule 19.7 forbids a credential and a path.
            return Err(io::Error::other(format!(
                "transcript: opportunity {} does not match the record",
                ledger.recorded
            )));
        }
        Ok(())
    }
}

/// Resolved inputs for a replay of the named length, as the observer's own parser produces them.
///
/// The transcript path is supplied because the shared parser requires it — rules 13.2 and 19.2 — and
/// it names no file, because nothing in this file opens one. That is rule 18.4 from the inside: the
/// parser validates the argument and retains nothing, and the host that opens it is `src/main.rs`.
fn replaying(ticks: &str) -> Options {
    let args = vec![
        "--policy",
        "llm",
        "--transcript-path",
        "transcript.jsonl",
        "--ticks",
        ticks,
        "--seed",
        "42",
    ];
    match options::parse(args).unwrap() {
        Startup::Run(options) => options,
        Startup::Help => panic!("expected a run"),
    }
}

/// Rules 20.4 and 20.4.1: one port for the run, lent every tick, never rebuilt.
///
/// The observable form of "never rebuilt" is that the port's own state accumulates across ticks. A
/// port constructed per tick would present a ledger reset to nothing every time, and this asserts the
/// opposite in both of the port's methods: the proposals accumulate, they arrive in tick order, and
/// each opportunity's record follows it.
#[test]
fn one_port_serves_the_whole_run_and_is_never_rebuilt() {
    let ledger = Rc::new(RefCell::new(Ledger::default()));
    let mut observer =
        Observer::with_port(replaying("5"), Some(Box::new(ScriptedPort::new(&ledger)))).unwrap();

    let mut after_each_tick = Vec::new();
    for _ in 0..5 {
        observer.advance().expect("a scripted port answers");
        after_each_tick.push(ledger.borrow().proposed.len());
    }

    // Twelve Mokiterions, none of which dies while every one of them waits, so every tick is a full
    // round of opportunities and the total is exact rather than a lower bound.
    assert_eq!(after_each_tick, vec![12, 24, 36, 48, 60]);
    assert_eq!(observer.snapshot().tick, 5);

    let ledger = ledger.borrow();
    // Rule 12.1's ordering: opportunity *n* belongs to the tick the engine reached it in, and the
    // ticks are non-decreasing. A rebuilt port would still see this — what it could not see is the
    // count above — so both are asserted.
    assert_eq!(ledger.proposed.len(), 60);
    for window in ledger.proposed.windows(2) {
        assert!(window[0].0 <= window[1].0, "{:?}", window);
    }
    assert_eq!(ledger.proposed.first().unwrap().0, 1);
    assert_eq!(ledger.proposed.last().unwrap().0, 5);

    // Rule 11.1's seam: the engine authors one record per exchange and hands it over, in a replay as
    // well as in a recording, because rule 12.1 makes it the same code path.
    assert_eq!(ledger.recorded, 60);
}

/// The port's failure ends the run, in the port's own words.
///
/// Rules 12.3 and 12.4 are the failures this carries for a real replay — a transcript from another
/// configuration and a transcript that ran out — and rule 12.5 makes them fatal rather than a
/// fallback. This host adds nothing to the message and swallows none of it: an operator whose replay
/// diverged has to be told which opportunity diverged, and that figure is the port's.
#[test]
fn a_port_failure_ends_the_run_in_the_ports_own_words() {
    let ledger = Rc::new(RefCell::new(Ledger::default()));
    let mut observer = Observer::with_port(
        replaying("5"),
        Some(Box::new(ScriptedPort::failing_at(&ledger, 13))),
    )
    .unwrap();

    // The first tick is a full round of twelve and completes; the thirteenth opportunity is the first
    // of the second tick.
    observer.advance().expect("the first tick is answered");
    let error = observer
        .advance()
        .expect_err("a port that reports a mismatch ends the run");
    assert!(error.contains("opportunity 13"), "{error}");
    assert!(error.contains("transcript"), "{error}");

    // Nothing about the failure names a path or a credential, because the port named neither.
    let lowered = error.to_lowercase();
    for forbidden in ["transcript.jsonl", "credential", "bearer", "http"] {
        assert!(!lowered.contains(forbidden), "{error}");
    }
}

/// Rule 20.8, reached through this host: `llm` with no port is refused on the first tick.
///
/// The pair cannot arrive from a command line — the shared parser refuses `llm` with no transcript,
/// and `src/main.rs` builds a port for every invocation that gets past it — so this is the failure of
/// a host that parsed the option and then forgot to act on it. It is asserted here because that is
/// exactly the failure `WO-MOK-025`'s ninth stop condition is about, and because the refusal is the
/// engine's own words rather than a second copy of them in this program.
#[test]
fn the_replay_source_with_no_port_is_refused_on_the_first_tick() {
    let mut observer = Observer::with_port(replaying("5"), None).unwrap();

    let error = observer
        .advance()
        .expect_err("rule 20.8: no port and no substitute source");
    assert!(error.contains("policy llm"), "{error}");
    assert!(error.contains("decision port"), "{error}");

    // Refused, not substituted: the run advanced no tick and reached no other source.
    assert_eq!(observer.snapshot().tick, 0);
    assert!(!observer.is_finished());
}

/// The four sources that decide for themselves are unaffected: no port, and none wanted.
///
/// Rule 20.9 is explicit that `None` is what they take, and `Observer::new` is kept as the
/// one-argument constructor so that every existing call site says so by construction. This asserts the
/// two constructors agree where they overlap, which is what makes the several dozen unchanged call
/// sites evidence of anything.
#[test]
fn a_source_that_decides_for_itself_needs_no_port() {
    let inputs = || match options::parse(vec!["--ticks", "3", "--seed", "42"]).unwrap() {
        Startup::Run(options) => options,
        Startup::Help => panic!("expected a run"),
    };

    let mut implicit = Observer::new(inputs()).unwrap();
    let mut explicit = Observer::with_port(inputs(), None).unwrap();
    for _ in 0..3 {
        implicit.advance().unwrap();
        explicit.advance().unwrap();
    }

    assert_eq!(implicit.snapshot(), explicit.snapshot());
    assert_eq!(implicit.progression(), Progression::Running);
    assert!(implicit.is_finished());
}

/// What the six panes did, reduced to what two runs under two sources can be compared on.
///
/// `VER-MOK-018` case **L31** asks that the roster, map, event log, inspector, filter and export
/// *"behave as they do under `social`"*, and two runs under two sources decide differently — different
/// cells, a different event log, a different selection — so comparing what the panes *say* would fail
/// on the runs rather than on the panes. What is comparable is what each pane *did*, and every field
/// below is read from that pane's own region rather than off the screen, so a claim about the roster is
/// a claim about the roster.
///
/// Each field holds under `social` today and each is a way this host could treat the fifth source
/// specially, which is what the case exists to rule out.
#[derive(Debug, PartialEq, Eq)]
struct PaneReport {
    /// The tick the run stopped at, which is the transcript's horizon here.
    tick: u64,
    /// Whether the roster accounts for every living Mokiterion: the ones it named plus the ones its
    /// own header declares hidden, against the count the engine reports.
    ///
    /// A count of names would differ between two runs for a reason that is not the source — at
    /// 120×40 the pane holds nine of twelve and says `hidden 3` — so what is compared is the
    /// accounting rather than the names.
    roster_accounts_for_every_living: bool,
    /// Whether the map pane drew anything: its region's non-blank cells, banded rather than counted,
    /// so that two runs whose Mokiterions stand in different cells still compare.
    map_drew: bool,
    /// Whether the log pane presented a record.
    log_presented: bool,
    /// Whether the selection reached the inspector: the observer reports one and the inspector's own
    /// region names it.
    inspector_names_the_selection: bool,
    /// Whether the filter narrowed or held what the log presents, and the label it moved to.
    filter_narrows: bool,
    filter_label: String,
    /// Whether the export key wrote a non-empty file where the operator asked.
    exported: bool,
}

/// Rule 8's footer, split into the fields it presents.
///
/// Split rather than compared whole because two runs retain different numbers of events, so the
/// `events` field differs for a reason that has nothing to do with the source. Splitting is what lets
/// the fields that must agree be asserted equal and the two that must differ be named.
fn footer_fields(footer: &str) -> BTreeMap<String, String> {
    footer
        .split("  ")
        .filter(|field| !field.is_empty())
        .map(|field| {
            let (label, value) = field.split_once(' ').unwrap_or((field, ""));
            (label.to_string(), value.to_string())
        })
        .collect()
}

/// Resolved inputs for a five-tick observed run under the named source, exporting where told.
fn observing(policy: &str, export: &str) -> Options {
    let mut args = vec!["--policy", policy, "--ticks", "5", "--seed", "42"];
    if policy == "llm" {
        // Required by the shared parser — rules 13.2 and 19.2 — and opened by nobody here, as in
        // `replaying` above: this file's port is scripted.
        args.extend(["--transcript-path", "transcript.jsonl"]);
    }
    args.extend(["--export", export]);
    match options::parse(args).unwrap() {
        Startup::Run(options) => options,
        Startup::Help => panic!("expected a run"),
    }
}

/// The viewport this case draws at: wide enough and tall enough that `layout::resolve` places all
/// three optional panes on the screen rather than behind an overlay, so each can be read in its own
/// region. The observer announces the inspector as overlay-only below 140 columns.
const VIEWPORT: (u16, u16) = (160, 44);

/// The first `Mnn` identifier a row names, which on a roster row is that row's own subject.
fn first_identifier(row: &str) -> Option<String> {
    let characters: Vec<char> = row.chars().collect();
    characters
        .windows(3)
        .find(|window| window[0] == 'M' && window[1].is_ascii_digit() && window[2].is_ascii_digit())
        .map(|window| window.iter().collect())
}

/// The Mokiterions the roster pane lists, read one entry row at a time.
///
/// Every identifier in the pane is not the same thing as every entry in it: under `social` a row's
/// action names a *target* — `approach M03` — and counting those would report more entries than the
/// pane has. So the subject is taken as the first identifier on each entry row, and the rows that
/// continue an entry are skipped by their indentation: an entry row opens with the Mokiterion's name
/// against the border, and a continuation row and the pane's own header both open with a space.
fn roster_entries(roster: &str) -> BTreeSet<String> {
    roster
        .lines()
        .filter_map(|row| {
            let body = row.trim_start_matches(['│', '┌', '└']);
            if body.starts_with(' ') {
                return None;
            }
            first_identifier(body)
        })
        .collect()
}

/// A `label <number>` figure a pane's own header states, or `None` where the header omits it.
fn figure(text: &str, label: &str) -> Option<u64> {
    text.split_whitespace()
        .skip_while(|word| *word != label)
        .nth(1)
        .and_then(|value| value.parse().ok())
}

/// One frame, drawn at [`VIEWPORT`].
fn drawn(observer: &mut Observer) -> Buffer {
    let (width, height) = VIEWPORT;
    let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("a test backend");
    terminal
        .draw(|target| render::draw(target, observer))
        .expect("drawing into a buffer");
    terminal.backend().buffer().clone()
}

/// One region's rows, joined. Reading a pane rather than the screen is what makes a claim about a
/// pane a claim about that pane, and it is the idiom `VER-MOK-005`'s public tier already uses.
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

/// How many cells of a region are not blank.
fn ink(text: &str) -> usize {
    text.chars().filter(|glyph| !glyph.is_whitespace()).count()
}

/// Drives one observed run to its horizon under operator control, then reads each pane in turn.
///
/// The operator acts *between* ticks rather than after the run, because the case says *"under
/// operator control"*: a host that only worked once the run was over would satisfy an assertion made
/// at the end and would not satisfy the case. The frame is drawn every tick for the same reason.
fn observe(policy: &str, port: Option<Box<dyn Proposer>>, export: &Path) -> (PaneReport, String) {
    const KEYS: [KeyCode; 5] = [
        KeyCode::Tab,
        KeyCode::Char('z'),
        KeyCode::Char('j'),
        KeyCode::PageUp,
        KeyCode::Char('f'),
    ];

    let destination = export.to_str().expect("a UTF-8 temporary path");
    let mut observer = Observer::with_port(observing(policy, destination), port).unwrap();

    // One operator act per tick, and the run is exactly as long as the sequence.
    for key in KEYS {
        observer.handle_key(press(key)).expect("no binding fails");
        observer.advance().expect("the source answers");
        let _ = drawn(&mut observer);
    }
    let tick = observer.snapshot().tick;
    let living = observer.snapshot().agents.len();

    // The selection, which is what the inspector presents: `Tab` moves it onto a Mokiterion.
    observer.handle_key(press(KeyCode::Tab)).expect("selecting");
    let selection = observer.selection().map(str::to_string);

    // The filter, and what it does to what the log presents.
    let before = observer.presented().len();
    observer
        .handle_key(press(KeyCode::Char('e')))
        .expect("the filter key");
    let after = observer.presented().len();
    let filter_label = observer.filter().label();

    // The export, written where the operator asked.
    observer
        .handle_key(press(KeyCode::Char('x')))
        .expect("the export key");
    let exported = fs::read(export).is_ok_and(|bytes| !bytes.is_empty());

    let buffer = drawn(&mut observer);
    let (width, height) = VIEWPORT;
    let panes = layout::resolve(Rect::new(0, 0, width, height));
    let roster = region(&buffer, panes.roster.expect("a roster pane at this width"));
    let inspector = region(&buffer, panes.inspector.expect("an inspector pane"));
    let log = region(&buffer, panes.log.expect("a log pane"));
    let footer = region(&buffer, panes.footer).trim_end().to_string();

    // The roster's accounting: the header states how many are living and how many it could not fit,
    // and the two together have to cover every Mokiterion the engine reports.
    let stated = figure(&roster, "living");
    let hidden = figure(&roster, "hidden").unwrap_or(0);
    let listed = u64::try_from(roster_entries(&roster).len()).expect("a small roster");
    let counted = u64::try_from(living).expect("a small population");

    let report = PaneReport {
        tick,
        roster_accounts_for_every_living: stated == Some(counted) && listed + hidden == counted,
        map_drew: ink(&region(&buffer, panes.view)) > 20,
        log_presented: log.contains("tick="),
        inspector_names_the_selection: selection
            .as_deref()
            .is_some_and(|id| observer.selected_agent().is_some() && inspector.contains(id)),
        filter_narrows: after <= before,
        filter_label,
        exported,
    };
    (report, footer)
}

/// A key press, in the shape the observer's own handler takes.
fn press(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

/// `VER-MOK-018` case **L31**: the observer replays this source to the transcript's horizon under
/// operator control, and its panes behave as they do under `social`.
///
/// The environment the case names first — *"no credential in the environment and no network
/// reachable"* — is a property of this file rather than an assertion in it: nothing here reads an
/// environment variable, opens a socket or spawns a process, and the port is the scripted stub the
/// file header describes. `SPEC-MOK-007` rule 12.2 is what makes that sufficient rather than a gap:
/// a replay reaches no provider *whether or not* a credential is present, in both hosts, so there is
/// no state of the environment this case would come out differently in.
///
/// **The comparison is a differential against `social`** at the same seed for the same five ticks,
/// because the case's own standard is *"as they do under `social`"*. An assertion written only under
/// this source would be satisfied by a host that treated it specially in some way the assertion did
/// not think to look at; a differential fails on any difference at all in the fields it reads.
/// [`PaneReport`] is what it reads, and the footer is compared field by field, since rule 8's
/// `events` count belongs to the run and not to the source.
#[test]
fn the_observer_replays_this_source_to_the_horizon_with_every_pane() {
    let directory = std::env::temp_dir().join("mokiterions-l31-observer-replay");
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("a writable temporary directory");

    let ledger = Rc::new(RefCell::new(Ledger::default()));
    let (ported, ported_footer) = observe(
        "llm",
        Some(Box::new(ScriptedPort::moving(&ledger))),
        &directory.join("llm.txt"),
    );
    let (deciding, deciding_footer) = observe("social", None, &directory.join("social.txt"));

    // The horizon, reached under the port, with every opportunity of it answered and recorded.
    assert_eq!(ported.tick, 5, "the replay stopped short of the horizon");
    assert_eq!(ledger.borrow().proposed.len(), 60);
    assert_eq!(ledger.borrow().recorded, 60);

    // Every pane, compared as a whole so that a single difference names itself in the failure.
    assert_eq!(
        ported, deciding,
        "a pane behaves differently under the model-backed source"
    );

    // Rule 8's footer names the fifth source, in the field that names `social`'s, and agrees with it
    // everywhere the run does not decide the value.
    let ported_fields = footer_fields(&ported_footer);
    let deciding_fields = footer_fields(&deciding_footer);
    assert_eq!(ported_fields.get("source").map(String::as_str), Some("llm"));
    assert_eq!(
        deciding_fields.get("source").map(String::as_str),
        Some("social")
    );
    assert_eq!(
        ported_fields.keys().collect::<Vec<_>>(),
        deciding_fields.keys().collect::<Vec<_>>(),
        "the footer presents a different set of fields under this source"
    );
    for field in ["seed", "ticks", "density", "tick"] {
        assert_eq!(
            ported_fields.get(field),
            deciding_fields.get(field),
            "the footer's {field} differs from `social`'s"
        );
    }

    // And none of it passed vacuously: every pane did something under both.
    assert!(
        ported.roster_accounts_for_every_living
            && ported.map_drew
            && ported.log_presented
            && ported.inspector_names_the_selection
            && ported.exported,
        "{ported:?}"
    );

    let _ = fs::remove_dir_all(&directory);
    println!(
        "the observer reached tick {} under both sources over 60 answered opportunit(ies); \
         panes {:?}; footer {:?}",
        ported.tick, ported, ported_footer
    );
}
