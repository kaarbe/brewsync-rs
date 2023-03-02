use std::fs;
use std::path::Path;

pub struct FileMaker;

impl FileMaker {

    pub fn new() -> FileMaker {
        return FileMaker {};
    }

    pub fn make_backup_dir(&self) -> Result<String, String> {
        let does_exist = Path::new(".brewsync").exists();
        if does_exist {
            return Ok(String::from("Backup directory found"));
        }
        return match fs::create_dir(".brewsync") {
            Ok(_) => Ok(String::from("Backup directory created")),
            Err(_error) => Err(String::from("Unable to create backup dir")),
        };
    }
}
