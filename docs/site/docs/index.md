# vergil-fixture-rust-app

This is a Vergil test fixture. It has no users.

It holds a small Rust command-line application so that Vergil's support for
Rust applications runs against a real one: local validation with
`vrg-validate`, CI, the dependency and license audit, this docs site and the
tagged release. Each fixture run adds one small increment to the program and
drives it through Vergil's full cycle, from change to release.

Why the fixture exists, and how a fixture run works, is set out in the
[spec of epic vergil-project/.github#1091](https://github.com/vergil-project/.github/blob/develop/epics/1091-language-fixture-repos/spec.md).

## What it exercises

- `vrg-validate`'s Rust checks: `cargo fmt`, `cargo clippy`, `cargo check`,
  tests at 100% line coverage with `cargo llvm-cov`, and `cargo deny` against
  the fleet's packaged license policy.
- Three real runtime dependencies, `clap`, `anyhow` and `serde_json`, for the
  audit and the dependency tooling to work on.
- The fleet's CI and CD workflows, this site and the release.

Nothing it builds is published to a package registry.

## Next

- [Getting Started](getting-started.md): build and run it.
- [Usage](usage.md): its subcommands.
