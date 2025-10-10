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
    Encrypt { key: i16, plain_text: String },
    Decrypt { key: i16, encrypted_text: String },
    Brute { encoded_text: String, threshold: Option<f32> },
}

#[derive(Subcommand)]
enum MorseCommand {
    Encode {
        text: String,
        #[arg(long, default_value_t = String::from("international"))]
        lang: String, 
    },
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
