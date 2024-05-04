use crate::parser::Profile;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
lazy_static! {
    pub static ref PROFILE: Mutex<HashMap<String, Profile>> = Mutex::new(HashMap::new());
}
