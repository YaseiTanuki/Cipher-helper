//! # Caesar Cipher Command-Line Interface
//!
//! This module provides a command-line interface for the Caesar cipher implementation,
//! allowing users to encrypt, decrypt, and perform brute-force attacks on text.
//!
//! ## Overview
//!
//! The CLI supports three main operations:
//! - **Encrypt**: Transform plaintext to ciphertext using a specified key
//! - **Decrypt**: Transform ciphertext back to plaintext using a specified key
//! - **Brute-force**: Attempt to recover plaintext by trying all possible keys
//!
//! ## Usage
//!
//! ### Basic Commands
//!
//! ```bash
//! # Encrypt text with key 3
//! caesar encrypt 3 "HELLO WORLD"
//!
//! # Decrypt text with key 3
//! caesar decrypt 3 "KHOOR ZRUOG"
//!
//! # Brute-force attack (try all keys)
//! caesar brute "KHOOR ZRUOG"
//!
//! # Brute-force with confidence threshold
//! caesar brute --threshold 0.5 "KHOOR ZRUOG"
//! ```
//!
//! ### Command Details
//!
//! #### Encrypt
//! ```bash
//! caesar encrypt <KEY> <PLAINTEXT>
//! ```
//!
//! - `<KEY>`: Shift value (can be negative or >26, will be normalized)
//! - `<PLAINTEXT>`: Text to encrypt (wrap in quotes for multi-word)
//!
//! #### Decrypt
//! ```bash
//! caesar decrypt <KEY> <CIPHERTEXT>
//! ```
//!
//! - `<KEY>`: Shift value used for encryption
//! - `<CIPHERTEXT>`: Text to decrypt (wrap in quotes for multi-word)
//!
//! #### Brute-force
//! ```bash
//! caesar brute [OPTIONS] <CIPHERTEXT>
//! ```
//!
//! - `<CIPHERTEXT>`: Text to analyze
//! - `--threshold <THRESHOLD>`: Minimum confidence score (0.0-1.0)
//!
//! ## Examples
//!
//! ```bash
//! # Simple encryption
//! $ caesar encrypt 13 "attack at dawn"
//! nggnpx ng qnja
//!
//! # Decryption with the same key
//! $ caesar decrypt 13 "nggnpx ng qnja"
//! attack at dawn
//!
//! # Brute-force attack on short ciphertext
//! $ caesar brute "URYYB"
//! 01. [Key: 18, meaningful Ratio: 1.00] HELLO
//! 02. [Key: 19, meaningful Ratio: 0.00] GDKKN
//! 03. [Key: 20, meaningful Ratio: 0.00] FCJJM
//! [... 23 more results ...]
//! ```
//!
//! ## Algorithm Notes
//!
//! - Keys are normalized using modulo 26 arithmetic
//! - Only alphabetic characters are transformed; others pass through unchanged
//! - Brute-force results are sorted by confidence score (highest first)
//! - Confidence scoring uses word frequency analysis against a built-in word list
//!
//! ## Exit Codes
//!
//! - `0`: Successful operation
//! - `1`: Invalid arguments or key out of range


use clap::{Parser, Subcommand};
use cryptan::{BruteForce, CaesarCipher, ClassicalCipher};
use cryptan::Morse;
use cryptan::Codec;
use cryptan::{parse_key_i8, print_usage};

#[derive(Parser)]
#[command(name = "caesar_cipher_method")]
#[command(version = "0.1.0")]
#[command(about = "Caesar cipher CLI (also usable as lib)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    group: CipherGroup,
}

#[derive(Subcommand)]
enum CipherGroup {
    Caesar {
        #[command(subcommand)]
        action: CaesarCommand,
    },
    Morse {
        #[command(subcommand)]
        action: MorseCommand,
    },
}

#[derive(Subcommand)]
enum CaesarCommand {
    /// Encrypt plain text with a key
    Encrypt { key: i16, plain_text: String },
    /// Decrypt encrypted text with a key
    Decrypt { key: i16, encrypted_text: String },
    /// Brute-force attack to find the key and decrypt text
    Brute { encoded_text: String, threshold: Option<f32> },
}

#[derive(Subcommand)]
enum MorseCommand {
    /// Encode text to Morse code
    Encode {
        text: String,
        #[arg(long, default_value_t = String::from("international"))]
        lang: String, 
    },
    /// Decode Morse code to text
    Decode {
        code: String,
        #[arg(long, default_value_t = String::from("international"))]
        lang: String, 
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.group {
        CipherGroup::Caesar { action } => match action {
            CaesarCommand::Encrypt { key, plain_text } => {
                let key = match parse_key_i8(key) {
                    Ok(k) => k,
                    Err(e) => {
                        eprintln!("Invalid <key>: {} ({})", key, e);
                        print_usage();
                        return;
                    }
                };

                let caesar = CaesarCipher::from_key(key);
                println!("{}", caesar.encrypt(&plain_text));
            }

            CaesarCommand::Decrypt { key, encrypted_text } => {
                let key = match parse_key_i8(key) {
                    Ok(k) => k,
                    Err(e) => {
                        eprintln!("Invalid <key>: {} ({})", key, e);
                        print_usage();
                        return;
                    }
                };

                let caesar = CaesarCipher::from_key(key);
                println!("Decrypted result: {}", caesar.decrypt(&encrypted_text));
            }

            CaesarCommand::Brute { encoded_text, threshold } => {
                let mut caesar = CaesarCipher::from_key(0);
                let mut results = caesar.bruteforce(&encoded_text, threshold);

                results.sort_by(|a, b| {
                    let ra = a.meaningful_ratio.unwrap_or(0.0);
                    let rb = b.meaningful_ratio.unwrap_or(0.0);
                    rb.partial_cmp(&ra).unwrap_or(std::cmp::Ordering::Equal)
                });

                if results.is_empty() {
                    println!("No meaningful result found.");
                } else {
                    for (i, r) in results.into_iter().enumerate() {
                        println!("{:02}. {}", i + 1, r);
                    }
                }
            }
        },

        CipherGroup::Morse { action } => match action {
            MorseCommand::Encode { text, lang } => {
                let morse = Morse::from_lang(&lang);
                println!("{}", morse.encode(&text));
            }
            MorseCommand::Decode { code, lang } => {
                let morse = Morse::from_lang(&lang);
                println!("{}", morse.decode(&code));
            }
        },
    }
}
