use crate::DecodedResult;

pub trait ClassicalCipher {
    fn encrypt(&self, input: &str) -> String;
    fn decrypt(&self, input: &str) -> String;
}

pub trait BruteForce {
    fn bruteforce(&mut self, input: &str, threshold: Option<f32>) -> Vec<DecodedResult>;
}

pub trait Codec {
    fn encode(&self, input: &str) -> String;
    fn decode(&self, input: &str) -> String;
}
