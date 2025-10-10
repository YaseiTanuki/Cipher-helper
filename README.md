# Cryptan

Cryptan is a small Rust toolkit and CLI for experimenting with the classic Caesar cipher. It provides:

- A command-line binary (installed as `caesar` via `cargo install` or run with `cargo run`).
- A reusable library API exposing a builder-style `CaesarCipher` and a `DecodedResult` payload used by the bruteforce routine.

## Features

- Encrypt/decrypt using integer rotation keys (supports negative and >26 keys; rotation is modulo 26).
- Brute-force all 26 rotations and optionally filter results by a "meaningfulness" ratio computed against `public/words.txt`.

## Requirements

- Rust (edition 2021). Recent rustup-stable releases work (1.70+ is sufficient).

## Build and install

From the repository root:

```powershell
# build & run locally
cargo run -- <subcommand> [args...]

# or install the binary into your cargo bin directory
cargo install --path .
```

When installed, the binary name comes from `Cargo.toml` ([[bin]] name = "caesar").

## CLI usage

The CLI supports three subcommands: `encrypt`, `decrypt`, and `brute`.

Examples:

```powershell
# encrypt: cargo run -- encrypt <key:i16> "plain text"
cargo run -- encrypt 3 "attack at dawn"

# decrypt: cargo run -- decrypt <key:i16> "encrypted text"
cargo run -- decrypt 3 "dwwdfn dw gdzq"

# brute-force: cargo run -- brute "encoded text" [threshold:f32]
# threshold is optional; if omitted all candidates are returned (threshold defaults to 0.0)
cargo run -- brute "ftue rcjj" 0.6
```

Note: `cargo run --` passes the following tokens to the binary. If you installed the package with `cargo install`, run the `caesar` binary directly (e.g. `caesar brute "ftue rcjj"`).

## Library usage

There are no top-level convenience functions called `caesar_encrypt`/`caesar_decrypt` in the current code. Use the provided types instead:

### Basic Example

```rust
use cryptan::{CaesarCipher, ClassicalCipher};

fn main() {
    let c = CaesarCipher::from_key(3);
    let secret = c.encrypt("attack at dawn");
    println!("Encrypted: {}", secret);
    assert_eq!(secret, "dwwdfn dw gdzq");

    let recovered = c.decrypt(&secret);
    println!("Decrypted: {}", recovered);
    assert_eq!(recovered, "attack at dawn");

    println!("✅ Caesar cipher test passed!");
}
```

For brute-force you get a `Vec<DecodedResult>` (see `src/utils/utils_struct.rs`) where each `DecodedResult` includes `text`, `key`, and optional `meaningful_ratio`.

### Brute-force Example

```rust
use cryptan::{CaesarCipher, BruteForce};

fn main() {
    let ciphertext = "dwwdfn dw gdzq";
    let mut cipher = CaesarCipher::from_key(0);

    println!("Attempting brute-force attack on: {}", ciphertext);
    let candidates = cipher.bruteforce(ciphertext, Some(0.3));

    for (i, candidate) in candidates.iter().enumerate() {
        println!("{:02}. Key: {}, Text: '{}', Confidence: {:.3}",
                i + 1,
                candidate.key,
                candidate.text,
                candidate.meaningful_ratio.unwrap_or(0.0));
    }
}
```

## Important implementation notes

- CLI: the binary is configured in `Cargo.toml` ([[bin]] name = "caesar", path = "src/main.rs"). The clap-based CLI is defined in `src/main.rs` and exposes `encrypt`, `decrypt`, and `brute` subcommands.
- Cipher implementation: `src/classical/caesar/caesar.rs` provides the `CaesarCipher` type implementing the `ClassicalCipher` and `BruteForce` traits (`src/traits.rs`).
- Meaningfulness scoring: `src/utils/utils.rs::meaningful_ratio` uses the plain word list loaded from `public/words.txt` via `load_set`.
- Decoded output formatting: `src/utils/utils_struct.rs::DecodedResult` implements `Display` with colored output.

## Project layout (partial)

```
Cipher-helper/
├── Cargo.toml
├── LICENSE
├── README.md
├── public/
│   └── words.txt
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── classical/    # module: `cryptan::classical`
│   │   └── caesar/   # module: `cryptan::classical::caesar`
│   ├── traits.rs
   │   
│   └── utils/        # module: `cryptan::utils`
└── target/
   └── ...
```

## Contributing

Contributions welcome. Follow repository conventions:

- Format code with `cargo fmt`.
- Run basic checks with `cargo clippy` and `cargo test` (add tests under `src/` or `tests/` as needed).

## License

This crate is licensed under Apache-2.0 (see `Cargo.toml`).

## Acknowledgements & used crates

Thanks to the maintainers of the [dwyl/english-words](https://github.com/dwyl/english-words) repository for the public word list included at `public/words.txt`.

Primary crates used in this project:

- [clap](https://crates.io/crates/clap) — Command-line argument parsing and subcommand parsing for the binary (declares CLI, derives parsers from structs).
- [colored](https://crates.io/crates/colored) — Adds ANSI color helpers for terminal output (used by `DecodedResult` Display impl).
- [log](https://crates.io/crates/log) — Logging facade for libraries and binaries; provides logging macros (`info!`, `warn!`, etc.).
- [env_logger](https://crates.io/crates/env_logger) — A logger implementation that reads log level from environment variables (integrates with `log`).

Special thanks to the maintainers and contributors of the linked projects for their time and open-source work — this project benefits from your efforts.