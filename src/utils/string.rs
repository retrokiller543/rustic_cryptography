use crate::config::Config;
use crate::error::Error;
use crate::utils::set_panic_hook;
use anyhow::{bail, Result};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

macro_rules! encode {
    ($input:ident, $config:ident) => {
        if $config.is_base64() {
            let base64_str = crate::utils::string::base64_encode_string($input);
            crate::utils::string::encode_string(&base64_str, $config)?
        } else {
            crate::utils::string::encode_string($input, $config)?
        }
    };
}

macro_rules! decode {
    ($input:ident, $config:ident) => {
        if $config.is_base64() {
            let decoded = crate::utils::string::decode_string($input, $config)?;
            crate::utils::string::base64_decode_string(&decoded)?
        } else {
            crate::utils::string::decode_string($input, $config)?
        }
    };
}

pub(crate) use {decode, encode};

pub fn base64_encode_string(input: &str) -> String {
    BASE64_STANDARD.encode(input.as_bytes())
}

pub fn base64_decode_string(input: &str) -> Result<String> {
    let decoded = BASE64_STANDARD.decode(input)?;
    Ok(String::from_utf8(decoded)?)
}

pub fn encode_string(input: &str, config: &Config) -> Result<Vec<usize>> {
    set_panic_hook();

    let input = input.to_lowercase(); // Ensure the input is in lowercase

    let mut result = Vec::new();

    for c in input.chars() {
        if let Some(index) = config.alphabet().find(c) {
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
    let alphabet_length = config.alphabet.chars().count();
    let decoded = encoded_indices
        .iter()
        .map(|&x| {
            if x < alphabet_length {
                config.alphabet.chars().nth(x).ok_or(Error::CharacterParseError(x))
            } else {
                Err(Error::CharacterParseError(x))
            }
        })
        .collect::<Result<String, _>>()?;

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