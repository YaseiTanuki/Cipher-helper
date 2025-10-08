use std::fs;
use std::collections::HashSet;

/// Normalizes a shift key to a valid range for Caesar cipher operations.
///
/// Caesar ciphers operate on the 26-letter alphabet, so any shift value can be
/// reduced modulo 26. This function performs that normalization using Euclidean
/// remainder to handle negative values correctly.
///
/// # Arguments
///
/// * `key` - The raw shift key (can be any i8 value, including negative)
///
/// # Returns
///
/// A normalized shift value between 0 and 25 inclusive.
///
/// # Examples
///
/// ```rust
/// use cryptan::normalize_shift;
///
/// assert_eq!(normalize_shift(3), 3);
/// assert_eq!(normalize_shift(29), 3);  // 29 % 26 = 3
/// assert_eq!(normalize_shift(-1), 25); // -1 + 26 = 25
/// assert_eq!(normalize_shift(-27), 25); // -27 + 52 = 25, but Euclidean remainder gives 25
/// ```
pub fn normalize_shift(key: i8) -> u8 {
    (key as i16).rem_euclid(26) as u8
}

/// Parses a 16-bit integer key into an 8-bit integer, validating the range.
///
/// This function ensures that the provided key fits within the range of i8,
/// which is used internally by the Caesar cipher implementation.
///
/// # Arguments
///
/// * `k` - The key value to parse
///
/// # Returns
///
/// * `Ok(i8)` - The parsed key if it's within valid range
/// * `Err(String)` - An error message if the key is out of range
///
/// # Examples
///
/// ```rust
/// use cryptan::parse_key_i8;
///
/// assert_eq!(parse_key_i8(3), Ok(3));
/// assert_eq!(parse_key_i8(127), Ok(127));
/// assert_eq!(parse_key_i8(128), Err("Key out of i8 range: 128".to_string()));
/// assert_eq!(parse_key_i8(-129), Err("Key out of i8 range: -129".to_string()));
/// ```
pub fn parse_key_i8(k: i16) -> Result<i8, String> {
    if k < i8::MIN as i16 || k > i8::MAX as i16 {
        Err(format!("Key out of i8 range: {}", k))
    } else {
        Ok(k as i8)
    }
}

/// Loads a word list from a file for use in cryptanalysis.
///
/// This function reads a text file containing a list of words (one per line)
/// and returns them as a HashSet for efficient lookup during meaningfulness
/// analysis.
///
/// # Arguments
///
/// * `path` - Path to the word list file
///
/// # Returns
///
/// A HashSet containing all words from the file, trimmed and converted to owned strings.
///
/// # Panics
///
/// Panics if the file cannot be read.
///
/// # Examples
///
/// ```rust,no_run
/// use cryptan::load_set;
/// use std::collections::HashSet;
///
/// let words: HashSet<String> = load_set("public/words.txt");
/// assert!(words.contains("hello"));
/// assert!(words.contains("world"));
/// ```
pub fn load_set(path: &str) -> HashSet<String> {
    let data = fs::read_to_string(path).expect("read");
    data.lines().map(|s| s.trim().to_owned()).collect()
}

/// Calculates the meaningfulness ratio of text based on word frequency analysis.
///
/// This function analyzes how many words in the given text appear in a reference
/// word list, providing a score between 0.0 and 1.0 indicating how likely the
/// text is to be valid English (or whatever language the word list represents).
///
/// # Arguments
///
/// * `text` - The text to analyze
/// * `wordlist` - Reference word list for comparison
///
/// # Returns
///
/// * `Ok(f32)` - The meaningfulness ratio (0.0 to 1.0)
/// * `Err(Box<dyn std::error::Error>)` - An error if analysis fails
///
/// # Examples
///
/// ```rust
/// use cryptan::{load_set, meaningful_ratio};
/// use std::collections::HashSet;
///
/// let wordlist: HashSet<String> = ["hello", "world", "test"].iter().map(|s| s.to_string()).collect();
/// let ratio = meaningful_ratio("hello world", &wordlist);
/// assert_eq!(ratio.unwrap(), 1.0); // Both words found
///
/// let ratio = meaningful_ratio("xyz abc", &wordlist);
/// assert_eq!(ratio.unwrap(), 0.0); // No words found
/// ```
pub fn meaningful_ratio(text: &str, wordlist: &HashSet<String>) -> Result<f32, Box<dyn std::error::Error>> {
    let word_set: std::collections::HashSet<String> = wordlist.into_iter().map(String::from).collect();

    let tokens: Vec<String> = text
        .split_whitespace()
        .map(|t| t.trim_matches(|c: char| !c.is_ascii_alphabetic()).to_ascii_lowercase())
        .filter(|t| !t.is_empty())
        .collect();

    if tokens.is_empty() {
        return Ok(0.0);
    }

    let meaningful_count = tokens.iter().filter(|t| word_set.contains(*t)).count();
    Ok(meaningful_count as f32 / tokens.len() as f32)
}