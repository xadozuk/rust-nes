use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Bit;
impl Op for Bit
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = self.operand(mode, registers, memory);

        registers.p.set_zero(*registers.a & value == 0);
        registers.p.set_overflow(value & 0b0100_0000 != 0);
        registers.p.set_negative(value & 0b1000_0000 != 0);
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn set_zero()
    {
        let (op, mut r, mut m) = test_op(Bit);

        r.a.set(0b1000_0000);
        m.write_u16(0x0000, 0x1000);
        m.write(0x1000, 0b0000_0000);

        op.call(AddressingMode::Absolute, &mut r, &mut m);

        assert!(r.p.is_zero());
        assert!(!r.p.has_overflown());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn not_zero_but_overflow_and_negative()
    {
        let (op, mut r, mut m) = test_op(Bit);

        r.a.set(0b1000_0000);
        m.write_u16(0x0000, 0x1000);
        m.write(0x1000, 0b1100_0000);

        op.call(AddressingMode::Absolute, &mut r, &mut m);

        assert!(!r.p.is_zero());
        assert!(r.p.has_overflown());
        assert!(r.p.is_negative());
    }

}