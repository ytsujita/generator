use super::config::Config;

use std::fs;

pub(crate) fn generate_files(
    config_file_name: &str,
    overwrite_all_conflict_files: bool,
    skip_all_conflict_files: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(config_file_name)?;
    let config: Config = serde_yaml::from_str(config_str.as_str())?;
    if let Some(route_path_config) = &config.route_path_config {
        super::template_navigation::generate_navigation(
            &config.application_name,
            route_path_config,
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    if let Some(application_config) = &config.application_config {
        super::template_use_case::generate_use_case(
            &config.application_name,
            application_config,
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    if let Some(riverpod_config) = &config.riverpod_config {
        super::template_provider::generate_providers(
            riverpod_config,
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    if let Some(route_path_config) = &config.route_path_config {
        super::template_widget::generate_widget(
            &config.application_name,
            route_path_config,
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    if let Some(route_path_config) = &config.route_path_config {
        super::template_i18n::generate_i18n(
            route_path_config,
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    super::format::format_import()?;
    Ok(())
}
