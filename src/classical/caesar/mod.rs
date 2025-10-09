//! # Caesar Cipher Implementation
//!
//! This module provides a complete implementation of the Caesar cipher, one of the
//! oldest and simplest encryption techniques. The Caesar cipher is a type of
//! substitution cipher where each letter in the plaintext is shifted by a fixed
//! number of positions in the alphabet.
//!
//! ## Overview
//!
//! The Caesar cipher operates on the 26-letter Latin alphabet, shifting each letter
//! by a fixed key value. For example, with a shift of 3:
//! - 'A' becomes 'D'
//! - 'B' becomes 'E'
//! - ...
//! - 'Z' becomes 'C'
//!
//! Non-alphabetic characters are preserved unchanged.
//!
//! ## Features
//!
//! - **Encryption & Decryption**: Standard Caesar cipher operations
//! - **Brute-force Attacks**: Systematic testing of all 26 possible keys
//! - **Meaningful Text Analysis**: Word frequency analysis for cryptanalysis
//! - **Flexible Key Handling**: Support for negative keys and key ranges > 26
//! - **Builder Pattern**: Convenient object-oriented API
//!
//! ## Quick Start
//!
//! ### Basic Usage
//!
//! ```rust
//! use cryptan::{CaesarCipher, ClassicalCipher};
//!
//! // Create a cipher with shift of 3
//! let cipher = CaesarCipher::from_key(3);
//!
//! let plaintext = "HELLO WORLD";
//! let encrypted = cipher.encrypt(plaintext);
//! let decrypted = cipher.decrypt(&encrypted);
//!
//! assert_eq!(decrypted, plaintext);
//! ```
//!
//! ### Brute-force Cryptanalysis
//!
//! ```rust
//! use cryptan::{CaesarCipher, BruteForce};
//!
//! let ciphertext = "KHOOR ZRUOG";
//! let mut cipher = CaesarCipher::from_key(0);
//!
//! // Try all possible keys, filter by meaningfulness
//! let candidates = cipher.bruteforce(ciphertext, Some(0.5));
//!
//! for candidate in candidates {
//!     println!("Key: {}, Text: '{}', Confidence: {:.3}",
//!              candidate.key,
//!              candidate.text,
//!              candidate.meaningful_ratio.unwrap_or(0.0));
//! }
//! ```
//!
//! ## Mathematical Background
//!
//! The Caesar cipher can be described mathematically as:
//!
//! - **Encryption**: `E(x) = (x + k) mod 26`
//! - **Decryption**: `D(x) = (x - k) mod 26`
//!
//! Where `x` is the position of a letter in the alphabet (A=0, B=1, ..., Z=25)
//! and `k` is the shift key.
//!
//! ## Security Considerations
//!
//! ⚠️ **Educational Purpose Only**: The Caesar cipher is extremely vulnerable to
//! brute-force attacks due to its small key space (only 26 possibilities). It
//! should never be used for securing sensitive information.
//!
//! ## Examples in History
//!
//! The Caesar cipher is named after Julius Caesar, who reportedly used it to
//! communicate with his generals. Each letter was shifted by 3 positions:
//! "VENI VIDI VICI" became "YHQL YLGL YLFL".

pub mod caesar;

/// Builder-style cipher type and the decoded result payload.
pub use caesar::*;
