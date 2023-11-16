use std::{fs::read_to_string, io};

pub fn read_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let data = read_to_string(file_path)?;

    let lines = data.lines()
        .map(|text| text.to_string())
        .collect::<Vec<String>>();

    Ok(lines)
}