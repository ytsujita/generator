use std::{fs, io, path::PathBuf};

pub(crate) fn input_yes(message: &str) -> bool {
    println!("{}", message);
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    let res = buffer.trim().to_string();
    *"yes" == res || res == String::from('y')
}

pub(crate) fn create_dir(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path_buf = PathBuf::from(path);
    if path_buf.exists() {
        return Ok(());
    }
    fs::create_dir_all(path)?;
    Ok(())
}
