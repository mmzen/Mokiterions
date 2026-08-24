//! Public tier, `SPEC-MOK-002` rule 8: the process boundary.
//!
//! Relocated from `src/main.rs` under `WO-MOK-003`. Every test here needs only `execute`,
//! which rule 4 moved to the library target, `cli::USAGE`, and two byte buffers.
//! Assertions are verbatim, as rule 12 requires.
//!
//! One test added 2026-08-24 under `WO-MOK-025` needs a third thing: a decision port, which is
//! `execute`'s fifth parameter and reaches it through `Proposer`, already public. It is the process
//! boundary's own question — whether a path an operator typed can reach the library — so it belongs
//! here rather than at the internal tier, where no path is ever supplied.

use std::io::{self, Write};

use mokiterions::simulation::{Action, DecisionRequest, Proposer};
use mokiterions::{cli, execute};

/// The failing-writer helper, relocated with the test that needs it. `SPEC-MOK-002` leaves
/// its location unspecified, and it belongs beside the only test that uses it.
struct FailingWriter;

impl Write for FailingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn help_exits_successfully() {
    let mut output = Vec::new();
    let mut errors = Vec::new();

    let code = execute(["--help"], &mut output, &mut errors, None, None);

    assert_eq!(code, 0);
    assert_eq!(String::from_utf8(output).unwrap(), cli::USAGE);
    assert!(errors.is_empty());
}

#[test]
fn invalid_configuration_exits_with_code_two() {
    let mut output = Vec::new();
    let mut errors = Vec::new();

    let code = execute(["--ticks", "0"], &mut output, &mut errors, None, None);

    assert_eq!(code, 2);
    assert!(output.is_empty());
    let errors = String::from_utf8(errors).unwrap();
    assert!(errors.contains("greater than zero"));
    assert!(errors.contains("Usage:"));
}

/// Added under `WO-MOK-004`. The two existing standard-error assertions check the substring
/// `Usage:`, which the synopsis alone satisfies, so neither would notice an options block
/// that reached standard output and not standard error. `REQ-MOK-018` is a property of the
/// text and holds on both paths, so this pins the diagnostic path to the whole constant.
#[test]
fn the_diagnostic_path_appends_the_whole_usage_text() {
    let mut output = Vec::new();
    let mut errors = Vec::new();

    let code = execute(["--ticks", "0"], &mut output, &mut errors, None, None);

    assert_eq!(code, 2);
    assert!(output.is_empty());
    let errors = String::from_utf8(errors).unwrap();
    assert!(errors.starts_with("configuration error: "), "{errors}");
    assert!(errors.ends_with(cli::USAGE), "{errors}");
}

#[test]
fn a_density_resolving_to_no_resources_exits_with_code_two_before_initialization() {
    let mut output = Vec::new();
    let mut errors = Vec::new();

    let code = execute(["--density", "0.01"], &mut output, &mut errors, None, None);

    assert_eq!(code, 2);
    assert!(
        output.is_empty(),
        "rejection must happen before any simulation output"
    );
    let errors = String::from_utf8(errors).unwrap();
    assert!(errors.contains("zero resources"), "{errors}");
    assert!(errors.contains("Usage:"));
}

/// `REQ-MOK-031`, `REQ-MOK-033`: the third source reaches the process boundary, and the trait is
/// reported once per Mokiterion at initialization and never restated afterwards.
///
/// Added under `WO-MOK-010`. This is the only place the whole path — argument, configuration,
/// initialization, run, standard output — is exercised for the new source, so it is where an
/// option accepted by the parser but unreachable from `execute` would be caught. The
/// once-and-only-once assertion is the reporting obligation stated as a count: a value that is
/// fixed for the run must not be re-announced, and a per-tick restatement would be the first
/// symptom of a trait that had become mutable.
#[test]
fn the_trait_aware_source_runs_to_completion_and_reports_each_trait_once() {
    let mut output = Vec::new();
    let mut errors = Vec::new();

    let code = execute(
        ["--policy", "individual", "--ticks", "50", "--seed", "42"],
        &mut output,
        &mut errors,
        None,
        None,
    );

    assert_eq!(code, 0);
    assert!(errors.is_empty(), "{}", String::from_utf8_lossy(&errors));
    let output = String::from_utf8(output).unwrap();

    assert_eq!(
        output
            .matches("event=decision_source_selected result=source:individual")
            .count(),
        1
    );
    assert_eq!(output.matches("event=agent_initialized").count(), 12);
    assert_eq!(
        output.matches("waste_tolerance:").count(),
        12,
        "the trait is stated at initialization and nowhere else"
    );

    // `REQ-MOK-032`: every Mokiterion starts at the lower bound of the `fear` range. The pairing
    // with `waste_tolerance:` is what makes this an initialization assertion: it is the field that
    // follows `fear` on an `agent_initialized` line and appears on no other line.
    assert_eq!(
        output.matches(",fear:0,waste_tolerance:").count(),
        12,
        "a Mokiterion was initialized with fear away from the range's lower bound"
    );
    for number in 1..=12 {
        let subject = format!("subject=M{number:02} ");
        assert_eq!(
            output
                .lines()
                .filter(|line| line.contains(&subject))
                .filter(|line| line.contains("waste_tolerance:"))
                .count(),
            1,
            "M{number:02} does not state its trait exactly once"
        );
    }
    assert_eq!(output.matches("summary ").count(), 1);
}

/// `REQ-MOK-057`, `REQ-MOK-058`: the fourth source reaches the process boundary, and a run under
/// it exits successfully whatever its outcome.
///
/// The exit code is the point of the last assertion. `REQ-MOK-058` states that habitability is a
/// measured property rather than a runtime error, so a run that depopulates or that never reaches a
/// kill must still exit `0` — an engine that failed the process on its own survivor count would be
/// reading a population aggregate, which `REQ-MOK-059` forbids outright.
#[test]
fn the_social_source_runs_to_completion_and_exits_successfully() {
    let mut output = Vec::new();
    let mut errors = Vec::new();

    let code = execute(
        ["--policy", "social", "--ticks", "200", "--seed", "42"],
        &mut output,
        &mut errors,
        None,
        None,
    );

    assert_eq!(code, 0);
    assert!(errors.is_empty(), "{}", String::from_utf8_lossy(&errors));
    let output = String::from_utf8(output).unwrap();

    assert_eq!(
        output
            .matches("event=decision_source_selected result=source:social")
            .count(),
        1
    );
    assert_eq!(output.matches("event=agent_initialized").count(), 12);
    assert_eq!(output.matches("summary ").count(), 1);
    // The source is reached, not merely selected: a run under it proposes what no other source
    // can propose.
    assert!(
        output.contains("event=threat_resolved") || output.contains("event=attack_resolved"),
        "two hundred ticks under `social` resolved nothing between Mokiterions"
    );
}

#[test]
fn output_failure_exits_with_code_one() {
    let mut output = FailingWriter;
    let mut errors = Vec::new();

    let code = execute(["--ticks", "1"], &mut output, &mut errors, None, None);

    assert_eq!(code, 1);
    assert!(String::from_utf8(errors).unwrap().contains("runtime error"));
}

/// A port that answers every opportunity and keeps every record, needing no file.
///
/// The only helper in this file besides `FailingWriter`, and it is here for the same reason: it belongs
/// beside the one test that uses it. `SPEC-MOK-007` rule 1.1 is what makes it sufficient — the engine's
/// one interface for a decision from outside itself names no provider, no transport and no file, so a
/// port assembled in a test is indistinguishable to the engine from the reader-backed one a host lends.
struct WaitingPort {
    records: Vec<String>,
}

impl Proposer for WaitingPort {
    fn propose(&mut self, _request: DecisionRequest) -> Option<Action> {
        Some(Action::Wait)
    }

    fn record(&mut self, record: &str) -> io::Result<()> {
        self.records.push(record.to_string());
        Ok(())
    }
}

/// `SPEC-MOK-006` rule 1.2 and `SPEC-MOK-007` rule 12.1.1: the library resolves no transcript path.
///
/// The option's two halves are split across two targets — the parser validates it and retains nothing,
/// the host opens it — and the gap between them is where a path could reach the library unnoticed. This
/// closes it from the library's side: the value below cannot be opened, cannot be created and is not a
/// legal path on either platform, and the run completes anyway, because nothing in this target ever
/// looks at it. The decisions come from the port the caller lent, which is the whole of rule 12.1.1.
#[test]
fn the_library_never_opens_the_transcript_it_was_told_about() {
    let mut output = Vec::new();
    let mut errors = Vec::new();
    let mut port = WaitingPort {
        records: Vec::new(),
    };

    let code = execute(
        [
            "--policy",
            "llm",
            "--ticks",
            "2",
            "--seed",
            "42",
            "--transcript-path",
            "\0/no/such/path/that/could/ever/open",
        ],
        &mut output,
        &mut errors,
        None,
        Some(&mut port),
    );

    assert_eq!(code, 0, "{}", String::from_utf8_lossy(&errors));
    assert!(errors.is_empty(), "{}", String::from_utf8_lossy(&errors));
    let output = String::from_utf8(output).unwrap();
    assert!(
        output.contains("event=decision_source_selected result=source:llm"),
        "{output}"
    );
    // Thirty-six records: the head's one per Mokiterion, written before the first exchange, and then
    // twelve opportunities in each of two ticks. Nothing dies while every Mokiterion waits, so the
    // figure is exact rather than a lower bound, and every opportunity reached the port rather than a
    // source the engine chose for itself.
    assert_eq!(port.records.len(), 12 + 24);

    // And the run's own text says nothing about the path it was handed, which is rule 19.7: the engine
    // resolved none and has nothing to say about the one a host opened.
    assert!(!output.contains("no/such/path"), "{output}");
}
