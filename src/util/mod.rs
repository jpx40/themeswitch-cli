pub mod env;
pub mod path;
use camino::Utf8Path;
use chrono::format;
use compact_str::ToCompactString;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn file_to_hashmap(path: &str) -> Result<HashMap<String, String>, String> {
    let path_str = path::expand_path(path).unwrap_or_else(|err| panic!("{}", err));
    let path = Path::new(&path_str);
    if path.is_file() {
        let ext = path
            .extension()
            .unwrap_or_else(|| panic!("no extension for {}", path_str));
        let ext = ext.to_string_lossy().to_compact_string();
        match ext.as_str() {
            "toml" => Ok(toml_to_hashmap(&path_str)),
            "json" => Ok(json_to_hashmap(&path_str)),
            "ron" => Ok(ron_to_hashmap(&path_str)),

            _ => Err(format!("Unsupported file extension | {}", ext)),
        }
    } else {
        Err(format!("{path_str} is not a file"))
    }
}
pub fn ron_to_hashmap(path: &str) -> HashMap<String, String> {
    let file = std::fs::read_to_string(path).unwrap_or_else(|err| panic!("{}", err));

    ron::from_str::<HashMap<String, String>>(&file).unwrap_or_else(|err| panic!("{}", err))
}

pub fn toml_to_hashmap(path: &str) -> HashMap<String, String> {
    let file = std::fs::read_to_string(path).unwrap_or_else(|err| panic!("{}", err));

    toml::from_str::<HashMap<String, String>>(&file).unwrap_or_else(|err| panic!("{}", err))
}
pub fn json_to_hashmap(path: &str) -> HashMap<String, String> {
    let file = std::fs::read_to_string(path).unwrap_or_else(|err| panic!("{}", err));

    serde_json::from_str::<HashMap<String, String>>(&file).unwrap_or_else(|err| panic!("{}", err))
}
