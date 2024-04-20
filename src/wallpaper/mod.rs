use camino::Utf8Path;
use core::str;
use execute::Execute;
use lazy_static::{lazy_static, LazyStatic};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use std::{fs::File, path, sync::Mutex};

lazy_static! {
    pub static ref ACTIVE_WALLPAPER: Mutex<Wallpaper> = Mutex::new(Wallpaper::new(
        "default.png".to_string(),
        "~/.wallpaper/default.png".to_string()
    ));
}
lazy_static! {
    pub static ref ACTIVE_GROUP: Mutex<Group> =
        Mutex::new(Group::new("default_group".to_string(),));
}
lazy_static! {
    pub static ref ACTIVE_WALLPAPER_LIST: Mutex<WallpaperList> = Mutex::new(WallpaperList::new());
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WallpaperList {
    imports: Option<Vec<Imports>>,
    pub list: Vec<Group>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Imports {}
impl WallpaperList {
    pub fn new() -> WallpaperList {
        WallpaperList {
            list: Vec::new(),
            imports: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    pub name: String,
    pub list: Vec<Wallpaper>,
}
impl Group {
    pub fn new(name: String) -> Group {
        Group {
            name,
            list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Wallpaper {
    pub name: String,
    pub path: String,
    pub config: Option<WallpaperConfig>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WallpaperConfig {}

impl Wallpaper {
    pub fn new(name: String, path: String) -> Wallpaper {
        Wallpaper {
            name,
            path,
            config: None,
        }
    }
    pub fn set_wallpaper(&self) -> Result<(), String> {
        let mut path: String = String::new();
        if self.path.contains('~') {
            path = match dirs::home_dir() {
                Some(p) => {
                    let path = self.path.clone().replace("~", "");
                    p.canonicalize()
                        .unwrap_or_else(|err| panic!("{}", err.to_string()))
                        .to_string_lossy()
                        .to_string()
                        + &path
                }
                None => return Err("Failed to read file".to_string()),
            };
        } else {
            path = match dirs::config_dir() {
                Some(p) => {
                    p.canonicalize()
                        .unwrap_or_else(|err| panic!("{}", err.to_string()))
                        .to_string_lossy()
                        .to_string()
                        + "/swww/"
                        + &self.path
                }
                None => return Err("Failed to read file".to_string()),
            };
        }
        println!("{}", path);
        // let path = Utf8Path::new(&path)
        //     .canonicalize_utf8()
        //     .unwrap_or_else(|err| {
        //         panic!("{}", err.to_string());
        //     });

        if !Path::new(&path).exists() && !Path::new(&path).is_file() {
            return Err("File not found".to_string());
        }
        let mut sh = Command::new("swww");
        //https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
        if let Ok(mut value) = ACTIVE_WALLPAPER.lock() {
            *value = self.clone()
        }

        match &self.config {
            Some(c) => sh.arg("img").arg(&path),
            None => sh.arg("img").arg(&path),
        };

        if let Err(e) = sh.execute_check_exit_status_code(0) {
            return Err(e.to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Wallpaper;

    #[test]
    fn test_set_wallpaper() {
        let w = Wallpaper {
            name: "cat_lofi_cafe".to_string(),
            path: "~/.config/swww/Tokyo-Night/edger_lucy_neon.jpg".to_string(),
            config: None,
        };

        match w.set_wallpaper() {
            Ok(_) => println!("test_set_wallpaper was successfull"),
            Err(e) => eprintln!("{e}"),
        }
    }
}
