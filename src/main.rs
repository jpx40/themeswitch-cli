use clap::{
    arg, command, value_parser, ArgAction, ArgGroup, Command, Parser, Subcommand, ValueEnum,
};
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process;
use toml;
mod cli;
mod conf;
mod template;
mod theme;
mod utils;

#[derive(Parser, Debug)]
struct Cli {
    // args: Args,
    // #[clap(subcommand)]
    // command: Command,
    theme: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Config {
    theme: Option<String>,
    applications_theme: Option<HashMap<String, bool>>,
}

fn main() {
    // let matches = command!().arg(arg!(--app <APP>)).get_matches();
    // match &matches.get_one::<String>("app").clone() {
    //     Some(String::from("neovim")) => println!(
    //         "{:?}, got neovim",
    //         get_app_val("neovim").unwrap_or_else(|err| panic!("{}", err))
    //     ),
    //     _ => {}
    // };
    let cli = Cli::parse();
}

fn get_app_val(app: &str) -> Result<bool, String> {
    let config_file = "config.toml";
    let config = conf::read_config(config_file).unwrap_or_else(|err| panic!("{}", err));
    if let Some(apps) = config.applications_theme {
        for (key, value) in apps {
            if key == app {
                return Ok(value);
            }
        }
    }
    return Err("key not found".to_string());
}
