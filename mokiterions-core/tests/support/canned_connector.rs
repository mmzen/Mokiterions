//! The canned connector: `SPEC-MOK-007` rule 20.5's offline fixture.
//!
//! It speaks the connector protocol of rule 10 — the protocol `docs/CONNECTOR_PROTOCOL.md`
//! documents — and answers from a script instead of from a model. **It performs no network I/O,
//! reads no credential, and names no provider.** That is its whole point: the spawn, the framing,
//! the two gates, the usage accounting, the cost arithmetic, the spend ceiling and the retry path
//! are all exercisable with no provider, no credential and no cost.
//!
//! It is **not** a reference implementation of a real connector, and `VER-MOK-018` case `S2` states
//! plainly that checking it establishes nothing about an operator's. Rule 10.6 is explicit that this
//! specification does not constrain, and cannot constrain, a program the operator supplies.
//!
//! It lives under `tests/support/` rather than directly in `tests/` because Cargo auto-discovers
//! `tests/*.rs` as integration-test targets and this is not a test — it is a program a test spawns.
//! `mokiterions-core/Cargo.toml` declares it as a `[[bin]]`, which is what makes it a real child
//! process with a path an integration test can reach through `CARGO_BIN_EXE_canned-connector`.
//! That declaration is `WO-HUP-002`'s recorded owner decision of 2026-08-28, taken because the
//! fixture cannot be a child process without a target and `WO-MOK-026`'s own scope prose did not
//! settle it.
//!
//! # The script
//!
//! The path is read from `MOKITERIONS_CANNED_SCRIPT`. With no script the connector answers `wait`
//! to everything, which is always well-formed: `wait` is a core verb with no parameter and block D
//! offers it at every opportunity.
//!
//! One directive per line. Blank lines and lines beginning with `#` are ignored. **When the script
//! is exhausted the last directive repeats**, so a two-line script drives a ten-thousand-exchange
//! run and a test states only what it cares about.
//!
//! ```text
//! ok <verb> [<parameter>] [prompt=N] [cached=N] [output=N] [reasoning=N] [model=ID] [level=TEXT]
//! error <kind> <message ...>
//! malformed <raw text>
//! close
//! ```
//!
//! - `ok` answers with an action, what answered, and usage counts. Counts default to `prompt=1000
//!   cached=900 output=8 reasoning=0`, a ratio of 0.9, which is above `REQ-MOK-070`'s 0.85 and
//!   therefore the uninteresting case a test has to opt out of rather than into. `model` and `level`
//!   default to `canned-connector` and `none`.
//! - **The level is `level=` and not `reasoning=`** because `reasoning=` is already the reasoning
//!   *token count*. Rule 10.4a puts both on one response — a level that is a string beside a count
//!   that is an integer, at two levels of the object — and one script key could not set two values.
//! - A response that answers without naming what answered is reachable through `malformed`, which
//!   writes a raw line. That is how the host's rule 10.4c handling gets exercised — an action with
//!   neither field fails the grammar check and becomes a counted fallback under rule 9.5 — without a
//!   directive of its own, and every `ok` therefore names both.
//! - `error` answers with rule 10.4's error form. `<kind>` is any of the four the protocol fixes;
//!   the connector does not police it, because a connector that could only emit valid errors could
//!   not exercise the host's handling of an invalid one.
//! - `malformed` writes the raw text as the whole line. It exists so the host's parsing can be
//!   driven off the protocol entirely — a connector is untrusted in whole under rule 10.7, and a
//!   fixture that could only emit well-formed lines could not demonstrate that.
//! - `close` stops reading and exits `0` with the request unanswered, which is a connector that
//!   died mid-run.

use std::env;
use std::io::{self, BufRead, Write};
use std::process::ExitCode;

/// Usage counts as they appear on a response, in the order rule 10.4 lists them.
struct Usage {
    prompt: u64,
    cached: u64,
    output: u64,
    reasoning: u64,
}

impl Default for Usage {
    fn default() -> Self {
        // A ratio of 0.9. Above `REQ-MOK-070`'s 0.85 deliberately: a fixture whose default
        // failed the obligation would make every test that does not care about the ratio
        // depend on it.
        Self {
            prompt: 1000,
            cached: 900,
            output: 8,
            reasoning: 0,
        }
    }
}

/// What answered, as rule 10.4a has a response report it: the model identifier and the reasoning
/// level, both accompanying the action. A response that carries an action and names neither fails
/// the host's grammar check under rule 10.4c, so every `ok` directive names both.
struct Answered {
    model: String,
    level: String,
}

impl Default for Answered {
    fn default() -> Self {
        Self {
            // This fixture, not a provider. The module's first claim is that it names none, and a
            // default of `gpt-5.6-luna` would have every test that does not care about the
            // identifier assert a real model's name — and would read, in a run record produced
            // from a script, exactly as a live run's record reads.
            model: "canned-connector".to_string(),
            // Rule 8.5 fixes the level a run may use at `none`.
            level: "none".to_string(),
        }
    }
}

enum Directive {
    Ok {
        verb: String,
        parameter: Option<String>,
        answered: Answered,
        usage: Usage,
    },
    Error {
        kind: String,
        message: String,
    },
    Malformed {
        raw: String,
    },
    Close,
}

impl Directive {
    /// One script line to one directive. An unparseable line is a fixture defect and is reported
    /// as one rather than silently becoming a `wait`: a fixture that quietly does something else
    /// than the test asked would make a passing test meaningless.
    fn parse(line: &str) -> Result<Self, String> {
        let mut words = line.split_whitespace();
        let head = words.next().ok_or_else(|| "empty directive".to_string())?;
        match head {
            "close" => Ok(Self::Close),
            "malformed" => Ok(Self::Malformed {
                raw: line["malformed".len()..].trim().to_string(),
            }),
            "error" => {
                let kind = words
                    .next()
                    .ok_or_else(|| "error needs a kind".to_string())?;
                Ok(Self::Error {
                    kind: kind.to_string(),
                    message: words.collect::<Vec<_>>().join(" "),
                })
            }
            "ok" => {
                let verb = words
                    .next()
                    .ok_or_else(|| "ok needs a verb".to_string())?
                    .to_string();
                let mut parameter = None;
                let mut answered = Answered::default();
                let mut usage = Usage::default();
                for word in words {
                    match word.split_once('=') {
                        Some(("prompt", value)) => usage.prompt = number(value)?,
                        Some(("cached", value)) => usage.cached = number(value)?,
                        Some(("output", value)) => usage.output = number(value)?,
                        Some(("reasoning", value)) => usage.reasoning = number(value)?,
                        Some(("model", value)) => answered.model = value.to_string(),
                        Some(("level", value)) => answered.level = value.to_string(),
                        Some((key, _)) => return Err(format!("unknown field `{key}`")),
                        // The first bare word after the verb is the parameter; a second is a
                        // defect rather than a value silently dropped.
                        None if parameter.is_none() => parameter = Some(word.to_string()),
                        None => return Err(format!("unexpected word `{word}`")),
                    }
                }
                Ok(Self::Ok {
                    verb,
                    parameter,
                    answered,
                    usage,
                })
            }
            other => Err(format!("unknown directive `{other}`")),
        }
    }

    /// The response line, without its terminator. `None` for `close`.
    fn response(&self) -> Option<String> {
        match self {
            Self::Close => None,
            Self::Malformed { raw } => Some(raw.clone()),
            Self::Error { kind, message } => Some(format!(
                "{{\"protocol\":1,\"error\":{{\"kind\":\"{}\",\"message\":\"{}\"}}}}",
                escape(kind),
                escape(message)
            )),
            Self::Ok {
                verb,
                parameter,
                answered,
                usage,
            } => {
                let mut action = format!("{{\"verb\":\"{}\"", escape(verb));
                if let Some(parameter) = parameter {
                    action.push_str(&format!(",\"parameter\":\"{}\"", escape(parameter)));
                }
                action.push('}');
                // The field order rule 10.4a lists and `docs/CONNECTOR_PROTOCOL.md`'s worked
                // exchange shows: the action, then what answered, then what it cost. JSON does not
                // care and a host reading this must not either, but a fixture whose lines differ in
                // shape from the document a connector author reads is a second thing to keep true.
                Some(format!(
                    "{{\"protocol\":1,\"action\":{action},\"model\":\"{}\",\"reasoning\":\"{}\",\
                     \"usage\":{{\"prompt\":{},\"cached_prompt\":{},\"output\":{},\
                     \"reasoning\":{}}}}}",
                    escape(&answered.model),
                    escape(&answered.level),
                    usage.prompt,
                    usage.cached,
                    usage.output,
                    usage.reasoning
                ))
            }
        }
    }
}

fn number(value: &str) -> Result<u64, String> {
    value
        .parse()
        .map_err(|_| format!("`{value}` is not a count"))
}

/// The two characters a JSON string may not carry raw. The fixture writes no control character
/// and no backslash of its own, so this is the whole of what it needs.
fn escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

fn load_script() -> Result<Vec<Directive>, String> {
    let Some(path) = env::var_os("MOKITERIONS_CANNED_SCRIPT") else {
        return Ok(vec![Directive::Ok {
            verb: "wait".to_string(),
            parameter: None,
            answered: Answered::default(),
            usage: Usage::default(),
        }]);
    };
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("cannot read script {}: {error}", path.to_string_lossy()))?;
    let mut directives = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        directives
            .push(Directive::parse(line).map_err(|why| format!("line {}: {why}", index + 1))?);
    }
    if directives.is_empty() {
        return Err("script holds no directive".to_string());
    }
    Ok(directives)
}

fn main() -> ExitCode {
    let directives = match load_script() {
        Ok(directives) => directives,
        Err(why) => {
            // Standard error, never standard output: standard output carries response lines and
            // nothing else, which the protocol's checklist states as an obligation on every
            // connector including this one.
            let _ = writeln!(io::stderr(), "canned-connector: {why}");
            return ExitCode::from(2);
        }
    };

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // `enumerate` rather than a counter of our own: the index is the exchange number, and the
    // last directive repeating is `min` against the script's length rather than a separate rule.
    for (answered, line) in stdin.lock().lines().enumerate() {
        // A read failure on the pipe is the host going away, which is not this fixture's error.
        let Ok(_request) = line else { break };

        // The request is not inspected. A connector may inspect it; this one answers from its
        // script, which is what makes a test's expectation depend on the script rather than on
        // the prompt the engine happened to compose.
        let directive = &directives[answered.min(directives.len() - 1)];
        let Some(response) = directive.response() else {
            break;
        };

        if writeln!(stdout, "{response}").is_err() || stdout.flush().is_err() {
            // The host closed the pipe. Nothing to report and nobody to report it to.
            break;
        }
    }

    ExitCode::SUCCESS
}
