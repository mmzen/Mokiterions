//! The Mokiterions terminal observer.
//!
//! The observer is a separate package from the engine and depends on it as a library. It reads
//! authoritative state through the engine's read-only observation surface and changes simulation
//! state only through the single-tick advance of `SPEC-MOK-003` rule 1.
//!
//! Start-up order is fixed by the specification's error table: inputs are parsed and rejected,
//! then the configuration is rejected, then the viewport floor is checked — all on standard error,
//! all before the terminal is entered. Nothing is written to the alternate screen until every
//! refusal has had its chance, and the terminal is restored on every exit path including a panic.

mod authority;
mod export;
mod layout;
mod options;
mod render;
mod spatial;
mod state;

/// `VER-MOK-005`'s cross-cutting cases, which need every module at once and ship no code.
#[cfg(test)]
mod verification;

use std::env;
use std::io::{self, Write};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event as TerminalEvent};

use crate::options::Startup;
use crate::state::{Observer, Progression};

/// Rule 6.1: at most one frame every 33 milliseconds.
const FRAME_INTERVAL: Duration = Duration::from_millis(33);

/// Rule 6.2: input is polled at most every 16 milliseconds.
const INPUT_INTERVAL: Duration = Duration::from_millis(16);

/// What start-up produced: a refusal with its exit code, or an observer ready to run.
///
/// The observer is boxed because it holds the whole world and the retained records, and a refusal
/// holds one byte; keeping the two arms the same size would make every refusal carry the world.
enum Launch {
    Exit(u8),
    Observe(Box<Observer>),
}

/// Parses, validates and refuses, writing every diagnostic to the caller's streams.
///
/// `viewport` is the terminal's current size, or `None` when it could not be read; in that case
/// the floor is not judged here and entering the terminal decides instead. Taking it as an
/// argument keeps the refusal testable without a terminal.
fn prepare<I, S, W, E>(
    args: I,
    stdout: &mut W,
    stderr: &mut E,
    viewport: Option<(u16, u16)>,
) -> Launch
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
    W: Write,
    E: Write,
{
    let options = match options::parse(args) {
        Ok(Startup::Help) => {
            return match stdout.write_all(options::USAGE.as_bytes()) {
                Ok(()) => Launch::Exit(0),
                Err(error) => {
                    let _ = writeln!(stderr, "output error: {error}");
                    Launch::Exit(1)
                }
            };
        }
        Ok(Startup::Run(options)) => options,
        Err(error) => {
            let _ = writeln!(stderr, "configuration error: {error}");
            let _ = stderr.write_all(options::USAGE.as_bytes());
            return Launch::Exit(2);
        }
    };

    // The engine's own rejection, in the engine's own words, with the engine's own exit code.
    let observer = match Observer::new(options) {
        Ok(observer) => observer,
        Err(error) => {
            let _ = writeln!(stderr, "configuration error: {error}");
            return Launch::Exit(2);
        }
    };

    // Rule 5's floor. Refusing here means a terminal too small to observe in is never altered.
    if let Some((width, height)) = viewport
        && layout::below_floor(width, height)
    {
        let _ = writeln!(
            stderr,
            "viewport error: the terminal is {width} x {height}; the observer requires at least {} x {}",
            layout::MIN_WIDTH,
            layout::MIN_HEIGHT
        );
        return Launch::Exit(2);
    }

    Launch::Observe(Box::new(observer))
}

fn main() -> ExitCode {
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();

    // Read before the terminal is entered, so the floor refusal happens on the operator's own
    // screen. A size that cannot be read is not itself a refusal.
    let viewport = ratatui::crossterm::terminal::size().ok();

    let mut observer = match prepare(env::args().skip(1), &mut stdout, &mut stderr, viewport) {
        Launch::Exit(code) => {
            let _ = stdout.flush();
            let _ = stderr.flush();
            return ExitCode::from(code);
        }
        Launch::Observe(observer) => observer,
    };
    let _ = stdout.flush();

    // `try_init` enters the alternate screen with raw input and installs a panic hook that
    // restores the terminal, which is what makes rule "panic on any path" hold.
    let mut terminal = match ratatui::try_init() {
        Ok(terminal) => terminal,
        Err(error) => {
            let _ = writeln!(stderr, "terminal error: {error}");
            let _ = stderr.flush();
            return ExitCode::from(1);
        }
    };

    let outcome = observe(&mut terminal, &mut observer);
    // Unconditional: the success path, the failure path and the panic path all restore.
    let restoration = ratatui::try_restore();

    let mut code = 0u8;
    if let Err(error) = outcome {
        let _ = writeln!(stderr, "runtime error: {error}");
        code = 1;
    }
    if let Err(error) = restoration {
        let _ = writeln!(stderr, "terminal error: {error}");
        code = 1;
    }
    if code == 0 {
        report(&observer, &mut stderr);
    }
    let _ = stderr.flush();
    ExitCode::from(code)
}

/// The event loop.
///
/// Wall-clock time is read here and only here, to decide when rule 1.2 advances and when rule 6
/// draws. It is never passed to the engine and never enters an authoritative value.
fn observe(terminal: &mut DefaultTerminal, observer: &mut Observer) -> Result<(), String> {
    let mut last_advance = Instant::now();
    let mut last_draw: Option<Instant> = None;
    let mut last_poll: Option<Instant> = None;
    // The first frame is drawn before anything is waited for, so a held run is visible at once.
    let mut force_draw = true;

    loop {
        let now = Instant::now();

        if due(last_poll, now, INPUT_INTERVAL) {
            last_poll = Some(now);
            // Every pending event is drained and applied exactly once (rule 6.2).
            loop {
                match event::poll(Duration::ZERO) {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(error) => {
                        // Reported in the header; progression continues under rule 1.
                        observer.set_notice(format!("input read failed: {error}"));
                        break;
                    }
                }
                match event::read() {
                    Ok(TerminalEvent::Key(key)) => {
                        let response = observer.handle_key(key)?;
                        if response.quit {
                            observer.mark_ended_early();
                            return Ok(());
                        }
                        force_draw |= response.force_draw;
                    }
                    // A resize reaches the layout on the next frame and reaches nothing else.
                    Ok(TerminalEvent::Resize(..)) => force_draw = true,
                    Ok(_) => {}
                    Err(error) => {
                        observer.set_notice(format!("input read failed: {error}"));
                        break;
                    }
                }
            }
        }

        let interval = tick_interval(observer.speed());
        if observer.progression() == Progression::Running && !observer.is_finished() {
            if now.duration_since(last_advance) >= interval {
                observer.advance()?;
                // Measured from this advance, never from the schedule: falling behind slows the
                // run and never advances two ticks in zero elapsed time (rule 1.2).
                last_advance = now;
            }
        } else {
            // Holding does not accrue a debt that would burst on release.
            last_advance = now;
        }

        if force_draw || due(last_draw, now, FRAME_INTERVAL) {
            force_draw = false;
            last_draw = Some(now);
            if let Err(error) = terminal.draw(|frame| render::draw(frame, observer)) {
                // A draw failure is a presentation failure, not a simulation result: it is
                // reported and the run continues.
                observer.set_notice(format!("draw failed: {error}"));
            }
        }

        let idle = idle_for(observer, interval, last_draw, last_poll, last_advance);
        if !idle.is_zero() {
            std::thread::sleep(idle);
        }
    }
}

/// Rule 1.2's interval: `1000 / speed` milliseconds.
fn tick_interval(speed: u32) -> Duration {
    Duration::from_millis(1000 / u64::from(speed.max(1)))
}

/// Whether an interval has elapsed. An event that has never happened is due immediately.
fn due(last: Option<Instant>, now: Instant, interval: Duration) -> bool {
    last.is_none_or(|last| now.duration_since(last) >= interval)
}

/// How long to sleep: until the nearest of the three deadlines, so the loop neither spins nor
/// oversleeps a cadence.
fn idle_for(
    observer: &Observer,
    interval: Duration,
    last_draw: Option<Instant>,
    last_poll: Option<Instant>,
    last_advance: Instant,
) -> Duration {
    let now = Instant::now();
    let remaining = |last: Option<Instant>, interval: Duration| match last {
        None => Duration::ZERO,
        Some(last) => interval.saturating_sub(now.duration_since(last)),
    };

    let mut idle = remaining(last_poll, INPUT_INTERVAL).min(remaining(last_draw, FRAME_INTERVAL));
    if observer.progression() == Progression::Running && !observer.is_finished() {
        idle = idle.min(remaining(Some(last_advance), interval));
    }
    idle
}

/// Rule 12.5: a run the operator ended reports itself as ended early, never as completed.
fn report<E: Write>(observer: &Observer, stderr: &mut E) {
    let tick = observer.snapshot().tick;
    if observer.ended_early() {
        let _ = writeln!(
            stderr,
            "ended early at tick {tick} of {}; the retained events are a prefix of the unobserved run",
            observer.config().tick_limit
        );
    } else if let Some(reason) = observer.termination_reason() {
        let _ = writeln!(stderr, "completed at tick {tick}: {reason}");
    } else {
        let _ = writeln!(stderr, "ended at tick {tick}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepared(args: &[&str], viewport: Option<(u16, u16)>) -> (Launch, String, String) {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let launch = prepare(args.to_vec(), &mut stdout, &mut stderr, viewport);
        (
            launch,
            String::from_utf8(stdout).unwrap(),
            String::from_utf8(stderr).unwrap(),
        )
    }

    fn code_of(launch: &Launch) -> Option<u8> {
        match launch {
            Launch::Exit(code) => Some(*code),
            Launch::Observe(_) => None,
        }
    }

    #[test]
    fn help_exits_successfully_on_standard_output() {
        let (launch, stdout, stderr) = prepared(&["--help"], Some((160, 48)));
        assert_eq!(code_of(&launch), Some(0));
        assert_eq!(stdout, options::USAGE);
        assert!(stderr.is_empty());
    }

    #[test]
    fn an_invalid_input_is_refused_before_the_terminal_with_code_two() {
        for args in [
            vec!["--ticks", "0"],
            vec!["--speed", "3"],
            vec!["--density", "0.01"],
            vec!["--policy", "oracle"],
            vec!["--export"],
            vec!["--seed"],
        ] {
            let (launch, stdout, stderr) = prepared(&args, Some((160, 48)));
            assert_eq!(code_of(&launch), Some(2), "{args:?}");
            assert!(stdout.is_empty(), "{args:?} wrote to standard output");
            assert!(stderr.contains("configuration error"), "{args:?}: {stderr}");
        }
    }

    #[test]
    fn a_viewport_below_the_floor_is_refused_with_both_dimensions_and_code_two() {
        let (launch, stdout, stderr) = prepared(&[], Some((33, 21)));
        assert_eq!(code_of(&launch), Some(2));
        assert!(stdout.is_empty());
        assert!(stderr.contains("33 x 21"), "{stderr}");
        assert!(stderr.contains("34 x 22"), "{stderr}");

        // Exactly at the floor it is accepted.
        let (launch, _, stderr) = prepared(&[], Some((34, 22)));
        assert!(code_of(&launch).is_none(), "{stderr}");

        // A size that cannot be read is not itself a refusal.
        let (launch, _, _) = prepared(&[], None);
        assert!(code_of(&launch).is_none());
    }

    /// `--export` is data at start-up: validated as a string, never opened, never used to read.
    #[test]
    fn an_export_path_is_not_touched_at_start_up() {
        let directory = env::temp_dir().join("mokiterions-startup-export");
        let _ = std::fs::remove_dir_all(&directory);
        let path = directory.join("nested").join("events.log");
        let path = path.to_str().unwrap().to_string();

        let (launch, _, stderr) = prepared(&["--export", &path], Some((160, 48)));
        assert!(code_of(&launch).is_none(), "{stderr}");
        assert!(!std::fs::exists(&path).unwrap_or(false));
        assert!(!std::fs::exists(&directory).unwrap_or(false));
    }

    #[test]
    fn the_tick_interval_is_a_thousand_milliseconds_over_the_speed() {
        assert_eq!(tick_interval(1), Duration::from_millis(1000));
        assert_eq!(tick_interval(8), Duration::from_millis(125));
        assert_eq!(tick_interval(64), Duration::from_millis(15));
        // Never a division by zero, whatever reaches it.
        assert_eq!(tick_interval(0), Duration::from_millis(1000));
    }

    #[test]
    fn a_cadence_that_has_never_run_is_due_and_one_just_run_is_not() {
        let now = Instant::now();
        assert!(due(None, now, FRAME_INTERVAL));
        assert!(!due(Some(now), now, FRAME_INTERVAL));
        assert!(due(Some(now), now + FRAME_INTERVAL, FRAME_INTERVAL));
        assert!(!due(
            Some(now),
            now + Duration::from_millis(32),
            FRAME_INTERVAL
        ));
    }

    #[test]
    fn a_run_the_operator_ended_reports_itself_as_ended_early() {
        let Launch::Observe(mut observer) = prepared(&["--ticks", "3"], Some((160, 48))).0 else {
            panic!("expected an observer");
        };
        observer.advance().unwrap();
        observer.mark_ended_early();

        let mut stderr = Vec::new();
        report(&observer, &mut stderr);
        let stderr = String::from_utf8(stderr).unwrap();
        assert!(stderr.contains("ended early at tick 1 of 3"), "{stderr}");
        assert!(stderr.contains("prefix"), "{stderr}");

        // Run to the engine's own end and it reports completion with the engine's reason.
        while !observer.is_finished() {
            observer.advance().unwrap();
        }
        observer.mark_ended_early();
        let mut stderr = Vec::new();
        report(&observer, &mut stderr);
        let stderr = String::from_utf8(stderr).unwrap();
        assert!(
            stderr.contains("completed at tick 3: tick_limit"),
            "{stderr}"
        );
    }

    /// The loop sleeps rather than spins, and never longer than the shortest cadence.
    #[test]
    fn the_idle_wait_never_exceeds_the_nearest_deadline() {
        let Launch::Observe(observer) = prepared(&[], Some((160, 48))).0 else {
            panic!("expected an observer");
        };
        let now = Instant::now();

        // Nothing has been drawn or polled yet, so there is nothing to wait for.
        assert_eq!(
            idle_for(&observer, tick_interval(8), None, None, now),
            Duration::ZERO
        );
        // Having just polled and drawn, the wait is bounded by the input cadence.
        let idle = idle_for(&observer, tick_interval(1), Some(now), Some(now), now);
        assert!(idle <= INPUT_INTERVAL, "{idle:?}");
    }
}
