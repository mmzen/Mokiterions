//! `SPEC-MOK-002` rule 9.4 to 9.6: the export.
//!
//! The export is the observer's only retainable artifact and the only observer output
//! admissible as evidence. It therefore carries authoritative records only, in authoritative
//! order, in the `SPEC-MOK-001` line format, and it ignores any active filter.

use std::fs;
use std::io::{self, BufWriter, Write};

use crate::state::EventBuffer;

/// The default path, resolved when `--export` was not supplied. It is relative to the working
/// directory, so no absolute path enters the file name or the frame.
pub fn default_path(seed: u64, tick: u64) -> String {
    format!("mokiterions-events-seed{seed}-ticks{tick}.log")
}

/// Writes every retained record and the closing statement.
///
/// The content is a function of the retained records alone: no wall-clock timestamp, no path,
/// no environment-specific value and no credential. Two runs sharing seed, configuration,
/// decision source and stopping tick therefore produce byte-identical files.
pub fn write_records<W: Write>(writer: &mut W, events: &EventBuffer) -> io::Result<()> {
    for event in events.iter() {
        writeln!(writer, "{event}")?;
    }
    writeln!(
        writer,
        "# retained={} truncated={}",
        events.len(),
        events.truncated()
    )
}

/// Writes the export to `path`.
///
/// A partial file is removed if it can be, and only when this call created it, so a failure
/// never presents an incomplete export as a complete one.
pub fn write_file(path: &str, events: &EventBuffer) -> Result<(), String> {
    let file = fs::File::create(path).map_err(|error| error.to_string())?;
    let mut writer = BufWriter::new(file);
    match write_records(&mut writer, events).and_then(|()| writer.flush()) {
        Ok(()) => Ok(()),
        Err(error) => {
            drop(writer);
            let _ = fs::remove_file(path);
            Err(error.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mokiterions_core::simulation::{Event, EventDetail, TerminationReason};

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
            "tick=4 subject=M03 event=survival_changed result=health:100->100,satiety:97->96,energy:97->96"
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

        for _ in 0..crate::state::EVENT_CAPACITY {
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
                crate::state::EVENT_CAPACITY
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
}
