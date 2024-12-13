use crate::{
    Read, Write, ReadWrite, BitAccessRead, BitAccessWrite, BitAccessReadWrite,
    FieldAccessRead, FieldAccessWrite, FieldAccessReadWrite,
};
pub struct ModeRegister {
    address: usize,
}
impl Register for ModeRegister {
    fn get_address(&self) -> usize {
        self.address
    }
}
impl Read for ModeRegister {}
impl Write for ModeRegister {}
impl ReadWrite for ModeRegister {}
impl BitAccessRead for ModeRegister {}
impl BitAccessWrite for ModeRegister {}
impl BitAccessReadWrite for ModeRegister {}
impl FieldAccessRead for ModeRegister {}
impl FieldAccessWrite for ModeRegister {}
impl FieldAccessReadWrite for ModeRegister {}
impl ModeRegister {
    pub fn new(address: usize) -> Self {
        Self { address }
    }
    ///Clock source select: This field defines whether a pre-scalar of 8 is applied to the baud rate generator input clock
    pub fn get_clk_src_sel(&self) -> bool {
        self.read_bit(0u32)
    }
    ///Clock source select: This field defines whether a pre-scalar of 8 is applied to the baud rate generator input clock
    pub fn set_clk_src_sel(&mut self, value: bool) {
        self.write_bit(0u32, value);
    }
    ///Character length select: Defines the number of bits in each character
    pub fn get_char_len(&self) -> CharLen {
        self.read_field(1u32..=2u32)
    }
    ///Character length select: Defines the number of bits in each character
    pub fn set_char_len(&mut self, value: CharLen) {
        self.write_field(1u32..=2u32, value);
    }
    ///Parity type select: Defines the expected parity to check on receive and the parity to generate on transmit
    pub fn get_parity(&self) -> Parity {
        self.read_field(3u32..=5u32)
    }
    ///Parity type select: Defines the expected parity to check on receive and the parity to generate on transmit
    pub fn set_parity(&mut self, value: Parity) {
        self.write_field(3u32..=5u32, value);
    }
    ///Number of stop bits: Defines the number of stop bits to detect on receive and to generate on transmit
    pub fn get_stop_bits(&self) -> StopBits {
        self.read_field(6u32..=7u32)
    }
    ///Number of stop bits: Defines the number of stop bits to detect on receive and to generate on transmit
    pub fn set_stop_bits(&mut self, value: StopBits) {
        self.write_field(6u32..=7u32, value);
    }
    ///Channel mode: Defines the mode of operation of the UART
    pub fn get_channel_mode(&self) -> ChannelMode {
        self.read_field(8u32..=9u32)
    }
    ///Channel mode: Defines the mode of operation of the UART
    pub fn set_channel_mode(&mut self, value: ChannelMode) {
        self.write_field(8u32..=9u32, value);
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CharLen {
    ///8 bits
    EightBits = 1u32,
    ///7 bits
    SevenBits = 2u32,
    ///6 bits
    SixBits = 3u32,
}
impl From<u32> for CharLen {
    fn from(value: u32) -> Self {
        match value {
            2u32 => CharLen::SevenBits,
            3u32 => CharLen::SixBits,
            x if (x & 1u32) == 0u32 => CharLen::EightBits,
            _ => panic!("Invalid value for enum #enum_name: {:#X}", value),
        }
    }
}
impl Into<u32> for CharLen {
    fn into(self) -> u32 {
        match self {
            CharLen::EightBits => 1u32,
            CharLen::SevenBits => 2u32,
            CharLen::SixBits => 3u32,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Parity {
    ///No parity
    NoParity = 7u32,
    ///Forced to 1 parity (mark)
    MarkParity = 3u32,
    ///Forced to 0 parity (space)
    SpaceParity = 2u32,
    ///Odd parity
    OddParity = 1u32,
    ///Even parity
    EvenParity = 0u32,
}
impl From<u32> for Parity {
    fn from(value: u32) -> Self {
        match value {
            3u32 => Parity::MarkParity,
            2u32 => Parity::SpaceParity,
            1u32 => Parity::OddParity,
            0u32 => Parity::EvenParity,
            x if (x & 7u32) == 4u32 => Parity::NoParity,
            _ => panic!("Invalid value for enum #enum_name: {:#X}", value),
        }
    }
}
impl Into<u32> for Parity {
    fn into(self) -> u32 {
        match self {
            Parity::NoParity => 7u32,
            Parity::MarkParity => 3u32,
            Parity::SpaceParity => 2u32,
            Parity::OddParity => 1u32,
            Parity::EvenParity => 0u32,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StopBits {
    ///1 stop bit
    OneStopBit = 0u32,
    ///1.5 stop bits
    OnePointFiveStopBits = 1u32,
    ///2 stop bits
    TwoStopBits = 2u32,
}
impl From<u32> for StopBits {
    fn from(value: u32) -> Self {
        match value {
            0u32 => StopBits::OneStopBit,
            1u32 => StopBits::OnePointFiveStopBits,
            2u32 => StopBits::TwoStopBits,
            _ => panic!("Invalid value for enum #enum_name: {:#X}", value),
        }
    }
}
impl Into<u32> for StopBits {
    fn into(self) -> u32 {
        match self {
            StopBits::OneStopBit => 0u32,
            StopBits::OnePointFiveStopBits => 1u32,
            StopBits::TwoStopBits => 2u32,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChannelMode {
    ///Normal mode
    NormalMode = 0u32,
    ///Automatic echo
    AutoEcho = 1u32,
    ///Local loopback
    LocalLoopback = 2u32,
    ///Remote loopback
    RemoteLoopback = 3u32,
}
impl From<u32> for ChannelMode {
    fn from(value: u32) -> Self {
        match value {
            0u32 => ChannelMode::NormalMode,
            1u32 => ChannelMode::AutoEcho,
            2u32 => ChannelMode::LocalLoopback,
            3u32 => ChannelMode::RemoteLoopback,
            _ => panic!("Invalid value for enum #enum_name: {:#X}", value),
        }
    }
}
impl Into<u32> for ChannelMode {
    fn into(self) -> u32 {
        match self {
            ChannelMode::NormalMode => 0u32,
            ChannelMode::AutoEcho => 1u32,
            ChannelMode::LocalLoopback => 2u32,
            ChannelMode::RemoteLoopback => 3u32,
        }
    }
}
