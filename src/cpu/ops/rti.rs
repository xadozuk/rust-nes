use crate::cpu::register::StatusRegister;

use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Rti);
impl Op for Rti
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let status_register = self.stack_pop(registers, memory);
        let pc_addr = self.stack_pop_u16(registers, memory);

        registers.p = StatusRegister::from(status_register);
        registers.pc.set(pc_addr);
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use super::*;

    // TODO: Check if test is well written
    #[test]
    fn simple()
    {
        let (op, mut r, mut m) = test_op(Rti);

        op.stack_push_u16(&mut r, &mut m, 0x8000);
        // NVss DIZC
        op.stack_push(&mut r, &mut m, 0b1100_0001);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x8000, *r.pc);
        assert_eq!(0xFF, *r.sp);

        assert!(r.p.is_negative());
        assert!(r.p.has_overflown());
        assert!(r.p.has_carry());
    }
}