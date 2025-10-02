pub fn normalize_shift(key: i8) -> u8 {
    (key as i16).rem_euclid(26) as u8
}