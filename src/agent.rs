// SPDX-FileCopyrightText: 2022-2026 The Inuus Developers
//
// SPDX-License-Identifier: Apache-2.0

//! This `module` defines the Inuus agent as a struct and implementation, for usage by the user,
//! and also by the JSON-RPC client.

use crate::models::keys::Keys as KeysModel;
use std::sync::Mutex;

/// This type alias defines `Keys` to a `Mutex` of the `Keys` (aliased as `KeysModel`) struct
/// defined in `crate::models::keys::Keys`.
type Keys = Mutex<KeysModel>;

/// `InuusAgent` holds one field for the shared `Mutex<Keys>` which
/// contains a `Vec<KeyContainer>` of keys, both private and public.
#[derive(Debug)]
pub struct InuusAgent {
    keys: Keys,
}
