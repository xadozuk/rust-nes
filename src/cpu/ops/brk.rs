use crate::cpu::{STACK_START, BRK_VECTOR, register::{BREAK_FLAG}};

use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Brk;
impl Op for Brk
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        self.stack_push_u16(registers, memory, *registers.pc);

        let status_register: u8 = Into::<u8>::into(&registers.p) | BREAK_FLAG;
        self.stack_push(registers, memory, status_register);

        registers.pc.set(memory.read_u16(BRK_VECTOR));
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use crate::cpu::{STACK_POINTER_START, STACK_END};
    use super::*;

    #[test]
    fn brk()
    {
        let (op, mut r, mut m) = test_op(Brk);

        r.pc.set(0x2345);
        r.p.set_negative(true);
        r.p.set_carry(true);

        m.write_u16(0xFFFE, 0x1234);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(STACK_POINTER_START - 3, *r.sp);
        assert_eq!(0x23, m.read(STACK_END));
        assert_eq!(0x45, m.read(STACK_END - 1));
        assert_eq!(0b1011_0001, m.read(STACK_END - 2));

        assert_eq!(0x1234, *r.pc);
    }
}