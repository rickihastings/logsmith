extern crate logsmith;

use logsmith::input_plugin::InputPlugin;
use config::{Config, ParseError};

struct InputStdin;

impl InputPlugin for InputStdin {
    fn validate_config(&self, cnf: &Config) -> Option<ParseError> {
        println!("{:#?}", cnf);

        // Each pipeline should have inputs, ensure any stdin inputs are correct here.
        // @todo - check unique name
        for pipeline in &cnf.pipeline {
            for input in &pipeline.inputs {
                if input.kind == "stdin" && input.name != "" {
                    return Some(ParseError::new("stdin kind must have a name"));
                }
            }
        }

        None
    }
}

#[no_mangle]
pub fn get_name() -> &'static str {
    "InputStdin"
}

#[no_mangle]
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[no_mangle]
pub fn get_plugin() -> *mut dyn InputPlugin {
    Box::into_raw(Box::new(InputStdin {}))
}
