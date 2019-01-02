use crypto::scrypt::{scrypt, ScryptParams};

lazy_static! {
    static ref SCRYPT_PARAMS: ScryptParams = ScryptParams::new(4, 8, 1);
}

pub fn generate_key(data: &str, salt_vec: &Vec<u8>) -> String {
    let mut data_vec = Vec::<u8>::new();
    for byte in data.bytes() {
        data_vec.push(byte);
    }

    let mut output_vec: Vec<u8> = vec![0; 32];
    scrypt(
        &data_vec,
        &salt_vec,
        &SCRYPT_PARAMS,
        output_vec.as_mut_slice(),
    );

    let mut output = String::new();
    for byte in output_vec.iter() {
        output.push_str(format!("{:02x}", byte).as_str());
    }

    output
}
