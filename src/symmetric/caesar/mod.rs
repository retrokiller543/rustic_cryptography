use anyhow::Result;
use crate::config;
use crate::utils::string::{base64_decode_string, base64_encode_string};

#[cfg(feature = "wasm")]
pub mod wasm_binding;

pub fn encrypt_string(input: &str, config: &config::Config, key: usize) -> Result<String> {
    let input = if config.is_base64() {
        base64_encode_string(input)
    } else {
        input.to_string()
    };

    if key == 0 {
        return Ok(input.to_string());
    }

    let alphabet: Vec<char> = config.alphabet().chars().collect();
    let alphabet_length = alphabet.len();
    let key = key % alphabet_length;

    let encrypted: String = input.chars()
        .map(|c| {
            if let Some(index) = alphabet.iter().position(|&a| a == c) {
                alphabet[(index + key) % alphabet_length]
            } else {
                c
            }
        })
        .collect();

    Ok(encrypted)
}

pub fn decrypt_string(input: &str, config: &config::Config, key: usize) -> Result<String> {
    if key == 0 {
        return Ok(input.to_string());
    }

    let alphabet: Vec<char> = config.alphabet().chars().collect();
    let alphabet_length = alphabet.len();
    let key = key % alphabet_length;

    let decrypted: String = if config.is_base64() {
        let encoded: String = input.chars()
            .map(|c| {
                if let Some(index) = alphabet.iter().position(|&a| a == c) {
                    alphabet[(index + alphabet_length - key) % alphabet_length]
                } else {
                    c
                }
            })
            .collect();

        base64_decode_string(&encoded)?
    } else {
        input.chars()
            .map(|c| {
                if let Some(index) = alphabet.iter().position(|&a| a == c) {
                    alphabet[(index + alphabet_length - key) % alphabet_length]
                } else {
                    c
                }
            })
            .collect()
    };

    Ok(decrypted)
}