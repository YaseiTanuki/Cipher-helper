//! Cryptan exposes Caesar cipher utilities as both a reusable library and CLI helpers.
//!
//! # Examples
//!
//! Encrypting and then decrypting a message:
//!
//! ```
//! use cryptan::{caesar_encrypt, caesar_decrypt};
//!
//! let secret = caesar_encrypt("attack at dawn", 3);
//! let decoded = caesar_decrypt(&secret, 3);
//! assert_eq!(decoded.text, "attack at dawn");
//! ```

pub mod caesar;
pub mod utils;
pub mod traits;

/// Types and helper functions for Caesar cipher operations.
pub use crate::caesar::{CaesarCipher, DecodedResult, caesar_encrypt, caesar_decrypt, caesar_brute_force, caesar_brute_force_all};
/// Utility helpers including shift normalization and optional Python integration.
pub use crate::utils::{py_meaningful_ratio, normalize_shift};
/// Traits describing common cipher capabilities.
pub use crate::traits::{BruteForce, Decrypt, Encrypt};
