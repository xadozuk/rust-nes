use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Iny);
impl Op for Iny
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        let value = registers.y.wrapping_add(1);

        registers.y.set(value);
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
        let (op, mut r, mut m) = test_op(Iny);

        r.y.set(0x0F);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x10, *r.y);

        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn with_wrapping()
    {
        let (op, mut r, mut m) = test_op(Iny);

        r.y.set(0xFF);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x00, *r.y);

        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn negative()
    {
        let (op, mut r, mut m) = test_op(Iny);

        r.y.set(0xFE);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0xFF, *r.y);

        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }
}