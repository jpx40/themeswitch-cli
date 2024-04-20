use crate::wallpaper::WallpaperList;
use lazy_static::{lazy_static, LazyStatic};
use ron;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path, sync::Mutex};

lazy_static! {
    static ref CONFIG: Mutex<Conf> = Mutex::new(Conf::new());
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conf {}

impl Conf {
    pub fn new() -> Conf {
        Conf {}
    }
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
