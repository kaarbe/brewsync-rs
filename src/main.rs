use std::io::Write;
use std::process::Command;

use crate::homebrew::Homebrew;
use crate::file_maker::FileMaker;

mod homebrew;
mod file_maker;
mod package_type;

fn main() {
    let is_installed = Homebrew::new(Command::new("brew")).check_if_installed();
    match is_installed {
        Ok(is_installed) => {
            if !is_installed {
                panic!("Homebrew not installed");
            }
        },
        Err(error) => panic!("{}", error),
    };

    match FileMaker::new().make_backup_dir() {
        Ok(()) => print!("Backup directory found/created"),
        Err(()) => panic!("Unable to create backup directory"),
    }

    let mut formulas_file = FileMaker::new()
        .make_for_formulas()
        .expect("Unable to create formulas file");
    let mut casks_file = FileMaker::new()
        .make_for_casks()
        .expect("Unable to create casks file");

    let casks_installed = Homebrew::new(Command::new("brew"))
        .get_installed_casks()
        .expect("Unable to read installed casks");
    let formulas_installed = Homebrew::new(Command::new("brew"))
        .get_installed_formulas()
        .expect("Unable to read installed formulas");
    
    casks_file.write_all(casks_installed.as_slice())
        .expect("Unable to write to file");
    formulas_file.write_all(formulas_installed.as_slice())
        .expect("Unable to write to file");
}
