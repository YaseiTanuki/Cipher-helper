use std::env;

use caesar_cipher_method::{BruteForce, CeasarCipher, Decode, Encode};

fn print_usage() {
    println!(
        "Usage:\n  caesar_cipher_method encode <key:i8> <plain_text>\n  caesar_cipher_method decode <key:i8> <encoded_text>\n  caesar_cipher_method brute <encoded_text>\n\nNotes:\n  - key can be negative or >26; rotation is modulo 26\n  - wrap multi-word text in quotes"
    );
}

fn parse_key(arg: &str) -> Result<i8, String> {
    arg.parse::<i16>()
        .map_err(|_| format!("Invalid key: {}", arg))
        .and_then(|k| {
            if k < i8::MIN as i16 || k > i8::MAX as i16 {
                Err("Key out of i8 range".to_string())
            } else {
                Ok(k as i8)
            }
        })
}

fn main() {
    let mut args = env::args().skip(1);
    let Some(cmd) = args.next() else {
        print_usage();
        return;
    };

    match cmd.as_str() {
        "encode" => {
            let Some(key_str) = args.next() else {
                eprintln!("Missing <key>");
                print_usage();
                return;
            };
            let Ok(key) = parse_key(&key_str) else {
                eprintln!("Invalid <key>: {}", key_str);
                print_usage();
                return;
            };
            let Some(plain) = args.next() else {
                eprintln!("Missing <plain_text>");
                print_usage();
                return;
            };

            let mut cipher = CeasarCipher::new();
            cipher.set_plain(plain);
            let encoded = cipher.encode(key);
            println!("{}", encoded);
        }
        "decode" => {
            let Some(key_str) = args.next() else {
                eprintln!("Missing <key>");
                print_usage();
                return;
            };
            let Ok(key) = parse_key(&key_str) else {
                eprintln!("Invalid <key>: {}", key_str);
                print_usage();
                return;
            };
            let Some(encoded_text) = args.next() else {
                eprintln!("Missing <encoded_text>");
                print_usage();
                return;
            };

            let mut cipher = CeasarCipher::new();
            cipher.set_encoded_text(encoded_text);
            let decoded = cipher.decode(key);
            println!("{}", decoded);
        }
        "brute" | "bruteforce" => {
            let Some(encoded_text) = args.next() else {
                eprintln!("Missing <encoded_text>");
                print_usage();
                return;
            };
            let mut cipher = CeasarCipher::new();
            cipher.set_encoded_text(encoded_text);
            println!("Brute force result:");
            cipher.brute_force();
        }
        _ => {
            eprintln!("Unknown command: {}", cmd);
            print_usage();
        }
    }
}