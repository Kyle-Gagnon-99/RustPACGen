use log::debug;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use stringcase::Caser;
use syn::Ident;

use crate::{
    config::{Field, Peripheral, Register},
    utils::StringOrNumber,
};

pub fn write_file(file_path: &str, content: &str) {
    // Make sure the directory exists, if not create it
    let parent_dir = std::path::Path::new(file_path).parent().unwrap();

    if !parent_dir.exists() {
        std::fs::create_dir_all(parent_dir).unwrap_or_else(|_| {
            panic!("Failed to create directory: {}", parent_dir.display());
        });
    }

    // Write the content to the file
    match std::fs::write(file_path, content) {
        Ok(_) => {
            debug!("Successfully wrote to file: {}", file_path);
        }
        Err(e) => {
            panic!("Failed to write to file: {}\n{}", file_path, e);
        }
    }
}

pub fn generate_crate(peripherals: &Vec<Peripheral>, output_dir: &str) {
    // Create the output directory if it does not exist
    std::fs::create_dir_all(output_dir).unwrap_or_else(|_| {
        panic!("Failed to create output directory: {}", output_dir);
    });

    // Generate the top level of the crate
    let top_level = generate_top_level(peripherals);

    // Write the top level to the lib.rs file
    let lib_file_path = format!("{}/lib.rs", output_dir);

    write_file(&lib_file_path, &top_level);

    // Generate the peripheral modules
    for peripheral in peripherals {
        let peripheral_content = generate_peripheral(&peripheral);
        let peripheral_name = peripheral.id.to_lowercase();
        let peripheral_file_path = format!("{}/{}/mod.rs", output_dir, peripheral_name);

        write_file(&peripheral_file_path, &peripheral_content);

        // For each register, generate the register module
        for register in &peripheral.registers {
            let register_content = generate_register(register);
            let register_name = register.id.to_lowercase();
            let register_file_path =
                format!("{}/{}/{}.rs", output_dir, peripheral_name, register_name);

            write_file(&register_file_path, &register_content);
        }
    }
}

/// Generates the top level of the crate. It will generate the lib.rs file that will
/// contain the declarations of the peripheral modules. It will also generate traits
/// used by the peripheral modules.
pub fn generate_top_level(peripherals: &Vec<Peripheral>) -> String {
    // First, collect all the peripheral names and make them identifiers
    let peripheal_names: Vec<Ident> = peripherals
        .iter()
        .map(|peripheral| {
            let name = peripheral.id.to_lowercase();
            format_ident!("{}", name)
        })
        .collect();

    // Generate the lib.rs file
    // For now, just generate the file with a comment block
    // and the peripheral module declarations
    let gen_content = quote! {
        //! Peripheral Access Crate for the generated project
        //!
        //! This crate provides a type-safe API for accessing the peripherals
        //! of the microcontroller.
        #![no_std]
        #(
            pub mod #peripheal_names;
        )*

        /// Trait for a register within a peripheral
        pub trait Register {
            fn get_address(&self) -> usize;
        }

        /// Trait used for reading from a register
        pub trait Read: Register {
            fn read(&self) -> u32 {
                let addr = self.get_address();
                debug_assert!(addr % 4 == 0, "Address {:#X} is not aligned to 4 bytes", addr);
                /// Default implementation: read from the memory address
                unsafe { core::ptr::read_volatile(addr as *const u32)}
            }
        }

        /// Trait used for writing to a register
        pub trait Write: Register {
            fn write(&mut self, value: u32) {
                let addr = self.get_address();
                debug_assert!(addr % 4 == 0, "Address {:#X} is not aligned to 4 bytes", addr);
                /// Default implementation: write to the memory address
                unsafe { core::ptr::write_volatile(addr as *mut u32, value)}
            }
        }

        /// Trait used to read and write to a register
        pub trait ReadWrite: Read + Write {
            fn modify<F>(&mut self, f: F)
            where
                F: FnOnce(&mut u32) -> u32,
            {
                let value = self.read();
                let modified_value = f(value);
                self.write(modified_value);
            }
        }

        /// Trait is used for reading a single bit from a Read-Only register
        pub trait BitAccessRead: Read {
            /// Reads a single bit from the register
            ///
            /// # Arguments
            /// bit: u32 - The bit to read
            ///
            /// # Returns
            /// bool - The value of the bit
            fn read_bit(&self, bit: u32) -> bool {
                (self.read() >> bit) & 1 == 1
            }
        }

        /// Trait is used for writing a single bit to a Write-Only register
        pub trait BitAccessWrite: Write {
            /// Writes a single bit without a mask to the register since it is Write-Only
            ///
            /// # Arguments
            /// bit: u32 - The bit to write to
            /// value: bool - The value to write to the bit
            fn write_bit_no_mask(&mut self, bit: u32, value: bool) {
                let mask = 1 << bit;
                let value = if value { mask } else { 0 };
                self.write(value as u32);
            }
        }

        /// Trait is used for reading and writing a single bit to a Read-Write register
        pub trait BitAccessReadWrite: BitAccessRead + BitAccessWrite {
            /// Modifies a single bit in the register
            ///
            /// # Arguments
            /// bit: u32 - The bit to modify
            /// f: F - The closure that modifies the bit
            fn modify_bit<F>(&mut self, bit: u32, f: F)
            where
                F: FnOnce(bool) -> bool,
            {
                let value = self.read_bit(bit);
                let modified_value = f(value);
                self.write_bit_no_mask(bit, modified_value);
            }

            /// Writes a single bit to the register. This will make sure that only the bit
            /// is modified and not the other bits in the register as it is a Read-Write register.
            ///
            /// # Arguments
            /// bit: u32 - The bit to write to
            /// value: bool - The value to write to the bit
            fn write_bit(&mut self, bit: u32, value: bool) {
                self.modify_bit(bit, |_| value);
            }
        }

        /// Trait used for accessing a field within a register
        pub trait FieldAccessRead: Read {
            /// Reads a field from the register
            ///
            /// # Arguments
            /// range: core::ops::RangeInclusive<u32> - The range of bits that make up the field
            ///
            /// # Returns
            /// u32 - The value of the field
            fn read_field(&self, range: core::ops::RangeInclusive<u32>) -> u32 {
                let value = self.read();
                let mask = ((1 << (range.end() - range.start() + 1)) - 1) << range.start();
                (value & mask) >> range.start()
            }
        }

        /// Trait used for writing to a field within a register
        /// This trait is for Write-Only registers
        pub trait FieldAccessWrite: Write {
            fn write_field_no_mask(&mut self, range: core::ops::RangeInclusive<u32>, value: u32) {
                let mask = ((1 << (range.end() - range.start() + 1)) - 1) << range.start();
                let value_shifted = (value << range.start()) & mask;
                self.write(value_shifted);
            }
        }

        /// Trait used for reading and writing to a field within a register
        /// This trait is for Read-Write registers
        pub trait FieldAccessReadWrite: FieldAccessRead + FieldAccessWrite {
            fn modify_field<F>(&mut self, range: core::ops::RangeInclusive<u32>, f: F)
            where
                F: FnOnce(u32) -> u32,
            {
                let current = self.read_field(range);
                let modified = f(current);
                self.write_field_no_mask(range, modified);
            }

            /// Writes a value to a field, applying the necessary mask.
            ///
            /// # Arguments
            /// - `range`: The range of bits for the field.
            /// - `value`: The value to write to the field.
            fn write_field(&mut self, range: core::ops::RangeInclusive<u32>, value: u32) {
                let mask = ((1 << (range.end() - range.start() + 1)) - 1) << range.start();
                let value_shifted = (value << range.start()) & mask;
                self.write((self.read() & !mask) | value_shifted);
            }
        }

        /// Trait used for resetting a register to its default value
        /// It only applies to registers that can be written to
        pub trait Resettable: Write {
            /// Returns the reset value of the register
            fn reset_value() -> u32;

            /// Resets the register to its default value
            /// This is the same as writing the reset value to the register
            /// The default implementation is to write the reset value to the register
            fn reset(&mut self) {
                self.write(Self::reset_value());
            }
        }
    };

    //debug!("Generated Content: #{:?}", &gen_content.to_string());
    let syn_tree = syn::parse2(gen_content).unwrap();

    prettyplease::unparse(&syn_tree)

    // Return the generated lib.rs file
}

pub fn generate_peripheral(peripheral: &Peripheral) -> String {
    let peripheral_name = format_ident!("{}", peripheral.id.to_camel_case());

    let register_initializers: Vec<TokenStream> = peripheral
        .registers
        .iter()
        .map(|reg| {
            let reg_name_struct = format_ident!("{}", reg.id.to_pascal_case());
            let reg_name_member = format_ident!("{}", reg.id.to_snake_case());
            let offset = reg.offset;

            quote! {
                #reg_name_member: #reg_name_struct::new(base_address + #offset),
            }
        })
        .collect();

    let register_mod_names: Vec<TokenStream> = peripheral
        .registers
        .iter()
        .map(|reg| {
            let reg_name = format_ident!("{}", reg.id.to_snake_case());
            quote! {
                pub mod #reg_name;
            }
        })
        .collect();

    let register_use_mod_names: Vec<TokenStream> = peripheral
        .registers
        .iter()
        .map(|reg| {
            let reg_name = format_ident!("{}", reg.id.to_snake_case());
            quote! {
                pub use #reg_name::#reg_name;
            }
        })
        .collect();

    //debug!("TokenStream: {:#?}", register);
    let gen_content = quote! {
        #(#register_mod_names)*

        #(#register_use_mod_names)*

        pub struct #peripheral_name {
            address: usize,
        }

        impl #peripheral_name {
            pub fn new(base_address: usize) -> Self {
                Self {
                    address: base_address,
                    #(#register_initializers)*
                }
            }
        }
    };
    debug!("Generated Content: #{:?}", &gen_content.to_string());
    let syn_tree = syn::parse2(gen_content).unwrap();

    prettyplease::unparse(&syn_tree)
}

pub fn generate_register(register: &Register) -> String {
    let register_ident = format_ident!("{}", register.id.to_pascal_case());
    let fields: Vec<TokenStream> = register
        .fields
        .as_ref()
        .unwrap()
        .iter()
        .map(|field| generate_field(field))
        .collect();

    let enum_fields: Vec<TokenStream> = register
        .fields
        .as_ref()
        .unwrap()
        .iter()
        .filter(|field| field.values.is_some() && !is_single_bit(&field.bit_range))
        .map(|field| generate_enum(field))
        .collect();

    let reg_access = match register.access.as_str() {
        "R" => quote! {
            impl Read for #register_ident {}
            impl BitAccessRead for #register_ident {}
            impl FieldAccessRead for #register_ident {}
        },
        "W" => quote! {
            impl Write for #register_ident {}
            impl BitAccessWrite for #register_ident {}
            impl FieldAccessWrite for #register_ident {}
        },
        "RW" => quote! {
            impl Read for #register_ident {}
            impl Write for #register_ident {}
            impl ReadWrite for #register_ident {}
            impl BitAccessRead for #register_ident {}
            impl BitAccessWrite for #register_ident {}
            impl BitAccessReadWrite for #register_ident {}
            impl FieldAccessRead for #register_ident {}
            impl FieldAccessWrite for #register_ident {}
            impl FieldAccessReadWrite for #register_ident {}
        },
        _ => quote! {},
    };

    let use_crate_traits = match register.access.as_str() {
        "R" => quote! {
            use crate::{Read, BitAccessRead, FieldAccessRead};
        },
        "W" => quote! {
            use crate::{Write, BitAccessWrite, FieldAccessWrite};
        },
        "RW" => quote! {
            use crate::{Read, Write, ReadWrite, BitAccessRead, BitAccessWrite, BitAccessReadWrite, FieldAccessRead, FieldAccessWrite, FieldAccessReadWrite};
        },
        _ => quote! {},
    };

    let gen_content = quote! {
        #use_crate_traits

        pub struct #register_ident {
            address: usize,
        }

        impl Register for #register_ident {
            fn get_address(&self) -> usize {
                self.address
            }
        }

        #reg_access

        impl #register_ident {
            pub fn new(address: usize) -> Self {
                Self { address }
            }

            #(#fields)*
        }

        #(#enum_fields)*
    };

    debug!("Generated Content: #{:?}", &gen_content.to_string());
    let syn_tree = syn::parse2(gen_content).unwrap();
    prettyplease::unparse(&syn_tree)
}

pub fn generate_field(field: &Field) -> TokenStream {
    let field_name = format_ident!("{}", field.id.to_snake_case());
    let bit_range = match parse_bit_range(&field.bit_range) {
        BitRange::Single(bit) => quote! {#bit},
        BitRange::Range(start, end) => quote! {#start..=#end},
    };
    let description = field.description.clone();
    let get_fn_name = format_ident!("get_{}", field_name);
    let set_fn_name = format_ident!("set_{}", field_name);

    // To generate the methods, we need to know the access type of the field
    // and if the field is a single bit or a multi-bit field
    // The values key can still be present in single bit fields as they are used to describe
    // what true and false mean for the bit
    // If the values key is not present, we assume that it does not have specific values
    // (e.g. the clock divisor for UART can be between 0 and 255)
    let access = field.access.as_str();
    let values: &Vec<crate::config::Values> = if field.values.is_some() {
        field.values.as_ref().unwrap()
    } else {
        &Vec::new()
    };

    // The generate_register function will implement the correct traits for the register
    // based on the access type of the register
    // For R (Read-Only) registers, we implement the Register, Read, BitAccessRead, and FieldAccessRead traits
    // For W (Write-Only) registers, we implement the Register, Write, BitAccessWrite, and FieldAccessWrite traits
    // For RW (Read-Write) registers, we implement the Register, Read, Write, ReadWrite, BitAccessRead, BitAccessWrite,
    // BitAccessReadWrite, FieldAccessRead, FieldAccessWrite, and FieldAccessReadWrite traits
    // The access will tell us if we need to generate the get and set methods
    // The implementation of the set method will vary slightly. Since Write-Only registers can not read from the register,
    // when we call the specific field's set method, it will only set the field and reset the other bits to 0 (since we can't read the register)
    // The function for Write-Only is called write_bit_no_mask and write_field_no_mask
    // For Read-Write registers, the set method will use the write_bit and write_field methods
    let is_single_bit = is_single_bit(&field.bit_range);
    let has_enum_values = values.len() > 0 && !is_single_bit;
    let field_type = if is_single_bit {
        quote! {bool}
    } else {
        if has_enum_values {
            let enum_name = format_ident!("{}", field.id.to_pascal_case());
            quote! {#enum_name}
        } else {
            quote! {u32}
        }
    };

    let get_fn = if access == "R" || access == "RW" {
        if is_single_bit {
            quote! {
                #[doc = #description]
                pub fn #get_fn_name(&self) -> bool {
                    self.read_bit(#bit_range)
                }
            }
        } else {
            quote! {
                #[doc = #description]
                pub fn #get_fn_name(&self) -> #field_type {
                    self.read_field(#bit_range)
                }
            }
        }
    } else {
        quote! {}
    };

    let set_fn = if access == "W" {
        if is_single_bit {
            quote! {
                #[doc = #description]
                pub fn #set_fn_name(&mut self, value: bool) {
                    self.write_bit_no_mask(#bit_range, value);
                }
            }
        } else {
            quote! {
                #[doc = #description]
                pub fn #set_fn_name(&mut self, value: #field_type) {
                    self.write_field_no_mask(#bit_range, value);
                }
            }
        }
    } else if access == "RW" {
        if is_single_bit {
            quote! {
                #[doc = #description]
                pub fn #set_fn_name(&mut self, value: bool) {
                    self.write_bit(#bit_range, value);
                }
            }
        } else {
            quote! {
                #[doc = #description]
                pub fn #set_fn_name(&mut self, value: #field_type) {
                    self.write_field(#bit_range, value);
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #get_fn

        #set_fn
    }
}

pub fn generate_enum(field: &Field) -> TokenStream {
    let enum_name = format_ident!("{}", field.id.to_pascal_case());
    let variants: Vec<TokenStream> = field
        .values
        .as_ref()
        .unwrap()
        .iter()
        .map(|value| {
            let variant_name = format_ident!("{}", value.name.as_ref().unwrap().to_pascal_case());
            let description = value.description.clone().unwrap_or_default();

            // The value could either be a string or a number
            // If it is a number, we take it as is (cause it is probably a binary)
            // If it is a string, we need to convert it to some number
            let value = enum_val_to_number(&value.value);

            quote! {
                #[doc = #description]
                #variant_name = #value,
            }
        })
        .collect();

    let wildcard_match: Vec<TokenStream> = field
        .values
        .as_ref()
        .unwrap()
        .iter()
        .filter(|value| {
            let enum_value = &enum_value_to_string(&value.value);
            enum_value.contains('X')
        })
        .map(|value| {
            let enum_value = &enum_value_to_string(&value.value);
            let variant_name = format_ident!("{}", value.name.as_ref().unwrap().to_pascal_case());
            let mask = compute_mask(enum_value);
            let value = computer_value(enum_value);
            quote! {
                x if (x & #mask) == #value => #enum_name::#variant_name,
            }
        })
        .collect();

    let variant_to_values: Vec<TokenStream> = field
        .values
        .as_ref()
        .unwrap()
        .iter()
        .map(|value| {
            let variant_name = format_ident!("{}", value.name.as_ref().unwrap().to_pascal_case());
            let value = enum_val_to_number(&value.value);
            quote! {
                #enum_name::#variant_name => #value,
            }
        })
        .collect();

    // Match the value to the enum variant
    // If it is a wildcard, it is handled separately so skip it
    let value_to_variant: Vec<TokenStream> = field
        .values
        .as_ref()
        .unwrap()
        .iter()
        .filter(|value| {
            let enum_value = &enum_value_to_string(&value.value);
            !enum_value.contains('X')
        })
        .map(|value| {
            let variant_name = format_ident!("{}", value.name.as_ref().unwrap().to_pascal_case());
            let value = enum_val_to_number(&value.value);
            quote! {
               #value => #enum_name::#variant_name,
            }
        })
        .collect();

    quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum #enum_name {
            #(#variants)*
        }

        impl From<u32> for #enum_name {
            fn from(value: u32) -> Self {
                match value {
                    #(#value_to_variant)*
                    #(#wildcard_match),*
                    _ => panic!("Invalid value for enum #enum_name: {:#X}", value),
                }
            }
        }

        impl Into<u32> for #enum_name {
            fn into(self) -> u32 {
                match self {
                    #(#variant_to_values)*
                }
            }
        }
    }
}

fn compute_mask(wildcard: &str) -> u32 {
    let bits = wildcard.replace('X', "1").replace("0b", "");
    u32::from_str_radix(&bits, 2).unwrap()
}

fn computer_value(wildcard: &str) -> u32 {
    let bits = wildcard.replace('X', "0").replace("0b", "");
    u32::from_str_radix(&bits, 2).unwrap()
}

fn enum_val_to_number(value: &StringOrNumber) -> u32 {
    match value {
        StringOrNumber::String(s) => {
            if s.starts_with("0b") {
                compute_mask(s)
            } else {
                s.parse::<u32>().unwrap()
            }
        }
        StringOrNumber::Number(n) => n.clone() as u32,
    }
}

fn enum_value_to_string(value: &StringOrNumber) -> String {
    match value {
        StringOrNumber::String(s) => s.clone(),
        StringOrNumber::Number(n) => n.to_string(),
    }
}

pub enum BitRange {
    Single(u32),
    Range(u32, u32),
}

pub fn parse_bit_range(bit_range: &str) -> BitRange {
    let parts = bit_range.split(":").collect::<Vec<&str>>();
    match parts.len() {
        1 => BitRange::Single(parts[0].parse().unwrap()),
        2 => BitRange::Range(parts[1].parse().unwrap(), parts[0].parse().unwrap()),
        _ => panic!("Invalid bit range: {}", bit_range),
    }
}

pub fn is_single_bit(bit_range: &str) -> bool {
    match parse_bit_range(bit_range) {
        BitRange::Single(_) => true,
        _ => false,
    }
}
