use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Dec;
impl Op for Dec
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let addr = self.operand_addr(mode, registers, memory);
        let mut value = memory.read(addr);

        value = value.wrapping_sub(1);

        memory.write(addr, value);
        registers.p.update_for_value(value);
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn dec()
    {
        let (op, mut r, mut m) = test_op(Dec);

        m.write(0x0000, 0x10);
        m.write(0x0010, 0x0F);

        op.call(AddressingMode::ZeroPage, &mut r, &mut m);

        assert_eq!(0x0E, m.read(0x0010));

        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn dec_with_wrapping()
    {
        let (op, mut r, mut m) = test_op(Dec);

        m.write(0x0000, 0x10);
        m.write(0x0010, 0x00);

        op.call(AddressingMode::ZeroPage, &mut r, &mut m);

        assert_eq!(0xFF, m.read(0x0010));

        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }

    #[test]
    fn dec_zero()
    {
        let (op, mut r, mut m) = test_op(Dec);

        m.write(0x0000, 0x10);
        m.write(0x0010, 0x01);

        op.call(AddressingMode::ZeroPage, &mut r, &mut m);

        assert_eq!(0x00, m.read(0x0010));

        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }
}