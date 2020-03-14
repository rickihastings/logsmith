pub mod input_plugin;

#[macro_use]
extern crate dlopen_derive;

use config::{Config, get_config};
use input_plugin::InputPluginApi;
use dlopen::wrapper::Container;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn run() {
    println!("Starting Logsmith {}", VERSION);

    // Load the config first. We do a multiple pass on the config here.
    // First we figure out if the basic structure is good, then we load
    // plugins. Then we pass the config through each plugin to determine
    // if the config is still valid, as each plugin can have it's own config items.
    match get_config() {
        Ok(cnf) => {
            load_input_plugins(&cnf);
        },
        Err(err) => {
            println!("{:#?}", err);
            std::process::exit(1);
        }
    }
}

fn load_input_plugins(cnf: &Config) {
    for plugin in &cnf.core.input_plugins {
        // Try load the plugin by name from the plugins directory
        let plugin_wrapper: Container<InputPluginApi> = unsafe { Container::load(plugin) }.unwrap();
        let plugin = unsafe { Box::from_raw(plugin_wrapper.get_plugin()) };

        println!("core: INFO: Loaded {} plugin {}", plugin_wrapper.get_name(), plugin_wrapper.get_version());
    }
}
