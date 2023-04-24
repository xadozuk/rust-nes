use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Dey;
impl Op for Dey
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        let value = registers.y.wrapping_sub(1);

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
        let (op, mut r, mut m) = test_op(Dey);

        r.y.set(0x0F);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x0E, *r.y);

        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn with_wrapping()
    {
        let (op, mut r, mut m) = test_op(Dey);

        r.y.set( 0x00);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0xFF, *r.y);

        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }

    #[test]
    fn zero()
    {
        let (op, mut r, mut m) = test_op(Dey);

        r.y.set(0x01);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x00, *r.y);

        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }
}