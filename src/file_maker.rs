use std::fs::create_dir;
use std::fs::File;
use std::path::Path;
use home::home_dir;
use crate::package_type::PackageType;

pub struct FileMaker;

impl FileMaker {

    pub fn new() -> FileMaker {
        return FileMaker {};
    }

    pub fn make_backup_dir(&self) -> Result<(), ()> {
        let brewsync_path = format!(
            "{}/{}", self.get_home_dir_path(), ".brewsync");
        if Path::new(&brewsync_path).exists() {
            return Ok(());
        }
        return match create_dir(brewsync_path) {
            Ok(_) => Ok(()),
            Err(_error) => Err(()),
        };
    }

    fn get_home_dir_path(&self) -> String {
        let err_msg = "Unable to read home directory";
        return home_dir()
            .expect(err_msg)
            .into_os_string()
            .into_string()
            .expect(err_msg);
    }
    
    pub fn make_for_formulas(&self) -> Result<File, ()> {
        return self.make_backup_file(PackageType::Formulae);
    }

    pub fn make_for_casks(&self) -> Result<File, ()> {
        return self.make_backup_file(PackageType::Cask);
    }

    fn make_backup_file(&self, ptype: PackageType) -> Result<File, ()> {
        let name = match ptype {
            PackageType::Formulae => "formulas",
            PackageType::Cask => "casks",
        };
        let file_path = format!(
            "{}/.brewsync/{}", self.get_home_dir_path(), name);
        return match File::create(file_path) {
            Ok(file) => Ok(file),
            Err(_error) => Err(()),
        };
    }
}
