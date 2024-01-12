fn main() {
    let encoded_text = encode("my phain_text".into(), 3);
    print!("{}", encoded_text);
}


fn encode(plain: String, key: u8) -> String {

    let mut new_char_code: u8;
    let mut encoded_text = String::from("");

    for char in plain.chars() {
        if char.is_alphabetic() {
            if char.is_uppercase() {
                new_char_code = ((char as u8) - 65 + key) % 26 + 65;
            }
            else {
                new_char_code = ((char as u8) - 97 + key) % 26 + 97;
            }
        }
        else {
            new_char_code = char as u8;
        }

        encoded_text.push(new_char_code as char);
    }

    return encoded_text;
}