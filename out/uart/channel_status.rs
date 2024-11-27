use crate::{Read, BitAccessRead, FieldAccessRead};
pub struct ChannelStatus {
    address: usize,
}
impl Register for ChannelStatus {
    fn get_address(&self) -> usize {
        self.address
    }
}
impl Read for ChannelStatus {}
impl BitAccessRead for ChannelStatus {}
impl FieldAccessRead for ChannelStatus {}
impl ChannelStatus {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
    /**Transmitter FIFO Nearly Full continuous status: This indicates that there is not enough space for the number of bytes currently specified in the WSIZE bits in the Mode register. If a write were currently attempted it would cause an overflow. 0: More than one byte is unused in the Tx FIFO 1: Only one byte is free in the Tx FIFO
*/
    pub fn get_tnful(&self) -> bool {
        self.read_bit(14u32)
    }
    /**Transmitter FIFO Trigger continuous status: 0: Tx FIFO fill level is less than TTRIG 1: Tx FIFO fill level is greater than or equal to TTRIG
*/
    pub fn get_ttrig(&self) -> bool {
        self.read_bit(13u32)
    }
    /**Receiver flow delay trigger continuous status: 0: Rx FIFO fill level is less than FDEL 1: Rx FIFO fill level is greater than or equal to FDEL
*/
    pub fn get_flowdel(&self) -> bool {
        self.read_bit(12u32)
    }
    /**Transmitter state machine active status: 0: Inactive state 1: Active state
*/
    pub fn get_tactive(&self) -> bool {
        self.read_bit(11u32)
    }
    /**Receiver state machine active status: 0: Inactive state 1: Active state
*/
    pub fn get_ractive(&self) -> bool {
        self.read_bit(10u32)
    }
    /**Transmitter FIFO Full continuous status: 0: Tx FIFO is not full 1: Tx FIFO is full
*/
    pub fn get_txfull(&self) -> bool {
        self.read_bit(4u32)
    }
    /**Transmitter FIFO Empty continuous status: 0: Tx FIFO is not empty 1: Tx FIFO is empty
*/
    pub fn get_txempty(&self) -> bool {
        self.read_bit(3u32)
    }
    /**Receiver FIFO Full continuous status: 0: Rx FIFO is not full 1: Rx FIFO is full
*/
    pub fn get_rxfull(&self) -> bool {
        self.read_bit(2u32)
    }
    /**Receiver FIFO Empty continuous status: 0: Rx FIFO is not empty 1: Rx FIFO is empty
*/
    pub fn get_rxempty(&self) -> bool {
        self.read_bit(1u32)
    }
    ///Receiver FIFO Trigger continuous status: 0: Rx FIFO fill level is less than RTRIG 1: Rx FIFO fill level is greater than or equal to RTRIG
    pub fn get_rxovr(&self) -> bool {
        self.read_bit(0u32)
    }
}
