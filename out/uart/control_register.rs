use crate::{
    Read, Write, ReadWrite, BitAccessRead, BitAccessWrite, BitAccessReadWrite,
    FieldAccessRead, FieldAccessWrite, FieldAccessReadWrite,
};
pub struct ControlRegister {
    address: usize,
}
impl Register for ControlRegister {
    fn get_address(&self) -> usize {
        self.address
    }
}
impl Read for ControlRegister {}
impl Write for ControlRegister {}
impl ReadWrite for ControlRegister {}
impl BitAccessRead for ControlRegister {}
impl BitAccessWrite for ControlRegister {}
impl BitAccessReadWrite for ControlRegister {}
impl FieldAccessRead for ControlRegister {}
impl FieldAccessWrite for ControlRegister {}
impl FieldAccessReadWrite for ControlRegister {}
impl ControlRegister {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
    ///Software reset for RX data path
    pub fn get_sw_rx_rst(&self) -> bool {
        self.read_bit(0u32)
    }
    ///Software reset for RX data path
    pub fn set_sw_rx_rst(&mut self, value: bool) {
        self.write_bit(0u32, value);
    }
    ///Software reset for TX data path
    pub fn get_sw_tx_rst(&self) -> bool {
        self.read_bit(1u32)
    }
    ///Software reset for TX data path
    pub fn set_sw_tx_rst(&mut self, value: bool) {
        self.write_bit(1u32, value);
    }
    ///Receiver enable
    pub fn get_rx_en(&self) -> bool {
        self.read_bit(2u32)
    }
    ///Receiver enable
    pub fn set_rx_en(&mut self, value: bool) {
        self.write_bit(2u32, value);
    }
    ///Receiver disable
    pub fn get_rx_dis(&self) -> bool {
        self.read_bit(3u32)
    }
    ///Receiver disable
    pub fn set_rx_dis(&mut self, value: bool) {
        self.write_bit(3u32, value);
    }
    ///Transmitter enable
    pub fn get_tx_en(&self) -> bool {
        self.read_bit(4u32)
    }
    ///Transmitter enable
    pub fn set_tx_en(&mut self, value: bool) {
        self.write_bit(4u32, value);
    }
    ///Transmitter disable
    pub fn get_tx_dis(&self) -> bool {
        self.read_bit(5u32)
    }
    ///Transmitter disable
    pub fn set_tx_dis(&mut self, value: bool) {
        self.write_bit(5u32, value);
    }
    ///Restart RX timeout counter
    pub fn get_restart_rx_timeout_counter(&self) -> bool {
        self.read_bit(6u32)
    }
    ///Restart RX timeout counter
    pub fn set_restart_rx_timeout_counter(&mut self, value: bool) {
        self.write_bit(6u32, value);
    }
    ///Start TX break
    pub fn get_start_tx_break(&self) -> bool {
        self.read_bit(7u32)
    }
    ///Start TX break
    pub fn set_start_tx_break(&mut self, value: bool) {
        self.write_bit(7u32, value);
    }
    ///Stop TX break
    pub fn get_stop_tx_break(&self) -> bool {
        self.read_bit(8u32)
    }
    ///Stop TX break
    pub fn set_stop_tx_break(&mut self, value: bool) {
        self.write_bit(8u32, value);
    }
}
