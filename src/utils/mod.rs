pub mod string;

pub fn set_panic_hook() {
    #[cfg(feature = "wasm")]
    console_error_panic_hook::set_once();
}