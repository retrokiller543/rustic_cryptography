use shielded_rust::symmetric::caesar::{encrypt_string, decrypt_string};
use shielded_rust::config::ConfigBuilder;

fn main() -> anyhow::Result<()> {
    let config = ConfigBuilder::new().without_base64().build();
    let input = "hej mitt namn är emil och jag är en idiot";
    let key = 63;
    let encoded = encrypt_string(input, &config, key)?;
    println!("{:?}", encoded);

    let decoded = decrypt_string(&encoded, &config, key)?;
    println!("{:?}", decoded);

    Ok(())
}
