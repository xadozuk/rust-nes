use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Eor;
impl Op for Eor
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = self.operand(mode, registers, memory);
        let result = *registers.a ^ value;

        registers.a.set(result);
        registers.p.update_for_value(result);
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
        let (op, mut r, mut m) = test_op(Eor);

        r.a.set(0b1010_0101);
        m.write(0x0000, 0b0101_1010);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0b1111_1111, *r.a);

        assert!(!r.p.is_zero());
        assert!(r.p.is_negative());
    }

    #[test]
    fn zero()
    {
        let (op, mut r, mut m) = test_op(Eor);

        r.a.set(0xFF);
        m.write(0x0000, 0xFF);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0x00, *r.a);

        assert!(r.p.is_zero());
        assert!(!r.p.is_negative());
    }

}