# Usage

```text
vergil-fixture-rust-app <COMMAND>
```

| Command | Prints |
|---|---|
| [`greet NAME`](#greet) | a greeting for `NAME` |
| [`sum [NUMBERS]...`](#sum) | the sum of the integers |
| [`max NUMBERS...`](#max) | the largest of the integers |
| [`min NUMBERS...`](#min) | the smallest of the integers |
| [`info`](#info) | the program's name and version as JSON |

`--help` prints the commands, and `--version` prints the program's name and
version.

## Exit codes

| Code | Meaning |
|---|---|
| `0` | success |
| `1` | the command failed; the reason is on standard error, after `error:` |
| `2` | a usage error, such as a missing command or an argument that does not parse |

## greet

Prints `Hello, NAME`.

```console
$ vergil-fixture-rust-app greet Ada
Hello, Ada
```

A name that is empty or only whitespace is an error:

```console
$ vergil-fixture-rust-app greet " "
error: the name must not be blank
```

## sum

Prints the sum of its arguments, which are 64-bit signed integers. Negative
numbers need no `--`. With no arguments, the sum is `0`.

```console
$ vergil-fixture-rust-app sum 1 2 3
6
$ vergil-fixture-rust-app sum -4 2
-2
```

A sum outside the 64-bit signed range is an error:

```console
$ vergil-fixture-rust-app sum 9223372036854775807 1
error: the sum overflows a 64-bit signed integer
```

## max

Prints the largest of its arguments, which are 64-bit signed integers.
Negative numbers need no `--`.

```console
$ vergil-fixture-rust-app max 1 3 2
3
$ vergil-fixture-rust-app max -4 -2
-2
```

With no arguments there is no largest number, so it is an error:

```console
$ vergil-fixture-rust-app max
error: max needs at least one number
```

## min

Prints the smallest of its arguments, which are 64-bit signed integers.
Negative numbers need no `--`.

```console
$ vergil-fixture-rust-app min 2 1 3
1
$ vergil-fixture-rust-app min -4 -2
-4
```

With no arguments there is no smallest number, so it is an error:

```console
$ vergil-fixture-rust-app min
error: min needs at least one number
```

## info

Prints one line of compact JSON with the program's name and version, the
crate's current version from `Cargo.toml`:

```console
$ vergil-fixture-rust-app info
{"name":"vergil-fixture-rust-app","version":"<VERSION>"}
```
