#[derive(Debug, Clone, Copy)]
pub enum KeyOrigin {
    #[cfg(feature = "bitwarden")]
    Bitwarden,
    #[cfg(feature = "onepassword")]
    OnePassword,
    Unselected,
}
