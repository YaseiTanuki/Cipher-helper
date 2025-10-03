pub mod caesar;
pub mod caesar_fn;

pub use crate::caesar::caesar::{CaesarCipher, DecodedResult};
pub use crate::caesar::caesar_fn::{brute_force, decode, encode, brute_force_all};