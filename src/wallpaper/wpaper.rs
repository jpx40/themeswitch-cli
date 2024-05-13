use crate::conf;
use crate::conf::Conf;
use crate::parser::Profile;
use crate::util::path::*;
use crate::util::path::*;
use crate::wallpaper::Group;
use crate::wallpaper::Wallpaper;
use crate::wallpaper::WallpaperConfig;
use crate::wallpaper::WallpaperList;
use crate::wallpaper::*;
use crate::Wpaper;
use camino::{ReadDirUtf8, Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Paper {
    Wallpaper(Wallpaper),
    Wpaper(Wpaper),
    Group(Group),
}

pub fn check_paper(paper: Paper) -> Result<Paper, String> {
    match paper {
        Paper::Wallpaper(wallpaper) => Ok(Paper::Wallpaper(wallpaper)),
        Paper::Group(group) => Ok(Paper::Group(group)),
        Paper::Wpaper(wallpaper) => {
            if wallpaper.path.is_some() {
                let path_str = wallpaper.path.unwrap();

                if Path::new(&path_str).exists() {
                    let path = Utf8Path::new(&path_str);

                    if path.is_dir() {
                        let name = path.file_name().unwrap().to_string_lossy().to_string();

                        let mut group = Group::new(name);

                        if let Some(config) = wallpaper.config {
                            for (k, v) in config.iter() {
                                match k.as_str() {
                                    "time" => group.config.set_time(v.to_string()),
                                    "engine" => group.config.set_engine(v.to_string()),
                                    "theme" => {}
                                    "timer" => group.config.set_time(v.to_string()),
                                    _ => {}
                                }
                            }
                        }

                        if let Some(engine) = wallpaper.engine {
                            group.config.set_engine(engine)
                        }
                        group.path = Some(path_str);
                        group.fill_wallpaper()?;
                        Ok(Paper::Group(group))
                    } else if path.is_file() {
                        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                        let name = file_name.split('.').next().unwrap().to_string();

                        let mut wp = Wallpaper::new(name, path_str);
                        if let Some(config) = wallpaper.config {
                            for (k, v) in config.iter() {
                                match k.as_str() {
                                    "time" => wp.config.set_time(v.to_string()),
                                    "engine" => wp.config.set_engine(v.to_string()),
                                    "theme" => {}
                                    "timer" => wp.config.set_time(v.to_string()),
                                    _ => {}
                                }
                            }
                        }
                        if let Some(engine) = wallpaper.engine {
                            wp.config.set_engine(engine)
                        }
                        return Ok(Paper::Wallpaper(wp));
                    } else {
                        return Err("path of wallpaper not found".to_string());
                    }
                } else {
                    Err("path of Wallpaper not found".to_string())
                }
            } else {
                Err("no path of wallpaper".to_string())
            }
        }
    }
}
