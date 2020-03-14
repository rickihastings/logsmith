use serde::{Serialize, Deserialize};
use toml::de::Error;
use toml::value::{Value, Table};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum InputType {
    Stdin,
    Http,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OutputType {
    Stdout,
}

#[derive(Deserialize)]
pub struct Config {
    pipeline: Vec<Pipeline>,
}

#[derive(Deserialize)]
pub struct Pipeline {
    name: String,
    path: String,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

#[derive(Deserialize)]
pub struct Input {
    name: String,
    kind: InputType,
    options: Option<Table>,
}

#[derive(Deserialize)]
pub struct Output {
    name: String,
    kind: OutputType,
}

// fn load_config() {
//     let contents = load_config()?;
//     let config = parse_config(contents)?;

// }

pub fn parse_config(contents: String) -> Result<Config, Error> {
    toml::from_str(&contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn can_parse_config() {
        let contents = r#"
            [[pipeline]]
            name = "test-pipeline"
            path = "./test.pipeline.toml"
            
                # This pipeline will combine two inputs, stdin, and http.
                # Pass them through pipeline.toml, and into one output stdout
                [[pipeline.inputs]]
                name = "stdin-input"
                kind = "stdin"
                
                [[pipeline.inputs]]
                name = "http-input"
                kind = "http"
                options = { port = 8080 }
                
                [[pipeline.outputs]]
                name = "stdout-output"
                kind = "stdout"
        "#;

        let config = parse_config(contents.to_string()).unwrap();
        let pipeline = &config.pipeline[0];
        
        assert_eq!(pipeline.name, "test-pipeline");
        assert_eq!(pipeline.path, "./test.pipeline.toml");

        let first_input = &pipeline.inputs[0];
        let second_input = &pipeline.inputs[1];

        assert_eq!(first_input.name, "stdin-input");
        assert_eq!(first_input.kind, InputType::Stdin);

        assert_eq!(second_input.name, "http-input");
        assert_eq!(second_input.kind, InputType::Http);

        // Options is stored as a BTreeMap, as they can vary
        // per input type.
        let mut options = BTreeMap::new();
        options.insert("port".to_string(), Value::Integer(8080));

        assert_eq!(second_input.options, Some(options));

        let first_output = &pipeline.outputs[0];

        assert_eq!(first_output.name, "stdout-output");
        assert_eq!(first_output.kind, OutputType::Stdout);
    }
}
