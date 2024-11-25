//! Peripheral Access Crate for the generated project
//!
//! This crate provides a type-safe API for accessing the peripherals
//! of the microcontroller.
#![no_std]
pub mod uart;
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
        unsafe { core::ptr::read_volatile(addr as *const u32) }
    }
}
/// Trait used for writing to a register
pub trait Write: Register {
    fn write(&mut self, value: u32) {
        let addr = self.get_address();
        debug_assert!(addr % 4 == 0, "Address {:#X} is not aligned to 4 bytes", addr);
        /// Default implementation: write to the memory address
        unsafe { core::ptr::write_volatile(addr as *mut u32, value) }
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
    fn write_field_no_mask(
        &mut self,
        range: core::ops::RangeInclusive<u32>,
        value: u32,
    ) {
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
