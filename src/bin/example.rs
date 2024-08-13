use shielded_rust::symmetric::caesar::{encrypt_string, decrypt_string};
use shielded_rust::config::ConfigBuilder;

fn main() -> anyhow::Result<()> {
    let config = ConfigBuilder::new().without_base64().build();
    let input = "hej mitt namn är emil och jag skickar hemliga meddelanden till snygga brudar, eller snarare bara en snygg brud, molly min livs kärlek och den person som jag kommer att gifta mig med i framtiden!";
    let key = 47;
    let encoded = encrypt_string(input, &config, key)?;
    println!("{:?}", encoded);

    let decoded = decrypt_string(&encoded, &config, key)?;
    println!("{:?}", decoded);

    Ok(())
}
