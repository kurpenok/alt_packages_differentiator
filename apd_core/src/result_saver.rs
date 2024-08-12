use crate::difference::PackagesDifference;

pub fn save(packages_difference: PackagesDifference, repo_1: &str, repo_2: &str, arch: &str) {
    let filename = format!("{repo_1}_{repo_2}_{arch}.json");

    std::fs::write(
        filename,
        serde_json::to_string_pretty(&packages_difference).unwrap(),
    )
    .expect("[-] Can`t write result to file!");
}
