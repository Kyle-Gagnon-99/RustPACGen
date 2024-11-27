use crate::{
    Read, Write, ReadWrite, BitAccessRead, BitAccessWrite, BitAccessReadWrite,
    FieldAccessRead, FieldAccessWrite, FieldAccessReadWrite,
};
pub struct ModemStatus {
    address: usize,
}
impl Register for ModemStatus {
    fn get_address(&self) -> usize {
        self.address
    }
}
impl Read for ModemStatus {}
impl Write for ModemStatus {}
impl ReadWrite for ModemStatus {}
impl BitAccessRead for ModemStatus {}
impl BitAccessWrite for ModemStatus {}
impl BitAccessReadWrite for ModemStatus {}
impl FieldAccessRead for ModemStatus {}
impl FieldAccessWrite for ModemStatus {}
impl FieldAccessReadWrite for ModemStatus {}
impl ModemStatus {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
    ///Flow Control Mode
    pub fn get_fcms(&self) -> bool {
        self.read_bit(8u32)
    }
    ///Flow Control Mode
    pub fn set_fcms(&mut self, value: bool) {
        self.write_bit(8u32, value);
    }
    /**Data Carrier Detect (DCD) input signal from PL (EMIOUARTxDCDN) status: 0: input is high 1: input is low
*/
    pub fn get_dcd(&self) -> bool {
        self.read_bit(7u32)
    }
    /**Ring Indicator (RI) input signal from PL (EMIOUARTxRIN) status: 0: input is high 1: input is low
*/
    pub fn get_ri(&self) -> bool {
        self.read_bit(6u32)
    }
    /**Data Set Ready (DSR) input signal from PL (EMIOUARTxDSRN) status: 0: input is high 1: input is low
*/
    pub fn get_dsr(&self) -> bool {
        self.read_bit(5u32)
    }
    /**Clear to Send (CTS) input signal from PL (EMIOUARTxCTSN) status: 0: input is high 1: input is low
*/
    pub fn get_cts(&self) -> bool {
        self.read_bit(4u32)
    }
    /**Delta Data Carrier Detect status: Indicates a change in state of the EMIOUARTxDCDN input since this bit was last cleared. 0: No change has occurred 1: Change has occurred
*/
    pub fn set_dcdx(&mut self, value: bool) {
        self.write_bit_no_mask(3u32, value);
    }
    /**Trailing Edge Ring Indicator status: Indicates that the EMIOUARTxRIN input has changed from high to low state since this bit was last cleared. 0: No trailing edge has occurred 1: Trailing edge has occurred
*/
    pub fn set_rix(&mut self, value: bool) {
        self.write_bit_no_mask(2u32, value);
    }
    /**Delta Data Set Ready status: Indicates a change in state of the EMIOUARTxDSRN input since this bit was last cleared. 0: No change has occurred 1: Change has occurred
*/
    pub fn set_dsrx(&mut self, value: bool) {
        self.write_bit_no_mask(1u32, value);
    }
    /**Delta Clear To Send status: Indicates a change in state of the EMIOUARTxCTSN input since this bit was last cleared. 0: No change has occurred 1: Change has occurred
*/
    pub fn set_ctsx(&mut self, value: bool) {
        self.write_bit_no_mask(0u32, value);
    }
}
