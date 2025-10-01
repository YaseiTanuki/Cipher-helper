# Copilot Instructions for Cipher-helper

## Project Overview
This is a Rust-based command-line utility for cipher operations, with a focus on modularity and extensibility. The main entry point is `src/main.rs`, and core logic is split across several modules:
- `src/lib.rs`: Shared library code and re-exports
- `src/caesar/`: Caesar cipher implementation (`caesar.rs`, `mod.rs`)
- `src/py_dict.rs`: Python-style dictionary utilities
- `src/traits.rs`: Common traits for extensibility

## Architecture & Data Flow
- The project uses Rust modules to separate cipher algorithms and shared utilities.
- The main function orchestrates user input, dispatches to cipher logic, and prints results.
- Traits in `traits.rs` define interfaces for cipher operations, enabling easy addition of new ciphers.
- The `caesar` module is a template for adding more cipher types.

## Developer Workflows
- **Build:** Use `cargo build` to compile. For development, use `cargo run` to execute the main binary.
- **Debug:** Standard Rust debugging tools apply. Use `cargo test` for any testable logic (add tests in `src/` or a `tests/` folder if needed).
- **Add a Cipher:** Create a new module in `src/`, implement required traits from `traits.rs`, and update `main.rs` to include the new cipher.

## Conventions & Patterns
- All cipher logic should implement the traits defined in `traits.rs` for consistency.
- Use module-level separation for each cipher type.
- Shared utilities (e.g., dictionary helpers) go in their own files (`py_dict.rs`).
- Avoid monolithic functions in `main.rs`; delegate to modules.

## Integration Points
- No external dependencies beyond standard Rust crates (see `Cargo.toml`).
- The project is designed for easy extension with new ciphers or utilities.

## Key Files
- `src/main.rs`: Entry point, user interaction, dispatch
- `src/traits.rs`: Cipher trait definitions
- `src/caesar/caesar.rs`: Example cipher implementation
- `src/py_dict.rs`: Utility functions

## Example: Adding a New Cipher
1. Create `src/newcipher/` and `src/newcipher.rs`.
2. Implement the trait(s) from `traits.rs`.
3. Update `main.rs` to use the new cipher.

---
For questions or unclear conventions, review module structure and trait usage in `src/traits.rs` and `src/caesar/caesar.rs`.
