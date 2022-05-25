//! Module for handling Bitwarden vault operations

/// `Vault` struct acts as a representation of the Bitwarden vault linked to `sshwarden`.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct Vault {
    /// Bitwarden internal ID of the Vault.
    id: String,
}

impl Vault {
    /// `Vault::new(id)` returns a new instance of the `Vault` struct.
    #[allow(dead_code)]
    pub fn new(id: &str) -> Self {
        Self {
            id: String::from(id),
        }
    }
}