mod cli_args;

use clap::Parser;

use apd_core::{
    packages_differentiator::get_packages_difference, packages_parser::get_packages,
    result_saver::save,
};
use cli_args::Args;

fn main() {
    let args = Args::try_parse().unwrap();

    let repo_1 = args.repo_1;
    let repo_2 = args.repo_2;
    let available_architectures = ["x86_64", "i586", "aarch64", "ppc64le"];

    for arch in available_architectures {
        let packages_data_1 = get_packages(&repo_1, arch);
        let packages_data_2 = get_packages(&repo_2, arch);

        match (packages_data_1, packages_data_2) {
            (Some(packages_data_1), Some(packages_data_2)) => {
                let difference = get_packages_difference(packages_data_1, packages_data_2);
                println!("[+] Difference ({repo_1} - {repo_2} - {arch}) received!");
                save(difference, &repo_1, &repo_2, arch);
            }
            (_, _) => println!("[-] Fatal error!"),
        }
    }
}
