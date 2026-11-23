//! `distinct [NUMBERS]...`: print how many different integers were given.

use std::collections::BTreeSet;
use std::io::Write;

use anyhow::Result;

/// The arguments of `distinct`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to tell apart.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// How many different integers `numbers` holds, counting each repeated value
/// once, which is 0 for none.
///
/// # Examples
///
/// ```
/// use vergil_fixture_rust_app::commands::distinct::unique;
///
/// assert_eq!(unique(&[3, -7, 3, 5]), 3);
/// assert_eq!(unique(&[]), 0);
/// ```
#[must_use]
pub fn unique(numbers: &[i64]) -> usize {
    numbers.iter().collect::<BTreeSet<_>>().len()
}

/// Writes how many different integers `args.numbers` holds to `out`.
///
/// # Errors
///
/// Fails when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", unique(&args.numbers))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::testing::capture;

    #[test]
    fn counts_different_numbers() {
        assert_eq!(unique(&[2, 9, 3]), 3);
    }

    #[test]
    fn counts_a_repeated_number_once() {
        assert_eq!(unique(&[4, 4, 4]), 1);
        assert_eq!(unique(&[-4, 4, -4, 0, 4]), 3);
    }

    #[test]
    fn tells_the_bounds_apart() {
        assert_eq!(unique(&[i64::MIN, i64::MAX, i64::MIN, 0]), 3);
    }

    #[test]
    fn no_numbers_count_as_zero() {
        assert_eq!(unique(&[]), 0);
    }

    #[test]
    fn run_writes_one_line() {
        let args = Args {
            numbers: vec![40, 2, 40],
        };
        let (result, out) = capture(64, |out| run(&args, out));
        result.unwrap();
        assert_eq!(out, b"2\n");
    }

    #[test]
    fn run_writes_zero_for_no_numbers() {
        let args = Args { numbers: vec![] };
        let (result, out) = capture(64, |out| run(&args, out));
        result.unwrap();
        assert_eq!(out, b"0\n");
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
