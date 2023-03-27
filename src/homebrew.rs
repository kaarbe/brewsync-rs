use std::process::Command;

use crate::package_type::PackageType;

pub fn is_installed() -> Option<bool> {
 return Command::new("brew")
        .arg("--version")
        .output()
        .map_or(
            None,
            |output| Some(String::from_utf8(output.stdout)))
        .map_or(
            None, 
            |stdout| Some(stdout.unwrap_or(String::from(""))))
        .map_or(
            None,
            |text| Some(!text.contains("brew: command not found")));
}

pub fn get_installed_formulas() -> Option<Vec<u8>> {
    return get_installed(PackageType::Formulae);
}

pub fn get_installed_casks() -> Option<Vec<u8>> {
    return get_installed(PackageType::Cask);
}

fn get_installed(package_type: PackageType) -> Option<Vec<u8>> {
    let type_arg = match package_type {
        PackageType::Formulae => "--formulae",
        PackageType::Cask => "--cask",
    };
    return Command::new("brew")
        .arg("list")
        .arg(type_arg)
        .output()
        .map_or(
            None,
            |output| Some(output.stdout));
}

