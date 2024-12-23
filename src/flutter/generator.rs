mod template_i18n;
mod template_navigation;
mod template_use_case;
mod template_widget;

use colored::Colorize;

use super::{config::Config, FlutterCommandError};

use std::fs;

pub(crate) fn generate_template_files(
    config_file_name: &str,
    overwrite_all_conflict_files: bool,
    skip_all_conflict_files: bool,
) -> Result<(), FlutterCommandError> {
    let config_str = fs::read_to_string(config_file_name)?;
    let config: Config = match serde_yaml::from_str(config_str.as_str()) {
        Ok(v) => v,
        Err(_) => {
            return Err(FlutterCommandError::InvalidFile {
                file_name: config_file_name.to_owned(),
            })
        }
    };
    if let Some(route_path_config) = &config.route_path_config {
        template_navigation::generate_navigation(
            &config.application_name,
            route_path_config,
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    if let Some(application_config) = &config.application_config {
        template_use_case::generate_use_case(
            &config.application_name,
            application_config,
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    if let Some(route_path_config) = &config.route_path_config {
        template_widget::generate_widget(
            &config.application_name,
            route_path_config,
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    if let Some(route_path_config) = &config.route_path_config {
        template_i18n::generate_i18n(
            route_path_config,
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    super::format::format_import()?;

    println!(
        "{}",
        "npx @puppeteer/browsers install chromedriver@stable".blue()
    );
    Ok(())
}
