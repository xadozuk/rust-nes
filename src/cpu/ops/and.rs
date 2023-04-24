use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct And;
impl Op for And
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = self.operand(mode, registers, memory);

        registers.a.set(*registers.a & value);

        registers.p.update_for_value(*registers.a);
    }
}

#[cfg(test)]
mod tests
{
    use crate::cpu::{ops::test_helpers::test_op};

    use super::*;

    #[test]
    fn immediate()
    {
        let (op, mut r, mut m) = test_op(And);

        r.a.set(0b1001_1001);
        m.write(0x0000, 0b1000_0000);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0b1000_0000, *r.a);
        assert!(r.p.is_negative());
        assert!(!r.p.is_zero());
    }
}