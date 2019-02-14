pub fn hex_to_bytes(data: &str) -> Vec<u8> {
    let input_chars: Vec<_> = data.chars().collect();

    input_chars
        .chunks(2)
        .map(|chunk| {
            let first = chunk[0].to_digit(16).unwrap();
            let second = chunk[1].to_digit(16).unwrap();
            ((first << 4) | second) as u8
        })
        .collect()
}

pub fn bytes_to_hex(data: &Vec<u8>) -> String {
    let mut output: String = String::new();

    for byte in data.clone() {
        output.push_str(&format!("{:02x}", byte));
    }

    output
}
