//! `mode NUMBERS...`: print the integer given most often, the smallest of them
//! on a tie.

use std::collections::BTreeMap;
use std::io::Write;

use anyhow::{Context, Result};

/// The arguments of `mode`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to search.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// The integer `numbers` holds most often. When several are given equally
/// often, it is the smallest of them.
///
/// # Examples
///
/// ```
/// use vergil_fixture_rust_app::commands::mode::most_common;
///
/// assert_eq!(most_common(&[3, -7, 3, 5]).unwrap(), 3);
/// assert_eq!(most_common(&[5, -7]).unwrap(), -7);
/// assert!(most_common(&[]).is_err());
/// ```
///
/// # Errors
///
/// Fails when `numbers` is empty, since no numbers have a mode.
pub fn most_common(numbers: &[i64]) -> Result<i64> {
    let mut counts = BTreeMap::new();
    for &n in numbers {
        *counts.entry(n).or_insert(0_usize) += 1;
    }
    // The map iterates in ascending order and `max_by_key` keeps the last of
    // equal keys, so iterating in reverse makes a tie go to the smallest.
    counts
        .into_iter()
        .rev()
        .max_by_key(|&(_, count)| count)
        .map(|(n, _)| n)
        .context("mode needs at least one number")
}

/// Writes the mode of `args.numbers` to `out`.
///
/// # Errors
///
/// Fails as [`most_common`] does, or when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", most_common(&args.numbers)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::testing::capture;

    const NONE: &str = "mode needs at least one number";

    #[test]
    fn finds_the_most_common_number() {
        assert_eq!(most_common(&[2, 9, 9, 3]).unwrap(), 9);
    }

    #[test]
    fn finds_the_most_common_negative_number() {
        assert_eq!(most_common(&[-4, -2, -4, 7]).unwrap(), -4);
    }

    #[test]
    fn one_number_is_its_own_mode() {
        assert_eq!(most_common(&[7]).unwrap(), 7);
    }

    #[test]
    fn a_tie_goes_to_the_smallest() {
        assert_eq!(most_common(&[9, 2, 9, 2, 5]).unwrap(), 2);
        assert_eq!(most_common(&[3, 1, 2]).unwrap(), 1);
        assert_eq!(most_common(&[4, -4]).unwrap(), -4);
    }

    #[test]
    fn handles_the_bounds() {
        assert_eq!(most_common(&[i64::MAX, i64::MIN, i64::MAX]).unwrap(), i64::MAX);
        assert_eq!(most_common(&[i64::MAX, i64::MIN]).unwrap(), i64::MIN);
    }

    #[test]
    fn rejects_no_numbers() {
        assert_eq!(most_common(&[]).unwrap_err().to_string(), NONE);
    }

    #[test]
    fn run_writes_one_line() {
        let args = Args {
            numbers: vec![40, 2, 40],
        };
        let (result, out) = capture(64, |out| run(&args, out));
        result.unwrap();
        assert_eq!(out, b"40\n");
    }

    #[test]
    fn run_writes_nothing_on_error() {
        let args = Args { numbers: vec![] };
        let (result, out) = capture(64, |out| run(&args, out));
        assert_eq!(result.unwrap_err().to_string(), NONE);
        assert!(out.is_empty());
    }

    #[test]
    fn run_reports_a_write_error() {
        let args = Args {
            numbers: vec![40, 2, 40],
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
