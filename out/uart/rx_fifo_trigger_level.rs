use crate::{
    Read, Write, ReadWrite, BitAccessRead, BitAccessWrite, BitAccessReadWrite,
    FieldAccessRead, FieldAccessWrite, FieldAccessReadWrite,
};
pub struct RxFifoTriggerLevel {
    address: usize,
}
impl Register for RxFifoTriggerLevel {
    fn get_address(&self) -> usize {
        self.address
    }
}
impl Read for RxFifoTriggerLevel {}
impl Write for RxFifoTriggerLevel {}
impl ReadWrite for RxFifoTriggerLevel {}
impl BitAccessRead for RxFifoTriggerLevel {}
impl BitAccessWrite for RxFifoTriggerLevel {}
impl BitAccessReadWrite for RxFifoTriggerLevel {}
impl FieldAccessRead for RxFifoTriggerLevel {}
impl FieldAccessWrite for RxFifoTriggerLevel {}
impl FieldAccessReadWrite for RxFifoTriggerLevel {}
impl RxFifoTriggerLevel {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
    ///Receiver FIFO trigger level value
    pub fn get_rx_trig(&self) -> u32 {
        self.read_field(0u32..=5u32)
    }
    ///Receiver FIFO trigger level value
    pub fn set_rx_trig(&mut self, value: u32) {
        self.write_field(0u32..=5u32, value);
    }
}
