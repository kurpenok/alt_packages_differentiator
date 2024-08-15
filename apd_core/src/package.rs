use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub epoch: u32,
    pub version: String,
    pub release: String,
}
