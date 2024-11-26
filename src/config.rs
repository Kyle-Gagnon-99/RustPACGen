use serde::Deserialize;

use crate::utils::StringOrNumber;

#[derive(Debug, Deserialize)]
pub struct ProjectWrapper {
    pub project: Project,
}

#[derive(Debug, Deserialize)]
pub struct PeripheralWrapper {
    pub peripheral: Peripheral,
}

/// Represents a single include directive in the project configuration
#[derive(Debug, Deserialize)]
pub struct Include {
    pub file: String,
}

/// Repersents the top-level project configuration
#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub board: String,
    pub memory_size: u64,
    pub memory_base: u32,
    pub includes: Vec<Include>,
}

/// Represents an enumerated value for a multi-bit field.
#[derive(Debug, Deserialize)]
pub struct Values {
    pub value: StringOrNumber,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Represents a single field within a register
#[derive(Debug, Deserialize)]
pub struct Field {
    pub id: String,
    pub bit_range: String,
    pub access: String,
    pub description: String,
    pub reset: Option<u32>,
    pub values: Option<Vec<Values>>,
}

/// Represents a single register within a peripheral
#[derive(Debug, Deserialize)]
pub struct Register {
    pub id: String,
    pub name: String,
    pub description: String,
    pub offset: usize,
    pub size: usize,
    pub reset: Option<u32>,
    pub access: String,
    pub fields: Option<Vec<Field>>,
}

/// Represents a single peripheral in the project configuration
#[derive(Debug, Deserialize)]
pub struct Peripheral {
    pub id: String,
    pub description: String,
    pub registers: Vec<Register>,
}
