use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Jmp;
impl Op for Jmp
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        let addr = match mode
        {
            // Respect buggy behavior
            AddressingMode::Indirect => {
                let addr = memory.read_u16(*registers.pc);
                let page = addr & 0xFF00; // Keep only the page number
                let lsb_addr  = (addr & 0xFF) as u8;

                let lsb = memory.read(page | lsb_addr as u16);
                let msb = memory.read(page | lsb_addr.wrapping_add(1) as u16);

                u16::from_le_bytes([lsb, msb])
            },
            _ => self.operand_addr(mode, registers, memory)
        };

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
        let (op, mut r, mut m) = test_op(Jmp);

        m.write_u16(0x0000, 0x1234);

        op.call(AddressingMode::Absolute, &mut r, &mut m);

        assert_eq!(0x1234, *r.pc);
    }

    #[test]
    fn buggy_indirect()
    {
        let (op, mut r, mut m) = test_op(Jmp);

        m.write(0x10FF, 0x89);
        m.write(0x1100, 0x12);
        m.write(0x1000, 0x34);

        m.write_u16(0x0000, 0x10FF);

        op.call(AddressingMode::Indirect, &mut r, &mut m);

        assert_eq!(0x3489, *r.pc);
    }

}