use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Tay;
impl Op for Tay
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        let value = *registers.a;

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
        let (op, mut r, mut m) = test_op(Tay);

        r.a.set(0x10);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x10, *r.y);

        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn zero()
    {
        let (op, mut r, mut m) = test_op(Tay);

        r.a.set(0x00);
        r.x.set(0xFF);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x00, *r.y);

        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn negative()
    {
        let (op, mut r, mut m) = test_op(Tay);

        r.a.set(0xFF);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0xFF, *r.y);

        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }
}