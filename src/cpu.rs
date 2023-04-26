mod register;
mod memory;
mod ops;

use std::fmt::{Debug};
use register::CpuRegisters;
use memory::Memory;
use ops::OpcodeMap;

use crate::rom::Rom;

use self::ops::{opcode_length, Opcode};

const ROM_START: u16          = 0x8000;
const STACK_START: u16        = 0x0100;
const STACK_END: u16          = 0x01FF;
const STACK_POINTER_START: u8 = 0xFF;

const NMI_VECTOR: u16   = 0xFFFA;
const RESET_VECTOR: u16 = 0xFFFC;
const BRK_VECTOR: u16   = 0xFFFE;

pub struct Cpu
{
    registers:  CpuRegisters,
    pub memory: Memory, // TODO: remove pub
    opcodes:    OpcodeMap,
}

impl Cpu
{
    pub fn new() -> Cpu
    {
        Cpu {
            registers:  CpuRegisters::new(),
            memory:     Memory::new(),
            opcodes:    ops::opcodes(),
        }
    }

    pub fn reset(&mut self)
    {
        self.registers.a.set(0);
        self.registers.x.set(0);
        self.registers.y.set(0);
        self.registers.sp.set(STACK_POINTER_START);
        self.registers.p.reset();

        self.registers.pc.set(
            self.memory.read_u16(RESET_VECTOR)
        );
    }

    pub fn load(&mut self, program: Vec<u8>)
    {
        self.load_at(ROM_START, program);
    }

    pub fn load_rom(&mut self, rom: Rom)
    {
        self.memory.load_rom(rom);
    }

    pub fn load_at(&mut self, start_addr: u16, program: Vec<u8>)
    {
        self.memory.write_slice(start_addr, &program[..]);
        self.memory.write_u16(0xFFFC, start_addr);
    }

    pub fn run<F>(&mut self, mut callback: F)
        where F: FnMut(&mut Cpu)
    {
        loop
        {
            println!("{:?}", &self);

            callback(self);

            let opcode = self.memory.read(*self.registers.pc);
            self.registers.pc += 1;

            let pc_state = *self.registers.pc;

            let op_metadata = self.opcodes.get(&opcode);

            match op_metadata
            {
                Some(metadata) =>
                {
                    self.print_opcode_debug(&metadata);

                    let op = &metadata.op;
                    // let args = &self.memory.read_slice(*self.registers.pc, op.args_len());
                    // self.registers.pc += op.args_len() as u16;

                    op.call(metadata.mode, &mut self.registers, &mut self.memory);

                    // If the PC has not moved, we progress over the operand
                    if pc_state == *self.registers.pc
                    {
                        self.registers.pc += (opcode_length(metadata.mode) - 1) as u16; // Remove the opcode byte as we already moved over it
                    }

                },
                None => panic!("Unsupported opcode 0x{:02X}", opcode),
            }

            // Exit on BRK
            // TODO: align with correct NES impl
            if self.registers.p.has_broken() { break }
        }
    }

    fn print_opcode_debug(&self, metadata: &Opcode)
    {
        println!("* {0:#04X} ({1:?}) - AddressingMode::{2:?}", metadata.opcode, metadata.op, metadata.mode);
        print!("> ");

        for operand_addr in 0..opcode_length(metadata.mode)-1
        {
            print!("{:#04X} ", self.memory.read(*self.registers.pc + operand_addr as u16));
        }

        println!("\n");
    }

}

impl Debug for Cpu
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        writeln!(f, "=== CPU ===")?;
        writeln!(f, "Registers\tA:{0:#04X} X:{1:#04X} Y:{2:#04X} SP:{3:#04X} PC:{4:#06X} P:{5:#010b} (NV__DIZC)",
            *self.registers.a,
            *self.registers.x,
            *self.registers.y,
            *self.registers.sp,
            *self.registers.pc,
            Into::<u8>::into(&self.registers.p)
        )?;
        write!(f, "Stack: \t\t")?;

        for sp_addr in *self.registers.sp..0xFF
        {
            write!(f, "{:#04X} ", self.memory.read(STACK_START + (sp_addr + 1) as u16))?;
        }

        writeln!(f, "")
    }
}