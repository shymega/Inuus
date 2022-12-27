#![allow(dead_code)]

use ssh_keys::{PrivateKey, PublicKey};

pub(crate) struct KeysHolder {
    pub(crate) keys: Vec<KeyHolder>,
}

impl KeysHolder {
    pub(crate) fn new() -> Self {
        Self { keys: Vec::new() }
    }
}

pub(crate) struct KeyHolder {
    pub(crate) human_name: String,
    pub(crate) cipher: String,
    pub(crate) private_key: PrivateKey,
    pub(crate) public_key: PublicKey,
    pub(crate) origin: String,
}

impl KeyHolder {
    pub(crate) fn new(
        human_name: &str,
        cipher: &str,
        private_key: PrivateKey,
        public_key: PublicKey,
        origin: &str,
    ) -> Self {
        Self {
            human_name: String::from(human_name),
            cipher: String::from(cipher),
            private_key,
            public_key,
            origin: String::from(origin),
        }
    }
}
