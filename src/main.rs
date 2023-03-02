use std::process::Command;

use crate::homebrew::Homebrew;

mod homebrew;

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
}
