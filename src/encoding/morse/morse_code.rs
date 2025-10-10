//! Morse code character mappings and language definitions.
//!
//! This module defines the character-to-Morse code mappings for both International
//! and American Morse code standards, along with the enumeration of supported
//! languages.
//!
//! The mappings are provided as static arrays that can be used to build the
//! bidirectional character/code mappings needed for encoding and decoding operations.

/// Enumeration of supported Morse code languages.
///
/// This enum defines the two main Morse code standards supported by the library.
/// Each variant corresponds to a different character mapping table.
pub enum MorseLang {
    /// International Morse Code - the standard used worldwide for amateur radio
    /// and maritime communications. This is the most commonly used standard today.
    International,
    /// American Morse Code - an older standard that was primarily used in North America.
    /// It differs from International Morse in some character mappings.
    American,
}

/// International Morse Code mapping.
///
/// This array contains the character-to-Morse code mappings for the International
/// Morse Code standard. Each tuple consists of a character and its corresponding
/// Morse code sequence represented as a string of dots and dashes.
///
/// The International Morse Code is the current standard used worldwide and is
/// particularly important for amateur radio communications.
///
/// # Character Coverage
///
/// The mapping includes:
/// - All alphabetic characters (A-Z)
/// - Numeric digits (0-9)
///
/// # Morse Code Notation
///
/// - `.` represents a dot (dit)
/// - `-` represents a dash (dah)
/// - Each character sequence is a unique combination of dots and dashes
///
/// # Examples
///
/// ```rust
/// use cryptan::encoding::morse::morse_code::INTERNATIONAL_MORSE;
///
/// // Find the Morse code for 'A'
/// let a_code = INTERNATIONAL_MORSE.iter()
///     .find(|(ch, _)| *ch == "A")
///     .map(|(_, code)| *code);
/// assert_eq!(a_code, Some(".-"));
///
/// // Find the Morse code for 'S' (SOS distress signal)
/// let s_code = INTERNATIONAL_MORSE.iter()
///     .find(|(ch, _)| *ch == "S")
///     .map(|(_, code)| *code);
/// assert_eq!(s_code, Some("..."));
/// ```
pub const INTERNATIONAL_MORSE: &[(&str, &str)] = &[
    ("A", ".-"),
    ("B", "-..."),
    ("C", "-.-."),
    ("D", "-.."),
    ("E", "."),
    ("F", "..-."),
    ("G", "--."),
    ("H", "...."),
    ("I", ".."),
    ("J", ".---"),
    ("K", "-.-"),
    ("L", ".-.."),
    ("M", "--"),
    ("N", "-."),
    ("O", "---"),
    ("P", ".--."),
    ("Q", "--.-"),
    ("R", ".-."),
    ("S", "..."),
    ("T", "-"),
    ("U", "..-"),
    ("V", "...-"),
    ("W", ".--"),
    ("X", "-..-"),
    ("Y", "-.--"),
    ("Z", "--.."),
    ("0", "-----"),
    ("1", ".----"),
    ("2", "..---"),
    ("3", "...--"),
    ("4", "....-"),
    ("5", "....."),
    ("6", "-...."),
    ("7", "--..."),
    ("8", "---.."),
    ("9", "----."),
];

/// American Morse Code mapping.
///
/// This array contains the character-to-Morse code mappings for the American
/// Morse Code standard. Each tuple consists of a character and its corresponding
/// Morse code sequence represented as a string of dots and dashes.
///
/// American Morse Code was developed in the 1840s and was widely used in North America.
/// It differs from International Morse in several character mappings and was largely
/// superseded by International Morse Code in the early 20th century.
///
/// # Character Coverage
///
/// The mapping includes:
/// - All alphabetic characters (A-Z)
/// - Numeric digits (0-9)
///
/// # Morse Code Notation
///
/// - `.` represents a dot (dit)
/// - `-` represents a dash (dah)
/// - Some characters have different representations compared to International Morse
///
/// # Historical Context
///
/// American Morse Code was particularly important for railroad communications
/// and early telegraph systems in North America. While largely obsolete today,
/// understanding both standards provides historical context for Morse code evolution.
///
/// # Examples
///
/// ```rust
/// use cryptan::encoding::morse::morse_code::AMERICAN_MORSE;
///
/// // Find the Morse code for 'C' (different from International)
/// let c_code = AMERICAN_MORSE.iter()
///     .find(|(ch, _)| *ch == "C")
///     .map(|(_, code)| *code);
/// assert_eq!(c_code, Some(".. ."));
///
/// // Compare with International Morse where 'C' is "-.-."
/// use cryptan::encoding::morse::morse_code::INTERNATIONAL_MORSE;
/// let int_c_code = INTERNATIONAL_MORSE.iter()
///     .find(|(ch, _)| *ch == "C")
///     .map(|(_, code)| *code);
/// assert_eq!(int_c_code, Some("-.-."));
/// ```
pub const AMERICAN_MORSE: &[(&str, &str)] = &[
    ("A", ".-"),
    ("B", "-..."),
    ("C", ".. ."),  // Different from International Morse
    ("D", "-.."),
    ("E", "."),
    ("F", "..-."),
    ("G", "--."),
    ("H", "...."),
    ("I", ".."),
    ("J", ".---"),
    ("K", "-.-"),
    ("L", ".-.."),
    ("M", "--"),
    ("N", "-."),
    ("O", "---"),
    ("P", ".--."),
    ("Q", "--.-"),
    ("R", ".-."),
    ("S", "..."),
    ("T", "-"),
    ("U", "..-"),
    ("V", "...-"),
    ("W", ".--"),
    ("X", "-..-"),
    ("Y", "-.--"),
    ("Z", "--.."),
    ("0", "-----"),
    ("1", ".----"),
    ("2", "..---"),
    ("3", "...--"),
    ("4", "....-"),
    ("5", "....."),
    ("6", "-...."),
    ("7", "--..."),
    ("8", "---.."),
    ("9", "----."),
];
