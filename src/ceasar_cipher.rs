pub struct CeasarCipher {
    plain: String,
    encoded_text: String,
}

pub trait Encode {
    fn encode(&self,  key: i8) -> String;
}

pub trait Decode {
    fn decode(&self, key: i8) -> String;
}

pub trait BruteForce {
    fn brute_force(&self);
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

    pub fn get_plain(&self) -> String {
        return self.plain.clone();
    }

    pub fn get_encoded_text(&self) -> String {
        return self.encoded_text.clone();
    }
}

impl Encode for CeasarCipher {
    fn encode(&self, key: i8) -> String {
        
        let mut new_char_code: i8;
        let mut encoded_text = String::from("");

        for char in self.plain.chars() {
            if char.is_alphabetic() {
                if char.is_uppercase() {
                    new_char_code = ((char as i8) - 65 + key).rem_euclid(26) + 65;
                }
                else {
                    new_char_code = ((char as i8) - 97 + key).rem_euclid(26) + 97;
                }
            }
            else {
                new_char_code = char as i8;
            }

            encoded_text.push(new_char_code as u8 as char);
        }

        return encoded_text;
    }
}

impl Decode for CeasarCipher {
    fn decode(&self, key: i8) -> String {
        let mut new_char_code: i8;
        let mut decoded_text = String::from("");

        for char in self.encoded_text.chars() {
            if char.is_alphabetic() {
                if char.is_uppercase() {
                    new_char_code = ((char as i8) - 65 - key).rem_euclid(26) + 65;
                }
                else {
                    new_char_code = ((char as i8) - 97 - key).rem_euclid(26) + 97;
                }
            }
            else {
                new_char_code = char as i8;
            }

            decoded_text.push(new_char_code as u8 as char);
        }

        return decoded_text;
    }
}

impl BruteForce for CeasarCipher {
    fn brute_force(&self) {
        let mut warned = false;
        for key in 0..26 {
            let decoded = self.decode(key);
            // Use Python-based check; if Python unavailable, skip printing
            let ratio = match crate::py_dict::py_meaningful_ratio(&decoded) {
                Ok(r) => r,
                Err(e) => {
                    if !warned {
                        eprintln!("[warn] Python wordfreq unavailable: {}", e);
                        warned = true;
                    }
                    0.0
                }
            };
            if ratio > 0.5 {
                print!("KEY: {0} (meaningful: {1:.0}%)\nDECODED TEXT: {2}\n\n", key, ratio * 100.0, decoded);
            }
        }
    }
}