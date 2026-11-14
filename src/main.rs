//! The `vergil-fixture-rust-app` binary: parse the command line, run the
//! subcommand, and turn its outcome into an exit code.

use std::io;
use std::process::ExitCode;

use clap::Parser;
use vergil_fixture_rust_app::{Cli, run};

fn main() -> ExitCode {
    // A usage error, `--help` and `--version` exit here, with clap's own codes.
    let cli = Cli::parse();
    match run(cli.command, &mut io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}
