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
use std::io;
use std::rc::Rc;

use mokiterions::simulation::{Action, DecisionRequest, Proposer};
use mokiterions_tui::options::{self, Options, Startup};
use mokiterions_tui::state::{Observer, Progression};

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
}

impl ScriptedPort {
    fn new(ledger: &Rc<RefCell<Ledger>>) -> Self {
        Self {
            ledger: Rc::clone(ledger),
            failing_at: None,
        }
    }

    fn failing_at(ledger: &Rc<RefCell<Ledger>>, opportunity: usize) -> Self {
        Self {
            ledger: Rc::clone(ledger),
            failing_at: Some(opportunity),
        }
    }
}

impl Proposer for ScriptedPort {
    fn propose(&mut self, request: DecisionRequest) -> Option<Action> {
        self.ledger
            .borrow_mut()
            .proposed
            .push((request.tick(), request.actor_id().to_string()));
        Some(Action::Wait)
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
