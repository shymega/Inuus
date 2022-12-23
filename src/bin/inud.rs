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

use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Command};
use config::{Config, ConfigError, Environment, File};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn try_load_conf(path: &PathBuf) -> Result<Config, ConfigError> {
    match Config::builder()
        .add_source(File::from(path.as_path().clone()))
        .add_source(Environment::with_prefix("INUD"))
        .build()
    {
        Ok(cfg) => Ok(cfg),
        Err(e) => Err(e),
    }
}

fn get_cfg_path_default() -> Option<PathBuf> {
    let def_path = dirs::config_dir()
        .unwrap()
        .join(PathBuf::from("inuus/config.toml"));

    if def_path.exists() {
        Some(def_path)
    } else {
        None
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
            get_cfg_path_default().expect("Unable to get a default conf path.")
        }
    };

    let _cfg = try_load_conf(&arg_cfg_path);

    Ok(())
}
