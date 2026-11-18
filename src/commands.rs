//! The subcommands. Each module holds a subcommand's arguments, its logic as a
//! plain function, and a `run` that writes the result.

pub mod greet;
pub mod info;
pub mod max;
pub mod min;
pub mod product;
pub mod sum;

#[cfg(test)]
mod testing {
    use anyhow::Result;

    /// Runs `run` with a writer that holds at most `capacity` bytes, and returns
    /// its result and what it wrote.
    ///
    /// Every `run` test uses this one writer type. A `run` is generic over its
    /// writer, and llvm-cov's region count takes each generic function's
    /// best-covered instantiation alone, so a command's success, error and
    /// write-error tests must share one instantiation for all of its regions
    /// to count as covered. A `capacity` too small for the output makes the
    /// write fail.
    pub(crate) fn capture(
        capacity: usize,
        run: impl FnOnce(&mut &mut [u8]) -> Result<()>,
    ) -> (Result<()>, Vec<u8>) {
        let mut buf = vec![0; capacity];
        let mut out: &mut [u8] = &mut buf;
        let result = run(&mut out);
        let written = capacity - out.len();
        buf.truncate(written);
        (result, buf)
    }
}
