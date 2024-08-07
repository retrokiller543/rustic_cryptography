use anyhow::Result;

use crate::config;
use crate::utils;
use crate::utils::string::decode_string;

#[cfg(feature = "wasm")]
pub mod wasm_bindgen;

pub fn encrypt_string(input: &str, config: &config::Config, key: usize) -> Result<String> {
    let encoded_indices = utils::string::encode_string(input, &config)?;

    let encrypted: Vec<usize> = encoded_indices.iter()
        .map(|index| (index + key) % config.alfabet().chars().count())
        .collect();

    decode_string(encrypted, config)
}