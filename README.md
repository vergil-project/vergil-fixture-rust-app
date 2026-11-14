# vergil-fixture-rust-app

This repository is a Vergil test fixture. It has no users. It holds a small,
real Rust command-line application so that Vergil's support for Rust
applications has something real to run against: local validation, CI, the
dependency and license audit, the docs site and the release. It is one of the
four fixture repos that epic
[vergil-project/.github#1091](https://github.com/vergil-project/.github/issues/1091)
describes in its
[spec](https://github.com/vergil-project/.github/blob/develop/epics/1091-language-fixture-repos/spec.md).

## Table of Contents

- [Status](#status)
- [Overview](#overview)
- [Getting Started](#getting-started)
- [License](#license)

## Status

A test fixture. It changes only through fixture runs, each of which adds one
small increment to the program.

## Overview

The program, `vergil-fixture-rust-app`, has three subcommands:

| Command | Prints |
|---|---|
| `greet NAME` | `Hello, NAME` |
| `sum [NUMBERS]...` | the sum of the integers, `0` for none |
| `info` | the program's name and version as JSON |

It exercises:

- `vrg-validate`'s Rust checks: `cargo fmt`, `cargo clippy`, `cargo check`,
  tests at 100% line coverage with `cargo llvm-cov`, and `cargo deny` against
  the fleet's packaged license policy;
- three real runtime dependencies, `clap`, `anyhow` and `serde_json`, for the
  audit and the dependency tooling to work on;
- the fleet's CI and CD workflows, the docs site and the tagged release.

Nothing it builds is published to a package registry (`publish = false` in
`Cargo.toml`).

## Getting Started

See the [documentation](https://vergil-project.github.io/vergil-fixture-rust-app/).

## License

MIT — see [LICENSE](LICENSE).
