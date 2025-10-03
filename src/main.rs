use clap::{Parser, Subcommand};
use cipher_helper::caesar::DecodedResult;
use cipher_helper::{decode, encode, brute_force, brute_force_all};

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
    /// Encode plain text with a key
    Encode {
        /// rotation key, can be negative or >26 (mod 26)
        key: i16,

        /// plain text (wrap multi-word in quotes)
        plain_text: String,
    },

    /// Decode encoded text with a key
    Decode {
        /// rotation key, can be negative or >26 (mod 26)
        key: i16,

        /// encoded text (wrap multi-word in quotes)
        encoded_text: String,
    },

    Brute {
        /// Print all results (short: -a, long: --all)
        #[arg(short = 'a', long = "all")]
        all: bool,

        encoded_text: String,
        threshold: Option<f32>,
    },
}

fn print_usage() {
    println!(
        "Usage:\n  caesar_cipher_method encode <key:i8> <plain_text>\n  caesar_cipher_method decode <key:i8> <encoded_text>\n  caesar_cipher_method brute [-a|--all] <encoded_text>\n\nNotes:\n  - key can be negative or >26; rotation is modulo 26\n  - wrap multi-word text in quotes"
    );
}

fn parse_key_i8(k: i16) -> Result<i8, String> {
    if k < i8::MIN as i16 || k > i8::MAX as i16 {
        Err(format!("Key out of i8 range: {}", k))
    } else {
        Ok(k as i8)
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { key, plain_text } => {
            let key = match parse_key_i8(key) {
                Ok(k) => k,
                Err(e) => {
                    eprintln!("Invalid <key>: {} ({})", key, e);
                    print_usage();
                    return;
                }
            };

            println!("{}", encode(&plain_text, key));
        } 

        Commands::Decode { key, encoded_text } => {
            let key = match parse_key_i8(key) {
                Ok(k) => k,
                Err(e) => {
                    eprintln!("Invalid <key>: {} ({})", key, e);
                    print_usage();
                    return;
                }
            };

            println!("Decoded result: {}", decode(&encoded_text, key));
        }

        Commands::Brute { all, encoded_text, threshold } => {
            println!("Brute force result:");

            let mut results: Vec<DecodedResult> = if all {
                brute_force_all(&encoded_text)
            } else {
                brute_force(&encoded_text, threshold)
            };

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
