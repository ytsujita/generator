use colored::Colorize;

use crate::terraform::init::init_terraform_project;
use crate::TerraformMode;

pub mod config;
pub mod gen;
pub mod init;

#[derive(thiserror::Error, Debug)]
pub(crate) enum TerraformCommandError {
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
}

pub(crate) fn command_handler(mode: TerraformMode) {
    let res = match mode {
        TerraformMode::Init {
            overwrite_conflict_files,
            skip_conflict_config_files,
        } => init_terraform_project(overwrite_conflict_files, skip_conflict_config_files),
    };
    match res {
        Ok(_) => {
            println!("{}", "completed!".green());
        }
        Err(ref err) => match err {
            TerraformCommandError::IOError(e) => {
                eprintln!("{}", format!("IO Error: {:?}", e).red());
            }
        },
    }
}
