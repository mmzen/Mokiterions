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
//! `WO-MOK-018` adds one option, and most of what has to be true of it is already asserted by
//! the tests above, which read the parser's own match arms rather than a list: an option added
//! to the parser and left out of the help fails there without being named. What the three tests
//! at the end of this file add is what those cannot reach — that the parser keeps no path
//! (`SPEC-MOK-006` rule 1.2), and that the binary target, which is the one place a path is
//! resolved, spells the option the same way the parser does.

use mokiterions::cli::{Command, USAGE, parse};
use mokiterions::simulation::{Config, Density, Policy};

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

fn config_with(policy: Policy) -> Config {
    Config {
        seed: 0,
        tick_limit: 100,
        policy,
        density: Density::DEFAULT,
        trace_actions: false,
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

/// The lines of the options block: the text between the `Options:` heading and the blank
/// line that closes it.
fn options_block() -> Vec<&'static str> {
    let mut lines = USAGE.lines().skip_while(|line| *line != "Options:");
    assert_eq!(lines.next(), Some("Options:"), "no options block in USAGE");
    lines.take_while(|line| !line.trim().is_empty()).collect()
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
    // hide the third source nor advertise a fourth.
    let policy = entry("--policy");
    assert!(policy.contains("baseline"), "{policy}");
    assert!(policy.contains("reference"), "{policy}");
    assert!(policy.contains("individual"), "{policy}");
    assert!(parse(["--policy", "baseline"]).is_ok());
    assert!(parse(["--policy", "reference"]).is_ok());
    assert!(parse(["--policy", "individual"]).is_ok());
    assert!(parse(["--policy", "random"]).is_err());

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
