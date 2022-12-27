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

use std::{path::PathBuf, io::Error as IoError};

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
    #[error("Error creating parent directory to configuration file.")]
    ConfParentPathCreateError(#[from] IoError)
}

fn try_load_conf(path: &PathBuf) -> Result<Config, ConfInitError> {
    Ok(Config::with_layers(&[
        Layer::Yaml(path.into()),
        Layer::Env(Some("INUUS_".to_string()))
        ])?)
}

fn get_cfg_path_default() -> Result<PathBuf, ConfInitError> {
    let def_path = dirs::config_dir()
        .unwrap()
        .join(PathBuf::from("inuus/config.yml"));

    // Try and create the parent path
    std::fs::create_dir_all(&def_path.parent().expect("Unable to get parent directory of configuration file."))?;

    Ok(def_path.clone())
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
async fn main() -> Result<()> {
    let args = get_args();
    let def_conf_path = get_cfg_path_default()
        .context("Unable to get default configuration path.")?;

    let arg_cfg_path = match args.get_one::<PathBuf>("config") {
        Some(p) => PathBuf::from(p),
        None => def_conf_path,
    };

    let _config = try_load_conf(&arg_cfg_path)
        .context("Unable to load and initialise configuration.")?;

    Ok(())
}
