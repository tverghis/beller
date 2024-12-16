use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::impls::defs::PdsUrl;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(default)]
    pub pds: PdsUrl,
}

pub enum ConfigOption {
    PdsEndpoint(Option<PdsUrl>),
}

impl Configuration {
    pub fn from_file() -> Self {
        read_config_file()
    }

    pub fn apply(&mut self, option: ConfigOption) -> &mut Self {
        #[allow(clippy::single_match)]
        match option {
            ConfigOption::PdsEndpoint(Some(endpoint)) => self.pds = endpoint,
            _ => {}
        };

        self
    }
}

fn read_config_file() -> Configuration {
    let config_file_path = std::env::var("BELLER_CONFIG_PATH");

    // If there's no file specified in the env var, we can skip
    // file parsing and just return early.
    let Ok(path) = config_file_path else {
        return Configuration::default();
    };

    if let Ok(mut f) = File::open(path) {
        let mut config = String::new();
        f.read_to_string(&mut config)
            .expect("failed to read config file");
        toml::from_str(&config).expect("failed to parse config file")
    } else {
        eprintln!("Failed to open configuration file");
        // TODO: handle this error case properly
        unimplemented!();
    }
}
