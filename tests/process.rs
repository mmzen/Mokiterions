//! Public tier, `SPEC-MOK-002` rule 8: the process boundary.
//!
//! Relocated from `src/main.rs` under `WO-MOK-003`. Every test here needs only `execute`,
//! which rule 4 moved to the library target, `cli::USAGE`, and two byte buffers.
//! Assertions are verbatim, as rule 12 requires.

use std::io::{self, Write};

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

    let code = execute(["--help"], &mut output, &mut errors);

    assert_eq!(code, 0);
    assert_eq!(String::from_utf8(output).unwrap(), cli::USAGE);
    assert!(errors.is_empty());
}

#[test]
fn invalid_configuration_exits_with_code_two() {
    let mut output = Vec::new();
    let mut errors = Vec::new();

    let code = execute(["--ticks", "0"], &mut output, &mut errors);

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

    let code = execute(["--ticks", "0"], &mut output, &mut errors);

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

    let code = execute(["--density", "0.01"], &mut output, &mut errors);

    assert_eq!(code, 2);
    assert!(
        output.is_empty(),
        "rejection must happen before any simulation output"
    );
    let errors = String::from_utf8(errors).unwrap();
    assert!(errors.contains("zero resources"), "{errors}");
    assert!(errors.contains("Usage:"));
}

#[test]
fn output_failure_exits_with_code_one() {
    let mut output = FailingWriter;
    let mut errors = Vec::new();

    let code = execute(["--ticks", "1"], &mut output, &mut errors);

    assert_eq!(code, 1);
    assert!(String::from_utf8(errors).unwrap().contains("runtime error"));
}
