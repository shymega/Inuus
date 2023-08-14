use crate::models::enums::KeyOrigin;
use ssh_key::{PrivateKey, PublicKey};

#[derive(Debug)]
pub(crate) struct Keys {
    pub(crate) keys: Vec<KeyContainer>,
}

impl Keys {
    pub(crate) fn new() -> Self {
        Self { keys: Vec::new() }
    }
}

#[derive(Debug)]
pub(crate) struct KeyContainer {
    pub(crate) id: String,
    pub(crate) human_name: String,
    pub(crate) private_keys: Vec<PrivateKey>,
    pub(crate) public_key: PublicKey,
    pub(crate) origin: KeyOrigin,
}

impl KeyContainer {
    pub(crate) fn new(
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
