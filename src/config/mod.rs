use std::path::PathBuf;

use serde::Deserialize;

use crate::defaults;

#[derive(Deserialize, Debug)]
pub struct ConfigOption {
    pub bit_field_version: Option<String>,
    pub volatile_register_version: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub bit_field_version: String,
    pub volatile_register_version: String,
}

/// Parses a configuration file that contains the versions of the dependencies.
/// If the configuration file is not provided, the default versions will be used.
/// If there are missing versions in the configuration file, the default versions will be used.
///
/// # Arguments
/// config_file: &Option<PathBuf> - The path to the configuration file.
///
/// # Returns
/// A Config struct containing the versions of the dependencies.
pub fn parse_config(config_file: &Option<PathBuf>) -> Config {
    // If there is a config file, parse it. Otherwise, return the default values.
    if let Some(config_file) = config_file {
        let content = std::fs::read_to_string(config_file).expect("Failed to read config file");
        let parsed_config: ConfigOption =
            toml::from_str(&content).expect("Failed to parse config file");

        // Check if there are missing versions and use the default values if they are missing
        Config {
            bit_field_version: parsed_config
                .bit_field_version
                .unwrap_or_else(|| defaults::BIT_FIELD_VERSION.to_string()),
            volatile_register_version: parsed_config
                .volatile_register_version
                .unwrap_or_else(|| defaults::VOLATILE_REGISTER_VERSION.to_string()),
        }
    } else {
        Config {
            bit_field_version: defaults::BIT_FIELD_VERSION.to_string(),
            volatile_register_version: defaults::VOLATILE_REGISTER_VERSION.to_string(),
        }
    }
}
