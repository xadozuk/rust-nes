use super::Op;

struct AdcImmediate;
impl Op for AdcImmediate
{
    fn call(&self, registers: &mut crate::cpu::CpuRegisters, args: &[u8])
    {
        let add_arg_result = registers.a.overflowing_add(args[0]);
        let add_carry_result = add_arg_result.0.overflowing_add(registers.p.has_carry() as u8);

        registers.p.set_carry(add_arg_result.1 || add_arg_result.1);
        registers.a.set(add_carry_result.0);

        registers.p.update_for_value(add_carry_result.0);

        registers.p.set_overflow(
            // Ref: http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
            *registers.a ^ add_arg_result.0 & args[0] ^ add_arg_result.0 & 0x80 != 0
        )
    }

    fn len(&self) -> usize
    {
        return 2;
    }
}

#[cfg(test)]
mod tests
{
    mod adc_immediate
    {
        use adc::AdcImmediate;
        use crate::cpu::ops::test_helpers::*;

        #[test]
        fn simple()
        {
            let (mut r, op) = test_op(AdcImmediate);

            r.a.set(0x0);

            op.call(&mut r, &[0x01]);

            assert_eq!(0x01, *r.a);
            assert!(!r.p.is_negative());
            assert!(!r.p.is_zero());
            assert!(!r.p.has_carry());
            assert!(!r.p.has_overflown());
        }

        #[test]
        fn with_carry_set()
        {
            let (mut r, op) = test_op(AdcImmediate);

            r.a.set(0x0);
            r.p.set_carry(true);

            op.call(&mut r, &[0x01]);

            assert_eq!(0x02, *r.a);
            assert!(!r.p.is_negative());
            assert!(!r.p.is_zero());
            assert!(!r.p.has_carry());
            assert!(!r.p.has_overflown());
        }

        #[test]
        fn overflow_u8_should_only_set_carry()
        {
            let (mut r, op) = test_op(AdcImmediate);

            r.a.set(0xFF);

            op.call(&mut r, &[0x01]);

            assert_eq!(0x00, *r.a);
            assert!(!r.p.is_negative());
            assert!(r.p.is_zero());
            assert!(r.p.has_carry());
            assert!(!r.p.has_overflown());
        }

        #[test]
        fn two_positive_number_resulting_in_negative_result_overflow_without_carry()
        {
            let (mut r, op) = test_op(AdcImmediate);

            r.a.set(0x50);

            // 0x50 + 0x50 = 0xa0 / 80 + 80 = -96 signed (160 unsigned)
            op.call(&mut r, &[0x50]);

            assert_eq!(0xa0, *r.a);
            assert!(r.p.is_negative());
            assert!(!r.p.is_zero());
            assert!(!r.p.has_carry());
            assert!(r.p.has_overflown());
        }

        #[test]
        fn two_negative_number_resulting_in_positive_result_overflow_with_carry()
        {
            let (mut r, op) = test_op(AdcImmediate);

            r.a.set(0xd0);

            // 0xd0 + 0x90 = 0x160
            // Unsigned: 208 + 144 = 352 (96 + carry)
            // Signed: -48 + -112 = 96
            op.call(&mut r, &[0x90]);

            assert_eq!(0x60, *r.a);
            assert!(!r.p.is_negative());
            assert!(!r.p.is_zero());
            assert!(r.p.has_carry());
            assert!(r.p.has_overflown());
        }

    }
}