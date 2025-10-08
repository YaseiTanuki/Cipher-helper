//! # Cryptan
//!
//! A comprehensive cryptography library providing classical cipher implementations
//! with a focus on the Caesar cipher and brute-force cryptanalysis.
//!
//! ## Overview
//!
//! Cryptan is designed as a playground for exploring classical cryptography concepts.
//! It provides both a convenient API for cipher operations and a command-line interface
//! for interactive use.
//!
//! ## Features
//!
//! - **Caesar Cipher**: Full implementation with encryption, decryption, and brute-force attacks
//! - **Brute-force Analysis**: Intelligent plaintext recovery using word frequency analysis
//! - **CLI Interface**: Easy-to-use command-line tool for cipher operations
//! - **Flexible API**: Both builder-style and functional interfaces available
//! - **Meaningful Text Detection**: Built-in word list analysis for cryptanalysis
//!
//! ## Quick Start
//!
//! ### Basic Usage
//!
//! ```rust
//! use cryptan::{CaesarCipher, ClassicalCipher};
//!
//! let cipher = CaesarCipher::from_key(3);
//! let encrypted = cipher.encrypt("HELLO WORLD");
//! let decrypted = cipher.decrypt(&encrypted);
//!
//! assert_eq!(decrypted, "HELLO WORLD");
//! ```
//!
//! ### Brute-force Attack
//!
//! ```rust
//! use cryptan::{CaesarCipher, BruteForce};
//!
//! let mut cipher = CaesarCipher::from_key(0);
//! let candidates = cipher.bruteforce("KHOOR ZRUOG", Some(0.5));
//!
//! // Results are sorted by meaningfulness score
//! for candidate in candidates {
//!     println!("Key: {}, Text: {}, Score: {:.3}",
//!              candidate.key, candidate.text, candidate.meaningful_ratio.unwrap_or(0.0));
//! }
//! ```
//!
//! ### Command Line Usage
//!
//! ```bash
//! # Encrypt text
//! caesar encrypt 3 "HELLO WORLD"
//!
//! # Decrypt text
//! caesar decrypt 3 "KHOOR ZRUOG"
//!
//! # Brute-force attack
//! caesar brute "KHOOR ZRUOG"
//! ```
//!
//! ## Architecture
//!
//! The library is organized into several modules:
//!
//! - [`classical`](classical/index.html): Classical cipher implementations
//! - [`traits`](traits/index.html): Common cipher trait definitions
//! - [`utils`](utils/index.html): Utility functions for cipher operations
//!
//! ## Security Note
//!
//! This library is intended for educational purposes and should not be used for
//! securing sensitive information. Classical ciphers like Caesar are easily broken
//! with modern cryptanalysis techniques.

pub mod classical;
pub mod utils;
pub mod traits;

/// Types and helper functions for Caesar cipher operations.
pub use classical::*;
/// Utility helpers including shift normalization and optional Python integration.
pub use utils::*;
/// Traits describing common cipher capabilities.
pub use traits::*;