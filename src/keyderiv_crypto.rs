use crypto::scrypt::{scrypt, ScryptParams};
use rust_sodium::crypto::secretbox::xsalsa20poly1305;
use lazy_static::lazy_static;

use crate::util::bytes_to_hex;

lazy_static! {
    static ref SCRYPT_PARAMS: ScryptParams = ScryptParams::new(4, 8, 1);
}

pub fn generate_key(data: &str, salt_vec: &Vec<u8>) -> String {
    let data_vec: Vec<u8> = data.bytes().collect();

    let mut output_vec: Vec<u8> = vec![0; 32];
    scrypt(
        &data_vec,
        &salt_vec,
        &SCRYPT_PARAMS,
        output_vec.as_mut_slice(),
    );

    bytes_to_hex(&output_vec)
}

pub fn encrypt_secret(
    data: &str,
    key_vec: &Vec<u8>,
    nonce_vec: &Vec<u8>,
) -> Result<String, String> {
    let key = match xsalsa20poly1305::Key::from_slice(&key_vec) {
        Some(val) => val,
        None => return Err("invalid key".to_string()),
    };

    let nonce = match xsalsa20poly1305::Nonce::from_slice(&nonce_vec) {
        Some(val) => val,
        None => return Err("invalid nonce".to_string()),
    };

    let data_vec: Vec<u8> = data.bytes().collect();
    let output_vec = xsalsa20poly1305::seal(&data_vec, &nonce, &key);

    Ok(bytes_to_hex(&output_vec))
}
