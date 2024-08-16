use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[arg(long)]
    pub repo_1: String,

    #[arg(long)]
    pub repo_2: String,

    #[arg(long)]
    pub arch: String,
}
