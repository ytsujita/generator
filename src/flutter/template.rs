use colored::Colorize;

use super::config::Config;

use std::fs;

pub(crate) fn generate_files(
    config_file_name: &str,
    overwrite_all_conflict_files: bool,
    skip_all_conflict_files: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match fs::read_to_string(config_file_name) {
        Ok(config_str) => {
            let config: Config = serde_yaml::from_str(config_str.as_str()).unwrap();
            super::template_navigation::generate_route_path(
                &config.route_path_config,
                overwrite_all_conflict_files,
                skip_all_conflict_files,
            )?;
            super::template_use_case::generate_use_case(
                &config.use_case_config,
                overwrite_all_conflict_files,
                skip_all_conflict_files,
            )?;
            super::template_provider::generate_provider()?;
            super::template_widget::generate_widget(
                &config.route_path_config,
                overwrite_all_conflict_files,
                skip_all_conflict_files,
            )?;
            Ok(())
        }
        Err(e) => {
            println!("{:?}", e);
            println!("{}", "Failed to read config file.".red());
            Ok(())
        }
    }
}
