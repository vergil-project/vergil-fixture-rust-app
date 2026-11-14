# Getting Started

The fixture is a Cargo package with one binary, `vergil-fixture-rust-app`.

## Build and run

From a clone of the repository:

```console
$ cargo run -- greet Ada
Hello, Ada
```

`cargo build --release` puts the binary at
`target/release/vergil-fixture-rust-app`.

## Validate

As in every Vergil repo, validation runs in the Vergil dev container, and
nothing else is the gate:

```bash
vrg-container-run -- vrg-validate
```

## Layout

| Path | Holds |
|---|---|
| `src/main.rs` | the binary: parse the command line, run, set the exit code |
| `src/lib.rs` | `run`, which dispatches a parsed subcommand |
| `src/cli.rs` | the `clap` parser and the `Command` enum |
| `src/commands/` | one module per subcommand, with its unit tests |
| `tests/cli.rs` | end-to-end tests that run the built binary |

## Adding a subcommand

A fixture run grows the program by one subcommand at a time, with no
restructuring:

1. Add `src/commands/<name>.rs` with the subcommand's `Args` (when it takes
   any), its logic as a plain function, a `run` that writes the result, and
   unit tests.
2. Declare the module in `src/commands.rs`.
3. Add a variant to `Command` in `src/cli.rs`, and its arm to the `match` in
   `run` in `src/lib.rs`.
4. Add its end-to-end tests to `tests/cli.rs` and its entry to the
   [Usage](usage.md) page.
