use std::process::Command;

use crate::homebrew::Homebrew;

mod homebrew;

fn main() {
    let is_installed = Homebrew::new(Command::new("brew")).check_if_installed();
    match is_installed {
        Ok(is_installed) => print!("Brew is installed: {}", is_installed),
        Err(_error) => print!("Error during installation check")
    };
}
