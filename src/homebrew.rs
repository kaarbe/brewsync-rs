use std::process::Command;
use std::process::Output;

use crate::package_type::PackageType;

pub struct Homebrew {
    command: Command,
}

impl Homebrew {

    pub fn new(command: Command) -> Homebrew {
        return Homebrew { command };
    }

    pub fn check_if_installed(&mut self) -> Result<bool, String> {
        let cmd_execution = self.command
            .arg("--version")
            .output();
        return match cmd_execution {
            Ok(output) => self.output_to_bool(output),
            Err(_error) => Err(String::from("Unable to check installation")),
        };
    }

    fn output_to_bool(&self, output: Output) -> Result<bool, String> {
        let res = String::from_utf8(output.stdout.clone());
        return match res {
            Ok(content) => Ok(!content.contains("brew: command not found")),
            Err(_error) => Err(String::from("Unable to convert to string")),
        };
    }
    
    pub fn get_installed(
            &self, ptype: PackageType) -> Result<Vec<String>, String> {
        let type_arg = match ptype {
            PackageType::Formulae => "--formulae",
            PackageType::Cask => "--cask",
        };
        let cmd_execution = self.command
            .arg("list")
            .arg(type_arg)
            .output();
        return match cmd_execution {
            Ok(output) => self.output_to_vec(output),
            Err(_error) => Err(String::from("Unable to convert to string")),
        };
    }

    fn output_to_vec(&self, output: Output) -> Result<Vec<String>, String> {

    }
}

