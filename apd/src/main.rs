mod cli_args;

use std::collections::HashSet;

use clap::Parser;

use apd_core::{
    packages_differentiator::get_packages_difference, packages_parser::get_packages,
    result_saver::save,
};
use cli_args::Args;

fn main() {
    let args = Args::try_parse();

    if args.is_err() {
        println!("[+] ALT Linux packages differentiator!");
        println!("[+] Usage: ./apd --repo-1 sisyphus --repo-2 p10 --arch x86_64");
        return;
    }

    let args = args.unwrap();

    let repo_1 = args.repo_1;
    let repo_2 = args.repo_2;
    let arch = args.arch;

    let available_repos = HashSet::from([
        "p9".to_string(),
        "p10".to_string(),
        "c10f1".to_string(),
        "sisyphus".to_string(),
    ]);
    let available_architectures = HashSet::from([
        "x86_64".to_string(),
        "i586".to_string(),
        "aarch64".to_string(),
        "ppc64le".to_string(),
    ]);

    if available_repos.contains(&repo_1)
        && available_repos.contains(&repo_2)
        && available_architectures.contains(&arch)
    {
        let packages_data_1 = get_packages(&repo_1, &arch);
        let packages_data_2 = get_packages(&repo_2, &arch);

        match (packages_data_1, packages_data_2) {
            (Some(packages_data_1), Some(packages_data_2)) => {
                let difference = get_packages_difference(packages_data_1, packages_data_2);
                println!("[+] Difference ({repo_1} - {repo_2} - {arch}) received!");
                save(difference, &repo_1, &repo_2, &arch);
            }
            (_, _) => println!("[-] Fatal error!"),
        }
    } else {
        println!("[-] Parameters are not valid!")
    }
}
