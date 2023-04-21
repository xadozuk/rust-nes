use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Bcc;
impl Op for Bcc
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        if !registers.p.has_carry()
        {
            registers.pc.set(self.operand_addr(mode, registers, memory));
        }
    }
}

pub struct Bcs;
impl Op for Bcs
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        if registers.p.has_carry()
        {
            registers.pc.set(self.operand_addr(mode, registers, memory));
        }
    }
}

pub struct Beq;
impl Op for Beq
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        if registers.p.is_zero()
        {
            registers.pc.set(self.operand_addr(mode, registers, memory));
        }
    }
}

pub struct Bmi;
impl Op for Bmi
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        if registers.p.is_negative()
        {
            registers.pc.set(self.operand_addr(mode, registers, memory));
        }
    }
}

pub struct Bne;
impl Op for Bne
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        if !registers.p.is_zero()
        {
            registers.pc.set(self.operand_addr(mode, registers, memory));
        }
    }
}

pub struct Bpl;
impl Op for Bpl
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        if !registers.p.is_negative()
        {
            registers.pc.set(self.operand_addr(mode, registers, memory));
        }
    }
}

pub struct Bvc;
impl Op for Bvc
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        if !registers.p.has_overflown()
        {
            registers.pc.set(self.operand_addr(mode, registers, memory));
        }
    }
}

pub struct Bvs;
impl Op for Bvs
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        if registers.p.has_overflown()
        {
            registers.pc.set(self.operand_addr(mode, registers, memory));
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use super::*;

    //TODO: impl
}