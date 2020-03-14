use dlopen::wrapper::WrapperApi;
use config::{Config, ParseError};

pub trait InputPlugin {
    fn validate_config(&self, config: &Config) -> Option<ParseError>;
}

#[derive(WrapperApi)]
pub struct InputPluginApi {
    get_name: extern fn() -> &'static str,
    get_version: extern fn() -> &'static str,
    get_plugin: extern fn() -> *mut dyn InputPlugin,
}
