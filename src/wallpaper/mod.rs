pub mod wpaper;
use camino::Utf8Path;
use core::str;
use execute::Execute;
use globset;
use itertools::Itertools;
use lazy_static::{lazy_static, LazyStatic};

use crate::util::path::*;
use crate::util::{self, path};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use std::{collections::HashMap, fs::File, path, sync::Mutex};
use walkdir;
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

lazy_static! {
    pub static ref BACKUP_GROUP: Mutex<Group> =
        Mutex::new(Group::new("default_group".to_string(),));
}
lazy_static! {
    pub static ref BACKUP_WALLPAPER_LIST: Mutex<WallpaperList> = Mutex::new(WallpaperList::new());
}

lazy_static! {
    pub static ref SWAP_TIME: Mutex<u32> = Mutex::new(0);
}
#[derive(Hash, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct WallpaperList {
    imports: Option<Vec<Imports>>,
    pub list: Vec<Group>,
    wallpaper_config: Option<WallpaperListConfig>,
}
#[derive(Hash, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Imports {}
impl WallpaperList {
    pub fn new() -> WallpaperList {
        WallpaperList {
            list: Vec::new(),
            imports: None,
            wallpaper_config: Some(WallpaperListConfig::new()),
        }
    }
}
#[derive(Hash, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct WallpaperListConfig {
    theme: Option<crate::theme::Theme>,
    time: Option<String>,
}
impl WallpaperListConfig {
    pub fn new() -> WallpaperListConfig {
        WallpaperListConfig {
            theme: None,
            time: None,
        }
    }
}
#[derive(Hash, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Group {
    pub name: String,
    pub path: Option<String>,
    pub list: Vec<Wallpaper>,
}
impl Group {
    pub fn new(name: String) -> Group {
        Group {
            name,
            path: None,
            list: Vec::new(),
        }
    }

    pub fn set_path(&mut self, path: &str) {
        let path = util::path::expand_path(path).unwrap_or_else(|e| panic!("{}", e));
        self.path = Some(path);
    }
    pub fn fill_wallpaper(&mut self) -> Result<(), String> {
        if let Some(path) = &self.path {
            let path = Utf8Path::new(path);
            if path.exists() {
                if path.is_dir() {
                    let walker = walkdir::WalkDir::new(path).unwrap().into_iter();
                    for entry in walker
                        .filter_entry(|e| !is_hidden(e))
                        .filter_entry(|e| !e.is_dir())
                    {
                        if let Ok(entry) = entry {
                            if entry.path().is_file() && entry.path().extension().is_some() {
                                let file_name = entry
                                    .file_name()
                                    .to_str()
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string();
                                let path: String = entry
                                    .path()
                                    .extension()
                                    .unwrap()
                                    .to_string_lossy()
                                    .unwrap()
                                    .to_string();

                                let extension =
                                    entry.path().strip_prefix(path).unwrap().to_str().unwrap();
                                let prefix: String = entry
                                    .path()
                                    .file_prefix()
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string();
                                match extension {
                                    "jpg" | "png" | "jpeg" | "webp" | "gif | .jpg" | ".png"
                                    | ".jpeg" | ".webp" | ".gif" => {
                                        let path = util::path::expand_path(&path)
                                            .unwrap_or_else(|err| panic!("{err}"));

                                        let wp = Wallpaper::new(prefix, path);
                                        self.list.push(wp);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    return Err("Not a directory".to_string());
                }
            }
            return Err("Path not found".to_string());
        }
        Ok(())
    }

    pub fn set_group(&mut self) {
        if let Ok(mut list) = ACTIVE_WALLPAPER_LIST.lock() {
            let len = list.list.len();
            if let Some(i) = list.list.iter().position(|x| x == self) {
                list.list.remove(i);
                list.list.insert(len + i, self.clone())
            };

            if !list.list.iter().all_unique() {
                //panic!("Duplicate group")
                list.list = list.list.clone().into_iter().dedup().collect();
            }

            if let Ok(mut value) = BACKUP_WALLPAPER_LIST.lock() {
                *value = list.clone();
            }
        }
        if let Ok(mut value) = ACTIVE_GROUP.lock() {
            *value = self.clone()
        }
        if let Ok(mut value) = BACKUP_GROUP.lock() {
            *value = self.clone()
        }
        let _ = self.list.first().unwrap().set_wallpaper();
    }
}

pub fn get_current_group() -> Group {
    ACTIVE_GROUP
        .lock()
        .unwrap_or_else(|err| panic!("{}", err))
        .clone()
}
pub fn get_current_wallpaper() -> Wallpaper {
    ACTIVE_WALLPAPER
        .lock()
        .unwrap_or_else(|err| panic!("{}", err))
        .clone()
}
#[derive(Hash, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Wallpaper {
    pub name: String,
    pub path: String,

    pub config: Option<WallpaperConfig>,
}
#[derive(Hash, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct WallpaperConfig {
    theme: Option<crate::theme::Theme>,
    time: Option<String>,
}

impl Wallpaper {
    pub fn new(name: String, path: String) -> Wallpaper {
        Wallpaper {
            name,
            path,
            config: None,
        }
    }
    pub fn set_wallpaper(&self) -> Result<(), String> {
        let path: String = match expand_path(&self.path) {
            Ok(path) => path,
            Err(err) => return Err(err.to_string()),
        };

        // if self.path.contains('~') {
        //     path = match dirs::home_dir() {
        //         Some(p) => {
        //             let path = self.path.clone().replace("~", "");
        //             p.canonicalize()
        //                 .unwrap_or_else(|err| panic!("{}", err.to_string()))
        //                 .to_string_lossy()
        //                 .to_string()
        //                 + &path
        //         }
        //         None => return Err("Failed to read file".to_string()),
        //     };
        // } else {
        //     path = match dirs::config_dir() {
        //         Some(p) => {
        //             p.canonicalize()
        //                 .unwrap_or_else(|err| panic!("{}", err.to_string()))
        //                 .to_string_lossy()
        //                 .to_string()
        //                 + "/swww/"
        //                 + &self.path
        //         }
        //         None => return Err("Failed to read file".to_string()),
        //     };
        // }
        // println!("{}", path);
        // let path = Utf8Path::new(&path)
        //     .canonicalize_utf8()
        //     .unwrap_or_else(|err| {
        //         panic!("{}", err.to_string());
        //     });

        // if !Path::new(&path).exists() && !Path::new(&path).is_file() {
        //     return Err("File not found".to_string());
        // }
        let mut sh = Command::new("swww");
        //https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
        if let Ok(mut value) = ACTIVE_WALLPAPER.lock() {
            *value = self.clone()
        }

        if let Ok(mut value) = ACTIVE_GROUP.lock() {
            if let Some(i) = value.list.iter().position(|x| x == self) {
                let len = value.list.len();
                value.list.remove(i);
                value.list.insert(len + i, self.clone());
            }
            if !value.list.iter().all_unique() {
                //panic!("Duplicate group")
                value.list = value.list.clone().into_iter().dedup().collect();
            }
        }

        if let Ok(mut value) = BACKUP_GROUP.lock() {
            *value = get_current_group();
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
