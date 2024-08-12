use apd_core::packages_parser::get_packages;

fn main() {
    match get_packages("p10", "x86_64") {
        Some(packages_data) => println!("{:?}", packages_data),
        None => println!("[-] Fatal error!"),
    }
}
