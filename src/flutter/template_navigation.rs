use crate::flutter::config::{
    NavigationConfig, RouteConfigType, RoutePathConfig, ShellRoutePathConfig,
};
use crate::utils::create_file;
use askama::Template;

use super::config::{DartField, ShellIndexType};

mod filters {
    use change_case::{camel_case, pascal_case, snake_case};

    pub fn pascal<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(pascal_case(&(s.to_string())))
    }

    pub fn camel<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(camel_case(&(s.to_string())))
    }

    pub fn snake<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(snake_case(&(s.to_string())))
    }
}

pub(super) struct Route {
    pub(super) name: String,
    pub(super) uri: Option<String>,
    pub(super) path_reg_exp: Option<String>,
    pub(super) dir_name: Option<String>,
    pub(super) fields: Option<Vec<DartField>>,
}

pub(super) struct ShellRoute {
    pub(super) name: String,
    pub(super) shell_index: ShellIndexType,
    pub(super) dir_name: Option<String>,
}

#[derive(Template)]
#[template(path = "flutter/lib/navigation/route_path.dart", escape = "none")]
struct RoutePathTemplate<'a> {
    pub(super) application_name: &'a str,
    pub(super) route_paths: &'a Vec<&'a Route>,
    pub(super) shell_route_paths: &'a Vec<&'a ShellRoute>,
    pub(super) default_route_path_name: &'a str,
}

#[derive(Template)]
#[template(
    path = "flutter/lib/navigation/main_route_information.dart",
    escape = "none"
)]
struct RouteInformationTemplate {}

#[derive(Template)]
#[template(
    path = "flutter/lib/navigation/main_router_delegate.dart",
    escape = "none"
)]
struct RouterDelegateTemplate {}

#[derive(Template)]
#[template(
    path = "flutter/lib/navigation/navigation_state_provider.dart",
    escape = "none"
)]
struct NavigationStateProviderTemplate {}

#[derive(Template)]
#[template(path = "flutter/lib/navigation/navigation_state.dart", escape = "none")]
struct NavigationStateTemplate {
    route_path_names: Vec<String>,
    default_route_path_name: String,
}

pub(super) fn generate_navigation(
    application_name: &str,
    route_path_config: &NavigationConfig,
    overwrite_all_conflict_files: bool,
    skip_all_conflict_files: bool,
) -> Result<(), std::io::Error> {
    let mut route_paths: Vec<&RoutePathConfig> = vec![];
    let mut shell_paths: Vec<&ShellRoutePathConfig> = vec![];
    for path in route_path_config.route_paths.iter() {
        get_route_from_config(path, &mut route_paths);
    }
    for path in route_path_config.route_paths.iter() {
        get_shell_route_from_config(path, &mut shell_paths);
    }
    let mut route_template: Vec<Route> = vec![];
    let mut shell_template: Vec<ShellRoute> = vec![];
    for route_path in route_paths {
        route_template.push(Route {
            name: route_path.name.clone(),
            uri: route_path.uri.clone(),
            path_reg_exp: route_path.path_reg_exp.clone(),
            dir_name: route_path.dir_name.clone(),
            fields: route_path.fields.clone(),
        })
    }
    for shell_path in shell_paths {
        shell_template.push(ShellRoute {
            name: shell_path.name.clone(),
            shell_index: shell_path.shell_index.clone(),
            dir_name: shell_path.dir_name.clone(),
        })
    }
    generate_route_path(
        application_name,
        route_path_config,
        overwrite_all_conflict_files,
        skip_all_conflict_files,
        &route_template,
        &shell_template,
    )?;
    generate_route_information_parser(
        route_path_config,
        overwrite_all_conflict_files,
        skip_all_conflict_files,
    )?;
    generate_router_delegate(
        route_path_config,
        overwrite_all_conflict_files,
        skip_all_conflict_files,
    )?;
    generate_navigation_state(
        route_path_config,
        overwrite_all_conflict_files,
        skip_all_conflict_files,
    )?;
    generate_navigation_state_provider(
        route_path_config,
        overwrite_all_conflict_files,
        skip_all_conflict_files,
    )?;
    Ok(())
}

fn generate_route_path(
    application_name: &str,
    route_path_config: &NavigationConfig,
    overwrite_conflict_file: bool,
    skip_conflict_file: bool,
    route_template: &[Route],
    shell_template: &[ShellRoute],
) -> Result<(), std::io::Error> {
    let route_path_template = RoutePathTemplate {
        application_name,
        route_paths: &route_template.iter().collect(),
        shell_route_paths: &shell_template.iter().collect(),
        default_route_path_name: match &route_path_config.default_route_path {
            RouteConfigType::RoutePath(r) => r.name.as_str(),
            RouteConfigType::ShellRoute(r) => r.name.as_str(),
        },
    };
    let render_result = route_path_template.render().unwrap();
    let file_name = "lib/navigation/route_path.dart";
    create_file(
        file_name,
        render_result.as_bytes(),
        overwrite_conflict_file,
        skip_conflict_file,
    )?;
    Ok(())
}

fn generate_navigation_state(
    route_path_config: &NavigationConfig,
    overwrite_conflict_file: bool,
    skip_conflict_file: bool,
) -> Result<(), std::io::Error> {
    let mut route_paths: Vec<&RoutePathConfig> = vec![];
    let mut shell_paths: Vec<&ShellRoutePathConfig> = vec![];
    for path in route_path_config.route_paths.iter() {
        get_route_from_config(path, &mut route_paths);
    }
    for path in route_path_config.route_paths.iter() {
        get_shell_route_from_config(path, &mut shell_paths);
    }
    let mut route_path_names: Vec<String> = vec![];
    let default_route_path_name: String = match &route_path_config.default_route_path {
        RouteConfigType::RoutePath(r) => r.name.clone(),
        RouteConfigType::ShellRoute(r) => r.name.clone(),
    };
    for route_path in route_paths {
        route_path_names.push(route_path.name.clone());
    }
    let template = NavigationStateTemplate {
        route_path_names,
        default_route_path_name,
    };
    let file_name = "lib/navigation/navigation_state.dart";
    create_file(
        file_name,
        template.render().unwrap().as_bytes(),
        overwrite_conflict_file,
        skip_conflict_file,
    )?;
    Ok(())
}

fn generate_route_information_parser(
    route_path_config: &NavigationConfig,
    overwrite_conflict_file: bool,
    skip_conflict_file: bool,
) -> Result<(), std::io::Error> {
    let _ = route_path_config;
    let template = RouteInformationTemplate {};
    let file_name = "lib/navigation/main_route_information.dart";
    create_file(
        file_name,
        template.render().unwrap().as_bytes(),
        overwrite_conflict_file,
        skip_conflict_file,
    )?;
    Ok(())
}

fn generate_router_delegate(
    route_path_config: &NavigationConfig,
    overwrite_conflict_file: bool,
    skip_conflict_file: bool,
) -> Result<(), std::io::Error> {
    let _ = route_path_config;
    let template = RouterDelegateTemplate {};
    let file_name = "lib/navigation/main_router_delegate.dart";
    create_file(
        file_name,
        template.render().unwrap().as_bytes(),
        overwrite_conflict_file,
        skip_conflict_file,
    )?;
    Ok(())
}

fn generate_navigation_state_provider(
    route_path_config: &NavigationConfig,
    overwrite_conflict_file: bool,
    skip_conflict_file: bool,
) -> Result<(), std::io::Error> {
    let _ = route_path_config;
    let template = NavigationStateProviderTemplate {};
    let file_name = "lib/navigation/navigation_state_provider.dart";
    create_file(
        file_name,
        template.render().unwrap().as_bytes(),
        overwrite_conflict_file,
        skip_conflict_file,
    )?;
    Ok(())
}

pub(super) fn get_route_from_config<'a>(
    node: &'a RouteConfigType,
    leaves: &mut Vec<&'a RoutePathConfig>,
) {
    match node {
        RouteConfigType::RoutePath(r) => {
            leaves.push(r);
            if let Some(children) = &r.children {
                for child in children {
                    get_route_from_config(child, leaves);
                }
            }
        }
        RouteConfigType::ShellRoute(s) => {
            for shell in s.shells.values() {
                get_route_from_config(shell, leaves);
            }
        }
    }
}

pub(super) fn get_shell_route_from_config<'a>(
    node: &'a RouteConfigType,
    leaves: &mut Vec<&'a ShellRoutePathConfig>,
) {
    match node {
        RouteConfigType::RoutePath(r) => {
            if let Some(children) = &r.children {
                for child in children {
                    get_shell_route_from_config(child, leaves);
                }
            }
        }
        RouteConfigType::ShellRoute(s) => {
            leaves.push(s);
            for shell in s.shells.values() {
                get_shell_route_from_config(shell, leaves);
            }
        }
    }
}
