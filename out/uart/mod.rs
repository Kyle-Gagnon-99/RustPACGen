//! Peripheral: #peripheral_name
//! Description: #peripheral.description
//! Registers:
//! #(#register_mod_names)*
pub mod control_register;
pub mod mode_register;
pub mod baud_gen;
pub use control_register::control_register;
pub use mode_register::mode_register;
pub use baud_gen::baud_gen;
pub struct uart {
    address: usize,
}
impl uart {
    pub fn new(base_address: usize) -> Self {
        Self {
            address: base_address,
            control_register: ControlRegister::new(base_address + 0usize),
            mode_register: ModeRegister::new(base_address + 4usize),
            baud_gen: BaudGen::new(base_address + 24usize),
        }
    }
}
