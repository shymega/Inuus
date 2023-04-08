use super::enums::KeyOrigin;
use ssh_keys::{PrivateKey, PublicKey};

#[derive(Debug)]
pub struct Keys {
    pub keys: Vec<KeyContainer>,
}

impl Keys {
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }
}

#[derive(Debug)]
pub struct KeyContainer {
    pub human_name: String,
    pub private_keys: Vec<PrivateKey>,
    pub public_key: PublicKey,
    pub origin: KeyOrigin,
}

impl KeyContainer {
    pub fn new(
        human_name: &str,
        private_keys: Vec<PrivateKey>,
        public_key: PublicKey,
        origin: KeyOrigin,
    ) -> Self {
        Self {
            human_name: String::from(human_name),
            private_keys,
            public_key,
            origin: origin,
        }
    }
}
