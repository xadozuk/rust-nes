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

    fn branch_taken(op: impl Op + 'static, setup: impl Fn(&mut CpuRegisters)) -> bool
    {
        let (op, mut r, mut m) = test_op(op);

        setup(&mut r);

        m.write(0x0000, 0x10);
        op.call(AddressingMode::Relative, &mut r, &mut m);

        0x11 == *r.pc
    }

    #[test]
    fn bcc()
    {
        assert!(branch_taken(Bcc, |r| r.p.set_carry(false)));
        assert!(!branch_taken(Bcc, |r| r.p.set_carry(true)));
    }

    #[test]
    fn bcs()
    {
        assert!(branch_taken(Bcs, |r| r.p.set_carry(true)));
        assert!(!branch_taken(Bcs, |r| r.p.set_carry(false)));
    }

    #[test]
    fn beq()
    {
        assert!(branch_taken(Beq, |r| r.p.set_zero(true)));
        assert!(!branch_taken(Beq, |r| r.p.set_zero(false)));
    }

    #[test]
    fn bmi()
    {
        assert!(branch_taken(Bmi, |r| r.p.set_negative(true)));
        assert!(!branch_taken(Bmi, |r| r.p.set_negative(false)));
    }

    #[test]
    fn bne()
    {
        assert!(branch_taken(Bne, |r| r.p.set_zero(false)));
        assert!(!branch_taken(Bne, |r| r.p.set_zero(true)));
    }

    #[test]
    fn bpl()
    {
        assert!(branch_taken(Bpl, |r| r.p.set_negative(false)));
        assert!(!branch_taken(Bpl, |r| r.p.set_negative(true)));
    }

    #[test]
    fn bvc()
    {
        assert!(branch_taken(Bvc, |r| r.p.set_overflow(false)));
        assert!(!branch_taken(Bvc, |r| r.p.set_overflow(true)));
    }

    #[test]
    fn bvs()
    {
        assert!(branch_taken(Bvs, |r| r.p.set_overflow(true)));
        assert!(!branch_taken(Bvs, |r| r.p.set_overflow(false)));
    }

}