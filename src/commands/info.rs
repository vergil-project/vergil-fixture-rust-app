//! `info`: print the program's name and version as JSON.

use std::io::Write;

use anyhow::Result;
use serde_json::{Value, json};

/// The program's name and version, as a JSON object.
#[must_use]
pub fn document() -> Value {
    json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
    })
}

/// Writes [`document`] to `out` as one line of compact JSON.
///
/// # Errors
///
/// Fails when writing to `out` fails.
pub fn run(out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", document())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_the_program_and_its_version() {
        let doc = document();
        assert_eq!(doc["name"], "vergil-fixture-rust-app");
        assert_eq!(doc["version"], env!("CARGO_PKG_VERSION"));
        assert_eq!(doc.as_object().map(serde_json::Map::len), Some(2));
    }

    #[test]
    fn run_writes_compact_json_on_one_line() {
        let mut out = Vec::new();
        run(&mut out).unwrap();
        let expected = format!(
            "{{\"name\":\"vergil-fixture-rust-app\",\"version\":\"{}\"}}\n",
            env!("CARGO_PKG_VERSION")
        );
        assert_eq!(String::from_utf8(out).unwrap(), expected);
    }
}
