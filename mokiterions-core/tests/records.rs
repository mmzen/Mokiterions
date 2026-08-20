//! The structured record stream, `SPEC-MOK-006`, from outside the engine.
//!
//! This tier drives the built binary and reads the file it wrote, so it asserts about the
//! artifact an operator actually gets: the bytes on disk, the exit code, the diagnostic, and
//! whether a file survives a failure. Nothing here reaches into the engine; the shapes that
//! only a contrived state produces are covered by the internal tier in
//! `src/simulation.rs`, and `SPEC-MOK-002` rule 9 keeps tests out of `src/main.rs`, which is
//! why the host's own behaviour is tested here rather than beside it.
//!
//! **The reader below is hand-written and deliberately strict.** `ARCH-MOK-001` and
//! `SPEC-MOK-006` rule 12.4 keep this package's dependency table empty, dev-dependencies
//! included, so there is no library to parse with. That is not a limitation here: a permissive
//! parser would accept a stream a consumer might reject, whereas this one refuses anything the
//! specification does not permit — an escape sequence, a float, an exponent, a leading zero, a
//! space, a character off rule 3.3's closed alphabet. Parsing the whole stream is therefore
//! itself the conformance check for rules 3.3 and 4.1, and every other test here gets the
//! parse for free.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const BINARY: &str = env!("CARGO_BIN_EXE_Mokiterions");

/// `SPEC-MOK-006` rule 3.3's closed value alphabet, and nothing else.
///
/// No quotation mark, no backslash, no code point below U+0020. That is what lets the writers
/// in the engine omit an escaping function and lets the reader below scan a quoted span by
/// looking for the next quotation mark.
fn on_alphabet(character: char) -> bool {
    character.is_ascii_alphanumeric()
        || matches!(character, '_' | '.' | '-' | '+' | ':' | ';' | '>')
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Text(String),
    Number(String),
    Bool(bool),
    Null,
    /// Field order is part of the stream under rule 2.3, so the pairs stay in a sequence. A map
    /// would discard exactly the property most of these tests exist to check.
    Object(Vec<(String, Value)>),
    Array(Vec<Value>),
}

impl Value {
    fn field(&self, key: &str) -> &Value {
        match self {
            Value::Object(pairs) => pairs
                .iter()
                .find(|(name, _)| name == key)
                .map(|(_, value)| value)
                .unwrap_or_else(|| panic!("no field {key} in {self:?}")),
            other => panic!("not an object: {other:?}"),
        }
    }

    fn keys(&self) -> Vec<&str> {
        match self {
            Value::Object(pairs) => pairs.iter().map(|(name, _)| name.as_str()).collect(),
            other => panic!("not an object: {other:?}"),
        }
    }

    fn pairs(&self) -> &[(String, Value)] {
        match self {
            Value::Object(pairs) => pairs,
            other => panic!("not an object: {other:?}"),
        }
    }

    fn items(&self) -> &[Value] {
        match self {
            Value::Array(items) => items,
            other => panic!("not an array: {other:?}"),
        }
    }

    /// The token as the text stream would render it. Every scalar in this stream is already the
    /// text stream's own rendering, which is rule 6.3 stated as a function.
    fn token(&self) -> String {
        match self {
            Value::Text(text) => text.clone(),
            Value::Number(digits) => digits.clone(),
            Value::Bool(true) => "true".to_string(),
            Value::Bool(false) => "false".to_string(),
            Value::Null => "null".to_string(),
            other => panic!("not a scalar: {other:?}"),
        }
    }

    fn integer(&self) -> u64 {
        match self {
            Value::Number(digits) => digits.parse().unwrap(),
            other => panic!("not a number: {other:?}"),
        }
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    index: usize,
    line: &'a str,
}

impl<'a> Reader<'a> {
    fn parse(line: &'a str) -> Value {
        let mut reader = Reader {
            bytes: line.as_bytes(),
            index: 0,
            line,
        };
        let value = reader.value();
        assert_eq!(
            reader.index,
            reader.bytes.len(),
            "trailing bytes in {line:?}"
        );
        value
    }

    fn peek(&self) -> u8 {
        *self
            .bytes
            .get(self.index)
            .unwrap_or_else(|| panic!("record ended early: {:?}", self.line))
    }

    fn take(&mut self, expected: u8) {
        assert_eq!(
            self.peek(),
            expected,
            "expected {:?} at byte {} of {:?}",
            expected as char,
            self.index,
            self.line
        );
        self.index += 1;
    }

    fn value(&mut self) -> Value {
        match self.peek() {
            b'{' => self.object(),
            b'[' => self.array(),
            b'"' => Value::Text(self.text()),
            b't' => {
                self.literal("true");
                Value::Bool(true)
            }
            b'f' => {
                self.literal("false");
                Value::Bool(false)
            }
            b'n' => {
                self.literal("null");
                Value::Null
            }
            _ => Value::Number(self.number()),
        }
    }

    fn literal(&mut self, word: &str) {
        assert!(
            self.line[self.index..].starts_with(word),
            "expected {word} at byte {} of {:?}",
            self.index,
            self.line
        );
        self.index += word.len();
    }

    fn object(&mut self) -> Value {
        self.take(b'{');
        let mut pairs = Vec::new();
        if self.peek() == b'}' {
            // Rule 6.7: an event with no field carries `"result":{}`.
            self.index += 1;
            return Value::Object(pairs);
        }
        loop {
            let key = self.text();
            self.take(b':');
            pairs.push((key, self.value()));
            match self.peek() {
                b',' => self.index += 1,
                _ => break,
            }
        }
        self.take(b'}');
        Value::Object(pairs)
    }

    fn array(&mut self) -> Value {
        self.take(b'[');
        let mut items = Vec::new();
        if self.peek() == b']' {
            self.index += 1;
            return Value::Array(items);
        }
        loop {
            items.push(self.value());
            match self.peek() {
                b',' => self.index += 1,
                _ => break,
            }
        }
        self.take(b']');
        Value::Array(items)
    }

    /// A quoted span. **Rule 3.3 is enforced here**, which is what makes the naive scan to the
    /// next quotation mark correct: no value may contain a quotation mark or a backslash, so
    /// there is nothing to escape and nothing to un-escape.
    fn text(&mut self) -> String {
        self.take(b'"');
        let start = self.index;
        while self.peek() != b'"' {
            let character = self.line[self.index..].chars().next().unwrap();
            assert!(
                on_alphabet(character),
                "{character:?} is off rule 3.3's closed alphabet, in {:?}",
                self.line
            );
            self.index += character.len_utf8();
        }
        let span = self.line[start..self.index].to_string();
        self.index += 1;
        span
    }

    /// A number. **Rule 4.1 is enforced here**: an integer, optionally negative, with no
    /// fractional part, no exponent, no leading zero and no leading `+`.
    fn number(&mut self) -> String {
        let start = self.index;
        if self.peek() == b'-' {
            self.index += 1;
        }
        while self.index < self.bytes.len() && self.bytes[self.index].is_ascii_digit() {
            self.index += 1;
        }
        let digits = &self.line[start..self.index];
        assert!(
            !digits.is_empty() && digits != "-",
            "empty number in {:?}",
            self.line
        );
        let magnitude = digits.trim_start_matches('-');
        assert!(
            magnitude == "0" || !magnitude.starts_with('0'),
            "leading zero in {digits} in {:?}",
            self.line
        );
        // Whatever follows must be structure, never `.`, `e`, `E` or `+`.
        if self.index < self.bytes.len() {
            let next = self.bytes[self.index];
            assert!(
                matches!(next, b',' | b'}' | b']'),
                "{:?} follows the number {digits} in {:?}",
                next as char,
                self.line
            );
        }
        digits.to_string()
    }
}

/// A whole recorded run: the text stream the process wrote to standard output, and the record
/// stream it wrote to the file, parsed.
struct Capture {
    text: String,
    records: Vec<Value>,
    stream: String,
    destination: PathBuf,
}

impl Capture {
    fn of(arguments: &[&str], label: &str) -> Self {
        let destination = scratch(label).join("records.jsonl");
        let output = invoke(arguments, Some(&destination));
        assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
        assert!(stderr(&output).is_empty(), "{}", stderr(&output));

        let bytes = fs::read(&destination).unwrap();
        // Rule 2.5 and the *Outputs* section: UTF-8, LF-terminated including the last line, no
        // BOM, no CR, no blank line. Checked on the bytes, before anything decodes them.
        assert!(!bytes.starts_with(&[0xEF, 0xBB, 0xBF]), "byte order mark");
        assert!(!bytes.contains(&b'\r'), "carriage return");
        assert!(bytes.ends_with(b"\n"), "no terminating newline");
        assert!(!bytes.windows(2).any(|pair| pair == b"\n\n"), "blank line");

        let stream = String::from_utf8(bytes).unwrap();
        let records = stream.lines().map(Reader::parse).collect();

        Self {
            text: String::from_utf8(output.stdout).unwrap(),
            records,
            stream,
            destination,
        }
    }

    fn kind(&self, index: usize) -> String {
        self.records[index].field("record").token()
    }

    fn of_kind(&self, kind: &str) -> Vec<&Value> {
        self.records
            .iter()
            .filter(|record| record.field("record").token() == kind)
            .collect()
    }
}

fn scratch(label: &str) -> PathBuf {
    let directory = std::env::temp_dir().join(format!("mokiterions-records-{label}"));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).unwrap();
    directory
}

fn invoke(arguments: &[&str], destination: Option<&Path>) -> Output {
    let mut command = Command::new(BINARY);
    command.args(arguments);
    if let Some(destination) = destination {
        command.arg("--events-path").arg(destination);
    }
    command.output().unwrap()
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).unwrap()
}

/// The reference capture for the shape tests: long enough to regenerate and to skip a
/// regeneration, with tracing on so that every event kind a shipped source can produce appears.
fn reference(label: &str) -> Capture {
    Capture::of(
        &["--seed", "42", "--ticks", "300", "--trace-actions"],
        label,
    )
}

// ---------------------------------------------------------------------------------------
// Framing, ordering and completeness
// ---------------------------------------------------------------------------------------

/// Rule 2.1, 2.2 and 9.1: one record per line, `record` first in every one, and the order
/// header, then per tick its events and then its metrics, then the run record last.
///
/// Tick 0 carries initialization events and no metrics record, because rule 7.1 counts
/// completed ticks and tick 0 is not one.
#[test]
fn the_stream_is_one_record_per_line_in_the_order_the_specification_fixes() {
    let capture = reference("order");

    assert_eq!(capture.kind(0), "header");
    assert_eq!(capture.kind(capture.records.len() - 1), "run");
    for (index, record) in capture.records.iter().enumerate() {
        assert_eq!(record.keys()[0], "record", "line {index}");
        let kind = record.field("record").token();
        assert!(
            matches!(kind.as_str(), "header" | "event" | "metrics" | "run"),
            "unknown record kind {kind}"
        );
        if index > 0 {
            assert_ne!(kind, "header", "a second header at line {index}");
        }
        if index + 1 < capture.records.len() {
            assert_ne!(kind, "run", "a run record before the end, at line {index}");
        }
    }

    // Every completed tick has exactly one metrics record, they ascend by one from tick 1, and
    // no event of a later tick precedes the metrics record of an earlier one.
    let mut completed = 0u64;
    for record in &capture.records[1..capture.records.len() - 1] {
        let tick = record.field("tick").integer();
        if record.field("record").token() == "metrics" {
            assert_eq!(tick, completed + 1, "metrics ticks must ascend by one");
            completed = tick;
        } else {
            assert!(
                tick >= completed,
                "an event of tick {tick} followed the metrics record of tick {completed}"
            );
        }
    }
    assert_eq!(completed, 300, "one metrics record per completed tick");
    assert_eq!(capture.of_kind("metrics").len(), 300);
    assert!(
        capture
            .of_kind("event")
            .iter()
            .any(|event| event.field("tick").integer() == 0),
        "tick 0 must carry its initialization events"
    );
}

/// Rule 2.4: no field the specification does not name.
///
/// Asserted as a closed key set rather than as a list of words to forbid, which is the stronger
/// statement: rule 8.7's classification, label, category, verdict and severity, and rule 8.8's
/// duration and wall-clock, are absent because *every* key not on this list is absent. The
/// forbidden vocabulary is then spelled out anyway, because a reader of this test should not
/// have to derive the prohibition from an allow-list.
#[test]
fn every_key_in_the_stream_is_a_key_the_specification_names() {
    const ALLOWED: [&str; 61] = [
        // Framing and the header
        "record",
        "schema",
        "engine",
        "config",
        "seed",
        "ticks",
        "policy",
        "density",
        "trace_actions", // Events
        "tick",
        "subject",
        "event",
        "result",
        "width",
        "height",
        "territories",
        "class",
        "position",
        "x",
        "y",
        "territory",
        "name",
        "health",
        "satiety",
        "energy",
        "fear",
        "waste_tolerance",
        "source",
        "from",
        "to",
        "food",
        "count",
        "reason",
        "proposal",
        "action",
        "direction",
        "status",
        "detail", // Metrics
        "living",
        "deaths",
        "population",
        "A",
        "B",
        "sum",
        "min",
        "max",
        "standing",
        "low",
        "medium",
        "high",
        "capacity",
        "depleted", // The run record
        "survivors",
        "crossings",
        "consumed",
        "regenerated",
        "regeneration_skipped",
        "final",
        "agents",
        "id",
        "died_at",
    ];

    fn collect<'a>(value: &'a Value, into: &mut Vec<&'a str>) {
        match value {
            Value::Object(pairs) => {
                for (key, nested) in pairs {
                    into.push(key);
                    collect(nested, into);
                }
            }
            Value::Array(items) => {
                for item in items {
                    collect(item, into);
                }
            }
            _ => {}
        }
    }

    let capture = reference("keys");
    let mut observed = Vec::new();
    for record in &capture.records {
        collect(record, &mut observed);
    }

    for key in &observed {
        assert!(ALLOWED.contains(key), "unnamed field {key}");
    }
    for key in ALLOWED {
        assert!(
            observed.contains(&key),
            "the reference capture never produced {key}, so this test does not cover it"
        );
    }

    // Rules 8.7 and 8.8, stated rather than merely implied.
    for forbidden in [
        "verdict",
        "severity",
        "classification",
        "category",
        "label",
        "grade",
        "score",
        "rating",
        "quality",
        "viable",
        "viability",
        "healthy",
        "threshold",
        "duration",
        "elapsed",
        "seconds",
        "millis",
        "timestamp",
        "clock",
        "wall",
    ] {
        assert!(
            !observed.contains(&forbidden),
            "{forbidden} appears as a key"
        );
    }
}

/// Rule 4.4: exactly two absences in the whole stream, and `null` appears nowhere else.
///
/// A `min` or `max` over an empty living population, and a survivor's `died_at`. Any other
/// `null` would be a field the engine failed to state rather than a fact that does not exist.
#[test]
fn null_appears_only_where_a_fact_does_not_exist() {
    fn nulls(value: &Value, path: &str, into: &mut Vec<String>) {
        match value {
            Value::Null => into.push(path.to_string()),
            Value::Object(pairs) => {
                for (key, nested) in pairs {
                    nulls(nested, key, into);
                }
            }
            Value::Array(items) => {
                for item in items {
                    nulls(item, path, into);
                }
            }
            _ => {}
        }
    }

    // A run that ends in extinction, so that both absences occur in one stream: an empty living
    // population's extrema, and — for the same run — no survivor, which is why a second capture
    // supplies the `died_at` absence.
    for (label, arguments) in [
        (
            "nulls-extinct",
            [
                "--seed",
                "3",
                "--ticks",
                "400",
                "--policy",
                "baseline",
                "--density",
                "0.15",
            ],
        ),
        (
            "nulls-survived",
            [
                "--seed",
                "42",
                "--ticks",
                "200",
                "--policy",
                "reference",
                "--density",
                "0.75",
            ],
        ),
    ] {
        let capture = Capture::of(&arguments, label);
        let mut absences = Vec::new();
        for record in &capture.records {
            nulls(record, "", &mut absences);
        }
        for absence in &absences {
            assert!(
                matches!(absence.as_str(), "min" | "max" | "died_at"),
                "an unexplained null at {absence} in {label}"
            );
        }
    }
}

/// Rules 3.2 and 5.5: no record carries a filesystem path, and no record carries the sink's own
/// destination.
///
/// Rule 3.3's alphabet already excludes both separators, so a path cannot appear as a value at
/// all and the reader above would have refused the stream. What this adds is the destination's
/// own name — `records.jsonl` and the directory holding it are spelled entirely on the alphabet,
/// so nothing but this test stands between a helpful future field and a record stream that
/// states where it was written.
#[test]
fn no_record_carries_a_path_or_the_sinks_own_destination() {
    let capture = reference("paths");

    for separator in ['/', '\\'] {
        assert!(
            !capture.stream.contains(separator),
            "a path separator reached the stream"
        );
    }

    let destination = capture.destination.to_str().unwrap();
    assert!(!capture.stream.contains(destination));
    for component in capture.destination.components() {
        let component = component.as_os_str().to_str().unwrap();
        assert!(
            !capture.stream.contains(component),
            "the destination component {component} reached the stream"
        );
    }
}

// ---------------------------------------------------------------------------------------
// Correspondence with the text stream
// ---------------------------------------------------------------------------------------

/// The text `result=` rendering, reconstructed from an event record's `result` object.
///
/// This is rule 6.6's walk, written out: each field in order, `key:value`, joined by `,`; a
/// coordinate object as `x:y`; a transition object as `from->to`; a proposal object as its
/// action word followed by a colon and its one remaining value where it has one; `status` as
/// its own token, since the text stream states the verdict as a bare word.
fn render_result(result: &Value) -> String {
    let mut rendered = Vec::new();
    for (key, value) in result.pairs() {
        let text = match value {
            Value::Object(pairs) => {
                let keys: Vec<&str> = pairs.iter().map(|(name, _)| name.as_str()).collect();
                match keys.as_slice() {
                    ["x", "y"] => format!("{}:{}", pairs[0].1.token(), pairs[1].1.token()),
                    ["from", "to"] => format!("{}->{}", pairs[0].1.token(), pairs[1].1.token()),
                    ["action"] => pairs[0].1.token(),
                    ["action", _] => format!("{}:{}", pairs[0].1.token(), pairs[1].1.token()),
                    other => panic!("unknown composite shape {other:?}"),
                }
            }
            scalar => scalar.token(),
        };
        rendered.push(format!("{key}:{text}"));
    }
    rendered.join(",")
}

/// `REQ-MOK-042` and rules 6.2, 6.6 and 9.3: the text stream is reconstructible from the record
/// stream, line for line, in order, with nothing left over on either side.
///
/// This is the strongest statement of the correspondence available from outside the engine. It
/// does not compare counts or sample a few lines: every event record is turned back into the
/// text line it accompanies and the two are compared as bytes, so a field in the wrong order, a
/// value rendered differently, a record for an event that was not emitted, or an event with no
/// record all fail here.
#[test]
fn every_text_event_line_is_reconstructible_from_its_event_record() {
    let capture = reference("reconstruct");

    let text_events: Vec<&str> = capture
        .text
        .lines()
        .filter(|line| line.starts_with("tick="))
        .collect();
    let records = capture.of_kind("event");
    assert_eq!(text_events.len(), records.len(), "one record per text line");
    assert!(text_events.len() > 3_000, "the capture must be substantial");

    for (line, record) in text_events.iter().zip(&records) {
        let reconstructed = format!(
            "tick={} subject={} event={} result={}",
            record.field("tick").token(),
            record.field("subject").token(),
            record.field("event").token(),
            render_result(record.field("result"))
        );
        assert_eq!(*line, reconstructed);
    }

    // Rule 9.3: the summary line and the run record are the pair that closes both streams, and
    // the text stream's last line is that summary line.
    assert!(
        capture
            .text
            .lines()
            .last()
            .unwrap()
            .starts_with("summary reason="),
        "the text stream must end with its summary line"
    );
}

/// Rule 8.3: the run record carries exactly the twelve figures the text summary line carries.
#[test]
fn the_run_record_reconstructs_the_summary_line() {
    let capture = reference("summary");
    let run = capture.records.last().unwrap();
    let territories = run.field("final").field("territories");
    let a = territories.field("A");
    let b = territories.field("B");

    let reconstructed = format!(
        "summary reason={} ticks={} survivors={} deaths={} territory_a={} territory_b={} \
         food_a_low={} food_a_medium={} food_a_high={} food_b_low={} food_b_medium={} \
         food_b_high={}",
        run.field("reason").token(),
        run.field("ticks").token(),
        run.field("survivors").token(),
        run.field("deaths").token(),
        a.field("population").token(),
        b.field("population").token(),
        a.field("low").token(),
        a.field("medium").token(),
        a.field("high").token(),
        b.field("low").token(),
        b.field("medium").token(),
        b.field("high").token(),
    );

    assert_eq!(capture.text.lines().last().unwrap(), reconstructed);
}

/// Rule 8.6: every cumulative figure equals the number of corresponding event records in the
/// same stream, and `survivors` and `deaths` equal the final metrics record's own figures.
///
/// A consistency property of the stream, so a disagreement is a defect the stream reveals
/// without reference to anything outside it.
#[test]
fn every_cumulative_figure_equals_its_event_record_count() {
    let capture = reference("cumulative");
    let events = capture.of_kind("event");
    let run = capture.records.last().unwrap();

    let counting = |kind: &str| {
        events
            .iter()
            .filter(|event| event.field("event").token() == kind)
            .count() as u64
    };
    let counting_field = |kind: &str, field: &str, value: &str| {
        events
            .iter()
            .filter(|event| {
                event.field("event").token() == kind
                    && event.field("result").field(field).token() == value
            })
            .count() as u64
    };

    assert_eq!(
        run.field("crossings").integer(),
        counting("territory_crossed")
    );
    for class in ["low", "medium", "high"] {
        assert_eq!(
            run.field("consumed").field(class).integer(),
            counting_field("food_consumed", "class", class),
            "consumed {class}"
        );
    }
    assert_eq!(
        run.field("regenerated").integer(),
        counting("food_regenerated")
    );
    for reason in ["depleted", "capacity"] {
        assert_eq!(
            run.field("regeneration_skipped").field(reason).integer(),
            counting_field("food_regeneration_skipped", "reason", reason),
            "regeneration_skipped {reason}"
        );
    }

    let final_metrics = *capture.of_kind("metrics").last().unwrap();
    assert_eq!(
        run.field("survivors").integer(),
        final_metrics.field("living").integer()
    );
    assert_eq!(
        run.field("deaths").integer(),
        final_metrics.field("deaths").integer()
    );

    // Rule 8.4: one entry per Mokiterion the run created, in ascending identifier order.
    let agents = run.field("agents").items();
    assert_eq!(agents.len(), 12);
    let identifiers: Vec<String> = agents
        .iter()
        .map(|agent| agent.field("id").token())
        .collect();
    let mut ascending = identifiers.clone();
    ascending.sort();
    assert_eq!(identifiers, ascending);
    assert_eq!(
        agents
            .iter()
            .filter(|agent| *agent.field("died_at") == Value::Null)
            .count() as u64,
        run.field("survivors").integer()
    );
}

/// Rule 7.4 and 7.5: each metrics record's own figures agree with each other and with the
/// events that produced them.
#[test]
fn every_metrics_record_is_internally_consistent() {
    let capture = reference("metrics");
    for record in capture.of_kind("metrics") {
        let living = record.field("living").integer();
        let deaths = record.field("deaths").integer();
        assert_eq!(living + deaths, 12, "rule 7.4: the two sum to the roster");
        assert_eq!(
            record.field("population").field("A").integer()
                + record.field("population").field("B").integer(),
            living
        );

        for (attribute, extremum) in [
            ("health", "min"),
            ("satiety", "min"),
            ("energy", "min"),
            ("fear", "max"),
        ] {
            let sum = record.field(attribute).field("sum").integer();
            let bound = record.field(attribute).field(extremum);
            if living == 0 {
                assert_eq!(sum, 0, "{attribute}");
                assert_eq!(*bound, Value::Null, "{attribute}");
            } else {
                // Rule 4.2 forbids the mean, so the sum is checked against the bound it must
                // respect rather than divided by the count beside it.
                let bound = bound.integer();
                if extremum == "min" {
                    assert!(sum >= bound * living, "{attribute}");
                } else {
                    assert!(sum <= bound * living, "{attribute}");
                }
            }
        }

        for territory in ["A", "B"] {
            let state = record.field("territories").field(territory);
            let standing = state.field("standing").integer();
            assert_eq!(
                standing,
                state.field("low").integer()
                    + state.field("medium").integer()
                    + state.field("high").integer(),
                "{territory}"
            );
            assert!(standing <= state.field("capacity").integer(), "{territory}");
            assert_eq!(
                *state.field("depleted"),
                Value::Bool(standing == 0),
                "{territory}"
            );
        }
    }
}

/// Rule 11.1: the text stream's bytes are identical with and without a sink, with no tolerance.
///
/// Every declared policy and tracing both ways, at the process boundary rather than at the
/// library's, so that the host's own additions — opening a file, buffering it, flushing it —
/// are inside the comparison.
#[test]
fn the_text_stream_is_byte_identical_with_and_without_a_sink() {
    for policy in ["baseline", "reference", "individual"] {
        for trace in [false, true] {
            let mut arguments = vec!["--seed", "42", "--ticks", "150", "--policy", policy];
            if trace {
                arguments.push("--trace-actions");
            }

            let without = invoke(&arguments, None);
            let destination = scratch(&format!("identity-{policy}-{trace}")).join("records.jsonl");
            let with = invoke(&arguments, Some(&destination));

            assert_eq!(without.status.code(), Some(0));
            assert_eq!(with.status.code(), Some(0));
            assert_eq!(without.stdout, with.stdout, "{policy} trace={trace}");
            assert!(without.stderr.is_empty() && with.stderr.is_empty());
            assert!(fs::metadata(&destination).unwrap().len() > 0);
        }
    }
}

/// The record stream is reproducible for a given configuration, exactly as the text stream is.
#[test]
fn the_record_stream_is_byte_identical_across_runs() {
    let arguments = ["--seed", "777", "--ticks", "120", "--policy", "individual"];
    let first = scratch("reproducible-first").join("records.jsonl");
    let second = scratch("reproducible-second").join("records.jsonl");

    assert_eq!(invoke(&arguments, Some(&first)).status.code(), Some(0));
    assert_eq!(invoke(&arguments, Some(&second)).status.code(), Some(0));
    assert_eq!(fs::read(&first).unwrap(), fs::read(&second).unwrap());
}

// ---------------------------------------------------------------------------------------
// The host's obligations: the option, the destination, and what survives a failure
// ---------------------------------------------------------------------------------------

/// Rule 13.1: a malformed sink argument is an invalid configuration. Exit `2`, the usage text on
/// the diagnostic stream, nothing run, and no file anywhere.
#[test]
fn a_malformed_sink_argument_exits_two_and_writes_nothing() {
    let directory = scratch("malformed");
    let existing = directory.join("records.jsonl");

    for arguments in [
        vec!["--events-path"],
        vec!["--events-path", "--seed"],
        vec!["--events-path", "-"],
        vec!["--events-path", ""],
        vec![
            "--events-path",
            existing.to_str().unwrap(),
            "--events-path",
            existing.to_str().unwrap(),
        ],
    ] {
        let output = invoke(&arguments, None);
        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
        assert!(
            stderr(&output).contains("Usage: Mokiterions"),
            "{arguments:?}"
        );
        assert!(!fs::exists(&existing).unwrap_or(false), "{arguments:?}");
    }
}

/// Rule 13.2: a well-formed path the platform refuses is a runtime failure. Exit `1`, the
/// reason reported, no tick run and no text observation record — a run that cannot be recorded
/// is not run.
#[test]
fn an_unopenable_destination_exits_one_and_runs_no_tick() {
    let destination = scratch("unopenable").join("absent").join("records.jsonl");
    let output = invoke(&["--seed", "0", "--ticks", "50"], Some(&destination));

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty(), "no text record may be written");
    // Rule 13.5: distinguishable from a text-stream failure, deterministic in form, and naming
    // the sink and the platform's reason and nothing else.
    let diagnostic = stderr(&output);
    assert!(
        diagnostic.starts_with("runtime error: record sink "),
        "{diagnostic}"
    );
    assert!(diagnostic.contains("records.jsonl"), "{diagnostic}");
    assert!(!fs::exists(&destination).unwrap_or(false));
}

/// `--help` resolves no destination and opens no file: rule 13.1's "runs nothing" extends to
/// the filesystem, and a host that opened the file first would leave one behind.
#[test]
fn help_opens_no_file() {
    let destination = scratch("help").join("records.jsonl");
    let output = invoke(&["--help"], Some(&destination));

    assert_eq!(output.status.code(), Some(0));
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .starts_with("Usage: Mokiterions")
    );
    assert!(!fs::exists(&destination).unwrap_or(false));
}

/// The option replaces a file already at the destination, holds only the new run's records, and
/// reports no failure.
#[test]
fn an_existing_destination_is_replaced_by_the_new_run() {
    let destination = scratch("overwrite").join("records.jsonl");
    fs::write(
        &destination,
        "stale content that must not survive\n".repeat(500),
    )
    .unwrap();
    let stale = fs::metadata(&destination).unwrap().len();

    let output = invoke(&["--seed", "1", "--ticks", "30"], Some(&destination));

    assert_eq!(output.status.code(), Some(0));
    assert!(stderr(&output).is_empty(), "{}", stderr(&output));
    let stream = fs::read_to_string(&destination).unwrap();
    assert!(!stream.contains("stale content"));
    assert!(stream.starts_with("{\"record\":\"header\""));
    assert!(
        stream
            .lines()
            .last()
            .unwrap()
            .starts_with("{\"record\":\"run\"")
    );
    assert_ne!(stream.len() as u64, stale);
}

/// A run whose standard output cannot be written: the child's end of the pipe is closed before
/// it writes, so the text stream fails partway through.
///
/// The exit code is `1` under `SPEC-MOK-001`, and rule 13.4 then applies to the destination:
/// a stream with no run record must not survive to be read as a complete run. `--ticks 1000`
/// puts more than a megabyte through the pipe, so the failure is certain rather than a race.
fn a_run_whose_text_stream_fails(destination: &Path) -> Output {
    let mut child = Command::new(BINARY)
        .args(["--seed", "0", "--ticks", "1000", "--trace-actions"])
        .arg("--events-path")
        .arg(destination)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    drop(child.stdout.take());
    child.wait_with_output().unwrap()
}

/// Rule 13.4: on a failure the binary removes the file it created.
#[test]
fn a_failed_run_removes_the_destination_it_created() {
    let destination = scratch("removal").join("records.jsonl");
    let output = a_run_whose_text_stream_fails(&destination);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        !fs::exists(&destination).unwrap_or(false),
        "a partial stream survived: {}",
        stderr(&output)
    );
}

/// Rule 13.4's bound: removal is limited to a destination this process created, and where that
/// cannot be established the process does not remove it and says so.
///
/// A program that deletes an operator's file on a write error is a worse outcome than a partial
/// stream, so the file stays and the diagnostic explains why.
#[test]
fn a_destination_the_process_did_not_create_is_not_removed() {
    let destination = scratch("not-removed").join("records.jsonl");
    fs::write(&destination, "an operator's file\n").unwrap();

    let output = a_run_whose_text_stream_fails(&destination);

    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    assert!(
        fs::exists(&destination).unwrap_or(false),
        "{}",
        stderr(&output)
    );
    let diagnostic = stderr(&output);
    assert!(diagnostic.contains("not removed"), "{diagnostic}");
    assert!(
        diagnostic.contains("did not create the destination"),
        "{diagnostic}"
    );
    // Rule 13.6: no new exit code, and the removal decision does not change the one in force.
    assert_eq!(output.status.code(), Some(1));
}

/// Rule 13.5: a sink failure is distinguishable from a text-stream failure, is deterministic in
/// form, and states the sink's identity and the platform's reason and nothing else.
#[test]
fn a_sink_diagnostic_is_distinguishable_from_a_text_stream_diagnostic() {
    let destination = scratch("diagnostic").join("absent").join("records.jsonl");
    let sink_failure = stderr(&invoke(&["--ticks", "10"], Some(&destination)));

    let text_destination = scratch("diagnostic-text").join("records.jsonl");
    let text_failure = stderr(&a_run_whose_text_stream_fails(&text_destination));

    // Both are runtime failures; only one names the sink.
    assert!(
        sink_failure.starts_with("runtime error: record sink "),
        "{sink_failure}"
    );
    assert!(
        text_failure.starts_with("runtime error: "),
        "{text_failure}"
    );
    assert!(!text_failure.contains("record sink"), "{text_failure}");

    // Deterministic in form: the same failure reported twice reads the same way.
    assert_eq!(
        sink_failure,
        stderr(&invoke(&["--ticks", "10"], Some(&destination)))
    );
    // One diagnostic each, one line each, so nothing accumulates per tick or per record.
    assert_eq!(sink_failure.lines().count(), 1, "{sink_failure}");
    assert_eq!(text_failure.lines().count(), 1, "{text_failure}");
}
