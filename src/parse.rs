use log::debug;

use crate::config::ProjectWrapper;

/// Parses the project configuration file into a Project struct
///
/// # Arguments
/// file_path: &str - The path to the project configuration file
///
/// # Returns
/// Project - The parsed project configuration
pub fn parse_project(file_path: &str) -> ProjectWrapper {
    let content = std::fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path));

    debug!("File content: {}", content);

    serde_yaml::from_str::<ProjectWrapper>(&content).unwrap()
}
