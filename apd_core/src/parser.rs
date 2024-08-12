use reqwest::blocking::Client;

use crate::api_response::PackagesData;

fn get_raw_packages_data(repo: &str, arch: &str) -> Option<String> {
    let api_url = "https://rdb.altlinux.org/api/export/branch_binary_packages/";
    let full_url = format!("{api_url}{repo}?arch={arch}");

    let client = Client::new();
    let response = match client.get(full_url).send() {
        Ok(response) => response,
        _ => {
            println!("[-] Invalid request!");
            return None;
        }
    };

    if response.status() == reqwest::StatusCode::OK {
        match response.text() {
            Ok(response_text) => Some(response_text),
            _ => {
                println!("[-] Response data unpacking error!");
                return None;
            }
        }
    } else {
        println!("[-] Request error!");
        println!("[-] Error code: {}", response.status());
        return None;
    }
}

fn parse_packages_data(raw_packages_data: String) -> Option<PackagesData> {
    match serde_json::from_str(&raw_packages_data) {
        Ok(packages_data) => Some(packages_data),
        _ => {
            println!("[-] Error response json reading!");
            return None;
        }
    }
}

pub fn get_packages(repo: &str, arch: &str) -> Option<PackagesData> {
    match get_raw_packages_data(repo, arch) {
        Some(raw_packages_data) => parse_packages_data(raw_packages_data),
        None => None,
    }
}
