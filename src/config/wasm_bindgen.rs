use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Config {
    alfabet: String,
    pub base64: bool,
    pub utf8: bool,
}

#[wasm_bindgen]
impl Config {
    #[wasm_bindgen(getter)]
    pub fn alfabet(&self) -> String {
        self.alfabet.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_alfabet(&mut self, alfabet: String) {
        self.alfabet = alfabet;
    }

    pub fn with_base64(mut self) -> Self {
        self.alfabet = super::BASE64_ALFABET.to_string();
        self.base64 = true;
        self
    }

    pub fn without_base64(mut self) -> Self {
        self.alfabet = super::DEFAULT_ALFABET.to_string();
        self.base64 = false;
        self
    }

    pub fn with_utf8(mut self) -> Self {
        self.alfabet = super::Config::generate_full_utf8_range();
        self.utf8 = true;
        self
    }

    pub fn from_pointer(ptr: *mut Config) -> Config {
        unsafe { *Box::from_raw(ptr) }
    }
}

impl From<super::Config> for Config {
    fn from(config: super::Config) -> Self {
        Self {
            alfabet: config.alfabet,
            base64: config.base64,
            utf8: config.utf8,
        }
    }
}

impl From<Config> for super::Config {
    fn from(config: Config) -> Self {
        Self {
            alfabet: config.alfabet,
            base64: config.base64,
            utf8: config.utf8,
        }
    }
}

impl From<&Config> for super::Config {
    fn from(config: &Config) -> Self {
        Self {
            alfabet: config.alfabet.clone(),
            base64: config.base64,
            utf8: config.utf8,
        }
    }
}

impl From<&super::Config> for Config {
    fn from(config: &super::Config) -> Self {
        Self {
            alfabet: config.alfabet.clone(),
            base64: config.base64,
            utf8: config.utf8,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alfabet: super::DEFAULT_ALFABET.to_string(),
            base64: false,
            utf8: false,
        }
    }
}


#[wasm_bindgen]
pub fn get_default_config() -> Config {
    Config::default()
}

#[wasm_bindgen]
pub fn get_default_alfabet() -> String {
    super::DEFAULT_ALFABET.to_string()
}

#[wasm_bindgen]
pub fn get_base64_alfabet() -> String {
    super::BASE64_ALFABET.to_string()
}