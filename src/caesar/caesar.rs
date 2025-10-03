//! Core Caesar cipher implementation.
//!
//! This module provides the `CaesarCipher` type along with supporting data
//! structures for encrypting, decrypting, and brute-forcing text that uses the
//! classical Caesar cipher.

use colored::*;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::normalize_shift;
use crate::traits::{BruteForce, Decrypt, Encrypt};

/// Stateful helper for encrypting, decrypting, and brute-forcing the Caesar cipher.
///
/// Typical usage involves building a `CaesarCipher` via either `new()`,
/// `from_plain()`, or `from_encrypted()` and then calling the trait methods that
/// operate on the stored buffers.
pub struct CaesarCipher {
    plain: String,
    encrypted_text: String,
}

/// Decryption payload plus metadata produced by Caesar cipher helpers.
#[derive(Debug)]
pub struct DecodedResult {
    /// The decrypted plain text.
    pub text: String,
    /// The rotation key that generated `text`.
    pub key: u8,
    /// Optional ratio of meaningful tokens, if Python integration succeeded.
    pub meaningful_ratio: Option<f32>,
}

impl Display for DecodedResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let key_str = format!("{}", self.key).bright_blue();
        let text_str = &self.text.green();
        if let Some(ratio) = self.meaningful_ratio {
            let ratio_str = format!("{:.2}", ratio).yellow();
            write!(f, "[Key: {}, meaningful Ratio: {}] {}", key_str, ratio_str, text_str)
        } else {
            write!(f, "[Key: {}] {}", key_str, text_str)
        }
    }
}

impl CaesarCipher {
    /// Create an empty cipher instance with no buffered text.
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptan::CaesarCipher;
    ///
    /// let cipher = CaesarCipher::new();
    /// assert!(cipher.get_plain().is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            plain: String::new(),
            encrypted_text: String::new(),
        }
    }

    /// Construct a cipher initialized with encrypted text.
    pub fn from_encrypted(encrypted_text: String) -> Self {
        Self {
            plain: String::new(),
            encrypted_text,
        }
    }

    /// Construct a cipher initialized with plain text.
    pub fn from_plain(plain: String) -> Self {
        Self {
            plain,
            encrypted_text: String::new(),
        }
    }

    /// Replace the plain text buffer and return `self` for chaining.
    pub fn set_plain(&mut self, new_plain: String) -> &mut Self {
        self.plain = new_plain;
        self
    }

    /// Replace the encrypted text buffer and return `self` for chaining.
    pub fn set_encrypted_text(&mut self, new_encrypted_text: String) -> &mut Self {
        self.encrypted_text = new_encrypted_text;
        self
    }

    /// Retrieve the current plain text buffer.
    pub fn get_plain(&self) -> String {
        self.plain.clone()
    }

    /// Retrieve the current encrypted text buffer.
    pub fn get_encrypted_text(&self) -> String {
        self.encrypted_text.clone()
    }
}

impl Encrypt for CaesarCipher {
    /// Encrypts `self.plain` with the provided rotation key and updates the encrypted buffer.
    fn encrypt(&mut self, key: Option<i8>) -> String {
        if self.plain.is_empty() {
            eprint!("[warn] Plain text is empty, nothing to encrypt.");
            return String::new();
        }

        let shift = normalize_shift(key.expect("Key is required for encryption"));
        let mut out = String::with_capacity(self.plain.len());

        for &b in self.plain.as_bytes() {
            let new_b = if b.is_ascii_uppercase() {
                ((b - b'A' + shift) % 26) + b'A'
            } else if b.is_ascii_lowercase() {
                ((b - b'a' + shift) % 26) + b'a'
            } else {
                b
            };
            out.push(new_b as char);
        }

        self.set_encrypted_text(out);

        self.encrypted_text.clone()
    }
}

impl Decrypt for CaesarCipher {
    /// Decrypts `self.encrypted_text` using the provided rotation key.
    /// The meaningful ratio is computed lazily via Python if available.
    fn decrypt(&self, key: Option<i8>) -> DecodedResult {
        let key_val = key.expect("Key is required for decryption");
        let shift = normalize_shift(-key_val);
        let mut out = String::with_capacity(self.encrypted_text.len());

        for &b in self.encrypted_text.as_bytes() {
            let new_b = if b.is_ascii_uppercase() {
                ((b - b'A' + shift) % 26) + b'A'
            } else if b.is_ascii_lowercase() {
                ((b - b'a' + shift) % 26) + b'a'
            } else {
                b
            };
            out.push(new_b as char);
        }

        let ratio = match crate::py_meaningful_ratio(&out) {
            Ok(r) => Some(r),
            Err(_) => None,
        };

        DecodedResult {
            text: out,
            key: key_val as u8,
            meaningful_ratio: ratio,
        }
    }
}

impl BruteForce for CaesarCipher {
    /// Attempts to recover plaintext across all 26 rotations, filtering by an optional threshold.
    fn brute_force(&self, threshold: Option<f32>) -> Vec<DecodedResult> {
        let mut results: Vec<DecodedResult> = Vec::new();
        let mut warned = false;

        for key in 0..26 {
            let decrypted = self.decrypt(Some(key));
            let ratio = decrypted.meaningful_ratio.unwrap_or(0.0);

            if decrypted.meaningful_ratio.is_none() && !warned {
                eprintln!("[warn] Python wordfreq unavailable or error when computing meaningful ratio.");
                warned = true;
            }

            if ratio >= threshold.unwrap_or(0.5) {
                results.push(decrypted);
            }
        }

        results
    }

    /// Returns every decoded variant for the 26 possible rotations.
    fn brute_force_all(&self) -> Vec<DecodedResult> {
        let mut results: Vec<DecodedResult> = Vec::new();

        for key in 0..26 {
            let decrypted = self.decrypt(Some(key));
            results.push(decrypted);
        }

        results
    }
}
