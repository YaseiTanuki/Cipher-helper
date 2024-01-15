pub mod ceasar_cipher;

use crate::ceasar_cipher::{CeasarCipher, Decode, Encode, BruteForce};

fn main() {
    let plain = "My plain text is Abc & 123".to_string();
    let key = 3;
    let mut ceasar = CeasarCipher::new();

    //Set plain text
    ceasar.set_plain(plain);

    //encode and save to object
    print!("Encoded: {}\n", ceasar.encode(key));
    ceasar.set_encoded_text(ceasar.encode(key));

    //decode (result can be save to object too)
    print!("Decode: {}\n", ceasar.decode(key));

    //Brute force
    print!("\nBrute force result:\n");
    ceasar.brute_force();
}