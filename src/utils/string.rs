use anyhow::{bail, Result};
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use crate::config::Config;
use crate::error::Error;

use super::set_panic_hook;

pub fn encode_string(input: &str, config: &Config) -> Result<Vec<usize>> {
    set_panic_hook();

    let mut input = input.to_lowercase();

    input = if config.is_base64() {
        BASE64_STANDARD_NO_PAD.encode(input.as_bytes())
    } else {
        input.to_owned()
    };

    let mut result = Vec::new();

    for c in input.chars() {
        if let Some(index) = config.alfabet().find(c) {
            result.push(index);
        } else {
            bail!(Error::IndexNotFound(c));
        }
    }

    Ok(result)
}

pub fn encode_asymmetric_string(input: &str, config: &Config) -> Result<Vec<usize>> {
    set_panic_hook();

    let mut encoded_indices = encode_string(input, config)?;
    for i in &mut encoded_indices {
        *i += 1; // Increment each index by 1 to avoid zeros
    }

    Ok(encoded_indices)
}

pub fn decode_string(encoded_indices: Vec<usize>, config: &Config) -> Result<String> {
    let alphabet_length = config.alfabet.chars().count();
    let decoded = encoded_indices
        .iter()
        .map(|&x| {
            if x < alphabet_length {
                config.alfabet
                    .chars()
                    .nth(x)
                    .ok_or(Error::CharacterParseError(x))
            } else {
                Err(Error::CharacterParseError(x))
            }
        })
        .collect::<Result<String, _>>()?;

    let decoded = if config.is_base64() {
        let decoded_bytes = BASE64_STANDARD_NO_PAD.decode(&decoded)?;
        String::from_utf8(decoded_bytes)?
    } else {
        decoded
    };

    Ok(decoded)
}

pub fn decode_asymmetric_string(encoded_indices: Vec<usize>, config: &Config) -> Result<String> {
    let mut adjusted_indices = Vec::new();

    for index in encoded_indices {
        if index > 0 {
            adjusted_indices.push(index - 1); // Decrement each index by 1
        } else {
            bail!(Error::IndexNotFound(char::from_digit(index as u32, 10).unwrap_or_default()));
        }
    }

    decode_string(adjusted_indices, config)
}