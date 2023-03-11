use std::process::Command;
use std::process::Output;

use crate::package_type::PackageType;

pub fn is_installed() -> Option<bool> {
    return Command::new("brew")
        .arg("--version")
        .output()
        .map_or(
            None,
            |output| output_to_bool(output)
        );
}

fn output_to_bool(output: Output) -> Option<bool> {
    return String::from_utf8(output.stdout.clone())
        .map_or(
            None,
            |content| Some(!content.contains("brew: command not found"))
        );
}

pub fn get_installed_formulas() -> Option<Vec<u8>> {
    return get_installed(PackageType::Formulae);
}

pub fn get_installed_casks() -> Option<Vec<u8>> {
    return get_installed(PackageType::Cask);
}

fn get_installed(ptype: PackageType) -> Option<Vec<u8>> {
    let type_arg = match ptype {
        PackageType::Formulae => "--formulae",
        PackageType::Cask => "--cask",
    };
    return Command::new("brew")
        .arg("list")
        .arg(type_arg)
        .output()
        .map_or(
            None,
            |output| Some(output.stdout)
        );
}

