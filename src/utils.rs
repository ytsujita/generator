use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
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

pub(crate) fn execute_external_command(command: String) -> Result<(), std::io::Error> {
    let command_arc = Arc::new(command.clone());
    let command_clone = Arc::clone(&command_arc);
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("Running {}", &command));
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
    handle.join().expect("Thread panicked!");
    let done_message = format!("{}", format!("{} is done!", command_arc).green());
    spinner.finish_with_message(done_message);
    Ok(())
}
