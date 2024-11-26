use super::super::config::Config;

use std::{
    fs::{self},
    path::PathBuf,
};

pub(crate) fn generate_files(
    config_file_name: &str,
    delete_all_conflict_file: bool,
    ignore_all_conflict_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if delete_all_conflict_file {
        let lib_dir_path = PathBuf::from("lib");
        if lib_dir_path.exists() {
            fs::remove_dir_all(&lib_dir_path)?;
        }
        fs::create_dir(&lib_dir_path)?;
    }
    let config_str = fs::read_to_string(config_file_name)?;
    let config: Config = serde_yaml::from_str(&config_str).unwrap();
    super::route_path_template::generate_route_path(
        config.route_path_config,
        delete_all_conflict_file,
        ignore_all_conflict_file,
    )?;
    super::use_case_template::generate_use_case(
        config.use_case_config,
        delete_all_conflict_file,
        ignore_all_conflict_file,
    )?;
    Ok(())
}
