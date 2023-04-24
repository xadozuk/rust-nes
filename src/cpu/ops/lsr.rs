use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Lsr;
impl Op for Lsr
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = self.operand(mode, registers, memory);

        let result = value >> 1;

        if let AddressingMode::Accumulator = mode
        {
            registers.a.set(result);
        }
        else
        {
            memory.write(
                self.operand_addr(mode, registers, memory),
                result
            );
        }

        registers.p.update_for_value(result);
        registers.p.set_carry(value & 0b0000_0001 != 0);
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn accumulator()
    {
        let (op, mut r, mut m) = test_op(Lsr);

        r.a.set(0b0000_0010);

        op.call(AddressingMode::Accumulator, &mut r, &mut m);

        assert_eq!(0b0000_0001, *r.a);
        assert!(!r.p.is_negative());
        assert!(!r.p.is_zero());
        assert!(!r.p.has_carry());
    }

    #[test]
    fn accumulator_zero()
    {
        let (op, mut r, mut m) = test_op(Lsr);

        r.a.set(0b0000_0001);

        op.call(AddressingMode::Accumulator, &mut r, &mut m);

        assert_eq!(0b0000_0000, *r.a);
        assert!(!r.p.is_negative());
        assert!(r.p.is_zero());
        assert!(r.p.has_carry());
    }
}