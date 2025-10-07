//! Caesar cipher primitives, including both object-oriented and free-function APIs.
//!
//! The `CaesarCipher` type offers a builder-style workflow, while the functions in
//! `caesar_fn` enable lightweight helpers for quick scripting or embedding.

pub mod caesar;

/// Builder-style cipher type and the decoded result payload.
pub use caesar::*;
