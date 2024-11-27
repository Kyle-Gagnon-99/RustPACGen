use crate::{Write, BitAccessWrite, FieldAccessWrite};
pub struct InterruptStatus {
    address: usize,
}
impl Register for InterruptStatus {
    fn get_address(&self) -> usize {
        self.address
    }
}
impl Write for InterruptStatus {}
impl BitAccessWrite for InterruptStatus {}
impl FieldAccessWrite for InterruptStatus {}
impl InterruptStatus {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
    ///Transmitter FIFO Overflow interrupt mask status
    pub fn set_tovr(&mut self, value: bool) {
        self.write_bit_no_mask(12u32, value);
    }
    ///Transmitter FIFO Nearly Full interrupt mask status
    pub fn set_tnful(&mut self, value: bool) {
        self.write_bit_no_mask(11u32, value);
    }
    ///Transmitter FIFO Trigger interrupt mask status
    pub fn set_ttrig(&mut self, value: bool) {
        self.write_bit_no_mask(10u32, value);
    }
    ///Delta Modem Status Indicator interrupt mask status
    pub fn set_ixr_dms(&mut self, value: bool) {
        self.write_bit_no_mask(9u32, value);
    }
    ///Receiver Timeout Error interrupt mask status
    pub fn set_ixr_tout(&mut self, value: bool) {
        self.write_bit_no_mask(8u32, value);
    }
    ///Receiver Parity Error interrupt mask status
    pub fn set_ixr_parity(&mut self, value: bool) {
        self.write_bit_no_mask(7u32, value);
    }
    ///Receiver Framing Error interrupt mask status
    pub fn set_ixr_framing(&mut self, value: bool) {
        self.write_bit_no_mask(6u32, value);
    }
    ///Receiver Overflow Error interrupt mask status
    pub fn set_ixr_over(&mut self, value: bool) {
        self.write_bit_no_mask(5u32, value);
    }
    ///Transmitter FIFO Full interrupt mask status
    pub fn set_ixr_txfull(&mut self, value: bool) {
        self.write_bit_no_mask(4u32, value);
    }
    ///Transmitter FIFO Empty interrupt mask status
    pub fn set_ixr_txempty(&mut self, value: bool) {
        self.write_bit_no_mask(3u32, value);
    }
    ///Receiver FIFO Full interrupt mask status
    pub fn set_ixr_rxfull(&mut self, value: bool) {
        self.write_bit_no_mask(2u32, value);
    }
    ///Receiver FIFO Empty interrupt mask status
    pub fn set_ixr_rxempty(&mut self, value: bool) {
        self.write_bit_no_mask(1u32, value);
    }
    ///Receiver FIFO Trigger interrupt mask status
    pub fn set_ixr_rxovr(&mut self, value: bool) {
        self.write_bit_no_mask(0u32, value);
    }
}
