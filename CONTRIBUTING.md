# Contributing to Universal Device SDK

## Setup

1. Install Rust (stable): <https://rustup.rs/>
2. Install cross-compilation toolchains:
   - ESP32: `cargo install espup && espup install`
   - ARM (STM32, RP2040): `rustup target add thumbv7em-none-eabihf thumbv6m-none-eabi`
   - AVR (Arduino): `rustup target add avr-unknown-gnu-atmega328`
3. Clone the repo:
   ```bash
   git clone https://github.com/dupewon/Universal-Device-SDK.git
   cd Universal-Device-SDK
   cargo build
   ```

## Coding Standards

- Format: `cargo fmt` — run before every commit
- Lint: `cargo clippy -- -D warnings` — zero warnings on merge
- Tests: unit tests in `#[cfg(test)]` modules, integration in `tests/`
- Commits: [Conventional Commits](https://www.conventionalcommits.org/) — `type(scope): subject`

## Pull Request Process

1. Create a feature branch: `feature/description` or `fix/description`
2. Ensure CI passes (fmt, clippy, test, doc)
3. Request review from at least two core contributors
4. Squash merge to main

## Code of Conduct

All contributors must adhere to the [Code of Conduct](CODE_OF_CONDUCT.md).
