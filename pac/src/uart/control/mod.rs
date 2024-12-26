use crate::*;
pub struct ControlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct ControlBuilder {
    value: u32,
}
impl FromBits<u32> for ControlBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<ControlBuilder, u32> for ControlRegister {
    fn read(&self) -> ControlBuilder {
        ControlBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<ControlBuilder, u32> for ControlRegister {
    fn zeroed() -> ControlBuilder {
        ControlBuilder::default()
    }
    fn write(&mut self, value: ControlBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<ControlBuilder, u32> for ControlRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(ControlBuilder) -> ControlBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for ControlBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bit(3usize, true);
        value.set_bit(8usize, true);
        Self { value }
    }
}
impl ControlBuilder {
    pub fn sw_rx_rst(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_sw_rx_rst(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn sw_tx_rst(&self) -> bool {
        self.value.get_bit(1usize)
    }
    pub fn with_sw_tx_rst(mut self, value: bool) -> Self {
        self.value.set_bit(1usize, value);
        self
    }
    pub fn rx_en(&self) -> bool {
        self.value.get_bit(2usize)
    }
    pub fn with_rx_en(mut self, value: bool) -> Self {
        self.value.set_bit(2usize, value);
        self
    }
    pub fn rx_dis(&self) -> bool {
        self.value.get_bit(3usize)
    }
    pub fn with_rx_dis(mut self, value: bool) -> Self {
        self.value.set_bit(3usize, value);
        self
    }
    pub fn tx_en(&self) -> bool {
        self.value.get_bit(4usize)
    }
    pub fn with_tx_en(mut self, value: bool) -> Self {
        self.value.set_bit(4usize, value);
        self
    }
    pub fn tx_dis(&self) -> bool {
        self.value.get_bit(5usize)
    }
    pub fn with_tx_dis(mut self, value: bool) -> Self {
        self.value.set_bit(5usize, value);
        self
    }
    pub fn rst_toc(&self) -> bool {
        self.value.get_bit(6usize)
    }
    pub fn with_rst_toc(mut self, value: bool) -> Self {
        self.value.set_bit(6usize, value);
        self
    }
    pub fn start_tx_break(&self) -> bool {
        self.value.get_bit(7usize)
    }
    pub fn with_start_tx_break(mut self, value: bool) -> Self {
        self.value.set_bit(7usize, value);
        self
    }
    pub fn stop_tx_break(&self) -> bool {
        self.value.get_bit(8usize)
    }
    pub fn with_stop_tx_break(mut self, value: bool) -> Self {
        self.value.set_bit(8usize, value);
        self
    }
}
