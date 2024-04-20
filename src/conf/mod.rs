use fs_extra::file;
use ron;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

use crate::wallpaper::WallpaperList;

pub fn read_wallpaper_config(path: String) -> Result<WallpaperList, String> {
    if !Path::new(&path).exists() && !Path::new(&path).is_file() {
        return Err("File not found".to_string());
    }
    let mut file = match File::open("foo.txt") {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    let mut buffer = String::new();
    if let Err(e) = file.read_to_string(&mut buffer) {
        return Err(e.to_string());
    }
    let out: WallpaperList =
        ron::from_str(&buffer).unwrap_or_else(|err| panic!("{}", err.to_string()));

    Ok(out)
}
