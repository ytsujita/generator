use clap::{Parser, Subcommand};
mod flutter;
mod python;
mod terraform;
mod utils;

use colored::Colorize;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, flatten_help = true)]
struct Args {
    /// Generate type
    #[command(subcommand)]
    gen_type: GenType,
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
    },
    Config {
        /// Overwrite all conflict files.
        #[arg(short)]
        overwrite_conflict_file: bool,
        /// Skip all conflict files.
        #[arg(short)]
        skip_conflict_file: bool,
    },
    Gen {
        /// Overwrite all conflict files.
        #[arg(short)]
        overwrite_conflict_files: bool,
        /// Skip all conflict files.
        #[arg(short)]
        skip_conflict_files: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum TerraformMode {
    Init,
    Config,
    Gen,
}

fn main() {
    let args = Args::parse();
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
                    overwrite_conflict_files: overwrite_all,
                    skip_conflict_config_files: ignore_conflict_config_file,
                } => flutter::init::init_flutter_app(overwrite_all, ignore_conflict_config_file),
                FlutterMode::Config {
                    overwrite_conflict_file: delete_conflict_config_file,
                    skip_conflict_file: ignore_conflict_config_file,
                } => flutter::config::generate_sample_config(
                    flutter_config_file_name,
                    delete_conflict_config_file,
                    ignore_conflict_config_file,
                ),
                FlutterMode::Gen {
                    overwrite_conflict_files: delete_all_conflict_file,
                    skip_conflict_files: ignore_all_conflict_file,
                } => flutter::template::generate_files(
                    flutter_config_file_name,
                    delete_all_conflict_file,
                    ignore_all_conflict_file,
                ),
            }
        }
        GenType::Terraform { mode } => {
            match mode {
                TerraformMode::Init => println!("terraform init"),
                TerraformMode::Config => println!("terraform config"),
                TerraformMode::Gen => println!("terraform gen"),
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
