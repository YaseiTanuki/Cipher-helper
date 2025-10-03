//! Core traits describing common Caesar cipher behaviors.

use crate::caesar::caesar::DecodedResult;

/// Defines encryption capabilities for cipher types.
pub trait Encrypt {
    /// Encrypts internal state using an optional rotation key.
    fn encrypt(&mut self, key: Option<i8>) -> String;
}

/// Defines decryption capabilities for cipher types.
pub trait Decrypt {
    /// Decrypts internal state using an optional rotation key.
    fn decrypt(&self, key: Option<i8>) -> DecodedResult;
}

/// Provides brute-force exploration helpers for cipher types.
pub trait BruteForce {
    /// Returns candidates meeting an optional meaningfulness threshold.
    fn brute_force(&self, threshold: Option<f32>) -> Vec<DecodedResult>;
    /// Returns every candidate without filtering.
    fn brute_force_all(&self) -> Vec<DecodedResult>;
}
