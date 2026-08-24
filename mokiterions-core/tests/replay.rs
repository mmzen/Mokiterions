//! The binary target's transcript, `SPEC-MOK-007` rules 12.1.1, 13.2, 18.4 and 20.1, from outside
//! the engine.
//!
//! This tier drives the built binary, because what it asserts about is the host and not the library:
//! which file gets opened, in which order, what the operator is told when it cannot be opened, and
//! what is left on disk afterwards. `SPEC-MOK-002` rule 9 keeps tests out of `src/main.rs`, which is
//! why the host's own behaviour is tested here.
//!
//! **The committed transcript**, `WO-MOK-025` item 12, is
//! `tests/transcript-seed0-ticks20-hunting.jsonl`, and the last group of tests below is what keeps
//! it honest. The same run is recorded in this process against a scripted hunting stub; the
//! committed file is required to be that recording byte for byte; and the built binary is then
//! required to replay the committed file into the recorded run's own standard output, byte for byte.
//! That is `VER-MOK-018`'s acceptance scenario **A1** — a run nobody paid for — and it is why the
//! transcript is committed at all: `.github/workflows/provider-credentials.yml` replays it on every
//! pull request, which is the whole of what `REQ-MOK-073` permits automation to do with this source.
//!
//! The recording reaches the engine through `execute` and a `Proposer` this file defines, both of
//! which are public. Nothing here needs `run_recording`, which is `pub(crate)`, and nothing here
//! widens an item to be tested — `SPEC-MOK-002` rule 9's condition for a test at this tier.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use mokiterions::execute;
use mokiterions::simulation::{
    Action, Config, DecisionRequest, Density, Direction, Policy, Proposer, ReplayPort, Simulation,
};

const BINARY: &str = env!("CARGO_BIN_EXE_Mokiterions");

/// An empty directory of this test's own, removed and recreated so a run cannot inherit a file from
/// a previous one.
fn scratch(label: &str) -> PathBuf {
    let directory = std::env::temp_dir().join(format!("mokiterions-replay-{label}"));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).unwrap();
    directory
}

fn invoke(arguments: &[&str]) -> Output {
    Command::new(BINARY).args(arguments).output().unwrap()
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).unwrap()
}

/// Rules 13.2 and 19.2: the fifth source with no transcript is a usage error, from the binary.
///
/// Exit `2` with the usage text after it, which is what every other configuration error does — the
/// refusal is about the options given and the operator is shown what the options are. Nothing runs,
/// so nothing reaches standard output.
#[test]
fn the_binary_refuses_the_replay_source_with_no_transcript() {
    let output = invoke(&["--policy", "llm", "--ticks", "3"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = stderr(&output);
    assert!(stderr.contains("--transcript-path"), "{stderr}");
    assert!(stderr.contains("Usage:"), "{stderr}");
}

/// Rule 18.4.3: a transcript under a source that decides for itself is a usage error, from the binary.
///
/// Including the default source, which is the case an operator reaches by naming a transcript and no
/// policy at all.
#[test]
fn the_binary_refuses_a_transcript_under_a_source_that_decides_for_itself() {
    for arguments in [
        vec!["--policy", "social", "--transcript-path", "t.jsonl"],
        vec!["--transcript-path", "t.jsonl"],
    ] {
        let output = invoke(&arguments);
        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
        let stderr = stderr(&output);
        assert!(stderr.contains("--transcript-path"), "{stderr}");
        assert!(stderr.contains("Usage:"), "{stderr}");
    }
}

/// Rule 13.2 applied to a read: a transcript the platform refuses is a runtime failure and not a
/// configuration error.
///
/// Exit `1`, the platform's own reason, and **no usage text**, which is the difference that tells an
/// operator whether to correct their command line or their filesystem. The message names the path
/// because rule 19.7 forbids only a path *the engine* resolved: this one is the operator's own
/// argument, resolved by the host, and naming it is the whole use of the message.
#[test]
fn a_transcript_that_cannot_be_read_exits_one_and_names_it() {
    let directory = scratch("unreadable");
    let missing = directory.join("no-such-transcript.jsonl");
    let missing = missing.to_str().unwrap();

    let output = invoke(&[
        "--policy",
        "llm",
        "--transcript-path",
        missing,
        "--ticks",
        "3",
    ]);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = stderr(&output);
    assert!(stderr.starts_with("runtime error: transcript "), "{stderr}");
    assert!(stderr.contains(missing), "{stderr}");
    assert!(!stderr.contains("Usage:"), "{stderr}");

    // A directory is not a transcript either, and it fails the same way rather than being read as an
    // empty one — which would replay as a transcript that ran out at the first opportunity.
    //
    // The assertions here are the missing file's, exactly, and that is the point: `fs::File::open` on
    // a directory *succeeds* on Linux and refuses only at the first read, so before the host forced
    // that read this case exited non-zero on both platforms while printing a whole tick's events on
    // one of them. `assert_ne!(code, Some(0))` was what let that pass, so the code is pinned to `1`
    // and the message is checked, the same way the missing file's is.
    let directory = directory.to_str().unwrap();
    let output = invoke(&[
        "--policy",
        "llm",
        "--transcript-path",
        directory,
        "--ticks",
        "3",
    ]);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let refusal = String::from_utf8(output.stderr).unwrap();
    assert!(
        refusal.starts_with("runtime error: transcript "),
        "{refusal}"
    );
    assert!(refusal.contains(directory), "{refusal}");
    assert!(!refusal.contains("Usage:"), "{refusal}");
}

/// The transcript is opened before the record sink, so a run that cannot obtain its decisions leaves
/// the filesystem exactly as it was.
///
/// The order is what makes this true rather than the removal logic: opening the transcript creates
/// nothing, so a failure there happens before any destination exists. The alternative order would
/// create a record file and then have to take it away again, and `SPEC-MOK-006` rule 13.4 bounds that
/// removal to a file this process created — so a destination that already existed would be replaced
/// with an empty one and then left, for a run that never started.
#[test]
fn a_transcript_that_cannot_be_read_creates_no_record_file() {
    let directory = scratch("ordering");
    let records = directory.join("records.jsonl");
    let missing = directory.join("no-such-transcript.jsonl");

    let output = invoke(&[
        "--policy",
        "llm",
        "--transcript-path",
        missing.to_str().unwrap(),
        "--events-path",
        records.to_str().unwrap(),
        "--ticks",
        "3",
    ]);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        !fs::exists(&records).unwrap_or(false),
        "a run that obtained no decisions created a record file"
    );
    // And it says nothing about the sink, because the sink was never reached.
    let stderr = stderr(&output);
    assert!(!stderr.contains("record sink"), "{stderr}");

    // An existing destination is likewise untouched: same bytes, not replaced with an empty file.
    fs::write(&records, b"operator's own file\n").unwrap();
    let output = invoke(&[
        "--policy",
        "llm",
        "--transcript-path",
        missing.to_str().unwrap(),
        "--events-path",
        records.to_str().unwrap(),
        "--ticks",
        "3",
    ]);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(fs::read(&records).unwrap(), b"operator's own file\n");
}

/// Rule 12.2: the refusals above open no socket, spawn no process and read no credential.
///
/// Asserted as what the operator is told, which is the only thing this tier can observe: a diagnostic
/// that named a provider, a host or a credential would mean the host had gone looking for one. The
/// list is the internal tier's rule 11.6 list, applied here to the refusal path rather than to a
/// transcript.
#[test]
fn a_refused_replay_names_no_provider_and_no_credential() {
    let directory = scratch("quiet");
    let missing = directory.join("no-such-transcript.jsonl");

    for arguments in [
        vec!["--policy", "llm"],
        vec!["--transcript-path", "t.jsonl"],
        vec![
            "--policy",
            "llm",
            "--transcript-path",
            missing.to_str().unwrap(),
        ],
    ] {
        let output = invoke(&arguments);
        let text = format!(
            "{}{}",
            stderr(&output),
            String::from_utf8_lossy(&output.stdout)
        )
        .to_lowercase();
        for forbidden in [
            "authorization",
            "bearer",
            "api_key",
            "apikey",
            "api-key",
            "credential",
            "secret",
            "password",
            "openai",
            "gpt-",
            "sk-",
            "http",
        ] {
            assert!(
                !text.contains(forbidden),
                "{arguments:?} produced {forbidden:?}"
            );
        }
    }
}

// -----------------------------------------------------------------------------------
// The committed transcript, `WO-MOK-025` item 12, and acceptance scenario A1.
// -----------------------------------------------------------------------------------

/// The committed transcript, relative to this package's manifest.
///
/// The name states the configuration because a transcript is only replayable against the run it was
/// recorded from: rule 12.6 claims byte-identity for the *matched* configuration and rule 12.3 makes
/// every other one a mismatch at the first opportunity. It is committed inside the engine's existing
/// `tests` directory and not in a directory of its own, because `WO-MOK-025` states that
/// `SPEC-MOK-004` rule 1's layout does not move at this stage and that this stage adds no directory.
const COMMITTED_TRANSCRIPT: &str = "tests/transcript-seed0-ticks20-hunting.jsonl";

/// The recorded run's configuration, shared by the recording and by every replay of it.
///
/// One list rather than two, for the reason the file name states. The workflow at
/// `.github/workflows/provider-credentials.yml` names the same figures on one line of its own, and
/// the replay there fails at the first opportunity if that line and this list stop agreeing.
const RECORDED_RUN: [&str; 7] = [
    "--policy",
    "llm",
    "--seed",
    "0",
    "--ticks",
    "20",
    "--trace-actions",
];

/// This package's own directory, joined with a relative path.
///
/// The engine's binary target resolves an operator's path against the process's working directory,
/// which is wherever `cargo test` happened to be run from. A committed file is found relative to the
/// manifest instead, so a test that reads one does not depend on that.
fn manifest_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

/// Block D's own grammar, most aggressive form first.
///
/// This is a stub's priority and not a policy proposal. It is chosen for the coverage `WO-MOK-025`
/// item 12 asks of the transcript — every Mokiterion acts, targeted actions are enumerated, food is
/// and is not co-located, and at least one Mokiterion dies — and `REQ-MOK-075` forbids reading
/// anything else into what a run under it produces. No test in this repository compares its outcome
/// with another source's, and `VER-MOK-018` case **L26** fails if one ever does.
const HUNTING_PRIORITY: [&str; 7] = [
    "fight ",
    "attack ",
    "eat ",
    "approach ",
    "move ",
    "sleep",
    "wait",
];

/// A stub that hunts: at every opportunity it takes the most aggressive action block D offers.
///
/// Every proposal is read out of the request it answers, so no proposal is one the engine's own rules
/// reject and rule 9.5's no-proposal case never arises. That is what the transcript is worth as a
/// fixture: a byte comparison of the recorded and the replayed run is then a comparison of the
/// decisions, and not of two runs that happened to take the same recovery path.
///
/// It is rule 1.1 again — the engine cannot tell this from the reader-backed port a host lends it, so
/// a transcript recorded here is a transcript. It reaches no provider, opens no socket and reads no
/// credential, which is `WO-MOK-025`'s *Out of scope* as a property of the fixture as well as of the
/// product.
#[derive(Default)]
struct HuntingPort {
    /// Every record the engine authored, in order: the transcript a recording host would write.
    written: Vec<String>,
    /// One entry per opportunity: the tick, the acting Mokiterion, and the form proposed.
    chosen: Vec<(u64, String, String)>,
    /// Block D of every request, kept so the coverage below is asserted against what the run
    /// actually enumerated rather than against what a reader expected it to.
    enumerated: Vec<String>,
}

impl Proposer for HuntingPort {
    fn propose(&mut self, request: DecisionRequest) -> Option<Action> {
        let permitted = request.permitted_set();
        let forms: Vec<&str> = permitted
            .lines()
            .filter(|line| line.starts_with("  "))
            .map(str::trim)
            .collect();
        let chosen = HUNTING_PRIORITY
            .iter()
            .find_map(|verb| forms.iter().find(|form| form.starts_with(verb)))
            .expect("block D enumerates at least one action this stub can read");
        let action = action_from(chosen).expect("block D enumerates in rule 8.2's grammar");

        self.enumerated.push(permitted.to_string());
        self.chosen.push((
            request.tick(),
            request.actor_id().to_string(),
            (*chosen).to_string(),
        ));
        Some(action)
    }

    fn record(&mut self, record: &str) -> io::Result<()> {
        self.written.push(record.to_string());
        Ok(())
    }
}

/// One enumerated form read back into an [`Action`].
///
/// Written out rather than derived, because the engine's rendering of block D is private and a stub
/// that guessed at it would propose an action the engine never enumerated. An unreadable form is
/// `None` and the caller panics on it: a verb this cannot read is block D having changed, which the
/// coverage assertions below could not tell apart from a run that never reached it.
fn action_from(form: &str) -> Option<Action> {
    let (verb, parameter) = match form.split_once(' ') {
        Some((verb, parameter)) => (verb, Some(parameter.to_string())),
        None => (form, None),
    };
    Some(match (verb, parameter) {
        ("wait", None) => Action::Wait,
        ("sleep", None) => Action::Sleep,
        ("eat", Some(food_id)) => Action::Eat { food_id },
        ("move", Some(direction)) => Action::Move {
            direction: direction_from(&direction)?,
        },
        ("attack", Some(target)) => Action::Attack { target },
        ("threaten", Some(target)) => Action::Threaten { target },
        ("fight", Some(target)) => Action::Fight { target },
        ("retreat", Some(target)) => Action::Retreat { target },
        ("surrender", Some(target)) => Action::Surrender { target },
        ("approach", Some(target)) => Action::Approach { target },
        ("avoid", Some(target)) => Action::Avoid { target },
        _ => return None,
    })
}

fn direction_from(name: &str) -> Option<Direction> {
    match name {
        "north" => Some(Direction::North),
        "east" => Some(Direction::East),
        "south" => Some(Direction::South),
        "west" => Some(Direction::West),
        _ => None,
    }
}

/// The recorded run: its transcript, its two streams, and the stub that answered it.
struct Recording {
    /// One record per line, each with a trailing newline, which is rule 11.2's framing and what
    /// `ReplayPort` reads.
    transcript: String,
    /// The recorded run's standard output, which a replay has to reproduce byte for byte.
    text: Vec<u8>,
    /// The recorded run's structured record stream, which a replay has to reproduce as well —
    /// `REQ-MOK-067` names both streams and `VER-MOK-018` case **L7** compares both.
    records: Vec<u8>,
    port: HuntingPort,
}

/// The committed transcript's own run, in this process, through the library's own door.
fn record_the_run() -> Recording {
    record_a_run(0, true)
}

/// One recorded run at a chosen seed and tracing selection.
///
/// The horizon is the committed transcript's twenty ticks for every cell, so the one configuration
/// the committed file belongs to is a cell of the sweep rather than a separate case.
fn record_a_run(seed: u64, trace: bool) -> Recording {
    let mut port = HuntingPort::default();
    let mut text = Vec::new();
    let mut records = Vec::new();
    let mut errors = Vec::new();

    let seed = seed.to_string();
    let mut arguments = vec!["--policy", "llm", "--seed", seed.as_str(), "--ticks", "20"];
    if trace {
        arguments.push("--trace-actions");
    }
    if seed == "0" && trace {
        // The list the replay workflow names on one line of its own, built here from the parameters.
        // Asserted rather than assumed, because a drift between the two would leave the committed
        // transcript belonging to a configuration no test records.
        assert_eq!(arguments[..], RECORDED_RUN[..], "RECORDED_RUN has moved");
    }

    // The transcript path is supplied because the shared parser requires one under this source, and
    // its value is anything at all because the library resolves no path — rule 12.1.1, asserted on
    // its own in `tests/process.rs`. The recording's destination is the port, which is where rule
    // 11.1 puts it.
    arguments.extend([
        "--transcript-path",
        "required by the parser, opened by nobody",
    ]);
    let code = execute(
        arguments.iter().copied(),
        &mut text,
        &mut errors,
        Some(&mut records),
        Some(&mut port),
    );

    assert_eq!(code, 0, "{}", String::from_utf8_lossy(&errors));
    assert!(errors.is_empty(), "{}", String::from_utf8_lossy(&errors));

    let transcript = port
        .written
        .iter()
        .map(|record| format!("{record}\n"))
        .collect();
    Recording {
        transcript,
        text,
        records,
        port,
    }
}

/// The first line at which two streams differ, described, or `None` when they agree.
///
/// A committed transcript is around a hundred kilobytes and a traced run's output is thousands of
/// lines, so a bare `assert_eq!` on either would report a difference by printing both in full. This
/// reports the line number and the two lines, which is what a reader needs in order to tell a
/// changed decision from a changed record format.
fn first_difference(left: &str, right: &str, left_name: &str, right_name: &str) -> Option<String> {
    for (index, (one, other)) in left.lines().zip(right.lines()).enumerate() {
        if one != other {
            return Some(format!(
                "line {}:\n  {left_name}: {one}\n  {right_name}: {other}",
                index + 1
            ));
        }
    }
    let (left_lines, right_lines) = (left.lines().count(), right.lines().count());
    if left_lines != right_lines {
        return Some(format!(
            "the streams agree for {} line(s) and then one ends: {left_name} has {left_lines} \
             line(s), {right_name} has {right_lines}",
            left_lines.min(right_lines),
        ));
    }
    None
}

/// The regeneration command, quoted in every failure regeneration is the answer to.
const REGENERATE: &str =
    "cargo test -p Mokiterions --test replay -- --ignored regenerate_the_committed_transcript";

/// The committed transcript is the run its name says it is.
///
/// This is the fixture's whole warrant. A transcript committed once and then left behind while the
/// engine's records changed shape would still replay — rule 12.3 matches on tick and Mokiterion, not
/// on a record's fields — and it would replay a run nobody could reproduce. Recording the same
/// configuration here and requiring the bytes to match is what keeps the file a measurement rather
/// than a memory.
#[test]
fn the_committed_transcript_is_the_run_it_says_it_is() {
    let recording = record_the_run();
    let path = manifest_path(COMMITTED_TRANSCRIPT);
    let committed = fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "{}: {error}\nregenerate it with\n  {REGENERATE}",
            path.display()
        )
    });

    if let Some(difference) =
        first_difference(&committed, &recording.transcript, "committed", "recorded")
    {
        panic!(
            "the committed transcript is not what this configuration records.\n{difference}\n\
             If the change is intended, regenerate it with\n  {REGENERATE}"
        );
    }
}

/// The coverage `VER-MOK-018`'s residual-uncertainty section says the reading cases rest on.
///
/// `WO-MOK-025` item 12 names four properties, and they are asserted against the run rather than
/// against the file: what the transcript covers is a fact about the recorded run, and a fixture whose
/// coverage was asserted by reading its own bytes would keep passing after the run behind it stopped
/// producing that coverage.
#[test]
fn the_committed_transcript_covers_what_the_reading_cases_rest_on() {
    let recording = record_the_run();
    let text = String::from_utf8(recording.text.clone()).unwrap();

    // Every Mokiterion acts.
    let actors: BTreeSet<&str> = recording
        .port
        .chosen
        .iter()
        .map(|(_, actor, _)| actor.as_str())
        .collect();
    for number in 1..=12 {
        let actor = format!("M{number:02}");
        assert!(actors.contains(actor.as_str()), "{actor} never acted");
    }
    assert_eq!(actors.len(), 12, "{actors:?}");

    // Targeted actions are enumerated — all seven of them, which is what `SPEC-MOK-001` rule 21
    // fixes and what cases L12 and L13 read a request's enumerated set for.
    let enumerated = recording.port.enumerated.join("\n");
    for verb in [
        "attack ",
        "threaten ",
        "fight ",
        "retreat ",
        "surrender ",
        "approach ",
        "avoid ",
    ] {
        assert!(
            enumerated.contains(&format!("  {verb}")),
            "no opportunity in this run enumerated {verb:?}"
        );
    }

    // Food is and is not co-located: `eat` is enumerated at some opportunities and not at others.
    assert!(
        recording
            .port
            .enumerated
            .iter()
            .any(|block| block.contains("  eat ")),
        "no opportunity had food underfoot"
    );
    assert!(
        recording
            .port
            .enumerated
            .iter()
            .any(|block| !block.contains("  eat ")),
        "every opportunity had food underfoot"
    );

    // At least one Mokiterion dies, and the run says so in its own stream.
    assert!(text.contains("event=agent_died"), "no Mokiterion died");
}

/// Acceptance scenario **A1**: the built binary replays the committed transcript into the recorded
/// run's own bytes, and exits 0.
///
/// This is the case the whole approach rests on, and it is asserted through the process boundary
/// because that is where a host can get it wrong in a way no library test would see: a transcript
/// opened in text mode, a reader wrapped after the first record, a path resolved against the wrong
/// directory. The recording it is compared against is produced in this process, so the comparison
/// needs no second committed file and cannot drift from the transcript it belongs to.
#[test]
fn the_binary_replays_the_committed_transcript_into_the_recorded_runs_bytes() {
    let recording = record_the_run();
    let path = manifest_path(COMMITTED_TRANSCRIPT);
    let path = path.to_str().expect("a UTF-8 path").to_string();

    let arguments: Vec<&str> = RECORDED_RUN
        .iter()
        .copied()
        .chain(["--transcript-path", path.as_str()])
        .collect();
    let output = invoke(&arguments);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));

    let replayed = String::from_utf8(output.stdout).expect("the run's output is UTF-8");
    let recorded = String::from_utf8(recording.text).expect("the run's output is UTF-8");
    if let Some(difference) = first_difference(&replayed, &recorded, "replayed", "recorded") {
        panic!("the replay is not the recorded run.\n{difference}");
    }
    assert_eq!(
        replayed.len(),
        recorded.len(),
        "the two runs differ in length"
    );
}

/// `VER-MOK-002`'s declared seeds, which `REQ-MOK-067` and `REQ-MOK-068` are both stated over.
const DECLARED_SEEDS: [u64; 5] = [0, 1, 42, 123, 777];

/// `REQ-MOK-067` over the whole declared matrix: `VER-MOK-018` case **L7**.
///
/// The case's own words are "a recorded run and a replay of it compare equal with `cmp` on standard
/// output and on the structured record stream, and have the same exit code … checked at every
/// declared seed, with tracing on and off". Ten cells, each recorded in this process against the
/// hunting stub and replayed through the built binary, with both streams and the status compared.
///
/// This is the sweep the test above is one cell of. That one exists separately because it replays the
/// **committed** file rather than a transcript written moments earlier, so it is the only one that can
/// fail on the committed bytes having drifted. Here the transcript is written to a scratch directory
/// per cell, which is what lets the other nine cells exist at all: no transcript is committed for
/// them, and committing ten would be ten files to keep in agreement with the recording instead of one.
///
/// Tracing off is not a formality, and it is not the same run observed twice: it moves both streams,
/// which the printed figures show — at seed 0 the text goes from 39,543 bytes to 78,315 and the record
/// stream from 77,820 to 139,638, while the 221 opportunities are the same 221. So each selection is
/// its own cell of the case rather than a second view of one, and a replay that agreed on the shorter
/// stream while diverging on the traced line for the same decision is a failure this reaches.
///
/// Nothing here reads an environment variable or reaches a network, in either direction: the library
/// resolves no path and opens nothing, which `tests/process.rs` asserts on its own, and the binary
/// opens the transcript this test wrote and the record file it names. A credential present in the
/// environment cannot change any byte compared below, which is the half of case **L20** this stage
/// can reach.
#[test]
fn a_recording_and_its_replay_agree_at_every_declared_seed_with_tracing_on_and_off() {
    let directory = scratch("declared-seeds");
    let mut compared = Vec::new();

    for seed in DECLARED_SEEDS {
        for trace in [false, true] {
            let cell = format!("seed{seed}-trace{}", if trace { "on" } else { "off" });
            let recording = record_a_run(seed, trace);
            assert!(
                !recording.transcript.is_empty() && !recording.records.is_empty(),
                "{cell}: the recording produced nothing to compare"
            );

            let transcript_path = directory.join(format!("{cell}.jsonl"));
            fs::write(&transcript_path, recording.transcript.as_bytes())
                .expect("the scratch directory is writable");
            let records_path = directory.join(format!("{cell}.records.jsonl"));

            let seed_text = seed.to_string();
            let transcript_text = transcript_path.to_str().expect("a UTF-8 path");
            let records_text = records_path.to_str().expect("a UTF-8 path");
            let mut arguments = vec!["--policy", "llm", "--seed", &seed_text, "--ticks", "20"];
            if trace {
                arguments.push("--trace-actions");
            }
            arguments.extend([
                "--transcript-path",
                transcript_text,
                "--events-path",
                records_text,
            ]);
            let output = invoke(&arguments);

            // The exit code half of the case: the recording returned `0` from `execute`, asserted in
            // `record_a_run`, and the replay of it exits `0` from the process.
            assert_eq!(output.status.code(), Some(0), "{cell}: {}", stderr(&output));
            assert!(output.stderr.is_empty(), "{cell}: {}", stderr(&output));

            let replayed = String::from_utf8(output.stdout).expect("the run's output is UTF-8");
            let recorded = String::from_utf8(recording.text).expect("the run's output is UTF-8");
            if let Some(difference) = first_difference(&replayed, &recorded, "replayed", "recorded")
            {
                panic!("{cell}: the replay is not the recorded run.\n{difference}");
            }
            assert_eq!(
                replayed.len(),
                recorded.len(),
                "{cell}: the texts differ in length"
            );

            let replayed_records = fs::read(&records_path).expect("the replay wrote its records");
            if let Some(difference) = first_difference(
                &String::from_utf8_lossy(&replayed_records),
                &String::from_utf8_lossy(&recording.records),
                "replayed",
                "recorded",
            ) {
                panic!("{cell}: the replayed record stream is not the recorded one.\n{difference}");
            }
            assert_eq!(
                replayed_records, recording.records,
                "{cell}: the record streams differ in bytes the line comparison did not reach"
            );

            compared.push(format!(
                "{cell} {} text byte(s) {} record byte(s) {} opportunit(ies)",
                recorded.len(),
                recording.records.len(),
                recording.port.chosen.len(),
            ));
        }
    }

    assert_eq!(compared.len(), 10, "{compared:#?}");

    // The figures a reader of the evidence needs, printed rather than only asserted: the horizon is
    // the same for every cell, so a cell that quietly stopped early is visible as a smaller run.
    for line in &compared {
        println!("{line}");
    }
}

/// Rule 12.3's third configuration: the density, which `VER-MOK-018` case **L8** names and no other
/// test reaches.
///
/// The seed and the horizon are checked in the engine's internal tier, where a transcript can be
/// edited record by record. The density cannot be checked there, because the helper that replays a
/// transcript in-process takes a seed and a tick limit and the density is a parsed option — so this is
/// the tier where a density mismatch exists at all.
///
/// It is not detected the way a seed mismatch is. A different seed changes the twelve names, so it
/// changes block B and the prefix digest with it, and rule 11.3.2's check refuses the first record. A
/// different density leaves every name alone: the same twelve Mokiterions are asked in the same order,
/// and the divergence has to arrive through the world instead. It does, and the run stops on it —
/// resources sit elsewhere, so the run's deaths fall at other ticks, and the acting order at some tick
/// is then not the recorded one. The assertion is therefore about the shape of the refusal and about
/// the run stopping short, not about a fixed tick: a density that failed to diverge before the horizon
/// would leave the summary line present, which is what this fails on.
#[test]
fn a_replay_at_another_density_fails_and_names_the_mismatch() {
    let path = manifest_path(COMMITTED_TRANSCRIPT);
    let path = path.to_str().expect("a UTF-8 path").to_string();

    // Above and below the default of 0.75, so the case does not rest on a scarcer world alone.
    for density in ["1.5", "0.5"] {
        let arguments: Vec<&str> = RECORDED_RUN
            .iter()
            .copied()
            .chain(["--density", density, "--transcript-path", path.as_str()])
            .collect();
        let output = invoke(&arguments);
        let complaint = stderr(&output);

        // Rule 19.4: a status distinct from a clean completion, and the opportunity named.
        assert_eq!(
            output.status.code(),
            Some(1),
            "density {density}: {complaint}"
        );
        assert!(
            complaint.contains("transcript: tick ") && complaint.contains(" actor "),
            "density {density}: the refusal names no opportunity: {complaint}"
        );
        assert!(
            complaint.contains("record is for actor") || complaint.contains("record is for tick"),
            "density {density}: the refusal names no mismatch: {complaint}"
        );
        // Rule 19.7: no path the engine resolved, so the transcript's own path is not quoted back.
        assert!(
            !complaint.contains(&path),
            "density {density}: the refusal quotes the path: {complaint}"
        );

        // Rule 12.3: no further ticks. The run reached some ticks and then stopped, so the closing
        // summary the horizon would have produced is absent.
        let produced = String::from_utf8(output.stdout).expect("the run's output is UTF-8");
        assert!(
            !produced.contains("summary reason="),
            "density {density}: the run reached its horizon anyway"
        );
        assert!(
            produced.contains("tick=1 "),
            "density {density}: the run produced no ticks at all, so nothing diverged"
        );
    }
}

/// One record stream from a source that decides for itself, for the differential below.
///
/// Separate from [`record_a_run`] rather than a parameter on it: that function guards `RECORDED_RUN`
/// and lends a port, and neither applies to a source that needs no port. What is shared is the
/// figures — the same twenty ticks at the same seed — because a differential between two runs of
/// different lengths would compare the sets of forms two different worlds happened to reach.
fn record_under_social(seed: u64, trace: bool) -> Vec<u8> {
    let mut text = Vec::new();
    let mut records = Vec::new();
    let mut errors = Vec::new();
    let seed = seed.to_string();
    let mut arguments = vec![
        "--policy",
        "social",
        "--seed",
        seed.as_str(),
        "--ticks",
        "20",
    ];
    if trace {
        arguments.push("--trace-actions");
    }
    let code = execute(
        arguments.iter().copied(),
        &mut text,
        &mut errors,
        Some(&mut records),
        None,
    );
    assert_eq!(code, 0, "{}", String::from_utf8_lossy(&errors));
    assert!(errors.is_empty(), "{}", String::from_utf8_lossy(&errors));
    records
}

/// One record's shape, with every value discarded.
///
/// The differential below is about form and not about content: two runs of two sources decide
/// differently, so every position, every survival figure and every outcome differs between them and a
/// comparison of records themselves would say nothing. What must not differ is the shape — which keys
/// a record carries, in which order, at which nesting depth, each holding which kind of value. A
/// record the port path emitted with an extra field, a renamed field, a reordered field or a number
/// where the other source wrote a string is a record that came from somewhere else.
///
/// Written here rather than taken from a JSON crate because `WO-MOK-025`'s *Out of scope* forbids
/// adding a crate to either package, and because the streams are the engine's own output in a closed
/// alphabet. **What it does not catch**: two values of the same kind that differ in a way form should
/// have expressed, such as an enumeration gaining a member. Rule 12's replay comparison is what
/// covers the values.
fn record_form(line: &str) -> String {
    let bytes = line.as_bytes();
    let mut form = String::new();
    let mut depth = 0usize;
    let mut index = 0usize;
    while index < bytes.len() {
        match bytes[index] {
            b'{' | b'[' => {
                depth += 1;
                index += 1;
            }
            b'}' | b']' => {
                depth = depth.saturating_sub(1);
                index += 1;
            }
            b'"' => {
                let close = closing_quote(bytes, index);
                let mut after = close + 1;
                while after < bytes.len() && bytes[after].is_ascii_whitespace() {
                    after += 1;
                }
                if bytes.get(after) != Some(&b':') {
                    // A string in a value position, which contributes no shape.
                    index = close + 1;
                    continue;
                }
                after += 1;
                while after < bytes.len() && bytes[after].is_ascii_whitespace() {
                    after += 1;
                }
                let kind = match bytes.get(after) {
                    Some(b'{') => "object",
                    Some(b'[') => "array",
                    Some(b'"') => "string",
                    Some(b't') | Some(b'f') => "boolean",
                    Some(b'n') => "null",
                    _ => "number",
                };
                let name = String::from_utf8_lossy(&bytes[index + 1..close]);
                form.push_str(&format!("{depth}.{name}={kind} "));
                // Left at the value's first byte, so an opening brace still counts toward the depth.
                index = after;
            }
            _ => index += 1,
        }
    }
    form
}

/// The index of the quote that closes the string opening at `start`, honouring backslash escapes.
fn closing_quote(bytes: &[u8], start: usize) -> usize {
    let mut index = start + 1;
    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 2,
            b'"' => return index,
            _ => index += 1,
        }
    }
    index
}

/// One record's kind: the `record` field and, for an event, the event's name.
fn record_kind(line: &str) -> String {
    let field = |name: &str| -> Option<String> {
        let needle = format!("\"{name}\":\"");
        let start = line.find(&needle)? + needle.len();
        let rest = &line[start..];
        Some(rest[..rest.find('"')?].to_string())
    };
    match (field("record"), field("event")) {
        (Some(record), Some(event)) => format!("{record}/{event}"),
        (Some(record), None) => record,
        _ => "unrecognised".to_string(),
    }
}

/// `VER-MOK-018` case **L1**: one path and no privilege, as a differential over every declared seed.
///
/// A proposal that arrived from a port is validated, resolved, traced and rejected by the code that
/// handles a proposal from any other source. The claim is not that the two runs agree — they cannot,
/// because two sources decide differently — but that **no record form belongs to the port path
/// alone**. A form only the port produced would mean a second path exists somewhere after the
/// proposal: its own validation, its own resolution, or its own record.
///
/// **Both sides are aggregated over all ten cells before anything is compared**, and that is a
/// measurement and not a preference. Per cell the inventories differ in both directions, because two
/// sources reach different worlds: at seed 0 `social` reaches `threat_resolved` and
/// `surrender_resolved` and the stub does not, and at seed 1 the stub reaches `territory_crossed` and
/// `social` does not — the stub moves toward food and crossed a boundary that run, and no proposal
/// under `social` happened to. Neither is a second code path, and a per-cell comparison would report
/// one. Aggregated, the question becomes whether the engine ever emits a form under the port that it
/// does not emit without one, which is the question the case asks.
///
/// The comparison then runs one way. A form present under `social` and absent under the port is the
/// stub's verb priority — a fixture's choice — so it is printed rather than asserted, and a reader can
/// see how far the stub reached. A form present under the port and absent everywhere under `social` is
/// the failure this case exists to catch.
///
/// Both tracing settings, because tracing is where a resolved action's own records appear, and those
/// are the records a privileged second path would most plausibly differ in.
#[test]
fn no_record_form_is_unique_to_the_port_at_any_declared_seed() {
    let forms = |stream: Vec<u8>| -> BTreeMap<String, BTreeSet<String>> {
        let text = String::from_utf8(stream).expect("the record stream is UTF-8");
        let mut map: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for line in text.lines() {
            map.entry(record_kind(line))
                .or_default()
                .insert(record_form(line));
        }
        map
    };

    let mut ported: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut deciding: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let absorb = |into: &mut BTreeMap<String, BTreeSet<String>>,
                  from: BTreeMap<String, BTreeSet<String>>| {
        for (kind, shapes) in from {
            into.entry(kind).or_default().extend(shapes);
        }
    };

    let mut compared = 0;
    for seed in DECLARED_SEEDS {
        for trace in [false, true] {
            absorb(&mut ported, forms(record_a_run(seed, trace).records));
            absorb(&mut deciding, forms(record_under_social(seed, trace)));
            compared += 1;
        }
    }
    assert_eq!(
        compared,
        DECLARED_SEEDS.len() * 2,
        "not every declared seed was compared"
    );
    assert!(
        ported.contains_key("event/decision_source_selected"),
        "the port-driven runs produced no recognisable records"
    );

    for (kind, shapes) in &ported {
        let theirs = deciding.get(kind).unwrap_or_else(|| {
            panic!("`{kind}` is a record kind only the port path produced, at every declared seed")
        });
        assert_eq!(
            shapes, theirs,
            "`{kind}` has a different form under the port"
        );
    }

    let unreached: Vec<&str> = deciding
        .keys()
        .filter(|kind| !ported.contains_key(*kind))
        .map(String::as_str)
        .collect();
    println!(
        "{compared} cells: {} record kind(s) under the port, all of them `social`'s own form; \
         {} of `social`'s kinds the stub never reached{}",
        ported.len(),
        unreached.len(),
        if unreached.is_empty() {
            String::new()
        } else {
            format!(" ({})", unreached.join(", "))
        }
    );
}

/// `VER-MOK-018` case **L2**: the record stream names the model-backed source exactly once, before
/// tick processing, on every declared seed.
///
/// Over the record stream and not the text stream, because the record stream is what a reader who
/// was not present consults. The text stream's line is checked elsewhere; a run whose text said
/// `llm` and whose records did not would be a run nobody could attribute afterwards.
///
/// Three things are asserted separately, because they fail separately. **Exactly once**: a second
/// record would mean the source was resolved twice, and rule 20.4's "one port for the whole run" is
/// the reason it must not be — a per-tick resolution is how a run silently changes source halfway.
/// **Before tick processing**: every record ahead of it is the header or a tick-0 initialization
/// record, so nothing was decided before the attribution existed. **Naming this source**: the value
/// is `llm`, not merely present, so a run under another policy cannot satisfy the case.
///
/// Both tracing settings at every seed, because tracing changes what else is in the stream and the
/// count above is a count over the whole of it.
#[test]
fn the_record_stream_names_the_source_once_before_any_tick_at_every_declared_seed() {
    let mut checked: Vec<String> = Vec::new();
    for seed in DECLARED_SEEDS {
        for trace in [false, true] {
            let cell = format!("seed{seed}-trace{}", if trace { "on" } else { "off" });
            let recording = record_a_run(seed, trace);
            let stream = String::from_utf8(recording.records).expect("the record stream is UTF-8");
            let lines: Vec<&str> = stream.lines().collect();

            let named: Vec<usize> = lines
                .iter()
                .enumerate()
                .filter(|(_, line)| line.contains("\"event\":\"decision_source_selected\""))
                .map(|(index, _)| index)
                .collect();
            assert_eq!(
                named.len(),
                1,
                "{cell}: {} decision-source records, expected one",
                named.len()
            );

            let record = lines[named[0]];
            assert!(
                record.contains("\"source\":\"llm\""),
                "{cell}: the source record names another source: {record}"
            );
            assert!(
                record.contains("\"tick\":0"),
                "{cell}: the source record is not at tick 0: {record}"
            );
            for earlier in &lines[..named[0]] {
                assert!(
                    earlier.starts_with("{\"record\":\"header\"") || earlier.contains("\"tick\":0"),
                    "{cell}: a tick-processing record precedes the source: {earlier}"
                );
            }
            checked.push(cell);
        }
    }
    assert_eq!(
        checked.len(),
        DECLARED_SEEDS.len() * 2,
        "not every declared seed was reached"
    );
    println!(
        "the source is named once, before any tick, in {} record stream(s): {}",
        checked.len(),
        checked.join(" ")
    );
}

/// Wraps a port and counts what the engine asked it, without changing an answer.
///
/// The engine tells nobody how many opportunities a run had; a host learns it by being asked. So the
/// figure the case below compares is collected here, at the seam, rather than derived from a tick
/// count and a population — which would be the same arithmetic the engine performs and would agree
/// with it whether or not the port was reached.
///
/// Generic over the port so that the wrapper cannot be the thing under test: it holds no proposal
/// logic of its own and forwards both of `Proposer`'s methods unchanged.
struct CountingPort<P: Proposer> {
    inner: P,
    /// The tick and the acting Mokiterion of every opportunity, in the order they arrived.
    opportunities: Vec<(u64, String)>,
    /// Rule 11.1's other half: the records the engine authored and handed over.
    authored: usize,
}

impl<P: Proposer> CountingPort<P> {
    fn new(inner: P) -> Self {
        Self {
            inner,
            opportunities: Vec::new(),
            authored: 0,
        }
    }
}

impl<P: Proposer> Proposer for CountingPort<P> {
    fn propose(&mut self, request: DecisionRequest) -> Option<Action> {
        // Read before forwarding: `propose` takes the request by value.
        self.opportunities
            .push((request.tick(), request.actor_id().to_string()));
        self.inner.propose(request)
    }

    fn record(&mut self, record: &str) -> io::Result<()> {
        self.authored += 1;
        self.inner.record(record)
    }
}

/// `VER-MOK-018` case **L29**: both of rule 20.5's two doors carry the port, in one suite.
///
/// The two doors are `execute`, which a host that owns the whole run calls once, and
/// `Simulation::advance_tick`, which a host that owns the clock calls per tick. Each is exercised
/// with a port here, in this file, deliberately: the engine has two hosts, and until this case both
/// doors were reached with a port but never in the same place — the text-stream host's door through
/// `execute` in this suite, the observer's door through `advance_tick` only from the observer's own
/// package. A signature change that dropped the port from either one therefore failed at whichever
/// host tried it next, which is a build error attributed to the host rather than to the engine. It
/// fails here now instead. `pub fn run` is not one of the doors and is not given a port.
///
/// The two doors are driven over the same recorded run, so the case says something beyond
/// compilation: the second door is asked the same 221 opportunities, in the same order, at the same
/// ticks and for the same Mokiterions as the first. What differs is only who owns the clock.
///
/// The port at the second door is the public [`ReplayPort`] over the committed transcript, which is
/// what makes this a replay rather than a second recording: it reads the file the first door
/// produced, reaches no provider, opens no socket and reads no credential.
///
/// **One thing the two doors do not share, and it is deliberate**: rule 11.1's prefix head. It is
/// written from `DecisionSource::open`, which the whole-run door calls once and the per-tick door
/// never calls — the observer builds a fresh source every tick, so a call there would write the head
/// twenty times, and it needs no head because rule 20.1 makes it the replay host and rule 11.8
/// leaves a replay writing no transcript. So door one authors twelve prefix records that door two
/// does not, and the comparison below is over the exchange records the two doors both author. The
/// twelve are counted from the recording rather than written down here, so that a run with another
/// population does not have to be re-counted by hand.
#[test]
fn both_of_rule_twenty_fives_doors_carry_the_port() {
    // Door one: `execute`, with the recording port this file defines.
    let recording = record_the_run();
    let recorded: Vec<(u64, String)> = recording
        .port
        .chosen
        .iter()
        .map(|(tick, actor, _form)| (*tick, actor.clone()))
        .collect();
    assert!(
        !recorded.is_empty(),
        "the recording reached no opportunity, so neither door is exercised"
    );

    // Door two: `Simulation::advance_tick`, with a replay of what door one produced. The host owns
    // the clock here, which is the whole difference between the two doors.
    let file = fs::File::open(manifest_path(COMMITTED_TRANSCRIPT)).expect("the transcript opens");
    let mut port = CountingPort::new(ReplayPort::new(io::BufReader::new(file)));
    let mut simulation = Simulation::new(Config {
        seed: 0,
        tick_limit: 20,
        policy: Policy::Llm,
        density: Density::DEFAULT,
        trace_actions: true,
    })
    .expect("the recorded run's configuration is valid");

    let mut ticks = 0;
    while !simulation.is_finished() {
        simulation
            .advance_tick(Some(&mut port))
            .expect("the replay reaches the transcript's horizon");
        ticks += 1;
        assert!(ticks <= 20, "the run did not stop at its horizon");
    }

    assert_eq!(
        ticks, 20,
        "the second door stopped short of the recorded run's horizon"
    );
    assert_eq!(
        port.opportunities, recorded,
        "the two doors were asked different opportunities"
    );
    let head = recording
        .port
        .written
        .iter()
        .filter(|record| record.starts_with("{\"transcript\":\"prefix\","))
        .count();
    assert_eq!(
        port.authored,
        recording.port.written.len() - head,
        "the two doors authored a different number of exchange records"
    );
    assert!(head > 0, "door one wrote no prefix head");

    println!(
        "execute and advance_tick both carried the port over {} opportunit(ies) and {} exchange \
         record(s); the head's {head} prefix record(s) are door one's alone",
        port.opportunities.len(),
        port.authored
    );
}

/// The committed file's own constraints: rule 11.6's absences, rule 11.2's framing, and its bytes.
///
/// The line-ending assertion is not pedantry. `core.autocrlf = true` on a Windows clone rewrites a
/// checked-out text file to CRLF, which would break the byte comparison above and would leave a
/// replay reading a stray carriage return at the end of every record. `.gitattributes` marks this
/// path `-text` for that reason, and this is what notices if that entry is ever lost.
#[test]
fn the_committed_transcript_carries_no_credential_and_no_conversion() {
    let bytes = fs::read(manifest_path(COMMITTED_TRANSCRIPT)).expect("the committed transcript");
    assert!(
        !bytes.contains(&b'\r'),
        "the committed transcript holds a carriage return: check the `-text` entry in .gitattributes"
    );

    let text = String::from_utf8(bytes).expect("a transcript is UTF-8");
    assert!(text.ends_with('\n'), "rule 11.2: one record per line");
    for line in text.lines() {
        assert!(
            line.starts_with("{\"transcript\":\"prefix\",")
                || line.starts_with("{\"transcript\":\"exchange\","),
            "not a record of either kind: {line}"
        );
    }

    // Rule 11.6, over the one transcript this repository publishes: no credential, no authorization
    // header, no provider account identifier. The list is the refusal path's list, applied here to
    // the artifact rather than to a diagnostic.
    let lowered = text.to_lowercase();
    for forbidden in [
        "authorization",
        "bearer",
        "api_key",
        "apikey",
        "api-key",
        "credential",
        "password",
        "openai",
        "sk-",
        "http",
    ] {
        assert!(
            !lowered.contains(forbidden),
            "the transcript holds {forbidden:?}"
        );
    }
}

/// Rewrites the committed transcript from this run.
///
/// Ignored by default, because it writes into the working tree: a test that edited a committed file
/// as a side effect of `cargo test` would make the suite's verdict depend on the order its tests ran
/// in, and would quietly repair a divergence a reader needs to see. Run it deliberately, read the
/// diff, and commit the file with the change that caused it.
///
/// ```text
/// cargo test -p Mokiterions --test replay -- --ignored regenerate_the_committed_transcript
/// ```
///
/// The bytes are written with LF endings on every platform, because that is what the comparison
/// reads and what `.gitattributes` keeps in the working tree.
#[test]
#[ignore = "writes into the working tree; run deliberately when the recorded run changes"]
fn regenerate_the_committed_transcript() {
    let recording = record_the_run();
    let path = manifest_path(COMMITTED_TRANSCRIPT);
    fs::write(&path, recording.transcript.as_bytes()).expect("the working tree is writable");

    let exchanges = recording
        .port
        .written
        .iter()
        .filter(|record| record.contains("\"transcript\":\"exchange\""))
        .count();
    println!(
        "wrote {}\n  {} bytes, {} record(s): {} prefix, {} exchange\n  {} opportunities over {} tick(s)",
        path.display(),
        recording.transcript.len(),
        recording.port.written.len(),
        recording.port.written.len() - exchanges,
        exchanges,
        recording.port.chosen.len(),
        recording
            .port
            .chosen
            .last()
            .map(|(tick, _, _)| *tick)
            .unwrap_or(0),
    );
}
