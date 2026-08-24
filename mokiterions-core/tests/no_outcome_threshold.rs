//! `VER-MOK-018` case **L26**: no outcome threshold exists for the model-backed source, anywhere in
//! the verification suite.
//!
//! Every other case in that document asks whether something is true of the engine. This one asks
//! whether something is *absent from the suite itself*, and `VER-MOK-018`'s opening section says why
//! in its own words: four decision sources in a row received a floor — `REQ-MOK-014` the reference
//! source, `REQ-MOK-034` the trait-aware source, `REQ-MOK-058` the social source, `REQ-MOK-060` three
//! of them — so *"a fifth contract arriving with no such case would read as an oversight to anyone who
//! had read the other four, and the correction would arrive as a 'missing' assertion that nobody
//! decided to add."* `INT-MOK-011` records the non-goal and `ADR-MOK-007` decision 7 its architectural
//! consequence. The case's last sentence is the whole of this file's purpose: **it fails when such an
//! assertion is added.**
//!
//! So this reads the suite's own source as text. That is unusual for a test and it is the only thing
//! that can carry the case: an absence cannot be observed by running anything, because a threshold
//! assertion that nobody has written produces no behaviour to observe. What it looks for is an
//! assertion that reaches for an outcome — a survivor count, a death count, a combat rate, an
//! extinction — inside a test that concerns this source.
//!
//! **What it is not.** It is not a ban on the vocabulary: the other four sources have floors and must
//! keep them, `baseline`'s extinction between ticks 119 and 193 is recorded as measurement, and this
//! file must not object to any of it. The pairing is what it objects to — an outcome assertion inside a
//! test that names the fifth source.

use std::fs;
use std::path::{Path, PathBuf};

/// Every file whose text is scanned, relative to the workspace root.
///
/// The engine's internal tier is a module inside `simulation.rs`, so the file is read whole and the
/// function split below finds the tests within it. The observer's package is included because
/// `VER-MOK-018` says *"anywhere in the verification suite"* and the observer is where this source's
/// second host is verified — a threshold added there would be as much an outcome assertion as one
/// added here.
const SCANNED: [&str; 4] = [
    "mokiterions-core/src",
    "mokiterions-core/tests",
    "mokiterions-tui/src",
    "mokiterions-tui/tests",
];

/// How the model-backed source is spelled. Any of these in a function's text puts that function in
/// scope for the scan.
const SOURCE: [&str; 4] = ["llm", "Llm", "LLM_SOURCE_NAME", "PortDecisionSource"];

/// What an outcome assertion reaches for.
///
/// Roots rather than words, so that a plural or a participle is covered by the same entry.
///
/// **`floor` and `ceiling` are deliberately not here**, and the reason was measured rather than
/// assumed: with them in the list this check failed on three assertions about the **spend ceiling** —
/// `ceiling_nanodollars` and `"ended":"ceiling"` — which is a cost bound `SPEC-MOK-007` rule 9.8
/// *requires* of this source and the opposite of what `VER-MOK-018` case L26 forbids. Nothing is lost:
/// L26's own examples are a "survivor floor" and a "death ceiling", and both name their population, so
/// the entries below catch them.
const OUTCOME: [&str; 9] = [
    "survivor", "death", "died", "casualt", "combat", "extinct", "lethal", "living", "mortalit",
];

/// What makes a line a measurement of how a run ended rather than a bound on how it may end.
///
/// `VER-MOK-018`'s opening section is explicit that this distinction exists: *"`baseline` goes extinct
/// between ticks 119 and 193 on every declared seed, and that is recorded as measurement, never as
/// failure."* A test that asserts the run record says `"ended":"extinction"` is asserting that the
/// engine reported why it stopped. A test that asserts a run *did not* go extinct would be a
/// threshold, and it names no termination record, so it is not exempted here.
const TERMINATION: [&str; 4] = [
    "\"ended\"",
    "TerminationReason",
    "reason=",
    "transcript_flag",
];

/// The workspace root, from this package's own manifest directory.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("this package sits in a workspace")
        .to_path_buf()
}

/// Every `.rs` file under the given directory, one level deep and recursively.
fn rust_files(directory: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        // A scanned directory that does not exist is a scan that silently covers less than it says,
        // so it is a failure rather than an empty result.
        Err(error) => panic!("{}: {error}", directory.display()),
    };
    for entry in entries {
        let path = entry.expect("a readable directory entry").path();
        if path.is_dir() {
            found.extend(rust_files(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
    found.sort();
    found
}

/// One file's text split into the functions it declares, each chunk labelled with its name.
///
/// Split at the lines that declare a function rather than by matching braces. A brace matcher would be
/// exact and would also have to understand strings, character literals and comments to stay exact,
/// which is a parser. The approximation is safe in the direction that matters: a chunk runs from one
/// function's declaration to the next, so it can only ever hold *more* text than that function's body
/// — never less. A threshold assertion cannot fall outside every chunk, so nothing hides between them.
/// The cost is the opposite error, an assertion attributed to a neighbouring function, which affects
/// only the name printed in the complaint.
fn functions(text: &str) -> Vec<(String, String)> {
    let mut chunks: Vec<(String, String)> = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        let declaration = trimmed
            .strip_prefix("pub ")
            .unwrap_or(trimmed)
            .strip_prefix("pub(crate) ")
            .unwrap_or_else(|| trimmed.strip_prefix("pub ").unwrap_or(trimmed));
        if declaration.starts_with("fn ") || declaration.starts_with("async fn ") {
            let name = declaration
                .trim_start_matches("async ")
                .trim_start_matches("fn ")
                .split(['(', '<'])
                .next()
                .unwrap_or("?")
                .to_string();
            chunks.push((name, String::new()));
        }
        match chunks.last_mut() {
            Some((_, body)) => {
                body.push_str(line);
                body.push('\n');
            }
            // Text before the first function: the module's own header and its items. Kept, under a
            // name that says so, because a threshold could be written as a constant.
            None => chunks.push((
                "<before the first function>".to_string(),
                format!("{line}\n"),
            )),
        }
    }
    chunks
}

/// The outcome roots an assertion line reaches for, if any.
///
/// A line is an assertion when it contains `assert`, and a comment is not one: a doc comment that
/// explains why there is no floor would otherwise be the thing that fails this check. That is not a
/// loophole — a threshold written into a comment asserts nothing.
fn outcome_in_assertion(line: &str) -> Vec<&'static str> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || !line.contains("assert") {
        return Vec::new();
    }
    // Against a copy with the backslashes removed, because the field is written into an assertion as
    // an escaped JSON string — `\"ended\"` in the source text — and a marker of `"ended"` would not
    // match it. Removing them costs nothing here: no marker contains a backslash.
    let unescaped = line.replace('\\', "");
    if TERMINATION.iter().any(|marker| unescaped.contains(marker)) {
        return Vec::new();
    }
    let lowered = line.to_lowercase();
    OUTCOME
        .iter()
        .copied()
        .filter(|root| lowered.contains(root))
        .collect()
}

#[test]
fn no_outcome_threshold_exists_for_the_model_backed_source() {
    // The detector's own negative case, first, so that a scan reporting nothing is a scan that would
    // have reported something. Without this the check passes on the day its vocabulary stops matching.
    assert_eq!(
        outcome_in_assertion("        assert!(survivors >= 5, \"the floor\");"),
        vec!["survivor"],
        "the detector no longer recognises an outcome assertion"
    );
    assert_eq!(
        outcome_in_assertion("        assert!(deaths <= 3);"),
        vec!["death"],
        "the detector no longer recognises a death ceiling"
    );
    assert!(
        outcome_in_assertion("        // no survivor floor is asserted for this source").is_empty(),
        "the detector reads comments as assertions"
    );
    assert!(
        outcome_in_assertion("        let survivors = world.living();").is_empty(),
        "the detector reads a measurement as an assertion"
    );
    assert!(
        outcome_in_assertion(
            "        assert!(record.contains(\"\\\"ended\\\":\\\"extinction\\\"\"));"
        )
        .is_empty(),
        "the detector reads a termination reason as a threshold"
    );
    assert!(
        outcome_in_assertion("        assert!(record.contains(\"ceiling_nanodollars\"));")
            .is_empty(),
        "the detector reads the spend ceiling as an outcome threshold"
    );

    let root = workspace_root();
    let mut complaints: Vec<String> = Vec::new();
    let mut in_scope = 0;
    let mut assertions = 0;
    let mut files = 0;

    for directory in SCANNED {
        for path in rust_files(&root.join(directory)) {
            files += 1;
            let text = fs::read_to_string(&path).expect("a readable source file");
            let relative = path
                .strip_prefix(&root)
                .unwrap_or(&path)
                .display()
                .to_string();
            for (name, body) in functions(&text) {
                if !SOURCE.iter().any(|spelling| body.contains(spelling)) {
                    continue;
                }
                in_scope += 1;
                for (offset, line) in body.lines().enumerate() {
                    if line.contains("assert") && !line.trim_start().starts_with("//") {
                        assertions += 1;
                    }
                    let roots = outcome_in_assertion(line);
                    if !roots.is_empty() {
                        complaints.push(format!(
                            "{relative}: {name} (+{offset}) asserts on {}: {}",
                            roots.join(", "),
                            line.trim()
                        ));
                    }
                }
            }
        }
    }

    // Vacuity: the scan has to have found the source somewhere. A rename that this file did not learn
    // about would otherwise leave nothing in scope and the case would hold over nothing at all.
    assert!(
        in_scope >= 20,
        "only {in_scope} function(s) name the model-backed source across {files} file(s); \
         either the scan is looking in the wrong place or the source is spelled another way"
    );

    assert!(
        complaints.is_empty(),
        "VER-MOK-018 case L26: an outcome threshold exists for the model-backed source.\n\
         This case fails when such an assertion is added, and INT-MOK-011 records the non-goal.\n{}",
        complaints.join("\n")
    );

    println!(
        "{files} file(s) scanned, {in_scope} function(s) name the model-backed source, \
         {assertions} assertion(s) among them, none on an outcome"
    );
}
