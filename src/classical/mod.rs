//! # Classical Cryptography Module
//!
//! This module contains implementations of classical ciphers, focusing on
//! historical encryption techniques that operate on the fundamental principles
//! of substitution and transposition.
//!
//! ## Overview
//!
//! Classical cryptography encompasses encryption methods developed before the
//! advent of modern computers. These ciphers typically rely on manual calculations
//! and can be broken with pencil-and-paper methods or simple systematic approaches.
//!
//! ## Currently Implemented
//!
//! - **Caesar Cipher**: Simple substitution cipher with shift-based encryption
//!
//! ## Future Plans
//!
//! - **Affine Cipher**: Mathematical extension of Caesar cipher
//! - **Vigenère Cipher**: Polybius square-based polyalphabetic cipher
//! - **Substitution Cipher**: General monoalphabetic substitution
//! - **Transposition Ciphers**: Columnar and rail fence transposition
//!
//! ## Educational Value
//!
//! Classical ciphers are excellent for learning cryptography fundamentals:
//! - Understanding key spaces and brute-force attacks
//! - Exploring frequency analysis and cryptanalysis
//! - Appreciating the evolution toward modern cryptography
//!
//! ## Security Note
//!
//! ⚠️ **Not Secure**: All classical ciphers in this module are vulnerable to
//! modern cryptanalysis and should only be used for educational purposes.
//! For real security, use modern cryptographic algorithms like AES.

pub mod caesar;

pub use caesar::*;
