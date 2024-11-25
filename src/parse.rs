use crate::config::{Peripheral, PeripheralWrapper, Project, ProjectWrapper};

/// Parses the project configuration file into a Project struct
///
/// # Arguments
/// file_path: &str - The path to the project configuration file
///
/// # Returns
/// Project - The parsed project configuration
pub fn parse_project(file_path: &str) -> Project {
    let content = std::fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path));

    serde_yaml::from_str::<ProjectWrapper>(&content)
        .unwrap()
        .project
}

/// Parses the peripheral configuration file into a Peripheral struct
///
/// # Arguments
/// file_path: &str - The path to the peripheral configuration file
///
/// # Returns
/// Peripheral - The parsed peripheral configuration
pub fn parse_peripheral(file_path: &str) -> Peripheral {
    let content = std::fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path));

    serde_yaml::from_str::<PeripheralWrapper>(&content)
        .unwrap()
        .peripheral
}
