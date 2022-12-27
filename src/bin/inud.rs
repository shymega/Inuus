//! This is the main entrypoint for the `sshwarden` project.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

use std::{path::PathBuf, io};

use anyhow::{Result, Context};
use thiserror::Error;

use clap::{Arg, ArgAction, ArgMatches, Command};
use twelf::{config, Layer, Error as TwelfError};

#[config]
#[allow(dead_code)]
struct Config {
    chosen_backend: String,
    control_socket: String,
    agent_socket: String,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Error)]
enum ConfInitError {
    #[error("Error building configuration")]
    ConfLoadError(#[from] TwelfError),
}

fn try_load_conf(path: &PathBuf) -> Result<Config, ConfInitError> {
    Ok(Config::with_layers(&[
        Layer::Yaml(path.join("config.yaml").into()),
        Layer::Env(Some("INUUS_".to_string()))
        ])?)
}

fn get_cfg_path_default() -> Result<PathBuf, io::Error> {
    let def_path = dirs::config_dir()
        .unwrap()
        .join(PathBuf::from("inuus/config.toml"));
    let path_parent = if let Some(p) = def_path.parent() {
        p.to_owned()
    } else { PathBuf::new() };

    // Try and create the parent path
    match std::fs::create_dir_all(&path_parent) {
        Ok(_) => Ok(def_path),
        Err(e) => return Err(e),
    }
}

fn get_args() -> ArgMatches {
    Command::new("inud")
        .version(VERSION)
        .author("Dom Rodriguez")
        .subcommand_required(true)
        .arg(
            Arg::new("verbosity")
                .short('v')
                .action(ArgAction::Count)
                .help("Increment to get verbose logs."),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .help("Optional path to TOML configuration."),
        )
        .subcommand(Command::new("spawn").about("Starts the daemon."))
        .get_matches()
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = get_args();

    let arg_cfg_path = match args.get_one::<PathBuf>("config") {
        Some(p) => PathBuf::from(p),
        None => {
            get_cfg_path_default().unwrap()
        }
    };

    let config = try_load_conf(&arg_cfg_path)
        .context("Unable to load and initialise configuration.")?;

    Ok(())
}
