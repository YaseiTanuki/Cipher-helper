use clap::{Parser, Subcommand};
use cipher_helper::{BruteForce, CaesarCipher, Decode, Encode};
use cipher_helper::caesar::DecodedResult;

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

    /// Brute-force the encoded text
    Brute {
        /// Print all results (short: -a, long: --all)
        #[arg(short = 'a', long = "all")]
        all: bool,

        /// encoded text to brute-force
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

            let mut cipher = CaesarCipher::from_plain(plain_text);
            let encoded = cipher.encode(Some(key));
            println!("{}", encoded);
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

            let cipher = CaesarCipher::from_encoded(encoded_text);
            let decoded = cipher.decode(Some(key));
            println!("Decoded result: {}", decoded); // in kiểu pretty
        }

        Commands::Brute { all, encoded_text, threshold } => {
            let mut cipher = CaesarCipher::new();
            cipher.set_encoded_text(encoded_text);

            println!("Brute force result:");

            // Lấy kết quả (chú ý: phương thức trả Vec<DecodedResult>)
            let mut results: Vec<DecodedResult> = if all {
                // tất cả kết quả
                cipher.brute_force_all()
            } else {
                // lọc theo threshold (threshold: Option<f32>)
                cipher.brute_force(threshold)
            };

            // Nếu muốn, sort theo meaningful_ratio giảm dần (None coi là 0.0)
            results.sort_by(|a, b| {
                let ra = a.meaningful_ratio.unwrap_or(0.0);
                let rb = b.meaningful_ratio.unwrap_or(0.0);
                // partial_cmp vì f32 có NaN; unwrap_or Equal nếu so sánh thất bại
                rb.partial_cmp(&ra).unwrap_or(std::cmp::Ordering::Equal)
            });

            if results.is_empty() {
                println!("Không tìm được kết quả — có thể do meaningful_ratio = None hoặc threshold quá cao.");
            } else {
                for (i, r) in results.into_iter().enumerate() {
                    // Dùng Display đã cài màu; nếu muốn hiển thị thêm ratio rõ ràng:
                    println!("{:02}. {}", i + 1, r);
                }
            }
        }

    }
}
