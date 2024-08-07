use rustic_cryptography::symmetric::caesar::encrypt_string;
use rustic_cryptography::config::ConfigBuilder;

fn main() -> anyhow::Result<()> {
    let config = ConfigBuilder::new().with_base64().build();
    let input = "hej mitt namn är emil och jag är en idiot";
    let encoded = encrypt_string(input, &config, 63)?;
    println!("{:?}", encoded);

    Ok(())
}
