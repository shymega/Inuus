//! This is the main library for the `sshwarden` project.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![allow(dead_code)] // TODO: TEMPORARY.

pub mod agent;
pub(crate) mod models;
pub(crate) mod secrets_providers;
