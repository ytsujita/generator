use std::path::Path;

use colored::Colorize;

use crate::{FlutterMode, APPLICATION_NAME};

pub mod config;
pub mod format;
pub mod init;
pub mod template;
pub mod template_i18n;
pub mod template_navigation;
pub mod template_use_case;
pub mod template_widget;

#[derive(thiserror::Error, Debug)]
pub(crate) enum FlutterCommandError {
    #[error("OS Error")]
    IO(#[from] std::io::Error),
    #[error("Invalid file: {file_name:?}")]
    InvalidFile { file_name: String },
    #[error("Execute external command error: {command_name:?}")]
    ExecuteExternalCommand { command_name: String },
}

pub(crate) fn command_handler(mode: FlutterMode) {
    let pubspec_yaml_path = Path::new("pubspec.yaml");
    if !pubspec_yaml_path.exists() {
        eprintln!("{}", "pubspec.yaml is not found.".red());
        return;
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
            FlutterCommandError::IO(e) => {
                eprintln!("{}", format!("IO Error: {:?}", e).red());
            }
            FlutterCommandError::InvalidFile { file_name } => {
                eprintln!("{}", format!("Invalid file: {}", file_name).red());
            }
            FlutterCommandError::ExecuteExternalCommand { command_name } => {
                eprintln!(
                    "{}",
                    format!("Execute external command error: {}", command_name).red()
                );
            }
        },
    };
}
