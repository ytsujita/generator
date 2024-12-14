use serde::{Deserialize, Serialize};

use crate::utils::create_file;

use super::super::APPLICATION_NAME;
use super::ConfigCommandError;
use std::{env, path::Path};

#[derive(Deserialize, Serialize)]
pub(crate) struct CliConfig {
    pub(crate) git_type: GitType,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GitType {
    Https,
    Ssh,
}

pub(crate) fn init_config_file(overwrite_conflict: bool) -> Result<(), ConfigCommandError> {
    let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") else {
        return Err(ConfigCommandError::EnvVarNotFoundError {
            variable_name: "XDG_CONFIG_HOME".to_string(),
        });
    };
    let path = Path::new(&xdg_config_home);
    let path_buf = path.join(APPLICATION_NAME);
    let config_file_name = "config.yaml";
    let path_buf = path_buf.join(config_file_name);
    let config = CliConfig {
        git_type: GitType::Ssh,
    };
    let config_str = serde_yaml::to_string(&config).unwrap();

    create_file(
        path_buf.to_str().unwrap(),
        config_str.as_bytes(),
        overwrite_conflict,
        false,
    )?;
    Ok(())
}
