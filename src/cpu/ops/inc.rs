use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Inc;
impl Op for Inc
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let addr = self.operand_addr(mode, registers, memory);
        let mut value = memory.read(addr);

        value = value.wrapping_add(1);

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
    fn simple()
    {
        let (op, mut r, mut m) = test_op(Inc);

        m.write(0x0000, 0x10);
        m.write(0x0010, 0x0F);

        op.call(AddressingMode::ZeroPage, &mut r, &mut m);

        assert_eq!(0x10, m.read(0x0010));

        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn with_wrapping()
    {
        let (op, mut r, mut m) = test_op(Inc);

        m.write(0x0000, 0x10);
        m.write(0x0010, 0xFF);

        op.call(AddressingMode::ZeroPage, &mut r, &mut m);

        assert_eq!(0x00, m.read(0x0010));

        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn negative()
    {
        let (op, mut r, mut m) = test_op(Inc);

        m.write(0x0000, 0x10);
        m.write(0x0010, 0xFE);

        op.call(AddressingMode::ZeroPage, &mut r, &mut m);

        assert_eq!(0xFF, m.read(0x0010));

        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }
}