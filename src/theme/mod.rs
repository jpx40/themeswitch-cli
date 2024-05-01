use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Theme {
    pub name: String,
}
