#[cfg(feature = "wasm")]
pub mod wasm_bindgen;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub const DEFAULT_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzåäö0123456789 .,!?';:\"()-";
pub const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConfigBuilder {
    pub alphabet: String,
    pub base64: bool,
    pub utf8: bool,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_alphabet(mut self, alphabet: String) -> Self {
        self.alphabet = alphabet;
        self
    }

    pub fn with_base64(mut self) -> Self {
        self.alphabet = BASE64_ALPHABET.to_string();
        self.base64 = true;
        self
    }

    pub fn without_base64(mut self) -> Self {
        self.alphabet = DEFAULT_ALPHABET.to_string();
        self.base64 = false;
        self
    }

    pub fn utf8(mut self) -> Self {
        self.alphabet = Config::generate_full_utf8_range();
        self.utf8 = true;
        self
    }

    pub fn build(self) -> Config {
        Config {
            alphabet: self.alphabet,
            base64: self.base64,
            utf8: self.utf8,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub alphabet: String,
    pub base64: bool,
    pub utf8: bool,
}

impl Config {
    pub fn alphabet(&self) -> String {
        self.alphabet.clone()
    }

    pub fn is_base64(&self) -> bool {
        self.base64
    }

    fn generate_full_utf8_range() -> String {
        let mut utf8_chars = String::new();
        for c in 0x0000..=0x10FFFF {
            if let Some(ch) = std::char::from_u32(c) {
                utf8_chars.push(ch);
            }
        }
        utf8_chars
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alphabet: DEFAULT_ALPHABET.to_string(),
            base64: false,
            utf8: false,
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}