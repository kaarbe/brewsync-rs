use std::process::Command;

use crate::package_type::PackageType;

pub fn is_installed() -> Option<bool> {
  return Command::new("brew")
    .arg("--version")
    .output()
    .map_or(None, |output| Some(String::from_utf8(output.stdout)))
    .map_or(None, |stdout| Some(stdout.unwrap_or(String::new())))
    .map_or(None, |text| Some(!text.contains("brew: command not found")));
}

pub fn get_installed_formulas() -> Option<Vec<u8>> {
  return get_installed(PackageType::Formulae);
}

pub fn get_installed_casks() -> Option<Vec<u8>> {
  return get_installed(PackageType::Cask);
}

fn get_installed(package_type: PackageType) -> Option<Vec<u8>> {
  return Command::new("brew")
    .arg("list")
    .arg(get_type_arg_for(package_type))
    .output()
    .map_or(None, |output| Some(output.stdout));
}

fn get_type_arg_for(package_type: PackageType) -> String {
  return match package_type {
    PackageType::Formulae => "--formulae".to_string(),
    PackageType::Cask => "--cask".to_string(),
  };
}
