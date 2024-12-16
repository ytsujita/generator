pub mod api_gateway_handler;
pub mod dynamodb;
pub mod region;

#[derive(thiserror::Error, Debug)]
pub(crate) enum AwsCommandError {
    #[error("OS Error")]
    IOError(#[from] std::io::Error),
    #[error("You must set env var: {variable_name:?}")]
    EnvVarNotFoundError { variable_name: String },
    #[error("Invalid file: {file_name:?}")]
    InvalidFileError { file_name: String },
    #[error("Unknown Error")]
    UnknownError,
}
