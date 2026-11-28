//! `evens [NUMBERS]...`: print how many of the integers given are even.

use std::io::Write;

use anyhow::Result;

/// The arguments of `evens`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to count the even ones of.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// How many of `numbers` are even, which is 0 for none. Negative numbers count
/// by their evenness too, so `-4` is even.
///
/// # Examples
///
/// ```
/// use vergil_fixture_rust_app::commands::evens::count_even;
///
/// assert_eq!(count_even(&[2, -4, 7, 0]), 3);
/// assert_eq!(count_even(&[]), 0);
/// ```
#[must_use]
pub fn count_even(numbers: &[i64]) -> usize {
    numbers.iter().filter(|&&n| n % 2 == 0).count()
}

/// Writes how many of `args.numbers` are even to `out`.
///
/// # Errors
///
/// Fails when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", count_even(&args.numbers))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::testing::capture;

    #[test]
    fn counts_the_even_numbers() {
        assert_eq!(count_even(&[2, 9, 4]), 2);
    }

    #[test]
    fn counts_negative_even_numbers() {
        assert_eq!(count_even(&[-4, 7, -3, 0]), 2);
    }

    #[test]
    fn tells_the_bounds_apart() {
        assert_eq!(count_even(&[i64::MIN, i64::MAX, 0]), 2);
    }

    #[test]
    fn no_numbers_count_as_zero() {
        assert_eq!(count_even(&[]), 0);
    }

    #[test]
    fn run_writes_one_line() {
        let args = Args {
            numbers: vec![40, 3, 2],
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
            numbers: vec![40, 3, 2],
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
