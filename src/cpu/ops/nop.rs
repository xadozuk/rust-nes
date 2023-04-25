use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Nop);
impl Op for Nop
{
    fn call(&self, _: AddressingMode, _: &mut CpuRegisters, _: &mut Memory)
    {

    }
}