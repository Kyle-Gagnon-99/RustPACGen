use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use proc_macro2::TokenStream;

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
