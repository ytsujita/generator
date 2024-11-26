use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use colored::Colorize;

use crate::config::generate_sample_config;
use crate::utils::input_yes;
use crate::TARGET_FILE_NAME;

const ANALYSIS_OPTIONS_YAML: &[u8] = include_bytes!("resources/analysis_options.yaml");
const L10N_YAML: &[u8] = include_bytes!("resources/l10n.yaml");
const BUILD_YAML: &[u8] = include_bytes!("resources/build.yaml");
const VSCODE_LAUNCH_JSON: &[u8] = include_bytes!("resources/.vscode/launch.json");
const VSCODE_SETTINGS_JSON: &[u8] = include_bytes!("resources/.vscode/settings.json");
const FLAVOR_ENV_FILE: &[u8] = include_bytes!("resources/flavor/.env.sample");

struct GenConfig<'a> {
    bytes: &'a [u8],
    file_path_name: &'a str,
}

pub(crate) fn init_flutter_app(overwrite_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let gen_configs: Vec<GenConfig> = vec![
        GenConfig {
            file_path_name: "build.yaml",
            bytes: BUILD_YAML,
        },
        GenConfig {
            file_path_name: "analysis_options.yaml",
            bytes: ANALYSIS_OPTIONS_YAML,
        },
        GenConfig {
            file_path_name: "l10n.yaml",
            bytes: L10N_YAML,
        },
        GenConfig {
            file_path_name: ".vscode/launch.json",
            bytes: VSCODE_LAUNCH_JSON,
        },
        GenConfig {
            file_path_name: ".vscode/settings.json",
            bytes: VSCODE_SETTINGS_JSON,
        },
        GenConfig {
            file_path_name: "flavor/.env.sample",
            bytes: FLAVOR_ENV_FILE,
        },
    ];
    for gen_config in gen_configs {
        generate_file(overwrite_all, gen_config.file_path_name, gen_config.bytes)?;
    }
    match generate_sample_config(TARGET_FILE_NAME) {
        Ok(_) => println!("{}", "generate config file successfully.".green()),
        Err(_) => println!("{}", "failed to generate config file.".red()),
    }

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
    println!("> flutter pub add {}", args.join(" "));
    Ok(())
}

fn generate_file(
    overwrite_all: bool,
    file_path_name: &str,
    bytes: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let path_buf = PathBuf::from(file_path_name);
    let parent = path_buf.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(parent.to_str().unwrap())?;
    }
    if !path_buf.exists() {
        if overwrite_all
            || input_yes(&format!(
                "{} file is exist. Do you want to overwrite it? (y/N)",
                file_path_name
            ))
        {
            let mut file = File::create(file_path_name)?;
            file.write_all(bytes)?;
            file.flush()?;
        } else {
            println!("passed {}", file_path_name);
        }
    } else {
        let mut file = File::create(file_path_name)?;
        file.write_all(bytes)?;
        file.flush()?;
    }
    Ok(())
}
