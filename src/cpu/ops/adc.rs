use crate::cpu::memory::Memory;
use crate::cpu::CpuRegisters;

use super::{Op, AddressingMode};

pub struct Adc;
impl Op for Adc
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let addr = self.get_operand_addr(mode, registers, memory);
        let value = memory.read(addr);

        let add_arg_result = registers.a.overflowing_add(value);
        let add_carry_result = add_arg_result.0.overflowing_add(registers.p.has_carry() as u8);

        registers.p.set_carry(add_arg_result.1 || add_arg_result.1);

        registers.p.update_for_value(add_carry_result.0);

        registers.p.set_overflow(
            // Ref: http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
            (*registers.a ^ add_carry_result.0) & (value ^ add_carry_result.0) & 0x80 != 0
        );

        registers.a.set(add_carry_result.0);
    }

    fn len(&self) -> usize
    {
        return 2;
    }

}

#[cfg(test)]
mod tests
{
    pub use super::Adc;
    pub use super::super::AddressingMode;
    pub use super::super::test_helpers::*;

    mod immediate
    {
        use super::*;

        #[test]
        fn simple()
        {
            let (op, mut r, mut m) = test_op(Adc);

            m.write(0x0000, 0x1);

            op.call(AddressingMode::Immediate, &mut r, &mut m);

            assert_eq!(0x01, *r.a);
            assert!(!r.p.is_negative());
            assert!(!r.p.is_zero());
            assert!(!r.p.has_carry());
            assert!(!r.p.has_overflown());
        }

        #[test]
        fn with_carry_set()
        {
            let (op, mut r, mut m) = test_op(Adc);

            r.p.set_carry(true);
            m.write(0x0000, 0x1);

            op.call(AddressingMode::Immediate, &mut r, &mut m);

            assert_eq!(0x02, *r.a);
            assert!(!r.p.is_negative());
            assert!(!r.p.is_zero());
            assert!(!r.p.has_carry());
            assert!(!r.p.has_overflown());
        }

        #[test]
        fn overflow_u8_should_only_set_carry()
        {
            let (op, mut r, mut m) = test_op(Adc);

            r.a.set(0xFF);
            m.write(0x0000, 0x1);

            op.call(AddressingMode::Immediate, &mut r, &mut m);

            assert_eq!(0x00, *r.a);
            assert!(!r.p.is_negative());
            assert!(r.p.is_zero());
            assert!(r.p.has_carry());
            assert!(!r.p.has_overflown());
        }

        #[test]
        fn two_positive_number_resulting_in_negative_result_overflow_without_carry()
        {
            let (op, mut r, mut m) = test_op(Adc);

            r.a.set(0x50);
            m.write(0x0000, 0x50);

            // 0x50 + 0x50 = 0xa0 / 80 + 80 = -96 signed (160 unsigned)
            op.call(AddressingMode::Immediate, &mut r, &mut m);

            assert_eq!(0xa0, *r.a);
            assert!(r.p.is_negative());
            assert!(!r.p.is_zero());
            assert!(!r.p.has_carry());
            assert!(r.p.has_overflown());
        }

        #[test]
        fn two_negative_number_resulting_in_positive_result_overflow_with_carry()
        {
            let (op, mut r, mut m) = test_op(Adc);

            // 0x50 + 0x50 = 0xa0 / 80 + 80 = -96 signed (160 unsigned)
            r.a.set(0xd0);
            m.write(0x0000, 0x90);

            // 0xd0 + 0x90 = 0x160
            // Unsigned: 208 + 144 = 352 (96 + carry)
            // Signed: -48 + -112 = 96
            op.call(AddressingMode::Immediate, &mut r, &mut m);

            assert_eq!(0x60, *r.a);
            assert!(!r.p.is_negative());
            assert!(!r.p.is_zero());
            assert!(r.p.has_carry());
            assert!(r.p.has_overflown());
        }
    }
}