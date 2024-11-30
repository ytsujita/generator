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

pub(crate) fn init_flutter_app(
    file_name: &str,
    generate_config_only: bool,
    overwrite_conflict_files: bool,
    skip_conflict_files: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    generate_sample_config(file_name, overwrite_conflict_files, skip_conflict_files)?;
    if generate_config_only {
        return Ok(());
    }
    copy_dir_recursive(
        Path::new("./"),
        overwrite_conflict_files,
        skip_conflict_files,
    )?;
    edit_pubspec_yaml()?;

    println!("{}", "completed!".green());
    println!("{}", "Please run below commands!".green());
    let args = vec![
        "intl",
        "hooks_riverpod",
        "flutter_hooks",
        "flutter_localizations:{\"sdk\":\"flutter\"}",
        "json_annotation",
        "logging",
        "mockito",
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
    println!("flutter pub add {}", args.join(" "));
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
        println!("{:?}", file);
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

fn edit_pubspec_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let pubspec_yaml_file_str = fs::read_to_string("pubspec.yaml")?;
    let mut pubspec_yaml: PubspecYaml = serde_yaml::from_str(&pubspec_yaml_file_str).unwrap();
    pubspec_yaml.flutter.insert("generate", Value::Bool(true));
    let mut file = File::create("pubspec.yaml")?;
    let config_str = serde_yaml::to_string(&pubspec_yaml).unwrap();
    file.write_all(config_str.as_bytes())?;
    file.flush()?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Value<'a> {
    Str(&'a str),
    Bool(bool),
    HashMap(std::collections::HashMap<&'a str, Value<'a>>),
}

#[derive(Debug, Deserialize, Serialize)]
struct PubspecYaml<'a> {
    pub(crate) name: &'a str,
    pub(crate) description: &'a str,
    pub(crate) publish_to: &'a str,
    pub(crate) version: &'a str,
    pub(crate) environment: Environment<'a>,
    pub(crate) dependencies: std::collections::HashMap<&'a str, Value<'a>>,
    pub(crate) dev_dependencies: std::collections::HashMap<&'a str, Value<'a>>,
    pub(crate) flutter: std::collections::HashMap<&'a str, Value<'a>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Environment<'a> {
    sdk: &'a str,
}
