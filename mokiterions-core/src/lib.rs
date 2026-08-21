//! The Mokiterions simulation engine.
//!
//! This crate owns every simulation rule fixed by `SPEC-MOK-001`, the closed public
//! interface fixed by `SPEC-MOK-002` rules 5 and 6, and the read-only observation surface
//! that `SPEC-MOK-003` adds to that interface. It has no external dependencies: that is what
//! `SPEC-MOK-002` rule 13's declared set records, and as of 2026-08-20 it is a measurement
//! this crate is held to rather than a prohibition. `ARCH-MOK-001` as amended admitted no
//! exception, including a dependency shared with `mokiterions-tui`; `ADR-MOK-006` replaced
//! that with a comparison, so an external crate reaching this graph without an amendment to
//! rule 13 refuses, and one prohibition is untouched — no user-interface crate may enter it,
//! which is `REQ-MOK-026` and the split `ADR-MOK-003` decided. What the simulation rules are
//! made of does not change either way: `ADR-MOK-006` decision 11 reserves simulation
//! semantics to this crate, so no third-party crate may supply a rule the specifications fix.
//!
//! The public interface carries values only. No public item yields a mutable borrow of, or
//! a reference into, the world grid, the agent collection, the resource collection, the
//! tick counter, the entropy state or the event log, in any build configuration including
//! test builds, and none returns a handle that permits mutation. That prohibition is
//! `SPEC-MOK-002` rule 6, and it preserves `REQ-MOK-004` and `ADR-MOK-001`. `SPEC-MOK-003`
//! narrowed it from a list of type names to the capability it exists to deny, because the
//! observation snapshots carry five of those names — `Coordinate`, `Direction`, `Territory`,
//! `FoodClass` and `Action` — by value. The ten others it lists stay private, including
//! `Observation` and `DecisionSource`, which carry the `ADR-MOK-001` trust boundary.
//!
//! `WO-MOK-010` grew that interface by exactly two things, both of them values: a third
//! `Policy` variant and a fourth attribute on the observation snapshot's Mokiterion entry.
//! The trait-aware source, the `Observation` it reads and the `waste_tolerance` that
//! observation carries all stay on the private side of the same boundary, so a Mokiterion's
//! trait reaches a host only as text in the retained event log.
//!
//! Two hosts drive the same interface: the `Mokiterions` binary, which streams the
//! `REQ-MOK-010` text record to standard output, and the `mokiterions-tui` observer, which
//! advances one tick at a time and reads snapshots. Neither is privileged; the engine owns
//! all mutable state and validates every proposed action regardless of its source.

use std::io::Write;

pub mod cli;
pub mod simulation;

use cli::Command;
use simulation::Simulation;

/// The process boundary. Maps arguments and the caller's writers to an exit code and owns
/// no state: `0` on success or help, `1` on output failure, `2` on invalid configuration,
/// with the usage text written to standard error on invalid configuration.
///
/// `records` is the structured record stream's sink, `SPEC-MOK-006`'s subject. It is written
/// when it is present and nothing is produced when it is absent, and the run is otherwise
/// the same run either way: the same text bytes, the same entropy draws, the same exit code.
/// **This function resolves no path, opens no file, creates no directory and removes none.**
/// The sink arrives already open, from a caller that owns the destination — rule 1.2, and
/// what keeps `SPEC-MOK-001`'s prohibition on interpreting input as a path true of the
/// library target. A failure to write the sink is an output failure and exits `1`, like a
/// failure to write standard output; rule 13.6 adds no code and changes the meaning of none.
///
/// The parameter takes `&mut dyn Write` rather than a second type parameter so that a caller
/// with no sink writes `None` and nothing else: an `Option<&mut W>` would leave `W`
/// unconstrained at every such call site, and every existing call site is one.
pub fn execute<I, S, W, E>(
    args: I,
    stdout: &mut W,
    stderr: &mut E,
    records: Option<&mut dyn Write>,
) -> u8
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

            match simulation.run_recording(stdout, records) {
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
