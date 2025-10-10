use std::fmt::{Display, Formatter, Result as FmtResult};
use colored::*;

/// Represents the result of a brute-force decryption attempt.
///
/// This struct contains the decrypted text, the key used for decryption,
/// and an optional meaningfulness score indicating how likely the result
/// is to be valid plaintext.
///
/// # Examples
///
/// ```rust
/// use cryptan::DecodedResult;
///
/// let result = DecodedResult {
///     text: "HELLO WORLD".to_string(),
///     key: 3,
///     meaningful_ratio: Some(0.95),
/// };
///
/// println!("{}", result); // Prints with colored formatting
/// ```
#[derive(Debug)]
pub struct DecodedResult {
    /// The decrypted plain text.
    ///
    /// This contains the text that was produced by applying the decryption
    /// algorithm with the associated key.
    pub text: String,

    /// The rotation key that generated `text`.
    ///
    /// For Caesar cipher, this represents the shift value (0-25) that was
    /// used to decrypt the ciphertext.
    pub key: u8,

    /// Optional ratio of meaningful tokens, if Python integration succeeded.
    ///
    /// This score represents how many words in the decrypted text appear
    /// in a reference word list, providing a confidence measure for the
    /// correctness of the decryption. Higher values (closer to 1.0) indicate
    /// more likely correct decryptions.
    pub meaningful_ratio: Option<f32>,
}

impl Display for DecodedResult {
    /// Formats the DecodedResult for display with colored output.
    ///
    /// The output format shows the key in blue, the text in green, and
    /// the meaningfulness ratio (if present) in yellow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cryptan::DecodedResult;
    ///
    /// let result = DecodedResult {
    ///     text: "HELLO WORLD".to_string(),
    ///     key: 3,
    ///     meaningful_ratio: Some(0.95),
    /// };
    ///
    /// // Prints: [Key: 3, meaningful Ratio: 0.95] HELLO WORLD
    /// // with appropriate colors
    /// println!("{}", result);
    /// ```
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