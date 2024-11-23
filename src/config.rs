use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProjectWrapper {
    pub project: Project,
}

/// Represents a single include directive in the project configuration
#[derive(Debug, Deserialize)]
pub struct Include {
    file: String,
}

/// Repersents the top-level project configuration
#[derive(Debug, Deserialize)]
pub struct Project {
    name: String,
    description: String,
    version: String,
    board: String,
    memory_size: u64,
    memory_base: u32,
    includes: Vec<Include>,
}

/// Represents an enumerated value for a multi-bit field.
#[derive(Debug, Deserialize)]
pub struct EnumValue {
    name: String,
    value: String, // Can be "0b01", "0b0X", or "0b00 | 0b01"
    description: Option<String>,
}

/// Represents a single field within a register
#[derive(Debug, Deserialize)]
pub struct Field {
    name: String,
    bit_range: String,
    access: String,
    description: Option<String>,
    value_description: Option<HashMap<u8, String>>,
    enums: Option<Vec<EnumValue>>,
}

/// Represents a single register within a peripheral
#[derive(Debug, Deserialize)]
pub struct Register {
    friendly_name: String,
    name: String,
    offset: u32,
    size: u32,
    access: String,
    reset: Option<u32>,
    fields: Vec<Field>,
}

/// Represents a single peripheral in the project configuration
#[derive(Debug, Deserialize)]
pub struct Peripheral {
    name: String,
    description: Option<String>,
    version: Option<String>,
    interrupt: Option<u32>,
    registers: Vec<Register>,
}
