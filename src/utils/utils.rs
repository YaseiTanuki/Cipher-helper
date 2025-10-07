use std::fs;
use std::collections::HashSet;

pub fn normalize_shift(key: i8) -> u8 {
    (key as i16).rem_euclid(26) as u8
}

pub fn parse_key_i8(k: i16) -> Result<i8, String> {
    if k < i8::MIN as i16 || k > i8::MAX as i16 {
        Err(format!("Key out of i8 range: {}", k))
    } else {
        Ok(k as i8)
    }
}

pub fn load_set(path: &str) -> HashSet<String> {
    let data = fs::read_to_string(path).expect("read");
    data.lines().map(|s| s.trim().to_owned()).collect()
}


pub fn meaningful_ratio(text: &str, wordlist: &HashSet<String>) -> Result<f32, Box<dyn std::error::Error>> {
    
    let word_set: std::collections::HashSet<String> = wordlist.into_iter().map(String::from).collect();

    let tokens: Vec<String> = text
        .split_whitespace()
        .map(|t| t.trim_matches(|c: char| !c.is_ascii_alphabetic()).to_ascii_lowercase())
        .filter(|t| !t.is_empty())
        .collect();

    if tokens.is_empty() {
        return Ok(0.0);
    }

    let meaningful_count = tokens.iter().filter(|t| word_set.contains(*t)).count();
    Ok(meaningful_count as f32 / tokens.len() as f32)
}