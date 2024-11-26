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
}
