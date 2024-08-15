use serde::{Deserialize, Serialize};

use crate::package::{Package, VersionReleasePackage};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackagesDifference {
    pub first_branch_unique_packages: Vec<Package>,
    pub second_branch_unique_packages: Vec<Package>,
    pub above_version_release_packages: Vec<VersionReleasePackage>,
}
