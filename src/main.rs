use clap::{Parser, Subcommand, ValueEnum};
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
    mode: Mode,
}

#[derive(Subcommand, Debug, Clone)]
enum Mode {
    Init {
        #[arg(short)]
        overwrite_all: bool,
    },
    Gen {
        gen_type: GenType,
        #[arg(short)]
        delete_all_conflict_file: bool,
        #[arg(short)]
        ignore_all_conflict_file: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum GenType {
    Flutter,
    Terraform,
    Python,
}

static TARGET_FILE_NAME: &str = "my_flutter_config.yaml";

fn main() {
    let pubspec_path = Path::new("pubspec.yaml");
    if !pubspec_path.is_file() {
        println!("{}", "pubspec.yaml is not found.".red());
        return;
    }
    let result = match Args::parse().mode {
        Mode::Gen {
            gen_type,
            delete_all_conflict_file,
            ignore_all_conflict_file,
        } => match gen_type {
            GenType::Flutter => flutter::flutter_template::generate_files(
                TARGET_FILE_NAME,
                delete_all_conflict_file,
                ignore_all_conflict_file,
            ),
            GenType::Terraform => Ok(()),
            GenType::Python => Ok(()),
        },
        Mode::Init { overwrite_all } => init_flutter_app::init_flutter_app(overwrite_all),
    };
    match result {
        Ok(_) => println!("{}", "done".green()),
        Err(error) => println!("{}", format!("{}", error).red()),
    }
}
