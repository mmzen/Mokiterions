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
/// Both are required by the parser for any live run. The prices' arithmetic is item 9's and is
/// checked at the internal tier, where the figures can be stated as integers rather than parsed out
/// of a string; **this ceiling is never reached**, because the operator declares dollars and rule
/// 14.2 accounts in cents, so two dollars is two hundred cents against a tick holding twelve
/// exchanges. The one case that reaches a ceiling declares [`REACHABLE_CEILING`] instead.
const CEILING: &str = "2";
const PRICES: &str = "125:13:1000:0";

/// Two US cents, as the operator writes them: the same option as [`CEILING`] with a figure one tick
/// can spend.
///
/// The two decimal places are load-bearing and are the reason this is a string rather than a number.
/// `--spend-ceiling` takes an amount in dollars and `cli::parse_minor_units` converts it to rule
/// 14.2's cents, so the smallest ceiling an operator can declare is `0.01` and this one is twice it —
/// two exchanges at [`WHOLE_CENT_EXCHANGE`]'s usage, which is case **L19**'s figure exactly.
const REACHABLE_CEILING: &str = "0.02";

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
    spending(connector, transcript, script, credential, CEILING)
}

/// [`engine`] with a ceiling of the case's own choosing.
///
/// The ceiling is a parameter rather than an argument the caller appends, because `--spend-ceiling`
/// may appear at most once: a second one is a configuration error and a test that added it would be
/// asserting on the parser's refusal instead of on the run.
fn spending(
    connector: &str,
    transcript: &Path,
    script: Option<&Path>,
    credential: Option<&str>,
    ceiling: &str,
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
        ceiling,
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

/// `SPEC-MOK-007` rule 15's run record, from the standard error of a live run.
///
/// **Standard error is the destination, decided by the repository owner on 2026-08-29** over a sixth
/// command-line option and over the structured record stream. Rule 15 leaves the destination to the
/// host and names none; rule 12.6 claims byte-identity between a replay and the recorded run for
/// standard output, the record stream and the exit code, and says outright that it is "not claimed
/// for standard error" — so this is the one stream on which a live-only line breaks nothing.
///
/// It is found by its own field and not by position, because `src/main.rs` reaps the connector after
/// the run and a child that exited badly is reported on this same stream. A record located as "the
/// last line" would then be the reaping message for exactly the runs where the connector misbehaved.
///
/// The field it is found by is `run_record` and **not** `SPEC-MOK-006` rule 8's `"record":"run"`. The
/// two are different records with different fields on different streams, and the spelling keeps them
/// apart: the record-stream one is written per rule 8.1 by the engine's own recording path, and this
/// one is rule 15's account of a live run's spending.
///
/// The panic is the assertion: every caller is a live run, and rule 15.1 makes the record owed.
fn run_record(output: &Output) -> String {
    let stderr = stderr(output);
    let mut found = stderr
        .lines()
        .filter(|line| line.contains("\"run_record\":"));
    let record = found
        .next()
        .unwrap_or_else(|| {
            panic!("rule 15: a live run reports a run record, and this one did not: {stderr}")
        })
        .to_string();
    // One run record per run, checked here rather than assumed, because the host writes this line at
    // two exits and a host that wrote it at both would satisfy every assertion below twice.
    assert!(found.next().is_none(), "two run records: {stderr}");
    record
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

/// A directive whose usage costs exactly one cent at [`PRICES`]: a thousand prompt tokens all served
/// from the cache at 13 microcents each, and 987 output tokens at 1,000 each.
///
/// One cent exactly, and that is the point: [`REACHABLE_CEILING`] is two of them, so the second
/// exchange reaches the ceiling and the third is never issued. A directive that cost a fraction of a
/// cent — the fixture's own default usage costs 0.0322 of one — would need thirty-two exchanges and
/// three ticks to reach the smallest declarable ceiling, and the count the case names would then
/// depend on how many Mokiterions were still alive in the third tick.
const WHOLE_CENT_EXCHANGE: &str = "ok wait prompt=1000 cached=1000 output=987 reasoning=0";

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
    // Rule 15.1 and the whole of what standard error carries: **the run record and nothing else.**
    // This assertion was `stderr.is_empty()` until 2026-08-29, and the equality below is that claim
    // kept rather than weakened — a successful live run's diagnostic stream is one line long and the
    // line is the account of what the run spent. A message of any other kind reaching this stream
    // still fails here.
    assert_eq!(
        stderr(&output),
        format!("{}\n", run_record(&output)),
        "a successful live run says one thing on standard error"
    );
    let text = stdout(&output);
    assert!(
        text.contains("event=decision_source_selected result=source:llm"),
        "{text}"
    );

    // Rule 15.2's figures reaching the host, against the twelve exchanges this run made at the
    // fixture's default usage. Multiplied out rather than written as constants, so a change to
    // `ROSTER` or to the connector's defaults moves the expectation with the run.
    let record = run_record(&output);
    for figure in [
        format!("\"exchanges\":{ROSTER}"),
        // The four totals as one block, and not as four substrings: `"reasoning"` appears twice in
        // this record — the level as a string and the token count as an integer, which is rule
        // 10.4a's two levels — so a bare `"reasoning":0` would be an assertion about whichever the
        // formatter happened to put first.
        format!(
            "\"tokens\":{{\"prompt\":{},\"cached_prompt\":{},\"output\":{},\"reasoning\":0}}",
            1_000 * ROSTER,
            900 * ROSTER,
            8 * ROSTER
        ),
        // Rule 14.5's ratio: nine hundred of every thousand prompt tokens came from the cache.
        String::from("\"cache_ratio_basis_points\":9000"),
        // Rule 15.3's zeros stated, and rule 15.4's mark absent because the count is.
        String::from("\"fallbacks\":0"),
        String::from("\"unfit_to_publish\":false"),
        // Rule 15.2's inputs, echoed: the seed and the horizon `engine` declares.
        String::from("\"seed\":42"),
        String::from("\"ticks\":1"),
        // The ceiling as rule 14.2's minor unit, which is `REACHABLE_CEILING`'s own reasoning read
        // the other way: the operator declared two dollars and the record states two hundred cents.
        String::from("\"ceiling_cents\":200"),
        // Rule 15.5's other ending, so the record says which one this was.
        String::from("\"ended\":\"tick_limit\""),
        String::from("\"tick_reached\":1"),
    ] {
        assert!(
            record.contains(&figure),
            "{figure} is missing from {record}"
        );
    }
    // **Rule 15.2's model identifier and reasoning level, retained from the connector's own
    // response.** This is the one assertion in the suite that shows the binding travelling: the
    // canned connector names itself in every response's `model` member, the port keeps the first one
    // it saw, and the record states it. A port that read the field and dropped it would pass every
    // other assertion above.
    assert!(
        record.contains("\"model\":\"canned-connector\""),
        "{record}"
    );
    assert!(record.contains("\"reasoning\":\"none\""), "{record}");

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
    // The live run that produced the transcript reported a run record, so the emptiness asserted in
    // the loop below is rule 15.6's difference between two runs rather than a feature nobody built.
    let _ = run_record(&recorded);

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
        // **Rule 15.6, and this is where it is worth measuring: a replay reports no run record.**
        // The recording above wrote one to this same stream, so the emptiness here is a difference
        // between the two runs and not the absence of a feature. It holds without a branch anywhere
        // asking which kind of run this was — `ReplayPort` takes `Proposer::accounting`'s default and
        // answers `None`, so there is nothing that had to guess.
        assert!(stderr(&output).is_empty(), "{}", stderr(&output));
        // And the replay produced the recording's own bytes, so the decisions came from the file.
        // Rule 12.6, whose byte-identity is claimed for standard output and *not* for standard error
        // — which is the whole reason the record above may differ between the two runs.
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

/// Rule 13.3: an **empty** credential is treated as absent and takes the arm absence takes.
///
/// The rule gives "absent, empty or malformed" one treatment and the connector implements the three
/// as one arm, guarded by whether the value it read is empty. `no_credential_refuses_on_the_first_exchange`
/// above enters that arm by removing the variable altogether, which leaves the guard itself
/// unexercised — and a connector that answered on an empty credential would pass every other test
/// in this file while sending a request with no authentication on it.
///
/// The assertion is that the outcome is the one absence produces, not merely that something failed:
/// the same exit code, the same fallback on every exchange, and the refusal on the first. A weaker
/// test that only checked for an error would pass against a connector that failed for an unrelated
/// reason.
///
/// `VER-MOK-018` case `L20` asks for a malformed credential as well, which `WO-MOK-032` records as a
/// stated deviation rather than covering. A non-Unicode value has no portable constructor, so
/// reaching it would need the first platform-conditional code in either package — to enter the same
/// arm this test already enters.
#[test]
fn an_empty_credential_is_treated_as_absent() {
    let directory = scratch("empty-credential");
    let transcript = directory.join("refused.jsonl");
    let script = script(&directory, &[&format!("credential {CREDENTIAL_VARIABLE}")]);

    let output = engine(CONNECTOR, &transcript, Some(&script), Some(""))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let recorded = fs::read_to_string(&transcript).unwrap();
    let exchanges = exchanges(&recorded);
    assert_eq!(exchanges.len(), ROSTER);
    for record in &exchanges {
        assert!(record.contains("\"fallback\":true"), "{record}");
        assert!(record.contains("\"verb\":\"wait\""), "{record}");
        assert!(record.contains("refused"), "{record}");
    }
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
    // about its status, and rule 15's record is therefore the whole of standard error.
    let record = run_record(&output);
    assert_eq!(
        stderr(&output),
        format!("{record}\n"),
        "the connector exited cleanly, so nothing but the record is reported"
    );
    // Rules 15.4 and 14.1 over a run that mostly failed: eleven of the twelve opportunities fell
    // back, the record marks the run unfit, and the one exchange that answered is the only one that
    // billed. A dead pipe reports no usage — rule 11.5's absence is not a zero — so the totals are
    // one exchange's and the count of exchanges is still twelve, because every opportunity spent one.
    assert!(
        record.contains(&format!("\"fallbacks\":{}", ROSTER - 1)),
        "{record}"
    );
    assert!(record.contains("\"unfit_to_publish\":true"), "{record}");
    assert!(
        record.contains(&format!("\"exchanges\":{ROSTER}")),
        "{record}"
    );
    assert!(
        record.contains(
            "\"tokens\":{\"prompt\":1000,\"cached_prompt\":900,\"output\":8,\"reasoning\":0}"
        ),
        "{record}"
    );
    // The binding survives the pipe's death, which is the "first reported, never replaced" decision
    // measured where it bites: one response named the model and eleven failures named nothing, and
    // the record still states what answered.
    assert!(
        record.contains("\"model\":\"canned-connector\""),
        "{record}"
    );
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

/// `SPEC-MOK-007` rules 14.6, 14.7 and 19.3, and `SPEC-MOK-006` rule 13.4's exception: a live run
/// that reaches its ceiling exits with a status of its own and leaves everything it wrote behind.
///
/// **Three assertions, and the middle one is the reason this test is at this tier.** The exit status
/// is `3`, which rule 19.3 requires to be distinct from a clean completion and from an error and
/// which the library names `CEILING_STOP_EXIT`; the record stream *survives*, which no library-tier
/// test can check because rule 13.4's removal is this host's act and nobody else's; and exactly two
/// exchanges were issued, which is case **L19**'s figure reached through a real child process
/// reporting its own usage rather than through a fixture declaring it.
///
/// Rule 13.4 has this host remove a record sink it created when the run failed, and rule 14.7
/// requires a ceiling-stopped stream to survive complete and readable to the tick reached. The two
/// disagree only in appearance: a ceiling stop is not a failure, and the host tells them apart by the
/// library's exit code. A host that compared against `0` alone would delete the evidence of every
/// ceiling-stopped run.
///
/// What the surviving stream does *not* carry is asserted too. There is no `SPEC-MOK-006` rule 8
/// run record and no `simulation_ended` event, because the run did not end — rule 8.9's `reason`
/// domain has no member for a stop and rule 15.5 forbids quoting a figure at a horizon the run did
/// not reach — and there is no summary line on standard output for the same reason.
///
/// **Rule 15's own record is present, on standard error, and it says the run stopped.** That is the
/// fourth assertion as of 2026-08-29 and it is what makes the third paragraph's absences legible: the
/// record stream is short of a rule 8 run record and the operator is nevertheless told the whole
/// account, so nothing here is a stream a reader mistakes for a complete run.
#[test]
fn a_live_run_stops_at_its_ceiling_and_leaves_the_record_stream_behind() {
    let directory = scratch("ceiling");
    let transcript = directory.join("ceiling.jsonl");
    let events = directory.join("ceiling-records.jsonl");
    let script = script(&directory, &[WHOLE_CENT_EXCHANGE]);

    let output = spending(
        CONNECTOR,
        &transcript,
        Some(&script),
        Some(CREDENTIAL),
        REACHABLE_CEILING,
    )
    .arg("--events-path")
    .arg(&events)
    .output()
    .unwrap();

    // Rule 19.3's fourth status. The literal is the contract a caller sees, which is why it is
    // written out here rather than imported: this tier asserts `0`, `1` and `2` the same way, and a
    // status a caller cannot predict is not a status.
    assert_eq!(output.status.code(), Some(3), "{}", stderr(&output));
    let complaint = stderr(&output);
    assert!(
        complaint.contains("spend ceiling reached at tick 1"),
        "{complaint}"
    );
    // Neither severity keyword: a ceiling stop is the run doing what it was asked.
    assert!(!complaint.contains("runtime error:"), "{complaint}");
    assert!(!complaint.contains("configuration error:"), "{complaint}");

    // Rule 15.5, both halves, through a real child that reported its own usage: the record names the
    // stop and states the tick reached. The cost is at the declared ceiling and not above it, which
    // is the *stop* rather than the report — `REQ-MOK-071`'s distinction, and a run that overshot
    // would state a larger figure here while passing every other assertion in this test.
    let record = run_record(&output);
    assert!(record.contains("\"ended\":\"ceiling\""), "{record}");
    assert!(record.contains("\"tick_reached\":1"), "{record}");
    assert!(record.contains("\"cost_cents\":2"), "{record}");
    assert!(record.contains("\"ceiling_cents\":2"), "{record}");
    assert!(record.contains("\"exchanges\":2"), "{record}");
    // The two lines standard error carries, and only those two: the note and the record.
    assert_eq!(
        complaint.lines().count(),
        2,
        "the ceiling note and the run record: {complaint}"
    );

    // Rule 14.6 as a count, and rule 13.4's exception: both files the host created are still here.
    let recorded = fs::read_to_string(&transcript).unwrap();
    assert_eq!(
        exchanges(&recorded).len(),
        2,
        "two cents at a cent an exchange, and the third is never issued"
    );
    assert_eq!(
        recorded
            .lines()
            .filter(|line| line.contains("\"transcript\":\"prefix\""))
            .count(),
        ROSTER,
        "the prefix head is complete: it is written before the first exchange"
    );
    for record in exchanges(&recorded) {
        assert!(record.contains("\"fallback\":false"), "{record}");
    }

    let stream = fs::read_to_string(&events).expect("the record stream survived the ceiling stop");
    assert!(
        stream
            .lines()
            .next()
            .unwrap()
            .contains("\"record\":\"header\""),
        "{stream}"
    );
    assert!(!stream.contains("\"record\":\"run\""), "{stream}");
    // And rule 15's record did not leak into this stream either, which is the destination decision
    // asserted where it would fail: rule 12.6 requires a replay to reproduce these bytes and a
    // replay reports no run record, so a rule 15 record here would make the two rules contradict.
    assert!(!stream.contains("\"run_record\":"), "{stream}");
    assert!(!stream.contains("simulation_ended"), "{stream}");
    for line in stream.lines() {
        assert!(line.starts_with('{') && line.ends_with('}'), "{line}");
    }
    assert!(
        !stdout(&output).contains("summary reason="),
        "{}",
        stdout(&output)
    );
}

/// `SPEC-MOK-007` rule 19.5 and `VER-MOK-018` case **R1**: a transport failure is retried a bounded
/// number of times and each attempt appears as its own transcript record.
///
/// The stub is the canned connector with a script that fails a fixed number of times, which is what
/// the case asks for: two transport errors and then an answer, so the first opportunity spends three
/// exchanges and every opportunity after it spends one. Fourteen records for twelve opportunities is
/// the whole claim — a host that retried nothing would write twelve, and one that retried without
/// recording the attempts would also write twelve.
///
/// **No record is marked as a fallback, and that is the half rule 15.4 rests on.** The opportunity
/// reached a decision, so nothing about it was rule 9.5's fallback: the two abandoned attempts are
/// marked `false` because they were not the decision, and the third is marked `false` because it
/// carried an action. A run that retried and succeeded is a clean run.
#[test]
fn a_transport_failure_is_retried_and_every_attempt_is_recorded() {
    let directory = scratch("retried");
    let transcript = directory.join("retried.jsonl");
    let script = script(
        &directory,
        &[
            "error transport the socket closed",
            "error transport the socket closed",
            "ok wait",
        ],
    );

    let output = engine(CONNECTOR, &transcript, Some(&script), Some(CREDENTIAL))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let recorded = fs::read_to_string(&transcript).unwrap();
    let exchanges = exchanges(&recorded);
    assert_eq!(
        exchanges.len(),
        ROSTER + 2,
        "twelve opportunities and two abandoned attempts"
    );
    for record in &exchanges {
        assert!(record.contains("\"fallback\":false"), "{record}");
    }

    // The three records of the first opportunity: same tick, same actor, and the failures ahead of
    // the answer. The actor is read from the first record rather than named, because which
    // Mokiterion is asked first is the engine's ordering and not this test's business.
    let first_actor = exchanges[0]
        .split_once("\"actor\":\"")
        .and_then(|(_, rest)| rest.split_once('"'))
        .map(|(actor, _)| actor)
        .expect("every record names its actor");
    for record in &exchanges[..3] {
        assert!(
            record.contains(&format!("\"actor\":\"{first_actor}\"")),
            "{record}"
        );
        assert!(record.contains("\"tick\":1"), "{record}");
    }
    for record in &exchanges[..2] {
        assert!(record.contains("the socket closed"), "{record}");
    }
    assert!(
        !exchanges[2].contains("the socket closed"),
        "{}",
        exchanges[2]
    );
    // And the fourth record has moved on, so the retrying was bounded by the answer rather than
    // running to the bound.
    assert!(
        !exchanges[3].contains(&format!("\"actor\":\"{first_actor}\"")),
        "{}",
        exchanges[3]
    );
}

/// Rule 19.5 and `VER-MOK-018` case **R2**: exhausted retries produce a counted fallback and the run
/// continues.
///
/// The script's one directive repeats, so every exchange of the run fails transiently and every
/// opportunity spends the bound: four attempts, forty-eight records for twelve opportunities, and
/// **exactly one of every four marked as a fallback**. That last figure is case **P5**'s
/// reconciliation in the only form available before the run record carries the count — rule 15.4's
/// figure is the number of records marked as fallbacks, so twelve marks for twelve opportunities is
/// what a count taken per opportunity looks like from outside. A port counting per attempt would
/// report forty-eight against these same twelve marks.
///
/// "The run continues" is the exit code and the text stream together. Rule 19.5 says an exhausted
/// retry takes rule 9.5's fallback "rather than ending the run", so this is a run that reached its
/// horizon, exits `0`, and holds twelve accepted `wait` decisions no provider chose.
#[test]
fn exhausted_retries_are_a_counted_fallback_and_the_run_continues() {
    let directory = scratch("exhausted");
    let transcript = directory.join("exhausted.jsonl");
    let script = script(&directory, &["error transport the socket closed"]);

    // The trace is asked for here and nowhere else in this file, because "the run continues" is a
    // claim about the *decisions* and not only about the exit code: rule 9.5's `wait` has to be
    // applied and accepted twelve times, and without the trace the text stream says nothing about
    // any individual opportunity.
    let output = engine(CONNECTOR, &transcript, Some(&script), Some(CREDENTIAL))
        .arg("--trace-actions")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));

    let recorded = fs::read_to_string(&transcript).unwrap();
    let exchanges = exchanges(&recorded);
    // Four attempts an opportunity: rule 19.5's bound of three retries.
    assert_eq!(exchanges.len(), ROSTER * 4, "four attempts an opportunity");
    let marked: Vec<usize> = exchanges
        .iter()
        .enumerate()
        .filter(|(_, record)| record.contains("\"fallback\":true"))
        .map(|(at, _)| at)
        .collect();
    assert_eq!(marked.len(), ROSTER, "one fallback an opportunity");
    // And it is the *last* attempt of each group that carries the mark, not the first: the earlier
    // ones were retried and a retried attempt is not a decision.
    assert_eq!(
        marked,
        (0..ROSTER)
            .map(|group| group * 4 + 3)
            .collect::<Vec<usize>>()
    );
    for record in &exchanges {
        assert!(record.contains("the socket closed"), "{record}");
        // Rule 9.5's `wait` on every one of them, and rule 9.7's prohibition with it: nothing
        // supplied a substitute action from elsewhere for an attempt that obtained none.
        assert!(
            record.contains("\"action\":{\"verb\":\"wait\"}"),
            "{record}"
        );
    }

    // Rules 9.5 and 9.8 in the text stream: the fallback is applied and accepted, and the run
    // reached its horizon rather than ending at the first exhausted opportunity.
    let text = stdout(&output);
    assert_eq!(text.matches("proposal:wait").count(), ROSTER, "{text}");
    assert!(!text.contains("status:rejected"), "{text}");
    assert!(text.contains("summary reason=tick_limit"), "{text}");
}

/// **A disclosure, not a property**: a transcript holding a retried exchange is not a replay input.
///
/// Rule 11.2 gives every attempt its own record and rule 12.3 has a replay consume one record per
/// decision opportunity, checking the tick and the actor. The two do not reconcile, and this is the
/// run that shows it: the first attempt's record is consumed by the first opportunity — it is marked
/// `"fallback":false` and carries rule 9.5's `wait`, so nothing distinguishes it from a legitimately
/// recorded decision — and the second opportunity then meets a record naming the first actor and
/// fails rule 12.3's check.
///
/// **The refusal is loud and specific, which is why this is disclosed rather than stopped on.** No
/// replay silently invents a run: the message names the opportunity and the actor the record was for,
/// and the exit code is rule 4's `1`. What it cannot say is that the record was an attempt, because
/// no field rule 11.3 fixes distinguishes one.
///
/// Repairing it means either a new field on the exchange record or rule 12.3 reading a group per
/// opportunity, both changes of substance in an approved artifact — `WO-MOK-026`'s stop-and-escalate
/// condition 6 reserves those to the owner. The consequence for this work order is recorded here so
/// it is not discovered later: **a live run that retried cannot supply case `L30`'s replay
/// identity**, and this test is what will fail if the reconciliation is ever decided.
#[test]
fn a_transcript_holding_a_retried_exchange_is_refused_by_its_own_replay() {
    let directory = scratch("retried-replay");
    let transcript = directory.join("retried.jsonl");
    let script = script(
        &directory,
        &["error transport the socket closed", "ok wait"],
    );

    let recorded = engine(CONNECTOR, &transcript, Some(&script), Some(CREDENTIAL))
        .output()
        .unwrap();
    assert_eq!(recorded.status.code(), Some(0), "{}", stderr(&recorded));
    assert_eq!(
        exchanges(&fs::read_to_string(&transcript).unwrap()).len(),
        ROSTER + 1,
        "one opportunity retried once"
    );

    let mut replay = Command::new(ENGINE);
    replay.args(["--policy", "llm", "--seed", "42", "--ticks", "1"]);
    replay.arg("--transcript-path").arg(&transcript);
    let output = replay.output().unwrap();

    assert_eq!(
        output.status.code(),
        Some(1),
        "the replay of a retried transcript must not pass: {}",
        stderr(&output)
    );
    let complaint = stderr(&output);
    assert!(complaint.starts_with("runtime error:"), "{complaint}");
    // Rule 12.3's own words for the disagreement it found: a record for an actor other than the one
    // being asked. It is the second opportunity that fails, the first having consumed the attempt.
    assert!(complaint.contains("record is for actor"), "{complaint}");
}
