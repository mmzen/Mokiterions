//! Public tier: export.
//!
//! `SPEC-MOK-004` rule 9 places these tests here. Every one of them was in
//! `src/export.rs`'s `#[cfg(test)] mod tests` block and reached the code through items that were
//! already public, so the move changes the path and nothing else: the assertions are verbatim and
//! no item was widened to bring them out. `SPEC-MOK-004` rule 12 is the obligation and the
//! per-test comparison under `WO-MOK-006` is the evidence.

use mokiterions::simulation::{Event, EventDetail, TerminationReason};
use mokiterions_tui::export::*;
use mokiterions_tui::state::EventBuffer;
use std::fs;

fn buffer() -> EventBuffer {
    let mut buffer = EventBuffer::new();
    buffer.push(Event {
        tick: 0,
        subject: "world".to_string(),
        detail: EventDetail::WorldInitialized {
            width: 128,
            height: 128,
            territories: 2,
        },
    });
    buffer.push(Event {
        tick: 4,
        subject: "M03".to_string(),
        detail: EventDetail::SurvivalChanged {
            health: (100, 100),
            satiety: (97, 96),
            energy: (97, 96),
            fear: (12, 22),
        },
    });
    buffer.push(Event {
        tick: 4,
        subject: "world".to_string(),
        detail: EventDetail::SimulationEnded {
            reason: TerminationReason::TickLimit,
        },
    });
    buffer
}

fn rendered(events: &EventBuffer) -> String {
    let mut bytes = Vec::new();
    write_records(&mut bytes, events).unwrap();
    String::from_utf8(bytes).unwrap()
}

#[test]
fn records_use_the_engines_own_line_format_in_authoritative_order() {
    let text = rendered(&buffer());
    let lines: Vec<&str> = text.lines().collect();

    assert_eq!(
        lines[0],
        "tick=0 subject=world event=world_initialized result=width:128,height:128,territories:2"
    );
    assert_eq!(
        lines[1],
        "tick=4 subject=M03 event=survival_changed result=health:100->100,satiety:97->96,energy:97->96,fear:12->22"
    );
    assert_eq!(
        lines[2],
        "tick=4 subject=world event=simulation_ended result=reason:tick_limit"
    );
    assert_eq!(lines[3], "# retained=3 truncated=false");
    assert_eq!(lines.len(), 4);
}

#[test]
fn the_closing_line_states_the_count_and_the_truncation() {
    let mut events = buffer();
    assert!(rendered(&events).ends_with("# retained=3 truncated=false\n"));

    for _ in 0..mokiterions_tui::state::EVENT_CAPACITY {
        events.push(Event {
            tick: 9,
            subject: "world".to_string(),
            detail: EventDetail::SimulationEnded {
                reason: TerminationReason::Extinction,
            },
        });
    }
    let text = rendered(&events);
    assert!(
        text.ends_with(&format!(
            "# retained={} truncated=true\n",
            mokiterions_tui::state::EVENT_CAPACITY
        )),
        "{}",
        text.lines().next_back().unwrap()
    );
}

#[test]
fn the_same_records_always_produce_the_same_bytes() {
    assert_eq!(rendered(&buffer()), rendered(&buffer()));
}

#[test]
fn nothing_environment_specific_reaches_the_file() {
    let text = rendered(&buffer());
    for forbidden in ["C:\\", "/home/", "/Users/", "AppData", "PATH=", "token"] {
        assert!(!text.contains(forbidden), "{forbidden}");
    }
    // Every line is either a record or the single closing statement.
    for line in text.lines() {
        assert!(
            line.starts_with("tick=") || line == "# retained=3 truncated=false",
            "{line}"
        );
    }
}

#[test]
fn the_default_path_is_relative_and_derived_from_the_run() {
    let path = default_path(42, 137);
    assert_eq!(path, "mokiterions-events-seed42-ticks137.log");
    assert!(!path.contains('/') && !path.contains('\\'));
}

#[test]
fn a_written_file_holds_exactly_the_rendered_records() {
    let directory = std::env::temp_dir().join("mokiterions-export-test");
    fs::create_dir_all(&directory).unwrap();
    let path = directory.join("events.log");
    let path = path.to_str().unwrap();

    write_file(path, &buffer()).unwrap();
    assert_eq!(fs::read_to_string(path).unwrap(), rendered(&buffer()));
    fs::remove_file(path).unwrap();
}

#[test]
fn an_unwritable_path_is_reported_and_leaves_nothing_behind() {
    let directory = std::env::temp_dir().join("mokiterions-export-missing");
    let _ = fs::remove_dir_all(&directory);
    let path = directory.join("nested").join("events.log");
    let path = path.to_str().unwrap().to_string();

    assert!(write_file(&path, &buffer()).is_err());
    assert!(!fs::exists(&path).unwrap_or(false));
}
