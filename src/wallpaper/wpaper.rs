use crate::conf::Conf;
use crate::parser::Profile;
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
    WallpaperList(WallpaperList),
}

pub fn check_paper(paper: Paper) {
    if let Paper::Wpaper(wpaper) = paper {
        if let Some(path) = &wpaper.path {
        
       }
        } else if let Some(wallpaper) = &wpaper.wallpaper {
            let wallpaper = wallpaper.to_vec();
        }
    }
}
