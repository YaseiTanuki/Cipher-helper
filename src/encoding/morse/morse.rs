pub use crate::encoding::morse::morse_code::{AMERICAN_MORSE, INTERNATIONAL_MORSE, MorseLang};
use crate::build_morse_maps;

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

    pub fn from_lang(lang: MorseLang) -> Self {
        let (map, rev_map) = match lang {
            MorseLang::International => build_morse_maps(INTERNATIONAL_MORSE),
            MorseLang::American => build_morse_maps(AMERICAN_MORSE),
        };
        Self { lang, map, rev_map }
    }

    pub fn set_lang(&mut self, lang: MorseLang) {
        let (map, rev_map) = match lang {
            MorseLang::International => build_morse_maps(INTERNATIONAL_MORSE),
            MorseLang::American => build_morse_maps(AMERICAN_MORSE),
        };
        self.lang = lang;
        self.map = map;
        self.rev_map = rev_map;
    }
}