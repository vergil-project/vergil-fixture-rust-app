# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.5] - 2026-11-22

### CI

- stop granting actions: read to the security job (#53)

### Chores

- cargo update

### Features

- add the count subcommand, and roll itoa back in Cargo.lock (#54)

## [0.1.4] - 2026-11-22

### Chores

- cargo update

### Features

- add the mean subcommand (#60)

## [0.1.3] - 2026-11-19

### CI

- pass the automation App secrets to the release (#43)

### Chores

- cargo update

### Features

- add the range subcommand (#48)

## [0.1.2] - 2026-11-18

### Chores

- cargo update

### Documentation

- describe the Rust gate as lines and regions, with the release build (#30)

### Features

- add the product subcommand (#34)

## [0.1.1] - 2026-11-17

### CI

- move the nightly ops cron to the 01:xx UTC hour (#15)

### Chores

- sync the managed files with the released templates (#14)
- sync the managed files with the released templates (#25)

### Documentation

- correct the info example, cargo-deny scope and doctest step (#17)

### Features

- add the min subcommand (#24)

### Testing

- bring region coverage to 100% before the Rust region gate (#19)

## [0.1.0] - 2026-11-14

### Chores

- step 3 - vergil.toml
- step 4 - config files
- step 6 - CI/CD workflows
- step 7 - docs site

### Features

- add the Rust CLI program and run CodeQL on it (#4)
- add the max subcommand (#8)
