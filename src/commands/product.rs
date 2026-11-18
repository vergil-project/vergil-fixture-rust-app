//! `product [NUMBERS]...`: print the product of some integers.

use std::io::Write;

use anyhow::{Context, Result};

/// The arguments of `product`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to multiply.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// The product of `numbers`, which is 1 for none.
///
/// # Examples
///
/// ```
/// use vergil_fixture_rust_app::commands::product::multiply;
///
/// assert_eq!(multiply(&[2, -3, 4]).unwrap(), -24);
/// assert_eq!(multiply(&[]).unwrap(), 1);
/// assert!(multiply(&[i64::MAX, 2]).is_err());
/// ```
///
/// # Errors
///
/// Fails when the product does not fit in an `i64`.
pub fn multiply(numbers: &[i64]) -> Result<i64> {
    numbers
        .iter()
        .try_fold(1_i64, |acc, &n| acc.checked_mul(n))
        .context("the product overflows a 64-bit signed integer")
}

/// Writes the product of `args.numbers` to `out`.
///
/// # Errors
///
/// Fails as [`multiply`] does, or when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", multiply(&args.numbers)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::testing::capture;

    const OVERFLOW: &str = "the product overflows a 64-bit signed integer";

    #[test]
    fn multiplies_its_numbers() {
        assert_eq!(multiply(&[2, 3, 4]).unwrap(), 24);
    }

    #[test]
    fn multiplies_negative_numbers() {
        assert_eq!(multiply(&[-4, 2]).unwrap(), -8);
        assert_eq!(multiply(&[-4, -2]).unwrap(), 8);
    }

    #[test]
    fn the_product_of_none_is_one() {
        assert_eq!(multiply(&[]).unwrap(), 1);
    }

    #[test]
    fn a_zero_makes_the_product_zero() {
        assert_eq!(multiply(&[5, 0, 7]).unwrap(), 0);
    }

    #[test]
    fn reaches_the_bounds_without_overflow() {
        assert_eq!(multiply(&[i64::MAX]).unwrap(), i64::MAX);
        assert_eq!(multiply(&[i64::MIN]).unwrap(), i64::MIN);
        assert_eq!(multiply(&[i64::MAX, -1]).unwrap(), -i64::MAX);
    }

    #[test]
    fn reports_an_overflow() {
        assert_eq!(multiply(&[i64::MAX, 2]).unwrap_err().to_string(), OVERFLOW);
    }

    #[test]
    fn reports_an_underflow() {
        assert_eq!(multiply(&[i64::MIN, 2]).unwrap_err().to_string(), OVERFLOW);
    }

    #[test]
    fn reports_the_negated_minimum_as_an_overflow() {
        assert_eq!(multiply(&[i64::MIN, -1]).unwrap_err().to_string(), OVERFLOW);
    }

    #[test]
    fn run_writes_one_line() {
        let args = Args {
            numbers: vec![6, 7],
        };
        let (result, out) = capture(64, |out| run(&args, out));
        result.unwrap();
        assert_eq!(out, b"42\n");
    }

    #[test]
    fn run_writes_nothing_on_error() {
        let args = Args {
            numbers: vec![i64::MAX, 2],
        };
        let (result, out) = capture(64, |out| run(&args, out));
        assert_eq!(result.unwrap_err().to_string(), OVERFLOW);
        assert!(out.is_empty());
    }

    #[test]
    fn run_reports_a_write_error() {
        let args = Args {
            numbers: vec![6, 7],
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
