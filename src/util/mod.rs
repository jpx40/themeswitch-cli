pub mod env;
pub mod path;

use std::collections::HashMap;
pub fn json_to_hashmap(path: &str) -> HashMap<String, String> {
    let path = path::expand_path(path).unwrap_or_else(|err| panic!("{}", err));
    let file = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("{}", err));

    serde_json::from_str::<HashMap<String, String>>(&file).unwrap_or_else(|err| panic!("{}", err))
}
