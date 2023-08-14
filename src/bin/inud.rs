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

use std::{io::Error as IoError, path::PathBuf};

use anyhow::{Context, Result};
use thiserror::Error;

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use twelf::{config, Error as TwelfError, Layer};

#[config]
#[allow(dead_code)]
struct Config {
    chosen_backend: String,
    control_socket: String,
    agent_socket: String,
}

#[derive(Debug, Error)]
enum ConfInitError {
    #[error("Error building configuration")]
    ConfLoadError(#[from] TwelfError),
    #[error("Error creating parent directory to configuration file.")]
    ConfParentPathCreateError(#[from] IoError),
}

#[derive(Debug, Error)]
enum ArgsError {
    #[error("Could not get config path from Clap, including default")]
    ArgConfigPathGetError(#[source] clap::parser::MatchesError),
}

type ConfInitResult<T, E = ConfInitError> = Result<T, E>;

fn try_load_conf(path: &PathBuf) -> ConfInitResult<Config> {
    Ok(Config::with_layers(&[
        Layer::Yaml(path.into()),
        Layer::Env(Some(String::from("INUUS_"))),
    ])?)
}

fn get_default_cfg_path() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join(PathBuf::from("inuus/config.yml"))
}

fn get_args() -> ArgMatches {
    Command::new(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .arg(
            Arg::new("verbosity")
                .short('v')
                .action(ArgAction::Count)
                .help("Increment to get verbose logs."),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .value_name("FILE")
                .value_parser(value_parser!(PathBuf))
                .default_value(get_default_cfg_path().into_os_string())
                .help("Optional path to TOML configuration."),
        )
        .subcommand(Command::new("spawn").about("Starts the daemon."))
        .get_matches()
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = get_args();

    let _cfg = try_load_conf(
        args.try_get_one::<PathBuf>("config")
            .map_err(ArgsError::ArgConfigPathGetError)
            .context("Maybe the conf path is missing?")?
            .unwrap(),
    )
    .context("Error loading configuration into memory.")?;

    Ok(())
}
