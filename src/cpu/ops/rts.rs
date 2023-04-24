use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Rts;
impl Op for Rts
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let pc_addr = self.stack_pop_u16(registers, memory);
        registers.pc.set(pc_addr + 1);
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
        let (op, mut r, mut m) = test_op(Rts);

        op.stack_push_u16(&mut r, &mut m, 0x8000);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x8001, *r.pc);
    }
}