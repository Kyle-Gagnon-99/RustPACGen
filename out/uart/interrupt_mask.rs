use crate::{Read, BitAccessRead, FieldAccessRead};
pub struct InterruptMask {
    address: usize,
}
impl Register for InterruptMask {
    fn get_address(&self) -> usize {
        self.address
    }
}
impl Read for InterruptMask {}
impl BitAccessRead for InterruptMask {}
impl FieldAccessRead for InterruptMask {}
impl InterruptMask {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
    ///Transmitter FIFO Overflow interrupt status
    pub fn get_tovr(&self) -> bool {
        self.read_bit(12u32)
    }
    ///Transmitter FIFO Nearly Full interrupt mask status
    pub fn get_tnful(&self) -> bool {
        self.read_bit(11u32)
    }
    ///Transmitter FIFO Trigger interrupt mask status
    pub fn get_ttrig(&self) -> bool {
        self.read_bit(10u32)
    }
    ///Delta Modem Status Indicator interrupt mask status
    pub fn get_ixr_dms(&self) -> bool {
        self.read_bit(9u32)
    }
    ///Receiver Timeout Error interrupt mask status
    pub fn get_ixr_tout(&self) -> bool {
        self.read_bit(8u32)
    }
    ///Receiver Parity Error interrupt mask status
    pub fn get_ixr_parity(&self) -> bool {
        self.read_bit(7u32)
    }
    ///Receiver Framing Error interrupt mask status
    pub fn get_ixr_framing(&self) -> bool {
        self.read_bit(6u32)
    }
    ///Receiver Overflow Error interrupt mask status
    pub fn get_ixr_over(&self) -> bool {
        self.read_bit(5u32)
    }
    ///Transmitter FIFO Full interrupt mask status
    pub fn get_ixr_txfull(&self) -> bool {
        self.read_bit(4u32)
    }
    ///Transmitter FIFO Empty interrupt mask status
    pub fn get_ixr_txempty(&self) -> bool {
        self.read_bit(3u32)
    }
    ///Receiver FIFO Full interrupt mask status
    pub fn get_ixr_rxfull(&self) -> bool {
        self.read_bit(2u32)
    }
    ///Receiver FIFO Empty interrupt mask status
    pub fn get_ixr_rxempty(&self) -> bool {
        self.read_bit(1u32)
    }
    ///Receiver FIFO Trigger interrupt mask status
    pub fn get_ixr_rxovr(&self) -> bool {
        self.read_bit(0u32)
    }
}
