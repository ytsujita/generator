use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::flutter::config::{RoutePath, RoutePathConfig, RouteType, ShellRoutePath};
use crate::utils::{create_dir, input_yes};
use askama::Template;

mod filters {
    use change_case::camel_case;

    pub fn camel<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(camel_case(&(s.to_string())))
    }
}

struct Route {
    name: String,
    uri: Option<String>,
    path_reg_exp: Option<String>,
}

struct ShellRoute {
    name: String,
    shell_index_enum_names: Vec<String>,
}

#[derive(Template)]
#[template(path = "navigation/route_path.dart", escape = "none")]
struct RoutePathTemplate<'a> {
    route_paths: &'a Vec<&'a Route>,
    shell_route_paths: &'a Vec<&'a ShellRoute>,
    default_route_path_name: &'a str,
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

pub(super) fn generate_route_path(
    route_path_config: RoutePathConfig,
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
        })
    }
    for shell_path in shell_paths {
        shell_template.push(ShellRoute {
            name: shell_path.name.clone(),
            shell_index_enum_names: shell_path.shell_index_enum_names.clone(),
        })
    }
    let route_path_template = RoutePathTemplate {
        route_paths: &route_template.iter().collect(),
        shell_route_paths: &shell_template.iter().collect(),
        default_route_path_name: &route_path_config.default_route_path_name,
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
    Ok(())
}
