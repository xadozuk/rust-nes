use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Stx;
impl Op for Stx
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let addr = self.operand_addr(mode, registers, memory);

        memory.write(addr, *registers.x);
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
        let (op, mut r, mut m) = test_op(Stx);

        r.x.set(0x80);
        m.write(0x0000, 0x10);

        op.call(AddressingMode::ZeroPage, &mut r, &mut m);

        assert_eq!(0x80, m.read(0x0010));
    }
}