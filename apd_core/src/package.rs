use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionsDifference {
    pub old_version: String,
    pub current_version: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleasesDifference {
    pub old_release: String,
    pub current_release: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionReleasePackage {
    pub name: String,
    pub version: VersionsDifference,
    pub release: ReleasesDifference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub release: String,
}
