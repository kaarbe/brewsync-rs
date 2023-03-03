use std::fs::create_dir;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use home::home_dir;

enum PackageType {
    Formulae,
    Cask,
}

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
        return match create_dir(".brewsync") {
            Ok(_) => Ok(String::from("Backup directory created")),
            Err(_error) => Err(String::from("Unable to create backup dir")),
        };
    }
    
    pub fn make_for_formulas(&self) -> Result<File, String> {
        return self.make_backup_file(PackageType::Formulae);
    }

    pub fn make_for_casks(&self) -> Result<File, String> {
        return self.make_backup_file(PackageType::Cask);
    }

    fn make_backup_file(&self, ptype: PackageType) -> Result<File, String> {
        let name = match ptype {
            PackageType::Formulae => "formulas",
            PackageType::Cask => "casks",
        };
        let home_dir: Option<PathBuf> = home_dir();
        if home_dir.is_none() {
            return Err(String::from("Unable to read home directory"));
        }
        let home_dir_value = home_dir.map(|path_buf| {
            return path_buf
                .into_os_string()
                .into_string()
        }).unwrap();
        if home_dir_value.is_err() {
            return Err(String::from("Unable to read home directory"));
        }
        let file_path: String = format!("{}/{}", home_dir_value.unwrap(), name);
        return match File::create(file_path) {
            Ok(file) => Ok(file),
            Err(_error) => Err(String::from("Unable to create backup file")),
        };
    }
}
