//! Caesar cipher primitives, including both object-oriented and free-function APIs.
//!
//! The `CaesarCipher` type offers a builder-style workflow, while the functions in
//! `caesar_fn` enable lightweight helpers for quick scripting or embedding.

pub mod caesar;
pub mod caesar_fn;

/// Builder-style cipher type and the decoded result payload.
pub use crate::caesar::caesar::{CaesarCipher, DecodedResult};
/// Free-function helpers for encrypting, decrypting, and brute-forcing text.
pub use crate::caesar::caesar_fn::{caesar_brute_force, caesar_decrypt, caesar_encrypt, caesar_brute_force_all};
