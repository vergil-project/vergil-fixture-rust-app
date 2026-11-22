//! `count [NUMBERS]...`: print how many integers were given.

use std::io::Write;

use anyhow::Result;

/// The arguments of `count`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to count.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// How many integers `numbers` holds, which is 0 for none.
///
/// # Examples
///
/// ```
/// use vergil_fixture_rust_app::commands::count::tally;
///
/// assert_eq!(tally(&[3, -7, 5]), 3);
/// assert_eq!(tally(&[]), 0);
/// ```
#[must_use]
pub fn tally(numbers: &[i64]) -> usize {
    numbers.len()
}

/// Writes how many integers `args.numbers` holds to `out`.
///
/// # Errors
///
/// Fails when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", tally(&args.numbers))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::testing::capture;

    #[test]
    fn counts_its_numbers() {
        assert_eq!(tally(&[2, 9, 3]), 3);
    }

    #[test]
    fn counts_repeated_and_negative_numbers() {
        assert_eq!(tally(&[-4, -4, 0, i64::MIN, i64::MAX]), 5);
    }

    #[test]
    fn no_numbers_count_as_zero() {
        assert_eq!(tally(&[]), 0);
    }

    #[test]
    fn run_writes_one_line() {
        let args = Args {
            numbers: vec![40, 2, 42],
        };
        let (result, out) = capture(64, |out| run(&args, out));
        result.unwrap();
        assert_eq!(out, b"3\n");
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
            numbers: vec![40, 2, 42],
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
