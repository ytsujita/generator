use askama::Template;
use change_case::snake_case;

use crate::utils::create_file;

use super::config::{DartType, ProviderType, RiverpodConfig};

mod filters {
    use change_case::camel_case;

    pub fn camel<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        Ok(camel_case(&(s.to_string())))
    }
}

#[derive(Template)]
#[template(path = "flutter/lib/provider/notifier/provider.dart", escape = "none")]
pub(super) struct ProviderTemplate<'a> {
    pub(super) name: String,
    pub(super) provider_type: &'a ProviderType,
    pub(super) auto_dispose: bool,
    pub(super) family_type: &'a Option<DartType>,
    pub(super) state_name: String,
    pub(super) state_path: &'a Option<String>,
}

pub(super) fn generate_providers(
    riverpod_config: &RiverpodConfig,
    overwrite_all_conflict_files: bool,
    skip_all_conflict_files: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(vec) = &riverpod_config.providers {
        for provider_config in vec {
            let template = ProviderTemplate {
                name: provider_config.name.clone(),
                provider_type: &provider_config.provider_type,
                auto_dispose: provider_config.auto_dispose,
                family_type: &provider_config.family_type,
                state_name: format!("{}", &provider_config.state),
                state_path: match &provider_config.state {
                    super::config::DartType::NewClass(n) => &Some(n.name.clone()),
                    super::config::DartType::RefClass(r) => &Some(r.name.clone()),
                    _ => &None,
                },
            };
            let render_res = template.render().unwrap();
            let file_name = match &provider_config.dir_name {
                Some(v) => format!(
                    "lib/provider/notifier/{}/{}_provider.dart",
                    v,
                    &snake_case(&provider_config.name),
                ),
                None => format!(
                    "lib/provider/notifier/{}_provider.dart",
                    &snake_case(&provider_config.name),
                ),
            };
            create_file(
                &file_name,
                render_res.as_bytes(),
                overwrite_all_conflict_files,
                skip_all_conflict_files,
            )?;
        }
    }
    Ok(())
}
