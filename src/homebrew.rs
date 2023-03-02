use std::process::Command;
use std::process::Output;
use std::io::Error;


pub struct Homebrew {
    command: Command,
}

impl Homebrew {

    pub fn new(command: Command) -> Homebrew {
        return Homebrew { command };
    }

    pub fn check_if_installed(&mut self) -> Result<bool, String> {
        let cmd_execution: Result<Output, Error> = self.command
            .arg("--version")
            .output();
        return match cmd_execution {
            Ok(output) => self.extract_from(output),
            Err(_error) => Err(String::from("Unable to check installation")),
        };
    }

    fn extract_from(&self, output: Output) -> Result<bool, String> {
        let res = String::from_utf8(output.stdout.clone());
        return match res {
            Ok(content) => Ok(!content.contains("brew: command not found")),
            Err(_error) => Err(String::from("Unable to convert to string")),
        };
    }
}

