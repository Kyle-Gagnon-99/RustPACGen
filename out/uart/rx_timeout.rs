use crate::{
    Read, Write, ReadWrite, BitAccessRead, BitAccessWrite, BitAccessReadWrite,
    FieldAccessRead, FieldAccessWrite, FieldAccessReadWrite,
};
pub struct RxTimeout {
    address: usize,
}
impl Register for RxTimeout {
    fn get_address(&self) -> usize {
        self.address
    }
}
impl Read for RxTimeout {}
impl Write for RxTimeout {}
impl ReadWrite for RxTimeout {}
impl BitAccessRead for RxTimeout {}
impl BitAccessWrite for RxTimeout {}
impl BitAccessReadWrite for RxTimeout {}
impl FieldAccessRead for RxTimeout {}
impl FieldAccessWrite for RxTimeout {}
impl FieldAccessReadWrite for RxTimeout {}
impl RxTimeout {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
    ///Receiver timeout value
    pub fn get_rto(&self) -> u32 {
        self.read_field(0u32..=7u32)
    }
    ///Receiver timeout value
    pub fn set_rto(&mut self, value: u32) {
        self.write_field(0u32..=7u32, value);
    }
}
