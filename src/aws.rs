pub mod api_gateway_handler;
pub mod dynamodb;
pub mod region;

#[derive(thiserror::Error, Debug)]
pub(crate) enum AwsCommandError {
    #[error("OS Error")]
    IOError(#[from] std::io::Error),
}
