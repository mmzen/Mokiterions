use crate::simulation::{Config, Density, Policy, UnitPrices};

/// The usage text, whose content is specified by `SPEC-MOK-001`'s *Help output* section
/// under `REQ-MOK-018`. Every default stated below is the value `parse` applies when the
/// option is omitted; `tests/cli.rs` holds the two equal, so neither can move alone.
///
/// One literal per output line, concatenated at compile time: the alternative is a single
/// escaped literal too long to read, and a multi-line literal would take its line endings
/// from however the file was checked out.
///
/// Amended 2026-08-22 under `WO-MOK-024`. What an option means, what its accepted values
/// mean and what happens when it is omitted are stated in that option's own entry, so
/// nothing an operator needs is carried by prose they have to find. The `--policy`
/// values and what `--density` binds together were previously stated below the block; they
/// are stated in their entries now, and once, as `SPEC-MOK-001` requires. Blank lines
/// separate the entries: the options block ends at the first line that is not indented.
///
/// Amended again 2026-08-23 under `WO-MOK-025` for `SPEC-MOK-007` rule 18. `--policy` gained a
/// fifth value, and rule 18.1 leaves the four existing values, their order and their own text
/// alone: `llm` is added after them and nothing above it moved. Two sentences did have to
/// change, and rule 18.3 names one of them — "None of the four learns anything or calls a
/// model; all four are deterministic" was a true statement about a closed set of four and
/// became false about a set of five, so it now says of which values it holds. The description
/// above the options block said "the same options always produce exactly the same run", which
/// the fifth value falsifies just as squarely; rule 18.3 does not name it, and it is corrected
/// anyway, because shipping a help text that states something untrue is a defect whichever rule
/// happened to notice it.
///
/// The `llm` entry names no option. Rule 20.4 puts the connector in a host's hands and the
/// options that reach one arrive with the transcript, so the entry states what the value is and
/// what happens with no connector — rule 20.8's refusal, which is permanent — rather than
/// naming a flag that does not exist yet.
///
/// Amended again 2026-08-24 under `WO-MOK-025` for `SPEC-MOK-007` rule 18.4, which adds
/// `--transcript-path`. It is the first option **both hosts act on**: the engine's binary target
/// opens it and so does the observer, which rule 18.4.2 makes the whole of that host's share of
/// this source. So it joins the shared block below rather than the engine-only prose, and the
/// exit-status sentence gained two clauses — the options that do not go together, which is rules
/// 18.4.3 and 13.2, and a transcript that cannot be read, which is neither a configuration error
/// nor an output failure and had no clause at all.
///
/// Amended again 2026-08-29 under `WO-MOK-026` for `SPEC-MOK-007` rule 14.3a, which adds
/// `--prices`. It is the second of these options whose value is retained rather than discarded, and
/// its entry says what the four numbers are, in what unit and in what order, because a price list
/// written in the wrong order produces a plausible cost figure and no error at all. It does not
/// state a default, because rule 14.3 forbids one. The `--live` entry's "all three" became "all
/// four" in the same act: a help text that enumerates a set is a help text that has to be counted
/// again whenever the set grows.
///
/// **A continuation line must not begin with `--`, and this is written down because it was found by
/// failing rather than by reading.** `tests/cli.rs` reads the options block by taking a line whose
/// first non-blank characters are `--` to open a new entry, which is the only way to know where one
/// entry ends without a second declaration of the list. Rewrapping the `--live` entry so that
/// `--connector-path` and `--prices` began lines therefore invented two entries and shadowed the
/// real `--prices` one, and three tests failed at once. The wrap is chosen around that constraint —
/// "named" ends one line and "and" opens the next — rather than the constraint being loosened, and
/// the `--prices` entry says "the option called --live" for the same reason and in the same idiom
/// `--transcript-output`'s entry already used.
///
/// The five entries `--seed`, `--ticks`, `--policy`, `--density` and `--transcript-path` are
/// shared verbatim with the observer's own usage text. `mokiterions-tui/tests/options.rs` holds
/// each of them equal to this constant, so the two texts cannot drift apart while describing the
/// same input.
pub const USAGE: &str = concat!(
    "Usage: Mokiterions [--seed <number>] [--ticks <number>]\n",
    "                   [--policy <baseline|reference|individual|social|llm>]\n",
    "                   [--density <percent>] [--trace-actions]\n",
    "                   [--events-path <path>] [--transcript-path <path>]\n",
    "                   [--connector-path <path>] [--live]\n",
    "                   [--transcript-output <path>] [--spend-ceiling <amount>]\n",
    "                   [--prices <prompt:cached:output:reasoning>]\n",
    "       Mokiterions --help\n",
    "\n",
    "Mokiterions simulates a small closed world. Twelve creatures, each also called a\n",
    "Mokiterion, live on a 128 by 128 grid split into two territories, look for food,\n",
    "eat it, and can die. Nothing is learned and nothing is random beyond the seed:\n",
    "the same options produce exactly the same run, with the one exception named in\n",
    "the policy entry below. Given no options at all it runs 100 turns of the default\n",
    "world and prints what happened.\n",
    "\n",
    "Options:\n",
    "\n",
    "  --seed <number>\n",
    "      Starting number for every random draw the run makes. Change it for a\n",
    "      different world; keep it to repeat one exactly. Default: 0.\n",
    "\n",
    "  --ticks <number>\n",
    "      How many turns to run. In one turn every living Mokiterion gets one\n",
    "      decision. Must be greater than zero. A run stops earlier only when no\n",
    "      Mokiterion is left alive. Default: 100.\n",
    "\n",
    "  --policy <baseline|reference|individual|social|llm>\n",
    "      How each Mokiterion chooses its next action. The first four are fixed sets\n",
    "      of rules: none of them learns anything or calls a model, and all four are\n",
    "      deterministic. The fifth asks a model and is neither. Default: reference.\n",
    "      baseline    Chooses at random among the actions that are legal for it\n",
    "                  this turn. The control case, for comparison.\n",
    "      reference   Walks toward the nearest food it can see and eats it, but\n",
    "                  refuses food whose value it would partly waste.\n",
    "      individual  Like reference, except each Mokiterion will waste a little,\n",
    "                  by an amount the seed fixes for it alone, so two Mokiterions\n",
    "                  in the same position can act differently.\n",
    "      social      Like individual while it sees nobody else. When it does see\n",
    "                  another Mokiterion it may strike back, attack, threaten,\n",
    "                  close in, or keep away, depending on how afraid it is.\n",
    "      llm         Asks a language model, one question per Mokiterion per turn,\n",
    "                  each in its own context with no memory of any other. The\n",
    "                  model is reached through a separate connector program you\n",
    "                  supply, never by this program itself, so what it answers is\n",
    "                  not fixed by the seed and two runs may differ. A recorded\n",
    "                  run replays exactly from its transcript. Given no connector,\n",
    "                  the run is refused rather than run some other way.\n",
    "\n",
    "  --density <percent>\n",
    "      How much food the world holds, as a percentage of one territory's 8192\n",
    "      cells, written with at most two decimal places. Default: 0.75. The one\n",
    "      value sets three things together: the food present at the start, the most\n",
    "      a territory can ever hold, and the level regrowth aims for. It must leave\n",
    "      at least one resource per territory and must not exceed 100. Runs are\n",
    "      comparable only with runs at the same density.\n",
    "\n",
    "  --trace-actions\n",
    "      Also print one trace line for every living Mokiterion every turn, giving\n",
    "      the action it proposed and whether the engine accepted it. Off unless\n",
    "      given. Tracing only observes: the same seed produces the same run either\n",
    "      way.\n",
    "\n",
    "  --events-path <path>\n",
    "      Also write a machine-readable record stream to this file, replacing any\n",
    "      file already there. No record stream is written unless given. Standard\n",
    "      output is byte-for-byte the same whether or not this option is used, and\n",
    "      nothing ever reads the file back.\n",
    "\n",
    "  --transcript-path <path>\n",
    "      Read this run's decisions from a transcript of an earlier run, rather\n",
    "      than asking a model for them again. This is how --policy llm replays;\n",
    "      use --live instead to make a new recording. One of the two is needed\n",
    "      with --policy llm and they may not be given together, and both are\n",
    "      refused with any other policy, which needs no transcript and would\n",
    "      ignore one. The replayed run reproduces the recorded one exactly and\n",
    "      stops with an error rather than guessing if the transcript does not\n",
    "      match the run being replayed.\n",
    "\n",
    "  --connector-path <path>\n",
    "      A program you supply that this one runs and talks to, which is the\n",
    "      only way a model is ever reached: this program makes no network call\n",
    "      itself and never reads your credentials. The connector reads them\n",
    "      from its own environment. Needed with --live and refused without it.\n",
    "      docs/CONNECTOR_PROTOCOL.md says how to write one.\n",
    "\n",
    "  --live\n",
    "      Ask a model for this run's decisions instead of replaying recorded\n",
    "      ones. This is the only option that spends money, and it is off\n",
    "      unless given. It will not start unless all four of the options\n",
    "      named --connector-path and --transcript-output and --spend-ceiling\n",
    "      and --prices are given too. Two runs with the same seed may differ,\n",
    "      which no other option can cause.\n",
    "\n",
    "  --transcript-output <path>\n",
    "      Write this live run's decisions to this file, replacing any file\n",
    "      already there, so the run can be replayed later using the option\n",
    "      called --transcript-path instead. Required with --live: a run that\n",
    "      spent its exchanges without recording them would have cost money and\n",
    "      left no evidence. It may not be given together with the option\n",
    "      called --transcript-path.\n",
    "\n",
    "  --spend-ceiling <amount>\n",
    "      The most this run may spend, as an amount with at most two decimal\n",
    "      places, such as 2 or 2.50. Checked before each question rather than\n",
    "      after it, so the run stops rather than passing the figure. Required\n",
    "      with --live. The run reports what it spent either way.\n",
    "\n",
    "  --prices <prompt:cached:output:reasoning>\n",
    "      What the model charges, as four whole numbers of US cents per million\n",
    "      tokens, separated by colons and given in that order, such as\n",
    "      125:13:1000:0 for $1.25, $0.13, $10.00 and nothing. Required with\n",
    "      the option called --live, and there is no built-in list: prices are\n",
    "      the provider's to change, so a run states the ones it was costed at\n",
    "      and a later reader can check the figure it reports. Find them in the\n",
    "      provider's own pricing page. The second number is the reduced price\n",
    "      for prompt tokens the provider serves from its cache.\n",
    "\n",
    "  --help\n",
    "      Print this text and exit without running a simulation.\n",
    "\n",
    "Options may appear in any order, and each may appear at most once.\n",
    "\n",
    "What is printed: one line naming the chosen policy, then one line per notable\n",
    "event as the run proceeds, then a closing summary of how the run ended. All of\n",
    "it goes to standard output. A configuration error goes to standard error,\n",
    "followed by this text.\n",
    "\n",
    "Exit status: 0 when the run finished or this text was printed, 2 when an option\n",
    "was unknown, repeated, missing its value or outside what it accepts, and when\n",
    "the options given do not go together, and 1 when output could not be written or\n",
    "a transcript could not be read or written.\n",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Help,
    Run(Config),
}

/// An operator's amount to an integer count of minor units, with **no floating-point value at any
/// point**. `SPEC-MOK-007` rule 14.2 fixes cost as integer arithmetic in a stated minor unit, and
/// `SPEC-MOK-006` forbids a floating-point value in a stream, so a `f64` here would have to be
/// converted back before anything could be reported and would carry its rounding into the figure.
///
/// `Density::parse` is the precedent and this follows it: at most two decimal places, both digits
/// significant, and the fractional part padded rather than truncated so that `2.5` is 250 minor
/// units and not 25.
///
/// `2` is 200 minor units. A bare integer is the common case and the operator should not have to
/// write `2.00` to get it.
fn parse_minor_units(value: &str) -> Result<u64, String> {
    let (whole, fraction) = match value.split_once('.') {
        Some((whole, fraction)) => (whole, fraction),
        None => (value, ""),
    };
    if whole.is_empty() || !whole.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("expected an amount such as 2 or 2.50".into());
    }
    if fraction.len() > 2 || !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("expected at most two decimal places".into());
    }
    let major: u64 = whole
        .parse()
        .map_err(|_| "the amount is larger than this program can hold".to_string())?;
    // Padded, not truncated: `.5` is fifty minor units, and reading it as five would silently
    // authorise a tenth of what the operator wrote.
    let minor: u64 = match fraction.len() {
        0 => 0,
        1 => {
            fraction
                .parse::<u64>()
                .map_err(|_| "invalid amount".to_string())?
                * 10
        }
        _ => fraction.parse().map_err(|_| "invalid amount".to_string())?,
    };
    major
        .checked_mul(100)
        .and_then(|units| units.checked_add(minor))
        .ok_or_else(|| "the amount is larger than this program can hold".to_string())
}

pub fn parse<I, S>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();
    let mut seed = None;
    let mut ticks = None;
    let mut policy = None;
    let mut density = None;
    let mut trace_actions = false;
    // A `bool` rather than the value, because nothing retains the value. `SPEC-MOK-006`
    // rule 1.2 keeps every path out of the library target, so this parser validates the
    // option and forgets it; the binary target reads the argument it will open. The flag
    // exists only to enforce the at-most-once rule every other option follows.
    let mut events_path = false;
    // A `bool` for the same reason and by the same rule: `SPEC-MOK-007` rule 18.4 names
    // `--events-path` as the precedent this option follows exactly, and rule 12.1.1 puts the
    // opening in the host. Unlike the sink, both hosts open this one, so both re-read the raw
    // argument — the observer as well as the engine's binary target.
    let mut transcript_path = false;
    // Three `bool`s and two values, and which is which is `VER-MOK-018` case `S6a` rather than a
    // preference. `--connector-path` and `--transcript-output` carry paths, so rule 18.4 has this
    // parser validate them and forget them; the binary target re-reads the raw argument it opens.
    // `--live` carries nothing to forget. `--spend-ceiling` and `--prices` carry quantities the run
    // acts on, so they are the two new options whose values the configuration retains — `S6a`
    // "scopes the discard rule to paths and this is not one", in rule 14.3a's own words. See
    // `Config::spend_ceiling` and `Config::prices`.
    let mut connector_path = false;
    let mut transcript_output = false;
    let mut live = false;
    let mut spend_ceiling = None;
    let mut prices = None;
    let mut help = false;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--seed" => {
                if seed.is_some() {
                    return Err("--seed may appear at most once".into());
                }
                let value = option_value(&args, index, "--seed")?;
                seed = Some(
                    value
                        .parse::<u64>()
                        .map_err(|_| format!("invalid --seed value: {value}"))?,
                );
                index += 2;
            }
            "--ticks" => {
                if ticks.is_some() {
                    return Err("--ticks may appear at most once".into());
                }
                let value = option_value(&args, index, "--ticks")?;
                let parsed = value
                    .parse::<u64>()
                    .map_err(|_| format!("invalid --ticks value: {value}"))?;
                if parsed == 0 {
                    return Err("--ticks must be greater than zero".into());
                }
                ticks = Some(parsed);
                index += 2;
            }
            "--policy" => {
                if policy.is_some() {
                    return Err("--policy may appear at most once".into());
                }
                let value = option_value(&args, index, "--policy")?;
                policy = Some(Policy::parse(value).ok_or_else(|| {
                    format!(
                        "invalid --policy value: {value}; expected baseline, reference, individual, social, or llm"
                    )
                })?);
                index += 2;
            }
            "--density" => {
                if density.is_some() {
                    return Err("--density may appear at most once".into());
                }
                let value = option_value(&args, index, "--density")?;
                density = Some(
                    Density::parse(value)
                        .map_err(|reason| format!("invalid --density value: {value}; {reason}"))?,
                );
                index += 2;
            }
            "--trace-actions" => {
                if trace_actions {
                    return Err("--trace-actions may appear at most once".into());
                }
                trace_actions = true;
                index += 1;
            }
            "--events-path" => {
                if events_path {
                    return Err("--events-path may appear at most once".into());
                }
                // `option_value` unchanged, so a missing value and a value beginning with
                // `--` are both a missing value here for the same reason they are elsewhere.
                let value = option_value(&args, index, "--events-path")?;
                // `SPEC-MOK-006`'s *Inputs*: both spellings conventionally denote a standard
                // stream, and a sink interleaved with the text stream cannot leave that
                // stream's bytes unchanged. Rejecting the spelling is cheaper than defining
                // a behavior for it. Every other property of the value is the platform's,
                // and a path the platform refuses is a runtime failure rather than invalid
                // configuration, under rule 13.2.
                if value.is_empty() || value == "-" {
                    return Err(format!(
                        "invalid --events-path value: {value}; expected a file path, and no path denotes a standard stream"
                    ));
                }
                events_path = true;
                index += 2;
            }
            "--transcript-path" => {
                if transcript_path {
                    return Err("--transcript-path may appear at most once".into());
                }
                let value = option_value(&args, index, "--transcript-path")?;
                // `SPEC-MOK-007` rule 18.4 requires the empty value and the single `-` to be
                // rejected "for the reason `SPEC-MOK-001`'s `--events-path` bullet gives". The
                // reason transfers even though the direction does not: `-` conventionally denotes
                // a standard stream, and standard input is the observer's raw keyboard input, so
                // a transcript read from it would consume the operator's keystrokes and the
                // operator would be left with a run they could not steer or quit.
                if value.is_empty() || value == "-" {
                    return Err(format!(
                        "invalid --transcript-path value: {value}; expected a file path, and no path denotes a standard stream"
                    ));
                }
                transcript_path = true;
                index += 2;
            }
            "--connector-path" => {
                if connector_path {
                    return Err("--connector-path may appear at most once".into());
                }
                let value = option_value(&args, index, "--connector-path")?;
                // The same rejection as every other path-carrying option, for a reason that is
                // this option's own: `-` conventionally denotes a standard stream, and the
                // connector is a *program to spawn*. A standard stream is not an executable, so
                // the spelling could only ever be a mistake.
                if value.is_empty() || value == "-" {
                    return Err(format!(
                        "invalid --connector-path value: {value}; expected a path to a connector program, and no path denotes a standard stream"
                    ));
                }
                connector_path = true;
                index += 2;
            }
            "--transcript-output" => {
                if transcript_output {
                    return Err("--transcript-output may appear at most once".into());
                }
                let value = option_value(&args, index, "--transcript-output")?;
                if value.is_empty() || value == "-" {
                    return Err(format!(
                        "invalid --transcript-output value: {value}; expected a file path, and no path denotes a standard stream"
                    ));
                }
                transcript_output = true;
                index += 2;
            }
            "--live" => {
                if live {
                    return Err("--live may appear at most once".into());
                }
                live = true;
                index += 1;
            }
            "--spend-ceiling" => {
                if spend_ceiling.is_some() {
                    return Err("--spend-ceiling may appear at most once".into());
                }
                let value = option_value(&args, index, "--spend-ceiling")?;
                spend_ceiling = Some(parse_minor_units(value).map_err(|reason| {
                    format!("invalid --spend-ceiling value: {value}; {reason}")
                })?);
                index += 2;
            }
            "--prices" => {
                if prices.is_some() {
                    return Err("--prices may appear at most once".into());
                }
                let value = option_value(&args, index, "--prices")?;
                prices = Some(
                    UnitPrices::parse(value)
                        .map_err(|reason| format!("invalid --prices value: {value}; {reason}"))?,
                );
                index += 2;
            }
            "--help" => {
                if help {
                    return Err("--help may appear at most once".into());
                }
                help = true;
                index += 1;
            }
            unknown => return Err(format!("unknown option: {unknown}")),
        }
    }

    if help {
        return Ok(Command::Help);
    }

    // The two checks below are the only ones in this parser that read two options together, and
    // they are here rather than in an arm because options may appear in any order: the arm for
    // `--transcript-path` cannot know whether `--policy` follows it.
    //
    // `--help` is decided first, so neither refusal can make the usage text unobtainable. That
    // ordering differs from every rejection above, which beat `--help` because they are rejections
    // *of a value*; these two are about a combination, and an operator who asks what the
    // combinations are must not be answered with a complaint about the one they typed.
    let policy = policy.unwrap_or_default();

    // `SPEC-MOK-007` rule 18.4.3: rejected, not accepted and ignored. The rule gives the reason
    // and it is worth keeping here — "an option is an operator's stated intent, and an operator
    // who names a transcript for a `social` run has misunderstood something that a silent success
    // would leave misunderstood". Note that the default source is enough to trigger it: a
    // transcript with no `--policy` at all is a transcript for `reference`.
    if transcript_path && policy != Policy::Llm {
        return Err(format!(
            "--transcript-path is only used by --policy llm; --policy {policy} was selected, which obtains its own decisions"
        ));
    }

    // Rule 18.4.3 again, for the three options `WO-MOK-026` adds: rejected when another source is
    // selected, not accepted and ignored. One arm each rather than a loop, so the diagnostic names
    // the option the operator actually typed.
    for (present, option) in [
        (connector_path, "--connector-path"),
        (transcript_output, "--transcript-output"),
        (live, "--live"),
        (spend_ceiling.is_some(), "--spend-ceiling"),
        (prices.is_some(), "--prices"),
    ] {
        if present && policy != Policy::Llm {
            return Err(format!(
                "{option} is only used by --policy llm; --policy {policy} was selected, which obtains its own decisions"
            ));
        }
    }

    // Rule 18.4.4: the two transcript options are mutually exclusive, and giving both is a usage
    // error under rule 19.2. A run reads a transcript or writes one and never both, so there is no
    // reading of the pair that could be honoured — refusing is the only answer that does not
    // silently pick one.
    if transcript_path && transcript_output {
        return Err(
            "--transcript-path and --transcript-output may not be given together: a run either replays from a recorded transcript or records a live one, never both"
                .into(),
        );
    }

    // Rule 13.2 — "when the live-mode selection is absent, the run replays if a transcript was
    // supplied and otherwise refuses with the usage-error status" — and rule 19.2, which names
    // "a replay with no transcript" among the usage errors. Under this work order there is no
    // live-mode selection at all, so the condition is unconditional; `WO-MOK-026` adds the
    // selection and this check gains its second term rather than moving.
    //
    // Made here, by the parser both hosts share, so rule 20.3's refusal holds for the observer by
    // construction rather than by a second implementation of it. It also refuses for the engine's
    // binary target, which rule 20.1 makes the recording host — correctly, because a recording
    // host with nothing to record from and no connector to reach is in exactly the same position.
    // Rule 13.2 as it now reads with a live-mode selection in existence. `WO-MOK-025` left this
    // check unconditional and said so — "under this work order there is no live-mode selection at
    // all, so the condition is unconditional; `WO-MOK-026` adds the selection and this check gains
    // its second term rather than moving". This is that second term.
    //
    // A run under this source obtains its decisions from a transcript or from a connector. With
    // neither it has no source at all, which rule 19.2 names among the usage errors as "a replay
    // with no transcript", and rule 20.3 requires of the observer specifically.
    if policy == Policy::Llm && !transcript_path && !live {
        return Err(
            "--policy llm needs --transcript-path <path> to replay, or --live to record a new run: this source obtains its decisions from one or the other, and is never given a different source"
                .into(),
        );
    }

    // Rule 13.1's selection half, stated as a configuration error rather than discovered at the
    // first exchange. The credential half is the connector's and is checked nowhere here: rule
    // 13.1 puts the two conditions in two components deliberately, so that no single one of them
    // can authorise spending.
    if live && !connector_path {
        return Err(
            "--live needs --connector-path <path>: a live run reaches the model through a connector program you supply, and this engine makes no provider call itself"
                .into(),
        );
    }

    // Rule 19.6: a live run whose exchanges were spent and not recorded has produced cost and no
    // evidence, which is the one failure worth refusing in advance rather than discovering.
    if live && !transcript_output {
        return Err(
            "--live needs --transcript-output <path>: a live run records every exchange, and one that spent its exchanges without recording them would have produced cost and no evidence"
                .into(),
        );
    }

    // Rule 14.6 needs a ceiling to check against, and rule 19.2 names "a live run with no ceiling"
    // among the usage errors. Refused before any tick, which is where a spend limit has to be
    // decided: a run that discovered it had no ceiling after its first exchange has already spent.
    if live && spend_ceiling.is_none() {
        return Err(
            "--live needs --spend-ceiling <amount>: a live run spends money, and the ceiling is checked before each exchange rather than after it"
                .into(),
        );
    }

    // Rule 14.3: "The declared unit prices are inputs of the run, not compiled-in constants." With
    // no prices a live run cannot do rule 14.2's arithmetic, cannot make rule 14.6's pre-exchange
    // check and cannot report rule 15.2's cost, and the only other thing it could do is the one
    // thing rule 14.3 forbids. So it is refused here, in rule 19.2's own shape and beside the
    // ceiling's refusal above — both before any tick and before any provider call.
    if live && prices.is_none() {
        return Err(
            "--live needs --prices <prompt:cached:output:reasoning>: cost is computed from the provider's prices, which are the provider's to change and are therefore declared for each run rather than built into this program"
                .into(),
        );
    }

    Ok(Command::Run(Config {
        seed: seed.unwrap_or(0),
        tick_limit: ticks.unwrap_or(100),
        policy,
        density: density.unwrap_or_default(),
        trace_actions,
        spend_ceiling,
        prices,
    }))
}

fn option_value<'a>(args: &'a [String], index: usize, option: &str) -> Result<&'a str, String> {
    let value = args
        .get(index + 1)
        .ok_or_else(|| format!("missing value for {option}"))?;
    if value.starts_with("--") {
        return Err(format!("missing value for {option}"));
    }
    Ok(value)
}
