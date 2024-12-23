use change_case::snake_case;

use crate::utils::create_dir;

use super::super::config::{NavigationConfig, RoutePathConfig, ShellRoutePathConfig};
use super::template_navigation::{get_route_from_config, get_shell_route_from_config};

pub(crate) fn generate_i18n(
    route_path_config: &NavigationConfig,
    _overwrite_all_conflict_files: bool,
    _skip_all_conflict_files: bool,
) -> Result<(), std::io::Error> {
    let mut route_paths: Vec<&RoutePathConfig> = vec![];
    let mut shell_paths: Vec<&ShellRoutePathConfig> = vec![];
    for path in route_path_config.route_paths.iter() {
        get_route_from_config(path, &mut route_paths);
    }
    for path in route_path_config.route_paths.iter() {
        get_shell_route_from_config(path, &mut shell_paths);
    }
    for route_path in route_paths {
        if let Some(val) = &route_path.dir_name {
            create_dir(&format!("lib/i18n/{}", snake_case(val)))?
        };
    }
    for shell_path in shell_paths {
        if let Some(val) = &shell_path.dir_name {
            create_dir(&format!("lib/i18n/{}", snake_case(val)))?
        };
    }
    Ok(())
}
