use askama::Template;
use change_case::snake_case;

use crate::utils::create_file;

use super::config::DomainConfig;

mod filters {
    use change_case::{pascal_case, snake_case};

    pub fn pascal<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(pascal_case(&(s.to_string())))
    }

    pub fn snake<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(snake_case(&(s.to_string())))
    }
}

#[derive(Template)]
#[template(
    path = "flutter/lib/domain/repository/repository.dart",
    escape = "none"
)]
pub(super) struct RepositoryTemplate<'a> {
    pub(super) name: &'a str,
}

#[derive(Template)]
#[template(
    path = "flutter/lib/infrastructure/repository/repository_impl.dart",
    escape = "none"
)]
pub(super) struct RepositoryImplTemplate<'a> {
    pub(super) name: &'a str,
}

#[derive(Template)]
#[template(path = "flutter/lib/domain/service/service.dart", escape = "none")]
pub(super) struct ServiceTemplate<'a> {
    pub(super) name: &'a str,
}

#[derive(Template)]
#[template(
    path = "flutter/lib/infrastructure/service/service_impl.dart",
    escape = "none"
)]
pub(super) struct ServiceImplTemplate<'a> {
    pub(super) name: &'a str,
}

pub(super) fn generate_domain_files(
    domain_config: &DomainConfig,
    overwrite_all_conflict_files: bool,
    skip_all_conflict_files: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let names = match &domain_config.repositories {
        Some(v) => v.iter().map(|s| s.name.clone()).collect(),
        None => vec![],
    };
    for name in names.iter() {
        let repo = RepositoryTemplate { name };
        let render_result = repo.render().unwrap();
        let file_name = format!("lib/domain/repository/{}_repository.dart", snake_case(name));
        create_file(
            &file_name,
            render_result.as_bytes(),
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
        let repo_impl = RepositoryImplTemplate { name };
        let render_result = repo_impl.render().unwrap();
        let file_name = format!(
            "lib/infrastructure/repository_impl/{}_repository_impl.dart",
            snake_case(name)
        );
        create_file(
            &file_name,
            render_result.as_bytes(),
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    // services
    let names = match &domain_config.services {
        Some(v) => v.iter().map(|s| s.name.clone()).collect(),
        None => vec![],
    };
    for name in names.iter() {
        let service = ServiceTemplate { name };
        let render_result = service.render().unwrap();
        let file_name = format!("lib/domain/service/{}_service.dart", snake_case(name));
        create_file(
            &file_name,
            render_result.as_bytes(),
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
        let service_impl = ServiceImplTemplate { name };
        let render_result = service_impl.render().unwrap();
        let file_name = format!(
            "lib/infrastructure/service_impl/{}_service_impl.dart",
            snake_case(name)
        );
        create_file(
            &file_name,
            render_result.as_bytes(),
            overwrite_all_conflict_files,
            skip_all_conflict_files,
        )?;
    }
    Ok(())
}
