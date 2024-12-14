use colored::Colorize;

pub mod init;

#[derive(thiserror::Error, Debug)]
pub(crate) enum ConfigCommandError {
    #[error("OS Error")]
    IOError(#[from] std::io::Error),
    #[error("You must set env var: {variable_name:?}")]
    EnvVarNotFoundError { variable_name: String },
    #[error("Unknown Error")]
    UnknownError,
}

pub(crate) fn command_handler(overwrite_conflict_file: bool) {
    match super::config::init::init_config_file(overwrite_conflict_file) {
        Ok(_) => {
            println!("{}", "completed".green());
        }
        Err(err) => {
            eprintln!("{}", format!("IO error: {:?}", err).red());
        }
    };
}
