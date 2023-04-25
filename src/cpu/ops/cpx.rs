use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Cpx);
impl Op for Cpx
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = self.operand(mode, registers, memory);
        let result = registers.x.wrapping_sub(value);

        registers.p.set_carry(*registers.x >= value);
        registers.p.update_for_value(result);
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn x_gt_m()
    {
        let (op, mut r, mut m) = test_op(Cpx);

        r.x.set(0x1C);
        m.write(0x0000, 0x0C);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert!(r.p.has_carry());
        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn x_lt_m()
    {
        let (op, mut r, mut m) = test_op(Cpx);

        r.x.set(0x0C);
        m.write(0x0000, 0x1C);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert!(!r.p.has_carry());
        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }

    #[test]
    fn x_eq_m()
    {
        let (op, mut r, mut m) = test_op(Cpx);

        r.x.set(0x1C);
        m.write(0x0000, 0x1C);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert!(r.p.has_carry());
        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }
}