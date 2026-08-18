//! `SPEC-MOK-003` rule 9.4 to 9.6: the export.
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
