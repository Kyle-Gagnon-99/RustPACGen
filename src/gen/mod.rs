use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use stringcase::Caser;
use utils::{format_token_stream, write_to_file_str};

use crate::{
    config::Config,
    parse::{Peripheral, PeripheralAccess},
};

pub mod utils;

/// Generates the peripheral access crate code using syn and quote.
///
/// # Arguments
/// out_dir: &PathBuf - The output directory where the crate will be generated to.
/// pac: &PeripheralAccess - The parsed peripheral access data structure.
/// config_file: &Config - The configuration file that contains the versions of the dependencies.
pub fn generate_pac_code(
    out_dir: &PathBuf,
    crate_name: &str,
    pac: &PeripheralAccess,
    config_file: &Config,
) {
    // First, make sure the output directory exists. If it doesn't, create it.
    if !out_dir.exists() {
        std::fs::create_dir_all(out_dir).unwrap();
    }

    // Next, create a src directory inside the output directory.
    let src_dir = out_dir.join("src");
    if !src_dir.exists() {
        std::fs::create_dir_all(&src_dir).unwrap();
    }

    // Create the Cargo.toml file in the output directory
    let cargo_toml = gen_cargo_toml(crate_name, config_file);
    let cargo_toml_path = out_dir.join("Cargo.toml");
    write_to_file_str(&cargo_toml_path, &cargo_toml).unwrap();

    // Generate the lib.rs file
    let lib_code = gen_lib_code(pac);
    let lib_code_formatted = format_token_stream(&lib_code);
    let lib_rs_path = src_dir.join("lib.rs");
    write_to_file_str(&lib_rs_path, &lib_code_formatted).unwrap();

    // Generate the peripheral modules
    for peripheral in &pac.peripherals {
        let peripheral_mod_code = gen_peripheral_module(peripheral);
        let peripheral_mod_code_formatted = format_token_stream(&peripheral_mod_code);
        let peripheral_mod_path = src_dir.join(format!("{}.rs", peripheral.name.to_snake_case()));
        write_to_file_str(&peripheral_mod_path, &peripheral_mod_code_formatted).unwrap();
    }
}

/// Generates a Cargo.toml file for the peripheral access crate.
/// Since the Cargo.toml file is not a Rust source file, we will
/// use a String to represent the contents of the file.
///
/// # Arguments
/// crate_name: &str - The name of the crate.
/// config: &Config - The configuration file that contains the versions of the dependencies.
///
/// # Returns
/// The contents of the generated Cargo.toml file.
pub fn gen_cargo_toml(crate_name: &str, config: &Config) -> String {
    format!(
        r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
bit_field = "{bit_field_version}"
volatile_register = "{volatile_register_version}"
        "#,
        crate_name = crate_name,
        bit_field_version = config.bit_field_version,
        volatile_register_version = config.volatile_register_version
    )
}

pub fn gen_lib_code(pac: &PeripheralAccess) -> TokenStream {
    // Create an empty token stream to hold the generated code
    let mut tokens = TokenStream::new();

    // Add a no_std attribute to the top of the file
    let no_std_attr = quote! {
        #![no_std]
    };

    tokens.extend(no_std_attr);

    // Add the volatile_register and bit_field imports
    let imports = quote! {
        pub use bit_field::BitField;
        pub use volatile_register::{RO, WO, RW};
    };

    tokens.extend(imports);

    // Get the list of peripherals from the parsed data to generate module declarations for each peripheral
    for peripheral in &pac.peripherals {
        let peripheral_name = &peripheral.name;
        let peripheral_name_ident = format_ident!("{}", peripheral_name.to_snake_case().trim());

        let peripheral_mod_decl = quote! {
            pub mod #peripheral_name_ident;
        };

        tokens.extend(peripheral_mod_decl);
    }

    // Next generate the traits for the peripheral access crate
    let trait_tokens = quote! {
        pub trait VolatileRegister<RawType: Sized> {
            fn get_inner(&self) -> &VolatileCell<RawType>;
        }

        /// Converts a builder struct into raw bits and vice versa
        ///
        /// RawType: The raw bits type (a primitive integer type)
        pub trait FromBits<RawType: Sized> {
            /// Converts the raw bits into the builder
            fn from_bits(bits: RawType) -> Self;

            /// Converts the builder into the raw bits
            fn into_bits(&self) -> RawType;
        }

        /// A readable register
        pub trait RegisterRO<Builder, RawType>
        where
            RawType: Sized,
            Builder: Copy + Clone + FromBits<RawType>,
        {
            /// Reads the value from the register
            fn read(&self) -> Builder;
        }

        /// A writable register
        pub trait RegisterWO<Builder, RawType>
        where
            RawType: Sized,
            Builder: Copy + Clone + FromBits<RawType> + Default,
        {
            /// Zeroes out the register and returns the builder
            fn zeroed() -> Builder;

            /// Writes the value to the register
            fn write(&mut self, value: Builder);
        }

        /// A readable and writable register
        pub trait RegisterRW<Builder, RawType>:
            RegisterRO<Builder, RawType> + RegisterWO<Builder, RawType>
        where
            RawType: Sized,
            Builder: Copy + Clone + FromBits<RawType> + Default,
        {
            fn modify<F>(&mut self, f: F)
            where
                F: FnOnce(Builder) -> Builder;
        }
    };

    tokens.extend(trait_tokens);

    tokens
}

pub fn gen_peripheral_module(peripheral: &Peripheral) -> TokenStream {
    let mut tokens = TokenStream::new();

    tokens
}
