use crate::{normalize_shift, load_set};
use crate::{ClassicalCipher, BruteForce};
use crate::DecodedResult;
use log::{warn, info};
use crate::{meaningful_ratio};

pub struct CaesarCipher {
    key: i8,
}

impl CaesarCipher {

    pub fn new() -> Self {
        Self {
            key: 0,
        }
    }

    pub fn from_key(key: i8) -> Self {
        Self {
            key,
        }
    }

    pub fn set_key(&mut self, key: i8) {
        self.key = key;
    }
}

impl ClassicalCipher for CaesarCipher {
    /// Encrypts `self.plain` with the provided rotation key and updates the encrypted buffer.
    fn encrypt(&self, input: &str) -> String {
        if self.key == 0 {
            warn!("Key is 0, no encryption performed.");
            return input.to_string();
        }

        let shift = normalize_shift(self.key);
        let mut out = String::with_capacity(input.len());

        for &b in input.as_bytes() {
            let new_b = if b.is_ascii_uppercase() {
                ((b - b'A' + shift) % 26) + b'A'
            } else if b.is_ascii_lowercase() {
                ((b - b'a' + shift) % 26) + b'a'
            } else {
                b
            };
            out.push(new_b as char);
        }

        out
    }

    fn decrypt(&self, input: &str) -> String {
        if self.key == 0 {
            warn!("Key is 0, no decryption performed.");
            return input.to_string();
        }

        let shift = normalize_shift(-self.key);
        let mut out = String::with_capacity(input.len());

        for &b in input.as_bytes() {
            let new_b = if b.is_ascii_uppercase() {
                ((b - b'A' + shift) % 26) + b'A'
            } else if b.is_ascii_lowercase() {
                ((b - b'a' + shift) % 26) + b'a'
            } else {
                b
            };
            out.push(new_b as char);
        }

        out
    }
}

impl BruteForce for CaesarCipher {
    /// Attempts to recover plaintext across all 26 rotations, filtering by an optional threshold.
    fn bruteforce(&mut self, input: &str, threshold: Option<f32>) -> Vec<DecodedResult> {
        let mut results: Vec<DecodedResult> = Vec::new();

        let mut threshold_value: f32;

        if threshold.is_none() {
            info!("No threshold provided, returning all candidates.");
            threshold_value = 0.0;
        }
        else {
            threshold_value = threshold.unwrap();
            if threshold_value < 0.0 || threshold_value > 1.0 {
                warn!("Threshold {} out of range [0.0, 1.0], defaulting to 0.0", threshold_value);
                threshold_value = 0.0;
            }
        }
        let wordslist = load_set("public/words.txt");
        for key in 0..26 {
            self.set_key(key as i8);
            let decrypted_text = self.decrypt(input);
            let ratio = match meaningful_ratio(&decrypted_text, &wordslist) {
                Ok(r) => Some(r),
                Err(_) => None,
            };
            if ratio.is_none() {
                warn!("Meaningful ratio computation failed, possibly due to missing word frequency data.");
            }
            else if ratio.unwrap_or(0.0) < threshold_value {
                continue;
            }
            let result = DecodedResult {
                text: decrypted_text,
                key: key as u8,
                meaningful_ratio: ratio,
            };
            results.push(result);
        }

        results
    }
}