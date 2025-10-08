use crate::DecodedResult;

/// Core trait for classical cipher implementations.
///
/// This trait defines the fundamental operations that all classical ciphers should support:
/// encryption and decryption of text. Classical ciphers typically operate on alphabetic
/// characters while preserving non-alphabetic characters.
///
/// # Examples
///
/// ```rust
/// use cryptan::{ClassicalCipher, CaesarCipher};
///
/// let cipher = CaesarCipher::from_key(3);
/// let encrypted = cipher.encrypt("Hello, World!");
/// let decrypted = cipher.decrypt(&encrypted);
///
/// assert_eq!(decrypted, "Hello, World!");
/// ```
pub trait ClassicalCipher {
    /// Encrypts the given plaintext.
    ///
    /// # Arguments
    ///
    /// * `input` - The plaintext to encrypt
    ///
    /// # Returns
    ///
    /// The encrypted ciphertext as a String
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cryptan::{ClassicalCipher, CaesarCipher};
    ///
    /// let cipher = CaesarCipher::from_key(3);
    /// let result = cipher.encrypt("ABC");
    /// // Result depends on the specific cipher implementation
    /// ```
    fn encrypt(&self, input: &str) -> String;

    /// Decrypts the given ciphertext.
    ///
    /// # Arguments
    ///
    /// * `input` - The ciphertext to decrypt
    ///
    /// # Returns
    ///
    /// The decrypted plaintext as a String
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cryptan::{ClassicalCipher, CaesarCipher};
    ///
    /// let cipher = CaesarCipher::from_key(3);
    /// let encrypted = cipher.encrypt("ABC");
    /// let decrypted = cipher.decrypt(&encrypted);
    /// assert_eq!(decrypted, "ABC");
    /// ```
    fn decrypt(&self, input: &str) -> String;
}

/// Trait for ciphers that support brute-force cryptanalysis.
///
/// This trait enables systematic testing of all possible keys to recover plaintext.
/// It's particularly useful for classical ciphers with small key spaces like Caesar cipher.
///
/// # Examples
///
/// ```rust
/// use cryptan::{BruteForce, CaesarCipher};
///
/// let mut cipher = CaesarCipher::from_key(0);
/// let candidates = cipher.bruteforce("KHOOR ZRUOG", Some(0.3));
///
/// for candidate in candidates {
///     println!("Key: {}, Text: '{}', Confidence: {:.3}",
///              candidate.key,
///              candidate.text,
///              candidate.meaningful_ratio.unwrap_or(0.0));
/// }
/// ```
pub trait BruteForce {
    /// Attempts to recover plaintext across all possible keys, filtering by meaningfulness.
    ///
    /// This method systematically tries all possible keys and evaluates each result
    /// for "meaningfulness" using word frequency analysis. Results below the threshold
    /// are filtered out.
    ///
    /// # Arguments
    ///
    /// * `input` - The ciphertext to analyze
    /// * `threshold` - Optional minimum meaningfulness score (0.0 to 1.0). If None, all candidates are returned.
    ///
    /// # Returns
    ///
    /// A vector of [`DecodedResult`] structs containing candidate plaintexts with their
    /// associated keys and meaningfulness scores, sorted by score (highest first).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cryptan::{BruteForce, CaesarCipher};
    ///
    /// let mut cipher = CaesarCipher::from_key(0);
    ///
    /// // Get all candidates
    /// let all_results = cipher.bruteforce("KHOOR", None);
    ///
    /// // Get only high-confidence results
    /// let good_results = cipher.bruteforce("KHOOR", Some(0.7));
    /// ```
    ///
    /// # Implementation Notes
    ///
    /// - Results are sorted by meaningfulness score in descending order
    /// - The meaningfulness calculation uses word frequency analysis against a built-in word list
    /// - For ciphers with large key spaces, this operation may be computationally expensive
    fn bruteforce(&mut self, input: &str, threshold: Option<f32>) -> Vec<DecodedResult>;
}
