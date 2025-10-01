// src/caesar_cipher.rs

use crate::traits::{Decode, Encode, BruteForce};
use std::fmt::{Display, Formatter, Result as FmtResult};
use colored::*;

pub struct CaesarCipher {
    plain: String,
    encoded_text: String,
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
            encoded_text: String::new(),
        }
    }

    pub fn set_plain(&mut self, new_plain: String) {
        self.plain = new_plain;
    }

    pub fn set_encoded_text(&mut self, new_encoded_text: String) {
        self.encoded_text = new_encoded_text;
    }

    // Trả về &str thay vì clone nếu cần hiệu năng
    pub fn get_plain(&self) -> &str {
        &self.plain
    }

    pub fn get_encoded_text(&self) -> &str {
        &self.encoded_text
    }
}

// helper: chuẩn hoá shift về [0,25]
fn normalize_shift(key: i8) -> u8 {
    (key as i16).rem_euclid(26) as u8
}

impl Encode for CaesarCipher {
    fn encode(&self, key: Option<i8>) -> String {
        let shift = normalize_shift(key.expect("Key is required for encoding"));
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

        out
    }
}

impl Decode for CaesarCipher {
    fn decode(&self, key: Option<i8>) -> DecodedResult {
        let key_val = key.expect("Key is required for decoding");
        let shift = normalize_shift(-key_val);
        let mut out = String::with_capacity(self.encoded_text.len());

        for &b in self.encoded_text.as_bytes() {
            let new_b = if b.is_ascii_uppercase() {
                ((b - b'A' + shift) % 26) + b'A'
            } else if b.is_ascii_lowercase() {
                ((b - b'a' + shift) % 26) + b'a'
            } else {
                b
            };
            out.push(new_b as char);
        }

        // Tính meaningful_ratio nếu có py_dict, hoặc None nếu không muốn tính
        let ratio = match crate::py_dict::py_meaningful_ratio(&out) {
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
    // Trả về các kết quả có meaningful_ratio > threshold (nếu có)
    fn brute_force(&self, threshold: Option<f32>) -> Vec<DecodedResult> {
        let mut results: Vec<DecodedResult> = Vec::new();
        let mut warned = false;

        for key in 0..26 {
            let decoded = self.decode(Some(key));
            let ratio = decoded.meaningful_ratio.unwrap_or(0.0);

            if decoded.meaningful_ratio.is_none() && !warned {
                eprintln!("[warn] Python wordfreq unavailable or error when computing meaningful ratio.");
                warned = true;
            }

            if ratio >= threshold.unwrap_or(0.5) {
                results.push(decoded);
            }
        }

        results
    }

    // Trả về tất cả kết quả (không filter)
    fn brute_force_all(&self) -> Vec<DecodedResult> {
        let mut results: Vec<DecodedResult> = Vec::new();

        for key in 0..26 {
            let decoded = self.decode(Some(key));
            results.push(decoded);
        }

        results
    }
}