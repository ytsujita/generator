use askama::Template;
use change_case::snake_case;

use super::template_navigation::{get_route_from_config, get_shell_route_from_config};
use crate::flutter::config::{NavigationConfig, RoutePathConfig, ShellRoutePathConfig};
use crate::utils::create_file;

mod filters {
    use change_case::{camel_case, pascal_case};

    pub fn pascal<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(pascal_case(&(s.to_string())))
    }

    pub fn camel<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(camel_case(&(s.to_string())))
    }
}

#[derive(Template)]
#[template(path = "flutter/lib/widget/page/widget.dart", escape = "none")]
pub(super) struct RouteWidgetTemplate<'a> {
    pub(super) widget_name: &'a str,
}

#[derive(Template)]
#[template(path = "flutter/lib/widget/page/shell_widget.dart", escape = "none")]
pub(super) struct ShellRouteWidgetTemplate<'a> {
    pub(super) shell_name: &'a str,
}

pub(super) fn generate_widget(
    route_path_config: &NavigationConfig,
    overwrite_all_conflict_files: bool,
    skip_all_conflict_files: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut route_paths: Vec<&RoutePathConfig> = vec![];
    let mut shell_paths: Vec<&ShellRoutePathConfig> = vec![];
    for path in route_path_config.route_paths.iter() {
        get_route_from_config(path, &mut route_paths);
    }
    for path in route_path_config.route_paths.iter() {
        get_shell_route_from_config(path, &mut shell_paths);
    }
    for route_path in route_paths {
        let template = RouteWidgetTemplate {
            widget_name: &route_path.name,
        };
        let render_result = template.render().unwrap();
        let file_name = match &route_path.dir_name {
            Some(val) => {
                format!(
                    "lib/widget/page/{}/{}.dart",
                    snake_case(val),
                    snake_case(&route_path.name),
                )
            }
            None => {
                format!("lib/widget/page/{}.dart", snake_case(&route_path.name),)
            }
        };
        create_file(
            file_name.as_str(),
            render_result.as_bytes(),
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    for shell_path in shell_paths {
        let template = ShellRouteWidgetTemplate {
            shell_name: &shell_path.name,
        };
        let render_result = template.render().unwrap();
        let file_name = match &shell_path.dir_name {
            Some(val) => {
                format!(
                    "lib/widget/page/{}/{}.dart",
                    snake_case(val),
                    snake_case(&shell_path.name)
                )
            }
            None => {
                format!("lib/widget/page/{}.dart", snake_case(&shell_path.name))
            }
        };
        create_file(
            file_name.as_str(),
            render_result.as_bytes(),
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    Ok(())
}
