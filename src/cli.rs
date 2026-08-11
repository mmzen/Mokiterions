use crate::simulation::Config;

pub const USAGE: &str = "Usage: Mokiterions [--seed <u64>] [--ticks <u64>] [--trace-actions]\n       Mokiterions --help\n";

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
                trace_actions: false,
            })
        );
    }

    #[test]
    fn options_work_in_any_order() {
        assert_eq!(
            parse(["--trace-actions", "--ticks", "7", "--seed", "42"]).unwrap(),
            Command::Run(Config {
                seed: 42,
                tick_limit: 7,
                trace_actions: true,
            })
        );
    }

    #[test]
    fn duplicates_and_missing_values_are_rejected() {
        assert!(parse(["--seed", "1", "--seed", "2"]).is_err());
        assert!(parse(["--ticks", "--trace-actions"]).is_err());
        assert!(parse(["--trace-actions", "--trace-actions"]).is_err());
        assert!(parse(["--unknown"]).is_err());
    }
}
