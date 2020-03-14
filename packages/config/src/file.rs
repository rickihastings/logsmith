use std::fs::File;
use std::io::{Read, Result};

pub fn load_config(config_path: String) -> Result<String> {
    let mut cnf_file = File::open(config_path)?;
    let mut contents = String::new();
    cnf_file.read_to_string(&mut contents)?;
    Ok(contents)
}
