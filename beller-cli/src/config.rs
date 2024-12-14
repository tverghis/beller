use std::{
    fs::File,
    io::{self, Read},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Configuration {
    pds: PdsConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PdsConfig {
    endpoint: String,
}

unsafe impl Sync for PdsConfig {}
unsafe impl Sync for Configuration {}

impl Default for PdsConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://bsky.social".into(),
        }
    }
}

impl Configuration {
    pub fn from_config_file() -> Self {
        let config_path = ProjectDirs::from("com", "beller", "beller")
            .expect("failed to initialize ProjectDirs")
            .config_dir()
            .join("config.toml");

        match File::open(config_path) {
            Ok(mut f) => {
                let mut config = String::new();
                f.read_to_string(&mut config)
                    .expect("failed to read config file");
                toml::from_str(&config).expect("failed to parse config file")
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    eprintln!("Failed to open configuration file");
                }
                Self::default()
            }
        }
    }
}
