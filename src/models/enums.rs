#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum KeyOrigin {
    #[cfg(feature = "bitwarden")]
    Bitwarden,
    #[cfg(feature = "_1password")]
    _1Password,
    #[cfg(feature = "pass")]
    Pass,
    #[default]
    Unselected,
}
