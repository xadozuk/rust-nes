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
mod nop;
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
use std::fmt::Debug;
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

#[derive(Debug)]
pub struct Opcode
{
    pub opcode: u8,
    pub mode:   AddressingMode,
    pub op:     Box<dyn Op>
}

pub trait Op : Debug
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
        (0x79, AddressingMode::AbsoluteY, adc::Adc),
        (0x61, AddressingMode::IndirectX, adc::Adc),
        (0x71, AddressingMode::IndirectY, adc::Adc),

        (0x29, AddressingMode::Immediate, and::And),
        (0x25, AddressingMode::ZeroPage,  and::And),
        (0x35, AddressingMode::ZeroPageX, and::And),
        (0x2D, AddressingMode::Absolute,  and::And),
        (0x3D, AddressingMode::AbsoluteX, and::And),
        (0x39, AddressingMode::AbsoluteY, and::And),
        (0x21, AddressingMode::IndirectX, and::And),
        (0x31, AddressingMode::IndirectY, and::And),

        (0x0A, AddressingMode::Accumulator, asl::Asl),
        (0x06, AddressingMode::ZeroPage,    asl::Asl),
        (0x16, AddressingMode::ZeroPageX,   asl::Asl),
        (0x0E, AddressingMode::Absolute,    asl::Asl),
        (0x1E, AddressingMode::AbsoluteX,   asl::Asl),

        (0x90, AddressingMode::Relative, branch::Bcc),
        (0xB0, AddressingMode::Relative, branch::Bcs),
        (0xF0, AddressingMode::Relative, branch::Beq),
        (0x30, AddressingMode::Relative, branch::Bmi),
        (0xD0, AddressingMode::Relative, branch::Bne),
        (0x10, AddressingMode::Relative, branch::Bpl),
        (0x50, AddressingMode::Relative, branch::Bvc),
        (0x70, AddressingMode::Relative, branch::Bvs),

        (0x24, AddressingMode::ZeroPage, bit::Bit),
        (0x2C, AddressingMode::Absolute, bit::Bit),

        (0x00, AddressingMode::Implicit, brk::Brk),

        (0x18, AddressingMode::Implicit, flags::Clc),
        (0xD8, AddressingMode::Implicit, flags::Cld),
        (0x58, AddressingMode::Implicit, flags::Cli),
        (0xB8, AddressingMode::Implicit, flags::Clv),
        (0x38, AddressingMode::Implicit, flags::Sec),
        (0xF8, AddressingMode::Implicit, flags::Sed),
        (0x78, AddressingMode::Implicit, flags::Sei),

        (0xC9, AddressingMode::Immediate, cmp::Cmp),
        (0xC5, AddressingMode::ZeroPage,  cmp::Cmp),
        (0xD5, AddressingMode::ZeroPageX, cmp::Cmp),
        (0xCD, AddressingMode::Absolute,  cmp::Cmp),
        (0xDD, AddressingMode::AbsoluteX, cmp::Cmp),
        (0xD9, AddressingMode::AbsoluteY, cmp::Cmp),
        (0xC1, AddressingMode::IndirectX, cmp::Cmp),
        (0xD1, AddressingMode::IndirectY, cmp::Cmp),

        (0xE0, AddressingMode::Immediate, cpx::Cpx),
        (0xE4, AddressingMode::ZeroPage,  cpx::Cpx),
        (0xEC, AddressingMode::Absolute,  cpx::Cpx),

        (0xC0, AddressingMode::Immediate, cpy::Cpy),
        (0xC4, AddressingMode::ZeroPage,  cpy::Cpy),
        (0xCC, AddressingMode::Absolute,  cpy::Cpy),

        (0xC6, AddressingMode::ZeroPage,  dec::Dec),
        (0xD6, AddressingMode::ZeroPageX, dec::Dec),
        (0xCE, AddressingMode::Absolute,  dec::Dec),
        (0xDE, AddressingMode::AbsoluteX, dec::Dec),

        (0xCA, AddressingMode::Implicit, dex::Dex),
        (0x88, AddressingMode::Implicit, dey::Dey),

        (0x49, AddressingMode::Immediate, eor::Eor),
        (0x45, AddressingMode::ZeroPage,  eor::Eor),
        (0x55, AddressingMode::ZeroPageX, eor::Eor),
        (0x4D, AddressingMode::Absolute,  eor::Eor),
        (0x5D, AddressingMode::AbsoluteX, eor::Eor),
        (0x59, AddressingMode::AbsoluteY, eor::Eor),
        (0x41, AddressingMode::IndirectX, eor::Eor),
        (0x51, AddressingMode::IndirectY, eor::Eor),

        (0xE6, AddressingMode::ZeroPage,  inc::Inc),
        (0xF6, AddressingMode::ZeroPageX, inc::Inc),
        (0xEE, AddressingMode::Absolute,  inc::Inc),
        (0xFE, AddressingMode::AbsoluteX, inc::Inc),

        (0xE8, AddressingMode::Implicit, inx::Inx),
        (0xC8, AddressingMode::Implicit, iny::Iny),

        (0x4C, AddressingMode::Absolute, jmp::Jmp),
        (0x6C, AddressingMode::Indirect, jmp::Jmp),

        (0x20, AddressingMode::Absolute, jsr::Jsr),

        (0xA9, AddressingMode::Immediate, lda::Lda),
        (0xA5, AddressingMode::ZeroPage,  lda::Lda),
        (0xB5, AddressingMode::ZeroPageX, lda::Lda),
        (0xAD, AddressingMode::Absolute,  lda::Lda),
        (0xBD, AddressingMode::AbsoluteX, lda::Lda),
        (0xB9, AddressingMode::AbsoluteY, lda::Lda),
        (0xA1, AddressingMode::IndirectX, lda::Lda),
        (0xB1, AddressingMode::IndirectY, lda::Lda),

        (0xA2, AddressingMode::Immediate, ldx::Ldx),
        (0xA6, AddressingMode::ZeroPage,  ldx::Ldx),
        (0xB6, AddressingMode::ZeroPageY, ldx::Ldx),
        (0xAE, AddressingMode::Absolute,  ldx::Ldx),
        (0xBE, AddressingMode::AbsoluteY, ldx::Ldx),

        (0xA0, AddressingMode::Immediate, ldy::Ldy),
        (0xA4, AddressingMode::ZeroPage,  ldy::Ldy),
        (0xB4, AddressingMode::ZeroPageX, ldy::Ldy),
        (0xAC, AddressingMode::Absolute,  ldy::Ldy),
        (0xBC, AddressingMode::AbsoluteX, ldy::Ldy),

        (0x4A, AddressingMode::Accumulator, lsr::Lsr),
        (0x46, AddressingMode::ZeroPage,    lsr::Lsr),
        (0x56, AddressingMode::ZeroPageX,   lsr::Lsr),
        (0x4E, AddressingMode::Absolute,    lsr::Lsr),
        (0x5E, AddressingMode::AbsoluteX,   lsr::Lsr),

        (0xEA, AddressingMode::Implicit, nop::Nop),

        (0x09, AddressingMode::Immediate, ora::Ora),
        (0x05, AddressingMode::ZeroPage,  ora::Ora),
        (0x15, AddressingMode::ZeroPageX, ora::Ora),
        (0x0D, AddressingMode::Absolute,  ora::Ora),
        (0x1D, AddressingMode::AbsoluteX, ora::Ora),
        (0x19, AddressingMode::AbsoluteY, ora::Ora),
        (0x01, AddressingMode::IndirectX, ora::Ora),
        (0x11, AddressingMode::IndirectY, ora::Ora),

        (0x48, AddressingMode::Implicit, pha::Pha),
        (0x08, AddressingMode::Implicit, php::Php),
        (0x68, AddressingMode::Implicit, pla::Pla),
        (0x28, AddressingMode::Implicit, plp::Plp),

        (0x2A, AddressingMode::Accumulator, rol::Rol),
        (0x26, AddressingMode::ZeroPage,    rol::Rol),
        (0x36, AddressingMode::ZeroPageX,   rol::Rol),
        (0x2E, AddressingMode::Absolute,    rol::Rol),
        (0x3E, AddressingMode::AbsoluteX,   rol::Rol),

        (0x6A, AddressingMode::Accumulator, ror::Ror),
        (0x66, AddressingMode::ZeroPage,    ror::Ror),
        (0x76, AddressingMode::ZeroPageX,   ror::Ror),
        (0x6E, AddressingMode::Absolute,    ror::Ror),
        (0x7E, AddressingMode::AbsoluteX,   ror::Ror),

        (0x40, AddressingMode::Implicit, rti::Rti),
        (0x60, AddressingMode::Implicit, rts::Rts),

        (0xE9, AddressingMode::Immediate, sbc::Sbc),
        (0xE5, AddressingMode::ZeroPage,  sbc::Sbc),
        (0xF5, AddressingMode::ZeroPageX, sbc::Sbc),
        (0xED, AddressingMode::Absolute,  sbc::Sbc),
        (0xFD, AddressingMode::AbsoluteX, sbc::Sbc),
        (0xF9, AddressingMode::AbsoluteY, sbc::Sbc),
        (0xE1, AddressingMode::IndirectX, sbc::Sbc),
        (0xF1, AddressingMode::IndirectY, sbc::Sbc),

        (0x85, AddressingMode::ZeroPage,  sta::Sta),
        (0x95, AddressingMode::ZeroPageX, sta::Sta),
        (0x8D, AddressingMode::Absolute,  sta::Sta),
        (0x9D, AddressingMode::AbsoluteX, sta::Sta),
        (0x99, AddressingMode::AbsoluteY, sta::Sta),
        (0x81, AddressingMode::IndirectX, sta::Sta),
        (0x91, AddressingMode::IndirectY, sta::Sta),

        (0x86, AddressingMode::ZeroPage,  stx::Stx),
        (0x96, AddressingMode::ZeroPageY, stx::Stx),
        (0x8E, AddressingMode::Absolute,  stx::Stx),

        (0x84, AddressingMode::ZeroPage,  sty::Sty),
        (0x94, AddressingMode::ZeroPageX, sty::Sty),
        (0x8C, AddressingMode::Absolute,  sty::Sty),

        (0xAA, AddressingMode::Implicit, tax::Tax),
        (0xA8, AddressingMode::Implicit, tay::Tay),
        (0xBA, AddressingMode::Implicit, tsx::Tsx),
        (0x8A, AddressingMode::Implicit, txa::Txa),
        (0x9A, AddressingMode::Implicit, txs::Txs),
        (0x98, AddressingMode::Implicit, tya::Tya)
    )
}

pub fn opcode_length(mode: AddressingMode) -> u8
{
    match mode
    {
        AddressingMode::Immediate => 2,
        AddressingMode::ZeroPage | AddressingMode::ZeroPageX | AddressingMode::ZeroPageY => 2,
        AddressingMode::Relative => 2,
        AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY => 3,
        AddressingMode::Indirect  => 3,
        AddressingMode::IndirectX | AddressingMode::IndirectY => 2,
        AddressingMode::Accumulator => 1,
        AddressingMode::Implicit => 1,
    }
}

#[cfg(test)]
mod tests
{
    use super::{*, test_helpers::test_op};

    op!(DummyOp);
    impl Op for DummyOp
    {
        fn call(&self, _: AddressingMode, _: &mut CpuRegisters, _: &mut Memory) {}
    }

    #[test]
    #[should_panic(expected = "You cannot get operand address for Implicit addressing mode")]
    fn implicit_operand_addr()
    {
        let (op, r, m) = test_op(DummyOp);
        op.operand_addr(AddressingMode::Implicit, &r, &m);
    }

    #[test]
    #[should_panic]
    fn implicit_operand()
    {
        let (op, r, m) = test_op(DummyOp);
        op.operand(AddressingMode::Implicit, &r, &m);
    }

    #[test]
    fn accumulator()
    {
        let (op, mut r, m) = test_op(DummyOp);

        r.a.set(0x50);

        assert_eq!(0x50, op.operand(AddressingMode::Accumulator, &r, &m));
    }

    #[test]
    #[should_panic(expected = "You cannot get operand address for Accumulator addressing mode")]
    fn accumulator_operand_addr()
    {
        let (op, r, m) = test_op(DummyOp);

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
        let (op, r, mut m) = test_op(DummyOp);

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
        let (op, r, mut m) = test_op(DummyOp);

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
        let (op, r, mut m) = test_op(DummyOp);

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