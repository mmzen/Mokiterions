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
//! `WO-MOK-025` grows it by one interface and one value type: `simulation::Proposer`, which
//! is `SPEC-MOK-007` rule 1.1's single door for a decision source that does not live here, and
//! `simulation::DecisionRequest`, the one decision opportunity it carries. Both carry values
//! only — the request is composed of the engine's own observation-derived values and owned
//! strings, and a proposal comes back as `Action` or as nothing. **The port names no provider,
//! no transport, no model, no credential, no file and no mode**, so nothing about how a
//! proposal was obtained is expressible in this crate's interface, and rule 20.6's reason for
//! the shape holds: a private adapter implements the private `DecisionSource` in terms of the
//! public port, which is what keeps `Observation` and `DecisionSource` off the public side of
//! the `ADR-MOK-001` boundary while the source itself is public.
//!
//! **This crate builds no port, holds none and closes none, and performs no filesystem
//! operation and spawns no process — for that source as for every other.** Rule 20.4 puts the
//! port's construction and lifetime in the host, which is why both entry points that take one
//! borrow it rather than own it.
//!
//! Two hosts drive the same interface: the `Mokiterions` binary, which streams the
//! `REQ-MOK-010` text record to standard output, and the `mokiterions-tui` observer, which
//! advances one tick at a time and reads snapshots. Neither is privileged; the engine owns
//! all mutable state and validates every proposed action regardless of its source — including a
//! proposal that arrived through the port, which `ADR-MOK-001` fixes as untrusted input and
//! `SPEC-MOK-007` rule 9.6 counts as an ordinary rejected proposal when the rules reject it.

use std::io::Write;

pub mod cli;
pub mod simulation;

use cli::Command;
use simulation::{MISSING_DECISION_PORT, Policy, Proposer, Simulation};

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
/// `port` is `SPEC-MOK-007`'s decision port, the interface through which the `llm` source
/// obtains a proposal from outside this engine. It is `None` for the four deterministic sources
/// and is ignored when one of them is selected, exactly as an absent sink is ignored and
/// without being an error, which is rule 20.9. **Like the sink it arrives already built, from a
/// host that owns it for the whole run**: rule 20.4 puts the port's construction, its lifetime
/// and its closure in the host, so this function spawns no process, opens no connection, reads
/// no environment variable and knows neither what is behind the port nor whether the run is
/// live or a replay.
///
/// Both parameters take `&mut dyn` rather than a further type parameter so that a caller with
/// neither writes `None, None` and nothing else: an `Option<&mut W>` would leave `W`
/// unconstrained at every such call site, and every existing call site is one.
pub fn execute<I, S, W, E>(
    args: I,
    stdout: &mut W,
    stderr: &mut E,
    records: Option<&mut dyn Write>,
    port: Option<&mut dyn Proposer>,
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
            // `SPEC-MOK-007` rule 20.8's refusal, stated here as well as in the library's own
            // check, and for the exit code rather than for the diagnosis. Reaching the same
            // refusal through `run_recording` would report it as a runtime error and exit `1`,
            // an *output* failure, which this is not: it is an invalid configuration and rule 4
            // fixes `2` for one. Rule 19.2 additionally requires it before any tick, which
            // being before `Simulation::new` satisfies with room to spare.
            //
            // The usage text is not written after it, following `Simulation::new`'s precedent
            // rather than the argument parser's. The operator's arguments may be well-formed:
            // what is missing is something only the calling host can supply, so pointing the
            // operator at their own command line would name the wrong mistake.
            if config.policy == Policy::Llm && port.is_none() {
                let _ = writeln!(stderr, "configuration error: {MISSING_DECISION_PORT}");
                return 2;
            }

            let mut simulation = match Simulation::new(config) {
                Ok(simulation) => simulation,
                Err(error) => {
                    let _ = writeln!(stderr, "configuration error: {error}");
                    return 2;
                }
            };

            match simulation.run_recording(stdout, records, port) {
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
