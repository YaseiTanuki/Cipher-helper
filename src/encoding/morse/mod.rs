//! Morse code encoding and decoding implementation.
//!
//! This module provides comprehensive Morse code functionality supporting both
//! International and American Morse code standards. It includes character
//! mappings, encoding/decoding operations, and language switching capabilities.
//!
//! The main interface is provided through the [`Morse`] struct which implements
//! the [`Codec`] trait for consistent encoding and decoding operations.
//!
//! # Supported Standards
//!
//! - **International Morse Code**: The standard used worldwide for amateur radio
//!   and maritime communications
//! - **American Morse Code**: An older standard primarily used in North America
//!
//! # Example
//!
//! ```rust
//! use cryptan::encoding::morse::{Morse, MorseLang};
//!
//! // Create a Morse encoder with International standard
//! let mut morse = Morse::new();
//!
//! // Encode a message
//! let encoded = morse.encode("SOS");
//! println!("SOS in Morse: {}", encoded);
//!
//! // Decode the message back
//! let decoded = morse.decode(&encoded);
//! println!("Decoded: {}", decoded);
//!
//! // Switch to American Morse code
//! morse.set_lang("american");
//! let american_encoded = morse.encode("SOS");
//! println!("SOS in American Morse: {}", american_encoded);
//! ```

pub mod morse;
pub mod morse_code;

pub use morse::Morse;
pub use morse_code::{AMERICAN_MORSE, INTERNATIONAL_MORSE, MorseLang};