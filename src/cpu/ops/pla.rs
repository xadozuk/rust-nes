use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Pla;
impl Op for Pla
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = self.stack_pop(registers, memory);
        registers.a.set(value);

        registers.p.update_for_value(value);
    }
}

#[cfg(test)]
mod tests
{
    use crate::cpu::STACK_START;

    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn simple()
    {
        let (op, mut r, mut m) = test_op(Pla);

        r.a.set(0x00);

        r.sp.set(0x80);
        m.write(STACK_START + 0x81, 0x50);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x50, *r.a);
        assert_eq!(0x81, *r.sp);

        assert!(!r.p.is_negative());
        assert!(!r.p.is_zero());
    }

    #[test]
    fn zero()
    {
        let (op, mut r, mut m) = test_op(Pla);

        r.a.set(0x50);
        r.sp.set(0x80);
        m.write(STACK_START + 0x81, 0x00);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x00, *r.a);

        assert!(!r.p.is_negative());
        assert!(r.p.is_zero());
    }

    #[test]
    fn negative()
    {
        let (op, mut r, mut m) = test_op(Pla);

        r.a.set(0x00);
        r.sp.set(0x80);
        m.write(STACK_START + 0x81, 0xFF);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0xFF, *r.a);

        assert!(r.p.is_negative());
        assert!(!r.p.is_zero());
    }
}