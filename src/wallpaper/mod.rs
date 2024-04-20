use camino::Utf8Path;
use core::str;
use execute::Execute;
use lazy_static::{lazy_static, LazyStatic};
use std::path::Path;
use std::process::Command;
use std::{fs::File, path, sync::Mutex};

lazy_static! {
    static ref ACTIVE_WALLPAPER: Mutex<Wallpaper> = Mutex::new(Wallpaper::new(
        "default.png".to_string(),
        "~/.wallpaper/default.png".to_string()
    ));
}
lazy_static! {
    static ref ACTIVE_GROUP: Mutex<Group> = Mutex::new(Group::new("default_group".to_string(),));
}
lazy_static! {
    static ref ACTIVE_WALLPAPER_LIST: Mutex<WallpaperList> = Mutex::new(WallpaperList::new());
}

#[derive(Debug, Clone)]
pub struct WallpaperList {
    list: Vec<Group>,
}
impl WallpaperList {
    fn new() -> WallpaperList {
        WallpaperList { list: Vec::new() }
    }
}
#[derive(Debug, Clone)]
pub struct Group {
    name: String,
    list: Vec<Wallpaper>,
}
impl Group {
    fn new(name: String) -> Group {
        Group {
            name,
            list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Wallpaper {
    pub name: String,
    pub path: String,
    pub config: Option<WallpaperConfig>,
}
#[derive(Debug, Clone)]
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
        println!("{}", self.path);
        let path = Utf8Path::new(&self.path)
            .canonicalize_utf8()
            .unwrap_or_else(|err| {
                panic!("{}", err.to_string());
            });

        if !Path::new(&path).exists() && !Path::new(&path).is_file() {
            return Err("File not found".to_string());
        }
        let mut sh = Command::new("swww");
        //https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
        if let Ok(mut value) = ACTIVE_WALLPAPER.lock() {
            *value = self.clone()
        }
        println!("{}", path);
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
