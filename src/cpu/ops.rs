#[macro_use]
mod macros;

#[cfg(test)]
mod test_helpers;

mod adc;

use std::collections::HashMap;
use super::{CpuRegisters, Memory};

#[derive(Debug, Clone, Copy)]
pub enum AddressingMode
{
    Immediate,
    ZeroPage,
    Absolute,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY
}

pub type OpcodeMap = HashMap<u8, Opcode>;
pub struct Opcode
{
    pub opcode: u8,
    pub mode:   AddressingMode,
    pub op:     Box<dyn Op>
}

pub trait Op
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory);

    fn len(&self) -> usize;
    fn args_len(&self) -> usize
    {
        self.len() - 1
    }

    fn get_operand_addr(&self, mode: AddressingMode, register: &CpuRegisters, memory: &Memory) -> u16
    {


        match mode
        {
            AddressingMode::Immediate => *register.pc,
            AddressingMode::ZeroPage  => memory.read(*register.pc) as u16,
            AddressingMode::Absolute  => memory.read_u16(*register.pc),

            _ => panic!("Mode {:?} is not supported", mode),
        }
    }
}

pub fn opcodes() -> OpcodeMap
{
    opcodes!(
        (0x69, AddressingMode::Immediate, adc::Adc),
        (0x65, AddressingMode::ZeroPage,  adc::Adc)
    )
}