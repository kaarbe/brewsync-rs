use std::fs::create_dir;
use std::fs::File;
use std::path::Path;
use home::home_dir;
use crate::package_type::PackageType;

pub struct FileMaker {
    brewsync_path: String
}

impl FileMaker {

    pub fn new() -> FileMaker {
        let err_msg = "Unable to read home directory";
        let home_path = home_dir()
            .expect(err_msg)
            .into_os_string()
            .into_string()
            .expect(err_msg);
        let brewsync_path = format!("{}/{}", home_path, ".brewsync");
        return FileMaker { brewsync_path };
    }

    pub fn make_backup_dir(&self) -> Result<(), ()> {
        if Path::new(&self.brewsync_path).exists() {
            return Ok(());
        }
        return match create_dir(&self.brewsync_path) {
            Ok(_) => Ok(()),
            Err(_error) => Err(()),
        };
    }
    
    pub fn make_for_formulas(&self) -> Option<File> {
        return self.make_backup_file(PackageType::Formulae);
    }

    pub fn make_for_casks(&self) -> Option<File> {
        return self.make_backup_file(PackageType::Cask);
    }

    fn make_backup_file(&self, package_type: PackageType) -> Option<File> {
        let file_name = match package_type {
            PackageType::Formulae => "formulas",
            PackageType::Cask => "casks",
        };
        let file_path = format!("{}/{}", self.brewsync_path, file_name);
        return File::create(file_path)
            .map_or(
                None,
                |file| Some(file)
            );
    }
}

