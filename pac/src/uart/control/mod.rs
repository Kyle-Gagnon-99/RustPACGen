use crate::*;
pub struct ControlRegister {
    inner: volatile_register::RW<u32>,
}
#[derive(Debug, Clone, Copy)]
pub struct ControlBuilder {
    value: u32,
}
