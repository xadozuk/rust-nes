use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Clc);
impl Op for Clc
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        registers.p.set_carry(false);
    }
}

op!(Cld);
impl Op for Cld
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        registers.p.set_decimal_mode(false);
    }
}

op!(Cli);
impl Op for Cli
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        registers.p.set_interrupt_disable(false);
    }
}

op!(Clv);
impl Op for Clv
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        registers.p.set_overflow(false);
    }
}

op!(Sec);
impl Op for Sec
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        registers.p.set_carry(true);
    }
}

op!(Sed);
impl Op for Sed
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        registers.p.set_decimal_mode(true);
    }
}

op!(Sei);
impl Op for Sei
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        registers.p.set_interrupt_disable(true);
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn clc()
    {
        let (op, mut r, mut m) = test_op(Clc);

        r.p.set_carry(true);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert!(!r.p.has_carry());
    }

    #[test]
    fn cld()
    {
        let (op, mut r, mut m) = test_op(Cld);

        r.p.set_decimal_mode(true);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert!(!r.p.decimal_mode());
    }

    #[test]
    fn cli()
    {
        let (op, mut r, mut m) = test_op(Cli);

        r.p.set_interrupt_disable(true);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert!(!r.p.interrupt_disabled());
    }

    #[test]
    fn clv()
    {
        let (op, mut r, mut m) = test_op(Clv);

        r.p.set_overflow(true);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert!(!r.p.has_overflown());
    }

    #[test]
    fn sec()
    {
        let (op, mut r, mut m) = test_op(Sec);

        r.p.set_carry(false);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert!(r.p.has_carry());
    }

    #[test]
    fn sed()
    {
        let (op, mut r, mut m) = test_op(Sed);

        r.p.set_decimal_mode(false);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert!(r.p.decimal_mode());
    }

    #[test]
    fn sei()
    {
        let (op, mut r, mut m) = test_op(Sei);

        r.p.set_interrupt_disable(false);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert!(r.p.interrupt_disabled());
    }

}