use std::fmt::{Display, Formatter, Result as FmtResult};
use colored::*;

#[derive(Debug)]
pub struct DecodedResult {
    /// The decrypted plain text.
    pub text: String,
    /// The rotation key that generated `text`.
    pub key: u8,
    /// Optional ratio of meaningful tokens, if Python integration succeeded.
    pub meaningful_ratio: Option<f32>,
}

impl Display for DecodedResult {
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