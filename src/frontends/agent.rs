use crate::models::keys::KeysHolder;
use std::sync;

pub type KeysHolderMutex = sync::Mutex<KeysHolder>;

#[derive(Debug)]
pub struct InuusAgent {
    keys: KeysHolderMutex,
}
