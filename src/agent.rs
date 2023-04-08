//! This `module` defines the Inuus agent as a struct and implementation, for usage by the user,
//! and also by the JSON-RPC client.

use crate::models::keys::Keys;
use std::sync::Mutex;

/// This type alias defines `KeysMutex` to a `Mutex` of the `Keys` struct defined in
/// `crate::models::keys::Keys`.
pub type KeysMutex = Mutex<Keys>;

/// `InuusAgent` holds one (unused, lint disabled) field for the shared `Mutex<Keys>` which
/// contains a `Vec<KeyContainer>` of keys, both private and public.
#[derive(Debug)]
#[allow(dead_code)]
pub struct InuusAgent {
    keys: KeysMutex,
}
