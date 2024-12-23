use serde::{Deserialize, Deserializer};

/// The parse::utils module contains utility functions for post-processing the parsed data.

/// The BitRange enum represents a range of bits in a register field.
#[derive(Debug)]
pub enum BitRange {
    Single(usize),
    Range(usize, usize),
}

impl BitRange {
    /// Parses a string representation of a bit range into a BitRange enum.
    ///
    /// # Arguments
    /// bit_range: &str - The string representation of the bit range. It is in the format
    ///                   of "[start:end]" or "[bit]".
    pub fn from_string(bit_range: &str) -> BitRange {
        // Remove the brackets from the bit range string
        let bit_range = bit_range.trim_matches(|c| c == '[' || c == ']');

        // Split the bit range string into parts
        let parts: Vec<&str> = bit_range.split(':').collect();

        // Match the number of parts to determine the type of bit range
        match parts.len() {
            1 => BitRange::Single(parts[0].parse().unwrap()),
            2 => BitRange::Range(parts[0].parse().unwrap(), parts[1].parse().unwrap()),
            _ => panic!("Invalid bit range: {}", bit_range),
        }
    }
}

/// Deserializes a bit range from a string representation.
///
/// The bit range is expected to be in the format of "[start:end]" or "[bit]".
///
/// # Arguments
/// deserializer: D - The deserializer for the bit range string.
///
/// # Returns
/// A Result containing the deserialized BitRange enum.
pub fn deserialize_bit_range<'de, D>(deserializer: D) -> Result<BitRange, D::Error>
where
    D: Deserializer<'de>,
{
    let bit_range: String = Deserialize::deserialize(deserializer)?;
    Ok(BitRange::from_string(&bit_range))
}

#[derive(Debug)]
pub enum Access {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    WriteToClear,
}

impl Access {
    /// Parses a string representation of an access type into an Access enum.
    ///
    /// # Arguments
    /// access: &str - The string representation of the access type. It can be one of
    ///
    /// - "read-only"
    /// - "write-only"
    /// - "read-write"
    /// - "write-to-clear"
    ///
    /// # Returns
    /// The Access enum corresponding to the access type.
    pub fn from_string(access: &str) -> Access {
        match access {
            "read-only" => Access::ReadOnly,
            "write-only" => Access::WriteOnly,
            "read-write" => Access::ReadWrite,
            "write-to-clear" => Access::WriteToClear,
            _ => panic!("Invalid access type: {}", access),
        }
    }
}

/// Deserializes an access type from a string representation.
///
/// The access type is expected to be one of the following:
///
/// - "read-only"
/// - "write-only"
/// - "read-write"
/// - "write-to-clear"
///
/// # Arguments
/// deserializer: D - The deserializer for the access type string.
///
/// # Returns
/// A Result containing the deserialized Access enum.
pub fn deserialize_access<'de, D>(deserializer: D) -> Result<Access, D::Error>
where
    D: Deserializer<'de>,
{
    let access: String = Deserialize::deserialize(deserializer)?;
    Ok(Access::from_string(&access))
}

/// Deserializes an access type from a string representation.
///
/// The access type is expected to be one of the following:
///
/// - "read-only"
/// - "write-only"
/// - "read-write"
/// - "write-to-clear"
///
/// # Arguments
/// deserializer: D - The deserializer for the access type string.
///
/// # Returns
/// A Result containing the deserialized Access enum.
pub fn deserialize_access_with_option<'de, D>(deserializer: D) -> Result<Option<Access>, D::Error>
where
    D: Deserializer<'de>,
{
    let access: Option<String> = Deserialize::deserialize(deserializer)?;
    match access {
        Some(access) => Ok(Some(Access::from_string(&access))),
        None => Ok(None),
    }
}

/// Parses a hexadecimal string into a usize.
///
/// # Arguments
/// hex_string: &str - The hexadecimal string to parse.
///
/// # Returns
/// The parsed usize value.
pub fn parse_hex_string(hex_string: &str) -> usize {
    usize::from_str_radix(hex_string.trim_start_matches("0x"), 16).unwrap()
}

/// Parses a binary string into a usize.
///
/// # Arguments
/// bin_string: &str - The binary string to parse.
///
/// # Returns
/// The parsed usize value.
pub fn parse_bin_string(bin_string: &str) -> usize {
    usize::from_str_radix(bin_string.trim_start_matches("0b"), 2).unwrap()
}

/// Parses a decimal string into a usize.
///
/// # Arguments
/// dec_string: &str - The decimal string to parse.
///
/// # Returns
/// The parsed usize value.
pub fn parse_dec_string(dec_string: &str) -> usize {
    dec_string.parse().unwrap()
}

/// Parses a number string into a usize.
///
/// The number string can be in the following formats:
///
/// - Hexadecimal: "0x..."
/// - Binary: "0b..."
/// - Decimal: "..."
///
/// # Arguments
/// num_string: &str - The number string to parse.
///
/// # Returns
/// The parsed usize value.
pub fn parse_num_string(num_string: &str) -> usize {
    if num_string.starts_with("0x") {
        parse_hex_string(num_string)
    } else if num_string.starts_with("0b") {
        parse_bin_string(num_string)
    } else {
        parse_dec_string(num_string)
    }
}

/// Deserializes a number from a string representation.
///
/// The number can be in the following formats:
/// - Hexadecimal: "0x..."
/// - Binary: "0b..."
/// - Decimal: "..."
///
/// # Arguments
/// deserializer: D - The deserializer for the number string.
///
/// # Returns
/// A Result containing the deserialized usize value.
pub fn deserialize_num<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let num_string: String = Deserialize::deserialize(deserializer)?;
    Ok(parse_num_string(&num_string))
}

/// Deserializes a number from a string representation.
///
/// The number can be in the following formats:
/// - Hexadecimal: "0x..."
/// - Binary: "0b..."
/// - Decimal: "..."
///
/// # Arguments
/// deserializer: D - The deserializer for the number string.
///
/// # Returns
/// A Result containing the deserialized usize value.
pub fn deserialize_num_with_option<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let num_string: Option<String> = Deserialize::deserialize(deserializer)?;
    match num_string {
        Some(num_string) => Ok(Some(parse_num_string(&num_string))),
        None => Ok(None),
    }
}
