//! Module for 'secrets providers'.

/// `SecretsProviderTrait` defines a shared specification for an secrets provider.
pub trait SecretsProviderTrait {
    /// `get_secret_by_uuid(&str)` takes a UUID of a 'secret', and returns a `String` of the
    /// secret's content.
    /// TODO: Change to shared representation of a SSH key.
    fn get_secret_by_uuid(uuid: &str) -> String;
    /// `get_secret_by_path(&str)` takes a slash-delimited `&str` to a `secret`, and returns a
    /// `String` of the secret's content.
    fn get_secret_by_path(path: &str) -> String; // we use a `&str` for the path, because `pass` doesn't use `Path` or `PathBuf`.
}

#[cfg(feature = "bitwarden")]
pub mod bitwarden;

#[cfg(feature = "_1password")]
pub(crate) mod _1password;

#[cfg(feature = "pass")]
pub mod pass;
