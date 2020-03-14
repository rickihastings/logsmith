use std::fs::File;

pub fn load_config(config_path: &'str) -> Result<String> {
    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
