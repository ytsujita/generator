use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::config::UseCaseConfig;
use crate::utils::{create_dir, input_yes};
use askama::Template;
use change_case::lower_case;

mod filters {
    use change_case::camel_case;

    pub fn camel<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(camel_case(&(s.to_string())))
    }
}

struct UseCaseExceptionTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "application/use_case/use_case.dart", escape = "none")]
struct UseCaseTemplate<'a> {
    name: &'a str,
    return_type: &'a str,
    is_future_call: bool,
    exceptions: &'a Vec<&'a UseCaseExceptionTemplate<'a>>,
}

pub(super) fn generate_use_case(
    config: UseCaseConfig,
    delete_all_conflict_file: bool,
    ignore_all_conflict_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for use_case in &config.use_cases {
        let mut exceptions: Vec<UseCaseExceptionTemplate> = Vec::new();
        for exception in &use_case.exceptions {
            exceptions.push(UseCaseExceptionTemplate {
                name: &exception.name,
            });
        }
        let use_case_template = UseCaseTemplate {
            name: &use_case.name,
            return_type: "SampleReturnType",
            is_future_call: false,
            exceptions: &exceptions.iter().collect(),
        };
        let render_result = use_case_template.render().unwrap();
        let file_name = &format!(
            "lib/application/use_case/{}.dart",
            lower_case(&use_case.name)
        );
        let use_case_file_path = Path::new(&file_name);
        let parent_dir_path = use_case_file_path.parent();
        if let Some(v) = parent_dir_path {
            if v.is_dir() {
                let path_str = v.to_str();
                if let Some(path) = path_str {
                    create_dir(path).unwrap()
                }
            } else {
                println!("Invalid path: {}", &file_name);
            }
        };

        if !use_case_file_path.is_file() {
            if ignore_all_conflict_file {
                continue;
            }
            if delete_all_conflict_file {
                let mut file = File::create(file_name)?;
                file.write_all(render_result.as_bytes())?;
                file.flush()?;
                continue;
            }
            if input_yes(&format!(
                "found file: {}, Do you want to overwrite it? (y/N)",
                use_case.name,
            )) {
                let mut file = File::create(file_name)?;
                file.write_all(render_result.as_bytes())?;
                file.flush()?;
            } else {
                continue;
            }
        }
        let mut file = File::create(file_name)?;
        file.write_all(render_result.as_bytes())?;
        file.flush()?;
    }
    Ok(())
}
