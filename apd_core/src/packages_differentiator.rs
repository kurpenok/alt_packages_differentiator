use crate::api_response::{Package, PackagesData};
use crate::difference::PackagesDifference;

fn get_difference_with_version(
    packages_1: &Vec<Package>,
    packages_2: &Vec<Package>,
) -> Vec<Package> {
    let mut difference: Vec<Package> = Vec::new();

    for package in packages_1 {
        for compare_package in packages_2 {
            if package.name == compare_package.name && package.version > compare_package.version {
                difference.push(package.clone());
                break;
            }
        }
    }

    difference
}

fn get_difference(packages_1: &Vec<Package>, packages_2: &Vec<Package>) -> Vec<Package> {
    let mut difference: Vec<Package> = Vec::new();

    for package in packages_1 {
        for compare_package in packages_2 {
            if package.name == compare_package.name && package.version == compare_package.version {
                difference.push(package.clone());
                break;
            }
        }
    }

    difference
}

pub fn get_packages_difference(
    packages_data_1: PackagesData,
    packages_data_2: PackagesData,
) -> PackagesDifference {
    let packages_1 = packages_data_1.packages;
    let packages_2 = packages_data_2.packages;

    let first_branch_unique_packages = get_difference(&packages_1, &packages_2);
    let second_branch_unique_packages = get_difference(&packages_2, &packages_1);
    let packages_with_above_version: Vec<Package> =
        get_difference_with_version(&packages_1, &packages_2);

    PackagesDifference {
        first_branch_unique_packages,
        second_branch_unique_packages,
        packages_with_above_version,
    }
}
