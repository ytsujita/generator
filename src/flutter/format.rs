use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use super::init::PubspecYaml;
use super::FlutterCommandError;

pub(crate) fn format_import() -> Result<(), FlutterCommandError> {
    let pubspec_yaml_file_str = fs::read_to_string("pubspec.yaml")?;
    let pubspec_yaml: PubspecYaml = serde_yaml::from_str(&pubspec_yaml_file_str).unwrap();
    let lib_dir = Path::new("lib");
    if let Err(e) = process_directory(lib_dir, pubspec_yaml.name) {
        eprintln!("Error processing directory: {}", e);
    } else {
        println!("All imports converted successfully.");
    }
    Ok(())
}

fn convert_imports_to_relative(
    file_path: &Path,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let import_regex = Regex::new(&format!(
        r#"import\s+['"]package:{}([^'"]+)['"];"#,
        project_name
    ))
    .unwrap();
    let mut new_content = content.clone();
    for cap in import_regex.captures_iter(&content) {
        let package_path = &cap[1];
        let file_name = file_path.to_str().unwrap().replace("\\", "/");
        let result = get_relative_path(&file_name[3..], package_path);
        let import_statement = format!("import 'package:{}{}';", project_name, package_path);
        let new_import_statement = format!("import '{}';", result);
        new_content = new_content.replace(&import_statement, &new_import_statement);
    }
    let mut file = File::create(file_path)?;
    file.write_all(new_content.as_bytes())?;
    Ok(())
}

fn process_directory(dir: &Path, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            process_directory(&path, project_name)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("dart") {
            convert_imports_to_relative(&path, project_name)?;
        }
    }
    Ok(())
}

fn get_relative_path<'a>(from: &'a str, to: &'a str) -> String {
    let from_parts: Vec<&str> = from.split('/').collect();
    let to_parts: Vec<&str> = to.split('/').collect();
    let mut common_prefix_length = 0;
    for (from_part, to_part) in from_parts.iter().zip(&to_parts) {
        if from_part == to_part {
            common_prefix_length += 1;
        } else {
            break;
        }
    }
    let mut result = vec![".."; from_parts.len() - common_prefix_length - 1];
    for part in &to_parts[common_prefix_length..] {
        result.push(part);
    }
    result.join("/")
}
