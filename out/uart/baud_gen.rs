use crate::{
    Read, Write, ReadWrite, BitAccessRead, BitAccessWrite, BitAccessReadWrite,
    FieldAccessRead, FieldAccessWrite, FieldAccessReadWrite,
};
pub struct BaudGen {
    address: usize,
}
impl Register for BaudGen {
    fn get_address(&self) -> usize {
        self.address
    }
}
impl Read for BaudGen {}
impl Write for BaudGen {}
impl ReadWrite for BaudGen {}
impl BitAccessRead for BaudGen {}
impl BitAccessWrite for BaudGen {}
impl BitAccessReadWrite for BaudGen {}
impl FieldAccessRead for BaudGen {}
impl FieldAccessWrite for BaudGen {}
impl FieldAccessReadWrite for BaudGen {}
impl BaudGen {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
    ///Baud rate clock divisor value
    pub fn get_cd(&self) -> u32 {
        self.read_field(0u32..=15u32)
    }
    ///Baud rate clock divisor value
    pub fn set_cd(&mut self, value: u32) {
        self.write_field(0u32..=15u32, value);
    }
}
