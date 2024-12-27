use std::path::PathBuf;

/// The parse module contains logic and data structures for parsing the input JSON file
/// using the serde library.
use serde::Deserialize;
use utils::{Access, BitRange};

pub mod utils;

/// The Manifest struct represents the top-level definition of the peripheral access crate.
/// It contains the name of the peripheral access crate, the register size, whether or not
/// the crate is no_std compatible, and a list of file names that relate to the peripherals.
pub struct Manifest {
    pub name: String,
    pub register_size: usize,
    pub is_no_std: Option<bool>,
    pub peripherals: Vec<String>,
}

/// The PeripheralAccess struct represents the top-level definition
/// of the input JSON file.
///
/// It contains the name of the peripheral access crate and a list of peripherals.
#[derive(Deserialize, Debug)]
pub struct PeripheralAccess {
    pub name: String,
    pub register_size: usize,
    pub is_no_std: Option<bool>,
    pub peripherals: Vec<Peripheral>,
}

/// The Peripheral struct represents a peripheral device in the input JSON file.
///
/// It contains the name of the peripheral, the base address of the peripheral,
/// and a list of registers for the device.
#[derive(Deserialize, Debug)]
pub struct Peripheral {
    pub name: String,
    #[serde(deserialize_with = "utils::deserialize_num")]
    pub base_address: usize,
    pub registers: Vec<Register>,
}

/// The Register struct represents a register in a peripheral device.
///
/// It contains the name of the register, the offset from the base address,
/// the size of the register in bits, the access type of the register, a description,
/// and a list of fields in the register.
#[derive(Deserialize, Debug)]
pub struct Register {
    pub name: String,
    #[serde(deserialize_with = "utils::deserialize_num")]
    pub offset: usize,
    pub size: usize,
    #[serde(deserialize_with = "utils::deserialize_access")]
    pub access: Access,
    pub description: String,
    pub fields: Option<Vec<Field>>,
}

/// The Field struct represents a field in a register.
///
/// It contains the name of the field, the bit range of the field,
/// a description, the access type of the field, a default value,
/// and optionally a list of enums for the field.
#[derive(Deserialize, Debug)]
pub struct Field {
    pub name: String,
    #[serde(deserialize_with = "utils::deserialize_bit_range")]
    pub bit_range: BitRange,
    pub description: String,
    #[serde(default, deserialize_with = "utils::deserialize_access_with_option")]
    pub access: Option<Access>,
    pub default_value: Option<usize>,
    pub enums: Option<Enum>,
}

/// The Enum struct represents an enumeration value for a field.
///
/// It contains the name of the enumeration, the value of the enumeration,
/// a description, and optionally whether the enumeration is the default value.
///
/// Enums are used to represent the possible values of a field. It can be used
/// for single bit fields but most of the time it is not really necessary.
///
/// Most common use cases is when a multi-bit field has a few possible values.
#[derive(Deserialize, Debug)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
}

/// The EnumValues struct represents a single enumeration value for a field.
#[derive(Deserialize, Debug)]
pub struct EnumValue {
    pub name: String,
    #[serde(deserialize_with = "utils::deserialize_num")]
    pub value: usize,
    pub description: String,
    pub is_default: Option<bool>,
}

/// Parses the input JSON string into a PeripheralAccess struct.
///
/// # Arguments
/// input: &PathBuf - The path to the input JSON file.
///
/// # Returns
/// A Result containing the parsed PeripheralAccess struct.
pub fn parse_input(input: &PathBuf) -> Result<PeripheralAccess, serde_json::Error> {
    let input_file = std::fs::File::open(input).unwrap();
    let input_reader = std::io::BufReader::new(input_file);
    serde_json::from_reader(input_reader)
}
