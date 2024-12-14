use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Configuration {
    pub pds: Pds,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pds {
    pub endpoint: String,
}

impl Default for Pds {
    fn default() -> Self {
        Self {
            endpoint: "https://bsky.social".into(),
        }
    }
}

pub enum ConfigOption {
    PdsEndpoint(Option<String>),
}

impl Configuration {
    pub fn from_file() -> Self {
        read_config_file()
    }

    pub fn apply(&mut self, option: ConfigOption) -> &mut Self {
        match option {
            ConfigOption::PdsEndpoint(Some(endpoint)) => self.pds.endpoint = endpoint,
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

    match File::open(path) {
        Ok(mut f) => {
            let mut config = String::new();
            f.read_to_string(&mut config)
                .expect("failed to read config file");
            toml::from_str(&config).expect("failed to parse config file")
        }
        _ => {
            eprintln!("Failed to open configuration file");
            unimplemented!();
        }
    }
}
