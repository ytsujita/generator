use std::fmt;

#[derive(Debug, Clone)]
pub(crate) struct InvalidConfig {
    pub(crate) file_name: String,
}

impl fmt::Display for InvalidConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid config file: {}", self.file_name)
    }
}

impl std::error::Error for InvalidConfig {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EnvVarNotFoundError {
    pub(crate) env_name: String,
}

impl fmt::Display for EnvVarNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You must set environment var: {}", self.env_name)
    }
}

impl std::error::Error for EnvVarNotFoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
