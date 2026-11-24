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
| [`product [NUMBERS]...`](#product) | the product of the integers |
| [`range NUMBERS...`](#range) | the largest of the integers less the smallest |
| [`mean NUMBERS...`](#mean) | the mean of the integers, rounded toward zero |
| [`count [NUMBERS]...`](#count) | how many integers were given |
| [`median NUMBERS...`](#median) | the median of the integers, rounded toward zero |
| [`distinct [NUMBERS]...`](#distinct) | how many different integers were given |
| [`mode NUMBERS...`](#mode) | the most common of the integers, the smallest on a tie |
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

## product

Prints the product of its arguments, which are 64-bit signed integers.
Negative numbers need no `--`. With no arguments, the product is `1`.

```console
$ vergil-fixture-rust-app product 2 3 4
24
$ vergil-fixture-rust-app product -4 2
-8
```

A product outside the 64-bit signed range is an error:

```console
$ vergil-fixture-rust-app product 9223372036854775807 2
error: the product overflows a 64-bit signed integer
```

## range

Prints the spread of its arguments, which are 64-bit signed integers: the
largest less the smallest. Negative numbers need no `--`.

```console
$ vergil-fixture-rust-app range 2 9 3
7
$ vergil-fixture-rust-app range -4 7
11
```

With no arguments there is no spread, so it is an error:

```console
$ vergil-fixture-rust-app range
error: range needs at least one number
```

A spread outside the 64-bit signed range is an error:

```console
$ vergil-fixture-rust-app range -9223372036854775808 9223372036854775807
error: the range overflows a 64-bit signed integer
```

## mean

Prints the mean of its arguments, which are 64-bit signed integers, rounded
toward zero. Negative numbers need no `--`. The mean lies between the
smallest and the largest argument, so unlike a sum it never overflows.

```console
$ vergil-fixture-rust-app mean 2 4 9
5
$ vergil-fixture-rust-app mean -1 -2
-1
```

With no arguments there is no mean, so it is an error:

```console
$ vergil-fixture-rust-app mean
error: mean needs at least one number
```

## count

Prints how many arguments it was given, which are 64-bit signed integers.
Negative numbers need no `--`, and with no arguments it prints `0`.

```console
$ vergil-fixture-rust-app count 2 -4 9
3
$ vergil-fixture-rust-app count
0
```

## median

Prints the median of its arguments, which are 64-bit signed integers: the
middle one once they are sorted, or, for an even count, the mean of the two
middle ones, rounded toward zero. Negative numbers need no `--`, and the
median never overflows.

```console
$ vergil-fixture-rust-app median 9 -4 2
2
$ vergil-fixture-rust-app median 8 -4 2 5
3
```

With no arguments there is no median, so it is an error:

```console
$ vergil-fixture-rust-app median
error: median needs at least one number
```

## distinct

Prints how many different integers it was given, which are 64-bit signed
integers, counting a repeated value once. Negative numbers need no `--`, and
with no arguments it prints `0`.

```console
$ vergil-fixture-rust-app distinct 2 -4 2 9
3
$ vergil-fixture-rust-app distinct
0
```

## mode

Prints the integer it was given most often, of 64-bit signed integers. When
several are given equally often, it prints the smallest of them. Negative
numbers need no `--`.

```console
$ vergil-fixture-rust-app mode 2 -4 9 -4
-4
$ vergil-fixture-rust-app mode 9 2 9 2
2
```

With no arguments it is an error:

```console
$ vergil-fixture-rust-app mode
error: mode needs at least one number
```

## info

Prints one line of compact JSON with the program's name and version, the
crate's current version from `Cargo.toml`:

```console
$ vergil-fixture-rust-app info
{"name":"vergil-fixture-rust-app","version":"<VERSION>"}
```
