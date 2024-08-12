use serde::{Deserialize, Serialize};

use crate::api_response::Package;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackagesDifference {
    pub first_branch_unique_packages: Vec<Package>,
    pub second_branch_unique_packages: Vec<Package>,
    pub packages_with_above_version: Vec<Package>,
}
