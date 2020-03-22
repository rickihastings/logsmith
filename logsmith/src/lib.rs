extern crate dynamic_reload;

mod input_plugin;
mod plugins;

// Re-export all dependencies so modules can use them
// without needing to bundle their own versions.
pub use config as config;

use plugins::Plugins;
use input_plugin::{new_input_plugin, InputPlugin};
use config::{Config, ParseError, get_config};
use std::collections::HashMap;
use std::sync::Arc;
use dynamic_reload::{DynamicReload, Search, PlatformName};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct App {
    plugins: Plugins,
    input_plugins: HashMap<String, InputPlugin>,
}

impl App {
    pub fn new() -> Self {
        Self {
            plugins: Plugins::new(),
            input_plugins: HashMap::new(),
        }
    }

    pub fn run(self) {
        println!("Starting Logsmith {}", VERSION);

        // Load the config first. We do a multiple pass on the config here.
        // First we figure out if the basic structure is good, then we load
        // plugins. Then we pass the config through each plugin to determine
        // if the config is still valid, as each plugin can have it's own config items.
        match get_config() {
            Ok(cnf) => {
                self.load_input_plugins(cnf);
            },
            Err(err) => {
                exit_with_error(err);
            }
        };
    }

    fn load_input_plugins(mut self, cnf: Config) {
        // Setup the reload handler. A temporary directory will be created inside the target/debug
        // where plugins will be loaded from. That is because on some OS:es loading a shared lib
        // will lock the file so we can't overwrite it so this works around that issue.
        let mut reload_handler = DynamicReload::new(Some(vec!["target/debug/plugins"]),
                                                    None,
                                                    Search::Default);

        for name in &cnf.core.input_plugins {
            match reload_handler.add_library(name, PlatformName::Yes) {
                Ok(lib) => {
                    // This is the dylib/so, it's not that useful on it's own though because none of
                    // the values are cached. We don't really want to be using this raw API everywhere.
                    self.plugins.add_plugin(&lib);

                    let input_plugin_lib = unsafe {
                        new_input_plugin(name, &lib)
                    };

                    self.input_plugins.insert(name.to_string(), input_plugin_lib.clone());

                    if let Some(err) = (input_plugin_lib.validate_config)(cnf.clone()) {
                        exit_with_error(err);
                    }
                },
                Err(err) => {
                    println!("core: ERROR: Failed to load {}", name);
                    exit_with_error(err);
                }
            }
        }
    }
}

fn exit_with_error<T>(err: T)
where
    T: std::fmt::Debug
{
    println!("{:#?}", err);
    std::process::exit(1);
}
