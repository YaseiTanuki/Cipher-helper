pub mod ceasar_cipher;

use crate::ceasar_cipher::{CeasarCipher, Decode, Encode};

fn main() {
    let plain = "My plain text is Abc & 123".to_string();
    let key = 3;
    let mut ceasar = CeasarCipher::new();

    ceasar.set_plain(plain);
    ceasar.encode(key);

    print!("Encoded: {}\n", &ceasar.get_encoded_text());

    ceasar.set_plain("".to_string());
    print!("Plain text is removed from object\n");

    ceasar.decode(key as i8);
    print!("Decoded: {}", ceasar.get_plain())
}