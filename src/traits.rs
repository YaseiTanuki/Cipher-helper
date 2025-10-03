use crate::caesar::caesar::DecodedResult;

pub trait Encrypt {
    fn encrypt(&mut self, key: Option<i8>) -> String;
}

pub trait Decrypt {
    fn decrypt(&self, key: Option<i8>) -> DecodedResult;
}

pub trait BruteForce {
    fn brute_force(&self, threshold: Option<f32>) -> Vec<DecodedResult>;
    fn brute_force_all(&self) -> Vec<DecodedResult>;
}
