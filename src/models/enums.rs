// SPDX-FileCopyrightText: 2022-2026 The Inuus Developers
//
// SPDX-License-Identifier: Apache-2.0

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
