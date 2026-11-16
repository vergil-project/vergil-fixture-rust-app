//! `greet NAME`: print a greeting.

use std::io::Write;

use anyhow::{Result, bail};

/// The arguments of `greet`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Who to greet.
    pub name: String,
}

/// The greeting for `name`.
///
/// # Errors
///
/// Fails when `name` is empty or only whitespace.
pub fn greeting(name: &str) -> Result<String> {
    if name.trim().is_empty() {
        bail!("the name must not be blank");
    }
    Ok(format!("Hello, {name}"))
}

/// Writes the greeting for `args.name` to `out`.
///
/// # Errors
///
/// Fails as [`greeting`] does, or when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", greeting(&args.name)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::testing::capture;

    #[test]
    fn greets_by_name() {
        assert_eq!(greeting("Ada").unwrap(), "Hello, Ada");
    }

    #[test]
    fn keeps_the_name_as_given() {
        assert_eq!(greeting("Grace Hopper").unwrap(), "Hello, Grace Hopper");
    }

    #[test]
    fn rejects_an_empty_name() {
        let err = greeting("").unwrap_err();
        assert_eq!(err.to_string(), "the name must not be blank");
    }

    #[test]
    fn rejects_a_whitespace_name() {
        let err = greeting(" \t ").unwrap_err();
        assert_eq!(err.to_string(), "the name must not be blank");
    }

    #[test]
    fn run_writes_one_line() {
        let args = Args {
            name: "Ada".to_owned(),
        };
        let (result, out) = capture(64, |out| run(&args, out));
        result.unwrap();
        assert_eq!(out, b"Hello, Ada\n");
    }

    #[test]
    fn run_writes_nothing_on_error() {
        let args = Args {
            name: String::new(),
        };
        let (result, out) = capture(64, |out| run(&args, out));
        assert_eq!(result.unwrap_err().to_string(), "the name must not be blank");
        assert!(out.is_empty());
    }

    #[test]
    fn run_reports_a_write_error() {
        let args = Args {
            name: "Ada".to_owned(),
        };
        let (result, out) = capture(0, |out| run(&args, out));
        let kind = result
            .unwrap_err()
            .downcast_ref::<std::io::Error>()
            .map(std::io::Error::kind);
        assert_eq!(kind, Some(std::io::ErrorKind::WriteZero));
        assert!(out.is_empty());
    }
}
