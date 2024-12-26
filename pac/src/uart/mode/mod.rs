use crate::*;
pub struct ModeRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct ModeBuilder {
    value: u32,
}
impl FromBits<u32> for ModeBuilder {
    fn from_bits(bits: u32) -> Self {
        Self { value: bits }
    }
    fn into_bits(&self) -> u32 {
        self.value
    }
}
impl RegisterRO<ModeBuilder, u32> for ModeRegister {
    fn read(&self) -> ModeBuilder {
        ModeBuilder::from_bits(self.inner.read())
    }
}
impl RegisterWO<ModeBuilder, u32> for ModeRegister {
    fn zeroed() -> ModeBuilder {
        ModeBuilder::default()
    }
    fn write(&mut self, value: ModeBuilder) {
        unsafe {
            self.inner.write(value.into_bits());
        }
    }
}
impl RegisterRW<ModeBuilder, u32> for ModeRegister {
    fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(ModeBuilder) -> ModeBuilder,
    {
        let value = self.read();
        let modified = f(value);
        self.write(modified);
    }
}
impl Default for ModeBuilder {
    fn default() -> Self {
        let mut value: u32 = 0;
        value.set_bits(1usize..=2usize, CharacterLength::EightBits as u32);
        value.set_bits(3usize..=5usize, ParityType::Even as u32);
        value.set_bits(6usize..=7usize, StopBits::One as u32);
        value.set_bits(8usize..=9usize, ChannelMode::Normal as u32);
        Self { value }
    }
}
impl ModeBuilder {
    pub fn clk_sel(&self) -> bool {
        self.value.get_bit(0usize)
    }
    pub fn with_clk_sel(mut self, value: bool) -> Self {
        self.value.set_bit(0usize, value);
        self
    }
    pub fn char_len(&self) -> Option<CharacterLength> {
        CharacterLength::try_from(self.value.get_bits(1usize..=2usize)).ok()
    }
    pub fn with_char_len(mut self, value: CharacterLength) -> Self {
        self.value.set_bits(1usize..=2usize, Into::<u32>::into(value));
        self
    }
    pub fn parity(&self) -> Option<ParityType> {
        ParityType::try_from(self.value.get_bits(3usize..=5usize)).ok()
    }
    pub fn with_parity(mut self, value: ParityType) -> Self {
        self.value.set_bits(3usize..=5usize, Into::<u32>::into(value));
        self
    }
    pub fn num_stop_bits(&self) -> Option<StopBits> {
        StopBits::try_from(self.value.get_bits(6usize..=7usize)).ok()
    }
    pub fn with_num_stop_bits(mut self, value: StopBits) -> Self {
        self.value.set_bits(6usize..=7usize, Into::<u32>::into(value));
        self
    }
    pub fn channel_mode(&self) -> Option<ChannelMode> {
        ChannelMode::try_from(self.value.get_bits(8usize..=9usize)).ok()
    }
    pub fn with_channel_mode(mut self, value: ChannelMode) -> Self {
        self.value.set_bits(8usize..=9usize, Into::<u32>::into(value));
        self
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterLength {
    EightBits = 0usize as u8,
    SevenBits = 2usize as u8,
    SixBits = 3usize as u8,
}
impl Default for CharacterLength {
    fn default() -> Self {
        Self::EightBits
    }
}
impl From<CharacterLength> for u8 {
    fn from(value: CharacterLength) -> Self {
        value as u8
    }
}
impl From<CharacterLength> for u32 {
    fn from(value: CharacterLength) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for CharacterLength {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(CharacterLength::EightBits),
            2usize => Ok(CharacterLength::SevenBits),
            3usize => Ok(CharacterLength::SixBits),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for CharacterLength {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(CharacterLength::EightBits),
            2usize => Ok(CharacterLength::SevenBits),
            3usize => Ok(CharacterLength::SixBits),
            _ => Err(()),
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParityType {
    Even = 0usize as u8,
    Odd = 1usize as u8,
    Space = 2usize as u8,
    Mark = 3usize as u8,
    None = 4usize as u8,
}
impl Default for ParityType {
    fn default() -> Self {
        Self::Even
    }
}
impl From<ParityType> for u8 {
    fn from(value: ParityType) -> Self {
        value as u8
    }
}
impl From<ParityType> for u32 {
    fn from(value: ParityType) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for ParityType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(ParityType::Even),
            1usize => Ok(ParityType::Odd),
            2usize => Ok(ParityType::Space),
            3usize => Ok(ParityType::Mark),
            4usize => Ok(ParityType::None),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for ParityType {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(ParityType::Even),
            1usize => Ok(ParityType::Odd),
            2usize => Ok(ParityType::Space),
            3usize => Ok(ParityType::Mark),
            4usize => Ok(ParityType::None),
            _ => Err(()),
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    One = 0usize as u8,
    OneAndHalf = 1usize as u8,
    Two = 2usize as u8,
}
impl Default for StopBits {
    fn default() -> Self {
        Self::One
    }
}
impl From<StopBits> for u8 {
    fn from(value: StopBits) -> Self {
        value as u8
    }
}
impl From<StopBits> for u32 {
    fn from(value: StopBits) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for StopBits {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(StopBits::One),
            1usize => Ok(StopBits::OneAndHalf),
            2usize => Ok(StopBits::Two),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for StopBits {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(StopBits::One),
            1usize => Ok(StopBits::OneAndHalf),
            2usize => Ok(StopBits::Two),
            _ => Err(()),
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelMode {
    Normal = 0usize as u8,
    AutoEcho = 1usize as u8,
    LocalLoop = 2usize as u8,
    RemoteLoop = 3usize as u8,
}
impl Default for ChannelMode {
    fn default() -> Self {
        Self::Normal
    }
}
impl From<ChannelMode> for u8 {
    fn from(value: ChannelMode) -> Self {
        value as u8
    }
}
impl From<ChannelMode> for u32 {
    fn from(value: ChannelMode) -> Self {
        value as u32
    }
}
impl TryFrom<u8> for ChannelMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(ChannelMode::Normal),
            1usize => Ok(ChannelMode::AutoEcho),
            2usize => Ok(ChannelMode::LocalLoop),
            3usize => Ok(ChannelMode::RemoteLoop),
            _ => Err(()),
        }
    }
}
impl TryFrom<u32> for ChannelMode {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value as usize {
            0usize => Ok(ChannelMode::Normal),
            1usize => Ok(ChannelMode::AutoEcho),
            2usize => Ok(ChannelMode::LocalLoop),
            3usize => Ok(ChannelMode::RemoteLoop),
            _ => Err(()),
        }
    }
}
