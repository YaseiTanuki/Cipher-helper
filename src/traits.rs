use crate::caesar::caesar::DecodedResult;

pub trait Encode {
    fn encode(&self, key: Option<i8>) -> String;
}

pub trait Decode {
    fn decode(&self, key: Option<i8>) -> DecodedResult;
}

pub trait BruteForce {
    fn brute_force(&self, threshold: Option<f32>) -> Vec<DecodedResult>;
    fn brute_force_all(&self) -> Vec<DecodedResult>;
}
