//! `sum [NUMBERS]...`: print the sum of some integers.

use std::io::Write;

use anyhow::{Context, Result};

/// The arguments of `sum`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// The integers to add.
    #[arg(allow_negative_numbers = true)]
    pub numbers: Vec<i64>,
}

/// The sum of `numbers`, which is 0 for none.
///
/// # Errors
///
/// Fails when the sum does not fit in an `i64`.
pub fn total(numbers: &[i64]) -> Result<i64> {
    numbers
        .iter()
        .try_fold(0_i64, |acc, &n| acc.checked_add(n))
        .context("the sum overflows a 64-bit signed integer")
}

/// Writes the sum of `args.numbers` to `out`.
///
/// # Errors
///
/// Fails as [`total`] does, or when writing to `out` fails.
pub fn run(args: &Args, out: &mut impl Write) -> Result<()> {
    writeln!(out, "{}", total(&args.numbers)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const OVERFLOW: &str = "the sum overflows a 64-bit signed integer";

    #[test]
    fn adds_its_numbers() {
        assert_eq!(total(&[1, 2, 3]).unwrap(), 6);
    }

    #[test]
    fn adds_negative_numbers() {
        assert_eq!(total(&[-4, 2]).unwrap(), -2);
    }

    #[test]
    fn the_sum_of_none_is_zero() {
        assert_eq!(total(&[]).unwrap(), 0);
    }

    #[test]
    fn reaches_the_bounds_without_overflow() {
        assert_eq!(total(&[i64::MAX]).unwrap(), i64::MAX);
        assert_eq!(total(&[i64::MIN]).unwrap(), i64::MIN);
    }

    #[test]
    fn reports_an_overflow() {
        assert_eq!(total(&[i64::MAX, 1]).unwrap_err().to_string(), OVERFLOW);
    }

    #[test]
    fn reports_an_underflow() {
        assert_eq!(total(&[i64::MIN, -1]).unwrap_err().to_string(), OVERFLOW);
    }

    #[test]
    fn run_writes_one_line() {
        let mut out = Vec::new();
        let args = Args {
            numbers: vec![40, 2],
        };
        run(&args, &mut out).unwrap();
        assert_eq!(out, b"42\n");
    }
}
