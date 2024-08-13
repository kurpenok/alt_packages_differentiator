use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::package::Package;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackagesDifference {
    pub first_branch_unique_packages: HashSet<Package>,
    pub second_branch_unique_packages: HashSet<Package>,
    pub packages_with_above_version: HashSet<Package>,
}
