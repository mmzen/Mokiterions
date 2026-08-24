use crate::simulation::{Config, Density, Policy};

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
/// The five entries `--seed`, `--ticks`, `--policy`, `--density` and `--transcript-path` are
/// shared verbatim with the observer's own usage text. `mokiterions-tui/tests/options.rs` holds
/// each of them equal to this constant, so the two texts cannot drift apart while describing the
/// same input.
pub const USAGE: &str = concat!(
    "Usage: Mokiterions [--seed <number>] [--ticks <number>]\n",
    "                   [--policy <baseline|reference|individual|social|llm>]\n",
    "                   [--density <percent>] [--trace-actions]\n",
    "                   [--events-path <path>] [--transcript-path <path>]\n",
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
    "      than asking a model for them again. Required with --policy llm, and\n",
    "      refused with any other policy, which needs no transcript and would\n",
    "      ignore one. The replayed run reproduces the recorded one exactly, and\n",
    "      stops with an error rather than guessing if the transcript does not\n",
    "      match the run being replayed.\n",
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
    "a transcript could not be read.\n",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Help,
    Run(Config),
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
    if policy == Policy::Llm && !transcript_path {
        return Err(
            "--policy llm needs --transcript-path <path>: the transcript supplies this run's decisions, and a run under this policy is never given a different source"
                .into(),
        );
    }

    Ok(Command::Run(Config {
        seed: seed.unwrap_or(0),
        tick_limit: ticks.unwrap_or(100),
        policy,
        density: density.unwrap_or_default(),
        trace_actions,
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
