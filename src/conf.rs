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

pub fn read_config(file: &str) -> Result<Config, toml::de::Error> {
    let home = home_dir().unwrap().to_string_lossy().to_string();
    let config_file: String = home + file;
    let mut config_str = String::new();
    let mut file = File::open(file).unwrap();
    file.read_to_string(&mut config_str).unwrap();
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
