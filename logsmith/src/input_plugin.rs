use std::sync::Arc;
use dynamic_reload::{Lib, Symbol};

type GetName = extern fn() -> &'static str;
type GetVersion = extern fn() -> &'static str;
type ValidateConfig = extern fn(cnf: config::Config) -> Option<config::ParseError>;

pub type InputPlugin = Arc<Plugin>;

pub struct Plugin {
    pub lib: Arc<Lib>,
    pub name: &'static str,
    pub version: &'static str,
    pub validate_config: ValidateConfig,
}

pub unsafe fn new_input_plugin(plugin_name: &String, plugin: &Arc<Lib>) -> InputPlugin {
    let name: Symbol<GetName> = plugin.lib.get(b"get_name")
        .expect(&format!("core: ERROR: Failed to load get_name function for {}", plugin_name));

    let version: Symbol<GetVersion> = plugin.lib.get(b"get_version")
        .expect(&format!("core: ERROR: Failed to load get_version function for {}", plugin_name));

    let validate_config: Symbol<ValidateConfig> = plugin.lib.get(b"validate_config")
        .expect(&format!("core: ERROR: Failed to load validate_config function for {}", plugin_name));

    println!("core: INFO: Loaded {} plugin {}", name(), version());

    Arc::new(Plugin {
        lib: plugin.clone(),
        name: name(),
        version: version(),
        validate_config: *validate_config,
    })
}

