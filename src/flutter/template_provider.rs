use crate::flutter::config::{RoutePath, RoutePathConfig, RouteType, ShellRoutePath};
use crate::utils::{create_dir, create_file, input_yes};

pub(super) fn generate_provider() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

struct Route {
    pub(super) name: String,
    pub(super) uri: Option<String>,
    pub(super) path_reg_exp: Option<String>,
    pub(super) dir_name: Option<String>,
}

struct ShellRoute {
    pub(super) name: String,
    pub(super) shell_index_enum_names: Vec<String>,
    pub(super) dir_name: Option<String>,
}

pub(super) fn generate_providers(
    route_path_config: &RoutePathConfig,
    delete_all_conflict_file: bool,
    ignore_all_conflict_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut route_paths: Vec<&RoutePath> = vec![];
    let mut shell_paths: Vec<&ShellRoutePath> = vec![];
    for path in route_path_config.route_paths.iter() {
        get_route_from_config(path, &mut route_paths);
    }
    for path in route_path_config.route_paths.iter() {
        get_shell_route_from_config(path, &mut shell_paths);
    }
    for route_path in route_paths {
        let dir_name = route_path.dir_name.clone();
    }
    for shell_path in shell_paths {
        let dir_name = shell_path.dir_name.clone();
    }
    Ok(())
}

fn get_route_from_config<'a>(node: &'a RouteType, leaves: &mut Vec<&'a RoutePath>) {
    match node {
        RouteType::RoutePath(r) => {
            leaves.push(r);
            if let Some(children) = &r.children {
                for child in children {
                    get_route_from_config(child, leaves);
                }
            }
        }
        RouteType::ShellRoute(s) => {
            for shell in s.shells.iter() {
                get_route_from_config(shell, leaves);
            }
        }
    }
}

fn get_shell_route_from_config<'a>(node: &'a RouteType, leaves: &mut Vec<&'a ShellRoutePath>) {
    match node {
        RouteType::RoutePath(r) => {
            if let Some(children) = &r.children {
                for child in children {
                    get_shell_route_from_config(child, leaves);
                }
            }
        }
        RouteType::ShellRoute(s) => {
            leaves.push(s);
            for shell in s.shells.iter() {
                get_shell_route_from_config(shell, leaves);
            }
        }
    }
}
