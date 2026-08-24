//! The binary target's transcript, `SPEC-MOK-007` rules 12.1.1, 13.2, 18.4 and 20.1, from outside
//! the engine.
//!
//! This tier drives the built binary, because what it asserts about is the host and not the library:
//! which file gets opened, in which order, what the operator is told when it cannot be opened, and
//! what is left on disk afterwards. `SPEC-MOK-002` rule 9 keeps tests out of `src/main.rs`, which is
//! why the host's own behaviour is tested here.
//!
//! **What is not here yet, and why.** A replay that reproduces a recorded run's bytes is asserted at
//! the internal tier, where a scripted port can record a run in the first place. Asserting it through
//! the binary needs a transcript file to exist, and no program in this repository can write one until
//! a live run exists — `WO-MOK-025` puts the connector out of scope and `WO-MOK-026` supplies it. The
//! committed transcript that closes this gap is item 12 of the same work order and lands beside the
//! test that reads it. Until then this file asserts the refusals, which are reachable now, and states
//! the omission rather than leaving the file looking complete.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

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
