use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Nop;
impl Op for Nop
{
    fn call(&self, _: AddressingMode, _: &mut CpuRegisters, is_zero: &mut Memory)
    {

    }
}