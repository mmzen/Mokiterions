//! Public tier: options.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Every one of them was in
//! `src/options.rs`'s `#[cfg(test)] mod tests` block and reached the code through items that were
//! already public, so the move changes the path and nothing else: the assertions are verbatim and
//! no item was widened to bring them out. `SPEC-MOK-004` rule 12 is the obligation and the
//! per-test comparison under `WO-MOK-006` is the evidence.
//!
//! One test below is not from that move. `the_usage_text_advertises_every_policy_the_engine_accepts`
//! was added under `WO-MOK-010`, when the third policy the engine accepts was found missing from the
//! observer's usage text. It reaches `USAGE`, `parse` and `Policy::parse`, all of which were already
//! public, so rule 9's condition holds for it as well.

use mokiterions::simulation::{Density, Policy};
use mokiterions_tui::options::*;

fn run(args: &[&str]) -> Options {
    match parse(args.to_vec()).unwrap() {
        Startup::Run(options) => options,
        Startup::Help => panic!("expected a run"),
    }
}

#[test]
fn defaults_match_the_specified_values() {
    let options = run(&[]);

    assert_eq!(options.speed, 8);
    assert!(!options.start_paused);
    assert_eq!(options.export_path, None);
    assert_eq!(options.config.seed, 0);
    assert_eq!(options.config.tick_limit, 100);
    assert_eq!(options.config.policy, Policy::Reference);
    assert_eq!(options.config.density, Density::DEFAULT);
}

#[test]
fn tracing_is_always_on_and_cannot_be_turned_off() {
    assert!(run(&[]).config.trace_actions);
    assert!(run(&["--trace-actions"]).config.trace_actions);
}

#[test]
fn simulation_inputs_keep_the_engine_parser_and_its_rejections() {
    let options = run(&[
        "--seed",
        "42",
        "--ticks",
        "7",
        "--policy",
        "baseline",
        "--density",
        "1.5",
    ]);
    assert_eq!(options.config.seed, 42);
    assert_eq!(options.config.tick_limit, 7);
    assert_eq!(options.config.policy, Policy::Baseline);
    assert_eq!(options.config.density, Density::parse("1.50").unwrap());

    // Every engine rejection is the engine's, unchanged.
    assert!(parse(vec!["--ticks", "0"]).is_err());
    assert!(parse(vec!["--density", "0.01"]).is_err());
    assert!(parse(vec!["--policy", "random"]).is_err());
    assert!(parse(vec!["--seed", "1", "--seed", "2"]).is_err());
    assert!(parse(vec!["--unknown"]).is_err());
}

#[test]
fn the_usage_text_advertises_every_policy_the_engine_accepts() {
    // `SPEC-MOK-003` rule 5 requires identical parsing, not identical prose, so the observer's
    // usage text is its own and nothing but this test keeps the list it advertises from falling
    // behind the list the engine parses. The match is exhaustive deliberately: a sixth policy
    // stops the compilation here rather than shipping an accepted value the help omits. The
    // fourth one did exactly that under `WO-MOK-016`, and the fifth is why this comment now
    // says sixth — `WO-MOK-025`'s `llm` was stopped here, as designed, and the entry it forced
    // into the observer's usage text is the engine's own verbatim.
    for policy in [
        Policy::Baseline,
        Policy::Reference,
        Policy::Individual,
        Policy::Social,
        Policy::Llm,
    ] {
        let name = match policy {
            Policy::Baseline => "baseline",
            Policy::Reference => "reference",
            Policy::Individual => "individual",
            Policy::Social => "social",
            Policy::Llm => "llm",
        };
        assert_eq!(Policy::parse(name), Some(policy), "{name}");
        assert_eq!(run(&selecting(name)).config.policy, policy, "{name}");
        assert!(USAGE.contains(name), "the usage text omits --policy {name}");
    }
}

/// The arguments that select a source, together with whatever else that source requires.
///
/// Added 2026-08-24 under `WO-MOK-025`, and the engine's own `tests/cli.rs` holds the same helper
/// for the same reason: `--policy llm` alone stopped parsing when `--transcript-path` arrived,
/// because `SPEC-MOK-007` rules 13.2 and 19.2 make a replay with no transcript a usage error and the
/// observer inherits that refusal from the engine's parser rather than making it again.
fn selecting(policy: &str) -> Vec<&str> {
    let mut args = vec!["--policy", policy];
    if policy == "llm" {
        args.push("--transcript-path");
        args.push("transcript.jsonl");
    }
    args
}

#[test]
fn observer_inputs_are_validated() {
    assert_eq!(run(&["--speed", "64"]).speed, 64);
    assert!(run(&["--start-paused"]).start_paused);
    assert_eq!(
        run(&["--export", "events.log"]).export_path.as_deref(),
        Some("events.log")
    );

    assert!(parse(vec!["--speed", "3"]).is_err());
    assert!(parse(vec!["--speed", "0"]).is_err());
    assert!(parse(vec!["--speed", "128"]).is_err());
    assert!(parse(vec!["--speed", "abc"]).is_err());
    assert!(parse(vec!["--speed"]).is_err());
    assert!(parse(vec!["--speed", "--start-paused"]).is_err());
    assert!(parse(vec!["--speed", "8", "--speed", "16"]).is_err());
    assert!(parse(vec!["--start-paused", "--start-paused"]).is_err());
    assert!(parse(vec!["--export"]).is_err());
    assert!(parse(vec!["--export", ""]).is_err());
    assert!(parse(vec!["--export", "a", "--export", "b"]).is_err());
}

#[test]
fn help_wins_over_every_other_input() {
    assert_eq!(parse(vec!["--help"]).unwrap(), Startup::Help);
    assert_eq!(
        parse(vec!["--speed", "16", "--help", "--start-paused"]).unwrap(),
        Startup::Help
    );
}

#[test]
fn an_export_path_is_taken_verbatim_as_data() {
    // A path that looks like an option value, a flag or a traversal is a string.
    for path in ["-x", "a b/c.log", "../../events.log", "sub/dir/events.log"] {
        assert_eq!(
            run(&["--export", path]).export_path.as_deref(),
            Some(path),
            "{path}"
        );
    }
}

#[test]
fn speed_steps_are_clamped_at_both_ends() {
    assert_eq!(faster(1), 2);
    assert_eq!(faster(32), 64);
    assert_eq!(faster(64), 64);
    assert_eq!(slower(64), 32);
    assert_eq!(slower(2), 1);
    assert_eq!(slower(1), 1);
}

/// One entry of a usage text: the line naming the option, and every line indented beneath it.
fn entry(usage: &str, option: &str) -> String {
    let opening = format!("  {option} ");
    let mut lines = usage.lines().skip_while(|line| !line.starts_with(&opening));
    let first = lines
        .next()
        .unwrap_or_else(|| panic!("the usage text has no entry for {option}"));
    let mut text = String::from(first);
    for line in lines.take_while(|line| line.starts_with("      ")) {
        text.push('\n');
        text.push_str(line);
    }
    text
}

/// `WO-MOK-024`: the engine inputs are described here in the engine's own words.
///
/// Extended 2026-08-24 under `WO-MOK-025` from four options to five. `--transcript-path` joins the
/// list because it is the first engine option **both hosts act on**: `SPEC-MOK-007` rule 18.4.2 makes
/// it the whole of the observer's share of the fifth source, so an operator reads about it in this
/// program's help and has to be told the same thing the engine tells them.
///
/// `SPEC-MOK-003`'s *Start-up inputs* section gives them "identical names, identical parsing,
/// identical validation, identical defaults and identical rejection behavior" and leaves their
/// meaning to `SPEC-MOK-001`, which the observer therefore may not restate differently. The
/// observer prints its own text because its synopsis and its own three inputs differ, so the
/// five shared entries exist twice; a copy is only safe while it stays a copy. This reads each
/// entry out of the observer's text and requires the engine's text to contain it byte for byte,
/// so an edit to one description alone fails here and names the option. Neither text is
/// compared to a literal declared in this file, which would move the drift one level up.
#[test]
fn the_shared_entries_are_the_engines_own_words() {
    for option in [
        "--seed",
        "--ticks",
        "--policy",
        "--density",
        "--transcript-path",
    ] {
        let ours = entry(USAGE, option);
        assert!(
            ours.lines().count() > 1,
            "{option} has no description in the observer's text"
        );
        assert!(
            mokiterions::cli::USAGE.contains(&ours),
            "the observer describes {option} in words the engine's own help does not use:\n{ours}"
        );
    }
}

/// `SPEC-MOK-007` rule 18.4: the observer re-reads the raw transcript argument, because the engine's
/// parser validates it and retains nothing.
///
/// The value reaching the field is the operator's own, verbatim, and the engine's configuration is
/// untouched by it — which is the observable form of "the library resolves no path". A path that
/// looks like a flag, a traversal or a name with spaces is a string here, exactly as `--export` is.
#[test]
fn the_transcript_path_is_re_read_verbatim_and_reaches_no_configuration() {
    for path in [
        "transcript.jsonl",
        "-x",
        "a b/c.jsonl",
        "../../t.jsonl",
        "sub/dir/t.jsonl",
    ] {
        let options = run(&["--policy", "llm", "--transcript-path", path]);
        assert_eq!(options.transcript_path.as_deref(), Some(path), "{path}");
        // Every other resolved input is what the same run without a transcript would carry, so
        // nothing about the run is decided by the path.
        assert_eq!(options.config, run(&selecting("llm")).config, "{path}");
    }

    // Read out of the list the engine's parser accepted, so the scan cannot pick up a value: the
    // token appears only at an option position.
    assert_eq!(
        run(&["--transcript-path", "t.jsonl", "--policy", "llm"])
            .transcript_path
            .as_deref(),
        Some("t.jsonl")
    );
    // No value can ever be this token, which is what makes the positional scan exact: every value
    // option in either parser refuses a value beginning with `--`.
    assert!(parse(vec!["--export", "--transcript-path"]).is_err());
    assert!(parse(vec!["--policy", "llm", "--seed", "--transcript-path"]).is_err());
}

/// Rule 18.4.2 and stop condition 9 of `WO-MOK-025`: the observer diagnoses this option rather than
/// accepting it and ignoring it.
///
/// The observer forwards every argument it does not recognise to the engine's parser, so an option
/// that parser accepts reaches this program whether or not this program does anything with it. That
/// is what produced GitHub issue 40 for `--events-path`, and it is the failure this asserts is not
/// repeated: the transcript path is present exactly when the source that needs it was selected, and
/// absent otherwise, so there is no invocation in which it is accepted and dropped.
#[test]
fn the_transcript_option_is_never_accepted_and_ignored() {
    assert!(run(&selecting("llm")).transcript_path.is_some());

    for policy in ["baseline", "reference", "individual", "social"] {
        // Refused, not carried and not dropped.
        assert!(
            parse(vec!["--policy", policy, "--transcript-path", "t.jsonl"]).is_err(),
            "{policy}"
        );
        assert_eq!(run(&selecting(policy)).transcript_path, None, "{policy}");
    }
    assert_eq!(run(&[]).transcript_path, None);
}

/// Rules 13.2, 19.2 and 20.3: this program refuses the fifth source with no transcript, at start-up.
///
/// It refuses because the engine's shared parser refuses — the observer makes no second copy of the
/// rule — and the refusal reaches the operator as an ordinary configuration error, before the terminal
/// is entered. Rule 20.3's other half is asserted as the absence of a fallback: no other source's name
/// appears in the message, because the run is refused rather than run some other way.
#[test]
fn the_replay_source_with_no_transcript_is_refused_at_start_up() {
    let refusal = parse(vec!["--policy", "llm"])
        .expect_err("a replay with no transcript is refused before the terminal is entered");
    assert!(refusal.contains("--transcript-path"), "{refusal}");
    assert!(refusal.contains("llm"), "{refusal}");
    for other in ["baseline", "reference", "individual", "social"] {
        assert!(
            !refusal.contains(other),
            "{refusal} offers {other} as a substitute"
        );
    }

    // The engine's own words, so the two hosts refuse identically.
    assert_eq!(
        Some(refusal),
        mokiterions::cli::parse(vec!["--policy", "llm"]).err()
    );
}

/// A usage text with its line wrapping removed and lowercased.
///
/// A sentence in this text is wrapped at 79 columns, so an assertion written against the text as
/// printed is partly an assertion about where the sentence happened to break — and rewrapping a
/// paragraph would fail it while changing nothing an operator reads differently.
fn unwrapped(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// This program's usage text says what its share of the fifth source is, and that it is required.
///
/// The prose is the observer's own — `SPEC-MOK-003` rule 5 requires identical parsing and not
/// identical prose — but what it has to keep saying is fixed by rule 20.2: this host replays and never
/// asks a model, and the reason is the frame budget rather than a preference. An operator who reads
/// only this program's help must not come away expecting it to run a live exchange.
#[test]
fn the_usage_text_states_that_this_host_only_replays() {
    let text = unwrapped(USAGE);
    assert!(text.contains("only replays --policy llm"), "{USAGE}");
    assert!(text.contains("never asks a model"), "{USAGE}");
    assert!(text.contains("required"), "{USAGE}");

    // The two exit-status clauses the option added, and both are the parser's behaviour.
    assert!(text.contains("do not go together"), "{USAGE}");
    assert!(text.contains("transcript could not be read"), "{USAGE}");
    assert!(parse(vec!["--policy", "llm"]).is_err());
    assert!(parse(vec!["--transcript-path", "t.jsonl"]).is_err());
}

/// All three targets spell the option identically.
///
/// It is spelled in the engine's parser, which validates it and keeps nothing; in the engine's binary
/// target, which opens it; and in this program, which also opens it. Rule 18.4 is what makes three
/// copies necessary, and this is what keeps them one option: a rename in any single place leaves a
/// program that accepts a transcript and replays nothing.
#[test]
fn every_target_spells_the_transcript_option_the_same_way() {
    let source = include_str!("../src/options.rs");
    let declaration = source
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("const TRANSCRIPT_PATH_OPTION"))
        .expect("this program declares the option whose value it reads");
    let spelling = declaration
        .split('"')
        .nth(1)
        .expect("the declaration states a string literal");

    // Accepted by the engine's parser, and by this program's, which is the same parser.
    assert!(
        parse(vec!["--policy", "llm", spelling, "t.jsonl"]).is_ok(),
        "{spelling} is not the option the parser accepts"
    );
    assert!(
        entry(USAGE, spelling).lines().count() > 1,
        "{spelling} has no entry in this program's help"
    );
    assert!(
        mokiterions::cli::USAGE.contains(&format!("  {spelling} ")),
        "{spelling} has no entry in the engine's help"
    );

    // Spelled once outside its own declaration in each of the two targets that read a value with it.
    for (label, target) in [
        ("this program's parser", source),
        (
            "this program's binary target",
            include_str!("../src/main.rs"),
        ),
    ] {
        let uses = target
            .lines()
            .map(str::trim)
            .filter(|line| !line.starts_with("//") && line.contains(&format!("\"{spelling}\"")))
            .count();
        assert!(uses <= 1, "{label} spells {spelling} {uses} times");
    }
}
