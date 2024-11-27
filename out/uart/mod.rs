pub mod control_register;
pub mod mode_register;
pub mod interrupt_enable;
pub mod interrupt_disable;
pub mod interrupt_mask;
pub mod interrupt_status;
pub mod baud_gen;
pub mod rx_timeout;
pub mod rx_fifo_trigger_level;
pub mod modem_ctrl;
pub mod modem_status;
pub mod channel_status;
pub use control_register::control_register;
pub use mode_register::mode_register;
pub use interrupt_enable::interrupt_enable;
pub use interrupt_disable::interrupt_disable;
pub use interrupt_mask::interrupt_mask;
pub use interrupt_status::interrupt_status;
pub use baud_gen::baud_gen;
pub use rx_timeout::rx_timeout;
pub use rx_fifo_trigger_level::rx_fifo_trigger_level;
pub use modem_ctrl::modem_ctrl;
pub use modem_status::modem_status;
pub use channel_status::channel_status;
pub struct uart {
    address: usize,
}
impl uart {
    pub fn new(base_address: usize) -> Self {
        Self {
            address: base_address,
            control_register: ControlRegister::new(base_address + 0usize),
            mode_register: ModeRegister::new(base_address + 4usize),
            interrupt_enable: InterruptEnable::new(base_address + 8usize),
            interrupt_disable: InterruptDisable::new(base_address + 12usize),
            interrupt_mask: InterruptMask::new(base_address + 16usize),
            interrupt_status: InterruptStatus::new(base_address + 20usize),
            baud_gen: BaudGen::new(base_address + 24usize),
            rx_timeout: RxTimeout::new(base_address + 28usize),
            rx_fifo_trigger_level: RxFifoTriggerLevel::new(base_address + 32usize),
            modem_ctrl: ModemCtrl::new(base_address + 36usize),
            modem_status: ModemStatus::new(base_address + 40usize),
            channel_status: ChannelStatus::new(base_address + 44usize),
        }
    }
}
