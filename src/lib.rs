//! This library is intended for educational purposes and should not be used for
//! securing sensitive information. Classical ciphers like Caesar are easily broken
//! with modern cryptanalysis techniques.

pub mod classical;
pub mod encoding;
pub mod utils;
pub mod traits;

/// Types and helper functions for Caesar cipher operations.
pub use classical::*;
/// Encoding schemes like Morse code.
pub use encoding::*;
/// Utility helpers including shift normalization
pub use utils::*;
/// Traits describing common cipher capabilities.
pub use traits::*;