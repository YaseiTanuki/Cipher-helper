// src/caesar_cipher.rs

use crate::traits::{Decrypt, Encrypt, BruteForce};
use std::fmt::{Display, Formatter, Result as FmtResult};
use colored::*;
use crate::normalize_shift;

pub struct CaesarCipher {
    plain: String,
    encrypted_text: String,
}

#[derive(Debug)]
pub struct DecodedResult {
    pub text: String,
    pub key: u8,
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

impl CaesarCipher {
    pub fn new() -> Self {
        Self {
            plain: String::new(),
            encrypted_text: String::new(),
        }
    }

    pub fn from_encrypted(encrypted: String) -> Self {
        Self {
            plain: String::new(),
            encrypted_text: encrypted,
        }
    }

    pub fn from_plain(plain: String) -> Self {
        Self {
            plain: plain, 
            encrypted_text: String::new(),
        }
    }

    pub fn set_plain(&mut self, new_plain: String) -> &mut Self {
        self.plain = new_plain;
        self
    }

    pub fn set_encrypted_text(&mut self, new_encrypted_text: String) -> &mut Self {
        self.encrypted_text = new_encrypted_text;
        self
    }

    pub fn get_plain(&self) -> String {
        self.plain.clone()
    }

    pub fn get_encrypted_text(&self) -> String {
        self.encrypted_text.clone()
    }
}

impl Encrypt for CaesarCipher {
    fn encrypt(&mut self, key: Option<i8>) -> String {
        if self.plain.is_empty() {
            eprint!("[warn] Plain text is empty, nothing to encrypt.");
            return String::new();
        }
        let shift = normalize_shift(key.expect("Key is required for encryption"));
        let mut out = String::with_capacity(self.plain.len());

        for &b in self.plain.as_bytes() {
            let new_b = if b.is_ascii_uppercase() {
                ((b - b'A' + shift) % 26) + b'A'
            } else if b.is_ascii_lowercase() {
                ((b - b'a' + shift) % 26) + b'a'
            } else {
                b
            };
            out.push(new_b as char);
        }

        self.set_encrypted_text(out);

        self.encrypted_text.clone()
    }
}

impl Decrypt for CaesarCipher {
    fn decrypt(&self, key: Option<i8>) -> DecodedResult {
        let key_val = key.expect("Key is required for decryption");
        let shift = normalize_shift(-key_val);
        let mut out = String::with_capacity(self.encrypted_text.len());

        for &b in self.encrypted_text.as_bytes() {
            let new_b = if b.is_ascii_uppercase() {
                ((b - b'A' + shift) % 26) + b'A'
            } else if b.is_ascii_lowercase() {
                ((b - b'a' + shift) % 26) + b'a'
            } else {
                b
            };
            out.push(new_b as char);
        }

        let ratio = match crate::py_meaningful_ratio(&out) {
            Ok(r) => Some(r),
            Err(_) => None,
        };

        DecodedResult {
            text: out,
            key: key_val as u8,
            meaningful_ratio: ratio,
        }
    }
}

impl BruteForce for CaesarCipher {
    fn brute_force(&self, threshold: Option<f32>) -> Vec<DecodedResult> {
        let mut results: Vec<DecodedResult> = Vec::new();
        let mut warned = false;

        for key in 0..26 {
            let decrypted = self.decrypt(Some(key));
            let ratio = decrypted.meaningful_ratio.unwrap_or(0.0);

            if decrypted.meaningful_ratio.is_none() && !warned {
                eprintln!("[warn] Python wordfreq unavailable or error when computing meaningful ratio.");
                warned = true;
            }

            if ratio >= threshold.unwrap_or(0.5) {
                results.push(decrypted);
            }
        }

        results
    }
    
    fn brute_force_all(&self) -> Vec<DecodedResult> {
        let mut results: Vec<DecodedResult> = Vec::new();

        for key in 0..26 {
            let decrypted = self.decrypt(Some(key));
            results.push(decrypted);
        }

        results
    }
}
