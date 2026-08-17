//! Library target for the Mokiterions simulation foundation.
//!
//! The public interface of this target is a closed enumeration owned by `SPEC-MOK-002`
//! rule 5. It carries values only: configuration in, and copies of already-reported
//! outcome facts out. No public item yields a mutable borrow of, an owned copy of, or a
//! reference into the world grid, the agent collection, the resource collection, the tick
//! counter, the entropy state, or the event log, in any build configuration including
//! test builds. That prohibition is `SPEC-MOK-002` rule 6 and it preserves `REQ-MOK-004`
//! and `ADR-MOK-001`.

use std::io::Write;

pub mod cli;
pub mod simulation;

use cli::Command;
use simulation::Simulation;

/// The process boundary. Maps arguments and two writers to an exit code and owns no
/// state: `0` on success or help, `1` on output failure, `2` on invalid configuration,
/// with the usage text written to standard error on invalid configuration.
pub fn execute<I, S, W, E>(args: I, stdout: &mut W, stderr: &mut E) -> u8
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
