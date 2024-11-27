use colored::Colorize;

use crate::utils::{create_dir, create_file, CreateFileType};

const ANALYSIS_OPTIONS_YAML: &[u8] =
    include_bytes!("../../resources/flutter/analysis_options.yaml");
const L10N_YAML: &[u8] = include_bytes!("../../resources/flutter/l10n.yaml");
const BUILD_YAML: &[u8] = include_bytes!("../../resources/flutter/build.yaml");
const VSCODE_LAUNCH_JSON: &[u8] = include_bytes!("../../resources/flutter/.vscode/launch.json");
const VSCODE_SETTINGS_JSON: &[u8] = include_bytes!("../../resources/flutter/.vscode/settings.json");
const FLAVOR_ENV_FILE: &[u8] = include_bytes!("../../resources/flutter/flavor/.env.sample");
const MIDDLEWARE: &[u8] = include_bytes!("../../resources/flutter/middleware/shelf_server.dart");

struct GenConfig<'a> {
    bytes: &'a [u8],
    file_path_name: &'a str,
}

pub(crate) fn init_flutter_app(
    overwrite_all: bool,
    ignore_conflict_config_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
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
        GenConfig {
            file_path_name: "middleware/shelf_server.dart",
            bytes: MIDDLEWARE,
        },
    ];
    for gen_config in gen_configs {
        generate_file(
            overwrite_all,
            ignore_conflict_config_file,
            gen_config.file_path_name,
            gen_config.bytes,
        )?;
    }

    let ddd_architecture_directories = vec![
        "lib/widget/components/",
        "lib/widget/page/",
        "lib/widget/theme/",
        "lib/provider/notifier/",
        "lib/provider/state/",
        "lib/navigation/",
        "lib/l10n/",
        "lib/infrastructure/repository_impl/",
        "lib/infrastructure/service_impl/",
        "lib/infrastructure/query_use_case_impl/",
        "lib/domain/entity/",
        "lib/domain/exception/",
        "lib/domain/repository/",
        "lib/domain/service/",
        "lib/common/extension/",
        "lib/common/utils/",
        "lib/application/use_case/",
        "lib/application/command_use_case_impl/",
        "lib/__mock__/",
        "middleware/",
    ];
    for ddd_dir in ddd_architecture_directories {
        create_dir(ddd_dir)?;
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
    println!("flutter pub add {}", args.join(" "));
    Ok(())
}

fn generate_file(
    overwrite: bool,
    ignore_conflict_config_file: bool,
    file_path_name: &str,
    bytes: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut create_file_type = CreateFileType::None;
    if overwrite {
        create_file_type = CreateFileType::Overwrite;
    }
    if ignore_conflict_config_file {
        create_file_type = CreateFileType::SkipConflict;
    }
    create_file(file_path_name, bytes, create_file_type)
}
