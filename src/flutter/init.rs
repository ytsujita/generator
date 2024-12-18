use std::fs::File;
use std::io::Write;
use std::{fs, path::Path};

use colored::Colorize;
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};

use crate::flutter::config::generate_sample_config;
use crate::utils::{self, copy_dir_recursive};

use super::FlutterCommandError;

static SRC_DIR: Dir = include_dir!("resources/flutter/");

pub(crate) fn init_flutter_app(
    file_name: &str,
    generate_config_only: bool,
    overwrite_conflict_files: bool,
    skip_conflict_files: bool,
) -> Result<(), FlutterCommandError> {
    let pubspec_yaml_file_str = fs::read_to_string("pubspec.yaml")?;
    let mut pubspec_yaml: PubspecYaml = serde_yaml::from_str(&pubspec_yaml_file_str).unwrap();
    generate_sample_config(
        pubspec_yaml.name,
        file_name,
        overwrite_conflict_files,
        skip_conflict_files,
    )?;
    if generate_config_only {
        return Ok(());
    }
    edit_pubspec_yaml(&mut pubspec_yaml)?;
    copy_dir_recursive(
        &SRC_DIR,
        Path::new("./"),
        overwrite_conflict_files,
        skip_conflict_files,
    )?;
    let args = vec![
        "flutter_hooks",
        "hooks_riverpod",
        "intl",
        "json_annotation",
        "logging",
        "mockito",
        "slang",
        "slang_flutter",
        "url_strategy",
        "uuid",
        "dev:build_runner",
        "dev:flutter_launcher_icons",
        "dev:flutter_lints",
        "dev:import_sorter",
        "dev:json_serializable",
        "dev:pubspec_dependency_sorter",
        "dev:rename_app",
        "dev:shelf",
        "dev:shelf_cors_headers",
        "dev:source_gen",
    ];
    let command_list = [
        format!("flutter pub add {}", args.join(" ")),
        "flutter pub get".to_string(),
        "flutter pub run pubspec_dependency_sorter".to_string(),
        "dart run slang".to_string(),
        "dart fix --apply".to_string(),
        "dart format .".to_string(),
        "flutter pub run import_sorter:main".to_string(),
    ];
    for command_name in command_list.iter() {
        if let Err(err) = utils::execute_external_command(command_name.clone()) {
            eprintln!(
                "{}",
                format!("{} is failed!: {:?}", command_name, err).red()
            );
            return Err(FlutterCommandError::ExecuteExternalCommand {
                command_name: command_name.clone(),
            });
        };
    }
    Ok(())
}

fn edit_pubspec_yaml(pubspec_yaml: &mut PubspecYaml) -> Result<(), std::io::Error> {
    pubspec_yaml.flutter.insert("generate", Value::Bool(true));
    pubspec_yaml.import_sorter = Some(ImportSorter {
        relative: true,
        emojis: false,
        comments: false,
    });
    let map: std::collections::HashMap<_, _> =
        [("sdk", Value::Str("flutter"))].iter().cloned().collect();
    pubspec_yaml
        .dependencies
        .insert("flutter_localizations", Value::HashMap(map));
    let mut file = File::create("pubspec.yaml")?;
    let config_str = serde_yaml::to_string(&pubspec_yaml).unwrap();
    file.write_all(config_str.as_bytes())?;
    file.flush()?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub(crate) enum Value<'a> {
    Str(&'a str),
    Bool(bool),
    HashMap(std::collections::HashMap<&'a str, Value<'a>>),
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct PubspecYaml<'a> {
    pub(crate) name: &'a str,
    pub(crate) description: &'a str,
    pub(crate) publish_to: &'a str,
    pub(crate) version: &'a str,
    pub(crate) environment: Environment<'a>,
    pub(crate) dependencies: std::collections::HashMap<&'a str, Value<'a>>,
    pub(crate) dev_dependencies: std::collections::HashMap<&'a str, Value<'a>>,
    pub(crate) flutter: std::collections::HashMap<&'a str, Value<'a>>,
    pub(crate) import_sorter: Option<ImportSorter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ImportSorter {
    pub(crate) relative: bool,
    pub(crate) emojis: bool,
    pub(crate) comments: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Environment<'a> {
    sdk: &'a str,
}
