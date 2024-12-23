use crate::flutter::config::{ApplicationConfig, UseCaseType};
use crate::utils::create_file;
use askama::Template;
use change_case::snake_case;

mod filters {
    use change_case::{camel_case, pascal_case};

    pub fn camel<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(camel_case(&(s.to_string())))
    }

    pub fn pascal<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(pascal_case(&(s.to_string())))
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
    application_name: &'a str,
    name: &'a str,
    exceptions: &'a Vec<&'a UseCaseExceptionTemplate>,
    use_case_type: &'a UseCaseType,
    file_name: &'a str,
}

#[derive(Template)]
#[template(
    path = "flutter/lib/application/command_use_case_impl/command_use_case_impl.dart",
    escape = "none"
)]
struct CommandUseCaseImplTemplate<'a> {
    application_name: &'a str,
    name: &'a str,
    file_name: &'a str,
}

#[derive(Template)]
#[template(
    path = "flutter/lib/infrastructure/query_use_case_impl/query_use_case_impl.dart",
    escape = "none"
)]
struct QueryUseCaseImplTemplate<'a> {
    application_name: &'a str,
    name: &'a str,
    file_name: &'a str,
}

pub(super) fn generate_use_case(
    application_name: &str,
    config: &ApplicationConfig,
    delete_all_conflict_file: bool,
    ignore_all_conflict_file: bool,
) -> Result<(), std::io::Error> {
    for use_case in &config.use_cases {
        let mut exceptions: Vec<UseCaseExceptionTemplate> = Vec::new();
        for exception in &use_case.exceptions {
            exceptions.push(UseCaseExceptionTemplate {
                name: String::from(&exception.name),
            });
        }
        let dir_name: String = match &use_case.dir_name {
            Some(val) => {
                format!("lib/application/use_case/{}", val)
            }
            None => String::from("lib/application/use_case"),
        };
        let file_name = &format!("{}/{}.dart", dir_name, snake_case(&use_case.name));
        let file_name_without_common: String = match &use_case.dir_name {
            Some(val) => format!("{}/{}.dart", val, snake_case(&use_case.name)),
            None => format!("{}.dart", snake_case(&use_case.name)),
        };
        let use_case_template = UseCaseTemplate {
            application_name,
            name: &use_case.name,
            exceptions: &exceptions.iter().collect(),
            use_case_type: &use_case.use_case_type,
            file_name: &file_name_without_common,
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
                        format!("lib/application/command_use_case_impl/{}", val)
                    }
                    None => String::from("lib/application/command_use_case_impl"),
                };
                let file_name = &format!("{}/{}.dart", dir_name, snake_case(&use_case.name));
                let command_use_case_impl_template = CommandUseCaseImplTemplate {
                    application_name,
                    name: &use_case.name,
                    file_name: &file_name_without_common,
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
                        format!("lib/infrastructure/query_use_case_impl/{}", val)
                    }
                    None => String::from("lib/infrastructure/query_use_case_impl"),
                };
                let file_name = &format!("{}/{}.dart", dir_name, snake_case(&use_case.name));
                let query_use_case_impl_template = QueryUseCaseImplTemplate {
                    application_name,
                    name: &use_case.name,
                    file_name: &file_name_without_common,
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
