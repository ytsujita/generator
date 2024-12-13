use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, path::Path};

use colored::Colorize;
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};

use crate::flutter::config::generate_sample_config;
use crate::utils::create_file;

use super::super::APPLICATION_NAME;

pub(crate) fn init_flutter_app(
    file_name: &str,
    generate_config_only: bool,
    overwrite_conflict_files: bool,
    skip_conflict_files: bool,
) -> Result<(), Box<dyn std::error::Error>> {
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
        Path::new("./"),
        overwrite_conflict_files,
        skip_conflict_files,
    )?;
    println!("{}", "completed!".green());
    println!("{}", "Please run below commands!".blue());
    let args = vec![
        "flutter_hooks",
        "hooks_riverpod",
        "intl",
        "json_annotation",
        "logging",
        "mockito",
        "shelf",
        "shelf_cors_headers",
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
        "dev:source_gen",
    ];
    println!("> flutter pub add {}", args.join(" "));
    println!("> flutter pub get");
    println!("> dart run slang");
    println!("> dart fix --apply");
    println!("> dart format .");
    println!("> flutter pub run import_sorter:main");
    println!("> {} flutter gen", APPLICATION_NAME);
    Ok(())
}

static SRC_DIR: Dir = include_dir!("resources/flutter/");

fn copy_dir_recursive(
    dst: &Path,
    overwrite_all: bool,
    ignore_conflict_config_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
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
                let file_buf: PathBuf = PathBuf::from_str(file_path)?;
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

fn edit_pubspec_yaml(pubspec_yaml: &mut PubspecYaml) -> Result<(), Box<dyn std::error::Error>> {
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
