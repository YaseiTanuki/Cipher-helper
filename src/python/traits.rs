#![cfg(feature = "python")]

use crate::DecodedResult;

pub trait PyBruteForce {
    fn py_bruteforce(&self, input: &str, threshold: Option<f32>) -> Vec<DecodedResult>;
}