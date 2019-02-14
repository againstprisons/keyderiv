use crate::config::Target as ConfigTarget;
use crate::util::hex_to_bytes;

#[derive(Clone)]
pub struct Target {
    pub name: String,
    pub shared_secret: Vec<u8>,
    pub index_key: Vec<u8>,
    pub encrypt_key: Vec<u8>,
}

impl Target {
    pub fn new(cfgtarget: ConfigTarget) -> Target {
        Target {
            name: cfgtarget.name,
            shared_secret: hex_to_bytes(&cfgtarget.shared_secret),
            index_key: hex_to_bytes(&cfgtarget.index_key),
            encrypt_key: hex_to_bytes(&cfgtarget.encrypt_key),
        }
    }
}
