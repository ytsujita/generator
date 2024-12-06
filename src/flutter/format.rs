use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use super::init::PubspecYaml;

pub(crate) fn format_import() -> Result<(), Box<dyn std::error::Error>> {
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
        let relative_path = format!(".{}", package_path.replace('.', "/"));
        let import_statement = format!("import 'package:{}{}';", project_name, package_path);
        let new_import_statement = format!("import '{}.dart';", relative_path);
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
