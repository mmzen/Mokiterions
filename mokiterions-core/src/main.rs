//! The `Mokiterions` binary: the host that owns the process's streams and files.
//!
//! Everything the library refuses to do lives here. `SPEC-MOK-006` rule 1.2 keeps path
//! resolution, file creation, overwriting and removal out of the engine's library target
//! entirely, so this target resolves the record stream's destination, opens it, hands the
//! open sink to [`execute`], and afterwards decides whether the file it created may survive.
//! The engine never learns the path, which is what keeps rule 5.5 — no record carries the
//! sink's own destination — a property of the design rather than of a writer's discipline.

use std::env;
use std::fs;
use std::io::{self, BufWriter, Write};
use std::process::ExitCode;

use mokiterions::cli::{self, Command};
use mokiterions::execute;

/// The option whose value names the record stream's destination.
///
/// Spelled here and in the parser in `cli.rs`, because the parser validates the option and
/// keeps nothing while this target reads the value it will open. `tests/cli.rs` holds the two
/// spellings equal, so neither can move alone.
const EVENTS_PATH_OPTION: &str = "--events-path";

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();

    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut stderr = BufWriter::new(stderr.lock());

    let code = run(&arguments, &mut stdout, &mut stderr);

    let _ = stderr.flush();
    ExitCode::from(code)
}

/// The whole of this target's behavior, over writers rather than the process's own streams.
///
/// Standard output is flushed in here rather than by the caller, because a flush failure is
/// part of the exit code and the exit code is what decides whether a created record file may
/// survive.
fn run<W: Write, E: Write>(arguments: &[String], stdout: &mut W, stderr: &mut E) -> u8 {
    // The arguments are parsed twice: once here for the verdict, once inside `execute` for the
    // run. `cli::parse` is a pure function of its input, so the two agree by construction, and
    // the alternative — resolving a destination before knowing the configuration is valid —
    // would touch a path for a process that runs nothing. `--help` and an invalid
    // configuration therefore open no file, which is rule 13.1's "runs nothing" taken to
    // include the filesystem.
    let destination = match cli::parse(arguments.iter().cloned()) {
        Ok(Command::Run(_)) => events_path(arguments),
        Ok(Command::Help) | Err(_) => None,
    };

    let Some(destination) = destination else {
        let mut code = execute(arguments.iter().cloned(), stdout, stderr, None);
        if stdout.flush().is_err() {
            code = 1;
        }
        return code;
    };

    // Rule 13.2: a well-formed path the platform refuses is a runtime failure, reported with
    // the platform's reason, exiting `1` before any tick runs and before any text record is
    // written. Opening first is what makes that true — a run that cannot be recorded is not
    // run.
    //
    // `create_new` is attempted before the truncating open so that "this process created the
    // destination" is established by the platform rather than assumed, which is what rule 13.4
    // bounds removal to. A destination that already exists is then replaced, as the option's
    // contract requires, and remembered as one this process did not create, so that a later
    // failure leaves the operator's file where it is instead of deleting it.
    let opened = match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&destination)
    {
        Ok(file) => Ok((file, true)),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            fs::File::create(&destination).map(|file| (file, false))
        }
        Err(error) => Err(error),
    };
    let (file, created) = match opened {
        Ok(pair) => pair,
        Err(error) => {
            let _ = writeln!(stderr, "runtime error: record sink {destination}: {error}");
            return 1;
        }
    };

    let mut sink = BufWriter::new(file);
    let mut code = execute(arguments.iter().cloned(), stdout, stderr, Some(&mut sink));

    // One flush, and then the buffer and the file are taken apart by hand. `BufWriter`'s `Drop`
    // flushes and discards the result, and after a failed flush that is precisely the retry
    // rule 13.3 forbids; `into_parts` does not flush, so the unwritten bytes are handed back
    // here and dropped. That is the whole reason it is used instead of letting the value fall
    // out of scope.
    if let Err(error) = sink.flush() {
        let _ = writeln!(stderr, "runtime error: record sink {destination}: {error}");
        code = 1;
    }
    let (file, _unwritten) = sink.into_parts();
    // Closed before any removal is attempted, because a platform may refuse to remove a file
    // that is still open. Closing itself is unobservable — `File`'s `Drop` cannot report it and
    // the standard library offers no way to ask — so the flush above is where rule 13.3's
    // "including a failed flush or close" is actually observed.
    drop(file);

    if stdout.flush().is_err() {
        code = 1;
    }

    if code != 0 {
        // Rule 13.4: no partial stream survives to be read as a complete run. The condition is
        // the exit code rather than the sink's own failure, because a text-stream failure ends
        // the run just as abruptly and leaves a record stream with no run record, which is
        // just as partial and just as misleading.
        if created {
            if let Err(error) = fs::remove_file(&destination) {
                // In addition to the original failure, never instead of it, and the exit code
                // does not move.
                let _ = writeln!(
                    stderr,
                    "runtime error: record sink {destination}: removal failed: {error}"
                );
            }
        } else {
            // "Where that cannot be established the process does not remove it and says so."
            let _ = writeln!(
                stderr,
                "runtime error: record sink {destination}: not removed: this process did not create the destination"
            );
        }
    }

    code
}

/// The value of [`EVENTS_PATH_OPTION`], from an argument list that has already parsed.
///
/// A positional scan is exact here, and exact only here: `cli::parse` consumes every value
/// option as a pair and rejects any value that begins with `--`, so in a list that parsed
/// successfully the token can appear only at an option position and never as some other
/// option's value. This function is not called on a list that failed to parse, which is why
/// the caller checks the verdict first.
fn events_path(arguments: &[String]) -> Option<String> {
    let position = arguments
        .iter()
        .position(|argument| argument == EVENTS_PATH_OPTION)?;
    arguments.get(position + 1).cloned()
}
