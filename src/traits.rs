use crate::DecodedResult;

pub trait ClassicalCipher {
    fn encrypt(&self, input: &str) -> String;
    fn decrypt(&self, input: &str) -> String;
}

pub trait BruteForce {
    /// Returns candidates meeting an optional meaningfulness threshold.
    fn bruteforce(&mut self, input: &str, threshold: Option<f32>) -> Vec<DecodedResult>;
}
