use std::env;
use std::io::{self, BufWriter, Write};
use std::process::ExitCode;

use mokiterions::cli::{self, Command};
use mokiterions::simulation::Simulation;

fn execute<I, S, W, E>(args: I, stdout: &mut W, stderr: &mut E) -> u8
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
    W: Write,
    E: Write,
{
    match cli::parse(args) {
        Ok(Command::Help) => match stdout.write_all(cli::USAGE.as_bytes()) {
            Ok(()) => 0,
            Err(error) => {
                let _ = writeln!(stderr, "output error: {error}");
                1
            }
        },
        Ok(Command::Run(config)) => {
            let mut simulation = match Simulation::new(config) {
                Ok(simulation) => simulation,
                Err(error) => {
                    let _ = writeln!(stderr, "configuration error: {error}");
                    return 2;
                }
            };

            match simulation.run(stdout) {
                Ok(_) => 0,
                Err(error) => {
                    let _ = writeln!(stderr, "runtime error: {error}");
                    1
                }
            }
        }
        Err(error) => {
            let _ = writeln!(stderr, "configuration error: {error}");
            let _ = stderr.write_all(cli::USAGE.as_bytes());
            2
        }
    }
}

fn main() -> ExitCode {
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut stderr = BufWriter::new(stderr.lock());

    let mut code = execute(env::args().skip(1), &mut stdout, &mut stderr);
    if stdout.flush().is_err() {
        code = 1;
    }
    let _ = stderr.flush();
    ExitCode::from(code)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
