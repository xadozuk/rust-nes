use crate::cpu::register::StatusRegister;

use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Plp;
impl Op for Plp
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let status_register_value = self.stack_pop(registers, memory);
        registers.p = StatusRegister::from(status_register_value);
    }
}

#[cfg(test)]
mod tests
{
    use crate::cpu::STACK_START;

    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn simple()
    {
        let (op, mut r, mut m) = test_op(Plp);

        r.sp.set(0x80);
        m.write(STACK_START + 0x81, 0b1100_0001);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x81, *r.sp);

        assert!(r.p.has_carry());
        assert!(r.p.has_overflown());
        assert!(r.p.is_negative());
    }
}