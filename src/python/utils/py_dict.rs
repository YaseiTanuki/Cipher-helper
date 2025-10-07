#![cfg(feature = "python")]
use pyo3::prelude::*;

use pyo3::types::PyModule;

pub fn py_meaningful_ratio(text: &str) -> PyResult<f32> {
    Python::with_gil(|py| -> PyResult<f32> {
        let wordfreq = PyModule::import_bound(py, "wordfreq")?;
        let tokenize = wordfreq.getattr("tokenize")?;
        let word_frequency = wordfreq.getattr("word_frequency")?;

        let raw_tokens: Vec<String> = tokenize
            .call1((text, "en"))?
            .extract::<Vec<String>>()?;

        let tokens: Vec<String> = raw_tokens
            .into_iter()
            .map(|t| t.to_ascii_lowercase())
            .filter(|t| !t.is_empty() && t.chars().all(|c| c.is_ascii_alphabetic()))
            .collect();

        if tokens.is_empty() {
            return Ok(0.0);
        }

        let mut meaningful = 0usize;
        for token in tokens.iter() {
            let freq: f64 = word_frequency.call1((token, "en"))?.extract()?;
            if freq > 0.0 {
                meaningful += 1;
            }
        }
        Ok(meaningful as f32 / tokens.len() as f32)
    })
}