mod register;
mod memory;
mod ops;

use register::CpuRegisters;
use memory::Memory;
use ops::OpcodeMap;

const ROM_START: u16 = 0x8000;
const STACK_POINTER_START: u8 = 0xFF;

const RESET_HANDLER_ADDR: u16 = 0xFFFC;

pub struct Cpu
{
    registers:  CpuRegisters,
    memory:     Memory,
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
            self.memory.read_u16(RESET_HANDLER_ADDR)
        );
    }

    pub fn load(&mut self, program: Vec<u8>)
    {
        self.memory.write_slice(ROM_START, &program[..]);
        self.memory.write_u16(0xFFFC, ROM_START);
    }

    pub fn run(&mut self)
    {
        loop
        {
            let opcode = self.memory.read(*self.registers.pc);
            self.registers.pc += 1;

            let opcode = self.opcodes.get(&opcode);

            match opcode
            {
                Some(opcode) =>
                {
                    let op = &opcode.op;
                    // let args = &self.memory.read_slice(*self.registers.pc, op.args_len());
                    // self.registers.pc += op.args_len() as u16;

                    op.call(opcode.mode, &mut self.registers, &mut self.memory);
                },
                None => todo!(),
            }

            if self.registers.p.has_broke() { break }
        }
    }

}