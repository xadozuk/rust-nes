use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Bit;
impl Op for Bit
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = self.operand(mode, registers, memory);

        registers.p.set_zero(*registers.a & value == 0);
        registers.p.set_overflow(value & 0b0100_0000 != 0);
        registers.p.set_negative(value & 0b1000_0000 != 0);
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use super::*;

    //TODO: impl
}