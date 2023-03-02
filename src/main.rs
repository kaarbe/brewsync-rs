use std::process::Command;

use crate::homebrew::Homebrew;
use crate::file_maker::FileMaker;

mod homebrew;
mod file_maker;

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
    let make_dir_result = FileMaker::new().make_backup_dir();
    match make_dir_result {
        Ok(result) => print!("{}", result),
        Err(error) => panic!("{}", error),
    }
}
