use crate::{
    Read, Write, ReadWrite, BitAccessRead, BitAccessWrite, BitAccessReadWrite,
    FieldAccessRead, FieldAccessWrite, FieldAccessReadWrite,
};
pub struct ModemCtrl {
    address: usize,
}
impl Register for ModemCtrl {
    fn get_address(&self) -> usize {
        self.address
    }
}
impl Read for ModemCtrl {}
impl Write for ModemCtrl {}
impl ReadWrite for ModemCtrl {}
impl BitAccessRead for ModemCtrl {}
impl BitAccessWrite for ModemCtrl {}
impl BitAccessReadWrite for ModemCtrl {}
impl FieldAccessRead for ModemCtrl {}
impl FieldAccessWrite for ModemCtrl {}
impl FieldAccessReadWrite for ModemCtrl {}
impl ModemCtrl {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
    ///Automatic flow control mode
    pub fn get_fcm(&self) -> bool {
        self.read_bit(5u32)
    }
    ///Automatic flow control mode
    pub fn set_fcm(&mut self, value: bool) {
        self.write_bit(5u32, value);
    }
    /**Request to send output control: This bit is ignored if automatic flow control mode is enabled by FCM being high. If FCM is low, the value of this bit is inverted when applied to the EMIOUARTxRTSN output.
*/
    pub fn get_rts(&self) -> bool {
        self.read_bit(1u32)
    }
    /**Request to send output control: This bit is ignored if automatic flow control mode is enabled by FCM being high. If FCM is low, the value of this bit is inverted when applied to the EMIOUARTxRTSN output.
*/
    pub fn set_rts(&mut self, value: bool) {
        self.write_bit(1u32, value);
    }
    /**Data Terminal Ready: The value of this bit is inverted when applied to the EMIOUARTxDTRN output.
*/
    pub fn get_dtr(&self) -> bool {
        self.read_bit(0u32)
    }
    /**Data Terminal Ready: The value of this bit is inverted when applied to the EMIOUARTxDTRN output.
*/
    pub fn set_dtr(&mut self, value: bool) {
        self.write_bit(0u32, value);
    }
}
