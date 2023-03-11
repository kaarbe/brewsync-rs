use std::fs::create_dir;
use std::fs::File;
use std::path::Path;
use home::home_dir;
use crate::package_type::PackageType;

pub struct FileMaker {
    home_path: String
}

impl FileMaker {

    pub fn new() -> FileMaker {
        let err_msg = "Unable to read home directory";
        let home_path = home_dir()
            .expect(err_msg)
            .into_os_string()
            .into_string()
            .expect(err_msg);
        return FileMaker { home_path };
    }

    pub fn make_backup_dir(&self) -> Result<(), ()> {
        let brewsync_path = format!(
            "{}/{}", self.home_path, ".brewsync");
        if Path::new(&brewsync_path).exists() {
            return Ok(());
        }
        return match create_dir(brewsync_path) {
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

    fn make_backup_file(&self, ptype: PackageType) -> Option<File> {
        let name = match ptype {
            PackageType::Formulae => "formulas",
            PackageType::Cask => "casks",
        };
        let file_path = format!(
            "{}/.brewsync/{}", self.home_path, name);
        return File::create(file_path)
            .map_or(
                None,
                |file| Some(file)
            );
    }
}
