use crate::cpu::memory::Memory;
use crate::cpu::CpuRegisters;

use super::{Op, AddressingMode};

op!(Sbc);
impl Op for Sbc
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        // SBC:
        // A - M - (1-C)
        // A - M - (1-C) + 256
        // A + (255-M) + C

        let value = self.operand(mode, registers, memory);
        let one_compl_value = value ^ 0xFF; // Flip bits

        let first_add = registers.a.overflowing_add(one_compl_value);
        let second_add = first_add.0.overflowing_add(registers.p.has_carry() as u8);

        // Set carry as addition op (we 1's complemented N)
        registers.p.set_carry(first_add.1 || second_add.1);

        registers.p.update_for_value(second_add.0);

        registers.p.set_overflow(
            // Ref: http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
            (*registers.a ^ second_add.0) & (one_compl_value ^ second_add.0) & 0x80 != 0
        );

        registers.a.set(second_add.0);
    }

}

#[cfg(test)]
mod tests
{
    // Reference: https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html

    use super::*;
    use super::super::test_helpers::*;

    #[test]
    fn simple()
    {
        let (op, mut r, mut m) = test_op(Sbc);

        r.a.set(0x10);
        r.p.set_carry(true);
        m.write(0x0000, 0x03);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0x0D, *r.a);
        assert!(r.p.has_carry());
        assert!(!r.p.is_negative());
        assert!(!r.p.is_zero());
        assert!(!r.p.has_overflown());
    }

    #[test]
    fn with_borrow_set()
    {
        let (op, mut r, mut m) = test_op(Sbc);

        r.a.set(0x09);
        r.p.set_carry(false); // Borrow = !Carry
        m.write(0x0000, 0x01);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0x07, *r.a);
        assert!(!r.p.is_negative());
        assert!(!r.p.is_zero());
        assert!(r.p.has_carry());
        assert!(!r.p.has_overflown());
    }

    #[test]
    fn unsigned_borrow_but_no_signed_overflow()
    {
        let (op, mut r, mut m) = test_op(Sbc);

        r.a.set(0x50);
        m.write(0x0000, 0x70);
        r.p.set_carry(true);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0xE0, *r.a);
        assert!(r.p.is_negative());
        assert!(!r.p.is_zero());
        assert!(!r.p.has_carry());
        assert!(!r.p.has_overflown());
    }

    #[test]
    fn unsigned_borrow_and_signed_overflow()
    {
        let (op, mut r, mut m) = test_op(Sbc);

        r.a.set(0x50);
        m.write(0x0000, 0xb0);
        r.p.set_carry(true);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0xA0, *r.a);
        assert!(r.p.is_negative());
        assert!(!r.p.is_zero());
        assert!(!r.p.has_carry());
        assert!(r.p.has_overflown());
    }

    #[test]
    fn no_unsigned_borrow_but_signed_overflow()
    {
        let (op, mut r, mut m) = test_op(Sbc);

        r.a.set(0xD0);
        m.write(0x0000, 0x70);
        r.p.set_carry(true);

        op.call(AddressingMode::Immediate, &mut r, &mut m);

        assert_eq!(0x60, *r.a);
        assert!(!r.p.is_negative());
        assert!(!r.p.is_zero());
        assert!(r.p.has_carry());
        assert!(r.p.has_overflown());
    }
}