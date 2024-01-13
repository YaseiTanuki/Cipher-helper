pub mod ceasar_cipher;

use crate::ceasar_cipher::CeasarCipher;
use ceasar_cipher::Encode;

fn main() {
    let plain = "my plain text is 123 & XYZ".to_string();
    let key = 3;
    let mut ceasar = CeasarCipher::new();

    ceasar.set_plain(plain);
    ceasar.encode(key);

    print!("{}", ceasar.get_encoded_text());
}