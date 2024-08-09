use wasm_bindgen::prelude::*;

use crate::config;

use super::encrypt_string as super_encrypt_string;

#[wasm_bindgen]
pub fn encrypt_string(input: &str, config: &config::wasm_bindgen::Config, key: usize) -> String {
    let config: config::Config = config.into();
    super_encrypt_string(input, &config, key).unwrap()
}