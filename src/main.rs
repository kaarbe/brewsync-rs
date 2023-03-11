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

    let mut formulas_file = match FileMaker::new().make_for_formulas() {
        Ok(file) => file,
        Err(_err) => panic!("Cannot create backup file"),
    };
    let mut casks_file = match FileMaker::new().make_for_casks() {
        Ok(file) => file,
        Err(_err) => panic!("Cannot create backup file"),
    };

    let casks_read_result = Homebrew::new(Command::new("brew"))
        .get_installed_casks();

    let formulas_read_result = Homebrew::new(Command::new("brew"))
        .get_installed_formulas();

    let casks_read = match casks_read_result {
        Ok(casks) => casks,
        Err(_err) => panic!("Cannot read installed casks"),
    };
    let formulas_read = match formulas_read_result {
        Ok(formulas) => formulas,
        Err(_error) => panic!("Cannot read formulas installed"),
    };

    match casks_file.write_all(casks_read.as_slice()) {
        Ok(()) => print!("{}", String::from("Casks backup done")),
        Err(_error) => panic!("Cannot write to backup file"),
    }
    
    match formulas_file.write_all(formulas_read.as_slice()) {
        Ok(()) => print!("{}", String::from("Formulas backup done")),
        Err(_error) => panic!("Cannot write to backup file"),
    }
}
