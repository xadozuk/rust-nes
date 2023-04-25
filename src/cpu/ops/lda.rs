use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Lda);
impl Op for Lda
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = self.operand(mode, registers, memory);

        registers.a.set(value);
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
        let (op, mut r, mut m) = test_op(Lda);

        m.write(0x0000, 0x10);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0x10, *r.a);
        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn zero()
    {
        let (op, mut r, mut m) = test_op(Lda);

        m.write(0x0000, 0x00);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0x00, *r.a);
        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn negative()
    {
        let (op, mut r, mut m) = test_op(Lda);

        m.write(0x0000, 0xFF);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0xFF, *r.a);
        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }
}