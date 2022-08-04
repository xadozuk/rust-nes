mod register;
mod ops;

use register::{Register, StatusRegister};
use ops::{Op, Opcodes};

pub struct CpuRegisters
{
    p:  StatusRegister,
    pc: Register<u16>,
    sp: Register<u8>,

    a:  Register<u8>,
    x:  Register<u8>,
    y:  Register<u8>,
}

impl CpuRegisters
{
    pub fn new() -> CpuRegisters
    {
        CpuRegisters {
            p: StatusRegister::new(),
            pc: Register::new(),
            sp: Register::new(),

            a: Register::new(),
            x: Register::new(),
            y: Register::new(),
        }
    }
}

pub struct Cpu
{
    registers: CpuRegisters,

    program: Vec<u8>, // TODO: move to RAM,
    opcodes: Opcodes,
}

impl Cpu
{
    pub fn new() -> Cpu
    {
        Cpu {
            registers: CpuRegisters::new(),

            program: vec![],
            opcodes: ops::ops(),
        }
    }

    pub fn load(&mut self, program: Vec<u8>)
    {
        self.program = program;
    }

    pub fn run(&mut self)
    {
        loop
        {
            let opcode = self.program[*self.registers.pc as usize];
            self.registers.pc += 1;

            let opcode = self.opcodes.get(&opcode);

            match opcode
            {
                Some(op) =>
                {
                    let pc = *self.registers.pc as usize;
                    let args = &self.program[pc..(pc+op.args_len())];
                    self.registers.pc += op.args_len() as u16;

                    op.call(&mut self.registers, args);
                },
                None => todo!(),
            }

            if self.registers.p.has_broke() { break }
        }
    }
}