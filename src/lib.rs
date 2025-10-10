//! This library is intended for educational purposes and should not be used for
//! securing sensitive information. Classical ciphers like Caesar are easily broken
//! with modern cryptanalysis techniques.

pub mod classical;
pub mod encoding;
pub mod utils;
pub mod traits;

/// Types and helper functions for Caesar cipher operations.
pub use classical::*;
/// Encoding schemes for various cipher and communication systems.
///
/// This module provides implementations of different encoding schemes including
/// Morse code with support for both International and American standards.
/// The encoding schemes implement the `Codec` trait for consistent encoding
/// and decoding operations across different systems.
///
/// # Examples
///
/// ```rust
/// use cryptan::Morse;
/// use cryptan::Codec;
///
/// let morse = Morse::new();
/// let encoded = morse.encode("HELLO");
/// let decoded = morse.decode(&encoded);
/// ```
pub use encoding::*;
/// Utility helpers including shift normalization and optional Python integration.
pub use utils::*;
/// Traits describing common cipher capabilities.
pub use traits::*;