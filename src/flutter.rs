use std::path::Path;

use colored::Colorize;

use crate::{FlutterMode, APPLICATION_NAME};

pub mod config;
pub mod format;
pub mod init;
pub mod template;
pub mod template_i18n;
pub mod template_navigation;
pub mod template_provider;
pub mod template_use_case;
pub mod template_widget;

#[derive(thiserror::Error, Debug)]
pub(crate) enum FlutterCommandError {
    #[error("OS Error")]
    IOError(#[from] std::io::Error),
    #[error("You must set env var: {variable_name:?}")]
    EnvVarNotFoundError { variable_name: String },
    #[error("Invalid file: {file_name:?}")]
    InvalidFileError { file_name: String },
    #[error("Unknown Error")]
    UnknownError,
}

pub(crate) fn command_handler(mode: FlutterMode) {
    let pubspec_yaml_path = Path::new("pubspec.yaml");
    if !pubspec_yaml_path.exists() {
        eprintln!("{}", "pubspec.yaml is not found.".red());
    }
    let flutter_config_file_name = format!("{}_flutter_config.yaml", APPLICATION_NAME);
    let res = match mode {
        FlutterMode::Init {
            overwrite_conflict_files,
            skip_conflict_config_files,
            config_only,
        } => super::flutter::init::init_flutter_app(
            &flutter_config_file_name,
            config_only,
            overwrite_conflict_files,
            skip_conflict_config_files,
        ),
        FlutterMode::Gen {
            overwrite_conflict_files,
            skip_conflict_files,
        } => super::flutter::template::generate_files(
            &flutter_config_file_name,
            overwrite_conflict_files,
            skip_conflict_files,
        ),
        FlutterMode::Format => super::flutter::format::format_import(),
    };
    match res {
        Ok(_) => {
            println!("{}", "completed!".green());
        }
        Err(err) => match err {
            FlutterCommandError::IOError(e) => {
                eprintln!("{}", format!("IO Error: {:?}", e).red());
            }
            FlutterCommandError::UnknownError => {
                eprintln!("{}", "Unknown Error".red());
            }
            FlutterCommandError::EnvVarNotFoundError { variable_name } => {
                eprintln!(
                    "{}",
                    format!("You must set env var: {}", variable_name).red()
                );
            }
            FlutterCommandError::InvalidFileError { file_name } => {
                eprintln!("{}", format!("Invalid file: {}", file_name).red());
            }
        },
    };
}
