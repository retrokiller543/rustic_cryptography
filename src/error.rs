use thiserror::Error;

use crate::config::Config;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("invalid base64")]
    InvalidBase64,
    #[error("invalid config {0} at key {1}")]
    InvalidConfig(Config, String),
    #[error("invalid key {0}")]
    InvalidKey(String),
    #[error("invalid input {0}")]
    InvalidInput(String),
    #[error("Index not found for char {0}")]
    IndexNotFound(char),
    #[error("Character parse error at index {0}")]
    CharacterParseError(usize)
}