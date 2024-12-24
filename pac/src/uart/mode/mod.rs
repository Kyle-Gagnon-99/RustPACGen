use crate::*;
pub struct ModeRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct ModeBuilder {
    value: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterLength {
    EightBits = 0isize,
    SevenBits = 2isize,
    SixBits = 3isize,
}
impl Default for CharacterLength {
    fn default() -> Self {
        Self::SixBits
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParityType {
    Even = 0isize,
    Odd = 1isize,
    Space = 2isize,
    Mark = 3isize,
    None = 4isize,
}
impl Default for ParityType {
    fn default() -> Self {
        Self::Even
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    One = 0isize,
    OneAndHalf = 1isize,
    Two = 2isize,
}
impl Default for StopBits {
    fn default() -> Self {
        Self::One
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelMode {
    Normal = 0isize,
    AutoEcho = 1isize,
    LocalLoop = 2isize,
    RemoteLoop = 3isize,
}
impl Default for ChannelMode {
    fn default() -> Self {
        Self::Normal
    }
}
