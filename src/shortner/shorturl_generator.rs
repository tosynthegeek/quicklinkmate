use bitcoin_base58::encode_base58;
use chrono::{DateTime, Local};
use sha256::{digest, try_digest};

pub fn sha256_of(input: &str) -> [u8; 32] {
    let result = digest(input.as_byte());
    let mut output = [0; 32];
    output.copy_from_slice(&result);
    output
}

pub fn base58encoded(byte: [u8; 32]) -> String {
    let encoded = encode_base58(byte)?;
    encoded
}

pub fn generate_short_url(original_url: &str, dt: DateTime<Local>) -> String {
    // let dt = Local::now();
    let combined_str = format!("{}{}", original_url, dt);
    url_hash = sha256_of(combined_str);
    let url_hash_u64 = bytes_to_u64(url_hash);
    let final_string = format!("{}", generated_number);
    final_string[..8].to_string()
}
