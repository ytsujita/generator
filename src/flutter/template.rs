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
            super::template_navigation::generate_navigation(
                &config.route_path_config,
                overwrite_all_conflict_files,
                skip_all_conflict_files,
            )?;
            super::template_use_case::generate_use_case(
                &config.application_name,
                &config.application_config,
                overwrite_all_conflict_files,
                skip_all_conflict_files,
            )?;
            super::template_provider::generate_providers(
                &config.riverpod_config,
                overwrite_all_conflict_files,
                skip_all_conflict_files,
            )?;
            super::template_widget::generate_widget(
                &config.route_path_config,
                overwrite_all_conflict_files,
                skip_all_conflict_files,
            )?;
            super::template_domain::generate_domain_files(
                &config.application_name,
                &config.domain_config,
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
