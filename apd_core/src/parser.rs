use reqwest::blocking::Client;

pub fn get_brach_packages_data(repo: &str) -> Option<String> {
    let api_url = "https://rdb.altlinux.org/api/export/branch_binary_packages/";
    let full_url = format!("{api_url}{repo}");

    let client = Client::new();
    let response = match client.get(full_url).send() {
        Ok(response) => response,
        _ => {
            println!("[-] Request error!");
            return None;
        }
    };

    if response.status() == 200 {
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
