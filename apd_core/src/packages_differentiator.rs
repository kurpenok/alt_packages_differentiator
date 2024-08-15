use itertools::izip;
use std::collections::HashMap;

use crate::api_response::PackagesData;
use crate::difference::PackagesDifference;
use crate::package::Package;

fn get_above_version(version_1: String, version_2: String) -> String {
    let splitted_version_1: Vec<&str> = version_1.split(".").collect();
    let splitted_version_2: Vec<&str> = version_2.split(".").collect();

    for (part_1, part_2) in izip!(&splitted_version_1, &splitted_version_2) {
        let part_1 = part_1.parse::<i32>();
        let part_2 = part_2.parse::<i32>();

        if part_1.is_err() || part_2.is_err() {
            return version_1;
        }
        let part_1: i32 = part_1.unwrap();
        let part_2: i32 = part_2.unwrap();

        if part_1 == part_2 {
            continue;
        } else if part_1 < part_2 {
            return version_2;
        } else {
            return version_1;
        }
    }

    version_1
}

fn get_difference_with_version(
    packages_1: &Vec<Package>,
    packages_2: &Vec<Package>,
) -> Vec<Package> {
    vec![]
    //let mut difference: HashSet<Package> = HashSet::new();
    //
    //let mut packages_hm_2: HashMap<String, Package> = HashMap::new();
    //for package in packages_2 {
    //    packages_hm_2.insert(package.name.clone(), package.clone());
    //}
    //
    //for package in packages_1 {
    //    if packages_hm_2.contains_key(&package.name)
    //        && package.version
    //            == get_above_version(
    //                package.version.clone(),
    //                packages_hm_2.contains_key(&package.name).to_string(),
    //            )
    //    {
    //        difference.insert(package.clone());
    //    }
    //}
    //
    //difference
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
            epoch: package.epoch,
            version: package.version,
            release: package.release,
        });
    }

    for package in packages_data_2.packages {
        packages_2.push(Package {
            name: package.name,
            epoch: package.epoch,
            version: package.version,
            release: package.release,
        });
    }

    let first_branch_unique_packages = get_difference_by_name(&packages_1, &packages_2);
    let second_branch_unique_packages = get_difference_by_name(&packages_2, &packages_1);
    let packages_with_above_version = get_difference_with_version(&packages_1, &packages_2);

    PackagesDifference {
        first_branch_unique_packages,
        second_branch_unique_packages,
        packages_with_above_version,
    }
}

#[cfg(test)]
mod tests {
    use crate::packages_differentiator::get_above_version;

    #[test]
    fn test_get_packages() {
        let packages_data = crate::packages_parser::get_packages("p11", "x86_64");
        assert_eq!(packages_data.is_some(), true);
    }

    #[test]
    fn test_get_above_version() {
        let version_1: String = "1.10.100".to_string();
        let version_2: String = "1.10.99".to_string();

        assert_eq!(get_above_version(version_1, version_2), "1.10.100");
    }
}
