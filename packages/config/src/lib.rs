mod file;

use std::fmt;
use serde::Deserialize;
use toml::value::{Table, Value};

#[derive(Deserialize)]
pub struct ParseError;

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "config: ERROR: Unable to parse config. Please ensure the specified file exists, and is valid TOML.")
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "config: ERROR: Unable to parse config. Please ensure the specified file exists, and is valid TOML.")
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub core: Core,
    pub pipeline: Vec<Pipeline>,
}

#[derive(Deserialize)]
pub struct Core {
    pub input_plugins: Vec<String>,
}

#[derive(Deserialize)]
pub struct Pipeline {
    pub name: String,
    pub path: String,
}

pub fn get_config() -> Result<Config, ParseError> {
    match file::load_config("../logsmith.toml".to_string()) {
        Ok(contents) => parse_config(contents),
        Err(err) => {
            println!("config: ERROR: {}", err);
            Err(ParseError {})
        }
    }
}

fn parse_config(contents: String) -> Result<Config, ParseError> {
    match toml::from_str::<Config>(&contents) {
        Ok(config) => Ok(config),
        Err(err) => {
            println!("config: ERROR: {}", err);
            Err(ParseError {})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn can_parse_config() {
        let contents = r#"
        [core]
        input_plugins = [
            "./plugins/libinput_stdin.dylib"
        ]

        [[pipeline]]
        name = "test-pipeline"
        path = "./test.pipeline.toml"
        "#;

        let cnf = parse_config(contents.to_string()).unwrap();

        let pipeline = &cnf.pipeline[0];

        assert_eq!(pipeline.name, "test-pipeline");
        assert_eq!(pipeline.path, "./test.pipeline.toml");

        // let first_input = &pipeline.inputs[0];
        // let second_input = &pipeline.inputs[1];

        // assert_eq!(first_input.get("name").unwrap(), Value::String("stdin-input".to_string()));
        // assert_eq!(first_input.get("kind"), "stdin");

        // assert_eq!(second_input["name"], "http-input");
        // assert_eq!(second_input["kind"], "http");

        // // Options is stored as a BTreeMap, as they can vary
        // // per input type.
        // let mut options = BTreeMap::new();
        // options.insert("port".to_string(), Value::Integer(8080));

        // assert_eq!(second_input.options, Some(options));

        // let first_output = &pipeline.outputs[0];

        // assert_eq!(first_output.name, "stdout-output");
        // assert_eq!(first_output.kind, OutputType::Stdout);
    }
}
