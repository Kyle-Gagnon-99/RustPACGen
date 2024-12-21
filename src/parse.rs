use serde::Deserialize;

/// The parse module contains logic and data structures for parsing the input JSON file
/// using the serde library.
///

#[derive(Deserialize, Debug)]
pub struct PeripheralAccess {
    pub name: String,
    pub peripherals: Vec<Peripheral>,
}

#[derive(Deserialize, Debug)]
pub struct Peripheral {
    pub name: String,
    pub base_address: String,
    pub registers: Vec<Register>,
}

#[derive(Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub offset: String,
    pub size: usize,
    pub access: String,
}
