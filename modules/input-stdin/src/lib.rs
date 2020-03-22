extern crate logsmith;

use logsmith::config::{Config, ParseError};

#[no_mangle]
pub fn get_name() -> &'static str {
    "InputStdin"
}

#[no_mangle]
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[no_mangle]
pub fn validate_config(cnf: Config) -> Option<ParseError> {
    // Each pipeline should have inputs, ensure any stdin inputs are correct here.
    for pipeline in &cnf.pipeline {
        let mut names: Vec<String> = Vec::new();

        for input in &pipeline.inputs {
            if input.kind == "stdin" {
                if input.kind == "stdin" && input.name == "" {
                    return Some(ParseError::new("stdin kind must have a name"));
                }

                if names.contains(&input.name) {
                    return Some(ParseError::new("stdin kind must have a unique name"));
                }

                names.push(input.name.clone());
            }
        }
    }

    println!("{:#?}", cnf);

    None
}
