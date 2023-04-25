use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Jsr);
impl Op for Jsr
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        self.stack_push_u16(registers, memory, *registers.pc + 1); // PC is after opcode, 2 bytes operand, -1 (JSR)
        let addr = self.operand_addr(mode, registers, memory);

        registers.pc.set(addr);
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
        let (op, mut r, mut m) = test_op(Jsr);

        r.pc.set(0x200);
        m.write_u16(0x0200, 0x1234);

        op.call(AddressingMode::Absolute, &mut r, &mut m);

        assert_eq!(0x1234, *r.pc);
        assert_eq!(0x0202, op.stack_peek_u16(&r, &m));
    }
}