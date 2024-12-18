pub mod generate_aws_api_gateway_lambda_handler;

use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, path::Path};

use askama::Template;
use include_dir::{include_dir, Dir};
use strum::IntoEnumIterator;

use crate::aws::region::AwsRegion;
use crate::utils::create_file;

use dialoguer::{Input, Select};

use super::TerraformCommandError;

mod filters {
    use change_case::param_case;

    pub fn kebab<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(param_case(&(s.to_string())))
    }
}

static SRC_DIR: Dir = include_dir!("resources/terraform/");

#[derive(Template)]
#[template(path = "terraform/envs/provider.tf", escape = "none")]
struct ProviderTemplate<'a> {
    pub(self) project_name: &'a str,
    pub(self) env_name: &'a str,
    pub(self) region_name: &'a str,
}

pub(crate) fn init_terraform_project(
    overwrite_conflict_files: bool,
    skip_conflict_files: bool,
) -> Result<(), TerraformCommandError> {
    let project_name = input_project_name()?;
    let region_name = input_region_name()?;
    copy_dir_recursive(
        Path::new("./"),
        overwrite_conflict_files,
        skip_conflict_files,
    )?;
    for env in ["dev", "test", "prod"] {
        generate_provider_template(
            env,
            &project_name,
            &region_name,
            overwrite_conflict_files,
            skip_conflict_files,
        )?;
    }
    Ok(())
}

fn copy_dir_recursive(
    dst: &Path,
    overwrite_all: bool,
    ignore_conflict_config_file: bool,
) -> Result<(), std::io::Error> {
    if !dst.exists() {
        fs::create_dir(dst)?;
    }
    let glob = "**/*";
    for file in SRC_DIR.find(glob).unwrap() {
        let dst_path = dst.join(file.path());
        match file {
            include_dir::DirEntry::Dir(d) => {
                fs::create_dir_all(d.path().as_os_str().to_str().unwrap()).unwrap();
            }
            include_dir::DirEntry::File(f) => {
                let file_path = f.path().as_os_str().to_str().unwrap();
                let file_buf: PathBuf = PathBuf::from_str(file_path).unwrap();
                if let Some(parent) = file_buf.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)?;
                    }
                }
                let _ = create_file(
                    dst_path.as_os_str().to_str().unwrap(),
                    f.contents(),
                    overwrite_all,
                    ignore_conflict_config_file,
                );
            }
        }
    }
    Ok(())
}

fn generate_provider_template(
    env_name: &str,
    project_name: &str,
    region_name: &str,
    overwrite_all_conflict_files: bool,
    skip_all_conflict_files: bool,
) -> Result<(), std::io::Error> {
    let template = ProviderTemplate {
        env_name,
        project_name,
        region_name,
    };
    let render_res = template.render().unwrap();
    let file_name = format!("envs/{}/provider.tf", env_name,);
    create_file(
        &file_name,
        render_res.as_bytes(),
        overwrite_all_conflict_files,
        skip_all_conflict_files,
    )?;
    Ok(())
}

fn input_project_name() -> Result<String, TerraformCommandError> {
    let mut project_name: String = String::from("");
    while project_name.is_empty() {
        project_name = match Input::new().with_prompt("Project Name").interact_text() {
            Ok(val) => val,
            Err(err) => match err {
                dialoguer::Error::IO(e) => return Err(TerraformCommandError::IOError(e)),
            },
        }
    }
    Ok(project_name)
}

fn input_region_name() -> Result<String, TerraformCommandError> {
    let mut regions: Vec<String> = vec![];
    for region in AwsRegion::iter() {
        regions.push(format!("{}", region));
    }
    let default_index = AwsRegion::get_index(&AwsRegion::ApNortheast1).unwrap_or(0);
    let choice: usize = match Select::new()
        .with_prompt("Region Name")
        .items(&regions)
        .default(default_index)
        .interact()
    {
        Ok(val) => val,
        Err(err) => match err {
            dialoguer::Error::IO(e) => return Err(TerraformCommandError::IOError(e)),
        },
    };
    let region_name: &str = &regions[choice];
    Ok(region_name.to_owned())
}
