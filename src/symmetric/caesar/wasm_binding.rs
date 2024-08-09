use wasm_bindgen::prelude::*;

use crate::config;

use super::encrypt_string as super_encrypt_string;
use super::decrypt_string as super_decrypt_string;

#[wasm_bindgen]
pub fn encrypt_string(input: &str, config: &config::wasm_bindgen::Config, key: usize) -> String {
    let config: config::Config = config.into();
    super_encrypt_string(input, &config, key).unwrap()
}

#[wasm_bindgen]
pub fn decrypt_string(input: &str, config: &config::wasm_bindgen::Config, key: usize) -> String {
    let config: config::Config = config.into();
    super_decrypt_string(input, &config, key).unwrap()
}