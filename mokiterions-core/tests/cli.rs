//! Public tier, `SPEC-MOK-002` rule 8: argument parsing.
//!
//! Relocated from `src/cli.rs` under `WO-MOK-003`. Every test here needs only
//! `cli::parse`, `cli::Command`, `simulation::Config`, `simulation::Policy`, and
//! `simulation::Density`, all of which were already public. Assertions are verbatim; only
//! the path by which the code is reached changed, as rule 12 requires.
//!
//! The help-content tests added under `WO-MOK-004` share that subject: what the usage text
//! states about an option's default is only worth stating if it is the value this parser
//! applies, so the two are held equal here. None of them compares the text to a literal
//! declared in this file, which would move the drift one level up rather than remove it.
//!
//! `WO-MOK-026` adds `simulation::UnitPrices` to the imports above, which is the first thing
//! this file needs that was not already public before it needed it. It is here because
//! `--prices` is the one option in this parser whose value is **retained**: the tests for the
//! three paths assert that a different value produces the same configuration, and the test for
//! this one has to assert the opposite, which cannot be done without reading the four fields.
//!
//! `WO-MOK-019` adds one option, and most of what has to be true of it is already asserted by
//! the tests above, which read the parser's own match arms rather than a list: an option added
//! to the parser and left out of the help fails there without being named. What the three tests
//! at the end of this file add is what those cannot reach — that the parser keeps no path
//! (`SPEC-MOK-006` rule 1.2), and that the binary target, which is the one place a path is
//! resolved, spells the option the same way the parser does.

use mokiterions::cli::{Command, USAGE, parse};
use mokiterions::simulation::{Config, Density, Policy, UnitPrices};

#[test]
fn defaults_are_stable() {
    assert_eq!(
        parse(Vec::<String>::new()).unwrap(),
        Command::Run(Config {
            seed: 0,
            tick_limit: 100,
            policy: Policy::Reference,
            density: Density::DEFAULT,
            trace_actions: false,
            spend_ceiling: None,
            prices: None,
        })
    );
}

#[test]
fn options_work_in_any_order() {
    assert_eq!(
        parse([
            "--trace-actions",
            "--ticks",
            "7",
            "--density",
            "1.5",
            "--policy",
            "baseline",
            "--seed",
            "42"
        ])
        .unwrap(),
        Command::Run(Config {
            seed: 42,
            tick_limit: 7,
            policy: Policy::Baseline,
            density: Density::parse("1.50").unwrap(),
            trace_actions: true,
            spend_ceiling: None,
            prices: None,
        })
    );
}

#[test]
fn both_policies_are_selectable_and_reference_is_the_default() {
    let baseline = parse(["--policy", "baseline"]).unwrap();
    let reference = parse(["--policy", "reference"]).unwrap();
    let default = parse(Vec::<String>::new()).unwrap();

    assert_eq!(baseline, Command::Run(config_with(Policy::Baseline)));
    assert_eq!(reference, Command::Run(config_with(Policy::Reference)));
    assert_eq!(default, reference);
}

/// `REQ-MOK-033`: the third value is selectable, and adding it moved neither the default nor
/// the two values that were already there.
///
/// A named sibling rather than an edit to the test above, whose name states what it asserted
/// when there were two policies. `WO-MOK-010` does not rename inherited tests: a rename cannot
/// be told from a removal in the census `VER-MOK-010` requires, and the assertions above are
/// still true and still worth keeping.
#[test]
fn the_trait_aware_policy_is_selectable_and_does_not_become_the_default() {
    assert_eq!(
        parse(["--policy", "individual"]).unwrap(),
        Command::Run(config_with(Policy::Individual))
    );

    // The owner kept `reference` as the default under `WO-MOK-010`, so the third value is
    // opt-in and an unflagged invocation still selects the source it selected before.
    assert_eq!(
        parse(Vec::<String>::new()).unwrap(),
        Command::Run(config_with(Policy::Reference))
    );
    assert_ne!(
        parse(Vec::<String>::new()).unwrap(),
        Command::Run(config_with(Policy::Individual))
    );

    // The name is exact: neither an abbreviation nor a near miss selects it.
    assert!(parse(["--policy", "individuals"]).is_err());
    assert!(parse(["--policy", "Individual"]).is_err());
    assert!(parse(["--policy", "indiv"]).is_err());
    assert!(parse(["--policy", "individual", "--policy", "reference"]).is_err());
}

/// `REQ-MOK-057`: the fourth value is selectable, and adding it moved neither the default nor
/// the three values that were already there.
///
/// A named sibling again, for the reason recorded above: `WO-MOK-016` renames no inherited test.
/// The default matters more here than it did under `WO-MOK-010`. `SPEC-MOK-001` records that
/// `social` is not proposed as the default *because* the survivor floor `REQ-MOK-058` states for
/// it sits three below `REQ-MOK-014`'s, so an invocation that silently selected it would ship a
/// less habitable world than the one the default promises.
#[test]
fn the_social_policy_is_selectable_and_does_not_become_the_default() {
    assert_eq!(
        parse(["--policy", "social"]).unwrap(),
        Command::Run(config_with(Policy::Social))
    );

    assert_eq!(
        parse(Vec::<String>::new()).unwrap(),
        Command::Run(config_with(Policy::Reference))
    );
    assert_ne!(
        parse(Vec::<String>::new()).unwrap(),
        Command::Run(config_with(Policy::Social))
    );

    // The name is exact: neither an abbreviation nor a near miss selects it.
    assert!(parse(["--policy", "socials"]).is_err());
    assert!(parse(["--policy", "Social"]).is_err());
    assert!(parse(["--policy", "soc"]).is_err());
    assert!(parse(["--policy", "society"]).is_err());
    assert!(parse(["--policy", "social", "--policy", "reference"]).is_err());
}

/// `REQ-MOK-057`: the diagnostic for an unknown policy names every value the parser accepts.
///
/// The message is what an operator reads after mistyping, and a message that omits a value is a
/// value nobody finds. It is asserted here rather than only through the help text because the two
/// strings are separate and nothing but this keeps them agreeing.
#[test]
fn the_unknown_policy_diagnostic_names_every_accepted_value() {
    let message = parse(["--policy", "random"])
        .expect_err("an unknown policy is a configuration error")
        .to_string();

    for name in ["baseline", "reference", "individual", "social", "llm"] {
        assert!(message.contains(name), "{message} omits {name}");
        assert!(parse(selecting(name)).is_ok(), "{name} is not accepted");
    }
}

/// The arguments that select a source, together with whatever else that source requires.
///
/// Added 2026-08-24 under `WO-MOK-025`. `--policy llm` alone stopped parsing when
/// `--transcript-path` arrived: `SPEC-MOK-007` rule 13.2 makes a replay with no transcript a usage
/// error, and rule 19.2 names it as one. The assertions that use this are about whether a value is
/// *accepted as a source*, so they supply what the source needs and leave the combination rules to
/// the tests written for them. The other four sources need nothing beyond themselves.
///
/// The path is never opened by anything this file calls — `SPEC-MOK-006` rule 1.2 keeps every path
/// out of the library target — so it names no file that has to exist.
fn selecting(policy: &str) -> Vec<String> {
    let mut args = vec!["--policy".to_string(), policy.to_string()];
    if policy == "llm" {
        args.push("--transcript-path".to_string());
        args.push("transcript.jsonl".to_string());
    }
    args
}

fn config_with(policy: Policy) -> Config {
    Config {
        seed: 0,
        tick_limit: 100,
        policy,
        density: Density::DEFAULT,
        trace_actions: false,
        spend_ceiling: None,
        prices: None,
    }
}

#[test]
fn duplicates_and_missing_values_are_rejected() {
    assert!(parse(["--seed", "1", "--seed", "2"]).is_err());
    assert!(parse(["--ticks", "--trace-actions"]).is_err());
    assert!(parse(["--trace-actions", "--trace-actions"]).is_err());
    assert!(parse(["--unknown"]).is_err());
    assert!(parse(["--policy", "baseline", "--policy", "reference"]).is_err());
    assert!(parse(["--policy", "random"]).is_err());
    assert!(parse(["--policy"]).is_err());
    assert!(parse(["--density", "0.75", "--density", "1.50"]).is_err());
    assert!(parse(["--density"]).is_err());
}

#[test]
fn density_is_accepted_in_the_specified_forms_and_rejected_otherwise() {
    // The default is the declared default density, and it is written as a percentage.
    assert_eq!(
        parse(Vec::<String>::new()).unwrap(),
        Command::Run(config_with(Policy::Reference))
    );

    // Trailing zeros and omitted decimals denote the same density.
    assert_eq!(
        parse(["--density", "1.5"]).unwrap(),
        parse(["--density", "1.50"]).unwrap()
    );
    assert_eq!(
        parse(["--density", "2"]).unwrap(),
        parse(["--density", "2.00"]).unwrap()
    );

    // A density resolving to no resources is invalid configuration, not a valid empty
    // world, because an emptied territory can never regenerate.
    assert!(parse(["--density", "0.01"]).is_err());
    assert!(parse(["--density", "0"]).is_err());

    assert!(parse(["--density", "0.751"]).is_err());
    assert!(parse(["--density", "abc"]).is_err());
    assert!(parse(["--density", "1.2.3"]).is_err());
    assert!(parse(["--density", "-1"]).is_err());
    assert!(parse(["--density", "101"]).is_err());
}

// Help content, `REQ-MOK-018`. Three oracles are used below and none of them is the
// constant itself: the specification, transcribed where a list of options or defaults is
// needed; the parser's own match arms, read out of the source rather than restated; and the
// configuration the parser actually applies. An assertion written only against `USAGE`
// would hold for every possible value of `USAGE`, including an empty one.

/// The lines of the options block: the indented text that follows the `Options:` heading,
/// blank separator lines removed.
///
/// The block closes at the first line that carries text in column one, which is where
/// `SPEC-MOK-001`'s *Help output* section puts the statement on order and repetition. Under
/// `WO-MOK-024` the block gained a blank line between entries, so a blank line no longer
/// closes it; nothing else about how this reads the text changed, and no assertion below was
/// touched.
fn options_block() -> Vec<&'static str> {
    let mut lines = USAGE.lines().skip_while(|line| *line != "Options:");
    assert_eq!(lines.next(), Some("Options:"), "no options block in USAGE");
    lines
        .take_while(|line| line.trim().is_empty() || line.starts_with(' '))
        .filter(|line| !line.trim().is_empty())
        .collect()
}

/// The explanatory prose that follows the options block.
fn prose() -> String {
    let block = options_block();
    let last = block.last().expect("the options block is not empty");
    let end = USAGE.find(last).expect("a block line appears in USAGE") + last.len();
    USAGE[end..].to_string()
}

/// Whether a line of the options block opens a new entry rather than continuing one.
fn names_an_option(line: &str) -> bool {
    line.trim_start().starts_with("--")
}

/// One entry in full, continuation lines included. An entry runs from the line naming its
/// option to the line before the next option is named.
fn entry(option: &str) -> String {
    let block = options_block();
    let start = block
        .iter()
        .position(|line| match line.trim_start().strip_prefix(option) {
            Some(rest) => rest.is_empty() || rest.starts_with(' '),
            None => false,
        })
        .unwrap_or_else(|| panic!("the options block has no entry for {option}"));

    let mut text = block[start].trim().to_string();
    for line in &block[start + 1..] {
        if names_an_option(line) {
            break;
        }
        text.push(' ');
        text.push_str(line.trim());
    }
    text
}

/// An entry with its option and value placeholder removed, leaving the prose an operator
/// reads to learn what the option does.
fn description(option: &str) -> String {
    let entry = entry(option);
    let rest = entry
        .strip_prefix(option)
        .expect("an entry begins with its option")
        .trim_start();
    let rest = match rest.strip_prefix('<') {
        Some(after) => after.split_once('>').expect("a placeholder is closed").1,
        None => rest,
    };
    rest.trim().to_string()
}

/// The default value an entry states, or `None` where it states no value. The token is
/// taken from the printed text, so the test never carries its own copy of the value.
fn documented_default(option: &str) -> Option<String> {
    let stated = entry(option).split_once("Default: ")?.1.to_string();
    let token = stated.split_whitespace().next()?;
    Some(token.trim_end_matches('.').to_string())
}

/// The options the block describes, in the order it describes them.
fn documented_options() -> Vec<String> {
    options_block()
        .into_iter()
        .filter(|line| names_an_option(line))
        .map(|line| {
            line.split_whitespace()
                .next()
                .expect("an entry names an option")
                .to_string()
        })
        .collect()
}

/// The options the synopsis names, in the order it names them.
fn options_named_before_the_block() -> Vec<String> {
    USAGE
        .lines()
        .take_while(|line| *line != "Options:")
        .flat_map(str::split_whitespace)
        .map(|token| token.trim_start_matches('[').trim_end_matches(']'))
        .filter(|token| token.starts_with("--"))
        .map(str::to_string)
        .collect()
}

/// The options the parser accepts, read out of its own match arms. A hand-written list here
/// would pass unchanged after an option is added to the parser and left out of the help,
/// which is the failure this is written to catch.
fn options_the_parser_accepts() -> Vec<String> {
    include_str!("../src/cli.rs")
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("\"--") && line.contains("\" => {"))
        .map(|line| {
            line[1..]
                .split('"')
                .next()
                .expect("a match arm names an option")
                .to_string()
        })
        .collect()
}

/// The configuration the program applies for the given arguments.
fn run_config<I, S>(args: I) -> Config
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    match parse(args).unwrap() {
        Command::Run(config) => config,
        Command::Help => panic!("expected a run configuration"),
    }
}

#[test]
fn every_option_the_synopsis_names_has_an_options_entry() {
    let named = options_named_before_the_block();
    assert!(!named.is_empty(), "the synopsis names no option");
    assert_eq!(
        documented_options(),
        named,
        "the synopsis and the options block must name the same options in the same order"
    );

    for option in &named {
        assert!(
            description(option).split_whitespace().count() >= 3,
            "{option} has no description of its effect: {}",
            entry(option)
        );
    }
}

#[test]
fn the_documented_options_are_exactly_the_options_the_parser_accepts() {
    let mut documented = documented_options();
    let mut accepted = options_the_parser_accepts();
    documented.sort();
    accepted.sort();
    assert_eq!(
        documented, accepted,
        "every option the parser accepts is described, and nothing else is"
    );

    // The set above is read from the source text, so it is confirmed against the running
    // parser: a described option must not be rejected as unknown.
    for option in &documented {
        let outcome = format!("{:?}", parse([option.as_str()]));
        assert!(
            !outcome.contains("unknown option"),
            "the help describes {option}, which the parser does not accept: {outcome}"
        );
    }
}

#[test]
fn each_documented_default_parses_to_the_applied_default() {
    // The four options that take a value and declare a default, transcribed from
    // `SPEC-MOK-001`'s *Help output* table rather than derived from the text, so that a
    // default dropped from the help fails here instead of being skipped.
    let applied = run_config(Vec::<String>::new());

    for option in ["--seed", "--ticks", "--policy", "--density"] {
        let stated = documented_default(option)
            .unwrap_or_else(|| panic!("{option} declares a default and must state it"));

        assert_eq!(
            run_config([option, stated.as_str()]),
            applied,
            "the help states {stated} as the default for {option}, which is not the value \
             the program applies when {option} is omitted"
        );
    }
}

#[test]
fn the_entries_state_the_constraints_that_decide_validity() {
    // Each stated constraint is paired with a value the parser rejects, so the text is
    // required to describe a rule the program enforces rather than a plausible one.
    let ticks = description("--ticks").to_lowercase();
    assert!(ticks.contains("greater than zero"), "{ticks}");
    assert!(parse(["--ticks", "0"]).is_err());

    // The value set is stated in the placeholder, so the whole entry is read here. Every value
    // the parser accepts is named and every value it names is accepted, so the help can neither
    // hide a source nor advertise one that does not exist.
    let policy = entry("--policy");
    assert!(policy.contains("baseline"), "{policy}");
    assert!(policy.contains("reference"), "{policy}");
    assert!(policy.contains("individual"), "{policy}");
    assert!(policy.contains("social"), "{policy}");
    assert!(policy.contains("llm"), "{policy}");
    assert!(parse(["--policy", "baseline"]).is_ok());
    assert!(parse(["--policy", "reference"]).is_ok());
    assert!(parse(["--policy", "individual"]).is_ok());
    assert!(parse(["--policy", "social"]).is_ok());
    assert!(parse(selecting("llm")).is_ok());
    assert!(parse(["--policy", "random"]).is_err());

    // `SPEC-MOK-007` rule 18.2's three claims about the fifth value, each asserted against the
    // entry rather than trusted: it reaches a model through a connector program the operator
    // supplies, it is not deterministic in itself, and it replays deterministically from a
    // transcript. The wording is free to change; what it has to keep saying is not.
    let policy = policy.to_lowercase();
    assert!(policy.contains("connector"), "{policy}");
    assert!(policy.contains("transcript"), "{policy}");
    assert!(policy.contains("replays"), "{policy}");
    assert!(policy.contains("not fixed by the seed"), "{policy}");

    // Rule 18.3: the sentence that said this of all four became false of five, and the entry now
    // says of which values determinism holds. An entry that still claimed it of every value would
    // pass every assertion above.
    assert!(
        !policy.contains("all five are"),
        "the entry claims determinism of the fifth value: {policy}"
    );
    assert!(
        policy.contains("all four are"),
        "the entry no longer states which values are deterministic: {policy}"
    );

    let density = description("--density").to_lowercase();
    assert!(density.contains("two"), "{density}");
    assert!(density.contains("decimal"), "{density}");
    assert!(parse(["--density", "0.751"]).is_err());
}

#[test]
fn the_flags_state_their_effect_and_no_default_value() {
    let trace = entry("--trace-actions");
    let effect = description("--trace-actions").to_lowercase();
    assert!(effect.contains("trace"), "{effect}");
    assert!(effect.contains("off unless"), "{effect}");

    // Printing a default *value* for a flag would invite a value the parser rejects, so
    // the entry states the behaviour on omission instead, and that is the behaviour applied.
    assert!(!trace.contains("Default:"), "{trace}");
    assert!(!run_config(Vec::<String>::new()).trace_actions);
    assert!(parse(["--trace-actions", "false"]).is_err());

    let help = entry("--help");
    assert!(!help.contains("Default:"), "{help}");
    assert_eq!(parse(["--help"]).unwrap(), Command::Help);
}

#[test]
fn each_declared_default_is_stated_once() {
    // Four of the five declared defaults are values, each stated in its own entry and
    // nowhere else in the text.
    assert_eq!(USAGE.matches("Default: ").count(), 4, "{USAGE}");
    for option in ["--seed", "--ticks", "--policy", "--density"] {
        let entry = entry(option);
        assert_eq!(entry.matches("Default: ").count(), 1, "{entry}");
    }

    // The fifth is the flag's off-state. The explanatory prose restates neither a default
    // nor the density constraint: a second copy of a fact is the drift this requirement
    // exists to prevent, and prose is where the old copy lived.
    let trace = entry("--trace-actions");
    assert_eq!(trace.matches("Off unless given").count(), 1, "{trace}");

    let prose = prose().to_lowercase();
    assert!(!prose.contains("default"), "{prose}");
    assert!(!prose.contains("decimal"), "{prose}");
}

#[test]
fn the_help_text_states_order_and_repetition() {
    let text = USAGE.to_lowercase();
    assert!(text.contains("any order"), "{USAGE}");
    assert!(text.contains("at most once"), "{USAGE}");

    // Both claims are the parser's behaviour.
    assert_eq!(
        run_config(["--seed", "1", "--ticks", "2"]),
        run_config(["--ticks", "2", "--seed", "1"])
    );
    assert!(parse(["--seed", "1", "--seed", "1"]).is_err());
}

/// `SPEC-MOK-006` rule 1.2: the parser validates the sink option and retains no path.
///
/// The configuration it produces is the one an unflagged invocation produces, whatever value the
/// option carried, which is the observable form of "the library resolves no path". Everything
/// the parser does reject about the value it rejects for a stated reason: a missing value, a
/// value that is really the next option, and the two spellings that conventionally denote a
/// standard stream. Every other property of the value belongs to the platform, so a path that
/// cannot be opened parses and then fails at runtime under rule 13.2 rather than here.
#[test]
fn the_sink_option_is_validated_and_its_value_is_not_retained() {
    let unflagged = run_config(Vec::<String>::new());
    assert_eq!(run_config(["--events-path", "records.jsonl"]), unflagged);
    assert_eq!(
        run_config(["--events-path", "somewhere-else.jsonl"]),
        unflagged
    );
    assert_eq!(
        run_config(["--events-path", "records.jsonl", "--seed", "9"]),
        run_config(["--seed", "9"])
    );

    assert!(parse(["--events-path"]).is_err());
    assert!(parse(["--events-path", "--seed"]).is_err());
    assert!(parse(["--events-path", ""]).is_err());
    assert!(parse(["--events-path", "-"]).is_err());
    assert!(parse(["--events-path", "a.jsonl", "--events-path", "b.jsonl"]).is_err());

    // Not this parser's business, and accepted for that reason.
    assert!(parse(["--events-path", "no/such/directory/records.jsonl"]).is_ok());
    assert!(parse(["--events-path", "-leading-dash.jsonl"]).is_ok());
    assert!(parse(["--events-path", "records.jsonl"]).is_ok());
}

/// The sink option's entry describes what it writes and what it replaces, and states no default.
///
/// A default value here would be a path printed in the help, and an operator who read it would
/// have been told where the program writes when in fact it writes nowhere unless asked.
#[test]
fn the_sink_option_states_what_it_writes_and_what_it_replaces() {
    let entry = entry("--events-path");
    let effect = description("--events-path").to_lowercase();

    assert!(effect.contains("record stream"), "{effect}");
    assert!(effect.contains("replacing"), "{effect}");
    assert!(effect.contains("unless given"), "{effect}");

    assert!(!entry.contains("Default:"), "{entry}");
    assert_eq!(documented_default("--events-path"), None, "{entry}");
}

/// The parser and the binary target spell the option identically.
///
/// The spelling exists twice because the two halves of the option are split between the two
/// targets: the parser validates it and keeps nothing, and the binary reads the value it will
/// open, since `SPEC-MOK-006` rule 1.2 keeps every path out of the library. A rename in one
/// place would leave a program that accepts the option and then records nothing — the option
/// would parse, and the destination would never be found. This holds the two equal.
#[test]
fn the_binary_and_the_parser_spell_the_sink_option_the_same_way() {
    let host = include_str!("../src/main.rs");
    let declaration = host
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("const EVENTS_PATH_OPTION"))
        .expect("the binary target declares the option whose value it reads");
    let spelling = declaration
        .split('"')
        .nth(1)
        .expect("the declaration states a string literal")
        .to_string();

    assert!(
        options_the_parser_accepts().contains(&spelling),
        "the binary reads {spelling}, which the parser does not accept"
    );
    assert!(
        documented_options().contains(&spelling),
        "the binary reads {spelling}, which the help does not describe"
    );

    // Spelled once in the host, through that constant, so this test has one thing to hold.
    let uses = host
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with("//") && line.contains(&format!("\"{spelling}\"")))
        .count();
    assert_eq!(
        uses, 1,
        "the host must spell {spelling} only in its constant"
    );
}

/// `SPEC-MOK-007` rule 18.4: the transcript option is validated exactly as the sink option is, and
/// its value is retained no more than the sink's is.
///
/// The rule names `--events-path` as the precedent this option follows, so the assertions here are
/// that test's assertions over this option: the configuration produced is the one the same
/// invocation with a different value produces, whatever value it carried; a missing value, a value
/// that is really the next option, the empty string and the single `-` are refused; and every other
/// property of the value belongs to the platform, so a path that cannot be opened parses here and
/// fails at runtime.
///
/// Every invocation below selects `llm`, because rule 18.4.3 refuses the option under any other
/// source. That is asserted separately, immediately after this.
#[test]
fn the_transcript_option_is_validated_and_its_value_is_not_retained() {
    let selected = run_config(selecting("llm"));
    assert_eq!(
        run_config(["--policy", "llm", "--transcript-path", "elsewhere.jsonl"]),
        selected
    );
    assert_eq!(
        run_config([
            "--policy",
            "llm",
            "--transcript-path",
            "elsewhere.jsonl",
            "--seed",
            "9",
        ]),
        run_config([
            "--policy",
            "llm",
            "--transcript-path",
            "a.jsonl",
            "--seed",
            "9",
        ])
    );
    // The one field a transcript could plausibly have moved is the source itself, and it is the
    // source the operator named rather than one the transcript implied.
    assert_eq!(selected.policy, Policy::Llm);

    assert!(parse(["--policy", "llm", "--transcript-path"]).is_err());
    assert!(parse(["--policy", "llm", "--transcript-path", "--seed"]).is_err());
    assert!(parse(["--policy", "llm", "--transcript-path", ""]).is_err());
    assert!(parse(["--policy", "llm", "--transcript-path", "-"]).is_err());
    assert!(
        parse([
            "--policy",
            "llm",
            "--transcript-path",
            "a.jsonl",
            "--transcript-path",
            "b.jsonl",
        ])
        .is_err()
    );

    // Not this parser's business, and accepted for that reason.
    assert!(
        parse([
            "--policy",
            "llm",
            "--transcript-path",
            "no/such/directory/t.jsonl",
        ])
        .is_ok()
    );
    assert!(
        parse([
            "--policy",
            "llm",
            "--transcript-path",
            "-leading-dash.jsonl",
        ])
        .is_ok()
    );

    // Order is the parser's own promise and this option keeps it, combination check included.
    assert_eq!(
        run_config(["--transcript-path", "a.jsonl", "--policy", "llm"]),
        selected
    );
}

/// Rule 18.4.3: a transcript under a source that obtains its own decisions is refused, not accepted
/// and ignored.
///
/// The default source is enough to trigger it. An operator who names a transcript and no source has
/// asked for a `reference` run with a transcript, which is the misunderstanding the rule exists to
/// report rather than to absorb, and it is the case a check written against an explicit `--policy`
/// would miss.
#[test]
fn the_transcript_option_is_refused_under_every_source_that_decides_for_itself() {
    for policy in ["baseline", "reference", "individual", "social"] {
        let refusal = parse(["--policy", policy, "--transcript-path", "t.jsonl"])
            .expect_err("a transcript under a self-deciding source is a usage error");
        assert!(refusal.contains("--transcript-path"), "{refusal}");
        assert!(refusal.contains(policy), "{refusal} does not name {policy}");
    }

    let refusal = parse(["--transcript-path", "t.jsonl"])
        .expect_err("a transcript with no source is a transcript for the default source");
    assert!(refusal.contains("reference"), "{refusal}");
}

/// Rules 13.2, 19.2 and 20.3: the replay source with no transcript is refused, and the refusal names
/// what is missing.
///
/// It is made by the parser both hosts share, which is what makes rule 20.3's start-up refusal in the
/// observer a property of the parse rather than a second implementation of the same rule. The refusal
/// is a usage error and not a fallback: rule 13.1 makes replay the default and rule 20.3 forbids
/// reaching for another source, so there is nothing for the run to do instead.
#[test]
fn the_replay_source_with_no_transcript_is_refused_and_the_refusal_names_the_option() {
    let refusal =
        parse(["--policy", "llm"]).expect_err("a replay with no transcript is a usage error");
    assert!(refusal.contains("--transcript-path"), "{refusal}");
    assert!(refusal.contains("llm"), "{refusal}");

    // Not a fallback to any other source, asserted as the absence of every other source's name.
    for other in ["baseline", "reference", "individual", "social"] {
        assert!(
            !refusal.contains(other),
            "{refusal} offers {other} as a substitute"
        );
    }
}

/// Neither combination refusal can make the usage text unobtainable.
///
/// Both are decided after `--help`, which is the opposite of every rejection inside the parse loop:
/// those reject a *value* and beat `--help`, and these two are about a combination. An operator who
/// asks what the combinations are must be answered with the text rather than with a complaint about
/// the one they typed — and that text is the only place the combination rules are written down.
#[test]
fn help_survives_both_combination_refusals() {
    assert_eq!(parse(["--help", "--policy", "llm"]).unwrap(), Command::Help);
    assert_eq!(parse(["--policy", "llm", "--help"]).unwrap(), Command::Help);
    assert_eq!(
        parse(["--help", "--transcript-path", "t.jsonl"]).unwrap(),
        Command::Help
    );
    assert_eq!(
        parse([
            "--transcript-path",
            "t.jsonl",
            "--policy",
            "social",
            "--help"
        ])
        .unwrap(),
        Command::Help
    );

    // A rejected *value* still beats `--help`, unchanged by the two checks added beside it.
    assert!(parse(["--help", "--transcript-path", ""]).is_err());
    assert!(parse(["--help", "--policy", "oracle"]).is_err());
}

/// The transcript option's entry states what it replays, when it is required and when it is refused,
/// and states no default.
///
/// A default value here would be a path printed in the help, and the option has no default: it is
/// required under one source and refused under the other four, which is what the entry has to say.
#[test]
fn the_transcript_option_states_when_it_is_required_and_when_it_is_refused() {
    let entry = entry("--transcript-path");
    let effect = description("--transcript-path").to_lowercase();

    assert!(effect.contains("transcript"), "{effect}");
    // Amended under `WO-MOK-026`. This assertion read `required with --policy llm` until
    // `SPEC-MOK-007` rule 18.4.4 gave the source a second way to obtain decisions: with a
    // connector it records rather than replays, and `--transcript-output` is what it writes
    // to. So this option is no longer *required* by the source, and a help text still saying
    // so would be stating something untrue — which is the defect rule 18.3 exists to correct,
    // met here in a test rather than in the text.
    assert!(effect.contains("--policy llm replays"), "{effect}");
    assert!(effect.contains("--live"), "{effect}");
    assert!(effect.contains("may not be given together"), "{effect}");
    assert!(effect.contains("refused with any other policy"), "{effect}");
    // Rule 12.6's byte-identity and rule 12.3's refusal, which together are what decides whether an
    // operator can trust a replayed run at all.
    assert!(effect.contains("exactly"), "{effect}");
    assert!(effect.contains("error rather than guessing"), "{effect}");

    assert!(!entry.contains("Default:"), "{entry}");
    assert_eq!(documented_default("--transcript-path"), None, "{entry}");
}

/// The exit-status sentence states the two outcomes this option added.
///
/// A combination the parser refuses exits `2`, like every other usage error, and a transcript that
/// cannot be read exits `1`, like every other runtime failure — and neither had a clause before this
/// option existed. The text is the operator's only statement of what a code means.
#[test]
fn the_exit_status_states_the_refused_combination_and_the_unreadable_transcript() {
    // Wrapping removed first: these sentences are wrapped at 79 columns, and an assertion written
    // against the text as printed would also be an assertion about where a sentence broke.
    let status = USAGE
        .split_once("Exit status: ")
        .expect("the help states the exit statuses")
        .1
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();

    assert!(status.contains("do not go together"), "{status}");
    assert!(status.contains("transcript could not be read"), "{status}");

    // The first claim is the parser's, and the parser is what refuses both combinations.
    assert!(parse(["--policy", "llm"]).is_err());
    assert!(parse(["--transcript-path", "t.jsonl"]).is_err());
}

/// The parser and the binary target spell the transcript option identically.
///
/// A named sibling of the sink option's test, against the same failure and for the same reason: the
/// spelling exists twice because the parser validates the option and keeps nothing while the binary
/// reads the value it will open. A rename in one place would leave a program that accepts a
/// transcript and then replays nothing — and unlike the sink, whose absence is silence, this one
/// reaches rule 20.8's refusal on the first tick, so the operator would be told the host built no
/// port for an option they did supply.
#[test]
fn the_binary_and_the_parser_spell_the_transcript_option_the_same_way() {
    let host = include_str!("../src/main.rs");
    let declaration = host
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("const TRANSCRIPT_PATH_OPTION"))
        .expect("the binary target declares the option whose value it reads");
    let spelling = declaration
        .split('"')
        .nth(1)
        .expect("the declaration states a string literal")
        .to_string();

    assert!(
        options_the_parser_accepts().contains(&spelling),
        "the binary reads {spelling}, which the parser does not accept"
    );
    assert!(
        documented_options().contains(&spelling),
        "the binary reads {spelling}, which the help does not describe"
    );

    let uses = host
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with("//") && line.contains(&format!("\"{spelling}\"")))
        .count();
    assert_eq!(
        uses, 1,
        "the host must spell {spelling} only in its constant"
    );
}

/// A complete live-run invocation, with the prices under test and everything else rule 13.1, rule
/// 19.6 and rule 14.6 require of one.
///
/// The engine's parser refuses `--live` without a connector, without an output transcript and
/// without a ceiling, each with its own message, so no shorter argument list reaches the prices at
/// all and an assertion about a price value can only be made of a whole invocation. None of the
/// three paths is opened by anything this file calls: `SPEC-MOK-006` rule 1.2 keeps every path out
/// of the library target, so they name no file that has to exist.
fn a_live_run(prices: &str) -> Vec<String> {
    [
        "--policy",
        "llm",
        "--live",
        "--connector-path",
        "connector",
        "--transcript-output",
        "out.jsonl",
        "--spend-ceiling",
        "2",
        "--prices",
        prices,
    ]
    .iter()
    .map(|argument| argument.to_string())
    .collect()
}

/// `SPEC-MOK-007` rule 14.3a: the parser validates the prices and **retains** the four values.
///
/// This is the one option in this parser whose test is the negation of the three above it. The
/// sink, the transcript and the two other paths are validated and discarded, and their tests assert
/// that a different value produces the same configuration; `VER-MOK-018` case `S6a` scopes that
/// discard rule to paths, and rule 14.3a says of this one that the run computes with the values, so
/// a configuration that forgot them would leave rule 14.2's arithmetic nothing to arithmetic with.
///
/// What the parser rejects it rejects for a stated reason, and every rejection below is a value an
/// operator could plausibly type: a decimal where cents are asked for, a price list with a field
/// missing or one too many, a thousands separator, a signed number, a space after a colon. The unit
/// is not checkable from the value — `125` is a legal number of cents and a legal number of dollars
/// — so the entry states the unit and `the_prices_entry_states_its_unit_and_its_order` holds it to
/// it. That is the whole of what protects an operator from a hundred-fold error here, and it is
/// stated rather than left implicit.
#[test]
fn the_prices_option_is_validated_and_its_four_values_are_retained() {
    let config = run_config(a_live_run("125:13:1000:0"));
    assert_eq!(
        config.prices,
        Some(UnitPrices {
            prompt: 125,
            cached: 13,
            output: 1000,
            reasoning: 0,
        })
    );

    // Retained, not discarded: a different price list is a different configuration. The three path
    // options' tests assert the equality this asserts the inequality of, and the difference between
    // them is rule 14.3a's word "retains".
    assert_ne!(
        run_config(a_live_run("126:13:1000:0")).prices,
        config.prices
    );
    assert_ne!(
        run_config(a_live_run("125:13:1000:1")).prices,
        config.prices
    );

    // Zero is a legal price and is the rule's own example, `SPEC-MOK-007` fixing the reasoning
    // level at `none` for this stage. A parser that rejected it would refuse the documented value.
    assert!(parse(a_live_run("0:0:0:0")).is_ok());

    // The widest value the type holds is accepted and the next one is refused for saying so, rather
    // than wrapping to a small price and costing the run a hundredfold less than it was told.
    assert!(parse(a_live_run(&format!("{}:0:0:0", u64::MAX))).is_ok());
    let refusal = parse(a_live_run("18446744073709551616:0:0:0"))
        .expect_err("a price this program cannot hold is a usage error");
    assert!(refusal.contains("18446744073709551616"), "{refusal}");

    // Rejected, each for a reason the message states.
    for value in [
        "",
        "125",
        "125:13",
        "125:13:1000",
        "125:13:1000:0:0",
        "125:13:1000:",
        ":13:1000:0",
        "1.25:13:1000:0",
        "125:13:1000:0.0",
        "-125:13:1000:0",
        "+125:13:1000:0",
        "1,250:13:1000:0",
        "125: 13:1000:0",
        "125;13;1000;0",
        "one:13:1000:0",
        "0x7d:13:1000:0",
    ] {
        let refusal = match parse(a_live_run(value)) {
            Err(refusal) => refusal,
            Ok(_) => panic!("--prices {value} was accepted"),
        };
        assert!(
            refusal.contains("--prices"),
            "the refusal of {value} does not name the option: {refusal}"
        );
    }

    // Validated exactly as every other value-taking option is: a missing value, a value that is
    // really the next option, and a repetition.
    let mut missing = a_live_run("125:13:1000:0");
    missing.pop();
    assert!(parse(missing).is_err());
    let mut next_option = a_live_run("125:13:1000:0");
    next_option.pop();
    next_option.push("--seed".to_string());
    assert!(parse(next_option).is_err());
    let mut twice = a_live_run("125:13:1000:0");
    twice.push("--prices".to_string());
    twice.push("125:13:1000:0".to_string());
    assert!(parse(twice).is_err());
}

/// The four prices are read in the stated order, and a transposition is a different run.
///
/// This is why `UnitPrices` is a named type rather than four bare integers or a `[u64; 4]`, and
/// `SPEC-MOK-002` rule 5's 2026-08-29 amendment records that as the ground for admitting it. Three
/// of the four values are plausible in each other's positions and they differ by nearly two orders
/// of magnitude, so a parser that filled the fields in the wrong order would pass every assertion
/// above — each value is a legal price — and would cost a run eighty times what it was told.
///
/// The four fields are read separately rather than compared against a second `parse`, which is the
/// reason they are public: comparing one `parse` against another proves nothing about order,
/// because a transposed parser transposes both sides of the comparison equally.
#[test]
fn the_prices_are_read_in_the_stated_order_and_a_transposition_is_a_different_run() {
    let prices = run_config(a_live_run("1:2:3:4"))
        .prices
        .expect("a live run retains its prices");
    assert_eq!(prices.prompt, 1);
    assert_eq!(prices.cached, 2);
    assert_eq!(prices.output, 3);
    assert_eq!(prices.reasoning, 4);

    // The order the help states, read out of the text rather than restated here, so that a
    // placeholder renamed in the entry and not in the parser fails rather than passing quietly.
    let placeholder = entry("--prices")
        .split_once('<')
        .expect("the entry states a placeholder")
        .1
        .split_once('>')
        .expect("a placeholder is closed")
        .0
        .to_string();
    assert_eq!(
        placeholder.split(':').collect::<Vec<_>>(),
        ["prompt", "cached", "output", "reasoning"],
        "the entry states an order the parser does not apply"
    );

    // Any transposition of two distinct prices is a different configuration.
    assert_ne!(
        run_config(a_live_run("13:125:1000:0")).prices,
        run_config(a_live_run("125:13:1000:0")).prices
    );
}

/// Rule 14.3: a live run with no declared prices is refused, and not run at a guess.
///
/// The refusal is the whole of what keeps rule 14.3's prohibition enforceable. With no prices a
/// live run cannot do rule 14.2's arithmetic, cannot make rule 14.6's check before an exchange and
/// cannot report rule 15.2's cost, so the only thing left for an implementation to do is compile
/// the prices in — which is the one thing that rule forbids. Refused before any tick, beside the
/// ceiling's own refusal, because a run that discovered it had no prices after its first exchange
/// has already spent money it cannot account for.
#[test]
fn a_live_run_with_no_prices_is_refused_before_any_tick() {
    let mut without = a_live_run("125:13:1000:0");
    without.truncate(without.len() - 2);
    let refusal = parse(without).expect_err("a live run with no prices is a usage error");
    assert!(refusal.contains("--prices"), "{refusal}");
    assert!(refusal.contains("--live"), "{refusal}");

    // Not a fallback to a compiled-in list, asserted as the absence of any number in the message: a
    // refusal that named a price would be offering one.
    assert!(
        !refusal.chars().any(char::is_numeric),
        "the refusal offers a price of its own: {refusal}"
    );

    // The type has no `Default` for the same reason, which is a compile-time fact and is asserted
    // here as the absence of the trait's own spelling from the declaration.
    let source = include_str!("../src/simulation.rs");
    let declaration = source
        .split_once("pub struct UnitPrices")
        .expect("the engine declares the price type")
        .0;
    let derives = declaration
        .lines()
        .next_back()
        .expect("the declaration is preceded by its attributes");
    assert!(
        !derives.contains("Default"),
        "UnitPrices derives Default, which rule 14.3 forbids: {derives}"
    );
}

/// `a_live_run` minus one option and the value that follows it, found by name.
///
/// By name rather than by index, so that the argument list may grow or be reordered without
/// silently changing which refusal each test below provokes. `cli::parse` checks the four options a
/// live run requires in the order connector, transcript, ceiling, prices, so removing exactly one
/// pair leaves the other three satisfied and the refusal under test is the only one reachable.
///
/// The prices are declared here and in one place, because they are the argument this list is most
/// likely to be made to grow: `SPEC-MOK-007` rule 14.3a fixes their count, and a fifth price would
/// move this literal and nothing else in the three tests below.
fn a_live_run_without(option: &str) -> Vec<String> {
    let mut without = a_live_run("125:13:1000:0");
    let at = without
        .iter()
        .position(|argument| argument == option)
        .unwrap_or_else(|| panic!("a complete live run declares {option}"));
    without.drain(at..at + 2);
    without
}

/// Rule 13.1's selection half: a live run with no connector is refused, and this engine reaches no
/// provider by itself.
///
/// `a_live_run` states in its own comment that the parser refuses `--live` without a connector, and
/// until this test nothing asserted it. Every live-run test in this file supplies all four options,
/// so the check could be deleted and the suite would stay green — and `VER-MOK-018` case `L20`
/// requires this refusal, which makes it a case with no assertion under it.
///
/// The refusal names no path of its own, asserted as the absence of either separator. Rule 13.1
/// puts the model behind a program the operator supplies, so a message offering a path would be
/// supplying the thing the rule reserves to the operator.
#[test]
fn a_live_run_with_no_connector_is_refused_before_any_tick() {
    let refusal = parse(a_live_run_without("--connector-path"))
        .expect_err("a live run with no connector is a usage error");
    assert!(refusal.contains("--connector-path"), "{refusal}");
    assert!(refusal.contains("--live"), "{refusal}");
    assert!(
        !refusal.contains('/') && !refusal.contains('\\'),
        "the refusal offers a connector path of its own: {refusal}"
    );
}

/// Rule 19.6: a live run with nowhere to record its exchanges is refused, not run unrecorded.
///
/// The failure this prevents is the one that cannot be undone. A run refused for any other reason
/// has spent nothing; a run that spent its exchanges and recorded none of them has produced cost
/// and no evidence, and there is no later step that recovers the records. That is why the check is
/// in the parser and not at the first write, and why the assertion is here rather than in a test
/// that inspects a transcript — there would be no transcript to inspect.
///
/// Named beside the connector's test and asserted the same way, including that the message supplies
/// no path.
#[test]
fn a_live_run_with_no_transcript_output_is_refused_before_any_tick() {
    let refusal = parse(a_live_run_without("--transcript-output"))
        .expect_err("a live run with no transcript output is a usage error");
    assert!(refusal.contains("--transcript-output"), "{refusal}");
    assert!(refusal.contains("--live"), "{refusal}");
    assert!(
        !refusal.contains('/') && !refusal.contains('\\'),
        "the refusal offers an output path of its own: {refusal}"
    );
}

/// Rules 14.6 and 19.2: a live run with no declared ceiling is refused before the first exchange.
///
/// This is the refusal whose absence from the suite cost the most. Rule 14.6 has the ceiling
/// checked before each exchange, and `REQ-MOK-071` makes the ceiling a stop rather than a report, so
/// a live run that reached its first exchange without one would have no limit to check and nothing
/// to stop it. The check that prevents it is one condition in this parser, and until this test any
/// change that dropped it would have left every test in both packages green.
///
/// Refused *before any tick* is the whole of it: a run that discovered it had no ceiling after its
/// first exchange has already spent money outside any limit, which is the failure rule 19.2 names as
/// a usage error rather than a run-time one.
///
/// The refusal proposes no amount, asserted as the absence of any numeral, for the same reason the
/// prices refusal proposes no price: the ceiling is the operator's declaration under rule 14.6, and
/// a message naming a figure would be offering one on the program's behalf.
#[test]
fn a_live_run_with_no_ceiling_is_refused_before_any_tick() {
    let refusal = parse(a_live_run_without("--spend-ceiling"))
        .expect_err("a live run with no ceiling is a usage error");
    assert!(refusal.contains("--spend-ceiling"), "{refusal}");
    assert!(refusal.contains("--live"), "{refusal}");
    assert!(
        !refusal.chars().any(char::is_numeric),
        "the refusal offers a ceiling of its own: {refusal}"
    );
}

/// Rule 18.4.3: prices under a source that obtains its own decisions are refused, not accepted and
/// ignored.
///
/// A named sibling of the transcript option's test, against the same failure. Prices for a
/// `social` run are a stated intent that cannot be honoured — that source spends nothing and has no
/// cost to compute — and the default source is enough to trigger it, a price list with no
/// `--policy` at all being a price list for `reference`.
#[test]
fn the_prices_option_is_refused_under_every_source_that_decides_for_itself() {
    for policy in ["baseline", "reference", "individual", "social"] {
        let refusal = parse(["--policy", policy, "--prices", "125:13:1000:0"])
            .expect_err("prices under a self-deciding source are a usage error");
        assert!(refusal.contains("--prices"), "{refusal}");
        assert!(refusal.contains(policy), "{refusal} does not name {policy}");
    }

    let refusal = parse(["--prices", "125:13:1000:0"])
        .expect_err("prices with no source are prices for the default source");
    assert!(refusal.contains("reference"), "{refusal}");
}

/// The prices entry states the unit, the order and where to find the values, and states no default.
///
/// The unit is the load-bearing claim and the one a value cannot carry: `125` is a legal number of
/// cents and an equally legal number of dollars, so an entry that omitted the unit would leave an
/// operator one hundred-fold error away from a run this repository pays for, and no assertion about
/// the parser could catch it. `SPEC-MOK-007` rule 14.2 fixes the unit as the US cent and rule 14.3a
/// fixes the denominator as a million tokens; both are asserted against the printed text here.
///
/// There is no default, and the entry must not print one: a price printed in the help is a
/// compiled-in price wherever an operator copies it from, which is what rule 14.3 forbids.
#[test]
fn the_prices_entry_states_its_unit_and_its_order() {
    let entry = entry("--prices");
    let effect = description("--prices").to_lowercase();

    // Rule 14.2's unit and rule 14.3a's denominator.
    assert!(effect.contains("us cents"), "{effect}");
    assert!(effect.contains("per million tokens"), "{effect}");
    assert!(effect.contains("whole numbers"), "{effect}");

    // Rule 14.3a's separator and order, and the one field an operator is most likely to misread.
    assert!(effect.contains("colons"), "{effect}");
    assert!(effect.contains("in that order"), "{effect}");
    assert!(effect.contains("from its cache"), "{effect}");

    // Rule 14.3's prohibition, stated to the operator rather than only enforced: the prices are the
    // provider's to change, so the program carries none and the run states the ones it used.
    assert!(effect.contains("no built-in list"), "{effect}");
    assert!(effect.contains("required with"), "{effect}");
    assert!(effect.contains("--live"), "{effect}");

    // The example is a value this parser accepts, so the text cannot document a form it refuses.
    let example = effect
        .split_whitespace()
        .find(|token| token.matches(':').count() == 3)
        .unwrap_or_else(|| panic!("the entry gives no example price list: {effect}"))
        .trim_end_matches(|character: char| !character.is_ascii_digit());
    assert!(
        parse(a_live_run(example)).is_ok(),
        "the entry gives {example} as an example, which the parser refuses"
    );

    assert!(!entry.contains("Default:"), "{entry}");
    assert_eq!(documented_default("--prices"), None, "{entry}");
}
