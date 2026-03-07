# Contributing to uroman-fast

Thank you for your interest in contributing to uroman-fast. This document explains how to set up the project, run tests, and submit changes.

## Code of conduct

By participating in this project, you agree to be respectful and constructive. Harassment and discriminatory behavior are not tolerated.

## How to contribute

- **Bug reports**: Open an issue describing the bug, steps to reproduce, and your environment (OS, Python version).
- **Feature requests**: Open an issue with a clear description and use case.
- **Code changes**: Open a pull request from a branch (see below).

## Development setup

### Prerequisites

- **Rust** (latest stable): [rustup](https://rustup.rs/)
- **Python 3.8+** (e.g. 3.12)
- **maturin**: `pip install maturin`

### Build and install (editable)

From the repository root:

```bash
pip install maturin
maturin develop --release
```
