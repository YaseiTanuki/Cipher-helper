//! Encoding schemes for various cipher and communication systems.
//!
//! This module provides implementations of different encoding schemes commonly
//! used in cryptography and telecommunications. Currently supports Morse code
//! with both International and American standards.
//!
//! The encoding schemes in this module implement the [`Codec`] trait from the
//! parent crate, providing a consistent interface for encoding and decoding
//! operations.
//!
//! # Example
//!
//! ```rust
//! use cryptan::encoding::morse::Morse;
//!
//! // Create a new Morse encoder with International standard
//! let morse = Morse::new();
//!
//! // Encode text to Morse code
//! let encoded = morse.encode("HELLO WORLD");
//! println!("Encoded: {}", encoded);
//!
//! // Decode Morse code back to text
//! let decoded = morse.decode(&encoded);
//! println!("Decoded: {}", decoded);
//! ```

pub mod morse;

pub use morse::*;
