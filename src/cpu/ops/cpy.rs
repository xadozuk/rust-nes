use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Cpy);
impl Op for Cpy
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = self.operand(mode, registers, memory);
        let result = registers.y.wrapping_sub(value);

        registers.p.set_carry(*registers.y >= value);
        registers.p.update_for_value(result);
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn y_gt_m()
    {
        let (op, mut r, mut m) = test_op(Cpy);

        r.y.set(0x1C);
        m.write(0x0000, 0x0C);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert!(r.p.has_carry());
        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn y_lt_m()
    {
        let (op, mut r, mut m) = test_op(Cpy);

        r.y.set(0x0C);
        m.write(0x0000, 0x1C);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert!(!r.p.has_carry());
        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }

    #[test]
    fn y_eq_m()
    {
        let (op, mut r, mut m) = test_op(Cpy);

        r.y.set(0x1C);
        m.write(0x0000, 0x1C);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert!(r.p.has_carry());
        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }
}