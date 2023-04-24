use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Rol;
impl Op for Rol
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let value = match mode
        {
            AddressingMode::Accumulator => *registers.a,
            _ => self.operand(mode, registers, memory)
        };

        let mut rotated_value = value << 1;

        if registers.p.has_carry()
        {
            rotated_value |= 0x01; // 0b0000_0001;
        }

        match mode
        {
            AddressingMode::Accumulator => registers.a.set(rotated_value),
            _ => memory.write(self.operand_addr(mode, registers, memory), rotated_value)
        }

        registers.p.set_carry(value & 0b1000_0000 != 0);
        registers.p.update_for_value(rotated_value);

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
        let (op, mut r, mut m) = test_op(Rol);

        r.a.set(0b0101_0101);

        op.call(AddressingMode::Accumulator, &mut r, &mut m);

        assert_eq!(0b1010_1010, *r.a);

        assert!(r.p.is_negative());
        assert!(!r.p.is_zero());
        assert!(!r.p.has_carry());
    }

    #[test]
    fn zero()
    {
        let (op, mut r, mut m) = test_op(Rol);

        r.a.set(0x00);

        op.call(AddressingMode::Accumulator, &mut r, &mut m);

        assert_eq!(0x00, *r.a);

        assert!(!r.p.is_negative());
        assert!(r.p.is_zero());
        assert!(!r.p.has_carry());
    }

    #[test]
    fn will_carry()
    {
        let (op, mut r, mut m) = test_op(Rol);

        r.a.set(0b1000_0001);

        op.call(AddressingMode::Accumulator, &mut r, &mut m);

        assert_eq!(0b0000_0010, *r.a);

        assert!(!r.p.is_negative());
        assert!(!r.p.is_zero());
        assert!(r.p.has_carry());
    }

    #[test]
    fn with_previous_carry()
    {
        let (op, mut r, mut m) = test_op(Rol);

        r.p.set_carry(true);
        r.a.set(0b0000_1000);

        op.call(AddressingMode::Accumulator, &mut r, &mut m);

        assert_eq!(0b0001_0001, *r.a);

        assert!(!r.p.is_negative());
        assert!(!r.p.is_zero());
        assert!(!r.p.has_carry());
    }
}