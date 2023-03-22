use std::io::Write;
use clap::{ Parser, Subcommand };

use crate::file_maker::FileMaker;

mod homebrew;
mod file_maker;
mod package_type;

#[derive(Parser)]
#[command(
    author = "kaarbe",
    about = "Backups names of all packages installed via Homebrew",
    name = "brewsync",
    version,
)]
struct Args {
   #[command(subcommand)]
   action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Configure Brewsync with values other than default
    Config,
} 

fn main() {
    let args = Args::parse();
    match args.action {
        Action::Config => print!("configuration path"),
    }

    let is_installed = homebrew::is_installed()
        .expect("Unable to verify Homebrew installation");
    if !is_installed {
        panic!("Homebrew not installed");
    }

    let file_maker = FileMaker::new();
    file_maker.make_backup_dir()
        .expect("Unable to create backup directory");

    let mut formulas_file = file_maker.make_for_formulas()
        .expect("Unable to create formulas file");
    let mut casks_file = file_maker.make_for_casks()
        .expect("Unable to create casks file");

    let casks_installed = homebrew::get_installed_casks()
        .expect("Unable to read installed casks");
    let formulas_installed = homebrew::get_installed_formulas()
        .expect("Unable to read installed formulas");
    
    casks_file.write_all(casks_installed.as_slice())
        .expect("Unable to write to file");
    formulas_file.write_all(formulas_installed.as_slice())
        .expect("Unable to write to file");
}
