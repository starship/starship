use std::fs::File;
use std::io::{Read, Result};

/// Return the string contents of a file
pub fn read_file(file_name: &str) -> Result<String> {
    let mut file = File::open(file_name)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;
    Ok(data)
}
