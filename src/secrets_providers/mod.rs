//! Module for 'secrets providers'.

/// `SecretsProviderTrait` defines a shared specification for an secrets provider.
pub(crate) trait SecretsProviderTrait {
    /// `get_secret_by_uuid(&str)` takes a UUID of a 'secret', and returns a `String` of the
    /// secret's content.
    /// TODO: Change to shared representation of a SSH key.
    fn get_secret_by_uuid(_uuid: &str) -> String {
        String::new()
    }
    /// `get_secret_by_path(&str)` takes a slash-delimited `&str` to a `secret`, and returns a
    /// `String` of the secret's content.
    /// We use a `&str` for the path, because `pass` doesn't use `Path` or `PathBuf`.
    fn get_secret_by_path(_path: &str) -> String {
        String::new()
    }
}

#[cfg(feature = "bitwarden")]
pub(crate) mod bitwarden;

#[cfg(feature = "_1password")]
pub(crate) mod _1password;

#[cfg(feature = "pass")]
pub(crate) mod pass;
