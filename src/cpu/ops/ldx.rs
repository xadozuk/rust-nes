use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Ldx;
impl Op for Ldx
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = self.operand(mode, registers, memory);

        registers.x.set(value);
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
        let (op, mut r, mut m) = test_op(Ldx);

        m.write(0x0000, 0x10);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0x10, *r.x);
        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn zero()
    {
        let (op, mut r, mut m) = test_op(Ldx);

        m.write(0x0000, 0x00);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0x00, *r.x);
        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn negative()
    {
        let (op, mut r, mut m) = test_op(Ldx);

        m.write(0x0000, 0xFF);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0xFF, *r.x);
        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }
}