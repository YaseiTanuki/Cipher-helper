// src/caesar_cipher.rs

use crate::traits::{Decode, Encode, BruteForce};

pub struct CaesarCipher {
    plain: String,
    encoded_text: String,
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
    fn decode(&self, key: Option<i8>) -> String {
        // decode bằng cách dịch ngược: -key
        let shift = normalize_shift(-key.expect("Key is required for decoding"));
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

        out
    }
}

impl BruteForce for CaesarCipher {
    fn brute_force(&self) {
        let mut warned = false;

        for key in 0..26 {
            let decoded = self.decode(Some(key));
            // Gọi hàm py_meaningful_ratio giống trước — giữ behavior cũ
            let ratio = match crate::py_dict::py_meaningful_ratio(&decoded) {
                Ok(r) => r,
                Err(e) => {
                    if !warned {
                        eprintln!("[warn] Python wordfreq unavailable: {}", e);
                        warned = true;
                    }
                    0.0
                }
            };

            if ratio > 0.5 {
                println!(
                    "KEY: {} (meaningful: {:.0}%)\nDECODED TEXT: {}\n",
                    key,
                    ratio * 100.0,
                    decoded
                );
            }
        }
    }

    fn brute_force_all(&self) {
        for key in 0..26 {
            let decoded = self.decode(Some(key));
            println!("KEY: {}\nDECODED TEXT: {}\n", key, decoded);
        }
    }
}
