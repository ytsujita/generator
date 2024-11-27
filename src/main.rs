use clap::{Parser, Subcommand};
mod config;
mod flutter;
mod init_flutter_app;
mod utils;

use colored::Colorize;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, flatten_help = true)]
struct Args {
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
        #[arg(short)]
        overwrite_all: bool,
    },
    Config {
        #[arg(short)]
        delete_conflict_config_file: bool,
        #[arg(short)]
        ignore_conflict_config_file: bool,
    },
    Gen {
        #[arg(short)]
        delete_all_conflict_file: bool,
        #[arg(short)]
        ignore_all_conflict_file: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum TerraformMode {
    Init,
    Config,
    Gen,
}

static TARGET_FILE_NAME: &str = "my_flutter_config.yaml";

fn main() {
    let pubspec_path = Path::new("pubspec.yaml");
    if !pubspec_path.is_file() {
        println!("{}", "pubspec.yaml is not found.".red());
        return;
    }
    let args = Args::parse();
    let result = match args.gen_type {
        GenType::Flutter { mode } => match mode {
            FlutterMode::Init { overwrite_all } => {
                init_flutter_app::init_flutter_app(overwrite_all)
            }
            FlutterMode::Config {
                delete_conflict_config_file,
                ignore_conflict_config_file,
            } => config::generate_sample_config(
                TARGET_FILE_NAME,
                delete_conflict_config_file,
                ignore_conflict_config_file,
            ),
            FlutterMode::Gen {
                delete_all_conflict_file,
                ignore_all_conflict_file,
            } => flutter::flutter_template::generate_files(
                TARGET_FILE_NAME,
                delete_all_conflict_file,
                ignore_all_conflict_file,
            ),
        },
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
