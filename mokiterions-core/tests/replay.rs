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

use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use mokiterions::execute;
use mokiterions::simulation::{Action, DecisionRequest, Direction, Proposer};

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
    let output = invoke(&[
        "--policy",
        "llm",
        "--transcript-path",
        directory.to_str().unwrap(),
        "--ticks",
        "3",
    ]);
    assert_ne!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
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

/// The recorded run: its transcript, its standard output, and the stub that answered it.
struct Recording {
    /// One record per line, each with a trailing newline, which is rule 11.2's framing and what
    /// `ReplayPort` reads.
    transcript: String,
    /// The recorded run's standard output, which a replay has to reproduce byte for byte.
    text: Vec<u8>,
    port: HuntingPort,
}

/// The recorded run, in this process, through the library's own door.
fn record_the_run() -> Recording {
    let mut port = HuntingPort::default();
    let mut text = Vec::new();
    let mut errors = Vec::new();

    // The transcript path is supplied because the shared parser requires one under this source, and
    // its value is anything at all because the library resolves no path — rule 12.1.1, asserted on
    // its own in `tests/process.rs`. The recording's destination is the port, which is where rule
    // 11.1 puts it.
    let arguments = RECORDED_RUN.iter().copied().chain([
        "--transcript-path",
        "required by the parser, opened by nobody",
    ]);
    let code = execute(arguments, &mut text, &mut errors, None, Some(&mut port));

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
