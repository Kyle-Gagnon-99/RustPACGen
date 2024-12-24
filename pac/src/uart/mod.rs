pub use volatile_register::{RO, WO, RW};
pub use bit_field::BitField;
pub mod control;
pub mod mode;
#[repr(C)]
pub struct RegisterBlock {
    pub control: control::ControlRegister,
    pub mode: mode::ModeRegister,
    pub ier: volatile_register::RW<u32>,
}
