#[derive(Default, Debug, Clone, Copy)]
pub enum KeyOrigin {
    #[cfg(feature = "bitwarden")]
    Bitwarden,
    #[cfg(feature = "onepassword")]
    OnePassword,
    #[cfg(feature = "pass")]
    Pass,
    #[default]
    Unselected,
}
