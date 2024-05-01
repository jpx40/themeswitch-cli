use crate::wallpaper::WallpaperList;
use etcetera::base_strategy::{choose_base_strategy, BaseStrategy};
use lazy_static::{lazy_static, LazyStatic};
use ron;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str,
    sync::Mutex,
};
mod toml_stuff;

// lots of stuff copied from https://github.com/helix-editor/helix/blob/master/helix-loader/src/lib.rs

//TODO: need to change when found a name
pub const NAME: &str = "wayland_utils";

static CONFIG_FILE: once_cell::sync::OnceCell<PathBuf> = once_cell::sync::OnceCell::new();

static LOG_FILE: once_cell::sync::OnceCell<PathBuf> = once_cell::sync::OnceCell::new();
lazy_static! {
    static ref CONFIG: Mutex<Conf> = Mutex::new(Conf::new());
}

pub fn initialize_log_file(specified_file: Option<PathBuf>) {
    let log_file = specified_file.unwrap_or_else(default_log_file);
    ensure_parent_dir(&log_file);
    LOG_FILE.set(log_file).ok();
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conf {}

impl Conf {
    pub fn new() -> Conf {
        Conf {}
    }
}

pub fn default_log_file() -> PathBuf {
    cache_dir().join(NAME.to_string() + ".log")
}

pub fn cache_dir() -> PathBuf {
    // TODO: allow env var override
    let strategy = choose_base_strategy().expect("Unable to find the cache directory!");
    let mut path = strategy.cache_dir();
    path.push(NAME);
    path
}
pub fn initialize_config_file(specified_file: Option<PathBuf>) {
    let config_file = specified_file.unwrap_or_else(default_config_file);
    ensure_parent_dir(&config_file);
    CONFIG_FILE.set(config_file).ok();
}

pub fn read_config(path: String) -> Result<Conf, String> {
    if !Path::new(&path).exists() && !Path::new(&path).is_file() {
        return Err("File not found".to_string());
    }
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    let mut buffer = String::new();
    if let Err(e) = file.read_to_string(&mut buffer) {
        return Err(e.to_string());
    }
    let out: Conf = match ron::from_str(&buffer) {
        Ok(b) => b,
        Err(e) => return Err(e.to_string()),
    };

    Ok(out)
}

pub fn read_wallpaper_config(path: String) -> Result<WallpaperList, String> {
    if !Path::new(&path).exists() && !Path::new(&path).is_file() {
        return Err("File not found".to_string());
    }
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    let mut buffer = String::new();
    if let Err(e) = file.read_to_string(&mut buffer) {
        return Err(e.to_string());
    }
    let out: WallpaperList = match ron::from_str(&buffer) {
        Ok(b) => b,
        Err(e) => return Err(e.to_string()),
    };

    Ok(out)
}
pub fn config_dir() -> PathBuf {
    // TODO: allow env var override
    let strategy = choose_base_strategy().expect("Unable to find the config directory!");
    let mut path = strategy.config_dir();
    path.push(NAME);
    path
}

fn default_config_file() -> PathBuf {
    config_dir().join("config.toml")
}

fn ensure_parent_dir(path: &Path) {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).ok();
        }
    }
}
