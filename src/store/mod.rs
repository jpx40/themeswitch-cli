use crate::parser::Imports;
use crate::parser::Profile;
use crate::Variable;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
lazy_static! {
    pub static ref PROFILE: Mutex<HashMap<String, Profile>> = Mutex::new(HashMap::new());
}

lazy_static! {
    pub static ref IMPORT: Mutex<Vec<Imports>> = Mutex::new(Vec::new());
}

lazy_static! {
    pub static ref GLOBAL_VARIABEL: Mutex<HashMap<String, Variable>> = Mutex::new(HashMap::new());
}
