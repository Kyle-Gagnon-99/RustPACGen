use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use proc_macro2::TokenStream;
use quote::quote;

/// Creates a folder at a given path.
/// If the folder already exists, it does nothing.
///
/// # Arguments
/// folder_path: &PathBuf - The path to the folder to create.
///
/// # Returns
/// A Result containing the success or error of the folder creation operation.
pub fn create_folder(folder_path: &PathBuf) -> std::io::Result<()> {
    if !folder_path.exists() {
        std::fs::create_dir_all(folder_path)?;
    }
    Ok(())
}

/// Writes a given string to a given file path
///
/// # Arguments
/// file_path: &str - The path to the file to write to.
/// content: &str - The content to write to the file.
///
/// # Returns
/// A Result containing the success or error of the file write operation.
pub fn write_to_file_str(file_path: &PathBuf, content: &str) -> std::io::Result<()> {
    let path = Path::new(file_path);
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())
}

/// Formats a TokenStream into a formatted Rust code string.
///
/// # Arguments
/// token_stream: TokenStream - The TokenStream to format.
///
/// # Returns
/// A formatted Rust code string.
pub fn format_token_stream(token_stream: &TokenStream) -> String {
    prettyplease::unparse(&syn::parse_file(token_stream.to_string().as_str()).unwrap())
}

/// Calculates the number of reserved register regions between two offsets.
///
/// # Arguments
/// previous_offset: usize - The offset of the previous register.
/// current_offset: usize - The offset of the current register.
/// register_size: Option<usize> - The size of the register. If None, the size is assumed to be 32 bits (4 bytes).
///
/// # Returns
/// The number of reserved register regions between the two offsets.
pub fn calculate_reserved_region(
    previous_offset: usize,
    current_offset: usize,
    register_size: Option<usize>,
) -> usize {
    if current_offset > previous_offset {
        (current_offset - previous_offset) / register_size.unwrap_or(4)
    } else {
        0
    }
}

/// Gets the smallest Rust type that can hold a given size in bits.
///
/// # Arguments
/// size: usize - The size in bits.
///
/// # Returns
/// A TokenStream representing the smallest Rust type that can hold the given size.
pub fn get_smallest_rust_type(size: usize) -> TokenStream {
    match size {
        1..=8 => quote! { u8 },
        9..=16 => quote! { u16 },
        17..=32 => quote! { u32 },
        33..=64 => quote! { u64 },
        65..=128 => quote! { u128 },
        _ => quote! { u128 },
    }
}

/// Converts a usize to a boolean.
///
/// # Arguments
/// value: usize - The value to convert to a boolean.
///
/// # Returns
/// The boolean value of the usize. True if the value is not 0, false otherwise.
pub fn usize_to_bool(value: usize) -> bool {
    value != 0
}
