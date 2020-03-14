extern crate logsmith;

use logsmith::input_plugin::InputPlugin;
use config::Config;

struct InputStdin;

impl InputPlugin for InputStdin {
    fn validate_config(&self, config: Config) -> bool {
        return true
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
