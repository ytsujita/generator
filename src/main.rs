use clap::{Parser, Subcommand};
mod dynamodb;
mod flutter;
mod terraform;
mod utils;

use colored::Colorize;
use std::env;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, flatten_help = true)]
struct Args {
    /// Generate type
    #[command(subcommand)]
    gen_type: GenType,
    /// target dir
    #[arg(default_value = ".", short = 'd', long = "dir")]
    dir: String,
}

#[derive(Subcommand, Debug, Clone)]
enum GenType {
    Flutter {
        #[command(subcommand)]
        mode: FlutterMode,
    },
    Terraform {
        #[command(subcommand)]
        mode: TerraformMode,
    },
    Python,
}

#[derive(Subcommand, Debug, Clone)]
enum FlutterMode {
    Init {
        /// Overwrite all conflict files.
        #[arg(short)]
        overwrite_conflict_files: bool,
        /// Skip all conflict files.
        #[arg(short)]
        skip_conflict_config_files: bool,
        /// Generate only config file.
        #[arg(short)]
        config_only: bool,
    },
    Gen {
        /// Overwrite all conflict files.
        #[arg(short)]
        overwrite_conflict_files: bool,
        /// Skip all conflict files.
        #[arg(short)]
        skip_conflict_files: bool,
    },
    Format,
}

#[derive(Subcommand, Debug, Clone)]
enum TerraformMode {
    Init {
        /// Overwrite all conflict files.
        #[arg(short)]
        overwrite_conflict_files: bool,
        /// Skip all conflict files.
        #[arg(short)]
        skip_conflict_config_files: bool,
    },
    Gen,
}

fn main() {
    let args = Args::parse();
    let new_dir = Path::new(args.dir.as_str());
    if let Err(e) = env::set_current_dir(new_dir) {
        eprintln!("Failed to change directory. {}", e);
        return;
    }
    let result = match args.gen_type {
        GenType::Flutter { mode } => {
            let pubspec_path = Path::new("pubspec.yaml");
            if !pubspec_path.is_file() {
                println!("{}", "pubspec.yaml is not found.".red());
                return;
            }
            let flutter_config_file_name = "my_flutter_config.yaml";
            match mode {
                FlutterMode::Init {
                    overwrite_conflict_files,
                    skip_conflict_config_files,
                    config_only,
                } => flutter::init::init_flutter_app(
                    flutter_config_file_name,
                    config_only,
                    overwrite_conflict_files,
                    skip_conflict_config_files,
                ),
                FlutterMode::Gen {
                    overwrite_conflict_files,
                    skip_conflict_files,
                } => flutter::template::generate_files(
                    flutter_config_file_name,
                    overwrite_conflict_files,
                    skip_conflict_files,
                ),
                FlutterMode::Format => flutter::format::format_import(),
            }
        }
        GenType::Terraform { mode } => {
            let _ = match mode {
                TerraformMode::Init {
                    overwrite_conflict_files,
                    skip_conflict_config_files,
                } => terraform::init::init_terraform_project(
                    overwrite_conflict_files,
                    skip_conflict_config_files,
                ),
                TerraformMode::Gen => Ok(()),
            };
            Ok(())
        }
        GenType::Python => Ok(()),
    };
    match result {
        Ok(_) => println!("{}", "done".green()),
        Err(error) => println!("{}", format!("{}", error).red()),
    }
}
