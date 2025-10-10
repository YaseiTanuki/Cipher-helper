pub use crate::encoding::morse::morse_code::{AMERICAN_MORSE, INTERNATIONAL_MORSE, MorseLang};
use crate::build_morse_maps;
use crate::traits::Codec;

use std::collections::HashMap;

pub struct Morse {
    lang: MorseLang,
    pub map: HashMap<char, String>,
    pub rev_map: HashMap<String, char>,
}

impl Morse {
    pub fn new() -> Self {
        let lang = MorseLang::International;
        let (map, rev_map) = build_morse_maps(INTERNATIONAL_MORSE);
        Self { lang, map, rev_map }
    }

    pub fn from_lang(lang: &str) -> Self {
        let lang = match lang.to_lowercase().as_str() {
            "american" => MorseLang::American,
            _ => MorseLang::International,
        };
        let (map, rev_map) = match lang {
            MorseLang::International => build_morse_maps(INTERNATIONAL_MORSE),
            MorseLang::American => build_morse_maps(AMERICAN_MORSE),
        };
        Self { lang, map, rev_map }
    }

    pub fn set_lang(&mut self, lang: &str) {
        let lang = match lang.to_lowercase().as_str() {
            "american" => MorseLang::American,
            _ => MorseLang::International,
        };
        let (map, rev_map) = match lang {
            MorseLang::International => build_morse_maps(INTERNATIONAL_MORSE),
            MorseLang::American => build_morse_maps(AMERICAN_MORSE),
        };
        self.lang = lang;
        self.map = map;
        self.rev_map = rev_map;
    }

    pub fn get_lang(&self) -> String {
        match self.lang {
            MorseLang::International => "International".to_string(),
            MorseLang::American => "American".to_string(),
        }
    }
}

impl Codec for Morse {
    fn encode(&self, input: &str) -> String {
        let mut result = Vec::new();
        for ch in input.chars() {
            match ch {
                ' ' => result.push("/".to_string()),

                ch if ch.is_ascii_alphanumeric() => {
                    if let Some(code) = self.map.get(&ch.to_ascii_uppercase()) {
                        result.push(code.clone());
                    }
                },

                _ => {
                    println!("Warning: Unsupported character '{}', skipping.", ch);
                }
            }
        }
        result.join(" ")
    }

    fn decode(&self, input: &str) -> String {
        let mut result = Vec::new();
        for code in input.split_whitespace() {
            match code {
                "/" => result.push(' '.to_string()),

                code if self.rev_map.get(&code.to_string()).is_some() => {
                    if let Some(&ch) = self.rev_map.get(code) {
                        result.push(ch.to_string());
                    }
                },

                _ => {
                    println!("Warning: Unsupported Morse code '{}', skipping.", code);
                }
            }
        }
        result.join("")
    }

}