use crate::flutter::config::{UseCaseConfig, UseCaseType};
use crate::utils::create_file;
use askama::Template;
use change_case::snake_case;

mod filters {
    use change_case::camel_case;

    pub fn camel<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(camel_case(&(s.to_string())))
    }
}

struct UseCaseExceptionTemplate {
    name: String,
}

#[derive(Template)]
#[template(
    path = "flutter/lib/application/use_case/use_case.dart",
    escape = "none"
)]
struct UseCaseTemplate<'a> {
    name: &'a str,
    return_type: &'a str,
    is_future_call: bool,
    exceptions: &'a Vec<&'a UseCaseExceptionTemplate>,
    use_case_type: &'a UseCaseType,
    file_nest_size: i32,
    file_name: &'a str,
}

#[derive(Template)]
#[template(
    path = "flutter/lib/application/command_use_case_impl/command_use_case_impl.dart",
    escape = "none"
)]
struct CommandUseCaseImplTemplate<'a> {
    name: &'a str,
    return_type: &'a str,
    is_future_call: bool,
    file_nest_size: i32,
    file_name: &'a str,
}

#[derive(Template)]
#[template(
    path = "flutter/lib/infrastructure/query_use_case_impl/query_use_case_impl.dart",
    escape = "none"
)]
struct QueryUseCaseImplTemplate<'a> {
    name: &'a str,
    return_type: &'a str,
    is_future_call: bool,
    file_nest_size: i32,
    file_name: &'a str,
}

pub(super) fn generate_use_case(
    config: &UseCaseConfig,
    delete_all_conflict_file: bool,
    ignore_all_conflict_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for use_case in &config.use_cases {
        let mut exceptions: Vec<UseCaseExceptionTemplate> = Vec::new();
        for exception in &use_case.exceptions {
            exceptions.push(UseCaseExceptionTemplate {
                name: String::from(&exception.name),
            });
        }
        let dir_name: String = match &use_case.dir_name {
            Some(val) => {
                let res: String = val
                    .split("/")
                    .map(snake_case)
                    .collect::<Vec<String>>()
                    .join("/");
                format!("lib/application/use_case/{}", res)
            }
            None => String::from("lib/application/use_case"),
        };
        let file_name = &format!("{}/{}.dart", dir_name, snake_case(&use_case.name));
        let file_name_without_common: String = match &use_case.dir_name {
            Some(val) => format!("{}/{}.dart", val, snake_case(&use_case.name)),
            None => format!("{}.dart", snake_case(&use_case.name)),
        };
        let nest_size: i32 = std::cmp::max(
            (dir_name.chars().filter(|&c| c == '/').count() as i32) - 2,
            0,
        );
        let use_case_template = UseCaseTemplate {
            name: &use_case.name,
            return_type: "SampleReturnType",
            is_future_call: false,
            exceptions: &exceptions.iter().collect(),
            use_case_type: &use_case.use_case_type,
            file_name: &file_name_without_common,
            file_nest_size: nest_size,
        };
        let render_result = use_case_template.render().unwrap();
        create_file(
            file_name,
            render_result.as_bytes(),
            delete_all_conflict_file,
            ignore_all_conflict_file,
        )?;
        match use_case.use_case_type {
            UseCaseType::Command => {
                let dir_name: String = match &use_case.dir_name {
                    Some(val) => {
                        let res: String = val
                            .split("/")
                            .map(snake_case)
                            .collect::<Vec<String>>()
                            .join("/");
                        format!("lib/application/command_use_case_impl/{}", res,)
                    }
                    None => String::from("lib/application/command_use_case_impl"),
                };
                let file_name = &format!("{}/{}.dart", dir_name, snake_case(&use_case.name));
                let command_use_case_impl_template = CommandUseCaseImplTemplate {
                    name: &use_case.name,
                    return_type: "SampleReturnType",
                    is_future_call: false,
                    file_name: &file_name_without_common,
                    file_nest_size: nest_size,
                };
                let render_result = command_use_case_impl_template.render().unwrap();
                create_file(
                    file_name,
                    render_result.as_bytes(),
                    delete_all_conflict_file,
                    ignore_all_conflict_file,
                )?;
            }
            UseCaseType::Query => {
                let dir_name: String = match &use_case.dir_name {
                    Some(val) => {
                        let res: String = val
                            .split("/")
                            .map(snake_case)
                            .collect::<Vec<String>>()
                            .join("/");
                        format!("lib/infrastructure/query_use_case_impl/{}", res)
                    }
                    None => String::from("lib/infrastructure/query_use_case_impl"),
                };
                let file_name = &format!("{}/{}.dart", dir_name, snake_case(&use_case.name));
                let query_use_case_impl_template = QueryUseCaseImplTemplate {
                    name: &use_case.name,
                    return_type: "SampleReturnType",
                    is_future_call: false,
                    file_name: &file_name_without_common,
                    file_nest_size: nest_size,
                };
                let render_result = query_use_case_impl_template.render().unwrap();
                create_file(
                    file_name,
                    render_result.as_bytes(),
                    delete_all_conflict_file,
                    ignore_all_conflict_file,
                )?;
            }
        }
    }
    Ok(())
}
