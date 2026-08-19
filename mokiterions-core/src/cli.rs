use crate::simulation::{Config, Density, Policy};

/// The usage text, whose content is specified by `SPEC-MOK-001`'s *Help output* section
/// under `REQ-MOK-018`. Every default stated below is the value `parse` applies when the
/// option is omitted; `tests/cli.rs` holds the two equal, so neither can move alone.
///
/// One literal per output line, concatenated at compile time: the alternative is a single
/// escaped literal too long to read, and a multi-line literal would take its line endings
/// from however the file was checked out.
pub const USAGE: &str = concat!(
    "Usage: Mokiterions [--seed <u64>] [--ticks <u64>]\n",
    "                   [--policy <baseline|reference|individual>]\n",
    "                   [--density <percent>] [--trace-actions]\n",
    "       Mokiterions --help\n",
    "\n",
    "Options:\n",
    "  --seed <u64>                   Entropy stream seed. Default: 0.\n",
    "  --ticks <u64>                  Ticks to run; must be greater than zero.\n",
    "                                 Default: 100.\n",
    "  --policy <baseline|reference|individual>\n",
    "                                 Decision source. Default: reference.\n",
    "  --density <percent>            Resource density per territory, at most two\n",
    "                                 decimal places. Default: 0.75.\n",
    "  --trace-actions                Emit one action trace per living-agent decision\n",
    "                                 opportunity. Off unless given.\n",
    "  --help                         Print this usage and exit without running.\n",
    "\n",
    "Options may appear in any order and at most once.\n",
    "\n",
    "The reference policy is a deterministic development instrument, not autonomous\n",
    "behavior. It seeks and consumes perceived food so that world viability can be\n",
    "measured. The baseline policy selects uniformly among valid actions. The\n",
    "individual policy seeks and consumes as the reference policy does, except that\n",
    "each Mokiterion also accepts food it would partly waste, in proportion to its own\n",
    "waste tolerance, which is derived from the seed and its identifier.\n",
    "\n",
    "--density is the percentage of a territory's cells that hold a resource. It sets\n",
    "the initial endowment, the territory capacity, and the replenishment target\n",
    "together. Only the densities declared in the requirements carry a population\n",
    "viability floor.\n",
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
                        "invalid --policy value: {value}; expected baseline, reference, or individual"
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
        Ok(Command::Help)
    } else {
        Ok(Command::Run(Config {
            seed: seed.unwrap_or(0),
            tick_limit: ticks.unwrap_or(100),
            policy: policy.unwrap_or_default(),
            density: density.unwrap_or_default(),
            trace_actions,
        }))
    }
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
