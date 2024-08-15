use itertools::izip;

use crate::package::{Package, ReleasesDifference, VersionReleasePackage, VersionsDifference};

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

fn get_above_release(release_1: String, release_2: String) -> String {
    let number_release_1 = &release_1.split(".").collect::<Vec<&str>>()[0][3..].parse::<i32>();
    let number_release_2 = &release_2.split(".").collect::<Vec<&str>>()[0][3..].parse::<i32>();

    if number_release_1.is_err() || number_release_2.is_err() {
        return release_1;
    }

    if number_release_1.clone().unwrap() > number_release_2.clone().unwrap() {
        return release_1;
    }
    release_2
}

pub fn calculate_version_release_difference(
    package_1: &Package,
    package_2: &Package,
) -> VersionReleasePackage {
    if package_1.version == package_2.version {
        if package_1.release
            == get_above_release(package_1.release.clone(), package_2.release.clone())
        {
            VersionReleasePackage {
                name: package_1.name.clone(),
                versions_difference: VersionsDifference {
                    old_version: package_1.version.clone(),
                    current_version: package_1.version.clone(),
                },
                releases_difference: ReleasesDifference {
                    old_release: package_2.release.clone(),
                    current_release: package_1.release.clone(),
                },
            }
        } else {
            VersionReleasePackage {
                name: package_1.name.clone(),
                versions_difference: VersionsDifference {
                    old_version: package_1.version.clone(),
                    current_version: package_1.version.clone(),
                },
                releases_difference: ReleasesDifference {
                    old_release: package_1.release.clone(),
                    current_release: package_2.release.clone(),
                },
            }
        }
    } else {
        if package_1.version
            == get_above_version(package_1.version.clone(), package_2.version.clone())
        {
            VersionReleasePackage {
                name: package_1.name.clone(),
                versions_difference: VersionsDifference {
                    old_version: package_2.version.clone(),
                    current_version: package_1.version.clone(),
                },
                releases_difference: ReleasesDifference {
                    old_release: package_2.release.clone(),
                    current_release: package_1.release.clone(),
                },
            }
        } else {
            VersionReleasePackage {
                name: package_1.name.clone(),
                versions_difference: VersionsDifference {
                    old_version: package_1.version.clone(),
                    current_version: package_2.version.clone(),
                },
                releases_difference: ReleasesDifference {
                    old_release: package_1.release.clone(),
                    current_release: package_2.release.clone(),
                },
            }
        }
    }
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

    #[test]
    fn test_get_above_release() {
        let release_1: String = "alt1.g12345".to_string();
        let release_2: String = "alt2.10".to_string();

        assert_eq!(get_above_release(release_1, release_2), "alt2.10");
    }

    #[test]
    fn test_calculate_version_release_difference() {
        let package_1: Package = Package {
            name: "test_package".to_string(),
            version: "1.0.0".to_string(),
            release: "alt2".to_string(),
        };
        let package_2: Package = Package {
            name: "test_package".to_string(),
            version: "1.0.1".to_string(),
            release: "alt1".to_string(),
        };

        assert_eq!(
            calculate_version_release_difference(&package_1, &package_2),
            VersionReleasePackage {
                name: "test_package".to_string(),
                versions_difference: VersionsDifference {
                    old_version: "1.0.0".to_string(),
                    current_version: "1.0.1".to_string()
                },
                releases_difference: ReleasesDifference {
                    old_release: "alt2".to_string(),
                    current_release: "alt1".to_string()
                },
            }
        )
    }
}
