use serde::{Deserialize, Serialize};

use crate::api_response::Package;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackagesDifference {
    first_branch_unique_packages: Vec<Package>,
    second_branch_unique_packages: Vec<Package>,
    packages_with_above_version: Vec<Package>,
}
