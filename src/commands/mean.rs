//! `mean NUMBERS...`: print the mean of some integers, rounded toward zero.

use std::io::Write;

use anyhow::{Context, Result};

/// The arguments of `mean`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to average.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// The mean of `numbers`, rounded toward zero.
///
/// The sum is taken in 128 bits, so it cannot overflow, and the mean always
/// lies between the smallest and the largest number, so it fits in an `i64`.
///
/// # Examples
///
/// ```
/// use vergil_fixture_rust_app::commands::mean::average;
///
/// assert_eq!(average(&[2, 3, 7]).unwrap(), 4);
/// assert_eq!(average(&[-2, -3, -7]).unwrap(), -4);
/// assert_eq!(average(&[i64::MAX, i64::MAX]).unwrap(), i64::MAX);
/// assert!(average(&[]).is_err());
/// ```
///
/// # Errors
///
/// Fails when `numbers` is empty, since no numbers have a mean.
///
/// # Panics
///
/// Never: a mean of `i64`s lies within the `i64` range.
pub fn average(numbers: &[i64]) -> Result<i64> {
    let (&first, rest) = numbers
        .split_first()
        .context("mean needs at least one number")?;
    let (sum, count) = rest
        .iter()
        .fold((i128::from(first), 1_i128), |(sum, count), &n| {
            (sum + i128::from(n), count + 1)
        });
    Ok(i64::try_from(sum / count).expect("a mean of i64s fits in an i64"))
}

/// Writes the mean of `args.numbers` to `out`.
///
/// # Errors
///
/// Fails as [`average`] does, or when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", average(&args.numbers)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::testing::capture;

    const NONE: &str = "mean needs at least one number";

    #[test]
    fn averages_its_numbers() {
        assert_eq!(average(&[2, 4, 9]).unwrap(), 5);
    }

    #[test]
    fn rounds_toward_zero() {
        assert_eq!(average(&[1, 2]).unwrap(), 1);
        assert_eq!(average(&[-1, -2]).unwrap(), -1);
    }

    #[test]
    fn the_mean_of_one_number_is_itself() {
        assert_eq!(average(&[-7]).unwrap(), -7);
    }

    #[test]
    fn reaches_the_bounds_without_overflow() {
        assert_eq!(average(&[i64::MAX, i64::MAX, i64::MAX]).unwrap(), i64::MAX);
        assert_eq!(average(&[i64::MIN, i64::MIN, i64::MIN]).unwrap(), i64::MIN);
        assert_eq!(average(&[i64::MIN, i64::MAX]).unwrap(), 0);
    }

    #[test]
    fn rejects_no_numbers() {
        assert_eq!(average(&[]).unwrap_err().to_string(), NONE);
    }

    #[test]
    fn run_writes_one_line() {
        let args = Args {
            numbers: vec![40, 2, 42],
        };
        let (result, out) = capture(64, |out| run(&args, out));
        result.unwrap();
        assert_eq!(out, b"28\n");
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
