use super::enums::KeyOrigin;
use ssh_keys::{PrivateKey, PublicKey};

#[derive(Debug)]
pub struct KeysHolder {
    pub keys: Vec<KeyHolder>,
}

impl KeysHolder {
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }
}

#[derive(Debug)]
pub struct KeyHolder {
    pub human_name: String,
    pub cipher: String,
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
    pub origin: KeyOrigin,
}

impl KeyHolder {
    pub fn new(
        human_name: &str,
        cipher: &str,
        private_key: PrivateKey,
        public_key: PublicKey,
        origin: KeyOrigin,
    ) -> Self {
        Self {
            human_name: String::from(human_name),
            cipher: String::from(cipher),
            private_key,
            public_key,
            origin: origin,
        }
    }
}
