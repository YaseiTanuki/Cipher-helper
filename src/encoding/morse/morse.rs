//! Morse code encoder/decoder implementation.
//!
//! This module contains the [`Morse`] struct which provides encoding and decoding
//! functionality for Morse code. It supports both International and American
//! Morse code standards and implements the [`Codec`] trait.
//!
//! The Morse struct maintains bidirectional mappings between characters and their
//! Morse code representations, allowing for efficient encoding and decoding operations.

use crate::encoding::morse::morse_code::{AMERICAN_MORSE, INTERNATIONAL_MORSE, MorseLang};
use crate::build_morse_maps;
use crate::traits::Codec;

use std::collections::HashMap;

/// A Morse code encoder and decoder supporting International and American standards.
///
/// The `Morse` struct provides methods to encode text into Morse code and decode
/// Morse code back into text. It supports switching between International Morse Code
/// (used worldwide) and American Morse Code standards.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use cryptan::Morse;
/// use cryptan::Codec;
///
/// let morse = Morse::new();
/// let encoded = morse.encode("HELLO");
/// println!("Encoded: {}", encoded); // "... .  .-..  .-..  ---"
///
/// let decoded = morse.decode(&encoded);
/// println!("Decoded: {}", decoded); // "HELLO"
/// ```
///
/// ## Language Switching
///
/// ```rust
/// use cryptan::Morse;
/// use cryptan::Codec;
///
/// let mut morse = Morse::new();
///
/// // Start with International Morse (default)
/// let international = morse.encode("A");
/// println!("International 'A': {}", international); // ".-"
///
/// // Switch to American Morse
/// morse.set_lang("american");
/// let american = morse.encode("A");
/// println!("American 'A': {}", american); // ".-"
///
/// // Can also create directly with specific language
/// let american_morse = Morse::from_lang("american");
/// ```
pub struct Morse {
    lang: MorseLang,
    pub map: HashMap<char, String>,
    pub rev_map: HashMap<String, char>,
}

impl Morse {
    /// Creates a new Morse encoder/decoder with International Morse Code as default.
    ///
    /// This constructor initializes the encoder with the International Morse Code
    /// standard, which is the most widely used standard for amateur radio and
    /// maritime communications worldwide.
    ///
    /// # Returns
    ///
    /// A `Morse` instance configured for International Morse Code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cryptan::Morse;
    /// use cryptan::Codec;
    ///
    /// let morse = Morse::new();
    /// let encoded = morse.encode("SOS");
    /// assert_eq!(encoded, "... --- ...");
    /// ```
    pub fn new() -> Self {
        let lang = MorseLang::International;
        let (map, rev_map) = build_morse_maps(INTERNATIONAL_MORSE);
        Self { lang, map, rev_map }
    }

    pub fn from_lang(lang: &str) -> Self {
        let lang = match lang.to_lowercase().as_str() {
            "american" => MorseLang::American,
            _ => MorseLang::International,
        };
        let (map, rev_map) = match lang {
            MorseLang::International => build_morse_maps(INTERNATIONAL_MORSE),
            MorseLang::American => build_morse_maps(AMERICAN_MORSE),
        };
        Self { lang, map, rev_map }
    }

    pub fn set_lang(&mut self, lang: &str) {
        let lang = match lang.to_lowercase().as_str() {
            "american" => MorseLang::American,
            _ => MorseLang::International,
        };
        let (map, rev_map) = match lang {
            MorseLang::International => build_morse_maps(INTERNATIONAL_MORSE),
            MorseLang::American => build_morse_maps(AMERICAN_MORSE),
        };
        self.lang = lang;
        self.map = map;
        self.rev_map = rev_map;
    }

    pub fn get_lang(&self) -> String {
        match self.lang {
            MorseLang::International => "International".to_string(),
            MorseLang::American => "American".to_string(),
        }
    }
}

impl Codec for Morse {
    /// Encodes text into Morse code.
    ///
    /// This method converts alphabetic characters and numbers to their Morse code
    /// equivalents using the currently selected language standard. Spaces are
    /// represented as forward slashes (`/`) in the output.
    ///
    /// # Arguments
    ///
    /// * `input` - The text to encode into Morse code
    ///
    /// # Returns
    ///
    /// A string containing the Morse code representation where:
    /// - Each character is represented by its Morse code sequence
    /// - Character codes are separated by spaces
    /// - Words are separated by forward slashes (`/`)
    /// - Unsupported characters are skipped with a warning message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cryptan::Morse;
    /// use cryptan::Codec;
    ///
    /// let morse = Morse::new();
    ///
    /// // Encode a simple message
    /// let encoded = morse.encode("HI");
    /// assert_eq!(encoded,(".... .."));
    ///
    /// // Encode with spaces (word separators)
    /// let encoded = morse.encode("HI THERE");
    /// assert_eq!(encoded, ".... .. / - .... . .-. .");
    /// ```
    fn encode(&self, input: &str) -> String {
        let mut result = Vec::new();
        for ch in input.chars() {
            match ch {
                ' ' => result.push("/".to_string()),

                ch if ch.is_ascii_alphanumeric() => {
                    if let Some(code) = self.map.get(&ch.to_ascii_uppercase()) {
                        result.push(code.clone());
                    }
                },

                _ => {
                    println!("Warning: Unsupported character '{}', skipping.", ch);
                }
            }
        }
        result.join(" ")
    }

    /// Decodes Morse code into text.
    ///
    /// This method converts Morse code sequences back into their corresponding
    /// alphabetic characters and numbers using the currently selected language standard.
    /// Forward slashes (`/`) in the input are interpreted as word separators.
    ///
    /// # Arguments
    ///
    /// * `input` - The Morse code string to decode
    ///
    /// # Returns
    ///
    /// A string containing the decoded text where:
    /// - Morse code sequences are converted back to characters
    /// - Forward slashes (`/`) are converted to spaces (word separators)
    /// - Unsupported Morse code sequences are skipped with a warning message
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cryptan::Morse;
    /// use cryptan::Codec;
    ///
    /// let morse = Morse::new();
    ///
    /// // Decode a simple message
    /// let decoded = morse.decode(".... ..");
    /// assert_eq!(decoded, "HI");
    ///
    /// // Decode with word separators
    /// let decoded = morse.decode(".... .. / .-- --- .-. .-.. -..");
    /// assert_eq!(decoded, "HI WORLD");
    /// ```
    fn decode(&self, input: &str) -> String {
        let mut result = Vec::new();
        for code in input.split_whitespace() {
            match code {
                "/" => result.push(' '.to_string()),

                code if self.rev_map.get(&code.to_string()).is_some() => {
                    if let Some(&ch) = self.rev_map.get(code) {
                        result.push(ch.to_string());
                    }
                },

                _ => {
                    println!("Warning: Unsupported Morse code '{}', skipping.", code);
                }
            }
        }
        result.join("")
    }

}