use std::io::Write;

use crate::file_maker::FileMaker;

mod homebrew;
mod file_maker;
mod package_type;

fn main() {
    let is_installed = homebrew::is_installed()
        .expect("Unable to verify Homebrew installation");
    if !is_installed {
        panic!("Homebrew not installed");
    };

    let file_maker = FileMaker::new();
    match file_maker.make_backup_dir() {
        Ok(()) => print!("Backup directory found/created"),
        Err(()) => panic!("Unable to create backup directory"),
    }

    let mut formulas_file = file_maker 
        .make_for_formulas()
        .expect("Unable to create formulas file");
    let mut casks_file = file_maker
        .make_for_casks()
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
