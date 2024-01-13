pub struct CeasarCipher {
    plain: String,
    encoded_text: String,
}

pub trait Encode {
    fn encode(&mut self,  key: u8) -> String;
}

impl CeasarCipher {

    pub fn new() -> Self {
        return Self {plain: "".to_string(), encoded_text: "".to_string()}
    }

    pub fn set_plain (&mut self, new_plain: String) {
        self.plain = new_plain;
    }

    pub fn set_encoded_text(&mut self, new_encoded_text: String) {
        self.encoded_text = new_encoded_text;
    }

    pub fn get_plain(self) -> String {
        return self.plain;
    }

    pub fn get_encoded_text(self) -> String {
        return self.encoded_text;
    }
}

impl Encode for CeasarCipher {
    fn encode(&mut self, key: u8) -> String{
        
        let mut new_char_code: u8;
        let mut encoded_text = String::from("");

        for char in self.plain.chars() {
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

        self.set_encoded_text(encoded_text.to_owned());
        return encoded_text;
    }
}