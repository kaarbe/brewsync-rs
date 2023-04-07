use crate::file_maker::FileMaker;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Write;
use std::process::ExitCode;

mod file_maker;
mod homebrew;
mod package_type;

#[derive(Parser)]
#[command(
  author = "kaarbe",
  about = "Backups names of all packages installed via Homebrew",
  name = "brewsync",
  version
)]
struct Args {
  #[command(subcommand)]
  subcommand: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
  /// Configure Brewsync with values other than default
  Config,
}

fn main() -> ExitCode {
  return match Args::parse().subcommand {
    Some(subcommand) => handle_subcommand(subcommand),
    None => handle_main_command(),
  };
}

fn handle_subcommand(command: Command) -> ExitCode {
  match command {
    Command::Config => {
      let file_maker: FileMaker;
      if let Some(maker) = FileMaker::new() {
        file_maker = maker;
      } else {
        eprint!("Failure: Unable to read home directory path");
        return ExitCode::FAILURE;
      }

      if file_maker.make_config_dir().is_err() {
        eprint!("Failure: Unable to create backup directory");
        return ExitCode::FAILURE;
      }
      return ExitCode::SUCCESS;
    },
  };
}

fn handle_main_command() -> ExitCode {
  match homebrew::is_installed() {
    None => {
      eprint!("Failure: Unable to verify Homebrew installation");
      return ExitCode::FAILURE;
    }
    Some(result) => {
      if result == false {
        eprint!("Failure: Homebrew not installed");
        return ExitCode::FAILURE;
      }
    }
  };

  let file_maker: FileMaker;
  if let Some(maker) = FileMaker::new() {
    file_maker = maker;
  } else {
    eprint!("Failure: Unable to read home directory path");
    return ExitCode::FAILURE;
  }

  if file_maker.make_backup_dir().is_err() {
    eprint!("Failure: Unable to create backup directory");
    return ExitCode::FAILURE;
  }

  let mut formulas_file: File;
  if let Some(file) = file_maker.make_for_formulas() {
    formulas_file = file;
  } else {
    eprint!("Failure: Unable to create formulas file");
    return ExitCode::FAILURE;
  }

  let mut casks_file: File;
  if let Some(file) = file_maker.make_for_casks() {
    casks_file = file;
  } else {
    eprint!("Failure: Unable to create casks file");
    return ExitCode::FAILURE;
  }

  let casks_installed: Vec<u8>;
  if let Some(casks) = homebrew::get_installed_casks() {
    casks_installed = casks;
  } else {
    eprint!("Failure: Unable to read installed casks");
    return ExitCode::FAILURE;
  }

  let formulas_installed: Vec<u8>;
  if let Some(formulas) = homebrew::get_installed_formulas() {
    formulas_installed = formulas;
  } else {
    eprint!("Failure: Unable to read installed formulas");
    return ExitCode::FAILURE;
  }

  if casks_file.write_all(casks_installed.as_slice()).is_err() {
    eprint!("Failure: Unable to write to file");
    return ExitCode::FAILURE;
  }

  if formulas_file
    .write_all(formulas_installed.as_slice())
    .is_err()
  {
    eprint!("Failure: Unable to write to file");
    return ExitCode::FAILURE;
  }
  return ExitCode::SUCCESS;
}
