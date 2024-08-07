fn main() {
    // 512MB = 0x200000000
    #[cfg(target_arch = "wasm32")]
    println!("cargo:rustc-link-arg=-zstack-size=0x200000000");
}
