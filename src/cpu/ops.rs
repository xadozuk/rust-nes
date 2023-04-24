#[macro_use]
mod macros;

#[cfg(test)]
mod test_helpers;

// Import Ops
mod adc;
mod and;
mod asl;
mod branch;
mod bit;
mod brk;
mod flags;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod eor;
mod inc;
mod inx;
mod iny;
mod jmp;
mod jsr;
mod lda;
mod ldx;
mod ldy;
mod lsr;
mod ora;
mod pha;
mod php;
mod pla;
mod plp;
mod rol;
mod ror;
mod rti;
mod rts;
mod sbc;
mod sta;
mod stx;
mod sty;
mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;

use std::{collections::HashMap};
use super::{CpuRegisters, Memory, STACK_START};

#[derive(Debug, Clone, Copy)]
pub enum AddressingMode
{
    Implicit,
    Accumulator,
    Relative,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY
}

pub type OpcodeMap = HashMap<u8, Opcode>;
pub struct Opcode
{
    pub opcode: u8,
    pub mode:   AddressingMode,
    pub op:     Box<dyn Op>
}

pub trait Op
{
    fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory);

    fn operand_addr(&self, mode: AddressingMode, registers: &CpuRegisters, memory: &Memory) -> u16
    {
        match mode
        {
            AddressingMode::Immediate => *registers.pc,
            AddressingMode::ZeroPage  => memory.read(*registers.pc) as u16,
            AddressingMode::ZeroPageX => memory.read(*registers.pc).wrapping_add(*registers.x) as u16,
            AddressingMode::ZeroPageY => memory.read(*registers.pc).wrapping_add(*registers.y) as u16,

            AddressingMode::Absolute  => memory.read_u16(*registers.pc),
            AddressingMode::AbsoluteX => memory.read_u16(*registers.pc) + *registers.x as u16,
            AddressingMode::AbsoluteY => memory.read_u16(*registers.pc) + *registers.y as u16,

            AddressingMode::Indirect => memory.read_u16(memory.read_u16(*registers.pc)),

            AddressingMode::IndirectX => {
                let lsb_addr = memory.read(*registers.pc).wrapping_add(*registers.x);

                let lsb = memory.read(lsb_addr as u16);
                let msb = memory.read(lsb_addr.wrapping_add(1) as u16);

                u16::from_le_bytes([lsb, msb])
            },

            AddressingMode::IndirectY => {
                let lsb_addr = memory.read(*registers.pc);

                let lsb = memory.read(lsb_addr as u16);
                let msb = memory.read(lsb_addr.wrapping_add(1) as u16);

                u16::from_le_bytes([lsb, msb]) + *registers.y as u16
            },

            AddressingMode::Relative => {
                let jump_size = memory.read(*registers.pc) as i8; // To conserve sign between casting

                registers.pc
                    .wrapping_add(1) // Add the operand size (in CPU cycle it is added after op execution)
                    .wrapping_add(jump_size as u16)
            }

            AddressingMode::Accumulator => panic!("You cannot get operand address for Accumulator addressing mode"),
            AddressingMode::Implicit => panic!("You cannot get operand address for Implicit addressing mode")
        }
    }

    fn operand(&self, mode: AddressingMode, registers: &CpuRegisters, memory: &Memory) -> u8
    {
        if let AddressingMode::Accumulator = mode { return *registers.a; }

        memory.read(self.operand_addr(mode, registers, memory))
    }

    fn stack_push(&self, registers: &mut CpuRegisters, memory: &mut Memory, value: u8)
    {
        memory.write(STACK_START + registers.sp.decrement() as u16, value);
    }

    fn stack_push_u16(&self, registers: &mut CpuRegisters, memory: &mut Memory, value: u16)
    {
        let msb = (value >> 8) as u8;
        let lsb = (value & 0xFF) as u8;

        // Push msb first, then lsb
        self.stack_push(registers, memory, msb);
        self.stack_push(registers, memory, lsb);
    }

    fn stack_pop(&self, registers: &mut CpuRegisters, memory: &Memory) -> u8
    {
        memory.read(STACK_START + registers.sp.increment() as u16)
    }

    fn stack_pop_u16(&self, registers: &mut CpuRegisters, memory: &Memory) -> u16
    {
        let lsb = memory.read(STACK_START + registers.sp.increment() as u16);
        let msb = memory.read(STACK_START + registers.sp.increment() as u16);

        ((msb as u16) << 8) as u16 | lsb as u16
    }

    // Mainly used for testing
    fn stack_peek(&self, registers: &CpuRegisters, memory: &Memory) -> u8
    {
        memory.read(STACK_START + registers.sp.wrapping_add(1) as u16)
    }

    fn stack_peek_u16(&self, registers: &CpuRegisters, memory: &Memory) -> u16
    {
        let lsb = memory.read(STACK_START + registers.sp.wrapping_add(1) as u16);
        let msb = memory.read(STACK_START + registers.sp.wrapping_add(2) as u16);

        ((msb as u16) << 8) as u16 | lsb as u16
    }

}

pub fn opcodes() -> OpcodeMap
{
    opcodes!(
        (0x69, AddressingMode::Immediate, adc::Adc),
        (0x65, AddressingMode::ZeroPage,  adc::Adc),
        (0x75, AddressingMode::ZeroPageX, adc::Adc),
        (0x6D, AddressingMode::Absolute,  adc::Adc),
        (0x7D, AddressingMode::AbsoluteX, adc::Adc),
        (0x79, AddressingMode::AbsoluteY, adc::Adc)
        // TODO: ADC Indirect,X
        // TODO: ADC Indirect,Y
    )
}

#[cfg(test)]
mod tests
{
    use super::{*, test_helpers::test_op};

    struct DummyOp;
    impl Op for DummyOp
    {
        fn call(&self, mode: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory) {}
    }

    #[test]
    #[should_panic(expected = "You cannot get operand address for Implicit addressing mode")]
    fn implicit_operand_addr()
    {
        let (op, mut r, mut m) = test_op(DummyOp);
        op.operand_addr(AddressingMode::Implicit, &r, &m);
    }

    #[test]
    #[should_panic]
    fn implicit_operand()
    {
        let (op, mut r, mut m) = test_op(DummyOp);
        op.operand(AddressingMode::Implicit, &r, &m);
    }

    #[test]
    fn accumulator()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        r.a.set(0x50);

        assert_eq!(0x50, op.operand(AddressingMode::Accumulator, &r, &m));
    }

    #[test]
    #[should_panic(expected = "You cannot get operand address for Accumulator addressing mode")]
    fn accumulator_operand_addr()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        op.operand_addr(AddressingMode::Accumulator, &r, &m);
    }

    #[test]
    fn immediate()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        r.pc.set(0x1000);
        m.write(0x1000, 0x50);

        assert_eq!(0x1000, op.operand_addr(AddressingMode::Immediate, &r, &m));
        assert_eq!(0x50, op.operand(AddressingMode::Immediate, &r, &m));
    }

    #[test]
    fn zero_page()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write(0x0000, 0x80);
        m.write(0x0080, 0xFF);

        assert_eq!(0x0080, op.operand_addr(AddressingMode::ZeroPage, &r, &m));
        assert_eq!(0xFF, op.operand(AddressingMode::ZeroPage, &r, &m));
    }

    #[test]
    fn zero_page_x()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write(0x0000, 0x80);
        r.x.set(0x0F);

        m.write(0x008F, 0xAA);

        assert_eq!(0x008F, op.operand_addr(AddressingMode::ZeroPageX, &r, &m));
        assert_eq!(0xAA, op.operand(AddressingMode::ZeroPageX, &r, &m));
    }

    #[test]
    fn zero_page_x_page_wrap()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write(0x0000, 0xFF);
        r.x.set(0x81);

        m.write(0x0080, 0xAA);

        assert_eq!(0x0080, op.operand_addr(AddressingMode::ZeroPageX, &r, &m));
        assert_eq!(0xAA, op.operand(AddressingMode::ZeroPageX, &r, &m));
    }

    #[test]
    fn zero_page_y()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write(0x0000, 0x80);
        r.y.set(0x0C);

        m.write(0x008C, 0xCC);

        assert_eq!(0x008C, op.operand_addr(AddressingMode::ZeroPageY, &r, &m));
        assert_eq!(0xCC, op.operand(AddressingMode::ZeroPageY, &r, &m));
    }

    #[test]
    fn zero_page_y_page_wrap()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write(0x0000, 0xFF);
        r.y.set(0x81);

        m.write(0x0080, 0xAA);

        assert_eq!(0x0080, op.operand_addr(AddressingMode::ZeroPageY, &r, &m));
        assert_eq!(0xAA, op.operand(AddressingMode::ZeroPageY, &r, &m));
    }

    #[test]
    fn absolute()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write_u16(0x0000, 0x1234);

        m.write(0x1234, 0x80);

        assert_eq!(0x1234, op.operand_addr(AddressingMode::Absolute, &r, &m));
        assert_eq!(0x80, op.operand(AddressingMode::Absolute, &r, &m));
    }

    #[test]
    fn absolute_x()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write_u16(0x0000, 0x2000);
        r.x.set(0x92);

        m.write(0x2092, 0x80);

        assert_eq!(0x2092, op.operand_addr(AddressingMode::AbsoluteX, &r, &m));
        assert_eq!(0x80, op.operand(AddressingMode::AbsoluteX, &r, &m));
    }

    #[test]
    fn absolute_y()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write_u16(0x0000, 0x2000);
        r.y.set(0x92);

        m.write(0x2092, 0x80);

        assert_eq!(0x2092, op.operand_addr(AddressingMode::AbsoluteY, &r, &m));
        assert_eq!(0x80, op.operand(AddressingMode::AbsoluteY, &r, &m));
    }

    #[test]
    fn indirect()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write_u16(0x0000, 0x2000);
        m.write_u16(0x2000, 0x4000);

        m.write(0x4000, 0x80);

        assert_eq!(0x4000, op.operand_addr(AddressingMode::Indirect, &r, &m));
        assert_eq!(0x80, op.operand(AddressingMode::Indirect, &r, &m));
    }

    #[test]
    fn indirect_x()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write(0x0000, 0x80);
        r.x.set(0x0F);

        m.write_u16(0x008F, 0x4000);
        m.write(0x4000, 0xFF);

        assert_eq!(0x4000, op.operand_addr(AddressingMode::IndirectX, &r, &m));
        assert_eq!(0xFF, op.operand(AddressingMode::IndirectX, &r, &m));
    }

    #[test]
    fn indirect_x_page_wrap()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        r.pc.set(0xFF00);
        m.write(0xFF00, 0xFF);
        r.x.set(0x00);

        m.write(0x00FF, 0x34);
        m.write(0x0000, 0x12);
        m.write(0x1234, 0xFF);

        assert_eq!(0x1234, op.operand_addr(AddressingMode::IndirectX, &r, &m));
        assert_eq!(0xFF, op.operand(AddressingMode::IndirectX, &r, &m));
    }

    #[test]
    fn indirect_y()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        m.write(0x0000, 0x80);

        m.write_u16(0x0080, 0x1200);
        r.y.set(0x34);

        m.write(0x1234, 0xFF);

        assert_eq!(0x1234, op.operand_addr(AddressingMode::IndirectY, &r, &m));
        assert_eq!(0xFF, op.operand(AddressingMode::IndirectY, &r, &m));
    }

    #[test]
    fn indirect_y_page_wrap()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        r.pc.set(0xFF00);
        m.write(0xFF00, 0xFF);

        m.write(0x00FF, 0x34);
        m.write(0x0000, 0x12);
        r.y.set(0x02);

        m.write(0x1236, 0xFF);

        assert_eq!(0x1236, op.operand_addr(AddressingMode::IndirectY, &r, &m));
        assert_eq!(0xFF, op.operand(AddressingMode::IndirectY, &r, &m));
    }

    #[test]
    fn relative_forward()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        r.pc.set(0x0000);
        m.write(0x0000, 0x10);

        // 0x0010 + 1 (operand)
        assert_eq!(0x0011, op.operand_addr(AddressingMode::Relative, &r, &m));
    }

    #[test]
    fn relative_backward()
    {
        let (op, mut r, mut m) = test_op(DummyOp);

        r.pc.set(0x1000);
        m.write(0x1000, 0xF6); // -10 as signed

        // 0x1000 + 1 - 0xF6
        assert_eq!(0x0FF7, op.operand_addr(AddressingMode::Relative, &r, &m));
    }

}