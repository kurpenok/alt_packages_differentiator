use itertools::izip;

use crate::package::{Package, VersionReleasePackage};

//let old_and_current_packages: Vec<Package> = Vec::new();
//VersionReleasePackage {
//                name: package.name.clone(),
//                version: VersionsDifference {
//                    old_version: old_and_current_packages[0].version.clone(),
//                    current_version: old_and_current_packages[1].version.clone(),
//                },
//                release: ReleasesDifference {
//                    old_release: old_and_current_packages[.release.clone(),
//                    current_release: old_and_current_packages[1].release.clone(),
//                },
//            }

pub fn calculate_version_release_difference(
    package_1: &Package,
    package_2: &Package,
) -> VersionReleasePackage {
    todo!();
}

pub fn get_above_version(version_1: String, version_2: String) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_above_version() {
        let version_1: String = "1.10.100".to_string();
        let version_2: String = "1.10.99".to_string();

        assert_eq!(get_above_version(version_1, version_2), "1.10.100");
    }
}
