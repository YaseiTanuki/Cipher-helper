use crate::normalize_shift;
use crate::py_meaningful_ratio;
use crate::DecodedResult;

/// Free function version of encode
pub fn encode(text: &str, key: i8) -> String {
    let shift = normalize_shift(key);
    let mut out = String::with_capacity(text.len());

    for &b in text.as_bytes() {
        let new_b = if b.is_ascii_uppercase() {
            ((b - b'A' + shift) % 26) + b'A'
        } else if b.is_ascii_lowercase() {
            ((b - b'a' + shift) % 26) + b'a'
        } else {
            b
        };
        out.push(new_b as char);
    }

    out
}

/// Free function version of decode
pub fn decode(encoded: &str, key: i8) -> DecodedResult {
    let shift = normalize_shift(-key);
    let mut out = String::with_capacity(encoded.len());

    for &b in encoded.as_bytes() {
        let new_b = if b.is_ascii_uppercase() {
            ((b - b'A' + shift) % 26) + b'A'
        } else if b.is_ascii_lowercase() {
            ((b - b'a' + shift) % 26) + b'a'
        } else {
            b
        };
        out.push(new_b as char);
    }

    let ratio = match py_meaningful_ratio(&out) {
        Ok(r) => Some(r),
        Err(_) => None,
    };

    DecodedResult {
        text: out,
        key: key as u8,
        meaningful_ratio: ratio,
    }
}

/// Free function version of brute_force
pub fn brute_force(encoded: &str, threshold: Option<f32>) -> Vec<DecodedResult> {
    let mut results: Vec<DecodedResult> = Vec::new();
    let mut warned = false;

    for key in 0..26 {
        let decoded = decode(encoded, key);
        let ratio = decoded.meaningful_ratio.unwrap_or(0.0);

        if decoded.meaningful_ratio.is_none() && !warned {
            eprintln!("[warn] Python wordfreq unavailable or error when computing meaningful ratio.");
            warned = true;
        }

        if ratio >= threshold.unwrap_or(0.5) {
            results.push(decoded);
        }
    }

    results
}

/// Free function version of brute_force_all
pub fn brute_force_all(encoded: &str) -> Vec<DecodedResult> {
    let mut results: Vec<DecodedResult> = Vec::new();
    for key in 0..26 {
        results.push(decode(encoded, key));
    }
    results
}
