//! The `Mokiterions` binary: the host that owns the process's streams and files.
//!
//! Everything the library refuses to do lives here. `SPEC-MOK-006` rule 1.2 keeps path
//! resolution, file creation, overwriting and removal out of the engine's library target
//! entirely, so this target resolves the record stream's destination, opens it, hands the
//! open sink to [`execute`], and afterwards decides whether the file it created may survive.
//! The engine never learns the path, which is what keeps rule 5.5 — no record carries the
//! sink's own destination — a property of the design rather than of a writer's discipline.
//!
//! Under `WO-MOK-025` it owns a second file for the same reason. `SPEC-MOK-007` rule 20.1 makes
//! this target the recording host and rule 12.1.1 says the host "opens the transcript and lends
//! the engine an already-open reader", so the port is built here, from a file this target opened,
//! and it lives for the whole run — rule 20.4.1, and the cursor rule 12.1's ordering depends on
//! is the reason it may not be rebuilt per tick.
//!
//! **Under `WO-MOK-026` it spawns the connector, and it is the only thing in this repository that
//! does.** Rule 10.1 makes the connector an operator-named executable started as a child process,
//! `SPEC-MOK-006` rule 1.2 keeps every path and every process out of the library target, and rule
//! 20.1 makes this the only host a live run is reachable from at all. So the spawn is here, the two
//! pipes are connected here, the child is reaped here, and the library is handed two streams and a
//! transcript sink and told nothing else.
//!
//! **This target reads no credential, and the reading is not merely absent — there is no code for
//! it.** Rules 10.5 and 13.4 put the credential in the connector's own environment and nowhere
//! else, and the mechanism is that a spawned child inherits this process's environment by default:
//! the pass-through is `std::process::Command`'s behaviour with no environment call made on it, so
//! this file contains no `env::var`, no `env` builder call and no variable name. `env::args` is the
//! whole of this target's contact with `std::env`, and `VER-MOK-018` case `S3a` is what checks that
//! claim rather than trusting this sentence.

use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::process::{Child, Command as Spawn, ExitCode, Stdio};

use mokiterions::cli::{self, Command};
use mokiterions::execute;
use mokiterions::simulation::{ConnectorPort, Proposer, ReplayPort};

/// The option whose value names the record stream's destination.
///
/// Spelled here and in the parser in `cli.rs`, because the parser validates the option and
/// keeps nothing while this target reads the value it will open. `tests/cli.rs` holds the two
/// spellings equal, so neither can move alone.
const EVENTS_PATH_OPTION: &str = "--events-path";

/// The option whose value names the transcript this run replays.
///
/// Spelled here for the reason above, and held equal to the parser's spelling by the same test.
/// The difference from the sink is the direction: this file is opened for reading and is never
/// created, replaced or removed, so none of the sink's survival reasoning below applies to it.
const TRANSCRIPT_PATH_OPTION: &str = "--transcript-path";

/// The option whose value names the connector program this run spawns.
///
/// Spelled here and in the parser for the reason the two above give, and held equal by the same
/// test. This one is neither opened nor created: it is handed to the platform as a program to
/// execute, which is `SPEC-MOK-007` rule 10.1 and the one thing in this repository that starts a
/// process.
const CONNECTOR_PATH_OPTION: &str = "--connector-path";

/// The option whose value names the transcript this run writes.
///
/// The sink counterpart of [`TRANSCRIPT_PATH_OPTION`], and rule 18.4.4 makes the two mutually
/// exclusive: a run replays from a recorded transcript or records a live one, never both. The
/// parser refuses the pair, so this target never has to decide which one an operator meant.
const TRANSCRIPT_OUTPUT_OPTION: &str = "--transcript-output";

/// The option that selects a live run, and therefore rule 13.1's first gate.
///
/// It takes no value, so it is looked for rather than read. Rule 13.2 is the whole of its effect
/// here: **absent it, no connector is spawned at all** — not spawned and then refused, not started
/// and told to make no call. The spawn lives inside the one branch this flag opens.
const LIVE_OPTION: &str = "--live";

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
    let (destination, transcript, connector, output, live) =
        match cli::parse(arguments.iter().cloned()) {
            Ok(Command::Run(_)) => (
                argument_after(arguments, EVENTS_PATH_OPTION),
                argument_after(arguments, TRANSCRIPT_PATH_OPTION),
                argument_after(arguments, CONNECTOR_PATH_OPTION),
                argument_after(arguments, TRANSCRIPT_OUTPUT_OPTION),
                contains_option(arguments, LIVE_OPTION),
            ),
            Ok(Command::Help) | Err(_) => (None, None, None, None, false),
        };

    // `SPEC-MOK-007` rule 13.2 and rule 13.1's selection gate, as a branch and not as a check.
    // **Nothing below this point spawns anything**, and nothing inside the branch asks again
    // whether a live run was selected: the flag decides once, here, and the whole of the live path
    // is on the other side of it. Rule 13.1 is why the credential is not consulted anywhere near
    // this decision — "neither component can satisfy the other's condition", so this host decides
    // the selection and the connector decides the credential, and a host that checked both would
    // be a single component able to authorise spending.
    //
    // A live run and a replay cannot coexist: rule 18.4.4's mutual exclusion is enforced by the
    // parser above, so the early return here cannot skip a transcript somebody asked to replay.
    if live {
        return run_live(arguments, stdout, stderr, destination, connector, output);
    }

    // The transcript is opened first, and it is only ever read. Two reasons, and the second is
    // the one that fixes the order: opening it creates nothing, so a failure here leaves the
    // filesystem exactly as it was; and a run whose decisions cannot be obtained must not first
    // have created a record file for the removal logic below to take away again. A missing
    // transcript is then a failure that touched nothing.
    let mut port = match transcript.as_deref() {
        None => None,
        Some(path) => match fs::File::open(path) {
            // Buffered because `ReplayPort` reads a line at a time and an unbuffered `File`
            // would reach the platform once per byte for every record in the transcript.
            Ok(file) => {
                let mut reader = BufReader::new(file);
                // The first read is forced here, because opening is not where every platform
                // refuses. A directory opens successfully on Linux and fails only when it is
                // read, which without this would be after the run had begun and printed its
                // first tick — the partial run the paragraph above says cannot happen, and the
                // one CI's Linux lane found. `fill_buf` peeks without consuming, so the port
                // below reads exactly the bytes it would have read anyway, and an empty
                // transcript is not an error here: it replays as one that ran out at the first
                // opportunity, which is a different case with its own treatment.
                if let Err(error) = reader.fill_buf() {
                    let _ = writeln!(stderr, "runtime error: transcript {path}: {error}");
                    return 1;
                }
                Some(ReplayPort::new(reader))
            }
            Err(error) => {
                // Rule 13.2's treatment, applied to a read: a well-formed path the platform
                // refuses is a runtime failure and not invalid configuration, so it exits `1` and
                // the usage text is not written after it. Rule 19.7 forbids an error message
                // carrying a path the *engine* resolved; this one is the operator's own argument,
                // resolved by this target, and naming it is the whole use of the message.
                let _ = writeln!(stderr, "runtime error: transcript {path}: {error}");
                return 1;
            }
        },
    };

    run_with_port(
        arguments,
        stdout,
        stderr,
        destination,
        port.as_mut().map(|port| port as &mut dyn Proposer),
    )
}

/// The run itself, once the decision port exists — the record sink, the call into the library,
/// the flushes and rule 13.4's removal.
///
/// Extracted from `run` under `WO-MOK-026` and unchanged in what it does. The reason for the
/// extraction is that there are now two ways to obtain a port and exactly one run to lend it to:
/// a transcript opened for reading, and a connector spawned and connected. Everything from here
/// down is identical for both, and a second copy of rule 13.4's removal logic — the one place in
/// this target that deletes a file — is the copy that would drift.
///
/// The port arrives as `Option<&mut dyn Proposer>` and not as a generic, because the two callers
/// hold different concrete types and this function's whole contract is that it cannot tell them
/// apart. `SPEC-MOK-007` rule 20.4 is the same statement one level down: the host owns the port
/// and lends it, and what it lends is an interface.
fn run_with_port<W: Write, E: Write>(
    arguments: &[String],
    stdout: &mut W,
    stderr: &mut E,
    destination: Option<String>,
    port: Option<&mut dyn Proposer>,
) -> u8 {
    let Some(destination) = destination else {
        let mut code = execute(arguments.iter().cloned(), stdout, stderr, None, port);
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
    let mut code = execute(
        arguments.iter().cloned(),
        stdout,
        stderr,
        Some(&mut sink),
        port,
    );

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

/// The value following the named option, from an argument list that has already parsed.
///
/// A positional scan is exact here, and exact only here: `cli::parse` consumes every value
/// option as a pair and rejects any value that begins with `--`, so in a list that parsed
/// successfully the token can appear only at an option position and never as some other
/// option's value. This function is not called on a list that failed to parse, which is why
/// the caller checks the verdict first.
///
/// One function for both of this target's options rather than one each: the two differ only in
/// which spelling they look for, and a second copy of this reasoning is a second place for it to
/// be got wrong.
fn argument_after(arguments: &[String], option: &str) -> Option<String> {
    let position = arguments.iter().position(|argument| argument == option)?;
    arguments.get(position + 1).cloned()
}

/// Whether the named valueless option appears in an argument list that has already parsed.
///
/// [`argument_after`]'s reasoning about positions applies unchanged, and this needs less of it:
/// a flag has no value to fetch, so only the token's presence matters. It exists because
/// `cli::Config` carries no field for `--live` — the selection changes which host does the run
/// rather than how the engine runs, so `SPEC-MOK-007` rule 13.2 keeps it on this side of the
/// library boundary entirely and there is nothing for the parser to hand back.
fn contains_option(arguments: &[String], option: &str) -> bool {
    arguments.iter().any(|argument| argument == option)
}

/// A live run: spawn the connector, connect its two pipes, lend the port, reap the child.
///
/// **The only function in this repository that starts a process**, which is `SPEC-MOK-007` rule
/// 10.1 and `SPEC-MOK-006` rule 1.2 together. The library target is handed an already-connected
/// reader, an already-connected writer and an already-open transcript sink, and acquires no path,
/// no process handle and no environment read — the condition `WO-MOK-026` names as its second
/// stop-and-escalate case is met here rather than escalated, because the port's shape is what
/// keeps all three on this side of the call.
///
/// **The credential is not read here and there is no code that could read it.** The child inherits
/// this process's environment because that is `Spawn`'s default and no environment call is made on
/// the builder below; rule 10.5 then puts the read in the connector, and rule 13.1's second gate is
/// enforced by a component this one cannot inspect. That is the point of the two-gate design: this
/// host cannot tell whether the credential is present, so it cannot decide to spend money.
///
/// The order of the four steps is chosen and not incidental:
///
/// 1. the spawn, so that a connector the platform cannot start has created nothing;
/// 2. the pipes, taken from the child before anything else can borrow it;
/// 3. the transcript, created with `create_new` so an existing one is never replaced;
/// 4. the run, inside a block, so the port — and with it the child's standard input — is dropped
///    before the child is waited for.
///
/// Step 4's scope is the whole reason the port owns its streams by value. Dropping it closes the
/// pipe, the connector reads end-of-file on its own input and exits, and [`reap`] then returns
/// immediately. A port that borrowed the child's standard input would leave the pipe open, the
/// connector would wait for a request that never comes, and `wait` would never return.
fn run_live<W: Write, E: Write>(
    arguments: &[String],
    stdout: &mut W,
    stderr: &mut E,
    destination: Option<String>,
    connector: Option<String>,
    output: Option<String>,
) -> u8 {
    // Rule 18.4.1's interlocks make both of these present in any list that parsed as a run with
    // `--live`, so this is not a second gate and cannot fire from the command line. It is written
    // rather than asserted because the consequence of being wrong is spawning a connector with
    // nowhere to record what it was paid for, and a diagnostic is a better outcome than a panic
    // for the one property of this function that costs money to get wrong.
    let (Some(connector), Some(output)) = (connector, output) else {
        let _ = writeln!(
            stderr,
            "configuration error: {LIVE_OPTION} requires {CONNECTOR_PATH_OPTION} and {TRANSCRIPT_OUTPUT_OPTION}"
        );
        return 2;
    };

    // Two pipes and nothing else. Standard error is left inherited on purpose: rule 10.2 gives
    // the connector one channel for protocol and this leaves it one for its own diagnostics,
    // which reach the operator's terminal without passing through the engine at all. There is no
    // `.env`, `.env_clear`, `.env_remove` or `.envs` call on this builder, and rule 10.5 is why.
    let mut child = match Spawn::new(&connector)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            // Rule 19.7 forbids a message carrying a path the *engine* resolved. This one is the
            // operator's own argument, resolved by this target, and naming it is the message's
            // whole use — a connector that will not start is a configuration the operator fixes.
            let _ = writeln!(stderr, "runtime error: connector {connector}: {error}");
            return 1;
        }
    };

    // Both are `Some`, because the two `piped` calls above are what create them and nothing has
    // taken them yet. The pattern is written out rather than unwrapped for the reason given above.
    let (Some(requests), Some(responses)) = (child.stdin.take(), child.stdout.take()) else {
        let _ = writeln!(
            stderr,
            "runtime error: connector {connector}: the platform provided no pipes"
        );
        reap(&mut child, &connector, stderr);
        return 1;
    };

    // `create_new`, and deliberately stricter than the record sink above, which replaces what it
    // finds. A live transcript is evidence that was paid for: replacing one silently is the single
    // filesystem outcome in this target that cannot be undone and cannot be re-derived without
    // spending money again. So an existing destination is a refusal, not an overwrite.
    //
    // Created after the spawn, so that a connector that will not start leaves the filesystem
    // exactly as it was. The cost of that order is this failure path, which has a child to reap.
    let mut sink = match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output)
    {
        Ok(file) => file,
        Err(error) => {
            let _ = writeln!(stderr, "runtime error: transcript {output}: {error}");
            // The pipes go first and the child is waited for second, in that order: dropping the
            // write end is what lets the connector reach end-of-file and exit on its own.
            drop(requests);
            drop(responses);
            reap(&mut child, &connector, stderr);
            return 1;
        }
    };

    // Unbuffered, because the port flushes after every record and a buffer that is always empty is
    // a buffer whose failures happen somewhere else. Two writes per exchange reach the platform,
    // against rule 20.2's estimated 0.4–0.8 seconds of provider latency for the same exchange.
    let code = {
        let mut port = ConnectorPort::new(BufReader::new(responses), requests, &mut sink);
        run_with_port(arguments, stdout, stderr, destination, Some(&mut port))
    };
    // The port is gone here, and with it the child's standard input.

    // The transcript is never removed, whatever the exit code — the opposite of the record sink's
    // treatment under rule 13.4. A failed live run spent real money, and the transcript is the
    // only account of what it bought.
    drop(sink);

    reap(&mut child, &connector, stderr);
    code
}

/// Waits for the connector to exit and reports an exit that was not successful.
///
/// **The exit code does not move.** Rule 19.1 fixes this target's exit code to the run's own
/// outcome, and the connector's status is neither a configuration error nor a runtime failure of
/// the engine: a run whose every exchange succeeded is a successful run even if the child then
/// exited badly on its way out. So this reports and returns nothing.
///
/// The message therefore carries neither of the two severity keywords the artifacts fix —
/// `configuration error:` and `runtime error:` are the whole set and inventing a third would put a
/// word in the diagnostic surface that no artifact admits. A component-prefixed sentence says what
/// happened without claiming a severity this target is not entitled to assign.
fn reap<E: Write>(child: &mut Child, connector: &str, stderr: &mut E) {
    match child.wait() {
        Ok(status) if status.success() => {}
        Ok(status) => match status.code() {
            Some(code) => {
                let _ = writeln!(stderr, "connector {connector}: exited with status {code}");
            }
            // A signalled child on a platform that has signals. There is no code to report and
            // the absence is the fact worth reporting.
            None => {
                let _ = writeln!(
                    stderr,
                    "connector {connector}: ended without an exit status"
                );
            }
        },
        Err(error) => {
            let _ = writeln!(
                stderr,
                "connector {connector}: could not be waited for: {error}"
            );
        }
    }
}
