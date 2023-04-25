use crate::cpu::register::BREAK_FLAG;

use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Php);
impl Op for Php
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        self.stack_push(registers, memory, BREAK_FLAG | Into::<u8>::into(&registers.p));
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
        let (op, mut r, mut m) = test_op(Php);

        r.p.set_carry(true);
        r.p.set_negative(true);
        r.p.set_overflow(true);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0b1111_0001, op.stack_peek(&r, &m));
        assert_eq!(0xFE, *r.sp);
    }
}