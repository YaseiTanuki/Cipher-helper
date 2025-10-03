# Cryptan

Cryptan is a Rust toolkit for experimenting with the classic Caesar cipher. It provides a command-line interface and reusable library helpers for encrypting, decrypting, and brute-forcing ciphertext. Optional Python integration (via `pyo3`) uses the `wordfreq` package to score decoded results.

## Features

- Encrypt plaintext with arbitrary rotation keys, including negative and >26 values.
- Decrypt ciphertext for a known key.
- Brute-force search possible keys, returning the most meaningful results first.
- Optional scoring of decoded text using Python's `wordfreq` frequencies.
- Usable as a CLI (`caesar`) or as a Rust library (`cryptan`).

## Requirements

- Rust 1.70+ (edition 2021).
- Python 3.7+ with the `wordfreq` package installed when you want meaningfulness scoring.

Install the Python dependency with:

```shell
pip install wordfreq
```

If `wordfreq` is unavailable, brute-force output is still produced but without meaningfulness scores.

## Installation

### From source

Clone this repository and build the CLI locally:

```shell
cargo install --path .
```

This installs the `caesar` binary in your Cargo bin directory.

### Library usage

Add the crate as a dependency in your `Cargo.toml` (use a path while developing locally):

```toml
[dependencies]
cryptan = { path = "../Cipher-helper" }
```

## Command-line usage

List the available subcommands:

```shell
caesar --help
```

Encrypt plaintext with a key:

```shell
caesar encrypt 3 "attack at dawn"
```

Decrypt ciphertext when you know the key:

```shell
caesar decrypt 3 "dwwdfn dw gdzq"
```

Brute-force every rotation and rely on meaningfulness scoring (defaults to threshold 0.5):

```shell
caesar brute "ftue rcjj" --threshold 0.6
```

Show every decoded variant regardless of score:

```shell
caesar brute --all "ftue rcjj"
```

## Library examples

Encrypt and decrypt in your own code using the convenience functions exported by `cryptan`:

```rust
use cryptan::{caesar_encrypt, caesar_decrypt};

let secret = caesar_encrypt("attack at dawn", 3);
assert_eq!(secret, "dwwdfn dw gdzq");

let recovered = caesar_decrypt(&secret, 3);
assert_eq!(recovered.text, "attack at dawn");
```

For advanced scenarios you can work directly with the `CaesarCipher` type and implement the `Encrypt`, `Decrypt`, or `BruteForce` traits.

## Project layout

- `src/main.rs`: CLI entrypoint using `clap`.
- `src/lib.rs`: Library exports for Caesar helpers and utilities.
- `src/caesar/`: Caesar cipher implementation and brute-force logic.
- `src/utils/`: Shared utilities, including Python integration for meaningfulness scoring.

## Contributing

Issues and pull requests are welcome. Please ensure Rust code is formatted (`cargo fmt`) and passes clippy checks (`cargo clippy`) before submitting.

## License

This project is currently unlicensed. Please add a license file before distributing binaries or publishing the crate.
