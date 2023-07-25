use crate::models::enums::KeyOrigin;
use ssh_key::{PrivateKey, PublicKey};

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
    pub id: String,
    pub human_name: String,
    pub private_keys: Vec<PrivateKey>,
    pub public_key: PublicKey,
    pub origin: KeyOrigin,
}

impl KeyContainer {
    pub fn new(
        id: &str,
        human_name: &str,
        private_keys: Vec<PrivateKey>,
        public_key: PublicKey,
        origin: KeyOrigin,
    ) -> Self {
        Self {
            id: String::from(id),
            human_name: String::from(human_name),
            private_keys,
            public_key,
            origin,
        }
    }
}
