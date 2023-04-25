use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Inx);
impl Op for Inx
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        let value = registers.x.wrapping_add(1);

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
        let (op, mut r, mut m) = test_op(Inx);

        r.x.set(0x0F);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x10, *r.x);

        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn with_wrapping()
    {
        let (op, mut r, mut m) = test_op(Inx);

        r.x.set(0xFF);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x00, *r.x);

        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn negative()
    {
        let (op, mut r, mut m) = test_op(Inx);

        r.x.set(0xFE);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0xFF, *r.x);

        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }
}