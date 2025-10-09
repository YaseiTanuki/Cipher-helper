use clap::{Parser, Subcommand};
use cryptan::{BruteForce, CaesarCipher, ClassicalCipher, DecodedResult};
use cryptan::parse_key_i8;

#[derive(Parser)]
#[command(name = "caesar_cipher_method")]
#[command(version = "0.1.0")]
#[command(about = "Caesar cipher CLI (also usable as lib)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        key: i16,
        plain_text: String,
    },

    Decrypt {
        key: i16,
        encrypted_text: String,
    },

    Brute {
        encoded_text: String,
        threshold: Option<f32>,
    },
}

fn print_usage() {
    println!(
        "Usage:\n  caesar_cipher_method encrypt <key:i8> <plain_text>\n  caesar_cipher_method decrypt <key:i8> <encrypted_text>\n  caesar_cipher_method brute [-a|--all] <encrypted_text>\n\nNotes:\n  - key can be negative or >26; rotation is modulo 26\n  - wrap multi-word text in quotes"
    );
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { key, plain_text } => {
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

        Commands::Decrypt { key, encrypted_text } => {
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

        Commands::Brute { encoded_text, threshold} => {

            let mut caesar = CaesarCipher::from_key(0);
            let mut results: Vec<DecodedResult>;

            results = caesar.bruteforce(&encoded_text, threshold);
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

    }
}
