use std::io::{self};
use std::fs;
use pyo3::prelude::*;

use subparse::{get_subtitle_format, parse_bytes};
use subparse;


// Similar to subparse::SubtitleEntry, but flattens out the time
// entries into milliseconds
#[pyclass(get_all)]
struct SubtitleEntry {
    start: i64,
    end: i64,
    line: Option<String>
}

impl From<subparse::SubtitleEntry> for SubtitleEntry {
    fn from(subtitle_entry: subparse::SubtitleEntry) -> Self {
        SubtitleEntry {
            start: subtitle_entry.timespan.start.msecs(),
            end: subtitle_entry.timespan.end.msecs(),
            line: match subtitle_entry.line {
                Some(line) => Some(line.clone()),
                None => None
            }
        }
    }
}


#[pyfunction]
fn parse_subtitle(input_path: &str) -> Result<Vec<SubtitleEntry>, io::Error> {
    let file_path = std::path::Path::new(input_path);
    let file_contents: Vec<u8> = fs::read(input_path)?;

    // TODO Prefer explicit error handling over unwrap here
    let subtitle_format = get_subtitle_format(
        file_path.extension(),
        &file_contents
    ).unwrap();

    let entries = parse_bytes(
        subtitle_format, 
        &file_contents,
        None,
        24.0   // TODO make this a parameter
    ).unwrap().get_subtitle_entries().unwrap();


    // I couldn't get map().collect() to work
    let mut py_entries = vec![];

    for entry in entries {
        py_entries.push(
            SubtitleEntry::from(entry)
        )
    }

    Ok(py_entries)
}

/// A Python module implemented in Rust.
#[pymodule]
fn subparsepy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_subtitle, m)?)?;
    m.add_class::<SubtitleEntry>()?;
    Ok(())
}