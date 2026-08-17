use crate::simulation::{Config, Density, Policy};

pub const USAGE: &str = "Usage: Mokiterions [--seed <u64>] [--ticks <u64>] [--policy <baseline|reference>]\n                   [--density <percent>] [--trace-actions]\n       Mokiterions --help\n\nThe reference policy is a deterministic development instrument, not autonomous\nbehavior. It seeks and consumes perceived food so that world viability can be\nmeasured. The baseline policy selects uniformly among valid actions.\n\n--density is the percentage of a territory's cells that hold a resource, with at\nmost two decimal places. It defaults to 0.75. It sets the initial endowment, the\nterritory capacity, and the replenishment target together. Only the densities\ndeclared in the requirements carry a population viability floor.\n";

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
                    format!("invalid --policy value: {value}; expected baseline or reference")
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
