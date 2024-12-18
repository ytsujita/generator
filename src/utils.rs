use include_dir::Dir;
use std::any::Any;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Clone)]
enum CreateFileType {
    Overwrite,
    SkipConflict,
    None,
}

pub(crate) fn create_file(
    file_path_name: &str,
    bytes: &[u8],
    overwrite_conflict: bool,
    skip_conflict: bool,
) -> Result<(), std::io::Error> {
    let path_buf = PathBuf::from(file_path_name);
    let parent = path_buf.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(parent.to_str().unwrap())?;
    }
    let mut create_file_type = CreateFileType::None;
    if overwrite_conflict {
        create_file_type = CreateFileType::Overwrite;
    }
    if skip_conflict {
        create_file_type = CreateFileType::SkipConflict;
    }

    match create_file_type {
        CreateFileType::Overwrite => {
            let mut file = File::create(file_path_name)?;
            file.write_all(bytes)?;
            file.flush()?;
            Ok(())
        }
        CreateFileType::SkipConflict => {
            if path_buf.exists() {
                return Ok(());
            }
            let mut file = File::create(file_path_name)?;
            file.write_all(bytes)?;
            file.flush()?;
            Ok(())
        }
        CreateFileType::None => {
            if !path_buf.exists() {
                let mut file = File::create(file_path_name)?;
                file.write_all(bytes)?;
                file.flush()?;
                return Ok(());
            }
            if path_buf.exists()
                && input_yes(&format!(
                    "{} file is exist. Do you want to overwrite it?",
                    file_path_name
                ))
            {
                let mut file = File::create(file_path_name)?;
                file.write_all(bytes)?;
                file.flush()?;
            }
            Ok(())
        }
    }
}

pub(crate) fn input_yes(message: &str) -> bool {
    print!("{} (y/N): ", message);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    let res = buffer.trim().to_string();
    *"yes" == res || res == String::from('y')
}

pub(crate) fn create_dir(path: &str) -> Result<(), std::io::Error> {
    let path_buf = PathBuf::from(path);
    if path_buf.exists() {
        return Ok(());
    }
    fs::create_dir_all(path)?;
    Ok(())
}

pub(crate) fn execute_external_command(command: String) -> Result<(), Box<dyn Any + Send>> {
    let command_arc = Arc::new(command.clone());
    let command_clone = Arc::clone(&command_arc);
    let spinner = ProgressBar::new_spinner();
    let command_name = command.clone();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("Running {}", command_name));
    let handle = std::thread::spawn(move || {
        let is_windows = std::env::consts::OS == "windows";
        let (shell, arg) = if is_windows {
            ("cmd", "/C")
        } else {
            ("bash", "-c")
        };
        let _output = std::process::Command::new(shell)
            .arg(arg) // コマンドに渡す引数を指定します。
            .arg(&**command_clone) // コマンドを実行し、その出力を取得します。
            .output() // コマンドを実行し、その出力を取得します。
            .expect("failed to execute process");
    });
    while !handle.is_finished() {
        spinner.tick();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    spinner.finish_and_clear();
    let done_message = match handle.join() {
        Ok(_) => format!("{}", format!("{} is done!", command_name).green()),
        Err(err) => {
            return Err(err);
        }
    };
    println!("{}", done_message);
    Ok(())
}

pub(crate) fn copy_dir_recursive(
    src: &Dir,
    dst: &Path,
    overwrite_all_conflict_files: bool,
    ignore_all_conflict_files: bool,
) -> Result<(), std::io::Error> {
    if !dst.exists() {
        fs::create_dir(dst)?;
    }
    let glob = "**/*";
    for file in src.find(glob).unwrap() {
        let dst_path = dst.join(file.path());
        match file {
            include_dir::DirEntry::Dir(d) => {
                fs::create_dir_all(d.path().as_os_str().to_str().unwrap()).unwrap();
            }
            include_dir::DirEntry::File(f) => {
                let file_path = f.path().as_os_str().to_str().unwrap();
                let file_buf: PathBuf = PathBuf::from_str(file_path).unwrap();
                if let Some(parent) = file_buf.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)?;
                    }
                }
                let _ = create_file(
                    dst_path.as_os_str().to_str().unwrap(),
                    f.contents(),
                    overwrite_all_conflict_files,
                    ignore_all_conflict_files,
                );
            }
        }
    }
    Ok(())
}
