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

struct ThemeConfig {
    theme: String,
}

struct Theme {
    theme: String,
    file: String,
    applications: Option<Vec<String>>,
}
impl Theme {
    fn new(theme: String, file: String) -> Theme {
        Theme {
            theme,
            file,
            applications: None,
        }
    }
}

struct ThemeLixt {
    theme: Option<Vec<String>>,
}
