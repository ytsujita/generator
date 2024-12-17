use clap::{Parser, Subcommand};
mod aws;
mod config;
mod flutter;
mod terraform;
mod utils;

use std::env;
use std::path::Path;

static APPLICATION_NAME: &str = "easy_gen";

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
    Config {
        /// Overwrite config files.
        #[arg(short)]
        overwrite_conflict_file: bool,
    },
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
}

fn main() {
    let args = Args::parse();
    let new_dir = Path::new(args.dir.as_str());
    if let Err(e) = env::set_current_dir(new_dir) {
        eprintln!("Failed to change directory. {}", e);
        return;
    }
    match args.gen_type {
        GenType::Flutter { mode } => flutter::command_handler(mode),
        GenType::Terraform { mode } => terraform::command_handler(mode),
        GenType::Config {
            overwrite_conflict_file,
        } => config::command_handler(overwrite_conflict_file),
    };
}
