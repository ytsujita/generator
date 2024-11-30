use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::flutter::config::{RoutePath, RoutePathConfig, RouteType, ShellRoutePath};
use crate::utils::{create_dir, create_file, input_yes};
use askama::Template;

mod filters {
    use change_case::{camel_case, snake_case};

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
}

pub(super) struct ShellRoute {
    pub(super) name: String,
    pub(super) shell_index_enum_names: Vec<String>,
    pub(super) dir_name: Option<String>,
}

#[derive(Template)]
#[template(path = "flutter/lib/navigation/route_path.dart", escape = "none")]
pub(super) struct RoutePathTemplate<'a> {
    pub(super) route_paths: &'a Vec<&'a Route>,
    pub(super) shell_route_paths: &'a Vec<&'a ShellRoute>,
    pub(super) default_route_path_name: &'a str,
}

pub(super) fn get_route_from_config<'a>(node: &'a RouteType, leaves: &mut Vec<&'a RoutePath>) {
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

pub(super) fn get_shell_route_from_config<'a>(
    node: &'a RouteType,
    leaves: &mut Vec<&'a ShellRoutePath>,
) {
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

pub(super) fn generate_route_path(
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
    let mut route_template: Vec<Route> = vec![];
    let mut shell_template: Vec<ShellRoute> = vec![];
    for route_path in route_paths {
        route_template.push(Route {
            name: route_path.name.clone(),
            uri: route_path.uri.clone(),
            path_reg_exp: route_path.path_reg_exp.clone(),
            dir_name: route_path.dir_name.clone(),
        })
    }
    for shell_path in shell_paths {
        shell_template.push(ShellRoute {
            name: shell_path.name.clone(),
            shell_index_enum_names: shell_path.shell_index_enum_names.clone(),
            dir_name: shell_path.dir_name.clone(),
        })
    }
    let route_path_template = RoutePathTemplate {
        route_paths: &route_template.iter().collect(),
        shell_route_paths: &shell_template.iter().collect(),
        default_route_path_name: match &route_path_config.default_route_path_name {
            RouteType::RoutePath(r) => r.name.as_str(),
            RouteType::ShellRoute(r) => r.name.as_str(),
        },
    };
    create_dir("lib/navigation").unwrap();
    let render_result = route_path_template.render().unwrap();
    let file_name = "lib/navigation/route_path.dart";
    let route_path_file_path = Path::new(&file_name);
    let parent_dir_path = route_path_file_path.parent();
    if let Some(v) = parent_dir_path {
        if v.is_dir() {
            let path_str = v.to_str();
            if let Some(path) = path_str {
                create_dir(path).unwrap()
            }
        } else {
            println!("Invalid path: {:?}", v);
        }
    };
    if ignore_all_conflict_file {
        return Ok(());
    }
    if delete_all_conflict_file {
        let mut file = File::create(file_name)?;
        file.write_all(render_result.as_bytes())?;
        file.flush()?;
        return Ok(());
    }
    if !route_path_file_path.exists()
        || (route_path_file_path.exists()
            && input_yes(&format!(
                "found file: {}, Do you want to overwrite it?",
                file_name,
            )))
    {
        let mut file = File::create(file_name)?;
        file.write_all(render_result.as_bytes())?;
        file.flush()?;
    }
    generate_navigation_state(
        route_path_config,
        delete_all_conflict_file,
        ignore_all_conflict_file,
    )?;
    generate_router_delegate(
        route_path_config,
        delete_all_conflict_file,
        ignore_all_conflict_file,
    )?;
    generate_route_information_parser(
        route_path_config,
        delete_all_conflict_file,
        ignore_all_conflict_file,
    )?;
    generate_navigation_state_provider(
        route_path_config,
        delete_all_conflict_file,
        ignore_all_conflict_file,
    )?;
    Ok(())
}

const NAVIGATION_STATE_PROVIDER_BYTES: &[u8] =
    include_bytes!("../../templates/flutter/lib/navigation/navigation_state_provider.dart");
const ROUTE_INFORMATION: &[u8] =
    include_bytes!("../../templates/flutter/lib/navigation/main_route_information.dart");
const ROUTER_DELEGATE: &[u8] =
    include_bytes!("../../templates/flutter/lib/navigation/main_router_delegate.dart");
const NAVIGATION_STATE: &[u8] =
    include_bytes!("../../templates/flutter/lib/navigation/navigation_state.dart");

pub(super) fn generate_navigation_state(
    route_path_config: &RoutePathConfig,
    delete_all_conflict_file: bool,
    ignore_all_conflict_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = route_path_config;
    create_file(
        "lib/navigation/navigation_state.dart",
        NAVIGATION_STATE,
        delete_all_conflict_file,
        ignore_all_conflict_file,
    )?;
    Ok(())
}

pub(super) fn generate_route_information_parser(
    route_path_config: &RoutePathConfig,
    delete_all_conflict_file: bool,
    ignore_all_conflict_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = route_path_config;
    create_file(
        "lib/navigation/main_route_information.dart",
        ROUTE_INFORMATION,
        delete_all_conflict_file,
        ignore_all_conflict_file,
    )?;
    Ok(())
}

pub(super) fn generate_router_delegate(
    route_path_config: &RoutePathConfig,
    delete_all_conflict_file: bool,
    ignore_all_conflict_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = route_path_config;
    create_file(
        "lib/navigation/main_router_delegate.dart",
        ROUTER_DELEGATE,
        delete_all_conflict_file,
        ignore_all_conflict_file,
    )?;
    Ok(())
}

pub(super) fn generate_navigation_state_provider(
    route_path_config: &RoutePathConfig,
    delete_all_conflict_file: bool,
    ignore_all_conflict_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = route_path_config;
    create_file(
        "lib/navigation/navigation_state_provider.dart",
        NAVIGATION_STATE_PROVIDER_BYTES,
        delete_all_conflict_file,
        ignore_all_conflict_file,
    )?;
    Ok(())
}
