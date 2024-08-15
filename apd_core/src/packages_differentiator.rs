use std::collections::HashMap;

use crate::api_response::PackagesData;
use crate::difference::PackagesDifference;
use crate::package::{Package, VersionReleasePackage};
use crate::version_release_differentiator::calculate_version_release_difference;

fn get_difference_by_version_release(
    packages_1: &Vec<Package>,
    packages_2: &Vec<Package>,
) -> Vec<VersionReleasePackage> {
    let mut difference: Vec<VersionReleasePackage> = Vec::new();

    let mut packages_hm_2: HashMap<String, Package> = HashMap::new();
    for package in packages_2 {
        packages_hm_2.insert(package.name.clone(), package.clone());
    }

    for package in packages_1 {
        if packages_hm_2.contains_key(&package.name) {
            difference.push(calculate_version_release_difference(
                &package,
                &packages_hm_2[&package.name],
            ));
        }
    }

    difference
}

fn get_difference_by_name(packages_1: &Vec<Package>, packages_2: &Vec<Package>) -> Vec<Package> {
    let mut difference: Vec<Package> = Vec::new();

    let mut packages_hm_2: HashMap<String, Package> = HashMap::new();
    for package in packages_2 {
        packages_hm_2.insert(package.name.clone(), package.clone());
    }

    for package in packages_1 {
        if !packages_hm_2.contains_key(&package.name) {
            difference.push(package.clone());
        }
    }

    difference
}

pub fn get_packages_difference(
    packages_data_1: PackagesData,
    packages_data_2: PackagesData,
) -> PackagesDifference {
    let mut packages_1: Vec<Package> = Vec::new();
    let mut packages_2: Vec<Package> = Vec::new();

    for package in packages_data_1.packages {
        packages_1.push(Package {
            name: package.name,
            version: package.version,
            release: package.release,
        });
    }

    for package in packages_data_2.packages {
        packages_2.push(Package {
            name: package.name,
            version: package.version,
            release: package.release,
        });
    }

    let first_branch_unique_packages = get_difference_by_name(&packages_1, &packages_2);
    let second_branch_unique_packages = get_difference_by_name(&packages_2, &packages_1);
    let above_version_release_packages =
        get_difference_by_version_release(&packages_1, &packages_2);

    PackagesDifference {
        first_branch_unique_packages,
        second_branch_unique_packages,
        above_version_release_packages,
    }
}
