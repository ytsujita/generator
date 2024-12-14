use colored::Colorize;

use crate::terraform::init::init_terraform_project;
use crate::TerraformMode;

pub mod config;
pub mod gen;
pub mod init;

#[derive(thiserror::Error, Debug)]
pub(crate) enum TerraformCommandError {
    #[error("OS Error")]
    IOError(#[from] std::io::Error),
    #[error("You must set env var: {variable_name:?}")]
    EnvVarNotFoundError { variable_name: String },
    #[error("Unknown Error")]
    UnknownError,
}

pub(crate) fn command_handler(mode: TerraformMode) {
    let res = match mode {
        TerraformMode::Init {
            overwrite_conflict_files,
            skip_conflict_config_files,
        } => init_terraform_project(overwrite_conflict_files, skip_conflict_config_files),
        TerraformMode::Gen => Ok(()),
    };
    match res {
        Ok(_) => {
            println!("{}", "completed!".green());
        }
        Err(ref err) => match err {
            TerraformCommandError::IOError(e) => {
                eprintln!("{}", format!("IO Error: {:?}", e).red());
            }
            TerraformCommandError::EnvVarNotFoundError { .. } => {
                eprintln!("{}", format!("{}", err).red());
            }
            TerraformCommandError::UnknownError => {
                eprintln!("{}", "Unknown Error".red());
            }
        },
    }
}
