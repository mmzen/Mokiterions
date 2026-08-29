//! The connector: `SPEC-MOK-007` rules 10, 13 and 20.1, from outside the engine.
//!
//! This tier drives the built binary against the built canned connector, because what it asserts
//! about is the host: which process gets started, what it inherits, which file gets created, what
//! the operator is told, and what is left on disk afterwards. `SPEC-MOK-002` rule 9 keeps tests out
//! of `src/main.rs`, so the host's own behaviour is tested here; the port's reading of a response is
//! tested at the internal tier in `src/simulation.rs`, where a stream can be assembled in memory.
//!
//! **No provider, no network and no credential.** The connector is
//! `tests/support/canned_connector.rs`, which answers from a script, and the credential these tests
//! set is a value of their own invention in a variable of their own invention. `VER-MOK-018` case
//! `S2` is explicit that exercising it establishes nothing about an operator's connector — rule 10.6
//! says this specification cannot constrain a program the operator supplies — so what is checked
//! here is the **host's** half of every obligation and never the connector's.
//!
//! # Rule 13's two gates, as four rows
//!
//! Rule 13.1 puts the selection in the host and the credential in the connector, so that "neither
//! component can satisfy the other's condition". The four combinations and the test that takes each:
//!
//! | live selection | credential | outcome | test |
//! |---|---|---|---|
//! | absent | absent | nothing is spawned | `no_connector_is_spawned_without_the_live_selection` |
//! | absent | present | nothing is spawned | `no_connector_is_spawned_without_the_live_selection` |
//! | present | absent | spawned, refuses on the first exchange | `no_credential_refuses_on_the_first_exchange` |
//! | present | present | spawned, answers | `a_live_run_spawns_the_connector_and_records_every_exchange` |
//!
//! The two absent-selection rows are one test because rule 13.2's refusal does not consult the
//! credential and must not: a host whose behaviour changed with a variable it claims never to read
//! would be reading it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const ENGINE: &str = env!("CARGO_BIN_EXE_Mokiterions");

/// The canned connector's own path, which exists because `mokiterions-core/Cargo.toml` declares it
/// as a `[[bin]]`. That declaration is what makes rule 10.1's "operator-named executable spawned as
/// a child process" reachable from a test at all.
const CONNECTOR: &str = env!("CARGO_BIN_EXE_canned-connector");

/// The variable the canned connector reads its script from.
///
/// It is set on the **engine**, never on this test process, and the connector nevertheless sees it.
/// That is the environment pass-through of rule 10.5 demonstrated without a credential in sight: a
/// child inherits the spawning process's environment, `src/main.rs` makes no environment call on the
/// builder, and every scripted test in this file depends on the inheritance working.
const SCRIPT_VARIABLE: &str = "MOKITERIONS_CANNED_SCRIPT";

/// The variable the `credential` directive reads, invented here.
///
/// No artifact fixes a name — rule 13.3 leaves the variable to the connector — so a test that
/// asserted one would be asserting something this repository does not decide.
const CREDENTIAL_VARIABLE: &str = "MOKITERIONS_TEST_CREDENTIAL";

/// A value of this test's own, shaped like a key so that a leak detector has something to find.
///
/// It authenticates nothing and reaches nothing: the canned connector compares it against emptiness
/// and makes no call. `the_credential_value_reaches_no_produced_byte` is what holds the four
/// prohibitions — the repository, the library target, every workflow, every produced byte — to the
/// one place a value exists in this suite at all.
const CREDENTIAL: &str = "sk-canned-0000-authenticates-nothing";

/// Two US dollars, which is `WO-MOK-026`'s fixed ceiling, and the prices from rule 14.3a's example.
///
/// Both are required by the parser for any live run and neither is exercised here: the ceiling is
/// item 10's and the arithmetic is item 9's. They appear because a live invocation is invalid
/// without them.
const CEILING: &str = "2";
const PRICES: &str = "125:13:1000:0";

/// An empty directory of this test's own, removed and recreated so a run cannot inherit a file from
/// a previous one. `tests/replay.rs`'s helper, with this file's own prefix.
fn scratch(label: &str) -> PathBuf {
    let directory = std::env::temp_dir().join(format!("mokiterions-connector-{label}"));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).unwrap();
    directory
}

/// A script file for the canned connector. Its last directive repeats, so two lines drive a
/// twelve-exchange run.
fn script(directory: &Path, lines: &[&str]) -> PathBuf {
    let path = directory.join("script");
    fs::write(&path, lines.join("\n")).unwrap();
    path
}

/// A live invocation of the built engine, with everything `--live` requires and nothing else.
///
/// One tick, because one tick is twelve decision opportunities at the default density — enough for
/// every property in this file, including the ones that need a second exchange to differ from the
/// first.
///
/// The two variables are set or removed explicitly rather than left to whatever this test process
/// happens to hold, which is what makes the absent cases mean "absent" rather than "probably absent".
fn engine(
    connector: &str,
    transcript: &Path,
    script: Option<&Path>,
    credential: Option<&str>,
) -> Command {
    let mut command = Command::new(ENGINE);
    command.args([
        "--policy",
        "llm",
        "--live",
        "--seed",
        "42",
        "--ticks",
        "1",
        "--spend-ceiling",
        CEILING,
        "--prices",
        PRICES,
    ]);
    command.arg("--connector-path").arg(connector);
    command.arg("--transcript-output").arg(transcript);
    match script {
        Some(path) => command.env(SCRIPT_VARIABLE, path),
        None => command.env_remove(SCRIPT_VARIABLE),
    };
    match credential {
        Some(value) => command.env(CREDENTIAL_VARIABLE, value),
        None => command.env_remove(CREDENTIAL_VARIABLE),
    };
    command
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

/// The exchange records of a transcript, in order.
fn exchanges(transcript: &str) -> Vec<&str> {
    transcript
        .lines()
        .filter(|line| line.contains("\"transcript\":\"exchange\""))
        .collect()
}

/// The twelve Mokiterions the default density produces at every declared seed, which is the number
/// of prefix records and the number of opportunities in one tick.
const ROSTER: usize = 12;

/// Rules 10.1, 10.2, 13.1 and 20.1: a live run starts the connector, exchanges with it over the two
/// pipes, and records every exchange.
///
/// The fourth row of the gate table: both conditions met. The assertions are the host's three
/// obligations in one run — the child was started (nothing else could have answered), the framing
/// worked in both directions (twelve answers arrived in order), and rule 19.6's transcript exists
/// with a record for every exchange.
///
/// `fallback:false` on all twelve is the load-bearing one. A host that spawned the connector and
/// then failed to parse its answers would still exit `0` with a full transcript under rule 9.5, so
/// the count of records proves the spawn and only the flag proves the reading.
#[test]
fn a_live_run_spawns_the_connector_and_records_every_exchange() {
    let directory = scratch("live");
    let transcript = directory.join("live.jsonl");
    let script = script(&directory, &["ok wait"]);

    let output = engine(CONNECTOR, &transcript, Some(&script), Some(CREDENTIAL))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(stderr(&output).is_empty(), "{}", stderr(&output));
    let text = stdout(&output);
    assert!(
        text.contains("event=decision_source_selected result=source:llm"),
        "{text}"
    );

    let recorded = fs::read_to_string(&transcript).unwrap();
    assert_eq!(
        recorded.lines().count(),
        ROSTER + ROSTER,
        "one prefix per Mokiterion and one exchange per opportunity"
    );
    let exchanges = exchanges(&recorded);
    assert_eq!(exchanges.len(), ROSTER);
    for record in &exchanges {
        assert!(record.contains("\"fallback\":false"), "{record}");
        assert!(record.contains("\"verb\":\"wait\""), "{record}");
        // Rule 11.5's counts, as the connector reported them and not as an estimate: the fixture's
        // defaults, arriving through the port's reading of the `usage` member.
        assert!(
            record.contains(
                "\"usage\":{\"prompt\":1000,\"cached_prompt\":900,\"output\":8,\"reasoning\":0}"
            ),
            "{record}"
        );
        // Rule 11.3: the response as received, in full. The model identifier is in it because the
        // connector put it there, which is also rule 10.4c's grammar check having passed.
        assert!(record.contains("canned-connector"), "{record}");
    }
}

/// Rule 13.2: **with no live selection nothing is spawned at all** — not spawned and refused, not
/// started and told to make no call.
///
/// The first two rows of the gate table, and the mechanism is the point: the connector path names a
/// program that cannot exist, so a host that spawned it would fail with exit `1` and say so. The run
/// exits `0` instead, which it can only do by never having reached the platform with that path.
///
/// A replay is used because rule 18.4.3 admits `--connector-path` only under `--policy llm` and rule
/// 13.2 gives that source a transcript or nothing. The transcript replayed is the one the test above
/// records, at the same seed and horizon, so the run has real decisions to reach — a refusal for
/// some other reason would not demonstrate anything about the spawn.
///
/// Both credential rows are taken in one loop. Rule 13.1 puts the credential in a component this one
/// cannot inspect, so the two must be indistinguishable here, and a difference between the iterations
/// would mean the host had looked.
#[test]
fn no_connector_is_spawned_without_the_live_selection() {
    let directory = scratch("unselected");
    let transcript = directory.join("recorded.jsonl");
    let script = script(&directory, &["ok wait"]);
    let recorded = engine(CONNECTOR, &transcript, Some(&script), Some(CREDENTIAL))
        .output()
        .unwrap();
    assert_eq!(recorded.status.code(), Some(0), "{}", stderr(&recorded));

    // A path with nothing at it. Well formed, so the parser accepts it, and unstartable, so a host
    // that reached the platform with it would fail loudly.
    let unstartable = directory.join("no-such-connector");

    for credential in [None, Some(CREDENTIAL)] {
        let mut command = Command::new(ENGINE);
        command.args(["--policy", "llm", "--seed", "42", "--ticks", "1"]);
        command.arg("--transcript-path").arg(&transcript);
        command.arg("--connector-path").arg(&unstartable);
        match credential {
            Some(value) => command.env(CREDENTIAL_VARIABLE, value),
            None => command.env_remove(CREDENTIAL_VARIABLE),
        };
        let output = command.output().unwrap();

        assert_eq!(
            output.status.code(),
            Some(0),
            "with credential {}: {}",
            credential.is_some(),
            stderr(&output)
        );
        assert!(stderr(&output).is_empty(), "{}", stderr(&output));
        // And the replay produced the recording's own bytes, so the decisions came from the file.
        assert_eq!(stdout(&output), stdout(&recorded));
    }
}

/// Rule 13.3: with no credential the connector makes no provider call and answers an error **on the
/// first exchange**, in its own terms, naming no value.
///
/// The third row of the gate table. The host's half is all this can check, and it is the half worth
/// checking: an error response is a counted fallback under rules 9.5 and 19.5a and **not** an abort
/// under rule 9.8, so the run has real ticks, exits `0` under rule 19.1, and leaves a transcript in
/// which every exchange is visible as a fallback.
///
/// "On the first exchange" is asserted as the first record rather than as a timestamp: the connector
/// refuses before answering anything, so record one already carries the refusal.
#[test]
fn no_credential_refuses_on_the_first_exchange() {
    let directory = scratch("ungated");
    let transcript = directory.join("refused.jsonl");
    let script = script(&directory, &[&format!("credential {CREDENTIAL_VARIABLE}")]);

    let output = engine(CONNECTOR, &transcript, Some(&script), None)
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let recorded = fs::read_to_string(&transcript).unwrap();
    let exchanges = exchanges(&recorded);
    assert_eq!(exchanges.len(), ROSTER);
    for record in &exchanges {
        assert!(record.contains("\"fallback\":true"), "{record}");
        // Rule 9.5's fallback is `wait` and rule 9.7 forbids a substitute from another source.
        assert!(record.contains("\"verb\":\"wait\""), "{record}");
        // The connector's own error, recorded as received. `refused` is one of rule 19.5a's four
        // kinds and the one that is not retried, which is why a run with no credential does not
        // spend four attempts per opportunity discovering the same thing twelve times.
        assert!(record.contains("refused"), "{record}");
    }
    // The refusal is the *first* thing the connector says, not something it reaches after answering.
    assert!(exchanges[0].contains("refused"), "{}", exchanges[0]);
}

/// `REQ-MOK-073`, `ADR-MOK-001` and `VER-MOK-018` case `C1`: the credential's value reaches no
/// produced byte.
///
/// Every byte this run produces is searched — the transcript, the record stream, standard output and
/// standard error — for the value the connector was given. The variable's *name* is expected to
/// appear in the refusal case and does not appear here; rule 13.3 has a connector name the variable
/// and never its value, which is exactly the distinction this test rests on.
///
/// This is the run in which a credential exists at all in this suite, which is why the search is
/// here and not spread across the file: the other tests have nothing to leak.
#[test]
fn the_credential_value_reaches_no_produced_byte() {
    let directory = scratch("quiet");
    let transcript = directory.join("quiet.jsonl");
    let events = directory.join("quiet-records.jsonl");
    let script = script(&directory, &[&format!("credential {CREDENTIAL_VARIABLE}")]);

    let output = engine(CONNECTOR, &transcript, Some(&script), Some(CREDENTIAL))
        .arg("--events-path")
        .arg(&events)
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    // The credential was present, so the connector answered rather than refusing: the run this
    // searches is one in which the value was actually used for something.
    let recorded = fs::read_to_string(&transcript).unwrap();
    for record in exchanges(&recorded) {
        assert!(record.contains("\"fallback\":false"), "{record}");
    }

    for (name, produced) in [
        ("the transcript", recorded.clone()),
        ("the record stream", fs::read_to_string(&events).unwrap()),
        ("standard output", stdout(&output)),
        ("standard error", stderr(&output)),
    ] {
        assert!(
            !produced.contains(CREDENTIAL),
            "the credential's value appears in {name}"
        );
        // And no fragment of it either, which is what a truncated or partially escaped leak would
        // look like. Twelve characters is short enough to catch a prefix and long enough not to
        // match text a run legitimately produces.
        assert!(
            !produced.contains(&CREDENTIAL[..12]),
            "a fragment of the credential appears in {name}"
        );
    }
}

/// Rule 19.6 and the transcript's own irreplaceability: an existing destination is refused, and the
/// file that was there is untouched.
///
/// Stricter than `--events-path`, which replaces what it finds, and the asymmetry is deliberate: a
/// live transcript is evidence somebody paid for, and replacing one silently is the only filesystem
/// outcome in this target that cannot be re-derived without spending money again.
///
/// The refusal happens after the spawn, which is why this also checks that the run does not hang:
/// the child is reaped rather than left holding a pipe.
#[test]
fn an_existing_transcript_destination_is_refused_and_left_untouched() {
    let directory = scratch("occupied");
    let transcript = directory.join("already-here.jsonl");
    let script = script(&directory, &["ok wait"]);
    fs::write(&transcript, b"an earlier run's evidence\n").unwrap();

    let output = engine(CONNECTOR, &transcript, Some(&script), Some(CREDENTIAL))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let complaint = stderr(&output);
    assert!(complaint.contains("transcript"), "{complaint}");
    // A runtime failure and not invalid configuration: the options were well formed and the
    // platform refused, so the usage text does not follow it.
    assert!(!complaint.contains("Usage:"), "{complaint}");
    assert_eq!(
        fs::read_to_string(&transcript).unwrap(),
        "an earlier run's evidence\n"
    );
    // Nothing ran, so nothing was recorded to standard output either.
    assert!(!stdout(&output).contains("tick="), "{}", stdout(&output));
}

/// A connector the platform cannot start fails before the transcript exists.
///
/// The order of the host's two acts is what this checks. The spawn comes first, so a connector that
/// will not start leaves the filesystem exactly as it was — an empty transcript from a run that
/// never exchanged anything would be evidence of nothing, and would then have to be removed by the
/// same logic rule 13.4 confines to the record sink.
#[test]
fn a_connector_that_cannot_be_started_creates_no_transcript() {
    let directory = scratch("unstartable");
    let transcript = directory.join("never.jsonl");
    let missing = directory.join("no-such-connector");

    let output = engine(
        missing.to_str().unwrap(),
        &transcript,
        None,
        Some(CREDENTIAL),
    )
    .output()
    .unwrap();

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let complaint = stderr(&output);
    assert!(complaint.contains("connector"), "{complaint}");
    assert!(!complaint.contains("Usage:"), "{complaint}");
    assert!(!transcript.exists(), "a transcript survived a failed spawn");
}

/// A connector that dies mid-run leaves a run of fallbacks and the run still exits `0`.
///
/// Rule 9.8 forbids aborting on an exchange that yielded nothing, and rule 19.1 fixes the exit code
/// at `0` for a run that reached its horizon. So a connector that answers once and then closes its
/// output produces one real decision and eleven fallbacks, and the run is a real run — which is the
/// whole reason the fallback is `wait` and not a failure.
///
/// The script's last directive repeats, so `close` on line two applies to every exchange from the
/// second onwards.
#[test]
fn a_connector_that_dies_mid_run_leaves_a_run_of_fallbacks() {
    let directory = scratch("dies");
    let transcript = directory.join("truncated.jsonl");
    let script = script(&directory, &["ok wait", "close"]);

    let output = engine(CONNECTOR, &transcript, Some(&script), Some(CREDENTIAL))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let recorded = fs::read_to_string(&transcript).unwrap();
    let exchanges = exchanges(&recorded);
    assert_eq!(
        exchanges.len(),
        ROSTER,
        "every opportunity is recorded, answered or not"
    );
    assert!(
        exchanges[0].contains("\"fallback\":false"),
        "{}",
        exchanges[0]
    );
    for record in &exchanges[1..] {
        assert!(record.contains("\"fallback\":true"), "{record}");
        // Rule 11.3's "or the error": the response field carries what the port has instead of a
        // line, which for a closed pipe is the platform's own reason.
        assert!(record.contains("connector"), "{record}");
    }
    // The connector exited `0` — it closed its output deliberately — so there is nothing to report
    // about its status.
    assert!(stderr(&output).is_empty(), "{}", stderr(&output));
}

/// A connector that exits badly is reported, and the exit code does not move.
///
/// Rule 19.1 fixes this target's exit code to the run's own outcome, and the connector's status is
/// not the engine's failure: a run that reached its horizon is a successful run whatever the child
/// did on its way out. The status is still worth telling the operator, because a connector exiting
/// `2` is why their run was twelve fallbacks.
///
/// The failure is induced through the fixture's own contract — a script it cannot read is a fixture
/// defect and it exits `2` — so this needs no second program.
#[test]
fn a_connector_that_exits_badly_is_reported_without_changing_the_exit_code() {
    let directory = scratch("badly");
    let transcript = directory.join("empty.jsonl");
    let missing = directory.join("no-such-script");

    let output = engine(CONNECTOR, &transcript, Some(&missing), Some(CREDENTIAL))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let complaint = stderr(&output);
    assert!(complaint.contains("connector"), "{complaint}");
    assert!(complaint.contains("status 2"), "{complaint}");
    // Neither of the two severity keywords the artifacts fix, because this is neither.
    assert!(!complaint.contains("runtime error:"), "{complaint}");
    assert!(!complaint.contains("configuration error:"), "{complaint}");
    // Every opportunity is a fallback, and the transcript says so rather than being absent.
    let recorded = fs::read_to_string(&transcript).unwrap();
    assert_eq!(exchanges(&recorded).len(), ROSTER);
    for record in exchanges(&recorded) {
        assert!(record.contains("\"fallback\":true"), "{record}");
    }
}

/// Rule 10.4c: a response that answers and names neither what answered nor at what level fails the
/// grammar check and becomes a counted fallback.
///
/// The host's reading of an untrusted line, from outside. Rule 10.7 declares a connector's output
/// untrusted **in whole**, and the two fields are the ones a host cannot supply for itself: a run
/// record that named no model would be an account of spending with nothing to attribute it to.
///
/// The line is otherwise perfectly well formed — the protocol, an action, a legal verb — so what is
/// checked is the grammar rule and not the parser.
#[test]
fn a_response_that_names_neither_model_nor_level_is_a_fallback() {
    let directory = scratch("nameless");
    let transcript = directory.join("nameless.jsonl");
    let script = script(
        &directory,
        &["malformed {\"protocol\":1,\"action\":{\"verb\":\"sleep\"}}"],
    );

    let output = engine(CONNECTOR, &transcript, Some(&script), Some(CREDENTIAL))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let recorded = fs::read_to_string(&transcript).unwrap();
    for record in exchanges(&recorded) {
        assert!(record.contains("\"fallback\":true"), "{record}");
        // `wait` and not the `sleep` the line proposed: a response that fails the grammar check
        // yields no action at all, and rule 9.5's fallback is `wait`.
        assert!(
            record.contains("\"action\":{\"verb\":\"wait\"}"),
            "{record}"
        );
        // And the line is recorded as received, so a reader can see what was refused.
        assert!(record.contains("sleep"), "{record}");
    }
}
