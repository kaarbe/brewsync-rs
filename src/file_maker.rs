use crate::package_type::PackageType;
use home::home_dir;
use std::fs::create_dir;
use std::fs::File;
use std::path::Path;

pub struct FileMaker {
  brewsync_path: String,
  config_path: String,
}

impl FileMaker {
  pub fn new() -> Option<FileMaker> {
    let home_path_read: Option<String> = home_dir()
        .map_or(None, |path_buf| Some(path_buf.into_os_string()))
        .map_or(None, |os_string| os_string.into_string().ok());
    if let Some(home_path) = home_path_read {
      let brewsync_path = format!("{}/{}", home_path, ".brewsync");
      let config_path = format!("{}/{}", home_path, ".config");
      return Some(FileMaker { brewsync_path, config_path });
    } else {
      return None;
    }
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
    return File::create(file_path).ok();
  }

  pub fn make_config_dir(&self) -> Result<(), ()> {
    if Path::new(&self.config_path).exists() {
      return Ok(());
    }
    return match create_dir(&self.config_path) {
      Ok(_) => Ok(()),
      Err(_error) => Err(()),
    };
  }
}
