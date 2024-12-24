use std::path::PathBuf;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use stringcase::Caser;
use utils::{
    calculate_reserved_region, create_folder, format_token_stream, get_smallest_rust_type,
    write_to_file_str,
};

use crate::{
    config::Config,
    parse::{
        utils::{Access, BitRange},
        Enum, Peripheral, PeripheralAccess, Register,
    },
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
    let lib_code = gen_lib_code(pac, pac.is_no_std);
    let lib_code_formatted = format_token_stream(&lib_code);
    let lib_rs_path = src_dir.join("lib.rs");
    write_to_file_str(&lib_rs_path, &lib_code_formatted).unwrap();

    // Generate the peripheral modules
    for peripheral in &pac.peripherals {
        gen_peripherals(peripheral, &src_dir);
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
volatile-register = "{volatile_register_version}"
        "#,
        crate_name = crate_name,
        bit_field_version = config.bit_field_version,
        volatile_register_version = config.volatile_register_version
    )
}

pub fn gen_lib_code(pac: &PeripheralAccess, is_no_std: Option<bool>) -> TokenStream {
    // Create an empty token stream to hold the generated code
    let mut tokens = TokenStream::new();

    // Add a no_std attribute to the top of the file
    let no_std_attr = if is_no_std.unwrap_or(true) {
        quote! {
            #![no_std]
        }
    } else {
        quote! {}
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

/// Generates the code for the peripherals in the peripheral access crate.
/// This function will generate a module for each peripheral and a module in the peripheral module for each register.
///
/// # Arguments
/// peripheral: &Peripheral - The peripheral to generate code for.
/// src_dir: &PathBuf - The path to the src directory where the code will be generated.
pub fn gen_peripherals(peripheral: &Peripheral, src_dir: &PathBuf) {
    // First, create a folder for the peripheral module
    let peripheral_name = &peripheral.name;
    let peripheral_name_snake = peripheral_name.to_snake_case();
    create_folder(&src_dir.join(&peripheral_name_snake)).unwrap();

    // Create a folder for each register module
    for register in &peripheral.registers {
        if register.fields.is_some() {
            let register_name = &register.name;
            let register_name_snake = register_name.to_snake_case();
            create_folder(
                &src_dir
                    .join(&peripheral_name_snake)
                    .join(&register_name_snake),
            )
            .unwrap();
        }
    }

    // Generate the peripheral module
    let peripheral_mod = gen_peripheral_module(peripheral);
    let peripheral_mod_formatted = format_token_stream(&peripheral_mod);
    let peripheral_mod_path = src_dir.join(&peripheral_name_snake).join("mod.rs");
    write_to_file_str(&peripheral_mod_path, &peripheral_mod_formatted).unwrap();

    // Next, generate the register modules (for those with fields)
    for register in &peripheral.registers {
        if register.fields.is_some() {
            gen_register_module(register, &src_dir.join(&peripheral_name_snake));
        }
    }
}

/// Generates the top level module for a peripheral.
///
/// # Arguments
/// peripheral: &Peripheral - The peripheral to generate the module for.
///
/// # Returns
/// A TokenStream representing the generated module.
pub fn gen_peripheral_module(peripheral: &Peripheral) -> TokenStream {
    // The top level module includes the RegisterBlock and declarations for the register modules

    // First, get the register module declarations
    // A module is only generated for a register if it has at least one field
    let register_mod_decls = peripheral.registers.iter().map(|register| {
        if register.fields.is_some() {
            let register_name = &register.name;
            let register_name_ident = format_ident!("{}", register_name.to_snake_case().trim());

            quote! {
                pub mod #register_name_ident;
            }
        } else {
            quote! {}
        }
    });

    let mut register_block_fields = quote! {};
    let mut previous_offset = 0;

    for (i, register) in peripheral.registers.iter().enumerate() {
        let offset = register.offset;

        // Add reserved fields if there's a gap
        let reserved_count = calculate_reserved_region(previous_offset, offset, None);
        if reserved_count > 0 {
            let reserved_field = format_ident!("_reserved{}", i);
            register_block_fields = quote! {
                #register_block_fields
                pub #reserved_field: [u32; #reserved_count],
            };
        }

        let reg_name = format_ident!("{}", register.name.to_lowercase());
        let reg_type = if register.fields.is_some() {
            let reg_module_name = format_ident!("{}", register.name.to_lowercase());
            let reg_struct_name =
                format_ident!("{}Register", register.name.to_pascal_case().trim());
            quote! { #reg_module_name::#reg_struct_name }
        } else {
            match &register.access {
                Access::ReadOnly => quote! { volatile_register::RO<u32> },
                Access::WriteOnly => quote! { volatile_register::WO<u32> },
                Access::ReadWrite | Access::WriteToClear => quote! { volatile_register::RW<u32> },
            }
        };

        register_block_fields = quote! {
            #register_block_fields
            pub #reg_name: #reg_type,
        };

        previous_offset = offset + (register.size / 8);
    }

    // Next, we need to generate a token stream for the imports
    // We need to import the volatile_register crate and the register modules
    let imports = quote! {
        pub use volatile_register::{RO, WO, RW};
        pub use bit_field::BitField;
    };

    // Return the token stream
    quote! {
        #imports

        #(#register_mod_decls)*

        #[repr(C)]
        pub struct RegisterBlock {
            #register_block_fields
        }
    }
}

/// Generates the register module for a register.
pub fn gen_register_module(register: &Register, module_file_path: &PathBuf) {
    //
    let register_name = format_ident!("{}", register.name.to_snake_case().trim());
    let register_struct_name = format_ident!("{}Register", register.name.to_pascal_case().trim());
    let register_builder_name = format_ident!("{}Builder", register.name.to_pascal_case().trim());

    // Unwrap is safe here because we know that the fields are Some
    let fields = register.fields.as_ref().unwrap();

    // For now, for each register, generate the enums of the fields
    // Collect the enums into a single token stream
    let enums = fields
        .iter()
        .filter_map(|field| field.enums.as_ref().map(|e| (e, &field.bit_range)))
        .map(|(enum_def, bit_range)| generate_enums(enum_def, bit_range))
        .collect::<Vec<TokenStream>>();

    // Get the Rust type that can hold the register size
    let register_size = register.size;
    let register_type = get_smallest_rust_type(register_size);

    let register_module = quote! {
        use crate::*;

        pub struct #register_struct_name {
            inner: volatile_register::RW<#register_type>,
        }

        #[derive(Debug, Clone, Copy)]
        pub struct #register_builder_name {
            value: #register_type,
        }

        #(#enums)*
    };

    // For now, safe off the register_module generation
    let register_module_formatted = format_token_stream(&register_module);
    let register_module_path = module_file_path
        .join(&register_name.to_string())
        .join("mod.rs");
    write_to_file_str(&register_module_path, &register_module_formatted).unwrap();
}

fn generate_enums(enum_def: &Enum, bit_range: &BitRange) -> TokenStream {
    // Get the rust type that can hold the enum size
    let enum_size = match bit_range {
        BitRange::Single(bit) => *bit,
        BitRange::Range(end, start) => end - start + 1,
    };
    let enum_type = get_smallest_rust_type(enum_size);
    let enum_name = format_ident!("{}", enum_def.name.to_pascal_case().trim());
    let mut variants = quote! {};
    let mut default_variant = None;

    for (_i, value) in enum_def.values.iter().enumerate() {
        let variant_name = format_ident!("{}", value.name.to_pascal_case().trim());
        let variant_value = value.value as isize;
        variants = quote! {
            #variants
            #variant_name = #variant_value,
        };

        if value.is_default.unwrap_or(false) || default_variant.is_none() {
            default_variant = Some(variant_name.clone());
        }
    }

    let default_variant = default_variant.expect("Enum must have at least one variant");

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #enum_name {
            #variants
        }

        impl Default for #enum_name {
            fn default() -> Self {
                Self::#default_variant
            }
        }

        impl From<#enum_name> for #enum_type {
            fn from(value: #enum_name) -> Self {
                value as #enum_type
            }
        }

        impl TryFrom<#enum_type> for #enum_name {
            type Error = ();

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    #(Self::#variants as u8 => Ok(Self::#variants)),*,
                    _ => Err(()),
                }
            }
        }
    }
}
